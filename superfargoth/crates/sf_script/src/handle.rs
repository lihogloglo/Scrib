//! Entity handles (scripting.md "Entity handles and entity scripts").
//!
//! A script never holds an ECS entity. It holds a [`Handle`]: the 128-bit stable id of a
//! placement (`sf_world::PlacementId`'s bits, or a runtime-spawned id from the placement overlay).
//! This crate stays independent of `sf_world`; `sf_script_bevy` converts and resolves handles
//! (`is_loaded`, position, ...) through API modules. Handles are plain data: they serialize as
//! their id and compare by id.
//!
//! Method syntax (`h:position()`): a handle's `__index` falls back to a per-context method table
//! that API modules fill at install time ([`handle_methods`]), so modules living with other
//! systems add methods without this crate knowing them. The table is read-only once every
//! module is installed.

use mlua::{Lua, MetaMethod, Table, UserData, UserDataMethods, Value};
use serde::{Deserialize, Serialize};

/// Registry table: method name → function taking the handle first.
pub(crate) const HANDLE_METHODS: &str = "sf.handle_methods";

/// A stable placement id as seen by scripts.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Handle(pub u128);

/// The context's handle method table. API modules add methods in [`ApiModule::install`]
/// (`methods.raw_set("position", f)`, `f` taking the handle as its first argument).
///
/// [`ApiModule::install`]: crate::ApiModule::install
pub fn handle_methods(lua: &Lua) -> mlua::Result<Table> {
    lua.named_registry_value(HANDLE_METHODS)
}

impl UserData for Handle {
    fn add_methods<M: UserDataMethods<Self>>(methods: &mut M) {
        methods.add_method("id", |_, h, ()| Ok(format!("{:032x}", h.0)));
        methods.add_meta_method(MetaMethod::Eq, |_, h, other: mlua::AnyUserData| {
            Ok(other.borrow::<Handle>().is_ok_and(|o| *o == *h))
        });
        methods.add_meta_method(MetaMethod::ToString, |_, h, ()| {
            Ok(format!("Handle({:032x})", h.0))
        });
        // Methods registered by API modules; unknown names are errors, not nil, so typos surface.
        methods.add_meta_function(MetaMethod::Index, |lua, (_h, name): (Value, mlua::LuaString)| {
            let t: Option<Table> = lua.named_registry_value(HANDLE_METHODS).ok();
            match t.map(|t| t.raw_get::<Value>(&name)).transpose()? {
                Some(f @ Value::Function(_)) => Ok(f),
                _ => Err(mlua::Error::runtime(format!(
                    "handle has no method {:?}",
                    name.to_string_lossy()
                ))),
            }
        });
    }
}
