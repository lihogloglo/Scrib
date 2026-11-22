//! Override tests: priority resolution, exclusion, paint, explicit placement, masks (exact assertions).
#![allow(clippy::float_cmp, reason = "logic tests assert exact values (the scatter is deterministic IEEE arithmetic)")]

use crate::tests::{flat, on_channel0, species};
use crate::*;

const TILE: TileKey = TileKey::new(0, 0);

fn ws(id: &str, weight: f32) -> WeightedSpecies {
    WeightedSpecies { species: id.into(), weight }
}

fn only(region: Region, priority: i32, ids: &[&str]) -> OnlyRegion {
    OnlyRegion { region, priority, species: ids.iter().map(|id| ws(id, 1.0)).collect() }
}

fn scatter(cover: &Groundcover, set: &OverrideSet, s: SpeciesIndex) -> Vec<Instance> {
    let ov = Overrides::new(cover, set).unwrap();
    let mut out = Vec::new();
    Scatterer::new(cover, &ov, ScatterSettings::default()).scatter_tile(&flat(&[1.0]), TILE, s, 0, &mut out);
    out
}

fn density(cover: &Groundcover, set: &OverrideSet, x: f64, z: f64, s: SpeciesIndex) -> f32 {
    let ov = Overrides::new(cover, set).unwrap();
    Scatterer::new(cover, &ov, ScatterSettings::default()).density_at(&flat(&[1.0]), x, z, s)
}

#[test]
fn overlapping_only_regions_resolve_to_the_highest_priority() {
    let cover = on_channel0(vec![species("a", 0.5, 2.0), species("b", 0.5, 2.0), species("c", 0.5, 2.0)]);
    let (a, b, c) = (SpeciesIndex(0), SpeciesIndex(1), SpeciesIndex(2));
    // Low: x in [0, 12], only a. High: x in [6, 16], only b. Listed high first so order cannot be what wins.
    let set = OverrideSet {
        only: vec![only(Region::rect([6.0, -1.0], [16.0, 17.0]), 5, &["b"]), only(Region::rect([0.0, -1.0], [12.0, 17.0]), 1, &["a"])],
        ..OverrideSet::default()
    };
    let d = |x, s| density(&cover, &set, x, 8.0, s);
    // Only the low region: a.
    assert_eq!((d(3.0, a), d(3.0, b), d(3.0, c)), (2.0, 0.0, 0.0));
    // Overlap: the high one wins.
    assert_eq!((d(9.0, a), d(9.0, b), d(9.0, c)), (0.0, 2.0, 0.0));
    // Outside both: the rule table.
    assert_eq!((d(20.0, a), d(20.0, b), d(20.0, c)), (2.0, 2.0, 2.0));
    // The scatter agrees.
    assert!(scatter(&cover, &set, a).iter().all(|i| i.local[0] <= 6.0));
    assert!(scatter(&cover, &set, b).iter().all(|i| i.local[0] >= 6.0));
    assert!(scatter(&cover, &set, c).is_empty());
    // Equal priority: the later region wins.
    let tie = OverrideSet {
        only: vec![only(Region::rect([0.0, 0.0], [16.0, 16.0]), 2, &["a"]), only(Region::rect([0.0, 0.0], [16.0, 16.0]), 2, &["b"])],
        ..OverrideSet::default()
    };
    assert_eq!((density(&cover, &tie, 8.0, 8.0, a), density(&cover, &tie, 8.0, 8.0, b)), (0.0, 2.0));
    // An "only" weight replaces the rule weight.
    let weighted = OverrideSet {
        only: vec![OnlyRegion { region: Region::circle([8.0, 8.0], 4.0), priority: 0, species: vec![ws("c", 0.25)] }],
        ..OverrideSet::default()
    };
    assert_eq!(density(&cover, &weighted, 8.0, 8.0, c), 0.5);
}

#[test]
fn exclusion_zeroes_density() {
    let cover = on_channel0(vec![species("a", 0.5, 2.0)]);
    let a = SpeciesIndex(0);
    let full = OverrideSet { exclusions: vec![Region::rect([-1.0, -1.0], [17.0, 17.0])], ..OverrideSet::default() };
    assert!(scatter(&cover, &full, a).is_empty());
    let disc = OverrideSet { exclusions: vec![Region::circle([8.0, 8.0], 5.0)], ..OverrideSet::default() };
    let out = scatter(&cover, &disc, a);
    assert!(!out.is_empty());
    assert!(out.iter().all(|i| (i.local[0] - 8.0).hypot(i.local[2] - 8.0) > 5.0));
    assert_eq!(density(&cover, &disc, 8.0, 8.0, a), 0.0);
    assert_eq!(density(&cover, &disc, 15.0, 15.0, a), 2.0);
    // Exclusion beats an "only" region.
    let both = OverrideSet { only: vec![only(Region::circle([8.0, 8.0], 8.0), 9, &["a"])], ..disc.clone() };
    assert_eq!(density(&cover, &both, 8.0, 8.0, a), 0.0);
}

#[test]
fn paint_multiplies_its_species_group_only() {
    let cover = on_channel0(vec![species("a", 0.5, 2.0), species("b", 0.5, 2.0)]);
    let (a, b) = (SpeciesIndex(0), SpeciesIndex(1));
    // 2×2 texels over the tile: (0,0)=0, (1,0)=51, (0,1)=255, (1,1)=102.
    let paint = DensityPaint { origin: [0.0, 0.0], size_m: 16.0, resolution: 2, texels: vec![0, 51, 255, 102], species: vec!["a".into()] };
    let set = OverrideSet { paints: vec![paint.clone()], ..OverrideSet::default() };
    assert_eq!(density(&cover, &set, 4.0, 4.0, a), 0.0);
    assert_eq!(density(&cover, &set, 12.0, 4.0, a), 2.0 * (51.0 / 255.0));
    assert_eq!(density(&cover, &set, 4.0, 12.0, a), 2.0);
    assert_eq!(density(&cover, &set, 12.0, 12.0, a), 2.0 * (102.0 / 255.0));
    assert_eq!(density(&cover, &set, 20.0, 4.0, a), 2.0, "outside the paint");
    assert_eq!(density(&cover, &set, 4.0, 4.0, b), 2.0, "b is not in the group");
    // The scatter leaves the zero texel empty.
    assert!(scatter(&cover, &set, a).iter().all(|i| !(i.local[0] < 8.0 && i.local[2] < 8.0)));
    // Two paints multiply; an empty group means every species.
    let all = DensityPaint { species: vec![], texels: vec![128; 4], ..paint };
    let set2 = OverrideSet { paints: vec![set.paints[0].clone(), all], ..OverrideSet::default() };
    let half = 128.0 / 255.0;
    assert_eq!(density(&cover, &set2, 4.0, 12.0, a), 2.0 * half);
    assert_eq!(density(&cover, &set2, 12.0, 12.0, a), 2.0 * (102.0 / 255.0) * half);
    assert_eq!(density(&cover, &set2, 4.0, 4.0, b), 2.0 * half);
}

#[test]
fn explicit_placement_suppresses_its_range_class_only() {
    let mut mid = species("m", 0.5, 2.0);
    mid.range = RangeClass::Mid;
    let cover = on_channel0(vec![species("n", 0.5, 2.0), mid]);
    let (near, mid) = (SpeciesIndex(0), SpeciesIndex(1));
    let set = OverrideSet {
        explicit: vec![ExplicitRegion { region: Region::rect([-1.0, -1.0], [17.0, 17.0]), range: RangeClass::Near }],
        ..OverrideSet::default()
    };
    assert!(scatter(&cover, &set, near).is_empty());
    // The Mid species is untouched: byte-identical to no overrides.
    assert_eq!(scatter(&cover, &set, mid), scatter(&cover, &OverrideSet::default(), mid));
    assert!(!scatter(&cover, &set, mid).is_empty());
}

#[test]
fn masked_region_applies_only_inside_the_mask() {
    let cover = on_channel0(vec![species("a", 0.5, 2.0)]);
    let a = SpeciesIndex(0);
    // Box over the whole tile, mask 2×1 over x: left half inside (255), right half outside (127).
    let region = Region {
        shape: Shape::Box { min: [0.0, 0.0], max: [16.0, 16.0] },
        mask: Some(Mask { origin: [0.0, 0.0], texel_m: 8.0, width: 2, height: 2, texels: vec![255, 127, 128, 0] }),
    };
    let set = OverrideSet { exclusions: vec![region], ..OverrideSet::default() };
    assert_eq!(density(&cover, &set, 4.0, 4.0, a), 0.0);
    assert_eq!(density(&cover, &set, 12.0, 4.0, a), 2.0);
    assert_eq!(density(&cover, &set, 4.0, 12.0, a), 0.0);
    assert_eq!(density(&cover, &set, 12.0, 12.0, a), 2.0);
    assert!(scatter(&cover, &set, a).iter().all(|i| i.local[0] >= 8.0));
}

#[test]
fn bad_overrides_are_rejected() {
    let cover = on_channel0(vec![species("a", 0.5, 2.0)]);
    let unknown = OverrideSet { only: vec![only(Region::circle([0.0, 0.0], 1.0), 0, &["nope"])], ..OverrideSet::default() };
    assert_eq!(Overrides::new(&cover, &unknown).unwrap_err(), BuildError::UnknownSpecies("nope".into()));
    let paint = OverrideSet {
        paints: vec![DensityPaint { origin: [0.0, 0.0], size_m: 16.0, resolution: 4, texels: vec![0; 3], species: vec![] }],
        ..OverrideSet::default()
    };
    assert!(matches!(Overrides::new(&cover, &paint), Err(BuildError::InvalidOverride(_))));
    let inverted = OverrideSet { exclusions: vec![Region::rect([2.0, 0.0], [1.0, 1.0])], ..OverrideSet::default() };
    assert!(matches!(Overrides::new(&cover, &inverted), Err(BuildError::InvalidOverride(_))));
}
