//! Based movement (IN11a): a character stands on a moving base (a lift, a platform, a cart
//! bed), rides its motion and keeps its velocity on leaving it (UE `MovementBase` and
//! "impart base velocity", Jolt `CharacterVirtual` ground velocity). The base is a plain
//! kinematic platform the fake probe moves: it advances first each tick, then the mover
//! steps (the base ticks before the characters on it, as in UE and Mover 2.0), and every
//! down cast that hits it reports its point velocity and yaw rate as `sf_physics` does.

#![allow(clippy::cast_sign_loss)]

use super::*;

/// A square kinematic platform: top at `centre.y`, `half` meters each side of its centre
/// in its own frame, moving at `vel` and turning at `w` rad/s about the vertical through
/// its centre. Off it, static ground at `floor` (if any).
struct Platform {
    centre: Vec3,
    vel: Vec3,
    w: f32,
    angle: f32,
    half: f32,
    floor: Option<f32>,
    /// The centre before the last `advance` (contacts read the tick's motion from it).
    prev: Vec3,
    /// A wall fixed on the platform (IN11c1): a box in its frame, not turning with it
    /// (walls are only put on platforms that don't turn).
    wall: Option<Aabb>,
    /// A static wall in the world.
    post: Option<Aabb>,
}

/// An axis-aligned box.
#[derive(Clone, Copy)]
struct Aabb {
    min: Vec3,
    max: Vec3,
}

impl Aabb {
    fn offset(self, by: Vec3) -> Self {
        Self { min: self.min + by, max: self.max + by }
    }

    /// The first touch of an upright capsule (centre `c`, radius `r`, segment half height
    /// `hh`) swept along horizontal unit `dir` within `max`: (distance, outward normal). The
    /// box is grown by `r` with square corners (head-on tests only). Starting inside, it
    /// is a hit at 0 only when moving into the box (a cast leaving an overlap passes, as
    /// parry's with `stop_at_penetration` off).
    fn sweep(&self, c: Vec3, r: f32, hh: f32, dir: Vec3, max: f32) -> Option<(f32, Vec3)> {
        if c.y + hh + r < self.min.y || c.y - hh - r > self.max.y {
            return None;
        }
        let (lo, hi) = (self.min - r, self.max + r);
        if (lo.x..=hi.x).contains(&c.x) && (lo.z..=hi.z).contains(&c.z) {
            let faces = [(c.x - lo.x, Vec3::NEG_X), (hi.x - c.x, Vec3::X), (c.z - lo.z, Vec3::NEG_Z), (hi.z - c.z, Vec3::Z)];
            let n = faces.iter().min_by(|a, b| a.0.total_cmp(&b.0)).unwrap().1;
            return (dir.dot(n) < 0.0).then_some((0.0, n));
        }
        let (mut enter, mut exit, mut n) = (f32::NEG_INFINITY, f32::INFINITY, Vec3::ZERO);
        for (o, d, l, h, axis) in [(c.x, dir.x, lo.x, hi.x, Vec3::X), (c.z, dir.z, lo.z, hi.z, Vec3::Z)] {
            if d.abs() < 1e-9 {
                if !(l..=h).contains(&o) {
                    return None;
                }
                continue;
            }
            let (t0, t1) = ((l - o) / d, (h - o) / d);
            let (near, far) = (t0.min(t1), t0.max(t1));
            if near > enter {
                enter = near;
                n = -axis * d.signum();
            }
            exit = exit.min(far);
        }
        (enter <= exit && (0.0..=max).contains(&enter)).then_some((enter, n))
    }
}

impl Platform {
    fn moving(vel: Vec3) -> Self {
        Self { centre: Vec3::ZERO, vel, w: 0.0, angle: 0.0, half: 50.0, floor: None, prev: Vec3::ZERO, wall: None, post: None }
    }

    fn turning(w: f32) -> Self {
        Self { w, ..Self::moving(Vec3::ZERO) }
    }

    fn advance(&mut self, dt: f32) {
        self.prev = self.centre;
        self.centre += self.vel * dt;
        self.angle += self.w * dt;
    }

    /// The wall sweeps (horizontal capsule casts): the post (static) and the platform's
    /// wall where it is now (moving with it).
    fn sweep(&self, q: &ProbeQuery) -> Option<GroundHit> {
        let c = xyz(q.origin);
        let post = self.post.and_then(|b| b.sweep(c, q.radius, q.half_height, q.dir, q.max)).map(|(t, n)| (t, n, None));
        let wall = self.wall.and_then(|b| b.offset(self.centre).sweep(c, q.radius, q.half_height, q.dir, q.max)).map(|(t, n)| (t, n, Some(self.vel)));
        let (t, n, v) = [post, wall].into_iter().flatten().min_by(|a, b| a.0.total_cmp(&b.0))?;
        let hit = GroundHit::new(WorldPos::ORIGIN.offset(c + q.dir * t - n * q.radius), n, t, SurfaceId::DEFAULT);
        Some(v.map_or(hit, |v| hit.on_body(GroundBody(7), v, Vec3::ZERO)))
    }

    /// `x` in the platform's frame (its centre, its turn).
    fn local(&self, x: WorldPos) -> Vec3 {
        Quat::from_rotation_y(-self.angle) * (xyz(x) - self.centre)
    }
}

impl GroundProbe for Platform {
    fn probe(&self, queries: &[ProbeQuery], out: &mut [Option<GroundHit>]) {
        for (q, o) in queries.iter().zip(out.iter_mut()) {
            *o = None;
            if !q.is_down() {
                if q.radius > 0.0 && q.dir.y.abs() < 1e-6 {
                    *o = self.sweep(q);
                }
                continue;
            }
            let at = xyz(q.origin);
            let l = self.local(q.origin);
            let d = at.y - self.centre.y;
            if l.x.abs() <= self.half && l.z.abs() <= self.half && (0.0..=q.max).contains(&d) {
                let point = Vec3::new(at.x, self.centre.y, at.z);
                let r = Vec3::new(point.x - self.centre.x, 0.0, point.z - self.centre.z);
                let v = self.vel + Vec3::Y.cross(r) * self.w;
                let hit = GroundHit::new(WorldPos::ORIGIN.offset(point), Vec3::Y, d, SurfaceId(2));
                *o = Some(hit.on_body(GroundBody(7), v, Vec3::Y * self.w));
            } else if let Some(f) = self.floor
                && (0.0..=q.max).contains(&(at.y - f))
            {
                *o = Some(GroundHit::new(WorldPos::ORIGIN.offset(Vec3::new(at.x, f, at.z)), Vec3::Y, at.y - f, SurfaceId::DEFAULT));
            }
        }
    }

    /// The platform's wall, as `sf_physics` reports a moving body: the face it came in by
    /// (the side of its pose before this tick's motion the capsule was on), the signed
    /// separation from that face where the wall is now.
    fn contacts(&self, queries: &[ProbeQuery], out: &mut [Option<GroundHit>]) {
        for (q, o) in queries.iter().zip(out.iter_mut()) {
            *o = None;
            let Some(wall) = self.wall else { continue };
            let (c, r) = (xyz(q.origin), q.radius);
            let (was, now) = (wall.offset(self.prev), wall.offset(self.centre));
            if c.y + q.half_height + r < now.min.y || c.y - q.half_height - r > now.max.y {
                continue;
            }
            let gaps = [(c.x - was.max.x, Vec3::X), (was.min.x - c.x, Vec3::NEG_X), (c.z - was.max.z, Vec3::Z), (was.min.z - c.z, Vec3::NEG_Z)];
            let n = gaps.iter().max_by(|a, b| a.0.total_cmp(&b.0)).unwrap().1;
            // Across the face: the capsule must overlap it (grown by the radius).
            let side = Vec3::new(n.z.abs(), 0.0, n.x.abs());
            if c.dot(side) < now.min.dot(side) - r || c.dot(side) > now.max.dot(side) + r {
                continue;
            }
            let face = if n.x + n.z > 0.0 { now.max.dot(n.abs()) } else { now.min.dot(n.abs()) };
            let sep = (c.dot(n.abs()) - face) * (n.x + n.z) - r;
            if sep <= q.max {
                let hit = GroundHit::new(WorldPos::ORIGIN.offset(c - n * (sep + r)), n, sep, SurfaceId::DEFAULT);
                *o = Some(hit.on_body(GroundBody(7), self.vel, Vec3::ZERO));
            }
        }
    }
}

/// A character standing on `base` at platform-local `spot` facing `yaw`, grounded on it
/// (one step with the base held still).
fn stand_on(base: &Platform, spot: Vec3, yaw: f32, params: &MoverParams, mover: &mut KinematicMover, dt: f32) -> Locomotion {
    let at = base.centre + Quat::from_rotation_y(base.angle) * spot;
    let mut loco = Locomotion::new(WorldPos::ORIGIN.offset(at), yaw);
    mover.step(MoveInput { loco: &mut loco, intent: LocomotionIntent::IDLE, params, root: RootMotion::default() }, base, dt);
    assert!(loco.grounded.is_some_and(|g| g.body == Some(GroundBody(7))), "not on the base: {:?}", loco.grounded);
    loco
}

/// One tick: the base moves, then the character.
fn tick(base: &mut Platform, loco: &mut Locomotion, intent: LocomotionIntent, params: &MoverParams, mover: &mut KinematicMover, dt: f32) {
    base.advance(dt);
    mover.step(MoveInput { loco, intent, params, root: RootMotion::default() }, &*base, dt);
}

/// An idle character on `base` for `seconds`: its platform-local drift (m), its yaw drift
/// relative to the base's turn (rad), and the largest per-tick own motion (`moved`, m and
/// rad) the animation graph sees.
fn idle_ride(mut base: Platform, spot: Vec3, hz: f32, seconds: f32) -> (f32, f32, f32, f32) {
    let (params, dt) = (MoverParams::default(), 1.0 / hz);
    let mut mover = KinematicMover::new();
    let yaw0 = 0.4;
    let mut loco = stand_on(&base, spot, yaw0, &params, &mut mover, dt);
    let start = base.local(loco.position);
    let (mut own, mut own_yaw) = (0.0_f32, 0.0_f32);
    for _ in 0..(seconds * hz).round() as usize {
        tick(&mut base, &mut loco, LocomotionIntent::IDLE, &params, &mut mover, dt);
        assert!(loco.grounded.is_some(), "fell off the base at {:?}", xyz(loco.position));
        own = own.max(loco.moved.translation.length());
        own_yaw = own_yaw.max(loco.moved.yaw.abs());
    }
    let drift = (base.local(loco.position) - start).length();
    let yaw = wrap_angle(loco.motion.yaw - (yaw0 + base.angle)).abs();
    (drift, yaw, own, own_yaw)
}

/// Jumps (4 m/s) after 0.5 s idle on `base`: the platform-local landing spot's horizontal
/// offset from the take-off spot (m), and the airborne time (s).
fn jump_ride(mut base: Platform, hz: f32) -> (f32, f32) {
    let (params, dt) = (MoverParams::default(), 1.0 / hz);
    let mut mover = KinematicMover::new();
    let mut loco = stand_on(&base, Vec3::new(1.0, 0.0, -2.0), 0.0, &params, &mut mover, dt);
    for _ in 0..(0.5 * hz).round() as usize {
        tick(&mut base, &mut loco, LocomotionIntent::IDLE, &params, &mut mover, dt);
    }
    let take_off = base.local(loco.position);
    loco.launch(4.0);
    let mut air = 0;
    loop {
        tick(&mut base, &mut loco, LocomotionIntent::IDLE, &params, &mut mover, dt);
        air += 1;
        if loco.grounded.is_some() {
            break;
        }
        assert!(air < (3.0 * hz) as usize, "never landed");
    }
    assert!(loco.grounded.is_some_and(|g| g.body == Some(GroundBody(7))), "landed off the base");
    assert_eq!(loco.inherited, Vec3::ZERO, "landing clears the inherited velocity");
    let land = base.local(loco.position);
    (Vec3::new(land.x - take_off.x, 0.0, land.z - take_off.z).length(), air as f32 * dt)
}

const MOVING: Vec3 = Vec3::new(2.0, 0.5, -1.0);

#[test]
fn an_idle_character_keeps_its_spot_on_a_moving_platform() {
    for hz in [30.0, 60.0, 144.0] {
        let (drift, _, own, _) = idle_ride(Platform::moving(MOVING), Vec3::new(1.0, 0.0, -2.0), hz, 5.0);
        eprintln!("moving platform {hz} Hz: drift {drift:.2e} m over 5 s, max own motion {own:.2e} m/tick");
        assert!(drift < 1e-3, "{hz} Hz: drifted {drift} m");
        // The ride is the base's motion, not the character's: the graph sees it standing.
        assert!(own < 1e-4, "{hz} Hz: own motion {own} m/tick");
    }
}

#[test]
fn on_a_turning_disc_the_character_keeps_its_spot_and_turns_with_it() {
    for hz in [30.0, 60.0, 144.0] {
        let (drift, yaw, own, own_yaw) = idle_ride(Platform::turning(1.0), Vec3::new(3.0, 0.0, 0.0), hz, 5.0);
        eprintln!("disc 1 rad/s {hz} Hz: drift {drift:.2e} m, yaw off the disc {yaw:.2e} rad, max own motion {own:.2e} m {own_yaw:.2e} rad/tick");
        assert!(drift < 1e-3, "{hz} Hz: drifted {drift} m");
        assert!(yaw < 1e-3, "{hz} Hz: yaw {yaw} rad off the disc");
        assert!(own < 1e-4 && own_yaw < 1e-4, "{hz} Hz: own motion {own} m, {own_yaw} rad");
    }
}

#[test]
fn a_jump_on_a_moving_platform_lands_on_the_same_spot() {
    let g = MoverParams::default().gravity;
    for hz in [30.0, 60.0, 144.0] {
        let (off, air) = jump_ride(Platform::moving(MOVING), hz);
        eprintln!("jump on moving platform {hz} Hz: lands {off:.2e} m from take-off (platform-local), air {air:.3} s");
        assert!(off < 1e-3, "{hz} Hz: landed {off} m off");
        // Relative to the base it is a plain 4 m/s jump.
        assert!((air - 8.0 / g).abs() < 2.5 / hz, "{hz} Hz: air time {air} s");
    }
}

#[test]
fn walking_off_a_moving_platform_keeps_its_velocity() {
    let hz = 60.0;
    let (params, dt) = (MoverParams::default(), 1.0 / hz);
    let mut mover = KinematicMover::new();
    let mut base = Platform { centre: Vec3::new(0.0, 1.0, 0.0), half: 2.0, floor: Some(-20.0), ..Platform::moving(Vec3::new(2.0, 0.0, 0.0)) };
    let mut loco = stand_on(&base, Vec3::ZERO, 0.0, &params, &mut mover, dt);
    let walk = toward(Vec3::new(0.0, 0.0, 3.0));
    let mut ticks = 0;
    while loco.grounded.is_some() {
        tick(&mut base, &mut loco, walk, &params, &mut mover, dt);
        ticks += 1;
        assert!(ticks < 300, "never walked off");
    }
    assert_eq!(loco.inherited, Vec3::new(2.0, 0.0, 0.0), "the base's velocity is kept");
    let (before, own) = (xyz(loco.position), loco.motion.velocity);
    tick(&mut base, &mut loco, walk, &params, &mut mover, dt);
    assert!(loco.grounded.is_none());
    let v = (xyz(loco.position) - before) / dt;
    eprintln!("first airborne tick: {:.4} m/s along the base (own {:.4}), {:.4} m/s ahead", v.x, own.x, v.z);
    assert!((v.x - 2.0 - own.x).abs() < 0.05, "kept {} m/s of the base's 2", v.x);
    assert!(v.z > 2.0, "the walk carries on: {} m/s", v.z);
}

#[test]
fn based_movement_agrees_across_tick_rates() {
    let ride = |hz| idle_ride(Platform::moving(MOVING), Vec3::new(1.0, 0.0, -2.0), hz, 5.0);
    let disc = |hz| idle_ride(Platform::turning(1.0), Vec3::new(3.0, 0.0, 0.0), hz, 5.0);
    let jump = |hz| jump_ride(Platform::moving(MOVING), hz);
    let (r30, r144) = (ride(30.0), ride(144.0));
    let (d30, d144) = (disc(30.0), disc(144.0));
    let (j30, j144) = (jump(30.0), jump(144.0));
    eprintln!("30 vs 144 Hz: ride drift {:.2e}/{:.2e} m, disc drift {:.2e}/{:.2e} m yaw {:.2e}/{:.2e} rad, jump {:.2e}/{:.2e} m air {:.3}/{:.3} s", r30.0, r144.0, d30.0, d144.0, d30.1, d144.1, j30.0, j144.0, j30.1, j144.1);
    assert!((r30.0 - r144.0).abs() < 1e-3 && (d30.0 - d144.0).abs() < 1e-3 && (d30.1 - d144.1).abs() < 1e-3);
    assert!((j30.0 - j144.0).abs() < 1e-3);
    // Air time is quantized to the tick: within a 30 Hz tick of each other.
    assert!((j30.1 - j144.1).abs() < 1.0 / 30.0 + 1e-4, "air {} vs {} s", j30.1, j144.1);
}

// --- IN11c1: walls on and beside a moving base -------------------------------------------

/// A platform (top at 0, 50 m each side) carries an idle character at 2 m/s toward a static
/// wall (x 3..3.5, its foot 0.1 m above the deck so the platform passes under it), and
/// 0.5 m/s along it. Returns the deepest penetration past the skin (m), the final gap to
/// the wall (m), the character's world speed across and along the wall over the last
/// second (m/s), and how far the platform slid on under it (m).
fn carried_into_a_wall(hz: f32) -> (f32, f32, f32, f32, f32) {
    let (params, dt) = (MoverParams::default(), 1.0 / hz);
    let mut mover = KinematicMover::new();
    let post = Aabb { min: Vec3::new(3.0, 0.1, -50.0), max: Vec3::new(3.5, 3.0, 50.0) };
    let mut base = Platform { post: Some(post), ..Platform::moving(Vec3::new(2.0, 0.0, 0.5)) };
    let mut loco = stand_on(&base, Vec3::ZERO, 0.0, &params, &mut mover, dt);
    let (mut deepest, mut last) = (0.0_f32, xyz(loco.position));
    let ticks = (4.0 * hz).round() as usize;
    for k in 0..ticks {
        tick(&mut base, &mut loco, LocomotionIntent::IDLE, &params, &mut mover, dt);
        assert!(loco.grounded.is_some_and(|g| g.body == Some(GroundBody(7))), "left the base at {:?}", xyz(loco.position));
        let at = xyz(loco.position);
        deepest = deepest.max(at.x + params.radius - post.min.x);
        if k + 1 == ticks - hz as usize {
            last = at;
        }
    }
    let at = xyz(loco.position);
    let v = at - last;
    (deepest, post.min.x - (at.x + params.radius), v.x, v.z, -base.local(loco.position).x)
}

#[test]
fn a_platform_carrying_a_character_into_a_wall_stops_it_there() {
    let skin = MoverParams::default().skin;
    for hz in [30.0, 144.0] {
        let (deepest, gap, across, along, slid) = carried_into_a_wall(hz);
        eprintln!("carried into a wall {hz} Hz: deepest {deepest:.2e} m (skin {skin}), gap {gap:.2e} m, speed across {across:.2e} along {along:.3} m/s, platform slid on {slid:.3} m under it");
        assert!(deepest <= 0.0, "{hz} Hz: {deepest} m into the wall");
        // The skin is kept along the sweep (as `slide` does), so a ride meeting the wall at
        // 14° from head-on stops skin × cos 14° from it.
        let cos = 2.0 / 4.25_f32.sqrt();
        assert!((gap - skin * cos).abs() < 1e-4, "{hz} Hz: stops {gap} m from the wall");
        assert!(across.abs() < 1e-4, "{hz} Hz: still moving {across} m/s into the wall");
        // Collide and slide: the ride along the wall carries on.
        assert!((along - 0.5).abs() < 1e-3, "{hz} Hz: {along} m/s along the wall");
        // 8 m of platform travel less the 2.69 m to the wall slid on under the character.
        assert!((slid - (8.0 - 2.69)).abs() < 0.05, "{hz} Hz: the platform slid {slid} m");
    }
}

/// A platform (top at 0, 1 m each side) with a wall on its front edge (local x 0.8..1,
/// 2 m high) runs at `speed` along +x into a character standing on the static floor 1 m
/// ahead. Returns the deepest overlap of the wall into the capsule (m, negative: never
/// touched), the character's mean speed over the last half second (m/s), the final gap
/// to the wall face (m), and its largest own motion per tick (m).
fn pushed_by_a_wall(speed: f32, hz: f32) -> (f32, f32, f32, f32) {
    let (params, dt) = (MoverParams::default(), 1.0 / hz);
    let mut mover = KinematicMover::new();
    let wall = Aabb { min: Vec3::new(0.8, 0.0, -1.0), max: Vec3::new(1.0, 2.0, 1.0) };
    let mut base = Platform { half: 1.0, floor: Some(0.0), wall: Some(wall), ..Platform::moving(Vec3::new(speed, 0.0, 0.0)) };
    let mut loco = Locomotion::new(WorldPos::ORIGIN.offset(Vec3::new(2.0, 0.0, 0.0)), 0.0);
    mover.step(MoveInput { loco: &mut loco, intent: LocomotionIntent::IDLE, params: &params, root: RootMotion::default() }, &base, dt);
    assert!(loco.grounded.is_some_and(|g| g.body.is_none()), "not on the floor: {:?}", loco.grounded);
    let (mut deepest, mut own, mut half_way) = (f32::NEG_INFINITY, 0.0_f32, xyz(loco.position));
    let ticks = (1.0 * hz).round() as usize;
    for k in 0..ticks {
        tick(&mut base, &mut loco, LocomotionIntent::IDLE, &params, &mut mover, dt);
        let at = xyz(loco.position);
        assert!(loco.grounded.is_some_and(|g| g.body.is_none()) && at.y.abs() < 1e-5, "{speed} m/s {hz} Hz tick {k}: off the floor at {at:?}: {:?}", loco.grounded);
        let face = base.centre.x + wall.max.x;
        assert!(at.x > face, "{speed} m/s {hz} Hz: tunnelled, wall face {face} past the character at {}", at.x);
        deepest = deepest.max(face - (at.x - params.radius));
        own = own.max(loco.moved.translation.length());
        if k + 1 == ticks / 2 {
            half_way = at;
        }
    }
    let at = xyz(loco.position);
    let v = (at.x - half_way.x) / (dt * (ticks - ticks / 2) as f32);
    (deepest, v, at.x - params.radius - (base.centre.x + wall.max.x), own)
}

#[test]
fn a_wall_on_a_moving_platform_pushes_a_character_beside_it() {
    let skin = MoverParams::default().skin;
    for speed in [10.0, 30.0] {
        for hz in [30.0, 144.0] {
            let (deepest, v, gap, own) = pushed_by_a_wall(speed, hz);
            eprintln!("pushed by a {speed} m/s wall {hz} Hz: deepest overlap {deepest:.2e} m, speed {v:.4} m/s, gap {gap:.2e} m (skin {skin}), max own motion {own:.2e} m/tick");
            assert!(deepest <= -skin + 1e-4, "{speed} m/s {hz} Hz: the wall is {deepest} m into the capsule");
            assert!((v - speed).abs() < 1e-3 * speed, "{speed} m/s {hz} Hz: pushed at {v} m/s");
            assert!((gap - skin).abs() < 1e-4, "{speed} m/s {hz} Hz: {gap} m ahead of the wall");
            // Pushed, not walking: the graph sees it standing.
            assert!(own < 1e-4, "{speed} m/s {hz} Hz: own motion {own} m/tick");
        }
    }
}
