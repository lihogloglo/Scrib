//! Morrowind's directive stack rules: `OpenMW`'s `AiSequence` as an `sf_ai` [`StackPolicy`]
//! (`docs/specs/ai-adapter.md` "Stack policy", slice `MA2b`).
//!
//! Ported from `OpenMW` (GPL-3.0) at commit `5f3aafb010a26b789b6a5e590b4e218d1ddf0cd9`:
//! - `apps/openmw/mwmechanics/aisequence.cpp:386-477` (`AiSequence::stack`, `cancelOther` true,
//!   the default every script and mechanics caller uses): a non-combat package (`isActualAiPackage`,
//!   `:243-246`: wander, travel, escort, follow, activate) first stops combat (`stopCombat`,
//!   `:212-215`, every combat package goes); then, if the new package `shouldCancelPreviousAi`,
//!   every package that `canCancel` goes; then the new one is inserted before the first package
//!   whose `getPriority()` is lower or equal (newest first among equals; no source rank), else at
//!   the end.
//! - `aisequence.cpp:492-534` (`AiSequence::fill`): record packages are appended in record order
//!   (`push_back`, no `stack`), so a [`DirectiveSource::Record`] push goes to the bottom untouched.
//! - `aisequence.cpp:346-366` (`AiSequence::execute`): a finished package that is an actual
//!   package and `getRepeat()` is reset and appended to the end (`push_back(package->clone())`)
//!   before the original is erased; anything else just leaves. A package's duration running out is
//!   `execute` returning true too, so [`DirectiveEnd::Expired`] of a [`Limit::For`] repeats like
//!   [`DirectiveEnd::Completed`]; an erased (cancelled) package never comes back.
//! - Options (`aipackage.hpp:37-46` defaults; `aicombat.hpp:94-101`, `aiwander.hpp:96-101`,
//!   `aitravel.hpp:40-46`, `aiescort.hpp:37-43`, `aifollow.hpp:57-64`; `aiactivate.hpp` keeps the
//!   defaults): mapped from the [`DirectiveKind`] by [`options`], not stored on the directive.
//!   `OpenMW`'s priority is the package type's, so [`DirectiveSpec::priority`] is not read here.
//!
//! Not expressible on today's kinds (no `Pursue`, `Cast`, `Face`, `AvoidDoor`, `Breathe` or
//! `InternalTravel` kind, and a policy sees one actor's stack only): the return-travel `stack`
//! pushes under combat/pursue/cast when the top is a wander or the stack is empty
//! (`aisequence.cpp:408-430`; our wander directive stays below the fight and is resumed instead),
//! the cast-replaces-cast rule (`:449-455`), follow/escort ending allies' combat against the new
//! target's side (`:394-404`), follow resetting the packages below to their initial position
//! (`:463-467`), the executor scratch reset (`mAiState.reset`), the nearest-combat-target rotation
//! in `execute` (`:277-341`, the Combat drive's choice) and the combat/pursuit counters (derived
//! from the stack: count the [`DirectiveKind::Fight`] entries).

use sf_ai::{Directive, DirectiveEnd, DirectiveKind, DirectiveSource, Limit, StackEdit, StackPolicy};

/// `OpenMW`'s per-package options (`AiPackage::Options`, `aipackage.hpp:37-46`) for one kind.
/// `repeat` is per package instance, so it stays on [`sf_ai::DirectiveSpec::repeat`].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[allow(clippy::struct_excessive_bools, reason = "mirrors OpenMW's AiPackage::Options flags")]
pub struct Options {
    /// Insertion order: higher goes above (`getPriority`).
    pub priority: u8,
    /// Removed when a package that `should_cancel_previous_ai` is stacked (`canCancel`).
    pub can_cancel: bool,
    /// Stacking it removes every package that `can_cancel` (`shouldCancelPreviousAi`).
    pub should_cancel_previous_ai: bool,
    /// The executor scales walk speed to the distance (`useVariableSpeed`).
    pub use_variable_speed: bool,
    /// The actor sides with the package's target in fights (`sideWithTarget`).
    pub side_with_target: bool,
    /// The actor follows its target through load doors (`followTargetThroughDoors`).
    pub follow_target_through_doors: bool,
    /// Runs even when the actor is out of the AI processing range (`alwaysActive`).
    pub always_active: bool,
}

impl Options {
    /// `AiPackage::Options{}` (`aipackage.hpp:37-46`).
    pub const DEFAULT: Self = Self {
        priority: 0,
        can_cancel: true,
        should_cancel_previous_ai: true,
        use_variable_speed: false,
        side_with_target: false,
        follow_target_through_doors: false,
        always_active: false,
    };
}

impl Default for Options {
    fn default() -> Self {
        Self::DEFAULT
    }
}

/// The options of the `OpenMW` package a directive kind stands for. [`DirectiveKind::Custom`] has
/// no `OpenMW` package: it gets the base defaults and is not an actual package.
pub fn options(kind: &DirectiveKind) -> Options {
    let d = Options::DEFAULT;
    match kind {
        // AiWander (aiwander.hpp:96-101).
        DirectiveKind::StayNear { .. } => Options { use_variable_speed: true, ..d },
        // AiTravel (aitravel.hpp:40-46).
        DirectiveKind::GoTo(_) => Options { use_variable_speed: true, always_active: true, ..d },
        // AiEscort (aiescort.hpp:37-43).
        DirectiveKind::Escort { .. } => Options { use_variable_speed: true, side_with_target: true, ..d },
        // AiFollow (aifollow.hpp:57-64).
        DirectiveKind::Follow(_) => Options {
            use_variable_speed: true,
            side_with_target: true,
            follow_target_through_doors: true,
            ..d
        },
        // AiActivate (aiactivate.hpp: no makeDefaultOptions of its own).
        DirectiveKind::Use(_) | DirectiveKind::Custom(_) => d,
        // AiCombat (aicombat.hpp:94-101).
        DirectiveKind::Fight(_) => {
            Options { priority: 1, can_cancel: false, should_cancel_previous_ai: false, ..d }
        }
    }
}

/// `isActualAiPackage` (`aisequence.cpp:243-246`): wander, travel, escort, follow and activate,
/// the packages Morrowind itself knows (they stop combat, repeat, and `GetCurrentAIPackage`
/// reports them).
pub fn is_actual_package(kind: &DirectiveKind) -> bool {
    matches!(
        kind,
        DirectiveKind::StayNear { .. }
            | DirectiveKind::GoTo(_)
            | DirectiveKind::Escort { .. }
            | DirectiveKind::Follow(_)
            | DirectiveKind::Use(_)
    )
}

/// `OpenMW`'s `AiSequence` rules (module docs): priority only, cancellation by
/// `shouldCancelPreviousAi`/`canCancel`, record packages appended, repeats sent to the back.
#[derive(Clone, Copy, Debug, Default)]
pub struct MorrowindPolicy;

impl StackPolicy for MorrowindPolicy {
    fn push(&self, stack: &mut StackEdit<'_>, d: Directive) {
        // AiSequence::fill: record order, nothing cancelled.
        if d.spec.source == DirectiveSource::Record {
            stack.push_bottom(d);
            return;
        }
        let new = options(&d.spec.kind);
        // "Stop combat when a non-combat AI package is added."
        if is_actual_package(&d.spec.kind) {
            stack.cancel_where(|e| matches!(e.spec.kind, DirectiveKind::Fight(_)));
        }
        if new.should_cancel_previous_ai {
            stack.cancel_where(|e| options(&e.spec.kind).can_cancel);
        }
        let at = stack.iter().position(|e| options(&e.spec.kind).priority <= new.priority).unwrap_or(stack.len());
        stack.insert(at, d);
    }

    fn on_end(&self, stack: &mut StackEdit<'_>, ended: &Directive, end: DirectiveEnd) {
        // Not after an absolute deadline (ours, for schedules: the copy would expire at once),
        // nor a zero-length `For` (it would loop at one instant).
        let repeats = ended.spec.repeat
            && is_actual_package(&ended.spec.kind)
            && end != DirectiveEnd::Cancelled
            && !matches!(ended.spec.limit, Some(Limit::Until(_) | Limit::For { minutes: 0 }));
        if repeats {
            let again = stack.renew(ended);
            stack.push_bottom(again);
        }
    }
}
