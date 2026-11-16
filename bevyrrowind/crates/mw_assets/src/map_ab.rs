//! Global A/B of the texture packs' companion maps on every `StandardMaterial` (TX-lab,
//! `docs/wiki/engine/textures.md` "Switches and counters"): normal maps (`--off normalmaps`,
//! console `flip normalmaps`), ORM maps (`--off orm`, `flip orm`) and parallax depth maps
//! (`--off parallax`, `flip parallax`; tx-pom).
//!
//! Materials are always built with their maps ([`crate::Surface::to_material`]); while a switch is
//! off, [`apply_material_maps`] takes the maps out of every material that has them (new ones the
//! frame they appear, before their first extraction) and keeps the removed handles and factors,
//! so turning it back on restores each material field for field, live. Off = what the material
//! would be without the companion: no `normal_map_texture`; no `metallic_roughness_texture` /
//! `occlusion_texture` and the pre-TS-2 factors ([`PLAIN_METALLIC`], [`PLAIN_ROUGHNESS`],
//! [`PLAIN_REFLECTANCE`]); no `depth_map` and Bevy's default parallax fields (scale, method,
//! layers: the method is a pipeline-key bit, so the stripped material keys as one without a depth
//! companion). Only texture slots and these factors change, never a `*_channel`, so
//! material-class folding (`sf_world_bevy::matclass`) sees what it saw. Images stay loaded while
//! off (an A/B, not a memory saving).
//!
//! The parallax a depth-mapped material gets is the quality preset's, live: `texture.parallax.layers`
//! and `texture.parallax.relief_steps` (tunables slice T5a) set [`MaterialMaps`]' preset; a change
//! retunes every depth-mapped material (the method is a pipeline-key bit: they re-specialise), 0
//! layers takes their depth maps out (as the switch does) and back. New loads bind the preset's
//! ([`set_parallax`]): none at 0 layers (the depth map isn't even bound), so a run started at 0 has
//! no depth maps to give back for models loaded meanwhile (they get them when they load again).

use std::collections::HashMap;
use std::sync::RwLock;
use std::sync::atomic::{AtomicBool, Ordering};

use bevy::pbr::ParallaxMappingMethod;
use bevy::prelude::*;
use sf_tunables_bevy::sf_tunables::{Def, Group, Lifetime, Source, Unit, def};
use sf_tunables_bevy::TunablesAppExt;

/// Metallic factor of a material without an ORM map (2002 assets carry no PBR data).
pub const PLAIN_METALLIC: f32 = 0.0;
/// Perceptual roughness of a material without an ORM map: matte.
pub const PLAIN_ROUGHNESS: f32 = 0.85;
/// Reflectance of a material without an ORM map: low.
pub const PLAIN_REFLECTANCE: f32 = 0.3;

/// `parallax_depth_scale` of a depth-mapped material: the march's full depth as a fraction of the
/// texture's UV extent. `OpenMW`'s parallax for the same `_nh` maps on the same UVs offsets by
/// `height × 0.04 − 0.02` (`files/shaders/lib/material/parallax.glsl` `PARALLAX_SCALE`, `4e718b5`):
/// the same 0.04 span. Both packs we know author for that shader (textures-adapter.md §7), so no
/// per-pack scale. Bevy's default is 0.1 ("anything over may look distorted").
pub const PARALLAX_DEPTH_SCALE: f32 = 0.04;

static NORMAL_MAPS: AtomicBool = AtomicBool::new(true);
static ORM_MAPS: AtomicBool = AtomicBool::new(true);
static DEPTH_MAPS: AtomicBool = AtomicBool::new(true);
static PARALLAX: RwLock<Option<Parallax>> = RwLock::new(Some(Parallax::HIGH));

/// The start state of [`MaterialMaps`] (the app's `--off normalmaps` / `--off orm` /
/// `--off parallax`); call before the app is built.
pub fn set_material_maps(normal_maps: bool, orm: bool, parallax: bool) {
    NORMAL_MAPS.store(normal_maps, Ordering::Relaxed);
    ORM_MAPS.store(orm, Ordering::Relaxed);
    DEPTH_MAPS.store(parallax, Ordering::Relaxed);
}

/// The parallax occlusion of a material with a depth companion (Bevy's `StandardMaterial` parallax
/// fields), per quality preset.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Parallax {
    /// `max_parallax_layer_count`: layers at grazing angles (1 looking straight on).
    pub max_layers: f32,
    /// `parallax_mapping_method`.
    pub method: ParallaxMappingMethod,
}

impl Parallax {
    /// The High preset's (and the default without a preset): 32 layers, 5 relief steps.
    pub const HIGH: Self = Self { max_layers: 32.0, method: ParallaxMappingMethod::Relief { max_steps: 5 } };

    /// `max_layers` layers, refined by `relief_steps` binary-search steps (relief mapping) or, for
    /// `None`, by one interpolation (parallax occlusion mapping).
    pub fn new(max_layers: u8, relief_steps: Option<u8>) -> Self {
        Self {
            max_layers: f32::from(max_layers),
            method: relief_steps
                .map_or(ParallaxMappingMethod::Occlusion, |steps| ParallaxMappingMethod::Relief { max_steps: u32::from(steps) }),
        }
    }

    /// `(layers, relief steps)` as the tunables hold them (0 steps: one interpolation).
    #[expect(clippy::cast_possible_truncation, clippy::cast_sign_loss, reason = "layer counts are small whole numbers")]
    fn layers_and_steps(self) -> (u32, u32) {
        let steps = match self.method {
            ParallaxMappingMethod::Relief { max_steps } => max_steps,
            ParallaxMappingMethod::Occlusion => 0,
        };
        (self.max_layers as u32, steps)
    }

    /// Sets `material`'s method and layers to these (its depth map and scale stay).
    fn retune(self, material: &mut StandardMaterial) {
        material.parallax_mapping_method = self.method;
        material.max_parallax_layer_count = self.max_layers;
    }

    /// Binds `depth` as `material`'s depth map with these settings and [`PARALLAX_DEPTH_SCALE`].
    pub fn apply(self, material: &mut StandardMaterial, depth: Handle<Image>) {
        material.depth_map = Some(depth);
        material.parallax_depth_scale = PARALLAX_DEPTH_SCALE;
        material.parallax_mapping_method = self.method;
        material.max_parallax_layer_count = self.max_layers;
    }
}

/// Sets the parallax materials with a depth companion get (`None`: no depth map bound, the Low
/// preset); call before models load.
pub fn set_parallax(parallax: Option<Parallax>) {
    if let Ok(mut current) = PARALLAX.write() {
        *current = parallax;
    }
}

/// The parallax [`set_parallax`] chose.
pub fn parallax() -> Option<Parallax> {
    PARALLAX.read().map_or(Some(Parallax::HIGH), |p| *p)
}

/// Which companion maps materials keep (write the flags to flip them live).
#[derive(Resource, Debug)]
pub struct MaterialMaps {
    /// Normal maps (`normal_map_texture`).
    pub normal_maps: bool,
    /// ORM maps (`metallic_roughness_texture`, `occlusion_texture` and their factors).
    pub orm: bool,
    /// Parallax depth maps (`depth_map` and the parallax fields).
    pub parallax: bool,
    /// The preset's parallax (`texture.parallax.layers`, 0: none; `texture.parallax.relief_steps`,
    /// 0: one interpolation): [`MaterialMaps::preset`].
    pub parallax_layers: u32,
    pub parallax_relief_steps: u32,
    stash: HashMap<AssetId<StandardMaterial>, Stashed>,
    /// The flags the last pass applied (a restore pass runs only when one turns back on).
    applied: Switches,
    /// The preset the last pass applied (a retune pass runs only when it changes).
    applied_preset: Option<Parallax>,
}

impl Default for MaterialMaps {
    fn default() -> Self {
        Self::new(NORMAL_MAPS.load(Ordering::Relaxed), ORM_MAPS.load(Ordering::Relaxed), DEPTH_MAPS.load(Ordering::Relaxed))
    }
}

impl MaterialMaps {
    /// The switches; the preset is the one new loads bind ([`parallax`]).
    pub fn new(normal_maps: bool, orm: bool, parallax: bool) -> Self {
        let preset = self::parallax();
        let (layers, steps) = preset.map_or((0, 0), Parallax::layers_and_steps);
        Self {
            normal_maps,
            orm,
            parallax,
            parallax_layers: layers,
            parallax_relief_steps: steps,
            stash: HashMap::new(),
            applied: Switches::ALL_ON,
            applied_preset: preset,
        }
    }

    /// The preset's parallax: `None` at 0 layers.
    pub fn preset(&self) -> Option<Parallax> {
        let clamp = |v: u32| u8::try_from(v).unwrap_or(u8::MAX);
        (self.parallax_layers > 0)
            .then(|| Parallax::new(clamp(self.parallax_layers), (self.parallax_relief_steps > 0).then(|| clamp(self.parallax_relief_steps))))
    }

    /// Materials holding removed maps.
    pub fn stashed(&self) -> usize {
        self.stash.len()
    }

    fn switches(&self) -> Switches {
        Switches { normal_maps: self.normal_maps, orm: self.orm, parallax: self.parallax }
    }
}

/// The three switches' state at once.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Switches {
    normal_maps: bool,
    orm: bool,
    parallax: bool,
}

impl Switches {
    const ALL_ON: Self = Self { normal_maps: true, orm: true, parallax: true };

    /// Whether a switch is on here and was off in `before`.
    fn turned_on_since(self, before: Self) -> bool {
        (self.normal_maps && !before.normal_maps) || (self.orm && !before.orm) || (self.parallax && !before.parallax)
    }
}

/// What the switches took out of one material.
#[derive(Clone, Debug, Default)]
pub struct Stashed {
    normal: Option<Handle<Image>>,
    orm: Option<OrmSlots>,
    depth: Option<DepthSlots>,
}

#[derive(Clone, Debug)]
struct OrmSlots {
    metallic_roughness: Handle<Image>,
    occlusion: Option<Handle<Image>>,
    metallic: f32,
    perceptual_roughness: f32,
    reflectance: f32,
}

#[derive(Clone, Debug)]
struct DepthSlots {
    map: Handle<Image>,
    scale: f32,
    method: ParallaxMappingMethod,
    layers: f32,
}

impl Stashed {
    /// Whether `material` holds a map the switches turn off.
    fn needs_strip(material: &StandardMaterial, on: Switches) -> bool {
        (!on.normal_maps && material.normal_map_texture.is_some())
            || (!on.orm && material.metallic_roughness_texture.is_some())
            || (!on.parallax && material.depth_map.is_some())
    }

    /// Takes the maps the switches turn off out of `material` into `self`.
    fn strip(&mut self, material: &mut StandardMaterial, on: Switches) {
        if !on.normal_maps && let Some(normal) = material.normal_map_texture.take() {
            self.normal = Some(normal);
        }
        if !on.orm && let Some(metallic_roughness) = material.metallic_roughness_texture.take() {
            self.orm = Some(OrmSlots {
                metallic_roughness,
                occlusion: material.occlusion_texture.take(),
                metallic: material.metallic,
                perceptual_roughness: material.perceptual_roughness,
                reflectance: material.reflectance,
            });
            material.metallic = PLAIN_METALLIC;
            material.perceptual_roughness = PLAIN_ROUGHNESS;
            material.reflectance = PLAIN_REFLECTANCE;
        }
        if !on.parallax && let Some(map) = material.depth_map.take() {
            self.depth = Some(DepthSlots {
                map,
                scale: material.parallax_depth_scale,
                method: material.parallax_mapping_method,
                layers: material.max_parallax_layer_count,
            });
            let plain = StandardMaterial::default();
            material.parallax_depth_scale = plain.parallax_depth_scale;
            material.parallax_mapping_method = plain.parallax_mapping_method;
            material.max_parallax_layer_count = plain.max_parallax_layer_count;
        }
    }

    /// Whether [`Stashed::restore`] would give `material` something back.
    fn restores(&self, on: Switches) -> bool {
        (on.normal_maps && self.normal.is_some()) || (on.orm && self.orm.is_some()) || (on.parallax && self.depth.is_some())
    }

    /// Puts back what the switches allow again.
    fn restore(&mut self, material: &mut StandardMaterial, on: Switches) {
        if on.normal_maps && let Some(normal) = self.normal.take() {
            material.normal_map_texture = Some(normal);
        }
        if on.orm && let Some(slots) = self.orm.take() {
            material.metallic_roughness_texture = Some(slots.metallic_roughness);
            material.occlusion_texture = slots.occlusion;
            material.metallic = slots.metallic;
            material.perceptual_roughness = slots.perceptual_roughness;
            material.reflectance = slots.reflectance;
        }
        if on.parallax && let Some(slots) = self.depth.take() {
            material.depth_map = Some(slots.map);
            material.parallax_depth_scale = slots.scale;
            material.parallax_mapping_method = slots.method;
            material.max_parallax_layer_count = slots.layers;
        }
    }

    pub fn is_empty(&self) -> bool {
        self.normal.is_none() && self.orm.is_none() && self.depth.is_none()
    }
}

/// Applies [`MaterialMaps`]: restores stashed maps when a switch turns back on, then strips every
/// material that holds a map a switch keeps off. Nothing runs while all are on and nothing is
/// stashed. `PostUpdate` before the asset events, so a new material is extracted already stripped.
pub(crate) fn apply_material_maps(mut maps: ResMut<MaterialMaps>, materials: Option<ResMut<Assets<StandardMaterial>>>) {
    let Some(mut materials) = materials else { return };
    if maps.preset() != maps.applied_preset {
        retune_parallax(maps.bypass_change_detection(), &mut materials);
    }
    let on = maps.switches();
    if on == Switches::ALL_ON && maps.stash.is_empty() {
        return;
    }
    let maps = maps.bypass_change_detection();
    let turned_on = on.turned_on_since(maps.applied);
    maps.applied = on;
    if turned_on {
        // The preset at 0 layers keeps the depth maps out whatever the switch says.
        let restore_on = Switches { parallax: on.parallax && maps.applied_preset.is_some(), ..on };
        maps.stash.retain(|id, stashed| {
            if !stashed.restores(restore_on) {
                return true;
            }
            // A material dropped meanwhile: forget its maps.
            let Some(mut material) = materials.get_mut(*id) else { return false };
            stashed.restore(&mut material, restore_on);
            !stashed.is_empty()
        });
    }
    if on == Switches::ALL_ON {
        return;
    }
    let strip: Vec<AssetId<StandardMaterial>> =
        materials.iter().filter(|(_, m)| Stashed::needs_strip(m, on)).map(|(id, _)| id).collect();
    for id in strip {
        if let Some(mut material) = materials.get_mut(id) {
            maps.stash.entry(id).or_default().strip(&mut material, on);
        }
    }
}

/// Depth maps only (the preset's strip and restore leave the other maps to their switches).
const DEPTH_ONLY_OFF: Switches = Switches { normal_maps: true, orm: true, parallax: false };
const DEPTH_ONLY_ON: Switches = Switches { normal_maps: false, orm: false, parallax: true };

/// The preset changed: new loads bind it ([`set_parallax`]); at 0 layers every depth map goes into the
/// stash, otherwise the stashed ones come back (unless the switch keeps them off) and every
/// depth-mapped material, stashed or not, takes its method and layers.
fn retune_parallax(maps: &mut MaterialMaps, materials: &mut Assets<StandardMaterial>) {
    let preset = maps.preset();
    maps.applied_preset = preset;
    // The global is process-wide: unit tests running in parallel read it (`cooked.rs`).
    #[cfg(not(test))]
    set_parallax(preset);
    let Some(p) = preset else {
        let ids: Vec<AssetId<StandardMaterial>> = materials.iter().filter(|(_, m)| m.depth_map.is_some()).map(|(id, _)| id).collect();
        for id in ids {
            if let Some(mut material) = materials.get_mut(id) {
                maps.stash.entry(id).or_default().strip(&mut material, DEPTH_ONLY_OFF);
            }
        }
        return;
    };
    for stashed in maps.stash.values_mut() {
        if let Some(depth) = stashed.depth.as_mut() {
            (depth.method, depth.layers) = (p.method, p.max_layers);
        }
    }
    if maps.parallax {
        maps.stash.retain(|id, stashed| {
            if stashed.depth.is_none() {
                return true;
            }
            let Some(mut material) = materials.get_mut(*id) else { return false };
            stashed.restore(&mut material, DEPTH_ONLY_ON);
            !stashed.is_empty()
        });
    }
    #[allow(clippy::float_cmp)] // exact: change detection
    let ids: Vec<AssetId<StandardMaterial>> = materials
        .iter()
        .filter(|(_, m)| m.depth_map.is_some() && (m.parallax_mapping_method != p.method || m.max_parallax_layer_count != p.max_layers))
        .map(|(id, _)| id)
        .collect();
    for id in ids {
        if let Some(mut material) = materials.get_mut(id) {
            p.retune(&mut material);
        }
    }
}

/// `texture.parallax.layers`: most depth layers the parallax march steps through (Bevy's
/// `max_parallax_layer_count`, at grazing angles; 1 looking straight on); 0: no parallax, the depth
/// map isn't bound. Retunes loaded materials (a pipeline re-specialisation).
pub const PARALLAX_LAYERS: Def<u32> = def!(count "texture.parallax.layers" = 32)
    .range(0, 255)
    .unit(Unit::Count)
    .help("Parallax occlusion: most depth layers at grazing angles (0: off, no depth map bound; a run started at 0 has none to give back)")
    .source(Source::BevyDefault("bevy_pbr::StandardMaterial::max_parallax_layer_count (16); High 32 (textures.md T4d)"))
    .group(Group::Textures)
    .lifetime(Lifetime::Recompile);

/// `texture.parallax.relief_steps`: binary-search refinement steps after the march (Bevy's
/// `ParallaxMappingMethod::Relief`); 0: one interpolation (`Occlusion`).
pub const PARALLAX_RELIEF_STEPS: Def<u32> = def!(count "texture.parallax.relief_steps" = 5)
    .range(0, 32)
    .unit(Unit::Count)
    .help("Parallax relief refinement steps after the march (0: one interpolation, parallax occlusion mapping)")
    .source(Source::BevyDefault("bevy_pbr::ParallaxMappingMethod::DEFAULT_RELIEF_MAPPING (5 steps)"))
    .group(Group::Textures)
    .lifetime(Lifetime::Recompile);


/// Binds the parallax tunables into [`MaterialMaps`] (after `MwAssetsPlugin`, which inits it).
pub fn add_parallax_tunables(app: &mut App) {
    let _layers = app.add_tunable(&PARALLAX_LAYERS).bind_resource(|m: &mut MaterialMaps, v| m.parallax_layers = v);
    let _steps = app.add_tunable(&PARALLAX_RELIEF_STEPS).bind_resource(|m: &mut MaterialMaps, v| m.parallax_relief_steps = v);
}

#[cfg(test)]
mod tests {
    use super::*;

    /// T5a: the preset's parallax retunes loaded depth-mapped materials live; 0 layers takes their
    /// depth maps out and a level back on returns them with the new settings; other maps untouched.
    #[test]
    fn parallax_preset_retunes_strips_and_restores_live() {
        let mut app = app(MaterialMaps::new(true, true, true));
        let keep = app.world_mut().resource_mut::<Assets<StandardMaterial>>().add(mapped());
        let id = keep.id();
        app.update();
        let set = |app: &mut App, layers: u32, steps: u32| {
            let mut maps = app.world_mut().resource_mut::<MaterialMaps>();
            (maps.parallax_layers, maps.parallax_relief_steps) = (layers, steps);
            app.update();
        };
        set(&mut app, 16, 0);
        let m = get(&app, id);
        assert_eq!((m.parallax_mapping_method, m.max_parallax_layer_count), (ParallaxMappingMethod::Occlusion, 16.0));
        set(&mut app, 0, 0);
        let m = get(&app, id);
        assert!(m.depth_map.is_none() && m.normal_map_texture.is_some(), "depth out, normals kept");
        set(&mut app, 64, 8);
        let m = get(&app, id);
        assert!(m.depth_map.is_some());
        assert_eq!((m.parallax_mapping_method, m.max_parallax_layer_count), (ParallaxMappingMethod::Relief { max_steps: 8 }, 64.0));
        assert_eq!(app.world().resource::<MaterialMaps>().stashed(), 0);
    }

    fn mapped() -> StandardMaterial {
        let mut material = StandardMaterial {
            normal_map_texture: Some(Handle::default()),
            metallic_roughness_texture: Some(Handle::default()),
            occlusion_texture: Some(Handle::default()),
            metallic: 1.0,
            perceptual_roughness: 1.0,
            reflectance: 0.5,
            ..StandardMaterial::default()
        };
        Parallax::new(32, Some(8)).apply(&mut material, Handle::default());
        material
    }

    fn app(maps: MaterialMaps) -> App {
        let mut app = App::new();
        app.add_plugins((MinimalPlugins, AssetPlugin::default()))
            .init_asset::<Image>()
            .init_asset::<StandardMaterial>()
            .insert_resource(maps)
            .add_systems(PostUpdate, apply_material_maps);
        app
    }

    fn get(app: &App, id: AssetId<StandardMaterial>) -> StandardMaterial {
        app.world().resource::<Assets<StandardMaterial>>().get(id).expect("material").clone()
    }

    /// Off strips the maps (the pre-TS-2 factors for ORM, Bevy's defaults for parallax), on
    /// restores every field as it was; one switch leaves the others' maps alone. Materials added
    /// while off are stripped too.
    #[test]
    fn flipping_off_then_on_restores_identical_materials() {
        let mut app = app(MaterialMaps::new(true, true, true));
        let keep = app.world_mut().resource_mut::<Assets<StandardMaterial>>().add(mapped());
        let id = keep.id();
        let original = format!("{:?}", get(&app, id));
        app.update();
        assert_eq!(format!("{:?}", get(&app, id)), original, "all on: untouched");

        app.world_mut().resource_mut::<MaterialMaps>().normal_maps = false;
        app.update();
        let m = get(&app, id);
        assert!(m.normal_map_texture.is_none());
        assert!(m.metallic_roughness_texture.is_some() && m.occlusion_texture.is_some() && m.metallic > 0.5, "ORM kept");
        assert!(m.depth_map.is_some(), "depth kept");

        app.world_mut().resource_mut::<MaterialMaps>().orm = false;
        app.world_mut().resource_mut::<MaterialMaps>().parallax = false;
        let keep_late = app.world_mut().resource_mut::<Assets<StandardMaterial>>().add(mapped());
        let late = keep_late.id();
        app.update();
        let plain = StandardMaterial::default();
        for id in [id, late] {
            let m = get(&app, id);
            assert!(m.normal_map_texture.is_none() && m.metallic_roughness_texture.is_none() && m.occlusion_texture.is_none());
            assert_eq!((m.metallic, m.perceptual_roughness, m.reflectance), (PLAIN_METALLIC, PLAIN_ROUGHNESS, PLAIN_REFLECTANCE));
            assert!(m.depth_map.is_none());
            assert_eq!(
                (m.parallax_depth_scale, m.parallax_mapping_method, m.max_parallax_layer_count),
                (plain.parallax_depth_scale, plain.parallax_mapping_method, plain.max_parallax_layer_count),
                "as without a depth companion"
            );
        }
        assert_eq!(app.world().resource::<MaterialMaps>().stashed(), 2);

        app.world_mut().resource_mut::<MaterialMaps>().normal_maps = true;
        app.update();
        assert!(get(&app, id).normal_map_texture.is_some() && get(&app, id).metallic_roughness_texture.is_none());

        app.world_mut().resource_mut::<MaterialMaps>().parallax = true;
        app.update();
        assert!(get(&app, id).depth_map.is_some() && get(&app, id).metallic_roughness_texture.is_none());

        app.world_mut().resource_mut::<MaterialMaps>().orm = true;
        app.update();
        assert_eq!(format!("{:?}", get(&app, id)), original, "field for field");
        assert_eq!(format!("{:?}", get(&app, late)), original);
        assert_eq!(app.world().resource::<MaterialMaps>().stashed(), 0);
    }

    /// The defaults stripped ORM falls back to are the ones a material without a companion gets.
    #[test]
    fn plain_factors_match_the_material_builder() {
        let surface = crate::Surface {
            base_color: Color::WHITE,
            base_texture: None,
            base_clamp: [false; 2],
            glow_texture: None,
            normal_texture: None,
            depth_texture: None,
            orm: None,
            emissive: LinearRgba::BLACK,
            foliage: false,
            alpha_mode: AlphaMode::Opaque,
            double_sided: false,
            unlit: false,
        };
        let m = surface.to_material(|_, _, _| None);
        assert_eq!((m.metallic, m.perceptual_roughness, m.reflectance), (PLAIN_METALLIC, PLAIN_ROUGHNESS, PLAIN_REFLECTANCE));
    }

    /// The preset's settings land on Bevy's fields: Occlusion without relief steps, Relief with.
    #[test]
    fn parallax_settings_pick_bevys_method() {
        let mut m = StandardMaterial::default();
        Parallax::new(16, None).apply(&mut m, Handle::default());
        assert_eq!((m.parallax_mapping_method, m.max_parallax_layer_count), (ParallaxMappingMethod::Occlusion, 16.0));
        assert!(m.depth_map.is_some() && (m.parallax_depth_scale - PARALLAX_DEPTH_SCALE).abs() < f32::EPSILON);
        assert_eq!(Parallax::new(32, Some(5)), Parallax::HIGH);
        assert_eq!(Parallax::HIGH.method, ParallaxMappingMethod::DEFAULT_RELIEF_MAPPING, "Bevy's relief default");
    }
}
