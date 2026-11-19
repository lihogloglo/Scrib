//! Anchors of a scaled world: authored patches that move rigidly instead of scaling (spec
//! `bevyrrowind/docs/specs/worldgen-adapter.md` "Same map at 10×", step 3; framework spec `worldgen.md`).
//!
//! A scaled world maps a source point `p` (metres, x east, y north) to `S·p`. An anchor keeps its authored
//! content at 1:1: every point inside its footprint moves rigidly with the anchor's centre `c`,
//! `world = S·c + (p − c)` in x, y and `height + L` (the anchor's lift). Points outside every footprint scale.
//!
//! `anchors.json` (guide folder, format 1) holds the transform: the scale and per anchor its `name`, `stamp`
//! (index into the guide's `stamps.json`, `null` for a point anchor, which has no stamp and whose members are
//! grounded on the final ground), `source_centre_m`, `world_centre_m`, `lift_m`, the member reference
//! keys (adapter-defined strings a placement is matched by) and the footprint as discs in the source frame. The
//! file is generic: an adapter decides what its members are.

use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::{Error, Result};

/// Version of `anchors.json`.
pub const ANCHORS_FORMAT: u32 = 1;

/// The file name in a guide folder.
pub const ANCHORS_FILE: &str = "anchors.json";

/// Frame text stored in `anchors.json`.
pub const FRAME: &str = "source_centre_m and footprint discs: source frame metres (x east, y north); world_centre_m: \
     the scaled guide frame (x east, y north); a footprint point p maps to world_centre_m + (p - source_centre_m), \
     height + lift_m";

/// An anchor's footprint: the union of discs `[x, y, radius]` in the source frame (metres).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Footprint {
    pub discs: Vec<[f64; 3]>,
}

impl Footprint {
    /// Whether `p` (source frame) lies in a disc (edge included).
    pub fn contains(&self, p: [f64; 2]) -> bool {
        self.discs.iter().any(|&[x, y, r]| (p[0] - x).hypot(p[1] - y) <= r)
    }

    /// Distance from `p` to the footprint, 0 inside.
    pub fn distance(&self, p: [f64; 2]) -> f64 {
        self.discs
            .iter()
            .map(|&[x, y, r]| ((p[0] - x).hypot(p[1] - y) - r).max(0.0))
            .fold(f64::INFINITY, f64::min)
    }

    /// `[min_x, min_y, max_x, max_y]` of the discs, `None` when empty.
    pub fn bounds(&self) -> Option<[f64; 4]> {
        let first = self.discs.first()?;
        let mut b = [first[0] - first[2], first[1] - first[2], first[0] + first[2], first[1] + first[2]];
        for &[x, y, r] in &self.discs[1..] {
            b = [b[0].min(x - r), b[1].min(y - r), b[2].max(x + r), b[3].max(y + r)];
        }
        Some(b)
    }
}

/// One anchor's place in the scaled world.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AnchorPlace {
    pub name: String,
    /// Index of its stamp in the guide's `stamps.json`; `None` for a point anchor (no stamp: its members are
    /// grounded on the final ground, each keeping its height above the source ground; `lift_m` is a fallback).
    pub stamp: Option<usize>,
    /// Centre `c`, source frame metres.
    pub source_centre_m: [f64; 2],
    /// `S·c`, scaled guide frame metres.
    pub world_centre_m: [f64; 2],
    /// Height added to every point of the anchor, metres.
    pub lift_m: f64,
    /// Member reference keys (adapter-defined).
    pub members: Vec<String>,
    pub footprint: Footprint,
}

impl AnchorPlace {
    /// The rigid move of a source point `p` at height `h`: `(S·c + (p − c), h + L)`.
    pub fn rigid(&self, p: [f64; 2], h: f64) -> ([f64; 2], f64) {
        let [cx, cy] = self.source_centre_m;
        let [wx, wy] = self.world_centre_m;
        ([wx + (p[0] - cx), wy + (p[1] - cy)], h + self.lift_m)
    }
}

/// The source → scaled world map: rigid inside anchor footprints, uniform scale elsewhere.
#[derive(Clone, Debug, PartialEq)]
pub struct AnchorTransform {
    pub scale: f64,
    pub anchors: Vec<AnchorPlace>,
    /// Footprint bounds per anchor (prefilter), `[min_x, min_y, max_x, max_y]`.
    bounds: Vec<[f64; 4]>,
}

#[derive(Serialize, Deserialize)]
struct AnchorsFile {
    format: u32,
    frame: String,
    scale: f64,
    anchors: Vec<AnchorPlace>,
}

impl AnchorTransform {
    pub fn new(scale: f64, anchors: Vec<AnchorPlace>) -> Self {
        let bounds = anchors
            .iter()
            .map(|a| a.footprint.bounds().unwrap_or([f64::INFINITY, f64::INFINITY, f64::NEG_INFINITY, f64::NEG_INFINITY]))
            .collect();
        Self { scale, anchors, bounds }
    }

    /// The anchor whose footprint holds source point `p` (the first by index when footprints touch).
    pub fn anchor_of(&self, p: [f64; 2]) -> Option<usize> {
        self.bounds
            .iter()
            .zip(&self.anchors)
            .position(|(b, a)| p[0] >= b[0] && p[0] <= b[2] && p[1] >= b[1] && p[1] <= b[3] && a.footprint.contains(p))
    }

    /// A source point `p` (x east, y north) at height `h` in the scaled world: rigid through its anchor inside a
    /// footprint, `(S·p, S·h)` elsewhere (the plain scale; callers outside anchors snap to the final terrain).
    pub fn source_to_world(&self, p: [f64; 2], h: f64) -> ([f64; 2], f64) {
        match self.anchor_of(p) {
            Some(i) => self.anchors[i].rigid(p, h),
            None => ([p[0] * self.scale, p[1] * self.scale], h * self.scale),
        }
    }

    /// Reads `dir/anchors.json`; `Ok(None)` when the guide has none.
    pub fn read(dir: &Path) -> Result<Option<Self>> {
        let path = dir.join(ANCHORS_FILE);
        if !path.exists() {
            return Ok(None);
        }
        let file: AnchorsFile = serde_json::from_slice(&std::fs::read(path)?)?;
        if file.format != ANCHORS_FORMAT {
            return Err(Error::Invalid(format!("anchors.json format {} ({ANCHORS_FORMAT} expected)", file.format)));
        }
        if !(file.scale.is_finite() && file.scale > 0.0) {
            return Err(Error::Invalid("anchors.json scale must be > 0".into()));
        }
        Ok(Some(Self::new(file.scale, file.anchors)))
    }

    /// Writes `dir/anchors.json` (the folder is created).
    pub fn write(&self, dir: &Path) -> Result<()> {
        std::fs::create_dir_all(dir)?;
        let file =
            AnchorsFile { format: ANCHORS_FORMAT, frame: FRAME.into(), scale: self.scale, anchors: self.anchors.clone() };
        std::fs::write(dir.join(ANCHORS_FILE), serde_json::to_vec(&file)?)?;
        Ok(())
    }
}

#[cfg(test)]
#[allow(clippy::float_cmp)] // exact results are the point of these tests
mod tests {
    use super::*;

    fn place() -> AnchorPlace {
        let c = [100.0, -50.0];
        AnchorPlace {
            name: "a".into(),
            stamp: Some(0),
            source_centre_m: c,
            world_centre_m: [c[0] * 10.0, c[1] * 10.0],
            lift_m: 42.5,
            members: vec!["k1".into(), "k2".into()],
            footprint: Footprint { discs: vec![[95.0, -50.0, 6.0], [108.0, -48.0, 4.0]] },
        }
    }

    #[test]
    fn member_moves_rigidly_and_outside_scales() {
        let t = AnchorTransform::new(10.0, vec![place()]);
        let p = [95.0, -50.0];
        let (w, h) = t.source_to_world(p, 3.0);
        assert_eq!(w, [1000.0 + (95.0 - 100.0), -500.0]);
        assert_eq!(h, 3.0 + 42.5);
        assert_eq!(t.anchor_of([500.0, 500.0]), None);
        assert_eq!(t.source_to_world([500.0, 1.0], 2.0), ([5000.0, 10.0], 20.0));
        assert_eq!(t.anchors[0].footprint.distance(p), 0.0);
    }

    #[test]
    fn anchors_json_round_trip() {
        let dir = std::env::temp_dir().join(format!("sf_worldgen_anchors_{}", std::process::id()));
        let t = AnchorTransform::new(10.0, vec![place()]);
        t.write(&dir).unwrap();
        let back = AnchorTransform::read(&dir).unwrap().unwrap();
        std::fs::remove_dir_all(&dir).ok();
        assert_eq!(back, t);
        assert!(AnchorTransform::read(&dir).unwrap().is_none());
    }
}
