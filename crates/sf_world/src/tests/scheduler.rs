use std::time::Instant;

use glam::{DVec3, IVec3, Quat, Vec2, Vec3};
use sf_coords::{WorldPos, WorldTransform};

use super::{Lcg, models, placement, ws};
use crate::*;

const DT: f64 = 1.0 / 120.0;
const MAIN_STAGES: [Stage; 5] = [Stage::Deactivate, Stage::Despawn, Stage::Spawn, Stage::Upload, Stage::Activate];

/// A `w × d` grid of 128 m cells in worldspace `name`, one cell-sized block
/// per cell (plus `extra` small objects), starting at the origin.
fn grid_world(name: &str, w: i32, d: i32, extra: u32) -> CellDirectory {
    let params = IndexParams::default();
    let mut input = Vec::new();
    for x in 0..w {
        for z in 0..d {
            let c = DVec3::new(f64::from(x) * 128.0 + 64.0, 1.0, f64::from(z) * 128.0 + 64.0);
            input.push(placement(&format!("{name}/{x}/{z}"), c, Some("block.mesh"), PlacementClass::Static));
            for i in 0..extra {
                let off = DVec3::new(f64::from(i % 10) * 12.0 - 60.0, 0.0, f64::from(i / 10) * 12.0 - 60.0);
                input.push(placement(&format!("{name}/{x}/{z}/{i}"), c + off, Some("small.mesh"), PlacementClass::Static));
            }
        }
    }
    let (pages, _) = build_pages(&params, input, &models);
    build_directory(ws(name), &params, &pages)
}

fn interior(name: &str) -> CellDirectory {
    let params = IndexParams { partition: Partition::Single, ..IndexParams::default() };
    let input = (0..20).map(|i| placement(&format!("{name}/{i}"), DVec3::new(f64::from(i), 0.0, 3.0), Some("small.mesh"), PlacementClass::Static));
    let (pages, _) = build_pages(&params, input, &models);
    build_directory(ws(name), &params, &pages)
}

fn anchor(id: u32, role: AnchorRole, world: &str, pos: DVec3) -> Anchor {
    Anchor::new(AnchorId(id), role, Location::new(ws(world), WorldPos::from_f64(pos)))
}

/// Drives a scheduler like an engine would: async stages complete a few
/// frames after they start, main-thread items cost pseudo-random time.
struct Sim {
    s: Scheduler,
    frame: u64,
    now: f64,
    pending: Vec<(RequestId, Stage, u64)>,
    log: Vec<CellChange>,
    trace: Vec<String>,
    rng: Lcg,
    items_per_stage: u32,
    /// Stage reports that reached their limit (budget checks were exercised).
    saturated: u32,
}

impl Sim {
    fn new(config: SchedulerConfig, worlds: Vec<CellDirectory>) -> Self {
        let mut s = Scheduler::new(config);
        for d in worlds {
            assert!(s.add_worldspace(d, None));
        }
        Self { s, frame: 0, now: 0.0, pending: Vec::new(), log: Vec::new(), trace: Vec::new(), rng: Lcg(7), items_per_stage: 6, saturated: 0 }
    }

    fn frame(&mut self, anchors: &[Anchor]) -> TickReport {
        let report = self.s.update(&TickInput { now: self.now, anchors });
        self.trace.push(format!("{report:?}"));
        self.check_in_flight();
        for id in &report.start {
            let stage = self.s.request(*id).unwrap().stage;
            self.pending.push((*id, stage, self.frame + 3));
        }
        let frame = self.frame;
        let due: Vec<_> = self.pending.iter().filter(|p| p.2 <= frame).copied().collect();
        self.pending.retain(|p| p.2 > frame);
        for (id, stage, _) in due {
            match stage {
                Stage::Fetch => self.s.fetched(id, self.now, 4, 3 << 20),
                Stage::Assets => self.s.assets_loaded(id, self.now),
                _ => unreachable!(),
            };
        }
        for stage in MAIN_STAGES {
            let items = self.items_per_stage;
            let rng = &mut self.rng;
            let mut costs = Vec::new();
            let r = self.s.run_stage(stage, &mut |req| {
                let cost = if stage == Stage::Upload { f64::from(1u32 << 20) } else { rng.range(0.05, 0.6) };
                costs.push(cost);
                StageStep { cost, done: req.progress + 1 >= items }
            });
            // Budget rule: every item started under the limit, so the
            // overshoot is at most the last item.
            let mut sum = 0.0;
            for c in &costs {
                assert!(sum < r.limit, "{stage:?} started an item over budget");
                sum += c;
            }
            assert!((sum - r.used).abs() < 1e-9);
            if r.used >= r.limit {
                self.saturated += 1;
            }
            self.trace.push(format!("{stage:?} {r:?}"));
            self.log.extend(r.completed);
        }
        self.frame += 1;
        self.now += DT;
        report
    }

    fn check_in_flight(&self) {
        let b = self.s.config().budgets;
        let burst = if self.s.in_burst() { b.burst_factor } else { 1.0 };
        let (_, fetching) = self.s.queue_depth(Stage::Fetch);
        #[allow(clippy::cast_precision_loss)]
        let fetching = fetching as f64;
        assert!(fetching <= f64::from(b.fetches_in_flight) * burst);
        let assets: u32 = self.s.requests().filter(|r| r.stage == Stage::Assets && r.running).map(|r| r.assets.max(1)).sum();
        // Weighted cap: may pass by at most one request's assets (4 here).
        assert!(f64::from(assets) < f64::from(b.assets_in_flight) * burst + 4.0);
    }

    fn run(&mut self, frames: u32, anchors: &[Anchor]) {
        for _ in 0..frames {
            self.frame(anchors);
        }
    }

    fn count(&self, cell: IVec3, pred: impl Fn(&CellChange) -> bool) -> usize {
        self.log.iter().filter(|c| c.cell == cell && pred(c)).count()
    }
}

/// The streaming cell of grid-world column `x`, row `z`.
fn cell(x: i32, z: i32) -> IVec3 {
    IVec3::new(x + 1, 1, z + 1)
}

#[test]
fn reaches_target_levels_one_step_at_a_time() {
    let mut sim = Sim::new(SchedulerConfig::default(), vec![grid_world("w", 12, 3, 0)]);
    let player = anchor(0, AnchorRole::Gameplay, "w", DVec3::new(64.0, 1.0, 192.0));
    let camera = anchor(1, AnchorRole::Visual, "w", DVec3::new(64.0, 1.0, 192.0));
    sim.run(800, &[player, camera]);
    // Distances to the content boxes: column n starts at 128·n m, the
    // anchor stands at 64 m.
    let level = |x| sim.s.level(&ws("w"), cell(x, 1));
    assert_eq!(level(0), Level::Active);
    assert_eq!(level(2), Level::Active); // 192 m: R0 load radius, inclusive
    assert_eq!(level(3), Level::Near); // 320 m
    assert_eq!(level(6), Level::Near); // 704 m
    assert_eq!(level(7), Level::Far); // 832 m: inside R1's unload band only
    assert_eq!(level(11), Level::Far); // 1344 m
    assert_eq!(sim.s.requests().count(), 0);
    assert_eq!(sim.s.worldspace_state(&ws("w")), WorldspaceState::Prefetched);
    sim.s.set_active_worldspace(Some(ws("w")));
    assert_eq!(sim.s.worldspace_state(&ws("w")), WorldspaceState::Active);
    for c in &sim.log {
        assert!(c.to == c.from.up().unwrap() || c.to == c.from.down().unwrap());
    }
    // Per cell, each change starts where the last ended.
    for x in 0..12 {
        let mut level = Level::Unloaded;
        for c in sim.log.iter().filter(|c| c.cell == cell(x, 1)) {
            assert_eq!(c.from, level);
            level = c.to;
        }
    }
}

/// The processing-range test: a box is covered only when every cell it
/// touches is at the level (R0 for actors: streaming.md "Processing range").
#[test]
fn covers_needs_every_touched_cell_at_the_level() {
    let mut sim = Sim::new(SchedulerConfig::default(), vec![grid_world("w", 12, 3, 0)]);
    let player = anchor(0, AnchorRole::Gameplay, "w", DVec3::new(64.0, 1.0, 192.0));
    let camera = anchor(1, AnchorRole::Visual, "w", DVec3::new(64.0, 1.0, 192.0));
    sim.run(800, &[player, camera]);
    let at = |x: f64| WorldPos::from_f64(DVec3::new(x, 1.0, 192.0));
    let s = &sim.s;
    // Column 2 (256..384 m) is Active, column 3 Near (see the test above).
    assert!(s.covers(&ws("w"), at(64.0), 2.0, Level::Active));
    assert!(s.covers(&ws("w"), at(320.0), 2.0, Level::Active));
    assert!(!s.covers(&ws("w"), at(448.0), 2.0, Level::Active));
    // A margin reaching into column 3 fails for R0, passes for R1.
    assert!(!s.covers(&ws("w"), at(320.0), 128.0, Level::Active));
    assert!(s.covers(&ws("w"), at(320.0), 128.0, Level::Near));
    // Outside the content and unknown worldspaces: never covered.
    assert!(!s.covers(&ws("w"), at(-500.0), 2.0, Level::Far));
    assert!(!s.covers(&ws("nowhere"), at(64.0), 2.0, Level::Far));
}

/// x position of a scripted oscillation around `center`.
fn wobble(center: f64, amp: f64, t: f64) -> (f64, f64) {
    let w = std::f64::consts::TAU / 3.0;
    (center + amp * (w * t).sin(), amp * w * (w * t).cos())
}

#[test]
fn ring_boundaries_do_not_thrash() {
    let mut sim = Sim::new(SchedulerConfig::default(), vec![grid_world("w", 10, 1, 0)]);
    let target = cell(5, 0); // content box starts at x = 640
    let go = |sim: &mut Sim, center: f64, seconds: f64| {
        let start = sim.now;
        while sim.now - start < seconds {
            let (x, v) = wobble(center, 20.0, sim.now - start);
            let mut a = anchor(0, AnchorRole::Gameplay, "w", DVec3::new(x, 1.0, 64.0));
            #[allow(clippy::cast_possible_truncation)]
            {
                a.velocity = Vec3::new(v as f32, 0.0, 0.0);
            }
            sim.frame(&[a]);
        }
    };
    let promotions = |sim: &Sim| sim.count(target, |c| c.to == Level::Active);
    let demotions = |sim: &Sim| sim.count(target, |c| c.from == Level::Active);

    // Wobble ±20 m (with matching velocity) across the R0 load radius (192 m).
    go(&mut sim, 640.0 - 192.0, 12.0);
    assert_eq!((promotions(&sim), demotions(&sim)), (1, 0));
    // One real crossing out of the unload radius (256 m)...
    go(&mut sim, 640.0 - 320.0, 2.0);
    assert_eq!((promotions(&sim), demotions(&sim)), (1, 1));
    // ...then wobble across the unload radius: nothing comes back.
    go(&mut sim, 640.0 - 256.0, 12.0);
    assert_eq!((promotions(&sim), demotions(&sim)), (1, 1));
    assert_eq!(sim.s.level(&ws("w"), target), Level::Unloaded);
    // And one real crossing in.
    go(&mut sim, 640.0 - 100.0, 2.0);
    assert_eq!((promotions(&sim), demotions(&sim)), (2, 1));
}

fn scripted_run() -> (Vec<String>, Vec<CellChange>) {
    let mut sim = Sim::new(SchedulerConfig::default(), vec![grid_world("w", 40, 40, 3), interior("inside")]);
    let link = DoorLink {
        a: LinkEnd {
            worldspace: ws("w"),
            frame: WorldTransform::from_pos(WorldPos::from_f64(DVec3::new(320.0, 1.0, 315.0))),
            arrival: WorldTransform::from_pos(WorldPos::from_f64(DVec3::new(320.0, 1.0, 318.0))),
            door: None,
            extent: Vec2::ZERO,
        },
        b: LinkEnd {
            worldspace: ws("inside"),
            frame: WorldTransform::IDENTITY,
            arrival: WorldTransform::from_pos(WorldPos::from_f64(DVec3::new(0.0, 0.0, 2.0))),
            door: None,
            extent: Vec2::ZERO,
        },
        two_way: true,
    };
    sim.s.set_links(vec![link]);
    for f in 0..1200u32 {
        let t = f64::from(f) * DT;
        // A run across the map toward the door, then a fast flight.
        let speed = if t < 5.0 { 7.0 } else { 60.0 };
        let p = DVec3::new(300.0 + t * speed * 0.8, 1.0, 300.0 + t * speed * 0.6);
        let mut player = anchor(0, AnchorRole::Gameplay, "w", p);
        #[allow(clippy::cast_possible_truncation)]
        {
            player.velocity = Vec3::new((speed * 0.8) as f32, 0.0, (speed * 0.6) as f32);
        }
        let mut camera = anchor(1, AnchorRole::Visual, "w", p + DVec3::new(-3.0, 2.0, -3.0));
        #[allow(clippy::cast_possible_truncation)]
        {
            camera.view_dir = Some(Quat::from_rotation_y(t as f32 * 0.3) * Vec3::NEG_Z);
        }
        sim.frame(&[player, camera]);
    }
    (sim.trace, sim.log)
}

#[test]
fn scheduler_is_deterministic_for_a_scripted_path() {
    let (trace_a, log_a) = scripted_run();
    let (trace_b, log_b) = scripted_run();
    assert!(!log_a.is_empty());
    assert!(log_a.iter().any(|c| c.worldspace == ws("inside")), "the door was prefetched on the way");
    assert_eq!(log_a, log_b);
    assert_eq!(trace_a, trace_b);
}

#[test]
fn budgets_are_never_exceeded_by_more_than_one_item() {
    // Dense content and a fast flight saturate every stage; Sim::frame
    // asserts the budget rule on every stage report.
    let mut sim = Sim::new(SchedulerConfig::default(), vec![grid_world("w", 30, 30, 20)]);
    sim.items_per_stage = 40;
    for f in 0..600u32 {
        let x = 100.0 + f64::from(f) * DT * 60.0 * 4.0;
        let a = anchor(0, AnchorRole::Gameplay, "w", DVec3::new(x, 1.0, 1900.0));
        let c = anchor(1, AnchorRole::Visual, "w", DVec3::new(x, 1.0, 1900.0));
        sim.frame(&[a, c]);
    }
    assert!(sim.saturated > 50, "budgets were exercised ({} saturated stage runs)", sim.saturated);
}

#[test]
fn r0_and_view_direction_set_priority() {
    let mut s = Scheduler::new(SchedulerConfig::default());
    s.add_worldspace(grid_world("w", 21, 21, 0), None);
    let center = DVec3::new(10.0 * 128.0 + 64.0, 1.0, 10.0 * 128.0 + 64.0);
    let player = anchor(0, AnchorRole::Gameplay, "w", center);
    let mut camera = anchor(1, AnchorRole::Visual, "w", center);
    camera.view_dir = Some(Vec3::X);
    let report = s.update(&TickInput { now: 0.0, anchors: &[player, camera] });
    // R0 requests start first.
    let classes: Vec<u8> = report.start.iter().map(|id| s.request(*id).unwrap().class).collect();
    assert!(classes.windows(2).all(|w| w[0] <= w[1]), "{classes:?}");
    assert_eq!(classes[0], 0);
    // At equal distance, the cell ahead outranks the cell behind.
    let prio = |c: IVec3| s.requests().find(|r| r.cell == c).unwrap().priority;
    let (ahead, behind) = (prio(cell(16, 10)), prio(cell(4, 10)));
    assert!(ahead < behind, "{ahead} vs {behind}");
    assert!((behind / ahead - 2.0).abs() < 0.01, "cells behind count double");
}

#[test]
fn velocity_prefetches_along_travel() {
    // The block of cell 3 starts at x = 384; the anchor stands 210 m short
    // of it, just outside the R0 load radius.
    let pos = DVec3::new(384.0 - 210.0, 1.0, 64.0);
    let mut s = Scheduler::new(SchedulerConfig::default());
    s.add_worldspace(grid_world("w", 6, 1, 0), None);
    let still = anchor(0, AnchorRole::Gameplay, "w", pos);
    s.update(&TickInput { now: 0.0, anchors: std::slice::from_ref(&still) });
    assert!(s.requests().all(|r| r.cell != cell(3, 0)));
    let mut moving = still;
    moving.velocity = Vec3::new(60.0, 0.0, 0.0);
    s.update(&TickInput { now: DT, anchors: &[moving] });
    assert!(s.requests().any(|r| r.cell == cell(3, 0)));
}

#[test]
fn leaving_cancels_pending_loads() {
    let mut sim = Sim::new(SchedulerConfig::default(), vec![grid_world("w", 40, 1, 0)]);
    let near = anchor(0, AnchorRole::Visual, "w", DVec3::new(64.0, 1.0, 64.0));
    let first = sim.frame(&[near]);
    assert!(!first.start.is_empty());
    // Teleport 4.5 km away: everything pending near the start is cancelled.
    let away = anchor(0, AnchorRole::Visual, "w", DVec3::new(4600.0, 1.0, 64.0));
    let report = sim.frame(std::slice::from_ref(&away));
    assert!(!report.cancelled.is_empty());
    assert!(report.cancelled.iter().all(|r| r.cell.x <= 10), "columns 0-9 are past the unload radius");
    for r in &first.start {
        assert!(sim.s.request(*r).is_none());
        assert!(!sim.s.fetched(*r, sim.now, 1, 1), "late completions of cancelled requests are ignored");
    }
    sim.run(300, &[away]);
    assert!(sim.s.resident(&ws("w")).all(|(c, _)| c.x > 10));
}

#[test]
fn door_prefetch_with_hysteresis() {
    let link = DoorLink {
        a: LinkEnd {
            worldspace: ws("w"),
            frame: WorldTransform::from_pos(WorldPos::from_f64(DVec3::new(100.0, 1.0, 64.0))),
            arrival: WorldTransform::from_pos(WorldPos::from_f64(DVec3::new(100.0, 1.0, 66.0))),
            door: None,
            extent: Vec2::ZERO,
        },
        b: LinkEnd {
            worldspace: ws("inside"),
            frame: WorldTransform::IDENTITY,
            arrival: WorldTransform::from_pos(WorldPos::from_f64(DVec3::new(0.0, 0.0, 2.0))),
            door: None,
            extent: Vec2::ZERO,
        },
        two_way: true,
    };
    let mut sim = Sim::new(SchedulerConfig::default(), vec![grid_world("w", 4, 1, 0), interior("inside")]);
    sim.s.set_links(vec![link]);
    sim.s.set_active_worldspace(Some(ws("w")));
    let at = |x: f64| anchor(0, AnchorRole::Gameplay, "w", DVec3::new(x, 1.0, 64.0));
    let inside = |sim: &Sim| sim.s.level(&ws("inside"), IVec3::ZERO);

    sim.run(60, &[at(130.0)]);
    assert_eq!(inside(&sim), Level::Unloaded);
    assert_eq!(sim.s.worldspace_state(&ws("inside")), WorldspaceState::Inactive);
    let r = sim.frame(&[at(120.0)]);
    let classes: Vec<u8> = r.start.iter().filter_map(|id| sim.s.request(*id)).filter(|r| r.worldspace == ws("inside")).map(|r| r.class).collect();
    assert_eq!(classes, vec![1], "prefetch ranks just below R0");
    sim.run(120, &[at(120.0)]);
    assert_eq!(inside(&sim), Level::Active);
    assert_eq!(sim.s.worldspace_state(&ws("inside")), WorldspaceState::Prefetched);
    // Inside the 32 m keep radius it stays...
    sim.run(120, &[at(128.0)]);
    assert_eq!(inside(&sim), Level::Active);
    // ...beyond it, it streams out.
    sim.run(120, &[at(140.0)]);
    assert_eq!(inside(&sim), Level::Unloaded);
    assert_eq!(sim.s.worldspace_state(&ws("inside")), WorldspaceState::Inactive);
    assert_eq!(sim.count(IVec3::ZERO, |c| c.worldspace == ws("inside") && c.to == Level::Active), 1);
}

#[test]
fn a_worldspace_left_lingers_near_the_way_back() {
    // Door at the interior's origin leading out to x = 100 in "w".
    let link = DoorLink {
        a: LinkEnd {
            worldspace: ws("w"),
            frame: WorldTransform::from_pos(WorldPos::from_f64(DVec3::new(100.0, 1.0, 64.0))),
            arrival: WorldTransform::from_pos(WorldPos::from_f64(DVec3::new(100.0, 1.0, 66.0))),
            door: None,
            extent: Vec2::ZERO,
        },
        b: LinkEnd {
            worldspace: ws("inside"),
            frame: WorldTransform::IDENTITY,
            arrival: WorldTransform::from_pos(WorldPos::from_f64(DVec3::new(0.0, 0.0, 2.0))),
            door: None,
            extent: Vec2::ZERO,
        },
        two_way: true,
    };
    let mut sim = Sim::new(SchedulerConfig::default(), vec![grid_world("w", 12, 1, 0), interior("inside")]);
    sim.s.set_links(vec![link.clone()]);
    let out = DVec3::new(100.0, 1.0, 64.0);
    let both = |world: &str, p: DVec3| [anchor(0, AnchorRole::Gameplay, world, p), anchor(1, AnchorRole::Visual, world, p)];
    sim.s.set_active_worldspace(Some(ws("w")));
    sim.run(300, &both("w", out));
    let resident = |sim: &Sim| sim.s.resident(&ws("w")).count();
    let before = resident(&sim);
    assert!(before > 4, "{before}");
    // Through the door: the exterior stays resident, hidden, while the
    // player stands near the door back, long after `linger_secs`...
    sim.s.set_active_worldspace(Some(ws("inside")));
    sim.s.begin_burst();
    sim.run(120 * 8, &both("inside", DVec3::new(0.0, 0.0, 2.0)));
    assert_eq!(resident(&sim), before);
    assert_eq!(sim.s.worldspace_state(&ws("w")), WorldspaceState::Prefetched);
    assert!(!sim.s.in_burst(), "the hold waits for the active worldspace only");
    // ...and streams out once they walk away and the linger time is over.
    sim.run(120 * 6, &both("inside", DVec3::new(0.0, 0.0, 60.0)));
    assert_eq!(resident(&sim), 0);

    // With lingering off, leaving drops everything but the door prefetch.
    let config = SchedulerConfig { linger_secs: 0.0, ..SchedulerConfig::default() };
    let mut sim = Sim::new(config, vec![grid_world("w", 12, 1, 0), interior("inside")]);
    sim.s.set_links(vec![link]);
    sim.run(300, &both("w", out));
    sim.run(600, &both("inside", DVec3::new(0.0, 0.0, 2.0)));
    assert!(resident(&sim) < before, "{} of {before}", resident(&sim));
}

#[test]
fn missing_worldspaces_are_reported() {
    let mut s = Scheduler::new(SchedulerConfig::default());
    let a = anchor(0, AnchorRole::Gameplay, "nowhere", DVec3::ZERO);
    let r = s.update(&TickInput { now: 0.0, anchors: &[a.clone(), a] });
    assert_eq!(r.missing_worldspaces, vec![ws("nowhere")]);
}

#[test]
fn burst_mode_lifts_budgets_until_near_rings_are_resident() {
    let mut sim = Sim::new(SchedulerConfig::default(), vec![grid_world("w", 20, 20, 5)]);
    let p = DVec3::new(1280.0, 1.0, 1280.0);
    let anchors = [anchor(0, AnchorRole::Gameplay, "w", p), anchor(1, AnchorRole::Visual, "w", p)];
    sim.s.begin_burst();
    let mut frames = 0;
    loop {
        let r = sim.frame(&anchors);
        frames += 1;
        if r.burst_ended {
            break;
        }
        assert!(frames < 2000, "burst never ended");
    }
    assert!(!sim.s.in_burst());
    // Everything within R1 is resident at Near or better.
    let near: Vec<_> = sim.s.resident(&ws("w")).filter(|(_, l)| *l >= Level::Near).collect();
    assert!(near.len() > 100, "{}", near.len());
    assert!(sim.s.requests().all(|r| r.to < Level::Near || r.kind == RequestKind::Unload));
    let report = sim.s.run_stage(Stage::Spawn, &mut |_| StageStep { cost: 0.0, done: true });
    assert_eq!(report.limit, 1.0);
}

/// Scheduler tick cost at scale: ~100k placements in ~5k cells, a player and
/// a camera flying at 60 m/s. Run with
/// `cargo test -p sf_world --release -- --ignored --nocapture tick_cost`.
#[test]
#[ignore = "perf measurement; run in release"]
fn tick_cost() {
    let t = Instant::now();
    let dir = grid_world("w", 71, 71, 19);
    let built = t.elapsed();
    let placements: u32 = dir.cells.iter().map(|c| c.placements).sum();
    let mut sim = Sim::new(SchedulerConfig::default(), vec![dir]);
    let mut update = std::time::Duration::ZERO;
    let mut worst = std::time::Duration::ZERO;
    let frames = 2000u32;
    for f in 0..frames {
        let x = 300.0 + f64::from(f) * DT * 60.0;
        let p = DVec3::new(x, 1.0, x * 0.7 + 500.0);
        let mut player = anchor(0, AnchorRole::Gameplay, "w", p);
        player.velocity = Vec3::new(60.0, 0.0, 42.0);
        let mut camera = anchor(1, AnchorRole::Visual, "w", p);
        camera.view_dir = Some(Vec3::new(1.0, 0.0, 0.7).normalize());
        let anchors = [player, camera];
        let t = Instant::now();
        let report = sim.s.update(&TickInput { now: sim.now, anchors: &anchors });
        let dt = t.elapsed();
        update += dt;
        worst = worst.max(dt);
        // Hand the rest of the frame to the simulated engine.
        sim.frame_after_update(report);
    }
    let resident = sim.s.resident(&ws("w")).count();
    println!(
        "index build: {built:?} for {placements} placements in {} cells; tick: {:.1} µs mean, {:.1} µs worst over {frames} frames; {resident} cells resident",
        5041,
        update.as_secs_f64() * 1e6 / f64::from(frames),
        worst.as_secs_f64() * 1e6,
    );
}

impl Sim {
    /// [`Sim::frame`] minus the update (perf test times the update alone).
    fn frame_after_update(&mut self, report: TickReport) {
        for id in &report.start {
            let stage = self.s.request(*id).unwrap().stage;
            self.pending.push((*id, stage, self.frame + 3));
        }
        let frame = self.frame;
        let due: Vec<_> = self.pending.iter().filter(|p| p.2 <= frame).copied().collect();
        self.pending.retain(|p| p.2 > frame);
        for (id, stage, _) in due {
            if stage == Stage::Fetch {
                self.s.fetched(id, self.now, 4, 3 << 20);
            } else {
                self.s.assets_loaded(id, self.now);
            }
        }
        for stage in MAIN_STAGES {
            let items = self.items_per_stage;
            self.s.run_stage(stage, &mut |req| StageStep { cost: 0.1, done: req.progress + 1 >= items });
        }
        self.frame += 1;
        self.now += DT;
    }
}

#[test]
fn far_ring_by_page_loads_whole_pages() {
    let mut config = SchedulerConfig::default();
    config.rings.near = RingRadii::new(100.0, 120.0);
    config.rings.far = RingRadii::new(500.0, 600.0);
    let camera = anchor(1, AnchorRole::Visual, "w", DVec3::new(64.0, 1.0, 192.0));
    let levels = |far_by_page: bool| -> Vec<Level> {
        let mut sim = Sim::new(SchedulerConfig { far_by_page, ..config }, vec![grid_world("w", 24, 3, 0)]);
        sim.run(600, std::slice::from_ref(&camera));
        assert_eq!(sim.s.requests().count(), 0);
        (0..24).map(|x| sim.s.level(&ws("w"), cell(x, 1))).collect()
    };
    // Columns 0..=6 are cells 1..=7: page 0. Column x starts at 128·x m.
    let by_cell = levels(false);
    let by_page = levels(true);
    assert!(by_cell[0..=1].iter().all(|l| *l == Level::Near), "{by_cell:?}");
    assert!(by_cell[2..=4].iter().all(|l| *l == Level::Far), "{by_cell:?}");
    assert!(by_cell[5..].iter().all(|l| *l == Level::Unloaded), "{by_cell:?}");
    // Page 0 is in reach as a whole (distance 0); page 1 starts 832 m away.
    assert!(by_page[2..=6].iter().all(|l| *l == Level::Far), "{by_page:?}");
    assert!(by_page[7..].iter().all(|l| *l == Level::Unloaded), "{by_page:?}");
}
