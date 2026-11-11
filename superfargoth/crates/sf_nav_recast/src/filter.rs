//! Our [`QueryFilter`] as Detour's (`dtQueryFilter`): a polygon passes on our include/exclude
//! flags and a finite area cost; moving across a polygon costs meters times its area's cost.

use navmesh_rs::detour::{self, MeshTile, Poly};
use sf_nav::{AreaId, PolyFlags, QueryFilter};

pub(crate) struct DtFilter<'a>(pub &'a QueryFilter);

impl detour::QueryFilter for DtFilter<'_> {
    #[inline]
    fn pass_filter(&self, _r: detour::PolyRef, _tile: &MeshTile, poly: &Poly) -> bool {
        self.0
            .passes(AreaId(poly.area()), PolyFlags::from_bits_retain(poly.flags))
    }

    #[inline]
    fn cost(
        &self,
        pa: &[f32; 3],
        pb: &[f32; 3],
        _prev: Option<(detour::PolyRef, &MeshTile, &Poly)>,
        cur: (detour::PolyRef, &MeshTile, &Poly),
        _next: Option<(detour::PolyRef, &MeshTile, &Poly)>,
    ) -> f32 {
        let d = glam::Vec3::from_array(*pa).distance(glam::Vec3::from_array(*pb));
        d * self.0.area_cost(AreaId(cur.2.area()))
    }
}
