//! A readiness change played by a clip: drawing or sheathing a weapon (character.md "Held
//! items"; combat.md "Animation integration"). The clip has an attach moment where the held
//! props move to their new mounts (UE's attach anim notify in an equip montage, Morrowind's
//! `Equip Attach` / `Unequip Detach` text keys, Skyrim's `weaponDraw` / `weaponSheathe`
//! events); the change itself ends with the clip.
//!
//! Gameplay time: the host advances a [`ReadyChange`] on its fixed tick from the clip's baked
//! key time ([`ReadyClip::from_clip`]), so when the prop moves never depends on the drawn
//! animation (net-ready: presentation never feeds gameplay). The clip plays from the same
//! elapsed time. The framework names no event: the content passes its own key name.

use sf_anim::Clip;

use crate::Readiness;

/// A draw or sheathe clip's timing: the attach moment and the length, seconds.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ReadyClip {
    /// When the held props move (`0 <= key <= length`).
    pub key: f32,
    pub length: f32,
}

impl ReadyClip {
    /// No clip: the props move at once and the change ends at once.
    pub const INSTANT: Self = Self { key: 0.0, length: 0.0 };

    /// `key` clamped into `0..=length`; a negative or non-finite value counts as 0.
    #[must_use]
    pub fn new(key: f32, length: f32) -> Self {
        let length = if length.is_finite() { length.max(0.0) } else { 0.0 };
        let key = if key.is_finite() { key.clamp(0.0, length) } else { 0.0 };
        Self { key, length }
    }

    /// `clip`'s length and the time of its first event named `event`; `None` without that event.
    #[must_use]
    pub fn from_clip(clip: &Clip, event: &str) -> Option<Self> {
        clip.events.iter().find(|e| e.name == event).map(|e| Self::new(e.time, clip.duration))
    }
}

/// A readiness change in progress (see the module docs).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ReadyChange {
    /// The readiness the change ends in.
    pub to: Readiness,
    pub clip: ReadyClip,
    /// Seconds since the change started (clamped to the clip length).
    pub elapsed: f32,
}

impl ReadyChange {
    #[must_use]
    pub fn start(to: Readiness, clip: ReadyClip) -> Self {
        Self { to, clip, elapsed: 0.0 }
    }

    /// Advances by `dt` seconds (negative counts as 0); returns whether the change has ended.
    pub fn advance(&mut self, dt: f32) -> bool {
        self.elapsed = (self.elapsed + dt.max(0.0)).min(self.clip.length);
        self.done()
    }

    /// The clip has played out.
    #[must_use]
    pub fn done(&self) -> bool {
        self.elapsed >= self.clip.length
    }

    /// Whether the props have moved: the attach key has passed.
    #[must_use]
    pub fn attached(&self) -> bool {
        self.elapsed >= self.clip.key
    }

    /// The readiness the held props show now: [`Self::to`] from the key on, the other one before.
    #[must_use]
    pub fn shown(&self) -> Readiness {
        match (self.attached(), self.to) {
            (true, to) => to,
            (false, Readiness::Drawn) => Readiness::Sheathed,
            (false, Readiness::Sheathed) => Readiness::Drawn,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_draw_shows_the_prop_from_the_key_and_ends_with_the_clip() {
        let mut c = ReadyChange::start(Readiness::Drawn, ReadyClip::new(0.25, 0.5));
        assert_eq!((c.shown(), c.done()), (Readiness::Sheathed, false));
        assert!(!c.advance(0.125));
        assert_eq!(c.shown(), Readiness::Sheathed);
        assert!(!c.advance(0.125));
        assert_eq!(c.shown(), Readiness::Drawn, "at the key");
        assert!(c.advance(0.3), "past the end");
        assert!((c.elapsed - 0.5).abs() < 1e-6, "clamped to the length");
    }

    #[test]
    fn a_sheathe_hides_the_prop_at_the_key() {
        let mut c = ReadyChange::start(Readiness::Sheathed, ReadyClip::new(0.5, 1.0));
        assert_eq!(c.shown(), Readiness::Drawn);
        c.advance(0.5);
        assert_eq!(c.shown(), Readiness::Sheathed);
        assert!(!c.done());
    }

    #[test]
    fn no_clip_is_instant_and_bad_keys_clamp() {
        let c = ReadyChange::start(Readiness::Drawn, ReadyClip::INSTANT);
        assert!(c.done() && c.shown() == Readiness::Drawn);
        assert_eq!(ReadyClip::new(2.0, 1.0), ReadyClip { key: 1.0, length: 1.0 });
        assert_eq!(ReadyClip::new(f32::NAN, -1.0), ReadyClip::INSTANT);
        assert_eq!(ReadyClip::new(-0.5, 1.0), ReadyClip { key: 0.0, length: 1.0 });
    }
}
