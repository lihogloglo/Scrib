//! Mesh UV density (textures.md §6 "Demand", slice S3b): how many UV units one metre of a
//! cooked model's surface spans, per material. The texture streamer's wanted level reads it
//! (`texels_per_m = ρ · W`); a material without data falls back to 1 UV per metre.
//!
//! **Definition** (UE's "mesh UV density", the inverse of its texel factor). Per LOD0 triangle,
//! with world area `A` (model-space m², the cooked positions) and UV area `a` (UV channel 0):
//! the triangle's density is `d = sqrt(A / a)` metres per UV unit, weighted by `sqrt(A)`.
//! A material's density is the weighted mean of its triangles' `d` after the lowest and the
//! highest 10 % of the total weight are dropped (outliers: slivers, stretched seams), and
//! `ρ = 1 / d` UV units per metre. Triangles with `A` or `a` at or below [`SMALL_AREA`] are
//! ignored (degenerate, or a UV-collapsed face that samples one texel), so a material whose
//! every triangle is degenerate, or a model without UVs, has no density (`None`).
//!
//! Sources: UE `UStaticMesh::UpdateUVChannelData` + `FUVDensityAccumulator` (StaticMesh.cpp,
//! MeshUVChannelInfo.h: `PushTriangle(area, uv_area)` keeps `sqrt(area / uv_area)` weighted by
//! `sqrt(area)`, `GetUVDensity` drops the weight tails before the weighted mean; one
//! `LocalUVDensities` value per material slot, computed at build, never at runtime); Unity
//! `Mesh.GetUVDistributionMetric` (the mean world-area / UV-area ratio, also baked at import).
//! Read from the engines' public docs and source as remembered, not re-checked line by line.
//!
//! **Scale.** ρ is model space. A placement scaled by `s` covers `s` times more metres per UV, so
//! its ρ is `ρ / s`; the demand bake divides by the smallest axis scale (the densest direction).
//!
//! The `mesh.uv_density` step ([`MeshUvDensityStep`]) is separate from `mesh.cook` on purpose: it
//! reads the cooked product, so changing the rule here re-runs this cheap step only, never the
//! mesh cook (whose key every runtime mesh shares).

use serde::{Deserialize, Serialize};
use sf_bake::{BakeError, PlanCx, ProductKey, RunCx, Step};

use crate::steps::MeshCookStep;
use crate::{Cooked, CookedModel, FormatError};

/// Areas at or below this (m² or UV²) are degenerate (UE's `SMALL_NUMBER`).
pub const SMALL_AREA: f64 = 1e-8;
/// Share of the total weight dropped at each end before the weighted mean (UE's outlier filter).
pub const TAIL: f64 = 0.1;
/// Version of the rule above: part of the step's key.
pub const RULE_VERSION: u32 = 1;

/// One triangle's density and weight: `(sqrt(A / a), sqrt(A))`, or `None` when degenerate.
pub fn triangle_density(p: [[f32; 3]; 3], uv: [[f32; 2]; 3]) -> Option<(f64, f64)> {
    let sub = |a: [f32; 3], b: [f32; 3]| [f64::from(b[0] - a[0]), f64::from(b[1] - a[1]), f64::from(b[2] - a[2])];
    let (e1, e2) = (sub(p[0], p[1]), sub(p[0], p[2]));
    let cross = [e1[1] * e2[2] - e1[2] * e2[1], e1[2] * e2[0] - e1[0] * e2[2], e1[0] * e2[1] - e1[1] * e2[0]];
    let area = 0.5 * (cross[0] * cross[0] + cross[1] * cross[1] + cross[2] * cross[2]).sqrt();
    let (u1, u2) = (
        [f64::from(uv[1][0] - uv[0][0]), f64::from(uv[1][1] - uv[0][1])],
        [f64::from(uv[2][0] - uv[0][0]), f64::from(uv[2][1] - uv[0][1])],
    );
    let uv_area = 0.5 * (u1[0] * u2[1] - u1[1] * u2[0]).abs();
    (area.is_finite() && uv_area.is_finite() && area > SMALL_AREA && uv_area > SMALL_AREA)
        .then(|| ((area / uv_area).sqrt(), area.sqrt()))
}

/// Accumulates triangles of one material (UE's `FUVDensityAccumulator`).
#[derive(Clone, Debug, Default)]
pub struct DensityAccumulator {
    /// `(density in metres per UV, weight)`.
    elements: Vec<(f64, f64)>,
}

impl DensityAccumulator {
    /// Adds one triangle (ignored when degenerate).
    pub fn push(&mut self, p: [[f32; 3]; 3], uv: [[f32; 2]; 3]) {
        if let Some(e) = triangle_density(p, uv) {
            self.elements.push(e);
        }
    }

    /// The material's ρ in UV units per metre (module docs), `None` without a usable triangle.
    pub fn rho(&self) -> Option<f32> {
        let mut e = self.elements.clone();
        e.sort_by(|a, b| a.0.total_cmp(&b.0));
        let total: f64 = e.iter().map(|x| x.1).sum();
        let (lo, hi) = (total * TAIL, total * (1.0 - TAIL));
        let (mut at, mut sum, mut kept) = (0.0, 0.0, 0.0);
        for (d, w) in e {
            // The part of [at, at + w] inside the kept window [lo, hi].
            let part = (at + w).min(hi) - at.max(lo);
            if part > 0.0 {
                sum += d * part;
                kept += part;
            }
            at += w;
        }
        #[allow(clippy::cast_possible_truncation)] // a density in f32 range
        (kept > 0.0 && sum > 0.0).then(|| (kept / sum) as f32)
    }
}

/// ρ per material of `model` (indexed like `model.materials`), from LOD0 and UV channel 0.
pub fn uv_density(model: &CookedModel) -> Result<Vec<Option<f32>>, FormatError> {
    let mut acc = vec![DensityAccumulator::default(); model.materials.len()];
    if model.has_uvs {
        for sub in model.lods.first().map_or(&[][..], |l| &l.submeshes[..]) {
            let Some(slot) = acc.get_mut(sub.material as usize) else { continue };
            let (vertices, indices) = sub.decode()?;
            for &[i0, i1, i2] in indices.as_chunks::<3>().0 {
                let v = |i: u32| vertices.get(i as usize);
                let (Some(a), Some(b), Some(c)) = (v(i0), v(i1), v(i2)) else { continue };
                slot.push([a.position, b.position, c.position], [a.uv, b.uv, c.uv]);
            }
        }
    }
    Ok(acc.iter().map(DensityAccumulator::rho).collect())
}

/// The `mesh.uv_density` product: ρ per material of the cooked model (empty for a model the cook
/// skipped). Postcard.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct UvDensity {
    /// UV units per metre, model space, per material; `None`: no usable UVs (fallback ρ).
    pub materials: Vec<Option<f32>>,
}

impl UvDensity {
    /// The density of one cooked product (`Cooked::Skinned` from its bind pose).
    pub fn of(cooked: &Cooked) -> Result<Self, FormatError> {
        let model = match cooked {
            Cooked::Static(m) => m,
            Cooked::Skinned(s) => &s.model,
            Cooked::Skipped { .. } => return Ok(Self::default()),
        };
        Ok(Self { materials: uv_density(model)? })
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        postcard::to_stdvec(self).expect("UvDensity serializes")
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self, postcard::Error> {
        postcard::from_bytes(bytes)
    }
}

/// `mesh.uv_density`: subject = the model path ([`MeshCookStep`]'s subject). Reads only that
/// model's `mesh.cook` product, a static input, so only a changed cook re-runs it.
pub struct MeshUvDensityStep;

impl MeshUvDensityStep {
    pub const ID: &'static str = "mesh.uv_density";
}

impl Step for MeshUvDensityStep {
    fn id(&self) -> &'static str {
        Self::ID
    }
    fn version(&self) -> u32 {
        1
    }
    fn plan(&self, subject: &str, cx: &mut PlanCx) -> Result<(), BakeError> {
        cx.product(MeshCookStep::ID, subject);
        cx.param("rule", RULE_VERSION.to_le_bytes().to_vec());
        Ok(())
    }
    fn run(&self, subject: &str, cx: &mut RunCx<'_>) -> Result<Vec<u8>, BakeError> {
        let bytes = cx.product(MeshCookStep::ID, subject)?;
        let err = |e: FormatError| BakeError::step(&ProductKey::new(Self::ID, subject), e);
        let cooked = Cooked::from_bytes(&bytes).map_err(err)?;
        Ok(UvDensity::of(&cooked).map_err(err)?.to_bytes())
    }
}

#[cfg(test)]
mod tests;
