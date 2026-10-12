//! Record identity rules for TES3 kinds whose editor ID is not stable.

use sf_records::RecordId;
use tes3::esp::{Cell, Dialogue, DialogueInfo, Landscape, PathGrid};

/// Exterior cells are identified by grid alone: their display name is just
/// the region (or a settlement name) and plugins may change it. Interior
/// cells are identified by name. `@` cannot start an interior name in
/// practice, so the namespaces cannot collide.
pub fn cell(cell: &Cell) -> RecordId {
    if cell.is_interior() { RecordId::new(&cell.name) } else { exterior(cell.data.grid) }
}

pub fn exterior((x, y): (i32, i32)) -> RecordId {
    RecordId::from(format!("@{x},{y}"))
}

pub fn landscape(land: &Landscape) -> RecordId {
    exterior(land.grid)
}

/// A content file's land texture index (see `LandTextureSlot`):
/// `"<file name>#<INTV>"`. File names compare case-insensitively.
pub fn ltex_slot(file: &str, index: u32) -> RecordId {
    RecordId::from(format!("{file}#{index}"))
}

/// Path grids name their cell and give a grid; the pair is stable.
pub fn path_grid(grid: &PathGrid) -> RecordId {
    let (x, y) = grid.data.grid;
    RecordId::from(format!("{}@{x},{y}", grid.cell))
}

/// Dialogue responses only make sense under their topic, so the topic is
/// part of the key: `"<topic>\u{1f}<info id>"` (unit separator, which no
/// editor ID contains).
pub fn info(topic: &Dialogue, info: &DialogueInfo) -> RecordId {
    RecordId::from(format!("{}\u{1f}{}", topic.id, info.id))
}

/// Splits an info key back into (topic, info id).
pub fn split_info(id: &RecordId) -> Option<(&str, &str)> {
    id.as_str().split_once('\u{1f}')
}
