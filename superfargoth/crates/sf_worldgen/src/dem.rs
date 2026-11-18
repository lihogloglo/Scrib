//! Real elevation data as a guide: `GeoTIFF` tiles in geographic coordinates (EPSG:4326) reprojected to a local
//! metric grid and written in the guide v1 format ([`crate::guide`]), for reality checks of generated terrain
//! against real places at the same resolution (worldgen WG-RC).
//!
//! - Reading: single-band `GeoTIFF` (`tiff` crate; DEFLATE or LZW, horizontal or floating-point predictor, f32 /
//!   f64 / i16 / u16 / i32 samples), georeferenced by `ModelTiepointTag` + `ModelPixelScaleTag`, honouring
//!   `GTRasterTypeGeoKey` (pixel-is-area, the default, or pixel-is-point) and `GDAL_NODATA`. Projected models and
//!   geographic systems other than WGS 84 are rejected rather than misread.
//! - Mosaic: tiles of one resolution placed on a common lattice; anywhere no tile covers is no data. (Copernicus
//!   GLO-30 on the AWS registry has no tiles over open ocean; its readme says to read them as height 0.)
//! - Projection: local equirectangular around the requested centre with the WGS 84 meridional and prime-vertical
//!   radii at the centre latitude, so x = east metres, y = north metres. Scale error grows with distance from the
//!   centre latitude as `tan(lat0) · Δlat` (about 1 % at 25 km from 64° N), fine for look tests of a few tens of km.
//! - Resampling: Catmull-Rom bicubic ([`crate::resample::bicubic_with`]) in source pixel space; wet source pixels
//!   (no data, or at or below `sea_max_m`) count as sea level 0 in the kernel. The wet/dry class of an output
//!   sample is its nearest source pixel's, so the coastline is never smeared into new shallows; wet samples get
//!   `fill_m`, dry ones are clamped to >= 0 so the kernel's overshoot near the coast cannot turn land into sea.
//!
//! Orientation is the guide's: row 0 = north edge, column 0 = west edge; `origin_m` puts the requested centre at
//! local (0, 0).

use std::fs::File;
use std::io::{BufReader, Read, Seek};
use std::path::{Path, PathBuf};

use rayon::prelude::*;
use tiff::decoder::{Decoder, DecodingResult, Limits};
use tiff::tags::Tag;

use crate::resample::bicubic_with;
use crate::{Error, Result};

/// WGS 84 semi-major axis, metres.
const WGS84_A: f64 = 6_378_137.0;
/// WGS 84 first eccentricity squared.
const WGS84_E2: f64 = 0.006_694_379_990_14;
/// `GeoTIFF` key ids (`GeoTIFF` 1.1, OGC 19-008r4 §7).
const KEY_MODEL_TYPE: u16 = 1024;
const KEY_RASTER_TYPE: u16 = 1025;
const KEY_GEOGRAPHIC_TYPE: u16 = 2048;
/// `GTModelTypeGeoKey` value: geographic latitude-longitude.
const MODEL_GEOGRAPHIC: u16 = 2;
/// `GTRasterTypeGeoKey` value: pixel-is-point.
const RASTER_PIXEL_IS_POINT: u16 = 2;
/// EPSG code of WGS 84 geographic 2D.
const EPSG_WGS84: u16 = 4326;
/// Window margin around the sampled area, source pixels (the bicubic kernel reaches 2).
const WINDOW_MARGIN_PX: f64 = 3.0;
/// Largest extent the local equirectangular frame is used for, metres (scale error ~ tan(lat0)·Δlat).
const MAX_EXTENT_M: f64 = 200_000.0;
/// Tile alignment tolerance on the common lattice, source pixels.
const ALIGN_TOL_PX: f64 = 1e-3;

/// A geographic raster: pixel centres on a regular lon/lat lattice, row 0 = north.
#[derive(Debug, Clone)]
pub struct GeoRaster {
    pub w: usize,
    pub h: usize,
    /// Longitude of the centre of column 0, degrees.
    pub lon0: f64,
    /// Latitude of the centre of row 0, degrees.
    pub lat0: f64,
    /// Column step east, degrees (> 0).
    pub dlon: f64,
    /// Row step south, degrees (> 0).
    pub dlat: f64,
    /// Heights, metres, row-major; NaN = no data.
    pub data: Vec<f32>,
}

fn tiff_err(what: &str, e: tiff::TiffError) -> Error {
    Error::Invalid(format!("{what}: {e}"))
}

impl GeoRaster {
    /// Reads a `GeoTIFF` file.
    pub fn read(path: &Path) -> Result<Self> {
        Self::decode(BufReader::new(File::open(path)?))
            .map_err(|e| Error::Invalid(format!("{}: {e}", path.display())))
    }

    /// Decodes a `GeoTIFF` from a stream (first image only; COG overviews are ignored).
    pub fn decode<R: Read + Seek>(r: R) -> Result<Self> {
        let mut dec = Decoder::new(r)
            .map_err(|e| tiff_err("tiff", e))?
            .with_limits(Limits::unlimited());
        let (w, h) = dec.dimensions().map_err(|e| tiff_err("dimensions", e))?;
        let (w, h) = (w as usize, h as usize);
        if w < 2 || h < 2 {
            return Err(Error::Invalid(format!("raster {w}x{h} too small")));
        }
        let scale = dec
            .get_tag_f64_vec(Tag::ModelPixelScaleTag)
            .map_err(|e| tiff_err("ModelPixelScaleTag", e))?;
        let tie = dec
            .get_tag_f64_vec(Tag::ModelTiepointTag)
            .map_err(|e| tiff_err("ModelTiepointTag", e))?;
        if scale.len() < 2 || tie.len() < 6 {
            return Err(Error::Invalid("malformed pixel scale or tiepoint".into()));
        }
        let (sx, sy) = (scale[0], scale[1]);
        if !(sx > 0.0 && sy > 0.0) {
            return Err(Error::Invalid(format!(
                "pixel scale ({sx}, {sy}) must be > 0"
            )));
        }
        let keys = dec
            .find_tag_unsigned_vec::<u16>(Tag::GeoKeyDirectoryTag)
            .map_err(|e| tiff_err("GeoKeyDirectoryTag", e))?
            .ok_or_else(|| Error::Invalid("no GeoKeyDirectoryTag (not a GeoTIFF)".into()))?;
        let key = |id: u16| geo_key(&keys, id);
        if key(KEY_MODEL_TYPE) != Some(MODEL_GEOGRAPHIC) {
            return Err(Error::Invalid(format!(
                "model type {:?}: only geographic (lat/lon) rasters are supported",
                key(KEY_MODEL_TYPE)
            )));
        }
        if let Some(gcs) = key(KEY_GEOGRAPHIC_TYPE)
            && gcs != EPSG_WGS84
        {
            return Err(Error::Invalid(format!(
                "geographic type EPSG:{gcs}: only EPSG:4326 is supported"
            )));
        }
        // Raster space: pixel (c, r) covers [c, c+1) x [r, r+1) when pixel-is-area, so its centre is c + 0.5.
        let centre_off = if key(KEY_RASTER_TYPE) == Some(RASTER_PIXEL_IS_POINT) {
            0.0
        } else {
            0.5
        };
        let lon0 = tie[3] + (centre_off - tie[0]) * sx;
        let lat0 = tie[4] - (centre_off - tie[1]) * sy;
        let nodata = match dec.find_tag(Tag::GdalNodata) {
            Ok(Some(v)) => {
                let s = v
                    .into_string()
                    .map_err(|e| tiff_err("GDAL_NODATA", e))?;
                let s = s.trim_matches(|c: char| c == '\0' || c.is_whitespace());
                Some(s.parse::<f64>().map_err(|_| {
                    Error::Invalid(format!("GDAL_NODATA {s:?} is not a number"))
                })?)
            }
            Ok(None) => None,
            Err(e) => return Err(tiff_err("GDAL_NODATA", e)),
        };
        let img = dec.read_image().map_err(|e| tiff_err("image", e))?;
        let mut data: Vec<f32> = match img {
            DecodingResult::F32(v) => v,
            DecodingResult::F64(v) => v.into_iter().map(|x| x as f32).collect(),
            DecodingResult::I16(v) => v.into_iter().map(f32::from).collect(),
            DecodingResult::U16(v) => v.into_iter().map(f32::from).collect(),
            DecodingResult::I32(v) => v.into_iter().map(|x| x as f32).collect(),
            DecodingResult::U8(v) => v.into_iter().map(f32::from).collect(),
            _ => return Err(Error::Invalid("unsupported sample type".into())),
        };
        if data.len() != w * h {
            return Err(Error::Invalid(format!(
                "{} samples for {w}x{h}: only single-band rasters are supported",
                data.len()
            )));
        }
        let nodata = nodata.map(|v| v as f32);
        for v in &mut data {
            if !v.is_finite() || nodata.is_some_and(|nd| v.to_bits() == nd.to_bits()) {
                *v = f32::NAN;
            }
        }
        Ok(Self {
            w,
            h,
            lon0,
            lat0,
            dlon: sx,
            dlat: sy,
            data,
        })
    }

    /// Sample at (column, row), NaN outside.
    #[inline]
    fn at(&self, c: isize, r: isize) -> f32 {
        if c < 0 || r < 0 || c as usize >= self.w || r as usize >= self.h {
            f32::NAN
        } else {
            self.data[r as usize * self.w + c as usize]
        }
    }

    /// Fractional (column, row) of a geographic position.
    #[inline]
    fn pixel_of(&self, lat: f64, lon: f64) -> (f64, f64) {
        ((lon - self.lon0) / self.dlon, (self.lat0 - lat) / self.dlat)
    }
}

/// Value of one `GeoTIFF` key stored inline (`TIFFTagLocation` 0), if present.
fn geo_key(dir: &[u16], id: u16) -> Option<u16> {
    let n = usize::from(*dir.get(3)?);
    dir.get(4..4 + 4 * n)?
        .as_chunks::<4>()
        .0
        .iter()
        .find(|k| k[0] == id && k[1] == 0)
        .map(|k| k[3])
}

/// Mosaics `tiles` (one resolution, one lattice) into a window covering `lat` x `lon` (degrees, min..max) plus
/// a kernel margin; NaN where no tile covers.
pub fn mosaic(tiles: &[GeoRaster], lat: [f64; 2], lon: [f64; 2]) -> Result<GeoRaster> {
    let first = tiles
        .first()
        .ok_or_else(|| Error::Invalid("no tiles".into()))?;
    let (dlon, dlat) = (first.dlon, first.dlat);
    for t in tiles {
        if ((t.dlon - dlon) / dlon).abs() > 1e-9 || ((t.dlat - dlat) / dlat).abs() > 1e-9 {
            return Err(Error::Invalid(format!(
                "tiles differ in resolution ({} x {} vs {dlon} x {dlat} degrees)",
                t.dlon, t.dlat
            )));
        }
    }
    // Lattice anchored on the first tile's pixel (0, 0).
    let c_min = ((lon[0] - first.lon0) / dlon - WINDOW_MARGIN_PX).floor();
    let c_max = ((lon[1] - first.lon0) / dlon + WINDOW_MARGIN_PX).ceil();
    let r_min = ((first.lat0 - lat[1]) / dlat - WINDOW_MARGIN_PX).floor();
    let r_max = ((first.lat0 - lat[0]) / dlat + WINDOW_MARGIN_PX).ceil();
    let w = (c_max - c_min) as usize + 1;
    let h = (r_max - r_min) as usize + 1;
    if w.checked_mul(h).is_none_or(|n| n >= 1 << 31) {
        return Err(Error::Invalid(format!("mosaic window {w}x{h} too large")));
    }
    let mut out = GeoRaster {
        w,
        h,
        lon0: first.lon0 + c_min * dlon,
        lat0: first.lat0 - r_min * dlat,
        dlon,
        dlat,
        data: vec![f32::NAN; w * h],
    };
    for t in tiles {
        let oc = (t.lon0 - out.lon0) / dlon;
        let or = (out.lat0 - t.lat0) / dlat;
        if (oc - oc.round()).abs() > ALIGN_TOL_PX || (or - or.round()).abs() > ALIGN_TOL_PX {
            return Err(Error::Invalid(format!(
                "tile at ({}, {}) is off the common lattice by ({:.4}, {:.4}) px",
                t.lat0,
                t.lon0,
                oc - oc.round(),
                or - or.round()
            )));
        }
        let (oc, or) = (oc.round() as isize, or.round() as isize);
        for r in 0..t.h as isize {
            let wr = r + or;
            if wr < 0 || wr as usize >= h {
                continue;
            }
            for c in 0..t.w as isize {
                let wc = c + oc;
                if wc < 0 || wc as usize >= w {
                    continue;
                }
                let v = t.data[r as usize * t.w + c as usize];
                let slot = &mut out.data[wr as usize * w + wc as usize];
                // Overlapping tiles (shared edge rows): first valid value wins.
                if slot.is_nan() {
                    *slot = v;
                }
            }
        }
    }
    Ok(out)
}

/// Local equirectangular frame: x east, y north, metres, (0, 0) at the centre.
#[derive(Debug, Clone, Copy)]
pub struct LocalFrame {
    pub lat0: f64,
    pub lon0: f64,
    /// Metres per degree of latitude at `lat0` (meridional radius).
    pub m_per_deg_lat: f64,
    /// Metres per degree of longitude at `lat0` (prime-vertical radius · cos lat0).
    pub m_per_deg_lon: f64,
}

impl LocalFrame {
    /// Frame at (lat0, lon0) degrees with the WGS 84 radii of curvature there.
    pub fn wgs84(lat0: f64, lon0: f64) -> Self {
        let (s, c) = lat0.to_radians().sin_cos();
        let q = 1.0 - WGS84_E2 * s * s;
        let m = WGS84_A * (1.0 - WGS84_E2) / (q * q.sqrt());
        let n = WGS84_A / q.sqrt();
        let rad = std::f64::consts::PI / 180.0;
        Self {
            lat0,
            lon0,
            m_per_deg_lat: m * rad,
            m_per_deg_lon: n * c * rad,
        }
    }

    /// (lat, lon) degrees of local (x, y) metres.
    #[inline]
    pub fn to_geo(&self, x: f64, y: f64) -> (f64, f64) {
        (
            self.lat0 + y / self.m_per_deg_lat,
            self.lon0 + x / self.m_per_deg_lon,
        )
    }

    /// Local (x, y) metres of (lat, lon) degrees.
    #[inline]
    pub fn to_local(&self, lat: f64, lon: f64) -> (f64, f64) {
        (
            (lon - self.lon0) * self.m_per_deg_lon,
            (lat - self.lat0) * self.m_per_deg_lat,
        )
    }
}

/// What to import.
#[derive(Debug, Clone)]
pub struct ImportParams {
    /// Centre, degrees north.
    pub centre_lat: f64,
    /// Centre, degrees east.
    pub centre_lon: f64,
    /// Extent `[east-west, north-south]`, metres.
    pub extent_m: [f64; 2],
    /// Output sample spacing, metres.
    pub spacing_m: f64,
    /// Source heights at or below this are sea, metres (Copernicus open ocean is 0).
    pub sea_max_m: f64,
    /// Height written for sea and no-data samples, metres (< 0).
    pub fill_m: f64,
}

/// Summary statistics of an imported grid.
#[derive(Debug, Clone, serde::Serialize)]
pub struct DemStats {
    pub min_m: f64,
    pub max_m: f64,
    /// Highest sample: `[col, row]`.
    pub peak_col_row: [usize; 2],
    /// Highest sample in local metres (x east, y north from the centre).
    pub peak_local_m: [f64; 2],
    /// Highest sample, `[lat, lon]` degrees.
    pub peak_lat_lon: [f64; 2],
    /// Share of samples at or above sea level 0.
    pub land_share: f64,
    pub land_km2: f64,
}

/// An imported grid in the guide frame.
#[derive(Debug, Clone)]
pub struct Imported {
    pub w: usize,
    pub h: usize,
    pub spacing_m: f64,
    /// Local metres of sample (0, 0) (north-west corner): x east, y north.
    pub origin_m: [f64; 2],
    /// Heights, metres, row-major, row 0 = north.
    pub heights: Vec<f32>,
    pub fill_m: f64,
    pub stats: DemStats,
}

/// Reprojects and resamples `tiles` to the local grid `p` describes.
pub fn import(tiles: &[GeoRaster], p: &ImportParams) -> Result<Imported> {
    let s = p.spacing_m;
    if s.is_nan() || s <= 0.0 {
        return Err(Error::Invalid("spacing must be > 0".into()));
    }
    if p.extent_m.iter().any(|&e| e.is_nan() || e < 4.0 * s || e > MAX_EXTENT_M) {
        return Err(Error::Invalid(format!(
            "extent {:?} m out of range (4 samples .. {MAX_EXTENT_M} m)",
            p.extent_m
        )));
    }
    if p.centre_lat.is_nan() || p.centre_lat.abs() > 85.0 || !p.centre_lon.is_finite() {
        return Err(Error::Invalid(format!(
            "centre ({}, {}) out of range",
            p.centre_lat, p.centre_lon
        )));
    }
    if p.fill_m.is_nan() || p.fill_m >= 0.0 {
        return Err(Error::Invalid("fill must be below sea level 0".into()));
    }
    let w = (p.extent_m[0] / s).round() as usize;
    let h = (p.extent_m[1] / s).round() as usize;
    let frame = LocalFrame::wgs84(p.centre_lat, p.centre_lon);
    let origin_m = [-(w as f64 - 1.0) * 0.5 * s, (h as f64 - 1.0) * 0.5 * s];
    // Geographic bounds of the sample lattice (the frame is affine, so the corners bound it).
    let (lat_s, lon_w) = frame.to_geo(origin_m[0], origin_m[1] - (h as f64 - 1.0) * s);
    let (lat_n, lon_e) = frame.to_geo(origin_m[0] + (w as f64 - 1.0) * s, origin_m[1]);
    let src = mosaic(tiles, [lat_s, lat_n], [lon_w, lon_e])?;
    let sea_max = p.sea_max_m;
    let wet = |v: f32| v.is_nan() || f64::from(v) <= sea_max;
    let kernel = |c: isize, r: isize| {
        let v = src.at(c, r);
        if wet(v) { 0.0 } else { f64::from(v) }
    };
    let fill = p.fill_m as f32;
    let heights: Vec<f32> = (0..h)
        .into_par_iter()
        .flat_map_iter(|row| {
            let y = origin_m[1] - row as f64 * s;
            let (src, frame) = (&src, &frame);
            (0..w).map(move |col| {
                let x = origin_m[0] + col as f64 * s;
                let (lat, lon) = frame.to_geo(x, y);
                let (fc, fr) = src.pixel_of(lat, lon);
                if wet(src.at(fc.round() as isize, fr.round() as isize)) {
                    fill
                } else {
                    bicubic_with(fc, fr, kernel).max(0.0) as f32
                }
            })
        })
        .collect();
    let stats = stats_of(&heights, w, s, origin_m, &frame);
    Ok(Imported {
        w,
        h,
        spacing_m: s,
        origin_m,
        heights,
        fill_m: p.fill_m,
        stats,
    })
}

fn stats_of(heights: &[f32], w: usize, s: f64, origin_m: [f64; 2], frame: &LocalFrame) -> DemStats {
    let mut min = f32::INFINITY;
    let mut peak = (f32::NEG_INFINITY, 0usize);
    let mut land = 0usize;
    for (i, &v) in heights.iter().enumerate() {
        min = min.min(v);
        if v > peak.0 {
            peak = (v, i);
        }
        if v >= 0.0 {
            land += 1;
        }
    }
    let (col, row) = (peak.1 % w, peak.1 / w);
    let px = origin_m[0] + col as f64 * s;
    let py = origin_m[1] - row as f64 * s;
    let (plat, plon) = frame.to_geo(px, py);
    DemStats {
        min_m: f64::from(min),
        max_m: f64::from(peak.0),
        peak_col_row: [col, row],
        peak_local_m: [px, py],
        peak_lat_lon: [plat, plon],
        land_share: land as f64 / heights.len() as f64,
        land_km2: land as f64 * s * s * 1e-6,
    }
}

/// Writes `imp` as a guide v1 folder (`heights.f32` + `meta.json`); `extra` keys are merged into the meta
/// (provenance; the v1 keys win).
pub fn write_guide(dir: &Path, imp: &Imported, extra: serde_json::Value) -> Result<()> {
    std::fs::create_dir_all(dir)?;
    let mut bytes = Vec::with_capacity(imp.heights.len() * 4);
    for v in &imp.heights {
        bytes.extend_from_slice(&v.to_le_bytes());
    }
    std::fs::write(dir.join("heights.f32"), &bytes)?;
    let mut meta = match extra {
        serde_json::Value::Object(m) => m,
        serde_json::Value::Null => serde_json::Map::new(),
        _ => return Err(Error::Invalid("guide extra meta must be an object".into())),
    };
    let v1 = serde_json::json!({
        "format": 1,
        "dims": [imp.w, imp.h],
        "sample_spacing_m": imp.spacing_m,
        "sea_level_m": 0.0,
        "fill_m": imp.fill_m,
        "origin_m": imp.origin_m,
        "orientation": "row-major, row 0 = north edge, column 0 = west edge; sample (col, row) at local \
                        x = origin_m.x + spacing col (east), y = origin_m.y - spacing row (north); (0, 0) = centre",
        "stats": imp.stats,
    });
    if let serde_json::Value::Object(m) = v1 {
        meta.extend(m);
    }
    let f = File::create(dir.join("meta.json"))?;
    serde_json::to_writer_pretty(f, &serde_json::Value::Object(meta))?;
    Ok(())
}

/// One `import-dem` run.
#[derive(Debug, Clone)]
pub struct ImportRequest {
    /// `GeoTIFF` files, or folders whose `*.tif` / `*.tiff` files are all read.
    pub inputs: Vec<PathBuf>,
    pub params: ImportParams,
    pub out: PathBuf,
    /// Place name for the meta.
    pub name: Option<String>,
    /// Source line and licence line, copied into the meta.
    pub source: Option<String>,
    pub licence: Option<String>,
}

/// Lists the tile files of `inputs` (folders expanded, sorted for a stable order).
pub fn tile_files(inputs: &[PathBuf]) -> Result<Vec<PathBuf>> {
    let mut files = Vec::new();
    for p in inputs {
        if p.is_dir() {
            let mut found: Vec<PathBuf> = std::fs::read_dir(p)?
                .filter_map(|e| e.ok().map(|e| e.path()))
                .filter(|f| {
                    f.extension()
                        .and_then(|e| e.to_str())
                        .is_some_and(|e| e.eq_ignore_ascii_case("tif") || e.eq_ignore_ascii_case("tiff"))
                })
                .collect();
            found.sort();
            files.extend(found);
        } else {
            files.push(p.clone());
        }
    }
    if files.is_empty() {
        return Err(Error::Invalid("no GeoTIFF tiles found".into()));
    }
    Ok(files)
}

/// Reads the tiles, imports and writes the guide; returns the import.
pub fn run_import(req: &ImportRequest) -> Result<Imported> {
    let files = tile_files(&req.inputs)?;
    let tiles = files
        .iter()
        .map(|f| GeoRaster::read(f))
        .collect::<Result<Vec<_>>>()?;
    let imp = import(&tiles, &req.params)?;
    let p = &req.params;
    let extra = serde_json::json!({
        "name": req.name,
        "source": req.source,
        "licence": req.licence,
        "tiles": files.iter().filter_map(|f| f.file_name().map(|n| n.to_string_lossy().into_owned())).collect::<Vec<_>>(),
        "centre_lat_lon": [p.centre_lat, p.centre_lon],
        "extent_m": p.extent_m,
        "sea_max_m": p.sea_max_m,
        "projection": "local equirectangular at the centre, WGS 84 meridional and prime-vertical radii; \
                       Catmull-Rom bicubic in source pixels, wet/dry class from the nearest source pixel",
        "source_resolution_deg": tiles.first().map(|t| [t.dlon, t.dlat]),
    });
    write_guide(&req.out, &imp, extra)?;
    Ok(imp)
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use tiff::encoder::{Compression, DeflateLevel, TiffEncoder, colortype};

    use super::*;
    use crate::guide::Guide;

    /// Fixture layout: pixel centres on a lon/lat lattice; written pixel-is-area or pixel-is-point.
    struct Fixture {
        w: u32,
        h: u32,
        lon0: f64,
        lat0: f64,
        d: f64,
        point: bool,
        nodata: Option<f32>,
    }

    impl Fixture {
        fn tiepoint(&self) -> [f64; 6] {
            let off = if self.point { 0.0 } else { 0.5 };
            [0.0, 0.0, 0.0, self.lon0 - off * self.d, self.lat0 + off * self.d, 0.0]
        }

        fn keys(&self) -> [u16; 16] {
            let rt = if self.point { 2 } else { 1 };
            [1, 1, 0, 3, 1024, 0, 1, 2, 1025, 0, 1, rt, 2048, 0, 1, 4326]
        }

        fn values(&self, f: &impl Fn(f64, f64) -> f64) -> Vec<f64> {
            let mut v = Vec::with_capacity((self.w * self.h) as usize);
            for r in 0..self.h {
                for c in 0..self.w {
                    v.push(f(
                        self.lat0 - f64::from(r) * self.d,
                        self.lon0 + f64::from(c) * self.d,
                    ));
                }
            }
            v
        }

        /// f32 samples, DEFLATE.
        fn f32_tiff(&self, f: impl Fn(f64, f64) -> f64) -> Vec<u8> {
            let data: Vec<f32> = self
                .values(&f)
                .into_iter()
                .map(|v| {
                    if v.is_nan() {
                        self.nodata.unwrap_or(f32::NAN)
                    } else {
                        v as f32
                    }
                })
                .collect();
            let mut buf = Cursor::new(Vec::new());
            let mut enc = TiffEncoder::new(&mut buf)
                .unwrap()
                .with_compression(Compression::Deflate(DeflateLevel::Balanced));
            let mut img = enc.new_image::<colortype::Gray32Float>(self.w, self.h).unwrap();
            self.tags(img.encoder());
            img.write_data(&data).unwrap();
            buf.into_inner()
        }

        /// i16 samples, uncompressed (SRTM-style).
        fn i16_tiff(&self, f: impl Fn(f64, f64) -> f64) -> Vec<u8> {
            let data: Vec<i16> = self
                .values(&f)
                .into_iter()
                .map(|v| {
                    if v.is_nan() {
                        self.nodata.unwrap() as i16
                    } else {
                        v.round() as i16
                    }
                })
                .collect();
            let mut buf = Cursor::new(Vec::new());
            let mut enc = TiffEncoder::new(&mut buf).unwrap();
            let mut img = enc.new_image::<colortype::GrayI16>(self.w, self.h).unwrap();
            self.tags(img.encoder());
            img.write_data(&data).unwrap();
            buf.into_inner()
        }

        fn tags<W: std::io::Write + Seek, K: tiff::encoder::TiffKind>(
            &self,
            e: &mut tiff::encoder::DirectoryEncoder<'_, W, K>,
        ) {
            e.write_tag(Tag::ModelPixelScaleTag, &[self.d, self.d, 0.0][..])
                .unwrap();
            e.write_tag(Tag::ModelTiepointTag, &self.tiepoint()[..])
                .unwrap();
            e.write_tag(Tag::GeoKeyDirectoryTag, &self.keys()[..])
                .unwrap();
            if let Some(nd) = self.nodata {
                e.write_tag(Tag::GdalNodata, format!("{nd}").as_str())
                    .unwrap();
            }
        }
    }

    fn decode(bytes: Vec<u8>) -> GeoRaster {
        GeoRaster::decode(Cursor::new(bytes)).unwrap()
    }

    const LAT0: f64 = 40.0;
    const LON0: f64 = 10.0;
    /// 1 arc-second.
    const D: f64 = 1.0 / 3600.0;

    fn plane(lat: f64, lon: f64) -> f64 {
        2000.0 + 20_000.0 * (lon - LON0) + 50_000.0 * (lat - LAT0)
    }

    /// 0.04 deg square tile (144 px) around (LAT0, LON0).
    fn fixture(point: bool) -> Fixture {
        Fixture {
            w: 144,
            h: 144,
            lon0: LON0 - 0.02,
            lat0: LAT0 + 0.02,
            d: D,
            point,
            nodata: None,
        }
    }

    fn params(spacing: f64, extent: f64) -> ImportParams {
        ImportParams {
            centre_lat: LAT0,
            centre_lon: LON0,
            extent_m: [extent, extent],
            spacing_m: spacing,
            sea_max_m: 0.0,
            fill_m: -30.0,
        }
    }

    /// Expected height of output sample (col, row) for a function of (lat, lon).
    fn expect(imp: &Imported, col: usize, row: usize, f: impl Fn(f64, f64) -> f64) -> f64 {
        let fr = LocalFrame::wgs84(LAT0, LON0);
        let x = imp.origin_m[0] + col as f64 * imp.spacing_m;
        let y = imp.origin_m[1] - row as f64 * imp.spacing_m;
        let (lat, lon) = fr.to_geo(x, y);
        f(lat, lon)
    }

    #[test]
    fn georeference_pixel_is_area_and_point_agree() {
        for point in [false, true] {
            let g = decode(fixture(point).f32_tiff(plane));
            assert_eq!((g.w, g.h), (144, 144));
            assert!((g.lon0 - (LON0 - 0.02)).abs() < 1e-12, "{point}: {}", g.lon0);
            assert!((g.lat0 - (LAT0 + 0.02)).abs() < 1e-12, "{point}: {}", g.lat0);
            assert!((g.dlon - D).abs() < 1e-15 && (g.dlat - D).abs() < 1e-15);
            // Row 0 is north: the plane rises north, so row 0 is higher than the last row.
            assert!(g.data[0] > g.data[143 * 144]);
            assert!((f64::from(g.data[0]) - plane(g.lat0, g.lon0)).abs() < 1e-3);
        }
    }

    #[test]
    fn plane_is_exact_at_every_sample_and_north_up() {
        let tile = decode(fixture(false).f32_tiff(plane));
        // 3 km at 20 m: well inside the 0.04 deg tile (~4.4 x 3.4 km).
        let imp = import(&[tile], &params(20.0, 3000.0)).unwrap();
        assert_eq!((imp.w, imp.h), (150, 150));
        for &(c, r) in &[(0, 0), (149, 0), (0, 149), (149, 149), (75, 75), (10, 120)] {
            let got = f64::from(imp.heights[r * imp.w + c]);
            let want = expect(&imp, c, r, plane);
            assert!((got - want).abs() < 2e-3, "({c}, {r}): {got} vs {want}");
        }
        // North-west corner higher than south-west (row 0 = north), east higher than west.
        assert!(imp.heights[0] > imp.heights[149 * 150]);
        assert!(imp.heights[149] > imp.heights[0]);
        // Peak at the north-east corner.
        assert_eq!(imp.stats.peak_col_row, [149, 0]);
        assert!(imp.stats.peak_local_m[0] > 0.0 && imp.stats.peak_local_m[1] > 0.0);
        assert!((imp.stats.land_share - 1.0).abs() < 1e-12);
    }

    #[test]
    fn cone_apex_exact_flanks_close_sea_filled() {
        let fr = LocalFrame::wgs84(LAT0, LON0);
        // Apex on the source pixel at (LAT0, LON0); slope 0.5, radius 600 m; nodata outside.
        let cone = |lat: f64, lon: f64| {
            let (x, y) = fr.to_local(lat, lon);
            let h = 300.0 - 0.5 * x.hypot(y);
            if h > 0.0 { h } else { f64::NAN }
        };
        let mut fx = fixture(false);
        fx.nodata = Some(-32767.0);
        let tile = decode(fx.f32_tiff(cone));
        assert!(tile.data.iter().any(|v| v.is_nan()), "nodata decoded as NaN");
        // Odd sample count: the centre sample sits at local (0, 0), on the apex pixel.
        let p = ImportParams {
            extent_m: [61.0 * 20.0, 61.0 * 20.0],
            ..params(20.0, 0.0)
        };
        let imp = import(&[tile], &p).unwrap();
        assert_eq!((imp.w, imp.h), (61, 61));
        let centre = 30 * 61 + 30;
        assert!((f64::from(imp.heights[centre]) - 300.0).abs() < 1e-3, "apex {}", imp.heights[centre]);
        assert_eq!(imp.stats.peak_col_row, [30, 30]);
        assert!(imp.stats.peak_local_m[0].abs() < 1e-9 && imp.stats.peak_local_m[1].abs() < 1e-9);
        // Flank (200 m east, 100 m north of the apex): within a fraction of one source step of slope.
        let got = f64::from(imp.heights[(30 - 5) * 61 + 40]);
        let want = 300.0 - 0.5 * 200.0_f64.hypot(100.0);
        assert!((got - want).abs() < 0.5, "flank {got} vs {want}");
        // Corners are 849 m from the apex: sea (nodata), filled.
        for i in [0, 60, 60 * 61, 61 * 61 - 1] {
            assert!((imp.heights[i] - -30.0).abs() < f32::EPSILON, "corner {i}: {}", imp.heights[i]);
        }
        // Every sample is either the fill or dry (>= 0): no new shallows.
        assert!(imp.heights.iter().all(|&v| (v - -30.0).abs() < f32::EPSILON || v >= 0.0));
        // Land share near the disc area (pi r^2 / box), within the coastline's sample quantisation.
        let disc = std::f64::consts::PI * 600.0 * 600.0 / (61.0 * 20.0 * 61.0 * 20.0);
        assert!((imp.stats.land_share - disc).abs() < 0.03, "land {} vs {disc}", imp.stats.land_share);
    }

    #[test]
    fn mosaic_seam_exact_and_uncovered_is_sea() {
        // West and east halves (72 px each) of the plane, i16 metres with a nodata code; the third quarter of
        // the requested area (south of the tiles) has no tile at all.
        let mut west = fixture(false);
        west.w = 72;
        west.nodata = Some(-32768.0);
        let mut east = fixture(true);
        east.w = 72;
        east.lon0 = west.lon0 + 72.0 * D;
        east.nodata = Some(-32768.0);
        let int_plane = |lat: f64, lon: f64| plane(lat, lon).round();
        let tiles = [decode(west.i16_tiff(int_plane)), decode(east.i16_tiff(int_plane))];
        // Whole-metre source: the output follows the plane within the rounding (0.5 m times the kernel's sum of
        // absolute weights), continuous across the seam between a pixel-is-area and a pixel-is-point tile.
        let imp = import(&tiles, &params(20.0, 3000.0)).unwrap();
        let mid = imp.h / 2;
        // Across the seam (x = 0 east-west at the lattice midpoint), heights rise monotonically east.
        let row = &imp.heights[mid * imp.w..(mid + 1) * imp.w];
        assert!(row.windows(2).all(|p| p[1] > p[0]), "monotonic across the seam");
        for c in [0, 74, 75, 149] {
            let got = f64::from(row[c]);
            let want = expect(&imp, c, mid, plane);
            assert!((got - want).abs() < 0.8, "col {c}: {got} vs {want}");
        }
        // A request reaching past the tiles' south edge (tile spans 0.02 deg ~ 2.2 km south of centre).
        let mut far = params(20.0, 3000.0);
        far.centre_lat = LAT0 - 0.02;
        let imp = import(&tiles, &far).unwrap();
        assert!((imp.heights[imp.w * imp.h - 1] - -30.0).abs() < f32::EPSILON, "uncovered = fill");
        assert!(imp.heights[0] > 0.0, "covered north-west corner is land");
        assert!(imp.stats.land_share > 0.3 && imp.stats.land_share < 0.7);
    }

    #[test]
    fn guide_round_trip() {
        let tile = decode(fixture(false).f32_tiff(plane));
        let imp = import(&[tile], &params(25.0, 2000.0)).unwrap();
        let dir = std::env::temp_dir().join(format!("sf_worldgen_dem_{}", std::process::id()));
        write_guide(&dir, &imp, serde_json::json!({ "name": "fixture" })).unwrap();
        let g = Guide::read(&dir).unwrap();
        assert_eq!((g.w, g.h), (imp.w, imp.h));
        assert!((g.spacing_m - 25.0).abs() < 1e-12);
        assert_eq!(g.origin_m.map(f64::to_bits), imp.origin_m.map(f64::to_bits));
        assert!(g.sea_level_m.abs() < 1e-12);
        assert_eq!(g.fill_m, Some(-30.0));
        assert_eq!(g.heights, imp.heights);
        // Centre at local (0, 0).
        let cx = g.origin_m[0] + (g.w as f64 - 1.0) * 0.5 * g.spacing_m;
        let cy = g.origin_m[1] - (g.h as f64 - 1.0) * 0.5 * g.spacing_m;
        assert!(cx.abs() < 1e-9 && cy.abs() < 1e-9);
        std::fs::remove_dir_all(&dir).ok();
    }

    #[test]
    fn rejects_projected_and_bad_params() {
        let fx = fixture(false);
        let mut bytes = fx.f32_tiff(plane);
        // Patch the model type key value (2 -> 1 = projected) inside the GeoKeyDirectory payload.
        let pat: Vec<u8> = [1024u16, 0, 1, 2].iter().flat_map(|v| v.to_le_bytes()).collect();
        let at = bytes.windows(pat.len()).position(|w| w == pat.as_slice()).unwrap();
        bytes[at + 6] = 1;
        assert!(GeoRaster::decode(Cursor::new(bytes)).is_err());
        let tile = decode(fx.f32_tiff(plane));
        let mut p = params(20.0, 3000.0);
        p.fill_m = 5.0;
        assert!(import(std::slice::from_ref(&tile), &p).is_err());
        assert!(import(std::slice::from_ref(&tile), &params(20.0, 40.0)).is_err());
        assert!(import(&[tile], &params(0.0, 3000.0)).is_err());
    }
}
