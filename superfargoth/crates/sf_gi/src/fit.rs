//! Fitting a probe grid to a box (`docs/specs/gi.md` "Probe volumes", interiors): one grid
//! over the worldspace's content bounds, padded by one probe spacing on every side, at the
//! finest spacing (a multiple of [`VolumeBudget::min_spacing`]) whose grid stays within the
//! probe budget.

use glam::{UVec3, Vec3};

use crate::GiSettings;

/// The largest grid a fitted volume may use, and its finest spacing.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct VolumeBudget {
    /// Probes per axis at most (each at least 3: one probe of padding on both sides).
    pub max_dims: UVec3,
    /// Metres between probes at least; the spacing is a whole multiple of it.
    pub min_spacing: f32,
}

impl Default for VolumeBudget {
    /// 32x16x32 probes (16k) at 1 m or coarser (spec "Probe volumes").
    fn default() -> Self {
        Self { max_dims: UVec3::new(32, 16, 32), min_spacing: 1.0 }
    }
}

/// A grid fitted to a box: centre, spacing and probe counts.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct VolumeFit {
    /// The grid's centre (the box's centre).
    pub centre: Vec3,
    /// Metres between probes.
    pub spacing: f32,
    pub dims: UVec3,
}

impl VolumeFit {
    /// `base` with this fit's dims and spacing (rays, budget and hysteresis kept).
    pub fn settings(&self, base: GiSettings) -> GiSettings {
        GiSettings { dims: self.dims, spacing: self.spacing, ..base }
    }
}

/// Slack for float error when counting probes (an extent of exactly `n` spacings is `n`).
const EPSILON: f32 = 1e-4;

/// Fits a grid over the box `min..max`, padded by one spacing on every side.
///
/// The spacing is the smallest multiple of `budget.min_spacing` (at least that) with
/// `extent / spacing + 2 <= max_dims` on every axis, i.e. `ceil(max(extent / (max_dims - 2)) /
/// min_spacing) * min_spacing`; dims are `ceil(extent / spacing) + 2`. `None` for an empty or
/// non-finite box, or a non-positive spacing.
pub fn fit_volume(min: Vec3, max: Vec3, budget: VolumeBudget) -> Option<VolumeFit> {
    let step = budget.min_spacing;
    if !(min.is_finite() && max.is_finite() && step.is_finite() && step > 0.0) || min.cmpgt(max).any() {
        return None;
    }
    let extent = max - min;
    let inner = budget.max_dims.max(UVec3::splat(3)).as_vec3() - 2.0;
    let raw = (extent / inner).max_element();
    let spacing = (((raw / step) - EPSILON).ceil().max(1.0)) * step;
    let cells = ((extent / spacing) - EPSILON).ceil().max(Vec3::ZERO);
    let dims = cells.as_uvec3() + 2;
    Some(VolumeFit { centre: (min + max) * 0.5, spacing, dims })
}

#[cfg(test)]
#[allow(clippy::float_cmp)] // exact values by design (whole-metre spacings)
mod tests {
    use super::*;

    fn fit(min: [f32; 3], max: [f32; 3]) -> VolumeFit {
        fit_volume(Vec3::from_array(min), Vec3::from_array(max), VolumeBudget::default()).expect("a fit")
    }

    #[test]
    fn a_room_gets_one_metre_probes_and_one_probe_of_padding() {
        let f = fit([0.0, 0.0, 0.0], [14.0, 6.0, 20.0]);
        assert_eq!(f, VolumeFit { centre: Vec3::new(7.0, 3.0, 10.0), spacing: 1.0, dims: UVec3::new(16, 8, 22) });
    }

    #[test]
    fn fractional_extents_round_the_probe_count_up() {
        let f = fit([-3.2, 0.5, 1.0], [3.3, 4.0, 9.9]);
        assert_eq!(f.spacing, 1.0);
        assert_eq!(f.dims, UVec3::new(9, 6, 11));
    }

    #[test]
    fn an_extent_of_exactly_the_budget_stays_at_the_finest_spacing() {
        let f = fit([0.0; 3], [30.0, 14.0, 30.0]);
        assert_eq!((f.spacing, f.dims), (1.0, UVec3::new(32, 16, 32)));
    }

    #[test]
    fn a_large_interior_coarsens_to_whole_metres_within_the_budget() {
        let f = fit([0.0; 3], [100.0, 10.0, 40.0]);
        // 100 m over 30 inner probes: 3.33 m, rounded up to 4 m.
        assert_eq!((f.spacing, f.dims), (4.0, UVec3::new(27, 5, 12)));
        let g = fit([-500.0, -20.0, -10.0], [500.0, 20.0, 10.0]);
        assert_eq!((g.spacing, g.dims), (34.0, UVec3::new(32, 4, 3)));
    }

    #[test]
    fn spacing_is_a_multiple_of_the_minimum() {
        let budget = VolumeBudget { max_dims: UVec3::new(32, 16, 32), min_spacing: 0.5 };
        let f = fit_volume(Vec3::ZERO, Vec3::new(20.0, 3.0, 5.0), budget).unwrap();
        // 20 / 30 = 0.67 m: rounded up to 1.0 (two half-metre steps).
        assert_eq!((f.spacing, f.dims), (1.0, UVec3::new(22, 5, 7)));
        let small = fit_volume(Vec3::ZERO, Vec3::new(4.0, 2.0, 4.0), budget).unwrap();
        assert_eq!((small.spacing, small.dims), (0.5, UVec3::new(10, 6, 10)));
    }

    #[test]
    fn a_point_gets_a_two_probe_grid() {
        let f = fit([1.0, 2.0, 3.0], [1.0, 2.0, 3.0]);
        assert_eq!(f, VolumeFit { centre: Vec3::new(1.0, 2.0, 3.0), spacing: 1.0, dims: UVec3::splat(2) });
    }

    #[test]
    fn the_grid_covers_the_box_with_a_spacing_of_padding_within_budget() {
        let budget = VolumeBudget::default();
        for (min, max) in [([0.0, 0.0, 0.0], [17.3, 4.1, 63.0]), ([-40.0, -2.0, 5.0], [41.0, 9.0, 6.0]), ([0.0; 3], [0.1, 0.1, 0.1])] {
            let (min, max) = (Vec3::from_array(min), Vec3::from_array(max));
            let f = fit_volume(min, max, budget).unwrap();
            let half = f.dims.as_vec3() * f.spacing * 0.5;
            assert!((f.centre - half).cmple(min - f.spacing + 1e-3).all(), "{f:?}");
            assert!((f.centre + half).cmpge(max + f.spacing - 1e-3).all(), "{f:?}");
            assert!(f.dims.cmple(budget.max_dims).all(), "{f:?}");
        }
    }

    #[test]
    fn empty_or_broken_boxes_get_no_fit() {
        let b = VolumeBudget::default();
        assert_eq!(fit_volume(Vec3::ONE, Vec3::ZERO, b), None);
        assert_eq!(fit_volume(Vec3::ZERO, Vec3::splat(f32::INFINITY), b), None);
        assert_eq!(fit_volume(Vec3::ZERO, Vec3::ONE, VolumeBudget { min_spacing: 0.0, ..b }), None);
    }

    #[test]
    fn settings_keep_the_rest() {
        let f = fit([0.0; 3], [14.0, 6.0, 20.0]);
        let s = f.settings(GiSettings::default());
        assert_eq!((s.dims, s.spacing, s.rays, s.hysteresis), (f.dims, 1.0, 64, 0.95));
    }
}
