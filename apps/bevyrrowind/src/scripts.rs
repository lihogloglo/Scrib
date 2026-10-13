//! Mod scripts in the streamed view (ROADMAP M5 L2; superfargoth `docs/specs/scripting.md`
//! "Execution model", "Entity handles"): the profile's script contexts run in
//! `sf_script_bevy`'s slots, placements entering R0 become script handles, and Space (the
//! activate key) activates the placement under the crosshair.
//!
//! Contexts come from the loaded profile (`mods::script_plans`, one per mod with
//! `content.scripts`, in load order), plus the shared `mw.legacy` context owned by the builtin
//! mod `mw` (scripting-adapter.md; W3 starts translated scripts in it, L2 only creates it).

use std::sync::Arc;
use std::time::Duration;

use bevy::prelude::*;
use sf_script::{ContextConfig, Handle, Limits, ScriptRuntime, ScriptSource};
use sf_script_bevy::{
    Activated, ActivationOutcome, PLAYER, Placed, ScriptFact, ScriptPlacements, ScriptSlot, Scripts, ScriptsPlugin,
    table,
};
use sf_vfs::Vfs;
use sf_world::{IndexedPlacement, PlacementClass};
use sf_world_bevy::{ActivateCx, CellLoaded, LoadedModel, PlacementSpawner, PlacementSpawners, SpawnCx, WorldPosOf};

/// Id of the shared legacy context (scripting-adapter.md "Instances").
pub const LEGACY_CONTEXT: &str = "mw.legacy";

/// How far the crosshair reaches, meters (as door activation, `doors::REACH_M`).
const REACH_M: f32 = 4.0;
/// Bounding spheres the eye is inside of and larger than this are skipped (a building around
/// the camera is not what it looks at).
const ENCLOSING_M: f32 = 3.0;
/// Smallest pick sphere, meters (tiny items stay clickable).
const MIN_PICK_M: f32 = 0.25;

/// Script context configs for the loaded profile, in load order, then `mw.legacy`.
fn configs() -> Vec<ContextConfig> {
    let mut out: Vec<ContextConfig> = crate::mods::script_plans()
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
        .add_systems(PostStartup, hook_placements)
        .add_systems(Update, (track_player, cell_facts, activate).chain().before(ScriptSlot::Update))
        .add_systems(Update, report.after(ScriptSlot::Update));
}

/// Wraps the placement spawners so R0 activation registers the placement as a script handle.
fn hook_placements(spawners: Option<ResMut<PlacementSpawners>>) {
    let Some(mut spawners) = spawners else { return };
    for class in [PlacementClass::Static, PlacementClass::Dynamic, PlacementClass::Door, PlacementClass::Light, PlacementClass::Actor] {
        if let Some(inner) = spawners.for_class(class).cloned() {
            spawners.set(class, Arc::new(ScriptHandleHook(inner)));
        }
    }
}

/// Registers placements entering R0 in [`ScriptPlacements`] and removes them on leaving.
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
    }
    fn deactivate(&self, world: &mut World, cx: &ActivateCx<'_>, placement: &IndexedPlacement) {
        self.0.deactivate(world, cx, placement);
        if let Some(mut placements) = world.get_resource_mut::<ScriptPlacements>() {
            placements.loaded.remove(&Handle(placement.placement.id.0));
        }
    }
}

/// The player's entry (the activating camera stands in for the player until M7).
fn track_player(
    cams: Query<Entity, With<crate::doors::DoorUser>>,
    positions: WorldPosOf,
    mut placements: ResMut<ScriptPlacements>,
) {
    let Some(here) = cams.single().ok().and_then(|cam| positions.location(cam)) else { return };
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

/// Scaffold `CrosshairPick`: the loaded placement whose bounding sphere the view ray enters
/// first within reach. Removal path: M4 P11's `sf_interaction` picks with a physics ray cast
/// (collider → placement) and owns the activate key; this module then only maps its
/// `Activated` to scripts.
fn pick(placements: &ScriptPlacements, worldspace: &str, eye: [f64; 3], forward: Vec3) -> Option<(Handle, f32)> {
    let mut best: Option<(f32, f32, Handle)> = None;
    for (h, p) in &placements.loaded {
        if p.worldspace != worldspace {
            continue;
        }
        #[allow(clippy::cast_possible_truncation, reason = "nearby offsets")]
        let c = Vec3::new((p.position[0] - eye[0]) as f32, (p.position[1] - eye[1]) as f32, (p.position[2] - eye[2]) as f32);
        let r = p.radius.max(MIN_PICK_M);
        if c.length() > REACH_M + r {
            continue;
        }
        let along = c.dot(forward);
        let d2 = c.length_squared() - along * along;
        if d2 > r * r {
            continue;
        }
        let mut t = along - (r * r - d2).sqrt();
        if t < 0.0 {
            if r > ENCLOSING_M || along < 0.0 {
                continue;
            }
            t = 0.0;
        }
        if t > REACH_M {
            continue;
        }
        if best.is_none_or(|(bt, br, _)| (t, r) < (bt, br)) {
            best = Some((t, r, *h));
        }
    }
    best.map(|(t, _, h)| (h, t))
}

/// Space activates the crosshair target (`--activate N`: automatically, once a second).
fn activate(
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut auto: ResMut<AutoActivate>,
    cams: Query<&GlobalTransform, With<crate::doors::DoorUser>>,
    placements: Res<ScriptPlacements>,
    mut out: MessageWriter<Activated>,
) {
    let mut automatic = false;
    if auto.remaining > 0 && !placements.loaded.is_empty() {
        auto.timer.tick(time.delta());
        if auto.timer.is_finished() {
            automatic = true;
            auto.remaining -= 1;
            auto.timer = Timer::from_seconds(1.0, TimerMode::Once);
        }
    }
    if !automatic && !keys.just_pressed(KeyCode::Space) {
        return;
    }
    let (Ok(global), Some(player)) = (cams.single(), placements.player.as_ref()) else { return };
    match pick(&placements, &player.worldspace, player.position, global.forward().as_vec3()) {
        Some((target, dist)) => {
            let record = placements.loaded.get(&target).map_or("?", |p| p.record.as_str());
            info!("activate: {record} ({dist:.1} m, handle {:032x})", target.0);
            out.write(Activated { target, actor: PLAYER });
        }
        None if automatic => {
            // Name the nearest placement, for a `--camera` that faces it.
            let d = |p: &Placed| (0..3).map(|i| (p.position[i] - player.position[i]).powi(2)).sum::<f64>().sqrt();
            let nearest = placements
                .loaded
                .values()
                .filter(|p| p.worldspace == player.worldspace && p.radius > 0.0)
                .min_by(|a, b| d(a).total_cmp(&d(b)));
            warn!(
                "--activate: nothing under the crosshair within {REACH_M} m of {:.1?} ({} loaded; nearest: {:?})",
                player.position,
                placements.loaded.len(),
                nearest.map(|p| (&p.record, d(p), p.position))
            );
        }
        None => {}
    }
}

/// Logs each activation's outcome (dev: scripts' `"handled"` replaces the default action).
fn report(mut outcomes: MessageReader<ActivationOutcome>, placements: Res<ScriptPlacements>) {
    for o in outcomes.read() {
        let record = placements.loaded.get(&o.activation.target).map_or("?", |p| p.record.as_str());
        info!("activation of {record}: {}", if o.handled { "handled by a script" } else { "default action" });
    }
}
