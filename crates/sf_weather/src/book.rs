//! [`WeatherBook`]: the weather records resolved into index form, built
//! once from the record database (and again when weather records change),
//! so `evaluate` does no id lookups.

use std::collections::HashMap;

use sf_records::{RecordDb, RecordId};

use crate::hash;
use crate::records::{Climate, PrecipKind, WeatherPreset};

/// Index of a preset in a [`WeatherBook`].
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct PresetIx(pub u32);

/// Index of a climate in a [`WeatherBook`].
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ClimateIx(pub u32);

/// Index of a precipitation kind in a [`WeatherBook`].
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct PrecipIx(pub u32);

pub(crate) struct BookPreset {
    pub id: RecordId,
    pub key: u64,
    pub record: WeatherPreset,
    /// `record.precipitation` resolved (unknown kinds dropped).
    pub precip: Vec<(PrecipIx, f32)>,
}

pub(crate) struct BookClimate {
    pub id: RecordId,
    pub key: u64,
    pub record: Climate,
    /// Resolved positive weights.
    pub weights: Vec<(PresetIx, f32)>,
    /// Interval mean and the jitter half-amplitude (a quarter of the range),
    /// game hours; and the transition clamped to the minimum interval.
    pub mean: f64,
    pub jitter: f64,
    pub min_interval: f64,
    pub transition: f64,
}

/// Resolved weather records.
#[derive(Default)]
pub struct WeatherBook {
    pub(crate) presets: Vec<BookPreset>,
    pub(crate) climates: Vec<BookClimate>,
    pub(crate) precip: Vec<(RecordId, PrecipKind)>,
    preset_index: HashMap<RecordId, PresetIx>,
    climate_index: HashMap<RecordId, ClimateIx>,
    /// Unresolved references and bad values, for the mod report.
    pub warnings: Vec<String>,
}

fn ix(i: usize) -> u32 {
    u32::try_from(i).expect("fewer than 2^32 weather records")
}

impl WeatherBook {
    /// Resolves the live weather records of a database (kinds registered by
    /// [`crate::register_kinds`]).
    pub fn from_db(db: &RecordDb) -> Self {
        Self::new(
            db.iter::<WeatherPreset>().map(|(id, p)| (id.clone(), p.clone())),
            db.iter::<Climate>().map(|(id, c)| (id.clone(), c.clone())),
            db.iter::<PrecipKind>().map(|(id, k)| (id.clone(), k.clone())),
        )
    }

    /// Resolves records given directly (tests, tools).
    pub fn new(
        presets: impl IntoIterator<Item = (RecordId, WeatherPreset)>,
        climates: impl IntoIterator<Item = (RecordId, Climate)>,
        precip: impl IntoIterator<Item = (RecordId, PrecipKind)>,
    ) -> Self {
        let mut book = Self { precip: precip.into_iter().collect(), ..Self::default() };
        let precip_index: HashMap<RecordId, PrecipIx> =
            book.precip.iter().enumerate().map(|(i, (id, _))| (id.clone(), PrecipIx(ix(i)))).collect();
        for (id, record) in presets {
            let mut resolved = Vec::new();
            for term in &record.precipitation {
                match precip_index.get(&term.kind) {
                    Some(&k) => resolved.push((k, term.intensity.max(0.0))),
                    None => book.warnings.push(format!("preset {id}: unknown precipitation kind {}", term.kind)),
                }
            }
            book.preset_index.insert(id.clone(), PresetIx(ix(book.presets.len())));
            book.presets.push(BookPreset { key: hash::key(id.as_str()), id, record, precip: resolved });
        }
        for (id, record) in climates {
            let weights = book.resolve_weights(&id, &record.weights);
            let (a, b) = record.change_interval_hours;
            let (a, b) = (f64::from(a.max(0.01)), f64::from(b));
            let b = b.max(a);
            if weights.is_empty() {
                book.warnings.push(format!("climate {id}: no preset with a positive weight"));
            }
            book.climate_index.insert(id.clone(), ClimateIx(ix(book.climates.len())));
            book.climates.push(BookClimate {
                key: hash::key(id.as_str()),
                id,
                weights,
                mean: f64::midpoint(a, b),
                jitter: 0.25 * (b - a),
                min_interval: a,
                transition: f64::from(record.transition_hours.max(0.0)).min(a),
                record,
            });
        }
        book
    }

    pub(crate) fn resolve_weights(&mut self, owner: &RecordId, weights: &[(RecordId, f32)]) -> Vec<(PresetIx, f32)> {
        let mut out = Vec::new();
        for (preset, w) in weights {
            match self.preset_index.get(preset) {
                Some(&p) if *w > 0.0 => out.push((p, *w)),
                Some(_) => {}
                None => self.warnings.push(format!("climate {owner}: unknown preset {preset}")),
            }
        }
        out
    }

    pub fn preset(&self, id: &RecordId) -> Option<PresetIx> {
        self.preset_index.get(id).copied()
    }

    pub fn climate(&self, id: &RecordId) -> Option<ClimateIx> {
        self.climate_index.get(id).copied()
    }

    pub fn preset_id(&self, p: PresetIx) -> &RecordId {
        &self.presets[p.0 as usize].id
    }

    pub fn preset_record(&self, p: PresetIx) -> &WeatherPreset {
        &self.presets[p.0 as usize].record
    }

    pub fn climate_id(&self, c: ClimateIx) -> &RecordId {
        &self.climates[c.0 as usize].id
    }

    pub fn climate_record(&self, c: ClimateIx) -> &Climate {
        &self.climates[c.0 as usize].record
    }

    pub fn precip_id(&self, k: PrecipIx) -> &RecordId {
        &self.precip[k.0 as usize].0
    }

    pub fn precip_kind(&self, k: PrecipIx) -> &PrecipKind {
        &self.precip[k.0 as usize].1
    }

    pub(crate) fn p(&self, p: PresetIx) -> &BookPreset {
        &self.presets[p.0 as usize]
    }

    pub(crate) fn c(&self, c: ClimateIx) -> &BookClimate {
        &self.climates[c.0 as usize]
    }
}
