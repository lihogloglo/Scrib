//! Cache and obstacle lifecycle (DetourTileCache.cpp).
use super::{Layer, LayerCompressor, LayerHeader, RawCompressor};
use super::{builder, layer};
use crate::common::{cosf, floorf, sinf};
use crate::detour::{self, NavMesh, NavMeshCreateParams};
use crate::{Error, Limits, Usage, Workspace};
use alloc::vec::Vec;
const NONE: usize = usize::MAX;
const MAX_TOUCHED: usize = 8;
const MAX_REQUESTS: usize = 64;
/// Opaque salted compressed-tile handle (always 32 bits).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CompressedTileRef(u32);
impl CompressedTileRef {
    /// Upstream handle bits.
    pub fn bits(self) -> u32 {
        self.0
    }
}
/// Opaque salted obstacle handle (always 32 bits).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ObstacleRef(u32);
impl ObstacleRef {
    /// Upstream handle bits.
    pub fn bits(self) -> u32 {
        self.0
    }
}
/// Cache configuration. Construct with `new`, then customize public settings.
///
/// ```compile_fail
/// use navmesh_rs::tilecache::TileCacheParams;
/// let p = TileCacheParams::new(32, 32, 0.3, 0.2, 128, 128);
/// let TileCacheParams { orig, cs, ch, width, height, walkable_height,
///     walkable_radius, walkable_climb, max_simplification_error, max_tiles,
///     max_obstacles } = p;
/// ```
#[non_exhaustive]
#[derive(Clone, Debug)]
pub struct TileCacheParams {
    /// Grid origin.
    pub orig: [f32; 3],
    /// Horizontal cell size.
    pub cs: f32,
    /// Vertical cell size.
    pub ch: f32,
    /// Tile width in cells.
    pub width: u8,
    /// Tile depth in cells.
    pub height: u8,
    /// Agent height in world units.
    pub walkable_height: f32,
    /// Agent radius in world units.
    pub walkable_radius: f32,
    /// Agent climb in world units.
    pub walkable_climb: f32,
    /// Contour simplification tolerance in cells.
    pub max_simplification_error: f32,
    /// Compressed tile capacity.
    pub max_tiles: usize,
    /// Obstacle capacity (at most 65536).
    pub max_obstacles: usize,
}
impl TileCacheParams {
    /// Creates a cache configuration with demo agent settings.
    pub fn new(
        width: u8,
        height: u8,
        cs: f32,
        ch: f32,
        max_tiles: usize,
        max_obstacles: usize,
    ) -> Self {
        Self {
            orig: [0.0; 3],
            cs,
            ch,
            width,
            height,
            walkable_height: 2.0,
            walkable_radius: 0.6,
            walkable_climb: 0.9,
            max_simplification_error: 1.3,
            max_tiles,
            max_obstacles,
        }
    }
    fn validate(&self) -> Result<(), Error> {
        if self.width == 0
            || self.height == 0
            || self.max_tiles == 0
            || self.max_tiles > (1 << 22)
            || self.max_obstacles == 0
            || self.max_obstacles > 65536
            || self.orig.iter().any(|v| !v.is_finite())
            || ![self.cs, self.ch, self.walkable_height]
                .iter()
                .all(|v| v.is_finite() && *v > 0.0)
            || ![
                self.walkable_radius,
                self.walkable_climb,
                self.max_simplification_error,
            ]
            .iter()
            .all(|v| v.is_finite() && *v >= 0.0)
            || !(self.cs * f32::from(self.width)).is_finite()
            || !(self.cs * f32::from(self.height)).is_finite()
        {
            return Err(Error::InvalidParameter("tile cache parameters"));
        }
        voxel(self.walkable_climb / self.ch)?;
        Ok(())
    }
}
/// Shapes accepted by the cache. Derived bounds are validated at insertion.
#[non_exhaustive]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ObstacleShape {
    /// Upright cylinder.
    Cylinder {
        /// Bottom centre.
        pos: [f32; 3],
        /// Radius.
        radius: f32,
        /// Height.
        height: f32,
    },
    /// Axis-aligned box.
    Box {
        /// Minimum bounds.
        bmin: [f32; 3],
        /// Maximum bounds.
        bmax: [f32; 3],
    },
    /// Box rotated around the y axis.
    OrientedBox {
        /// Centre.
        center: [f32; 3],
        /// Positive half extents.
        half_extents: [f32; 3],
        /// Rotation, radians.
        yaw: f32,
    },
}
impl ObstacleShape {
    /// Broad-phase bounds, using upstream's oriented-box expansion.
    pub fn bounds(&self) -> ([f32; 3], [f32; 3]) {
        match *self {
            Self::Cylinder {
                pos,
                radius,
                height,
            } => (
                [pos[0] - radius, pos[1], pos[2] - radius],
                [pos[0] + radius, pos[1] + height, pos[2] + radius],
            ),
            Self::Box { bmin, bmax } => (bmin, bmax),
            Self::OrientedBox {
                center: c,
                half_extents: h,
                ..
            } => {
                let r = 1.41 * h[0].max(h[2]);
                (
                    [c[0] - r, c[1] - h[1], c[2] - r],
                    [c[0] + r, c[1] + h[1], c[2] + r],
                )
            }
        }
    }
    fn validate(&self) -> Result<(), Error> {
        match self {
            Self::Cylinder {
                pos,
                radius,
                height,
            } => {
                if pos.iter().any(|v| !v.is_finite())
                    || !radius.is_finite()
                    || *radius <= 0.0
                    || !height.is_finite()
                    || *height <= 0.0
                {
                    return Err(Error::InvalidParameter("cylinder"));
                }
            }
            Self::Box { .. } => {}
            Self::OrientedBox {
                center,
                half_extents,
                yaw,
            } => {
                if center.iter().any(|v| !v.is_finite())
                    || half_extents.iter().any(|v| !v.is_finite() || *v <= 0.0)
                    || !yaw.is_finite()
                {
                    return Err(Error::InvalidParameter("oriented box"));
                }
            }
        }
        let (a, b) = self.bounds();
        bounds(&a, &b)?;
        Ok(())
    }
    /// Upstream shape code: cylinder 0, box 1, oriented box 2.
    pub fn type_code(&self) -> u8 {
        match self {
            Self::Cylinder { .. } => 0,
            Self::Box { .. } => 1,
            Self::OrientedBox { .. } => 2,
        }
    }
}
/// Observable obstacle lifecycle.
///
/// ```compile_fail
/// use navmesh_rs::tilecache::ObstacleState;
/// fn code(s: ObstacleState) -> u8 {
///     match s { ObstacleState::Processing => 1, ObstacleState::Processed => 2,
///         ObstacleState::Removing => 3 }
/// }
/// ```
#[non_exhaustive]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ObstacleState {
    /// Queued or rebuilding.
    Processing,
    /// Fully applied.
    Processed,
    /// Removal queued or rebuilding.
    Removing,
}
impl ObstacleState {
    /// Upstream state code.
    pub fn code(self) -> u8 {
        match self {
            Self::Processing => 1,
            Self::Processed => 2,
            Self::Removing => 3,
        }
    }
}
/// Read-only occupant state. Empty slots are returned as `None`.
#[derive(Clone, Debug)]
pub struct Obstacle {
    shape: ObstacleShape,
    state: ObstacleState,
    salt: u16,
    touched: [usize; MAX_TOUCHED],
    count: usize,
    pending: usize,
    remove_queued: bool,
}
impl Obstacle {
    /// Shape.
    pub fn shape(&self) -> ObstacleShape {
        self.shape
    }
    /// Lifecycle state.
    pub fn state(&self) -> ObstacleState {
        self.state
    }
    /// Salt of this occupant.
    pub fn salt(&self) -> u16 {
        self.salt
    }
}
/// Application hook for areas, flags and optional off-mesh links.
pub trait MeshProcess {
    /// Map walkable area ids and fill flags before tile creation.
    fn process(&self, areas: &mut [u8], flags: &mut [u16]);
    /// Supply optional tile-creation settings, such as off-mesh connection slices.
    fn configure<'a>(&'a self, _params: &mut NavMeshCreateParams<'a>) {}
}
/// Walkable area 63 becomes ground (0), and all polygons receive flag 1.
#[derive(Clone, Copy, Debug, Default)]
pub struct DefaultMeshProcess;
impl MeshProcess for DefaultMeshProcess {
    fn process(&self, areas: &mut [u8], flags: &mut [u16]) {
        for (a, f) in areas.iter_mut().zip(flags) {
            if *a == 63 {
                *a = 0;
            }
            *f = 1;
        }
    }
}
#[derive(Clone, Default)]
struct TileSlot {
    salt: u32,
    data: Option<Vec<u8>>,
    header: Option<LayerHeader>,
    next: usize,
    head: usize,
}
#[derive(Clone, Copy)]
struct Association {
    tile: usize,
    obstacle: usize,
    prev: usize,
    next: usize,
    pending: bool,
}
impl Default for Association {
    fn default() -> Self {
        Self {
            tile: NONE,
            obstacle: NONE,
            prev: NONE,
            next: NONE,
            pending: false,
        }
    }
}
/// Incremental tile cache. One `update` rebuilds at most one tile.
///
/// Calls reset the reported usage. Requests and reverse tile/obstacle indices
/// avoid a scan of the whole obstacle pool for each rebuilt tile. Errors preserve
/// the front rebuild for retry; earlier completed rebuilds remain applied.
pub struct TileCache<C = RawCompressor, P = DefaultMeshProcess> {
    params: TileCacheParams,
    compressor: C,
    process: P,
    ws: Workspace,
    tiles: Vec<TileSlot>,
    buckets: Vec<usize>,
    free_tiles: Vec<usize>,
    tile_bits: u32,
    salt_mask: u32,
    obstacles: Vec<Option<Obstacle>>,
    salts: Vec<u16>,
    free_obstacles: Vec<usize>,
    associations: Vec<Association>,
    requests: Vec<(usize, bool)>,
    queue: Vec<usize>,
    queue_pos: usize,
    queued: Vec<bool>,
}
fn work(ws: &mut Workspace, n: usize) -> Result<(), Error> {
    ws.work(u64::try_from(n).map_err(|_| Error::SizeOverflow)?)
}
fn bounds(a: &[f32; 3], b: &[f32; 3]) -> Result<(), Error> {
    if !(0..3).all(|i| a[i].is_finite() && b[i].is_finite() && a[i] <= b[i]) {
        return Err(Error::InvalidInput("bounds"));
    }
    Ok(())
}
fn voxel(v: f32) -> Result<i32, Error> {
    if !v.is_finite() || v < -2147483648.0 || v >= 2147483648.0 {
        return Err(Error::FormatLimit("voxel coordinate"));
    }
    Ok(v as i32)
} // Range checked before the non-wrapping float conversion.
fn hash(x: i32, y: i32, mask: usize) -> usize {
    let x = u32::from_ne_bytes(x.to_ne_bytes());
    let y = u32::from_ne_bytes(y.to_ne_bytes());
    usize::try_from(
        x.wrapping_mul(0x8da6b343)
            .wrapping_add(y.wrapping_mul(0xd8163841)),
    )
    .unwrap()
        & mask
}
impl<C: LayerCompressor, P: MeshProcess> TileCache<C, P> {
    /// Allocates bounded tile, obstacle, request and reverse-index pools.
    pub fn new(
        params: TileCacheParams,
        compressor: C,
        process: P,
        limits: Limits,
    ) -> Result<Self, Error> {
        params.validate()?;
        let mut ws = Workspace::with_limits(limits);
        let nt = params.max_tiles;
        let no = params.max_obstacles;
        work(
            &mut ws,
            nt.checked_add(no.checked_mul(8).ok_or(Error::SizeOverflow)?)
                .ok_or(Error::SizeOverflow)?,
        )?;
        let tile_bits = nt.next_power_of_two().trailing_zeros();
        let salt_mask = (1u32 << (32 - tile_bits).min(31)) - 1;
        let mut tiles = ws.vec(nt, TileSlot::default())?;
        for t in &mut tiles {
            t.salt = 1;
            t.next = NONE;
            t.head = NONE;
        }
        let mut ft = ws.vec_with_capacity(nt)?;
        ft.extend((0..nt).rev());
        let mut fo = ws.vec_with_capacity(no)?;
        fo.extend((0..no).rev());
        Ok(Self {
            params,
            compressor,
            process,
            tiles,
            buckets: ws.vec((nt / 4).max(1).next_power_of_two(), NONE)?,
            free_tiles: ft,
            tile_bits,
            salt_mask,
            obstacles: ws.vec(no, None)?,
            salts: ws.vec(no, 1)?,
            free_obstacles: fo,
            associations: ws.vec(no * 8, Association::default())?,
            requests: ws.vec_with_capacity(MAX_REQUESTS)?,
            queue: ws.vec_with_capacity(nt)?,
            queue_pos: 0,
            queued: ws.vec(nt, false)?,
            ws,
        })
    }
    /// Configuration.
    pub fn params(&self) -> &TileCacheParams {
        &self.params
    }
    /// Last call's resource use, including helpers.
    pub fn usage(&self) -> Usage {
        self.ws.usage()
    }
    /// Changes limits for subsequent calls.
    pub fn set_limits(&mut self, limits: Limits) {
        self.ws.set_limits(limits);
    }
    fn tile_ref(&self, i: usize) -> CompressedTileRef {
        CompressedTileRef((self.tiles[i].salt << self.tile_bits) | u32::try_from(i).unwrap())
    }
    fn tile_index(&self, r: CompressedTileRef) -> Result<usize, Error> {
        let i = usize::try_from(r.0 & ((1 << self.tile_bits) - 1)).unwrap();
        if r.0 == 0
            || i >= self.tiles.len()
            || self.tiles[i].data.is_none()
            || self.tiles[i].salt != r.0 >> self.tile_bits
        {
            return Err(Error::InvalidRef);
        }
        Ok(i)
    }
    fn obstacle_ref(&self, i: usize) -> ObstacleRef {
        ObstacleRef((u32::from(self.salts[i]) << 16) | u32::try_from(i).unwrap())
    }
    fn obstacle_index(&self, r: ObstacleRef) -> Result<usize, Error> {
        let i = usize::try_from(r.0 & 65535).unwrap();
        if r.0 == 0
            || i >= self.obstacles.len()
            || self.obstacles[i].is_none()
            || u32::from(self.salts[i]) != r.0 >> 16
        {
            return Err(Error::InvalidRef);
        }
        Ok(i)
    }
    /// Looks up an occupant by handle.
    pub fn obstacle(&self, r: ObstacleRef) -> Option<&Obstacle> {
        self.obstacle_index(r)
            .ok()
            .and_then(|i| self.obstacles[i].as_ref())
    }
    /// Looks up an occupant by slot, together with its salted handle.
    pub fn obstacle_at(&self, i: usize) -> Option<(ObstacleRef, &Obstacle)> {
        self.obstacles
            .get(i)?
            .as_ref()
            .map(|o| (self.obstacle_ref(i), o))
    }
    /// Header of a resident compressed tile.
    pub fn tile_header(&self, r: CompressedTileRef) -> Option<&LayerHeader> {
        self.tile_index(r)
            .ok()
            .and_then(|i| self.tiles[i].header.as_ref())
    }
    /// Original compressed bytes.
    pub fn tile_data(&self, r: CompressedTileRef) -> Option<&[u8]> {
        self.tile_index(r)
            .ok()
            .and_then(|i| self.tiles[i].data.as_deref())
    }
    /// Validates the entire layer before accepting ownership.
    pub fn add_tile(&mut self, data: Vec<u8>) -> Result<CompressedTileRef, Error> {
        self.ws.begin();
        if !self.requests.is_empty() || self.queue_pos < self.queue.len() {
            return Err(Error::InvalidParameter(
                "tile mutation during obstacle work",
            ));
        }
        let l = layer::decode(&mut self.ws, &self.compressor, &data)?;
        let h = l.header;
        if h.width != self.params.width || h.height != self.params.height {
            return Err(Error::InvalidInput("layer dimensions differ from cache"));
        }
        let b = hash(h.tx, h.ty, self.buckets.len() - 1);
        let mut i = self.buckets[b];
        let mut count = 0;
        while i != NONE {
            work(&mut self.ws, 1)?;
            let a = self.tiles[i].header.as_ref().unwrap();
            if a.tx == h.tx && a.ty == h.ty {
                count += 1;
                if a.tlayer == h.tlayer {
                    return Err(Error::TileSlotOccupied);
                }
            }
            i = self.tiles[i].next;
        }
        if count == 32 {
            return Err(Error::FormatLimit("layers per tile"));
        }
        self.ws.charge_bytes::<u8>(data.len())?;
        let i = self.free_tiles.pop().ok_or(Error::NoFreeTile)?;
        let salt = self.tiles[i].salt;
        self.tiles[i] = TileSlot {
            salt,
            data: Some(data),
            header: Some(h),
            next: self.buckets[b],
            head: NONE,
        };
        self.buckets[b] = i;
        Ok(self.tile_ref(i))
    }
    /// Removes a layer when it has no obstacle associations or queued rebuilds.
    pub fn remove_tile(&mut self, r: CompressedTileRef) -> Result<Vec<u8>, Error> {
        self.ws.begin();
        let i = self.tile_index(r)?;
        if !self.requests.is_empty() || self.queue_pos < self.queue.len() {
            return Err(Error::InvalidParameter(
                "tile mutation during obstacle work",
            ));
        }
        if self.tiles[i].head != NONE || self.queued[i] {
            return Err(Error::InvalidParameter("tile has obstacle work"));
        }
        let (b, prev) = self.removal_chain(i)?;
        Ok(self.remove_checked(i, b, prev))
    }

    fn removal_chain(&mut self, i: usize) -> Result<(usize, usize), Error> {
        let h = self.tiles[i].header.as_ref().ok_or(Error::InvalidRef)?;
        let b = hash(h.tx, h.ty, self.buckets.len() - 1);
        let mut k = self.buckets[b];
        let mut prev = NONE;
        while k != i {
            work(&mut self.ws, 1)?;
            prev = k;
            k = self.tiles[k].next;
        }
        Ok((b, prev))
    }

    fn remove_checked(&mut self, i: usize, b: usize, prev: usize) -> Vec<u8> {
        if prev == NONE {
            self.buckets[b] = self.tiles[i].next;
        } else {
            self.tiles[prev].next = self.tiles[i].next;
        }
        let data = self.tiles[i].data.take().unwrap_or_default();
        let salt = (self.tiles[i].salt + 1) & self.salt_mask;
        self.tiles[i] = TileSlot {
            salt: salt.max(1),
            next: NONE,
            head: NONE,
            ..TileSlot::default()
        };
        self.free_tiles.push(i);
        data
    }
    /// Evicts a layer for streaming (not in recastnavigation) and returns its bytes.
    /// Unlike [`TileCache::remove_tile`], a layer with obstacle associations may
    /// go: the obstacles stay in world space and only their association with this
    /// layer is dropped; [`TileCache::load_tile`] restores it. Refused while
    /// obstacle requests or rebuilds are pending (drain [`TileCache::update`]
    /// first). The navmesh tile built from the layer is the caller's to unload.
    pub fn unload_tile(&mut self, r: CompressedTileRef) -> Result<Vec<u8>, Error> {
        self.ws.begin();
        let i = self.tile_index(r)?;
        if !self.requests.is_empty() || self.queue_pos < self.queue.len() {
            return Err(Error::InvalidParameter(
                "tile mutation during obstacle work",
            ));
        }
        let mut count = 0;
        let mut a = self.tiles[i].head;
        while a != NONE {
            work(&mut self.ws, 1)?;
            count += 1;
            a = self.associations[a].next;
        }
        work(&mut self.ws, count)?;
        if self.queued[i] {
            return Err(Error::InvalidParameter("tile has obstacle work"));
        }
        let (b, prev) = self.removal_chain(i)?;
        let mut a = self.tiles[i].head;
        while a != NONE {
            let next = self.associations[a].next;
            self.drop_association(a);
            a = next;
        }
        Ok(self.remove_checked(i, b, prev))
    }

    /// Removes association `a` from its tile's list and from its obstacle's touched
    /// set, keeping the obstacle's entries dense (`touched[..count]` at
    /// associations `obstacle * 8 ..`).
    fn drop_association(&mut self, a: usize) {
        let e = self.associations[a];
        let o = e.obstacle;
        self.unlink(a);
        let Some(ob) = self.obstacles.get_mut(o).and_then(|x| x.as_mut()) else {
            return;
        };
        let j = a - o * MAX_TOUCHED;
        let last = ob.count.saturating_sub(1);
        if ob.count == 0 || j > last {
            return;
        }
        if j != last {
            ob.touched[j] = ob.touched[last];
            let from = o * MAX_TOUCHED + last;
            let m = self.associations[from];
            if m.tile != NONE {
                if m.prev == NONE {
                    self.tiles[m.tile].head = a;
                } else {
                    self.associations[m.prev].next = a;
                }
                if m.next != NONE {
                    self.associations[m.next].prev = a;
                }
            }
            self.associations[a] = m;
            self.associations[from] = Association::default();
        }
        ob.touched[last] = NONE;
        ob.count = last;
    }

    /// Adds a layer for streaming (not in recastnavigation): [`TileCache::add_tile`]
    /// plus re-association of every live obstacle overlapping the layer, and the
    /// layer queued for rebuild so [`TileCache::update`] builds its navmesh tile
    /// with them. Allowed while rebuilds are queued, refused while obstacle
    /// requests are pending. [`Error::BufferTooSmall`] (nothing added) when an
    /// overlapping obstacle already touches the maximum number of layers.
    pub fn load_tile(&mut self, data: Vec<u8>) -> Result<CompressedTileRef, Error> {
        if !self.requests.is_empty() {
            return Err(Error::InvalidParameter(
                "tile mutation during obstacle work",
            ));
        }
        // Run add_tile's checks against a drained queue, then restore it.
        let (queue, pos) = (core::mem::take(&mut self.queue), self.queue_pos);
        self.queue_pos = 0;
        let overlaps = (|| {
            let l = layer::decode(&mut self.ws, &self.compressor, &data)?;
            let h = l.header;
            work(&mut self.ws, self.obstacles.len())?;
            let mut v = self.ws.vec_with_capacity(self.obstacles.len())?;
            for (o, ob) in self.obstacles.iter().enumerate() {
                let Some(ob) = ob else { continue };
                if ob.state == ObstacleState::Removing {
                    continue;
                }
                let (a, b) = ob.shape.bounds();
                if self.layer_overlaps(&h, &a, &b) {
                    if ob.count >= MAX_TOUCHED {
                        return Err(Error::BufferTooSmall);
                    }
                    v.push(o);
                }
            }
            Ok(v)
        })();
        let added = overlaps.and_then(|v| self.add_tile(data).map(|r| (r, v)));
        self.queue = queue;
        self.queue_pos = pos;
        let (r, overlaps) = added?;
        let i = self.tile_index(r)?;
        for o in overlaps {
            let Some(ob) = self.obstacles[o].as_mut() else {
                continue;
            };
            let j = ob.count;
            ob.touched[j] = i;
            ob.count += 1;
            let a = o * MAX_TOUCHED + j;
            let next = self.tiles[i].head;
            self.associations[a] = Association {
                tile: i,
                obstacle: o,
                prev: NONE,
                next,
                pending: false,
            };
            if next != NONE {
                self.associations[next].prev = a;
            }
            self.tiles[i].head = a;
        }
        if !self.queued[i] {
            self.queue.push(i);
            self.queued[i] = true;
        }
        Ok(r)
    }

    /// Whether the box `a..b` overlaps the used area of a layer (the test of
    /// upstream's `queryTiles`).
    fn layer_overlaps(&self, h: &LayerHeader, a: &[f32; 3], b: &[f32; 3]) -> bool {
        let cs = self.params.cs;
        let lo = [
            h.bmin[0] + f32::from(h.minx) * cs,
            h.bmin[1],
            h.bmin[2] + f32::from(h.miny) * cs,
        ];
        let hi = [
            h.bmin[0] + (f32::from(h.maxx) + 1.0) * cs,
            h.bmax[1],
            h.bmin[2] + (f32::from(h.maxy) + 1.0) * cs,
        ];
        (0..3).all(|j| a[j] <= hi[j] && b[j] >= lo[j])
    }

    /// All layers at a coordinate, in upstream's reverse insertion order.
    pub fn tiles_at(&mut self, x: i32, y: i32) -> Result<Vec<CompressedTileRef>, Error> {
        self.ws.begin();
        let mut out = self.ws.vec_with_capacity(32)?;
        let mut k = self.buckets[hash(x, y, self.buckets.len() - 1)];
        while k != NONE {
            work(&mut self.ws, 1)?;
            let h = self.tiles[k].header.as_ref().unwrap();
            if h.tx == x && h.ty == y {
                out.push(self.tile_ref(k));
            }
            k = self.tiles[k].next;
        }
        Ok(out)
    }
    fn query(&mut self, a: &[f32; 3], b: &[f32; 3], out: &mut [usize]) -> Result<usize, Error> {
        bounds(a, b)?;
        let p = &self.params;
        let tw = f32::from(p.width) * p.cs;
        let th = f32::from(p.height) * p.cs;
        let x0 = voxel(floorf((a[0] - p.orig[0]) / tw))?;
        let x1 = voxel(floorf((b[0] - p.orig[0]) / tw))?;
        let y0 = voxel(floorf((a[2] - p.orig[2]) / th))?;
        let y1 = voxel(floorf((b[2] - p.orig[2]) / th))?;
        let nx =
            u64::try_from(i64::from(x1) - i64::from(x0) + 1).map_err(|_| Error::SizeOverflow)?;
        let ny =
            u64::try_from(i64::from(y1) - i64::from(y0) + 1).map_err(|_| Error::SizeOverflow)?;
        self.ws
            .work(nx.checked_mul(ny).ok_or(Error::SizeOverflow)?)?;
        let mut n = 0;
        for y in y0..=y1 {
            for x in x0..=x1 {
                let mut k = self.buckets[hash(x, y, self.buckets.len() - 1)];
                while k != NONE {
                    work(&mut self.ws, 1)?;
                    let h = self.tiles[k].header.as_ref().unwrap();
                    if h.tx == x && h.ty == y {
                        let lo = [
                            h.bmin[0] + f32::from(h.minx) * p.cs,
                            h.bmin[1],
                            h.bmin[2] + f32::from(h.miny) * p.cs,
                        ];
                        let hi = [
                            h.bmin[0] + (f32::from(h.maxx) + 1.0) * p.cs,
                            h.bmax[1],
                            h.bmin[2] + (f32::from(h.maxy) + 1.0) * p.cs,
                        ];
                        if (0..3).all(|j| a[j] <= hi[j] && b[j] >= lo[j]) {
                            if n == out.len() {
                                return Err(Error::BufferTooSmall);
                            }
                            out[n] = k;
                            n += 1;
                        }
                    }
                    k = self.tiles[k].next;
                }
            }
        }
        Ok(n)
    }
    /// Queries tight layer bounds. Capacity exhaustion is an error, never silent truncation.
    pub fn query_tiles(
        &mut self,
        a: &[f32; 3],
        b: &[f32; 3],
        max: usize,
    ) -> Result<Vec<CompressedTileRef>, Error> {
        self.ws.begin();
        let mut idx = self.ws.vec(max.min(self.tiles.len()), 0usize)?;
        let n = self.query(a, b, &mut idx)?;
        let mut out = self.ws.vec_with_capacity(n)?;
        for i in &idx[..n] {
            out.push(self.tile_ref(*i));
        }
        Ok(out)
    }
    /// Enqueues a validated shape, rejecting more than eight touched layers atomically.
    pub fn add_obstacle(&mut self, shape: ObstacleShape) -> Result<ObstacleRef, Error> {
        self.ws.begin();
        shape.validate()?;
        if self.requests.len() == MAX_REQUESTS || self.free_obstacles.is_empty() {
            return Err(Error::BufferTooSmall);
        }
        let (a, b) = shape.bounds();
        let mut touched = [NONE; 8];
        let n = self.query(&a, &b, &mut touched)?;
        let i = self.free_obstacles.pop().unwrap();
        self.obstacles[i] = Some(Obstacle {
            shape,
            state: ObstacleState::Processing,
            salt: self.salts[i],
            touched,
            count: n,
            pending: 0,
            remove_queued: false,
        });
        self.requests.push((i, true));
        Ok(self.obstacle_ref(i))
    }
    /// Enqueues removal. Stale, fabricated and already-removing handles are errors.
    pub fn remove_obstacle(&mut self, r: ObstacleRef) -> Result<(), Error> {
        self.ws.begin();
        let i = self.obstacle_index(r)?;
        if self.obstacles[i].as_ref().unwrap().remove_queued {
            return Err(Error::InvalidParameter("obstacle already removing"));
        }
        if self.requests.len() == MAX_REQUESTS {
            return Err(Error::BufferTooSmall);
        }
        self.obstacles[i].as_mut().unwrap().remove_queued = true;
        self.requests.push((i, false));
        Ok(())
    }
    fn unlink(&mut self, a: usize) {
        let e = self.associations[a];
        if e.tile == NONE {
            return;
        }
        if e.prev == NONE {
            self.tiles[e.tile].head = e.next;
        } else {
            self.associations[e.prev].next = e.next;
        }
        if e.next != NONE {
            self.associations[e.next].prev = e.prev;
        }
        self.associations[a] = Association::default();
    }
    fn finish(&mut self, i: usize) {
        if self.obstacles[i].as_ref().unwrap().state == ObstacleState::Removing {
            for a in i * 8..i * 8 + 8 {
                self.unlink(a);
            }
            self.obstacles[i] = None;
            self.salts[i] = self.salts[i].wrapping_add(1).max(1);
            self.free_obstacles.push(i);
        } else {
            self.obstacles[i].as_mut().unwrap().state = ObstacleState::Processed;
        }
    }
    fn process_requests(&mut self) -> Result<(), Error> {
        // Charge the entire bounded request phase before mutation.
        work(&mut self.ws, self.requests.len() * 18)?;
        for r in 0..self.requests.len() {
            let (i, add) = self.requests[r];
            if add {
                let o = self.obstacles[i].as_mut().unwrap();
                let touched = o.touched;
                let n = o.count;
                o.pending = n;
                for (j, &t) in touched[..n].iter().enumerate() {
                    let a = i * 8 + j;
                    let next = self.tiles[t].head;
                    self.associations[a] = Association {
                        tile: t,
                        obstacle: i,
                        prev: NONE,
                        next,
                        pending: true,
                    };
                    if next != NONE {
                        self.associations[next].prev = a;
                    }
                    self.tiles[t].head = a;
                    if !self.queued[t] {
                        self.queue.push(t);
                        self.queued[t] = true;
                    }
                }
            } else {
                let o = self.obstacles[i].as_mut().unwrap();
                o.state = ObstacleState::Removing;
                o.pending = o.count;
                for j in 0..o.count {
                    let t = o.touched[j];
                    self.associations[i * 8 + j].pending = true;
                    if !self.queued[t] {
                        self.queue.push(t);
                        self.queued[t] = true;
                    }
                }
            }
            // Zero-touch processing/removal completes even with no rebuild. Upstream leaves it stuck.
            if self.obstacles[i].as_ref().unwrap().pending == 0 {
                // A following remove request for this slot must still see the occupant.
                if add {
                    self.obstacles[i].as_mut().unwrap().state = ObstacleState::Processed;
                } else {
                    self.finish(i);
                }
            }
        }
        self.requests.clear();
        Ok(())
    }
    /// Rebuilds at most one queued layer. Returns true once requests and rebuilds are drained.
    /// `dt` is validated even though upstream does not use it.
    pub fn update(&mut self, dt: f32, nav: &mut NavMesh) -> Result<bool, Error> {
        self.ws.begin();
        if !dt.is_finite() || dt < 0.0 {
            return Err(Error::InvalidParameter("update dt"));
        }
        if self.queue_pos == self.queue.len() {
            self.queue.clear();
            self.queue_pos = 0;
            self.process_requests()?;
        }
        if self.queue_pos < self.queue.len() {
            let t = self.queue[self.queue_pos];
            // Upstream scans every slot, including newly added obstacles whose
            // requests are deferred by the existing queue (pending == 0).
            // Reserve slot, pending-list and completion/unlink work before mutation.
            work(&mut self.ws, self.obstacles.len() * 26)?;
            self.rebuild(t, nav)?;
            for i in 0..self.obstacles.len() {
                let Some(o) = self.obstacles[i].as_mut() else {
                    continue;
                };
                if matches!(o.state, ObstacleState::Processing | ObstacleState::Removing) {
                    for j in 0..o.count {
                        let a = i * 8 + j;
                        if self.associations[a].pending && o.touched[j] == t {
                            self.associations[a].pending = false;
                            o.pending -= 1;
                            break;
                        }
                    }
                    if o.pending == 0 {
                        self.finish(i);
                    }
                }
            }
            self.queued[t] = false;
            self.queue_pos += 1;
        }
        Ok(self.queue_pos == self.queue.len() && self.requests.is_empty())
    }
    /// Builds tile data without mutating the navmesh (also useful for serialization).
    pub fn build_tile_data(&mut self, r: CompressedTileRef) -> Result<Option<Vec<u8>>, Error> {
        self.ws.begin();
        let i = self.tile_index(r)?;
        self.build(i)
    }
    fn build(&mut self, i: usize) -> Result<Option<Vec<u8>>, Error> {
        let mut l = layer::decode(
            &mut self.ws,
            &self.compressor,
            self.tiles[i].data.as_ref().unwrap(),
        )?;
        let mut a = self.tiles[i].head;
        while a != NONE {
            work(&mut self.ws, 1)?;
            let e = self.associations[a];
            let o = self.obstacles[e.obstacle].as_ref().unwrap();
            if o.state != ObstacleState::Removing {
                mark(
                    &mut self.ws,
                    &mut l,
                    self.params.cs,
                    self.params.ch,
                    o.shape,
                    0,
                )?;
            }
            a = e.next;
        }
        let climb = voxel(self.params.walkable_climb / self.params.ch)?;
        builder::regions(&mut self.ws, &mut l, climb)?;
        let c = builder::contours(
            &mut self.ws,
            &l,
            climb,
            self.params.max_simplification_error,
        )?;
        let mut m = builder::poly_mesh(&mut self.ws, &c)?;
        if m.areas.is_empty() {
            return Ok(None);
        }
        self.process.process(&mut m.areas, &mut m.flags);
        let h = &l.header;
        let p = &self.params;
        let mut params = NavMeshCreateParams {
            verts: &m.verts,
            vert_count: i32::try_from(m.verts.len() / 3).map_err(|_| Error::SizeOverflow)?,
            polys: &m.polys,
            poly_flags: &m.flags,
            poly_areas: &m.areas,
            poly_count: i32::try_from(m.areas.len()).map_err(|_| Error::SizeOverflow)?,
            nvp: 6,
            tile_x: h.tx,
            tile_y: h.ty,
            tile_layer: h.tlayer,
            bmin: h.bmin,
            bmax: h.bmax,
            walkable_height: p.walkable_height,
            walkable_radius: p.walkable_radius,
            walkable_climb: p.walkable_climb,
            cs: p.cs,
            ch: p.ch,
            build_bv_tree: false,
            ..NavMeshCreateParams::default()
        };
        self.process.configure(&mut params);
        // Reserve a conservative bound for builder outputs and temporary arrays before entering Detour.
        let bound = nav_build_bound(&params)?;
        self.ws.charge_bytes::<u8>(bound)?;
        work(&mut self.ws, bound)?;
        detour::create_nav_mesh_data(&params).map(Some)
    }
    fn rebuild(&mut self, i: usize, nav: &mut NavMesh) -> Result<(), Error> {
        let data = self.build(i)?;
        let h = self.tiles[i].header.as_ref().unwrap();
        if let Some(k) = nav.tile_index_at(h.tx, h.ty, h.tlayer) {
            nav.remove_tile(nav.tile_ref(k))?;
        }
        if let Some(data) = data {
            nav.add_tile(data)?;
        }
        Ok(())
    }
    /// Rebuilds one layer immediately, including currently applied obstacles.
    pub fn build_nav_mesh_tile(
        &mut self,
        r: CompressedTileRef,
        nav: &mut NavMesh,
    ) -> Result<(), Error> {
        self.ws.begin();
        let i = self.tile_index(r)?;
        self.rebuild(i, nav)
    }
    /// Rebuilds layers at a coordinate in upstream order.
    pub fn build_nav_mesh_tiles_at(
        &mut self,
        x: i32,
        y: i32,
        nav: &mut NavMesh,
    ) -> Result<(), Error> {
        self.ws.begin();
        let mut k = self.buckets[hash(x, y, self.buckets.len() - 1)];
        while k != NONE {
            work(&mut self.ws, 1)?;
            let h = self.tiles[k].header.as_ref().unwrap();
            if h.tx == x && h.ty == y {
                self.rebuild(k, nav)?;
            }
            k = self.tiles[k].next;
        }
        Ok(())
    }
}
fn nav_build_bound(p: &NavMeshCreateParams<'_>) -> Result<usize, Error> {
    p.verts
        .len()
        .checked_mul(32)
        .and_then(|n| n.checked_add(p.polys.len().checked_mul(256)?))
        .and_then(|n| n.checked_add(p.off_mesh_con_verts.len().checked_mul(512)?))
        .and_then(|n| n.checked_add(1024))
        .ok_or(Error::SizeOverflow)
}
fn mark(
    ws: &mut Workspace,
    l: &mut Layer,
    cs: f32,
    ch: f32,
    shape: ObstacleShape,
    area: u8,
) -> Result<(), Error> {
    let orig = l.header.bmin;
    let ics = 1.0 / cs;
    let ich = 1.0 / ch;
    let (a, b) = shape.bounds();
    let w = i32::from(l.header.width);
    let h = i32::from(l.header.height);
    let (cx, cz, aux, xhalf, zhalf) = match shape {
        ObstacleShape::OrientedBox {
            center: c,
            half_extents: he,
            yaw,
        } => {
            let ca = cosf(yaw * 0.5);
            let sa = sinf(-yaw * 0.5);
            (
                (c[0] - orig[0]) * ics,
                (c[2] - orig[2]) * ics,
                [ca * sa, ca * ca - 0.5],
                he[0] * ics + 0.5,
                he[2] * ics + 0.5,
            )
        }
        _ => (0.0, 0.0, [0.0; 2], 0.0, 0.0),
    };
    let (x0, x1, z0, z1) = if let ObstacleShape::OrientedBox {
        half_extents: he, ..
    } = shape
    {
        let r = 1.41 * he[0].max(he[2]);
        (
            voxel(floorf(cx - r * ics))?,
            voxel(floorf(cx + r * ics))?,
            voxel(floorf(cz - r * ics))?,
            voxel(floorf(cz + r * ics))?,
        )
    } else {
        (
            voxel(floorf((a[0] - orig[0]) * ics))?,
            voxel(floorf((b[0] - orig[0]) * ics))?,
            voxel(floorf((a[2] - orig[2]) * ics))?,
            voxel(floorf((b[2] - orig[2]) * ics))?,
        )
    };
    let y0 = voxel(floorf((a[1] - orig[1]) * ich))?;
    let y1 = voxel(floorf((b[1] - orig[1]) * ich))?;
    if x1 < 0 || z1 < 0 || x0 >= w || z0 >= h {
        return Ok(());
    }
    let (x0, x1, z0, z1) = (x0.max(0), x1.min(w - 1), z0.max(0), z1.min(h - 1));
    work(
        ws,
        usize::try_from((x1 - x0 + 1) * (z1 - z0 + 1)).map_err(|_| Error::SizeOverflow)?,
    )?;
    for z in z0..=z1 {
        for x in x0..=x1 {
            match shape {
                ObstacleShape::Cylinder { pos, radius, .. } => {
                    let r = radius / cs + 0.5;
                    let px = (pos[0] - orig[0]) * ics;
                    let pz = (pos[2] - orig[2]) * ics;
                    let dx = x as f32 + 0.5 - px;
                    let dz = z as f32 + 0.5 - pz;
                    if dx * dx + dz * dz > r * r {
                        continue;
                    }
                }
                ObstacleShape::OrientedBox { .. } => {
                    let xx = 2.0 * (x as f32 - cx);
                    let zz = 2.0 * (z as f32 - cz);
                    let xr = aux[1] * xx + aux[0] * zz;
                    let zr = aux[1] * zz - aux[0] * xx;
                    if xr > xhalf || xr < -xhalf || zr > zhalf || zr < -zhalf {
                        continue;
                    }
                }
                _ => {}
            }
            let i = usize::try_from(x + z * w).unwrap();
            let y = i32::from(l.heights[i]);
            if y >= y0 && y <= y1 {
                l.areas[i] = area;
            }
        }
    }
    Ok(())
}

/// Marks cells covered by a shape using the pinned voxel rasterization rules.
/// Area zero carves; ids 1..=63 remain walkable. Region state is invalidated.
pub fn mark_obstacle_area(
    ws: &mut Workspace,
    layer: &mut Layer,
    cs: f32,
    ch: f32,
    shape: ObstacleShape,
    area: u8,
) -> Result<(), Error> {
    ws.begin();
    if !cs.is_finite() || cs <= 0.0 || !ch.is_finite() || ch <= 0.0 || area > 63 {
        return Err(Error::InvalidParameter("area marking settings"));
    }
    shape.validate()?;
    mark(ws, layer, cs, ch, shape, area)?;
    layer.regs.fill(255);
    layer.reg_count = 0;
    layer.regions_built = false;
    Ok(())
}
