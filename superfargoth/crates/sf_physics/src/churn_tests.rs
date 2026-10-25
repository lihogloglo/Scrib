//! Streaming churn stress (physcrash, 2026-10-19): terrain chunks, placed
//! sleeping items and walker capsules inserted, moved and removed the way
//! `sf_physics_bevy` streaming and the walkers do, with query refreshes
//! between inserts and up to 20 steps a frame (time-lapse). Debug builds run
//! rapier's persistent-island validation every step, so a broken island
//! invariant panics here.
#![allow(clippy::cast_precision_loss, clippy::cast_possible_truncation, clippy::cast_possible_wrap)]

use glam::{IVec3, Vec3};
use sf_coords::WorldPos;

use crate::character::{CharacterShape, PushParams};
use crate::tests::at;
use crate::{BodyDesc, BodyId, Layer, PhysicsParams, PhysicsWorld, ShapeRef};

/// Tiny deterministic generator (no rand dependency in tests).
struct Lcg(u64);

impl Lcg {
    fn next(&mut self) -> u32 {
        self.0 = self.0.wrapping_mul(6_364_136_223_846_793_005).wrapping_add(1_442_695_040_888_963_407);
        (self.0 >> 33) as u32
    }
    fn below(&mut self, n: u32) -> u32 {
        self.next() % n.max(1)
    }
    fn unit(&mut self) -> f32 {
        self.below(10_000) as f32 / 10_000.0
    }
}

const GRID: i32 = 3;
const CHUNK: f32 = 8.0;

fn chunk_center(c: i32) -> Vec3 {
    let (x, z) = (c % GRID, c / GRID);
    Vec3::new((x as f32 + 0.5) * CHUNK, -0.5, (z as f32 + 0.5) * CHUNK)
}

fn insert_chunk(w: &mut PhysicsWorld, c: i32) -> BodyId {
    let pose = at(WorldPos::ORIGIN.offset(chunk_center(c)));
    w.insert(&BodyDesc::fixed(pose, ShapeRef::Box(Vec3::new(CHUNK * 0.5, 0.5, CHUNK * 0.5)), Layer::TERRAIN))
}

/// A cluster of touching placed items (a pile on the floor), inserted asleep.
fn insert_cluster(w: &mut PhysicsWorld, center: Vec3) -> Vec<BodyId> {
    let mut ids = Vec::new();
    for i in 0..3 {
        for j in 0..2 {
            let p = center + Vec3::new(i as f32 * 0.3, 0.15 + j as f32 * 0.3, 0.0);
            let desc = BodyDesc::item(at(WorldPos::ORIGIN.offset(p)), ShapeRef::Box(Vec3::splat(0.15)), Layer::PROP, Some(2.0));
            ids.push(w.insert(&desc));
        }
    }
    ids
}

struct Walker {
    id: BodyId,
    from: Vec3,
    to: Vec3,
    t: f32,
}

/// Runs `frames` frames of churn in the game's order: walkers move, spawn and
/// despawn capsules, streaming inserts terrain and refreshes the queries
/// (`refresh`), item cells come and go, something knocks an item (`knocks`),
/// then up to 20 fixed steps run.
fn churn(seed: u64, frames: usize, refresh: bool, knocks: bool) {
    let mut rng = Lcg(seed);
    let mut w = PhysicsWorld::new(WorldPos::ORIGIN, PhysicsParams::default());
    let shape = CharacterShape { radius: 0.35, height: 1.8 };
    let push = PushParams::default();
    let mut chunks: Vec<Option<BodyId>> = (0..GRID * GRID).map(|c| Some(insert_chunk(&mut w, c))).collect();
    w.refresh_queries();
    let mut clusters: Vec<Vec<BodyId>> = Vec::new();
    let mut walkers: Vec<Walker> = Vec::new();
    let span = GRID as f32 * CHUNK;
    for _ in 0..frames {
        // Walkers: move every capsule along its leg, sometimes despawn one.
        for k in &mut walkers {
            k.t = (k.t + 0.02).min(1.0);
            let feet = k.from.lerp(k.to, k.t);
            w.move_character(k.id, WorldPos::ORIGIN.offset(feet), shape);
        }
        walkers.retain(|k| {
            let keep = k.t < 1.0;
            if !keep {
                w.remove(k.id);
            }
            keep
        });
        // Walkers going live: a new capsule next to a cluster, walking through it.
        let mut inserted_fixed_or_kinematic = false;
        if rng.below(4) == 0 && walkers.len() < 12 && !clusters.is_empty() {
            let c = &clusters[rng.below(clusters.len() as u32) as usize];
            if let Some(target) = c.first().and_then(|id| w.pose(*id)) {
                let to = target.pos.to_f64().as_vec3() * Vec3::new(1.0, 0.0, 1.0);
                let from = to + Vec3::new(rng.unit() * 2.0 - 1.0, 0.0, rng.unit() * 2.0 - 1.0) * 0.6;
                let id = w.insert(&BodyDesc::character(WorldPos::ORIGIN.offset(from), shape, &push));
                walkers.push(Walker { id, from, to: to + (to - from), t: 0.0 });
                inserted_fixed_or_kinematic = true;
            }
        }
        // Streaming: terrain chunks out and back in.
        if rng.below(6) == 0 {
            let c = rng.below((GRID * GRID) as u32) as usize;
            if let Some(id) = chunks[c].take() {
                w.remove(id);
            } else {
                chunks[c] = Some(insert_chunk(&mut w, c as i32));
                inserted_fixed_or_kinematic = true;
            }
        }
        if refresh && inserted_fixed_or_kinematic {
            w.refresh_queries();
        }
        // Streaming: item cells out and in.
        if rng.below(5) == 0 && !clusters.is_empty() {
            let gone = clusters.swap_remove(rng.below(clusters.len() as u32) as usize);
            for id in gone {
                w.remove(id);
            }
        }
        if rng.below(4) == 0 && clusters.len() < 10 {
            let p = Vec3::new(rng.unit() * (span - 2.0) + 1.0, 0.0, rng.unit() * (span - 2.0) + 1.0);
            clusters.push(insert_cluster(&mut w, p));
            if refresh {
                w.refresh_queries();
            }
        }
        // Something knocks an item (a push, a spell): its pile wakes and splits.
        if knocks && rng.below(3) == 0 && !clusters.is_empty() {
            let c = &clusters[rng.below(clusters.len() as u32) as usize];
            let id = c[rng.below(c.len() as u32) as usize];
            let dir = Vec3::new(rng.unit() - 0.5, 0.3, rng.unit() - 0.5) * 6.0;
            w.apply_impulse(id, dir, None);
        }
        // The floating origin moves now and then (a whole-cell shift).
        if rng.below(40) == 0 {
            let cell = if w.anchor().cell == IVec3::ZERO { IVec3::X } else { IVec3::ZERO };
            w.rebase(cell);
            if refresh {
                w.refresh_queries();
            }
        }
        // Time-lapse: up to 20 fixed steps this frame.
        let steps = 1 + rng.below(20);
        for _ in 0..steps {
            w.step();
        }
        // Chunks must come back eventually or items fall out.
        if rng.below(3) == 0
            && let Some(c) = chunks.iter().position(Option::is_none)
        {
            chunks[c] = Some(insert_chunk(&mut w, c as i32));
            if refresh {
                w.refresh_queries();
            }
        }
        w.drain_events().for_each(drop);
    }
}

/// Regression (physcrash): with the old collision-only refresh, rapier's
/// debug validation failed within a few hundred frames (unregistered capsule
/// contacts, force-event pair list, persistent island links).
#[test]
fn streaming_churn_keeps_rapier_consistent() {
    for seed in 1..=8 {
        churn(seed, 600, true, true);
    }
}

#[test]
fn churn_without_refresh_keeps_rapier_consistent() {
    for seed in 1..=4 {
        churn(seed, 600, false, true);
    }
}

/// Queries between steps: inserts and a rebase are seen after
/// `refresh_queries` alone (no collision-only pass, no step).
#[test]
fn refresh_without_step_sees_inserts_and_rebase() {
    use sf_controller::{GroundProbe, ProbeQuery};
    let mut w = PhysicsWorld::new(WorldPos::ORIGIN, PhysicsParams::default());
    let top = WorldPos::ORIGIN.offset(Vec3::new(4.0, 3.0, 4.0));
    let floor = |w: &PhysicsWorld| {
        w.probe(Layer::TERRAIN).probe_one(ProbeQuery::down(top, 10.0)).map(|h| h.point.to_f64().y)
    };
    insert_chunk(&mut w, 0);
    w.refresh_queries();
    let y = floor(&w).expect("chunk seen after refresh");
    assert!(y.abs() < 1e-4, "{y}");
    w.rebase(IVec3::X);
    w.refresh_queries();
    let y = floor(&w).expect("chunk seen after rebase");
    assert!(y.abs() < 1e-3, "{y}");
}
