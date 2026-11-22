//! [`TerrainGround`]: the streamed terrain (`sf_terrain_bevy`) as a [`GroundSource`] (slice GC-2b1).
//!
//! - **Heights and normals**: the resident level-0 height tiles (`TerrainHeightsMap`, base + edits), the same
//!   bilinear heights and central-difference normals as every other CPU query. Coarser levels never answer: a
//!   point off the resident level-0 tiles is `valid = false`.
//! - **Channels**: the level-0 layer maps (`TerrainLayersMap`, 1 m samples), the nearest sample's four
//!   `(layer, weight)` pairs as `weight / 255`, each added to its layer's channel through the caller's
//!   [`LayerChannels`] remap (layers it doesn't list feed nothing). `channels` is resized in place: no allocation
//!   per sample once it has grown.
//! - **Water**: `water_distance = height − sea level`, the sea level the caller passes (the worldspace record's,
//!   `sf_world::Worldspace::sea_level`); no sea level: `f32::MAX`. Lakes and rivers (`sf_water` bodies other
//!   than the sea) are not read yet (a gap).
//! - **Readiness**: a rect is ready once every level-0 tile it touches is listed in the layer map and, where that
//!   tile has a map, its height tile is resident too. A tile known to have no map (no terrain there) is ready:
//!   it scatters nothing.
//!
//! **Freshness.** Scatter jobs run on tasks, so the source holds an `Arc` snapshot (the listed tiles' `Arc`
//! height tiles, the edits, the layer maps), never a `Res` borrow. [`TerrainGround::refresh`] swaps in a new
//! snapshot when the resident set changed (a pointer comparison per listed tile otherwise); the
//! [`TerrainGroundPlugin`] runs it every frame after `TerrainSystems::Update` for the [`TerrainGroundRef`]
//! resource. The same `Arc<TerrainGround>` stays in `GroundcoverSource`, so a refresh rebuilds no tile. A job
//! samples only ready rects and the terrain evicts tiles far past groundcover ranges, so a swap mid-job changes
//! nothing it reads, except a terrain edit landing in that rect (edited ground is not rescattered: a gap).

use std::collections::HashMap;
use std::sync::{Arc, PoisonError, RwLock};

use bevy_app::{App, Plugin, PostUpdate};
use bevy_ecs::prelude::*;
use bevy_math::{DVec3, Vec3};
use sf_coords::WorldPos;
use sf_groundcover::{GroundSample, GroundSampler, Rect};
use sf_terrain_bevy::sf_terrain::{LAYER_SAMPLES, LayerId, LayerMap, TILE_QUADS, TerrainHeights, TileCoord, global_coords};
use sf_terrain_bevy::{TerrainHeightsMap, TerrainLayersMap, TerrainSystems};
use sf_world::WorldspaceId;

use crate::GroundSource;

/// Terrain layer → groundcover channel (the rule table's `ChannelRule::channel`). Several layers may share a
/// channel (their weights add); a layer it doesn't list feeds none.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct LayerChannels {
    /// `(LayerId.0, channel)` sorted by layer id: real ids are 32-bit name hashes (`sf_terrain::layer_id`), not
    /// indices, so the map is sparse.
    map: Vec<(u32, u16)>,
    count: usize,
}

impl LayerChannels {
    /// Dense form: `map[n]` is the channel of `LayerId(n)` (synthetic worlds whose ids are small indices).
    pub fn new(map: Vec<Option<u16>>) -> Self {
        Self::from_pairs(
            map.into_iter().enumerate().filter_map(|(i, c)| Some((LayerId(u32::try_from(i).ok()?), c?))),
        )
    }

    /// Sparse form: each listed layer's channel (a layer listed twice keeps its last channel). The channel
    /// count is the highest listed channel + 1.
    pub fn from_pairs(pairs: impl IntoIterator<Item = (LayerId, u16)>) -> Self {
        let mut map: Vec<(u32, u16)> = pairs.into_iter().map(|(id, c)| (id.0, c)).collect();
        map.sort_by_key(|&(id, _)| id);
        map.reverse();
        map.dedup_by_key(|&mut (id, _)| id);
        map.reverse();
        let count = map.iter().map(|&(_, c)| usize::from(c) + 1).max().unwrap_or(0);
        Self { map, count }
    }

    /// Channel `n` = layer `n`, for layers `0..layers`.
    pub fn identity(layers: u16) -> Self {
        Self::new((0..layers).map(Some).collect())
    }

    /// How many channels a sample fills.
    pub fn count(&self) -> usize {
        self.count
    }

    /// Layer `id`'s channel, if it has one.
    pub fn channel(&self, id: LayerId) -> Option<usize> {
        let i = self.map.binary_search_by_key(&id.0, |&(l, _)| l).ok()?;
        Some(usize::from(self.map[i].1))
    }
}

/// What a scatter job reads: the listed level-0 tiles' heights (and the edits) and layer maps.
#[derive(Default)]
struct Snapshot {
    heights: TerrainHeights,
    layers: HashMap<TileCoord, Option<Arc<LayerMap>>>,
}

/// The streamed terrain of one worldspace as a [`GroundSource`]. Build it once per worldspace, share it as
/// `Arc` with `GroundcoverSource::ground` and [`TerrainGroundRef`]; it stays fresh through
/// [`TerrainGround::refresh`].
pub struct TerrainGround {
    ws: WorldspaceId,
    sea_level: Option<f32>,
    channels: LayerChannels,
    snapshot: RwLock<Arc<Snapshot>>,
}

/// Level-0 global samples per tile edge.
#[allow(clippy::cast_precision_loss)]
// tunables: const (the terrain format's tile edge)
const TILE_SAMPLES_F64: f64 = TILE_QUADS as f64;
/// Level-0 height samples per layer sample (heights 0.5 m, layers 1 m).
// tunables: const (the terrain format's sample spacings)
const HEIGHTS_PER_LAYER: f64 = 2.0;

/// The level-0 tile index of continuous global sample `g` on one axis.
#[allow(clippy::cast_possible_truncation)]
fn tile_index(g: f64) -> i32 {
    (g / TILE_SAMPLES_F64).floor() as i32
}

/// The nearest layer sample of global sample `g` in tile `t` (one axis).
#[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
fn layer_index(g: f64, t: i32) -> usize {
    let local = (g - f64::from(t) * TILE_SAMPLES_F64) / HEIGHTS_PER_LAYER;
    #[allow(clippy::cast_precision_loss)]
    let last = (LAYER_SAMPLES - 1) as f64;
    local.round().clamp(0.0, last) as usize
}

/// Global sample of absolute metres `m` on one axis (`sf_terrain::global_coords`, without the `WorldPos`).
fn global(m: f64) -> f64 {
    (m + 128.0) * 2.0
}

impl TerrainGround {
    /// The terrain of `ws`, its sea at `sea_level` (world Y, metres; `None`: no sea), layers mapped by
    /// `channels`. Nothing is resident until the first [`Self::refresh`].
    pub fn new(ws: WorldspaceId, sea_level: Option<f32>, channels: LayerChannels) -> Self {
        Self { ws, sea_level, channels, snapshot: RwLock::new(Arc::new(Snapshot::default())) }
    }

    pub fn worldspace(&self) -> &WorldspaceId {
        &self.ws
    }

    /// The layer → channel remap it was built with.
    pub fn channels(&self) -> &LayerChannels {
        &self.channels
    }

    fn current(&self) -> Arc<Snapshot> {
        self.snapshot.read().unwrap_or_else(PoisonError::into_inner).clone()
    }

    /// Swap in a new snapshot if the resident level-0 tiles of this worldspace changed (layer maps, height tiles
    /// or edits). Returns whether it did.
    pub fn refresh(&self, heights: &TerrainHeightsMap, layers: &TerrainLayersMap) -> bool {
        let live_h = heights.0.get(&self.ws);
        let empty = HashMap::new();
        let live_l = layers.0.get(&self.ws).unwrap_or(&empty);
        let old = self.current();
        if !stale(&old, live_h, live_l) {
            return false;
        }
        let mut snap = Snapshot { heights: TerrainHeights::new(), layers: live_l.clone() };
        if let Some(h) = live_h {
            for c in live_l.keys() {
                if let Some(t) = h.get(*c) {
                    snap.heights.insert(t.clone());
                }
            }
            *snap.heights.edits_mut() = h.edits().clone();
        }
        *self.snapshot.write().unwrap_or_else(PoisonError::into_inner) = Arc::new(snap);
        true
    }
}

/// Whether `old` differs from the live tiles: another layer set or map, another height tile, other edits.
fn stale(old: &Snapshot, heights: Option<&TerrainHeights>, layers: &HashMap<TileCoord, Option<Arc<LayerMap>>>) -> bool {
    if old.layers.len() != layers.len() {
        return true;
    }
    let edits = heights.map_or(0, |h| h.edits().revision());
    if old.heights.edits().revision() != edits {
        return true;
    }
    layers.iter().any(|(c, m)| {
        let same_map = match (old.layers.get(c), m) {
            (Some(Some(a)), Some(b)) => Arc::ptr_eq(a, b),
            (Some(None), None) => true,
            _ => false,
        };
        let same_tile = match (old.heights.get(*c), heights.and_then(|h| h.get(*c))) {
            (Some(a), Some(b)) => Arc::ptr_eq(a, b),
            (None, None) => true,
            _ => false,
        };
        !same_map || !same_tile
    })
}

impl GroundSampler for TerrainGround {
    fn sample(&self, x: f64, z: f64, out: &mut GroundSample) {
        let snap = self.snapshot.read().unwrap_or_else(PoisonError::into_inner);
        out.channels.clear();
        out.channels.resize(self.channels.count(), 0.0);
        let p = WorldPos::from_f64(DVec3::new(x, 0.0, z));
        let Some(height) = snap.heights.height(p) else {
            out.height = 0.0;
            out.normal = Vec3::Y;
            out.water_distance = f32::MAX;
            out.valid = false;
            return;
        };
        out.height = height;
        out.normal = snap.heights.normal(p).unwrap_or(Vec3::Y);
        out.water_distance = self.sea_level.map_or(f32::MAX, |sea| height - sea);
        out.valid = true;
        let g = global_coords(p);
        let (tx, tz) = (tile_index(g.x), tile_index(g.y));
        if let Some(Some(map)) = snap.layers.get(&TileCoord::new(0, tx, tz)) {
            for (id, w) in map.sample(layer_index(g.x, tx), layer_index(g.y, tz)) {
                if let (true, Some(ch)) = (w > 0, self.channels.channel(id)) {
                    out.channels[ch] += f32::from(w) / 255.0;
                }
            }
        }
    }
}

impl GroundSource for TerrainGround {
    fn ready(&self, rect: Rect) -> bool {
        let snap = self.current();
        let (x0, x1) = (tile_index(global(rect.min[0])), tile_index(global(rect.max[0])));
        let (z0, z1) = (tile_index(global(rect.min[1])), tile_index(global(rect.max[1])));
        (z0..=z1).all(|tz| {
            (x0..=x1).all(|tx| {
                let c = TileCoord::new(0, tx, tz);
                match snap.layers.get(&c) {
                    None => false,
                    Some(None) => true,
                    Some(Some(_)) => snap.heights.get(c).is_some(),
                }
            })
        })
    }
}

/// The [`TerrainGround`] the [`TerrainGroundPlugin`] keeps fresh (the same `Arc` as `GroundcoverSource::ground`).
#[derive(Resource, Clone)]
pub struct TerrainGroundRef(pub Arc<TerrainGround>);

/// Refreshes [`TerrainGroundRef`] every frame in `PostUpdate` after `TerrainSystems::Update`.
pub struct TerrainGroundPlugin;

impl Plugin for TerrainGroundPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            PostUpdate,
            refresh_terrain_ground.after(TerrainSystems::Update).run_if(resource_exists::<TerrainGroundRef>),
        );
    }
}

/// [`TerrainGround::refresh`] for [`TerrainGroundRef`].
pub fn refresh_terrain_ground(
    ground: Res<TerrainGroundRef>,
    heights: Option<Res<TerrainHeightsMap>>,
    layers: Option<Res<TerrainLayersMap>>,
) {
    if let (Some(heights), Some(layers)) = (heights, layers) {
        ground.0.refresh(&heights, &layers);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sf_terrain_bevy::sf_terrain::{BRICK_SAMPLES, BrickCoord, EditBrick, HeightTile, ResidentTile, TILE_SAMPLES};

    /// Tile (0, 0, 0) spans x, z in [-128, 128) m: a ramp 10 m at its west edge rising 0.25 m per metre east.
    fn ramp(c: TileCoord) -> Arc<ResidentTile> {
        let mut t = HeightTile::flat(c, 0.0);
        for j in 0..TILE_SAMPLES {
            for i in 0..TILE_SAMPLES {
                #[allow(clippy::cast_precision_loss)]
                let x_m = i as f32 * 0.5;
                t.heights[j * TILE_SAMPLES + i] = 10.0 + 0.25 * x_m;
            }
        }
        Arc::new(ResidentTile::new(t))
    }

    /// Layer 3 alone west of the tile's middle (layer sample 128), layer 5 at 200/255 over 3 at 55/255 east.
    fn two_layers(c: TileCoord) -> Arc<LayerMap> {
        let mut samples = Vec::with_capacity(LAYER_SAMPLES * LAYER_SAMPLES);
        for _j in 0..LAYER_SAMPLES {
            for i in 0..LAYER_SAMPLES {
                samples.push(if i < 128 {
                    [(LayerId(3), 255), (LayerId(3), 0), (LayerId(3), 0), (LayerId(3), 0)]
                } else {
                    [(LayerId(5), 200), (LayerId(3), 55), (LayerId(5), 0), (LayerId(5), 0)]
                });
            }
        }
        Arc::new(LayerMap::from_samples(c, &samples).unwrap())
    }

    fn ws() -> WorldspaceId {
        WorldspaceId::new("ground")
    }

    fn ground() -> TerrainGround {
        TerrainGround::new(ws(), Some(20.0), LayerChannels::new(vec![None, None, None, Some(0), None, Some(1)]))
    }

    fn resident(tiles: &[(TileCoord, bool, bool)]) -> (TerrainHeightsMap, TerrainLayersMap) {
        let (mut h, mut l) = (TerrainHeights::new(), HashMap::new());
        for &(c, height, map) in tiles {
            if height {
                h.insert(ramp(c));
            }
            l.insert(c, map.then(|| two_layers(c)));
        }
        (TerrainHeightsMap(HashMap::from([(ws(), h)])), TerrainLayersMap(HashMap::from([(ws(), l)])))
    }

    fn rect(x0: f64, x1: f64) -> Rect {
        Rect { min: [x0, -10.0], max: [x1, 10.0] }
    }

    fn close(a: f32, b: f32) -> bool {
        (a - b).abs() < 1e-5
    }

    #[test]
    fn layer_channels_remap() {
        let c = LayerChannels::new(vec![None, Some(2), None, Some(0)]);
        assert_eq!(c.count(), 3);
        assert_eq!((c.channel(LayerId(1)), c.channel(LayerId(3))), (Some(2), Some(0)));
        assert_eq!((c.channel(LayerId(0)), c.channel(LayerId(9))), (None, None));
        assert_eq!(LayerChannels::identity(4).channel(LayerId(3)), Some(3));
        assert_eq!(LayerChannels::default().count(), 0);
    }

    #[test]
    fn layer_channels_sparse_hashed_ids() {
        let c = LayerChannels::from_pairs([(LayerId(0xDEAD_BEEF), 1), (LayerId(7), 0), (LayerId(u32::MAX), 4), (LayerId(7), 2)]);
        assert_eq!(c.count(), 5);
        assert_eq!(c.channel(LayerId(0xDEAD_BEEF)), Some(1));
        assert_eq!(c.channel(LayerId(u32::MAX)), Some(4));
        assert_eq!(c.channel(LayerId(7)), Some(2), "last listing wins");
        assert_eq!(c.channel(LayerId(8)), None);
    }

    /// GC-2b1: height, normal, water distance and channels on a resident ramp tile with two layers; nothing off it.
    #[test]
    fn samples_heights_normals_and_layer_channels() {
        let src = ground();
        let a = TileCoord::new(0, 0, 0);
        let (h, l) = resident(&[(a, true, true)]);
        assert!(src.refresh(&h, &l));
        let mut out = GroundSample::blank();
        // West half: 50.25 m east of the tile's edge (layer sample 50).
        src.sample(-128.0 + 50.25, 3.0, &mut out);
        assert!(out.valid);
        assert!(close(out.height, 10.0 + 0.25 * 50.25), "{}", out.height);
        assert!(close(out.water_distance, out.height - 20.0));
        let n = Vec3::new(-0.25, 1.0, 0.0).normalize();
        assert!(out.normal.abs_diff_eq(n, 1e-5), "{:?}", out.normal);
        assert_eq!(out.channels, vec![1.0, 0.0]);
        // East half (layer sample 200): the blend, channels reused in place.
        let cap = out.channels.as_ptr();
        src.sample(-128.0 + 200.2, -40.0, &mut out);
        assert_eq!(out.channels.as_ptr(), cap, "no reallocation");
        assert!(close(out.channels[0], 55.0 / 255.0) && close(out.channels[1], 200.0 / 255.0), "{:?}", out.channels);
        assert!(close(out.height, 10.0 + 0.25 * 200.2));
        // The next tile east: nothing resident.
        src.sample(130.0, 0.0, &mut out);
        assert!(!out.valid);
        assert_eq!((out.channels.len(), out.water_distance), (2, f32::MAX));
        assert!(out.channels.iter().all(|&w| w == 0.0));
        // No sea: far from water.
        let dry = TerrainGround::new(ws(), None, LayerChannels::identity(6));
        dry.refresh(&h, &l);
        dry.sample(0.0, 0.0, &mut out);
        assert_eq!((out.water_distance, out.channels.len()), (f32::MAX, 6));
        assert!(close(out.channels[5], 200.0 / 255.0) && close(out.channels[3], 55.0 / 255.0));
    }

    /// GC-2b1: a rect is ready once its tiles' layer maps and heights are resident; a known-empty tile is ready;
    /// the snapshot follows removals and late height tiles; an unchanged terrain swaps nothing.
    #[test]
    fn ready_and_snapshot_follow_residency() {
        let src = ground();
        let (a, b) = (TileCoord::new(0, 0, 0), TileCoord::new(0, 1, 0));
        let inside = rect(-100.0, 100.0);
        let across = rect(100.0, 150.0);
        assert!(!src.ready(inside), "nothing before the first refresh");
        // Layer map listed, height tile not yet (a GPU-filled world's twin is late).
        let (h, l) = resident(&[(a, false, true)]);
        assert!(src.refresh(&h, &l));
        assert!(!src.ready(inside));
        let (h, l) = resident(&[(a, true, true)]);
        assert!(src.refresh(&h, &l), "the late height tile");
        assert!(src.ready(inside));
        assert!(!src.refresh(&h, &l), "unchanged: no swap");
        assert!(!src.ready(across), "tile b is not resident");
        let mut l2 = TerrainLayersMap(l.0.clone());
        l2.0.get_mut(&ws()).unwrap().insert(b, None);
        assert!(src.refresh(&h, &l2));
        assert!(src.ready(across), "b is known to have no terrain");
        let mut out = GroundSample::blank();
        src.sample(150.0, 0.0, &mut out);
        assert!(!out.valid);
        // An edit (+1 m over the brick holding x = z = 0) reaches the snapshot.
        let mut edited = TerrainHeightsMap(h.0.clone());
        let deltas = vec![1.0; BRICK_SAMPLES * BRICK_SAMPLES];
        edited.0.get_mut(&ws()).unwrap().edits_mut().set_brick(BrickCoord::of_sample(256, 256), EditBrick { deltas });
        assert!(src.refresh(&edited, &l2));
        src.sample(0.0, 0.0, &mut out);
        assert!(close(out.height, 10.0 + 0.25 * 128.0 + 1.0), "{}", out.height);
        assert!(!src.refresh(&edited, &l2));
        // Tile a evicted.
        let (h, l) = resident(&[(b, false, false)]);
        assert!(src.refresh(&h, &l));
        assert!(!src.ready(inside));
        src.sample(0.0, 0.0, &mut out);
        assert!(!out.valid);
        // Another worldspace's tiles are not ours.
        let other = TerrainLayersMap(HashMap::from([(WorldspaceId::new("other"), l.0[&ws()].clone())]));
        assert!(src.refresh(&TerrainHeightsMap::default(), &other));
        assert!(!src.ready(across));
    }

    /// The plugin's system refreshes the shared ground from the terrain resources.
    #[test]
    fn plugin_system_refreshes_the_shared_ground() {
        use bevy_ecs::system::RunSystemOnce;
        let shared = Arc::new(ground());
        let mut world = World::new();
        let (h, l) = resident(&[(TileCoord::new(0, 0, 0), true, true)]);
        world.insert_resource(h);
        world.insert_resource(l);
        world.insert_resource(TerrainGroundRef(shared.clone()));
        assert!(!shared.ready(rect(-1.0, 1.0)));
        world.run_system_once(refresh_terrain_ground).unwrap();
        assert!(shared.ready(rect(-1.0, 1.0)));
    }
}
