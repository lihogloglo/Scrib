//! Rigid-body physics (ROADMAP M4; superfargoth physics.md, physics-adapter.md):
//! `sf_physics_bevy` on the streamed world.
//!
//! - R0 statics and doors become `Fixed` colliders from their `collision.cook`
//!   shapes: [`hook`] wraps their spawners after `exterior.rs` builds them.
//!   Terrain heightfield chunks follow the exterior's R0 cells. Interiors are
//!   their own physics worlds (one per worldspace; only the active one steps).
//! - Items (P4; [`MwItems`]): `Dynamic` placements and carryable lights get
//!   their `body.cook` shape as sleeping dynamic bodies, mass from the record
//!   weight (`mw_physics`), held `Fixed` where `items.check` found them
//!   unsupported (and carryable lights always). Moved items persist through
//!   the `PlacementOverlay` when their cell unloads. Statics whose authored
//!   collision is a box around their shelves (`items.check` `supports`)
//!   simulate with their `support.cook` render mesh, so the items rest on
//!   the shelves instead of starting inside the box.
//! - `V` throws a 3 kg ball from the camera (PH-1 without a player
//!   controller); the 8 newest stay.
//! - `--physics-debug` (or `G`) draws collider wireframes around the camera.
//! - `--off physics` leaves physics out. The walkers keep their own
//!   `StaticWorld` either way for now: `PhysicsProbe` replaces it once
//!   walkers.rs moves over (P2 handoff), and it stays as the physics-off path.
//! - Every 10 s the log gets the peaks: step and insert time per frame.

use std::collections::{HashMap, HashSet, VecDeque};
use std::sync::{Arc, OnceLock};

use bevy::camera::visibility::RenderLayers;
use bevy::prelude::*;
use mw_physics::ItemRecords;
use sf_bake::{Manifest, Store};
use sf_controller::steps::COLLISION_COOK;
use sf_controller::{CollisionMesh, StaticShape};
use sf_coords::WorldTransform;
use sf_physics::steps::{BODY_COOK, Hold, ITEMS_CHECK, ItemChecks, SUPPORT_COOK};
use sf_physics::{BodyDesc, BodyKind, BodyShape, Layer, ShapeRef};
use sf_physics_bevy::{
    ItemBody, ItemContent, Physics, PhysicsBody, PhysicsBodySource, PhysicsDebug, PhysicsPlugin, PhysicsShapeSource,
    PhysicsSettings, PhysicsStats, PhysicsStreaming, hook_items, hook_physics,
};
use sf_records::RecordDb;
use sf_world::{Placement, PlacementClass, WorldspaceId};
use sf_world_bevy::big_space::prelude::CellCoord;
use sf_world_bevy::{ActiveWorldspace, PlacementSpawners, from_grid, transform_to_grid};

/// Adds physics to the streamed viewer. `db` gives item weights and the
/// carryable lights (without it, items weigh their volume × 400 kg/m³).
pub fn add(app: &mut App, store: Store, manifest: Manifest, debug: bool, db: Option<Arc<RecordDb>>) {
    let (body_store, body_manifest) = (store.clone(), manifest.clone());
    let bodies = move |model: &str| -> Option<Arc<BodyShape>> {
        let digest = body_manifest.get(BODY_COOK, model)?;
        BodyShape::from_bytes(&body_store.get(&digest).ok()?).ok().map(Arc::new)
    };
    let items = MwItems::new(db.as_deref(), store.clone(), manifest.clone());
    let (support_store, support_manifest) = (store.clone(), manifest.clone());
    let supports = move |model: &str| -> Option<Arc<StaticShape>> {
        let mesh = CollisionMesh::from_bytes(&support_store.get(&support_manifest.get(SUPPORT_COOK, model)?).ok()?).ok()?;
        StaticShape::new(&mesh).map(Arc::new)
    };
    let source = move |model: &str| -> Option<Arc<StaticShape>> {
        let digest = manifest.get(COLLISION_COOK, model)?;
        let bytes = store.get(&digest).ok()?;
        let mesh = CollisionMesh::from_bytes(&bytes).ok()?;
        StaticShape::new(&mesh).map(Arc::new)
    };
    app.add_plugins(PhysicsPlugin { settings: PhysicsSettings::default(), ..PhysicsPlugin::default() })
        .insert_resource(PhysicsShapeSource(Arc::new(source)))
        .insert_resource(PhysicsBodySource { loader: Arc::new(bodies), items: Arc::new(items), supports: Some(Arc::new(supports)) })
        .insert_resource(PhysicsDebug { enabled: debug, ..PhysicsDebug::default() })
        .init_resource::<Peaks>()
        .init_resource::<Balls>()
        .add_systems(PostStartup, hook)
        .add_systems(Update, (toggle_debug, throw_ball, peaks.after(sf_physics_bevy::PhysicsSystems::WriteBack)));
}

/// The Morrowind item rules for the runtime (physics-adapter.md "Items").
struct MwItems {
    records: ItemRecords,
    /// `items.check` holds by placement id (held items only), read from the
    /// bake on a thread at startup; the first activation waits for it.
    holds: Arc<OnceLock<ItemsCheck>>,
}

/// What the runtime reads from every `items.check` product.
#[derive(Default)]
struct ItemsCheck {
    /// Held items by placement id.
    holds: HashMap<u128, Hold>,
    /// Statics simulated with their `support.cook` mesh.
    supports: HashSet<u128>,
}

impl MwItems {
    fn new(db: Option<&RecordDb>, store: Store, manifest: Manifest) -> Self {
        let holds = Arc::new(OnceLock::new());
        let fill = holds.clone();
        std::thread::spawn(move || {
            let mut read = ItemsCheck::default();
            for key in manifest.products.keys().filter(|k| k.step == ITEMS_CHECK) {
                let checks = manifest.get(&key.step, &key.subject).and_then(|d| store.get(&d).ok());
                let Some(Ok(checks)) = checks.map(|b| ItemChecks::from_bytes(&b)) else { continue };
                read.holds.extend(checks.items.iter().filter(|c| c.hold != Hold::No).map(|c| (c.id, c.hold)));
                read.supports.extend(checks.supports.iter().copied());
            }
            info!(
                "physics: items.check holds {} items until grabbed; {} statics with support meshes",
                read.holds.len(),
                read.supports.len()
            );
            let _ = fill.set(read);
        });
        Self { records: db.map(ItemRecords::new).unwrap_or_default(), holds }
    }
}

impl ItemContent for MwItems {
    fn is_item(&self, placement: &Placement) -> bool {
        self.records.is_item(placement)
    }

    fn item_body(&self, _: &WorldspaceId, placement: &Placement, volume: f32) -> ItemBody {
        let held = mw_physics::always_hold(placement) || self.holds.wait().holds.contains_key(&placement.id.0);
        ItemBody { mass: Some(self.records.mass(placement, volume)), hold: held }
    }

    fn is_support(&self, _: &WorldspaceId, placement: &Placement) -> bool {
        self.holds.wait().supports.contains(&placement.id.0)
    }
}

/// Wraps the spawners `exterior.rs` built (Startup) with the physics hook:
/// statics, doors and items; among the lights, the carryable ones.
fn hook(spawners: Option<ResMut<PlacementSpawners>>) {
    if let Some(mut spawners) = spawners {
        hook_physics(&mut spawners, &[PlacementClass::Static, PlacementClass::Door, PlacementClass::Dynamic]);
        hook_items(&mut spawners, &[PlacementClass::Light]);
    }
}

/// Debug balls thrown with `V`, oldest first.
#[derive(Resource, Default)]
struct Balls {
    live: VecDeque<Entity>,
    look: Option<(Handle<Mesh>, Handle<StandardMaterial>)>,
}

const BALL_RADIUS: f32 = 0.12;
const BALL_MASS: f32 = 3.0;
const BALL_SPEED: f32 = 12.0;
const BALLS_KEPT: usize = 8;

/// `V`: a ball from the camera along the view, a PH-1 push without a player
/// controller (physics-adapter.md test scenes).
#[allow(clippy::needless_pass_by_value, clippy::too_many_arguments)]
fn throw_ball(
    keys: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
    camera: Query<(&CellCoord, &Transform, &ChildOf, Option<&RenderLayers>), With<Camera3d>>,
    active: Res<ActiveWorldspace>,
    mut physics: ResMut<Physics>,
    mut balls: ResMut<Balls>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    if !keys.just_pressed(KeyCode::KeyV) {
        return;
    }
    let (Some((cell, t, parent, layers)), Some(ws)) = (camera.iter().next(), active.0.clone()) else { return };
    let eye = from_grid(cell, t);
    let forward = t.forward().as_vec3();
    let pose = WorldTransform { pos: eye.offset(forward * 0.6), ..WorldTransform::IDENTITY };
    let ball = BodyDesc::new(BodyKind::Dynamic { asleep: false }, pose, ShapeRef::Sphere(BALL_RADIUS), Layer::PROP);
    let id = physics.insert(&ws, &BodyDesc { mass: Some(BALL_MASS), ccd: true, ..ball });
    if let Some(w) = physics.world_mut(&ws) {
        w.set_velocity(id, forward * BALL_SPEED, Vec3::ZERO);
    }
    let (mesh, material) = balls
        .look
        .get_or_insert_with(|| {
            let material = StandardMaterial { base_color: Color::srgb(0.9, 0.25, 0.1), perceptual_roughness: 0.6, ..default() };
            (meshes.add(Sphere::new(BALL_RADIUS)), materials.add(material))
        })
        .clone();
    let (grid, local) = transform_to_grid(&pose);
    let body = PhysicsBody { worldspace: ws, id };
    let mut entity = commands.spawn((Mesh3d(mesh), MeshMaterial3d(material), grid, local, ChildOf(parent.parent()), body));
    if let Some(layers) = layers {
        entity.insert(layers.clone());
    }
    balls.live.push_back(entity.id());
    while balls.live.len() > BALLS_KEPT {
        if let Some(old) = balls.live.pop_front() {
            commands.entity(old).despawn();
        }
    }
}

#[allow(clippy::needless_pass_by_value)]
fn toggle_debug(keys: Res<ButtonInput<KeyCode>>, mut draw: ResMut<PhysicsDebug>) {
    if keys.just_pressed(KeyCode::KeyG) {
        draw.enabled = !draw.enabled;
        let on = draw.enabled;
        info!("physics colliders {}", if on { "on" } else { "off" });
    }
}

/// Worst frames since the last report.
#[derive(Resource, Default)]
struct Peaks {
    since: f32,
    frames: u32,
    step_ms: f32,
    step_frame_ms: f32,
    insert_ms: f32,
    inserts: u32,
    over_budget: u32,
    dropped: u32,
    rebases: u32,
    /// Most bodies awake at once (PH-7: 0 with nobody near the items).
    awake: usize,
}

/// physics.md budgets: an idle R0 step ≤ 0.3 ms, insertion ≤ 0.5 ms per frame.
const STEP_BUDGET_MS: f32 = 0.3;
const INSERT_BUDGET_MS: f32 = 0.5;

#[allow(clippy::needless_pass_by_value)]
fn peaks(time: Res<Time>, stats: Res<PhysicsStats>, mut p: ResMut<Peaks>, physics: Res<Physics>, streaming: Res<PhysicsStreaming>) {
    if p.since + time.delta_secs() >= 10.0 {
        // Items that never sleep (PH-7: none with nobody near).
        for (id, start, moved, v, w) in streaming.awake_items(&physics).into_iter().take(6) {
            let at = start.pos.to_f64();
            info!(
                "physics: awake item {:032x} placed at {:.2} {:.2} {:.2}: moved {moved:.3} m, {v:.3} m/s, {w:.2} rad/s",
                id.0, at.x, at.y, at.z
            );
        }
    }
    let s = *stats;
    p.frames += 1;
    p.step_ms = p.step_ms.max(s.max_step_ms);
    p.step_frame_ms = p.step_frame_ms.max(s.step_ms);
    p.insert_ms = p.insert_ms.max(s.insert_ms);
    p.inserts = p.inserts.max(s.inserts);
    p.over_budget += u32::from(s.max_step_ms > STEP_BUDGET_MS || s.insert_ms > INSERT_BUDGET_MS);
    p.dropped += s.dropped_steps;
    p.rebases += s.rebases;
    p.awake = p.awake.max(s.awake);
    p.since += time.delta_secs();
    if p.since < 10.0 {
        return;
    }
    info!(
        "physics: {} bodies ({} items, {} terrain chunks, {} pending, {} awake, peak {}) | peaks over {} frames: step {:.3} ms \
         ({:.3} ms/frame), insert {:.3} ms/frame ({} bodies) | {} frames over budget, {} dropped steps, {} rebases",
        s.bodies,
        s.items,
        s.terrain_chunks,
        s.pending,
        s.awake,
        p.awake,
        p.frames,
        p.step_ms,
        p.step_frame_ms,
        p.insert_ms,
        p.inserts,
        p.over_budget,
        p.dropped,
        p.rebases
    );
    *p = Peaks::default();
}
