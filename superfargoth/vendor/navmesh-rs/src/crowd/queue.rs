//! Budgeted sliced path queue, translated from DetourPathQueue.cpp; see NOTICE.
use super::*;
use crate::detour::query::DetachedQuery;
struct Request {
    handle: u32,
    start: PolyRef,
    end: PolyRef,
    start_pos: V3,
    end_pos: V3,
    filter: usize,
    status: u32,
    keep: u8,
    path: Vec<PolyRef>,
}
/// An eight-slot FIFO of sliced path searches; results expire after two updates.
pub struct PathQueue {
    requests: Vec<Request>,
    head: usize,
    next: u32,
    max_path: usize,
    query: Option<DetachedQuery>,
}
impl PathQueue {
    /// Allocates queue paths and the sliced-search node pool.
    pub fn new(
        ws: &mut Workspace,
        nav: &NavMesh,
        max_path: usize,
        max_nodes: usize,
    ) -> Result<Self, Error> {
        ws.begin();
        Self::new_in(ws, nav, max_path, max_nodes)
    }
    pub(crate) fn new_in(
        ws: &mut Workspace,
        nav: &NavMesh,
        max_path: usize,
        max_nodes: usize,
    ) -> Result<Self, Error> {
        if max_path == 0 {
            return Err(Error::InvalidParameter("queue max_path"));
        }
        ws.work(1)?;
        // Query node pools, hashes and heap, including the small local pool.
        ws.charge_bytes::<u8>(
            max_nodes
                .checked_mul(128)
                .and_then(|v| v.checked_add(16384))
                .ok_or(Error::SizeOverflow)?,
        )?;
        let mut requests = ws.vec_with_capacity(8)?;
        for _ in 0..8 {
            requests.push(Request {
                handle: 0,
                start: 0,
                end: 0,
                start_pos: [0.; 3],
                end_pos: [0.; 3],
                filter: 0,
                status: 0,
                keep: 0,
                path: ws.vec_with_capacity(max_path)?,
            });
        }
        Ok(Self {
            requests,
            head: 0,
            next: 1,
            max_path,
            query: Some(DetachedQuery::new(nav, max_nodes)?),
        })
    }
    /// Selects the upstream corrections the queue's sliced searches apply
    /// ([`UpstreamFixes::SLICED_CROSS_SIDE`]). Takes effect for the next search
    /// step; set it before enqueueing for a consistent result.
    pub fn set_fixes(&mut self, fixes: UpstreamFixes) {
        if let Some(q) = self.query.as_mut() {
            q.set_fixes(fixes);
        }
    }
    /// Enqueues a search using an index into the filter array passed to `update`.
    /// Refs are checked against `nav`. A full queue returns [`Error::BufferTooSmall`].
    pub fn request(
        &mut self,
        nav: &NavMesh,
        start: PolyRef,
        end: PolyRef,
        start_pos: V3,
        end_pos: V3,
        filter: usize,
    ) -> Result<u32, Error> {
        finite(start_pos)?;
        finite(end_pos)?;
        if !nav.is_valid_poly_ref(start) || !nav.is_valid_poly_ref(end) {
            return Err(Error::InvalidRef);
        }
        if filter >= 16 {
            return Err(Error::InvalidParameter("queue filter"));
        }
        let Some(q) = self.requests.iter_mut().find(|r| r.handle == 0) else {
            return Err(Error::BufferTooSmall);
        };
        let h = self.next;
        self.next = self.next.wrapping_add(1);
        if self.next == 0 {
            self.next = 1;
        }
        q.handle = h;
        q.start = start;
        q.end = end;
        q.start_pos = start_pos;
        q.end_pos = end_pos;
        q.filter = filter;
        q.status = 0;
        q.keep = 0;
        q.path.clear();
        Ok(h)
    }
    /// Upstream status bits of a request; an unknown handle reports failure.
    pub fn status(&self, handle: u32) -> u32 {
        if handle == 0 {
            return Status::FAILURE;
        }
        self.requests
            .iter()
            .find(|r| r.handle == handle)
            .map_or(Status::FAILURE, |q| q.status)
    }
    /// Takes a completed result and releases its slot.
    pub fn take_result(&mut self, handle: u32, out: &mut Vec<PolyRef>) -> Result<Status, Error> {
        if handle == 0 {
            return Err(Error::InvalidRef);
        }
        let q = self
            .requests
            .iter_mut()
            .find(|q| q.handle == handle)
            .ok_or(Error::InvalidRef)?;
        if q.status & Status::FAILURE != 0 {
            q.handle = 0;
            q.status = 0;
            return Err(Error::Failed);
        }
        if q.status & (Status::SUCCESS | Status::FAILURE) == 0 {
            return Err(Error::Failed);
        }
        out.try_reserve(q.path.len())
            .map_err(|_| Error::AllocationFailed)?;
        out.clear();
        out.extend_from_slice(&q.path);
        let st = Status((q.status & 0x0ff_ffff) | Status::SUCCESS);
        q.handle = 0;
        q.status = 0;
        Ok(st)
    }
    /// Runs at most `max_iters` node expansions, retaining incomplete searches.
    pub fn update(
        &mut self,
        ws: &mut Workspace,
        nav: &NavMesh,
        max_iters: usize,
        filters: &[DefaultFilter; 16],
    ) -> Result<(), Error> {
        ws.begin();
        self.update_in(ws, nav, max_iters, filters)
    }
    pub(crate) fn update_in(
        &mut self,
        ws: &mut Workspace,
        nav: &NavMesh,
        max_iters: usize,
        filters: &[DefaultFilter; 16],
    ) -> Result<(), Error> {
        ws.work(max_iters as u64)?;
        let state = self.query.take().ok_or(Error::Failed)?;
        let mut query = state.attach(nav);
        let mut iters = max_iters;
        for _ in 0..8 {
            let q = &mut self.requests[self.head % 8];
            if q.handle == 0 {
                self.head = self.head.wrapping_add(1);
                continue;
            }
            if q.status & (Status::SUCCESS | Status::FAILURE) != 0 {
                q.keep += 1;
                if q.keep > 2 {
                    q.handle = 0;
                    q.status = 0;
                }
                self.head = self.head.wrapping_add(1);
                continue;
            }
            let f = &filters[q.filter];
            if q.status == 0 {
                q.status = query
                    .init_sliced_find_path(q.start, q.end, &q.start_pos, &q.end_pos, f, 0)
                    .map_or_else(|e| crate::detour::status_of(&e).0, |s| s.0);
            }
            if q.status & Status::IN_PROGRESS != 0 {
                match query.update_sliced_find_path(iters, f) {
                    Ok((s, n)) => {
                        q.status = s.0;
                        iters -= n;
                    }
                    Err(e) => q.status = crate::detour::status_of(&e).0,
                }
            }
            if q.status & Status::SUCCESS != 0 {
                q.status = query
                    .finalize_sliced_find_path(f, &mut q.path, self.max_path)
                    .map_or_else(|e| crate::detour::status_of(&e).0, |s| s.0);
            }
            if iters == 0 {
                break;
            }
            self.head = self.head.wrapping_add(1);
        }
        self.query = Some(query.detach());
        Ok(())
    }
}
