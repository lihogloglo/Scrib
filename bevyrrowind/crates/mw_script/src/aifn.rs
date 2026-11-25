//! The AI package functions as typed calls (ai-adapter.md "Packages -> directives", slice
//! `MA2c1`): the interpreter decodes `AIWander`, `AITravel`, `AIEscort[Cell]`, `AIFollow[Cell]`,
//! `GetAIPackageDone` and `GetCurrentAIPackage` by name once per table and hands each call to
//! [`crate::Host::ai`] as an [`AiCall`]. What a package does lives in the host (`mw_rpg` records
//! the push, the game maps it onto `sf_ai` directives); this module only reads the arguments.
//!
//! Argument semantics ported from `OpenMW` (GPL-3.0) at commit `5f3aafb`,
//! `apps/openmw/mwscript/aiextensions.cpp` and the signatures in
//! `components/compiler/extensions0.cpp:35-40`:
//! - `AITravel x y z [reset]` (`OpAiTravel`, :64-94), `AIEscort actor duration x y z [reset]`
//!   (`OpAiEscort`, :96-132), `AIEscortCell actor cell duration x y z [reset]` (`OpAiEscortCell`,
//!   :134-179), `AIFollow` / `AIFollowCell` with the same shapes (`OpAiFollow` :325-361,
//!   `OpAiFollowCell` :363-401). Coordinates are floats; "the value of the reset argument doesn't
//!   actually matter": the package repeats as soon as any optional argument is present. Escort's
//!   duration is truncated to whole hours (`static_cast<int>`), follow's stays a float.
//! - `AIWander range duration time [idle idle2 .. idle9] [reset]` (`OpAiWander`, :197-252): range,
//!   duration and time truncated to integers; the first optional argument (the chance for `Idle`)
//!   is dropped; the next eight are the `Idle2`-`Idle9` chances, each clamped to 0-255, and any of
//!   them makes the package repeat; the one after them sets repeat (`!= 0`); the rest are ignored.
//! - `GetAIPackageDone` (`OpGetAiPackageDone`, :181-195): 1 when the current package finished.
//!   `GetCurrentAIPackage` (`OpGetCurrentAIPackage`, :403-422): the type id of the package that
//!   ran last ([`package_type`]), -1 for none (also for a non-actor or a finished death).

use crate::ast::{Arg, Ident};

/// `OpenMW`'s `AiPackageTypeId` (`apps/openmw/mwmechanics/aipackagetypeid.hpp`), as
/// `GetCurrentAIPackage` returns it. Combat, pursue, face and the other internal packages are
/// never returned ("for compatibility").
pub mod package_type {
    pub const NONE: i32 = -1;
    pub const WANDER: i32 = 0;
    pub const TRAVEL: i32 = 1;
    pub const ESCORT: i32 = 2;
    pub const FOLLOW: i32 = 3;
    pub const ACTIVATE: i32 = 4;
}

/// Wander's `Idle2`-`Idle9` chances (`OpAiWander`'s loop `for (int i = 2; i <= 9 ...)`).
pub const WANDER_IDLES: usize = 8;

/// One AI package function, as the table names it.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum AiFn {
    Wander,
    Travel,
    Escort,
    EscortCell,
    Follow,
    FollowCell,
    PackageDone,
    CurrentPackage,
}

impl AiFn {
    pub const ALL: [AiFn; 8] = [
        AiFn::Wander,
        AiFn::Travel,
        AiFn::Escort,
        AiFn::EscortCell,
        AiFn::Follow,
        AiFn::FollowCell,
        AiFn::PackageDone,
        AiFn::CurrentPackage,
    ];

    /// The function table's canonical name.
    pub fn name(self) -> &'static str {
        match self {
            AiFn::Wander => "AIWander",
            AiFn::Travel => "AITravel",
            AiFn::Escort => "AIEscort",
            AiFn::EscortCell => "AIEscortCell",
            AiFn::Follow => "AIFollow",
            AiFn::FollowCell => "AIFollowCell",
            AiFn::PackageDone => "GetAIPackageDone",
            AiFn::CurrentPackage => "GetCurrentAIPackage",
        }
    }

    /// The function of a table name (case-insensitive).
    pub fn from_name(name: &str) -> Option<AiFn> {
        AiFn::ALL.into_iter().find(|f| f.name().eq_ignore_ascii_case(name))
    }
}

/// A point in the target cell's (or the actor's own cell's) Morrowind units, as written.
pub type Point = [f32; 3];

/// One AI package call with its arguments read as `OpenMW` reads them.
#[derive(Clone, Debug, PartialEq)]
pub enum AiCall {
    /// `AIWander`: `range` units around where it starts, for `hours` game hours (0: forever),
    /// `hour` the authored time of day; `idles` the `Idle2`-`Idle9` chances given (at most
    /// [`WANDER_IDLES`], in order).
    Wander { range: i32, hours: i32, hour: i32, idles: Vec<u8>, repeat: bool },
    /// `AITravel` to `to` in the actor's own cell.
    Travel { to: Point, repeat: bool },
    /// `AIEscort` / `AIEscortCell`: lead `actor` to `to` (in `cell` when given) for `hours`
    /// whole game hours (0 or less: no limit).
    Escort { actor: Ident, cell: Option<Ident>, hours: i32, to: Point, repeat: bool },
    /// `AIFollow` / `AIFollowCell`: follow `actor` for `hours` game hours (0 or less: no limit),
    /// done near `to` (in `cell` when given).
    Follow { actor: Ident, cell: Option<Ident>, hours: f32, to: Point, repeat: bool },
    /// `GetAIPackageDone`.
    PackageDone,
    /// `GetCurrentAIPackage`.
    CurrentPackage,
}

impl AiCall {
    /// Whether the call reads a value (the getters) rather than pushing a package.
    pub fn is_read(&self) -> bool {
        matches!(self, AiCall::PackageDone | AiCall::CurrentPackage)
    }

    /// The function that made the call (its table name).
    pub fn name(&self) -> &'static str {
        match self {
            AiCall::Wander { .. } => AiFn::Wander.name(),
            AiCall::Travel { .. } => AiFn::Travel.name(),
            AiCall::Escort { cell: None, .. } => AiFn::Escort.name(),
            AiCall::Escort { cell: Some(_), .. } => AiFn::EscortCell.name(),
            AiCall::Follow { cell: None, .. } => AiFn::Follow.name(),
            AiCall::Follow { cell: Some(_), .. } => AiFn::FollowCell.name(),
            AiCall::PackageDone => AiFn::PackageDone.name(),
            AiCall::CurrentPackage => AiFn::CurrentPackage.name(),
        }
    }
}

/// Builds an [`AiCall`] from evaluated arguments: `num(i)` is argument `i` as a number (0 when
/// absent), `id(i)` argument `i` as an id, `args` the call's arguments as parsed (optional ones
/// absent, never `Missing`).
pub(crate) fn decode(
    function: AiFn,
    args: &[Arg],
    num: &mut dyn FnMut(usize) -> f64,
    id: &dyn Fn(usize) -> Ident,
) -> AiCall {
    // `Type_Float` in `OpenMW`'s runtime: the script's f64 narrowed.
    #[allow(clippy::cast_possible_truncation)]
    let mut float = |i: usize| num(i) as f32;
    // Optional arguments present after the `fixed` required ones.
    let extra = |fixed: usize| args.len().saturating_sub(fixed);
    match function {
        AiFn::Travel => {
            AiCall::Travel { to: [float(0), float(1), float(2)], repeat: extra(3) > 0 }
        }
        AiFn::Escort | AiFn::EscortCell | AiFn::Follow | AiFn::FollowCell => {
            let cell = matches!(function, AiFn::EscortCell | AiFn::FollowCell);
            let first = 1 + usize::from(cell);
            let hours = float(first);
            let to = [float(first + 1), float(first + 2), float(first + 3)];
            let repeat = extra(first + 4) > 0;
            let actor = id(0);
            let cell = cell.then(|| id(1));
            if matches!(function, AiFn::Escort | AiFn::EscortCell) {
                AiCall::Escort { actor, cell, hours: crate::interp::to_int(f64::from(hours)), to, repeat }
            } else {
                AiCall::Follow { actor, cell, hours, to, repeat }
            }
        }
        AiFn::Wander => {
            let range = crate::interp::to_int(num(0));
            let hours = crate::interp::to_int(num(1));
            let hour = crate::interp::to_int(num(2));
            // Argument 3 is the unused `Idle` chance; 4..=11 are `Idle2`-`Idle9`; 12 is reset.
            let idles: Vec<u8> = (4..args.len().min(4 + WANDER_IDLES))
                .map(|i| {
                    let v = crate::interp::to_int(num(i)).clamp(0, i32::from(u8::MAX));
                    u8::try_from(v).unwrap_or(u8::MAX)
                })
                .collect();
            let reset = 4 + WANDER_IDLES;
            let repeat = if args.len() > reset {
                crate::interp::to_int(num(reset)) != 0
            } else {
                !idles.is_empty()
            };
            AiCall::Wander { range, hours, hour, idles, repeat }
        }
        AiFn::PackageDone => AiCall::PackageDone,
        AiFn::CurrentPackage => AiCall::CurrentPackage,
    }
}
