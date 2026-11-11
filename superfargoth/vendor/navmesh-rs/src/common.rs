//! Errors, limits, the workspace and the floating-point helpers shared by every module.

use alloc::vec::Vec;
use core::fmt;

/// Why a call failed.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub enum Error {
    /// A parameter is out of its valid range (the message names it).
    InvalidParameter(&'static str),
    /// Input data is malformed: NaN or infinite coordinates, out-of-range indices,
    /// mismatched array lengths.
    InvalidInput(&'static str),
    /// A size computation overflowed.
    SizeOverflow,
    /// The call would exceed [`Limits::max_bytes`] or [`Limits::max_work`].
    LimitExceeded,
    /// An allocation failed.
    AllocationFailed,
    /// The result does not fit an upstream format limit (for example more than
    /// 65,535 vertices in a polygon mesh, or more than 255 spans in a column).
    FormatLimit(&'static str),
    /// A navmesh query found nothing to return (upstream's plain `DT_FAILURE`), for
    /// example a null polygon reference or no polygon to pick a random point on.
    Failed,
    /// A polygon or tile reference is stale, out of range or of the wrong kind.
    InvalidRef,
    /// Navmesh tile bytes are malformed.
    InvalidTileData(TileDataError),
    /// Stored tile state (polygon flags and areas) was rejected.
    InvalidTileState(TileStateError),
    /// A tile already occupies the tile location.
    TileSlotOccupied,
    /// The navmesh has no free tile slot (or the requested slot is in use).
    NoFreeTile,
    /// Advancing a tile slot salt would reuse an old reference generation.
    SaltExhausted,
    /// Ear clipping was incomplete in strict mode (not in recastnavigation).
    PartialTriangulation {
        /// Number of incomplete contours encountered.
        failed_contours: usize,
        /// Number of expected triangles not emitted.
        missing_triangles: usize,
    },
    /// An output buffer is too small.
    BufferTooSmall,
    /// A tile lies outside a keyed navmesh's grid (not in recastnavigation).
    TileOutsideGrid,
    /// A resumable or sliced search was continued after a tile it had touched was
    /// unloaded or removed (not in recastnavigation); restart the search.
    MeshChanged,
    /// The call was stopped by its [`CancelToken`] (not in recastnavigation).
    Cancelled,
}

/// Why stored tile state was rejected on restore.
#[non_exhaustive]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub enum TileStateError {
    /// The buffer is shorter than the tile's state needs.
    TooShort,
    /// The magic number is not `'DNMS'`.
    WrongMagic,
    /// The version does not match the state format being read.
    WrongVersion,
    /// The state was stored for a different tile (or a different incarnation of the
    /// same slot): its tile ref does not equal the tile's current ref.
    /// Streaming unload blobs also reject a mismatching content hash.
    WrongTile,
}

/// Why navmesh tile bytes were rejected.
#[non_exhaustive]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub enum TileDataError {
    /// The magic number is not `'DNAV'`.
    WrongMagic,
    /// The version is not 7.
    WrongVersion,
    /// A count in the header is negative or the sections do not fit the buffer.
    BadSize,
    /// The tile has more polygons than the navmesh's poly-ref bits can address.
    TooManyPolys,
    /// A polygon's vertex, neighbour or type field is out of range.
    BadPoly,
    /// A detail mesh, detail vertex or detail triangle index is out of range.
    BadDetail,
    /// A bounding-volume node is out of range.
    BadBvTree,
    /// An off-mesh connection is out of range.
    BadOffMeshConnection,
    /// A coordinate is NaN or infinite.
    NonFinite,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::InvalidParameter(p) => write!(f, "invalid parameter: {p}"),
            Error::InvalidInput(p) => write!(f, "invalid input: {p}"),
            Error::SizeOverflow => f.write_str("size overflow"),
            Error::LimitExceeded => f.write_str("limit exceeded"),
            Error::AllocationFailed => f.write_str("allocation failed"),
            Error::FormatLimit(p) => write!(f, "format limit: {p}"),
            Error::Failed => f.write_str("query failed"),
            Error::InvalidRef => f.write_str("invalid reference"),
            Error::InvalidTileData(r) => write!(f, "invalid tile data: {r:?}"),
            Error::InvalidTileState(r) => write!(f, "invalid tile state: {r:?}"),
            Error::TileSlotOccupied => f.write_str("tile location already occupied"),
            Error::SaltExhausted => f.write_str("tile slot salt exhausted"),
            Error::NoFreeTile => f.write_str("no free tile slot"),
            Error::PartialTriangulation {
                failed_contours,
                missing_triangles,
            } => write!(
                f,
                "partial triangulation: {failed_contours} contours, {missing_triangles} missing triangles"
            ),
            Error::BufferTooSmall => f.write_str("buffer too small"),
            Error::TileOutsideGrid => f.write_str("tile outside the keyed grid"),
            Error::MeshChanged => f.write_str("navmesh changed under the search"),
            Error::Cancelled => f.write_str("cancelled"),
        }
    }
}

/// A shared flag that stops work in progress (not in recastnavigation).
///
/// Clones share one flag. Setting it is irreversible; make a new token for the
/// next job. A [`Workspace`] given a token (through the bake job API) checks it
/// with one relaxed atomic load at every work charge and then fails the running
/// call with [`Error::Cancelled`].
///
/// ```
/// use navmesh_rs::CancelToken;
///
/// let token = CancelToken::new();
/// let other = token.clone();
/// assert!(!token.is_cancelled());
/// other.cancel();
/// assert!(token.is_cancelled());
/// ```
#[derive(Clone, Debug, Default)]
pub struct CancelToken(alloc::sync::Arc<core::sync::atomic::AtomicBool>);

impl CancelToken {
    /// A token that is not cancelled.
    pub fn new() -> Self {
        Self::default()
    }

    /// Cancels every clone of this token.
    pub fn cancel(&self) {
        self.0.store(true, core::sync::atomic::Ordering::Relaxed);
    }

    /// Whether [`cancel`](Self::cancel) has been called on any clone.
    pub fn is_cancelled(&self) -> bool {
        self.0.load(core::sync::atomic::Ordering::Relaxed)
    }
}

#[cfg(feature = "std")]
impl std::error::Error for Error {}

/// Per-call resource limits.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Limits {
    /// Largest number of bytes one call may allocate (output included).
    pub max_bytes: usize,
    /// Largest amount of work one call may do, counted in span, cell and vertex visits.
    pub max_work: u64,
    /// Largest navmesh tile, in bytes, a tile reader accepts.
    pub max_tile_bytes: usize,
    /// Largest polygon count of one navmesh tile.
    pub max_tile_polys: usize,
    /// Largest vertex count of one navmesh tile.
    pub max_tile_verts: usize,
    /// Largest link count of one navmesh tile.
    pub max_tile_links: usize,
    /// Largest detail mesh, detail vertex or detail triangle count of one navmesh tile.
    pub max_tile_detail: usize,
}

impl Limits {
    /// No limits.
    pub const UNLIMITED: Limits = Limits {
        max_bytes: usize::MAX,
        max_work: u64::MAX,
        max_tile_bytes: usize::MAX,
        max_tile_polys: usize::MAX,
        max_tile_verts: usize::MAX,
        max_tile_links: usize::MAX,
        max_tile_detail: usize::MAX,
    };
}

impl Default for Limits {
    fn default() -> Self {
        Limits::UNLIMITED
    }
}

/// Resources used by the last call.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Usage {
    /// Bytes allocated.
    pub bytes: usize,
    /// Work units done.
    pub work: u64,
}

/// Limits and accounting for one thread of calls.
///
/// Every stage starts by resetting the usage, allocates through the workspace
/// (fallibly, checked against [`Limits::max_bytes`]) and charges its work
/// (checked against [`Limits::max_work`]).
#[derive(Clone, Debug, Default)]
pub struct Workspace {
    limits: Limits,
    usage: Usage,
    /// Set only by the crate's own pipelines while every compact field they pass
    /// to a stage was built by this crate and not exposed to caller edits since.
    compact_trusted: bool,
    pub(crate) regions_removed_small: u32,
    /// Checked at every work charge when present.
    cancel: Option<CancelToken>,
}

impl Workspace {
    /// A workspace without limits.
    pub fn new() -> Self {
        Self::default()
    }

    /// A workspace with the given limits.
    pub fn with_limits(limits: Limits) -> Self {
        Workspace {
            limits,
            usage: Usage::default(),
            compact_trusted: false,
            regions_removed_small: 0,
            cancel: None,
        }
    }

    /// Stops calls on this workspace with [`Error::Cancelled`] once `token` is
    /// cancelled (checked at each work charge). `None` removes the token.
    pub fn set_cancel_token(&mut self, token: Option<CancelToken>) {
        self.cancel = token;
    }

    /// Whether the compact field passed to the running stage is known valid, so the
    /// whole-grid connection scan can be skipped (its work is still charged).
    #[inline]
    pub(crate) fn compact_trusted(&self) -> bool {
        self.compact_trusted
    }

    pub(crate) fn set_compact_trusted(&mut self, trusted: bool) {
        self.compact_trusted = trusted;
    }

    /// The limits applied to each call.
    pub fn limits(&self) -> Limits {
        self.limits
    }

    /// Changes the limits for later calls.
    pub fn set_limits(&mut self, limits: Limits) {
        self.limits = limits;
    }

    /// Resources used by the last call.
    pub fn usage(&self) -> Usage {
        self.usage
    }

    pub(crate) fn begin(&mut self) {
        self.usage = Usage::default();
    }

    /// Charges `n` work units.
    #[inline]
    pub(crate) fn work(&mut self, n: u64) -> Result<(), Error> {
        if let Some(token) = &self.cancel
            && token.is_cancelled()
        {
            return Err(Error::Cancelled);
        }
        self.usage.work = self.usage.work.saturating_add(n);
        if self.usage.work > self.limits.max_work {
            Err(Error::LimitExceeded)
        } else {
            Ok(())
        }
    }

    pub(crate) fn charge_bytes<T>(&mut self, n: usize) -> Result<(), Error> {
        let bytes = n
            .checked_mul(core::mem::size_of::<T>())
            .ok_or(Error::SizeOverflow)?;
        self.usage.bytes = self.usage.bytes.saturating_add(bytes);
        if self.usage.bytes > self.limits.max_bytes {
            Err(Error::LimitExceeded)
        } else {
            Ok(())
        }
    }

    /// An empty vector with room for `n` elements.
    pub(crate) fn vec_with_capacity<T>(&mut self, n: usize) -> Result<Vec<T>, Error> {
        self.charge_bytes::<T>(n)?;
        let mut v = Vec::new();
        v.try_reserve_exact(n)
            .map_err(|_| Error::AllocationFailed)?;
        Ok(v)
    }

    /// A vector of `n` copies of `value`.
    pub(crate) fn vec<T: Clone>(&mut self, n: usize, value: T) -> Result<Vec<T>, Error> {
        let mut v = self.vec_with_capacity(n)?;
        v.resize(n, value);
        Ok(v)
    }

    /// Grows `v` by `additional` elements of capacity, charging the bytes.
    pub(crate) fn reserve<T>(&mut self, v: &mut Vec<T>, additional: usize) -> Result<(), Error> {
        if v.capacity() - v.len() >= additional {
            return Ok(());
        }
        let want = v
            .len()
            .checked_add(additional)
            .ok_or(Error::SizeOverflow)?
            .max(v.capacity().saturating_mul(2));
        self.charge_bytes::<T>(want - v.capacity())?;
        v.try_reserve_exact(want - v.len())
            .map_err(|_| Error::AllocationFailed)
    }

    /// Pushes onto `v`, growing it through the workspace.
    #[inline]
    pub(crate) fn push<T>(&mut self, v: &mut Vec<T>, value: T) -> Result<(), Error> {
        if v.len() == v.capacity() {
            self.reserve(v, 1)?;
        }
        v.push(value);
        Ok(())
    }
}

// Stable merge sort with caller-owned, fallibly allocated scratch. Taking the
// left element on ties preserves the pinned glibc qsort order.
#[cfg(any(feature = "recast", feature = "detour"))]
pub(crate) fn stable_sort<T: Copy>(
    values: &mut [T],
    scratch: &mut [T],
    compare: impl Fn(&T, &T) -> core::cmp::Ordering + Copy,
) {
    if values.len() < 2 {
        return;
    }
    let mid = values.len() / 2;
    let len = values.len();
    let (left, right) = values.split_at_mut(mid);
    let (sl, sr) = scratch[..len].split_at_mut(mid);
    stable_sort(left, sl, compare);
    stable_sort(right, sr, compare);
    let (mut a, mut b) = (0, mid);
    for out in scratch.iter_mut().take(values.len()) {
        if a < mid
            && (b == values.len()
                || compare(&values[a], &values[b]) != core::cmp::Ordering::Greater)
        {
            *out = values[a];
            a += 1;
        } else {
            *out = values[b];
            b += 1;
        }
    }
    values.copy_from_slice(&scratch[..values.len()]);
}

// ---------------------------------------------------------------------------
// Floating-point helpers. sqrt, floor and ceil are exact IEEE operations on
// every implementation; cos comes from the platform libm with `std` and from the
// `libm` crate otherwise (or when the `libm` feature is on).

#[inline]
pub(crate) fn sqrtf(x: f32) -> f32 {
    #[cfg(all(feature = "std", not(feature = "libm")))]
    {
        x.sqrt()
    }
    #[cfg(feature = "libm")]
    {
        libm::sqrtf(x)
    }
}

#[inline]
pub(crate) fn floorf(x: f32) -> f32 {
    #[cfg(all(feature = "std", not(feature = "libm")))]
    {
        x.floor()
    }
    #[cfg(feature = "libm")]
    {
        libm::floorf(x)
    }
}

#[inline]
pub(crate) fn ceilf(x: f32) -> f32 {
    #[cfg(all(feature = "std", not(feature = "libm")))]
    {
        x.ceil()
    }
    #[cfg(feature = "libm")]
    {
        libm::ceilf(x)
    }
}

/// `floorf(x) as i32` without the library call: truncate, then step down for a
/// negative fraction. Saturates and maps NaN to zero exactly like the `as` cast.
#[inline(always)]
pub(crate) fn floor_i32(x: f32) -> i32 {
    let t = x as i32;
    if (t as f32) > x {
        t.saturating_sub(1)
    } else {
        t
    }
}

/// `ceilf(x) as i32` without the library call (see [`floor_i32`]).
#[inline(always)]
pub(crate) fn ceil_i32(x: f32) -> i32 {
    let t = x as i32;
    if (t as f32) < x {
        t.saturating_add(1)
    } else {
        t
    }
}

#[inline]
pub(crate) fn cosf(x: f32) -> f32 {
    #[cfg(all(feature = "std", not(feature = "libm")))]
    {
        x.cos()
    }
    #[cfg(feature = "libm")]
    {
        libm::cosf(x)
    }
}

/// `rcMin` for floats: `a < b ? a : b` (not `f32::min`, which treats NaN differently).
#[inline(always)]
pub(crate) fn fmin(a: f32, b: f32) -> f32 {
    if a < b { a } else { b }
}

/// `rcMax` for floats: `a > b ? a : b`.
#[inline(always)]
pub(crate) fn fmax(a: f32, b: f32) -> f32 {
    if a > b { a } else { b }
}

/// `rcSqr`.
#[inline(always)]
pub(crate) fn sqr(a: f32) -> f32 {
    a * a
}

/// `rcClamp` for integers.
#[inline(always)]
pub(crate) fn clamp(v: i32, lo: i32, hi: i32) -> i32 {
    if v < lo {
        lo
    } else if v > hi {
        hi
    } else {
        v
    }
}

#[inline(always)]
pub(crate) fn vsub(a: &[f32], b: &[f32]) -> [f32; 3] {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}

#[inline(always)]
pub(crate) fn vcross(v1: &[f32; 3], v2: &[f32; 3]) -> [f32; 3] {
    [
        v1[1] * v2[2] - v1[2] * v2[1],
        v1[2] * v2[0] - v1[0] * v2[2],
        v1[0] * v2[1] - v1[1] * v2[0],
    ]
}

#[inline(always)]
pub(crate) fn vnormalize(v: &mut [f32; 3]) {
    let d = 1.0 / sqrtf(sqr(v[0]) + sqr(v[1]) + sqr(v[2]));
    v[0] *= d;
    v[1] *= d;
    v[2] *= d;
}

#[inline(always)]
pub(crate) fn vmin(mn: &mut [f32; 3], v: &[f32]) {
    mn[0] = fmin(mn[0], v[0]);
    mn[1] = fmin(mn[1], v[1]);
    mn[2] = fmin(mn[2], v[2]);
}

#[inline(always)]
pub(crate) fn vmax(mx: &mut [f32; 3], v: &[f32]) {
    mx[0] = fmax(mx[0], v[0]);
    mx[1] = fmax(mx[1], v[1]);
    mx[2] = fmax(mx[2], v[2]);
}

#[inline(always)]
pub(crate) fn v3(s: &[f32]) -> [f32; 3] {
    [s[0], s[1], s[2]]
}

#[cfg(any(feature = "crowd", feature = "tilecache"))]
#[inline]
pub(crate) fn sinf(x: f32) -> f32 {
    #[cfg(all(feature = "std", not(feature = "libm")))]
    {
        x.sin()
    }
    #[cfg(feature = "libm")]
    {
        libm::sinf(x)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn integer_floor_and_ceil_match_the_library_functions() {
        let values = [
            f32::NEG_INFINITY,
            -3.0e9,
            -2_147_483_648.0,
            -2.5,
            -1.0,
            -0.5,
            -0.0,
            0.0,
            0.25,
            0.999_999_94,
            1.0,
            1.5,
            2_147_483_520.0,
            3.0e9,
            f32::INFINITY,
            f32::NAN,
        ];
        for x in values {
            assert_eq!(floor_i32(x), floorf(x) as i32, "floor {x}");
            assert_eq!(ceil_i32(x), ceilf(x) as i32, "ceil {x}");
        }
    }
}
