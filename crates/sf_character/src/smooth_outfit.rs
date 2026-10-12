//! Smooth bodies at outfit assembly (spec `docs/specs/smooth-bodies.md`,
//! "Where it runs"): the visible parts of one outfit go through
//! [`smooth_body`] as one segmented set, then [`guard_seams`] over the
//! caller's probe poses (never worse than the original, seam by seam), and
//! the parts the step changed come back as [`DecodedPart`]s for
//! [`assemble`](crate::assemble): rest-space positions and normals, skin
//! weights on a rest-space palette ([`bind_palette`]).
//!
//! Left out (kept as authored): parts with morph targets (heads keep their
//! own mesh and bind-space deltas), parts the caller excludes (hair: it
//! stays on its joint), parts with a joint the skeleton lacks. A set with
//! fewer than two smoothable parts, or no probe poses, is left unchanged.

use glam::Affine3A;
use sf_model::{FormatError, MAX_INFLUENCES, SkinVertex};

use crate::assemble::{AssemblyPart, DecodedPart};
use crate::smooth::{GuardReport, SkinSurface, SmoothSettings, bind_palette, guard_seams, skinning_matrices, smooth_body};

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
}

impl Default for OutfitSmoothing {
    fn default() -> Self {
        Self { smooth: SmoothSettings::default(), guard_tolerance: 0.0005 }
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
    // (part index, decoded submeshes, rest surface)
    let mut set: Vec<(usize, DecodedPart, SkinSurface)> = Vec::new();
    for (i, part) in parts.iter().enumerate() {
        if exclude.get(i).copied().unwrap_or(false) || !part.cooked.morph_targets.is_empty() {
            continue;
        }
        let Ok(surface) = SkinSurface::from_cooked(part.cooked, lod, part.body_part, |n| rig.joint(n), &rig.rest_model) else { continue };
        if surface.triangles.is_empty() {
            continue;
        }
        let decoded = DecodedPart::decode(part.cooked, lod, part.body_part)?;
        debug_assert_eq!(decoded.submeshes.iter().map(|s| s.vertices.len()).sum::<usize>(), surface.positions.len());
        set.push((i, decoded, surface));
    }
    if set.len() < 2 {
        return Ok((out, report));
    }
    let original: Vec<SkinSurface> = set.iter().map(|(_, _, s)| s.clone()).collect();
    let mut smoothed = original.clone();
    let run = smooth_body(&mut smoothed, &rig.parents, &settings.smooth);
    report.guard = guard_seams(&original, &mut smoothed, &run.seams, &rig.probes, settings.guard_tolerance, settings.smooth.band);
    report.parts = set.len();
    report.seams = run.seams.len();
    report.welded = run.welded;
    report.band = run.band;
    for ((index, mut decoded, before), after) in set.into_iter().zip(smoothed) {
        if before == after {
            continue;
        }
        rewrite(&mut decoded, &after, rig);
        out[index] = Some(decoded);
        report.rewritten += 1;
    }
    Ok((out, report))
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
