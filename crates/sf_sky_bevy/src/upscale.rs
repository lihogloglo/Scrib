//! Temporal upscaling on the raster path ([`UpscalingSetting`] →
//! [`ActiveUpscaling`]): Bevy's DLSS Super Resolution (cargo feature
//! `dlss`, NVIDIA RTX). On the ray-traced path DLSS Ray Reconstruction
//! (`path.rs`) denoises and upscales instead, at the same quality.
//!
//! DLSS needs jittered rendering, a depth prepass and motion vectors
//! (`Dlss` requires `TemporalJitter`, `MipBias`, `DepthPrepass`,
//! `MotionVectorPrepass`) and MSAA off. Turning it on snapshots what the
//! camera had; turning it off (a settings change) restores that. The motion
//! vector prepass also gives every opaque prepass draw a fragment stage, so
//! Bevy 0.20's `VisibilityRange` dither works in the prepass (a depth-only
//! prepass has none: far-field.md "Bevy 0.20 pitfalls").

use bevy_app::{App, Update};
use bevy_ecs::prelude::*;

use crate::SkySystems;
use sf_sky::Upscaling;

/// The upscaling the player asked for (a graphics setting; change it at
/// runtime to switch).
#[derive(Resource, Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct UpscalingSetting(pub Upscaling);

/// The upscaling actually running on the raster path: [`UpscalingSetting`]
/// where this build and GPU support it, else `Off`. On the ray-traced path
/// it reports what the denoiser uses.
#[derive(Resource, Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct ActiveUpscaling(pub Upscaling);

/// Set `reset` to drop the temporal upscaler's history this frame (camera
/// cuts, teleports, floating-origin shifts whose motion vectors are wrong).
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
        .init_resource::<TemporalHistory>();
    #[cfg(feature = "dlss")]
    app.add_systems(
        Update,
        dlss::apply
            .in_set(SkySystems::RenderPath)
            .after(crate::path::PathSystems),
    );
    #[cfg(not(feature = "dlss"))]
    app.add_systems(Update, report_off.in_set(SkySystems::RenderPath));
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
        log::warn!("DLSS unavailable on this GPU or build; upscaling off");
    }
}

#[cfg(not(feature = "dlss"))]
fn report_off(mut active: ResMut<ActiveUpscaling>, mut history: ResMut<TemporalHistory>) {
    if history.reset {
        history.reset = false;
    }
    if active.0 != Upscaling::Off {
        active.0 = Upscaling::Off;
    }
}

/// DLSS Super Resolution on cameras; helpers shared with `path.rs`.
#[cfg(feature = "dlss")]
pub(crate) mod dlss {
    use bevy_anti_alias::dlss::{
        Dlss, DlssPerfQualityMode, DlssRayReconstructionFeature, DlssSuperResolutionFeature,
    };
    use bevy_core_pipeline::prepass::{DepthPrepass, MotionVectorPrepass};
    use bevy_ecs::prelude::*;
    use bevy_render::camera::{MipBias, TemporalJitter};
    use bevy_render::view::Msaa;

    use super::{ActiveUpscaling, TemporalHistory, UpscalingSetting, UpscalingSupport};
    use crate::{ActiveRenderPath, RenderPathCamera};
    use sf_sky::{RenderPath, UpscaleQuality, Upscaling};

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
    /// when upscaling is off (the denoiser still runs).
    pub(crate) fn ray_reconstruction_mode(setting: Upscaling) -> DlssPerfQualityMode {
        match setting {
            Upscaling::Off => DlssPerfQualityMode::Auto,
            Upscaling::Dlss(q) => mode(q),
        }
    }

    /// Camera state before Super Resolution changed it.
    #[derive(Component, Clone, Copy)]
    #[expect(clippy::struct_excessive_bools, reason = "one flag per marker component")]
    pub(crate) struct UpscaleSnapshot {
        msaa: Msaa,
        depth: bool,
        motion_vectors: bool,
        jitter: bool,
        mip_bias: bool,
    }

    #[allow(clippy::type_complexity)]
    pub(super) fn apply(
        mut commands: Commands,
        setting: Res<UpscalingSetting>,
        support: Res<UpscalingSupport>,
        path: Res<ActiveRenderPath>,
        mut active: ResMut<ActiveUpscaling>,
        mut history: ResMut<TemporalHistory>,
        mut cameras: Query<
            (
                Entity,
                Option<&mut Dlss<DlssSuperResolutionFeature>>,
                Option<&mut Dlss<DlssRayReconstructionFeature>>,
                Option<&UpscaleSnapshot>,
                Option<&Msaa>,
                (Has<DepthPrepass>, Has<MotionVectorPrepass>),
                (Has<TemporalJitter>, Has<MipBias>),
            ),
            With<RenderPathCamera>,
        >,
    ) {
        let raster = path.0 == RenderPath::Raster;
        let resolved = setting.0.resolve(if raster {
            support.dlss_super_resolution
        } else {
            support.dlss_ray_reconstruction
        });
        if active.0 != resolved {
            log::info!("upscaling: {} -> {}", active.0.label(), resolved.label());
            active.0 = resolved;
        }
        let wanted = match resolved {
            Upscaling::Dlss(q) if raster => Some(mode(q)),
            _ => None,
        };
        for (entity, sr, rr, snapshot, msaa, (depth, motion), (jitter, mip_bias)) in &mut cameras {
            // Ray Reconstruction follows the quality setting at runtime.
            if let Some(mut rr) = rr {
                let m = ray_reconstruction_mode(setting.0);
                if rr.perf_quality_mode != m {
                    rr.perf_quality_mode = m;
                }
                if history.reset {
                    rr.reset = true;
                }
            }
            let mut e = commands.entity(entity);
            match (wanted, sr) {
                (Some(m), Some(mut sr)) => {
                    if sr.perf_quality_mode != m {
                        sr.perf_quality_mode = m;
                    }
                    if history.reset {
                        sr.reset = true;
                    }
                }
                (Some(m), None) => {
                    if snapshot.is_none() {
                        e.insert(UpscaleSnapshot {
                            msaa: msaa.copied().unwrap_or_default(),
                            depth,
                            motion_vectors: motion,
                            jitter,
                            mip_bias,
                        });
                    }
                    e.insert((
                        Dlss::<DlssSuperResolutionFeature> {
                            perf_quality_mode: m,
                            ..Dlss::default()
                        },
                        Msaa::Off,
                    ));
                }
                (None, sr) => {
                    if sr.is_some() {
                        e.remove::<Dlss<DlssSuperResolutionFeature>>();
                    }
                    // Back to native raster: undo what Super Resolution
                    // added. On the ray-traced path Solari owns the camera;
                    // the snapshot waits for the way back.
                    if raster && let Some(s) = snapshot {
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
            }
        }
        history.reset = false;
    }
}

#[cfg(all(test, not(feature = "dlss")))]
mod tests {
    use super::*;
    use crate::ActiveRenderPath;
    use sf_sky::RenderPath;

    #[test]
    fn without_dlss_upscaling_reports_off() {
        let mut app = App::new();
        app.insert_resource(UpscalingSetting(Upscaling::Dlss(sf_sky::UpscaleQuality::Quality)))
            .insert_resource(ActiveRenderPath(RenderPath::Raster))
            .configure_sets(Update, SkySystems::RenderPath);
        build(&mut app);
        app.update();
        assert_eq!(app.world().resource::<ActiveUpscaling>().0, Upscaling::Off);
    }
}
