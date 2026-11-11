//! Cached local walls, translated from DetourLocalBoundary.cpp; see NOTICE.
use super::*;
/// Up to eight nearest wall segments from a local polygon neighbourhood.
#[derive(Clone, Debug)]
pub struct LocalBoundary {
    pub(crate) center: V3,
    pub(crate) segments: Vec<(f32, [f32; 6])>,
    polys: Vec<PolyRef>,
}
impl LocalBoundary {
    /// Allocates storage for the fixed upstream limits.
    pub fn new(ws: &mut Workspace) -> Result<Self, Error> {
        ws.begin();
        Self::new_in(ws)
    }
    pub(crate) fn new_in(ws: &mut Workspace) -> Result<Self, Error> {
        Ok(Self {
            center: [f32::MAX; 3],
            segments: ws.vec_with_capacity(8)?,
            polys: ws.vec_with_capacity(16)?,
        })
    }
    /// Invalidates the cache.
    pub fn reset(&mut self) {
        self.center = [f32::MAX; 3];
        self.segments.clear();
        self.polys.clear();
    }
    /// Cache center.
    pub fn center(&self) -> V3 {
        self.center
    }
    /// Nearest wall segments with squared distances, in ascending order.
    pub fn segments(&self) -> &[(f32, [f32; 6])] {
        &self.segments
    }
    /// Whether all cached polygons remain traversable.
    pub fn is_valid(&self, q: &NavMeshQuery<'_>, f: &dyn QueryFilter) -> bool {
        !self.polys.is_empty() && self.polys.iter().all(|&r| q.is_valid_poly_ref(r, f))
    }
    /// Rebuilds the cache at a position.
    pub fn update(
        &mut self,
        r: PolyRef,
        pos: V3,
        range: f32,
        q: &mut NavMeshQuery<'_>,
        f: &dyn QueryFilter,
    ) -> Result<(), Error> {
        finite(pos)?;
        positive(range)?;
        if r == 0 {
            self.reset();
            return Ok(());
        }
        self.center = pos;
        let mut polys = Vec::new();
        q.find_local_neighbourhood(r, &pos, range, f, &mut polys, 16)?;
        self.polys.clear();
        self.polys.extend(polys.iter().map(|p| p.0));
        self.segments.clear();
        let mut segs = Vec::new();
        for &r in &self.polys {
            q.poly_wall_segments(r, f, false, &mut segs, 18)?;
            for s in &segs {
                let d = distance_pt_seg_sqr_2d(&pos, &s.seg[..3], &s.seg[3..]).0;
                if d > sqr(range) {
                    continue;
                }
                let at = insert_pos(&self.segments, d, |s| s.0);
                if at < 8 {
                    if self.segments.len() == 8 {
                        self.segments.pop();
                    }
                    self.segments.insert(at, (d, s.seg));
                }
            }
        }
        Ok(())
    }
}
