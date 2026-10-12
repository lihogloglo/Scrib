//! A text listing of an animation file's groups, sections and events: the
//! body of `bevyrrowind anim <kf>` (and of the `anim_report` example).

use std::collections::BTreeSet;
use std::fmt::Write as _;

use sf_anim::{Looping, SourceClip};

use crate::Animation;

/// What [`report`] lists.
#[derive(Clone, Debug, Default)]
pub struct ReportOptions {
    /// One line per event, not only per group.
    pub events: bool,
    /// Only this group (case-insensitive).
    pub group: Option<String>,
}

/// Lists `animation` (read from `path`): per group its range, length, key
/// count, looping, sections, events, and Biped root travel.
pub fn report(path: &str, animation: &Animation, options: &ReportOptions) -> String {
    let groups = animation.groups();
    let mut out = String::new();
    let _ = writeln!(out, "{path}: {} animated bones, {} text-key lines, {} groups", animation.tracks.len(), animation.text_keys.len(), groups.len());
    let mut kinds = BTreeSet::new();
    for group in &groups {
        if options.group.as_ref().is_some_and(|o| !o.eq_ignore_ascii_case(&group.name)) {
            continue;
        }
        let clip = animation.source_clip(group);
        let looping = match group.looping() {
            Looping::Once => "once".to_owned(),
            Looping::Whole => "loop".to_owned(),
            Looping::Range(a, b) => format!("loop {a:.3}..{b:.3}"),
        };
        let _ = writeln!(
            out,
            "  {:<22} {:>8.3}..{:<8.3} {:>6.3} s  {:>4} keys  {:<18} {} events{}",
            group.name,
            group.start,
            group.stop,
            group.duration(),
            key_times(&clip),
            looping,
            group.events.len(),
            root_travel(&clip),
        );
        for s in &group.sections {
            let _ = writeln!(out, "      section {:<8} {:.3}..{:.3}", s.name, s.start, s.stop);
        }
        for e in &group.events {
            kinds.insert(e.name.clone());
            if options.events {
                let end = e.end.map(|end| format!("..{end:.3}")).unwrap_or_default();
                let _ = writeln!(out, "      {:>7.3}{end:<8} {:<16} {}", e.time, e.name, e.payload);
            }
        }
    }
    let _ = writeln!(out, "event kinds: {}", kinds.into_iter().collect::<Vec<_>>().join(", "));
    out
}

/// Distinct key times over all tracks.
fn key_times(clip: &SourceClip) -> usize {
    let mut times: Vec<f32> = clip
        .tracks
        .iter()
        .flat_map(|t| t.translation.iter().flat_map(|c| c.times.iter()).chain(t.rotation.iter().flat_map(|c| c.times.iter())).copied())
        .collect();
    times.sort_by(f32::total_cmp);
    times.dedup_by(|a, b| (*a - *b).abs() < 1e-4);
    times.len()
}

/// Translation travel of the Biped root bones over the clip (root-motion check).
fn root_travel(clip: &SourceClip) -> String {
    let mut out = String::new();
    for bone in ["Bip01", "Bip01 NonAccum"] {
        if let Some(c) = clip.tracks.iter().find(|t| t.joint.eq_ignore_ascii_case(bone)).and_then(|t| t.translation.as_ref()) {
            let d = c.sample(clip.duration) - c.sample(0.0);
            if d.length() > 0.01 {
                let _ = write!(out, "  [{bone} travels {:.2} {:.2} {:.2} m]", d.x, d.y, d.z);
            }
        }
    }
    out
}
