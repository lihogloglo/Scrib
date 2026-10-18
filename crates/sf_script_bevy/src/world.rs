//! Placement handles for scripts (scripting.md "Entity handles and entity scripts") and
//! `@sf/world` (API v0.1).
//!
//! A [`Handle`] is a placement's stable 128-bit id (`sf_world::PlacementId.0`), never an ECS
//! entity. [`ScriptPlacements`] is the loaded-placement map handles resolve through: the app's
//! streaming hooks fill it as placements enter and leave the active ring (R0), with the data the
//! API reads (record, worldspace, position). It is a slot resource, moved into the slot cell
//! while scripts run (`access.rs`).
//!
//! Structural changes (`spawn`, `despawn`) return at once (scripts read their own writes: a
//! spawned handle is in [`ScriptPlacements`] immediately) and queue a [`WorldCommand`] in the
//! [`ScriptWorld`] slot resource; the queue is flushed as messages when the slot closes, and the
//! content pack applies them (overlay + live spawn). Entity scripts (`define_script`, `attach`,
//! `state`) are `sf_script`'s, exposed here.

use std::collections::HashMap;

use bevy_ecs::prelude::*;
use sf_script::mlua::{self, AnyUserData, Lua, Table, Value};
use sf_script::{ApiModule, Handle, InstallCx};

use crate::access::{SlotAccess, SlotGuard, mark_changed, with_resource};

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

/// A structural change requested by a script, applied by the content pack after the slot.
#[derive(Message, Clone, Debug, PartialEq)]
pub enum WorldCommand {
    /// Create a new placement of `record` (its handle is already in use by the script).
    Spawn {
        handle: Handle,
        record: String,
        worldspace: String,
        position: [f64; 3],
        /// Heading, radians about the up axis.
        yaw: f64,
        /// Requesting context.
        context: String,
    },
    /// Remove a placement (spawned ones are forgotten; baked ones are marked removed).
    Despawn { handle: Handle, context: String },
}

/// Slot resource of `@sf/world`: the command queue and per-context spawn counters.
///
/// Spawned handles are `FNV-1a-128("sf.script.spawn", context, n)` with the top bit set: stable
/// for the same script run (determinism), distinct from baked ids in practice. The counters
/// persist with the placement overlay once saves exist (M7).
#[derive(Resource, Default, Debug)]
pub struct ScriptWorld {
    pub(crate) queue: Vec<WorldCommand>,
    spawned: HashMap<String, u64>,
}

impl ScriptWorld {
    fn next_handle(&mut self, context: &str) -> Handle {
        let n = self.spawned.entry(context.to_owned()).or_default();
        *n += 1;
        let mut h: u128 = 0x6c62_272e_07bb_0142_62b8_2175_6295_c58d;
        for b in b"sf.script.spawn\0".iter().chain(context.as_bytes()).chain(&[0]).chain(&n.to_le_bytes()) {
            h = (h ^ u128::from(*b)).wrapping_mul(0x0000_0000_0100_0000_0000_0000_0000_013b);
        }
        Handle(h | (1 << 127))
    }

    /// Takes the queued commands (the slot runner flushes them as messages).
    pub fn take_commands(&mut self) -> Vec<WorldCommand> {
        std::mem::take(&mut self.queue)
    }
}

/// `@sf/world`. Installing it also hands every context the runtime's slot guard.
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

/// A position argument: a handle (its position and worldspace) or `{ x, y, z, worldspace? }`
/// (default worldspace: the player's).
fn position_arg(lua: &Lua, at: &Value) -> mlua::Result<(String, [f64; 3])> {
    match at {
        Value::UserData(ud) => lookup(lua, ud, |p| (p.worldspace.clone(), p.position))?
            .ok_or_else(|| mlua::Error::runtime("position: the handle is not loaded")),
        Value::Table(t) => {
            let pos = [t.get::<f64>("x")?, t.get::<f64>("y")?, t.get::<f64>("z")?];
            let ws = match t.get::<Option<String>>("worldspace")? {
                Some(ws) => ws,
                None => with_resource::<ScriptPlacements, _>(lua, |p| p.player.as_ref().map(|p| p.worldspace.clone()))?
                    .ok_or_else(|| mlua::Error::runtime("position: no worldspace given and no player"))?,
            };
            Ok((ws, pos))
        }
        _ => Err(mlua::Error::runtime("expected a handle or { x, y, z, worldspace? }")),
    }
}

fn distance(a: [f64; 3], b: [f64; 3]) -> f64 {
    (0..3).map(|i| (a[i] - b[i]).powi(2)).sum::<f64>().sqrt()
}

impl ApiModule for WorldModule {
    fn name(&self) -> &'static str {
        "@sf/world"
    }
    fn version(&self) -> semver::Version {
        semver::Version::new(0, 1, 0)
    }
    #[allow(clippy::too_many_lines, reason = "one binding after another")]
    fn install(&self, lua: &Lua, cx: &InstallCx<'_>) -> mlua::Result<Table> {
        lua.set_app_data(SlotGuard(self.access.clone()));
        let t = lua.create_table()?;
        let methods = sf_script::handle_methods(lua)?;
        t.set("player", lua.create_function(|_, ()| Ok(PLAYER))?)?;
        let is_loaded = lua.create_function(|lua, h: AnyUserData| Ok(lookup(lua, &h, |_| ())?.is_some()))?;
        let record = lua.create_function(|lua, h: AnyUserData| lookup(lua, &h, |p| p.record.clone()))?;
        let worldspace = lua.create_function(|lua, h: AnyUserData| lookup(lua, &h, |p| p.worldspace.clone()))?;
        let position = lua.create_function(|lua, h: AnyUserData| {
            Ok(match lookup(lua, &h, |p| p.position)? {
                Some([x, y, z]) => (Value::Number(x), Value::Number(y), Value::Number(z)),
                None => (Value::Nil, Value::Nil, Value::Nil),
            })
        })?;
        let context = cx.context.to_owned();
        let despawn = lua.create_function(move |lua, h: AnyUserData| {
            let h = handle(&h)?;
            if h == PLAYER {
                return Err(mlua::Error::runtime("despawn: the player can't be despawned"));
            }
            let was = with_resource::<ScriptPlacements, _>(lua, |p| p.loaded.remove(&h).is_some())?;
            mark_changed::<ScriptPlacements>(lua)?;
            with_resource::<ScriptWorld, _>(lua, |w| w.queue.push(WorldCommand::Despawn { handle: h, context: context.clone() }))?;
            Ok(was)
        })?;
        for (name, f) in
            [("is_loaded", &is_loaded), ("record", &record), ("worldspace", &worldspace), ("position", &position), ("despawn", &despawn)]
        {
            t.set(name, f)?;
            methods.raw_set(name, f)?;
        }
        let context = cx.context.to_owned();
        t.set(
            "spawn",
            lua.create_function(move |lua, (record, at, yaw): (String, Value, Option<f64>)| {
                let (worldspace, position) = position_arg(lua, &at)?;
                let handle = with_resource::<ScriptWorld, _>(lua, |w| {
                    let handle = w.next_handle(&context);
                    w.queue.push(WorldCommand::Spawn {
                        handle,
                        record: record.clone(),
                        worldspace: worldspace.clone(),
                        position,
                        yaw: yaw.unwrap_or(0.0),
                        context: context.clone(),
                    });
                    handle
                })?;
                with_resource::<ScriptPlacements, _>(lua, |p| {
                    p.loaded.insert(handle, Placed { record, worldspace, position, radius: 0.0 });
                })?;
                mark_changed::<ScriptPlacements>(lua)?;
                Ok(handle)
            })?,
        )?;
        t.set(
            "nearby",
            lua.create_function(|lua, (at, radius): (Value, f64)| {
                let (ws, center) = position_arg(lua, &at)?;
                let mut found = with_resource::<ScriptPlacements, _>(lua, |p| {
                    p.loaded
                        .iter()
                        .filter(|(_, x)| x.worldspace == ws)
                        .map(|(h, x)| (distance(x.position, center), *h))
                        .filter(|(d, _)| *d <= radius)
                        .collect::<Vec<_>>()
                })?;
                // Distance, then id: a defined order, never hash order (determinism).
                found.sort_by(|a, b| a.0.total_cmp(&b.0).then(a.1.cmp(&b.1)));
                let out = lua.create_table_with_capacity(found.len(), 0)?;
                for (_, h) in found {
                    out.raw_push(h)?;
                }
                Ok(out)
            })?,
        )?;
        sf_script::install_entity_api(lua, &t)?;
        Ok(t)
    }
    fn types(&self) -> &'static str {
        "type Position = { x: number, y: number, z: number, worldspace: string? }\n\
         declare sf_world: {\n  \
         player: () -> Handle,\n  \
         is_loaded: (h: Handle) -> boolean,\n  \
         record: (h: Handle) -> string?,\n  \
         worldspace: (h: Handle) -> string?,\n  \
         -- World position in meters (nil, nil, nil when not loaded).\n  \
         position: (h: Handle) -> (number?, number?, number?),\n  \
         -- Loaded placements within `radius` meters, nearest first.\n  \
         nearby: (at: Handle | Position, radius: number) -> { Handle },\n  \
         -- A new placement of `record`; the handle is usable at once, the object appears after the slot.\n  \
         spawn: (record: string, at: Handle | Position, yaw: number?) -> Handle,\n  \
         despawn: (h: Handle) -> boolean,\n  \
         -- At context start: a named entity script (handlers: activated(h, payload, state), tick(h, dt, state)).\n  \
         define_script: (name: string, handlers: { [string]: (Handle, any, { [any]: any }) -> any }) -> (),\n  \
         -- Attach a defined script; returns the placement's state table (plain data, saved).\n  \
         attach: (h: Handle, script: string) -> { [any]: any },\n  \
         detach: (h: Handle) -> boolean,\n  \
         state: (h: Handle) -> { [any]: any }?,\n  \
         script_of: (h: Handle) -> string?,\n}\n"
    }
    fn handle_types(&self) -> &'static str {
        "  function is_loaded(self): boolean\n  function record(self): string?\n  function worldspace(self): string?\n  \
         function position(self): (number?, number?, number?)\n  function despawn(self): boolean\n  \
         function attach(self, script: string): { [any]: any }\n  function detach(self): boolean\n  \
         function state(self): { [any]: any }?\n  function script_of(self): string?\n"
    }
}
