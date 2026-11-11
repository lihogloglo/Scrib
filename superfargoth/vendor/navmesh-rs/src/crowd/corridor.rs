//! Local path maintenance, translated from DetourPathCorridor.cpp; see NOTICE.
use super::*;

/// A bounded polygon corridor, with constrained position and target.
#[derive(Clone, Debug)]
pub struct PathCorridor {
    pub(crate) path: Vec<PolyRef>,
    pub(crate) pos: V3,
    pub(crate) target: V3,
    max: usize,
}
impl PathCorridor {
    /// Allocates a corridor with room for at least three polygons.
    pub fn new(ws: &mut Workspace, max_path: usize) -> Result<Self, Error> {
        ws.begin();
        Self::new_in(ws, max_path)
    }
    pub(crate) fn new_in(ws: &mut Workspace, max_path: usize) -> Result<Self, Error> {
        if max_path < 3 {
            return Err(Error::InvalidParameter("max_path"));
        }
        Ok(Self {
            path: ws.vec_with_capacity(max_path)?,
            pos: [0.; 3],
            target: [0.; 3],
            max: max_path,
        })
    }
    /// Current polygon sequence.
    pub fn path(&self) -> &[PolyRef] {
        &self.path
    }
    /// Constrained current position.
    pub fn position(&self) -> V3 {
        self.pos
    }
    /// Constrained target.
    pub fn target(&self) -> V3 {
        self.target
    }
    /// First polygon, or zero for an empty corridor.
    pub fn first_poly(&self) -> PolyRef {
        self.path.first().copied().unwrap_or(0)
    }
    /// Last polygon, or zero for an empty corridor.
    pub fn last_poly(&self) -> PolyRef {
        self.path.last().copied().unwrap_or(0)
    }
    /// Resets to one polygon (zero denotes an invalid location).
    pub fn reset(&mut self, poly: PolyRef, pos: V3) -> Result<(), Error> {
        finite(pos)?;
        self.pos = pos;
        self.target = pos;
        self.path.clear();
        self.path.push(poly);
        Ok(())
    }
    /// Replaces the target and polygon sequence.
    pub fn set_corridor(&mut self, target: V3, path: &[PolyRef]) -> Result<(), Error> {
        finite(target)?;
        if path.is_empty() || path.len() > self.max {
            return Err(Error::InvalidParameter("corridor path"));
        }
        self.target = target;
        self.path.clear();
        self.path.extend_from_slice(path);
        Ok(())
    }
    pub(crate) fn merge(&mut self, visited: &[PolyRef], mode: u8) {
        let mut common = None;
        if mode == 1 {
            for i in 0..self.path.len() {
                for j in (0..visited.len()).rev() {
                    if self.path[i] == visited[j] {
                        common = Some((i, j));
                    }
                }
                if common.is_some() {
                    break;
                }
            }
        } else {
            for i in (0..self.path.len()).rev() {
                for j in (0..visited.len()).rev() {
                    if self.path[i] == visited[j] {
                        common = Some((i, j));
                    }
                }
                if common.is_some() {
                    break;
                }
            }
        }
        let Some((i, j)) = common else { return };
        if mode == 1 {
            self.path.truncate(i + 1);
            self.path.extend(
                visited[j + 1..]
                    .iter()
                    .take(self.max - self.path.len())
                    .copied(),
            );
        } else {
            let req = if mode == 0 { visited.len() - j } else { j };
            if req == 0 {
                return;
            }
            let orig = if mode == 0 { i + 1 } else { i };
            let req = req.min(self.max);
            let size = (self.path.len() - orig).min(self.max - req);
            self.path.resize(self.max, 0);
            self.path.copy_within(orig..orig + size, req);
            for k in 0..req {
                self.path[k] = if mode == 0 {
                    visited[visited.len() - 1 - k]
                } else {
                    visited[k]
                };
            }
            self.path.truncate(req + size);
        }
    }
    /// Steering corners, pruned before the first off-mesh connection.
    pub fn find_corners(
        &self,
        q: &NavMeshQuery<'_>,
        out: &mut Vec<StraightPathPoint>,
        max: usize,
    ) -> Result<(), Error> {
        q.find_straight_path(&self.pos, &self.target, &self.path, out, max, 0)?;
        while out.first().is_some_and(|p| {
            p.flags & STRAIGHTPATH_OFFMESH_CONNECTION == 0
                && vdist_2d_sqr(&p.pos, &self.pos) <= sqr(0.01)
        }) {
            out.remove(0);
        }
        if let Some(i) = out
            .iter()
            .position(|p| p.flags & STRAIGHTPATH_OFFMESH_CONNECTION != 0)
        {
            out.truncate(i + 1);
        }
        Ok(())
    }
    /// Shortcuts visible corridor polygons within the optimization range.
    pub fn optimize_visibility(
        &mut self,
        next: V3,
        range: f32,
        q: &mut NavMeshQuery<'_>,
        f: &dyn QueryFilter,
    ) -> Result<(), Error> {
        finite(next)?;
        positive(range)?;
        let mut dist = vdist_2d(&self.pos, &next);
        if dist < 0.01 {
            return Ok(());
        }
        dist = fmin(dist + 0.01, range);
        let goal = vmad(&self.pos, &vsub(&next, &self.pos), range / dist);
        let (hit, _) = q.raycast(self.first_poly(), &self.pos, &goal, f, 32)?;
        if hit.path.len() > 1 && hit.t > 0.99 {
            self.merge(&hit.path, 2);
        }
        Ok(())
    }
    /// Performs a bounded topology search along this corridor.
    pub fn optimize_topology(
        &mut self,
        q: &mut NavMeshQuery<'_>,
        f: &dyn QueryFilter,
    ) -> Result<bool, Error> {
        if self.path.len() < 3 {
            return Ok(false);
        }
        q.init_sliced_find_path(
            self.first_poly(),
            self.last_poly(),
            &self.pos,
            &self.target,
            f,
            0,
        )?;
        q.update_sliced_find_path(32, f)?;
        let mut res = Vec::new();
        q.finalize_sliced_find_path_partial(f, &self.path, &mut res, 32)?;
        if res.is_empty() {
            return Ok(false);
        }
        self.merge(&res, 2);
        Ok(true)
    }
    /// Constrains a movement to the surface and updates the corridor start.
    pub fn move_position(
        &mut self,
        pos: V3,
        q: &mut NavMeshQuery<'_>,
        f: &dyn QueryFilter,
    ) -> Result<(), Error> {
        finite(pos)?;
        let mut visited = Vec::new();
        let (mut result, _) =
            q.move_along_surface(self.first_poly(), &self.pos, &pos, f, &mut visited, 16)?;
        self.merge(&visited, 0);
        result[1] = q
            .poly_height(self.first_poly(), &result)
            .unwrap_or(self.pos[1]);
        self.pos = result;
        Ok(())
    }
    /// Constrains a target movement and updates the corridor end.
    pub fn move_target_position(
        &mut self,
        pos: V3,
        q: &mut NavMeshQuery<'_>,
        f: &dyn QueryFilter,
    ) -> Result<(), Error> {
        finite(pos)?;
        let mut visited = Vec::new();
        let (result, _) =
            q.move_along_surface(self.last_poly(), &self.target, &pos, f, &mut visited, 16)?;
        self.merge(&visited, 1);
        self.target = result;
        Ok(())
    }
    /// Advances over a connection, returning its endpoints and polygon pair.
    pub fn move_over_offmesh_connection(
        &mut self,
        r: PolyRef,
        q: &NavMeshQuery<'_>,
    ) -> Result<(V3, V3, [PolyRef; 2]), Error> {
        let mut prev = 0;
        let mut poly = self.first_poly();
        let mut n = 0;
        while n < self.path.len() && poly != r {
            prev = poly;
            poly = self.path[n];
            n += 1;
        }
        if n == self.path.len() {
            return Err(Error::InvalidRef);
        }
        self.path.drain(..n);
        let (start, end) = q.nav().off_mesh_connection_poly_end_points(prev, poly)?;
        self.pos = end;
        Ok((start, end, [prev, poly]))
    }
    /// Inserts a safe start while retaining the tail for replanning.
    pub fn fix_path_start(&mut self, r: PolyRef, pos: V3) -> Result<(), Error> {
        finite(pos)?;
        self.pos = pos;
        if self.path.is_empty() {
            self.path.push(r);
        }
        if self.path.len() < 3 {
            let last = self.last_poly();
            self.path.resize(3, 0);
            self.path[2] = last;
        }
        self.path[0] = r;
        self.path[1] = 0;
        Ok(())
    }
    /// Keeps the valid prefix and clamps the target to its last polygon.
    pub fn trim_invalid_path(
        &mut self,
        r: PolyRef,
        pos: V3,
        q: &NavMeshQuery<'_>,
        f: &dyn QueryFilter,
    ) -> Result<(), Error> {
        finite(pos)?;
        let n = self
            .path
            .iter()
            .take_while(|&&r| q.is_valid_poly_ref(r, f))
            .count();
        if n == self.path.len() {
            return Ok(());
        }
        if n == 0 {
            self.pos = pos;
            self.path.clear();
            self.path.push(r);
        } else {
            self.path.truncate(n);
        }
        self.target = q.closest_point_on_poly_boundary(self.last_poly(), &self.target)?;
        Ok(())
    }
    /// Checks the first `lookahead` polygons against the current mesh and filter.
    pub fn is_valid(&self, lookahead: usize, q: &NavMeshQuery<'_>, f: &dyn QueryFilter) -> bool {
        !self.path.is_empty()
            && self
                .path
                .iter()
                .take(lookahead)
                .all(|&r| q.is_valid_poly_ref(r, f))
    }
}
