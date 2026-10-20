//! Time-sliced sky environment map (sky-lighting.md "Environment map refresh").
//!
//! Bevy 0.20 re-filters a `GeneratedEnvironmentMapLight` (the sky's
//! `AtmosphereEnvironmentMapLight`) every frame: a copy, a mip chain and the
//! irradiance and radiance convolutions, ~0.8 ms of GPU at 1440p on a laptop
//! RTX 4060 (Nsight, bevyrrowind perf.md). The sky changes slowly, so engines
//! refresh sky captures at a lower rate (UE sky light real-time capture time
//! slicing, HDRP reflection probe update modes). Here the filtered maps keep
//! their last result and are refreshed every [`EnvMapRefresh`] frames: the
//! generator's systems only run for render entities that carry Bevy's
//! `RenderEnvironmentMap`, so on the other frames it is removed right after
//! extraction. A camera is always filtered on the first frame it is seen.

use bevy_app::App;
use bevy_ecs::entity::EntityHashSet;
use bevy_ecs::prelude::*;
use bevy_pbr::generate::RenderEnvironmentMap;
use bevy_render::extract_resource::{ExtractResource, ExtractResourcePlugin};
use bevy_render::{Render, RenderApp, RenderSystems};

/// Re-filter the sky environment map every this many frames (1: every frame,
/// Bevy's behaviour). A graphics setting.
#[derive(Resource, Clone, Copy, Debug, PartialEq, Eq)]
pub struct EnvMapRefresh(pub u32);

impl ExtractResource<RenderApp> for EnvMapRefresh {
    type Source = Self;

    fn extract_resource(source: &Self) -> Self {
        *source
    }
}

impl Default for EnvMapRefresh {
    fn default() -> Self {
        Self(8)
    }
}

pub(crate) fn build(app: &mut App) {
    app.init_resource::<EnvMapRefresh>().add_plugins(ExtractResourcePlugin::<EnvMapRefresh>::default());
    if let Some(render_app) = app.get_sub_app_mut(RenderApp) {
        render_app.add_systems(Render, skip_env_map_filtering.in_set(RenderSystems::CreateViews));
    }
}

fn skip_env_map_filtering(
    mut commands: Commands,
    refresh: Option<Res<EnvMapRefresh>>,
    maps: Query<Entity, With<RenderEnvironmentMap>>,
    mut frame: Local<u32>,
    mut seen: Local<EntityHashSet>,
) {
    let every = refresh.map_or(1, |r| r.0.max(1));
    let due = frame.is_multiple_of(every);
    *frame = frame.wrapping_add(1);
    for entity in &maps {
        if !seen.insert(entity) && !due {
            commands.entity(entity).remove::<RenderEnvironmentMap>();
        }
    }
}
