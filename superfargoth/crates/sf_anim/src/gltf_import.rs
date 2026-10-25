//! The framework's glTF 2.0 skeleton and clip importer (binary `.glb`, or
//! `.gltf` whose buffers are all embedded in a GLB `BIN` chunk), built on
//! the `gltf` crate, as `sf_model`'s mesh importer is.
//!
//! - **Skeleton**: every skin joint, every node an animation targets, and
//!   all their ancestors (an `Armature` node above the root joint carries
//!   a transform the joints' world space depends on), with the node's
//!   local TRS as the rest pose. Unnamed nodes are called `node<index>`;
//!   a name used twice gets `#<index>` appended.
//! - **Clips**: one per glTF animation (`anim<index>` when unnamed).
//!   Translation, rotation and scale channels become [`SourceTrack`]s with
//!   their interpolation (step, linear, cubic spline) kept; morph-weight
//!   channels become float curves named `<node>.morph<k>`. Keys whose time
//!   does not increase are dropped (some exporters repeat the last key).
//!
//! glTF is already Y up, right-handed, in meters, so nothing is converted.

use std::collections::{BTreeMap, HashSet};

use glam::{Quat, Vec3};
use gltf::animation::util::ReadOutputs;
use gltf::animation::{Interpolation as GltfInterpolation, Property};
use gltf::buffer::Source as BufferSource;

use crate::{AnimImporter, AnimSource, Curve, CurveValue, ImportError, Interpolation, SourceClip, SourceCurve, SourceJoint, SourceTrack, Trs};

/// Imports skeletons and clips from `.glb` / `.gltf`.
#[derive(Clone, Copy, Debug, Default)]
pub struct GltfAnimImporter;

impl AnimImporter for GltfAnimImporter {
    fn extensions(&self) -> &[&str] {
        &["glb", "gltf"]
    }

    fn version(&self) -> u32 {
        1
    }

    fn import(&self, path: &str, bytes: &[u8]) -> Result<AnimSource, ImportError> {
        let err = |message: String| ImportError { path: path.to_owned(), message };
        let gltf = gltf::Gltf::from_slice(bytes).map_err(|e| err(e.to_string()))?;
        let mut buffers = Vec::new();
        for buffer in gltf.document.buffers() {
            match buffer.source() {
                BufferSource::Bin => buffers.push(gltf.blob.clone().ok_or_else(|| err("missing BIN chunk".into()))?),
                BufferSource::Uri(uri) => {
                    return Err(err(format!("external buffer {uri:?}: ship a .glb instead")));
                }
            }
        }
        let doc = &gltf.document;
        let names = node_names(doc);
        let skeleton = skeleton(doc, &names);
        let get = |b: gltf::Buffer<'_>| buffers.get(b.index()).map(Vec::as_slice);
        let mut clips = Vec::new();
        for (anim, name) in doc.animations().zip(clip_names(doc)) {
            let mut tracks: BTreeMap<usize, SourceTrack> = BTreeMap::new();
            let mut curves = Vec::new();
            let mut duration = 0.0f32;
            for ch in anim.channels() {
                let node = ch.target().node();
                let reader = ch.reader(get);
                let what = || format!("animation {name:?}, node {:?}", names[node.index()]);
                let times: Vec<f32> = reader.read_inputs().ok_or_else(|| err(format!("{}: no key times", what())))?.collect();
                let outputs = reader.read_outputs().ok_or_else(|| err(format!("{}: no key values", what())))?;
                let interpolation = match ch.sampler().interpolation() {
                    GltfInterpolation::Step => Interpolation::Step,
                    GltfInterpolation::Linear => Interpolation::Linear,
                    GltfInterpolation::CubicSpline => Interpolation::CubicSpline,
                };
                let per_key = if interpolation == Interpolation::CubicSpline { 3 } else { 1 };
                if times.is_empty() {
                    continue;
                }
                duration = duration.max(times.iter().copied().fold(0.0, f32::max));
                let track = || SourceTrack { joint: names[node.index()].clone(), ..SourceTrack::default() };
                let bad = |what: String| err(format!("{what}: key count doesn't match the times"));
                match outputs {
                    ReadOutputs::Translations(v) => {
                        let values: Vec<Vec3> = v.map(Vec3::from).collect();
                        let c = curve(&times, values, interpolation).ok_or_else(|| bad(what()))?;
                        tracks.entry(node.index()).or_insert_with(track).translation = Some(c);
                    }
                    ReadOutputs::Rotations(v) => {
                        let values: Vec<Quat> = v.into_f32().map(Quat::from_array).collect();
                        let c = curve(&times, values, interpolation).ok_or_else(|| bad(what()))?;
                        tracks.entry(node.index()).or_insert_with(track).rotation = Some(c);
                    }
                    ReadOutputs::Scales(v) => {
                        let values: Vec<Vec3> = v.map(Vec3::from).collect();
                        let c = curve(&times, values, interpolation).ok_or_else(|| bad(what()))?;
                        tracks.entry(node.index()).or_insert_with(track).scale = Some(c);
                    }
                    ReadOutputs::MorphTargetWeights(w) => {
                        let w: Vec<f32> = w.into_f32().collect();
                        let stride = times.len() * per_key;
                        if stride == 0 || !w.len().is_multiple_of(stride) {
                            return Err(bad(what()));
                        }
                        let targets = w.len() / stride;
                        // glTF layout: per key (and per tangent/value slot), all targets.
                        for k in 0..targets {
                            let values: Vec<f32> = (0..stride).map(|i| w[i * targets + k]).collect();
                            let c = curve(&times, values, interpolation).ok_or_else(|| bad(what()))?;
                            curves.push(SourceCurve { name: format!("{}.morph{k}", names[node.index()]), curve: c });
                        }
                    }
                }
            }
            clips.push(SourceClip { name, duration, tracks: tracks.into_values().collect(), curves, events: Vec::new(), markers: Vec::new() });
        }
        let root_motion = skeleton.as_deref().and_then(|s| crate::detect_root_motion_joint(s, &clips).map(|j| s[j].name.clone()));
        Ok(AnimSource { skeleton, clips, root_motion })
    }
}

/// Skin joints and animated nodes, closed under "parent of", in node order.
fn skeleton(doc: &gltf::Document, names: &[String]) -> Option<Vec<SourceJoint>> {
    let parents = node_parents(doc);
    let mut wanted = vec![false; parents.len()];
    for skin in doc.skins() {
        for j in skin.joints() {
            wanted[j.index()] = true;
        }
    }
    for anim in doc.animations() {
        for ch in anim.channels() {
            if ch.target().property() != Property::MorphTargetWeights {
                wanted[ch.target().node().index()] = true;
            }
        }
    }
    for n in 0..wanted.len() {
        let mut p = if wanted[n] { parents[n] } else { None };
        while let Some(i) = p {
            if wanted[i] {
                break;
            }
            wanted[i] = true;
            p = parents[i];
        }
    }
    if !wanted.contains(&true) {
        return None;
    }
    let joint_of: BTreeMap<usize, usize> = wanted.iter().enumerate().filter(|(_, w)| **w).map(|(n, _)| n).zip(0..).collect();
    let joints = doc
        .nodes()
        .filter(|n| wanted[n.index()])
        .map(|n| {
            let (t, r, s) = n.transform().decomposed();
            SourceJoint {
                name: names[n.index()].clone(),
                parent: parents[n.index()].map(|p| joint_of[&p]),
                rest: Trs::new(Vec3::from(t), Quat::from_array(r).normalize(), Vec3::from(s)),
            }
        })
        .collect();
    Some(joints)
}

/// The clip names a glTF file holds, in file order, as
/// [`GltfAnimImporter`] names them (no buffers are read).
pub fn list_gltf_clips(path: &str, bytes: &[u8]) -> Result<Vec<String>, ImportError> {
    let gltf = gltf::Gltf::from_slice(bytes).map_err(|e| ImportError { path: path.to_owned(), message: e.to_string() })?;
    Ok(clip_names(&gltf.document))
}

/// A curve from glTF keys, dropping keys whose time doesn't increase.
/// `None` if the value count doesn't match.
fn curve<T: CurveValue>(times: &[f32], values: Vec<T>, interpolation: Interpolation) -> Option<Curve<T>> {
    let per_key = if interpolation == Interpolation::CubicSpline { 3 } else { 1 };
    if values.len() != times.len() * per_key {
        return None;
    }
    let mut out = Curve { times: Vec::with_capacity(times.len()), values: Vec::with_capacity(values.len()), interpolation };
    for (i, &t) in times.iter().enumerate() {
        if !t.is_finite() || out.times.last().is_some_and(|&last| t <= last) {
            continue;
        }
        out.times.push(t);
        out.values.extend_from_slice(&values[i * per_key..(i + 1) * per_key]);
    }
    (!out.times.is_empty()).then_some(out)
}

fn unique(names: impl Iterator<Item = (usize, Option<String>)>, unnamed: &str) -> Vec<String> {
    let mut seen = HashSet::new();
    names
        .map(|(i, name)| {
            let base = name.filter(|n| !n.is_empty()).unwrap_or_else(|| format!("{unnamed}{i}"));
            let name = if seen.contains(&base) { format!("{base}#{i}") } else { base };
            seen.insert(name.clone());
            name
        })
        .collect()
}

fn node_names(doc: &gltf::Document) -> Vec<String> {
    unique(doc.nodes().map(|n| (n.index(), n.name().map(str::to_owned))), "node")
}

fn clip_names(doc: &gltf::Document) -> Vec<String> {
    unique(doc.animations().map(|a| (a.index(), a.name().map(str::to_owned))), "anim")
}

fn node_parents(doc: &gltf::Document) -> Vec<Option<usize>> {
    let mut parents = vec![None; doc.nodes().len()];
    for n in doc.nodes() {
        for c in n.children() {
            parents[c.index()] = Some(n.index());
        }
    }
    parents
}
