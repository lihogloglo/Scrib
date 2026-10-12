//! The Morrowind side of the bake (`superfargoth/docs/specs/bake.md`).
//!
//! - [`NifImporter`]: NIF → `sf_model::Model` (spec:
//!   `docs/specs/nif-runtime.md`, "Bake adapter").
//! - [`VfsInputs`]: `sf_bake` inputs over the mounted VFS and the records.
//! - [`anim`]: actor animation (`skeleton.cook`, `anim.cook`,
//!   `animgraph.compile` with Morrowind rules) and [`anim::AnimLibrary`],
//!   the Bevy-free loading API.
//! - [`collision`]: the `collision.cook` rules (static collision for
//!   `StaticProbe`), registered by [`model_baker`].
//! - [`referenced_models`] and [`bake_models`]: what `bevyrrowind bake`
//!   runs, and [`BakeSummary`], what it prints.
//!
//! Bevy-free; the viewer reads the products through `sf_model_bevy`.

pub mod anim;
pub mod body;
pub mod collision;
mod importer;
mod inputs;
mod summary;
pub mod far;
pub mod script;
pub mod textures;
pub mod world;

pub use importer::{DROP_FRACTION, Dynamic, NifImporter, dynamic_parts, extras, to_model};
pub use inputs::VfsInputs;
pub use summary::{BakeSummary, LodLevelStats};

use std::collections::BTreeMap;
use std::path::PathBuf;
use std::sync::Arc;

use mw_install::MorrowindInstall;
use sf_bake::{BakeReport, Baker, Digest, ProductKey, Store};
use sf_model::steps::{Importers, MeshCookStep, MeshRestStep, MeshRigStep, ModelInfoStep};
use sf_model::{CookSettings, GltfImporter};
use sf_records::{RecordDb, RecordId};
use tes3::esp::{
    Activator, Alchemy, Apparatus, Armor, Bodypart, Book, Cell, Clothing, Container, Creature, Door, Ingredient, Light,
    Lockpick, MiscItem, Npc, Probe, RepairItem, Static, Weapon,
};

/// File extension of `mesh.cook` products in `baked://` asset paths.
pub const SFMESH_EXT: &str = "sfmesh";

/// The default cache root: `<local data dir>/bevyrrowind/bake`
/// (`%LOCALAPPDATA%` on Windows, `~/.local/share` on Linux).
pub fn default_cache_root() -> Option<PathBuf> {
    dirs::data_local_dir().map(|d| d.join("bevyrrowind").join("bake"))
}

/// The bake profile for an install's mod set: archives and content files
/// in load order. Loose files are tracked per product, not by profile.
pub fn profile_name(install: &MorrowindInstall) -> String {
    let mut set = String::new();
    for name in install.archives().into_iter().chain(install.game_files()) {
        set.push_str(&name.to_ascii_lowercase());
        set.push('\n');
    }
    format!("mw-{}", &Digest::of(set.as_bytes()).hex()[..12])
}

/// The importers the bake uses: NIF, plus the framework's glTF (a mod may
/// point a record at a `.glb`).
pub fn importers() -> Importers {
    Importers::new().with(Arc::new(GltfImporter)).with(Arc::new(NifImporter))
}

/// A record's mesh field as a VFS path under `meshes/` (the same rule as
/// `mw_world::model_path`).
pub fn model_path(mesh: &str) -> Option<String> {
    let normalized = mesh.trim().replace('\\', "/").to_ascii_lowercase();
    let normalized = normalized.trim_start_matches('/');
    if normalized.is_empty() {
        return None;
    }
    Some(if normalized.starts_with("meshes/") { normalized.to_owned() } else { format!("meshes/{normalized}") })
}

/// One model referenced by records.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ModelUse {
    /// Base records naming it.
    pub records: u32,
    /// Placed references (in any cell) whose base names it.
    pub placements: u32,
}

/// Every model path named by a record with a mesh (statics, doors,
/// containers, activators, lights, items, actors, body parts), with how
/// often records and placed references use it.
pub fn referenced_models(db: &RecordDb) -> BTreeMap<String, ModelUse> {
    let mut models: BTreeMap<String, ModelUse> = BTreeMap::new();
    let mut by_base: std::collections::HashMap<RecordId, String> = std::collections::HashMap::new();
    macro_rules! collect {
        ($($ty:ty),* $(,)?) => {$(
            for (id, record) in db.iter::<$ty>() {
                if let Some(path) = model_path(&record.mesh) {
                    models.entry(path.clone()).or_default().records += 1;
                    by_base.insert(id.clone(), path);
                }
            }
        )*};
    }
    collect!(
        Static, Door, Container, Activator, Light, MiscItem, Weapon, Armor, Clothing, Book, Alchemy, Apparatus,
        Ingredient, Lockpick, Probe, RepairItem, Creature, Npc, Bodypart,
    );
    for (_, cell) in db.iter::<Cell>() {
        for reference in cell.references.values() {
            if let Some(path) = by_base.get(&RecordId::new(&reference.id))
                && let Some(model) = models.get_mut(path)
            {
                model.placements += 1;
            }
        }
    }
    models
}

/// Bakes `model.info` and `mesh.cook` for every model, in parallel.
/// Returns the session's report and the manifest of what resolved.
pub fn bake_models(store: Store, inputs: Arc<VfsInputs>, models: &[String]) -> (BakeReport, sf_bake::Manifest) {
    let baker = model_baker(store, inputs);
    let report = baker.bake(&model_keys(models));
    (report, baker.manifest())
}

/// A baker with the model steps (`model.info`, `mesh.cook`, `mesh.rest`
/// for the far field's use of models `mesh.cook` skips, `mesh.rig` for
/// creature bodies ([`anim::ActorSets::body_rig_keys`]), and
/// `collision.cook`, see [`collision`]) registered. Registering bakes
/// nothing: callers pick the keys ([`model_keys`],
/// [`collision::collision_keys`]).
pub fn model_baker(store: Store, inputs: Arc<VfsInputs>) -> Baker {
    let mut baker = Baker::new(store, inputs);
    baker
        .register(ModelInfoStep { importers: importers() })
        .register(MeshCookStep { importers: importers(), settings: CookSettings::default() })
        .register(MeshRestStep { importers: importers(), settings: CookSettings::default() })
        .register(MeshRigStep { importers: importers(), settings: CookSettings::default() });
    collision::register(&mut baker);
    body::register(&mut baker);
    baker
}

/// `model.info` and `mesh.cook` of every model.
pub fn model_keys(models: &[String]) -> Vec<ProductKey> {
    models
        .iter()
        .flat_map(|m| [ProductKey::new(ModelInfoStep::ID, m.as_str()), ProductKey::new(MeshCookStep::ID, m.as_str())])
        .collect()
}

/// Which of `models` the profile has cooked statically: a `mesh.cook`
/// product exists and `model.info` says nothing animates. Reads only the
/// small info products. The rest takes the runtime NIF path.
pub fn cooked_models<'a>(
    store: &Store,
    manifest: &sf_bake::Manifest,
    models: impl IntoIterator<Item = &'a str>,
) -> std::collections::HashSet<String> {
    models
        .into_iter()
        .filter(|path| manifest.get(MeshCookStep::ID, path).is_some())
        .filter(|path| {
            let info = manifest.get(ModelInfoStep::ID, path).and_then(|d| store.get(&d).ok());
            info.and_then(|bytes| ModelInfoStep::decode(&bytes).ok()).is_some_and(|i| !i.animated && !i.skinned)
        })
        .map(str::to_owned)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn model_paths_normalize() {
        assert_eq!(model_path("x\\Ex_Rock.NIF").as_deref(), Some("meshes/x/ex_rock.nif"));
        assert_eq!(model_path("meshes\\a.nif").as_deref(), Some("meshes/a.nif"));
        assert_eq!(model_path("  "), None);
    }
}
