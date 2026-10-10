use super::*;

use sf_vfs::MemSource;
use tes3::esp::{CellFlags, DialogueInfo, Header, Landscape, Npc, ObjectFlags, Reference, Static};

fn header(file_type: FileType, masters: &[&str]) -> TES3Object {
    Header { file_type, masters: masters.iter().map(|m| ((*m).to_owned(), 0)).collect(), ..Header::default() }.into()
}

fn stat(id: &str, mesh: &str) -> TES3Object {
    Static { id: id.into(), mesh: mesh.into(), ..Static::default() }.into()
}

fn npc(id: &str, name: &str) -> TES3Object {
    Npc { id: id.into(), name: name.into(), ..Npc::default() }.into()
}

fn reference(mast_index: u32, refr_index: u32, id: &str) -> Reference {
    Reference { mast_index, refr_index, id: id.into(), ..Reference::default() }
}

fn interior(name: &str, refs: Vec<Reference>) -> TES3Object {
    let mut cell = Cell { name: name.into(), ..Cell::default() };
    cell.data.flags = CellFlags::IS_INTERIOR;
    for r in refs {
        cell.references.insert((r.mast_index, r.refr_index), r);
    }
    cell.into()
}

fn exterior(name: &str, grid: (i32, i32)) -> TES3Object {
    let mut cell = Cell { name: name.into(), ..Cell::default() };
    cell.data.grid = grid;
    cell.into()
}

fn bytes(objects: Vec<TES3Object>) -> Vec<u8> {
    Plugin { objects }.save_bytes().unwrap()
}

fn deleted<T: ObjectInfo + Into<TES3Object>>(mut record: T) -> TES3Object {
    record.set_deleted(true);
    record.into()
}

/// Morrowind.esm, Tribunal.esm (master: Morrowind), and two independent
/// plugins that both touch Fargoth.
fn fixture() -> Vfs {
    let mut topic = Dialogue { id: "Background".into(), ..Dialogue::default() };
    let morrowind = bytes(vec![
        header(FileType::Esm, &[]),
        stat("rock_01", "r\\rock.nif"),
        stat("doomed", "d.nif"),
        npc("fargoth", "Fargoth"),
        interior("Seyda Neen, Census Office", vec![reference(0, 1, "rock_01"), reference(0, 2, "fargoth")]),
        exterior("Seyda Neen", (-2, -9)),
        Landscape { grid: (-2, -9), ..Landscape::default() }.into(),
        topic.clone().into(),
        DialogueInfo { id: "1001".into(), text: "Hello".into(), ..DialogueInfo::default() }.into(),
    ]);
    topic.flags = ObjectFlags::default();
    let tribunal = bytes(vec![
        header(FileType::Esm, &["Morrowind.esm"]),
        npc("FARGOTH", "Fargoth the Tribunal"),
        deleted(Static { id: "doomed".into(), ..Static::default() }),
        // Moves rock 1 from Morrowind (master index 1), adds its own ref.
        interior("seyda neen, census office", vec![
            Reference { translation: [5.0, 0.0, 0.0], ..reference(1, 1, "rock_01") },
            reference(0, 1, "rock_01"),
        ]),
        // Same grid, different display name: still the same cell.
        exterior("Bitter Coast Region", (-2, -9)),
    ]);
    let mod_a = bytes(vec![header(FileType::Esp, &["Morrowind.esm"]), npc("fargoth", "Fargoth A")]);
    let mod_b = bytes(vec![
        header(FileType::Esp, &["Morrowind.esm"]),
        npc("fargoth", "Fargoth B"),
        // Deletes Morrowind's Fargoth ref from the census office.
        interior("Seyda Neen, Census Office", vec![Reference { deleted: Some(true), ..reference(1, 2, "fargoth") }]),
    ]);
    Vfs::builder()
        .mount(
            MemSource::new("Data Files")
                .with("Morrowind.esm", morrowind)
                .with("Tribunal.esm", tribunal)
                .with("Mod A.esp", mod_a)
                .with("Mod B.esp", mod_b),
        )
        .build()
}

fn names(list: &[&str]) -> Vec<String> {
    list.iter().map(|s| (*s).to_owned()).collect()
}

#[test]
fn esms_load_before_esps_and_masters_first() {
    let vfs = fixture();
    // Configured badly: plugin first, Tribunal before Morrowind.
    let loaded = load(&vfs, &names(&["Mod A.esp", "Tribunal.esm", "Morrowind.esm"])).unwrap();
    let order: Vec<&str> = loaded.files.iter().map(|f| f.name.as_str()).collect();
    assert_eq!(order, ["Morrowind.esm", "Tribunal.esm", "Mod A.esp"]);
    assert_eq!(loaded.files[1].masters, ["Morrowind.esm"]);
}

#[test]
fn bloodmoon_loads_after_tribunal_even_when_listed_first() {
    let mw = bytes(vec![header(FileType::Esm, &[])]);
    let tr = bytes(vec![header(FileType::Esm, &["Morrowind.esm"]), npc("x", "tribunal")]);
    let bm = bytes(vec![header(FileType::Esm, &["Morrowind.esm"]), npc("x", "bloodmoon")]);
    let vfs = Vfs::builder()
        .mount(MemSource::new("d").with("Morrowind.esm", mw).with("Tribunal.esm", tr).with("Bloodmoon.esm", bm))
        .build();
    // Steam's Morrowind.ini lists the files alphabetically.
    let loaded = load(&vfs, &names(&["Bloodmoon.esm", "Morrowind.esm", "Tribunal.esm"])).unwrap();
    let order: Vec<&str> = loaded.files.iter().map(|f| f.name.as_str()).collect();
    assert_eq!(order, ["Morrowind.esm", "Tribunal.esm", "Bloodmoon.esm"]);
    assert_eq!(loaded.db.get::<Npc>(&"x".into()).unwrap().name, "bloodmoon");
}

#[test]
fn later_files_replace_and_delete_records() {
    let loaded = load(&fixture(), &names(&["Morrowind.esm", "Tribunal.esm"])).unwrap();
    let db = &loaded.db;
    assert_eq!(db.get::<Npc>(&"fargoth".into()).unwrap().name, "Fargoth the Tribunal");
    assert!(db.get::<Static>(&"doomed".into()).is_none());
    assert_eq!(db.len::<Static>(), 1);
    assert_eq!(db.len::<Landscape>(), 1);
    assert!(db.get::<Landscape>(&keys::exterior((-2, -9))).is_some());
}

#[test]
fn cells_merge_references_with_global_keys() {
    let loaded = load(&fixture(), &names(&["Morrowind.esm", "Tribunal.esm", "Mod B.esp"])).unwrap();
    let mw = loaded.files[0].source.0;
    let tr = loaded.files[1].source.0;
    let cell = loaded.db.get::<Cell>(&"seyda neen, census office".into()).unwrap();

    let mut keys: Vec<(u32, u32)> = cell.references.keys().copied().collect();
    keys.sort_unstable();
    // Morrowind's rock (moved by Tribunal) and Tribunal's own rock 1 are
    // distinct; Morrowind's Fargoth ref was deleted by Mod B.
    assert_eq!(keys, [(mw, 1), (tr, 1)]);
    assert_eq!(cell.references[&(mw, 1)].translation[0].to_bits(), 5.0f32.to_bits());
    assert_eq!(cell.references[&(tr, 1)].mast_index, tr);
    assert!(loaded.warnings.is_empty(), "{:?}", loaded.warnings);
}

#[test]
fn exterior_cells_are_keyed_by_grid() {
    let loaded = load(&fixture(), &names(&["Morrowind.esm", "Tribunal.esm"])).unwrap();
    let cell = loaded.db.get::<Cell>(&keys::exterior((-2, -9))).unwrap();
    assert_eq!(cell.name, "Bitter Coast Region");
    assert_eq!(loaded.db.len::<Cell>(), 2);
}

#[test]
fn infos_are_keyed_under_their_topic() {
    let loaded = load(&fixture(), &names(&["Morrowind.esm"])).unwrap();
    let (id, info) = loaded.db.iter::<DialogueInfo>().next().unwrap();
    assert_eq!(keys::split_info(id), Some(("background", "1001")));
    assert_eq!(info.text, "Hello");
}

#[test]
fn independent_plugins_conflict_but_dlc_does_not() {
    let loaded = load(&fixture(), &names(&["Morrowind.esm", "Tribunal.esm", "Mod A.esp", "Mod B.esp"])).unwrap();
    let report = loaded.db.conflict_report();
    let npcs = report.kinds.iter().find(|k| k.kind == "NPC_").unwrap();
    // Tribunal -> Mod A -> Mod B all touched Fargoth; neither mod knows the
    // other (nor Tribunal), so it's a conflict. Mod B wins.
    assert_eq!(npcs.conflicts.len(), 1);
    assert_eq!(loaded.db.get::<Npc>(&"fargoth".into()).unwrap().name, "Fargoth B");
    // Tribunal's census-office edits were intended (it depends on Morrowind);
    // Mod B's deletion touches a cell Tribunal also edited, without knowing it.
    let cells = report.kinds.iter().find(|k| k.kind == "CELL").unwrap();
    assert_eq!(cells.conflicts.len(), 1);
}

#[test]
fn missing_files_and_masters_are_errors() {
    let vfs = fixture();
    assert!(matches!(load(&vfs, &names(&["Nope.esp"])), Err(LoadError::NotFound(_))));
    assert!(matches!(
        load(&vfs, &names(&["Tribunal.esm"])),
        Err(LoadError::Order(LoadOrderError::MissingDependency { .. }))
    ));
}

#[test]
fn mod_ids_are_sanitized() {
    assert_eq!(mod_id_for("Mod A (v2).esp").unwrap().as_str(), "mod_a__v2_.esp");
    assert_eq!(mod_id_for("Morrowind.esm").unwrap().as_str(), "morrowind.esm");
}
