//! Budgeted point-light shadows (spec `docs/specs/sky-lighting.md`,
//! "Local-light shadows").
//!
//! A shadowed point light renders a cube map (six views of every caster in
//! its range), so games give shadows to the few most important local lights
//! and fade them with distance (Unity HDRP's shadow fade distance and
//! max shadows on screen, UE's per-light shadow fade, Skyrim's shadowed-light
//! cap). Bevy 0.20 has the cube maps (`PointLight::shadow_maps_enabled`,
//! `PointLightShadowMap`) but neither a budget nor a per-light shadow
//! strength, so this module:
//!
//! - ranks the shown lights marked [`BudgetedShadow`] around the viewer, in
//!   every direction and whether on screen or not, by [`importance`] at the
//!   viewer (spec `docs/specs/shadows.md`, "Light selection"), with
//!   hysteresis ([`pick_shadowed`]), and lets at most
//!   [`PointShadowBudget::max`] hold a shadow map at once. Only lights
//!   sharing `RenderLayers` with the viewer compete (lights of other
//!   resident worldspaces never do). The pick depends on where the viewer
//!   stands, never on where it looks, so turning the camera changes nothing
//!   (cached-shadow engines pick the same way: HDRP cached punctual shadows,
//!   UE cached shadow maps, `OpenMW` Clockwork's slots by contribution at the
//!   camera including off-screen lights);
//! - keeps a slot holder's shadow map alive while it is off screen: holders
//!   carry Bevy's `NoFrustumCulling`, so they stay `ViewVisibility`-visible
//!   and Bevy keeps extracting them, their caster lists and their cube-face
//!   views (a light that loses `ViewVisibility` loses all three, and its
//!   cached cube with them);
//! - fades a shadow in and out (distance band and over time) natively in the
//!   PBR fragment shader via [`PointLightShadowFade`] (`mix(1.0, shadow, fade)`),
//!   without mutating [`PointLight::intensity`] or spawning duplicate light
//!   entities;
//! - for lights with a [`ShadowReach`], blends [`PointLight::range`] from a
//!   short unshadowed reach to a long shadowed one with that same fade, so an
//!   unshadowed light doesn't light far through walls and a light gaining or
//!   losing its map never pops (spec `docs/specs/lights.md`, "Reach follows
//!   shadow state").
//!
//! A holder carrying [`ShadowMapCached`] (the lamp shadow cache, `lamp_cache.rs`) keeps
//! `PointLight::shadow_maps_enabled` false (the cache shades it as shadowed through its own
//! component) and its shadow fade does not rise until the cache's faces are all captured.
//!
//! Also works around a Bevy 0.20 leak (shadows.md "Stale cube-face views"):
//! `drop_stale_light_views` despawns the six face views of a light that
//! lost its shadow, which Bevy's `prepare_lights` keeps alive.

use bevy_app::{App, Plugin, PostUpdate};
use bevy_camera::visibility::{InheritedVisibility, NoFrustumCulling, RenderLayers, VisibilitySystems};
use bevy_camera::{Camera, Camera3d, RenderTarget};
use bevy_ecs::entity::EntityHashMap;
use bevy_ecs::prelude::*;
use bevy_light::{NotShadowCaster, PointLight, PointLightShadowMap};
use bevy_mesh::Mesh3d;
use bevy_pbr::{
    ExtractedPointLight, PointAndSpotLightViewEntities, PointLightShadowFade, prepare_lights,
};
use bevy_render::{Render, RenderApp, RenderSystems};
use bevy_time::Time;
use bevy_transform::TransformSystems;
use bevy_transform::components::GlobalTransform;
use sf_sky::SkyState;
use sf_tunables::{AnyDef, Def, Group, Source, Unit, def};
use sf_tunables_bevy::TunablesAppExt;

use crate::{CurrentSky, SkyCamera, SkyRenderOptions};

/// How many point lights cast shadows, and how they fade. A graphics
/// setting: games map their quality presets onto it.
#[derive(Resource, Clone, Copy, Debug, PartialEq)]
pub struct PointShadowBudget {
    /// Shadow maps at once (0: no point-light shadows).
    pub max: usize,
    /// Full shadow within this distance of the viewer, meters.
    pub fade_start: f32,
    /// No shadow past this distance, meters.
    pub fade_end: f32,
    /// Seconds for a shadow to fade fully in or out when a light gains or
    /// loses its slot.
    pub fade_time: f32,
    /// A light holding a slot keeps it until a challenger's importance is
    /// higher by `1 / hysteresis` (no flicker between two lights at similar distances).
    pub hysteresis: f32,
    /// Face size of the cube maps (`PointLightShadowMap::size`).
    pub map_size: usize,
    /// Screen-space contact shadows (`PointLight::contact_shadows_enabled`)
    /// on the lights holding a shadow map; needs Bevy's `ContactShadows` on
    /// the camera. One ray march per lit pixel per light, so only the
    /// budgeted few get it, never their twins.
    pub contact_shadows: bool,
    /// Maximum number of top-ranked shadowed point lights that receive
    /// screen-space contact shadows (`shadow.lamp.contact_max`, default 8).
    /// Prevents scaling the atlas to 48+ lamps from multiplying contact raymarch cost.
    pub contact_max: usize,
    /// Lights whose shadow would be imperceptible against the sky's light
    /// (noon sun) get no shadow map. Off by default.
    pub contrast_gate: ShadowContrastGate,
}

impl Default for PointShadowBudget {
    /// Off: the budget is opt-in per game and preset. The tunables' defaults.
    fn default() -> Self {
        let [fade_start, fade_end] = LAMP_FADE.default_value();
        Self {
            max: LAMP_COUNT.default_value(),
            fade_start,
            fade_end,
            fade_time: LAMP_FADE_TIME.default_value(),
            hysteresis: LAMP_HYSTERESIS.default_value(),
            map_size: LAMP_MAP_SIZE.default_value(),
            contact_shadows: LAMP_CONTACT.default_value(),
            contact_max: sf_sky::shadow_atlas::CONTACT_MAX.default_value() as usize,
            contrast_gate: ShadowContrastGate::default(),
        }
    }
}

/// `shadow.lamp.count`: [`PointShadowBudget::max`]. Off by default: a game and its presets opt in.
pub const LAMP_COUNT: Def<usize> = def!("shadow.lamp.count" = 0_usize)
    .range(0, 64)
    .unit(Unit::Count)
    .help("Point lights holding a shadow map at once (0: no point-light shadows)")
    .source(Source::Guess)
    .group(Group::Shadows);

/// `shadow.lamp.map_size_px`: [`PointShadowBudget::map_size`] (`PointLightShadowMap::size`). A
/// change reallocates the cube-map array (one hitch).
pub const LAMP_MAP_SIZE: Def<usize> = def!("shadow.lamp.map_size_px" = 1024_usize)
    .range(64, 4096)
    .unit(Unit::Pixels)
    .help("Face size of the point-light shadow cube maps (a change reallocates them)")
    .source(Source::BevyDefault("bevy_light::PointLightShadowMap::size"))
    .group(Group::Shadows);

/// `shadow.lamp.fade_m`: [`PointShadowBudget::fade_start`] and [`PointShadowBudget::fade_end`]. The
/// framework's generic band; a game sets its own at the `Mod` layer (census "two truths").
pub const LAMP_FADE: Def<[f32; 2]> = def!(float2 "shadow.lamp.fade_m" = [15.0, 25.0])
    .range([0.0, 0.0], [500.0, 500.0])
    .unit(Unit::Meters)
    .help("Lamp shadows at full strength within the first distance of the viewer, gone past the second")
    .source(Source::Guess)
    .group(Group::Shadows);

/// `shadow.lamp.fade_time_s`: [`PointShadowBudget::fade_time`].
pub const LAMP_FADE_TIME: Def<f32> = def!(float "shadow.lamp.fade_time_s" = 0.3)
    .range(0.0, 5.0)
    .unit(Unit::Seconds)
    .help("Seconds for a lamp shadow to fade fully in or out when its light gains or loses a slot")
    .source(Source::Guess)
    .group(Group::Shadows);

/// `shadow.lamp.hysteresis`: [`PointShadowBudget::hysteresis`].
pub const LAMP_HYSTERESIS: Def<f32> = def!(float "shadow.lamp.hysteresis" = 0.8)
    .range(0.1, 1.0)
    .unit(Unit::Ratio)
    .help("A light holding a shadow slot keeps it until a challenger's importance is higher by 1 / this")
    .source(Source::Guess)
    .group(Group::Shadows);

/// `shadow.lamp.contact`: [`PointShadowBudget::contact_shadows`] (also needs the view's contact
/// shadows, `shadow.contact`).
pub const LAMP_CONTACT: Def<bool> = def!(bool "shadow.lamp.contact" = false)
    .help("Screen-space contact shadows on the lamps holding a shadow map (needs shadow.contact on the view)")
    .source(Source::BevyDefault("bevy_light::PointLight::contact_shadows_enabled"))
    .group(Group::Shadows);

/// `shadow.lamp.day_gate`: [`ShadowContrastGate::enabled`].
pub const DAY_GATE: Def<bool> = def!(bool "shadow.lamp.day_gate" = true)
    .help("Point lights whose shadow would be imperceptible against the sun and ambient get no shadow map")
    .source(Source::Owner("2026-11-17: on by default after the owner's play session with every gate on; A/B docs/ledger/perf.md 2026-11-17"))
    .group(Group::Shadows);

/// `shadow.lamp.day_gate.receiver_m`: [`ShadowContrastGate::receiver_distance`].
pub const DAY_GATE_RECEIVER: Def<f32> = def!(float "shadow.lamp.day_gate.receiver_m" = 2.0)
    .range(0.1, 20.0)
    .unit(Unit::Meters)
    .help("Typical distance from a point light to the surfaces its shadow falls on")
    .group(Group::Shadows);

/// `shadow.lamp.day_gate.contrast`: [`ShadowContrastGate::contrast_off`] and
/// [`ShadowContrastGate::contrast_full`]. Anchored on Weber's luminance
/// threshold (~1-2 % on a uniform field), raised for textured, tonemapped,
/// temporally resolved frames (contrast masking); nobody measured it.
pub const DAY_GATE_CONTRAST: Def<[f32; 2]> = def!(float2 "shadow.lamp.day_gate.contrast" = [0.05, 0.15])
    .range([0.0, 0.0], [1.0, 1.0])
    .unit(Unit::Ratio)
    .help("Shadow contrast (the light's share of a receiver's light): no shadow map below the first, full strength above the second")
    .group(Group::Shadows);

/// `shadow.lamp.face_cull.margin_deg`: `bevy_pbr::PointShadowFaceCull::margin_deg`, bound in
/// [`crate::SkyPlugin`] (ROADMAP 1.25 CW-3b; the switch itself is the game's). A cube face is
/// kept while a visible receiver lies within this angle of it: the lamp filter's 2x2 PCF and
/// the normal-offset bias reach a few texels across a seam (0.18 deg per texel at 512², half
/// that at the face edge), so 2 deg keeps ~10 texels of slack; nobody measured it.
pub const FACE_CULL_MARGIN: Def<f32> = def!(float "shadow.lamp.face_cull.margin_deg" = 2.0)
    .range(0.0, 30.0)
    .unit(Unit::Degrees)
    .help("Point shadow cube faces are culled unless a visible receiver lies within this angle of them")
    .group(Group::Shadows);

/// `shadow.lamp.face_cull`: `bevy_pbr::PointShadowFaceCull::enabled` (ROADMAP 1.25 CW-3b).
pub const FACE_CULL: Def<bool> = def!(bool "shadow.lamp.face_cull" = true)
    .help("Point shadow cube faces no camera sees a receiver of are not drawn")
    .source(Source::Owner("2026-11-17: on by default after the owner's play session with every gate on; A/B docs/ledger/perf.md 2026-11-17"))
    .group(Group::Shadows);

/// `shadow.lamp.batched_pass`: `bevy_pbr::PointShadowBatchedPass` (ROADMAP 1.25 CW-13a).
pub const BATCHED_PASS: Def<bool> = def!(bool "shadow.lamp.batched_pass" = false)
    .help("Batch all active point/spot shadow face passes into one single command encoder instead of one Core3dShadows schedule per face")
    .source(Source::Guess)
    .group(Group::Shadows);

/// This module's tunables (bound by [`crate::ShadowQualityPlugin`], tunables slice T4).
pub const TUNABLES: &[&dyn AnyDef] = &[
    &LAMP_COUNT,
    &LAMP_MAP_SIZE,
    &LAMP_FADE,
    &LAMP_FADE_TIME,
    &LAMP_HYSTERESIS,
    &LAMP_CONTACT,
    &DAY_GATE,
    &DAY_GATE_RECEIVER,
    &DAY_GATE_CONTRAST,
    &FACE_CULL,
    &FACE_CULL_MARGIN,
    &BATCHED_PASS,
];

/// Shadow maps only where a light's shadow would be seen: a point light's
/// shadow darkens a receiver by the light's share of the light it gets, its
/// contrast `E_light / (E_light + E_sky)`. Outdoors at noon the sun gives
/// ~100 klx and a lamp ~1 klx at a couple of meters: its shadow is a ~1 %
/// darkening nobody sees, yet its cube map costs a caster scan and six
/// views. The gate weighs each candidate's [`importance`] and shadow
/// strength by [`Self::weight`], so shadows fade out as the sun rises and
/// back in at dusk (continuous in the sky's light; the budget's hysteresis
/// and timed fade apply as before), and lights at weight 0 hold no map.
///
/// `E_sky` is the direct sun and moon on level ground plus the ambient floor
/// ([`sky_reference_lux`]); only viewers with a [`crate::SkyCamera`] have
/// one (interiors: no gate). The sky's own diffuse light (the environment
/// map, GPU-only) isn't counted, so the gate keeps shadows a little longer
/// than strictly needed (overcast, dusk, sun-shadowed receivers).
///
/// It decides only whether a light casts a shadow map, never its light:
/// [`PointLight::intensity`] is untouched. A light with a [`ShadowReach`]
/// still blends its range by shadow strength, as when it loses a slot.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ShadowContrastGate {
    /// Gate on ([`DAY_GATE`]).
    pub enabled: bool,
    /// Typical light-to-receiver distance, meters ([`DAY_GATE_RECEIVER`]);
    /// at most half the light's range.
    pub receiver_distance: f32,
    /// Contrast at or below which a light gets no shadow map.
    pub contrast_off: f32,
    /// Contrast at or above which the gate leaves a light as it is.
    pub contrast_full: f32,
}

impl Default for ShadowContrastGate {
    fn default() -> Self {
        let [contrast_off, contrast_full] = DAY_GATE_CONTRAST.default_value();
        Self { enabled: DAY_GATE.default_value(), receiver_distance: DAY_GATE_RECEIVER.default_value(), contrast_off, contrast_full }
    }
}

impl ShadowContrastGate {
    /// A shadow's contrast on a receiver lit `light_lux` by the light and
    /// `sky_lux` by everything else: `E_light / (E_light + E_sky)`, 1 in the dark.
    #[must_use]
    pub fn contrast(light_lux: f32, sky_lux: f32) -> f32 {
        let light = light_lux.max(0.0);
        let total = light + sky_lux.max(0.0);
        if total > 0.0 { light / total } else { 1.0 }
    }

    /// The gate's weight for a shadow of `contrast`: 0 at or below
    /// [`Self::contrast_off`], 1 at or above [`Self::contrast_full`], linear
    /// between. A light holding a slot counts its contrast `1 / hysteresis`
    /// times (the budget's hysteresis), so a holder near the threshold keeps
    /// its map while the sun drifts.
    #[must_use]
    pub fn weight(&self, contrast: f32, held: bool, hysteresis: f32) -> f32 {
        let c = if held { contrast / hysteresis.max(1e-3) } else { contrast };
        if c >= self.contrast_full {
            1.0
        } else if c <= self.contrast_off || self.contrast_full <= self.contrast_off {
            0.0
        } else {
            (c - self.contrast_off) / (self.contrast_full - self.contrast_off)
        }
    }

    /// Illuminance a point light of `intensity` lumens and `range` meters
    /// puts on a receiver facing it at [`Self::receiver_distance`] (at most
    /// half the range), lux: `Φ / (4π d²)` times Bevy's range window
    /// `(1 - (d/R)⁴)²`.
    #[must_use]
    pub fn receiver_lux(&self, intensity: f32, range: f32) -> f32 {
        let range = range.max(1e-3);
        let d = self.receiver_distance.min(0.5 * range).max(0.05);
        let window = (1.0 - (d / range).powi(4)).max(0.0).powi(2);
        intensity.max(0.0) / (4.0 * std::f32::consts::PI * d * d) * window
    }
}

/// The light a receiver gets outdoors without local lights, lux: the direct
/// sun and moon on level ground (`ground illuminance · sin(elevation)`) plus
/// a uniform ambient of luminance `ambient_nits` (`π · L`).
#[must_use]
pub fn sky_reference_lux(sky: &SkyState, ambient_nits: f32) -> f32 {
    let direct = |l: &sf_sky::SkyLight| l.ground_illuminance() * l.elevation.sin().max(0.0);
    direct(&sky.sun) + direct(&sky.moon) + std::f32::consts::PI * ambient_nits.max(0.0)
}

/// The steady intensity of a light whose [`PointLight::intensity`] is
/// animated (flicker, pulse), lumens: the [`ShadowContrastGate`] reads it
/// instead, so its shadow decision doesn't follow the flicker.
#[derive(Component, Clone, Copy, Debug, PartialEq)]
pub struct SteadyIntensity(pub f32);

impl PointShadowBudget {
    /// Shadow strength by distance to the viewer: 1 to `fade_start`, 0 past
    /// `fade_end`, linear between.
    pub fn distance_fade(&self, distance: f32) -> f32 {
        if distance <= self.fade_start {
            1.0
        } else if distance >= self.fade_end || self.fade_end <= self.fade_start {
            0.0
        } else {
            (self.fade_end - distance) / (self.fade_end - self.fade_start)
        }
    }
}

/// How much a point light's shadow matters to a viewer `distance` meters
/// away (spec `docs/specs/shadows.md`, "Light selection"): its strength at
/// the viewer times the share of the view its lit volume covers,
/// `(range / max(d, 1))² · min(1, range / d)²`, times `priority`.
///
/// Strength enters through `range`, not `intensity`: games set the range
/// where the light fades out (Bevy's cutoff), so `range²` tracks the
/// light's power, and it doesn't change with flicker (an animated
/// intensity would reshuffle the shadowed set every frame). Inside its
/// range a light scores its illuminance at the eye (`∝ range² / d²`);
/// outside, that falls off once more by its screen coverage. Clamped below
/// 1 m so the light you stand next to doesn't swamp the ranking.
#[must_use]
pub fn importance(range: f32, distance: f32, priority: f32) -> f32 {
    let near = (range / distance.max(1.0)).powi(2);
    let coverage = (range / distance.max(1e-3)).min(1.0).powi(2);
    priority.max(0.0) * near * coverage
}

/// A point light that may cast shadows within the [`PointShadowBudget`].
/// `priority` multiplies its [`importance`] (a carried torch over a lamp).
#[derive(Component, Clone, Copy, Debug)]
pub struct BudgetedShadow {
    pub priority: f32,
}

impl Default for BudgetedShadow {
    fn default() -> Self {
        Self { priority: 1.0 }
    }
}

/// Picks the candidates that hold a shadow map: the `max` best by
/// `(importance, holds a slot now)`, a holder's importance counting
/// `1 / hysteresis` times (so a challenger takes a held slot only once it is
/// that much more important; no flicker between two lights at similar
/// distances). Returns indices into `candidates`, best first; candidates of
/// no importance never hold a map.
#[must_use]
pub fn pick_shadowed(candidates: &[(f32, bool)], max: usize, hysteresis: f32) -> Vec<usize> {
    let boost = 1.0 / hysteresis.max(1e-3);
    let mut ranked: Vec<(f32, usize)> = candidates
        .iter()
        .enumerate()
        .filter(|(_, (score, _))| *score > 0.0)
        .map(|(i, &(score, held))| (if held { score * boost } else { score }, i))
        .collect();
    // Ties go to the earlier candidate, so equal lights never swap.
    ranked.sort_by(|a, b| b.0.total_cmp(&a.0).then(a.1.cmp(&b.1)));
    ranked.truncate(max);
    ranked.into_iter().map(|(_, i)| i).collect()
}

/// A [`BudgetedShadow`] light's range by shadow state: the budget writes
/// [`PointLight::range`] as [`Self::range`] of the light's shadow fade
/// (0 without a slot), so it reaches [`Self::shadowed`] only while its shadow
/// map can stop it at walls. Ranking uses [`Self::shadowed`], the reach a
/// slot would give, so a light's own state never feeds its importance.
/// Bevy's range window `(1 - (d/R)^4)^2` is exactly 0 at the range, so a
/// shorter range cuts the pool smoothly, without a ring.
#[derive(Component, Clone, Copy, Debug, PartialEq)]
pub struct ShadowReach {
    /// Range with no shadow map, meters.
    pub unshadowed: f32,
    /// Range with a full-strength shadow, meters.
    pub shadowed: f32,
}

impl ShadowReach {
    /// The range at shadow strength `fade` (0..=1): linear from
    /// [`Self::unshadowed`] to [`Self::shadowed`].
    #[must_use]
    pub fn range(&self, fade: f32) -> f32 {
        self.unshadowed + (self.shadowed - self.unshadowed) * fade.clamp(0.0, 1.0)
    }
}

/// Writes `range` into the light only when it moved (keeps change detection quiet).
fn set_range(light: &mut Mut<PointLight>, range: f32) {
    if (light.range - range).abs() > 1e-4 {
        light.range = range;
    }
}

/// Budget state of a [`BudgetedShadow`] light, added when it first gets a
/// shadow map.
#[derive(Component, Clone, Copy, Debug, Default)]
pub struct ShadowSlot {
    /// Current shadow strength, 0..=1.
    pub fade: f32,
    /// Holds one of the budget's shadow maps (fading in, shown or fading out).
    pub held: bool,
    /// The score [`budget_shadows`] ranked the light with this frame (importance by reach,
    /// distance and priority, times its contrast and luminance weights); 0 when it was not
    /// ranked. The lamp cache admits new holders into the Static Atlas in this order.
    pub importance: f32,
}

/// On a slot holder the budget gave Bevy's `NoFrustumCulling` to (so its
/// shadow map survives being off screen); both go when the slot is released.
/// A light that already had `NoFrustumCulling` keeps it and never gets this.
#[derive(Component, Clone, Copy, Debug, Default)]
pub struct ShadowHeldOffscreen;

/// On a slot holder whose cube map the lamp shadow cache manages (`crate::lamp_cache`, CW-4): the
/// budget leaves its `PointLight::shadow_maps_enabled` false (the vendored
/// `bevy_pbr::CachedPointShadowMap` shades it as shadowed) and, while `ready` is false (cube faces
/// not all captured), its shadow fade does not rise. The cache inserts and removes it.
#[derive(Component, Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct ShadowMapCached {
    /// Every cube face holds its static depth: the shadow may fade in.
    pub ready: bool,
}

/// The point-shadow budget on its own (`SkyPlugin` adds it; viewers without
/// a sky add it directly).
pub struct PointShadowPlugin;

impl Plugin for PointShadowPlugin {
    fn build(&self, app: &mut App) {
        add(app);
    }

    fn finish(&self, app: &mut App) {
        crate::lamp_cache::finish(app);
        crate::shadow_atlas::finish(app);
    }
}

fn add(app: &mut App) {
    crate::gpu_culled_casters::add(app);
    // CW-3b: per-face culling of lamp cube maps (vendored `bevy_pbr`, own patch); the game
    // turns it on (`enabled`), the margin is this crate's tunable.
    app.init_resource::<bevy_pbr::PointShadowFaceCull>()
        .init_resource::<bevy_pbr::PointShadowBatchedPass>()
        .register_tunables(TUNABLES);
    let _bound = app
        .add_tunable(&FACE_CULL_MARGIN)
        .bind_resource(|cull: &mut bevy_pbr::PointShadowFaceCull, margin| cull.margin_deg = margin);
    app.init_resource::<PointShadowBudget>()
        .add_systems(
            PostUpdate,
            (sync_map_size, budget_shadows, warn_casterless_shadows)
                .chain()
                .after(TransformSystems::Propagate)
                .before(VisibilitySystems::VisibilityPropagate),
        );
    crate::shadow_atlas::add(app);
    crate::lamp_cache::add(app);
    if let Some(render_app) = app.get_sub_app_mut(RenderApp) {
        render_app.add_systems(Render, drop_stale_light_views.in_set(RenderSystems::CreateViews).after(prepare_lights));
    }
}

/// Despawns the cube-face views of point and spot lights that have no
/// shadow map this frame (shadow turned off, or the light hidden).
///
/// Bevy 0.20's `prepare_lights` frees a light's views only while the light
/// sorts among the first `min(point lights, max cubes)` (42 at wgpu's 256
/// layers), shadowed lights first, then by entity. A light that loses its
/// shadow usually sorts far past that among hundreds of unshadowed lights,
/// and a hidden light isn't visited at all, so its six views stay alive and
/// each runs the whole `Core3d` schedule every frame (~0.35 ms of render
/// thread per face). Remove once Bevy frees them itself.
fn drop_stale_light_views(
    mut commands: Commands,
    mut lights: Query<(&mut PointAndSpotLightViewEntities, Option<&ExtractedPointLight>)>,
) {
    for (mut views, light) in &mut lights {
        if views.is_empty() || light.is_some_and(|l| l.shadow_maps_enabled) {
            continue;
        }
        for view in std::mem::take(&mut **views) {
            if let Ok(mut e) = commands.get_entity(view) {
                e.despawn();
            }
        }
    }
}

fn sync_map_size(budget: Res<PointShadowBudget>, map: Option<ResMut<PointLightShadowMap>>) {
    if let Some(mut map) = map
        && budget.is_changed()
        && map.size != budget.map_size
    {
        map.size = budget.map_size;
    }
}

type ViewerCameras<'w, 's> = Query<
    'w,
    's,
    (&'static Camera, &'static GlobalTransform, Option<&'static RenderTarget>, Option<&'static RenderLayers>, Has<SkyCamera>),
    With<Camera3d>,
>;

/// The viewer: the highest-order active 3D camera drawing to a window. Its
/// position, the `RenderLayers` it draws and whether it sees the sky
/// (a [`SkyCamera`]: outdoors).
fn viewer(cameras: &ViewerCameras) -> Option<(bevy_math::Vec3, RenderLayers, bool)> {
    cameras
        .iter()
        .filter(|(c, _, t, _, _)| c.is_active && matches!(t, None | Some(RenderTarget::Window(_))))
        .max_by_key(|(c, _, _, _, _)| c.order)
        .map(|(_, t, _, layers, sky)| (t.translation(), layers.cloned().unwrap_or_default(), sky))
}

/// The sky light a [`ShadowContrastGate`] compares lights against, lux, or
/// `None` (gate off, no sky viewer, no sky).
fn gate_reference(
    budget: &PointShadowBudget,
    outdoors: bool,
    sky: Option<&CurrentSky>,
    options: Option<&SkyRenderOptions>,
) -> Option<f32> {
    if !budget.contrast_gate.enabled || !outdoors {
        return None;
    }
    let sky = &sky?.0;
    let ambient = options.map_or(1.0, |o| {
        let c = crate::ambient_color(sky, o).to_linear();
        sf_sky::luminance(bevy_math::Vec3::new(c.red, c.green, c.blue))
    });
    Some(sky_reference_lux(sky, sky.ambient * ambient))
}

/// Initial interior fill stagger under 2D shadow atlas mode.
const ATLAS_NEW_SLOTS_PER_FRAME: usize = 6; // tunables: const (capacity: initial interior fill stagger)

/// Picks the shadowed lights and advances their native shader shadow fades.
#[allow(clippy::type_complexity, clippy::too_many_arguments)]
#[expect(
    clippy::too_many_lines,
    reason = "one pass: score, pick, then fade the picked lights; splitting it would thread the same query through helpers"
)]
pub(crate) fn budget_shadows(
    mut commands: Commands,
    budget: Res<PointShadowBudget>,
    atlas: Option<Res<crate::shadow_atlas::LampShadowAtlas>>,
    time: Res<Time>,
    cameras: ViewerCameras,
    sky: Option<Res<CurrentSky>>,
    sky_options: Option<Res<SkyRenderOptions>>,
    mut lights: Query<(
        Entity,
        &mut PointLight,
        Option<&mut ShadowSlot>,
        Option<&mut PointLightShadowFade>,
        &BudgetedShadow,
        Option<&ShadowReach>,
        &GlobalTransform,
        (
            &InheritedVisibility,
            Option<&RenderLayers>,
            Has<NoFrustumCulling>,
            Has<ShadowHeldOffscreen>,
            Option<&SteadyIntensity>,
            Option<&ShadowMapCached>,
        ),
    )>,
) {
    let viewer = viewer(&cameras);
    let eye = viewer.as_ref().map(|v| v.0);
    let outdoors = viewer.as_ref().is_some_and(|v| v.2);
    let reference = gate_reference(&budget, outdoors, sky.as_deref(), sky_options.as_deref());
    let gate = budget.contrast_gate;
    let atlas_cfg = atlas.as_deref().filter(|a| a.enabled).map(|a| a.config);
    // Low preset (`budget.max == 0`) keeps point shadows off even if atlas is toggled on.
    let effective_max = if budget.max == 0 {
        0
    } else if let Some(cfg) = atlas_cfg {
        cfg.max_lamps as usize
    } else {
        budget.max
    };
    let handover_spares = atlas_cfg.map_or(0, |cfg| cfg.handover_spares as usize);
    let slot_capacity = effective_max.saturating_add(handover_spares);
    // A cached lamp's shadow does not rise until its faces are captured (CW-4). Under the atlas
    // every slot holder is the lamp cache's: one it has not admitted yet, or cannot admit (Static
    // Atlas full), has no `ShadowMapCached` and waits the same way, unshadowed at its short reach
    // (F4), and the budget never turns Bevy's cube map on for it.
    let atlas_on = atlas_cfg.is_some();
    let awaits_map = |cached: Option<&ShadowMapCached>| cached.map_or(atlas_on, |c| !c.ready);
    let stock_map = |cached: Option<&ShadowMapCached>| !atlas_on && cached.is_none();

    // The candidates: shown lights of the viewer's `RenderLayers` (a light of
    // another resident worldspace lives in another `BigSpace` frame, where
    // the distance below means nothing) inside the fade band, in every
    // direction. Where the viewer looks plays no part, so a camera turn
    // changes neither the pick nor any fade: a holder off screen keeps its
    // slot and its cached map (`ShadowHeldOffscreen`). `InheritedVisibility`
    // is propagated later in `PostUpdate`, so this reads last frame's value.
    //
    // The contrast gate (outdoors, when on) weighs each candidate by how
    // visible its shadow would be against the sky's light: its importance
    // (so the most visible win the slots) and its shadow strength (so it
    // fades as the sun rises); weight 0 holds no map.
    let mut candidates: Vec<(f32, bool)> = Vec::new();
    let mut candidate_lights: Vec<(Entity, f32)> = Vec::new();
    let mut scores: EntityHashMap<f32> = EntityHashMap::default();
    if let Some((eye, layers, _)) = &viewer
        && effective_max > 0
    {
        for (entity, light, slot, _, prio, reach, at, (shown, light_layers, _, _, steady, _)) in &lights {
            let d = at.translation().distance(*eye);
            if shown.get() && d < budget.fade_end && layers.intersects(light_layers.unwrap_or(&RenderLayers::default())) {
                // The reach a slot would give, not the current blend.
                let range = reach.map_or(light.range, |r| r.shadowed);
                let held = slot.is_some_and(|s| s.held);
                let steady_lm = steady.map_or(light.intensity, |s| s.0);
                let weight = reference.map_or(1.0, |sky_lux| {
                    let lux = gate.receiver_lux(steady_lm, range);
                    gate.weight(ShadowContrastGate::contrast(lux, sky_lux), held, budget.hysteresis)
                });
                let lum_weight = if atlas_cfg.is_some() {
                    let c = light.color.to_linear();
                    let color_lum = sf_sky::luminance(bevy_math::Vec3::new(c.red, c.green, c.blue));
                    sf_sky::shadow_atlas::luminance_weight(steady_lm, color_lum)
                } else {
                    1.0
                };
                let score = importance(range, d, prio.priority) * weight * lum_weight;
                candidates.push((score, held));
                candidate_lights.push((entity, weight));
                scores.insert(entity, score);
            }
        }
    }
    let ranked: Vec<(Entity, f32)> =
        pick_shadowed(&candidates, effective_max, budget.hysteresis).into_iter().map(|i| candidate_lights[i]).collect();

    let rank_of = |entity: Entity| ranked.iter().position(|(e, _)| *e == entity);
    let target_of = |entity: Entity, at: &GlobalTransform| match (eye, rank_of(entity)) {
        (Some(eye), Some(rank)) => budget.distance_fade(at.translation().distance(eye)) * ranked[rank].1,
        _ => 0.0,
    };

    let step = if budget.fade_time > 0.0 {
        time.delta_secs() / budget.fade_time
    } else {
        1.0
    };

    // First pass: advance existing held slots and release any that have finished
    // fading out, so newly ranked lights can claim freed slots in the same frame.
    let mut held = 0usize;
    for (entity, _, slot, _, _, _, at, (.., cached)) in &mut lights {
        let Some(mut slot) = slot else {
            continue;
        };
        if !slot.held {
            continue;
        }
        let target = target_of(entity, at);
        let wait_capture = awaits_map(cached);
        let fade = if slot.fade < target {
            if wait_capture { slot.fade } else { (slot.fade + step).min(target) }
        } else {
            (slot.fade - step).max(target)
        };
        slot.fade = fade;
        if fade <= 0.0 && target <= 0.0 {
            slot.held = false;
        } else {
            held += 1;
        }
    }

    // Second pass: assign free slots to newly ranked lights and sync components.
    // When `fade_time > 0.0`, stagger new slot acquisitions so multiple lamps never
    // capture 12+ cubemap faces on the same frame (higher under 2D atlas where tiles
    // are smaller and captured asynchronously into the Static Atlas).
    let max_new_slots = if budget.fade_time <= 0.0 {
        effective_max
    } else if atlas_cfg.is_some() {
        ATLAS_NEW_SLOTS_PER_FRAME
    } else {
        1
    };
    let mut new_slots = 0usize;
    for (entity, mut light, slot, shadow_fade, _, reach, at, (_, _, unculled, pinned, _, cached)) in &mut lights {
        let rank = rank_of(entity);
        let target = match (eye, rank) {
            (Some(eye), Some(r)) => budget.distance_fade(at.translation().distance(eye)) * ranked[r].1,
            _ => 0.0,
        };
        let contact_allowed = rank.is_some_and(|r| r < budget.contact_max);
        let wait_capture = awaits_map(cached);
        // The ranked score, for the lamp cache's admission order (0 when not a candidate).
        let score = scores.get(&entity).copied().unwrap_or(0.0);
        let Some(mut slot) = slot else {
            let mut fade = 0.0;
            if target > 0.0 && held < slot_capacity && new_slots < max_new_slots {
                held += 1;
                new_slots += 1;
                fade = if wait_capture { 0.0 } else { step.min(target) };
                let shadows = fade > 0.0;
                let map = shadows && stock_map(cached);
                if light.shadow_maps_enabled != map {
                    light.shadow_maps_enabled = map;
                }
                let contact = shadows && budget.contact_shadows && contact_allowed;
                if light.contact_shadows_enabled != contact {
                    light.contact_shadows_enabled = contact;
                }
                commands.entity(entity).try_insert((
                    ShadowSlot { fade, held: true, importance: score },
                    PointLightShadowFade(fade),
                ));
                if !unculled {
                    commands.entity(entity).try_insert((NoFrustumCulling, ShadowHeldOffscreen));
                }
            }
            if let Some(reach) = reach {
                set_range(&mut light, reach.range(fade));
            }
            continue;
        };
        if slot.importance.to_bits() != score.to_bits() {
            slot.importance = score;
        }

        if !slot.held && target > 0.0 && held < slot_capacity && new_slots < max_new_slots {
            slot.held = true;
            held += 1;
            new_slots += 1;
            if !wait_capture {
                slot.fade = (slot.fade + step).min(target);
            }
        }

        // A holder stays in view for Bevy while it holds its slot.
        if slot.held && !unculled {
            commands.entity(entity).try_insert((NoFrustumCulling, ShadowHeldOffscreen));
        } else if !slot.held && pinned {
            commands.entity(entity).try_remove::<(NoFrustumCulling, ShadowHeldOffscreen)>();
        }

        let fade = slot.fade;
        let shadows = slot.held && fade > 0.0;
        if let Some(reach) = reach {
            set_range(&mut light, reach.range(if slot.held { fade } else { 0.0 }));
        }
        // A cached lamp's cube map is the cache's (its component shades it as shadowed).
        let map = shadows && stock_map(cached);
        if light.shadow_maps_enabled != map {
            light.shadow_maps_enabled = map;
        }
        let contact = shadows && budget.contact_shadows && contact_allowed;
        if light.contact_shadows_enabled != contact {
            light.contact_shadows_enabled = contact;
        }
        if let Some(mut shadow_fade) = shadow_fade {
            if (shadow_fade.0 - fade).abs() > 1e-5 {
                shadow_fade.0 = fade;
            }
        } else {
            commands.entity(entity).try_insert(PointLightShadowFade(fade));
        }
    }
}

/// Warns once per light when a [`BudgetedShadow`] light gains its first
/// shadow slot while its `RenderLayers` (layer 0 when absent) meet no
/// shadow caster's: Bevy builds a point light's caster list from meshes
/// sharing a layer with it, so such a shadow map stays empty (a light
/// spawned without its worldspace's layer is the usual cause).
///
/// Runs only when a light first gets a [`ShadowSlot`] (`Added`, so once per
/// light). The caster layers in use are cached and rescanned only when the
/// cache misses, so a caster layer that appeared since the last scan never
/// warns falsely; a stale cache can only miss a warning.
#[allow(clippy::type_complexity)]
fn warn_casterless_shadows(
    gained: Query<(Entity, Option<&RenderLayers>), (With<BudgetedShadow>, Added<ShadowSlot>)>,
    casters: Query<Option<&RenderLayers>, (With<Mesh3d>, Without<NotShadowCaster>)>,
    mut in_use: Local<Option<RenderLayers>>,
) {
    let scan = || casters.iter().fold(RenderLayers::none(), |all, l| all.union(l.unwrap_or(&RenderLayers::default())));
    for (entity, layers) in &gained {
        let layers = layers.cloned().unwrap_or_default();
        if in_use.as_ref().is_some_and(|c| c.intersects(&layers)) {
            continue;
        }
        let fresh = scan();
        let hit = fresh.intersects(&layers);
        *in_use = Some(fresh);
        if !hit {
            log::warn!(
                "point shadow on {entity}: its render layers {layers:?} meet no shadow caster's, so its shadow map stays empty (give the light its worldspace's layer)"
            );
        }
    }
}
