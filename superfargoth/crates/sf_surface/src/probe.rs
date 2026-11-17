//! `surface probe`: a shelter depth map as a world-anchored text grid, the numeric twin of the
//! on-screen texture viewer. An instrument for comparing what two redraws of a level captured
//! over the same world area: cells sit on the *world* grid (multiples of the cell size), so the
//! same window prints the same characters whatever the map's centre, as long as the capture
//! itself is the same.
//!
//! One character per cell, from the highest caster among the map texels whose centres lie in the
//! cell (a one-texel trunk shows): `.` no caster (the maps clear to depth 0, reverse Z), `#` no
//! texel of the map in the cell (outside it), else the caster's top relative to the camera's
//! height: `-` below [`PROBE_FLOOR_M`], `0`..`9` = ⌊(h − floor) / [`PROBE_BUCKET_M`]⌋, clamped.
//! Rows are world Z increasing downward (each starts with its cell centre's Z), columns world X
//! increasing to the right.
//!
//! The map geometry is `view::ShelterLevel`'s: looking straight down, view X = world X, texture
//! row increasing with world Z, depth 1 at the top plane (`up_m` above the centre) and 0 at the
//! bottom, so a caster's height over the centre is `depth × depth_m − down_m`.

use glam::{DVec2, DVec3, Mat4};

use crate::view::{SHELTER_SIZE, ShelterLevel};

/// Cell edge of the near level's grids, meters (4 of its 0.125 m texels).
// tunables: const (a print format: the instrument's resolution, no cost or look)
pub const PROBE_NEAR_CELL_M: f32 = 0.5;
/// Cell edge of the far level's grids, meters (its 1 m texel).
// tunables: const (a print format: the instrument's resolution, no cost or look)
pub const PROBE_FAR_CELL_M: f32 = 1.0;
/// Height of one digit, meters.
// tunables: const (a print format: one character's height bucket)
pub const PROBE_BUCKET_M: f32 = 2.0;
/// The bottom of digit `0` relative to the camera's height, meters; lower tops print `-`.
// tunables: const (a print format: where the digits start)
pub const PROBE_FLOOR_M: f32 = -1.0;
/// The window's half extent when the command names none, meters.
// tunables: const (a print format: the default window of a dev command)
pub const PROBE_DEFAULT_HALF_M: f32 = 16.0;
/// The widest window half, meters (the near level's whole map: 256 lines of 256 cells).
// tunables: const (a print format: the largest grid a dev command prints)
pub const PROBE_MAX_HALF_M: f32 = 64.0;

/// The line that explains the characters (printed once per probe).
pub const PROBE_LEGEND: &str = "legend: rows = world Z increasing downward (the label is the row's cell-centre Z), \
     columns = world X increasing rightward; '.' no caster, '#' outside the map, '-' top more than 1 m below the camera, \
     0-9 = floor((top - camera Y + 1) / 2): 0 is -1..1 m, 1 is 1..3 m, 9 is >= 17 m";

/// One captured map: which level drew it, where its centre stood and its depth texels.
#[derive(Clone, Copy, Debug)]
pub struct ProbeMap<'a> {
    /// What the header calls it (`near solid`, `far leaves`, ...).
    pub name: &'a str,
    pub level: ShelterLevel,
    /// The map's centre, world meters ([`map_center`]).
    pub center: DVec3,
    /// [`SHELTER_SIZE`]² depths, row-major, row 0 at the lowest world Z.
    pub depth: &'a [f32],
}

/// The world square to print.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ProbeWindow {
    /// Centre, world X and Z.
    pub center: DVec2,
    /// Half extent, meters.
    pub half_m: f32,
    /// Cell edge, meters ([`PROBE_NEAR_CELL_M`], [`PROBE_FAR_CELL_M`]).
    pub cell_m: f32,
}

/// The world centre of the map a level drew with `clip` (its `clip_from_world`, render space;
/// world = `origin` + render): X and Z where the clip's xy is 0, snapped to the level's world
/// texel grid as `ShelterLevel::center_world` placed them (the clip holds them as `f32`); Y where
/// the clip's depth is `down_m / depth_m`.
pub fn map_center(level: &ShelterLevel, clip: Mat4, origin: DVec3) -> DVec3 {
    let x = -clip.w_axis.x / clip.x_axis.x;
    let z = -clip.w_axis.y / clip.z_axis.y;
    let y = (level.down_m / level.depth_m() - clip.w_axis.z) / clip.y_axis.z;
    let texel = f64::from(level.texel_m());
    let snap = |w: f64| (w / texel).round() * texel;
    DVec3::new(snap(origin.x + f64::from(x)), origin.y + f64::from(y), snap(origin.z + f64::from(z)))
}

/// The character of a caster whose top is `h` meters above the camera's height.
#[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
pub fn height_char(h: f32) -> char {
    if h < PROBE_FLOOR_M {
        return '-';
    }
    let bucket = ((h - PROBE_FLOOR_M) / PROBE_BUCKET_M).floor().clamp(0.0, 9.0);
    char::from(b'0' + bucket as u8)
}

/// The map texels (along one axis) whose centres lie in world `[from, to)`, for a map starting
/// at world `map_min`: an empty range when none does.
#[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
fn texel_span(from: f64, to: f64, map_min: f64, texel: f64) -> std::ops::Range<usize> {
    let size = f64::from(SHELTER_SIZE);
    // Texel `k`'s centre is at `map_min + (k + 0.5) × texel`.
    let first = |w: f64| ((w - map_min) / texel - 0.5).ceil().clamp(0.0, size) as usize;
    first(from)..first(to)
}

/// `map` over `window` as text: a header line, then one line per row of cells (see the module
/// docs). `camera` is the camera's world position: heights print relative to its Y. The header
/// gives the map's name, the first cell's centre (world X, Z), the cell size, the grid size, the
/// map's centre, the camera and how many texels of the whole map hold a caster.
#[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss, clippy::cast_precision_loss)]
pub fn probe_grid(map: &ProbeMap, camera: DVec3, window: &ProbeWindow) -> Vec<String> {
    let size = SHELTER_SIZE as usize;
    if map.depth.len() != size * size {
        return vec![format!("{}: {} depth texels, expected {}", map.name, map.depth.len(), size * size)];
    }
    let level = &map.level;
    let cell = f64::from(window.cell_m);
    let texel = f64::from(level.texel_m());
    let half = f64::from(window.half_m.clamp(window.cell_m, PROBE_MAX_HALF_M));
    // Cells on the world grid: the window's low corner snapped down to a whole cell.
    let first_x = ((window.center.x - half) / cell).floor();
    let first_z = ((window.center.y - half) / cell).floor();
    let n = (2.0 * half / cell).ceil().max(1.0) as usize;
    let map_min = DVec2::new(map.center.x, map.center.z) - DVec2::splat(f64::from(level.half_m));
    let columns: Vec<_> =
        (0..n).map(|i| texel_span((first_x + i as f64) * cell, (first_x + i as f64 + 1.0) * cell, map_min.x, texel)).collect();
    // A depth's height over the camera: `depth × depth_m − down_m` over the centre.
    let base = (map.center.y - camera.y) as f32 - level.down_m;
    let casters = map.depth.iter().filter(|d| **d > 0.0).count();
    let mut lines = Vec::with_capacity(n + 1);
    lines.push(format!(
        "{}: first cell x {:.2} z {:.2}, cell {} m, {n}x{n} cells, map centre x {:.3} z {:.3} y {:.3}, \
         camera x {:.2} y {:.2} z {:.2}, {casters} caster texels of {}",
        map.name,
        (first_x + 0.5) * cell,
        (first_z + 0.5) * cell,
        window.cell_m,
        map.center.x,
        map.center.z,
        map.center.y,
        camera.x,
        camera.y,
        camera.z,
        size * size,
    ));
    for k in 0..n {
        let z0 = (first_z + k as f64) * cell;
        let rows = texel_span(z0, z0 + cell, map_min.y, texel);
        let mut line = format!("{:>10.2} ", z0 + 0.5 * cell);
        for cols in &columns {
            if rows.is_empty() || cols.is_empty() {
                line.push('#');
                continue;
            }
            let mut top = 0.0_f32;
            for r in rows.clone() {
                for d in &map.depth[r * size + cols.start..r * size + cols.end] {
                    top = top.max(*d);
                }
            }
            line.push(if top > 0.0 { height_char(base + top * level.depth_m()) } else { '.' });
        }
        lines.push(line);
    }
    lines
}
