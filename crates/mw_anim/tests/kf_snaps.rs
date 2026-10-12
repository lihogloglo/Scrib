//! Snap and rotation-spline audit on the real game data. Ignored by default:
//! `cargo test -p mw_anim --release --test kf_snaps -- --ignored --nocapture`
//! (`AUDIT_FILE=<substr>` limits the files and prints every snap).
//!
//! - *Snaps*: translation segments that jump far in one frame (root-motion
//!   wraps: the dreugh's `Bip01` jumps back meters between two keys a frame
//!   apart). Classified by where they sit relative to the text keys, and
//!   the cut clips' overshoot (how far a cubic swings outside the box its
//!   segment's two keys span) is reported, the thing a snap's smooth
//!   neighbors suffer from.
//! - *TCB rotations*: how far a Kochanek–Bartels quaternion spline (squad)
//!   through the keys departs from the slerp between them, and how sharp
//!   slerp's kinks are at the keys.

use std::collections::BTreeMap;

use glam::{Quat, Vec3};
use mw_install::MorrowindInstall;
use sf_vfs::Vfs;
use tes3::nif::{NiKeyframeData, NiRotKey, NiStream};

use mw_anim::convert::{SNAP_CHORD, SNAP_DURATION, SNAP_SPEED_RATIO};

fn quantiles(v: &mut [f32]) -> String {
    if v.is_empty() {
        return "-".into();
    }
    v.sort_by(f32::total_cmp);
    #[allow(clippy::cast_precision_loss, clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    let q = |p: f32| v[((v.len() - 1) as f32 * p) as usize];
    format!("p50 {:.3} p90 {:.3} p99 {:.3} max {:.3} (n {})", q(0.5), q(0.9), q(0.99), q(1.0), v.len())
}

/// Largest distance of a cubic curve outside the box of each segment's keys.
fn overshoot(curve: &sf_anim::Curve<Vec3>) -> f32 {
    let mut worst = 0.0_f32;
    for i in 1..curve.times.len() {
        let (t0, t1) = (curve.times[i - 1], curve.times[i]);
        let (p0, p1) = (curve.sample(t0), curve.sample(t1));
        for k in 1..16 {
            #[allow(clippy::cast_precision_loss)]
            let v = curve.sample(t0 + (t1 - t0) * k as f32 / 16.0);
            worst = worst.max((v - v.clamp(p0.min(p1), p0.max(p1))).length());
        }
    }
    worst
}

/// Quaternion log as a rotation vector (axis * angle).
fn log(q: Quat) -> Vec3 {
    let q = if q.w < 0.0 { -q } else { q };
    let (axis, angle) = q.to_axis_angle();
    axis * angle
}

fn exp(v: Vec3) -> Quat {
    let angle = v.length();
    if angle < 1e-9 { Quat::IDENTITY } else { Quat::from_axis_angle(v / angle, angle) }
}

/// Kochanek–Bartels quaternion spline (squad with TCB tangents in each
/// key's local log space, spacing-adjusted like the position keys).
struct Squad {
    keys: Vec<(f32, Quat)>,
    /// Inner control quaternions: (`a_i` leaving key `i`, `b_i` arriving at key `i`).
    inner: Vec<(Quat, Quat)>,
}

impl Squad {
    #[allow(clippy::many_single_char_names)]
    fn new(raw: &[(f32, Quat, f32, f32, f32)]) -> Self {
        let mut keys: Vec<(f32, Quat)> = Vec::with_capacity(raw.len());
        for &(t, q, ..) in raw {
            // Shortest path from the previous key, as slerp takes.
            let q = match keys.last() {
                Some(&(_, p)) if p.dot(q) < 0.0 => -q,
                _ => q,
            };
            keys.push((t, q));
        }
        let n = keys.len();
        let inner = (0..n)
            .map(|i| {
                let (t, q) = keys[i];
                let (_, _, ten, con, bias) = raw[i];
                let incoming = if i > 0 { log(keys[i - 1].1.inverse() * q) } else { Vec3::ZERO };
                let outgoing = if i + 1 < n { log(q.inverse() * keys[i + 1].1) } else { Vec3::ZERO };
                let (incoming, outgoing) = match (i > 0, i + 1 < n) {
                    (false, true) => (outgoing, outgoing),
                    (true, false) => (incoming, incoming),
                    _ => (incoming, outgoing),
                };
                let (si, so) = if i > 0 && i + 1 < n {
                    let (dp, dn) = (t - keys[i - 1].0, keys[i + 1].0 - t);
                    if dp + dn > 0.0 { (2.0 * dp / (dp + dn), 2.0 * dn / (dp + dn)) } else { (1.0, 1.0) }
                } else {
                    (1.0, 1.0)
                };
                let t_in = (incoming * ((1.0 - ten) * (1.0 - con) * (1.0 + bias) / 2.0) + outgoing * ((1.0 - ten) * (1.0 + con) * (1.0 - bias) / 2.0)) * si;
                let t_out = (incoming * ((1.0 - ten) * (1.0 + con) * (1.0 + bias) / 2.0) + outgoing * ((1.0 - ten) * (1.0 - con) * (1.0 - bias) / 2.0)) * so;
                // squad's derivative at x = 0 is log(q_i^-1 q_i+1) + 2 log(q_i^-1 a_i),
                // at x = 1 it is log(q_i^-1 q_i+1) - 2 log(q_i+1^-1 b_i+1).
                let a = q * exp((t_out - outgoing) * 0.5);
                let b = q * exp((incoming - t_in) * 0.5);
                (a, b)
            })
            .collect();
        Self { keys, inner }
    }

    fn sample(&self, i: usize, x: f32) -> Quat {
        let (q0, q1) = (self.keys[i].1, self.keys[i + 1].1);
        let (a, b) = (self.inner[i].0, self.inner[i + 1].1);
        q0.slerp(q1, x).slerp(a.slerp(b, x), 2.0 * x * (1.0 - x)).normalize()
    }
}

#[test]
#[ignore = "real install"]
#[allow(clippy::too_many_lines, clippy::cast_precision_loss)]
fn snaps_and_tcb_rotations() {
    let Ok((install, _)) = MorrowindInstall::detect(None) else {
        eprintln!("no Morrowind install detected; skipping");
        return;
    };
    let vfs = mw_vfs::mount_install(&install, Vfs::builder()).unwrap().vfs;
    let only = std::env::var("AUDIT_FILE").ok();
    let mut classes: BTreeMap<String, usize> = BTreeMap::new();
    let (mut snap_list, mut clip_overshoot) = (Vec::new(), Vec::new());
    let (mut squad_dev, mut kink, mut spacing) = (Vec::new(), Vec::new(), Vec::new());
    let mut worst_dev: Vec<(f32, String)> = Vec::new();
    let (mut segment_dev, mut skipped) = (Vec::new(), 0_usize);
    for path in vfs.paths() {
        let name = path.as_str();
        if !matches!(path.extension().map(str::to_ascii_lowercase).as_deref(), Some("kf" | "nif")) || only.as_deref().is_some_and(|o| !name.contains(o)) {
            continue;
        }
        let Ok(bytes) = vfs.read(path) else { continue };
        if !bytes.windows(14).any(|w| w == b"NiKeyframeData") {
            continue;
        }
        // TCB rotations (raw keys).
        // Actor animation only (files with text-key groups): activators spin
        // through 180-degree keys, where slerp and squad take different ways.
        let actor = mw_anim::Animation::from_file(name, &bytes).is_ok_and(|a| !a.groups().is_empty());
        if actor && let Ok(stream) = NiStream::from_bytes(&bytes) {
            for data in stream.objects_of_type::<NiKeyframeData>() {
                let NiRotKey::TCBKey(k) = &data.rotations.keys else { continue };
                if k.len() < 3 {
                    continue;
                }
                let raw: Vec<_> = k.iter().map(|k| (k.time, Quat::from_array(k.value.to_array()), k.tension, k.continuity, k.bias)).collect();
                let squad = Squad::new(&raw);
                let n = squad.keys.len();
                let angle = |i: usize| squad.keys[i].1.angle_between(squad.keys[i + 1].1);
                let rate = |i: usize| angle(i) / (squad.keys[i + 1].0 - squad.keys[i].0).max(1e-4);
                // Rotation snaps (a bone flipping back in one frame) and
                // half-turn segments (no shortest way) are left out: squad
                // through them would need the same curve breaks as positions.
                let wild = |i: usize| {
                    let neighbors = if i > 0 { rate(i - 1) } else { 0.0 }.max(if i + 2 < n { rate(i + 1) } else { 0.0 });
                    angle(i) > 120f32.to_radians() || (squad.keys[i + 1].0 - squad.keys[i].0 <= 0.1 && angle(i) > 10f32.to_radians() && rate(i) >= 4.0 * neighbors)
                };
                let mut dev = 0.0_f32;
                for i in 0..n - 1 {
                    if wild(i) || (i > 0 && wild(i - 1)) || (i + 2 < n && wild(i + 1)) {
                        skipped += 1;
                        continue;
                    }
                    let (q0, q1) = (squad.keys[i].1, squad.keys[i + 1].1);
                    spacing.push(squad.keys[i + 1].0 - squad.keys[i].0);
                    let mut seg = 0.0_f32;
                    for s in 1..16 {
                        let x = s as f32 / 16.0;
                        seg = seg.max(q0.slerp(q1, x).angle_between(squad.sample(i, x)).to_degrees());
                    }
                    segment_dev.push(seg);
                    dev = dev.max(seg);
                }
                for w in squad.keys.windows(3) {
                    let (dp, dn) = (w[1].0 - w[0].0, w[2].0 - w[1].0);
                    if dp > 1e-4 && dn > 1e-4 {
                        // Angular velocity change across the key (deg/s), slerp.
                        let (wi, wo) = (log(w[0].1.inverse() * w[1].1) / dp, log(w[1].1.inverse() * w[2].1) / dn);
                        kink.push((wo - wi).length().to_degrees());
                    }
                }
                squad_dev.push(dev);
                worst_dev.push((dev, name.to_owned()));
            }
        }
        // Snaps and cut overshoot.
        let Ok(anim) = mw_anim::Animation::from_file(name, &bytes) else { continue };
        let groups = anim.groups();
        let marks: Vec<(f32, String)> = groups
            .iter()
            .flat_map(|g| {
                let mut m = vec![(g.start, format!("{} start", g.name)), (g.stop, format!("{} stop", g.name))];
                if let Some((a, b)) = g.loop_range {
                    m.push((g.start + a, format!("{} loop start", g.name)));
                    m.push((g.start + b, format!("{} loop stop", g.name)));
                }
                m
            })
            .collect();
        if only.is_some() {
            eprintln!("{name} marks: {marks:?}");
        }
        for (bone, track) in &anim.tracks {
            let Some(curve) = &track.translation else { continue };
            let keys: Vec<_> = curve.keys().collect();
            if std::env::var("AUDIT_BONE").is_ok_and(|b| b == *bone) {
                for (t, p, i_tan, o_tan) in &keys {
                    eprintln!("    t {t:.4} p {p:.3} in {i_tan:.3} out {o_tan:.3}");
                }
            }
            let speed = |i: usize| (keys[i + 1].1 - keys[i].1).length() / (keys[i + 1].0 - keys[i].0).max(1e-4);
            for i in 0..keys.len().saturating_sub(1) {
                let ((t0, p0, ..), (t1, p1, ..)) = (keys[i], keys[i + 1]);
                let (chord, dt) = ((p1 - p0).length(), t1 - t0);
                let neighbors = if i > 0 { speed(i - 1) } else { 0.0 }.max(if i + 2 < keys.len() { speed(i + 1) } else { 0.0 });
                if dt <= 1e-4 || dt > SNAP_DURATION || chord < SNAP_CHORD || speed(i) < SNAP_SPEED_RATIO * neighbors {
                    continue;
                }
                let near = |t: f32| marks.iter().filter(|m| (m.0 - t).abs() < 2e-3).map(|m| m.1.clone()).collect::<Vec<_>>();
                let (at0, at1) = (near(t0), near(t1));
                let inside = groups.iter().any(|g| t0 >= g.start - 1e-3 && t1 <= g.stop + 1e-3);
                let class = match (at0.is_empty(), at1.is_empty()) {
                    (false, false) => "both keys on group marks",
                    (false, true) => "first key on a group mark",
                    (true, false) => "second key on a group mark",
                    (true, true) if inside => "inside a group, off marks",
                    (true, true) => "between groups",
                };
                *classes.entry(class.into()).or_default() += 1;
                snap_list.push(format!("{name} {bone}: {t0:.4}..{t1:.4} chord {chord:.2} m [{class}] {at0:?} {at1:?}"));
            }
        }
        for g in &groups {
            let clip = anim.source_clip(g);
            let worst = clip.tracks.iter().filter_map(|t| t.translation.as_ref()).map(overshoot).fold(0.0, f32::max);
            clip_overshoot.push((worst, format!("{name} {}", g.name)));
        }
    }
    eprintln!("translation snaps (convert::snap_breaks rule): {} : {classes:#?}", snap_list.len());
    for s in snap_list.iter().take(if only.is_some() { usize::MAX } else { 30 }) {
        eprintln!("  {s}");
    }
    clip_overshoot.sort_by(|a, b| b.0.total_cmp(&a.0));
    let mut v: Vec<f32> = clip_overshoot.iter().map(|c| c.0).collect();
    eprintln!(
        "cut clips: worst translation overshoot (m): {}; clips over 1 cm {}, over 10 cm {}",
        quantiles(&mut v),
        v.iter().filter(|&&e| e > 0.01).count(),
        v.iter().filter(|&&e| e > 0.1).count()
    );
    for (e, c) in clip_overshoot.iter().take(12) {
        eprintln!("  {e:.3} m: {c}");
    }
    eprintln!("rot TCB channels: squad vs slerp, max deviation (deg): {}", quantiles(&mut squad_dev));
    eprintln!(
        "  channels over 1 deg {}, over 5 deg {}",
        squad_dev.iter().filter(|&&d| d > 1.0).count(),
        squad_dev.iter().filter(|&&d| d > 5.0).count()
    );
    eprintln!("  per segment (deg): {}; segments skipped (snaps, half turns, and their neighbors) {skipped}", quantiles(&mut segment_dev));
    eprintln!("  slerp angular-velocity jump at interior keys (deg/s): {}", quantiles(&mut kink));
    eprintln!("  key spacing (s): {}", quantiles(&mut spacing));
    worst_dev.sort_by(|a, b| b.0.total_cmp(&a.0));
    for (d, f) in worst_dev.iter().take(8) {
        eprintln!("  {d:.1} deg: {f}");
    }
}
