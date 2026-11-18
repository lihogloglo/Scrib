//! **Stratovolcano** landforms (WG-S4): a parametric cone or a stamped exemplar heightfield, blended C1 into the
//! surrounding landscape.
//!
//! Parametric profile (height above the local base, at radius r from the vent):
//!
//! - flank, `rc ≤ r < R`: `f(r) = H · (e^{−(r−rc)/r0} − e^{−(R−rc)/r0}) / (1 − e^{−(R−rc)/r0})`. A concave-up
//!   exponential: stratovolcano flank profiles are well fitted by exponential curves (Karátson, Favalli, Tarquini,
//!   Fornaciai, Wörner, "The regular shape of stratovolcanoes: A DEM-based morphometrical approach", JVGR 193,
//!   2010; U = unverified against their fit table), the steep upper cone and the gentle apron of Etna, Fuji or
//!   Mayon. Concave profiles also follow from erosion/deposition balance (Lacey, Ockendon, Turcotte, "On the
//!   geomorphology of volcanoes", EPSL 54, 1981). `f(rc) = H` (the crater rim is the summit), `f(R) = 0`, strictly
//!   decreasing in between; the slope is `H / (r0 (1 − e^{−(R−rc)/r0}))` at the rim.
//! - crater, `r < rc`: a bowl `f(r) = H − d (1 − (r/rc)²)²`, floor `H − d` at the vent, rim at H.
//! - beyond R: 0.
//!
//! Asymmetry: the radius is warped by low-frequency integer-hash value noise, `r' = r (1 + a · n(x))`
//! ([`crate::perturb`], absolute world coordinates), so the cone is not a perfect circle (noise organises the
//! shape, it is not laid on top; worldgen.md "Stage rules").
//!
//! Blend: the cone sits on a base level, the median landscape height on the ring where the blend ends (the
//! surrounding landscape, not the region's own uplifted relief), with H set so the rim reaches the absolute
//! summit target, and replaces the landscape with weight
//! `w = 1 − smootherstep((r'/R − b0) / (b1 − b0))`. Smootherstep is C2 and both fields are smooth there, so the
//! blend is C1 (in fact C2 away from the rim). Sea samples are never touched and land stays at or above sea level.
//!
//! Exemplar: a guide v1 heightfield (e.g. a real DEM) at its own horizontal metres, centred on its highest sample,
//! its heights above a base quantile scaled so its top reaches the same summit target, blended with the same
//! weight over the largest radius that fits inside it (≤ R).
//!
//! Gullies: the runner lightly erodes the parametric cone (a young volcano is lightly eroded) with the existing
//! stream stage and no uplift ([`crate::stream::erode`], restore 0), its erodibility scaled by the blend weight.

/// Parametric stratovolcano shape.
#[derive(Debug, Clone, Copy, PartialEq, serde::Serialize)]
pub struct Strato {
    /// Rim height above the base, metres.
    pub height_m: f64,
    /// Base radius R, metres.
    pub radius_m: f64,
    /// Flank decay length r0, metres.
    pub decay_m: f64,
    /// Crater radius rc, metres.
    pub crater_radius_m: f64,
    /// Crater depth below the rim, metres.
    pub crater_depth_m: f64,
}

impl Strato {
    /// Height above the base at radius `r` metres.
    pub fn profile(&self, r: f64) -> f64 {
        let (h, big_r, r0, rc, d) = (
            self.height_m,
            self.radius_m,
            self.decay_m,
            self.crater_radius_m,
            self.crater_depth_m,
        );
        if r < rc {
            let t = 1.0 - (r / rc) * (r / rc);
            return h - d * t * t;
        }
        if r >= big_r {
            return 0.0;
        }
        let tail = (-(big_r - rc) / r0).exp();
        h * ((-(r - rc) / r0).exp() - tail) / (1.0 - tail)
    }
}

/// Quintic smootherstep on [0, 1] (Perlin 2002 fade), clamped.
pub fn smootherstep(t: f64) -> f64 {
    let t = t.clamp(0.0, 1.0);
    t * t * t * (t * (t * 6.0 - 15.0) + 10.0)
}

/// Blend weight of the landform at normalised radius `q = r'/R`: 1 inside `b0`, 0 beyond `b1`.
pub fn blend_weight(q: f64, b0: f64, b1: f64) -> f64 {
    if b1 <= b0 {
        return if q < b0 { 1.0 } else { 0.0 };
    }
    1.0 - smootherstep((q - b0) / (b1 - b0))
}
