//! Look-at on the shipped humanoid graph (AN8): the `LookAt` op spreads the host's look
//! target over chest, upper chest, neck and head. Each test runs a looking sim beside a free
//! one (no target, same clock): standing idle, the differences are the op's. Skipped without
//! the install. `cargo test -p mw_graphs --test it look_at:: -- --nocapture` prints the measurements.

use glam::{Quat, Vec3, Vec4};
use mw_anim::{Animation, rig};
use mw_graphs::sim::Sim;
use mw_graphs::{AnimSet, HUMANOID_SETS, Template};
use mw_install::MorrowindInstall;
use sf_vfs::{Vfs, VfsPath};

const DT: f32 = 1.0 / 60.0;

/// The chain as shipped (`build.rs` "Look-at"): joint, limit in degrees.
const CHAIN: [(&str, f32); 4] = [("Bip01 Spine1", 8.0), ("Bip01 Spine2", 10.0), ("Bip01 Neck", 30.0), ("Bip01 Head", 60.0)];

/// The shipped humanoid graph on `base_anim` with the Morrowind clips only (the gait set's
/// library clips are not needed: without them the set drops, as in a game without the
/// library); `None` (skip) without the install.
fn humanoid_sim() -> Option<Sim> {
    humanoid_sim_of(0)
}

fn humanoid_sim_of(index: usize) -> Option<Sim> {
    let Ok((install, _)) = MorrowindInstall::detect(None) else {
        eprintln!("no Morrowind install detected; skipping");
        return None;
    };
    let vfs = mw_vfs::mount_install(&install, Vfs::builder()).unwrap().vfs;
    let read = |path: &str| vfs.read(&VfsPath::new(path).ok()?).ok();
    let set = &HUMANOID_SETS[index];
    let skeleton = rig::skeleton(&mw_nif::load(&read(set.skeleton).unwrap()).unwrap()).unwrap();
    let anims = set
        .animations
        .iter()
        .filter_map(|a| Some(((*a).to_owned(), Animation::from_file(a, &read(a)?).unwrap())))
        .filter(|(_, a)| !a.text_keys.is_empty())
        .collect();
    let mut set = AnimSet::new(set.name, skeleton, anims).with_humanoid_rig();
    let v = mw_graphs::validate(&mut set, Template::Humanoid).unwrap();
    let clips = set.binding_clips(&v.graph).unwrap();
    Some(Sim::new(v.graph, set.skeleton.clone(), clips).unwrap())
}

fn joint(sim: &Sim, name: &str) -> usize {
    sim.skeleton.find(name).unwrap_or_else(|| panic!("no joint {name}"))
}

/// The head's aim axis in its frame (the graph's `forward`).
fn forward() -> Vec3 {
    Vec3::new(0.1175, 0.0, -0.9931).normalize()
}

/// A looking and a free sim, standing idle for `settle` seconds.
fn pair(settle: f32) -> Option<(Sim, Sim)> {
    let (mut look, mut free) = (humanoid_sim()?, humanoid_sim()?);
    look.run(settle, DT);
    free.run(settle, DT);
    Some((look, free))
}

fn head_aim(sim: &Sim) -> Vec3 {
    sim.out.model.rotation(joint(sim, "Bip01 Head")) * forward()
}

/// A point `distance` m from the head pivot at its height, `yaw` degrees to the left (+X).
fn around_head(sim: &Sim, yaw: f32, distance: f32) -> Vec3 {
    let head = sim.out.model.translation(joint(sim, "Bip01 Head"));
    head + distance * Vec3::new(yaw.to_radians().sin(), 0.0, yaw.to_radians().cos())
}

/// Yaw of a direction in degrees, + to the left (+X), 0 ahead (+Z).
fn yaw(v: Vec3) -> f32 {
    v.x.atan2(v.z).to_degrees()
}

/// Steps both sims one frame.
fn step(look: &mut Sim, free: &mut Sim, target: Option<Vec3>) {
    look.look_target = target;
    look.run(DT, DT);
    free.run(DT, DT);
}

/// Largest joint position (m) or rotation (quaternion component) difference between the two
/// model poses. Components, not `angle_between`: its `acos` turns a last-bit difference into
/// 7e-4 rad.
fn pose_gap(a: &Sim, b: &Sim) -> f32 {
    let (ma, mb) = (&a.out.model, &b.out.model);
    (0..a.skeleton.len())
        .map(|j| {
            let (qa, qb) = (ma.rotation(j), mb.rotation(j));
            let qb = if qa.dot(qb) < 0.0 { -qb } else { qb };
            ma.translation(j).distance(mb.translation(j)).max((Vec4::from(qa) - Vec4::from(qb)).abs().max_element())
        })
        .fold(0.0, f32::max)
}

/// The head's level gaze, measured: the character's forward (+Z) in `Bip01 Head`'s frame,
/// standing idle, per humanoid skeleton (the shipped `forward` is the male/female one).
#[test]
#[ignore = "measurement: head axis"]
fn head_axis() {
    for (index, set) in HUMANOID_SETS.iter().enumerate() {
        let Some(mut sim) = humanoid_sim_of(index) else { return };
        sim.run(1.0, DT);
        let r: Quat = sim.out.model.rotation(joint(&sim, "Bip01 Head"));
        println!("{}: character forward in the head's frame {:?}", set.name, r.inverse() * Vec3::Z);
    }
}

/// A target 2 m straight ahead at the head pivot's height (the aim's origin): the head
/// barely moves, the idle's own sway aside.
#[test]
fn a_target_ahead_barely_moves_the_head() {
    let Some((mut look, mut free)) = pair(0.5) else { return };
    let target = around_head(&free, 0.0, 2.0);
    let head = joint(&free, "Bip01 Head");
    let mut worst = 0f32;
    for frame in 0..120 {
        step(&mut look, &mut free, Some(target));
        if frame >= 30 {
            worst = worst.max(look.out.model.rotation(head).angle_between(free.out.model.rotation(head)).to_degrees());
        }
    }
    println!("ahead: head off the animated pose by {worst:.3} deg at most");
    assert!(worst < 1.0, "the head moved {worst} deg for a target straight ahead");
}

/// A target 60° to the left: the head turns to it, each chain joint within its limit, the
/// head's aim within the 80° total of the animated one.
#[test]
fn a_target_to_the_left_turns_the_head_within_the_limits() {
    let Some((mut look, mut free)) = pair(0.5) else { return };
    let target = around_head(&free, 60.0, 2.0);
    for _ in 0..90 {
        step(&mut look, &mut free, Some(target));
    }
    let head = joint(&look, "Bip01 Head");
    let (aim, animated) = (head_aim(&look), head_aim(&free));
    let want = (target - look.out.model.translation(head)).normalize();
    let shares: Vec<String> = CHAIN
        .iter()
        .map(|&(name, limit)| {
            let j = joint(&look, name);
            let added = look.out.pose.get(j).rotation.angle_between(free.out.pose.get(j).rotation).to_degrees();
            assert!(added <= limit + 0.01, "{name} turned {added} deg, limit {limit}");
            format!("{name} {added:.1}")
        })
        .collect();
    let (total, miss) = (aim.angle_between(animated).to_degrees(), aim.angle_between(want).to_degrees());
    println!("60 deg left: head yaw {:.1} (animated {:.1}), total {total:.1}, miss {miss:.2}; {}", yaw(aim), yaw(animated), shares.join(", "));
    assert!(yaw(aim) - yaw(animated) > 50.0, "the head turned {} deg toward a target 60 deg left", yaw(aim) - yaw(animated));
    assert!(total <= 80.01, "total {total} deg past the 80 deg limit");
    assert!(miss < 3.0, "the head aims {miss} deg off a reachable target");
}

/// A test phase: label, frames, target yaw (degrees, + left) per frame.
type Phase<'a> = (&'a str, usize, &'a dyn Fn(usize) -> f32);

/// Behind the character: a target held at 150° left after a look to the left, one jittering
/// across straight behind (±179°), and one sweeping from 60° left behind the back to 60°
/// right. The head never jumps from one side to the other between frames (sanity, printed).
#[test]
fn a_target_behind_never_flips_the_head() {
    let Some((mut look, mut free)) = pair(0.5) else { return };
    let mut last = yaw(head_aim(&look)) - yaw(head_aim(&free));
    let mut report = Vec::new();
    let mut worst = 0f32;
    let phases: [Phase; 5] = [
        ("60 left", 60, &|_| 60.0),
        ("150 left", 120, &|_| 150.0),
        ("+-179 jitter", 120, &|f| if f % 2 == 0 { 179.0 } else { -179.0 }),
        ("60 left again", 60, &|_| 60.0),
        #[allow(clippy::cast_precision_loss)]
        ("sweep 60 left, behind, 60 right in 3 s", 240, &|f| 60.0 + 240.0 * (f as f32 / 180.0).min(1.0)),
    ];
    for (label, frames, at) in phases {
        let mut phase = 0f32;
        for frame in 0..frames {
            let target = around_head(&free, at(frame), 2.0);
            step(&mut look, &mut free, Some(target));
            let now = yaw(head_aim(&look)) - yaw(head_aim(&free));
            phase = phase.max((now - last).abs());
            last = now;
        }
        report.push(format!("{label}: largest frame-to-frame head yaw change {phase:.2} deg, ends at {last:.1}"));
        worst = worst.max(phase);
    }
    println!("{}", report.join("\n"));
    assert!(worst < 10.0, "the head jumped {worst} deg in a frame");
}

/// The target cleared after a look: the weight springs back to 0 and the pose is the free
/// one again, exactly (the op is skipped).
#[test]
fn no_target_returns_to_the_animated_pose() {
    let Some((mut look, mut free)) = pair(0.5) else { return };
    let target = around_head(&free, 60.0, 2.0);
    for _ in 0..60 {
        step(&mut look, &mut free, Some(target));
    }
    let during = pose_gap(&look, &free);
    for _ in 0..120 {
        step(&mut look, &mut free, None);
    }
    let after = pose_gap(&look, &free);
    println!("pose gap: {during:.3} while looking, {after:.2e} 2 s after the target cleared");
    assert!(during > 0.1, "the look did nothing ({during})");
    assert!(after < 1e-4, "the pose is {after} off the free one after the look");
}
