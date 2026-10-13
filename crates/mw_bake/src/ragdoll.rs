//! `ragdoll.cook` for every actor skeleton (physics-adapter.md "Ragdolls"):
//! the framework step (`sf_ragdoll::steps`) over the `skeleton.cook` and
//! `rig.profile` subjects of [`ActorSets`], with Morrowind's rules
//! (`mw_physics::ragdoll_params`: humanoids 75 kg, ghosts keep their death
//! clip). First-person skeletons get none.

use std::collections::BTreeSet;
use std::sync::Arc;
use std::time::{Duration, Instant};

use sf_bake::{Baker, ProductKey};
use sf_ragdoll::steps::{RAGDOLL_COOK, RagdollCookStep, RagdollRules};
use sf_ragdoll::{CookParams, RagJointKind, RagdollDef, RagdollPolicy};

use crate::anim::ActorSets;

/// Morrowind's ragdoll rules: humanoid skeletons by subject.
pub struct MwRagdollRules {
    humanoid: BTreeSet<String>,
}

impl RagdollRules for MwRagdollRules {
    fn params(&self, skeleton: &str) -> CookParams {
        mw_physics::ragdoll_params(skeleton, self.humanoid.contains(skeleton))
    }
}

/// Skeleton subjects that get a ragdoll (all but first-person ones).
pub fn ragdoll_subjects(sets: &ActorSets) -> Vec<String> {
    sets.sets.keys().filter(|s| !s.to_ascii_lowercase().contains(".1st")).cloned().collect()
}

/// Registers `ragdoll.cook` with the rules for these sets.
pub fn register(baker: &mut Baker, sets: &ActorSets) {
    let humanoid = sets.sets.values().filter(|s| s.humanoid).map(|s| s.skeleton.clone()).collect();
    baker.register(RagdollCookStep(Arc::new(MwRagdollRules { humanoid })));
}

/// One skeleton's cooked ragdoll, for the census.
#[derive(Clone, Debug)]
pub struct RagdollRow {
    pub skeleton: String,
    pub humanoid: bool,
    pub actors: u32,
    pub policy: RagdollPolicy,
    pub bodies: usize,
    pub hinges: usize,
    pub mass: f32,
    pub regions: Vec<String>,
}

#[derive(Clone, Debug, Default)]
pub struct RagdollReport {
    pub rows: Vec<RagdollRow>,
    /// `(skeleton, error)`.
    pub failed: Vec<(String, String)>,
    pub wall: Duration,
}

/// Cooks the ragdolls of every actor skeleton (warm: cache hits).
pub fn bake_ragdolls(baker: &mut Baker, sets: &ActorSets) -> RagdollReport {
    register(baker, sets);
    let subjects = ragdoll_subjects(sets);
    let keys: Vec<ProductKey> = subjects.iter().map(|s| ProductKey::new(RAGDOLL_COOK, s.as_str())).collect();
    let t = Instant::now();
    let report = baker.bake(&keys);
    let mut out = RagdollReport { wall: t.elapsed(), ..RagdollReport::default() };
    out.failed = report.failures().map(|(k, e)| (k.subject.clone(), e.to_string())).collect();
    let manifest = baker.manifest();
    for s in &subjects {
        let Some(def) = manifest.get(RAGDOLL_COOK, s).and_then(|d| baker.store().get(&d).ok()).and_then(|b| RagdollDef::from_bytes(&b).ok()) else {
            continue;
        };
        let set = &sets.sets[s];
        out.rows.push(RagdollRow {
            skeleton: s.clone(),
            humanoid: set.humanoid,
            actors: set.actors,
            policy: def.policy,
            bodies: def.bodies.len(),
            hinges: def.joints.iter().filter(|j| matches!(j.kind, RagJointKind::Hinge { .. })).count(),
            mass: def.total_mass,
            regions: def.regions.clone(),
        });
    }
    out
}
