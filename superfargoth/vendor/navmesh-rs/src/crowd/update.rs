//! Ordered simulation phases, translated from DetourCrowd.cpp; see NOTICE.
use super::*;
use AgentState as S;
use TargetState as T;

/// Cuts an agent's corridor at its first polygon that is not resident and moves
/// the corridor's end to the point nearest the target on the last resident one.
fn trim_to_resident(a: &mut CrowdAgent, q: &NavMeshQuery<'_>) -> Result<(), Error> {
    let nav = q.nav();
    let path = a.corridor.path();
    let Some(keep) = path.iter().position(|&p| !nav.is_valid_poly_ref(p)) else {
        return Ok(());
    };
    if keep == 0 {
        let first = a.corridor.first_poly();
        return a.corridor.reset(first, a.npos);
    }
    let prefix: Vec<PolyRef> = path[..keep].to_vec();
    let target = q
        .closest_point_on_poly(prefix[keep - 1], &a.target_pos)
        .map_or(a.npos, |c| c.0);
    a.corridor.set_corridor(target, &prefix)?;
    a.boundary.reset();
    Ok(())
}

impl Crowd {
    fn check_paths(&mut self, q: &NavMeshQuery<'_>, dt: f32) -> Result<(), Error> {
        for &i in &self.active {
            let a = &mut self.agents[i];
            if a.state != S::Walking {
                continue;
            }
            a.replan_time += dt;
            let f = &self.filters[a.params.query_filter_type];
            let mut replan = false;
            let mut r = a.corridor.first_poly();
            let mut pos = a.npos;
            if !q.is_valid_poly_ref(r, f) {
                let (nr, np, _) = q.find_nearest_poly(&a.npos, &self.extents, f)?;
                r = nr;
                if r != 0 {
                    pos = np;
                }
                if r == 0 {
                    a.corridor.reset(0, pos)?;
                    a.partial = false;
                    a.boundary.reset();
                    a.state = S::Invalid;
                    continue;
                }
                a.corridor.fix_path_start(r, pos)?;
                a.boundary.reset();
                a.npos = pos;
                replan = true;
            }
            if matches!(a.target_state, T::None | T::Velocity) {
                continue;
            }
            // Streaming (not in recastnavigation): a target whose tile is unloaded
            // is waited for, not dropped.
            if a.target_state != T::Failed {
                if let Some(k) = a
                    .waiting_tile
                    .filter(|_| a.target_state == T::WaitingForTile)
                {
                    if q.nav().is_column_unloaded(k.x, k.y) {
                        trim_to_resident(a, q)?;
                        continue;
                    }
                }
                if let Some(k) =
                    a.corridor
                        .path()
                        .iter()
                        .find_map(|&r| match q.nav().ref_state(r) {
                            RefState::Unloaded(k) => Some(k),
                            _ => None,
                        })
                {
                    a.target_state = T::WaitingForTile;
                    a.waiting_tile = Some(k);
                    a.pathq = 0;
                    a.replan = false;
                    trim_to_resident(a, q)?;
                    continue;
                }
                match q.nav().ref_state(a.target_ref) {
                    RefState::Unloaded(k) => {
                        if a.target_state != T::WaitingForTile {
                            a.target_state = T::WaitingForTile;
                            a.waiting_tile = Some(k);
                            a.pathq = 0;
                            a.replan = false;
                        }
                        trim_to_resident(a, q)?;
                        continue;
                    }
                    _ if a.target_state == T::WaitingForTile => {
                        // Back (or replaced): plan again; a replaced target is
                        // relocated by the invalid-target branch below.
                        a.target_state = T::Requesting;
                        replan = true;
                    }
                    _ => {}
                }
            }
            if !matches!(a.target_state, T::None | T::Failed) {
                if !q.is_valid_poly_ref(a.target_ref, f) {
                    let (nr, np, _) = q.find_nearest_poly(&a.target_pos, &self.extents, f)?;
                    a.target_ref = nr;
                    if nr != 0 {
                        a.target_pos = np;
                    }
                    replan = true;
                }
                if a.target_ref == 0 {
                    a.corridor.reset(r, pos)?;
                    a.partial = false;
                    a.target_state = T::None;
                }
            }
            if !a.corridor.is_valid(10, q, f) {
                replan = true;
            }
            if a.target_state == T::Valid
                && a.replan_time > 1.
                && a.corridor.path.len() < 10
                && a.corridor.last_poly() != a.target_ref
            {
                replan = true;
            }
            if replan && a.target_state != T::None {
                a.pathq = 0;
                a.replan = true;
                a.target_state = if a.target_ref != 0 {
                    T::Requesting
                } else {
                    T::Failed
                };
            }
        }
        Ok(())
    }
    fn move_requests(
        &mut self,
        ws: &mut Workspace,
        nav: &NavMesh,
        q: &mut NavMeshQuery<'_>,
    ) -> Result<(), Error> {
        let mut queue = [0usize; 8];
        let mut nqueue = 0;
        for i in 0..self.agents.len() {
            let a = &mut self.agents[i];
            if !a.active || a.state == S::Invalid || matches!(a.target_state, T::None | T::Velocity)
            {
                continue;
            }
            let f = &self.filters[a.params.query_filter_type];
            if a.target_state == T::Requesting {
                let mut res = Vec::new();
                let mut pos = a.target_pos;
                let mut waiting = None;
                let result = (|| {
                    q.init_sliced_find_path(
                        a.corridor.first_poly(),
                        a.target_ref,
                        &a.npos,
                        &a.target_pos,
                        f,
                        0,
                    )?;
                    q.update_sliced_find_path(20, f)?;
                    let status = if a.replan {
                        q.finalize_sliced_find_path_partial(f, &a.corridor.path, &mut res, 32)?
                    } else {
                        q.finalize_sliced_find_path(f, &mut res, 32)?
                    };
                    if status.has_detail(Status::UNLOADED_BORDER) {
                        waiting = q.search_frontier().first().copied();
                    }
                    if let Some(&r) = res.last() {
                        if r != a.target_ref {
                            pos = q.closest_point_on_poly(r, &a.target_pos)?.0;
                        }
                    }
                    Ok::<(), Error>(())
                })();
                if result.is_err() || res.is_empty() {
                    res.clear();
                    res.push(a.corridor.first_poly());
                    pos = a.npos;
                }
                a.corridor.set_corridor(pos, &res)?;
                a.boundary.reset();
                a.partial = false;
                if let Some(k) = waiting {
                    a.target_state = T::WaitingForTile;
                    a.waiting_tile = Some(k);
                    a.pathq = 0;
                    a.replan = false;
                } else if res.last() == Some(&a.target_ref) {
                    a.target_state = T::Valid;
                    a.replan_time = 0.;
                } else {
                    a.target_state = T::WaitingForQueue;
                }
            }
            if a.target_state == T::WaitingForQueue {
                let time = a.replan_time;
                let at = if nqueue == 0 || time <= self.agents[queue[nqueue - 1]].replan_time {
                    nqueue
                } else {
                    (0..nqueue)
                        .find(|&j| time >= self.agents[queue[j]].replan_time)
                        .unwrap_or(nqueue)
                };
                if at < 8 {
                    let end = nqueue.min(7);
                    queue.copy_within(at..end, at + 1);
                    queue[at] = i;
                    nqueue = (nqueue + 1).min(8);
                }
            }
        }
        for &i in &queue[..nqueue] {
            let a = &mut self.agents[i];
            match self.pathq.request(
                nav,
                a.corridor.last_poly(),
                a.target_ref,
                a.corridor.target,
                a.target_pos,
                a.params.query_filter_type,
            ) {
                Ok(h) => {
                    a.pathq = h;
                    a.target_state = T::WaitingForPath;
                }
                // Capacity backpressure is observable in the agent's target state.
                Err(Error::BufferTooSmall) => {}
                Err(e) => return Err(e),
            }
        }
        self.pathq.update_in(ws, nav, 100, &self.filters)?;
        for a in &mut self.agents {
            if !a.active || a.target_state != T::WaitingForPath {
                continue;
            }
            let status = self.pathq.status(a.pathq);
            if status & Status::FAILURE != 0 {
                a.pathq = 0;
                a.target_state = if a.target_ref != 0 {
                    T::Requesting
                } else {
                    T::Failed
                };
                a.replan_time = 0.;
            } else if status & Status::SUCCESS != 0 {
                let mut res = Vec::new();
                let status = self.pathq.take_result(a.pathq, &mut res)?;
                a.partial = status.0 & Status::PARTIAL_RESULT != 0;
                let mut valid = !res.is_empty() && a.corridor.last_poly() == res[0];
                let npath = a.corridor.path.len();
                if valid {
                    if npath > 1 {
                        res.truncate(256 - (npath - 1));
                        let mut merged = Vec::new();
                        merged
                            .try_reserve(256)
                            .map_err(|_| Error::AllocationFailed)?;
                        merged.extend_from_slice(&a.corridor.path[..npath - 1]);
                        merged.extend_from_slice(&res);
                        res = merged;
                        let mut j = 0isize;
                        while j < res.len() as isize {
                            if j >= 1
                                && j + 1 < res.len() as isize
                                && res[(j - 1) as usize] == res[(j + 1) as usize]
                            {
                                res.drain((j - 1) as usize..(j + 1) as usize);
                                j -= 2;
                            }
                            j += 1;
                        }
                    }
                    let mut target = a.target_pos;
                    if res.last() != Some(&a.target_ref) {
                        match q.closest_point_on_poly(*res.last().ok_or(Error::Failed)?, &target) {
                            Ok((p, _)) => target = p,
                            Err(_) => valid = false,
                        }
                    }
                    if valid {
                        a.corridor.set_corridor(target, &res)?;
                        a.boundary.reset();
                        a.target_state = T::Valid;
                    }
                }
                if !valid {
                    a.target_state = T::Failed;
                }
                a.replan_time = 0.;
            }
        }
        Ok(())
    }
    /// [`Crowd::update`] under a work budget (not in recastnavigation): the workspace's
    /// work limit for the tick becomes the smaller of its own and what `budget` has
    /// left, and the work the tick charged is spent from `budget`.
    ///
    /// A tick is not resumable: its phases run in upstream's order and each sees the
    /// previous one's result. If `budget` cannot cover the up-front charge of
    /// `4096` units per agent (`Workspace::usage` after any tick shows the real
    /// cost) the outcome is [`Budgeted::Exhausted`] and the crowd is untouched. If
    /// it runs out later the outcome is also `Exhausted`, with the same caveat as a
    /// [`Error::LimitExceeded`] from `update`: earlier phases were applied, so the
    /// caller should discard that tick (and give the next one a larger budget or
    /// fewer agents). Within the budget the result equals `update`'s.
    pub fn update_budgeted(
        &mut self,
        ws: &mut Workspace,
        nav: &NavMesh,
        dt: f32,
        budget: &mut Budget,
    ) -> Result<Budgeted<()>, Error> {
        let saved = ws.limits();
        let allowed = saved.max_work.min(budget.remaining());
        ws.set_limits(Limits {
            max_work: allowed,
            ..saved
        });
        let result = self.update(ws, nav, dt);
        ws.set_limits(saved);
        let used = ws.usage().work;
        match result {
            Ok(()) => {
                budget.spend(used);
                Ok(Budgeted::Complete(()))
            }
            // The work counter went over the allowance: the budget (not the byte
            // limit) stopped the tick.
            Err(Error::LimitExceeded) if used > allowed => {
                budget.exhaust();
                Ok(Budgeted::Exhausted(()))
            }
            Err(e) => Err(e),
        }
    }

    /// Advances the crowd in upstream phase order. `dt` must be finite and
    /// nonnegative. Workspace limits bound allocation and charged simulation work.
    /// An error can leave earlier phases applied; discard that tick's result.
    pub fn update(&mut self, ws: &mut Workspace, nav: &NavMesh, dt: f32) -> Result<(), Error> {
        ws.begin();
        nonnegative(dt)?;
        // Reserve/query scratch and conservatively charge per-agent local query
        // work before mutating the simulation. Sampling charges its own work.
        ws.work((self.agents.len() as u64).saturating_mul(4096))?;
        ws.charge_bytes::<u8>(
            self.agents
                .len()
                .checked_mul(32768)
                .and_then(|n| n.checked_add(512 * 128 + 65536))
                .ok_or(Error::SizeOverflow)?,
        )?;
        let mut q = NavMeshQuery::new(nav, 512)?;
        self.velocity_samples = 0;
        self.active.clear();
        self.active.extend(
            self.agents
                .iter()
                .enumerate()
                .filter_map(|(i, a)| a.active.then_some(i)),
        );
        self.check_paths(&q, dt)?;
        self.move_requests(ws, nav, &mut q)?;
        let mut opt: Option<usize> = None;
        for &i in &self.active {
            let a = &mut self.agents[i];
            if a.state != S::Walking
                || matches!(a.target_state, T::None | T::Velocity)
                || a.params.update_flags & OPTIMIZE_TOPO == 0
            {
                continue;
            }
            a.topology_time += dt;
            let time = a.topology_time;
            if time >= 0.5 && opt.is_none_or(|j| time > self.agents[j].topology_time) {
                opt = Some(i);
            }
        }
        if let Some(i) = opt {
            let a = &mut self.agents[i];
            let _ = a
                .corridor
                .optimize_topology(&mut q, &self.filters[a.params.query_filter_type]);
            a.topology_time = 0.;
        }
        self.grid.clear();
        for (j, &i) in self.active.iter().enumerate() {
            let a = &self.agents[i];
            let p = a.npos;
            let r = a.params.radius;
            self.grid
                .add_item(j as u16, [p[0] - r, p[2] - r, p[0] + r, p[2] + r])?;
        }
        let all_candidates = self.fixes.contains(UpstreamFixes::NEAREST_NEIGHBOURS);
        let any_orca = self
            .active
            .iter()
            .any(|&i| matches!(self.agents[i].params.avoidance, AvoidanceMode::Orca(_)));
        // One id slot per active agent holds every candidate a query can return.
        let mut wide = if all_candidates || any_orca {
            ws.vec(self.active.len(), 0u16)?
        } else {
            Vec::new()
        };
        let mut candidate_stamps = if all_candidates || any_orca {
            ws.vec(self.active.len(), 0u16)?
        } else {
            Vec::new()
        };
        for &i in &self.active {
            let a = &mut self.agents[i];
            if a.state != S::Walking {
                continue;
            }
            let f = &self.filters[a.params.query_filter_type];
            let range = a.params.collision_query_range;
            if vdist_2d_sqr(&a.npos, &a.boundary.center) > sqr(range * 0.25)
                || !a.boundary.is_valid(&q, f)
            {
                a.boundary
                    .update(a.corridor.first_poly(), a.npos, range, &mut q, f)?;
            }
            let nearest = all_candidates || matches!(a.params.avoidance, AvoidanceMode::Orca(_));
            let neighbour_cap = match a.params.avoidance {
                AvoidanceMode::Orca(p) => {
                    usize::from(p.max_neighbours).min(self.active.len().saturating_sub(1))
                }
                _ => 6,
            };
            if a.neis.len() < neighbour_cap {
                let additional = neighbour_cap - a.neis.len();
                ws.reserve(&mut a.neis, additional)?;
                a.neis.resize(neighbour_cap, (0, 0.));
            }
            a.nneis = 0;
            let pos = a.npos;
            let height = a.params.height;
            let mut capped = [0u16; 32];
            let ids: &mut [u16] = if nearest { &mut wide } else { &mut capped };
            let bounds = [
                pos[0] - range,
                pos[2] - range,
                pos[0] + range,
                pos[2] + range,
            ];
            let n = if nearest {
                // For ORCA, a smaller complete query proves the same nearest
                // set once it contains the cap inside its inscribed circle.
                // Otherwise discard its stamps and query the full range. The
                // corrected sampled path keeps its exhaustive collection.
                let radius = (2. * a.params.radius * sqrtf(neighbour_cap as f32)).min(range);
                let narrow = matches!(a.params.avoidance, AvoidanceMode::Orca(_))
                    && neighbour_cap > 0
                    && radius < range;
                let mut n = 0;
                let mut enough = false;
                if narrow {
                    n = self.grid.query_all_in(
                        ws,
                        [
                            pos[0] - radius,
                            pos[2] - radius,
                            pos[0] + radius,
                            pos[2] + radius,
                        ],
                        ids,
                        &mut candidate_stamps,
                        (i + 1) as u16,
                    )?;
                    let mut inside = 0;
                    for &id in &ids[..n] {
                        ws.work(1)?;
                        let j = self.active[usize::from(id)];
                        let nei = &self.agents[j];
                        if i != j
                            && (pos[1] - nei.npos[1]).abs() < (height + nei.params.height) / 2.
                            && vdist_2d_sqr(&pos, &nei.npos) <= sqr(radius)
                        {
                            inside += 1;
                        }
                    }
                    enough = inside >= neighbour_cap;
                }
                if !enough {
                    for &id in &ids[..n] {
                        ws.work(1)?;
                        candidate_stamps[usize::from(id)] = 0;
                    }
                    n = self.grid.query_all_in(
                        ws,
                        bounds,
                        ids,
                        &mut candidate_stamps,
                        (i + 1) as u16,
                    )?;
                }
                n
            } else {
                self.grid.query_items_in(ws, bounds, ids)?
            };
            for &id in &ids[..n] {
                let j = self.active[id as usize];
                if i == j {
                    continue;
                }
                let nei = &self.agents[j];
                let mut diff = vsub(&pos, &nei.npos);
                if diff[1].abs() >= (height + nei.params.height) / 2. {
                    continue;
                }
                diff[1] = 0.;
                let dist = vlen_sqr(&diff);
                if dist > sqr(range) {
                    continue;
                }
                let a = &mut self.agents[i];
                let at = if nearest {
                    // Candidates arrive in grid order: order by (distance, index).
                    let kept = &a.neis[..a.nneis];
                    if kept
                        .last()
                        .is_some_and(|&(index, distance)| (distance, index) <= (dist, j))
                    {
                        a.nneis
                    } else {
                        kept.partition_point(|&(index, distance)| (distance, index) < (dist, j))
                    }
                } else {
                    insert_pos(&a.neis[..a.nneis], dist, |v| v.1)
                };
                if at < neighbour_cap {
                    let end = a.nneis.min(neighbour_cap - 1);
                    a.neis.copy_within(at..end, at + 1);
                    a.neis[at] = (j, dist);
                    a.nneis = (a.nneis + 1).min(neighbour_cap);
                }
            }
        }
        for &i in &self.active {
            let a = &mut self.agents[i];
            if a.state != S::Walking || matches!(a.target_state, T::None | T::Velocity) {
                continue;
            }
            let f = &self.filters[a.params.query_filter_type];
            a.corridor.find_corners(&q, &mut a.corners, 4)?;
            if a.params.update_flags & OPTIMIZE_VIS != 0 && !a.corners.is_empty() {
                let next = a.corners[1.min(a.corners.len() - 1)].pos;
                a.corridor.optimize_visibility(
                    next,
                    a.params.path_optimization_range,
                    &mut q,
                    f,
                )?;
            }
        }
        for &i in &self.active {
            let a = &mut self.agents[i];
            if a.state != S::Walking || matches!(a.target_state, T::None | T::Velocity) {
                continue;
            }
            if let Some(c) = a.corners.last() {
                if c.flags & STRAIGHTPATH_OFFMESH_CONNECTION != 0
                    && vdist_2d_sqr(&a.npos, &c.pos) < sqr(a.params.radius * 2.25)
                {
                    if let Ok((start, end, _)) =
                        a.corridor.move_over_offmesh_connection(c.r#ref, &q)
                    {
                        a.anim = Animation {
                            active: true,
                            init: a.npos,
                            start,
                            end,
                            t: 0.,
                            tmax: (vdist_2d(&start, &end) / a.params.max_speed) * 0.5,
                        };
                        a.state = S::Offmesh;
                        a.corners.clear();
                        a.nneis = 0;
                    }
                }
            }
        }
        for &i in &self.active {
            let a = &self.agents[i];
            if a.state != S::Walking || a.target_state == T::None {
                continue;
            }
            let mut dvel = [0.; 3];
            let speed;
            if a.target_state == T::Velocity {
                dvel = a.target_pos;
                speed = vlen(&a.target_pos);
            } else {
                if !a.corners.is_empty() {
                    let mut dir0 = vsub(&a.corners[0].pos, &a.npos);
                    dir0[1] = 0.;
                    if a.params.update_flags & ANTICIPATE_TURNS != 0 {
                        let mut dir1 = vsub(&a.corners[1.min(a.corners.len() - 1)].pos, &a.npos);
                        dir1[1] = 0.;
                        let len0 = vlen(&dir0);
                        let len1 = vlen(&dir1);
                        if len1 > 0.001 {
                            dir1 = vscale(&dir1, 1. / len1);
                        }
                        dvel = [
                            dir0[0] - dir1[0] * len0 * 0.5,
                            0.,
                            dir0[2] - dir1[2] * len0 * 0.5,
                        ];
                    } else {
                        dvel = dir0;
                    }
                    vnormalize(&mut dvel);
                }
                let slow = a.params.radius * 2.;
                let dist =
                    if let Some(c) = a.corners.last().filter(|c| c.flags & STRAIGHTPATH_END != 0) {
                        fmin(vdist_2d(&a.npos, &c.pos), slow)
                    } else {
                        slow
                    };
                speed = a.params.max_speed;
                dvel = vscale(&dvel, speed * (dist / slow));
            }
            if a.params.update_flags & SEPARATION != 0 {
                let range = a.params.collision_query_range;
                let inv = 1. / range;
                let mut w = 0.;
                let mut disp = [0.; 3];
                for &(j, _) in &a.neis[..a.nneis] {
                    let mut diff = vsub(&a.npos, &self.agents[j].npos);
                    diff[1] = 0.;
                    let ds = vlen_sqr(&diff);
                    if ds < 0.00001 || ds > sqr(range) {
                        continue;
                    }
                    let dist = sqrtf(ds);
                    let weight = a.params.separation_weight * (1. - sqr(dist * inv));
                    disp = vmad(&disp, &diff, weight / dist);
                    w += 1.;
                }
                if w > 0.0001 {
                    dvel = vmad(&dvel, &disp, 1. / w);
                    let ds = vlen_sqr(&dvel);
                    let desired = sqr(speed);
                    if ds > desired {
                        dvel = vscale(&dvel, desired / ds);
                    }
                }
            }
            self.agents[i].desired_speed = speed;
            self.agents[i].dvel = dvel;
        }
        let mut orca_scratch = OrcaScratch::default();
        let mut orca_neighbours = Vec::new();
        for &i in &self.active {
            let a = &self.agents[i];
            if a.state != S::Walking {
                continue;
            }
            let nvel = if a.params.update_flags & OBSTACLE_AVOIDANCE != 0
                && dt > 0.
                && let AvoidanceMode::Orca(p) = a.params.avoidance
            {
                orca_neighbours.clear();
                ws.reserve(&mut orca_neighbours, a.nneis)?;
                for &(j, _) in &a.neis[..a.nneis] {
                    orca_neighbours.push(OrcaAgent::crowd(&self.agents[j]));
                }
                let mut walls = [[0.; 4]; 8];
                for (out, &(_, s)) in walls.iter_mut().zip(&a.boundary.segments) {
                    *out = [s[0], s[2], s[3], s[5]];
                }
                let v = orca::velocity_in(
                    ws,
                    &OrcaAgent::crowd(a),
                    &orca_neighbours,
                    &walls[..a.boundary.segments.len()],
                    &p,
                    dt,
                    &mut orca_scratch,
                )?;
                [v[0], 0., v[1]]
            } else if a.params.update_flags & OBSTACLE_AVOIDANCE != 0
                && matches!(a.params.avoidance, AvoidanceMode::Sampled)
            {
                self.avoidance.reset();
                for &(j, _) in &a.neis[..a.nneis] {
                    let n = &self.agents[j];
                    self.avoidance
                        .add_circle(n.npos, n.params.radius, n.vel, n.dvel)?;
                }
                for &(_, s) in &a.boundary.segments {
                    if tri_area_2d(&a.npos, &s[..3], &s[3..]) < 0. {
                        continue;
                    }
                    self.avoidance.add_segment(v3(&s[..3]), v3(&s[3..]))?;
                }
                let (v, n) = self.avoidance.sample_velocity_adaptive_in(
                    ws,
                    a.npos,
                    a.params.radius,
                    a.desired_speed,
                    a.vel,
                    a.dvel,
                    &self.avoidance_params[a.params.obstacle_avoidance_type],
                )?;
                self.velocity_samples += n;
                v
            } else {
                a.dvel
            };
            self.agents[i].nvel = nvel;
        }
        for &i in &self.active {
            let a = &mut self.agents[i];
            if a.state != S::Walking {
                continue;
            }
            let max = a.params.max_acceleration * dt;
            let mut dv = vsub(&a.nvel, &a.vel);
            let ds = vlen(&dv);
            if ds > max {
                dv = vscale(&dv, max / ds);
            }
            a.vel = vadd(&a.vel, &dv);
            if matches!(a.params.avoidance, AvoidanceMode::Orca(_)) {
                // Acceleration interpolation can round back outside the disc.
                let v = orca::clip([a.vel[0], a.vel[2]], a.params.max_speed);
                a.vel = [v[0], 0., v[1]];
            }
            if vlen(&a.vel) > 0.0001 {
                a.npos = vmad(&a.npos, &a.vel, dt);
            } else {
                a.vel = [0.; 3];
            }
        }
        for _ in 0..4 {
            for &i in &self.active {
                let a = &self.agents[i];
                if a.state != S::Walking {
                    continue;
                }
                let mut disp = [0.; 3];
                let mut w = 0.;
                for &(j, _) in &a.neis[..a.nneis] {
                    let nei = &self.agents[j];
                    let mut diff = vsub(&a.npos, &nei.npos);
                    diff[1] = 0.;
                    let mut dist = vlen_sqr(&diff);
                    if dist > sqr(a.params.radius + nei.params.radius) {
                        continue;
                    }
                    dist = sqrtf(dist);
                    let mut pen = (a.params.radius + nei.params.radius) - dist;
                    if dist < 0.0001 {
                        diff = if i > j {
                            [-a.dvel[2], 0., a.dvel[0]]
                        } else {
                            [a.dvel[2], 0., -a.dvel[0]]
                        };
                        pen = 0.01;
                    } else {
                        pen = (1. / dist) * (pen * 0.5) * 0.7;
                    }
                    disp = vmad(&disp, &diff, pen);
                    w += 1.;
                }
                if w > 0.0001 {
                    disp = vscale(&disp, 1. / w);
                }
                self.agents[i].disp = disp;
            }
            for &i in &self.active {
                let a = &mut self.agents[i];
                if a.state == S::Walking {
                    a.npos = vadd(&a.npos, &a.disp);
                }
            }
        }
        for &i in &self.active {
            let a = &mut self.agents[i];
            if a.state != S::Walking {
                continue;
            }
            // Upstream keeps the corridor position if the local move fails.
            let _ =
                a.corridor
                    .move_position(a.npos, &mut q, &self.filters[a.params.query_filter_type]);
            a.npos = a.corridor.pos;
            if matches!(a.target_state, T::None | T::Velocity) {
                a.corridor.reset(a.corridor.first_poly(), a.npos)?;
                a.partial = false;
            }
        }
        for &i in &self.active {
            let a = &mut self.agents[i];
            let anim = &mut a.anim;
            if !anim.active {
                continue;
            }
            anim.t += dt;
            if anim.t > anim.tmax {
                anim.active = false;
                a.state = S::Walking;
                continue;
            }
            let ta = anim.tmax * 0.15;
            let tb = anim.tmax;
            a.npos = if anim.t < ta {
                vlerp(&anim.init, &anim.start, fclamp(anim.t / ta, 0., 1.))
            } else {
                vlerp(
                    &anim.start,
                    &anim.end,
                    fclamp((anim.t - ta) / (tb - ta), 0., 1.),
                )
            };
            a.vel = [0.; 3];
            a.dvel = [0.; 3];
        }
        for &i in &self.active {
            let a = &self.agents[i];
            finite(a.npos)?;
            finite(a.vel)?;
            finite(a.nvel)?;
            finite(a.dvel)?;
        }
        Ok(())
    }
}
