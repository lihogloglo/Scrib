//! GPU-filled worlds (`gpu_fill`, worldgen G2) without a GPU: what the main world asks for. Pieces go to the
//! GPU queue (no CPU height or macro writes, layers still CPU), the CPU twin asks only for its tiles, CDLOD bounds
//! come from the provider's analytic bounds.

use super::*;

/// A flat provider at 10 m whose analytic bounds are 5..15 m, with a GPU field.
struct Field;

impl TerrainTiles for Field {
    fn worldspace(&self) -> &WorldspaceId {
        static WS: std::sync::OnceLock<WorldspaceId> = std::sync::OnceLock::new();
        WS.get_or_init(|| WorldspaceId::new("field"))
    }
    fn has_tile(&self, _: TileCoord) -> bool {
        true
    }
    fn tile(&self, _: TileCoord) -> Option<Result<sf_terrain::TileProduct, String>> {
        None
    }
    fn macro_albedo(&self, _: TileCoord) -> Option<Result<sf_terrain::MacroProduct, String>> {
        None
    }
    fn layers_capped(&self, _: Option<u32>, _: &LayerCap) -> Option<Result<sf_terrain::LayerArrays, String>> {
        None
    }
    fn gpu_field(&self) -> Option<sf_terrain::GpuField> {
        Some(sf_terrain::GpuField { wgsl: "".into(), data: vec![0u32; 4].into(), label: "test".into() })
    }
    fn block_bounds(&self, _: TileCoord) -> Option<BlockBounds> {
        Some(BlockBounds { min: vec![5.0; BLOCKS * BLOCKS], max: vec![15.0; BLOCKS * BLOCKS] })
    }
}

fn state(gpu: bool) -> TerrainState {
    TerrainState {
        ws: WorldspaceId::new("field"),
        src: Arc::new(Field),
        material: Handle::default(),
        params: Handle::default(),
        heights: Handle::default(),
        normals: Handle::default(),
        layer_ids: Handle::default(),
        layer_weights: Handle::default(),
        macro_image: Handle::default(),
        patch: Handle::default(),
        quadrant: Handle::default(),
        table: default_layer_table([70, 70, 70, 255], 10.0),
        slots: HashMap::new(),
        tiles: HashMap::new(),
        height_windows: [None; HEIGHT_LEVELS],
        layer_windows: [None; LAYER_LEVELS],
        macro_windows: [None; MACRO_LEVELS],
        nodes: HashMap::new(),
        origin: None,
        shift_settle: 0,
        fill: 10.0,
        fill_slot: 0,
        fill_rgba: [70, 70, 70, 255],
        max_height: 10.0,
        select_rev: 0,
        gpu: gpu.then(|| GpuMode {
            bounds: HashMap::new(),
            ready: Arc::new(AtomicBool::new(true)),
            counters: Arc::new(GpuFillCounters::default()),
            layers: None,
        }),
        gate: SelectGate::default(),
    }
}

#[test]
fn gpu_world_queues_pieces_instead_of_cpu_heights() {
    let s = state(true);
    let w = ClipWindow { min: [0, 0] };
    let moved = ClipWindow { min: [16, 0] };
    for (kind, level) in [(Kind::Height, 3usize), (Kind::Macro, 2), (Kind::Layer, 1)] {
        let (mut writes, mut queue) = (Vec::new(), Vec::new());
        assert!(window_ready(&s, kind, w, level, 512, 1), "no tile is needed");
        let bytes = upload_level(&s, None, kind, level, Some(w), moved, &mut writes, &mut queue);
        match kind {
            Kind::Height | Kind::Macro => {
                assert!(writes.is_empty(), "{kind:?}: no CPU texture write");
                let texels: i64 = queue.iter().map(|r| r.rect.size[0] * r.rect.size[1]).sum();
                assert_eq!(texels, 16 * 512, "{kind:?}: the exposed strip");
                assert!(queue.iter().all(|r| r.level == level_u8(level)));
                let per = if kind == Kind::Height { 6 } else { 4 };
                assert_eq!(bytes, 16 * 512 * per);
                assert_eq!(queue[0].kind, if kind == Kind::Height { FillKind::Heights } else { FillKind::Macro });
            }
            Kind::Layer => {
                assert!(queue.is_empty() && !writes.is_empty(), "layers stay CPU (the default slot)");
            }
        }
    }
    // The tile path writes heights on the CPU.
    let cpu = state(false);
    let (mut writes, mut queue) = (Vec::new(), Vec::new());
    upload_level(&cpu, None, Kind::Height, 3, Some(w), moved, &mut writes, &mut queue);
    assert!(queue.is_empty() && writes.len() == 2);
}

#[test]
fn gpu_world_bounds_come_from_the_provider() {
    let mut s = state(true);
    // Nothing cached: the fill height (as a tile path with nothing resident).
    assert_eq!(node_bounds(&s, 2, 0, 0), (10.0, 10.0));
    let b = s.src.block_bounds(TileCoord::new(4, 0, 0)).unwrap();
    s.gpu.as_mut().unwrap().bounds.insert(TileCoord::new(4, 0, 0), Arc::new(b));
    assert_eq!(node_bounds(&s, 2, 0, 0), (5.0, 15.0), "a finer node finds its coarser tile's bounds");
}

#[test]
fn twin_asks_only_for_near_and_coarse_tiles() {
    let twin = TwinSettings { near_levels: 2, near_radius: 1, coarse_level: 7, max_loads: 1 };
    let cam = DVec2::new(10_000.0, -3_000.0);
    let wanted = Wanted::new(cam, 16);
    let tiles = twin_tiles(cam, &wanted, &twin);
    let count = |l: u8| tiles.iter().filter(|c| c.level == l).count();
    assert_eq!((count(0), count(1)), (9, 9), "3 x 3 around the camera's tile");
    assert!((2..7).all(|l| count(l) == 0), "no middle level");
    assert!((7..10).all(|l| count(l) >= 1), "the coarse windows");
    let home = TileCoord::new(0, 19, -6);
    assert!(tiles.contains(&home), "the camera's own level-0 tile");
    // The nearest first within a level.
    assert!(tile_distance2(home, cam) < tile_distance2(TileCoord::new(0, 20, -6), cam));
    assert!(TwinSettings::default().max_loads >= 1 && TerrainSettings::default().gpu_fill);
}

/// A GPU-filled world whose provider gives layer maps loads them for its near layer windows (levels 0..3) and
/// waits for them before writing a layer window; height and macro windows still need no tile.
#[test]
fn gpu_world_streams_near_layer_maps() {
    let cam = DVec2::new(10_000.0, -3_000.0);
    let wanted = Wanted::new(cam, 16);
    let tiles = gpu_layer_tiles(&wanted, 0);
    assert!(tiles.iter().all(|c| usize::from(c.level) < LAYER_LEVELS), "near layer levels only");
    assert!((0..LAYER_LEVELS).all(|l| tiles.iter().any(|c| usize::from(c.level) == l)));
    assert!(gpu_layer_tiles(&wanted, 128).is_superset(&tiles), "the margin only adds");
    let mut s = state(true);
    s.gpu.as_mut().unwrap().layers = Some(HashMap::new());
    let w = wanted.layers[1];
    assert!(!window_ready(&s, Kind::Layer, w, 1, 256, 0), "maps not loaded yet");
    assert!(window_ready(&s, Kind::Height, wanted.heights[1], 1, 512, 1));
    assert!(window_ready(&s, Kind::Macro, wanted.macros[1], 1, 256, 0));
    let mut needed = HashSet::new();
    window_tiles(w, 1, 256, 0, &mut needed);
    let maps = s.gpu.as_mut().unwrap().layers.as_mut().unwrap();
    for c in needed {
        maps.insert(c, LayerSlot::Absent);
    }
    assert!(window_ready(&s, Kind::Layer, w, 1, 256, 0), "absent maps fill the default slot");
    // Without layer maps (the Morrowind path has no GPU field at all) layers never wait.
    assert!(window_ready(&state(true), Kind::Layer, w, 1, 256, 0));
}
