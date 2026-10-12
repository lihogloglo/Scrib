//! What a bake did, for the CLI report.

use std::collections::BTreeMap;
use std::fmt;
use std::time::{Duration, Instant};

use rayon::prelude::*;
use sf_bake::{BakeError, BakeReport, StepStats, Store};
use sf_model::{CookSettings, Cooked};
use sf_model::steps::MeshCookStep;
use sf_vfs::{Vfs, VfsPath};

use crate::{ModelUse, dynamic_parts};

/// One LOD level across every cooked model that has it.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct LodLevelStats {
    /// Models whose chain reaches this level.
    pub models: u32,
    /// Triangles at this level, summed over those models.
    pub triangles: u64,
    /// LOD0 triangles of the same models.
    pub lod0_triangles: u64,
    /// Median error in meters.
    pub median_error: f32,
    /// Median error as a fraction of the model's bounding radius.
    pub median_relative_error: f32,
}

/// The analyzed result of a model bake.
#[derive(Clone, Debug, Default)]
pub struct BakeSummary {
    pub elapsed: Duration,
    pub steps: BTreeMap<&'static str, StepStats>,
    pub models: usize,
    pub cooked: usize,
    pub skipped: usize,
    /// Model files records name that the VFS doesn't have.
    pub missing: Vec<String>,
    /// Other failures, `(model, error)`.
    pub failed: Vec<(String, String)>,
    pub placements: u64,
    pub placements_cooked: u64,
    /// Reason -> (models, placements). A model can have several reasons.
    pub skip_reasons: BTreeMap<String, (usize, u64)>,
    /// Time to work out skip reasons (re-reads the skipped NIFs).
    pub reason_time: Duration,
    /// Index k: level k (LOD0 first).
    pub lods: Vec<LodLevelStats>,
    /// Index k: models whose chain has k + 1 levels.
    pub chain_lengths: Vec<u32>,
    /// Size of all cooked products (uncompressed).
    pub cooked_bytes: u64,
    /// Cooked models big enough for a LOD chain (LOD0 at least twice the
    /// cook's minimum triangle count), and how many of them got one.
    pub lod_eligible: u32,
    pub lod_eligible_with_chain: u32,
}

impl BakeSummary {
    pub fn analyze(report: &BakeReport, store: &Store, vfs: &Vfs, models: &BTreeMap<String, ModelUse>) -> Self {
        let mut summary = Self {
            elapsed: report.elapsed,
            steps: report.steps.clone(),
            models: models.len(),
            ..Self::default()
        };
        let mut skipped: Vec<(&str, String)> = Vec::new();
        // (level, error, relative error) samples, then triangles per level.
        let mut errors: Vec<Vec<(f32, f32)>> = Vec::new();
        for ((key, result), decoded) in report.products.iter().zip(decode_cooked(report, store)) {
            if key.step != MeshCookStep::ID {
                continue;
            }
            let uses = models.get(&key.subject).cloned().unwrap_or_default();
            summary.placements += u64::from(uses.placements);
            if let Err(error) = result {
                if is_missing(error) {
                    summary.missing.push(key.subject.clone());
                } else {
                    summary.failed.push((key.subject.clone(), error.to_string()));
                }
                continue;
            }
            let Some((bytes, cooked)) = decoded else { continue };
            summary.cooked_bytes += bytes;
            match cooked {
                Ok(Cooked::Static(model)) => {
                    summary.cooked += 1;
                    summary.placements_cooked += u64::from(uses.placements);
                    let levels = model.lods.len();
                    if summary.chain_lengths.len() < levels {
                        summary.chain_lengths.resize(levels, 0);
                    }
                    if let Some(n) = levels.checked_sub(1) {
                        summary.chain_lengths[n] += 1;
                    }
                    if summary.lods.len() < levels {
                        summary.lods.resize(levels, LodLevelStats::default());
                        errors.resize(levels, Vec::new());
                    }
                    let lod0 = u64::from(model.lods.first().map_or(0, sf_model::CookedLod::triangle_count));
                    if lod0 >= u64::from(CookSettings::default().min_triangles) * 2 {
                        summary.lod_eligible += 1;
                        summary.lod_eligible_with_chain += u32::from(levels > 1);
                    }
                    let radius = model.bounds.radius();
                    for (level, lod) in model.lods.iter().enumerate() {
                        let stats = &mut summary.lods[level];
                        stats.models += 1;
                        stats.triangles += u64::from(lod.triangle_count());
                        stats.lod0_triangles += lod0;
                        let relative = if radius > 0.0 { lod.error / radius } else { 0.0 };
                        errors[level].push((lod.error, relative));
                    }
                }
                Ok(Cooked::Skinned(_)) => {
                    summary.skipped += 1;
                    skipped.push((key.subject.as_str(), "deforming (skinned cook)".to_owned()));
                }
                Ok(Cooked::Skipped { reason }) => {
                    summary.skipped += 1;
                    skipped.push((key.subject.as_str(), reason));
                }
                Err(error) => summary.failed.push((key.subject.clone(), error)),
            }
        }
        for (stats, samples) in summary.lods.iter_mut().zip(&mut errors) {
            stats.median_error = median(samples.iter().map(|s| s.0).collect());
            stats.median_relative_error = median(samples.iter().map(|s| s.1).collect());
        }

        let started = Instant::now();
        let reasons: Vec<(Vec<String>, u64)> = skipped
            .par_iter()
            .map(|(path, cook_reason)| {
                let parts = VfsPath::new(path)
                    .ok()
                    .and_then(|p| vfs.read(&p).ok())
                    .and_then(|bytes| mw_nif::load(&bytes).ok())
                    .map(|nif| dynamic_parts(&nif).into_iter().map(|d| d.label().to_owned()).collect::<Vec<_>>())
                    .filter(|parts| !parts.is_empty())
                    .unwrap_or_else(|| vec![cook_reason.clone()]);
                (parts, u64::from(models.get(*path).map_or(0, |u| u.placements)))
            })
            .collect();
        for (parts, placements) in reasons {
            for part in parts {
                let entry = summary.skip_reasons.entry(part).or_default();
                entry.0 += 1;
                entry.1 += placements;
            }
        }
        summary.reason_time = started.elapsed();
        summary
    }
}

/// Each `mesh.cook` product's stored size and decoded model, in report order
/// (`None` for other steps and failed products). Reading ~6.6k products one
/// by one was most of a warm bake (1.0 s); in parallel it is ~0.1 s.
fn decode_cooked(report: &BakeReport, store: &Store) -> Vec<Option<(u64, Result<Cooked, String>)>> {
    report
        .products
        .par_iter()
        .map(|(key, result)| {
            let digest = result.as_ref().ok().filter(|_| key.step == MeshCookStep::ID)?;
            Some(match store.get(digest) {
                Ok(bytes) => (bytes.len() as u64, Cooked::from_bytes(&bytes).map_err(|e| e.to_string())),
                Err(e) => (0, Err(e.to_string())),
            })
        })
        .collect()
}

fn is_missing(error: &BakeError) -> bool {
    match error {
        BakeError::MissingInput(_) => true,
        BakeError::Dependency { source, .. } => is_missing(source),
        _ => false,
    }
}

fn median(mut values: Vec<f32>) -> f32 {
    if values.is_empty() {
        return 0.0;
    }
    values.sort_by(f32::total_cmp);
    values[values.len() / 2]
}

#[allow(clippy::cast_precision_loss)]
fn percent(part: u64, whole: u64) -> f64 {
    if whole == 0 { 0.0 } else { part as f64 * 100.0 / whole as f64 }
}

impl fmt::Display for BakeSummary {
    #[allow(clippy::cast_precision_loss)]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "steps:")?;
        writeln!(f, "  {:<12} {:>7} {:>7} {:>9} {:>7} {:>10} {:>10}", "step", "hits", "built", "overrides", "failed", "run (cpu)", "written")?;
        for (step, s) in &self.steps {
            writeln!(
                f,
                "  {step:<12} {:>7} {:>7} {:>9} {:>7} {:>9.1}s {:>8.1} MB",
                s.hits,
                s.built,
                s.overrides,
                s.failed,
                s.run_time.as_secs_f64(),
                s.bytes_written as f64 / 1e6
            )?;
        }
        let total = self.models as u64;
        writeln!(
            f,
            "\nmodels: {} referenced; {} cooked ({:.1}%), {} skipped, {} missing, {} failed",
            self.models,
            self.cooked,
            percent(self.cooked as u64, total),
            self.skipped,
            self.missing.len(),
            self.failed.len()
        )?;
        writeln!(
            f,
            "placed references: {} of {} use a cooked model ({:.1}%)",
            self.placements_cooked,
            self.placements,
            percent(self.placements_cooked, self.placements)
        )?;
        writeln!(f, "cooked products: {:.1} MB uncompressed", self.cooked_bytes as f64 / 1e6)?;

        let mut reasons: Vec<(&String, &(usize, u64))> = self.skip_reasons.iter().collect();
        reasons.sort_by(|a, b| b.1.0.cmp(&a.1.0).then(a.0.cmp(b.0)));
        writeln!(f, "\nskip reasons (top 10; a model can have several; {:.2?} to analyze):", self.reason_time)?;
        for (reason, (models, placements)) in reasons.iter().take(10) {
            writeln!(f, "  {models:>6} models {placements:>7} placements  {reason}")?;
        }

        writeln!(f, "\nLOD chains (levels including LOD0: models):")?;
        let lengths: Vec<String> =
            self.chain_lengths.iter().enumerate().map(|(i, n)| format!("{}: {n}", i + 1)).collect();
        writeln!(f, "  {}", lengths.join(", "))?;
        writeln!(
            f,
            "  {} of {} models with LOD0 >= {} triangles got a chain",
            self.lod_eligible_with_chain,
            self.lod_eligible,
            CookSettings::default().min_triangles * 2
        )?;
        writeln!(f, "  level  models   triangles   of LOD0  median error  median error/radius")?;
        for (level, s) in self.lods.iter().enumerate() {
            writeln!(
                f,
                "  LOD{level}  {:>7} {:>11} {:>8.1}% {:>11.4} m {:>14.3}%",
                s.models,
                s.triangles,
                percent(s.triangles, s.lod0_triangles),
                s.median_error,
                s.median_relative_error * 100.0
            )?;
        }

        for path in self.missing.iter().take(5) {
            writeln!(f, "missing: {path}")?;
        }
        if self.missing.len() > 5 {
            writeln!(f, "missing: ... {} more", self.missing.len() - 5)?;
        }
        for (path, error) in self.failed.iter().take(10) {
            writeln!(f, "failed: {path}: {error}")?;
        }
        writeln!(f, "\nwall time: {:.2?}", self.elapsed)
    }
}
