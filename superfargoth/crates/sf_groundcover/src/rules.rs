//! The rule table (input channels → weighted species) and [`Groundcover`], the validated species set with its
//! compiled rules (spec "Data model"). Channels are opaque indices into the per-point weight vector the
//! [`GroundSampler`](crate::GroundSampler) fills: terrain layer weights, worldgen material fields, whatever the
//! caller decides each index means.

use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::hash::species_hash;
use crate::species::{Span, Species};

/// A species and its weight in a rule or region list.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WeightedSpecies {
    pub species: String,
    pub weight: f32,
}

/// One input channel and the species it grows.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ChannelRule {
    /// Index into the sampled channel weights; a channel the sampler does not fill weighs 0.
    pub channel: u16,
    pub species: Vec<WeightedSpecies>,
}

/// Input channels → weighted species lists. A species' rule weight at a point is
/// `Σ rule weight × channel weight` over every rule listing it.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct RuleTable {
    pub rules: Vec<ChannelRule>,
}

/// Index of a species in its [`Groundcover`].
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SpeciesIndex(pub u16);

impl SpeciesIndex {
    #[inline]
    pub const fn get(self) -> usize {
        self.0 as usize
    }
}

#[derive(Debug, Error, PartialEq)]
pub enum BuildError {
    #[error("duplicate species id {0:?}")]
    DuplicateSpecies(String),
    #[error("unknown species id {0:?}")]
    UnknownSpecies(String),
    #[error("species {id:?}: {what}")]
    InvalidSpecies { id: String, what: &'static str },
    #[error("more than {max} species")]
    TooManySpecies { max: usize },
    #[error("{0}")]
    InvalidOverride(&'static str),
}

/// A species with what the scatter precomputes from it.
#[derive(Clone, Debug)]
pub(crate) struct Compiled {
    pub species: Species,
    pub id_hash: u64,
    /// `cos` of the slope limits: slope in `[min, max]` ⇔ `normal.y ∈ [cos max, cos min]`.
    pub cos_slope: Option<(f32, f32)>,
    /// `(channel, weight)` pairs from every rule listing this species.
    pub rules: Vec<(u16, f32)>,
}

/// A validated species set and its compiled rule table: what the scatter and the density twin read.
#[derive(Clone, Debug)]
pub struct Groundcover {
    pub(crate) species: Vec<Compiled>,
}

impl Groundcover {
    /// Validates the species and resolves the rule table's ids.
    pub fn new(species: Vec<Species>, rules: &RuleTable) -> Result<Self, BuildError> {
        if species.len() > usize::from(u16::MAX) {
            return Err(BuildError::TooManySpecies { max: usize::from(u16::MAX) });
        }
        let mut out: Vec<Compiled> = Vec::with_capacity(species.len());
        for s in species {
            validate(&s)?;
            if out.iter().any(|c| c.species.id == s.id) {
                return Err(BuildError::DuplicateSpecies(s.id));
            }
            let cos_slope = s.slope_deg.map(|span| {
                // Lower slope = higher normal.y.
                (span.max.to_radians().cos(), span.min.to_radians().cos())
            });
            out.push(Compiled { id_hash: species_hash(&s.id), cos_slope, rules: Vec::new(), species: s });
        }
        let mut cover = Self { species: out };
        for rule in &rules.rules {
            for ws in &rule.species {
                let i = cover.index_of(&ws.species).ok_or_else(|| BuildError::UnknownSpecies(ws.species.clone()))?;
                if !ws.weight.is_finite() || ws.weight < 0.0 {
                    return Err(BuildError::InvalidSpecies { id: ws.species.clone(), what: "rule weight must be finite and >= 0" });
                }
                cover.species[i.get()].rules.push((rule.channel, ws.weight));
            }
        }
        Ok(cover)
    }

    pub fn len(&self) -> usize {
        self.species.len()
    }

    pub fn is_empty(&self) -> bool {
        self.species.is_empty()
    }

    pub fn species(&self, i: SpeciesIndex) -> &Species {
        &self.species[i.get()].species
    }

    pub fn index_of(&self, id: &str) -> Option<SpeciesIndex> {
        self.species.iter().position(|c| c.species.id == id).map(|i| {
            // `new` caps the count at u16::MAX.
            #[allow(clippy::cast_possible_truncation)]
            let i = i as u16;
            SpeciesIndex(i)
        })
    }

    /// Every species index, in declaration order.
    pub fn indices(&self) -> impl Iterator<Item = SpeciesIndex> + '_ {
        (0..self.species.len()).map(|i| {
            #[allow(clippy::cast_possible_truncation)]
            let i = i as u16;
            SpeciesIndex(i)
        })
    }

    /// The rule weight of a species for a channel weight vector: `Σ rule weight × channel weight`.
    pub fn rule_weight(&self, i: SpeciesIndex, channels: &[f32]) -> f32 {
        self.species[i.get()]
            .rules
            .iter()
            .map(|&(ch, w)| w * channels.get(usize::from(ch)).copied().unwrap_or(0.0))
            .sum()
    }
}

fn validate(s: &Species) -> Result<(), BuildError> {
    let bad = |what| Err(BuildError::InvalidSpecies { id: s.id.clone(), what });
    if !(s.footprint_m.is_finite() && s.footprint_m > 0.0) {
        return bad("footprint_m must be finite and > 0");
    }
    if !(s.density.is_finite() && s.density >= 0.0) {
        return bad("density must be finite and >= 0");
    }
    let ranges: [Option<Span>; 6] =
        [s.slope_deg, s.altitude_m, s.water_m, Some(s.scale), Some(s.yaw_deg), Some(s.align_to_surface)];
    for span in ranges {
        if span.is_some_and(|r| !r.is_valid()) {
            return bad("a range has min > max or a non-finite end");
        }
    }
    if !(0.0..=1.0).contains(&s.align_to_surface.min) || !(0.0..=1.0).contains(&s.align_to_surface.max) {
        return bad("align_to_surface must lie in [0, 1]");
    }
    if s.scale.min <= 0.0 {
        return bad("scale must be > 0");
    }
    Ok(())
}
