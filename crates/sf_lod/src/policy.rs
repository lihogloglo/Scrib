//! Far policy: how each model is represented beyond L0, and the adapter
//! hook that overrides it.

use serde::{Deserialize, Serialize};
use sf_model::{AlphaMode, CookedModel};
use sf_world::Placement;

/// How a model appears in far-field proxies.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum FarPolicy {
    /// Merged into the cluster mesh (default for statics).
    #[default]
    Merge,
    /// Drawn as an octahedral impostor (foliage).
    Impostor,
    /// Not in any proxy.
    Drop,
}

/// What the default policy looks at (from the cooked model).
#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct ModelSummary {
    pub min: [f32; 3],
    pub max: [f32; 3],
    /// Fraction of LOD0's surface area in alpha-masked materials.
    pub masked_area: f32,
    pub triangles: u32,
}

impl ModelSummary {
    pub fn of(model: &CookedModel) -> Self {
        let mut total = 0.0f64;
        let mut masked = 0.0f64;
        let mut triangles = 0u32;
        if let Some(lod) = model.lods.first() {
            for sub in &lod.submeshes {
                let Ok((v, idx)) = sub.decode() else { continue };
                let is_masked = model
                    .materials
                    .get(sub.material as usize)
                    .is_some_and(|m| matches!(m.alpha, AlphaMode::Mask(_) | AlphaMode::Blend));
                for t in idx.as_chunks::<3>().0 {
                    let [a, b, c] = t.map(|i| glam::Vec3::from_array(v[i as usize].position));
                    let area = f64::from((b - a).cross(c - a).length() * 0.5);
                    total += area;
                    if is_masked {
                        masked += area;
                    }
                }
                triangles += sub.index_count / 3;
            }
        }
        #[allow(clippy::cast_possible_truncation)]
        let masked_area = if total > 0.0 { (masked / total) as f32 } else { 0.0 };
        Self { min: model.bounds.min, max: model.bounds.max, masked_area, triangles }
    }

    pub fn height(&self) -> f32 {
        self.max[1] - self.min[1]
    }

    pub fn width(&self) -> f32 {
        (self.max[0] - self.min[0]).max(self.max[2] - self.min[2])
    }
}

/// A light as seen from far away.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct FarLight {
    /// Linear RGB, roughly unit brightness.
    pub color: [f32; 3],
    /// Relative brightness (1 = a typical lantern).
    pub intensity: f32,
    /// Light radius in meters (sprite size scales with it).
    pub radius: f32,
    /// Offset of the light from the placement pivot, in model space.
    pub offset: [f32; 3],
}

impl Default for FarLight {
    /// A warm lantern.
    fn default() -> Self {
        Self { color: [1.0, 0.72, 0.42], intensity: 1.0, radius: 6.0, offset: [0.0; 3] }
    }
}

/// Adapter hook (Bevy-free): far policy and light parameters. Only the
/// adapter knows which of its models are trees and what its lights look
/// like; the framework defaults are heuristics.
pub trait FarSource: Send + Sync {
    /// Changes whenever any answer changes (part of every `lod.*` key).
    fn fingerprint(&self) -> [u8; 32];
    /// Override the default policy of a model.
    fn policy(&self, _model: &str, _summary: &ModelSummary) -> Option<FarPolicy> {
        None
    }
    /// Light parameters of a `Light` placement; `None` uses
    /// [`FarLight::default`].
    fn light(&self, _placement: &Placement) -> Option<FarLight> {
        None
    }
    /// Keep a placement out of its cell's near precombine (precombine.md):
    /// what only the content pack knows, e.g. statics a script can disable
    /// or move. Part of `lod.near`'s key through [`FarSource::fingerprint`].
    fn precombine_exclude(&self, _placement: &Placement) -> bool {
        false
    }
}

/// No overrides.
#[derive(Clone, Copy, Debug, Default)]
pub struct DefaultFarSource;

impl FarSource for DefaultFarSource {
    fn fingerprint(&self) -> [u8; 32] {
        [0; 32]
    }
}

/// The framework's default: mostly alpha-masked and taller than wide
/// means foliage (impostor); everything else merges.
pub fn default_policy(s: &ModelSummary) -> FarPolicy {
    if s.triangles == 0 {
        FarPolicy::Drop
    } else if s.masked_area > 0.5 && s.height() > s.width() {
        FarPolicy::Impostor
    } else {
        FarPolicy::Merge
    }
}

/// Whether a placement may be represented by far-field proxies at all
/// (by class and flags; the model's policy decides how). Shared by the
/// bake and the streaming spawner, which ends covered objects at `d1`.
pub fn far_eligible(p: &Placement) -> bool {
    p.far_eligible()
}
