//! Furniture interactable defs (`mw.interact.defs`, `IN32b2`; `bevyrrowind/docs/specs/interaction-adapter.md`
//! "Defs from the census", "As built (`IN32b2`)").
//!
//! One product (subject [`SUBJECT`]): every census furniture base record ([`census::furniture_records`]) with
//! its `InteractableDef`, keyed by base `RecordId` (sorted), plus the counts the bake report prints.
//!
//! Per record: the model's census triangles ([`census::model_soups`]), collision first, render geometry when the
//! collision has no usable top (the census rule, [`census::Shape::top_below`] under [`Kind::cap`]); that soup
//! and top go to [`mw_interaction::measure()`], then [`mw_interaction::def_for`] with the
//! [`FurnitureParams`] defaults. **Floor offset:** slots and approach points stand on the measured floor (the
//! soup's lowest point, [`Measured::floor`]), so a model authored with its origin off its floor still seats
//! its user on the ground; the report counts the models whose floor is off the origin by a centimetre or more.
//! Shelves and unmeasured records (no model, no usable top) get no def. **Class verb** (`IN32c0`): a def'd
//! `CONT` or `ACTI` keeps its class's primary use ([`mw_interaction::class_verb`]: open, use) as a slotless
//! verb ranked above its pose verbs; the report counts them ([`InteractDefs::container_defs`],
//! [`InteractDefs::activator_defs`]).
//!
//! The record list (id, kind, model) and both parameter sets are the step's plan parameters, every model a
//! tracked file read, so a mod editing a furniture record or replacing its mesh re-bakes the defs. Nothing is
//! registered at runtime yet (`IN32c` reads [`decode_interact_defs`]).

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;
use std::sync::Arc;
use std::time::{Duration, Instant};

use mw_interaction::{
    Basis, FurnitureParams, InteractableDef, Kind, MeasureParams, Measured, RecordClass, class_verb, def_for, measure,
};
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use sf_bake::{BakeError, Baker, PlanCx, ProductKey, RunCx, Step};
use sf_records::{RecordDb, RecordId};

use crate::census::{self, FurnitureRecord, Soups};

#[cfg(test)]
#[path = "interact_tests.rs"]
mod tests;

/// The step id.
pub const INTERACT_DEFS: &str = "mw.interact.defs";
/// The one subject: every furniture record.
pub const SUBJECT: &str = "furniture";

/// Counts of one census kind.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct KindCounts {
    /// Census base records of this kind.
    pub records: u32,
    pub defs: u32,
    pub slots: u32,
    /// Records with no measured shape (model failed or missing, no usable top).
    pub unmeasured: u32,
    /// Measured on the render geometry (the collision had no usable top).
    pub render: u32,
    /// Measured with the floor off the model origin by at least 1 cm.
    pub floor_offset: u32,
    /// Measured records by facing rule: geometry (backrest, headboard), long side, +Z, long end.
    pub upright: u32,
    pub long_side: u32,
    pub plus_z: u32,
    pub long_end: u32,
}

impl KindCounts {
    fn add_basis(&mut self, basis: Basis) {
        match basis {
            Basis::Upright => self.upright += 1,
            Basis::LongSide => self.long_side += 1,
            Basis::PlusZ => self.plus_z += 1,
            Basis::LongEnd => self.long_end += 1,
        }
    }
}

/// The `mw.interact.defs` product.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct InteractDefs {
    /// Sorted by base id.
    pub defs: Vec<InteractableDef>,
    /// By kind key ([`Kind::key`]).
    pub counts: BTreeMap<String, KindCounts>,
    /// Model path -> why it gave no triangles (missing, load error).
    pub failed: Vec<(String, String)>,
    /// Defs of `CONT` records (each with the container's open verb).
    pub container_defs: u32,
    /// Defs of `ACTI` records (each with the activator's use verb).
    pub activator_defs: u32,
}

impl InteractDefs {
    /// The counts of `kind` (zero when none).
    pub fn count(&self, kind: Kind) -> KindCounts {
        self.counts.get(kind.key()).cloned().unwrap_or_default()
    }

    /// The def of a base record.
    pub fn def(&self, base: &RecordId) -> Option<&InteractableDef> {
        self.defs.binary_search_by(|d| d.id.cmp(base)).ok().map(|i| &self.defs[i])
    }
}

/// Decodes a `mw.interact.defs` product.
pub fn decode_interact_defs(bytes: &[u8]) -> Result<InteractDefs, postcard::Error> {
    postcard::from_bytes(bytes)
}

/// One record measured: its soup's census top, then [`measure()`] (collision first, render fallback), with
/// the source used (`"collision"` / `"render"`).
pub fn measure_record(kind: Kind, soups: &Soups, p: &MeasureParams) -> Option<(Measured, &'static str)> {
    let (collision, render) = soups;
    [(collision, "collision"), (render, "render")].into_iter().find_map(|(soup, source)| {
        let top = census::shape(soup, source)?.top_below(kind.cap())?;
        Some((measure(kind, soup, top, p)?, source))
    })
}

/// The record class whose own primary use a def'd census record keeps (`STAT`: none).
fn record_class(record: &str) -> Option<RecordClass> {
    match record {
        "CONT" => Some(RecordClass::Container),
        "ACTI" => Some(RecordClass::Activator),
        _ => None,
    }
}

/// The product from the records and their models' soups (model path, lowercase -> soups or why none).
pub fn interact_defs(
    records: &[FurnitureRecord],
    soups: &BTreeMap<String, Result<Soups, String>>,
    furniture: &FurnitureParams,
    measuring: &MeasureParams,
) -> InteractDefs {
    let mut out = InteractDefs::default();
    for kind in Kind::ALL {
        out.counts.insert(kind.key().to_owned(), KindCounts::default());
    }
    for r in records {
        let counts = out.counts.entry(r.kind.key().to_owned()).or_default();
        counts.records += 1;
        let measured = match soups.get(&r.model.to_ascii_lowercase()) {
            Some(Ok(s)) => measure_record(r.kind, s, measuring),
            _ => None,
        };
        let Some((m, source)) = measured else {
            counts.unmeasured += 1;
            continue;
        };
        counts.add_basis(m.basis);
        counts.render += u32::from(source == "render");
        // Centimetres: the report's precision.
        counts.floor_offset += u32::from((m.floor * 100.0).round().abs() >= 1.0);
        if let Some(mut def) = def_for(&RecordId::new(&r.id), r.kind, &m, furniture) {
            if let Some(verb) = record_class(r.record).and_then(class_verb) {
                def.verbs.push(verb);
                match r.record {
                    "CONT" => out.container_defs += 1,
                    _ => out.activator_defs += 1,
                }
            }
            counts.defs += 1;
            counts.slots += u32::try_from(def.slots.len()).unwrap_or(u32::MAX);
            out.defs.push(def);
        }
    }
    out.defs.sort_by(|a, b| a.id.cmp(&b.id));
    out.failed = soups
        .iter()
        .filter_map(|(path, s)| s.as_ref().err().map(|e| (path.clone(), e.clone())))
        .collect();
    out
}

/// `mw.interact.defs` over a fixed record list.
pub struct InteractDefsStep {
    records: Vec<FurnitureRecord>,
    furniture: FurnitureParams,
    measuring: MeasureParams,
}

impl InteractDefsStep {
    /// Bump when the product's bytes change for the same inputs.
    pub const VERSION: u32 = 2;

    pub fn new(records: Vec<FurnitureRecord>, furniture: FurnitureParams, measuring: MeasureParams) -> Self {
        Self { records, furniture, measuring }
    }

    /// The record list as a plan parameter: one `id kind model` line per record.
    fn records_param(&self) -> Vec<u8> {
        use std::fmt::Write as _;
        self.records
            .iter()
            .fold(String::new(), |mut out, r| {
                let _ = writeln!(out, "{} {} {}", r.id, r.kind.key(), r.model);
                out
            })
            .into_bytes()
    }
}

impl Step for InteractDefsStep {
    fn id(&self) -> &'static str {
        INTERACT_DEFS
    }

    fn version(&self) -> u32 {
        Self::VERSION
    }

    fn plan(&self, _subject: &str, cx: &mut PlanCx) -> Result<(), BakeError> {
        cx.param("mw.interact.records", self.records_param())
            .param("mw.interact.furniture_params", format!("{:?}", self.furniture).into_bytes())
            .param("mw.interact.measure_params", format!("{:?}", self.measuring).into_bytes());
        Ok(())
    }

    fn run(&self, subject: &str, cx: &mut RunCx<'_>) -> Result<Vec<u8>, BakeError> {
        let models: BTreeSet<String> = self.records.iter().map(|r| r.model.to_ascii_lowercase()).collect();
        // Tracked reads, in path order; parsing in parallel.
        let mut bytes: Vec<(String, Result<Vec<u8>, String>)> = Vec::with_capacity(models.len());
        for path in models {
            let read = if cx.file_exists(&path)? {
                cx.read_file(&path).map_err(|e| e.to_string())
            } else {
                Err("missing".to_owned())
            };
            bytes.push((path, read));
        }
        let soups: BTreeMap<String, Result<Soups, String>> = bytes
            .into_par_iter()
            .map(|(path, read)| {
                let soups =
                    read.and_then(|b| mw_nif::load(&b).map_err(|e| e.to_string())).map(|nif| census::model_soups(&nif));
                (path, soups)
            })
            .collect();
        let out = interact_defs(&self.records, &soups, &self.furniture, &self.measuring);
        postcard::to_stdvec(&out).map_err(|e| BakeError::step(&ProductKey::new(INTERACT_DEFS, subject), e))
    }
}

/// What [`bake_interact`] did.
#[derive(Debug, Default)]
pub struct InteractReport {
    /// The product, when it baked and decoded.
    pub defs: Option<InteractDefs>,
    pub bytes: usize,
    /// Records to the step.
    pub prepare: Duration,
    /// The bake (a hit when nothing changed).
    pub bake: Duration,
    pub failures: Vec<(ProductKey, Arc<BakeError>)>,
}

impl fmt::Display for InteractReport {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "interact defs:")?;
        if let Some(d) = &self.defs {
            for kind in Kind::ALL {
                let c = d.count(kind);
                write!(f, " {} {} records, {} defs, {} slots", kind.key(), c.records, c.defs, c.slots)?;
                let rules = [
                    ("upright", c.upright),
                    ("long side", c.long_side),
                    ("+z", c.plus_z),
                    ("long end", c.long_end),
                    ("unmeasured", c.unmeasured),
                    ("render", c.render),
                    ("floor offset", c.floor_offset),
                ];
                let shown: Vec<String> =
                    rules.iter().filter(|(_, n)| *n > 0).map(|(name, n)| format!("{name} {n}")).collect();
                if !shown.is_empty() {
                    write!(f, " ({})", shown.join(", "))?;
                }
                write!(f, ";")?;
            }
            write!(f, " CONT {} defs, ACTI {} defs (class verb kept);", d.container_defs, d.activator_defs)?;
            write!(f, " {} models failed;", d.failed.len())?;
        }
        write!(
            f,
            " {} bytes, prepare {:.2?}, bake {:.2?}, {} failed",
            self.bytes,
            self.prepare,
            self.bake,
            self.failures.len()
        )
    }
}

/// Registers `mw.interact.defs` over the records' furniture (default parameters) and bakes it.
pub fn bake_interact(baker: &mut Baker, db: &RecordDb) -> InteractReport {
    let t = Instant::now();
    let step = InteractDefsStep::new(census::furniture_records(db), FurnitureParams::default(), MeasureParams::default());
    let mut report = InteractReport { prepare: t.elapsed(), ..InteractReport::default() };
    baker.register(step);
    let key = ProductKey::new(INTERACT_DEFS, SUBJECT);
    let t = Instant::now();
    let r = baker.bake(std::slice::from_ref(&key));
    report.bake = t.elapsed();
    report.failures.extend(r.failures().map(|(k, e)| (k.clone(), e.clone())));
    if let Ok(d) = baker.product(&key)
        && let Ok(bytes) = baker.store().get(&d)
    {
        report.bytes = bytes.len();
        report.defs = decode_interact_defs(&bytes).ok();
    }
    report
}
