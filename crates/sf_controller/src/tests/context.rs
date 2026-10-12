//! Context probes on synthetic geometry: boxes of several heights and
//! depths, ledges, stairs, overhangs, walls, drops and terrain slope edges.
#![allow(clippy::cast_sign_loss)]

use super::*;

const AHEAD: Vec3 = Vec3::Z;

/// One request from the origin facing +Z.
fn ask(world: &dyn GroundProbe, feet: WorldPos, query: ContextQuery) -> Option<ContextHit> {
    let params = ContextParams::default();
    let mut out = [None];
    ContextProbes::new().run(&[ContextRequest { feet, dir: AHEAD, params: &params, query }], world, &mut out);
    out[0]
}

/// A box across the path, its face `face` ahead, `depth` deep, `height` tall.
fn obstacle(world: &mut World, face: f32, depth: f32, height: f32) {
    world.add_box(Vec3::new(-2.0, 0.0, face), Vec3::new(2.0, height, face + depth));
}

fn obstacle_ahead(world: &World) -> Option<ContextHit> {
    ask(world, WorldPos::ORIGIN, ContextQuery::ObstacleAhead { range: 2.0 })
}

#[test]
fn obstacles_classify_by_height_and_depth() {
    // (height, depth) → affordance, at the default 1.8 m character
    // (step 0.45, vault ≤ 1.26 and ≤ 0.59 deep, mantle ≤ 2.7, climb ≤ 4.5).
    let table = [
        (0.3, 2.0, Affordance::Step),
        (0.6, 0.3, Affordance::Vault),
        (1.0, 0.4, Affordance::Vault),
        (1.2, 0.5, Affordance::Vault),
        (1.0, 2.0, Affordance::Mantle),
        (0.9, 0.9, Affordance::Mantle),
        (1.8, 2.0, Affordance::Mantle),
        (2.5, 2.0, Affordance::Mantle),
        (3.5, 2.0, Affordance::Climb),
        (5.0, 2.0, Affordance::Blocked),
    ];
    for (height, depth, kind) in table {
        let mut w = World::new();
        obstacle(&mut w, 1.0, depth, height);
        w.statics.rebuild();
        let hit = obstacle_ahead(&w).unwrap_or_else(|| panic!("nothing found for {height} × {depth}"));
        assert_eq!(hit.kind, kind, "{height} m high, {depth} m deep: {hit:?}");
        if kind == Affordance::Blocked {
            continue;
        }
        let lip = xyz(hit.point);
        assert!((lip.y - height).abs() < 0.01 && (lip.z - 1.0).abs() < 0.01, "{height}: lip {lip}");
        assert!(hit.normal.distance(Vec3::NEG_Z) < 1e-3, "{:?}", hit.normal);
        assert!((hit.height - height).abs() < 0.01);
        if kind == Affordance::Vault {
            assert!((hit.depth - depth).abs() < 0.01, "{height}: depth {} vs {depth}", hit.depth);
            let landing = xyz(hit.landing.expect("a vault lands"));
            assert!(landing.y.abs() < 0.01 && landing.z > 1.0 + depth, "{landing}");
        }
    }
}

#[test]
fn no_room_on_top_blocks_a_mantle_and_a_ramp_is_not_an_obstacle() {
    let mut w = World::new();
    obstacle(&mut w, 1.0, 2.0, 1.0);
    // A slab 0.6 m above the top: less than a crouch.
    w.add_box(Vec3::new(-2.0, 1.6, 1.2), Vec3::new(2.0, 1.9, 3.0));
    w.statics.rebuild();
    let hit = obstacle_ahead(&w).unwrap();
    assert_eq!(hit.kind, Affordance::Blocked, "{hit:?}");
    assert!((hit.clearance - 0.6).abs() < 0.01, "{hit:?}");

    let mut w = World::new();
    w.add_ramp(-0.5, 20f32.to_radians());
    w.statics.rebuild();
    let params = ContextParams::default();
    let mut out = [None];
    let feet = p(-1.0, 0.0, 0.0); // the ramp rises along +X
    let query = ContextQuery::ObstacleAhead { range: 2.0 };
    ContextProbes::new().run(&[ContextRequest { feet, dir: Vec3::X, params: &params, query }], &w, &mut out);
    assert_eq!(out[0], None);
}

#[test]
fn ledges_of_several_heights_give_the_lip_normal_and_hang_point() {
    let params = ContextParams::default();
    for top in [1.5, 2.0, 2.4] {
        let mut w = World::new();
        obstacle(&mut w, 0.7, 2.0, top);
        w.statics.rebuild();
        let hit = ask(&w, WorldPos::ORIGIN, ContextQuery::Ledge { reach: 1.0, max_height: 2.5 }).unwrap();
        assert_eq!(hit.kind, Affordance::Ledge);
        let lip = xyz(hit.point);
        assert!(lip.distance(Vec3::new(0.0, top, 0.7)) < 0.01, "{top}: lip {lip}");
        assert!(hit.normal.distance(Vec3::NEG_Z) < 1e-3);
        assert!((hit.depth - 0.3).abs() < 0.01 && hit.clearance > params.height);
        let (hang, yaw) = hit.hang_point(&params);
        let expect = Vec3::new(0.0, top - params.hang_drop, 0.7 - params.hang_offset);
        assert!(xyz(hang).distance(expect) < 0.01 && yaw.abs() < 1e-3, "{top}: hang {}", xyz(hang));
    }
    // A wall taller than the reach height has no ledge to grab.
    let mut w = World::new();
    obstacle(&mut w, 0.7, 2.0, 3.0);
    w.statics.rebuild();
    assert_eq!(ask(&w, WorldPos::ORIGIN, ContextQuery::Ledge { reach: 1.0, max_height: 2.5 }), None);
}

#[test]
fn stairs_are_steps_up_and_walkable_down() {
    let mut w = World::new();
    // Ten 0.2 m steps, 0.3 m deep, from z = 1.
    for k in 0..10 {
        let z = 1.0 + 0.3 * k as f32;
        w.add_box(Vec3::new(-1.0, 0.0, z), Vec3::new(1.0, 0.2 * (k + 1) as f32, z + 0.3));
    }
    w.statics.rebuild();
    let hit = obstacle_ahead(&w).unwrap();
    assert_eq!(hit.kind, Affordance::Step, "{hit:?}");
    assert!((hit.height - 0.2).abs() < 0.01 && (xyz(hit.point).z - 1.0).abs() < 0.01, "{hit:?}");
    // From the top landing, looking back down the flight: no drop.
    let params = ContextParams::default();
    let mut out = [None];
    let feet = p(0.0, 2.0, 3.85);
    let query = ContextQuery::DropAhead { range: 2.0, min_drop: 1.0 };
    ContextProbes::new().run(&[ContextRequest { feet, dir: Vec3::NEG_Z, params: &params, query }], &w, &mut out);
    assert_eq!(out[0], None, "{:?}", out[0]);
}

#[test]
fn overhangs_are_ducked_under_climbed_or_ignored() {
    // Bottom at 1.4 m: a crouch fits under it.
    let mut w = World::new();
    w.add_box(Vec3::new(-2.0, 1.4, 1.0), Vec3::new(2.0, 1.7, 3.0));
    w.statics.rebuild();
    let hit = obstacle_ahead(&w).unwrap();
    assert_eq!(hit.kind, Affordance::LowCeiling, "{hit:?}");
    assert!((hit.clearance - 1.4).abs() < 0.01, "{hit:?}");
    // Standing under it, the ceiling probe agrees.
    let c = ask(&w, p(0.0, 0.0, 2.0), ContextQuery::Ceiling { height: 1.8 }).unwrap();
    assert_eq!(c.kind, Affordance::LowCeiling);
    assert!((c.height - 1.4).abs() < 0.01 && c.normal.y < -0.99, "{c:?}");
    // Bottom at 0.7 m: too low to duck under, a slab to mantle onto.
    let mut w = World::new();
    w.add_box(Vec3::new(-2.0, 0.7, 1.0), Vec3::new(2.0, 1.0, 3.0));
    w.statics.rebuild();
    assert_eq!(obstacle_ahead(&w).map(|h| h.kind), Some(Affordance::Mantle));
    // Bottom above the head: nothing in the way.
    let mut w = World::new();
    w.add_box(Vec3::new(-2.0, 2.0, 1.0), Vec3::new(2.0, 2.3, 3.0));
    w.statics.rebuild();
    assert_eq!(obstacle_ahead(&w), None);
    assert_eq!(ask(&w, p(0.0, 0.0, 2.0), ContextQuery::Ceiling { height: 1.8 }), None);
}

#[test]
fn walls_beside_give_run_normals() {
    let mut w = World::new();
    // Facing +Z, right is -X: a wall 0.8 m to the right.
    w.add_box(Vec3::new(-1.3, 0.0, -5.0), Vec3::new(-0.8, 3.0, 5.0));
    w.statics.rebuild();
    let hit = ask(&w, WorldPos::ORIGIN, ContextQuery::WallBeside { side: Side::Right, range: 1.5 }).unwrap();
    assert_eq!(hit.kind, Affordance::Wall);
    assert!(hit.normal.distance(Vec3::X) < 1e-3 && (hit.depth - 0.8).abs() < 1e-3, "{hit:?}");
    assert!(hit.wall_tangent(AHEAD).distance(AHEAD) < 1e-3);
    assert_eq!(ask(&w, WorldPos::ORIGIN, ContextQuery::WallBeside { side: Side::Left, range: 1.5 }), None);
}

/// A plateau `top` high for z < `z0`, falling at `slope` (rise per run) to 0.
struct Cliff {
    top: f32,
    z0: f32,
    slope: f32,
}

impl HeightSurface for Cliff {
    fn height(&self, p: WorldPos) -> Option<f32> {
        Some((self.top - (xyz(p).z - self.z0).max(0.0) * self.slope).max(0.0))
    }
    fn normal(&self, p: WorldPos) -> Option<Vec3> {
        let h = self.height(p)?;
        Some(if h > 0.0 && h < self.top { Vec3::new(0.0, 1.0, self.slope).normalize() } else { Vec3::Y })
    }
    fn raycast(&self, origin: WorldPos, dir: Vec3, max: f32) -> Option<(f32, Vec3)> {
        // March 1 mm steps (a test surface).
        let steps = (max / 1e-3) as usize;
        (0..=steps).map(|i| i as f32 * 1e-3).find_map(|t| {
            let at = origin.offset(dir * t);
            (xyz(at).y <= self.height(at)?).then(|| (t, self.normal(at).unwrap()))
        })
    }
}

#[test]
fn drops_off_boxes_and_terrain_slope_edges() {
    // On a 2 m platform whose edge is 1 m ahead.
    let mut w = World::new();
    w.add_box(Vec3::new(-2.0, 0.0, -3.0), Vec3::new(2.0, 2.0, 1.0));
    w.statics.rebuild();
    let hit = ask(&w, p(0.0, 2.0, 0.0), ContextQuery::DropAhead { range: 2.0, min_drop: 1.0 }).unwrap();
    assert_eq!(hit.kind, Affordance::Drop);
    assert!((hit.height - 2.0).abs() < 0.01 && xyz(hit.point).distance(Vec3::new(0.0, 2.0, 1.0)) < 0.01, "{hit:?}");
    assert!(hit.normal.distance(Vec3::Z) < 1e-3 && (hit.depth - 1.0).abs() < 0.01);
    // A smaller fall than asked for is not reported.
    assert_eq!(ask(&w, p(0.0, 2.0, 0.0), ContextQuery::DropAhead { range: 2.0, min_drop: 2.5 }), None);

    // Terrain: a 3 m plateau ending in a 70° slope 1 m ahead.
    let world = |slope: f32| TerrainProbe(Cliff { top: 3.0, z0: 1.0, slope });
    let steep = world(70f32.to_radians().tan());
    let hit = ask(&steep, p(0.0, 3.0, 0.0), ContextQuery::DropAhead { range: 3.0, min_drop: 1.0 }).unwrap();
    assert_eq!(hit.kind, Affordance::Drop, "{hit:?}");
    assert!((hit.height - 3.0).abs() < 0.01 && (xyz(hit.point).z - 1.0).abs() < 0.05, "{hit:?}");
    // A 20° slope is walked down.
    let gentle = world(20f32.to_radians().tan());
    assert_eq!(ask(&gentle, p(0.0, 3.0, 0.0), ContextQuery::DropAhead { range: 3.0, min_drop: 1.0 }), None);
}

#[test]
fn batched_requests_equal_single_ones_and_warp_onto_the_top() {
    let mut w = World::new();
    obstacle(&mut w, 1.0, 2.0, 1.2);
    w.add_box(Vec3::new(-1.3, 0.0, -5.0), Vec3::new(-0.8, 3.0, 0.9));
    w.statics.rebuild();
    let params = ContextParams::default();
    let queries = [
        ContextQuery::ObstacleAhead { range: 2.0 },
        ContextQuery::Ledge { reach: 1.5, max_height: 2.5 },
        ContextQuery::DropAhead { range: 2.0, min_drop: 0.5 },
        ContextQuery::WallBeside { side: Side::Right, range: 1.5 },
        ContextQuery::Ceiling { height: 1.8 },
    ];
    let requests: Vec<ContextRequest<'_>> =
        (0..3).flat_map(|i| queries.map(|query| ContextRequest { feet: p(0.1 * i as f32, 0.0, 0.0), dir: AHEAD, params: &params, query })).collect();
    let mut batched = vec![None; requests.len()];
    ContextProbes::new().run(&requests, &w, &mut batched);
    let mut probes = ContextProbes::new();
    for (r, b) in requests.iter().zip(&batched) {
        let mut one = [None];
        probes.run(std::slice::from_ref(r), &w, &mut one);
        assert_eq!(one[0], *b);
    }
    assert_eq!(batched[0].map(|h| h.kind), Some(Affordance::Mantle));
    assert_eq!(batched[1].map(|h| h.kind), Some(Affordance::Ledge));
    assert_eq!(batched[3].map(|h| h.kind), Some(Affordance::Wall));
    let (target, yaw) = batched[0].unwrap().warp_target(0.5);
    assert!(xyz(target).distance(Vec3::new(0.0, 1.2, 1.5)) < 0.01 && yaw.abs() < 1e-3, "{}", xyz(target));
}

// --- Timing ------------------------------------------------------------------------------

/// The probe budget (locomotion.md: 0.2 ms for the 150-NPC crowd, ground
/// probes included): 150 characters in the town of `crowd_step_timing`,
/// each blocked by a vault box (the most expensive query).
/// `cargo test --release -p sf_controller -- --ignored --nocapture context_probe`
#[test]
#[ignore = "timing; run in release"]
fn context_probe_timing() {
    let mut world = World::new();
    for i in 0..20 {
        for j in 0..20 {
            let (x, z) = (i as f32 * 12.0, j as f32 * 12.0);
            world.add_box(Vec3::new(x, 0.0, z), Vec3::new(x + 6.0, 5.0, z + 6.0));
            world.add_box(Vec3::new(x + 7.0, 0.0, z), Vec3::new(x + 8.0, 0.3, z + 3.0));
            world.add_box(Vec3::new(x + 6.5, 0.0, z + 8.0), Vec3::new(x + 9.5, 1.0, z + 8.4));
        }
    }
    world.statics.rebuild();
    let params = ContextParams::default();
    let feet: Vec<WorldPos> = (0..150).map(|k| p(8.0 + (k % 15) as f32 * 12.0, 0.0, 7.0 + (k / 15) as f32 * 12.0)).collect();
    let params = &params;
    let mut probes = ContextProbes::new();
    let mut run = |queries: &[ContextQuery]| {
        let requests: Vec<ContextRequest<'_>> =
            feet.iter().flat_map(|&f| queries.iter().map(move |&query| ContextRequest { feet: f, dir: AHEAD, params, query })).collect();
        let mut out = vec![None; requests.len()];
        let mut times = Vec::new();
        for _ in 0..300 {
            let t0 = std::time::Instant::now();
            probes.run(&requests, &world, &mut out);
            times.push(t0.elapsed().as_secs_f64() * 1e3);
        }
        times.sort_by(f64::total_cmp);
        let vaults = out.iter().filter(|h| h.is_some_and(|h| h.kind == Affordance::Vault)).count();
        (times[150], times[285], vaults)
    };
    // The raw casts, for the breakdown: 2 m capsule sweeps, 5 m down-rays.
    let raw = |queries: Vec<ProbeQuery>| {
        let mut out = vec![None; queries.len()];
        let t0 = std::time::Instant::now();
        for _ in 0..100 {
            world.probe(&queries, &mut out);
        }
        t0.elapsed().as_secs_f64() * 10.0
    };
    let sweeps = raw(feet.iter().map(|f| ProbeQuery::capsule(f.offset(Vec3::Y * 1.0), AHEAD, 2.0, 0.3, 0.5)).collect());
    let rays = raw(feet.iter().map(|f| ProbeQuery::down(f.offset(Vec3::new(0.0, 4.5, 1.2)), 5.0)).collect());
    let balls = raw(feet.iter().map(|f| ProbeQuery::sphere(f.offset(Vec3::Y * 1.0), AHEAD, 2.0, 0.3)).collect());
    let flat = raw(feet.iter().map(|f| ProbeQuery::ray(f.offset(Vec3::Y * 0.6), AHEAD, 2.3)).collect());
    println!("raw, 150 each: sphere sweeps {balls:.3} ms, forward rays {flat:.3} ms");
    let statics_only = {
        let queries: Vec<ProbeQuery> = feet.iter().map(|f| ProbeQuery::capsule(f.offset(Vec3::Y * 1.0), AHEAD, 2.0, 0.3, 0.5)).collect();
        let mut out = vec![None; queries.len()];
        let t0 = std::time::Instant::now();
        for _ in 0..100 {
            world.statics.probe(&queries, &mut out);
        }
        t0.elapsed().as_secs_f64() * 10.0
    };
    println!("raw, 150 each: capsule sweeps {sweeps:.3} ms ({statics_only:.3} statics only), down-rays {rays:.3} ms");
    let (median, p95, vaults) = run(&[ContextQuery::ObstacleAhead { range: 2.0 }]);
    assert!(vaults == 150, "{vaults} of 150 found the vault box");
    println!("150 × obstacle ahead: median {median:.3} ms, p95 {p95:.3} ms");
    let all = [
        ContextQuery::ObstacleAhead { range: 2.0 },
        ContextQuery::Ledge { reach: 1.5, max_height: 2.5 },
        ContextQuery::DropAhead { range: 2.0, min_drop: 0.5 },
        ContextQuery::Ceiling { height: 1.8 },
    ];
    let (median, p95, _) = run(&all);
    println!("150 × 4 queries (obstacle, ledge, drop, ceiling): median {median:.3} ms, p95 {p95:.3} ms");
}
