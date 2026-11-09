//! Which placements live in the static scene, and promotion out of it (static-scene.md §1).
//!
//! The framework knows no game's record kinds or scripting: the adapter answers
//! [`SceneEligibility`] with plain facts, and [`classify`] applies the §1 rule (the precombine
//! "Members" rule, widened). A mixed model splits: opaque and alpha-masked parts go in the scene,
//! blended and additive parts spawn as an entity sharing the placement id.
//!
//! [`Promotions`] records, per loaded cell, the instances gameplay took over (a script or console
//! targets it, physics wakes it, a mod edits it, the player grabs it). One-way while the cell is
//! loaded; dropped with the cell (demotion is the unload). Never saved.

use std::collections::BTreeSet;

use crate::PlacementId;

/// What a placement's base record is, as far as the scene cares.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ClassKind {
    /// Plain scenery: the only kind the scene takes.
    Static,
    Door,
    Container,
    Item,
    Light,
    Actor,
    Activator,
    /// Editor or logic markers (never drawn as scenery).
    Marker,
}

/// How one model part (submesh) blends.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum MaterialKind {
    Opaque,
    AlphaMasked,
    /// Sorted alpha blending.
    Blended,
    Additive,
}

impl MaterialKind {
    /// Opaque and alpha-masked parts can be drawn unsorted from the table.
    pub fn in_scene(self) -> bool {
        matches!(self, Self::Opaque | Self::AlphaMasked)
    }
}

/// The adapter's facts about a placement (§1). The framework never names a game's rules: the
/// game's adapter maps its record kinds, scripts and model data onto these answers.
pub trait SceneEligibility {
    /// The base record's kind.
    fn class_kind(&self, placement: PlacementId) -> ClassKind;
    /// A script is attached to the base record or the placement.
    fn has_script(&self, placement: PlacementId) -> bool;
    /// A record or script names this placement (or its base) so gameplay may touch it.
    fn is_referenced(&self, placement: PlacementId) -> bool;
    /// The model went through the mesh cook (has a cooked LOD chain).
    fn model_cooked(&self, placement: PlacementId) -> bool;
    /// The model has controllers animating transforms, visibility or materials per frame. UV
    /// scroll and wind sway don't count: they are shader functions of time.
    fn has_animated_controllers(&self, placement: PlacementId) -> bool;
    /// The blend kind of each model part, in part order.
    fn part_materials(&self, placement: PlacementId) -> &[MaterialKind];
}

/// Why a placement stays an entity.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum EntityReason {
    /// A per-placement switch bars it from the scene whatever its record says: editor-only,
    /// no-LOD, near-only or starts-disabled (`sf_scene::bake::BARRED`, as the precombine
    /// "Members" rule `sf_lod::near::near_eligible` bars them). The framework answers this
    /// from the placement's own flags, never the adapter (`bake::classify_placement`).
    Barred,
    Class(ClassKind),
    Scripted,
    Referenced,
    NotCooked,
    Animated,
    /// Every part is blended or additive (or the model has none).
    NoScenePart,
}

/// Where a placement goes.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Eligibility {
    /// Every part in the static scene.
    Scene,
    /// Opaque/masked parts (part indices) in the scene; the rest spawn as an entity with the same
    /// placement id.
    Split { scene: Vec<usize>, entity: Vec<usize> },
    /// An entity, as today.
    Entity(EntityReason),
}

/// The §1 rule over the adapter's facts. Checks run cheapest first; the first failing one is
/// the reason.
///
/// This sees no per-placement switches: they are framework data, so the bake applies them
/// itself ([`EntityReason::Barred`], `bake::classify_placement`). Call that, not this, when you
/// have the placement.
pub fn classify(facts: &(impl SceneEligibility + ?Sized), placement: PlacementId) -> Eligibility {
    let class = facts.class_kind(placement);
    if class != ClassKind::Static {
        return Eligibility::Entity(EntityReason::Class(class));
    }
    if facts.has_script(placement) {
        return Eligibility::Entity(EntityReason::Scripted);
    }
    if facts.is_referenced(placement) {
        return Eligibility::Entity(EntityReason::Referenced);
    }
    if !facts.model_cooked(placement) {
        return Eligibility::Entity(EntityReason::NotCooked);
    }
    if facts.has_animated_controllers(placement) {
        return Eligibility::Entity(EntityReason::Animated);
    }
    let parts = facts.part_materials(placement);
    let (scene, entity): (Vec<usize>, Vec<usize>) = (0..parts.len()).partition(|&k| parts[k].in_scene());
    match (scene.is_empty(), entity.is_empty()) {
        (true, _) => Eligibility::Entity(EntityReason::NoScenePart),
        (false, true) => Eligibility::Scene,
        (false, false) => Eligibility::Split { scene, entity },
    }
}

/// One loaded cell's promoted placements (§1 "Promotion"). Drop it when the cell unloads.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Promotions {
    promoted: BTreeSet<PlacementId>,
}

impl Promotions {
    pub fn new() -> Self {
        Self::default()
    }

    /// Promotes `placement`. True the first time: the caller then deactivates the instance (sets
    /// [`crate::InstanceFlags::INACTIVE`] on its GPU copy) and spawns the entity. False after:
    /// promotion is one-way, there is nothing to undo until the cell unloads.
    pub fn promote(&mut self, placement: PlacementId) -> bool {
        self.promoted.insert(placement)
    }

    pub fn is_promoted(&self, placement: PlacementId) -> bool {
        self.promoted.contains(&placement)
    }

    pub fn len(&self) -> usize {
        self.promoted.len()
    }

    pub fn is_empty(&self) -> bool {
        self.promoted.is_empty()
    }

    /// Promoted placements, ascending.
    pub fn promoted(&self) -> impl Iterator<Item = PlacementId> + '_ {
        self.promoted.iter().copied()
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;

    #[derive(Default)]
    struct Facts {
        class: HashMap<u32, ClassKind>,
        scripted: Vec<u32>,
        referenced: Vec<u32>,
        raw: Vec<u32>,
        animated: Vec<u32>,
        parts: HashMap<u32, Vec<MaterialKind>>,
    }

    impl SceneEligibility for Facts {
        fn class_kind(&self, p: u32) -> ClassKind {
            self.class.get(&p).copied().unwrap_or(ClassKind::Static)
        }
        fn has_script(&self, p: u32) -> bool {
            self.scripted.contains(&p)
        }
        fn is_referenced(&self, p: u32) -> bool {
            self.referenced.contains(&p)
        }
        fn model_cooked(&self, p: u32) -> bool {
            !self.raw.contains(&p)
        }
        fn has_animated_controllers(&self, p: u32) -> bool {
            self.animated.contains(&p)
        }
        fn part_materials(&self, p: u32) -> &[MaterialKind] {
            self.parts.get(&p).map_or(&[Opaque][..], Vec::as_slice)
        }
    }

    use MaterialKind::{Additive, AlphaMasked, Blended, Opaque};

    #[test]
    fn plain_static_is_in_the_scene() {
        let mut f = Facts::default();
        f.parts.insert(2, vec![Opaque, AlphaMasked, Opaque]);
        assert_eq!(classify(&f, 1), Eligibility::Scene);
        assert_eq!(classify(&f, 2), Eligibility::Scene);
    }

    #[test]
    fn non_static_classes_stay_entities() {
        let mut f = Facts::default();
        let kinds = [
            ClassKind::Door,
            ClassKind::Container,
            ClassKind::Item,
            ClassKind::Light,
            ClassKind::Actor,
            ClassKind::Activator,
            ClassKind::Marker,
        ];
        for (p, k) in (0..).zip(kinds) {
            f.class.insert(p, k);
        }
        for (p, k) in (0..).zip(kinds) {
            assert_eq!(classify(&f, p), Eligibility::Entity(EntityReason::Class(k)));
        }
    }

    #[test]
    fn scripted_referenced_uncooked_and_animated_stay_entities() {
        let f = Facts { scripted: vec![1], referenced: vec![2], raw: vec![3], animated: vec![4], ..Facts::default() };
        assert_eq!(classify(&f, 1), Eligibility::Entity(EntityReason::Scripted));
        assert_eq!(classify(&f, 2), Eligibility::Entity(EntityReason::Referenced));
        assert_eq!(classify(&f, 3), Eligibility::Entity(EntityReason::NotCooked));
        assert_eq!(classify(&f, 4), Eligibility::Entity(EntityReason::Animated));
        // A door's class wins over its script: the cheapest check reports.
        let mut f = Facts { scripted: vec![5], ..Facts::default() };
        f.class.insert(5, ClassKind::Door);
        assert_eq!(classify(&f, 5), Eligibility::Entity(EntityReason::Class(ClassKind::Door)));
    }

    #[test]
    fn mixed_model_splits_and_all_blended_stays_entity() {
        let mut f = Facts::default();
        f.parts.insert(1, vec![Opaque, Blended, AlphaMasked, Additive]);
        f.parts.insert(2, vec![Blended, Additive]);
        f.parts.insert(3, vec![]);
        assert_eq!(classify(&f, 1), Eligibility::Split { scene: vec![0, 2], entity: vec![1, 3] });
        assert_eq!(classify(&f, 2), Eligibility::Entity(EntityReason::NoScenePart));
        assert_eq!(classify(&f, 3), Eligibility::Entity(EntityReason::NoScenePart));
    }

    #[test]
    fn promotion_is_one_way() {
        let mut p = Promotions::new();
        assert!(p.is_empty());
        assert!(p.promote(7));
        assert!(!p.promote(7));
        assert!(p.promote(3));
        assert!(p.is_promoted(7) && p.is_promoted(3) && !p.is_promoted(4));
        assert_eq!(p.promoted().collect::<Vec<_>>(), [3, 7]);
        assert_eq!(p.len(), 2);
    }
}
