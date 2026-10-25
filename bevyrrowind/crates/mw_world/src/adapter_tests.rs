//! World adapter tests on synthetic content: a master and a plugin that
//! edits, moves, and adds references, plus land and land textures.

use std::collections::BTreeSet;

use glam::DVec3;
use sf_records::RecordId;
use sf_terrain::{LayerId, SourceMosaic, SourceTileId, TerrainSource};
use sf_vfs::{MemSource, Vfs};
use sf_world::{Partition, PlacementClass, PlacementFlags, WorldSource};
use tes3::esp::{
    AtmosphereData, Cell, CellFlags, Door, FileType, Header, Landscape, LandscapeFlags, LandscapeTexture, MiscItem, Npc,
    Plugin, Reference, Static, TES3Object, TextureIndices, TravelDestination, VertexHeights,
};

use crate::convert::UNIT_METERS_F64;
use crate::terrain::{self, LAND_TEXELS, LAND_VERTS, LandData};
use crate::{MwTerrainSource, MwWorldSource, SourceOptions, WorldRecords, ids};

fn header(file_type: FileType, masters: &[&str]) -> TES3Object {
    Header { file_type, masters: masters.iter().map(|m| ((*m).to_owned(), 0)).collect(), ..Header::default() }.into()
}

fn stat(id: &str, mesh: &str) -> TES3Object {
    Static { id: id.into(), mesh: mesh.into(), ..Static::default() }.into()
}

fn door(id: &str) -> TES3Object {
    Door { id: id.into(), mesh: "d\\door.nif".into(), ..Door::default() }.into()
}

fn ltex(id: &str, index: u32, file: &str) -> TES3Object {
    LandscapeTexture { id: id.into(), index, file_name: file.into(), ..LandscapeTexture::default() }.into()
}

fn reference(mast_index: u32, refr_index: u32, id: &str, at: [f32; 3]) -> Reference {
    Reference { mast_index, refr_index, id: id.into(), translation: at, ..Reference::default() }
}

fn to(r: Reference, cell: &str, at: [f32; 3]) -> Reference {
    Reference { destination: Some(TravelDestination { translation: at, rotation: [0.0; 3], cell: cell.into() }), ..r }
}

fn cell(name: &str, grid: Option<(i32, i32)>, refs: Vec<Reference>) -> Cell {
    let mut cell = Cell { name: name.into(), ..Cell::default() };
    match grid {
        Some(g) => cell.data.grid = g,
        None => cell.data.flags = CellFlags::IS_INTERIOR,
    }
    for r in refs {
        cell.references.insert((r.mast_index, r.refr_index), r);
    }
    cell
}

fn land(grid: (i32, i32), vtex: u16) -> TES3Object {
    Landscape {
        grid,
        landscape_flags: LandscapeFlags::USES_VERTEX_HEIGHTS_AND_NORMALS | LandscapeFlags::USES_TEXTURES,
        vertex_heights: VertexHeights { offset: 0.0, data: Box::new([[0; 65]; 65]) },
        texture_indices: TextureIndices { data: Box::new([[vtex; 16]; 16]) },
        ..Landscape::default()
    }
    .into()
}

fn master() -> Vec<u8> {
    let mut room = cell("Room", None, vec![
        to(reference(0, 21, "door_out", [0.0, 0.0, 0.0]), "", [1000.0, 900.0, 0.0]),
        reference(0, 22, "DoorMarker", [0.0, 50.0, 0.0]),
        reference(0, 23, "guard", [10.0, 10.0, 0.0]),
    ]);
    room.data.flags |= CellFlags::HAS_WATER;
    room.water_height = Some(-70.0);
    room.atmosphere_data = Some(AtmosphereData {
        ambient_color: [255, 0, 0, 0],
        sunlight_color: [0; 4],
        fog_color: [0; 4],
        fog_density: 0.5,
    });
    let closet = cell("Closet", None, vec![
        to(reference(0, 11, "door_in", [0.0, 0.0, 0.0]), "Nowhere", [0.0; 3]),
        to(reference(0, 12, "door_in", [100.0, 0.0, 0.0]), "Room", [-3000.0, 0.0, 0.0]),
    ]);
    let outside = cell("", Some((0, 0)), vec![
        reference(0, 1, "rock", [100.0, 100.0, 0.0]),
        to(reference(0, 2, "door_in", [1000.0, 1000.0, 0.0]), "Room", [10.0, -100.0, 0.0]),
        reference(0, 3, "cup", [200.0, 200.0, 0.0]),
        reference(0, 4, "rock", [500.0, 500.0, 2000.0]),
        reference(0, 5, "ghost", [0.0; 3]),
    ]);
    let objects = vec![
        header(FileType::Esm, &[]),
        stat("rock", "r\\Rock.NIF"),
        stat("house", "x\\house.nif"),
        stat("DoorMarker", "marker_arrow.nif"),
        door("door_in"),
        door("door_out"),
        MiscItem { id: "cup".into(), mesh: "m\\cup.nif".into(), ..MiscItem::default() }.into(),
        Npc { id: "guard".into(), ..Npc::default() }.into(),
        ltex("grass", 0, "tx_grass.tga"),
        ltex("rock", 1, "tx_rock.tga"),
        room.into(),
        closet.into(),
        outside.into(),
        land((0, 0), 2),
    ];
    Plugin { objects }.save_bytes().unwrap()
}

fn plugin() -> Vec<u8> {
    // Moves Master's rock 1 from (0, 0) to (1, 0) and adds a house.
    let moved = Reference { moved_cell: Some((1, 0)), ..reference(1, 1, "rock", [8292.0, 100.0, 0.0]) };
    let outside = cell("", Some((0, 0)), vec![moved, reference(0, 1, "house", [300.0, 300.0, 0.0])]);
    let objects = vec![
        header(FileType::Esp, &["Master.esm"]),
        // Same index as Master's "grass": the plugin's LAND means sand.
        ltex("sand", 0, "tx_sand.tga"),
        outside.into(),
        land((0, 1), 1),
    ];
    Plugin { objects }.save_bytes().unwrap()
}

fn load(files: &[&str]) -> mw_records::LoadedContent {
    let vfs = Vfs::builder()
        .mount(MemSource::new("data").with("Master.esm", master()).with("Plugin.esp", plugin()))
        .build();
    let files: Vec<String> = files.iter().map(|f| (*f).to_owned()).collect();
    mw_records::load(&vfs, &files).unwrap()
}

fn region_of(source: &MwWorldSource, cell: &str) -> Vec<sf_world::Placement> {
    let ws = if cell.starts_with('@') { ids::exterior() } else { ids::interior(cell) };
    let mut out = Vec::new();
    source.placements(&ws, ids::region_id(&RecordId::new(cell)), &mut |p| out.push(p));
    out
}

fn fingerprint(source: &MwWorldSource, ws: &sf_world::WorldspaceId, cell: &str) -> sf_world::Fingerprint {
    let want = ids::region_id(&RecordId::new(cell));
    let mut found = None;
    source.regions(ws, &mut |r, f| {
        if r == want {
            found = Some(f);
        }
    });
    found.unwrap()
}

#[test]
fn worldspaces_cover_the_exterior_and_each_interior() {
    let loaded = load(&["Master.esm"]);
    let records = WorldRecords::build(&loaded.db);
    let names: BTreeSet<&str> = records.worldspaces.iter().map(|(id, _)| id.as_str()).collect();
    assert_eq!(names, BTreeSet::from(["@exterior", "closet", "room"]));
    let (_, room) = records.worldspaces.iter().find(|(id, _)| id.as_str() == "room").unwrap();
    assert_eq!(room.partition, Partition::Single);
    let water = room.sea_level.unwrap();
    assert!((f64::from(water) + 70.0 * UNIT_METERS_F64).abs() < 1e-5);
    let (_, env) = records.environments.iter().find(|(id, _)| id.as_str() == "room").unwrap();
    assert!((env.atmosphere.unwrap().ambient[0] - 1.0).abs() < f32::EPSILON);
    let (_, ext) = &records.worldspaces[0];
    assert!(ext.has_terrain && ext.partition == Partition::STREAMING && ext.sea_level == Some(0.0));
}

#[test]
fn doors_pair_when_they_point_back() {
    let loaded = load(&["Master.esm"]);
    let records = WorldRecords::build(&loaded.db);
    let stats = &records.doors;
    assert_eq!((stats.doors, stats.paired, stats.one_way), (3, 1, 1), "{stats:?}");
    assert_eq!(stats.dangling, [(RecordId::new("closet"), "Nowhere".to_owned())]);

    let two_way: Vec<_> = records.door_links.iter().filter(|(_, l)| l.two_way).collect();
    let link = &two_way[0].1;
    let ends = BTreeSet::from([link.a.worldspace.as_str(), link.b.worldspace.as_str()]);
    assert_eq!(ends, BTreeSet::from(["@exterior", "room"]));
    // Arriving in the room puts the traveler at the exterior door's DODT.
    let room_end = if link.a.worldspace.as_str() == "room" { &link.a } else { &link.b };
    let want = crate::convert::world_pos([10.0, -100.0, 0.0]);
    assert!(room_end.arrival.pos.delta(want).length() < 1e-6);
    assert_eq!(room_end.door, Some(ids::placement_id("Master.esm", 21)));

    let (_, one_way) = records.door_links.iter().find(|(_, l)| !l.two_way).unwrap();
    assert_eq!(one_way.a.worldspace.as_str(), "closet");
    assert_eq!(one_way.b.worldspace.as_str(), "room");
    assert!(one_way.b.door.is_none());
}

#[test]
fn placements_classify_and_ground() {
    let loaded = load(&["Master.esm"]);
    let source = MwWorldSource::build(&loaded.db, &SourceOptions::default());
    let outside = region_of(&source, "@0,0");
    let classes: Vec<PlacementClass> = outside.iter().map(|p| p.class).collect();
    // Sorted by key: rock, door, cup, high rock (ghost has no base).
    assert_eq!(classes, [PlacementClass::Static, PlacementClass::Door, PlacementClass::Dynamic, PlacementClass::Static]);
    assert!(outside[0].flags.contains(PlacementFlags::GROUNDED), "rock on flat land at 0");
    assert!(!outside[3].flags.contains(PlacementFlags::GROUNDED), "rock 28 m up");
    assert!(!outside[2].flags.contains(PlacementFlags::GROUNDED), "items never");
    assert_eq!(outside[0].model.as_ref().unwrap().as_str(), "meshes/r/rock.nif");
    assert_eq!(outside[0].base.kind.as_ref(), "STAT");
    assert_eq!(source.stats().unknown_bases, [(RecordId::new("@0,0"), "ghost".to_owned())]);

    let room = region_of(&source, "room");
    let marker = room.iter().find(|p| p.base.id.as_str() == "doormarker").unwrap();
    assert_eq!(marker.class, PlacementClass::Marker);
    assert_eq!(marker.flags, PlacementFlags::EDITOR_ONLY);
    assert!(room.iter().any(|p| p.class == PlacementClass::Actor));
    // Interiors are never grounded (no terrain).
    assert!(room.iter().all(|p| !p.flags.contains(PlacementFlags::GROUNDED)));
}

#[test]
fn moved_references_arrive_at_their_destination_with_stable_ids() {
    let base = load(&["Master.esm"]);
    let base_source = MwWorldSource::build(&base.db, &SourceOptions::default());
    let both = load(&["Master.esm", "Plugin.esp"]);
    let source = MwWorldSource::build(&both.db, &SourceOptions::default());

    let rock = ids::placement_id("Master.esm", 1);
    assert!(region_of(&base_source, "@0,0").iter().any(|p| p.id == rock));
    assert!(!region_of(&source, "@0,0").iter().any(|p| p.id == rock), "moved out of (0, 0)");
    let moved = region_of(&source, "@1,0");
    assert_eq!(moved.len(), 1);
    assert_eq!(moved[0].id, rock, "same id after the move");
    assert_eq!(moved[0].transform.pos.partition(0).x, 0, "~118 m east: still grid cell 0 of 256 m");
    let s = source.stats();
    assert_eq!((s.moved, s.moved_to_missing_cell, s.moved_outside_destination), (1, 1, 0));

    // The plugin's own reference 1 has its own id.
    let house = ids::placement_id("plugin.esp", 1);
    assert!(region_of(&source, "@0,0").iter().any(|p| p.id == house && p.base.id.as_str() == "house"));
    // No id is delivered twice.
    let all: Vec<_> = source.all_placements(&ids::exterior()).map(|p| p.id).collect();
    assert_eq!(all.len(), all.iter().collect::<BTreeSet<_>>().len());

    // Fingerprints change exactly where content changed.
    let ext = ids::exterior();
    assert_ne!(fingerprint(&base_source, &ext, "@0,0"), fingerprint(&source, &ext, "@0,0"));
    let room = ids::interior("Room");
    assert_eq!(fingerprint(&base_source, &room, "room"), fingerprint(&source, &room, "room"));
}

#[test]
fn land_textures_resolve_per_file() {
    let loaded = load(&["Master.esm", "Plugin.esp"]);
    let terrain = MwTerrainSource::new(&loaded.db);
    let ext = ids::exterior();
    let mut tiles = Vec::new();
    terrain.tiles(&ext, &mut |t| tiles.push(t));
    assert_eq!(tiles.len(), 2);
    assert_eq!(terrain.stats().unresolved_textures, 0);
    let palette = |grid| terrain.read(&ext, terrain::tile_id(grid)).unwrap().layers.unwrap().palette;
    // Master's LAND: VTEX 2 = Master's index 1 = "rock". Plugin's: VTEX 1 =
    // the plugin's index 0 = "sand", not Master's "grass".
    assert_eq!(palette((0, 0)), [RecordId::new("rock")]);
    assert_eq!(palette((0, 1)), [RecordId::new("sand")]);
    assert!(terrain.read(&sf_world::WorldspaceId::new("room"), SourceTileId(0)).is_none());

    let layers = terrain::terrain_layers(&loaded.db, None);
    let ids: Vec<&str> = layers.iter().map(|(id, _)| id.as_str()).collect();
    assert_eq!(ids, ["@default", "grass", "rock", "sand"]);
    let (_, sand) = &layers[3];
    assert_eq!(sand.albedo_height.as_str(), "textures/tx_sand.tga");
    assert_eq!(sand.surface.id.as_str(), "sand");
    assert!((f64::from(sand.tiling_m) - 512.0 * UNIT_METERS_F64).abs() < 1e-5);
}

#[test]
fn tile_ids_round_trip() {
    for grid in [(0, 0), (-1, 5), (i32::MIN, i32::MAX), (-42, -17)] {
        assert_eq!(terrain::tile_grid(terrain::tile_id(grid)), grid);
    }
}

#[test]
fn heights_accumulate_rows_then_columns() {
    let mut deltas = Box::new([[0i8; 65]; 65]);
    deltas[0][0] = 2; // first vertex: offset + 2
    deltas[0][1] = 1; // east of it: +1
    deltas[1][0] = -1; // next row starts from the previous row's first vertex
    let h = terrain::decode_heights(10.0, &deltas);
    assert!((h[0] - 12.0 * 8.0).abs() < f32::EPSILON);
    assert!((h[1] - 13.0 * 8.0).abs() < f32::EPSILON);
    assert!((h[64] - 13.0 * 8.0).abs() < f32::EPSILON, "rest of the row carries the sum");
    assert!((h[65] - 11.0 * 8.0).abs() < f32::EPSILON);
    assert!((h[66] - 11.0 * 8.0).abs() < f32::EPSILON);
}

#[test]
fn texture_indices_transpose_from_blocks() {
    let mut raw = [[0u16; 16]; 16];
    for (i, v) in raw.iter_mut().flatten().enumerate() {
        *v = u16::try_from(i).unwrap();
    }
    let t = terrain::transpose_textures(&raw);
    // The first 16 stored values are the south-west 4×4 block.
    assert_eq!(t[0], 0);
    assert_eq!(t[1], 1);
    assert_eq!(t[16], 4);
    assert_eq!(t[3 * 16 + 3], 15);
    // The second block sits east of the first.
    assert_eq!(t[4], 16);
    // The fifth block starts the second block row.
    assert_eq!(t[4 * 16], 64);
}

fn bumpy(grid: (i32, i32)) -> LandData {
    let heights = (0..LAND_VERTS * LAND_VERTS)
        .map(|k| {
            #[allow(clippy::cast_precision_loss)]
            let (x, y) = ((k % LAND_VERTS) as f32, (k / LAND_VERTS) as f32);
            ((x * 1.7).sin() * 300.0 + (y * 0.9).cos() * 200.0 + x * y).round()
        })
        .collect();
    LandData { grid, heights, colors: None, textures: None }
}

#[test]
fn engine_tiles_reproduce_the_source_surface() {
    let grids = [(3, -2), (4, -2)];
    let lands: Vec<LandData> = grids.into_iter().map(bumpy).collect();
    let tiles: Vec<_> = lands
        .iter()
        .map(|l| (terrain::tile_id(l.grid), terrain::source_tile_from(l, &mut |_| RecordId::new("x"))))
        .collect();
    let mosaic = SourceMosaic::new(tiles, terrain::fill_height(), &mut |_| LayerId(0)).unwrap();
    let mut worst = 0.0f64;
    for land in &lands {
        for s in 0..400u16 {
            // Points scattered over the cell, including both triangles of quads.
            let (fx, fy) = (f32::from(s % 20) / 20.0 + 0.013, f32::from(s / 20) / 20.0 + 0.029);
            let (lx, ly) = (fx * 8192.0, fy * 8192.0);
            let want = f64::from(land.height_at(lx, ly)) * UNIT_METERS_F64;
            let mw_x = f64::from(land.grid.0) * 8192.0 + f64::from(lx);
            let mw_y = f64::from(land.grid.1) * 8192.0 + f64::from(ly);
            let got = mosaic.reference_height(mw_x * UNIT_METERS_F64, -mw_y * UNIT_METERS_F64);
            worst = worst.max((f64::from(got) - want).abs());
        }
    }
    assert!(worst < 1e-3, "engine surface differs from the source by {worst} m");
}

#[test]
fn texture_grid_lands_one_vertex_east_and_south() {
    let mut land = bumpy((0, 0));
    let mut textures = [0u16; LAND_TEXELS * LAND_TEXELS];
    textures[0] = 7; // Morrowind texel (0, 0): the south-west one.
    land.textures = Some(textures);
    let tile = terrain::source_tile_from(&land, &mut |v| RecordId::from(format!("t{v}")));
    let layers = tile.layers.unwrap();
    let marked = layers.palette.iter().position(|p| p.as_str() == "t7").unwrap();
    let at: Vec<usize> = layers.texels.iter().enumerate().filter(|(_, t)| usize::from(**t) == marked).map(|(i, _)| i).collect();
    // Engine rows run north to south: the south-west texel is row 15, column 0.
    assert_eq!(at, [15 * 16]);
    let origin = layers.origin.to_f64();
    let want = DVec3::new(128.0, 0.0, -8064.0) * UNIT_METERS_F64;
    assert!((origin - want).abs().max_element() < 1e-5, "{origin} vs {want}");
    // Heights: sample (0, 0) is the north-west vertex.
    let h = tile.heights[0];
    assert!((f64::from(h) - f64::from(land.heights[64 * 65]) * UNIT_METERS_F64).abs() < 1e-4);
}
