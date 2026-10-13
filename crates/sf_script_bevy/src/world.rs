//! Placement handles for scripts (scripting.md "Entity handles and entity scripts") and the L2
//! subset of `@sf/world`.
//!
//! A [`Handle`] is a placement's stable 128-bit id (`sf_world::PlacementId.0`), never an ECS
//! entity. [`ScriptPlacements`] is the loaded-placement map handles resolve through: the app's
//! streaming hooks fill it as placements enter and leave the active ring (R0), with the data the
//! API reads (record, worldspace, position). It is a slot resource, moved into the slot cell
//! while scripts run (`access.rs`). L3 grows the module (spawn, attach, `h:position()` methods).

use std::collections::HashMap;

use bevy_ecs::prelude::*;
use sf_script::mlua::{self, AnyUserData, Lua, Table, Value};
use sf_script::{ApiModule, Handle, InstallCx};

use crate::access::{SlotAccess, SlotGuard, with_resource};

/// The player's handle (the player is not a placement). Placement ids are blake3 prefixes, so 0
/// never collides with one in practice.
pub const PLAYER: Handle = Handle(0);

/// What scripts can read about a loaded placement.
#[derive(Clone, Debug, PartialEq)]
pub struct Placed {
    /// Base record id.
    pub record: String,
    /// Worldspace id.
    pub worldspace: String,
    /// World position, meters (engine axes, f64: worldspaces are large).
    pub position: [f64; 3],
    /// Bounding radius, meters (0 without a model).
    pub radius: f32,
}

/// Loaded placements by handle, plus the player's position. Filled by the app's streaming hooks.
#[derive(Resource, Default, Debug)]
pub struct ScriptPlacements {
    pub loaded: HashMap<Handle, Placed>,
    /// The player as a placement-like entry (record = the player's record id, if any).
    pub player: Option<Placed>,
}

impl ScriptPlacements {
    pub fn get(&self, h: Handle) -> Option<&Placed> {
        if h == PLAYER { self.player.as_ref() } else { self.loaded.get(&h) }
    }
}

/// `@sf/world` (L2 subset): `player()`, `is_loaded(h)`, `record(h)`, `worldspace(h)`,
/// `position(h)`. Installing it also hands every context the runtime's slot guard.
pub struct WorldModule {
    pub(crate) access: SlotAccess,
}

fn handle(ud: &AnyUserData) -> mlua::Result<Handle> {
    ud.borrow::<Handle>().map(|h| *h).map_err(|_| mlua::Error::runtime("expected a handle"))
}

fn lookup<R>(lua: &Lua, h: &AnyUserData, f: impl FnOnce(&Placed) -> R) -> mlua::Result<Option<R>> {
    let h = handle(h)?;
    with_resource::<ScriptPlacements, _>(lua, |p| p.get(h).map(f))
}

impl ApiModule for WorldModule {
    fn name(&self) -> &'static str {
        "@sf/world"
    }
    fn version(&self) -> semver::Version {
        semver::Version::new(0, 1, 0)
    }
    fn install(&self, lua: &Lua, _cx: &InstallCx<'_>) -> mlua::Result<Table> {
        lua.set_app_data(SlotGuard(self.access.clone()));
        let t = lua.create_table()?;
        t.set("player", lua.create_function(|_, ()| Ok(PLAYER))?)?;
        t.set(
            "is_loaded",
            lua.create_function(|lua, h: AnyUserData| Ok(lookup(lua, &h, |_| ())?.is_some()))?,
        )?;
        t.set(
            "record",
            lua.create_function(|lua, h: AnyUserData| lookup(lua, &h, |p| p.record.clone()))?,
        )?;
        t.set(
            "worldspace",
            lua.create_function(|lua, h: AnyUserData| lookup(lua, &h, |p| p.worldspace.clone()))?,
        )?;
        t.set(
            "position",
            lua.create_function(|lua, h: AnyUserData| {
                Ok(match lookup(lua, &h, |p| p.position)? {
                    Some([x, y, z]) => (Value::Number(x), Value::Number(y), Value::Number(z)),
                    None => (Value::Nil, Value::Nil, Value::Nil),
                })
            })?,
        )?;
        Ok(t)
    }
    fn types(&self) -> &'static str {
        "declare world: {\n  player: () -> Handle,\n  is_loaded: (h: Handle) -> boolean,\n  \
         record: (h: Handle) -> string?,\n  worldspace: (h: Handle) -> string?,\n  \
         position: (h: Handle) -> (number?, number?, number?),\n}\n"
    }
}
