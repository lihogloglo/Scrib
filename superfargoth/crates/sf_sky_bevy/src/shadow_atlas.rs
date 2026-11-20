//! Bevy integration for the 2D Point Shadow Atlas (`CW-13b`, ROADMAP 1.25 CW-13).
//!
//! Policy and buddy allocator live Bevy-free in [`sf_sky::shadow_atlas`]; the render-world atlas
//! pass and shader live in vendored `bevy_pbr::point_shadow_atlas`. This module declares the Bevy
//! resources, live tunable bindings (`shadow.lamp.atlas.*` and `shadow.lamp.contact_max`), and
//! diagnostics gauges (`shadow_atlas/*`, `shadow_atlas/render/*`). The per-lamp caster tracking
//! and plan emission share [`crate::lamp_cache`]'s caster tracker so both CW-4 and CW-13b reuse
//! one battle-tested invalidation/classification pipeline.

use bevy_app::{App, PostUpdate};
use bevy_ecs::prelude::*;
use bevy_pbr::{
    AtlasPointShadowReport, PointShadowAtlasConfig, PointShadowAtlasFeedback,
    PointShadowAtlasStats,
};
use bevy_render::{Render, RenderApp, RenderSystems};
use sf_diag_bevy::DiagCounters;
use sf_sky::shadow_atlas::{self as policy, AtlasStats, ShadowAtlasConfig};
use sf_tunables_bevy::TunablesAppExt;

use crate::shadow_budget::{PointShadowBudget, budget_shadows};

/// Main-world switch and policy config for the 2D Point Shadow Atlas, bound live to
/// `shadow.lamp.atlas.*` tunables.
#[derive(Resource, Clone, Copy, Debug, Default, PartialEq)]
pub struct LampShadowAtlas {
    /// `shadow.lamp.atlas.enabled`: budgeted point lights use the unified Static + Live 2D Shadow
    /// Atlas instead of Bevy's cube array (supersedes CW-4 cube cache while enabled).
    pub enabled: bool,
    pub config: ShadowAtlasConfig,
}

/// Optional test/host feedback injection (mirrors [`crate::LampCacheReports`]).
#[derive(Resource, Default)]
pub struct LampShadowAtlasReports(pub Vec<(Entity, AtlasPointShadowReport)>);

/// Main-world diagnostics snapshot (`shadow_atlas/*` gauges).
#[derive(Resource, Clone, Copy, Debug, Default, PartialEq)]
pub struct LampShadowAtlasStats {
    pub atlas: AtlasStats,
    pub views: u32,
    pub moving_lamps: u32,
    pub movers: u32,
    pub captures: u32,
    pub mover_draws: u32,
    pub scans: u64,
}

pub(crate) fn add(app: &mut App) {
    app.register_tunables(policy::TUNABLES);
    bind(app, &policy::ENABLED, |a, v| a.enabled = v);
    bind(app, &policy::STATIC_SIZE_PX, |a, v| a.config.static_size_px = v);
    bind(app, &policy::LIVE_SIZE_PX, |a, v| a.config.live_size_px = v);
    bind(app, &policy::GUARD_TEXELS, |a, v| a.config.guard_texels = v);
    bind(app, &policy::TIER_512_M, |a, v| a.config.tier_512_m = v);
    bind(app, &policy::TIER_256_M, |a, v| a.config.tier_256_m = v);
    bind(app, &policy::TIER_HYSTERESIS, |a, v| a.config.tier_hysteresis = v);
    bind(app, &policy::RESIZE_COOLDOWN_S, |a, v| a.config.resize_cooldown_s = v);
    bind(app, &policy::MAX_LAMPS, |a, v| a.config.max_lamps = v);
    bind(app, &policy::HANDOVER_SPARES, |a, v| a.config.handover_spares = v);
    bind(app, &policy::SOFT_PCF_RADIUS, |a, v| a.config.soft_pcf_radius = v);
    let _contact = app
        .add_tunable(&policy::CONTACT_MAX)
        .bind_resource(|b: &mut PointShadowBudget, v| b.contact_max = v as usize);

    app.init_resource::<LampShadowAtlas>()
        .init_resource::<LampShadowAtlasReports>()
        .init_resource::<LampShadowAtlasStats>()
        .init_resource::<PointShadowAtlasConfig>()
        .init_resource::<PointShadowAtlasFeedback>()
        .add_systems(PostUpdate, sync_atlas_render_config.before(budget_shadows));
}

pub(crate) fn finish(app: &mut App) {
    let Some(counters) = app.world().get_resource::<DiagCounters>().cloned() else {
        return;
    };
    if let Some(render_app) = app.get_sub_app_mut(RenderApp) {
        if !render_app.world().contains_resource::<DiagCounters>() {
            render_app.insert_resource(counters);
        }
        render_app.add_systems(Render, render_atlas_gauges.in_set(RenderSystems::Cleanup));
    }
}

fn bind<T: sf_tunables::Type + Sync + Send>(
    app: &mut App,
    def: &sf_tunables::Def<T>,
    apply: impl for<'v> Fn(&mut LampShadowAtlas, T::Read<'v>) + Send + Sync + 'static,
) {
    let _bound = app.add_tunable(def).bind_resource(apply);
}

/// Syncs [`LampShadowAtlas`] into vendored `bevy_pbr::PointShadowAtlasConfig` (extracted to render world).
fn sync_atlas_render_config(
    atlas: Res<LampShadowAtlas>,
    mut render_cfg: ResMut<PointShadowAtlasConfig>,
) {
    let desired = PointShadowAtlasConfig {
        enabled: atlas.enabled,
        static_size: u32::from(atlas.config.clamped_static_size()),
        live_size: u32::from(atlas.config.clamped_live_size()),
        guard_texels: atlas.config.guard_texels,
        soft_pcf_radius: atlas.config.soft_pcf_radius,
        max_lamps: atlas.config.max_lamps + atlas.config.handover_spares,
    };
    render_cfg.set_if_neq(desired);
}

pub(crate) fn publish_atlas_gauges(counters: &DiagCounters, s: &LampShadowAtlasStats) {
    let count = sf_diag_bevy::sf_diag::unit::COUNT;
    let pct = "%";
    for (name, value) in [
        ("lamps", s.atlas.lamps),
        ("static_tiles", s.atlas.static_tiles),
        ("live_tiles", s.atlas.live_tiles),
        ("empty_faces", s.atlas.empty_faces),
        ("alloc_failures", s.atlas.alloc_failures),
        ("resizes", s.atlas.resizes),
        ("views", s.views),
        ("moving_lamps", s.moving_lamps),
        ("movers", s.movers),
        ("captures", s.captures),
        ("mover_draws", s.mover_draws),
    ] {
        counters.set_named(&format!("shadow_atlas/{name}"), count, f64::from(value));
    }
    #[allow(clippy::cast_precision_loss)]
    let pct_of = |used: u64, total: u64| if total > 0 { (used as f64 / total as f64) * 100.0 } else { 0.0 };
    counters.set_named("shadow_atlas/static_occ_pct", pct, pct_of(s.atlas.static_used_texels, s.atlas.static_total_texels));
    counters.set_named("shadow_atlas/live_occ_pct", pct, pct_of(s.atlas.live_used_texels, s.atlas.live_total_texels));
}

fn render_atlas_gauges(
    stats: Option<Res<PointShadowAtlasStats>>,
    counters: Res<DiagCounters>,
    mut active: Local<bool>,
) {
    let Some(s) = stats else { return };
    let idle = *s == PointShadowAtlasStats::default();
    if idle && !*active {
        return;
    }
    *active = !idle;
    let count = sf_diag_bevy::sf_diag::unit::COUNT;
    for (name, value) in [
        ("lamps", s.lamps),
        ("views", s.views),
        ("static_draws", s.static_draws),
        ("live_indirect", s.live_indirect_draws),
        ("live_direct", s.live_direct_draws),
        ("blits", s.blits),
        ("clears", s.clears),
    ] {
        counters.set_named(&format!("shadow_atlas/render/{name}"), count, f64::from(value));
    }
}
