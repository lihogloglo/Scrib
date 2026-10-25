//! Mod scripts in the streamed view (ROADMAP M5 L2; superfargoth `docs/specs/scripting.md`
//! "Execution model", "Entity handles"): the profile's script contexts run in
//! `sf_script_bevy`'s slots, placements entering R0 become script handles, and E (the
//! activate key) activates the player's focus (`player::Focus`, a physics ray). Unhandled
//! activations of a door open it (`doors.rs`); scripts' spawns and despawns (`WorldCommand`, M5 L3) are applied by the
//! `LiveSpawn` scaffold below; `core.game_time()` follows the sky clock.
//!
//! Contexts come from the loaded profile (`mods::script_plans`, one per mod with
//! `content.scripts`, in load order), plus the shared `mw.legacy` context owned by the builtin
//! mod `mw` (scripting-adapter.md; W3 starts translated scripts in it, L2 only creates it).

use std::collections::HashMap;
use std::sync::{Arc, PoisonError};
use std::time::Duration;

use bevy_app::{App, PostStartup, Update};
use bevy_asset::{AssetServer, UntypedHandle};
use bevy_camera::visibility::{RenderLayers, Visibility};
use bevy_ecs::prelude::*;
use bevy_ecs::system::SystemState;
use bevy_log::{info, warn};
use bevy_math::{DVec3, IVec3, Quat};
use bevy_time::{Time, Timer, TimerMode};
use bevy_transform::components::Transform;
use mw_controls_bevy::{UseAction, use_pressed};
use mw_game_core::doors::DoorUser;
use mw_game_core::player::{FocusUpdate, PlayerBody, PlayerMode};
use mw_player_bevy::Focus;
use sf_coords::WorldPos;
use sf_script::{ContextConfig, Handle, Limits, ScriptRuntime, ScriptSource};
use sf_script_bevy::{
    Activated, ActivationOutcome, PLAYER, Placed, ScriptFact, ScriptPlacements, ScriptSlot, Scripts, ScriptsPlugin,
    WorldCommand, table,
};
use sf_vfs::Vfs;
use sf_world::{IndexedPlacement, Level, Placement, PlacementClass, PlacementId, WorldspaceId};
use sf_world_bevy::{
    ActivateCx, CellLoaded, LoadedModel, PlacementSpawner, PlacementSpawners, SpawnCx, Streaming, WorldOverlay, WorldPosOf,
    WorldspaceRegistry,
};

/// Id of the shared legacy context (scripting-adapter.md "Instances").
pub const LEGACY_CONTEXT: &str = "mw.legacy";

/// Script context configs for the loaded profile, in load order, then `mw.legacy`.
fn configs() -> Vec<ContextConfig> {
    let mut out: Vec<ContextConfig> = mw_boot_bevy::mods::script_plans()
        .into_iter()
        .map(|p| {
            let mut limits = Limits::MOD;
            if let Some(mib) = p.limits.memory_mib {
                limits.memory_bytes = (mib as usize) << 20;
            }
            if let Some(ms) = p.limits.call_timeout_ms {
                limits.call_timeout = Duration::from_millis(u64::from(ms));
            }
            if let Some(us) = p.limits.frame_soft_us {
                limits.frame_soft = Duration::from_micros(u64::from(us));
            }
            ContextConfig {
                id: p.id.as_str().to_owned(),
                mods: p.mods.iter().map(|m| m.as_str().to_owned()).collect(),
                dependencies: p.dependencies.iter().map(|m| m.as_str().to_owned()).collect(),
                entry_points: p.entry_points.iter().map(|e| e.as_str().to_owned()).collect(),
                api: p.api,
                version: p.version,
                limits,
                seed: 0,
                // Profile `[settings.<id>]` reach scripts with the settings registry (U1/U6).
                settings: sf_script::StateValue::table(),
            }
        })
        .collect();
    out.push(ContextConfig {
        id: LEGACY_CONTEXT.to_owned(),
        entry_points: Vec::new(),
        limits: Limits::SHARED,
        ..ContextConfig::for_mod(mw_mods::builtin_id().as_str())
    });
    out
}

/// `--activate N`: activate the crosshair target N times once placements have streamed in.
#[derive(Resource)]
struct AutoActivate {
    remaining: u32,
    timer: Timer,
}

pub fn add(app: &mut App, vfs: Arc<Vfs>, auto_activate: u32) {
    let mut scripts = Scripts::new(ScriptRuntime::default());
    let configs = configs();
    let ids: Vec<String> = configs.iter().map(|c| c.id.clone()).collect();
    let source: Arc<dyn ScriptSource> = vfs;
    scripts.start(configs, source);
    let stopped: Vec<String> = scripts.stopped().map(|(id, s)| format!("{id} ({s:?})")).collect();
    info!("scripts: {} contexts [{}]{}", ids.len(), ids.join(", "), if stopped.is_empty() { String::new() } else { format!("; not running: {}", stopped.join(", ")) });
    app.insert_resource(scripts)
        .add_plugins(ScriptsPlugin)
        .insert_resource(AutoActivate { remaining: auto_activate, timer: Timer::from_seconds(3.0, TimerMode::Once) })
        .init_resource::<SpawnTemplates>()
        .init_resource::<LiveSpawned>()
        .init_resource::<ActivePlacements>()
        .add_systems(PostStartup, hook_placements)
        .add_systems(Update, (track_player, cell_facts, activate.after(FocusUpdate), game_clock).chain().before(ScriptSlot::Update))
        .add_systems(Update, (report, apply_world_commands).after(ScriptSlot::Update));
}

/// `core.game_time()`: game hours since the sky clock's epoch (24-hour days).
fn game_clock(time: Option<Res<sf_sky_bevy::SkyTime>>, mut scripts: ResMut<Scripts>) {
    scripts.bypass_change_detection().runtime.set_game_time(time.map(|t| t.0.days * 24.0));
}

/// One placement per record id seen entering R0: what a script's `world.spawn(record, ..)`
/// copies (model, class, scale). Part of the `LiveSpawn` scaffold.
#[derive(Resource, Default)]
struct SpawnTemplates(HashMap<String, (Placement, f32)>);

/// Holder entities of live script spawns, by handle.
#[derive(Resource, Default)]
struct LiveSpawned(HashMap<Handle, Entity>);

/// Scaffold `LiveSpawn`: applies scripts' [`WorldCommand`]s. A spawn goes into the placement
/// overlay (so the copy streams in with its cell from then on) and, when its cell is loaded, is
/// spawned live now through the NIF spawner under a holder entity; a despawn removes it from the
/// overlay and drops the live holder. Limits: only records already seen in R0 this session can be
/// spawned (the template gives the model), activation hooks (colliders) don't run for live
/// spawns, and a despawned *baked* placement stays visible until its cell reloads. Removal path:
/// `sf_world_bevy` streaming re-applies overlay changes to loaded cells (a "dirty cell" refetch),
/// then commands only edit the overlay and the real spawners and hooks do the rest.
fn apply_world_commands(world: &mut World, reader: &mut SystemState<MessageReader<'static, 'static, WorldCommand>>) {
    let commands: Vec<WorldCommand> = reader.get_mut(world).map(|mut r| r.read().cloned().collect()).unwrap_or_default();
    for c in commands {
        match c {
            WorldCommand::Spawn { handle, record, worldspace, position, yaw, context } => {
                live_spawn(world, handle, &record, &WorldspaceId::new(&worldspace), position, yaw, &context);
            }
            WorldCommand::Despawn { handle, context } => {
                if let Some(o) = world.get_resource::<WorldOverlay>() {
                    o.0.write().unwrap_or_else(PoisonError::into_inner).remove(PlacementId(handle.0));
                }
                let holder = world.resource_mut::<LiveSpawned>().0.remove(&handle);
                // The holder is gone already if its cell unloaded meanwhile.
                if let Some(Ok(e)) = holder.map(|h| world.get_entity_mut(h)) {
                    e.despawn();
                }
                info!("script {context}: despawned {:032x}{}", handle.0, if holder.is_some() { "" } else { " (overlay only)" });
            }
        }
    }
}

fn live_spawn(world: &mut World, handle: Handle, record: &str, ws: &WorldspaceId, position: [f64; 3], yaw: f64, context: &str) {
    let Some((template, radius)) = world.resource::<SpawnTemplates>().0.get(record).cloned() else {
        warn!("script {context}: spawn {record}: no placement of it seen yet (LiveSpawn scaffold copies a loaded one)");
        return;
    };
    let mut placement = template.clone();
    placement.id = PlacementId(handle.0);
    placement.transform.pos = WorldPos::from_f64(DVec3::from_array(position));
    #[allow(clippy::cast_possible_truncation, reason = "an angle")]
    let turn = Quat::from_rotation_y(yaw as f32);
    placement.transform.rotation = turn * template.transform.rotation;
    if let Some(o) = world.get_resource::<WorldOverlay>() {
        o.0.write().unwrap_or_else(PoisonError::into_inner).place(ws.clone(), placement.clone());
    }
    let Some(streaming) = world.get_resource::<Streaming>() else { return };
    let Some(partition) = streaming.scheduler().directory(ws).map(|d| d.partition) else { return };
    let cell = partition.cell_of(placement.transform.pos);
    let Some(cell_entity) = streaming.cell_entity(ws, cell) else {
        info!("script {context}: spawned {record} into an unloaded cell (appears when it streams in)");
        return;
    };
    let lod = streaming.config().lod;
    let layers = world.resource::<WorldspaceRegistry>().get(ws).map_or_else(RenderLayers::default, |e| RenderLayers::layer(e.layer));
    let Some(spawner) = world.get_resource::<PlacementSpawners>().and_then(|s| s.uncooked().cloned()) else { return };
    let holder = world.spawn((Transform::IDENTITY, Visibility::Inherited, ChildOf(cell_entity))).id();
    let t = &placement.transform;
    let cx = SpawnCx {
        worldspace: ws,
        cell,
        level: Level::Near,
        parent: holder,
        layers: &layers,
        local: Transform { translation: partition.cell_origin(cell).delta_f32(t.pos), rotation: t.rotation, scale: t.scale },
        model: None,
        max_distance: 1000.0,
        end_band: 0.0,
        lod: &lod,
    };
    let entities = spawner.spawn(world, &cx, &IndexedPlacement { placement, radius });
    world.resource_mut::<LiveSpawned>().0.insert(handle, holder);
    if let Some(p) = world.resource_mut::<ScriptPlacements>().loaded.get_mut(&handle) {
        p.radius = radius;
    }
    info!("script {context}: spawned {record} ({entities} entities) at {position:.2?}, handle {:032x}", handle.0);
}

/// Wraps the placement spawners so R0 activation registers the placement as a script handle.
fn hook_placements(spawners: Option<ResMut<PlacementSpawners>>) {
    let Some(mut spawners) = spawners else { return };
    let classes = [PlacementClass::Static, PlacementClass::Dynamic, PlacementClass::Door, PlacementClass::Light, PlacementClass::Actor, mw_world::source::CONTAINER];
    for class in classes {
        if let Some(inner) = spawners.for_class(class).cloned() {
            spawners.set(class, Arc::new(ScriptHandleHook(inner)));
        }
    }
}

/// An R0 placement that can leave the world through gameplay (an item taken): what its
/// spawner chain needs to deactivate it early (`inventory.rs`).
#[derive(Clone)]
pub struct ActivePlacement {
    pub placement: IndexedPlacement,
    pub worldspace: WorldspaceId,
    pub cell: IVec3,
    pub cell_entity: Entity,
}

/// R0 items and lights (the classes the player can take), by placement id.
#[derive(Resource, Default)]
pub struct ActivePlacements(pub HashMap<PlacementId, ActivePlacement>);

impl ActivePlacements {
    /// Runs `placement`'s deactivation hooks now (physics body, script handle, sounds), as if
    /// its cell had left R0, and forgets it. The caller removes it from the overlay and
    /// despawns its visuals.
    pub fn deactivate(world: &mut World, id: PlacementId) -> Option<ActivePlacement> {
        let active = world.get_resource_mut::<ActivePlacements>()?.0.remove(&id)?;
        let spawner = world.get_resource::<PlacementSpawners>().and_then(|s| s.for_class(active.placement.placement.class).cloned());
        if let Some(spawner) = spawner {
            let cx = ActivateCx { worldspace: &active.worldspace, cell: active.cell, cell_entity: active.cell_entity, discard: false };
            spawner.deactivate(world, &cx, &active.placement);
        }
        Some(active)
    }
}

/// Registers placements entering R0 in [`ScriptPlacements`] (items and lights also in
/// [`ActivePlacements`]) and removes them on leaving.
struct ScriptHandleHook(Arc<dyn PlacementSpawner>);

impl PlacementSpawner for ScriptHandleHook {
    fn spawn(&self, world: &mut World, cx: &SpawnCx<'_>, placement: &IndexedPlacement) -> u32 {
        self.0.spawn(world, cx, placement)
    }
    fn needs_model(&self, placement: &IndexedPlacement) -> bool {
        self.0.needs_model(placement)
    }
    fn prepare(&self, assets: &AssetServer, placement: &IndexedPlacement, model: Option<&LoadedModel>) -> Option<UntypedHandle> {
        self.0.prepare(assets, placement, model)
    }
    fn activate(&self, world: &mut World, cx: &ActivateCx<'_>, placement: &IndexedPlacement) {
        self.0.activate(world, cx, placement);
        let p = &placement.placement;
        if let Some(mut templates) = world.get_resource_mut::<SpawnTemplates>() {
            templates.0.entry(p.base.id.as_str().to_owned()).or_insert_with(|| (p.clone(), placement.radius));
        }
        if let Some(mut placements) = world.get_resource_mut::<ScriptPlacements>() {
            let pos = p.transform.pos.to_f64();
            placements.loaded.insert(
                Handle(p.id.0),
                Placed {
                    record: p.base.id.as_str().to_owned(),
                    worldspace: cx.worldspace.as_str().to_owned(),
                    position: [pos.x, pos.y, pos.z],
                    radius: placement.radius,
                },
            );
        }
        if matches!(p.class, PlacementClass::Dynamic | PlacementClass::Light)
            && let Some(mut active) = world.get_resource_mut::<ActivePlacements>()
        {
            let entry = ActivePlacement { placement: placement.clone(), worldspace: cx.worldspace.clone(), cell: cx.cell, cell_entity: cx.cell_entity };
            active.0.insert(p.id, entry);
        }
    }
    fn deactivate(&self, world: &mut World, cx: &ActivateCx<'_>, placement: &IndexedPlacement) {
        self.0.deactivate(world, cx, placement);
        if let Some(mut placements) = world.get_resource_mut::<ScriptPlacements>() {
            placements.loaded.remove(&Handle(placement.placement.id.0));
        }
        if let Some(mut active) = world.get_resource_mut::<ActivePlacements>() {
            active.0.remove(&placement.placement.id);
        }
    }
}

/// The player's entry: the player character (`player.rs`), else the activating camera (free
/// camera mode).
fn track_player(
    cams: Query<Entity, With<DoorUser>>,
    bodies: Query<Entity, With<PlayerBody>>,
    positions: WorldPosOf,
    mut placements: ResMut<ScriptPlacements>,
) {
    let who = bodies.single().or_else(|_| cams.single()).ok();
    let Some(here) = who.and_then(|e| positions.location(e)) else { return };
    let pos = here.pos.to_f64();
    placements.player =
        Some(Placed { record: "player".into(), worldspace: here.worldspace.as_str().to_owned(), position: [pos.x, pos.y, pos.z], radius: 0.0 });
}

/// `CellLoaded` → `"cell_loaded"` `{ worldspace, x, y, z, level }` (streaming cell) in the `Update` slot.
fn cell_facts(mut loaded: MessageReader<CellLoaded>, mut facts: MessageWriter<ScriptFact>) {
    use sf_script::StateValue::{Number, Str};
    for c in loaded.read() {
        facts.write(ScriptFact {
            slot: ScriptSlot::Update,
            name: "cell_loaded".into(),
            payload: table([
                ("worldspace", Str(c.worldspace.as_str().as_bytes().to_vec())),
                ("x", Number(f64::from(c.cell.x))),
                ("y", Number(f64::from(c.cell.y))),
                ("z", Number(f64::from(c.cell.z))),
                ("level", Str(format!("{:?}", c.level).into_bytes())),
            ]),
        });
    }
}

/// E activates the player's focus (`player::Focus`: `OpenMW`'s camera ray within
/// `iMaxActivateDist`, on the physics world, `player.rs`); `--activate N` activates it
/// automatically, once a second. Nothing activates while the player is paused (a menu or
/// dialogue is open), so a door can't travel under the journal.
#[allow(clippy::needless_pass_by_value)] // system parameters
fn activate(
    uses: Query<&bevy_enhanced_input::prelude::ActionEvents, With<bevy_enhanced_input::prelude::Action<UseAction>>>,
    time: Res<Time>,
    mut auto: ResMut<AutoActivate>,
    focus: Res<Focus>,
    mode: Res<PlayerMode>,
    placements: Res<ScriptPlacements>,
    mut out: MessageWriter<Activated>,
) {
    if mode.paused {
        return;
    }
    let mut automatic = false;
    if auto.remaining > 0 && !placements.loaded.is_empty() {
        auto.timer.tick(time.delta());
        if auto.timer.is_finished() {
            automatic = true;
            auto.remaining -= 1;
            auto.timer = Timer::from_seconds(1.0, TimerMode::Once);
        }
    }
    if !automatic && !use_pressed(&uses) {
        return;
    }
    if let Some(f) = &focus.target {
        info!("activate: {} ({}, {:.1} m, handle {:032x})", f.name, f.record, f.distance, f.handle.0);
        out.write(Activated { target: f.handle, actor: PLAYER });
        return;
    }
    let (true, Some(player)) = (automatic, placements.player.as_ref()) else { return };
    // Name the nearest placement, for a `--camera` that faces it.
    let d = |p: &Placed| (0..3).map(|i| (p.position[i] - player.position[i]).powi(2)).sum::<f64>().sqrt();
    let nearest = placements
        .loaded
        .values()
        .filter(|p| p.worldspace == player.worldspace && p.radius > 0.0)
        .min_by(|a, b| d(a).total_cmp(&d(b)));
    warn!(
        "--activate: nothing activatable under the crosshair at {:.1?} ({} loaded; nearest: {:?})",
        player.position,
        placements.loaded.len(),
        nearest.map(|p| (&p.record, d(p), p.position))
    );
}

/// Logs each activation's outcome (dev: scripts' `"handled"` replaces the default action).
fn report(mut outcomes: MessageReader<ActivationOutcome>, placements: Res<ScriptPlacements>) {
    for o in outcomes.read() {
        let record = placements.loaded.get(&o.activation.target).map_or("?", |p| p.record.as_str());
        info!("activation of {record}: {}", if o.handled { "handled by a script" } else { "default action" });
    }
}
