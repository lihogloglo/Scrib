//! Exterior terrain: `LAND` records as an `sf_terrain::TerrainSource`, and
//! `LTEX` records as terrain layers.
//!
//! Spec: `docs/specs/world-adapter.md` ("Terrain"). The format facts come
//! from `OpenMW` (`components/esm3/loadland.cpp`, `components/esmterrain`,
//! `components/terrain`), reused here under the GPL:
//!
//! - Heights: 65×65 vertices per 8192-unit cell (128 units apart), rows
//!   south to north. `VHGT` is a float offset plus int8 deltas: each row
//!   starts from the previous row's first vertex, each vertex from its west
//!   neighbor, and the sum is scaled by 8 units. A `LAND` without heights is
//!   flat at -2048 units (`OpenMW`'s `DEFAULT_HEIGHT`).
//! - Triangulation: a checkerboard. Quad `(i, j)` (vertex indices, Morrowind
//!   axes) is split along `(i, j)–(i+1, j+1)` when `i + j` is even, along
//!   the other diagonal when odd (`OpenMW`'s "diamond" index buffer, which its
//!   Bullet heightfields mirror with diamond subdivision).
//! - Vertex colors (`VCLR`): RGB per vertex, same layout as heights.
//! - Textures (`VTEX`): 16×16 indices, stored as 4×4 blocks of 4×4 texels
//!   (transposed on load). Value 0 is the default texture; `n` is the LTEX
//!   with index `n - 1` *in the file that defined this LAND*. The grid is
//!   not aligned with the cell: `OpenMW` reads the texel one to the west for
//!   its blend map and nudges the blend map by a quarter texel to "look like
//!   vanilla", which puts texel `(sx, sy)` over
//!   `x ∈ [sx + 1/4, sx + 5/4)`, `y ∈ [sy − 1/4, sy + 3/4)` in texel units
//!   (512 units) from the cell's south-west corner: shifted one vertex east
//!   and one vertex south.

use std::collections::HashMap;
use std::ops::Deref;

use glam::{DVec3, UVec2};
use mw_records::{LandTextureSlot, keys};
use sf_coords::WorldPos;
use sf_records::{RecordDb, RecordId};
use sf_terrain::{
    Diagonal, SourceLayers, SourceTile, SourceTileId, SourceTileInfo, SourceTint, TerrainSource, Triangulation,
};
use sf_vfs::{Vfs, VfsPath};
use sf_world::{Fingerprint, RecordRef, WorldspaceId};
use tes3::esp::{Landscape, LandscapeFlags, LandscapeTexture};

use crate::convert::{CELL_UNITS, UNIT_METERS_F64, grid_corner};
use crate::ids;

/// Vertices along a cell edge.
pub const LAND_VERTS: usize = 65;
/// Texture texels along a cell edge.
pub const LAND_TEXELS: usize = 16;
/// Units between neighboring vertices.
pub const VERTEX_UNITS: f32 = 128.0;
/// Units per texture texel (4 vertex spacings).
pub const TEXEL_UNITS: f32 = 512.0;
/// `VHGT` delta scale, units per step.
pub const HEIGHT_SCALE: f32 = 8.0;
/// Height of a `LAND` without heights, and of the sea floor where no `LAND`
/// exists (`OpenMW`'s `ESM::Land::DEFAULT_HEIGHT`), in units.
pub const DEFAULT_HEIGHT_UNITS: f32 = -2048.0;

/// The surface Morrowind's authors placed objects on, in engine tile terms.
///
/// Engine tiles run rows +Z (south), so a Morrowind quad `(i, j)` becomes
/// engine quad `(i, 63 − j)`: the parity flips and each diagonal mirrors
/// into the other. Net effect: even engine quads use the main diagonal.
pub const TRIANGULATION: Triangulation = Triangulation::Alternating(Diagonal::Main);

/// [`DEFAULT_HEIGHT_UNITS`] in meters: the bake's fill height.
#[allow(clippy::cast_possible_truncation)]
pub fn fill_height() -> f32 {
    (f64::from(DEFAULT_HEIGHT_UNITS) * UNIT_METERS_F64) as f32
}

/// The terrain layer used by texture index 0.
pub const DEFAULT_LAYER: &str = "@default";
/// The built-in default land texture.
pub const DEFAULT_TEXTURE: &str = "textures/_land_default.tga";

/// One cell's decoded `LAND`, in Morrowind units and axes.
#[derive(Clone, Debug, PartialEq)]
pub struct LandData {
    pub grid: (i32, i32),
    /// Heights in units, `[y * 65 + x]`, rows south to north.
    pub heights: Vec<f32>,
    /// Vertex colors, same layout. `None` without `VCLR`.
    pub colors: Option<Vec<[u8; 3]>>,
    /// Texture indices `[y * 16 + x]` (already transposed), rows south to
    /// north. `None` without `VTEX` (all default).
    pub textures: Option<[u16; LAND_TEXELS * LAND_TEXELS]>,
}

impl LandData {
    /// Decodes a `LAND` record. Data whose flag is off is absent, as in the
    /// original engine.
    pub fn decode(land: &Landscape) -> Self {
        let flags = land.landscape_flags;
        let heights = if flags.contains(LandscapeFlags::USES_VERTEX_HEIGHTS_AND_NORMALS) {
            decode_heights(land.vertex_heights.offset, &land.vertex_heights.data)
        } else {
            vec![DEFAULT_HEIGHT_UNITS; LAND_VERTS * LAND_VERTS]
        };
        let colors = flags
            .contains(LandscapeFlags::USES_VERTEX_COLORS)
            .then(|| land.vertex_colors.data.iter().flatten().copied().collect());
        let textures = flags.contains(LandscapeFlags::USES_TEXTURES).then(|| transpose_textures(&land.texture_indices.data));
        Self { grid: land.grid, heights, colors, textures }
    }

    /// Height in units at cell-local coordinates (units from the south-west
    /// corner, clamped to the cell), on the source triangulation.
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss, clippy::cast_precision_loss)]
    pub fn height_at(&self, x: f32, y: f32) -> f32 {
        let last = (LAND_VERTS - 1) as f32;
        let col = (x / VERTEX_UNITS).clamp(0.0, last);
        let row = (y / VERTEX_UNITS).clamp(0.0, last);
        let qx = (col.floor() as usize).min(LAND_VERTS - 2);
        let qy = (row.floor() as usize).min(LAND_VERTS - 2);
        let (fx, fy) = (col - qx as f32, row - qy as f32);
        let at = |qx: usize, qy: usize| self.heights[qy * LAND_VERTS + qx];
        let (h00, h10, h01, h11) = (at(qx, qy), at(qx + 1, qy), at(qx, qy + 1), at(qx + 1, qy + 1));
        if (qx + qy).is_multiple_of(2) {
            // Main diagonal (i, j)–(i+1, j+1).
            if fx >= fy { h00 + (h10 - h00) * fx + (h11 - h10) * fy } else { h00 + (h01 - h00) * fy + (h11 - h01) * fx }
        } else if fx + fy <= 1.0 {
            h00 + (h10 - h00) * fx + (h01 - h00) * fy
        } else {
            h11 + (h01 - h11) * (1.0 - fx) + (h10 - h11) * (1.0 - fy)
        }
    }
}

/// `VHGT` deltas to absolute heights in units (`OpenMW`'s accumulation order).
pub fn decode_heights(offset: f32, deltas: &[[i8; LAND_VERTS]; LAND_VERTS]) -> Vec<f32> {
    let mut out = vec![0.0; LAND_VERTS * LAND_VERTS];
    let mut row = offset;
    for (y, deltas) in deltas.iter().enumerate() {
        row += f32::from(deltas[0]);
        out[y * LAND_VERTS] = row * HEIGHT_SCALE;
        let mut col = row;
        for x in 1..LAND_VERTS {
            col += f32::from(deltas[x]);
            out[y * LAND_VERTS + x] = col * HEIGHT_SCALE;
        }
    }
    out
}

/// `VTEX` as stored (16 blocks of 4×4, block-major) to `[y * 16 + x]`.
pub fn transpose_textures(raw: &[[u16; LAND_TEXELS]; LAND_TEXELS]) -> [u16; LAND_TEXELS * LAND_TEXELS] {
    let mut out = [0; LAND_TEXELS * LAND_TEXELS];
    let mut read = raw.iter().flatten();
    for y1 in 0..4 {
        for x1 in 0..4 {
            for y2 in 0..4 {
                for x2 in 0..4 {
                    out[(y1 * 4 + y2) * LAND_TEXELS + x1 * 4 + x2] = *read.next().expect("256 texels");
                }
            }
        }
    }
    out
}

/// The `TerrainLayer` record id a `VTEX` value names in `file`'s `LAND`
/// records: the `LTEX` editor id, [`DEFAULT_LAYER`] for 0, `None` when the
/// file has no such index or the LTEX no longer exists.
pub fn layer_for(db: &RecordDb, file: &str, vtex: u16) -> Option<RecordId> {
    if vtex == 0 {
        return Some(RecordId::new(DEFAULT_LAYER));
    }
    let slot = db.get::<LandTextureSlot>(&keys::ltex_slot(file, u32::from(vtex) - 1))?;
    db.get::<LandscapeTexture>(&slot.texture).map(|_| slot.texture.clone())
}

/// A terrain layer as `terrain.md` defines `TerrainLayer`.
///
/// Scaffold (`TerrainLayerDef`): `sf_terrain::bake::TerrainLayer` (record
/// kind `terrain.layer`) is still being written behind `sf_terrain`'s `bake`
/// feature, so the adapter keeps its own copy; the bake host converts field
/// by field (paths as strings, `surface.id` as the surface record id).
/// Removal path: once that type settles, `terrain_layers` returns it.
#[derive(Clone, Debug, PartialEq)]
pub struct TerrainLayerDef {
    pub albedo_height: VfsPath,
    pub normal: Option<VfsPath>,
    pub arm: Option<VfsPath>,
    /// Meters per texture repeat. Morrowind repeats a land texture once per
    /// texel (512 units ≈ 7.3 m).
    pub tiling_m: f32,
    pub triplanar: bool,
    pub displacement_m: f32,
    pub surface: RecordRef,
}

/// Record kind of the surface types `TerrainLayerDef::surface` names.
pub const SURFACE_KIND: &str = "sf.surface";

/// Default surface classification of land textures by name. Morrowind has
/// no surface data, so this is an adapter rule: the first keyword found in
/// the LTEX id or texture file name wins; mods patch the layer record.
pub const SURFACE_RULES: &[(&str, &str)] = &[
    ("snow", "snow"),
    ("ice", "ice"),
    ("lava", "rock"),
    ("ash", "ash"),
    ("sand", "sand"),
    ("beach", "sand"),
    ("mud", "mud"),
    ("swamp", "mud"),
    ("cobble", "stone"),
    ("road", "stone"),
    ("tile", "stone"),
    ("grass", "grass"),
    ("moss", "grass"),
    ("rock", "rock"),
    ("stone", "rock"),
    ("cliff", "rock"),
    ("dirt", "dirt"),
];

/// Surface of a land texture by [`SURFACE_RULES`] (`dirt` when nothing
/// matches).
pub fn surface_for(id: &str, file_name: &str) -> RecordRef {
    let haystack = format!("{} {}", id.to_ascii_lowercase(), file_name.to_ascii_lowercase());
    let surface = SURFACE_RULES.iter().find(|(key, _)| haystack.contains(key)).map_or("dirt", |(_, s)| s);
    RecordRef::new(SURFACE_KIND, RecordId::new(surface))
}

/// Every live `LTEX` as a terrain layer, plus [`DEFAULT_LAYER`], sorted by
/// id. With a VFS, texture paths resolve to the file actually present
/// (`.dds` replaces `.tga`/`.bmp` as in the original engine).
pub fn terrain_layers(db: &RecordDb, vfs: Option<&Vfs>) -> Vec<(RecordId, TerrainLayerDef)> {
    let tiling_m = meters_f32(TEXEL_UNITS);
    let layer = |file: &str, surface: RecordRef| {
        let path = VfsPath::new(&mw_nif::texture_path(file)).ok()?;
        let path = match vfs {
            Some(vfs) => vfs.resolve_preferring(&path, &["dds"]).map_or(path, |(p, _)| p),
            None => path,
        };
        Some(TerrainLayerDef {
            albedo_height: path,
            normal: None,
            arm: None,
            tiling_m,
            triplanar: false,
            displacement_m: 0.0,
            surface,
        })
    };
    let mut out: Vec<(RecordId, TerrainLayerDef)> = db
        .iter::<LandscapeTexture>()
        .filter_map(|(id, ltex)| Some((id.clone(), layer(&ltex.file_name, surface_for(&ltex.id, &ltex.file_name))?)))
        .collect();
    if let Some(default) = layer(DEFAULT_TEXTURE, RecordRef::new(SURFACE_KIND, "dirt")) {
        out.push((RecordId::new(DEFAULT_LAYER), default));
    }
    out.sort_by(|a, b| a.0.cmp(&b.0));
    out
}

#[allow(clippy::cast_possible_truncation)]
fn meters_f32(units: f32) -> f32 {
    (f64::from(units) * UNIT_METERS_F64) as f32
}

/// The source tile id of exterior cell `(x, y)`.
#[allow(clippy::cast_sign_loss)]
pub fn tile_id((x, y): (i32, i32)) -> SourceTileId {
    SourceTileId((u64::from(x as u32) << 32) | u64::from(y as u32))
}

/// Inverse of [`tile_id`].
#[allow(clippy::cast_possible_truncation, clippy::cast_possible_wrap)]
pub fn tile_grid(id: SourceTileId) -> (i32, i32) {
    ((id.0 >> 32) as u32 as i32, id.0 as u32 as i32)
}

/// Counters from building a [`MwTerrainSource`].
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct TerrainStats {
    pub tiles: usize,
    pub without_heights: usize,
    pub without_colors: usize,
    pub without_textures: usize,
    /// Texture indices with no `LTEX` in the defining file (drawn with the
    /// default layer).
    pub unresolved_textures: usize,
    /// Distinct layers used.
    pub layers_used: usize,
}

/// One `LAND` as a source tile, in engine axes.
///
/// Heights and tint: origin at the cell's north-west vertex (rows step +Z,
/// south), 1.8288 m spacing, 65² samples. Layers: 16² texels of 7.3152 m
/// with the vanilla offset (module docs).
pub fn source_tile(db: &RecordDb, land: &Landscape, unresolved: &mut usize) -> SourceTile {
    let data = LandData::decode(land);
    let file = db
        .entry::<Landscape>(&keys::landscape(land))
        .and_then(sf_records::Entry::defined_by)
        .map(|s| db.source(s).name.clone())
        .unwrap_or_default();
    source_tile_from(&data, &mut |vtex| {
        layer_for(db, &file, vtex).unwrap_or_else(|| {
            *unresolved += 1;
            RecordId::new(DEFAULT_LAYER)
        })
    })
}

/// [`source_tile`] from decoded data and a `VTEX` → layer id resolver.
pub fn source_tile_from(data: &LandData, layer: &mut dyn FnMut(u16) -> RecordId) -> SourceTile {
    let (gx, gy) = data.grid;
    let n = LAND_VERTS;
    // Engine sample k (rows north to south) reads Morrowind vertex
    // (k % n, n − 1 − k / n) (rows south to north).
    let source = |k: usize| (n - 1 - k / n) * n + k % n;
    let heights = (0..n * n).map(|k| meters_f32(data.heights[source(k)])).collect();
    let corner = grid_corner(gx, gy);
    // Sample (0, 0) is the north-west vertex.
    let origin = WorldPos::from_f64(corner - DVec3::new(0.0, 0.0, CELL_UNITS * UNIT_METERS_F64));
    let spacing = meters_f32(VERTEX_UNITS);
    let size = UVec2::splat(65);

    let tint = data.colors.as_ref().map(|colors| SourceTint {
        origin,
        spacing,
        size,
        rgb: (0..n * n).map(|k| colors[source(k)]).collect(),
    });

    let textures = data.textures.unwrap_or([0; LAND_TEXELS * LAND_TEXELS]);
    let mut palette: Vec<RecordId> = Vec::new();
    let mut by_value: HashMap<u16, u16> = HashMap::new();
    let mut texels = Vec::with_capacity(LAND_TEXELS * LAND_TEXELS);
    for r in 0..LAND_TEXELS {
        for c in 0..LAND_TEXELS {
            let vtex = textures[(LAND_TEXELS - 1 - r) * LAND_TEXELS + c];
            let slot = *by_value.entry(vtex).or_insert_with(|| {
                let id = layer(vtex);
                let index = palette.iter().position(|p| *p == id).unwrap_or_else(|| {
                    palette.push(id);
                    palette.len() - 1
                });
                u16::try_from(index).expect("at most 256 layers per cell")
            });
            texels.push(slot);
        }
    }
    // Min corner of texel (0, 0) (north-west): x shifted one vertex east,
    // the grid's north edge one vertex south of the cell's.
    let texel = f64::from(TEXEL_UNITS);
    let layer_min = corner
        + DVec3::new(f64::from(VERTEX_UNITS), 0.0, -(16.0 * texel - f64::from(VERTEX_UNITS)))
            * UNIT_METERS_F64;
    let layers = SourceLayers {
        origin: WorldPos::from_f64(layer_min),
        spacing: meters_f32(TEXEL_UNITS),
        size: UVec2::splat(16),
        palette,
        texels,
    };
    SourceTile { origin, spacing, size, heights, surface: TRIANGULATION, layers: Some(layers), tint }
}

/// Content fingerprint of a source tile: its heights, layers, and tint.
pub fn tile_fingerprint(tile: &SourceTile) -> Fingerprint {
    let mut bytes = Vec::with_capacity(tile.heights.len() * 7 + 1024);
    bytes.extend_from_slice(&tile.origin.to_bytes());
    for h in &tile.heights {
        bytes.extend_from_slice(&h.to_le_bytes());
    }
    if let Some(layers) = &tile.layers {
        bytes.extend_from_slice(&layers.origin.to_bytes());
        for id in &layers.palette {
            bytes.extend_from_slice(id.as_str().as_bytes());
            bytes.push(0);
        }
        for t in &layers.texels {
            bytes.extend_from_slice(&t.to_le_bytes());
        }
    }
    if let Some(tint) = &tile.tint {
        bytes.push(1);
        bytes.extend(tint.rgb.iter().flatten());
    }
    Fingerprint::of(&bytes)
}

/// [`TerrainSource`] over the loaded `LAND` records of the exterior. Tile
/// infos and fingerprints are computed once; `read` decodes on demand.
pub struct MwTerrainSource<D> {
    db: D,
    tiles: Vec<SourceTileInfo>,
    stats: TerrainStats,
}

impl<D: Deref<Target = RecordDb>> MwTerrainSource<D> {
    pub fn new(db: D) -> Self {
        let mut stats = TerrainStats::default();
        let mut tiles = Vec::new();
        let mut used = std::collections::BTreeSet::new();
        for (_, land) in db.iter::<Landscape>() {
            let flags = land.landscape_flags;
            stats.without_heights += usize::from(!flags.contains(LandscapeFlags::USES_VERTEX_HEIGHTS_AND_NORMALS));
            stats.without_colors += usize::from(!flags.contains(LandscapeFlags::USES_VERTEX_COLORS));
            stats.without_textures += usize::from(!flags.contains(LandscapeFlags::USES_TEXTURES));
            let tile = source_tile(&db, land, &mut stats.unresolved_textures);
            if let Some(l) = &tile.layers {
                used.extend(l.palette.iter().cloned());
            }
            let (gx, gy) = land.grid;
            let min = grid_corner(gx, gy) - DVec3::new(0.0, 0.0, CELL_UNITS * UNIT_METERS_F64);
            let max = grid_corner(gx + 1, gy);
            tiles.push(SourceTileInfo {
                id: tile_id(land.grid),
                fingerprint: tile_fingerprint(&tile),
                min: WorldPos::from_f64(min),
                max: WorldPos::from_f64(max),
            });
        }
        tiles.sort_by_key(|t| t.id);
        stats.tiles = tiles.len();
        stats.layers_used = used.len();
        Self { db, tiles, stats }
    }

    pub fn stats(&self) -> &TerrainStats {
        &self.stats
    }

    /// Tile infos, sorted by id.
    pub fn infos(&self) -> &[SourceTileInfo] {
        &self.tiles
    }

    /// The decoded `LAND` of exterior cell `grid`.
    pub fn land(&self, grid: (i32, i32)) -> Option<LandData> {
        self.db.get::<Landscape>(&keys::exterior(grid)).map(LandData::decode)
    }
}

impl<D: Deref<Target = RecordDb> + Send + Sync> TerrainSource for MwTerrainSource<D> {
    fn tiles(&self, ws: &WorldspaceId, out: &mut dyn FnMut(SourceTileInfo)) {
        if ws.as_str() == ids::EXTERIOR {
            self.tiles.iter().copied().for_each(out);
        }
    }

    fn read(&self, ws: &WorldspaceId, tile: SourceTileId) -> Option<SourceTile> {
        if ws.as_str() != ids::EXTERIOR {
            return None;
        }
        let land = self.db.get::<Landscape>(&keys::exterior(tile_grid(tile)))?;
        Some(source_tile(&self.db, land, &mut 0))
    }
}

/// Terrain heights in Morrowind units by world position, for grounding
/// tests. Caches decoded cells.
pub struct HeightSampler<'a> {
    db: &'a RecordDb,
    cells: HashMap<(i32, i32), Option<LandData>>,
}

impl<'a> HeightSampler<'a> {
    pub fn new(db: &'a RecordDb) -> Self {
        Self { db, cells: HashMap::new() }
    }

    /// Terrain height (units) under a Morrowind position, `None` where the
    /// cell has no `LAND`.
    #[allow(clippy::cast_possible_truncation)]
    pub fn height(&mut self, x: f32, y: f32) -> Option<f32> {
        let grid = crate::convert::grid_of([x, y, 0.0]);
        let db = self.db;
        let land = self
            .cells
            .entry(grid)
            .or_insert_with(|| db.get::<Landscape>(&keys::exterior(grid)).map(LandData::decode))
            .as_ref()?;
        let local = |v: f32, g: i32| (f64::from(v) - f64::from(g) * CELL_UNITS) as f32;
        Some(land.height_at(local(x, grid.0), local(y, grid.1)))
    }
}
