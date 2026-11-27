//! Scatter v2 rock layer (spec `worldgen-scatter.md` "Rocks (WG-E3)"; slice WG-E3b: talus candidates and Pareto
//! sizes; WG-E3c: outcrop clusters and field stones; WG-E3e: mesh pick and pose of a kept rock, [`pick`]).
//!
//! **Talus.** Parents are counter-hashed points per parent tile (`scatter.tile_m`) at
//! `scatter.rock.talus_parent_per_ha` x the largest unit `rockfall`, thinned at the point (Lewis-Shedler) to scarp
//! cells (a mapped cliff, or slope >= `scatter.rock.scarp_slope_deg`) at the unit's own `rockfall`. Each parent
//! sheds Poisson(`scatter.rock.talus_child_mean`) blocks. A block draws its size `s` from the truncated Pareto law
//! ([`pareto_size`], `N(>s) ~ s^-D`, D = `scatter.rock.size_dim`), then its horizontal run-out `x ~ exp(-x / ell)`,
//! `ell = fall_ell_m (s / size_min_m)^sort_beta` (big blocks roll further: Statham 1976, Evans and Hungr, U; a 2021
//! cone survey found the sorting weak, V, and `sort_beta` 0 turns it off), cut at `reach_max_m`, along the parent's
//! downslope +- `talus_spread_deg`. It is kept below the shadow-angle line, `z_parent - z_land >= x
//! tan(shadow_angle_deg)` (Evans and Hungr 1993, abstract V), and on ground no steeper than
//! `scatter.rock.loose_slope_max_deg`. The sorting is a tendency: with an exponential run-out the rank correlation of
//! size and distance stays modest (about 0.1-0.25 at beta 0.4 on a 60 m scarp, spec "As built" WG-E3b).
//!
//! **Outcrop.** Parents at `scatter.rock.outcrop_parent_per_ha`, kept in units whose `[rock_units]` table lists
//! `outcrop` entries (the adapter gives entries only to its outcropping units). The parent is a block from the top
//! of the law (`s >= scatter.rock.outcrop_parent_min_m`), exempt from the loose-slope drop; it sheds
//! Poisson(`scatter.rock.child_mean`) children, sizes from the law cut at `child_size_ratio x s_parent`, offset by a
//! Gaussian of `scatter.rock.cluster_sigma_m` truncated at [`THOMAS_REACH_SIGMAS`] sigma (the vegetation process).
//!
//! **Field stones.** Clumps of Poisson(`scatter.rock.field_child_mean`) stones, Gaussian offset
//! `scatter.rock.field_cluster_sigma_m`, parents at the largest unit `field_stone_per_ha / field_child_mean`
//! thinned to the unit's own density at the parent; each stone is thinned by the slope hat (1 up to
//! `scatter.rock.field_slope_full_deg`, 0 at `loose_slope_max_deg`), sizes from the `[rocks.field]` row's law.
//! With a region prior ([`RockRules::prior`], WG-E3f.2) the parent bound is raised by `scatter.region_prior_max`
//! and each parent also thinned by its prior clamped to `[0, region_prior_max]` over that max; without one the
//! factors are 1 (today's stones bit for bit).
//!
//! **Regime** (RG-9j, `rockgen.md` "(e) Typology"). With a `[rock_regimes]` table each parent reads the climate
//! class at its point ([`crate::scatter_fields::FieldSample::class`]) and its regime (one clump, one regime, read
//! where the unit is read): the unit's regime override then sets the field-stone density and the entry lists (an
//! outcrop parent is kept where the regime's outcrop list is non-empty), and every block carries the id
//! ([`RockCandidate::regime`]). Without the table nothing samples the fields and the blocks are today's.
//!
//! **Determinism.** A block is a pure function of its parent's stream and the ground at the two points; a rect keeps
//! the blocks inside it from every parent tile within `reach_max_m`, so any covering build returns the same blocks
//! over the rect, bit for bit. Output order is the integer [`RockKey`].
//!
//! Nothing here names a game: units, cliffs and heights come from the caller's [`RockGround`].

use crate::Result;
use crate::filter::Declared;
use crate::scatter::{M2_PER_HA, THOMAS_REACH_SIGMAS, draw, hash_k, mix, poisson, stream, unit};
use crate::scatter_fields::{Fields, hat};
use crate::scatter_species::{Hat, RockContext, RockEntry, SpeciesFile};

#[cfg(test)]
#[path = "tests_scatter_rocks.rs"]
mod tests;

/// Salt of the talus parent stream in the scatter seed's hash space.
// tunables: const (hash salt, not a value)
const TALUS_SALT: u64 = 0x7a1a_5d00_0000_3e2d;
/// Salts of the outcrop and field-stone parent streams.
// tunables: const (hash salt, not a value)
const OUTCROP_SALT: u64 = 0x0c70_9a11_0000_51e3;
// tunables: const (hash salt, not a value)
const FIELD_SALT: u64 = 0xf1e1_d570_0000_9b47;
/// Multiplier of a parent index into its offspring stream (as the vegetation streams).
// tunables: const (hash constant, not a value)
const OFFSPRING_MUL: u64 = 0xa076_1d64_78bd_642f;
/// Salt of the pick stream (entry, variant, yaw, tilt) of a kept rock, its own beside the budget rank's.
// tunables: const (hash salt, not a value)
const PICK_SALT: u64 = 0x91c4_0e00_0000_7a3b;

/// The rock layer's scatter-wide values (spec "Tunables", `scatter.rock.*`).
#[must_use]
pub fn declared() -> Vec<Declared> {
    let d = |name, v, unit, min, max, source: &str, help| Declared { name, v, unit, min, max, source: source.into(), help };
    vec![
        d("scatter.rock.size_dim", 2.0, "ratio", 1.0, 3.5, "Reference(Turcotte 1986 fragmentation, U)",
          "exponent D of the truncated Pareto size law, N(>s) ~ s^-D"),
        d("scatter.rock.scarp_slope_deg", 50.0, "deg", 40.0, 75.0, "Guess",
          "a cell this steep (or a mapped cliff) is a scarp that sheds talus"),
        d("scatter.rock.loose_slope_max_deg", 45.0, "deg", 30.0, 60.0, "Guess",
          "a loose rock on ground steeper than this is dropped"),
        d("scatter.rock.shadow_angle_deg", 27.5, "deg", 15.0, 35.0, "Reference(Evans and Hungr 1993, abstract)",
          "a fallen block lies below the line of this angle from its source"),
        d("scatter.rock.reach_max_m", 150.0, "m", 20.0, 400.0, "Guess", "longest horizontal run-out of a block"),
        d("scatter.rock.fall_ell_m", 15.0, "m", 2.0, 100.0, "Guess",
          "mean run-out of the smallest block (exponential run-out)"),
        d("scatter.rock.sort_beta", 0.4, "ratio", 0.0, 1.0, "Guess",
          "size sorting: mean run-out x (s / size_min)^beta; 0 = no sorting"),
        d("scatter.rock.talus_parent_per_ha", 40.0, "1/ha", 0.0, 500.0, "Guess",
          "talus sources per hectare of scarp at rockfall 1"),
        d("scatter.rock.talus_spread_deg", 20.0, "deg", 0.0, 60.0, "Guess",
          "half angle of a block's fall direction about the source's downslope"),
        d("scatter.rock.talus_child_mean", 4.0, "count", 0.0, 40.0, "Guess", "mean blocks shed per talus source"),
        d("scatter.rock.outcrop_parent_per_ha", 2.0, "1/ha", 0.0, 50.0, "Guess", "outcrop parent blocks per hectare"),
        d("scatter.rock.outcrop_parent_min_m", 4.0, "m", 1.0, 40.0, "Guess", "smallest outcrop parent block"),
        d("scatter.rock.cluster_sigma_m", 6.0, "m", 2.0, 30.0, "Guess",
          "Gaussian offset of an outcrop's debris about its parent (WG-R sigma 3-15)"),
        d("scatter.rock.child_mean", 8.0, "count", 0.0, 40.0, "Guess", "mean debris blocks per outcrop parent"),
        d("scatter.rock.child_size_ratio", 0.4, "ratio", 0.05, 1.0, "Guess",
          "largest debris block as a share of its parent's size"),
        d("scatter.rock.field_cluster_sigma_m", 2.0, "m", 0.5, 20.0, "Guess", "Gaussian offset of a field-stone clump"),
        d("scatter.rock.field_child_mean", 5.0, "count", 1.0, 40.0, "Guess", "mean stones per field-stone clump"),
        d("scatter.rock.field_slope_full_deg", 15.0, "deg", 0.0, 45.0, "Guess",
          "field stones at full density up to this slope, none at loose_slope_max_deg"),
        d("scatter.rock.rock_k", 0.9, "ratio", 0.5, 1.2, "Guess",
          "two rocks conflict closer than rock_k (r_i + r_j), r = size / 2 (below 1 rocks may touch; WG-R, U)"),
    ]
}

/// The rock layer's values (see [`declared`] for units and meaning).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RockValues {
    pub size_dim: f64,
    pub scarp_slope_deg: f64,
    pub loose_slope_max_deg: f64,
    pub shadow_angle_deg: f64,
    pub reach_max_m: f64,
    pub fall_ell_m: f64,
    pub sort_beta: f64,
    pub talus_parent_per_ha: f64,
    pub talus_spread_deg: f64,
    pub talus_child_mean: f64,
    pub outcrop_parent_per_ha: f64,
    pub outcrop_parent_min_m: f64,
    pub cluster_sigma_m: f64,
    pub child_mean: f64,
    pub child_size_ratio: f64,
    pub field_cluster_sigma_m: f64,
    pub field_child_mean: f64,
    pub field_slope_full_deg: f64,
    pub rock_k: f64,
}

impl RockValues {
    /// Builds the values from declared entries (a species file's `declared`), ranges checked.
    pub fn from_declared(d: &[Declared]) -> Result<Self> {
        let get = |name: &str| -> Result<f64> {
            let p = d
                .iter()
                .find(|p| p.name == name)
                .ok_or_else(|| crate::Error::Invalid(format!("scatter rocks: value {name} not declared")))?;
            crate::recipe::check_range(name, p.v, p.min, p.max)?;
            Ok(p.v)
        };
        Ok(Self {
            size_dim: get("scatter.rock.size_dim")?,
            scarp_slope_deg: get("scatter.rock.scarp_slope_deg")?,
            loose_slope_max_deg: get("scatter.rock.loose_slope_max_deg")?,
            shadow_angle_deg: get("scatter.rock.shadow_angle_deg")?,
            reach_max_m: get("scatter.rock.reach_max_m")?,
            fall_ell_m: get("scatter.rock.fall_ell_m")?,
            sort_beta: get("scatter.rock.sort_beta")?,
            talus_parent_per_ha: get("scatter.rock.talus_parent_per_ha")?,
            talus_spread_deg: get("scatter.rock.talus_spread_deg")?,
            talus_child_mean: get("scatter.rock.talus_child_mean")?,
            outcrop_parent_per_ha: get("scatter.rock.outcrop_parent_per_ha")?,
            outcrop_parent_min_m: get("scatter.rock.outcrop_parent_min_m")?,
            cluster_sigma_m: get("scatter.rock.cluster_sigma_m")?,
            child_mean: get("scatter.rock.child_mean")?,
            child_size_ratio: get("scatter.rock.child_size_ratio")?,
            field_cluster_sigma_m: get("scatter.rock.field_cluster_sigma_m")?,
            field_child_mean: get("scatter.rock.field_child_mean")?,
            field_slope_full_deg: get("scatter.rock.field_slope_full_deg")?,
            rock_k: get("scatter.rock.rock_k")?,
        })
    }
}

impl Default for RockValues {
    fn default() -> Self {
        Self::from_declared(&declared()).expect("defaults are valid")
    }
}

/// The ground the rock layer reads (spec "Rocks (WG-E3)" "Inputs"): pure functions of position.
pub trait RockGround: Sync {
    /// Ground height at (x east, y north), m.
    fn height_m(&self, x: f64, y: f64) -> f64;
    /// (slope, degrees from horizontal; downslope direction, degrees clockwise from north).
    fn slope_aspect_deg(&self, x: f64, y: f64) -> (f64, f64);
    /// A mapped cliff at the point (the geology product's cliff cells).
    fn cliff(&self, x: f64, y: f64) -> bool;
    /// The rock unit at the point, by name (the adapter's default unit where no geology product exists); a unit
    /// the species file has no `[rock_units]` table for sheds nothing.
    fn unit(&self, x: f64, y: f64) -> Option<&str>;
}

/// The integer key of a rock candidate; output order.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct RockKey {
    pub context: RockContext,
    /// The parent tile.
    pub tile: (i32, i32),
    /// Parent index in the tile.
    pub parent: u32,
    /// Offspring index of the parent.
    pub offspring: u32,
}

/// One rock candidate before competition.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RockCandidate {
    pub key: RockKey,
    /// Position, m (x east, y north).
    pub pos: [f64; 2],
    /// Size mark (longest axis), m.
    pub size_m: f64,
    /// Second key of the competition priority (after the size mark).
    pub tiebreak: u64,
    /// The parent's position, m (a talus block's source on the scarp).
    pub source: [f64; 2],
    /// The weathering regime id at the parent (an index of [`SpeciesFile::rock_regime_names`], RG-9j); `None`
    /// without `[rock_regimes]`, or where the climate has no class or one the table does not map.
    pub regime: Option<u8>,
}

/// The truncated Pareto size of quantile `u` in `[0, 1)` (spec "Size law"): `s = s_min (1 - u (1 - (s_min /
/// s_max)^D))^(-1/D)`, in `[s_min, s_max]`.
#[must_use]
pub fn pareto_size(s_min: f64, s_max: f64, d: f64, u: f64) -> f64 {
    if s_max <= s_min {
        return s_min;
    }
    let t = 1.0 - (s_min / s_max).powf(d);
    (s_min * (1.0 - u * t).powf(-1.0 / d)).clamp(s_min, s_max)
}

/// The Gaussian offset of offspring `o` of stream `ob` (draws 0 and 1), `None` past [`THOMAS_REACH_SIGMAS`] sigma.
fn thomas_offset(ob: u64, o: u64, sigma: f64) -> Option<(f64, f64)> {
    let r = sigma * (-2.0 * (1.0 - draw(ob, o, 0)).ln()).sqrt();
    if r > THOMAS_REACH_SIGMAS * sigma {
        return None;
    }
    let (sn, cs) = (draw(ob, o, 1) * std::f64::consts::TAU).sin_cos();
    Some((r * cs, r * sn))
}

/// Calls `f(tile, parent index, tile stream, x, y)` for every counter-hashed parent of intensity `kappa` (per m2)
/// in the tiles whose parents may reach `rect` (box distance <= `reach`; an exact skip).
// Tile indices of world metres and hashed counts of a few thousand per tile.
#[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
fn for_parents(
    seed: u64,
    tile_m: f64,
    kappa: f64,
    rect: [f64; 4],
    reach: f64,
    mut f: impl FnMut((i32, i32), u64, u64, f64, f64),
) {
    let t = tile_m;
    let area = t * t;
    let tx0 = ((rect[0] - reach) / t).floor() as i32;
    let ty0 = ((rect[1] - reach) / t).floor() as i32;
    let tx1 = ((rect[2] + reach) / t).floor() as i32;
    let ty1 = ((rect[3] + reach) / t).floor() as i32;
    for ty in ty0..=ty1 {
        for tx in tx0..=tx1 {
            let (x0, y0) = (f64::from(tx) * t, f64::from(ty) * t);
            let base = stream(seed, tx, ty, 0);
            let n = (kappa * area + unit(mix(base))).floor() as u64;
            for j in 0..n {
                let (px, py) = (x0 + draw(base, j, 0) * t, y0 + draw(base, j, 1) * t);
                let dx = (rect[0] - px).max(px - rect[2]).max(0.0);
                let dy = (rect[1] - py).max(py - rect[3]).max(0.0);
                if dx.hypot(dy) <= reach {
                    f((tx, ty), j, base, px, py);
                }
            }
        }
    }
}

/// The rules of the rock candidates.
pub struct RockRules<'a> {
    /// Rock rows and units.
    pub file: &'a SpeciesFile,
    pub values: RockValues,
    pub seed: u64,
    /// Parent tile edge, m (`scatter.tile_m`).
    pub tile_m: f64,
    /// The field-stone region prior at (x, y) (the adapter's learned density over its land mean), clamped to
    /// `[0, scatter.region_prior_max]`; `None`: 1 everywhere.
    pub prior: Option<&'a (dyn Fn(f64, f64) -> f64 + Sync)>,
}

impl RockRules<'_> {
    /// The regime id at a point of `fields` (module docs "Regime"): a sample only when the file maps a class of
    /// the fields to a regime, else `None` everywhere without sampling.
    fn regime_at<'f>(&self, fields: &'f dyn Fields) -> impl Fn(f64, f64) -> Option<u8> + 'f {
        let ids = self.file.regime_ids(fields.class_names());
        let on = ids.iter().any(Option::is_some);
        move |x, y| {
            if !on {
                return None;
            }
            fields.sample(x, y).class.and_then(|c| ids.get(usize::from(c)).copied().flatten())
        }
    }

    /// How far a talus block lands from its source at most, m: the parents a rect needs lie within it.
    #[must_use]
    pub fn talus_reach_m(&self) -> f64 {
        self.values.reach_max_m
    }

    /// How far any block lands from its parent at most (talus run-out, outcrop and field-stone offsets), m.
    #[must_use]
    pub fn parent_reach_m(&self) -> f64 {
        let v = &self.values;
        v.reach_max_m.max(THOMAS_REACH_SIGMAS * v.cluster_sigma_m.max(v.field_cluster_sigma_m))
    }

    /// The largest block of any context, m (`size_max_m` over the `[rocks.*]` rows; 0 without one).
    #[must_use]
    pub fn size_max_m(&self) -> f64 {
        self.file.rocks.values().map(|r| r.size_max_m).fold(0.0, f64::max)
    }

    /// Every talus block inside `rect = [x0, y0, x1, y1]` (half open), sorted by key; none without a
    /// `[rocks.talus]` row.
    // Tile indices of world metres and hashed counts of a few thousand per tile.
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    #[must_use]
    pub fn talus_candidates(&self, ground: &dyn RockGround, fields: &dyn Fields, rect: [f64; 4]) -> Vec<RockCandidate> {
        let mut out = Vec::new();
        let Some(row) = self.file.rocks.get(&RockContext::Talus) else { return out };
        let v = &self.values;
        let rf_max = self.file.rock_units.values().map(|u| u.rockfall).fold(0.0, f64::max);
        let kappa_max = v.talus_parent_per_ha / M2_PER_HA * rf_max;
        if kappa_max <= 0.0 || v.talus_child_mean <= 0.0 {
            return out;
        }
        let regime_at = self.regime_at(fields);
        let rockfall = |x: f64, y: f64| {
            ground.unit(x, y).and_then(|u| self.file.rock_units.get(u)).map_or(0.0, |u| u.rockfall)
        };
        let (smin, smax) = (row.size_min_m, row.size_max_m);
        let tan_shadow = v.shadow_angle_deg.to_radians().tan();
        let spread = v.talus_spread_deg;
        let reach = v.reach_max_m;
        let t = self.tile_m;
        let area = t * t;
        let inside = |x: f64, y: f64| x >= rect[0] && x < rect[2] && y >= rect[1] && y < rect[3];
        let tx0 = ((rect[0] - reach) / t).floor() as i32;
        let ty0 = ((rect[1] - reach) / t).floor() as i32;
        let tx1 = ((rect[2] + reach) / t).floor() as i32;
        let ty1 = ((rect[3] + reach) / t).floor() as i32;
        for ty in ty0..=ty1 {
            for tx in tx0..=tx1 {
                let (x0, y0) = (f64::from(tx) * t, f64::from(ty) * t);
                let base = stream(self.seed ^ TALUS_SALT, tx, ty, 0);
                let n = (kappa_max * area + unit(mix(base))).floor() as u64;
                for j in 0..n {
                    let (px, py) = (x0 + draw(base, j, 0) * t, y0 + draw(base, j, 1) * t);
                    // Box distance to the rect: past the reach no block lands inside (an exact skip).
                    let dx = (rect[0] - px).max(px - rect[2]).max(0.0);
                    let dy = (rect[1] - py).max(py - rect[3]).max(0.0);
                    if dx.hypot(dy) > reach {
                        continue;
                    }
                    let (slope, aspect) = ground.slope_aspect_deg(px, py);
                    if !(ground.cliff(px, py) || slope >= v.scarp_slope_deg) {
                        continue;
                    }
                    if draw(base, j, 2) >= rockfall(px, py) / rf_max {
                        continue;
                    }
                    let zp = ground.height_m(px, py);
                    let regime = regime_at(px, py);
                    let m = poisson(v.talus_child_mean, draw(base, j, 3));
                    let ob = mix(base ^ (j + 1).wrapping_mul(0xa076_1d64_78bd_642f));
                    for o in 0..u64::from(m) {
                        let s = pareto_size(smin, smax, v.size_dim, draw(ob, o, 0));
                        let ell = v.fall_ell_m * (s / smin).powf(v.sort_beta);
                        let x = -ell * (1.0 - draw(ob, o, 1)).ln();
                        if x > reach {
                            continue;
                        }
                        let dir = (aspect + (2.0 * draw(ob, o, 2) - 1.0) * spread).to_radians();
                        let (qx, qy) = (px + x * dir.sin(), py + x * dir.cos());
                        if !inside(qx, qy) {
                            continue;
                        }
                        if zp - ground.height_m(qx, qy) < x * tan_shadow
                            || ground.slope_aspect_deg(qx, qy).0 > v.loose_slope_max_deg
                        {
                            continue;
                        }
                        let key =
                            RockKey { context: RockContext::Talus, tile: (tx, ty), parent: j as u32, offspring: o as u32 };
                        let tiebreak = hash_k(ob, o, 3);
                        out.push(RockCandidate { key, pos: [qx, qy], size_m: s, tiebreak, source: [px, py], regime });
                    }
                }
            }
        }
        out.sort_unstable_by_key(|c| c.key);
        out
    }
    /// Every outcrop block (parents and their debris) inside `rect = [x0, y0, x1, y1]` (half open), sorted by key
    /// (a parent is offspring 0, its debris 1..); none without a `[rocks.outcrop]` row.
    #[allow(clippy::cast_possible_truncation)] // parent and offspring indices of a few thousand at most
    #[must_use]
    pub fn outcrop_candidates(&self, ground: &dyn RockGround, fields: &dyn Fields, rect: [f64; 4]) -> Vec<RockCandidate> {
        let mut out = Vec::new();
        let Some(row) = self.file.rocks.get(&RockContext::Outcrop) else { return out };
        let v = &self.values;
        let kappa = v.outcrop_parent_per_ha / M2_PER_HA;
        if kappa <= 0.0 {
            return out;
        }
        let (smin, smax) = (row.size_min_m, row.size_max_m);
        let s_lo = v.outcrop_parent_min_m.clamp(smin, smax);
        let sigma = v.cluster_sigma_m;
        let inside = |x: f64, y: f64| x >= rect[0] && x < rect[2] && y >= rect[1] && y < rect[3];
        let regime_at = self.regime_at(fields);
        let outcrops = |x: f64, y: f64, regime: Option<u8>| {
            ground
                .unit(x, y)
                .and_then(|u| self.file.rock_units.get(u))
                .is_some_and(|u| !u.entries_in(RockContext::Outcrop, self.file.regime_name(regime)).is_empty())
        };
        let reach = THOMAS_REACH_SIGMAS * sigma;
        for_parents(self.seed ^ OUTCROP_SALT, self.tile_m, kappa, rect, reach, |tile, j, base, px, py| {
            let regime = regime_at(px, py);
            if !outcrops(px, py, regime) {
                return;
            }
            let sp = pareto_size(s_lo, smax, v.size_dim, draw(base, j, 2));
            let key = |o: u64| RockKey { context: RockContext::Outcrop, tile, parent: j as u32, offspring: o as u32 };
            if inside(px, py) {
                let tiebreak = hash_k(base, j, 4);
                out.push(RockCandidate { key: key(0), pos: [px, py], size_m: sp, tiebreak, source: [px, py], regime });
            }
            let cut = (v.child_size_ratio * sp).min(smax);
            if cut < smin {
                return;
            }
            let m = poisson(v.child_mean, draw(base, j, 3));
            let ob = mix(base ^ (j + 1).wrapping_mul(OFFSPRING_MUL));
            for o in 0..u64::from(m) {
                let Some((dx, dy)) = thomas_offset(ob, o, sigma) else { continue };
                let (x, y) = (px + dx, py + dy);
                if !inside(x, y) || ground.slope_aspect_deg(x, y).0 > v.loose_slope_max_deg {
                    continue;
                }
                let s = pareto_size(smin, cut, v.size_dim, draw(ob, o, 2));
                let tiebreak = hash_k(ob, o, 3);
                out.push(RockCandidate { key: key(o + 1), pos: [x, y], size_m: s, tiebreak, source: [px, py], regime });
            }
        });
        out.sort_unstable_by_key(|c| c.key);
        out
    }

    /// Every field stone inside `rect = [x0, y0, x1, y1]` (half open), sorted by key; none without a
    /// `[rocks.field]` row.
    #[allow(clippy::cast_possible_truncation)] // parent and offspring indices of a few thousand at most
    #[must_use]
    pub fn field_candidates(&self, ground: &dyn RockGround, fields: &dyn Fields, rect: [f64; 4]) -> Vec<RockCandidate> {
        let mut out = Vec::new();
        let Some(row) = self.file.rocks.get(&RockContext::Field) else { return out };
        let v = &self.values;
        let fs_max = self.file.rock_units.values().map(crate::scatter_species::RockUnit::field_stone_max).fold(0.0, f64::max);
        // The prior's bound raises the parents; each is thinned by `prior / pmax` (both exactly 1 without one).
        let pmax = if self.prior.is_some() { self.file.values.region_prior_max } else { 1.0 };
        let prior = |x: f64, y: f64| self.prior.map_or(1.0, |p| p(x, y).max(0.0).min(pmax) / pmax);
        let kappa = fs_max * pmax / M2_PER_HA / v.field_child_mean;
        if kappa <= 0.0 {
            return out;
        }
        let (smin, smax) = (row.size_min_m, row.size_max_m);
        let sigma = v.field_cluster_sigma_m;
        let slope_hat = Hat { e_min: -1.0, i_min: 0.0, i_max: v.field_slope_full_deg, e_max: v.loose_slope_max_deg };
        let inside = |x: f64, y: f64| x >= rect[0] && x < rect[2] && y >= rect[1] && y < rect[3];
        let regime_at = self.regime_at(fields);
        let density = |x: f64, y: f64, regime: Option<u8>| {
            let unit = ground.unit(x, y).and_then(|u| self.file.rock_units.get(u));
            unit.map_or(0.0, |u| u.field_stone_in(self.file.regime_name(regime)))
        };
        let reach = THOMAS_REACH_SIGMAS * sigma;
        for_parents(self.seed ^ FIELD_SALT, self.tile_m, kappa, rect, reach, |tile, j, base, px, py| {
            let regime = regime_at(px, py);
            if draw(base, j, 2) >= density(px, py, regime) / fs_max * prior(px, py) {
                return;
            }
            let m = poisson(v.field_child_mean, draw(base, j, 3));
            let ob = mix(base ^ (j + 1).wrapping_mul(OFFSPRING_MUL));
            for o in 0..u64::from(m) {
                let Some((dx, dy)) = thomas_offset(ob, o, sigma) else { continue };
                let (x, y) = (px + dx, py + dy);
                if !inside(x, y) || draw(ob, o, 3) >= hat(&slope_hat, ground.slope_aspect_deg(x, y).0) {
                    continue;
                }
                let s = pareto_size(smin, smax, v.size_dim, draw(ob, o, 2));
                let key = RockKey { context: RockContext::Field, tile, parent: j as u32, offspring: o as u32 };
                out.push(RockCandidate { key, pos: [x, y], size_m: s, tiebreak: hash_k(ob, o, 4), source: [px, py], regime });
            }
        });
        out.sort_unstable_by_key(|c| c.key);
        out
    }
}

/// The mesh and pose of one kept rock (spec "Mesh pick", "Pose"; slice WG-E3e), drawn at load.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RockPick<'a> {
    /// The entry drawn: a library class or a pack model of the rock's unit and context.
    pub entry: &'a RockEntry,
    /// The variant hash (`hash(instance, 2)`): the reader takes it modulo its class's variant count (a pack model
    /// ignores it).
    pub variant: u64,
    /// A pack model: its uniform scale `s / axis_m`. A library class: the size mark `s`, m (the reader divides by
    /// the class size, `rockgen.md` "scale").
    pub scale: f64,
    /// Yaw about the up axis, radians in `[0, 2 pi)`.
    pub yaw_rad: f64,
    /// Up axis, unit (x east, y north, z up): the row's `align` blend from vertical to the ground normal, tilted
    /// uniformly up to `scatter.rock.tilt_jitter_deg`.
    pub up: [f64; 3],
    /// Share of the mesh's short axis sunk below the ground: `lerp(sink_small, sink_large, t)`, `t = ln(s / s_min) /
    /// ln(s_max / s_min)` over the context's row (small stones sit deep, big blocks high).
    pub sink: f64,
}

/// The ground normal of (slope, downslope aspect) in degrees (aspect clockwise from north; x east, y north, z up).
fn normal_of(slope_deg: f64, aspect_deg: f64) -> [f64; 3] {
    let (ss, cs) = slope_deg.to_radians().sin_cos();
    let (sa, ca) = aspect_deg.to_radians().sin_cos();
    [ss * sa, ss * ca, cs]
}

fn norm3(v: [f64; 3]) -> [f64; 3] {
    let l = (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt();
    if l > 0.0 { v.map(|c| c / l) } else { [0.0, 0.0, 1.0] }
}

/// `up` tilted by `deg` towards the tangent direction at `turn` (a fraction of a turn).
fn tilted(up: [f64; 3], deg: f64, turn: f64) -> [f64; 3] {
    if deg <= 0.0 {
        return up;
    }
    let helper = if up[2].abs() < 0.9 { [0.0, 0.0, 1.0] } else { [1.0, 0.0, 0.0] };
    let t1 = norm3([
        up[1] * helper[2] - up[2] * helper[1],
        up[2] * helper[0] - up[0] * helper[2],
        up[0] * helper[1] - up[1] * helper[0],
    ]);
    let t2 = [up[1] * t1[2] - up[2] * t1[1], up[2] * t1[0] - up[0] * t1[2], up[0] * t1[1] - up[1] * t1[0]];
    let (sp, cp) = (turn * std::f64::consts::TAU).sin_cos();
    let (st, ct) = deg.to_radians().sin_cos();
    norm3([0, 1, 2].map(|k| up[k] * ct + (t1[k] * cp + t2[k] * sp) * st))
}

/// Whether `e` may carry a rock of size `s` (m): a class any size, a pack model only for `s` in `axis_m x
/// [scale_min, scale_max]`; a zero weight never.
#[must_use]
pub fn entry_fits(e: &RockEntry, s: f64) -> bool {
    match e {
        RockEntry::Class { weight, .. } => *weight > 0.0,
        RockEntry::Model { axis_m, scale_min, scale_max, weight, .. } => {
            *weight > 0.0 && s >= axis_m * scale_min && s <= axis_m * scale_max
        }
    }
}

/// The entry of the kind `model` (pack model or class) at the weighted quantile `v` in `[0, 1)` among the fitting
/// entries of that kind.
fn weighted(entries: &[RockEntry], s: f64, model: bool, total: f64, v: f64) -> Option<&RockEntry> {
    let fit = entries.iter().filter(|e| entry_fits(e, s) && matches!(e, RockEntry::Model { .. }) == model);
    let target = v * total;
    let mut acc = 0.0;
    let mut last = None;
    for e in fit {
        acc += e.weight();
        last = Some(e);
        if target < acc {
            return Some(e);
        }
    }
    last
}

/// The mesh and pose of the kept rock `c` (spec "Mesh pick", "Pose"; WG-E3e), a pure function of the scatter seed,
/// the rock's key and size, and the ground at its point and its parent's: `None` when its parent's unit has no
/// fitting entry for its context, or the file has no row for it.
///
/// **Entry** (`hash(instance, 1)`): among the unit's entries of the rock's context that fit `s` ([`entry_fits`]),
/// the kind first, a pack model with chance `prm.rock_pack_model_share` when both kinds fit, then the entry by weight
/// within its kind. When only library classes fit they carry the rock (the library fills the sizes no model covers);
/// when only pack models fit they carry it unless the share is 0 (0 = library only, the A/B arm). So the share is
/// the chance of a pack model wherever one fits and a class exists. **Variant** `hash(instance, 2)`; **yaw**
/// `hash(instance, 3)`; **tilt** amount `hash(instance, 4) x tilt_jitter_deg` towards `hash(instance, 5)`. The unit
/// is the parent's (a talus block is its scarp's rock), as the candidates read it; the entries are those of the
/// rock's regime ([`RockCandidate::regime`], RG-9j) when the unit overrides them there.
#[must_use]
pub fn pick<'f>(file: &'f SpeciesFile, prm: &crate::scatter::Params, ground: &dyn RockGround, c: &RockCandidate) -> Option<RockPick<'f>> {
    let row = file.rocks.get(&c.key.context)?;
    let rock_unit = file.rock_units.get(ground.unit(c.source[0], c.source[1])?)?;
    let entries = rock_unit.entries_in(c.key.context, file.regime_name(c.regime));
    let s = c.size_m;
    let total = |model: bool| -> f64 {
        entries.iter().filter(|e| entry_fits(e, s) && matches!(e, RockEntry::Model { .. }) == model).map(RockEntry::weight).sum()
    };
    let (w_model, w_class) = (total(true), total(false));
    let share = prm.rock_pack_model_share.clamp(0.0, 1.0);
    let p_model = match (w_model > 0.0, w_class > 0.0) {
        (true, true) => share,
        (true, false) if share > 0.0 => 1.0,
        (false, true) => 0.0,
        _ => return None,
    };
    let base = stream(prm.seed ^ PICK_SALT, c.key.tile.0, c.key.tile.1, c.key.context as u32);
    let inst = (u64::from(c.key.parent) << 32) | u64::from(c.key.offspring);
    let h = |k| hash_k(base, inst, k);
    let u = unit(h(1));
    let (model, v) = if u < p_model { (true, u / p_model) } else { (false, (u - p_model) / (1.0 - p_model)) };
    let entry = weighted(entries, s, model, if model { w_model } else { w_class }, v)?;
    let scale = match entry {
        RockEntry::Model { axis_m, .. } => s / axis_m,
        RockEntry::Class { .. } => s,
    };
    let t = if row.size_max_m > row.size_min_m {
        ((s / row.size_min_m).ln() / (row.size_max_m / row.size_min_m).ln()).clamp(0.0, 1.0)
    } else {
        0.0
    };
    let (slope, aspect) = ground.slope_aspect_deg(c.pos[0], c.pos[1]);
    let n = normal_of(slope, aspect);
    let a = row.align.clamp(0.0, 1.0);
    let aligned = norm3([a * n[0], a * n[1], (1.0 - a) + a * n[2]]);
    Some(RockPick {
        entry,
        variant: h(2),
        scale,
        yaw_rad: unit(h(3)) * std::f64::consts::TAU,
        up: tilted(aligned, unit(h(4)) * prm.rock_tilt_jitter_deg, unit(h(5))),
        sink: row.sink_small + (row.sink_large - row.sink_small) * t,
    })
}
