//! Scatter v2 rock layer (spec `worldgen-scatter.md` "Rocks (WG-E3)"; slice WG-E3b: talus candidates and Pareto
//! sizes).
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
//! **Determinism.** A block is a pure function of its parent's stream and the ground at the two points; a rect keeps
//! the blocks inside it from every parent tile within `reach_max_m`, so any covering build returns the same blocks
//! over the rect, bit for bit. Output order is the integer [`RockKey`].
//!
//! Nothing here names a game: units, cliffs and heights come from the caller's [`RockGround`].

use crate::Result;
use crate::filter::Declared;
use crate::scatter::{M2_PER_HA, draw, hash_k, mix, poisson, stream, unit};
use crate::scatter_species::{RockContext, SpeciesFile};

#[cfg(test)]
#[path = "tests_scatter_rocks.rs"]
mod tests;

/// Salt of the talus parent stream in the scatter seed's hash space.
// tunables: const (hash salt, not a value)
const TALUS_SALT: u64 = 0x7a1a_5d00_0000_3e2d;

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

/// The rules of the rock candidates.
pub struct RockRules<'a> {
    /// Rock rows and units.
    pub file: &'a SpeciesFile,
    pub values: RockValues,
    pub seed: u64,
    /// Parent tile edge, m (`scatter.tile_m`).
    pub tile_m: f64,
}

impl RockRules<'_> {
    /// How far a talus block lands from its source at most, m: the parents a rect needs lie within it.
    #[must_use]
    pub fn talus_reach_m(&self) -> f64 {
        self.values.reach_max_m
    }

    /// Every talus block inside `rect = [x0, y0, x1, y1]` (half open), sorted by key; none without a
    /// `[rocks.talus]` row.
    // Tile indices of world metres and hashed counts of a few thousand per tile.
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    #[must_use]
    pub fn talus_candidates(&self, ground: &dyn RockGround, rect: [f64; 4]) -> Vec<RockCandidate> {
        let mut out = Vec::new();
        let Some(row) = self.file.rocks.get(&RockContext::Talus) else { return out };
        let v = &self.values;
        let rf_max = self.file.rock_units.values().map(|u| u.rockfall).fold(0.0, f64::max);
        let kappa_max = v.talus_parent_per_ha / M2_PER_HA * rf_max;
        if kappa_max <= 0.0 || v.talus_child_mean <= 0.0 {
            return out;
        }
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
                        out.push(RockCandidate { key, pos: [qx, qy], size_m: s, tiebreak: hash_k(ob, o, 3), source: [px, py] });
                    }
                }
            }
        }
        out.sort_unstable_by_key(|c| c.key);
        out
    }
}
