//! The saved form (surface-weather.md "Persistence and co-op"): a field of the `sf/weather`
//! section, versioned by it. Co-op hosts send the same bytes on join.

use std::collections::HashMap;

use glam::IVec2;
use serde::{Deserialize, Serialize};
use sf_records::RecordId;
use sf_weather::WeatherBook;

use crate::{CellSurface, ClimateSurface, SurfaceState, Worldspace};

/// A saved cell: `(cell x, cell z, snow, ash)`.
pub type SavedCell = (i32, i32, u16, u16);

/// A worldspace's saved cells.
pub type SavedCells = (Worldspace, Vec<SavedCell>);

/// One quantization step of a saved channel.
pub const QUANTUM: f32 = 1.0 / 65535.0;

/// [`SurfaceState`] quantized to 1/65535, non-zero entries only, sorted (equal states give
/// equal bytes).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SurfaceSave {
    /// Game hours of the last step (`-∞`: never stepped).
    pub t_h: f64,
    /// `(climate, wet, puddle)` by climate id.
    pub climates: Vec<(RecordId, u16, u16)>,
    /// Per worldspace, `(cell x, cell z, snow, ash)`.
    pub cells: Vec<SavedCells>,
}

impl Default for SurfaceSave {
    fn default() -> Self {
        Self { t_h: f64::NEG_INFINITY, climates: Vec::new(), cells: Vec::new() }
    }
}

#[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
fn q(v: f32) -> u16 {
    (v.clamp(0.0, 1.0) * 65535.0).round() as u16
}

fn dq(v: u16) -> f32 {
    f32::from(v) / 65535.0
}

impl SurfaceState {
    /// The saved form; climates by their ids in `book`.
    pub fn to_save(&self, book: &WeatherBook) -> SurfaceSave {
        let mut climates: Vec<(RecordId, u16, u16)> = self
            .climates
            .iter()
            .enumerate()
            .take(book.climate_count())
            .map(|(i, c)| (sf_weather::ClimateIx(u32::try_from(i).expect("climate index fits u32")), q(c.wet), q(c.puddle)))
            .filter(|(_, w, p)| (*w, *p) != (0, 0))
            .map(|(c, w, p)| (book.climate_id(c).clone(), w, p))
            .collect();
        climates.sort_by(|a, b| a.0.cmp(&b.0));
        let mut cells: Vec<SavedCells> = self
            .cells
            .iter()
            .map(|(ws, m)| {
                let mut v: Vec<SavedCell> =
                    m.iter().map(|(c, s)| (c.x, c.y, q(s.snow), q(s.ash))).filter(|e| (e.2, e.3) != (0, 0)).collect();
                v.sort_unstable_by_key(|e| (e.0, e.1));
                (ws.clone(), v)
            })
            .filter(|(_, v)| !v.is_empty())
            .collect();
        cells.sort_by(|a, b| a.0.cmp(&b.0));
        SurfaceSave { t_h: self.t_h, climates, cells }
    }

    /// The state of a save; climates `book` no longer has (a removed mod) drop silently.
    pub fn from_save(save: &SurfaceSave, book: &WeatherBook) -> Self {
        let mut climates = vec![ClimateSurface::default(); book.climate_count()];
        for (id, w, p) in &save.climates {
            if let Some(c) = book.climate(id) {
                climates[c.0 as usize] = ClimateSurface { wet: dq(*w), puddle: dq(*p) };
            }
        }
        let cells: HashMap<Worldspace, HashMap<IVec2, CellSurface>> = save
            .cells
            .iter()
            .map(|(ws, v)| {
                let m: HashMap<IVec2, CellSurface> = v
                    .iter()
                    .filter(|e| (e.2, e.3) != (0, 0))
                    .map(|&(x, z, s, a)| (IVec2::new(x, z), CellSurface { snow: dq(s), ash: dq(a) }))
                    .collect();
                (ws.clone(), m)
            })
            .filter(|(_, m)| !m.is_empty())
            .collect();
        Self { t_h: save.t_h, climates, cells }
    }
}
