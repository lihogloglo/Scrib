//! Key-convention audit on the real game data (raw NIF keys, before any
//! conversion): which key types the data uses, and whether quadratic
//! (Bezier) tangents are stored per segment (Hermite on `[0, 1]`, the
//! convention `mw_nif::anim` follows) or per second. Ignored by default:
//! `cargo test -p mw_anim --release --test kf_curve_conventions -- --ignored --nocapture`.

use std::collections::BTreeMap;

use mw_install::MorrowindInstall;
use mw_nif::anim::Curve;
use sf_vfs::Vfs;
use tes3::nif::glam::Vec3;
use tes3::nif::{NiFloatKey, NiKeyframeData, NiPosKey, NiRotKey, NiStream};

#[derive(Default)]
struct Stats {
    /// Interior keys with unequal neighbor segments (ratio > 1.5) and motion.
    uneven: usize,
    /// ... whose in and out tangents are equal (per-time convention).
    equal_tangents: usize,
    /// ... whose `in / dt_prev` equals `out / dt_next` (per-segment convention, C1).
    per_segment_c1: usize,
    /// ... whose `in / dt_next` equals `out / dt_prev` (segments swapped).
    swapped_c1: usize,
    /// `log(|in| / |out|) / log(dt_prev / dt_next)` per uneven key.
    slope: Vec<f32>,
    /// `log2(|out| / |v * dt_next|)` for keys with motion: 0 means per segment.
    log_ratio_segment: Vec<f32>,
    /// `log2(|out| / |v|)` (`v` per second): 0 means per second.
    log_ratio_second: Vec<f32>,
}

fn audit_vec3(keys: &[(f32, Vec3, Vec3, Vec3)], stats: &mut Stats) {
    for w in keys.windows(3) {
        let [(t0, p0, _, _), (t1, _, in1, out1), (t2, p2, _, _)] = [w[0], w[1], w[2]];
        let (dp, dn) = (t1 - t0, t2 - t1);
        if dp <= 1e-5 || dn <= 1e-5 {
            continue;
        }
        let v = (p2 - p0) / (t2 - t0);
        let motion = (p2 - p0).length();
        if motion < 1e-3 || in1.length() < 1e-4 || out1.length() < 1e-4 {
            continue;
        }
        stats.log_ratio_segment.push((out1.length() / (v.length() * dn).max(1e-9)).log2());
        stats.log_ratio_second.push((out1.length() / v.length().max(1e-9)).log2());
        if dp / dn > 1.5 || dn / dp > 1.5 {
            stats.uneven += 1;
            let scale = in1.length().max(out1.length());
            if (in1 - out1).length() < 1e-3 * scale {
                stats.equal_tangents += 1;
            }
            let (a, b) = (in1 / dp, out1 / dn);
            if (a - b).length() < 1e-3 * a.length().max(b.length()) {
                stats.per_segment_c1 += 1;
            }
            let (a, b) = (in1 / dn, out1 / dp);
            if (a - b).length() < 1e-3 * a.length().max(b.length()) {
                stats.swapped_c1 += 1;
            }
            // 1: |in|/|out| = dp/dn (per segment); 0: equal; -1: swapped.
            stats.slope.push((in1.length() / out1.length()).log2() / (dp / dn).log2());
        }
    }
}

/// Worst segment of a curve: (largest tangent/chord for chords over one
/// unit, largest overshoot: distance of the curve outside the box its two
/// keys span, MW units).
fn overshoot(curve: &Curve<Vec3>) -> (f32, f32) {
    let keys: Vec<_> = curve.keys().collect();
    let (mut ratio, mut excursion) = (0.0_f32, 0.0_f32);
    for w in keys.windows(2) {
        let ((t0, p0, _, m0), (t1, p1, m1, _)) = (w[0], w[1]);
        let chord = (p1 - p0).length();
        if chord > 1.0 {
            ratio = ratio.max(m0.length().max(m1.length()) / chord);
        }
        for i in 1..32 {
            #[allow(clippy::cast_precision_loss)]
            let x = i as f32 / 32.0;
            let value = curve.sample(t0 + (t1 - t0) * x).unwrap();
            excursion = excursion.max((value - value.clamp(p0.min(p1), p0.max(p1))).length());
        }
    }
    (ratio, excursion)
}

/// The TCB curve without the key-spacing adjustment (the pre-fix `mw_nif`).
fn unadjusted_tcb(curve: &Curve<Vec3>) -> Curve<Vec3> {
    let keys: Vec<_> = curve.keys().collect();
    Curve::hermite((0..keys.len()).map(|i| {
        let (t, v, mut i_tan, mut o_tan) = keys[i];
        if i > 0 && i + 1 < keys.len() {
            let (dp, dn) = (t - keys[i - 1].0, keys[i + 1].0 - t);
            if dp + dn > 0.0 && dp > 0.0 && dn > 0.0 {
                i_tan /= 2.0 * dp / (dp + dn);
                o_tan /= 2.0 * dn / (dp + dn);
            }
        }
        (t, v, i_tan, o_tan)
    }))
}

/// |log2| of the speed ratio across each interior key whose `smooth` flag
/// is set (continuity 0: the curve should keep its velocity there).
fn speed_jumps(curve: &Curve<Vec3>, smooth: &[bool], out: &mut Vec<f32>) {
    let keys: Vec<_> = curve.keys().collect();
    for i in 1..keys.len().saturating_sub(1) {
        let (dp, dn) = (keys[i].0 - keys[i - 1].0, keys[i + 1].0 - keys[i].0);
        let (a, b) = (keys[i].2.length() / dp, keys[i].3.length() / dn);
        if smooth[i] && dp > 1e-4 && dn > 1e-4 && a > 1.0 && b > 1.0 {
            out.push((a / b).log2().abs());
        }
    }
}

fn median(v: &mut [f32]) -> f32 {
    if v.is_empty() {
        return f32::NAN;
    }
    v.sort_by(f32::total_cmp);
    v[v.len() / 2]
}

fn quantiles(v: &mut [f32]) -> String {
    if v.is_empty() {
        return "-".into();
    }
    v.sort_by(f32::total_cmp);
    #[allow(clippy::cast_precision_loss, clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    let q = |p: f32| v[((v.len() - 1) as f32 * p) as usize];
    format!("p10 {:.2} p50 {:.2} p90 {:.2} (n {})", q(0.1), q(0.5), q(0.9), v.len())
}

#[test]
#[ignore = "real install audit"]
#[allow(clippy::too_many_lines)]
fn nif_key_conventions() {
    let Ok((install, _)) = MorrowindInstall::detect(None) else {
        eprintln!("no Morrowind install detected; skipping");
        return;
    };
    let vfs = mw_vfs::mount_install(&install, Vfs::builder()).unwrap().vfs;
    let mut types: BTreeMap<String, usize> = BTreeMap::new();
    let mut pos = Stats::default();
    let mut float = Stats::default();
    let mut spacing: BTreeMap<String, Vec<f32>> = BTreeMap::new();
    let mut files = 0;
    let (mut tcb_ratio, mut tcb_excursion, mut worst) = (Vec::new(), Vec::new(), Vec::new());
    let (mut quad_ratio, mut quad_excursion) = (Vec::new(), Vec::new());
    let mut rot_tcb_bend = Vec::new();
    let (mut tcb_old_ratio, mut tcb_old_excursion, mut worst_old) = (Vec::new(), Vec::new(), Vec::new());
    let (mut jumps_new, mut jumps_old) = (Vec::new(), Vec::new());
    let only = std::env::var("AUDIT_FILE").ok();
    for path in vfs.paths() {
        let name = path.as_str();
        if !matches!(path.extension().map(str::to_ascii_lowercase).as_deref(), Some("kf" | "nif")) || only.as_deref().is_some_and(|o| !name.contains(o)) {
            continue;
        }
        let Ok(bytes) = vfs.read(path) else { continue };
        if !bytes.windows(14).any(|w| w == b"NiKeyframeData") {
            continue;
        }
        let Ok(stream) = NiStream::from_bytes(&bytes) else { continue };
        files += 1;
        for data in stream.objects_of_type::<NiKeyframeData>() {
            match &data.translations.keys {
                NiPosKey::LinKey(k) if !k.is_empty() => *types.entry("pos linear".into()).or_default() += 1,
                NiPosKey::BezKey(k) => {
                    *types.entry("pos quadratic".into()).or_default() += 1;
                    let keys: Vec<_> = k.iter().map(|k| (k.time, k.value, k.in_tan, k.out_tan)).collect();
                    audit_vec3(&keys, &mut pos);
                    let (ratio, excursion) = overshoot(&Curve::hermite(keys.iter().copied()));
                    quad_ratio.push(ratio);
                    quad_excursion.push(excursion);
                    worst.push((excursion, ratio, format!("quadratic {name}")));
                    if only.is_some() {
                        for key in &keys {
                            eprintln!("  t {:.4} p {:?} in {:?} out {:?}", key.0, key.1, key.2, key.3);
                        }
                    }
                }
                NiPosKey::TCBKey(k) => {
                    *types.entry("pos TCB".into()).or_default() += 1;
                    let curve = Curve::tcb(k.iter().map(|k| (k.time, k.value, k.tension, k.continuity, k.bias)));
                    let (ratio, excursion) = overshoot(&curve);
                    tcb_ratio.push(ratio);
                    tcb_excursion.push(excursion);
                    worst.push((excursion, ratio, format!("TCB {name}")));
                    let old = unadjusted_tcb(&curve);
                    let (old_ratio, old_excursion) = overshoot(&old);
                    tcb_old_ratio.push(old_ratio);
                    tcb_old_excursion.push(old_excursion);
                    worst_old.push((old_excursion, old_ratio, format!("TCB {name}")));
                    let smooth: Vec<bool> = k.iter().map(|k| k.continuity == 0.0).collect();
                    speed_jumps(&curve, &smooth, &mut jumps_new);
                    speed_jumps(&old, &smooth, &mut jumps_old);
                    if only.is_some() && excursion > 10.0 {
                        let tangents: Vec<_> = curve.keys().collect();
                        for (key, (_, _, i, o)) in k.iter().zip(tangents) {
                            eprintln!("  tcb t {:.4} p {:.1} tcb {} {} {} in {i:.1} out {o:.1}", key.time, key.value, key.tension, key.continuity, key.bias);
                        }
                    }
                    let s = spacing.entry("pos TCB uneven ratio".into()).or_default();
                    s.extend(k.windows(3).map(|w| ((w[2].time - w[1].time) / (w[1].time - w[0].time).max(1e-6)).max((w[1].time - w[0].time) / (w[2].time - w[1].time).max(1e-6))));
                }
                NiPosKey::LinKey(_) => {}
            }
            let mut floats = vec![("scale", &data.scales.keys)];
            match &data.rotations.keys {
                NiRotKey::LinKey(k) if !k.is_empty() => *types.entry("rot linear quat".into()).or_default() += 1,
                NiRotKey::BezKey(_) => *types.entry("rot quadratic quat".into()).or_default() += 1,
                NiRotKey::TCBKey(k) => {
                    *types.entry("rot TCB quat".into()).or_default() += 1;
                    if k.iter().any(|k| k.tension != 0.0 || k.continuity != 0.0 || k.bias != 0.0) {
                        *types.entry("rot TCB quat, non-zero parameters".into()).or_default() += 1;
                    }
                    // How far each key sits off the slerp of its neighbors
                    // (degrees): what a spline through the keys bends by and slerp
                    // between them doesn't.
                    let bend = k
                        .windows(3)
                        .map(|w| {
                            let x = (w[1].time - w[0].time) / (w[2].time - w[0].time).max(1e-6);
                            let mid = w[0].value.slerp(w[2].value, x);
                            mid.angle_between(w[1].value).to_degrees()
                        })
                        .fold(0.0_f32, f32::max);
                    rot_tcb_bend.push(bend);
                }
                NiRotKey::EulerKey(e) => {
                    *types.entry("rot XYZ".into()).or_default() += 1;
                    floats.extend(e.axes.iter().map(|a| ("euler axis", &a.keys)));
                }
                NiRotKey::LinKey(_) => {}
            }
            for (what, keys) in floats {
                match keys {
                    NiFloatKey::LinKey(k) if !k.is_empty() => *types.entry(format!("{what} linear")).or_default() += 1,
                    NiFloatKey::BezKey(k) => {
                        *types.entry(format!("{what} quadratic")).or_default() += 1;
                        let keys: Vec<_> = k.iter().map(|k| (k.time, Vec3::splat(k.value), Vec3::splat(k.in_tan), Vec3::splat(k.out_tan))).collect();
                        audit_vec3(&keys, &mut float);
                    }
                    NiFloatKey::TCBKey(_) => *types.entry(format!("{what} TCB")).or_default() += 1,
                    NiFloatKey::LinKey(_) => {}
                }
            }
        }
    }
    eprintln!("{files} files with keyframe data; channels by key type: {types:#?}");
    for (name, s) in [("pos quadratic", &mut pos), ("float quadratic", &mut float)] {
        eprintln!(
            "{name}: uneven interior keys {}: in == out {}, in/dt_prev == out/dt_next {}, in/dt_next == out/dt_prev {}",
            s.uneven, s.equal_tangents, s.per_segment_c1, s.swapped_c1
        );
        eprintln!("  log|in/out| / log(dt_prev/dt_next) (1 per segment, 0 per time, -1 swapped): {}", quantiles(&mut s.slope));
        eprintln!("  log2(|out| / |v dt_next|) (0 = per segment): {}", quantiles(&mut s.log_ratio_segment));
        eprintln!("  log2(|out| / |v|)         (0 = per second):  {}", quantiles(&mut s.log_ratio_second));
    }
    for (label, ratio, excursion, jumps) in [
        ("adjusted (as built)", &mut tcb_ratio, &mut tcb_excursion, &mut jumps_new),
        ("unadjusted (before)", &mut tcb_old_ratio, &mut tcb_old_excursion, &mut jumps_old),
    ] {
        eprintln!("pos TCB {label}: worst segment tangent/chord (chord > 1 unit): {}", quantiles(ratio));
        eprintln!(
            "  worst overshoot past a segment's keys (MW units): {} max {:.1}; channels over 1 unit {}",
            quantiles(excursion),
            excursion.last().copied().unwrap_or(0.0),
            excursion.iter().filter(|&&e| e > 1.0).count()
        );
        eprintln!(
            "  |log2 speed ratio| across continuity-0 keys (0 = C1): {}; keys over 2x {}",
            quantiles(jumps),
            jumps.iter().filter(|&&j| j > 1.0).count()
        );
    }
    worst_old.sort_by(|a, b| b.0.total_cmp(&a.0));
    worst_old.dedup_by(|a, b| a.2 == b.2);
    for (e, r, f) in worst_old.iter().take(5) {
        eprintln!("  before: {e:7.1} units, tangent/chord {r:5.1}: {f}");
    }
    eprintln!("pos quadratic channels, worst segment tangent/chord (chord > 1 unit): {}", quantiles(&mut quad_ratio));
    eprintln!(
        "pos quadratic channels, worst overshoot past a segment's keys (MW units): {} max {:.1}",
        quantiles(&mut quad_excursion),
        quad_excursion.last().copied().unwrap_or(0.0)
    );
    eprintln!("rot TCB channels, largest key bend off its neighbors' slerp (degrees): {}", quantiles(&mut rot_tcb_bend));
    worst.sort_by(|a, b| b.0.total_cmp(&a.0));
    worst.dedup_by(|a, b| a.2 == b.2);
    for (e, r, f) in worst.iter().take(12) {
        eprintln!("  {e:7.1} units, tangent/chord {r:5.1}: {f}");
    }
    for (name, v) in &mut spacing {
        eprintln!("{name}: median {:.2}", median(v));
    }
}
