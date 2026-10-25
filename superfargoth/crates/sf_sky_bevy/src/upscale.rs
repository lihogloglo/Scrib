//! Temporal resolve on the raster path ([`UpscalingSetting`] →
//! [`ActiveUpscaling`]): Bevy's TAA at native resolution, or DLSS Super
//! Resolution (cargo feature `dlss`, NVIDIA RTX), which falls back to TAA
//! where it isn't available. `Upscaling::Off` keeps MSAA (the Low preset).
//! On the ray-traced path DLSS Ray Reconstruction (`path.rs`) denoises and
//! upscales instead, at the same quality.
//!
//! Both need jittered rendering, a depth prepass and motion vectors
//! (`TemporalAntiAliasing` and `Dlss` require `TemporalJitter`, `MipBias`,
//! `DepthPrepass`, `MotionVectorPrepass`) and MSAA off. Turning one on
//! snapshots what the camera had; going back to `Off` (a settings change)
//! restores that. The motion vector prepass also gives every opaque prepass
//! draw a fragment stage, so Bevy 0.20's `VisibilityRange` dither works in
//! the prepass (a depth-only prepass has none: far-field.md "Bevy 0.20
//! pitfalls").

use bevy_anti_alias::taa::TemporalAntiAliasing;
use bevy_app::{App, Update};
use bevy_core_pipeline::prepass::{DepthPrepass, MotionVectorPrepass};
use bevy_ecs::prelude::*;
use bevy_render::camera::{MipBias, TemporalJitter};
use bevy_render::view::Msaa;

use crate::{ActiveRenderPath, RenderPathCamera, SkySystems};
use sf_sky::{RenderPath, Upscaling};

/// The upscaling the player asked for (a graphics setting; change it at
/// runtime to switch).
#[derive(Resource, Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct UpscalingSetting(pub Upscaling);

/// The temporal resolve actually running on the raster path:
/// [`UpscalingSetting`] where this build and GPU support it (DLSS falls back
/// to TAA). On the ray-traced path it reports what the denoiser uses.
#[derive(Resource, Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct ActiveUpscaling(pub Upscaling);

/// Set `reset` to drop the temporal history this frame (camera cuts,
/// teleports, floating-origin shifts whose motion vectors are wrong).
/// The plugin clears it after use.
#[derive(Resource, Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct TemporalHistory {
    pub reset: bool,
}

/// What this build and GPU support (filled once the renderer is up).
#[derive(Resource, Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct UpscalingSupport {
    /// DLSS Super Resolution.
    pub dlss_super_resolution: bool,
    /// DLSS Ray Reconstruction (the ray-traced path's denoiser).
    pub dlss_ray_reconstruction: bool,
}

pub(crate) fn build(app: &mut App) {
    app.init_resource::<UpscalingSetting>()
        .init_resource::<ActiveUpscaling>()
        .init_resource::<UpscalingSupport>()
        .init_resource::<TemporalHistory>()
        .add_systems(Update, apply.in_set(SkySystems::RenderPath).after(crate::path::PathSystems));
}

pub(crate) fn finish(app: &mut App) {
    #[cfg(feature = "dlss")]
    {
        use bevy_anti_alias::dlss::{DlssRayReconstructionSupported, DlssSuperResolutionSupported};
        let world = app.world();
        let support = UpscalingSupport {
            dlss_super_resolution: world.contains_resource::<DlssSuperResolutionSupported>(),
            dlss_ray_reconstruction: world.contains_resource::<DlssRayReconstructionSupported>(),
        };
        log::info!("upscaling support: {support:?}");
        app.insert_resource(support);
    }
    let support = *app.world().resource::<UpscalingSupport>();
    let requested = app.world().resource::<UpscalingSetting>().0;
    if requested.resolve(support.dlss_super_resolution) != requested {
        #[cfg(feature = "dlss")]
        let reason = if dlss_wgpu::plugin_available() {
            "on this GPU or driver"
        } else {
            "(DLSS plugin package missing: `dlss/` next to the executable or $SF_DLSS_PLUGIN_DIR)"
        };
        #[cfg(not(feature = "dlss"))]
        let reason = "in this build (no `dlss` feature)";
        log::warn!("DLSS unavailable {reason}; TAA instead");
    }
}

/// Camera state before a temporal resolve changed it.
#[derive(Component, Clone, Copy)]
#[expect(clippy::struct_excessive_bools, reason = "one flag per marker component")]
pub(crate) struct UpscaleSnapshot {
    msaa: Msaa,
    depth: bool,
    motion_vectors: bool,
    jitter: bool,
    mip_bias: bool,
}

#[cfg(feature = "dlss")]
type DlssState = dlss::State;
#[cfg(not(feature = "dlss"))]
type DlssState = ();

#[allow(clippy::type_complexity, clippy::needless_pass_by_value)]
fn apply(
    mut commands: Commands,
    setting: Res<UpscalingSetting>,
    support: Res<UpscalingSupport>,
    path: Res<ActiveRenderPath>,
    mut active: ResMut<ActiveUpscaling>,
    mut history: ResMut<TemporalHistory>,
    mut cameras: Query<
        (
            Entity,
            Option<&mut TemporalAntiAliasing>,
            DlssState,
            Option<&UpscaleSnapshot>,
            Option<&Msaa>,
            (Has<DepthPrepass>, Has<MotionVectorPrepass>),
            (Has<TemporalJitter>, Has<MipBias>),
        ),
        With<RenderPathCamera>,
    >,
) {
    let raster = path.0 == RenderPath::Raster;
    let resolved = setting.0.resolve(if raster { support.dlss_super_resolution } else { support.dlss_ray_reconstruction });
    if active.0 != resolved {
        log::info!("upscaling: {} -> {}", active.0.label(), resolved.label());
        active.0 = resolved;
    }
    let temporal = raster && resolved.is_temporal();
    let want_taa = raster && resolved == Upscaling::Taa;
    for (entity, taa, dlss_state, snapshot, msaa, (depth, motion_vectors), (jitter, mip_bias)) in &mut cameras {
        let mut e = commands.entity(entity);
        if temporal && snapshot.is_none() {
            e.insert(UpscaleSnapshot { msaa: msaa.copied().unwrap_or_default(), depth, motion_vectors, jitter, mip_bias });
        }
        match (want_taa, taa) {
            (true, Some(mut taa)) => {
                if history.reset {
                    taa.reset = true;
                }
            }
            (true, None) => {
                e.insert((TemporalAntiAliasing::default(), Msaa::Off));
            }
            (false, Some(_)) => {
                e.remove::<TemporalAntiAliasing>();
            }
            (false, None) => {}
        }
        #[cfg(feature = "dlss")]
        dlss::sync(&mut e, dlss_state, raster, resolved, setting.0, history.reset);
        #[cfg(not(feature = "dlss"))]
        let () = dlss_state;
        // Back to native MSAA raster: undo what the temporal resolve added.
        // On the ray-traced path Solari owns the camera; the snapshot waits
        // for the way back.
        if raster
            && !temporal
            && let Some(s) = snapshot
        {
            e.remove::<UpscaleSnapshot>().insert(s.msaa);
            if !s.depth {
                e.remove::<DepthPrepass>();
            }
            if !s.motion_vectors {
                e.remove::<MotionVectorPrepass>();
            }
            if !s.jitter {
                e.remove::<TemporalJitter>();
            }
            if !s.mip_bias {
                e.remove::<MipBias>();
            }
        }
    }
    history.reset = false;
}

/// DLSS Super Resolution on cameras; helpers shared with `path.rs`.
#[cfg(feature = "dlss")]
pub(crate) mod dlss {
    use bevy_anti_alias::dlss::{
        Dlss, DlssPerfQualityMode, DlssRayReconstructionFeature, DlssSuperResolutionFeature,
    };
    use bevy_ecs::prelude::*;
    use bevy_render::view::Msaa;

    use sf_sky::{UpscaleQuality, Upscaling};

    /// A camera's DLSS components (query data).
    pub(crate) type State = (
        Option<&'static mut Dlss<DlssSuperResolutionFeature>>,
        Option<&'static mut Dlss<DlssRayReconstructionFeature>>,
    );

    pub(crate) fn mode(quality: UpscaleQuality) -> DlssPerfQualityMode {
        match quality {
            UpscaleQuality::Auto => DlssPerfQualityMode::Auto,
            UpscaleQuality::Native => DlssPerfQualityMode::Dlaa,
            UpscaleQuality::Quality => DlssPerfQualityMode::Quality,
            UpscaleQuality::Balanced => DlssPerfQualityMode::Balanced,
            UpscaleQuality::Performance => DlssPerfQualityMode::Performance,
            UpscaleQuality::UltraPerformance => DlssPerfQualityMode::UltraPerformance,
        }
    }

    /// The Ray Reconstruction mode for a setting: its quality, or `Auto`
    /// when DLSS upscaling is off (the denoiser still runs).
    pub(crate) fn ray_reconstruction_mode(setting: Upscaling) -> DlssPerfQualityMode {
        match setting {
            Upscaling::Off | Upscaling::Taa => DlssPerfQualityMode::Auto,
            Upscaling::Dlss(q) => mode(q),
        }
    }

    /// Adds, updates or removes Super Resolution on one camera, and keeps
    /// Ray Reconstruction's mode in step with the setting.
    #[allow(clippy::type_complexity)]
    pub(super) fn sync(
        e: &mut EntityCommands,
        (sr, rr): (Option<Mut<Dlss<DlssSuperResolutionFeature>>>, Option<Mut<Dlss<DlssRayReconstructionFeature>>>),
        raster: bool,
        resolved: Upscaling,
        setting: Upscaling,
        reset: bool,
    ) {
        if let Some(mut rr) = rr {
            let m = ray_reconstruction_mode(setting);
            if rr.perf_quality_mode != m {
                rr.perf_quality_mode = m;
            }
            if reset {
                rr.reset = true;
            }
        }
        let wanted = match resolved {
            Upscaling::Dlss(q) if raster => Some(mode(q)),
            _ => None,
        };
        match (wanted, sr) {
            (Some(m), Some(mut sr)) => {
                if sr.perf_quality_mode != m {
                    sr.perf_quality_mode = m;
                }
                if reset {
                    sr.reset = true;
                }
            }
            (Some(m), None) => {
                e.insert((Dlss::<DlssSuperResolutionFeature> { perf_quality_mode: m, ..Dlss::default() }, Msaa::Off));
            }
            (None, Some(_)) => {
                e.remove::<Dlss<DlssSuperResolutionFeature>>();
            }
            (None, None) => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn app(setting: Upscaling) -> App {
        let mut app = App::new();
        app.insert_resource(UpscalingSetting(setting))
            .insert_resource(ActiveRenderPath(RenderPath::Raster))
            .configure_sets(Update, (SkySystems::RenderPath, crate::path::PathSystems));
        build(&mut app);
        app
    }

    #[cfg(not(feature = "dlss"))]
    #[test]
    fn without_dlss_upscaling_falls_back_to_taa() {
        let mut app = app(Upscaling::Dlss(sf_sky::UpscaleQuality::Quality));
        app.update();
        assert_eq!(app.world().resource::<ActiveUpscaling>().0, Upscaling::Taa);
    }

    /// TAA takes over the camera (jitter, prepasses, no MSAA); `Off` gives
    /// back exactly what it had.
    #[test]
    fn taa_snapshots_and_restores_the_camera() {
        let mut app = app(Upscaling::Taa);
        let cam = app.world_mut().spawn((RenderPathCamera, Msaa::Sample4)).id();
        app.update();
        let world = app.world();
        assert!(world.get::<TemporalAntiAliasing>(cam).is_some());
        assert_eq!(world.get::<Msaa>(cam), Some(&Msaa::Off));
        assert!(world.get::<MotionVectorPrepass>(cam).is_some() && world.get::<TemporalJitter>(cam).is_some());
        app.insert_resource(UpscalingSetting(Upscaling::Off));
        app.update();
        let world = app.world();
        assert!(world.get::<TemporalAntiAliasing>(cam).is_none());
        assert_eq!(world.get::<Msaa>(cam), Some(&Msaa::Sample4));
        assert!(world.get::<MotionVectorPrepass>(cam).is_none() && world.get::<TemporalJitter>(cam).is_none());
        assert!(world.get::<UpscaleSnapshot>(cam).is_none());
    }
}
