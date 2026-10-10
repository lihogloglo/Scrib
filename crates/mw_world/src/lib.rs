//! Morrowind cells as engine-neutral placements.
//!
//! A [`Placement`] is one reference resolved against its base record: what
//! to show (a model path), where (engine-space transform), and any light it
//! emits. Engines spawn placements however they like.

use mw_nif::coords::{self, Trs};
use sf_records::{RecordDb, RecordId};
use tes3::esp::{
    Activator, Alchemy, Apparatus, Armor, Book, Cell, Clothing, Container, Creature, Door, Ingredient, Light,
    LightFlags, Lockpick, MiscItem, Npc, Probe, Reference, RepairItem, Static, Weapon,
};
use tes3::nif::glam::Vec3;

/// What kind of base record a reference points at.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum BaseKind {
    Static,
    Door,
    Container,
    Activator,
    Light,
    Item,
    /// NPCs and creatures: assembled actors, not plain models (M3).
    Actor,
}

/// A light emitted by a placement.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LightInfo {
    /// sRGB color, 0..=1.
    pub color: [f32; 3],
    /// Radius in meters.
    pub radius: f32,
    pub flicker: bool,
    pub pulse: bool,
    pub fire: bool,
}

/// Base records the engine treats as editor-only markers: placed in cells
/// for gameplay (interior north, teleport and intervention targets) but
/// never rendered. Their meshes carry no marker flag; the ID is the signal.
pub const EDITOR_MARKERS: &[&str] =
    &["northmarker", "doormarker", "travelmarker", "templemarker", "divinemarker", "prisonmarker"];

/// One resolved reference.
#[derive(Clone, Debug, PartialEq)]
pub struct Placement {
    pub base_id: String,
    pub kind: BaseKind,
    /// VFS path of the model, e.g. `meshes/x/ex_hlaalu_b_01.nif`.
    pub model: Option<String>,
    pub transform: Trs,
    pub light: Option<LightInfo>,
    /// Gameplay-only marker (see [`EDITOR_MARKERS`]): keep, don't render.
    pub editor_only: bool,
}

/// Why a reference produced no placement.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Skipped {
    /// The base record does not exist in any kind we place.
    UnknownBase(String),
}

/// Normalizes a record's mesh field to a VFS path under `meshes/`.
pub fn model_path(mesh: &str) -> Option<String> {
    let normalized = mesh.trim().replace('\\', "/").to_ascii_lowercase();
    let normalized = normalized.trim_start_matches('/');
    if normalized.is_empty() {
        return None;
    }
    Some(if normalized.starts_with("meshes/") { normalized.to_owned() } else { format!("meshes/{normalized}") })
}

/// Resolves a base record ID across every placeable kind.
pub fn resolve_base(db: &RecordDb, id: &str) -> Option<(BaseKind, Option<String>, Option<LightInfo>)> {
    let key = RecordId::new(id);
    macro_rules! try_kinds {
        ($($ty:ty => $kind:expr),* $(,)?) => {
            $(if let Some(record) = db.get::<$ty>(&key) {
                return Some(($kind, model_path(&record.mesh), None));
            })*
        };
    }
    if let Some(light) = db.get::<Light>(&key) {
        return Some((BaseKind::Light, model_path(&light.mesh), light_info(light)));
    }
    try_kinds! {
        Static => BaseKind::Static, Door => BaseKind::Door, Container => BaseKind::Container,
        Activator => BaseKind::Activator,
        MiscItem => BaseKind::Item, Weapon => BaseKind::Item, Armor => BaseKind::Item,
        Clothing => BaseKind::Item, Book => BaseKind::Item, Alchemy => BaseKind::Item,
        Apparatus => BaseKind::Item, Ingredient => BaseKind::Item, Lockpick => BaseKind::Item,
        Probe => BaseKind::Item, RepairItem => BaseKind::Item,
        Npc => BaseKind::Actor, Creature => BaseKind::Actor,
    }
    // Leveled lists resolve at runtime (M7); unknown otherwise.
    None
}

fn light_info(light: &Light) -> Option<LightInfo> {
    let flags = light.data.flags;
    // Negative lights darken; no modern renderer equivalent. Lights that
    // start off are switched on by scripts later.
    if flags.contains(LightFlags::NEGATIVE) || flags.contains(LightFlags::OFF_BY_DEFAULT) || light.data.radius == 0 {
        return None;
    }
    let [r, g, b, _] = light.data.color;
    #[allow(clippy::cast_precision_loss)]
    let radius = light.data.radius as f32 * coords::UNIT_METERS;
    Some(LightInfo {
        color: [f32::from(r) / 255.0, f32::from(g) / 255.0, f32::from(b) / 255.0],
        radius,
        flicker: flags.intersects(LightFlags::FLICKER | LightFlags::FLICKER_SLOW),
        pulse: flags.intersects(LightFlags::PULSE | LightFlags::PULSE_SLOW),
        fire: flags.contains(LightFlags::FIRE),
    })
}

/// A reference's engine-space transform.
pub fn reference_transform(reference: &Reference) -> Trs {
    Trs {
        translation: coords::point(Vec3::from_array(reference.translation)).to_array(),
        rotation: coords::reference_rotation(reference.rotation),
        scale: reference.scale.unwrap_or(1.0),
    }
}

/// Resolves every reference of a cell, sorted by reference key for
/// deterministic output.
pub fn cell_placements(db: &RecordDb, cell: &Cell) -> (Vec<Placement>, Vec<Skipped>) {
    let mut references: Vec<(&(u32, u32), &Reference)> = cell.references.iter().collect();
    references.sort_unstable_by_key(|(key, _)| **key);

    let mut placements = Vec::with_capacity(references.len());
    let mut skipped = Vec::new();
    for (_, reference) in references {
        match resolve_base(db, &reference.id) {
            Some((kind, model, light)) => placements.push(Placement {
                base_id: reference.id.clone(),
                kind,
                model,
                transform: reference_transform(reference),
                light,
                editor_only: EDITOR_MARKERS.iter().any(|m| reference.id.eq_ignore_ascii_case(m)),
            }),
            None => skipped.push(Skipped::UnknownBase(reference.id.clone())),
        }
    }
    (placements, skipped)
}

/// Finds a cell by interior name or exterior grid (`"x,y"` or `"@x,y"`).
pub fn find_cell<'a>(db: &'a RecordDb, name: &str) -> Option<&'a Cell> {
    let trimmed = name.trim().trim_start_matches('@');
    if let Some((x, y)) = trimmed.split_once(',')
        && let (Ok(x), Ok(y)) = (x.trim().parse::<i32>(), y.trim().parse::<i32>())
    {
        return db.get::<Cell>(&mw_records::keys::exterior((x, y)));
    }
    db.get::<Cell>(&RecordId::new(name.trim()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use sf_vfs::{MemSource, Vfs};
    use tes3::esp::{CellFlags, FileType, Header, LightData, Plugin, TES3Object};

    fn content() -> mw_records::LoadedContent {
        let mut cell = Cell { name: "Test Room".into(), ..Cell::default() };
        cell.data.flags = CellFlags::IS_INTERIOR;
        for (i, id) in ["chair", "torch", "ghost", "fargoth", "NorthMarker"].iter().enumerate() {
            let index = u32::try_from(i).unwrap() + 1;
            let mut reference = Reference { refr_index: index, id: (*id).into(), ..Reference::default() };
            reference.translation = [64.0, 0.0, 0.0];
            reference.scale = Some(2.0);
            cell.references.insert((0, index), reference);
        }
        let objects: Vec<TES3Object> = vec![
            Header { file_type: FileType::Esm, ..Header::default() }.into(),
            Static { id: "chair".into(), mesh: "f\\Chair.NIF".into(), ..Static::default() }.into(),
            Light {
                id: "torch".into(),
                mesh: "l\\torch.nif".into(),
                data: LightData { radius: 256, color: [255, 128, 0, 0], flags: LightFlags::FLICKER, ..LightData::default() },
                ..Light::default()
            }
            .into(),
            Npc { id: "fargoth".into(), ..Npc::default() }.into(),
            Static { id: "NorthMarker".into(), mesh: "marker_north.nif".into(), ..Static::default() }.into(),
            cell.into(),
        ];
        let bytes = Plugin { objects }.save_bytes().unwrap();
        let vfs = Vfs::builder().mount(MemSource::new("data").with("Test.esm", bytes)).build();
        mw_records::load(&vfs, &["Test.esm".to_owned()]).unwrap()
    }

    #[test]
    fn places_statics_lights_and_actors() {
        let loaded = content();
        let cell = find_cell(&loaded.db, "test room").unwrap();
        let (placements, skipped) = cell_placements(&loaded.db, cell);

        assert_eq!(skipped, [Skipped::UnknownBase("ghost".into())]);
        let kinds: Vec<BaseKind> = placements.iter().map(|p| p.kind).collect();
        assert_eq!(kinds, [BaseKind::Static, BaseKind::Light, BaseKind::Actor, BaseKind::Static]);
        assert!(placements[3].editor_only && !placements[0].editor_only);

        let chair = &placements[0];
        assert_eq!(chair.model.as_deref(), Some("meshes/f/chair.nif"));
        assert!((chair.transform.translation[0] - 64.0 * coords::UNIT_METERS).abs() < 1e-6);
        assert!((chair.transform.scale - 2.0).abs() < f32::EPSILON);

        let light = placements[1].light.unwrap();
        assert!(light.flicker && !light.fire);
        assert!((light.radius - 256.0 * coords::UNIT_METERS).abs() < 1e-5);
        assert!((light.color[1] - 128.0 / 255.0).abs() < 1e-6);
    }

    #[test]
    fn model_paths_normalize() {
        assert_eq!(model_path("x\\Ex_Hlaalu.NIF").as_deref(), Some("meshes/x/ex_hlaalu.nif"));
        assert_eq!(model_path("meshes\\a.nif").as_deref(), Some("meshes/a.nif"));
        assert_eq!(model_path("  "), None);
    }

    #[test]
    fn finds_exterior_cells_by_grid() {
        let loaded = content();
        assert!(find_cell(&loaded.db, "-2, -9").is_none());
        assert!(find_cell(&loaded.db, "Test Room").is_some());
    }
}
