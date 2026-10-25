//! Smooth bodies at outfit assembly (spec `docs/specs/smooth-bodies.md`,
//! "Where it runs"): the visible parts of one outfit go through
//! [`smooth_body`] as one segmented set, then [`guard_seams`] over the
//! caller's probe poses (never worse than the original, seam by seam), and
//! the parts the step changed come back as [`DecodedPart`]s for
//! [`assemble`](crate::assemble): rest-space positions and normals, skin
//! weights on a rest-space palette ([`bind_palette`]).
//!
//! Then seam shading ([`blend_seam_normals`]) over every visible part:
//! rims take the normal of the surface they meet, so lighting doesn't step
//! at the seams.
//!
//! Left out (kept as authored): parts the caller excludes (hair: it stays
//! on its joint), parts with a joint the skeleton lacks. Parts with morph
//! targets (heads keep their own mesh and bind-space deltas) keep their
//! weights and positions; only their seam normals change, written back in
//! their own bind space. A set with fewer than two parts, or no probe poses,
//! is left unchanged.

use glam::{Affine3A, Mat3, Mat4};
use sf_model::{FormatError, MAX_INFLUENCES, SkinVertex};

use crate::assemble::{AssemblyPart, DecodedPart};
use crate::smooth::{GuardReport, SkinSurface, SmoothSettings, bind_palette, blend_seam_normals, guard_seams, skinning_matrices, smooth_body};

/// The skeleton the outfit is smoothed on, with the probe poses the guard
/// checks seams in. Built once per skeleton.
#[derive(Clone, Debug, Default)]
pub struct SmoothRig {
    pub names: Vec<String>,
    pub parents: Vec<Option<usize>>,
    /// Rest pose, model space.
    pub rest_model: Vec<Affine3A>,
    /// Skinning matrices (`posed · rest⁻¹`) per probe pose.
    pub probes: Vec<Vec<Affine3A>>,
}

impl SmoothRig {
    /// `posed`: model-space joint transforms per probe pose (bent elbows and
    /// knees, raised shoulders, locomotion frames: the adapter's choice).
    pub fn new(names: Vec<String>, parents: Vec<Option<usize>>, rest_model: Vec<Affine3A>, posed: &[Vec<Affine3A>]) -> Self {
        let probes = posed.iter().filter(|p| p.len() == rest_model.len()).map(|p| skinning_matrices(&rest_model, p)).collect();
        Self { names, parents, rest_model, probes }
    }

    fn joint(&self, name: &str) -> Option<u16> {
        self.names.iter().position(|n| n == name).and_then(|j| u16::try_from(j).ok())
    }
}

/// Settings of the assembly step.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct OutfitSmoothing {
    pub smooth: SmoothSettings,
    /// A seam may open this much (model units) more than the original in a
    /// probe pose before the guard reverts it (survey: 0.5 mm).
    pub guard_tolerance: f32,
    /// Seam shading ([`blend_seam_normals`]).
    pub seam_normals: bool,
}

impl Default for OutfitSmoothing {
    fn default() -> Self {
        Self { smooth: SmoothSettings::default(), guard_tolerance: 0.0005, seam_normals: true }
    }
}

/// What [`smooth_outfit`] did.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct OutfitSmoothReport {
    /// Parts that went into the step, and those it rewrote.
    pub parts: usize,
    pub rewritten: usize,
    /// Seam links, welded vertices, re-solved vertices.
    pub seams: usize,
    pub welded: usize,
    pub band: usize,
    pub guard: GuardReport,
    /// Rim vertices whose normal seam shading changed.
    pub normals: usize,
}

/// Smooths the outfit `parts` at `lod` (module docs). `exclude[i]` keeps
/// part `i` as authored. Returns one entry per part: `Some` for the parts
/// the step rewrote (pass it as [`AssemblyPart::decoded`]), `None` for the
/// rest.
pub fn smooth_outfit(
    parts: &[AssemblyPart<'_>],
    exclude: &[bool],
    lod: usize,
    rig: &SmoothRig,
    settings: &OutfitSmoothing,
) -> Result<(Vec<Option<DecodedPart>>, OutfitSmoothReport), FormatError> {
    let mut out: Vec<Option<DecodedPart>> = vec![None; parts.len()];
    let mut report = OutfitSmoothReport::default();
    if rig.probes.is_empty() {
        return Ok((out, report));
    }
    // (part index, decoded submeshes, rest surface, shading only)
    let mut set: Vec<(usize, DecodedPart, SkinSurface, bool)> = Vec::new();
    for (i, part) in parts.iter().enumerate() {
        if exclude.get(i).copied().unwrap_or(false) {
            continue;
        }
        let Ok(surface) = SkinSurface::from_cooked(part.cooked, lod, part.body_part, |n| rig.joint(n), &rig.rest_model) else { continue };
        if surface.triangles.is_empty() {
            continue;
        }
        let decoded = DecodedPart::decode(part.cooked, lod, part.body_part)?;
        debug_assert_eq!(decoded.submeshes.iter().map(|s| s.vertices.len()).sum::<usize>(), surface.positions.len());
        set.push((i, decoded, surface, !part.cooked.morph_targets.is_empty()));
    }
    if set.len() < 2 {
        return Ok((out, report));
    }
    // Weights and welds: parts without morph targets.
    let smoothable: Vec<usize> = (0..set.len()).filter(|&k| !set[k].3).collect();
    let original: Vec<SkinSurface> = smoothable.iter().map(|&k| set[k].2.clone()).collect();
    let mut smoothed = original.clone();
    if smoothed.len() >= 2 {
        let run = smooth_body(&mut smoothed, &rig.parents, &settings.smooth);
        report.guard = guard_seams(&original, &mut smoothed, &run.seams, &rig.probes, settings.guard_tolerance, settings.smooth.band);
        report.seams = run.seams.len();
        report.welded = run.welded;
        report.band = run.band;
    }
    report.parts = smoothable.len();
    let mut after: Vec<SkinSurface> = set.iter().map(|(_, _, s, _)| s.clone()).collect();
    for (&k, s) in smoothable.iter().zip(smoothed) {
        after[k] = s;
    }
    // Shading: every part, heads included.
    if settings.seam_normals {
        report.normals = blend_seam_normals(&mut after, &rig.parents, &settings.smooth);
    }
    for ((index, mut decoded, before, shading_only), after) in set.into_iter().zip(after) {
        if before == after {
            continue;
        }
        if shading_only {
            rewrite_normals(&mut decoded, &before, &after, rig);
        } else {
            rewrite(&mut decoded, &after, rig);
        }
        out[index] = Some(decoded);
        report.rewritten += 1;
    }
    Ok((out, report))
}

/// Writes changed rest-space normals back into a part that keeps its own
/// palette and bind space (morph-bearing parts): each changed normal goes
/// through the inverse of the vertex's blended bind-to-rest transform.
fn rewrite_normals(decoded: &mut DecodedPart, before: &SkinSurface, after: &SkinSurface, rig: &SmoothRig) {
    let to_rest: Vec<Option<Mat3>> = decoded
        .joints
        .iter()
        .map(|j| {
            rig.joint(&j.name).map(|r| Mat3::from_mat4(Mat4::from(rig.rest_model[usize::from(r)]) * Mat4::from_cols_array(&j.inverse_bind)))
        })
        .collect();
    let mut k = 0;
    for sub in &mut decoded.submeshes {
        for (v, s) in sub.vertices.iter_mut().zip(&sub.skin) {
            if before.normals[k] != after.normals[k] {
                let m: Mat3 = (0..MAX_INFLUENCES)
                    .filter(|&i| s.weights[i] > 0.0)
                    .filter_map(|i| to_rest.get(usize::from(s.joints[i])).copied().flatten().map(|m| m * s.weights[i]))
                    .fold(Mat3::ZERO, |a, b| a + b);
                if m.determinant().abs() > f32::EPSILON {
                    v.normal = (m.inverse() * after.normals[k]).normalize_or_zero().to_array();
                }
            }
            k += 1;
        }
    }
}

/// Writes a smoothed rest-space surface back into its decoded submeshes,
/// on a palette of the joints it uses (inverse bind `rest⁻¹`).
fn rewrite(decoded: &mut DecodedPart, surface: &SkinSurface, rig: &SmoothRig) {
    let mut used: Vec<u16> = surface.skin.iter().flat_map(|s| s.joints.iter().zip(&s.weights).filter(|(_, w)| **w > 0.0).map(|(j, _)| *j)).collect();
    used.sort_unstable();
    used.dedup();
    if used.is_empty() {
        used.push(0);
    }
    let names: Vec<String> = used.iter().map(|&j| rig.names[usize::from(j)].clone()).collect();
    let rest: Vec<Affine3A> = used.iter().map(|&j| rig.rest_model[usize::from(j)]).collect();
    decoded.joints = bind_palette(&names, &rest);
    let slot = |j: u16| used.binary_search(&j).ok().and_then(|k| u16::try_from(k).ok()).unwrap_or(0);
    let mut k = 0;
    for sub in &mut decoded.submeshes {
        for (v, s) in sub.vertices.iter_mut().zip(&mut sub.skin) {
            v.position = surface.positions[k].to_array();
            v.normal = surface.normals[k].to_array();
            let src = surface.skin[k];
            let mut skin = SkinVertex { joints: [0; MAX_INFLUENCES], weights: [0.0; MAX_INFLUENCES] };
            for n in 0..MAX_INFLUENCES {
                if src.weights[n] > 0.0 {
                    skin.joints[n] = slot(src.joints[n]);
                    skin.weights[n] = src.weights[n];
                }
            }
            *s = skin;
            k += 1;
        }
    }
}
