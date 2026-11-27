//! The rock layer's adapter half (spec `worldgen-scatter.md` "Rocks (WG-E3)", "Adapter data"; slices WG-E3f.*): the
//! source's learned rock models as pack-model entries of the rock units ([`rock_entries`]) and the field-stone region
//! prior ([`rock_prior`]).
//!
//! **Model entries** (owner 2026-11-26: the source's rocks stay beside the library). Every learned rock model
//! ([`Learned::rocks`]: one per rock base record, `terrain_rock*` and rocks of 14 m or more) becomes one
//! `{ kind = "model" }` entry in every context a rock unit already lists (a unit without an outcrop context gets no
//! outcrop model): `model` = its base record id (unique per learned model, where two records may share a mesh; the
//! game resolves it to the record's mesh), `axis_m` = the longest edge of its bounds, `[scale_min, scale_max]` = the
//! scale range of its references, and as weight its learned count in the unit's regions: `w(u) = sum over regions r
//! of count(r) x share(r, u)`, `share` the region's `geology.toml` unit share (its `[region."<name>"]` row, else the
//! table's default `[shares]`, normalised). The scatter's entries are per unit, not per region, so the region's
//! count reaches the unit through the shares the geology map itself draws its units from. Model entries are added to
//! the unit's own lists only: a regime override list (RG-9j) replaces them, as it replaces the classes.

use std::collections::BTreeMap;

use sf_worldgen::geology::GeologyTable;
use sf_worldgen::scatter_species::{RockContext, RockEntry};

use super::Learned;
use crate::RegionMap;

/// Per legend index of `regions`: the unit shares of its `[region."<name>"]` row (by the legend's `name`), else the
/// table's default `[shares]`, normalised to sum 1 (no share at all when they sum to 0).
#[must_use]
pub fn unit_shares(regions: &RegionMap, geology: &GeologyTable) -> Vec<BTreeMap<String, f64>> {
    regions
        .legend
        .iter()
        .map(|e| {
            let raw = geology.region.get(&e.name).map_or(&geology.shares, |r| &r.shares);
            let sum: f64 = raw.values().filter(|v| v.is_finite() && **v > 0.0).sum();
            if sum <= 0.0 {
                return BTreeMap::new();
            }
            raw.iter().filter(|(_, v)| v.is_finite() && **v > 0.0).map(|(u, v)| (u.clone(), v / sum)).collect()
        })
        .collect()
}

/// Adds the learned rock models of `learned` to its file's rock units as pack-model entries ([module docs](self)),
/// in base-record order; returns the entries added. A model whose bounds or scales are not positive, or whose weight
/// in a unit is 0, is left out of it.
pub fn rock_entries(learned: &mut Learned, regions: &RegionMap, geology: &GeologyTable) -> usize {
    let shares = unit_shares(regions, geology);
    let mut order: Vec<usize> = (0..learned.rocks.len()).collect();
    order.sort_by(|&a, &b| learned.reps[learned.rocks[a].rep].key.cmp(&learned.reps[learned.rocks[b].rep].key));
    let mut added = 0;
    for i in order {
        let s = &learned.rocks[i];
        let ok = |v: f32| v.is_finite() && v > 0.0;
        if !(ok(s.extent_m) && ok(s.scale[0]) && ok(s.scale[1])) {
            continue;
        }
        let model = learned.reps[s.rep].key.clone();
        for (name, unit) in &mut learned.file.rock_units {
            let weight: f64 = s
                .per_region
                .iter()
                .enumerate()
                .map(|(r, &n)| n as f64 * shares.get(r).and_then(|m| m.get(name)).copied().unwrap_or(0.0))
                .sum();
            if !(weight.is_finite() && weight > 0.0) {
                continue;
            }
            for context in [RockContext::Talus, RockContext::Outcrop, RockContext::Field] {
                let Some(list) = unit.entries.get_mut(&context).filter(|l| !l.is_empty()) else { continue };
                list.push(RockEntry::Model {
                    model: model.clone(),
                    axis_m: f64::from(s.extent_m),
                    scale_min: f64::from(s.scale[0]),
                    scale_max: f64::from(s.scale[1]),
                    weight,
                });
                added += 1;
            }
        }
    }
    added
}

/// The field-stone region prior (spec "Adapter data": learned from the source's rock references): per region,
/// its rock density `D(r)` ([`Learned::rock_density`]) over the land-weighted mean `M = sum D(r) a(r) / sum a(r)`,
/// `land[r]` = `a(r)` the guide land samples of the region (0 everywhere when `M` is 0). Its land-weighted mean is
/// 1, so a unit keeps its authored `field_stone_per_ha` on average; the scatter clamps it to
/// `scatter.region_prior_max` as the plants' prior.
#[must_use]
pub fn rock_prior(density: &[f64], land: &[u64]) -> Vec<f64> {
    let (mut num, mut den) = (0.0, 0.0);
    for (r, &a) in land.iter().enumerate() {
        num += density.get(r).copied().unwrap_or(0.0) * a as f64;
        den += a as f64;
    }
    let mean = if den > 0.0 { num / den } else { 0.0 };
    (0..land.len()).map(|r| if mean > 0.0 { density.get(r).copied().unwrap_or(0.0) / mean } else { 0.0 }).collect()
}
