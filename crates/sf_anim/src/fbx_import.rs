//! FBX skeleton and clip importer on `ufbx` (MIT OR Unlicense; Rust
//! bindings to the maintained single-file ufbx C library, built by `cc`,
//! so the bake needs a C compiler: MSVC on Windows, any cc elsewhere).
//!
//! - **Space**: ufbx converts every file to meters, +Y up, right-handed
//!   (`target_axes`, `target_unit_meters`) with
//!   `SpaceConversion::AdjustTransforms`, which folds the conversion into
//!   the node transforms it evaluates, so local transforms come out in the
//!   framework's space. Pre/post rotations, pivots and the FBX transform
//!   chain are resolved by ufbx too.
//! - **Skeleton**: every bone node (`LimbNode`) and all its ancestors
//!   except ufbx's synthetic scene root (an exporter's `Armature` node
//!   carries a transform the joints depend on, as in glTF); a file without
//!   bones uses its empty (null) nodes. Rest pose = the node's static
//!   local transform. A name used twice gets `#<index>` appended; Mixamo's
//!   `mixamorig:` prefix is kept (auto-map strips namespaces).
//! - **Clips**: one per animation stack (take), sampled at the scene's
//!   frame rate (`frames_per_second`, 30 when unset) from the stack's begin
//!   to its end, so clip time starts at 0 and keys are uniform (`anim.cook`
//!   sees the native rate). Channels that never leave the rest value are
//!   dropped.

use glam::{DQuat, DVec3, Quat};

use crate::{AnimImporter, AnimSource, Curve, ImportError, SourceClip, SourceJoint, SourceTrack, Trs, detect_root_motion_joint};

/// Imports `.fbx` (binary and ASCII, FBX 6 and 7) through ufbx.
#[derive(Clone, Copy, Debug, Default)]
pub struct FbxAnimImporter;

impl AnimImporter for FbxAnimImporter {
    fn extensions(&self) -> &[&str] {
        &["fbx"]
    }

    fn version(&self) -> u32 {
        1
    }

    fn import(&self, path: &str, bytes: &[u8]) -> Result<AnimSource, ImportError> {
        let err = |message: String| ImportError { path: path.to_owned(), message };
        let opts = ufbx::LoadOpts {
            ignore_geometry: true,
            ignore_embedded: true,
            target_axes: ufbx::CoordinateAxes::right_handed_y_up(),
            target_unit_meters: 1.0,
            space_conversion: ufbx::SpaceConversion::AdjustTransforms,
            ..ufbx::LoadOpts::default()
        };
        let scene = ufbx::load_memory(bytes, opts).map_err(|e| err(e.description.to_string()))?;
        let nodes = &scene.nodes;
        let any_bone = nodes.iter().any(|n| n.bone.is_some());
        let mut wanted: Vec<bool> = nodes
            .iter()
            .map(|n| !n.is_root && if any_bone { n.bone.is_some() } else { n.mesh.is_none() && n.light.is_none() && n.camera.is_none() })
            .collect();
        for i in 0..nodes.len() {
            if !wanted[i] {
                continue;
            }
            let mut p = nodes[i].parent.as_ref();
            while let Some(n) = p.filter(|n| !n.is_root) {
                let k = n.element.typed_id as usize;
                if wanted[k] {
                    break;
                }
                wanted[k] = true;
                p = n.parent.as_ref();
            }
        }
        let order: Vec<usize> = (0..nodes.len()).filter(|&i| wanted[i]).collect();
        if order.is_empty() {
            return Ok(AnimSource::default());
        }
        let mut joint_of = vec![None; nodes.len()];
        for (j, &i) in order.iter().enumerate() {
            joint_of[i] = Some(j);
        }
        let mut seen = std::collections::HashSet::new();
        let names: Vec<String> = order
            .iter()
            .map(|&i| {
                let base: &str = &nodes[i].element.name;
                let base = if base.is_empty() { format!("node{i}") } else { base.to_owned() };
                let name = if seen.contains(&base) { format!("{base}#{i}") } else { base };
                seen.insert(name.clone());
                name
            })
            .collect();
        let skeleton: Vec<SourceJoint> = order
            .iter()
            .zip(&names)
            .map(|(&i, name)| {
                let n = &nodes[i];
                let parent = n.parent.as_ref().and_then(|p| joint_of[p.element.typed_id as usize]);
                SourceJoint { name: name.clone(), parent, rest: trs(&n.local_transform) }
            })
            .collect();

        let fps = if scene.settings.frames_per_second > 0.0 { scene.settings.frames_per_second } else { 30.0 };
        let mut clips = Vec::new();
        for (s, stack) in scene.anim_stacks.iter().enumerate() {
            let span = (stack.time_end - stack.time_begin).max(0.0);
            #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss, clippy::cast_precision_loss)] // Frame counts fit.
            let frames = (span * fps).round() as usize + 1;
            #[allow(clippy::cast_precision_loss, clippy::cast_possible_truncation)]
            let times: Vec<f32> = (0..frames).map(|f| (f as f64 / fps) as f32).collect();
            let mut tracks = Vec::new();
            for (&i, (name, joint)) in order.iter().zip(names.iter().zip(&skeleton)) {
                let n = &nodes[i];
                #[allow(clippy::cast_precision_loss)]
                let samples: Vec<Trs> =
                    (0..frames).map(|f| trs(&ufbx::evaluate_transform(&stack.anim, n, stack.time_begin + f as f64 / fps))).collect();
                let rest = joint.rest;
                let t_moves = samples.iter().any(|t| t.translation.distance(rest.translation) > 1e-5);
                let r_moves = samples.iter().any(|t| crate::angle_between(t.rotation, rest.rotation) > 1e-5);
                let s_moves = samples.iter().any(|t| t.scale.distance(rest.scale) > 1e-5);
                if !(t_moves || r_moves || s_moves) {
                    continue;
                }
                let mut rotations: Vec<Quat> = Vec::with_capacity(frames);
                for t in &samples {
                    rotations.push(rotations.last().map_or(t.rotation, |&prev| crate::math::align_hemisphere(t.rotation, prev)));
                }
                tracks.push(SourceTrack {
                    joint: name.clone(),
                    translation: t_moves.then(|| Curve::linear(times.clone(), samples.iter().map(|t| t.translation).collect())),
                    rotation: r_moves.then(|| Curve::linear(times.clone(), rotations)),
                    scale: s_moves.then(|| Curve::linear(times.clone(), samples.iter().map(|t| t.scale).collect())),
                });
            }
            let name: &str = &stack.element.name;
            let name = if name.is_empty() { format!("anim{s}") } else { name.to_owned() };
            let duration = times.last().copied().unwrap_or(0.0);
            clips.push(SourceClip { name, duration, tracks, ..SourceClip::default() });
        }
        let root_motion = detect_root_motion_joint(&skeleton, &clips).map(|j| skeleton[j].name.clone());
        Ok(AnimSource { skeleton: Some(skeleton), clips, root_motion })
    }
}

fn trs(t: &ufbx::Transform) -> Trs {
    let v = |v: ufbx::Vec3| DVec3::new(v.x, v.y, v.z).as_vec3();
    let q = DQuat::from_xyzw(t.rotation.x, t.rotation.y, t.rotation.z, t.rotation.w).as_quat().normalize();
    Trs::new(v(t.translation), q, v(t.scale))
}
