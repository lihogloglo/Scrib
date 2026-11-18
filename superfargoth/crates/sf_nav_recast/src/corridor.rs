//! Detour corridor operations on [`RecastBackend`] (`docs/specs/nav.md`, NV8): surface movement,
//! string-pulled steering corners, closest-point clamping, and bounded local partial A* repair
//! (`DetourPathCorridor.cpp`).

use glam::Vec2;
use navmesh_rs::detour::{self, StraightPathPoint};
use sf_nav::{
    CORNER_PRUNE_DIST_M, NavError, NavPoint, PolyRef, QueryFilter, SurfaceMove,
};
use sf_world::Location;

use crate::filter::DtFilter;
use crate::{RecastBackend, backend, refs};

// tunables: const (Detour dtPathCorridor movePosition visited polygon buffer size)
const SURFACE_VISITED_CAP: usize = 16;

pub(crate) fn closest_point_on_poly(
    b: &RecastBackend,
    poly: PolyRef,
    at: &Location,
) -> Result<Location, NavError> {
    let (m, r) = b.resolve(poly).ok_or(NavError::UnknownPoly(poly))?;
    if m.key.ws != at.worldspace {
        return Err(NavError::UnknownPoly(poly));
    }
    let p = m.local(at.pos);
    let clamped = b.with_query(m, |q| {
        q.closest_point_on_poly_boundary(r, &p).map_err(backend)
    })??;
    Ok(m.world(&at.worldspace, clamped))
}

pub(crate) fn move_along_surface(
    b: &RecastBackend,
    from: &NavPoint,
    to: &Location,
    filter: &QueryFilter,
) -> Result<SurfaceMove, NavError> {
    let ws = &from.loc.worldspace;
    if &to.worldspace != ws {
        return Err(NavError::WrongWorldspace {
            query: ws.clone(),
            goal: to.worldspace.clone(),
        });
    }
    let (m, start_ref) = b
        .resolve(from.poly)
        .ok_or(NavError::UnknownPoly(from.poly))?;
    if &m.key.ws != ws {
        return Err(NavError::UnknownPoly(from.poly));
    }
    let start = m.local(from.loc.pos);
    let end = m.local(to.pos);
    let f = DtFilter(filter);
    let (result, visited) = b.with_query(m, |q| {
        if !q.is_valid_poly_ref(start_ref, &f) {
            return Err(NavError::UnknownPoly(from.poly));
        }
        let mut visited = Vec::new();
        let (mut result, _) = q
            .move_along_surface(start_ref, &start, &end, &f, &mut visited, SURFACE_VISITED_CAP)
            .map_err(backend)?;
        let last_ref = visited.last().copied().unwrap_or(start_ref);
        result[1] = q.poly_height(last_ref, &result).unwrap_or(start[1]);
        Ok::<_, NavError>((result, visited))
    })??;
    Ok(SurfaceMove {
        pos: m.world(ws, result),
        visited: visited.into_iter().map(|r| m.poly(r)).collect(),
    })
}

pub(crate) fn corridor_corners(
    b: &RecastBackend,
    pos: &Location,
    target: &Location,
    polys: &[PolyRef],
    max_corners: usize,
) -> Result<Vec<Location>, NavError> {
    let ws = &pos.worldspace;
    if &target.worldspace != ws {
        return Err(NavError::WrongWorldspace {
            query: ws.clone(),
            goal: target.worldspace.clone(),
        });
    }
    let Some(&first) = polys.first() else {
        return Err(NavError::Backend("empty corridor".into()));
    };
    if max_corners == 0 {
        return Ok(Vec::new());
    }
    let (m, _) = b.resolve(first).ok_or(NavError::UnknownPoly(first))?;
    if &m.key.ws != ws {
        return Err(NavError::UnknownPoly(first));
    }
    let dt_polys: Vec<detour::PolyRef> = polys
        .iter()
        .copied()
        .map_while(|p| {
            let (tag, r) = refs::split(p);
            (tag == m.tag && m.nav.is_valid_poly_ref(r)).then_some(r)
        })
        .collect();
    if dt_polys.is_empty() {
        return Err(NavError::UnknownPoly(first));
    }
    let start = m.local(pos.pos);
    let end = m.local(target.pos);
    let pts = b.with_query(m, |q| {
        let mut pts: Vec<StraightPathPoint> = Vec::new();
        q.find_straight_path(
            &start,
            &end,
            &dt_polys,
            &mut pts,
            max_corners.saturating_add(1),
            0,
        )
        .map_err(backend)?;
        let prune_r2 = CORNER_PRUNE_DIST_M * CORNER_PRUNE_DIST_M;
        while pts.first().is_some_and(|p| {
            p.flags & detour::STRAIGHTPATH_OFFMESH_CONNECTION == 0
                && Vec2::new(p.pos[0] - start[0], p.pos[2] - start[2]).length_squared() <= prune_r2
        }) {
            pts.remove(0);
        }
        if let Some(i) = pts
            .iter()
            .position(|p| p.flags & detour::STRAIGHTPATH_OFFMESH_CONNECTION != 0)
        {
            pts.truncate(i + 1);
        }
        pts.truncate(max_corners);
        Ok::<_, NavError>(pts)
    })??;
    Ok(pts.into_iter().map(|p| m.world(ws, p.pos)).collect())
}

pub(crate) fn repair_corridor(
    b: &RecastBackend,
    start: &NavPoint,
    target: &Location,
    corridor: &[PolyRef],
    filter: &QueryFilter,
    max_nodes: u32,
) -> Result<Vec<PolyRef>, NavError> {
    let ws = &start.loc.worldspace;
    if &target.worldspace != ws {
        return Err(NavError::WrongWorldspace {
            query: ws.clone(),
            goal: target.worldspace.clone(),
        });
    }
    let (m, start_ref) = b
        .resolve(start.poly)
        .ok_or(NavError::UnknownPoly(start.poly))?;
    if &m.key.ws != ws {
        return Err(NavError::UnknownPoly(start.poly));
    }
    let f = DtFilter(filter);
    let existing: Vec<detour::PolyRef> = corridor
        .iter()
        .copied()
        .filter_map(|p| {
            let (tag, r) = refs::split(p);
            (tag == m.tag).then_some(r)
        })
        .collect();
    let start_pos = m.local(start.loc.pos);
    let end_pos = m.local(target.pos);
    let max_path = b.cfg.max_path;
    let res = b.with_query(m, |q| {
        if !q.is_valid_poly_ref(start_ref, &f) {
            return Err(NavError::UnknownPoly(start.poly));
        }
        let valid_existing: Vec<detour::PolyRef> = existing
            .into_iter()
            .filter(|&r| q.is_valid_poly_ref(r, &f))
            .collect();
        let Some(&end_ref) = valid_existing.last() else {
            return Ok(vec![start_ref]);
        };
        q.init_sliced_find_path(start_ref, end_ref, &start_pos, &end_pos, &f, 0)
            .map_err(backend)?;
        q.update_sliced_find_path(max_nodes.max(1) as usize, &f)
            .map_err(backend)?;
        let mut res = Vec::new();
        q.finalize_sliced_find_path_partial(&f, &valid_existing, &mut res, max_path)
            .map_err(backend)?;
        Ok::<_, NavError>(res)
    })??;
    Ok(res.into_iter().map(|r| m.poly(r)).collect())
}
