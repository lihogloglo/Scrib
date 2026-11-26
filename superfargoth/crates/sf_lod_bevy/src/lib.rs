//! Bevy plugin for the far field: HLOD nodes (L1 per streaming cell
//! through the streaming far ring, L2/L3 resident per worldspace),
//! octahedral impostors, distant light sprites, the no-hole residency
//! fallback, and `far/*` counters. Spec: `docs/specs/far-field.md`.
//!
//! Engine first: proxies are `Mesh3d` + `StandardMaterial` (vertex colors,
//! extended only by the far shadow map: [`ProxyMaterial`])
//! entities and every swap between levels is a Bevy `VisibilityRange`
//! crossfade with `sf_lod`'s lead bands. Custom materials exist only where
//! Bevy has nothing: [`ImpostorMaterial`] and [`LightSpriteMaterial`].
//!
//! ```ignore
//! app.register_asset_source(BAKED_SOURCE, baked_source(store, manifest, extensions /* + baked_extensions() */))
//!    .add_plugins(WorldStreamingPlugin { config: far_streaming_config(StreamingConfig::default(), &levels) })
//!    .add_plugins(FarFieldPlugin { settings: FarFieldSettings { levels, ..default() } });
//! ```

mod assets;
mod depth_equal;
mod materials;
mod runtime;
#[cfg(test)]
mod shader_tests;

pub use assets::{
    FarDirectoryAsset, FarDirectoryLoader, FarLoadError, FarNodeAsset, FarNodeLoader, ImpostorAtlasAsset, ImpostorAtlasLoader, NearCombineLoader,
    ProxySurface,
};
pub use depth_equal::{DEPTH_EQUAL_MAIN_DEF, NO_PREPASS_COUNTER, depth_equal_main_pass};
pub use materials::{
    ATTRIBUTE_IMPOSTOR, ATTRIBUTE_SPRITE, BlendProxyExt, BlendProxyMaterial, FarShadowExt, ImpostorKey, ImpostorMaterial, ImpostorParams, LightSpriteMaterial,
    ProxyMaterial, SpriteParams, blend_proxy_material, impostor_mesh, light_mesh,
};
pub use runtime::{BlendProxyKey, FarMaterials, LEVEL_TINT, node_center};

use std::sync::Arc;

use bevy_app::{App, Plugin, PostUpdate, Startup, Update};
use bevy_asset::{AssetApp, embedded_asset};
use bevy_ecs::prelude::*;
use bevy_pbr::MaterialPlugin;
use bevy_shader::load_shader_library;
use sf_lod::products::{NEAR_STEP, node_subject};
use sf_lod::{FarLevels, NodeKey};
use sf_model_bevy::baked_path;
use sf_tunables::{AnyDef, Def, Group, Lifetime, Source, Unit, def};
use sf_world::{GateQuanta, RingRadii, WorldspaceId};
use sf_world_bevy::{FarProxies, Precombines, StreamingConfig, StreamingSystems};

/// Far-field settings. `levels.handoff` and `band` are replaced by the
/// baked values of each worldspace's directory; the screen (pixel error,
/// bias) is the runtime preset.
#[derive(Resource, Clone, Debug)]
pub struct FarFieldSettings {
    pub levels: FarLevels,
    /// L1 proxies cast shadows (into far cascades); L2+ never do.
    pub l1_shadows: bool,
    /// Light sprites never get smaller than this, in pixels.
    pub light_min_pixels: f32,
    /// How far behind the geometry they replace proxies draw, in pixels of
    /// depth at the reference screen (L1; L2 and L3 get 1.5× and 2×), so the
    /// finer representation wins wherever both draw in a lead band.
    pub proxy_depth_offset_px: f32,
    /// CW-2 selection gate (`sf_world::select_gate`): the residency pass re-runs only once the camera
    /// moved [`FAR_SELECT_MOVE_M`] or a node, the cells' residency or these settings changed (see
    /// `runtime::residency_pass`); while on, nodes count as needing their children that much early.
    /// Off by default until the owner's A/B.
    pub select_gate: GateQuanta,
    /// Scale from cooked linear emissive (`0..=1`) to `StandardMaterial::emissive`
    /// on blended proxies (`1.0` display-referred; hosts using calibrated physical
    /// exposure pass their nits scale with `emissive_exposure_weight = 1.0`).
    pub emissive_nits: f32,
    /// `StandardMaterial::emissive_exposure_weight` on blended proxies.
    pub emissive_exposure_weight: f32,
    /// [`IMPOSTOR_DEPTH_EQUAL`]. Live: a change re-specializes the impostor pipelines.
    pub impostor_depth_equal: bool,
}

/// `far.select.move_m`: [`FarFieldSettings::select_gate`] location quantum, also the guard band of the
/// no-hole fallback. Nobody measured it: far handoffs sit hundreds of meters out with bands of tens.
pub const FAR_SELECT_MOVE_M: Def<f32> = def!(float "far.select.move_m" = 2.0)
    .range(0.0, 32.0)
    .unit(Unit::Meters)
    .help("The far field re-checks which nodes must stay forced once the camera moved this far")
    .source(Source::Guess)
    .group(Group::ViewDistance);

/// `far.handoff_scale`: the far-field handoff `d1` (and every level after it) as a multiple of the
/// baked distances: `FarLevels::screen.bias`. The near streaming rings are sized from it at start
/// ([`far_streaming_config`]) and the cells' statics end at it, so it is read once (`Lifetime::Restart`):
/// a live change would open a gap or a double draw between near statics and their proxies.
pub const FAR_HANDOFF_SCALE: Def<f32> = def!(float "far.handoff_scale" = 1.0)
    .range(0.25, 4.0)
    .unit(Unit::Ratio)
    .help("Far-field handoff distances (and the near rings sized from them) as a multiple of the baked ones")
    .source(Source::Measured("docs/ledger/statics.md 2026-10-12: d1 300 m 11.55 ms / 400 m 11.87 / 600 m 15.05, kept 400 m"))
    .group(Group::ViewDistance)
    .lifetime(Lifetime::Restart);

/// `far.impostor.depth_equal`: impostors are alpha-tested once, in the camera prepass. In a view
/// with a depth prepass their main pass tests depth-equal, writes no depth and never discards
/// ([`depth_equal_main_pass`]; big-world-rendering.md "Fill rules", rule 5). Off is Bevy's stock
/// state (the alpha test in both passes), the A/B arm. The host binds it to
/// [`FarFieldSettings::impostor_depth_equal`].
pub const IMPOSTOR_DEPTH_EQUAL: Def<bool> = def!(bool "far.impostor.depth_equal" = true)
    .help("Far impostors are alpha-tested once, in the depth prepass; the main pass shades only the pixels it kept")
    .source(Source::Industry("UE r.EarlyZPassOnlyMaterialMasking: masked materials tested in the early Z pass, base pass at depth-equal"))
    .group(Group::ViewDistance);

/// This crate's tunables (live binding: tunables slice T2).
pub const TUNABLES: &[&dyn AnyDef] = &[&FAR_SELECT_MOVE_M, &FAR_HANDOFF_SCALE, &IMPOSTOR_DEPTH_EQUAL];

impl FarFieldSettings {
    /// The far-field gate with its declared quantum, on or off.
    pub fn select_gate_quanta(enabled: bool) -> GateQuanta {
        GateQuanta { enabled, move_m: FAR_SELECT_MOVE_M.default_value(), turn_deg: 0.0 }
    }

    /// The depth bias (`StandardMaterial::depth_bias`, wgpu constant bias) of
    /// a level's proxies. With a float depth buffer and reverse Z, one unit
    /// of constant bias is ~2^-23 of the depth value, i.e. a fixed fraction of
    /// the distance, as a pixel is: `px` pixels of depth = `px · m · 2^23`
    /// units, `m` the pixel size per meter of distance. Negative: behind.
    pub fn proxy_depth_bias(&self, level: u8) -> f32 {
        let scale = [1.0, 1.5, 2.0][usize::from(level.clamp(1, 3) - 1)];
        let per_meter = self.levels.pixel_size(1.0);
        -(self.proxy_depth_offset_px * scale * per_meter * 8_388_608.0)
    }
}

impl Default for FarFieldSettings {
    fn default() -> Self {
        Self {
            levels: FarLevels::default(),
            l1_shadows: true,
            light_min_pixels: 2.5,
            proxy_depth_offset_px: 2.0,
            select_gate: Self::select_gate_quanta(sf_world::SELECT_GATE.default_value()),
            emissive_nits: 1.0, // tunables: const (unit conversion: linear emissive to nits, the host sets its exposure calibration)
            emissive_exposure_weight: 0.0, // tunables: const (external contract: Bevy exposure mode paired with emissive_nits)
            impostor_depth_equal: IMPOSTOR_DEPTH_EQUAL.default_value(),
        }
    }
}

/// Debug views.
#[derive(Resource, Clone, Copy, Debug, Default)]
pub struct FarFieldDebug {
    /// Tint proxies by level: L1 green, L2 blue, L3 purple (objects keep
    /// their own colors).
    pub level_tint: bool,
}

/// The terrain's far shadow map for proxies and impostors (weather.md
/// "Distant shadows", tier 2). The host copies it in (e.g. from
/// `sf_terrain_bevy::FarShadowMap`); without it they keep the cascades only.
#[derive(Resource, Clone, Debug, Default, PartialEq)]
pub struct FarShadowInput {
    /// `Rg16Float`: (shadow height − base, occluder distance), meters.
    pub image: Option<bevy_asset::Handle<bevy_image::Image>>,
    /// Render space: (min x, min z, 1 / extent, base height); extent 0 = none.
    pub params: bevy_math::Vec4,
    /// Changes whenever the image contents do.
    pub generation: u32,
}

/// How strongly distant lights glow: 0 by day, 1 at night. The sky (or the
/// app) sets it from the sun's elevation.
#[derive(Resource, Clone, Copy, Debug)]
pub struct FarLightsNight(pub f32);

impl Default for FarLightsNight {
    fn default() -> Self {
        Self(1.0)
    }
}

/// What the far field holds this frame (also reported as `far/*` counters).
#[derive(Resource, Clone, Copy, Debug, Default)]
pub struct FarFieldStats {
    pub l1: usize,
    pub l2: usize,
    pub l3: usize,
    pub entities: usize,
    pub triangles: u64,
    pub impostors: u64,
    pub lights: u64,
    pub bytes: u64,
    /// Nodes forced fully visible because children are still loading.
    pub forced: usize,
    /// CPU time of the residency pass.
    pub select_ms: f64,
}

/// File extensions of far-field products for `sf_model_bevy::baked_source`.
pub fn baked_extensions() -> impl Iterator<Item = (String, String)> {
    sf_lod::products::EXTENSIONS.iter().map(|(s, e)| ((*s).to_owned(), (*e).to_owned()))
}

/// Streaming rings for a far field (far-field.md, "Residency"): R1 just past
/// the object handoff, R2 measured to each cell's L2 node (page) and wide
/// enough that L1 nodes are resident wherever their L2 parent hands over.
pub fn far_streaming_config(mut config: StreamingConfig, levels: &FarLevels) -> StreamingConfig {
    let d1 = levels.handoff(1) + levels.band_width(1);
    let d2 = levels.handoff(2);
    let rings = &mut config.scheduler.rings;
    rings.near = RingRadii::new(d1 + 80.0, d1 + 208.0);
    rings.far = RingRadii::new(d2 * 0.85, d2 * 0.95);
    rings.active.load = rings.active.load.min(rings.near.load);
    rings.active.unload = rings.active.unload.min(rings.near.unload);
    config.scheduler.far_by_page = true;
    config
}

/// System sets of [`FarFieldPlugin`].
#[derive(SystemSet, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum FarFieldSystems {
    /// `PostUpdate`: applies [`FarShadowInput`] to proxy and impostor materials
    /// after floating-origin and terrain updates so distant shadows never jump
    /// by a cell on origin-shift frames (SH-11).
    FarShadow,
}

/// The far field. Add after `WorldStreamingPlugin`.
#[derive(Default)]
pub struct FarFieldPlugin {
    pub settings: FarFieldSettings,
}

impl Plugin for FarFieldPlugin {
    fn build(&self, app: &mut App) {
        load_shader_library!(app, "shaders/impostor_common.wesl");
        load_shader_library!(app, "shaders/far_shadow_map.wesl");
        embedded_asset!(app, "shaders/proxy.wesl");
        embedded_asset!(app, "shaders/proxy_blend.wesl");
        embedded_asset!(app, "shaders/impostor.wesl");
        embedded_asset!(app, "shaders/impostor_prepass.wesl");
        embedded_asset!(app, "shaders/light_sprite.wesl");

        let shared = runtime::FarShared::default();
        let levels = &self.settings.levels;
        if !app.is_plugin_added::<sf_render_order_bevy::RenderOrderPlugin>() {
            // `TranslucentFog`, which the light sprites and blended proxies sample.
            app.add_plugins(sf_render_order_bevy::RenderOrderPlugin);
        }
        app.add_plugins((
            MaterialPlugin::<ImpostorMaterial>::default(),
            MaterialPlugin::<LightSpriteMaterial>::default(),
            MaterialPlugin::<ProxyMaterial>::default(),
            MaterialPlugin::<BlendProxyMaterial>::default(),
        ))
            .init_asset::<FarNodeAsset>()
            .init_asset::<FarDirectoryAsset>()
            .init_asset::<ImpostorAtlasAsset>()
            .register_asset_loader(FarNodeLoader)
            .register_asset_loader(FarDirectoryLoader)
            .register_asset_loader(ImpostorAtlasLoader)
            .register_asset_loader(NearCombineLoader)
            // Near levels spawn from the cells' precombines (precombine.md).
            .insert_resource(Precombines {
                path: Arc::new(|world: &World, ws: &WorldspaceId, cell| {
                    world
                        .get_resource::<FarWorldspaces>()
                        .is_none_or(|f| f.contains(ws))
                        .then(|| baked_path(NEAR_STEP, &node_subject(ws, NodeKey::of_cell(cell)), Some("sfnear")))
                }),
            })
            .insert_resource(self.settings.clone())
            .init_resource::<FarFieldDebug>()
            .init_resource::<FarWorldspaces>()
            .init_resource::<FarLightsNight>()
            .init_resource::<FarShadowInput>()
            .init_resource::<FarFieldStats>()
            .insert_resource(shared.clone())
            .insert_resource(FarProxies {
                proxies: Some(Arc::new(runtime::FarHook(shared))),
                handoff: Some((levels.handoff(1), levels.band_width(1))),
            })
            .add_systems(Startup, runtime::create_materials)
            .add_systems(
                Update,
                (
                    runtime::load_directories,
                    runtime::build_impostor_arrays,
                    runtime::spawn_upper_nodes,
                    runtime::residency_pass,
                    runtime::update_materials,
                )
                    .chain()
                    .after(StreamingSystems::Drive),
            )
            .add_systems(PostUpdate, runtime::update_far_shadow.in_set(FarFieldSystems::FarShadow));
    }

    fn finish(&self, app: &mut App) {
        // After every plugin's `build`, so the diag counters are found whatever the order.
        depth_equal::add_watch(app);
    }
}

#[cfg(test)]
mod tests {
    use super::FarFieldSettings;

    #[test]
    fn proxy_depth_bias_is_pixels_of_relative_depth() {
        let s = FarFieldSettings::default();
        // 2 px at 1440p / 70°: ~0.19% of the distance, in 2^-23 units.
        let per_meter = s.levels.pixel_size(1.0);
        let expected = -(s.proxy_depth_offset_px * per_meter * 8_388_608.0);
        assert!((s.proxy_depth_bias(1) - expected).abs() < 1.0);
        assert!((-s.proxy_depth_bias(1) / 8_388_608.0 - 0.0019).abs() < 0.0002);
        // Coarser levels sit further behind; always behind (negative).
        assert!(s.proxy_depth_bias(3) < s.proxy_depth_bias(2) && s.proxy_depth_bias(2) < s.proxy_depth_bias(1));
        assert!(s.proxy_depth_bias(1) < 0.0);
    }
}

/// The worldspaces that have a far field (`None`: every streamed one). Hosts
/// set it from the bake manifest, so worldspaces without one (interiors)
/// never request far products while they stream or are prefetched.
#[derive(Resource, Clone, Debug, Default, PartialEq, Eq)]
pub struct FarWorldspaces(pub Option<std::collections::BTreeSet<WorldspaceId>>);

impl FarWorldspaces {
    pub fn contains(&self, ws: &WorldspaceId) -> bool {
        self.0.as_ref().is_none_or(|set| set.contains(ws))
    }
}
