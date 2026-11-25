//! The light exposure system (slice `AI8d2`; `docs/specs/ai.md` "Light exposure (from game state,
//! never the GPU)"): each [`Perceivable`] target's light level from the host's [`AiLightInputs`],
//! sampled on the fixed tick with [`sf_ai::light_exposure`] and kept in its [`AiLightExposure`],
//! which `queue_sight` turns into the check's visibility ([`crate::sight::light_only_visibility`]).
//! Rendered light never enters (net rule 3), so the result is the same at 30 and 144 Hz.
//!
//! - **Off until the host writes it.** [`crate::AiSightPlugin`] does not insert [`AiLightInputs`]:
//!   without it no target is sampled, every [`AiLightExposure`] is removed and sight keeps the
//!   identity visibility. Its `Default` (sky 1, outdoors, no source) is "broad daylight".
//! - **Cadence.** A [`PerceptTarget::Player`] is sampled every tick; any other target on its
//!   staggered sight-period phase ([`sf_ai::due`] over `sight_period_s`, by its `ActorKey`), and at
//!   once when it has no sample yet. Observers check on that same period, so a check reads a
//!   sample at most one period old.
//! - **Sample point.** The target's chest (what sight rays aim at); outdoors (`interior` false)
//!   one roof ray straight up from the head, `light_search_m` long (a hit means a roof: the sky
//!   term × `roof_sky_mul`). Indoors no roof ray: the host's `sky` is then the interior's ambient.
//! - **Prefilter.** A linear distance cut: per sampled target, the sources on and within
//!   `light_search_m` of its chest are copied to a scratch list (the host is expected to list the
//!   loaded cells' lights, tens to a few hundred; a grid when a measurement asks for one).
//! - **Line of sight.** Not the [`sf_ai::RayQueue`]: its results arrive a tick or more later
//!   (deferred under the cap) and merge by (observer key, kind, subject), while `light_exposure`
//!   takes a synchronous callback and a player has no observer key. Instead, the same
//!   [`ProbeLos`] batch: pass 1 runs `light_exposure` with a recording callback (its choice of
//!   rays does not depend on their answers), one `probe` call casts every target's roof and
//!   source rays, pass 2 replays the answers in call order. The rays cast are charged to
//!   `max_rays_per_tick` ([`crate::AiRays::light_cast`]): sight gets the rest of the tick's budget.

use bevy_ecs::prelude::*;
use glam::Vec3;
use sf_ai::{ActorKey, LineOfSight, LightSource, LosRay, PerceptTarget, RayKind, RaySubject, due, light_exposure};
use sf_controller::GroundProbe;
use sf_controller_bevy::Mover;
use sf_coords::WorldPos;

use crate::perception::{AiPerceptionSettings, Perceivable, PerceivablePose};
use crate::sight::{AiPerceptClock, AiRays, LosScratch, ProbeLos};

/// What the host knows about light this tick (ai.md "Light exposure"): framework terms only. The
/// game writes it on the fixed tick before [`crate::AiPerceptionSystems::Sight`] (`MA16`: sky from
/// clock and weather, its lamps, torches and light spells as [`LightSource`]s).
#[derive(Resource, Clone, Debug, PartialEq)]
pub struct AiLightInputs {
    /// Sky level 0..1, already built from clock and weather; indoors, the interior's ambient.
    pub sky: f32,
    /// The active space is an interior: no roof ray, `sky` is the ambient as is.
    pub interior: bool,
    /// Gameplay lights of the loaded area (unsorted; the system's distance cut picks per target).
    pub sources: Vec<LightSource>,
}

impl Default for AiLightInputs {
    fn default() -> Self {
        // tunables: const (input default: full daylight, the identity until the host writes it)
        Self { sky: 1.0, interior: false, sources: Vec::new() }
    }
}

/// A target's light exposure, 0..1 (its last sample). Present only while [`AiLightInputs`] exists.
#[derive(Component, Clone, Copy, Debug, PartialEq)]
pub struct AiLightExposure(pub f32);

/// One sampled target of this tick: its rays are `rays[first..]` in call order, roof ray first.
#[derive(Clone, Copy, Debug)]
struct LightJob {
    entity: Entity,
    at: WorldPos,
    cands: (usize, usize),
    roof: Option<usize>,
    first_source_ray: usize,
}

/// Reused buffers (no per-tick allocation once warm).
#[derive(Resource, Default)]
pub(crate) struct LightScratch {
    jobs: Vec<LightJob>,
    cands: Vec<LightSource>,
    rays: Vec<LosRay>,
    clear: Vec<bool>,
    los: LosScratch,
}

/// Samples every due [`Perceivable`] (module docs) and writes its [`AiLightExposure`].
#[allow(clippy::needless_pass_by_value, clippy::type_complexity, clippy::too_many_arguments)]
pub(crate) fn sample_light<G: GroundProbe + Resource>(
    mut commands: Commands,
    inputs: Option<Res<AiLightInputs>>,
    settings: Res<AiPerceptionSettings>,
    clock: Res<AiPerceptClock>,
    probe: Option<Res<G>>,
    mut rays_res: ResMut<AiRays>,
    mut scratch: ResMut<LightScratch>,
    mut targets: Query<(Entity, &Perceivable, Option<&Mover>, Option<&PerceivablePose>, Option<&mut AiLightExposure>)>,
    lit: Query<Entity, With<AiLightExposure>>,
) {
    rays_res.light_cast = 0;
    let Some(inputs) = inputs else {
        for e in &lit {
            commands.entity(e).remove::<AiLightExposure>();
        }
        return;
    };
    let Some(probe) = probe else { return };
    let cfg = settings.light;
    let search = if cfg.light_search_m.is_finite() { cfg.light_search_m.max(0.0) } else { 0.0 };
    let period = settings.budget.sight_period_ticks(clock.tick_s);
    let LightScratch { jobs, cands, rays, clear, los } = &mut *scratch;
    jobs.clear();
    cands.clear();
    rays.clear();

    // Pass 1: choose each due target's candidates and record the rays `light_exposure` asks for.
    for (entity, p, mover, pose, exposure) in &targets {
        let due_now = match p.target {
            PerceptTarget::Player(_) => true,
            PerceptTarget::Actor(key) => exposure.is_none() || due(key, clock.tick, period),
        };
        if !due_now {
            continue;
        }
        let Some(feet) = mover.map(|m| m.loco.position).or(pose.map(|q| q.feet)) else { continue };
        let at = feet.offset(Vec3::Y * p.chest_m);
        let owner = match p.target {
            PerceptTarget::Actor(key) => key,
            PerceptTarget::Player(_) => ActorKey(0),
        };
        let subject = RaySubject::Target(p.target);
        let ray = |from: WorldPos, to: WorldPos| LosRay { owner, kind: RayKind::Light, subject, from, to };
        let start = cands.len();
        cands.extend(inputs.sources.iter().filter(|s| s.on && at.delta_f32(s.pos).length() <= search));
        let range = (start, cands.len());
        let roof = (!inputs.interior).then(|| {
            let head = feet.offset(Vec3::Y * p.head_m);
            rays.push(ray(head, head.offset(Vec3::Y * search)));
            rays.len() - 1
        });
        let first_source_ray = rays.len();
        let _ = light_exposure(
            at,
            inputs.sky,
            false,
            &cands[range.0..range.1],
            &mut |a, b| {
                rays.push(ray(a, b));
                false
            },
            &cfg,
        );
        jobs.push(LightJob { entity, at, cands: range, roof, first_source_ray });
    }

    // One batched probe call for every ray of the tick.
    clear.clear();
    clear.resize(rays.len(), false);
    ProbeLos { probe: &*probe, scratch: los }.cast(rays, clear);
    rays_res.light_cast = u32::try_from(rays.len()).unwrap_or(u32::MAX);

    // Pass 2: replay the answers in call order.
    for job in jobs.iter() {
        let roof_covered = job.roof.is_some_and(|i| !clear[i]);
        let mut next = job.first_source_ray;
        let light = light_exposure(
            job.at,
            inputs.sky,
            roof_covered,
            &cands[job.cands.0..job.cands.1],
            &mut |_, _| {
                let c = clear.get(next).copied().unwrap_or(false);
                next += 1;
                c
            },
            &cfg,
        );
        match targets.get_mut(job.entity) {
            Ok((.., Some(mut e))) => {
                e.set_if_neq(AiLightExposure(light));
            }
            _ => {
                commands.entity(job.entity).insert(AiLightExposure(light));
            }
        }
    }
}
