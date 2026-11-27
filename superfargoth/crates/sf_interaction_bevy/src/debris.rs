//! Cosmetic break debris (`docs/specs/interaction-destruction.md` "Shell and game", slice `IN1c`;
//! `physics` feature). Every client (host included) runs it on [`Destroyed`]; never gameplay, never
//! saved. The pieces are `sf_physics` bodies; the host gives them a mesh (the framework never loads
//! a model) and a place in its render grid on `Added<DebrisPiece>`.
//!
//! - **Gate**: `debris_lod(distance from PhysicsFocus to the hit, interact.debris.lod_m)`; no focus,
//!   no active physics world or an object that left the registry: no bodies (the stage swap and the
//!   effect are the host's either way).
//! - **Pieces**: for each of the crossed stage's `debris` records, the host's [`DebrisSets`] entry;
//!   each piece is placed at `object * local`, inserted awake on `Layer::DEBRIS` with
//!   `ITEM_DAMPING`, and launched away from the hit along `dir` at `interact.debris.speed_mps`, its
//!   direction, speed and spin jittered by `interact.debris.spread` from a stream seeded by the
//!   break's seed and the piece's rank, so every client spawns the same set.
//! - **Cap**: at most `interact.debris.awake_max` pieces not fading; past it the oldest start fading
//!   (the newest break is where the player looks); a tick's breaks past the cap keep their newest
//!   pieces. A break is never refused: the cap only trims its pieces.
//! - **Fade**: a piece starts fading when its body sleeps or `interact.debris.fade_s` after it
//!   spawned, whichever first, then scales to zero over `interact.debris.shrink_s` and despawns with
//!   its body. A piece whose body is gone (its world dropped) despawns at once.
//!
//! Order: `FixedUpdate`, [`DebrisSystems::Fade`] then [`DebrisSystems::Spawn`], after
//! `DestroySystems::Release` and `PhysicsSystems::Step` (sleep is read after the step; new bodies
//! first move in the next one), only when the app has `Physics` and `PhysicsFocus`. The timing
//! runs on the fixed timestep, as the bodies do.

use std::collections::BTreeMap;
use std::sync::Arc;

use bevy_ecs::prelude::*;
use bevy_time::{Fixed, Time};
use bevy_transform::components::Transform;
use glam::{Quat, Vec3};
use sf_coords::{WorldPos, WorldTransform};
use sf_interaction::{Frame, debris_lod, splitmix64};
use sf_physics_bevy::sf_physics::{BodyDesc, BodyKind, ITEM_DAMPING, Layer, ShapeRef};
use sf_physics_bevy::{Physics, PhysicsBody, PhysicsFocus, PhysicsSystems};
use sf_records::RecordId;
use sf_world::PlacementId;
use sf_world_bevy::transform_to_grid;

use crate::{DestroySystems, Destroyed, Destructibles, InteractionSettings};

/// One piece of a debris set, in the broken object's frame.
#[derive(Clone)]
pub struct DebrisPieceDef {
    pub shape: ShapeRef,
    /// The piece's frame relative to the object's.
    pub local: Frame,
    /// Simulated mass, kg (clamped by the physics params).
    pub mass: f32,
}

/// The host's debris sets: debris record -> its pieces (a per-material generic set or a model's
/// own pre-cut chunks).
#[derive(Resource, Clone, Default)]
pub struct DebrisSets(BTreeMap<RecordId, Arc<[DebrisPieceDef]>>);

impl DebrisSets {
    pub fn insert(&mut self, set: RecordId, pieces: impl Into<Arc<[DebrisPieceDef]>>) {
        self.0.insert(set, pieces.into());
    }

    #[must_use]
    pub fn get(&self, set: &RecordId) -> Option<&[DebrisPieceDef]> {
        self.0.get(set).map(|p| &**p)
    }
}

/// A live debris piece (with its `PhysicsBody`, `CellCoord` and `Transform`).
#[derive(Component, Clone, Debug)]
pub struct DebrisPiece {
    pub set: RecordId,
    /// Index into the set's pieces.
    pub piece: u16,
    /// The broken object.
    pub placement: PlacementId,
    /// Spawn rank on this client: the cap fades the lowest first.
    pub order: u64,
    /// Time since it spawned, s.
    pub age_s: f32,
}

/// A piece scaling to zero before it despawns.
#[derive(Component, Clone, Copy, Debug)]
pub struct DebrisFading {
    /// Shrink time left, s.
    pub left_s: f32,
    /// The whole shrink, s.
    pub total_s: f32,
    /// The scale when fading started.
    pub from: Vec3,
}

/// Debris systems, `FixedUpdate` (module docs).
#[derive(SystemSet, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum DebrisSystems {
    /// Ages pieces, starts fades (sleep, `fade_s`), shrinks and despawns.
    Fade,
    /// The tick's [`Destroyed`] into pieces, then the cap.
    Spawn,
}

/// Next spawn rank.
#[derive(Resource, Default)]
struct DebrisOrder(u64);

pub(crate) fn build(app: &mut bevy_app::App) {
    use bevy_app::FixedUpdate;
    app.init_resource::<DebrisSets>()
        .init_resource::<DebrisOrder>()
        .configure_sets(
            FixedUpdate,
            (DebrisSystems::Fade, DebrisSystems::Spawn)
                .chain()
                .after(DestroySystems::Release)
                .after(PhysicsSystems::Step)
                // An app without physics (tests, tools) runs the shell without debris.
                .run_if(resource_exists::<Physics>)
                .run_if(resource_exists::<PhysicsFocus>),
        )
        .add_systems(FixedUpdate, (fade.in_set(DebrisSystems::Fade), spawn.in_set(DebrisSystems::Spawn)));
}

/// A uniform `f32` in `[-1, 1)` from the stream, advancing it.
#[allow(clippy::cast_precision_loss)]
fn signed(state: &mut u64) -> f32 {
    *state = splitmix64(*state);
    // Top 24 bits: exact in an f32.
    ((*state >> 40) as f32 / (1u64 << 23) as f32) - 1.0
}

/// A piece's launch: linear and angular velocity, from the break's seed and the piece's rank.
fn launch(seed: u64, rank: u64, from_hit: Vec3, dir: Vec3, speed_mps: f32, spread: f32) -> (Vec3, Vec3) {
    let mut s = splitmix64(seed ^ rank);
    let base = (dir.normalize_or_zero() + from_hit.normalize_or_zero()).normalize_or(Vec3::Y);
    let jitter = Vec3::new(signed(&mut s), signed(&mut s), signed(&mut s));
    let way = (base + jitter * spread).normalize_or(base);
    let mps = (speed_mps * (1.0 + spread * signed(&mut s))).max(0.0);
    let spin = Vec3::new(signed(&mut s), signed(&mut s), signed(&mut s)) * (mps * spread * 4.0);
    (way * mps, spin)
}

fn world_pos(v: Vec3) -> WorldPos {
    WorldPos::from_f64(v.as_dvec3())
}

/// Ages, fades, shrinks and despawns pieces (module docs).
#[allow(clippy::needless_pass_by_value)]
fn fade(
    mut commands: Commands,
    settings: Res<InteractionSettings>,
    time: Res<Time<Fixed>>,
    mut physics: ResMut<Physics>,
    mut pieces: Query<(Entity, &mut DebrisPiece, Option<&mut DebrisFading>, &PhysicsBody, &mut Transform)>,
) {
    let dt = time.timestep().as_secs_f32();
    let p = settings.0;
    for (e, mut piece, fading, body, mut t) in &mut pieces {
        let sleeping = physics.world(&body.worldspace).and_then(|w| w.is_sleeping(body.id));
        let Some(sleeping) = sleeping else {
            commands.entity(e).despawn();
            continue;
        };
        if let Some(mut f) = fading {
            f.left_s -= dt;
            if f.left_s <= 0.0 {
                if let Some(w) = physics.world_mut(&body.worldspace) {
                    w.remove(body.id);
                }
                commands.entity(e).despawn();
            } else {
                t.scale = f.from * (f.left_s / f.total_s);
            }
            continue;
        }
        piece.age_s += dt;
        if sleeping || piece.age_s >= p.debris_fade_s {
            let total_s = p.debris_shrink_s.max(f32::EPSILON);
            commands.entity(e).insert(DebrisFading { left_s: total_s, total_s, from: t.scale });
        }
    }
}

/// One piece to spawn this tick.
struct NewPiece {
    piece: DebrisPiece,
    desc: BodyDesc,
    velocity: (Vec3, Vec3),
}

/// The tick's breaks into pieces, then the cap (module docs).
#[allow(clippy::needless_pass_by_value, clippy::too_many_arguments)]
fn spawn(
    mut commands: Commands,
    settings: Res<InteractionSettings>,
    registry: Res<Destructibles>,
    sets: Res<DebrisSets>,
    focus: Res<PhysicsFocus>,
    mut physics: ResMut<Physics>,
    mut order: ResMut<DebrisOrder>,
    mut breaks: MessageReader<Destroyed>,
    live: Query<(Entity, &DebrisPiece, &Transform), Without<DebrisFading>>,
) {
    let p = settings.0;
    let cap = usize::from(p.debris_awake_max);
    let mut new: Vec<NewPiece> = Vec::new();
    let (Some(view), Some(ws)) = (focus.0, physics.worlds.active().cloned()) else {
        breaks.clear();
        return;
    };
    for Destroyed(b) in breaks.read() {
        // `as f32`: the LOD distance is tens of metres.
        #[allow(clippy::cast_possible_truncation)]
        let distance = view.delta(world_pos(b.point)).length() as f32;
        if !debris_lod(distance, p.debris_lod_m) {
            continue;
        }
        let Some(stage) = registry
            .get(b.placement)
            .and_then(|d| Some((d.object, d.destructible()?.stages.get(usize::from(b.stage))?)))
        else {
            continue;
        };
        let (object, stage) = stage;
        let mut rank = 0u64;
        for set in &stage.debris {
            let Some(defs) = sets.get(set) else { continue };
            for (i, def) in defs.iter().enumerate() {
                rank += 1;
                let pos = object.translation + object.rotation * def.local.translation;
                let rotation: Quat = object.rotation * def.local.rotation;
                let pose = WorldTransform { pos: world_pos(pos), rotation, scale: Vec3::ONE };
                let desc = BodyDesc {
                    mass: Some(def.mass),
                    damping: ITEM_DAMPING,
                    ..BodyDesc::new(BodyKind::Dynamic { asleep: false }, pose, def.shape.clone(), Layer::DEBRIS)
                };
                let velocity = launch(b.seed, rank, pos - b.point, b.dir, p.debris_speed_mps, p.debris_spread);
                new.push(NewPiece {
                    piece: DebrisPiece {
                        set: set.clone(),
                        piece: u16::try_from(i).unwrap_or(u16::MAX),
                        placement: b.placement,
                        order: 0,
                        age_s: 0.0,
                    },
                    desc,
                    velocity,
                });
            }
        }
    }
    // The newest pieces win: a tick's breaks past the cap keep their last `cap` pieces.
    if new.len() > cap {
        new.drain(..new.len() - cap);
    }
    if new.is_empty() {
        return;
    }
    let mut old: Vec<(u64, Entity, Vec3)> = live.iter().map(|(e, d, t)| (d.order, e, t.scale)).collect();
    old.sort_unstable_by_key(|o| o.0);
    let excess = (old.len() + new.len()).saturating_sub(cap);
    let total_s = p.debris_shrink_s.max(f32::EPSILON);
    for (_, e, from) in old.into_iter().take(excess) {
        commands.entity(e).insert(DebrisFading { left_s: total_s, total_s, from });
    }
    for mut n in new {
        let id = physics.insert(&ws, &n.desc);
        if let Some(w) = physics.world_mut(&ws) {
            w.set_velocity(id, n.velocity.0, n.velocity.1);
        }
        n.piece.order = order.0;
        order.0 += 1;
        let (cell, transform) = transform_to_grid(&n.desc.pose);
        commands.spawn((n.piece, PhysicsBody { worldspace: ws.clone(), id }, cell, transform));
    }
}
