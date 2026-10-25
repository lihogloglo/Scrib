//! Probe poses for smooth bodies on the `Bip01` skeletons (superfargoth
//! `docs/specs/smooth-bodies.md`, "Survey rerun and guard"): the poses the
//! seam guard checks every smoothed seam in. Elbows and knees bent 90°,
//! shoulders raised 57°, and the `WalkForward` / `RunForward` clips at 25% and
//! 75% (root motion removed), as in `examples/smooth_survey.rs`.

use glam::{Affine3A, Quat, Vec3};
use sf_anim::{Clip, LocalPose, Skeleton};

/// Model-space joint transforms per probe pose, in `skeleton`'s joint
/// order. `clips`: the skeleton's cooked clips (locomotion groups are found
/// by name; a rig without them gets the bent poses only).
pub fn smooth_probes<'a>(skeleton: &Skeleton, clips: impl IntoIterator<Item = &'a Clip>) -> Vec<Vec<Affine3A>> {
    let rest = skeleton.rest_model();
    let at = |n: &str| skeleton.find(n).map(|j| Vec3::from(rest[j].translation));
    let (Some(head), Some(pelvis)) = (at("Bip01 Head"), at("Bip01 Pelvis")) else { return Vec::new() };
    let up = (head - pelvis).normalize_or_zero();
    // Toward the toes; our skeletons face +Z when a rig has no toe joints.
    let toes = match (at("Bip01 L Foot"), at("Bip01 L Toe0")) {
        (Some(foot), Some(toe)) => toe - foot,
        _ => Vec3::Z,
    };
    let forward = toes.reject_from(up).try_normalize().unwrap_or(Vec3::Z);
    // Rotates `joint`'s subtree about its pivot by `angle` around `axis` (or
    // the axis across its bone and `forward`), the way that moves `tip` along `wanted`.
    let bend = |pose: &mut Vec<Affine3A>, joint: &str, tip: &str, axis: Option<Vec3>, angle: f32, wanted: Vec3| -> Option<()> {
        let j = skeleton.find(joint)?;
        let pivot = Vec3::from(pose[j].translation);
        let tip_p = Vec3::from(pose[skeleton.find(tip)?].translation);
        let axis = axis.unwrap_or_else(|| (tip_p - pivot).cross(forward)).try_normalize()?;
        let m = |x: f32| (Quat::from_axis_angle(axis, x) * (tip_p - pivot)).dot(wanted);
        let best = if m(angle) >= m(-angle) { angle } else { -angle };
        let r = Affine3A::from_translation(pivot) * Affine3A::from_quat(Quat::from_axis_angle(axis, best)) * Affine3A::from_translation(-pivot);
        for k in skeleton.subtree(j) {
            pose[k] = r * pose[k];
        }
        Some(())
    };
    let half = std::f32::consts::FRAC_PI_2;
    let mut out = Vec::new();
    let mut add = |f: &dyn Fn(&mut Vec<Affine3A>, &str) -> Option<()>| {
        let mut pose = rest.clone();
        if ["R", "L"].iter().all(|s| f(&mut pose, s).is_some()) {
            out.push(pose);
        }
    };
    add(&|p, s| bend(p, &format!("Bip01 {s} Forearm"), &format!("Bip01 {s} Hand"), None, half, forward));
    add(&|p, s| bend(p, &format!("Bip01 {s} Calf"), &format!("Bip01 {s} Foot"), None, half, -forward));
    add(&|p, s| bend(p, &format!("Bip01 {s} UpperArm"), &format!("Bip01 {s} Hand"), Some(forward), 1.0, up));
    let clips: Vec<&Clip> = clips.into_iter().collect();
    for group in ["walkforward", "runforward"] {
        let Some(clip) = clips.iter().find(|c| is_group(&c.name, group)) else { continue };
        if clip.joint_count() != skeleton.len() {
            continue;
        }
        let mut local = LocalPose::new(skeleton.len());
        for phase in [0.25, 0.75] {
            clip.sample(clip.duration * phase, &mut local, None);
            let mut model: Vec<Affine3A> = Vec::with_capacity(skeleton.len());
            for j in 0..skeleton.len() {
                let m = local.get(j).to_affine();
                model.push(skeleton.parent(j).map_or(m, |p| model[p] * m));
            }
            // Root motion off: the shape matters, not where the body went.
            let root = model.first().copied().unwrap_or(Affine3A::IDENTITY);
            let fix = rest.first().copied().unwrap_or(Affine3A::IDENTITY) * root.inverse();
            out.push(model.into_iter().map(|m| fix * m).collect());
        }
    }
    out
}

/// Whether a clip name is `group` (ASCII case-insensitive; a path or
/// namespace prefix and a suffix after a non-letter are allowed).
fn is_group(name: &str, group: &str) -> bool {
    let name = name.rsplit(['/', ':']).next().unwrap_or(name).to_ascii_lowercase();
    name.strip_prefix(group).is_some_and(|rest| !rest.starts_with(|c: char| c.is_ascii_alphabetic()))
}

#[cfg(test)]
mod tests {
    #[test]
    fn locomotion_clips_are_found_by_group_name() {
        assert!(super::is_group("WalkForward", "walkforward"));
        assert!(super::is_group("meshes/xbase_anim.kf:WalkForward", "walkforward"));
        assert!(!super::is_group("SwimWalkForward", "walkforward"));
        assert!(!super::is_group("WalkForwardx", "walkforward"));
    }
}
