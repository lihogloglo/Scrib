//! A character's gameplay action state: saved, replicated, plain serde (action.md "Character
//! action state"). With `sf_controller::Locomotion` it is the whole gameplay body.

use serde::{Deserialize, Serialize};
use sf_controller::{ArrivalParams, Gait, GaitSpeeds, SpringParams};

use crate::command::ClassSet;
use crate::ids::{ClockTime, MovesetKey, RuleKey, TargetId, Tick};
use crate::rule::Mode;

/// Clock rate 1, in thousandths.
pub const RATE_ONE: u16 = 1000;

/// Time since the current move started, scaled by hit-stop.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ActionClock {
    pub elapsed: ClockTime,
    /// Thousandths: 1000 plays at speed, 0 freezes (hit-stop).
    pub rate: u16,
}

impl Default for ActionClock {
    fn default() -> Self {
        Self { elapsed: ClockTime::ZERO, rate: RATE_ONE }
    }
}

impl ActionClock {
    pub fn advance(&mut self) {
        self.elapsed.0 = self.elapsed.0.saturating_add(u32::from(self.rate));
    }
}

/// Movement mode and stance.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct ModeState {
    pub mode: Mode,
    /// Weapon drawn (set by the shell from equipment; combat rules arrive with E1).
    pub combat: bool,
    /// The equipped moveset (combat sheet): rules with `entry.moveset` need it. Set by the shell
    /// from equipment.
    pub moveset: Option<MovesetKey>,
}

/// Latched requests (two-stage, Sekiro model): no timer; starting any move clears them.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct InputBuffer {
    pub latched: ClassSet,
}

/// Tick timers (R1 adopt 3), stored as ticks so the windows are exact.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Timers {
    /// First airborne tick of a fall that didn't start with a launch: coyote time runs from it.
    pub left_ground: Option<Tick>,
    /// A Jump pressed in the air (jump buffer): fires on a landing within the buffer.
    pub jump_pressed: Option<Tick>,
    /// Rules on a redo cooldown, with the first tick they may start again.
    pub redo: Vec<(RuleKey, Tick)>,
}

/// What the layer remembers about the body between ticks.
#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct BodyMemory {
    pub grounded: bool,
    /// The last airborne vertical speed: the impact speed on the landing tick.
    pub air_vspeed: f32,
    /// This airtime began with a launch (no coyote jump).
    pub launched: bool,
}

/// One character's action state. Contains no Bevy types and no singleton: every character,
/// local, remote or AI, owns one.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ActionState {
    /// The current move rule.
    pub mov: RuleKey,
    /// The tick it started.
    pub started: Tick,
    pub clock: ActionClock,
    pub mode: ModeState,
    pub buffer: InputBuffer,
    pub timers: Timers,
    pub body: BodyMemory,
    /// The Warp owner's plan while a Warp move plays (B2a); `None` otherwise. Not serialized yet:
    /// the save section's postcard shape is versioned (`sf_save_bevy` `CHARACTER_VERSION`), and a
    /// save mid-mantle (under a second) loads with no plan, so the shell treats the move as Sim
    /// until it ends. Serialize it with a version bump when co-op (F) replicates moves.
    #[serde(skip)]
    pub warp: Option<WarpState>,
    /// A reaction the combat layer (or physics) forces: the next step starts it before anything
    /// else, whatever the windows say, then clears it.
    pub forced: Option<Forced>,
    /// The lock-on target (stable id); the shell senses its position ([`crate::Sense::lock_target`]).
    pub lock: Option<TargetId>,
    /// The interaction slot this character is entering, seated in or leaving (`IN2a`): set when a
    /// [`crate::WarpTarget::Slot`] move starts, cleared when a move outside the seat chain starts.
    /// Not serialized yet, as [`ActionState::warp`]: a save while seated loads with none, and the
    /// seated move then stands up as baked with no exit event (the shell re-claims nothing).
    #[serde(skip)]
    pub seat: Option<SeatState>,
    /// The slot [`crate::Sense::seat`] sensed last tick (target, slot index; `IN2b1`): a sense that
    /// differs is a new offer ([`crate::Entry::offered`]), answered once. Not serialized: a load
    /// sees the held claim as a new offer and answers it again.
    #[serde(skip)]
    pub seat_sensed: Option<(TargetId, u8)>,
    /// The slot an [`crate::Owner::Approach`] move walks to (`IN2c4`), set when it starts, cleared
    /// when any other move starts. Not serialized, as [`ActionState::seat`]: a save mid-approach
    /// loads with none and the approach ends at once as lost (the shell re-offers the claim).
    #[serde(skip)]
    pub approach: Option<crate::approach::ApproachState>,
}

/// The slot a character sits in ([`ActionState::seat`]).
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct SeatState {
    pub target: TargetId,
    /// The slot index on the target (`sf_interaction::SlotRef::slot`).
    pub slot: u8,
    /// The pose the user entered from, relative to the seated feet (world axes, world yaw): the
    /// exit lands here when the slot sense is gone ([`crate::WarpTarget::SlotExit`]).
    pub back: sf_anim::RootPose,
    /// The seated yaw (the enter pose's), radians: the [`crate::Owner::Seated`] hold faces it.
    pub yaw: f32,
    /// The enter warp landed ([`crate::EventKind::SeatEntered`] sent): leaving sends
    /// [`crate::EventKind::SeatExited`].
    pub entered: bool,
}

/// A forced move request ([`ActionState::forced`]).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Forced {
    pub rule: RuleKey,
    /// The side it came from (content-defined index, e.g. front, back, left, right).
    pub side: u8,
}

/// A Warp move's plan, fixed at start. Positions are relative to the feet on the start tick (the
/// shell keeps that world anchor), axes are world axes.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct WarpState {
    pub plan: sf_anim::WarpPlan,
    /// Hands on the lip (B2b): `None` when the slot clip has no hand contacts.
    pub grip: Option<crate::LedgeGrip>,
}

impl ActionState {
    /// Starting in move `start` (usually the locomotion rule), grounded.
    pub fn new(start: RuleKey) -> Self {
        Self {
            mov: start,
            started: Tick::default(),
            clock: ActionClock::default(),
            mode: ModeState::default(),
            buffer: InputBuffer::default(),
            timers: Timers::default(),
            body: BodyMemory { grounded: true, ..BodyMemory::default() },
            warp: None,
            forced: None,
            lock: None,
            seat: None,
            seat_sensed: None,
            approach: None,
        }
    }
}

/// Per-mode multipliers (action.md "Mode profiles"); defaults are neutral, packs tune them.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ModeProfile {
    pub speed_scale: f32,
    pub jump_scale: f32,
}

impl Default for ModeProfile {
    fn default() -> Self {
        Self { speed_scale: 1.0, jump_scale: 1.0 }
    }
}

/// Top speeds per gait (adapter data).
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct GaitTable {
    pub walk: GaitSpeeds,
    pub run: GaitSpeeds,
    pub sprint: GaitSpeeds,
    pub sneak: GaitSpeeds,
}

impl GaitTable {
    pub fn get(&self, gait: Gait) -> &GaitSpeeds {
        match gait {
            Gait::Walk => &self.walk,
            Gait::Run => &self.run,
            Gait::Sprint => &self.sprint,
            Gait::Sneak => &self.sneak,
        }
    }
}

impl Default for GaitTable {
    /// Generic human-scale placeholders; content packs supply their own.
    fn default() -> Self {
        let g = |forward, side, back| GaitSpeeds { forward, side, back };
        Self { walk: g(1.6, 1.4, 1.2), run: g(4.5, 3.8, 3.0), sprint: g(7.0, 5.0, 4.0), sneak: g(1.2, 1.0, 0.9) }
    }
}

/// A character's action tuning. Stat bands are applied before the step (jump height,
/// speeds); windows, buffers and latency are never scaled (action.md "Stat bands").
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct CharacterParams {
    pub gravity: f32,
    /// Normal-mode jump height, m.
    pub jump_height: f32,
    /// A Jump on any of the first this-many airborne ticks after walking off an edge is a
    /// ground jump.
    pub coyote_ticks: u8,
    /// A Jump pressed in the air up to this many ticks before the landing tick fires on it.
    pub jump_buffer_ticks: u8,
    /// Stick length below which there is no move input.
    pub deadzone: f32,
    /// Stick length from which the gait is Run (below: Walk, scaled).
    pub run_threshold: f32,
    /// Capsule height, m: probe height bands are in character heights.
    pub height: f32,
    /// Capsule radius, m: a ledge-top warp target stays this far inside the far edge.
    pub radius: f32,
    pub gaits: GaitTable,
    pub normal: ModeProfile,
    pub sneak: ModeProfile,
    pub free_run: ModeProfile,
    /// The body mover's braking into a stop (`sf_controller::MoverParams::arrival`): an
    /// [`crate::Owner::Approach`] walk brakes onto its target with it.
    pub arrival: ArrivalParams,
    /// The body mover's velocity half-life (`MoverParams::springs.halflife_velocity`), s: the
    /// approach predicts how far the body still coasts with it.
    pub halflife_velocity: f32,
}

impl Default for CharacterParams {
    fn default() -> Self {
        Self {
            gravity: 9.81,
            jump_height: 1.0,
            coyote_ticks: 6,
            jump_buffer_ticks: 6,
            deadzone: 0.15,
            run_threshold: 0.6,
            height: 1.8,
            radius: 0.3,
            gaits: GaitTable::default(),
            normal: ModeProfile::default(),
            sneak: ModeProfile::default(),
            free_run: ModeProfile::default(),
            arrival: ArrivalParams::default(),
            halflife_velocity: SpringParams::default().halflife_velocity,
        }
    }
}

impl CharacterParams {
    pub fn profile(&self, mode: Mode) -> &ModeProfile {
        match mode {
            Mode::Normal => &self.normal,
            Mode::Sneak => &self.sneak,
            Mode::FreeRun => &self.free_run,
        }
    }
}
