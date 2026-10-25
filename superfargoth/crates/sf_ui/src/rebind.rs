//! The keybinds screen's rebinding flow, Bevy-free (ui.md "Keybinds", U6).
//!
//! The player picks a slot, presses an input, and the binding applies, or the
//! screen asks how to resolve a conflict. [`Rebinder`] is that state machine;
//! the engine adapter feeds it the inputs pressed and held each frame as input
//! names.
//!
//! - **Arming**: capture starts only once every input is released, so the
//!   press that opened it (Enter, A, a click) is never captured.
//! - **Device**: only inputs of the slot's device and shape count; the rest are
//!   ignored (a pad press while rebinding a key).
//! - **Chords**: a modifier key waits; a key or mouse button pressed while it
//!   is held binds with the modifiers held (`key:Ctrl+KeyS`); releasing the
//!   modifier alone binds the modifier key.
//! - **No cancel input**: every input is bindable (menu open on Esc must be
//!   rebindable to Esc, Elden Ring lesson), so capture cancels after a timeout
//!   or when the screen calls [`Rebinder::cancel`] (pointer leaves, tab
//!   switch). Unity binds a cancel control instead; we don't reserve one.
//! - **Draining**: after a capture, UI actions stay blocked until every input
//!   is released, so the captured Esc doesn't also close the menu.

use crate::input::{Device, Input, InputKind, Mods};
use crate::keymap::{ActionShape, Keymap, KeymapError, Proposal, Resolution, SlotRef};

/// Seconds without a valid input before a capture cancels.
pub const CAPTURE_TIMEOUT: f32 = 5.0;

/// Inputs of one frame, as input names without modifiers (`key:ControlLeft`,
/// `key:KeyS`, `pad:South`, `stick:Left`, `mouse:Left`).
#[derive(Clone, Copy, Debug, Default)]
pub struct CaptureFrame<'a> {
    /// Newly pressed this frame (a stick counts when it crosses its threshold).
    pub pressed: &'a [String],
    /// Held now, the newly pressed included.
    pub held: &'a [String],
}

/// What a rebinding step produced, for the screen.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RebindEvent {
    /// The slot now holds `input`.
    Applied { slot: SlotRef, input: String },
    /// The input collides with other actions; ask, then [`Rebinder::resolve`].
    Conflict(Proposal),
    /// The input or the resolution was refused (a required action would lose
    /// its last binding, a wrong shape, ...). The keymap is unchanged.
    Refused { slot: SlotRef, error: KeymapError },
    /// Capture timed out or the screen cancelled.
    Cancelled { slot: SlotRef },
}

#[derive(Clone, Debug, Default)]
enum State {
    #[default]
    Idle,
    Capturing {
        slot: SlotRef,
        armed: bool,
        /// A modifier key pressed alone, bound if released alone.
        modifier: Option<String>,
        waited: f32,
    },
    Confirming(Proposal),
}

/// Keybinds screen state: idle, capturing an input, or confirming a conflict.
#[derive(Clone, Debug)]
pub struct Rebinder {
    state: State,
    draining: bool,
    /// Capture timeout in seconds.
    pub timeout: f32,
}

impl Default for Rebinder {
    fn default() -> Self {
        Self {
            state: State::Idle,
            draining: false,
            timeout: CAPTURE_TIMEOUT,
        }
    }
}

impl Rebinder {
    /// Starts capturing for `slot` (replacing any capture or pending conflict).
    pub fn start(&mut self, keymap: &Keymap, slot: SlotRef) -> Result<(), KeymapError> {
        let def = keymap
            .def(&slot.action)
            .ok_or_else(|| KeymapError::UnknownAction(slot.action.clone()))?;
        let slots = def.slot_count(slot.device);
        if slot.slot >= slots {
            return Err(KeymapError::NoSlot {
                action: slot.action,
                device: slot.device,
                slot: slot.slot,
                slots,
            });
        }
        self.state = State::Capturing {
            slot,
            armed: false,
            modifier: None,
            waited: 0.0,
        };
        Ok(())
    }

    /// Cancels a capture or a pending conflict.
    pub fn cancel(&mut self) -> Option<RebindEvent> {
        match std::mem::take(&mut self.state) {
            State::Idle => None,
            State::Capturing { slot, .. } => {
                self.draining = true;
                Some(RebindEvent::Cancelled { slot })
            }
            State::Confirming(p) => Some(RebindEvent::Cancelled { slot: p.slot }),
        }
    }

    /// The slot being captured.
    pub fn capturing(&self) -> Option<&SlotRef> {
        match &self.state {
            State::Capturing { slot, .. } => Some(slot),
            _ => None,
        }
    }

    /// The conflict waiting for the player's choice.
    pub fn pending(&self) -> Option<&Proposal> {
        match &self.state {
            State::Confirming(p) => Some(p),
            _ => None,
        }
    }

    /// UI actions must not fire: a capture is running, or its input is still
    /// held.
    pub fn blocks_ui(&self) -> bool {
        self.draining || matches!(self.state, State::Capturing { .. })
    }

    /// One frame of input. Returns what happened, if anything.
    pub fn update(
        &mut self,
        keymap: &mut Keymap,
        dt: f32,
        frame: CaptureFrame<'_>,
    ) -> Option<RebindEvent> {
        if self.draining && frame.held.is_empty() && self.capturing().is_none() {
            self.draining = false;
        }
        let State::Capturing {
            slot,
            armed,
            modifier,
            waited,
        } = &mut self.state
        else {
            return None;
        };
        *waited += dt;
        if !*armed {
            *armed = frame.held.is_empty();
            return self.timed_out();
        }
        let Some(shape) = keymap.def(&slot.action).map(|d| d.shape) else {
            return self.cancel();
        };
        let accepted = |name: &&String| {
            Input::parse(name).is_ok_and(|i| {
                i.device() == slot.device
                    && (shape_accepts(shape, i.kind) || i.modifier().is_some())
            })
        };
        let mut captured = None;
        for name in frame.pressed.iter().filter(accepted) {
            let input = Input::parse(name).ok()?;
            if slot.device == Device::Kbm && input.modifier().is_some() {
                if shape_accepts(shape, InputKind::Key) {
                    modifier.get_or_insert_with(|| name.clone());
                }
                continue;
            }
            if !shape_accepts(shape, input.kind) {
                continue;
            }
            let mods = if matches!(input.kind, InputKind::Key | InputKind::Mouse) {
                held_mods(frame.held)
            } else {
                Mods::default()
            };
            captured = Some(Input { mods, ..input }.canonical());
            break;
        }
        if captured.is_none()
            && let Some(m) = modifier
            && !frame.held.contains(m)
        {
            captured = Some(m.clone());
        }
        match captured {
            Some(input) => Some(self.finish(keymap, &input)),
            None => self.timed_out(),
        }
    }

    /// Applies the player's choice for the pending conflict.
    pub fn resolve(&mut self, keymap: &mut Keymap, resolution: Resolution) -> Option<RebindEvent> {
        let State::Confirming(proposal) = std::mem::take(&mut self.state) else {
            return None;
        };
        Some(match keymap.apply(&proposal, resolution) {
            Ok(()) => RebindEvent::Applied {
                slot: proposal.slot,
                input: proposal.input,
            },
            Err(error) => RebindEvent::Refused {
                slot: proposal.slot,
                error,
            },
        })
    }

    fn timed_out(&mut self) -> Option<RebindEvent> {
        match &self.state {
            State::Capturing { waited, .. } if *waited >= self.timeout => self.cancel(),
            _ => None,
        }
    }

    fn finish(&mut self, keymap: &mut Keymap, input: &str) -> RebindEvent {
        let State::Capturing { slot, .. } = std::mem::take(&mut self.state) else {
            unreachable!("finish runs while capturing");
        };
        self.draining = true;
        let proposal = match keymap.propose(&slot, input) {
            Ok(p) => p,
            Err(error) => return RebindEvent::Refused { slot, error },
        };
        if !proposal.conflicts.is_empty() {
            self.state = State::Confirming(proposal.clone());
            return RebindEvent::Conflict(proposal);
        }
        match keymap.apply(&proposal, Resolution::KeepBoth) {
            Ok(()) => RebindEvent::Applied {
                slot,
                input: proposal.input,
            },
            Err(error) => RebindEvent::Refused { slot, error },
        }
    }
}

/// Capture takes what the shape accepts, except single pad axes (a stick
/// press is captured whole, as `stick:`).
fn shape_accepts(shape: ActionShape, kind: InputKind) -> bool {
    kind != InputKind::Axis && shape.accepts(kind)
}

fn held_mods(held: &[String]) -> Mods {
    held.iter()
        .filter_map(|n| Input::parse(n).ok()?.modifier())
        .fold(Mods::default(), Mods::with)
}
