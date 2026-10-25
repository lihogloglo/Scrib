//! Slot world access (scripting.md "Execution model": the world guard).
//!
//! L1 decided on an app-data guard over `mlua::Lua::scope`; L2 decides how the slot's access
//! reaches it: **resources moved in for the slot**. API modules declare the resources they need
//! ([`ScriptsPlugin::slot_resource`](crate::ScriptsPlugin) / [`Scripts::register_slot_resource`]);
//! when a slot opens, each one is moved out of the `World` into one [`SlotAccess`] cell shared
//! by every context (each state's app data holds an `Arc` to it from context creation), and moved
//! back when the slot closes. API functions borrow them through [`with_resource`], which fails
//! with "outside a script slot" while the cell is closed, so a function captured and called
//! later never touches stale data.
//!
//! Why not the alternatives: a raw `World` pointer needs `unsafe` (the workspace forbids it);
//! `scope` costs ~0.5 µs per scoped function per slot and context (L1); installing a guard per
//! context per slot costs ~0.23 µs × contexts × slots (35 µs a frame at 50 mods). The shared
//! cell costs one move per declared resource per slot, independent of the number of contexts.
//! Writes are immediate (scripts read their own writes); the resources are moved without change
//! detection, so API modules that write call [`mark_changed`] for systems that use `is_changed`.

use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::sync::{Arc, Mutex, PoisonError};

use bevy_ecs::component::Mutable;
use bevy_ecs::prelude::*;
use sf_script::mlua;

type Boxed = Box<dyn Any + Send + Sync>;

/// The resources of the open slot, by type.
#[derive(Default)]
struct Open {
    resources: HashMap<TypeId, Boxed>,
    changed: Vec<TypeId>,
}

/// The cell every context's API functions read the open slot's resources from. Cloning shares it.
#[derive(Clone, Default)]
pub struct SlotAccess(Arc<Mutex<Option<Open>>>);

/// App data in every context: the runtime's [`SlotAccess`].
#[derive(Clone)]
pub(crate) struct SlotGuard(pub(crate) SlotAccess);

/// Moves one resource type between the world and the slot cell.
#[derive(Clone, Copy)]
pub(crate) struct Mover {
    pub(crate) id: TypeId,
    take: fn(&mut World) -> Option<Boxed>,
    put: fn(&mut World, Boxed, bool),
}

impl Mover {
    pub(crate) fn of<T: Resource<Mutability = Mutable> + Default>() -> Self {
        fn take<T: Resource<Mutability = Mutable> + Default>(world: &mut World) -> Option<Boxed> {
            let mut r = world.get_resource_mut::<T>()?;
            Some(Box::new(std::mem::take(r.bypass_change_detection())))
        }
        fn put<T: Resource<Mutability = Mutable> + Default>(world: &mut World, value: Boxed, changed: bool) {
            let (Ok(value), Some(mut r)) = (value.downcast::<T>(), world.get_resource_mut::<T>()) else {
                return;
            };
            if changed {
                *r = *value;
            } else {
                *r.bypass_change_detection() = *value;
            }
        }
        Self {
            id: TypeId::of::<T>(),
            take: take::<T>,
            put: put::<T>,
        }
    }
}

impl SlotAccess {
    /// Opens the slot: moves `movers`' resources in.
    pub(crate) fn open(&self, world: &mut World, movers: &[Mover]) {
        let mut open = Open::default();
        for m in movers {
            if let Some(r) = (m.take)(world) {
                open.resources.insert(m.id, r);
            }
        }
        *self.0.lock().unwrap_or_else(PoisonError::into_inner) = Some(open);
    }

    /// Closes the slot: moves the resources back.
    pub(crate) fn close(&self, world: &mut World, movers: &[Mover]) {
        let Some(mut open) = self.0.lock().unwrap_or_else(PoisonError::into_inner).take() else {
            return;
        };
        for m in movers {
            if let Some(r) = open.resources.remove(&m.id) {
                (m.put)(world, r, open.changed.contains(&m.id));
            }
        }
    }

    /// Whether a slot is open.
    pub fn is_open(&self) -> bool {
        self.0.lock().unwrap_or_else(PoisonError::into_inner).is_some()
    }

    pub(crate) fn with<T: Resource, R>(&self, mark: bool, f: impl FnOnce(&mut T) -> R) -> mlua::Result<R> {
        // `try_lock`: an API function that re-enters Lua while holding a resource gets an error,
        // not a deadlock.
        let mut guard = match self.0.try_lock() {
            Ok(g) => g,
            Err(std::sync::TryLockError::Poisoned(p)) => p.into_inner(),
            Err(std::sync::TryLockError::WouldBlock) => {
                return Err(mlua::Error::runtime("world access is already in use (re-entrant API call)"));
            }
        };
        let Some(open) = guard.as_mut() else {
            return Err(mlua::Error::runtime("world access outside a script slot"));
        };
        let id = TypeId::of::<T>();
        if mark && !open.changed.contains(&id) {
            open.changed.push(id);
        }
        let r = open
            .resources
            .get_mut(&id)
            .and_then(|b| b.downcast_mut::<T>())
            .ok_or_else(|| mlua::Error::runtime(format!("{} is not a script slot resource", std::any::type_name::<T>())))?;
        Ok(f(r))
    }
}

/// Runs `f` on the open slot's `T`. Errors outside a slot, for a resource no module declared, or
/// on re-entry. `f` must not call into Lua.
pub fn with_resource<T: Resource, R>(lua: &mlua::Lua, f: impl FnOnce(&mut T) -> R) -> mlua::Result<R> {
    let guard = lua
        .app_data_ref::<SlotGuard>()
        .ok_or_else(|| mlua::Error::runtime("world access outside a script slot (no slot guard)"))?;
    guard.0.with(false, f)
}

/// Marks `T` changed when the slot closes (for systems that use change detection).
pub fn mark_changed<T: Resource>(lua: &mlua::Lua) -> mlua::Result<()> {
    let guard = lua
        .app_data_ref::<SlotGuard>()
        .ok_or_else(|| mlua::Error::runtime("world access outside a script slot (no slot guard)"))?;
    guard.0.with::<T, ()>(true, |_| ())
}
