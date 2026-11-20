//! WG-X3 (`docs/specs/worldgen-adapter.md` "Same map at 10x", step 5): Morrowind's ground textures for a same-map
//! guide, as the framework's guide layers (`sf_worldgen::layers`, `layers.json` + `layers.u16`).
//!
//! **Source grid.** Every exterior `LAND`'s 16 x 16 `VTEX` texels through `mw_world::terrain::source_tile` (the
//! per-plugin `LTEX` resolution `layer_for` and the vanilla quarter-texel shift live there, not here), stitched on
//! one 7.3152 m lattice over the `LAND` box; cells without `LAND` name no layer (the default layer,
//! [`mw_world::terrain::DEFAULT_LAYER`]). Palette entries are the layer record ids the exterior bake hashes, so the
//! guide's layers index the bake's layer arrays.
//!
//! **Global grid** (outside settlements): the source grid scaled by S in origin and spacing (a texel becomes
//! S x 7.3 m), S = the guide's spacing over the source vertex spacing. **Stamp grids** (inside settlements): per
//! stamped anchor (`anchors.json` `stamp`), the source texels under its stamp's rectangle (plus one texel) at 1:1,
//! moved by the anchor's rigid offset `S·c − c` (`world_centre_m − source_centre_m`), so roads and plazas sit
//! exactly as in Morrowind under the moved statics. The runtime takes a stamp's grid where its weight is over one
//! half.
//!
//! Frames: source frame metres = engine X east, y = −Z north (`mw_world::convert`); the guide frame is the source
//! frame scaled by S (`worldgen-x10`).

// Grid maths: texel counts stay far below 2^31 and coordinates within a few hundred km, so index/float casts are
// exact enough by construction (as sf_worldgen).
#![allow(
    clippy::cast_precision_loss,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    clippy::cast_possible_wrap
)]

use std::collections::HashMap;
use std::fmt::Write as _;
use std::path::Path;
use std::time::Instant;

use mw_world::convert::UNIT_METERS_F64;
use mw_world::terrain::{DEFAULT_LAYER, LAND_TEXELS, TEXEL_UNITS, VERTEX_UNITS, source_tile};
use sf_records::RecordDb;
use sf_worldgen::anchors::AnchorTransform;
use sf_worldgen::filter::Declared;
use sf_worldgen::guide::Guide;
use sf_worldgen::layers::{GuideLayerGrid, GuideLayers, NO_LAYER};
use sf_worldgen::stamps::StampSet;
use tes3::esp::Landscape;

use crate::DumpError;

/// The declared layer values (spec `tunables.md` "Definition").
pub fn declared() -> Vec<Declared> {
    vec![Declared {
        name: "layers.blend_texels",
        v: 0.75,
        unit: "texels",
        min: 0.0,
        max: 4.0,
        source: "Industry(the exterior bake's own blend, mw_bake::world::terrain_config: the original engine's blend-map \
                 transition over half a texel)"
            .into(),
        help: "tent radius of the layer blend in texels of each grid (global: S x 7.3 m texels, stamps: 7.3 m)",
    }]
}

fn blend_texels() -> Result<f64, DumpError> {
    let d = declared();
    let p = &d[0];
    if p.v.is_finite() && p.v >= p.min && p.v <= p.max {
        Ok(p.v)
    } else {
        Err(DumpError::Invalid(format!("{} = {} outside [{}, {}]", p.name, p.v, p.min, p.max)))
    }
}

/// The exterior's texels on one lattice (module docs "Source grid"), engine frame, rows +Z (south).
pub struct SourceTexels {
    /// Engine metres (X, Z) of texel (0, 0)'s minimum (north-west) corner.
    pub min_xz: [f64; 2],
    pub dims: [usize; 2],
    /// Texel edge, metres.
    pub spacing_m: f64,
    pub palette: Vec<String>,
    /// Palette index per texel, [`NO_LAYER`] where no `LAND` is.
    pub texels: Vec<u16>,
    /// Per-texel sRGB RGB8 `VCLR` vertex-color tint (WG-X7c; `[255, 255, 255]` where a cell has no `VCLR`).
    pub tint: Vec<[u8; 3]>,
    /// `VTEX` values with no `LTEX` in their plugin (drawn with the default layer).
    pub unresolved: usize,
    pub cells: usize,
}

impl SourceTexels {
    /// Texel (col, row) of the lattice, or `NO_LAYER` outside.
    fn at(&self, col: i64, row: i64) -> u16 {
        let (w, h) = (self.dims[0] as i64, self.dims[1] as i64);
        if col < 0 || row < 0 || col >= w || row >= h {
            NO_LAYER
        } else {
            self.texels[(row * w + col) as usize]
        }
    }

    /// Tint at texel (col, row) of the lattice, or neutral white outside.
    fn tint_at(&self, col: i64, row: i64) -> [u8; 3] {
        let (w, h) = (self.dims[0] as i64, self.dims[1] as i64);
        if col < 0 || row < 0 || col >= w || row >= h {
            [255, 255, 255]
        } else {
            self.tint[(row * w + col) as usize]
        }
    }
}

/// Averages one cell's `VCLR` vertex grid (65×65) into 16×16 per-texel sRGB RGB8 tints.
fn cell_texel_tints(size: u32, rgb: &[[u8; 3]]) -> Vec<[u8; 3]> {
    let mut out = vec![[255u8; 3]; LAND_TEXELS * LAND_TEXELS];
    let n = size as usize;
    if n < 2 || rgb.len() != n * n {
        return out;
    }
    let step = (n - 1) / LAND_TEXELS;
    if step == 0 {
        return out;
    }
    for r in 0..LAND_TEXELS {
        for c in 0..LAND_TEXELS {
            let mut acc = [0u32; 3];
            let mut cnt = 0u32;
            for dr in 0..=step {
                let vr = (r * step + dr).min(n - 1);
                for dc in 0..=step {
                    let vc = (c * step + dc).min(n - 1);
                    let px = rgb[vr * n + vc];
                    for ch in 0..3 {
                        acc[ch] += u32::from(px[ch]);
                    }
                    cnt += 1;
                }
            }
            let d = cnt.max(1);
            out[r * LAND_TEXELS + c] = [(acc[0] / d) as u8, (acc[1] / d) as u8, (acc[2] / d) as u8];
        }
    }
    out
}

/// Stitches every exterior `LAND`'s layer texels and `VCLR` vertex tints.
pub fn source_texels(db: &RecordDb) -> Result<SourceTexels, DumpError> {
    let spacing_m = f64::from(TEXEL_UNITS) * UNIT_METERS_F64;
    let mut unresolved = 0;
    let mut cells = Vec::new();
    let mut lands: Vec<&Landscape> = db.iter::<Landscape>().map(|(_, l)| l).collect();
    lands.sort_by_key(|l| l.grid);
    for land in lands {
        let tile = source_tile(db, land, &mut unresolved);
        let Some(layers) = tile.layers else { continue };
        let o = layers.origin.to_f64();
        let ids: Vec<String> = layers.palette.iter().map(|r| r.as_str().to_owned()).collect();
        let tints = match &tile.tint {
            Some(t) => cell_texel_tints(t.size.x, &t.rgb),
            None => vec![[255u8; 3]; LAND_TEXELS * LAND_TEXELS],
        };
        cells.push(([o.x, o.z], ids, layers.texels, tints));
    }
    let first = cells.first().ok_or(DumpError::NoLand)?.0;
    let (mut lo, mut hi) = (first, first);
    for (o, _, _, _) in &cells {
        lo = [lo[0].min(o[0]), lo[1].min(o[1])];
        hi = [hi[0].max(o[0]), hi[1].max(o[1])];
    }
    let index = |v: f64| -> i64 { (v / spacing_m).round() as i64 };
    let dims = [(index(hi[0] - lo[0]) as usize) + LAND_TEXELS, (index(hi[1] - lo[1]) as usize) + LAND_TEXELS];
    let mut texels = vec![NO_LAYER; dims[0] * dims[1]];
    let mut tint = vec![[255u8; 3]; dims[0] * dims[1]];
    let mut palette: Vec<String> = Vec::new();
    let mut by_id: HashMap<String, u16> = HashMap::new();
    let n = cells.len();
    for (o, ids, cell, cell_tint) in cells {
        let (c0, r0) = (index(o[0] - lo[0]) as usize, index(o[1] - lo[1]) as usize);
        let global: Vec<u16> = ids
            .into_iter()
            .map(|id| {
                *by_id.entry(id.clone()).or_insert_with(|| {
                    palette.push(id);
                    u16::try_from(palette.len() - 1).expect("fewer than 65535 layers")
                })
            })
            .collect();
        for r in 0..LAND_TEXELS {
            for c in 0..LAND_TEXELS {
                let src_i = r * LAND_TEXELS + c;
                let dst_i = (r0 + r) * dims[0] + c0 + c;
                let t = cell[src_i];
                texels[dst_i] = global.get(usize::from(t)).copied().unwrap_or(NO_LAYER);
                tint[dst_i] = cell_tint[src_i];
            }
        }
    }
    Ok(SourceTexels { min_xz: lo, dims, spacing_m, palette, texels, tint, unresolved, cells: n })
}

/// The guide layers of a same-map guide: the global grid scaled by `scale`, one 1:1 grid per stamped anchor.
pub fn guide_layers(
    src: &SourceTexels,
    scale: f64,
    anchors: Option<&AnchorTransform>,
    stamps: Option<&StampSet>,
    blend_texels: f64,
) -> Result<(GuideLayers, usize), DumpError> {
    let t = src.spacing_m;
    let [x0, z0] = src.min_xz;
    let global = GuideLayerGrid {
        origin_m: [x0 * scale, -z0 * scale],
        dims: src.dims,
        spacing_m: t * scale,
        blend_m: blend_texels * t * scale,
        texels: src.texels.clone(),
        tint: Some(src.tint.clone()),
    };
    let mut grids: Vec<(usize, GuideLayerGrid)> = Vec::new();
    let mut skipped = 0;
    if let (Some(a), Some(set)) = (anchors, stamps) {
        for anchor in &a.anchors {
            let Some(k) = anchor.stamp else { continue };
            let Some(stamp) = set.stamps.get(k) else {
                skipped += 1;
                continue;
            };
            if grids.iter().any(|(j, _)| *j == k) {
                skipped += 1;
                continue;
            }
            // The rigid offset S·c − c, guide frame (x east, y north).
            let d = [anchor.world_centre_m[0] - anchor.source_centre_m[0], anchor.world_centre_m[1] - anchor.source_centre_m[1]];
            let [rx0, ry0, rx1, ry1] = stamp.rect_m();
            // Source engine X and Z of the rectangle, then texels (one more on every side for the blend).
            let (sx0, sx1) = (rx0 - d[0], rx1 - d[0]);
            let (sz0, sz1) = (-(ry1 - d[1]), -(ry0 - d[1]));
            let c0 = ((sx0 - x0) / t).floor() as i64 - 1;
            let c1 = ((sx1 - x0) / t).floor() as i64 + 1;
            let r0 = ((sz0 - z0) / t).floor() as i64 - 1;
            let r1 = ((sz1 - z0) / t).floor() as i64 + 1;
            let dims = [(c1 - c0 + 1) as usize, (r1 - r0 + 1) as usize];
            let mut texels = Vec::with_capacity(dims[0] * dims[1]);
            let mut tint = Vec::with_capacity(dims[0] * dims[1]);
            for r in r0..=r1 {
                for c in c0..=c1 {
                    texels.push(src.at(c, r));
                    tint.push(src.tint_at(c, r));
                }
            }
            grids.push((
                k,
                GuideLayerGrid {
                    origin_m: [x0 + c0 as f64 * t + d[0], -(z0 + r0 as f64 * t) + d[1]],
                    dims,
                    spacing_m: t,
                    blend_m: blend_texels * t,
                    texels,
                    tint: Some(tint),
                },
            ));
        }
    }
    grids.sort_by_key(|(k, _)| *k);
    let layers = GuideLayers { palette: src.palette.clone(), default_layer: DEFAULT_LAYER.to_owned(), global, stamps: grids };
    Ok((layers, skipped))
}

/// `bevyrrowind worldgen-layers --guide <dir>`: writes `layers.json` + `layers.u16` into a same-map guide and
/// returns the report (palette size, grid sizes, bytes, time).
pub fn run(db: &RecordDb, guide_dir: &Path) -> Result<String, DumpError> {
    let started = Instant::now();
    let inv = |e: sf_worldgen::Error| DumpError::Invalid(format!("{}: {e}", guide_dir.display()));
    let guide = Guide::read(guide_dir).map_err(inv)?;
    let scale = guide.spacing_m / (f64::from(VERTEX_UNITS) * UNIT_METERS_F64);
    let anchors = AnchorTransform::read(guide_dir).map_err(inv)?;
    let stamps = StampSet::read(guide_dir).map_err(inv)?;
    let src = source_texels(db)?;
    let read_s = started.elapsed().as_secs_f64();
    let (layers, skipped) = guide_layers(&src, scale, anchors.as_ref(), stamps.as_ref(), blend_texels()?)?;
    layers.write(guide_dir).map_err(inv)?;
    let mut out = String::new();
    let g = &layers.global;
    let _ = writeln!(out, "worldgen-layers: {}", guide_dir.display());
    let _ = writeln!(
        out,
        "  source: {} LAND cells, {} x {} texels of {:.4} m, {} unresolved VTEX (default layer)",
        src.cells, src.dims[0], src.dims[1], src.spacing_m, src.unresolved
    );
    let _ = writeln!(out, "  palette: {} layers (+ default {DEFAULT_LAYER})", layers.palette.len());
    let _ = writeln!(
        out,
        "  global grid: {} x {} texels of {:.3} m (scale {scale:.4}), origin ({:.1}, {:.1}) m, blend {:.2} m",
        g.dims[0], g.dims[1], g.spacing_m, g.origin_m[0], g.origin_m[1], g.blend_m
    );
    let stamp_texels: usize = layers.stamps.iter().map(|(_, s)| s.texels.len()).sum();
    let largest = layers.stamps.iter().map(|(_, s)| s.dims).max_by_key(|d| d[0] * d[1]);
    let _ = writeln!(
        out,
        "  stamp grids: {} of {} stamps ({} anchors), {stamp_texels} texels, largest {:?}; {skipped} skipped",
        layers.stamps.len(),
        stamps.as_ref().map_or(0, |s| s.stamps.len()),
        anchors.as_ref().map_or(0, |a| a.anchors.len()),
        largest
    );
    let bytes = std::fs::metadata(guide_dir.join("layers.u16")).map_or(0, |m| m.len())
        + std::fs::metadata(guide_dir.join("layers.json")).map_or(0, |m| m.len());
    let _ = writeln!(
        out,
        "  wrote layers.json + layers.u16: {bytes} bytes; {:.2} s (source {read_s:.2} s)",
        started.elapsed().as_secs_f64()
    );
    Ok(out)
}

#[cfg(test)]
#[allow(clippy::many_single_char_names)] // frame maths in the usual letters
mod tests {
    use super::*;
    use sf_worldgen::anchors::{AnchorPlace, Footprint};
    use sf_worldgen::stamps::Stamp;

    /// 32 x 32 source texels of 7.3152 m from engine (−117.0432, −117.0432), id by column band.
    fn src() -> SourceTexels {
        let t = f64::from(TEXEL_UNITS) * UNIT_METERS_F64;
        SourceTexels {
            min_xz: [-16.0 * t, -16.0 * t],
            dims: [32, 32],
            spacing_m: t,
            palette: vec!["a".into(), "b".into(), "c".into()],
            texels: (0..32 * 32).map(|k| ((k % 32) / 11) as u16).collect(),
            tint: vec![[255, 255, 255]; 32 * 32],
            unresolved: 0,
            cells: 4,
        }
    }

    #[test]
    fn global_scales_and_stamps_move_rigidly() {
        let s = src();
        let t = s.spacing_m;
        let scale = 10.0;
        // An anchor centred on source (20, −10) m; its stamp covers source x 0..40, y −30..10 at the world place.
        let (c, w) = ([20.0, -10.0], [200.0, -100.0]);
        let d = [w[0] - c[0], w[1] - c[1]];
        let stamp = Stamp {
            name: "town".into(),
            origin_m: [d[0], 10.0 + d[1]],
            dims: [21, 21],
            spacing_m: 2.0,
            heights: vec![0.0; 441],
            weights: vec![1.0; 441],
        };
        let anchor = AnchorPlace {
            name: "town".into(),
            stamp: Some(0),
            source_centre_m: c,
            world_centre_m: w,
            lift_m: 0.0,
            members: Vec::new(),
            footprint: Footprint::default(),
        };
        let a = AnchorTransform::new(scale, vec![anchor]);
        let set = StampSet { stamps: vec![stamp] };
        let (l, skipped) = guide_layers(&s, scale, Some(&a), Some(&set), 0.75).unwrap();
        assert_eq!(skipped, 0);
        assert_eq!(l.global.dims, [32, 32]);
        assert!((l.global.spacing_m - 10.0 * t).abs() < 1e-9 && (l.global.origin_m[0] + 160.0 * t).abs() < 1e-6);
        assert!((l.global.origin_m[1] - 160.0 * t).abs() < 1e-6, "north-west corner: y = −Z");
        let (k, g) = &l.stamps[0];
        assert_eq!(*k, 0);
        assert!((g.spacing_m - t).abs() < 1e-12);
        // Every stamp texel is the source texel at its world position minus the rigid offset.
        for r in 0..g.dims[1] {
            for col in 0..g.dims[0] {
                let x = g.origin_m[0] + (col as f64 + 0.5) * t - d[0];
                let y = g.origin_m[1] - (r as f64 + 0.5) * t - d[1];
                let (sc, sr) = (((x - s.min_xz[0]) / t).floor() as i64, ((-y - s.min_xz[1]) / t).floor() as i64);
                assert_eq!(g.texels[r * g.dims[0] + col], s.at(sc, sr), "texel ({col}, {r})");
            }
        }
        // It covers the stamp's rectangle.
        let [x0, y0, x1, y1] = set.stamps[0].rect_m();
        let gr = g.rect_m();
        assert!(gr[0] <= x0 && gr[1] <= y0 && gr[2] >= x1 && gr[3] >= y1, "{gr:?} vs {:?}", [x0, y0, x1, y1]);
        assert_eq!(l.default_layer, DEFAULT_LAYER);
    }
}
