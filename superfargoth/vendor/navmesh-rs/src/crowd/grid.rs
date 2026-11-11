//! Hashed spatial grid, translated from DetourProximityGrid.cpp; see NOTICE.
use super::*;
use crate::common::floorf;
#[derive(Clone, Copy, Default)]
struct Item {
    x: i32,
    y: i32,
    id: u16,
    next: usize,
}
/// Bounded spatial hash with upstream insertion and query order.
/// Coordinates outside upstream's signed 16-bit cell range are rejected.
pub struct ProximityGrid {
    inv: f32,
    pool: Vec<Item>,
    buckets: Vec<usize>,
    head: usize,
    bounds: [i32; 4],
}
impl ProximityGrid {
    /// Allocates the grid. The pool must fit upstream's 16-bit indices.
    pub fn new(ws: &mut Workspace, pool_size: usize, cell_size: f32) -> Result<Self, Error> {
        ws.begin();
        Self::new_in(ws, pool_size, cell_size)
    }
    pub(crate) fn new_in(
        ws: &mut Workspace,
        pool_size: usize,
        cell_size: f32,
    ) -> Result<Self, Error> {
        positive(cell_size)?;
        if pool_size == 0 || pool_size > 65535 {
            return Err(Error::InvalidParameter("grid pool_size"));
        }
        Ok(Self {
            inv: 1. / cell_size,
            pool: ws.vec(pool_size, Item::default())?,
            buckets: ws.vec(pool_size.next_power_of_two(), usize::MAX)?,
            head: 0,
            bounds: [65535, 65535, -65535, -65535],
        })
    }
    /// Clears the grid without reallocating.
    pub fn clear(&mut self) {
        self.head = 0;
        self.buckets.fill(usize::MAX);
        self.bounds = [65535, 65535, -65535, -65535];
    }
    fn hash(&self, x: i32, y: i32) -> usize {
        (x.wrapping_mul(73856093) ^ y.wrapping_mul(19349663)) as usize & (self.buckets.len() - 1)
    }
    fn cells(&self, b: [f32; 4]) -> Result<[i32; 4], Error> {
        if b.iter().any(|v| !v.is_finite()) || b[0] > b[2] || b[1] > b[3] {
            return Err(Error::InvalidParameter("grid bounds"));
        }
        let mut c = [0; 4];
        for i in 0..4 {
            let v = floorf(b[i] * self.inv);
            if !(-32768.0..=32767.0).contains(&v) {
                return Err(Error::FormatLimit("proximity cell coordinates"));
            }
            c[i] = v as i32;
        }
        // Bound a query's cell visits, including empty cells.
        if (i64::from(c[2]) - i64::from(c[0]) + 1) * (i64::from(c[3]) - i64::from(c[1]) + 1)
            > 1_000_000
        {
            return Err(Error::LimitExceeded);
        }
        Ok(c)
    }
    pub(crate) fn cell_work(&self, b: [f32; 4]) -> Result<u64, Error> {
        let [x0, y0, x1, y1] = self.cells(b)?;
        Ok((i64::from(x1) - i64::from(x0) + 1) as u64 * (i64::from(y1) - i64::from(y0) + 1) as u64)
    }
    /// Adds an item to every touched cell.
    /// Returns [`Error::BufferTooSmall`] without changing the grid if the pool
    /// cannot hold every touched cell.
    pub fn add_item(&mut self, id: u16, b: [f32; 4]) -> Result<(), Error> {
        let [x0, y0, x1, y1] = self.cells(b)?;
        let count = self.cell_work(b)?;
        if count > (self.pool.len() - self.head) as u64 {
            return Err(Error::BufferTooSmall);
        }
        self.bounds = [
            self.bounds[0].min(x0),
            self.bounds[1].min(y0),
            self.bounds[2].max(x1),
            self.bounds[3].max(y1),
        ];
        for y in y0..=y1 {
            for x in x0..=x1 {
                let h = self.hash(x, y);
                let i = self.head;
                self.head += 1;
                self.pool[i] = Item {
                    x,
                    y,
                    id,
                    next: self.buckets[h],
                };
                self.buckets[h] = i;
            }
        }
        Ok(())
    }
    /// Queries unique ids in upstream cell and hash-chain order.
    /// Charges every cell and visited hash entry to `ws`, including collisions.
    /// Stops at the first extra unique id if `ids` is full, returning `ids.len()`.
    pub fn query_items(
        &self,
        ws: &mut Workspace,
        b: [f32; 4],
        ids: &mut [u16],
    ) -> Result<usize, Error> {
        ws.begin();
        self.query_items_in(ws, b, ids)
    }
    pub(crate) fn query_items_in(
        &self,
        ws: &mut Workspace,
        b: [f32; 4],
        ids: &mut [u16],
    ) -> Result<usize, Error> {
        let [x0, y0, x1, y1] = self.cells(b)?;
        ws.work(self.cell_work(b)?)?;
        let mut n = 0;
        for y in y0..=y1 {
            for x in x0..=x1 {
                let mut i = self.buckets[self.hash(x, y)];
                while i != usize::MAX {
                    ws.work(1)?;
                    let item = self.pool[i];
                    if item.x == x && item.y == y && !ids[..n].contains(&item.id) {
                        if n == ids.len() {
                            return Ok(n);
                        }
                        ids[n] = item.id;
                        n += 1;
                    }
                    i = item.next;
                }
            }
        }
        Ok(n)
    }
    // Full candidate collection for corrected nearest-neighbour queries. A
    // caller-owned stamp table avoids rescanning an ever-growing output slice
    // for each occurrence of an id in the spatial hash. The default capped
    // query deliberately retains upstream's path and ordering.
    pub(crate) fn query_all_in(
        &self,
        ws: &mut Workspace,
        b: [f32; 4],
        ids: &mut [u16],
        stamps: &mut [u16],
        stamp: u16,
    ) -> Result<usize, Error> {
        let [x0, y0, x1, y1] = self.cells(b)?;
        ws.work(self.cell_work(b)?)?;
        let mut n = 0;
        for y in y0..=y1 {
            for x in x0..=x1 {
                let mut i = self.buckets[self.hash(x, y)];
                while i != usize::MAX {
                    ws.work(1)?;
                    let item = self.pool[i];
                    if item.x == x && item.y == y {
                        let entry = stamps
                            .get_mut(usize::from(item.id))
                            .ok_or(Error::BufferTooSmall)?;
                        if *entry != stamp {
                            *entry = stamp;
                            *ids.get_mut(n).ok_or(Error::BufferTooSmall)? = item.id;
                            n += 1;
                        }
                    }
                    i = item.next;
                }
            }
        }
        Ok(n)
    }
    /// Current occupied cell bounds, or the upstream empty sentinel bounds.
    pub fn bounds(&self) -> [i32; 4] {
        self.bounds
    }
    /// Number of entries at a cell.
    pub fn item_count_at(&self, x: i32, y: i32) -> usize {
        let mut i = self.buckets[self.hash(x, y)];
        let mut n = 0;
        while i != usize::MAX {
            let v = self.pool[i];
            if v.x == x && v.y == y {
                n += 1;
            }
            i = v.next;
        }
        n
    }
}
