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
//! - **Blades** (slice GC-3a2c): `sf_groundcover_bevy::blades::BladesPlugin` with the data's blade styles
//!   ([`blade_styles`]: a `bevy_grass` preset plus overrides; slice GC-3e1) for the data's blade species; drawn only
//!   while `groundcover.blades.enabled` is on (off by default). Blade fields get the exterior's render layer, as the batches.
//! - **Statics** (slice GC-M4): the exterior's grounded statics are exclusion regions (`groundcover_statics`), so
//!   nothing grows inside rocks, buildings, docks or stairs. Built once at `Startup` from the world index, the
//!   models' baked bounds and the baked terrain.
//! - **Off**: `--off groundcover` adds none of the renderer.
//!
//! Log lines: at `Startup`, `groundcover: statics read P placements, S boxes, excluded E, floating F, no ground G,
//! terrain tiles T in X ms`; once every model resolved, `groundcover: species S, channels matched M/N, unmatched
//! [..], models loaded L/T` and `groundcover: blade species B, styles registered R`. Gauges `groundcover/*`
//! (`sf_groundcover_bevy::publish_gauges`).

use std::sync::Arc;

use bevy::asset::{LoadState, RecursiveDependencyLoadState};
use bevy::camera::visibility::RenderLayers;
use bevy::prelude::*;
use bevy::world_serialization::WorldAsset;
use mw_world::groundcover::{BladeStyleDef, GroundcoverData};
use mw_world::ids;
use sf_bake::{Manifest, Store};
use sf_groundcover::{OverrideSet, Overrides, RenderKind};
use sf_groundcover_bevy::blades::{BladeBlock, BladeStyles, BladesPlugin, GrassStyle};
use sf_groundcover_bevy::{
    GroundcoverBatch, GroundcoverModel, GroundcoverModels, GroundcoverPlugin, GroundcoverSource, GroundcoverViewer,
    LayerChannels, TerrainGround, TerrainGroundPlugin, TerrainGroundRef,
};
use sf_terrain_bevy::TerrainCamera;
use sf_terrain_bevy::sf_terrain::LayerTable;
use sf_terrain_bevy::sf_terrain::bake::BakedTerrain;
use sf_world::WorldspaceId;
use sf_world_bevy::{WorldIndexSource, WorldspaceRecords, WorldspaceRegistry};

use crate::groundcover_statics::{self as statics, FootprintSettings};

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

/// The bake [`setup`] reads the statics' model bounds and the terrain under them from.
#[derive(Resource, Clone)]
pub struct GroundcoverBake {
    pub store: Store,
    pub manifest: Arc<Manifest>,
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
    /// Species drawn as blades (`RenderKind::Blade`).
    pub blade_species: usize,
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
        app.register_tunables(statics::TUNABLES)
            .insert_resource(GroundcoverBake { store: self.store.clone(), manifest: self.manifest.clone() });
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
        app.add_plugins((GroundcoverPlugin, TerrainGroundPlugin, BladesPlugin))
            .insert_resource(GroundcoverInputs { off: false, table })
            .add_systems(Startup, setup)
            .add_systems(Update, (load_models, batch_layers))
            .add_observer(viewer_on)
            .add_observer(viewer_off);
    }
}

/// The data's blade styles (`blade_styles` in the data file) as the renderer's: each entry's `base` preset (its
/// lower-case `bevy_grass` constant name, `"meadow"` = `GrassStyle::MEADOW`) with the entry's overrides applied. An
/// unknown preset is an error naming it and the style.
pub fn blade_styles(data: &GroundcoverData) -> Result<BladeStyles, String> {
    let mut styles = BladeStyles::default();
    for def in &data.blade_styles {
        styles.0.insert(def.name.clone(), blade_style(def)?);
    }
    Ok(styles)
}

/// One [`BladeStyleDef`] as a `GrassStyle`.
fn blade_style(def: &BladeStyleDef) -> Result<GrassStyle, String> {
    let mut s = GrassStyle::preset(&def.base.to_ascii_uppercase()).ok_or_else(|| {
        format!("groundcover data: blade style {:?} starts from preset {:?}, which bevy_grass does not have", def.name, def.base)
    })?;
    let srgb = |c: [f32; 3]| Color::srgb(c[0], c[1], c[2]);
    let (b, o) = (&mut s.blade, &def.blade);
    set(&mut b.height, o.height_m.map(Vec2::from));
    set(&mut b.width, o.width_m);
    set(&mut b.lean, o.lean);
    set(&mut b.stiffness, o.stiffness);
    set(&mut b.root_color, o.root_color.map(srgb));
    set(&mut b.tip_color, o.tip_color.map(srgb));
    set(&mut b.root_occlusion, o.root_occlusion);
    set(&mut b.translucency, o.translucency);
    set(&mut b.specular, o.specular);
    set(&mut b.twist, o.twist);
    set(&mut b.droop, o.droop);
    set(&mut b.fold, o.fold);
    set(&mut b.head_length, o.head_length);
    set(&mut b.head_width, o.head_width_m);
    set(&mut b.head_color, o.head_color.map(srgb));
    let (v, o) = (&mut s.variation, &def.variation);
    set(&mut v.patch_color, o.patch_color.map(srgb));
    set(&mut v.patch_amount, o.patch_amount);
    set(&mut v.patch_size, o.patch_size_m);
    set(&mut v.dry_tips, o.dry_tips);
    set(&mut v.dead_share, o.dead_share);
    set(&mut v.tufts, o.tufts);
    set(&mut v.tuft_spacing, o.tuft_spacing_m);
    set(&mut v.bare_share, o.bare_share);
    set(&mut v.bare_patch_size, o.bare_patch_size_m);
    set(&mut s.ground_tint.color, def.ground_tint.color.map(srgb));
    set(&mut s.ground_tint.amount, def.ground_tint.amount);
    set(&mut s.toon_bands, def.toon_bands);
    Ok(s)
}

/// `*field = value` when the data gives one.
fn set<T>(field: &mut T, value: Option<T>) {
    if let Some(v) = value {
        *field = v;
    }
}

/// The source and the terrain ground for `table` with `exclusions` (the statics' footprints), and what matched.
pub fn build_source(
    data: &GroundcoverData,
    table: &LayerTable,
    ws: WorldspaceId,
    sea_level: Option<f32>,
    exclusions: Vec<sf_groundcover::Region>,
) -> Result<(GroundcoverSource, Arc<TerrainGround>, SetupReport), String> {
    let cover = data.build().map_err(|e| e.to_string())?;
    let overrides = if exclusions.is_empty() {
        Overrides::none()
    } else {
        Overrides::new(&cover, &OverrideSet { exclusions, ..OverrideSet::default() }).map_err(|e| e.to_string())?
    };
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
        blade_species: data.species.iter().filter(|s| matches!(s.render, RenderKind::Blade { .. })).count(),
    };
    let ground = Arc::new(TerrainGround::new(ws, sea_level, LayerChannels::from_pairs(map.pairs)));
    let source = GroundcoverSource { cover: Arc::new(cover), overrides: Arc::new(overrides), ground: ground.clone() };
    Ok((source, ground, report))
}

/// `Startup`: inserts `GroundcoverSource` (with the statics' exclusions when the bake and the world index are
/// there), the data's `BladeStyles`, `TerrainGroundRef` and the model list ([`GroundcoverLoading`]); nothing under the off switch or without a
/// layer table.
pub fn setup(
    mut commands: Commands,
    inputs: Option<Res<GroundcoverInputs>>,
    records: Option<Res<WorldspaceRecords>>,
    bake: Option<Res<GroundcoverBake>>,
    index: Option<Res<WorldIndexSource>>,
    tunables: Option<Res<sf_tunables_bevy::Tunables>>,
) {
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
    let styles = match blade_styles(&data) {
        Ok(s) => s,
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
    let exclusions = match (bake, index) {
        (Some(bake), Some(index)) => static_exclusions(&bake, index.0.as_ref(), &ws, footprint_settings(tunables.as_deref())),
        _ => Vec::new(),
    };
    match build_source(&data, table, ws, sea_level, exclusions) {
        Ok((source, ground, report)) => {
            commands.insert_resource(source);
            commands.insert_resource(styles);
            commands.insert_resource(TerrainGroundRef(ground));
            commands.insert_resource(GroundcoverLoading { report, keys: data.model_keys(), ..default() });
        }
        Err(e) => error!("groundcover: {e}"),
    }
}

/// The footprint tunables' values (defaults for anything unregistered).
fn footprint_settings(tunables: Option<&sf_tunables_bevy::Tunables>) -> FootprintSettings {
    let read = |def: &sf_tunables_bevy::sf_tunables::Def<f32>| {
        tunables.and_then(|t| t.0.id_of(def).ok().map(|id| t.0.read(id))).unwrap_or_else(|| def.default_value())
    };
    FootprintSettings { reach_m: read(&statics::REACH_M), margin_m: read(&statics::MARGIN_M) }
}

/// The exclusion regions of `ws`'s grounded statics, with the log line; none on an error (logged).
fn static_exclusions(
    bake: &GroundcoverBake,
    index: &dyn sf_world::IndexReader,
    ws: &WorldspaceId,
    settings: FootprintSettings,
) -> Vec<sf_groundcover::Region> {
    let started = std::time::Instant::now();
    match statics::build(index, &bake.store, &bake.manifest, ws, settings) {
        Ok((regions, r)) => {
            info!(
                "groundcover: statics read {} placements, {} boxes, excluded {}, floating {}, no ground {}, terrain tiles {} in {:.0} ms",
                r.placements,
                r.statics,
                r.excluded,
                r.floating,
                r.no_ground,
                r.tiles,
                started.elapsed().as_secs_f64() * 1e3
            );
            regions
        }
        Err(e) => {
            warn!("groundcover: statics: {e}: nothing excluded");
            Vec::new()
        }
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
    styles: Option<Res<BladeStyles>>,
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
        info!("groundcover: blade species {}, styles registered {}", r.blade_species, styles.map_or(0, |s| s.0.len()));
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

/// New mesh batches and blade fields without a render layer yet.
type NewUnlayered = (Or<(Added<GroundcoverBatch>, Added<BladeBlock>)>, Without<RenderLayers>);

/// Groundcover (mesh batches and blade fields) draws in the exterior's view only (as the terrain patches).
fn batch_layers(
    mut commands: Commands,
    registry: Option<Res<WorldspaceRegistry>>,
    batches: Query<Entity, NewUnlayered>,
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
    use mw_world::groundcover::{BladeOverrides, GroundTintOverrides, VariationOverrides};
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
        assert_eq!(loading.report.blade_species, 1);
        let styles = world.get_resource::<BladeStyles>().expect("blade styles inserted");
        assert!(styles.0.contains_key("meadow"), "the data's blade style is registered");
    }

    fn def(base: &str) -> BladeStyleDef {
        BladeStyleDef {
            name: "test".into(),
            base: base.into(),
            blade: BladeOverrides::default(),
            variation: VariationOverrides::default(),
            ground_tint: GroundTintOverrides::default(),
            toon_bands: None,
        }
    }

    #[test]
    fn the_data_meadow_style_is_the_meadow_preset() {
        let styles = blade_styles(&GroundcoverData::builtin().unwrap()).unwrap();
        assert_eq!(styles.0.len(), 1);
        assert_eq!(styles.0["meadow"], GrassStyle::MEADOW);
    }

    #[test]
    fn an_override_lands_on_its_field_only() {
        let mut d = def("meadow");
        d.blade.tip_color = Some([0.62, 0.58, 0.30]);
        d.ground_tint.amount = Some(0.5);
        let got = blade_style(&d).unwrap();
        let mut want = GrassStyle::MEADOW;
        want.blade.tip_color = Color::srgb(0.62, 0.58, 0.30);
        want.ground_tint.amount = 0.5;
        assert_eq!(got, want);
        assert_ne!(got.blade.tip_color, GrassStyle::MEADOW.blade.tip_color);
    }

    #[test]
    fn every_override_field_is_applied() {
        let mut input = def("meadow");
        input.blade = BladeOverrides {
            height_m: Some([0.1, 0.2]),
            width_m: Some(0.01),
            lean: Some(0.5),
            stiffness: Some(0.9),
            root_color: Some([0.1, 0.1, 0.1]),
            tip_color: Some([0.2, 0.2, 0.2]),
            root_occlusion: Some(0.1),
            translucency: Some(0.2),
            specular: Some(0.4),
            twist: Some(0.5),
            droop: Some(0.6),
            fold: Some(0.7),
            head_length: Some(0.1),
            head_width_m: Some(0.05),
            head_color: Some([0.3, 0.3, 0.3]),
        };
        input.variation = VariationOverrides {
            patch_color: Some([0.4, 0.4, 0.4]),
            patch_amount: Some(0.3),
            patch_size_m: Some(5.0),
            dry_tips: Some(0.2),
            dead_share: Some(0.1),
            tufts: Some(0.4),
            tuft_spacing_m: Some(1.0),
            bare_share: Some(0.2),
            bare_patch_size_m: Some(2.0),
        };
        input.ground_tint = GroundTintOverrides { color: Some([0.5, 0.5, 0.5]), amount: Some(0.9) };
        input.toon_bands = Some(4);
        let style = blade_style(&input).unwrap();
        let grey = |v: f32| Color::srgb(v, v, v);
        let b = &style.blade;
        assert_eq!((b.height, b.width, b.lean, b.stiffness), (Vec2::new(0.1, 0.2), 0.01, 0.5, 0.9));
        assert_eq!((b.root_color, b.tip_color, b.head_color), (grey(0.1), grey(0.2), grey(0.3)));
        assert_eq!((b.root_occlusion, b.translucency, b.specular), (0.1, 0.2, 0.4));
        assert_eq!((b.twist, b.droop, b.fold, b.head_length, b.head_width), (0.5, 0.6, 0.7, 0.1, 0.05));
        let v = &style.variation;
        assert_eq!((v.patch_color, v.patch_amount, v.patch_size), (grey(0.4), 0.3, 5.0));
        assert_eq!((v.dry_tips, v.dead_share, v.tufts, v.tuft_spacing), (0.2, 0.1, 0.4, 1.0));
        assert_eq!((v.bare_share, v.bare_patch_size), (0.2, 2.0));
        assert_eq!((style.ground_tint.color, style.ground_tint.amount, style.toon_bands), (grey(0.5), 0.9, 4));
        // What the data can't set stays the preset's.
        assert_eq!((style.density, &style.species), (GrassStyle::MEADOW.density, &GrassStyle::MEADOW.species));
    }

    #[test]
    fn an_unknown_preset_is_an_error_naming_it() {
        let err = blade_style(&def("no_such_preset")).unwrap_err();
        assert!(err.contains("\"no_such_preset\"") && err.contains("\"test\""), "{err}");
        let mut data = GroundcoverData::builtin().unwrap();
        data.blade_styles[0].base = "no_such_preset".into();
        assert!(blade_styles(&data).unwrap_err().contains("no_such_preset"));
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
