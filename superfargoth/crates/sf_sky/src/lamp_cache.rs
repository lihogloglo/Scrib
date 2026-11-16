//! Static lamp shadow cache, the Bevy-free policy (ROADMAP 1.25 CW-4, slice S1). Spec:
//! `docs/specs/lamp-shadow-cache.md` "Design"; the vendored mechanism (S2) and the Bevy wiring
//! in `sf_sky_bevy::lamp_cache` (S3) act on what this decides.
//!
//! A cached lamp keeps the depth of its static casters and draws only its movers over a copy
//! of it. Its *tiles* (cube faces today, [`TileLayout::CUBE`]; an atlas layout later, CW-13)
//! each go through [`FaceState`]: **Capture** (its static casters are drawn, cleared first, at
//! most [`LampCacheConfig::capture_faces_per_frame`] per frame from a FIFO queue, until the host
//! confirms the draw), **Clean** (its live layer holds static depth: no view, no work),
//! **Movers** (a mover is in it: copy the store into the live layer, draw the movers over it,
//! at the lamp's rate tier), **Linger** (the movers left: one last copy made it clean, its view
//! stays [`LampCacheConfig::mover_linger_s`]) and **Dirty** (a static changed: captured again
//! once [`LampCacheConfig::recapture_interval_s`] has passed since the last capture).
//!
//! Each frame the host admits and invalidates lamps ([`LampCache::admit`],
//! [`LampCache::invalidate`]), reports movers per lamp ([`LampCache::set_movers`]) and confirms
//! the captures the render world drew ([`LampCache::confirm_capture`]), then calls
//! [`LampCache::plan`]: per lamp the tiles to capture, to draw movers into (loading the depth)
//! and to keep a view for, plus the copies to record before any shadow pass ([`CopyOrder`]).
//!
//! Rules from the spec: the mover decisions are per lamp, never per tile (SH-20b: no cut at a
//! cube seam), so a lamp's mover tiles draw together or wait together, both for the per-frame
//! cap (oldest lamp first) and for store layers ([`StorePool`], LRU; a lamp that cannot get
//! them draws no movers and counts in [`FrameStats::store_starved`]). Tiles with no static
//! caster are never captured or stored: their live layer is copied from the cleared layer.
//!
//! Open owner questions kept expressible, defaults as the spec: static-only lamps
//! ([`LampTier::StaticOnly`], [`LampCacheConfig::static_tier_slots`] 0) and vertex-animated
//! casters ([`CasterMobility::AnimatedStatic`], frozen unless
//! [`LampCacheConfig::animated_casters_move`]).

mod pool;
mod tunables;

#[cfg(test)]
mod tests;

use std::collections::{BTreeMap, VecDeque};

pub use pool::{StoreLayer, StorePool};
pub use tunables::{
    ANIMATED_CASTERS_MOVE, CAPTURE_FACES_PER_FRAME, ENABLED, LampCacheConfig,
    MAX_MOVER_FACES_PER_FRAME, MOVER_FAR_INTERVAL, MOVER_FULL_RATE, MOVER_HALF_RATE, MOVER_LINGER,
    RECAPTURE_INTERVAL, SETTLE_TIME, STATIC_TIER_SLOTS, STORE_FACES_PER_LAMP, TUNABLES,
};

/// Tiles a [`TileMask`] can hold.
pub const MAX_TILES: u8 = 8; // tunables: const (capacity: the bits of TileMask)

/// A set of a lamp's tiles, bit `i` = tile `i`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct TileMask(pub u8);

impl TileMask {
    pub const NONE: Self = Self(0);

    /// Tile `tile` alone.
    #[must_use]
    pub const fn single(tile: u8) -> Self {
        Self(1 << tile)
    }

    /// Tiles `0..tiles`.
    #[must_use]
    pub const fn first(tiles: u8) -> Self {
        if tiles >= MAX_TILES {
            Self(u8::MAX)
        } else {
            Self((1 << tiles) - 1)
        }
    }

    #[must_use]
    pub const fn contains(self, tile: u8) -> bool {
        tile < MAX_TILES && self.0 & (1 << tile) != 0
    }

    pub fn insert(&mut self, tile: u8) {
        self.0 |= 1 << tile;
    }

    #[must_use]
    pub const fn count(self) -> u32 {
        self.0.count_ones()
    }

    #[must_use]
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }

    #[must_use]
    pub const fn union(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }

    /// The tiles in the set, lowest first.
    pub fn iter(self) -> impl Iterator<Item = u8> {
        (0..MAX_TILES).filter(move |&t| self.contains(t))
    }
}

/// How many tiles a lamp has (6 cube faces today; 2 or 4 with CW-13's atlas).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TileLayout {
    tiles: u8,
}

impl TileLayout {
    /// A cube map: six faces, in the renderer's face order.
    pub const CUBE: Self = Self { tiles: 6 };

    /// `None` outside `1..=MAX_TILES`.
    #[must_use]
    pub const fn new(tiles: u8) -> Option<Self> {
        if tiles == 0 || tiles > MAX_TILES {
            None
        } else {
            Some(Self { tiles })
        }
    }

    #[must_use]
    pub const fn tiles(self) -> u8 {
        self.tiles
    }

    #[must_use]
    pub const fn all(self) -> TileMask {
        TileMask::first(self.tiles)
    }
}

/// The host's id of a lamp (an entity's bits in `sf_sky_bevy`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LampKey(pub u64);

/// What a cached lamp does with movers.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum LampTier {
    /// Holds a budget slot: static depth cached, movers drawn over it.
    #[default]
    Full,
    /// Past the budget (S6, owner question 1): captured once, never draws movers, takes no
    /// store layer. Admitted only while [`LampCacheConfig::static_tier_slots`] allows.
    StaticOnly,
}

/// How a shadow caster moves, set by the host (S3's tracker).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum CasterMobility {
    /// Never moves (GPU-culled statics, instance batches): static at once.
    Static,
    /// Static placement, animated in the vertex shader (wind on trees and banners): frozen in
    /// the cache unless [`LampCacheConfig::animated_casters_move`] (owner question 2).
    AnimatedStatic,
    /// A rigid caster (door, item, physics object): static once still for
    /// [`LampCacheConfig::settle_time_s`], a mover while it moves.
    #[default]
    Rigid,
    /// Skinned, morphed or marked movable: always a mover.
    Movable,
}

/// What a caster is to the cache this frame.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CasterClass {
    /// In the captured depth.
    Static,
    /// Drawn each due frame over the copy.
    Mover,
}

/// How often a lamp's movers are drawn ([`LampCacheConfig::rate_tier`]).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RateTier {
    /// Every frame.
    Full,
    /// Every second frame, lamps alternating.
    Half,
    /// Every [`LampCacheConfig::mover_far_interval_frames`] frames, lamps alternating.
    Far,
}

impl LampCacheConfig {
    /// The rate tier for a lamp whose nearest mover is `mover_distance_m` from the nearest camera.
    #[must_use]
    pub fn rate_tier(&self, mover_distance_m: f32) -> RateTier {
        if mover_distance_m <= self.mover_full_rate_m {
            RateTier::Full
        } else if mover_distance_m <= self.mover_half_rate_m {
            RateTier::Half
        } else {
            RateTier::Far
        }
    }

    /// Frames between two mover draws in `tier`.
    #[must_use]
    pub fn interval_frames(&self, tier: RateTier) -> u32 {
        match tier {
            RateTier::Full => 1,
            RateTier::Half => 2,
            RateTier::Far => self.mover_far_interval_frames.max(1),
        }
    }

    /// Whether a caster is in the cached depth or drawn as a mover; `still_for_s` is how long
    /// a [`CasterMobility::Rigid`] caster has not moved relative to the lamp.
    #[must_use]
    pub fn classify(&self, mobility: CasterMobility, still_for_s: f32) -> CasterClass {
        let moves = match mobility {
            CasterMobility::Static => false,
            CasterMobility::AnimatedStatic => self.animated_casters_move,
            CasterMobility::Rigid => still_for_s < self.settle_time_s,
            CasterMobility::Movable => true,
        };
        if moves {
            CasterClass::Mover
        } else {
            CasterClass::Static
        }
    }
}

/// A tile's state (module docs).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FaceState {
    /// Never captured since admission, or queued / drawn for a (re)capture not yet confirmed.
    Capture,
    /// Static depth in the live layer, nothing over it: no view, no work.
    Clean,
    /// A mover is in it this frame.
    Movers,
    /// Its movers left less than [`LampCacheConfig::mover_linger_s`] ago; clean, view kept.
    Linger,
    /// A static changed; waits for [`LampCacheConfig::recapture_interval_s`], clean meanwhile.
    Dirty,
}

/// Why a lamp's cache changes ([`LampCache::invalidate`]; spec §6).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Invalidation {
    /// The lamp despawned (cell unload).
    Despawned,
    /// The budget gave its slot to another lamp.
    SlotLost,
    /// It moved more than 1 mm against its statics: the stock path until it is still.
    Moved,
    /// Statics in its range spawned, despawned, moved, hid or showed: the `touched` tiles
    /// recapture; `statics` is every tile that has a static caster now.
    StaticsChanged {
        touched: TileMask,
        statics: TileMask,
    },
    /// Switched on or off, flicker, colour or range: depth only, nothing to do.
    LightChanged,
}

/// What an [`Invalidation`] did.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InvalidationEffect {
    /// The lamp left the cache: store layers freed, a re-admission captures again.
    Removed,
    /// These tiles capture again (now if never captured, else after the interval).
    Recapture(TileMask),
    /// Nothing (or the lamp is not cached).
    Nothing,
}

/// Why [`LampCache::admit`] refused.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AdmitError {
    AlreadyCached,
    /// Every slot of the tier is taken.
    NoSlot,
}

/// A lamp's movers this frame, after the host's visibility tests (the lamp's range sphere in a
/// camera frustum, SH-21b; the tile's frustum in a camera's, CW-3b).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MoverInput {
    /// Tiles with a mover in their frustum and the lamp's range.
    pub tiles: TileMask,
    /// From the lamp's nearest mover to the nearest camera ([`LampCacheConfig::rate_tier`]).
    pub distance_m: f32,
}

/// A whole-layer copy, recorded before any shadow pass of the frame.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CopyOp {
    /// Live layer -> store layer: the clean face's static depth, before movers go over it.
    Save(StoreLayer),
    /// Store layer -> live layer.
    Restore(StoreLayer),
    /// The permanently cleared layer -> live layer (a tile with no static caster).
    Clear,
}

/// A copy for one tile of one lamp.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CopyOrder {
    pub lamp: LampKey,
    pub tile: u8,
    pub op: CopyOp,
}

/// One cached lamp's frame. The vendored mechanism (S2) draws `capture | movers`, loads the
/// depth for `movers` (clears it for `capture`) and keeps a view, not rendered, for the rest of
/// `views`; tiles outside `views` have no view.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LampPlan {
    pub lamp: LampKey,
    /// Every tile captured at least once: the shadow may fade in. Until then its strength stays 0.
    pub ready: bool,
    /// Tiles drawn with static casters only, cleared first.
    pub capture: TileMask,
    /// Tiles drawn with movers only, over the depth already in the live layer.
    pub movers: TileMask,
    /// Tiles that keep a view (drawn this frame, movers not due this frame, lingering).
    pub views: TileMask,
}

/// What a frame did, for the `lamp_cache/*` gauges.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct FrameStats {
    pub lamps: u32,
    pub ready_lamps: u32,
    /// Tiles per [`FaceState`].
    pub capture_faces: u32,
    pub clean_faces: u32,
    pub mover_faces: u32,
    pub linger_faces: u32,
    pub dirty_faces: u32,
    /// Capture draws issued this frame.
    pub captures: u32,
    /// Copies this frame, by kind.
    pub saves: u32,
    pub restores: u32,
    pub clears: u32,
    /// Mover tiles drawn this frame.
    pub mover_draws: u32,
    /// Mover tiles of due lamps left for a later frame by the cap.
    pub deferred_faces: u32,
    /// Mover tiles not drawn because no store layer was free.
    pub store_starved: u32,
    /// Captures waiting or awaiting confirmation.
    pub queued: u32,
}

/// The output of [`LampCache::plan`].
#[derive(Clone, Debug, Default, PartialEq)]
pub struct FramePlan {
    /// The frame counter (starts at 1).
    pub frame: u64,
    /// One per cached lamp, by key.
    pub lamps: Vec<LampPlan>,
    pub copies: Vec<CopyOrder>,
    pub stats: FrameStats,
}

impl FramePlan {
    #[must_use]
    pub fn lamp(&self, lamp: LampKey) -> Option<&LampPlan> {
        self.lamps
            .binary_search_by_key(&lamp, |p| p.lamp)
            .ok()
            .map(|i| &self.lamps[i])
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Live {
    /// Not known to hold this lamp's depth (admitted, or a capture in flight).
    Unknown,
    /// Static depth only.
    Clean,
    /// Movers drawn over the static depth.
    Drawn,
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Capture {
    None,
    Queued,
    /// Drawn at `at_s`; redrawn each frame until confirmed.
    Issued {
        at_s: f32,
    },
}

#[derive(Clone, Copy, Debug)]
#[allow(
    clippy::struct_excessive_bools,
    reason = "independent per-tile flags, not a mode"
)]
struct Tile {
    statics: bool,
    captured: bool,
    capture: Capture,
    /// Statics changed while a capture was in flight: dirty again once it is confirmed.
    redo: bool,
    dirty: bool,
    last_capture_s: f32,
    live: Live,
    /// Holds this tile's current static depth (`Some` only for tiles with statics).
    store: Option<StoreLayer>,
    movers: bool,
    linger_until: Option<f32>,
}

impl Tile {
    fn new(statics: bool) -> Self {
        Self {
            statics,
            captured: false,
            capture: Capture::None,
            redo: false,
            dirty: false,
            last_capture_s: f32::NEG_INFINITY,
            live: Live::Unknown,
            store: None,
            movers: false,
            linger_until: None,
        }
    }

    fn state(&self) -> FaceState {
        if !self.captured || self.capture != Capture::None {
            FaceState::Capture
        } else if self.movers {
            FaceState::Movers
        } else if self.linger_until.is_some() {
            FaceState::Linger
        } else if self.dirty {
            FaceState::Dirty
        } else {
            FaceState::Clean
        }
    }
}

#[derive(Clone, Debug)]
struct Lamp {
    tier: LampTier,
    tiles: Vec<Tile>,
    /// Admission order: spreads half- and far-rate lamps over frames.
    phase: u64,
    last_mover_frame: Option<u64>,
    input: Option<MoverInput>,
    /// This frame's [`MoverInput::distance_m`] (infinite without movers).
    mover_distance_m: f32,
    capture_now: TileMask,
    movers_now: TileMask,
}

impl Lamp {
    fn ready(&self) -> bool {
        self.tiles.iter().all(|t| t.captured)
    }
}

/// The cache's state: lamps, capture queue, store pool. See the module docs.
#[derive(Clone, Debug)]
pub struct LampCache {
    layout: TileLayout,
    max_lamps: u32,
    config: LampCacheConfig,
    lamps: BTreeMap<LampKey, Lamp>,
    queue: VecDeque<(LampKey, u8)>,
    pool: StorePool,
    frame: u64,
    next_phase: u64,
    plan: FramePlan,
}

impl LampCache {
    /// `max_lamps` full-tier slots (the shadow budget's); the store pool holds
    /// `max_lamps * store_faces_per_lamp` layers.
    #[must_use]
    pub fn new(layout: TileLayout, max_lamps: u32, config: LampCacheConfig) -> Self {
        Self {
            layout,
            max_lamps,
            config,
            lamps: BTreeMap::new(),
            queue: VecDeque::new(),
            pool: StorePool::new(max_lamps * config.store_faces_per_lamp),
            frame: 0,
            next_phase: 0,
            plan: FramePlan::default(),
        }
    }

    #[must_use]
    pub fn layout(&self) -> TileLayout {
        self.layout
    }

    #[must_use]
    pub fn config(&self) -> &LampCacheConfig {
        &self.config
    }

    #[must_use]
    pub fn pool(&self) -> &StorePool {
        &self.pool
    }

    /// The last [`Self::plan`].
    #[must_use]
    pub fn last_plan(&self) -> &FramePlan {
        &self.plan
    }

    #[must_use]
    pub fn contains(&self, lamp: LampKey) -> bool {
        self.lamps.contains_key(&lamp)
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.lamps.len()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.lamps.is_empty()
    }

    /// Captures waiting or awaiting confirmation, in order.
    pub fn capture_queue(&self) -> impl Iterator<Item = (LampKey, u8)> + '_ {
        self.queue.iter().copied()
    }

    #[must_use]
    pub fn face_state(&self, lamp: LampKey, tile: u8) -> Option<FaceState> {
        self.tile(lamp, tile).map(Tile::state)
    }

    /// Every tile captured at least once.
    #[must_use]
    pub fn is_ready(&self, lamp: LampKey) -> Option<bool> {
        self.lamps.get(&lamp).map(Lamp::ready)
    }

    /// The store layer holding a tile's static depth.
    #[must_use]
    pub fn store_layer(&self, lamp: LampKey, tile: u8) -> Option<StoreLayer> {
        self.tile(lamp, tile).and_then(|t| t.store)
    }

    fn tile(&self, lamp: LampKey, tile: u8) -> Option<&Tile> {
        self.lamps.get(&lamp)?.tiles.get(usize::from(tile))
    }

    /// Admits a still lamp that holds a slot (or, [`LampTier::StaticOnly`], a static-tier
    /// slot); `statics` marks its tiles with a static caster. Its tiles capture from the next
    /// [`Self::plan`] on.
    ///
    /// # Errors
    /// The lamp is already cached, or its tier is full.
    pub fn admit(
        &mut self,
        lamp: LampKey,
        tier: LampTier,
        statics: TileMask,
    ) -> Result<(), AdmitError> {
        if self.lamps.contains_key(&lamp) {
            return Err(AdmitError::AlreadyCached);
        }
        let slots = match tier {
            LampTier::Full => self.max_lamps,
            LampTier::StaticOnly => self.config.static_tier_slots,
        };
        let used = self.lamps.values().filter(|l| l.tier == tier).count();
        if used >= slots as usize {
            return Err(AdmitError::NoSlot);
        }
        let tiles = (0..self.layout.tiles())
            .map(|t| Tile::new(statics.contains(t)))
            .collect();
        self.lamps.insert(
            lamp,
            Lamp {
                tier,
                tiles,
                phase: self.next_phase,
                last_mover_frame: None,
                input: None,
                mover_distance_m: f32::INFINITY,
                capture_now: TileMask::NONE,
                movers_now: TileMask::NONE,
            },
        );
        self.next_phase += 1;
        Ok(())
    }

    /// Applies an event to a cached lamp (spec §6).
    pub fn invalidate(&mut self, lamp: LampKey, cause: Invalidation) -> InvalidationEffect {
        match cause {
            Invalidation::Despawned | Invalidation::SlotLost | Invalidation::Moved => {
                if self.remove(lamp) {
                    InvalidationEffect::Removed
                } else {
                    InvalidationEffect::Nothing
                }
            }
            Invalidation::LightChanged => InvalidationEffect::Nothing,
            Invalidation::StaticsChanged { touched, statics } => {
                let Some(l) = self.lamps.get_mut(&lamp) else {
                    return InvalidationEffect::Nothing;
                };
                let mut recapture = TileMask::NONE;
                for (t, tile) in (0..).zip(l.tiles.iter_mut()) {
                    let had = tile.statics;
                    tile.statics = statics.contains(t);
                    if !touched.contains(t) && had == tile.statics {
                        continue;
                    }
                    match tile.capture {
                        Capture::Issued { .. } => tile.redo = true,
                        Capture::None if tile.captured => tile.dirty = true,
                        // A queued draw takes the new statics; a first capture is due anyway.
                        Capture::Queued | Capture::None => {}
                    }
                    recapture.insert(t);
                }
                InvalidationEffect::Recapture(recapture)
            }
        }
    }

    /// Takes a lamp out of the cache, freeing its store layers and queued captures.
    pub fn remove(&mut self, lamp: LampKey) -> bool {
        let Some(l) = self.lamps.remove(&lamp) else {
            return false;
        };
        for layer in l.tiles.iter().filter_map(|t| t.store) {
            self.pool.release(layer);
        }
        self.queue.retain(|&(k, _)| k != lamp);
        true
    }

    /// The lamp's movers for the next [`Self::plan`] (none if not called that frame).
    pub fn set_movers(&mut self, lamp: LampKey, movers: MoverInput) -> bool {
        self.lamps
            .get_mut(&lamp)
            .map(|l| l.input = Some(movers))
            .is_some()
    }

    /// The render world drew this tile's capture with every pipeline ready. Until confirmed,
    /// an issued capture is drawn again each frame. Hosts without that feedback confirm right
    /// after [`Self::plan`].
    pub fn confirm_capture(&mut self, lamp: LampKey, tile: u8) -> bool {
        let Some(t) = self
            .lamps
            .get_mut(&lamp)
            .and_then(|l| l.tiles.get_mut(usize::from(tile)))
        else {
            return false;
        };
        let Capture::Issued { at_s } = t.capture else {
            return false;
        };
        t.capture = Capture::None;
        t.captured = true;
        t.last_capture_s = at_s;
        t.live = Live::Clean;
        t.dirty = std::mem::take(&mut t.redo);
        self.queue.retain(|&e| e != (lamp, tile));
        true
    }

    /// Cube size, preset or texture changed: every tile captures again (lamps fade in again)
    /// and every store layer is freed.
    pub fn recapture_all(&mut self) {
        self.queue.clear();
        self.pool = StorePool::new(u32::try_from(self.pool.len()).unwrap_or(u32::MAX));
        for l in self.lamps.values_mut() {
            l.last_mover_frame = None;
            for t in &mut l.tiles {
                *t = Tile::new(t.statics);
            }
        }
    }

    /// New values. A new pool size drops every store copy: clean tiles save again when movers
    /// come, tiles with movers drawn over them recapture next frame. Lowering `max_lamps`
    /// evicts nobody; admissions wait until the count is under it.
    pub fn reconfigure(&mut self, max_lamps: u32, config: LampCacheConfig) {
        let layers = max_lamps * config.store_faces_per_lamp;
        self.max_lamps = max_lamps;
        self.config = config;
        if self.pool.len() == layers as usize {
            return;
        }
        self.pool = StorePool::new(layers);
        for l in self.lamps.values_mut() {
            for t in &mut l.tiles {
                if t.store.take().is_some() && t.live == Live::Drawn && t.capture == Capture::None {
                    // Movers are over the only static copy: no restore or mover draw until
                    // the recapture, which is due at once.
                    t.live = Live::Unknown;
                    t.dirty = true;
                    t.last_capture_s = f32::NEG_INFINITY;
                }
            }
        }
    }

    /// Decides this frame (module docs). `now_s` is the host's clock in seconds.
    pub fn plan(&mut self, now_s: f32) -> &FramePlan {
        self.frame += 1;
        let mut copies = Vec::new();
        let mut stats = FrameStats::default();
        self.update_movers(now_s, &mut copies);
        self.update_captures(now_s, &mut copies, &mut stats);
        self.draw_movers(&mut copies, &mut stats);

        let mut lamps = Vec::with_capacity(self.lamps.len());
        for (&key, l) in &self.lamps {
            let mut views = l.capture_now.union(l.movers_now);
            for (t, tile) in (0..).zip(&l.tiles) {
                match tile.state() {
                    FaceState::Capture => stats.capture_faces += 1,
                    FaceState::Clean => stats.clean_faces += 1,
                    FaceState::Movers => stats.mover_faces += 1,
                    FaceState::Linger => stats.linger_faces += 1,
                    FaceState::Dirty => stats.dirty_faces += 1,
                }
                if tile.movers || tile.linger_until.is_some() {
                    views.insert(t);
                }
            }
            let ready = l.ready();
            stats.lamps += 1;
            stats.ready_lamps += u32::from(ready);
            lamps.push(LampPlan {
                lamp: key,
                ready,
                capture: l.capture_now,
                movers: l.movers_now,
                views,
            });
        }
        for c in &copies {
            match c.op {
                CopyOp::Save(_) => stats.saves += 1,
                CopyOp::Restore(_) => stats.restores += 1,
                CopyOp::Clear => stats.clears += 1,
            }
        }
        stats.queued = u32::try_from(self.queue.len()).unwrap_or(u32::MAX);
        self.plan = FramePlan {
            frame: self.frame,
            lamps,
            copies,
            stats,
        };
        &self.plan
    }

    /// Takes this frame's mover input; a tile whose movers left gets its last copy and lingers.
    fn update_movers(&mut self, now_s: f32, copies: &mut Vec<CopyOrder>) {
        let frame = self.frame;
        for (&key, l) in &mut self.lamps {
            l.capture_now = TileMask::NONE;
            l.movers_now = TileMask::NONE;
            let input = l.input.take();
            l.mover_distance_m = input.map_or(f32::INFINITY, |m| m.distance_m);
            let mask = match l.tier {
                LampTier::Full => input.map_or(TileMask::NONE, |m| m.tiles),
                LampTier::StaticOnly => TileMask::NONE,
            };
            for (t, tile) in (0..).zip(l.tiles.iter_mut()) {
                let has = mask.contains(t);
                if tile.movers && !has {
                    if tile.live == Live::Drawn {
                        copies.push(CopyOrder {
                            lamp: key,
                            tile: t,
                            op: restore_op(&mut self.pool, tile, frame),
                        });
                        tile.live = Live::Clean;
                    }
                    tile.linger_until = Some(now_s + self.config.mover_linger_s);
                }
                if has || tile.linger_until.is_some_and(|until| now_s >= until) {
                    tile.linger_until = None;
                }
                tile.movers = has;
            }
        }
    }

    /// Queues due captures, clears empty tiles, issues the head of the queue.
    fn update_captures(&mut self, now_s: f32, copies: &mut Vec<CopyOrder>, stats: &mut FrameStats) {
        let interval = self.config.recapture_interval_s;
        for (&key, l) in &mut self.lamps {
            for (t, tile) in (0..).zip(l.tiles.iter_mut()) {
                let due = !tile.captured || (tile.dirty && now_s - tile.last_capture_s >= interval);
                if tile.capture != Capture::None || !due {
                    continue;
                }
                tile.dirty = false;
                if tile.statics {
                    tile.capture = Capture::Queued;
                    self.queue.push_back((key, t));
                } else {
                    // Nothing static to draw: the cleared layer is the capture.
                    if let Some(layer) = tile.store.take() {
                        self.pool.release(layer);
                    }
                    copies.push(CopyOrder {
                        lamp: key,
                        tile: t,
                        op: CopyOp::Clear,
                    });
                    tile.captured = true;
                    tile.last_capture_s = now_s;
                    tile.live = Live::Clean;
                }
            }
        }
        let issue = self.config.capture_faces_per_frame as usize;
        for &(key, t) in self.queue.iter().take(issue) {
            let Some(l) = self.lamps.get_mut(&key) else {
                continue;
            };
            let tile = &mut l.tiles[usize::from(t)];
            tile.capture = Capture::Issued { at_s: now_s };
            tile.live = Live::Unknown;
            if let Some(layer) = tile.store.take() {
                self.pool.release(layer);
            }
            l.capture_now.insert(t);
            stats.captures += 1;
        }
    }

    /// Due lamps, longest waiting first, draw all their mover tiles or none.
    fn draw_movers(&mut self, copies: &mut Vec<CopyOrder>, stats: &mut FrameStats) {
        let frame = self.frame;
        let mut due: Vec<(Option<u64>, LampKey)> = Vec::new();
        for (&key, l) in &self.lamps {
            if l.tier != LampTier::Full || !l.ready() || mover_tiles(l).is_empty() {
                continue;
            }
            let interval = u64::from(
                self.config
                    .interval_frames(self.config.rate_tier(l.mover_distance_m)),
            );
            let scheduled = (frame + l.phase).is_multiple_of(interval);
            let overdue = l.last_mover_frame.is_none_or(|f| frame - f >= interval);
            if scheduled || overdue {
                due.push((l.last_mover_frame, key));
            }
        }
        due.sort_unstable();

        let cap = self.config.max_mover_faces_per_frame;
        let mut drawn = 0u32;
        for (_, key) in due {
            let l = &self.lamps[&key];
            let tiles = mover_tiles(l);
            let k = tiles.count();
            if drawn > 0 && drawn + k > cap {
                stats.deferred_faces += k;
                continue;
            }
            let need = tiles
                .iter()
                .filter(|&t| {
                    let tile = &l.tiles[usize::from(t)];
                    tile.statics && tile.store.is_none()
                })
                .count();
            if self.pool.available(key) < need {
                stats.store_starved += u32::try_from(need).unwrap_or(u32::MAX);
                continue;
            }
            let mut victims = Vec::new();
            let l = self.lamps.get_mut(&key).expect("a due lamp is cached");
            for t in tiles.iter() {
                let tile = &mut l.tiles[usize::from(t)];
                match tile.live {
                    Live::Drawn => copies.push(CopyOrder {
                        lamp: key,
                        tile: t,
                        op: restore_op(&mut self.pool, tile, frame),
                    }),
                    Live::Clean if tile.statics && tile.store.is_none() => {
                        let (layer, victim) =
                            self.pool.take((key, t), frame).expect("checked available");
                        victims.extend(victim);
                        tile.store = Some(layer);
                        copies.push(CopyOrder {
                            lamp: key,
                            tile: t,
                            op: CopyOp::Save(layer),
                        });
                    }
                    Live::Clean | Live::Unknown => {}
                }
                if let Some(layer) = tile.store {
                    self.pool.set_pinned(layer, true);
                    self.pool.touch(layer, frame);
                }
                tile.live = Live::Drawn;
            }
            l.movers_now = tiles;
            l.last_mover_frame = Some(frame);
            drawn += k;
            stats.mover_draws += k;
            for (lamp, t) in victims {
                if let Some(v) = self.lamps.get_mut(&lamp) {
                    v.tiles[usize::from(t)].store = None;
                }
            }
        }
    }
}

/// Tiles with movers whose live layer holds this lamp's depth (not in a capture).
fn mover_tiles(l: &Lamp) -> TileMask {
    let mut mask = TileMask::NONE;
    for (t, tile) in (0..).zip(&l.tiles) {
        if tile.movers && tile.live != Live::Unknown {
            mask.insert(t);
        }
    }
    mask
}

/// The copy that makes a drawn tile clean again (its store layer unpinned), or clears it.
fn restore_op(pool: &mut StorePool, tile: &Tile, frame: u64) -> CopyOp {
    match tile.store {
        Some(layer) => {
            pool.set_pinned(layer, false);
            pool.touch(layer, frame);
            CopyOp::Restore(layer)
        }
        None => CopyOp::Clear,
    }
}
