//! Record AI packages (`AI_W`, `AI_T`, `AI_E`, `AI_F`, `AI_A` with their `CNDT` cells) as `sf_ai`
//! directives (`docs/specs/ai-adapter.md` "Packages -> directives", slice `MA2a`).
//!
//! Ported from `OpenMW` (GPL-3.0) at commit `5f3aafb010a26b789b6a5e590b4e218d1ddf0cd9`, how an
//! actor's ESM package list becomes its `AiSequence`:
//! - `apps/openmw/mwmechanics/aisequence.cpp:492-534` (`AiSequence::fill`): one package per record
//!   entry, appended in record order (`push_back`, no `stack`), `repeat = mShouldRepeat != 0`.
//! - Options (`aipackage.hpp:37-46`, `typedaipackage.hpp:16-19`): every package `fill` builds uses
//!   `makeDefaultOptions().withRepeat(..)`, so priority 0 (`mPriority = 0`; only combat 1,
//!   avoid-door/face/breathe 2 and cast 3 raise it), `canCancel` and `shouldCancelPreviousAi` true.
//!   The executor-side options (`aiwander.hpp:96`, `aitravel.hpp:40`, `aiescort.hpp:37`,
//!   `aifollow.hpp:57`: variable speed, side-with-target, follow through doors, always active) are
//!   not directive fields; `MA2b` maps them.
//! - Durations are game hours on an `int16_t` (`components/esm3/aipackage.hpp:25-46`). Wander ends
//!   when `mDuration && mRemainingDuration <= 0` (`aiwander.cpp:160,378`, `max(0, duration)`);
//!   escort and follow only count down when `mDuration > 0` (`aiescort.cpp:82-90`,
//!   `aifollow.cpp:159-171`). So a positive duration is a [`Limit::For`], zero or negative none.
//!   `OpenMW` counts the remaining duration only while the package executes, as `Limit::For` does.
//! - Escort and follow destinations are in the coordinates of the cell named by `CNDT`
//!   (`aiescort.cpp:93`, `aifollow.cpp:174-189`), else of the actor's own cell; travel always the
//!   actor's own cell (`aitravel.cpp:85`). An escort with `FLT_MAX` coordinates has no destination
//!   (`aiescort.cpp:105-115`, "TESCS allows the creation of Escort packages without a specific
//!   destination"): it ends as soon as the target is in range, so it is skipped and reported here.
//!   A follow without one follows until its duration runs out.
//!
//! Ours: ids resolve through the adapter's [`PackageResolver`]; a target nobody resolves is
//! skipped and reported ([`Skipped`]), not kept as a dead package. The wander idle table and
//! follow's end point ride beside the spec ([`PackageExtra`]); the stack policy is `MA2b`.

use mw_world::convert::world_pos;
use mw_world::ids::interior;
use sf_ai::{ActorKey, DirectiveKind, DirectiveSource, DirectiveSpec, Limit};
use sf_records::RecordId;
use sf_world::{Location, PlacementId, WorldspaceId};
use tes3::esp::AiPackage;

use crate::wander::Wander;

/// Priority of every record package (`OpenMW` `AiPackage::Options::mPriority` default,
/// `aipackage.hpp:39`; `AiSequence::fill` never sets it).
pub const RECORD_PACKAGE_PRIORITY: u8 = 0;

/// The coordinate value the editor writes for "no destination" (`aiescort.cpp:105`).
pub const NOWHERE: f32 = f32::MAX;

/// Game minutes per game hour.
const MINUTES_PER_HOUR: u32 = 60;

/// Resolves record ids to the world's keys (the adapter's reference index).
pub trait PackageResolver {
    /// An actor by record id (`player` included: the player's character is an actor).
    fn actor(&self, id: &str) -> Option<ActorKey>;
    /// A placed object by record id (`AI_A`'s target, `searchPtr` in `aiactivate.cpp:27`).
    fn placement(&self, id: &str) -> Option<PlacementId>;
}

/// What the mapping needs to know about the actor whose packages these are.
pub struct PackageCtx<'a> {
    /// The actor's record id: every directive's origin.
    pub actor: RecordId,
    /// Where the actor spawned: wander's home, and the worldspace of cell-less coordinates.
    pub home: Location,
    pub resolver: &'a dyn PackageResolver,
}

/// Package data a [`DirectiveKind`] has no room for.
#[derive(Clone, Debug, PartialEq)]
pub enum PackageExtra {
    None,
    /// `AI_W`'s idle table and authored hour (the radius is in the `StayNear`).
    Wander(Wander),
    /// `AI_F`'s end point: following is done once near it (`aifollow.cpp:174`).
    FollowUntil(Location),
}

/// One record package as a directive.
#[derive(Clone, Debug, PartialEq)]
pub struct RecordDirective {
    /// Index of the package in the record's list.
    pub index: usize,
    pub spec: DirectiveSpec,
    pub extra: PackageExtra,
}

/// Why a package was left out.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SkipReason {
    /// `AI_E` / `AI_F` target id no actor resolves.
    UnknownActor(String),
    /// `AI_A` target id no placement resolves.
    UnknownObject(String),
    /// `AI_E` with `NOWHERE` coordinates.
    NoDestination,
}

/// A package left out, for the caller's log.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Skipped {
    pub index: usize,
    /// `AI_W`, `AI_T`, `AI_E`, `AI_F` or `AI_A`.
    pub tag: &'static str,
    pub reason: SkipReason,
}

/// A record's package list mapped: directives in record order, plus what was skipped.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct PackageDirectives {
    pub directives: Vec<RecordDirective>,
    pub skipped: Vec<Skipped>,
}

impl PackageDirectives {
    /// Just the specs, in record order.
    pub fn specs(&self) -> impl Iterator<Item = &DirectiveSpec> {
        self.directives.iter().map(|d| &d.spec)
    }
}

/// The record tag of a package.
pub fn tag(package: &AiPackage) -> &'static str {
    match package {
        AiPackage::Wander(_) => "AI_W",
        AiPackage::Travel(_) => "AI_T",
        AiPackage::Escort(_) => "AI_E",
        AiPackage::Follow(_) => "AI_F",
        AiPackage::Activate(_) => "AI_A",
    }
}

/// Maps `packages` in record order (`AiSequence::fill`).
pub fn directives(packages: &[AiPackage], ctx: &PackageCtx<'_>) -> PackageDirectives {
    let mut out = PackageDirectives::default();
    for (index, package) in packages.iter().enumerate() {
        match directive(package, ctx) {
            Ok((spec, extra)) => out.directives.push(RecordDirective { index, spec, extra }),
            Err(reason) => out.skipped.push(Skipped { index, tag: tag(package), reason }),
        }
    }
    out
}

/// One package: its spec and extra data, or why it can't be served.
pub fn directive(
    package: &AiPackage,
    ctx: &PackageCtx<'_>,
) -> Result<(DirectiveSpec, PackageExtra), SkipReason> {
    let (kind, repeat, hours, extra) = match package {
        AiPackage::Wander(p) => {
            let wander = Wander::from_package(p);
            let kind = DirectiveKind::StayNear { home: ctx.home.clone(), radius: wander.distance };
            (kind, p.reset != 0, esm_hours(p.duration), PackageExtra::Wander(wander))
        }
        AiPackage::Travel(p) => {
            let to = at(ctx.home.worldspace.clone(), p.location);
            (DirectiveKind::GoTo(to), p.reset != 0, None, PackageExtra::None)
        }
        AiPackage::Escort(p) => {
            let who = resolve_actor(ctx, &p.target)?;
            if is_nowhere(p.location) {
                return Err(SkipReason::NoDestination);
            }
            let to = at(destination_worldspace(&p.cell, ctx), p.location);
            (DirectiveKind::Escort { who, to }, p.reset != 0, esm_hours(p.duration), PackageExtra::None)
        }
        AiPackage::Follow(p) => {
            let who = resolve_actor(ctx, &p.target)?;
            let extra = if is_nowhere(p.location) {
                PackageExtra::None
            } else {
                PackageExtra::FollowUntil(at(destination_worldspace(&p.cell, ctx), p.location))
            };
            (DirectiveKind::Follow(who), p.reset != 0, esm_hours(p.duration), extra)
        }
        AiPackage::Activate(p) => {
            let id = p.target.as_str();
            let target = ctx
                .resolver
                .placement(id)
                .ok_or_else(|| SkipReason::UnknownObject(id.to_owned()))?;
            (DirectiveKind::Use(target), p.reset != 0, None, PackageExtra::None)
        }
    };
    let mut spec = DirectiveSpec::new(kind, DirectiveSource::Record)
        .priority(RECORD_PACKAGE_PRIORITY)
        .origin(ctx.actor.clone());
    if repeat {
        spec = spec.repeating();
    }
    if let Some(hours) = hours {
        spec = spec.limit(Limit::For { minutes: u32::from(hours) * MINUTES_PER_HOUR });
    }
    Ok((spec, extra))
}

/// A record duration (game hours, `int16_t` in the format) as a limit: positive only.
#[allow(clippy::cast_possible_wrap)]
fn esm_hours(raw: u16) -> Option<u16> {
    let hours = raw as i16;
    u16::try_from(hours).ok().filter(|&h| h > 0)
}

fn is_nowhere(p: [f32; 3]) -> bool {
    p.contains(&NOWHERE)
}

fn at(worldspace: WorldspaceId, p: [f32; 3]) -> Location {
    Location::new(worldspace, world_pos(p))
}

/// `CNDT`'s cell when set (an interior), else the actor's own worldspace.
fn destination_worldspace(cell: &str, ctx: &PackageCtx<'_>) -> WorldspaceId {
    if cell.is_empty() { ctx.home.worldspace.clone() } else { interior(cell) }
}

fn resolve_actor(ctx: &PackageCtx<'_>, id: &str) -> Result<ActorKey, SkipReason> {
    ctx.resolver.actor(id).ok_or_else(|| SkipReason::UnknownActor(id.to_owned()))
}
