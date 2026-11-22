//! Overrides on top of the rule table (spec "Overrides"; Horizon painted maps, UE PCG exclusions and biome
//! priorities). All are world-XZ regions in metres:
//!
//! - **Density paint**: an R8 grid over a square (a terrain tile, usually); multiplies density by `texel / 255`
//!   for the species it lists (all when the list is empty).
//! - **Exclusion**: density 0 inside (roads, under buildings, explicit volumes).
//! - **"Only these species"**: inside, the region's own weighted list replaces the rule table (channels are not
//!   read); species it does not list get 0. Overlaps resolve to the highest `priority`; equal priorities to the
//!   region listed last (later layers override earlier ones, like mod load order).
//! - **Explicit placement**: content places its own instances inside; procedural species of the same
//!   [`RangeClass`] are suppressed there, other classes are untouched.
//!
//! A region is a box or a circle, optionally cut by a mask grid (inside where the mask texel is ≥ 128).

use serde::{Deserialize, Serialize};

use crate::rules::{BuildError, Groundcover, SpeciesIndex, WeightedSpecies};
use crate::species::RangeClass;

// tunables: const (R8 format: a mask texel at or above half-scale counts as inside)
const MASK_INSIDE_MIN: u8 = 128;

/// A world-XZ rectangle `[min, max]`, metres.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Rect {
    pub min: [f64; 2],
    pub max: [f64; 2],
}

impl Rect {
    fn overlaps(self, o: Rect) -> bool {
        self.min[0] <= o.max[0] && o.min[0] <= self.max[0] && self.min[1] <= o.max[1] && o.min[1] <= self.max[1]
    }

    fn intersect(self, o: Rect) -> Rect {
        Rect {
            min: [self.min[0].max(o.min[0]), self.min[1].max(o.min[1])],
            max: [self.max[0].min(o.max[0]), self.max[1].min(o.max[1])],
        }
    }
}

/// A region's outline in world XZ (x, z), metres.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Shape {
    /// Axis-aligned box, inclusive.
    Box { min: [f64; 2], max: [f64; 2] },
    /// Disc, inclusive.
    Circle { center: [f64; 2], radius_m: f64 },
}

/// An R8 mask grid: row-major, `width` texels along x, `height` rows along z, from `origin` (min corner).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Mask {
    pub origin: [f64; 2],
    pub texel_m: f64,
    pub width: u32,
    pub height: u32,
    pub texels: Vec<u8>,
}

impl Mask {
    fn rect(&self) -> Rect {
        Rect {
            min: self.origin,
            max: [
                self.origin[0] + f64::from(self.width) * self.texel_m,
                self.origin[1] + f64::from(self.height) * self.texel_m,
            ],
        }
    }

    fn inside(&self, x: f64, z: f64) -> bool {
        texel_at(self.origin, self.texel_m, self.width, self.height, x, z)
            .is_some_and(|i| self.texels[i] >= MASK_INSIDE_MIN)
    }
}

/// Index of the texel holding (x, z) in a `width × height` grid from `origin`, or `None` outside it.
fn texel_at(origin: [f64; 2], texel_m: f64, width: u32, height: u32, x: f64, z: f64) -> Option<usize> {
    let u = ((x - origin[0]) / texel_m).floor();
    let v = ((z - origin[1]) / texel_m).floor();
    if u < 0.0 || v < 0.0 || u >= f64::from(width) || v >= f64::from(height) {
        return None;
    }
    // In [0, width) and [0, height) after the test above.
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    let (u, v) = (u as usize, v as usize);
    Some(v * width as usize + u)
}

/// A shape, optionally cut by a mask.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Region {
    pub shape: Shape,
    #[serde(default)]
    pub mask: Option<Mask>,
}

impl Region {
    pub fn rect(min: [f64; 2], max: [f64; 2]) -> Self {
        Self { shape: Shape::Box { min, max }, mask: None }
    }

    pub fn circle(center: [f64; 2], radius_m: f64) -> Self {
        Self { shape: Shape::Circle { center, radius_m }, mask: None }
    }

    pub fn contains(&self, x: f64, z: f64) -> bool {
        let in_shape = match self.shape {
            Shape::Box { min, max } => x >= min[0] && x <= max[0] && z >= min[1] && z <= max[1],
            Shape::Circle { center, radius_m } => {
                let (dx, dz) = (x - center[0], z - center[1]);
                dx * dx + dz * dz <= radius_m * radius_m
            }
        };
        in_shape && self.mask.as_ref().is_none_or(|m| m.inside(x, z))
    }

    fn bounds(&self) -> Rect {
        let r = match self.shape {
            Shape::Box { min, max } => Rect { min, max },
            Shape::Circle { center, radius_m } => Rect {
                min: [center[0] - radius_m, center[1] - radius_m],
                max: [center[0] + radius_m, center[1] + radius_m],
            },
        };
        self.mask.as_ref().map_or(r, |m| r.intersect(m.rect()))
    }

    fn validate(&self) -> Result<(), BuildError> {
        let ok_shape = match self.shape {
            Shape::Box { min, max } => min.iter().chain(&max).all(|v| v.is_finite()) && min[0] <= max[0] && min[1] <= max[1],
            Shape::Circle { center, radius_m } => center.iter().all(|v| v.is_finite()) && radius_m.is_finite() && radius_m >= 0.0,
        };
        if !ok_shape {
            return Err(BuildError::InvalidOverride("region shape is not finite or has min > max / radius < 0"));
        }
        if let Some(m) = &self.mask
            && (!(m.texel_m.is_finite() && m.texel_m > 0.0)
                || m.texels.len() != m.width as usize * m.height as usize)
        {
            return Err(BuildError::InvalidOverride("mask texel_m must be > 0 and texels.len() == width × height"));
        }
        Ok(())
    }
}

/// A painted density multiplier over a square: `resolution × resolution` R8 texels from `origin` (min corner),
/// `size_m` on a side, row-major along x then z.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DensityPaint {
    pub origin: [f64; 2],
    pub size_m: f64,
    pub resolution: u32,
    pub texels: Vec<u8>,
    /// The species group it applies to; empty = every species.
    #[serde(default)]
    pub species: Vec<String>,
}

/// Content's "only these species here" region.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OnlyRegion {
    pub region: Region,
    pub priority: i32,
    pub species: Vec<WeightedSpecies>,
}

/// Content's explicit-placement region: suppresses procedural species of `range` inside.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ExplicitRegion {
    pub region: Region,
    pub range: RangeClass,
}

/// Every override, as authored (ids, not indices). Compile with [`Overrides::new`].
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct OverrideSet {
    #[serde(default)]
    pub paints: Vec<DensityPaint>,
    #[serde(default)]
    pub exclusions: Vec<Region>,
    #[serde(default)]
    pub only: Vec<OnlyRegion>,
    #[serde(default)]
    pub explicit: Vec<ExplicitRegion>,
}

#[derive(Clone, Debug)]
struct Paint {
    paint: DensityPaint,
    /// Per species index: does this paint apply?
    applies: Vec<bool>,
}

#[derive(Clone, Debug)]
struct Only {
    region: Region,
    /// Per species index: weight inside (0 = not listed).
    weights: Vec<f32>,
}

/// Compiled overrides. "Only" regions are kept sorted strongest first, so the first containing one wins.
#[derive(Clone, Debug, Default)]
pub struct Overrides {
    paints: Vec<Paint>,
    exclusions: Vec<Region>,
    only: Vec<Only>,
    explicit: Vec<ExplicitRegion>,
}

/// The overrides that can touch one rectangle (a tile): index lists into [`Overrides`].
#[derive(Clone, Debug, Default)]
pub struct OverrideView {
    paints: Vec<usize>,
    exclusions: Vec<usize>,
    only: Vec<usize>,
    explicit: Vec<usize>,
}

impl OverrideView {
    /// No override touches the rectangle.
    pub fn is_empty(&self) -> bool {
        self.paints.is_empty() && self.exclusions.is_empty() && self.only.is_empty() && self.explicit.is_empty()
    }
}

/// What the overrides do to one species at one point.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) enum Effect {
    /// Density 0 (exclusion, explicit placement of the same class, or not in the winning "only" list).
    Zero,
    /// `only`: the winning "only" region's weight (replaces the rule weight); `paint`: the paint product.
    Scale { only: Option<f32>, paint: f32 },
}

impl Overrides {
    /// No overrides.
    pub fn none() -> Self {
        Self::default()
    }

    /// Validates the set and resolves its species ids against `cover`.
    pub fn new(cover: &Groundcover, set: &OverrideSet) -> Result<Self, BuildError> {
        let resolve = |id: &str| cover.index_of(id).ok_or_else(|| BuildError::UnknownSpecies(id.to_owned()));
        let n = cover.len();

        let mut paints = Vec::with_capacity(set.paints.len());
        for p in &set.paints {
            if !(p.size_m.is_finite() && p.size_m > 0.0)
                || p.resolution == 0
                || p.texels.len() != p.resolution as usize * p.resolution as usize
                || !p.origin.iter().all(|v| v.is_finite())
            {
                return Err(BuildError::InvalidOverride("paint needs size_m > 0, resolution > 0, texels.len() == resolution²"));
            }
            let mut applies = vec![p.species.is_empty(); n];
            for id in &p.species {
                applies[resolve(id)?.get()] = true;
            }
            paints.push(Paint { paint: p.clone(), applies });
        }

        for r in &set.exclusions {
            r.validate()?;
        }
        for r in &set.explicit {
            r.region.validate()?;
        }

        let mut only: Vec<(i32, usize, Only)> = Vec::with_capacity(set.only.len());
        for (order, o) in set.only.iter().enumerate() {
            o.region.validate()?;
            let mut weights = vec![0.0; n];
            for ws in &o.species {
                if !ws.weight.is_finite() || ws.weight < 0.0 {
                    return Err(BuildError::InvalidOverride("only-region weight must be finite and >= 0"));
                }
                weights[resolve(&ws.species)?.get()] = ws.weight;
            }
            only.push((o.priority, order, Only { region: o.region.clone(), weights }));
        }
        // Strongest first: higher priority, then later in the list.
        only.sort_by(|a, b| b.0.cmp(&a.0).then(b.1.cmp(&a.1)));

        Ok(Self {
            paints,
            exclusions: set.exclusions.clone(),
            only: only.into_iter().map(|(_, _, o)| o).collect(),
            explicit: set.explicit.clone(),
        })
    }

    /// The overrides whose bounds overlap `rect` (in their stored order, so "only" stays strongest first).
    pub fn view(&self, rect: Rect) -> OverrideView {
        let pick = |rs: &mut dyn Iterator<Item = Rect>| rs.enumerate().filter(|(_, r)| r.overlaps(rect)).map(|(i, _)| i).collect();
        OverrideView {
            paints: pick(&mut self.paints.iter().map(|p| Rect {
                min: p.paint.origin,
                max: [p.paint.origin[0] + p.paint.size_m, p.paint.origin[1] + p.paint.size_m],
            })),
            exclusions: pick(&mut self.exclusions.iter().map(Region::bounds)),
            only: pick(&mut self.only.iter().map(|o| o.region.bounds())),
            explicit: pick(&mut self.explicit.iter().map(|e| e.region.bounds())),
        }
    }

    /// What the overrides in `view` do to species `s` (of class `range`) at (x, z).
    pub(crate) fn effect(&self, view: &OverrideView, x: f64, z: f64, s: SpeciesIndex, range: RangeClass) -> Effect {
        if view.explicit.iter().any(|&i| self.explicit[i].range == range && self.explicit[i].region.contains(x, z)) {
            return Effect::Zero;
        }
        if view.exclusions.iter().any(|&i| self.exclusions[i].contains(x, z)) {
            return Effect::Zero;
        }
        let only = view.only.iter().map(|&i| &self.only[i]).find(|o| o.region.contains(x, z)).map(|o| o.weights[s.get()]);
        if only == Some(0.0) {
            return Effect::Zero;
        }
        let mut paint = 1.0;
        for &i in &view.paints {
            let p = &self.paints[i];
            if !p.applies[s.get()] {
                continue;
            }
            let texel_m = p.paint.size_m / f64::from(p.paint.resolution);
            if let Some(t) = texel_at(p.paint.origin, texel_m, p.paint.resolution, p.paint.resolution, x, z) {
                paint *= f32::from(p.paint.texels[t]) / 255.0;
            }
        }
        if paint == 0.0 {
            return Effect::Zero;
        }
        Effect::Scale { only, paint }
    }
}
