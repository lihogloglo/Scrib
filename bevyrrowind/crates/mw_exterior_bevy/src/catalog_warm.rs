//! Pipeline warm-up W4b1 (`superfargoth/docs/specs/pipeline-warmup.md` §5.3 row 2, §9): the game's
//! side of `sf_world_bevy::CatalogWarmup`.
//!
//! - Entering `Loading` loads the profile's `pipeline.catalog` from the bake store (once per
//!   session) and starts the warm-up; the framework requests every representative and holds the
//!   reveal on it (`warmup: catalog ...` line when it resolves).
//! - Skinned representatives (`mesh.rig` creature bodies, body parts) map their materials like the
//!   character path does (`MwCharacters::material`: `cooked_material`, no companions, no fold).
//! - Builtins: `terrain`, `water`, `precipitation` take their systems' own live entities (mesh and
//!   material as those systems built them, flags from the entity); before one exists (the terrain
//!   in an interior, no sea yet) it is `NotYet` and retried on later reveals (a door hold). `sky`
//!   is skipped: Bevy's `Atmosphere` and the night sky are view pipelines, not mesh materials,
//!   created at view setup and already held by the gate's `waiting_pipelines()` term.

use std::collections::HashSet;
use std::sync::Arc;

use bevy::camera::visibility::VisibilityRange;
use bevy::light::NotShadowCaster;
use bevy::pbr::{Lightmap, Material, MeshMaterial3d};
use bevy::prelude::*;
use sf_app_bevy::AppPhase;
use sf_bake::{Manifest, Store};
use sf_world_bevy::{BuiltinOutcome, BuiltinSpecimens, CatalogWarmup, ModelMaterials, WarmSpecimen, specimen_flags};

/// Registers the skinned material mapping and the builtins, and the catalog load on `Loading`.
pub fn add(app: &mut App, store: Store, manifest: Arc<Manifest>, vfs: Arc<sf_vfs::Vfs>) {
    let world = app.world_mut();
    if let Some(mut warmup) = world.get_resource_mut::<CatalogWarmup>() {
        warmup.skinned_materials = Some(ModelMaterials(Arc::new(move |m, s| mw_assets::cooked::cooked_material(m, &vfs, s))));
    }
    if let Some(mut builtins) = world.get_resource_mut::<BuiltinSpecimens>() {
        builtins
            .register("terrain", live::<sf_terrain_bevy::TerrainMaterial>)
            .register("water", live::<mw_environment_bevy::water::WaterMaterial>)
            .register("precipitation", live::<sf_weather_bevy::sf_precip_bevy::PrecipMaterial>)
            .register("sky", |_| {
                BuiltinOutcome::Skip("not a mesh material (Atmosphere and night-sky view pipelines: the gate's waiting_pipelines term)".to_owned())
            });
    }
    app.add_systems(OnEnter(AppPhase::Loading), move |warmup: Option<ResMut<CatalogWarmup>>| {
        let Some(mut warmup) = warmup else { return };
        if !warmup.has_catalog() {
            if let Some(catalog) = sf_model::catalog::load_catalog(&store, &manifest) {
                info!(
                    "warmup: catalog loaded: {} specimens ({} skinned), {} builtins",
                    catalog.entries.len(),
                    catalog.skinned(),
                    catalog.builtins.len()
                );
                warmup.set_catalog(catalog);
            } else {
                info!("warmup: no pipeline.catalog in the bake (run `bevyrrowind bake`): no catalog warm-up");
            }
        }
        warmup.start();
    });
}

/// The live entities drawn with `M`: one specimen per distinct (mesh, material, flags).
fn live<M: Material>(world: &mut World) -> BuiltinOutcome {
    if !world.contains_resource::<Assets<M>>() {
        return BuiltinOutcome::Skip("its plugin is not in this app".to_owned());
    }
    let mut query = world.query::<(&Mesh3d, &MeshMaterial3d<M>, Has<NotShadowCaster>, Option<&VisibilityRange>, Option<&Lightmap>)>();
    let mut seen = HashSet::new();
    let specimens: Vec<WarmSpecimen> = query
        .iter(world)
        .filter_map(|(mesh, material, no_shadow, range, lightmap)| {
            let flags = specimen_flags(!no_shadow, range, lightmap, false, false);
            seen.insert((mesh.id(), material.id(), flags.bits())).then(|| WarmSpecimen::new(mesh.0.clone(), material.0.clone(), flags))
        })
        .collect();
    if specimens.is_empty() { BuiltinOutcome::NotYet("no live entity yet".to_owned()) } else { BuiltinOutcome::Ready(specimens) }
}
