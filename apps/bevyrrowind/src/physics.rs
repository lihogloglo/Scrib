//! Rigid-body physics (ROADMAP M4; superfargoth physics.md, physics-adapter.md):
//! `sf_physics_bevy` on the streamed world.
//!
//! - R0 statics, doors and (until P4 makes them sleeping bodies) dynamic
//!   placements become `Fixed` colliders from their `collision.cook` shapes:
//!   [`hook`] wraps their spawners after `exterior.rs` builds them. Terrain
//!   heightfield chunks follow the exterior's R0 cells. Interiors are their own
//!   physics worlds (one per worldspace; only the active one steps).
//! - `--physics-debug` (or `G`) draws collider wireframes around the camera.
//! - `--off physics` leaves physics out. The walkers keep their own
//!   `StaticWorld` either way for now: `PhysicsProbe` replaces it once
//!   walkers.rs moves over (P2 handoff), and it stays as the physics-off path.
//! - Every 10 s the log gets the peaks: step and insert time per frame.

use std::sync::Arc;

use bevy::prelude::*;
use sf_bake::{Manifest, Store};
use sf_controller::steps::COLLISION_COOK;
use sf_controller::{CollisionMesh, StaticShape};
use sf_physics::BodyShape;
use sf_physics::steps::BODY_COOK;
use sf_physics_bevy::{
    PhysicsBodySource, PhysicsDebug, PhysicsPlugin, PhysicsShapeSource, PhysicsSettings, PhysicsStats, hook_physics,
};
use sf_world::PlacementClass;
use sf_world_bevy::PlacementSpawners;

/// Adds physics to the streamed viewer.
pub fn add(app: &mut App, store: Store, manifest: Manifest, debug: bool) {
    // Items take their `body.cook` shape (fixed until P4 makes them dynamic).
    let (body_store, body_manifest) = (store.clone(), manifest.clone());
    let bodies = move |model: &str| -> Option<Arc<BodyShape>> {
        let digest = body_manifest.get(BODY_COOK, model)?;
        BodyShape::from_bytes(&body_store.get(&digest).ok()?).ok().map(Arc::new)
    };
    let source = move |model: &str| -> Option<Arc<StaticShape>> {
        let digest = manifest.get(COLLISION_COOK, model)?;
        let bytes = store.get(&digest).ok()?;
        let mesh = CollisionMesh::from_bytes(&bytes).ok()?;
        StaticShape::new(&mesh).map(Arc::new)
    };
    app.add_plugins(PhysicsPlugin { settings: PhysicsSettings::default(), ..PhysicsPlugin::default() })
        .insert_resource(PhysicsShapeSource(Arc::new(source)))
        .insert_resource(PhysicsBodySource { loader: Arc::new(bodies), classes: vec![PlacementClass::Dynamic] })
        .insert_resource(PhysicsDebug { enabled: debug, ..PhysicsDebug::default() })
        .init_resource::<Peaks>()
        .add_systems(PostStartup, hook)
        .add_systems(Update, (toggle_debug, peaks.after(sf_physics_bevy::PhysicsSystems::WriteBack)));
}

/// Wraps the spawners `exterior.rs` built (Startup) with the physics hook.
fn hook(spawners: Option<ResMut<PlacementSpawners>>) {
    if let Some(mut spawners) = spawners {
        hook_physics(&mut spawners, &[PlacementClass::Static, PlacementClass::Door, PlacementClass::Dynamic]);
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
}

/// physics.md budgets: an idle R0 step ≤ 0.3 ms, insertion ≤ 0.5 ms per frame.
const STEP_BUDGET_MS: f32 = 0.3;
const INSERT_BUDGET_MS: f32 = 0.5;

#[allow(clippy::needless_pass_by_value)]
fn peaks(time: Res<Time>, stats: Res<PhysicsStats>, mut p: ResMut<Peaks>) {
    let s = *stats;
    p.frames += 1;
    p.step_ms = p.step_ms.max(s.max_step_ms);
    p.step_frame_ms = p.step_frame_ms.max(s.step_ms);
    p.insert_ms = p.insert_ms.max(s.insert_ms);
    p.inserts = p.inserts.max(s.inserts);
    p.over_budget += u32::from(s.max_step_ms > STEP_BUDGET_MS || s.insert_ms > INSERT_BUDGET_MS);
    p.dropped += s.dropped_steps;
    p.rebases += s.rebases;
    p.since += time.delta_secs();
    if p.since < 10.0 {
        return;
    }
    info!(
        "physics: {} bodies ({} terrain chunks, {} pending, {} awake) | peaks over {} frames: step {:.3} ms ({:.3} ms/frame), \
         insert {:.3} ms/frame ({} bodies) | {} frames over budget, {} dropped steps, {} rebases",
        s.bodies,
        s.terrain_chunks,
        s.pending,
        s.awake,
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
