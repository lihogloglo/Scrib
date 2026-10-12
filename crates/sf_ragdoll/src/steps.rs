//! The `ragdoll.cook` bake step (subject: a `skeleton.cook` subject).

use std::sync::Arc;

use sf_anim::steps::{RIG_PROFILE, SKELETON_COOK};
use sf_anim::{RigProfile, Skeleton};
use sf_bake::{BakeError, PlanCx, ProductKey, RunCx, Step};

use crate::{CookParams, RAGDOLL_FORMAT, cook};

pub use crate::RAGDOLL_COOK;

/// Adapter rules per skeleton: policy, total mass, overrides (physics.md
/// "Adapter hooks"). Part of the bake key through [`CookParams::key_bytes`].
pub trait RagdollRules: Send + Sync {
    fn params(&self, skeleton: &str) -> CookParams;
}

/// Framework defaults (volume × water density, no overrides).
pub struct DefaultRagdollRules;

impl RagdollRules for DefaultRagdollRules {
    fn params(&self, _skeleton: &str) -> CookParams {
        CookParams::default()
    }
}

/// `ragdoll.cook`: the skeleton's [`crate::RagdollDef`] from its
/// `skeleton.cook` and `rig.profile` products.
pub struct RagdollCookStep(pub Arc<dyn RagdollRules>);

impl Step for RagdollCookStep {
    fn id(&self) -> &'static str {
        RAGDOLL_COOK
    }
    fn version(&self) -> u32 {
        1
    }
    fn plan(&self, subject: &str, cx: &mut PlanCx) -> Result<(), BakeError> {
        cx.product(SKELETON_COOK, subject).product(RIG_PROFILE, subject);
        cx.param("params", self.0.params(subject).key_bytes());
        cx.param("format", RAGDOLL_FORMAT.to_le_bytes().to_vec());
        Ok(())
    }
    fn run(&self, subject: &str, cx: &mut RunCx<'_>) -> Result<Vec<u8>, BakeError> {
        let key = ProductKey::new(RAGDOLL_COOK, subject);
        let skeleton = Skeleton::from_bytes(&cx.product(SKELETON_COOK, subject)?).map_err(|e| BakeError::step(&key, format!("skeleton: {e}")))?;
        let profile = RigProfile::from_bytes(&cx.product(RIG_PROFILE, subject)?).map_err(|e| BakeError::step(&key, format!("rig profile: {e}")))?;
        let def = cook(&skeleton, &profile, &self.0.params(subject)).map_err(|e| BakeError::step(&key, e))?;
        Ok(def.to_bytes())
    }
}
