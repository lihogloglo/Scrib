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
//!
//! **Ground** ([`Ground`], the scatter's [`RockGround`], spec "Rocks (WG-E3)" "Inputs"): unit and cliff from the
//! guide's `geology/` product ([`GeologyGrid`], nearest sample) when the guide has one, else no cliff and per region
//! its largest [`unit_shares`] unit (ties: the first name); slope and aspect from the guide heights (central
//! differences one sample apart, [`Grid::gradient`]); height the final ground ([`World::height`]), which the rocks
//! stand on.

use std::collections::BTreeMap;
use std::path::Path;

use sf_worldgen::geology::{GeologyGrid, GeologyTable};
use sf_worldgen::scatter_rocks::RockGround;
use sf_worldgen::scatter_species::{RockContext, RockEntry};

use super::{Grid, Learned, World};
use crate::{DumpError, RegionMap};

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

/// The geology product of a guide folder (`dir/geology/`), when it has one.
///
/// # Errors
/// A product that is present (its `meta.json` exists) but unreadable.
pub fn guide_geology(dir: &Path) -> Result<Option<GeologyGrid>, DumpError> {
    if !dir.join("geology").join("meta.json").exists() {
        return Ok(None);
    }
    GeologyGrid::read(dir).map(Some).map_err(|e| DumpError::Invalid(format!("{}: {e}", dir.join("geology").display())))
}

/// The game's rock ground ([module docs](self)).
pub struct Ground<'a> {
    world: &'a World<'a>,
    geology: Option<&'a GeologyGrid>,
    /// Per region legend index, its default unit (used without a geology product).
    defaults: Vec<Option<String>>,
}

impl<'a> Ground<'a> {
    /// The ground of `world`, on `geology` when given (its frame must be the guide's), else on `table`'s region
    /// shares.
    ///
    /// # Errors
    /// A geology product whose dims, spacing or origin are not the guide's.
    pub fn new(world: &'a World<'a>, geology: Option<&'a GeologyGrid>, table: &GeologyTable) -> Result<Self, DumpError> {
        if let Some(g) = geology {
            let w = &world.guide;
            let close = |a: f64, b: f64| (a - b).abs() <= 1e-6 * b.abs().max(1.0);
            if [g.w, g.h] != [w.w, w.h]
                || !close(g.spacing_m, w.spacing)
                || !close(g.origin_m[0], w.origin[0])
                || !close(g.origin_m[1], w.origin[1])
            {
                return Err(DumpError::Invalid(format!(
                    "rocks: the geology product ({}x{} @ {} m, origin {:?}) is not the guide ({}x{} @ {} m, origin {:?})",
                    g.w, g.h, g.spacing_m, g.origin_m, w.w, w.h, w.spacing, w.origin
                )));
            }
        }
        let defaults = unit_shares(world.regions, table)
            .into_iter()
            .map(|m| {
                let mut best: Option<(&String, f64)> = None;
                for (u, &v) in &m {
                    if best.is_none_or(|(_, b)| v > b) {
                        best = Some((u, v));
                    }
                }
                best.map(|(u, _)| u.clone())
            })
            .collect();
        Ok(Self { world, geology, defaults })
    }

    fn guide(&self) -> &Grid<'a> {
        &self.world.guide
    }
}

impl RockGround for Ground<'_> {
    fn height_m(&self, x: f64, y: f64) -> f64 {
        self.world.height(x, y)
    }

    fn slope_aspect_deg(&self, x: f64, y: f64) -> (f64, f64) {
        let (c, r) = self.guide().colrow(x, y);
        let (gx, gy) = self.guide().gradient(c, r);
        let slope = gx.hypot(gy).atan().to_degrees();
        // Downslope is -(gx, gy); its compass azimuth, clockwise from north.
        let aspect = if gx == 0.0 && gy == 0.0 { 0.0 } else { (-gx).atan2(-gy).to_degrees().rem_euclid(360.0) };
        (slope, aspect)
    }

    fn cliff(&self, x: f64, y: f64) -> bool {
        self.geology.is_some_and(|g| g.cliff_at(x, y))
    }

    fn unit(&self, x: f64, y: f64) -> Option<&str> {
        match self.geology {
            Some(g) => g.unit_at(x, y),
            None => self.defaults.get(usize::from(self.world.region(x, y)?))?.as_deref(),
        }
    }
}
