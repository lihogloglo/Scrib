//! `anim.cook` survey on the real game data, without the bake store: every
//! clip the actors use, cooked straight through `sf_anim::cook_clip_for`
//! in parallel. Prints the CPU time, the rate histogram and the error
//! distribution; with `SURVEY_OUT=<file>` writes one line per clip
//! (subject, rate, frames, max error, size, byte hash) for before/after
//! comparisons; with `SURVEY_DIAG=1` classifies why clips over the error
//! bound miss it. Ignored by default (about 2 min of CPU):
//! `cargo test -p mw_bake --release --test anim_cook_survey -- --ignored --nocapture`.

use std::collections::BTreeMap;
use std::fmt::Write as _;
use std::hash::{DefaultHasher, Hash, Hasher};
use std::sync::Arc;
use std::time::{Duration, Instant};

use mw_bake::anim::{self, GroupInfo, MwAnimRules, anim_importers};
use mw_install::MorrowindInstall;
use rayon::prelude::*;
use sf_anim::steps::{AnimRules, anim_subject};
use sf_anim::{AnimSource, BoundSource, ClipRules, CookSettings, Interpolation, RigProfile, Skeleton, SourceClip, Trs, angle_between};
use sf_records::RecordDb;
use sf_vfs::{Vfs, VfsPath};

struct Job {
    subject: String,
    skeleton: Arc<(Skeleton, Option<RigProfile>)>,
    source: Arc<AnimSource>,
    clip: String,
    rules: ClipRules,
}

fn jobs() -> Option<Vec<Job>> {
    let Ok((install, _)) = MorrowindInstall::detect(None) else {
        eprintln!("no Morrowind install detected; skipping");
        return None;
    };
    let vfs = mw_vfs::mount_install(&install, Vfs::builder().mount(mw_graphs::builtin_source())).unwrap().vfs;
    let db = mw_records::load(&vfs, &install.game_files()).unwrap().db;
    let (vfs, db): (Arc<Vfs>, Arc<RecordDb>) = (Arc::new(vfs), Arc::new(db));
    let sets = anim::actor_sets(&db, &vfs);
    let importers = anim_importers();
    let import = |path: &str| -> Arc<AnimSource> {
        let bytes = vfs.read(&VfsPath::new(path).unwrap()).unwrap();
        Arc::new(importers.for_path(path).unwrap().import(path, &bytes).unwrap())
    };
    let mut rules = MwAnimRules::default();
    let mut sources = BTreeMap::new();
    for file in sets.files() {
        let bytes = vfs.read(&VfsPath::new(file).unwrap()).unwrap();
        let groups = mw_anim::Animation::from_file(file, &bytes).unwrap().groups().iter().map(GroupInfo::of).collect();
        rules.add_groups(file, groups);
        sources.insert(file.to_owned(), import(file));
    }
    let mut skeletons = BTreeMap::new();
    for set in sets.sets.values() {
        let source = sources.get(&set.skeleton).cloned().unwrap_or_else(|| import(&set.skeleton));
        let mut sr = MwAnimRules::skeleton_rules();
        if sr.root_motion_joint.is_none() && sr.detect_root_motion {
            sr.root_motion_joint.clone_from(&source.root_motion);
        }
        let (skeleton, _) = Skeleton::with_rules(source.skeleton.as_ref().unwrap(), &sr).unwrap();
        rules.add_skeleton(set, &skeleton);
        skeletons.insert(set.skeleton.clone(), skeleton);
    }
    let mut jobs = Vec::new();
    for (path, skeleton) in skeletons {
        let profile = sf_anim::detect_profile(&skeleton, &rules.rig_hints(&path)).ok();
        let pair = Arc::new((skeleton, profile));
        for (file, group) in rules.set_groups(&path) {
            let spec = rules.clip(file, group, &path);
            jobs.push(Job {
                subject: anim_subject(&path, file, group),
                skeleton: pair.clone(),
                source: sources[file].clone(),
                clip: spec.source,
                rules: spec.rules,
            });
        }
    }
    Some(jobs)
}

struct Cooked {
    subject: String,
    rate: f32,
    frames: u32,
    error: f32,
    size: usize,
    hash: u64,
    time: Duration,
    diag: Option<Diag>,
}

/// Why a clip misses the bound: the source channel whose resampling (at
/// the cooked rate, ignoring quantization) moves a virtual vertex most.
struct Diag {
    joint: String,
    channel: &'static str,
    /// Error that channel alone causes, meters at its lever arm.
    error: f32,
    time: f32,
    /// The source segment around `time`: duration, interpolation.
    segment: f32,
    interpolation: Interpolation,
    /// Keys of the channel and the clip's duration.
    keys: usize,
    /// Error with 2x and 4x the cooked rate (same channel).
    error_2x: f32,
    error_4x: f32,
    /// Whole-pose model-space resampling error (no root motion, no quantization).
    model_error: f32,
    /// Cubic segment at `time`: largest tangent step over the chord.
    tangent_ratio: f32,
    /// Cooked error without root-motion extraction.
    error_no_root: f32,
}

fn sample_rot(c: &sf_anim::Curve<glam::Quat>, t: f32) -> glam::Quat {
    c.sample(t)
}

fn segment_at<T>(c: &sf_anim::Curve<T>, t: f32) -> f32 {
    let i = c.times.partition_point(|&k| k <= t).clamp(1, c.times.len().max(2) - 1);
    if c.times.len() < 2 { 0.0 } else { c.times[i] - c.times[i - 1] }
}

#[allow(clippy::cast_precision_loss)]
fn resample_error<T: Copy>(n: usize, start: f32, duration: f32, sample: impl Fn(f32) -> T, lerp: impl Fn(T, T, f32) -> T, dist: impl Fn(T, T) -> f32) -> (f32, f32) {
    let frames: Vec<T> = (0..n).map(|f| sample(start + if n < 2 { 0.0 } else { duration * f as f32 / (n - 1) as f32 })).collect();
    let mut worst = (0.0f32, 0.0f32);
    if n < 2 {
        return worst;
    }
    let dt = duration / (n - 1) as f32;
    for f in 0..n - 1 {
        for k in 1..8 {
            let a = k as f32 / 8.0;
            let t = dt * (f as f32 + a);
            let e = dist(sample(start + t), lerp(frames[f], frames[f + 1], a));
            if e > worst.0 {
                worst = (e, t);
            }
        }
    }
    worst
}

#[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss, clippy::cast_precision_loss, clippy::many_single_char_names, clippy::too_many_lines)]
fn diagnose(skeleton: &Skeleton, source: &SourceClip, rules: &ClipRules, rate: f32, shell: f32) -> Option<Diag> {
    let reach = skeleton.subtree_reach();
    let (start, end) = rules.range.unwrap_or((0.0, source.duration));
    let duration = end - start;
    let count = |r: f32| ((duration * r - 1e-3).ceil().max(1.0) as usize) + 1;
    let mut best: Option<Diag> = None;
    for track in &source.tracks {
        let Some(j) = skeleton.find(&track.joint) else { continue };
        let lever = shell + reach[j];
        let mut consider = |channel: &'static str, keys: usize, interp: Interpolation, err: &dyn Fn(f32) -> (f32, f32), seg: &dyn Fn(f32) -> f32| {
            let (e, t) = err(rate);
            if best.as_ref().is_none_or(|b| e > b.error) {
                best = Some(Diag {
                    joint: track.joint.clone(),
                    channel,
                    error: e,
                    time: t,
                    segment: seg(start + t),
                    interpolation: interp,
                    keys,
                    error_2x: err(rate * 2.0).0,
                    error_4x: err(rate * 4.0).0,
                    model_error: 0.0,
                    tangent_ratio: 0.0,
                    error_no_root: 0.0,
                });
            }
        };
        if let Some(c) = &track.rotation {
            let err = |r: f32| resample_error(count(r), start, duration, |t| sample_rot(c, t), sf_anim::nlerp, |a, b| angle_between(a, b) * lever);
            consider("rotation", c.times.len(), c.interpolation, &err, &|t| segment_at(c, t));
        }
        if let Some(c) = &track.translation {
            let err = |r: f32| resample_error(count(r), start, duration, |t| c.sample(t), glam::Vec3::lerp, glam::Vec3::distance);
            consider("translation", c.times.len(), c.interpolation, &err, &|t| segment_at(c, t));
        }
        if let Some(c) = &track.scale {
            let err = |r: f32| resample_error(count(r), start, duration, |t| c.sample(t), glam::Vec3::lerp, |a, b| a.distance(b) * lever);
            consider("scale", c.times.len(), c.interpolation, &err, &|t| segment_at(c, t));
        }
    }
    let mut best = best?;
    // Tangent check of the worst cubic translation segment.
    if let Some(c) = source.tracks.iter().find(|t| t.joint == best.joint).and_then(|t| t.translation.as_ref())
        && c.interpolation == Interpolation::CubicSpline
        && best.channel == "translation"
    {
        let t = start + best.time;
        let i = c.times.partition_point(|&k| k <= t).clamp(1, c.times.len() - 1) - 1;
        let dt = c.times[i + 1] - c.times[i];
        let (p0, m0, p1, m1) = (c.values[i * 3 + 1], c.values[i * 3 + 2], c.values[(i + 1) * 3 + 1], c.values[(i + 1) * 3]);
        best.tangent_ratio = (m0.length().max(m1.length()) * dt) / p0.distance(p1).max(1e-6);
    }
    // Whole pose, model space, at midpoints of the cooked rate.
    let n = count(rate);
    let j = skeleton.len();
    let mut bound = BoundSource::new(source, skeleton);
    bound.unbound.clear();
    let mut frames = vec![Trs::IDENTITY; n * j];
    for f in 0..n {
        bound.sample(start + duration * f as f32 / (n - 1).max(1) as f32, skeleton.rest(), &mut frames[f * j..(f + 1) * j]);
    }
    let (mut truth, mut mid) = (vec![Trs::IDENTITY; j], vec![Trs::IDENTITY; j]);
    let (mut a, mut b) = (sf_anim::ModelPose::new(j), sf_anim::ModelPose::new(j));
    let probes = [glam::Vec3::X * shell, glam::Vec3::Y * shell, glam::Vec3::Z * shell];
    for f in 0..n.saturating_sub(1) {
        bound.sample(start + duration * (f as f32 + 0.5) / (n - 1) as f32, skeleton.rest(), &mut truth);
        for k in 0..j {
            let (p, q) = (frames[f * j + k], frames[(f + 1) * j + k]);
            mid[k] = Trs { translation: p.translation.lerp(q.translation, 0.5), rotation: sf_anim::nlerp(p.rotation, q.rotation, 0.5), scale: p.scale.lerp(q.scale, 0.5) };
        }
        sf_anim::local_to_model(skeleton, &sf_anim::LocalPose::from_trs(&truth), &mut a, &glam::Affine3A::IDENTITY);
        sf_anim::local_to_model(skeleton, &sf_anim::LocalPose::from_trs(&mid), &mut b, &glam::Affine3A::IDENTITY);
        for (x, y) in a.mats.iter().zip(&b.mats) {
            for v in probes {
                best.model_error = best.model_error.max(x.transform_point3(v).distance(y.transform_point3(v)));
            }
        }
    }
    Some(best)
}

#[test]
#[ignore = "real install, ~2 min CPU"]
#[allow(clippy::too_many_lines, clippy::cast_precision_loss, clippy::cast_possible_truncation, clippy::cast_sign_loss)]
fn survey_every_actor_clip() {
    let Some(jobs) = jobs() else { return };
    let settings = CookSettings::default();
    let diag = std::env::var_os("SURVEY_DIAG").is_some();
    let wall = Instant::now();
    let mut cooked: Vec<Cooked> = jobs
        .par_iter()
        .map(|job| {
            let (skeleton, profile) = &*job.skeleton;
            let source = job.source.clips.iter().find(|c| c.name == job.clip).unwrap_or_else(|| panic!("{}: no clip", job.subject));
            let _ = BoundSource::new(source, skeleton);
            let t = Instant::now();
            let clip = sf_anim::cook_clip_for(skeleton, profile.as_ref(), source, &job.rules, &settings).unwrap();
            let time = t.elapsed();
            let bytes = clip.to_bytes();
            let mut h = DefaultHasher::new();
            bytes.hash(&mut h);
            let over = clip.meta.max_error > settings.error_bound;
            Cooked {
                subject: job.subject.clone(),
                rate: clip.rate,
                frames: clip.frame_count,
                error: clip.meta.max_error,
                size: bytes.len(),
                hash: h.finish(),
                time,
                diag: (diag && over).then(|| diagnose(skeleton, source, &job.rules, clip.rate, settings.shell_distance)).flatten().map(|mut d| {
                    let mut rules = job.rules.clone();
                    rules.root_motion.extract = false;
                    d.error_no_root = sf_anim::cook_clip_for(skeleton, profile.as_ref(), source, &rules, &settings).map_or(f32::NAN, |c| c.meta.max_error);
                    d
                }),
            }
        })
        .collect();
    let wall = wall.elapsed();
    cooked.sort_by(|a, b| a.subject.cmp(&b.subject));
    let cpu: Duration = cooked.iter().map(|c| c.time).sum();
    let mut rates: BTreeMap<u32, (usize, usize)> = BTreeMap::new();
    for c in &cooked {
        #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
        let e = rates.entry(c.rate.round() as u32).or_default();
        e.0 += 1;
        e.1 += usize::from(c.error > settings.error_bound);
    }
    let mut errors: Vec<f32> = cooked.iter().map(|c| c.error).collect();
    errors.sort_by(f32::total_cmp);
    let q = |p: f32| errors[((errors.len() - 1) as f32 * p) as usize];
    let over = cooked.iter().filter(|c| c.error > settings.error_bound).count();
    let bytes: usize = cooked.iter().map(|c| c.size).sum();
    eprintln!(
        "{} clips: wall {wall:.2?}, cook CPU {cpu:.2?} ({:.2} ms/clip); {over} over the bound; {:.1} MB",
        cooked.len(),
        cpu.as_secs_f64() * 1000.0 / cooked.len() as f64,
        bytes as f64 / 1e6
    );
    eprintln!("rates (Hz: clips, over the bound): {rates:?}");
    eprintln!("max error quantiles (m): p50 {:.2e} p90 {:.2e} p99 {:.2e} max {:.2e}", q(0.5), q(0.9), q(0.99), q(1.0));
    if let Some(path) = std::env::var_os("SURVEY_OUT") {
        let mut out = String::new();
        for c in &cooked {
            writeln!(out, "{}\t{}\t{}\t{:e}\t{}\t{:016x}", c.subject, c.rate, c.frames, c.error, c.size, c.hash).unwrap();
        }
        std::fs::write(path, out).unwrap();
    }
    if diag {
        let mut classes: BTreeMap<String, usize> = BTreeMap::new();
        let mut diags: Vec<(&Cooked, &Diag)> = cooked.iter().filter_map(|c| c.diag.as_ref().map(|d| (c, d))).collect();
        for (c, d) in &diags {
            let rate_dt = 1.0 / c.rate;
            let shape = if d.segment < rate_dt { "segment shorter than a frame" } else { "segment >= a frame" };
            let order = if d.error_2x > d.error * 0.4 { "error ~1/2 per doubling or worse" } else { "error <= 1/2.5 per doubling" };
            *classes.entry(format!("{} {:?}, {shape}, {order}", d.channel, d.interpolation)).or_default() += 1;
            let reaches = if d.error > 0.5 * c.error { "channel explains the error" } else { "channel explains < half" };
            let root = if d.error_no_root < 0.5 * c.error { "root-motion extraction explains > half (error halves or better without it)" } else { "not root motion" };
            *classes.entry(root.to_owned()).or_default() += 1;
            let model = if d.model_error > 0.5 * c.error { "whole-pose resampling explains the error" } else { "whole-pose resampling explains < half" };
            *classes.entry(model.to_owned()).or_default() += 1;
            *classes.entry(reaches.to_owned()).or_default() += 1;
        }
        eprintln!("over-bound clips by cause:");
        for (k, v) in &classes {
            eprintln!("  {v:5}  {k}");
        }
        diags.sort_by(|a, b| b.0.error.total_cmp(&a.0.error));
        for (c, d) in diags.iter().take(25) {
            eprintln!(
                "  {:.2e} @ {} Hz {}: {} {} {:?}, {} keys, err {:.2e} at t={:.3}, segment {:.4} s; 2x {:.2e} 4x {:.2e}",
                c.error, c.rate, c.subject, d.joint, d.channel, d.interpolation, d.keys, d.error, d.time, d.segment, d.error_2x, d.error_4x
            );
            eprintln!("      whole-pose resampling error {:.2e}, tangent/chord {:.1}, cooked without root motion {:.2e}", d.model_error, d.tangent_ratio, d.error_no_root
            );
        }
    }
}
