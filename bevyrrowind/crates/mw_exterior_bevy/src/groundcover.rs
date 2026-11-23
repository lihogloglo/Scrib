//! Groundcover in the exterior view (slice GC-2b3; specs `superfargoth/docs/specs/groundcover.md` "Renderers",
//! `bevyrrowind/docs/specs/groundcover-adapter.md`): the vanilla grass and fern models scattered on grass land
//! textures around the camera by `sf_groundcover_bevy`.
//!
//! - **Source**: the shipped species and rules (`mw_world::groundcover::GroundcoverData::builtin`), their LTEX
//!   channels resolved against the bake's terrain layer table (`BakedTerrain::layer_table`, the ids the layer maps
//!   carry), the ground the streamed terrain (`TerrainGround`) with the exterior record's sea level.
//! - **Models**: each model key loads as the runtime NIF (`mw_assets::NifLoader`, the statics' fallback path: the
//!   same texture resolution through the VFS and the `.ktx2` cap loader). The first visible mesh of the model is
//!   used, with its node transform baked in when it isn't the identity; the bend height is the renderer's (the mesh
//!   AABB top).
//! - **Viewer**: follows `TerrainCamera` (added and removed with it), so no ring moves in an interior; the batches
//!   draw on the exterior's render layer only, like the terrain patches (`mw_doors_bevy::terrain_layers`).
//! - **Off**: `--off groundcover` adds none of the renderer.
//!
//! Log line once every model resolved: `groundcover: species S, channels matched M/N, unmatched [..], models
//! loaded L/T`. Gauges `groundcover/*` (`sf_groundcover_bevy::publish_gauges`).

use std::sync::Arc;

use bevy::asset::{LoadState, RecursiveDependencyLoadState};
use bevy::camera::visibility::RenderLayers;
use bevy::prelude::*;
use bevy::world_serialization::WorldAsset;
use mw_world::groundcover::GroundcoverData;
use mw_world::ids;
use sf_bake::{Manifest, Store};
use sf_groundcover::Overrides;
use sf_groundcover_bevy::{
    GroundcoverBatch, GroundcoverModel, GroundcoverModels, GroundcoverPlugin, GroundcoverSource, GroundcoverViewer,
    LayerChannels, TerrainGround, TerrainGroundPlugin, TerrainGroundRef,
};
use sf_terrain_bevy::TerrainCamera;
use sf_terrain_bevy::sf_terrain::LayerTable;
use sf_terrain_bevy::sf_terrain::bake::BakedTerrain;
use sf_world::WorldspaceId;
use sf_world_bevy::{WorldspaceRecords, WorldspaceRegistry};

/// Groundcover in the exterior app. Add it after the wind plugin and the static scene's `GpuInstanceBatchPlugin`
/// (it adds that plugin itself when the scene left it out).
pub struct GroundcoverGamePlugin {
    pub store: Store,
    pub manifest: Arc<Manifest>,
    /// `--off groundcover`.
    pub off: bool,
}

/// What [`setup`] reads.
#[derive(Resource, Clone, Debug, Default)]
pub struct GroundcoverInputs {
    pub off: bool,
    /// The bake's terrain layer table; `None`: not baked (nothing is planted).
    pub table: Option<LayerTable>,
}

/// What [`setup`] built, for the ready log line.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct SetupReport {
    pub species: usize,
    /// Rule channels at least one terrain layer feeds.
    pub channels_matched: usize,
    pub channels: usize,
    /// LTEX names no layer carries.
    pub unmatched: Vec<String>,
    /// Model keys the species draw.
    pub models: usize,
}

/// The models still loading, and the counts for the log line.
#[derive(Resource, Debug, Default)]
pub struct GroundcoverLoading {
    pub report: SetupReport,
    pub keys: Vec<String>,
    pub pending: Vec<(String, Handle<WorldAsset>)>,
    /// The resolved NIFs, kept loaded.
    pub held: Vec<Handle<WorldAsset>>,
    pub loaded: usize,
    pub logged: bool,
}

impl Plugin for GroundcoverGamePlugin {
    fn build(&self, app: &mut App) {
        use sf_tunables_bevy::TunablesAppExt;
        if self.off {
            info!("groundcover: off (--off groundcover)");
            app.insert_resource(GroundcoverInputs { off: true, table: None });
            // FS-2c: the switch stays in the list, marked not built (mw_devtools_bevy part_switches).
            app.register_tunables(&[&sf_groundcover_bevy::tunables::ENABLED])
                .add_feature_switch(&sf_groundcover_bevy::tunables::GROUNDCOVER);
            return;
        }
        let table = match BakedTerrain::new(self.store.clone(), &self.manifest, ids::exterior()).layer_table() {
            Some(Ok(t)) => Some(t),
            Some(Err(e)) => {
                warn!("groundcover: terrain layer table: {e}");
                None
            }
            None => {
                warn!("groundcover: no baked terrain layer table: nothing is planted");
                None
            }
        };
        if !app.is_plugin_added::<bevy::pbr::gpu_instance_batch::GpuInstanceBatchPlugin>() {
            app.add_plugins(bevy::pbr::gpu_instance_batch::GpuInstanceBatchPlugin);
        }
        app.add_plugins((GroundcoverPlugin, TerrainGroundPlugin))
            .insert_resource(GroundcoverInputs { off: false, table })
            .add_systems(Startup, setup)
            .add_systems(Update, (load_models, batch_layers))
            .add_observer(viewer_on)
            .add_observer(viewer_off);
    }
}

/// The source and the terrain ground for `table`, and what matched.
pub fn build_source(
    data: &GroundcoverData,
    table: &LayerTable,
    ws: WorldspaceId,
    sea_level: Option<f32>,
) -> Result<(GroundcoverSource, Arc<TerrainGround>, SetupReport), String> {
    let cover = data.build().map_err(|e| e.to_string())?;
    let map = data.resolve(table);
    let mut fed: Vec<u16> = map.pairs.iter().map(|&(_, c)| c).collect();
    fed.sort_unstable();
    fed.dedup();
    let report = SetupReport {
        species: cover.len(),
        channels_matched: fed.len(),
        channels: data.channels.len(),
        unmatched: map.unmatched.clone(),
        models: data.model_keys().len(),
    };
    let ground = Arc::new(TerrainGround::new(ws, sea_level, LayerChannels::from_pairs(map.pairs)));
    let source = GroundcoverSource { cover: Arc::new(cover), overrides: Arc::new(Overrides::none()), ground: ground.clone() };
    Ok((source, ground, report))
}

/// `Startup`: inserts `GroundcoverSource`, `TerrainGroundRef` and the model list ([`GroundcoverLoading`]); nothing
/// under the off switch or without a layer table.
pub fn setup(mut commands: Commands, inputs: Option<Res<GroundcoverInputs>>, records: Option<Res<WorldspaceRecords>>) {
    let Some(inputs) = inputs else { return };
    if inputs.off {
        return;
    }
    let Some(table) = &inputs.table else { return };
    let data = match GroundcoverData::builtin() {
        Ok(d) => d,
        Err(e) => {
            error!("groundcover: {e}");
            return;
        }
    };
    let ws = ids::exterior();
    let sea_level = records.as_ref().and_then(|r| r.0.get(&ws)).and_then(|w| w.sea_level);
    if sea_level.is_none() {
        warn!("groundcover: the exterior record has no sea level: plants may stand under water");
    }
    match build_source(&data, table, ws, sea_level) {
        Ok((source, ground, report)) => {
            commands.insert_resource(source);
            commands.insert_resource(TerrainGroundRef(ground));
            commands.insert_resource(GroundcoverLoading { report, keys: data.model_keys(), ..default() });
        }
        Err(e) => error!("groundcover: {e}"),
    }
}

/// `Update`: starts every model load, then turns each loaded NIF into a `GroundcoverModel`; logs once all
/// resolved.
fn load_models(
    loading: Option<ResMut<GroundcoverLoading>>,
    server: Res<AssetServer>,
    worlds: Res<Assets<WorldAsset>>,
    standard: Res<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut models: ResMut<GroundcoverModels>,
) {
    let Some(mut loading) = loading else { return };
    let loading = &mut *loading;
    if !loading.keys.is_empty() {
        loading.pending = loading.keys.drain(..).map(|k| (k.clone(), server.load::<WorldAsset>(k))).collect();
    }
    let mut still = Vec::with_capacity(loading.pending.len());
    for (key, handle) in loading.pending.drain(..) {
        if matches!(server.load_state(&handle), LoadState::Failed(_)) {
            warn!("groundcover: {key}: failed to load");
            continue;
        }
        // Textures too (a failed texture still lets the model through, untextured).
        let settled = matches!(
            server.recursive_dependency_load_state(&handle),
            RecursiveDependencyLoadState::Loaded | RecursiveDependencyLoadState::Failed(_)
        );
        match worlds.get(&handle).filter(|_| settled) {
            Some(asset) => {
                if let Some(model) = model_of(&asset.world, &standard, &mut meshes) {
                    models.0.insert(key, model);
                    loading.loaded += 1;
                } else {
                    warn!("groundcover: {key}: no visible mesh");
                }
                loading.held.push(handle);
            }
            None => still.push((key, handle)),
        }
    }
    loading.pending = still;
    if loading.pending.is_empty() && !loading.logged {
        loading.logged = true;
        let r = &loading.report;
        info!(
            "groundcover: species {}, channels matched {}/{}, unmatched {:?}, models loaded {}/{}",
            r.species,
            r.channels_matched,
            r.channels,
            r.unmatched,
            loading.loaded,
            r.models
        );
    }
}

/// The first visible mesh of a NIF world and its material, the mesh in the model's frame.
fn model_of(world: &World, standard: &Assets<StandardMaterial>, meshes: &mut Assets<Mesh>) -> Option<GroundcoverModel> {
    let mut query = world.try_query::<(Entity, &Mesh3d, &MeshMaterial3d<StandardMaterial>)>()?;
    let mut found: Vec<(Entity, Handle<Mesh>, Handle<StandardMaterial>)> =
        query.iter(world).map(|(e, m, mat)| (e, m.0.clone(), mat.0.clone())).collect();
    // Spawn order is node order: the first node's mesh first.
    found.sort_by_key(|(e, _, _)| e.index());
    let (entity, mesh, material) = found.into_iter().find(|(e, _, _)| !hidden(world, *e))?;
    let material = standard.get(&material)?.clone();
    let frame = model_frame(world, entity);
    let mesh = if frame == Transform::IDENTITY {
        mesh
    } else {
        let baked = meshes.get(&mesh)?.clone().transformed_by(frame);
        meshes.add(baked)
    };
    Some(GroundcoverModel { mesh, material, bend_height_m: None })
}

/// Hidden itself or under a hidden node.
fn hidden(world: &World, mut e: Entity) -> bool {
    loop {
        let r = world.entity(e);
        if r.get::<Visibility>() == Some(&Visibility::Hidden) {
            return true;
        }
        let Some(parent) = r.get::<ChildOf>() else { return false };
        e = parent.parent();
    }
}

/// `e`'s transform in the model root's frame (the root's own included).
fn model_frame(world: &World, mut e: Entity) -> Transform {
    let mut t = Transform::IDENTITY;
    loop {
        let r = world.entity(e);
        if let Some(local) = r.get::<Transform>() {
            t = *local * t;
        }
        let Some(parent) = r.get::<ChildOf>() else { return t };
        e = parent.parent();
    }
}

/// Groundcover draws in the exterior's view only (as the terrain patches).
fn batch_layers(
    mut commands: Commands,
    registry: Option<Res<WorldspaceRegistry>>,
    batches: Query<Entity, (Added<GroundcoverBatch>, Without<RenderLayers>)>,
) {
    let Some(entry) = registry.and_then(|r| r.get(&ids::exterior())) else { return };
    for e in &batches {
        commands.entity(e).try_insert(RenderLayers::layer(entry.layer));
    }
}

fn viewer_on(add: On<Add<TerrainCamera>>, mut commands: Commands) {
    commands.entity(add.entity).try_insert(GroundcoverViewer);
}

fn viewer_off(remove: On<Remove<TerrainCamera>>, mut commands: Commands) {
    commands.entity(remove.entity).try_remove::<GroundcoverViewer>();
}

#[cfg(test)]
mod tests {
    use super::*;
    use bevy::ecs::system::RunSystemOnce;
    use sf_terrain_bevy::sf_terrain::{LayerInfo, layer_id};

    fn layer(record: &str) -> LayerInfo {
        LayerInfo {
            record: record.to_owned(),
            id: layer_id(record),
            tiling_m: 4.0,
            triplanar: false,
            displacement_m: 0.0,
            mean_albedo: [0.2, 0.3, 0.1],
            mean_height: 0.5,
            p90_height: 0.6,
            source_edge: 256,
        }
    }

    fn table() -> LayerTable {
        // Two grass LTEX ids (as `sf_records` stores them, lowercase) and one rock nothing plants on.
        LayerTable { layers: vec![layer("ai_grass"), layer("gl_grass_01"), layer("rm_rock_01")], ..LayerTable::default() }
    }

    #[test]
    fn setup_maps_the_grass_layers() {
        let mut world = World::new();
        world.insert_resource(GroundcoverInputs { off: false, table: Some(table()) });
        world.run_system_once(setup).unwrap();
        let source = world.get_resource::<GroundcoverSource>().expect("source inserted");
        assert!(!source.cover.is_empty());
        let ground = world.get_resource::<TerrainGroundRef>().expect("ground inserted");
        let ch = ground.0.channels();
        assert_eq!(ch.channel(layer_id("ai_grass")), Some(0), "AI_Grass feeds the ascadian channel");
        assert_eq!(ch.channel(layer_id("gl_grass_01")), Some(1), "GL_Grass_01 feeds the grazelands channel");
        assert_eq!(ch.channel(layer_id("rm_rock_01")), None);
        let loading = world.get_resource::<GroundcoverLoading>().unwrap();
        assert_eq!(loading.report.channels_matched, 2);
        assert_eq!(loading.keys.len(), loading.report.models);
    }

    #[test]
    fn off_inserts_nothing() {
        let mut world = World::new();
        world.insert_resource(GroundcoverInputs { off: true, table: Some(table()) });
        world.run_system_once(setup).unwrap();
        assert!(!world.contains_resource::<GroundcoverSource>());
        assert!(!world.contains_resource::<TerrainGroundRef>());
        assert!(!world.contains_resource::<GroundcoverLoading>());
    }
}
