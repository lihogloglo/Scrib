//! Climate classes (pipeline stage 6 (d)): one class id per sample from a (temperature, precipitation) table.
//!
//! The table is data, a rectilinear grid: ascending upper edges of the temperature bands (C) and of the
//! precipitation bands (mm/yr), and one class name per (temperature band, precipitation band) cell. A value below the
//! first edge is in the first band, at or above the last edge in the last band, exactly on an edge in the band above
//! it; NaN goes to the first band. A class id is the index of its name in order of first appearance (rows, then
//! columns). The shipped default (`climate_classes.toml`, [`ClassTable::whittaker`]) is a Whittaker-style biome table
//! whose band edges are read off Whittaker's 1975 diagram (its `source` line); games ship their own table in the same
//! format. The raw temperature and precipitation rasters stay the inputs of scatter rules; the class is a label next
//! to them.

use rayon::prelude::*;
use serde::Deserialize;

use crate::grid::Grid;
use crate::{Error, Result};

#[cfg(test)]
#[path = "tests_climate_classes.rs"]
mod tests;

/// The on-disk table (TOML).
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct TableFile {
    source: String,
    t_edges_c: Vec<f64>,
    p_edges_mm: Vec<f64>,
    classes: Vec<Vec<String>>,
}

/// A (temperature, precipitation) class table: band edges, one class id per cell, the class names.
#[derive(Debug, Clone, PartialEq)]
pub struct ClassTable {
    /// Upper edges of the temperature bands (C), strictly ascending.
    t_edges: Vec<f64>,
    /// Upper edges of the precipitation bands (mm/yr), strictly ascending.
    p_edges: Vec<f64>,
    /// Class id per cell, row-major: `(t_edges.len() + 1)` rows by `(p_edges.len() + 1)` columns.
    cells: Vec<u8>,
    /// Class names; an id is an index here.
    names: Vec<String>,
}

/// Checks one edge list: not empty, finite, strictly ascending.
fn check_edges(what: &str, edges: &[f64]) -> Result<()> {
    if edges.is_empty() {
        return Err(Error::Invalid(format!("climate classes: {what} is empty")));
    }
    if let Some(e) = edges.iter().find(|e| !e.is_finite()) {
        return Err(Error::Invalid(format!("climate classes: {what} has a non-finite edge {e}")));
    }
    if let Some((a, b)) = edges.iter().zip(&edges[1..]).find(|(a, b)| a >= b) {
        return Err(Error::Invalid(format!(
            "climate classes: {what} is not strictly ascending ({a} then {b})"
        )));
    }
    Ok(())
}

/// Band index of `v` among ascending upper `edges`: the number of edges at or below it (NaN gives 0).
fn band(edges: &[f64], v: f64) -> usize {
    edges.partition_point(|e| *e <= v)
}

impl ClassTable {
    /// The shipped Whittaker table.
    #[must_use]
    pub fn whittaker() -> Self {
        // The shipped text is fixed and parsed by a test (`the_shipped_table_parses`).
        Self::parse_toml(include_str!("climate_classes.toml"))
            .unwrap_or_else(|e| unreachable!("shipped climate_classes.toml: {e}"))
    }

    /// Parses a table file (TOML: `source`, `t_edges_c`, `p_edges_mm`, `classes`).
    ///
    /// # Errors
    /// An unknown key, an invalid `source`, edges that are empty, not finite or not strictly ascending, a `classes`
    /// matrix that is not (t edges + 1) rows by (p edges + 1) columns, an empty class name, or more than 255
    /// distinct names.
    pub fn parse_toml(text: &str) -> Result<Self> {
        let file: TableFile = toml::from_str(text)?;
        crate::recipe::check_source("climate classes", &file.source)?;
        check_edges("t_edges_c", &file.t_edges_c)?;
        check_edges("p_edges_mm", &file.p_edges_mm)?;
        let rows = file.t_edges_c.len() + 1;
        let cols = file.p_edges_mm.len() + 1;
        if file.classes.len() != rows {
            return Err(Error::Invalid(format!(
                "climate classes: {} rows, expected {rows} (t_edges_c + 1)",
                file.classes.len()
            )));
        }
        let mut names: Vec<String> = Vec::new();
        let mut cells = Vec::with_capacity(rows * cols);
        for (r, row) in file.classes.iter().enumerate() {
            if row.len() != cols {
                return Err(Error::Invalid(format!(
                    "climate classes: row {r} has {} classes, expected {cols} (p_edges_mm + 1)",
                    row.len()
                )));
            }
            for name in row {
                if name.is_empty() {
                    return Err(Error::Invalid(format!("climate classes: empty class name in row {r}")));
                }
                let id = if let Some(i) = names.iter().position(|n| n == name) {
                    i
                } else {
                    if names.len() == usize::from(u8::MAX) {
                        return Err(Error::Invalid(format!(
                            "climate classes: more than {} distinct names",
                            u8::MAX
                        )));
                    }
                    names.push(name.clone());
                    names.len() - 1
                };
                cells.push(
                    u8::try_from(id)
                        .map_err(|_| Error::Invalid(format!("climate classes: class id {id} past u8")))?,
                );
            }
        }
        Ok(Self {
            t_edges: file.t_edges_c,
            p_edges: file.p_edges_mm,
            cells,
            names,
        })
    }

    /// Class names; a class id is its index here (order of first appearance, rows then columns).
    #[must_use]
    pub fn names(&self) -> &[String] {
        &self.names
    }

    /// Class id of one (temperature C, precipitation mm/yr) pair.
    #[must_use]
    pub fn classify(&self, t_c: f64, p_mm: f64) -> u8 {
        let cols = self.p_edges.len() + 1;
        self.cells[band(&self.t_edges, t_c) * cols + band(&self.p_edges, p_mm)]
    }
}

/// Class id per sample.
///
/// # Errors
/// `Error::Invalid` when the two grids differ in size.
pub fn classes(temperature: &Grid, rain: &Grid, table: &ClassTable) -> Result<Vec<u8>> {
    if temperature.w != rain.w || temperature.h != rain.h || temperature.data.len() != rain.data.len() {
        return Err(Error::Invalid(format!(
            "climate classes: temperature grid {}x{} ({} samples), rain grid {}x{} ({} samples)",
            temperature.w,
            temperature.h,
            temperature.data.len(),
            rain.w,
            rain.h,
            rain.data.len()
        )));
    }
    Ok(temperature
        .data
        .par_iter()
        .zip(rain.data.par_iter())
        .map(|(&t, &p)| table.classify(t, p))
        .collect())
}
