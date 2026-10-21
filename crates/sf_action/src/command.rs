//! Commands: the only input to the action layer (action.md "Commands"), the frame-rate sampler
//! that turns device edges into one command per tick, and the per-player feed the host reads
//! remote commands from (net.md "Prediction").

use std::collections::VecDeque;

use glam::Vec2;
use serde::{Deserialize, Serialize};

use crate::ids::{PlayerId, Tick};

/// Number of action classes.
pub const CLASSES: usize = 13;

/// One vocabulary for buttons, buffers and windows.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[repr(u8)]
pub enum ActionClass {
    Light,
    Heavy,
    Offhand,
    Dodge,
    Jump,
    Up,
    Down,
    Guard,
    Interact,
    Item,
    /// The stick: active while it is outside the dead zone. Never latched.
    Move,
    /// Held: free-run (action.md "Mode profiles"). Never latched.
    Sprint,
    /// Pressed: toggles sneak. Never latched.
    Sneak,
}

impl ActionClass {
    pub const ALL: [Self; CLASSES] = [
        Self::Light,
        Self::Heavy,
        Self::Offhand,
        Self::Dodge,
        Self::Jump,
        Self::Up,
        Self::Down,
        Self::Guard,
        Self::Interact,
        Self::Item,
        Self::Move,
        Self::Sprint,
        Self::Sneak,
    ];

    pub const fn index(self) -> usize {
        self as usize
    }

    /// Fires from the buffer only while still held (Guard: action.md "The chooser" 3).
    pub const fn needs_hold(self) -> bool {
        matches!(self, Self::Guard)
    }
}

/// A set of action classes (a bit per class).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ClassSet(pub u16);

impl ClassSet {
    pub const EMPTY: Self = Self(0);
    pub const ALL: Self = Self((1 << CLASSES) - 1);
    /// Classes the buffer latches: everything but the state classes (stick, sprint, sneak).
    pub const LATCHABLE: Self =
        Self(Self::ALL.0 & !(1 << ActionClass::Move as u16 | 1 << ActionClass::Sprint as u16 | 1 << ActionClass::Sneak as u16));
    /// What a traversal `tail` window opens (action.md "Windows").
    pub const TAIL: Self = Self::of(&[ActionClass::Move, ActionClass::Jump, ActionClass::Dodge, ActionClass::Down]);

    pub const fn of(classes: &[ActionClass]) -> Self {
        let mut bits = 0u16;
        let mut i = 0;
        while i < classes.len() {
            bits |= 1 << classes[i] as u16;
            i += 1;
        }
        Self(bits)
    }

    pub const fn contains(self, c: ActionClass) -> bool {
        self.0 & (1 << c as u16) != 0
    }

    pub fn insert(&mut self, c: ActionClass) {
        self.0 |= 1 << c as u16;
    }

    pub fn remove(&mut self, c: ActionClass) {
        self.0 &= !(1 << c as u16);
    }

    #[must_use]
    pub const fn union(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }

    #[must_use]
    pub const fn intersection(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }

    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }

    pub fn iter(self) -> impl Iterator<Item = ActionClass> {
        ActionClass::ALL.into_iter().filter(move |c| self.contains(*c))
    }
}

/// Button state this tick, per action class.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct ButtonEdges {
    /// Went down since the last tick (also set when it went down and up again in between).
    pub pressed: ClassSet,
    /// Down at the sample.
    pub held: ClassSet,
    /// Went up since the last tick.
    pub released: ClassSet,
}

/// Look direction, radians (yaw about +Y, 0 faces +Z; pitch up positive). Aim input, never the
/// camera's final pose (camera.md "Rules").
#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct ViewDir {
    pub yaw: f32,
    pub pitch: f32,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum LockSide {
    #[default]
    Left,
    Right,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum LockRequest {
    #[default]
    None,
    Toggle,
    Next(LockSide),
}

/// One character's input for one fixed tick. Players, AI, scripts and the network all write it.
#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Command {
    pub tick: Tick,
    /// Set for people (local or remote); `None` for AI and scripts.
    pub player: Option<PlayerId>,
    /// Move input in world XZ (x, z), length up to 1: camera-relative already resolved.
    pub stick: Vec2,
    pub view: ViewDir,
    pub buttons: ButtonEdges,
    /// Ticks each class has been held, counting this one (0 when up).
    pub hold_ticks: [u16; CLASSES],
    pub lock_request: LockRequest,
}

impl Command {
    /// No input at `tick`.
    pub fn empty(tick: Tick) -> Self {
        Self { tick, ..Self::default() }
    }

    pub fn pressed(&self, c: ActionClass) -> bool {
        self.buttons.pressed.contains(c)
    }

    pub fn held(&self, c: ActionClass) -> bool {
        self.buttons.held.contains(c)
    }

    pub fn released(&self, c: ActionClass) -> bool {
        self.buttons.released.contains(c)
    }
}

/// One physical button bound to two classes: a tap fires `tap` on release, holding past
/// `threshold` ticks holds `hold` (Dodge on tap, Sprint on hold: action.md "The chooser" 3).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct TapHold {
    tap: ActionClass,
    hold: ActionClass,
    threshold: u16,
    down: bool,
    released: bool,
    ticks: u16,
}

/// The input shell's Bevy-free half: device events arrive at frame rate, and [`Self::sample`]
/// turns everything since the last tick into one [`Command`] (presses between ticks are
/// consumed on the first tick that samples them). The Bevy shell feeds it from
/// `bevy_enhanced_input` actions; nothing here knows about devices or bindings.
#[derive(Clone, Debug, Default)]
pub struct InputSampler {
    stick: Vec2,
    view: ViewDir,
    pressed: ClassSet,
    held: ClassSet,
    released: ClassSet,
    hold_ticks: [u16; CLASSES],
    lock: LockRequest,
    shared: Vec<TapHold>,
}

impl InputSampler {
    pub fn new() -> Self {
        Self::default()
    }

    /// Binds a shared tap/hold button; returns its index for [`Self::press_shared`].
    pub fn bind_tap_hold(&mut self, tap: ActionClass, hold: ActionClass, threshold_ticks: u16) -> usize {
        self.shared.push(TapHold { tap, hold, threshold: threshold_ticks.max(1), down: false, released: false, ticks: 0 });
        self.shared.len() - 1
    }

    pub fn press(&mut self, c: ActionClass) {
        if !self.held.contains(c) {
            self.pressed.insert(c);
        }
        self.held.insert(c);
    }

    pub fn release(&mut self, c: ActionClass) {
        if self.held.contains(c) {
            self.released.insert(c);
            self.held.remove(c);
        }
    }

    pub fn press_shared(&mut self, button: usize) {
        let b = &mut self.shared[button];
        if !b.down {
            b.down = true;
            b.ticks = 0;
        }
    }

    pub fn release_shared(&mut self, button: usize) {
        let b = &mut self.shared[button];
        if b.down {
            b.down = false;
            b.released = true;
        }
    }

    pub fn set_stick(&mut self, stick: Vec2) {
        self.stick = stick.clamp_length_max(1.0);
    }

    pub fn set_view(&mut self, view: ViewDir) {
        self.view = view;
    }

    pub fn request_lock(&mut self, request: LockRequest) {
        self.lock = request;
    }

    /// The command for `tick`; clears the edges.
    pub fn sample(&mut self, tick: Tick, player: Option<PlayerId>) -> Command {
        for i in 0..self.shared.len() {
            let b = self.shared[i];
            if b.released {
                if b.ticks < b.threshold {
                    // A tap: the tap class goes down and up within this tick.
                    self.pressed.insert(b.tap);
                    self.released.insert(b.tap);
                } else {
                    self.release(b.hold);
                }
                self.shared[i].released = false;
                self.shared[i].ticks = 0;
            } else if b.down {
                let ticks = b.ticks.saturating_add(1);
                self.shared[i].ticks = ticks;
                if ticks == b.threshold {
                    self.press(b.hold);
                }
            }
        }
        for c in ActionClass::ALL {
            let t = &mut self.hold_ticks[c.index()];
            *t = if self.held.contains(c) { t.saturating_add(1) } else { 0 };
        }
        let cmd = Command {
            tick,
            player,
            stick: self.stick,
            view: self.view,
            buttons: ButtonEdges { pressed: self.pressed, held: self.held, released: self.released },
            hold_ticks: self.hold_ticks,
            lock_request: self.lock,
        };
        self.pressed = ClassSet::EMPTY;
        self.released = ClassSet::EMPTY;
        self.lock = LockRequest::None;
        cmd
    }
}

/// Ticks a missing command repeats the last stick, view and held buttons before going empty
/// (net.md "Prediction": no stuck attacks).
pub const REPEAT_MISSING_TICKS: u8 = 6;

/// One player's incoming commands on the host, keyed by tick. Commands arrive redundantly and
/// out of order; [`Self::take`] yields exactly one per tick.
#[derive(Clone, Debug, Default)]
pub struct CommandFeed {
    pending: VecDeque<Command>,
    last: Option<Command>,
    missing: u8,
    next: Option<Tick>,
}

impl CommandFeed {
    pub fn new() -> Self {
        Self::default()
    }

    /// Queues a received command; duplicates and ticks already taken are ignored.
    pub fn insert(&mut self, cmd: Command) {
        if self.next.is_some_and(|n| cmd.tick < n) {
            return;
        }
        match self.pending.binary_search_by_key(&cmd.tick, |c| c.tick) {
            Ok(_) => {}
            Err(i) => self.pending.insert(i, cmd),
        }
    }

    /// The command for `tick`: the received one, else a repeat of the last one without new
    /// presses for up to [`REPEAT_MISSING_TICKS`], else empty.
    pub fn take(&mut self, tick: Tick) -> Command {
        while self.pending.front().is_some_and(|c| c.tick < tick) {
            self.pending.pop_front();
        }
        self.next = Some(tick.next());
        if self.pending.front().is_some_and(|c| c.tick == tick) {
            let cmd = self.pending.pop_front().unwrap_or_default();
            self.last = Some(cmd);
            self.missing = 0;
            return cmd;
        }
        self.missing = self.missing.saturating_add(1);
        match self.last {
            Some(last) if self.missing <= REPEAT_MISSING_TICKS => {
                let mut cmd = Command { tick, buttons: ButtonEdges { held: last.buttons.held, ..ButtonEdges::default() }, ..last };
                for c in ActionClass::ALL {
                    let t = &mut cmd.hold_ticks[c.index()];
                    *t = if cmd.buttons.held.contains(c) { t.saturating_add(u16::from(self.missing)) } else { 0 };
                }
                cmd.lock_request = LockRequest::None;
                cmd
            }
            last => Command { player: last.and_then(|c| c.player), ..Command::empty(tick) },
        }
    }
}
