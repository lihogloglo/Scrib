//! Notifications (ui-text.md "Notifications"): modal message boxes with
//! buttons, and timed toasts.
//!
//! Both are plain queues driven by the caller's clock, so their behaviour is
//! testable without an engine; `sf_ui_bevy` draws them on the Modal and
//! Notification layers.
//!
//! - [`MessageBoxQueue`]: one box at a time, first in first out (a second
//!   script's box waits for the first). Choosing a button pops the box and
//!   returns a [`MessageBoxChoice`] with its id, so the caller routes the
//!   answer back to whoever asked (a script's pending-choice slot).
//! - [`ToastQueue`]: short messages that show for a reading-time duration and
//!   fade. At most `max_visible` at once; when full, the oldest is retired
//!   early (fades out) and the new one shows as its slot frees, so nothing is
//!   lost and a burst never covers the screen. A repeat of a visible text
//!   restarts it with a count (`x2`) instead of stacking copies.

use std::collections::VecDeque;

/// Identifies one message box request.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MessageBoxId(pub u64);

/// A modal question: text and the button labels (already localized).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MessageBox {
    pub id: MessageBoxId,
    pub text: String,
    /// Button labels in order; never empty.
    pub buttons: Vec<String>,
    /// The button that Back (Esc, B) picks; `None` = Back does nothing.
    pub cancel: Option<usize>,
}

/// The answer to a message box: which button, by index.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct MessageBoxChoice {
    pub id: MessageBoxId,
    pub index: usize,
}

/// Message boxes waiting to be answered; the front one is shown.
#[derive(Clone, Debug, Default)]
pub struct MessageBoxQueue {
    queue: VecDeque<MessageBox>,
    next_id: u64,
    /// Bumped whenever the shown box changes.
    version: u64,
}

impl MessageBoxQueue {
    pub fn new() -> Self {
        Self::default()
    }

    /// Queues a box. `buttons` empty gets one button labelled `fallback`
    /// (the caller's localized "OK"), so a box can always be closed.
    pub fn show(
        &mut self,
        text: impl Into<String>,
        buttons: Vec<String>,
        fallback: &str,
    ) -> MessageBoxId {
        let buttons = if buttons.is_empty() {
            vec![fallback.to_owned()]
        } else {
            buttons
        };
        let id = MessageBoxId(self.next_id);
        self.next_id += 1;
        if self.queue.is_empty() {
            self.version += 1;
        }
        self.queue.push_back(MessageBox {
            id,
            text: text.into(),
            buttons,
            cancel: None,
        });
        id
    }

    /// Sets the button Back picks on a queued box. Returns false when the
    /// box is gone or the index is out of range.
    pub fn set_cancel(&mut self, id: MessageBoxId, cancel: Option<usize>) -> bool {
        match self.queue.iter_mut().find(|b| b.id == id) {
            Some(b) if cancel.is_none_or(|i| i < b.buttons.len()) => {
                b.cancel = cancel;
                true
            }
            _ => false,
        }
    }

    /// The box on screen.
    pub fn current(&self) -> Option<&MessageBox> {
        self.queue.front()
    }

    /// Boxes waiting behind the current one.
    pub fn pending(&self) -> usize {
        self.queue.len().saturating_sub(1)
    }

    pub fn is_empty(&self) -> bool {
        self.queue.is_empty()
    }

    /// Answers the current box with button `index`; the next box shows.
    /// `None` when no box is shown or the index is out of range.
    pub fn choose(&mut self, index: usize) -> Option<MessageBoxChoice> {
        let current = self.queue.front()?;
        if index >= current.buttons.len() {
            return None;
        }
        let id = current.id;
        self.queue.pop_front();
        self.version += 1;
        Some(MessageBoxChoice { id, index })
    }

    /// Back on the current box: picks its cancel button, if it has one.
    pub fn cancel(&mut self) -> Option<MessageBoxChoice> {
        let index = self.queue.front()?.cancel?;
        self.choose(index)
    }

    /// Withdraws a box without an answer (its asker went away). Returns
    /// whether it was queued.
    pub fn withdraw(&mut self, id: MessageBoxId) -> bool {
        let Some(at) = self.queue.iter().position(|b| b.id == id) else {
            return false;
        };
        self.queue.remove(at);
        if at == 0 {
            self.version += 1;
        }
        true
    }

    /// Changes whenever [`current`](Self::current) does.
    pub fn version(&self) -> u64 {
        self.version
    }
}

/// Identifies one toast.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ToastId(pub u64);

/// Toast timing and stacking. Durations are seconds of UI (real) time.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ToastConfig {
    /// Time on screen before fading: `base + per_char * chars`, clamped to
    /// `min..=max`, times `scale` (an accessibility setting: longer reading
    /// time, WCAG 2.2.1 "Timing Adjustable").
    pub base: f32,
    pub per_char: f32,
    pub min: f32,
    pub max: f32,
    pub scale: f32,
    pub fade_in: f32,
    pub fade_out: f32,
    /// Toasts on screen at once (fading ones included).
    pub max_visible: usize,
    /// Toasts waiting for a slot; past this the oldest waiting one is
    /// dropped (a flood of pickups never lags minutes behind).
    pub max_queued: usize,
    /// A repeat of a visible or queued text bumps its count instead of
    /// showing a copy.
    pub merge_repeats: bool,
}

impl Default for ToastConfig {
    /// About 17 characters per second on top of 1.5 s to notice the toast
    /// (reading-speed rules of thumb for subtitles), 2.5 s to 10 s.
    fn default() -> Self {
        Self {
            base: 1.5,
            per_char: 0.06,
            min: 2.5,
            max: 10.0,
            scale: 1.0,
            fade_in: 0.15,
            fade_out: 0.5,
            max_visible: 4,
            max_queued: 32,
            merge_repeats: true,
        }
    }
}

impl ToastConfig {
    /// Time on screen at full opacity for `text`.
    #[allow(clippy::cast_precision_loss)]
    pub fn duration(&self, text: &str) -> f32 {
        let chars = text.chars().count() as f32;
        (self.base + self.per_char * chars).clamp(self.min, self.max) * self.scale.max(0.0)
    }

    /// No fades (the reduce-motion setting).
    #[must_use]
    pub fn without_fades(mut self) -> Self {
        self.fade_in = 0.0;
        self.fade_out = 0.0;
        self
    }
}

/// Where a visible toast is in its life.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ToastPhase {
    FadingIn,
    Shown,
    FadingOut,
}

/// A toast on screen.
#[derive(Clone, Debug, PartialEq)]
pub struct Toast {
    pub id: ToastId,
    pub text: String,
    /// Times this text was pushed while it was up (shown as `x2`, ...).
    pub count: u32,
    /// Seconds since it appeared (or was restarted by a repeat).
    pub age: f32,
    /// Full-opacity time.
    pub hold: f32,
    /// Set when it was retired early; the fade-out starts at this age.
    retire_at: Option<f32>,
}

impl Toast {
    fn fade_out_start(&self, config: &ToastConfig) -> f32 {
        let natural = config.fade_in + self.hold;
        self.retire_at.map_or(natural, |r| r.min(natural))
    }

    fn end(&self, config: &ToastConfig) -> f32 {
        self.fade_out_start(config) + config.fade_out
    }

    pub fn phase(&self, config: &ToastConfig) -> ToastPhase {
        if self.age >= self.fade_out_start(config) {
            ToastPhase::FadingOut
        } else if self.age < config.fade_in {
            ToastPhase::FadingIn
        } else {
            ToastPhase::Shown
        }
    }

    /// Opacity in `0..=1` for drawing.
    pub fn opacity(&self, config: &ToastConfig) -> f32 {
        let out_start = self.fade_out_start(config);
        if self.age >= out_start {
            if config.fade_out <= 0.0 {
                return 0.0;
            }
            return (1.0 - (self.age - out_start) / config.fade_out).clamp(0.0, 1.0);
        }
        if config.fade_in <= 0.0 {
            return 1.0;
        }
        (self.age / config.fade_in).clamp(0.0, 1.0)
    }

    /// The text with its repeat count, as drawn.
    pub fn label(&self) -> String {
        if self.count > 1 {
            format!("{} (x{})", self.text, self.count)
        } else {
            self.text.clone()
        }
    }
}

/// Timed notifications: visible toasts oldest first, then a waiting queue.
#[derive(Clone, Debug, Default)]
pub struct ToastQueue {
    pub config: ToastConfig,
    visible: Vec<Toast>,
    waiting: VecDeque<(ToastId, String, u32)>,
    next_id: u64,
    version: u64,
}

impl ToastQueue {
    pub fn new(config: ToastConfig) -> Self {
        Self {
            config,
            ..Self::default()
        }
    }

    /// Queues a toast; it shows at once when a slot is free. A repeat of a
    /// visible or waiting text returns that toast's id.
    pub fn push(&mut self, text: impl Into<String>) -> ToastId {
        let text = text.into();
        if self.config.merge_repeats {
            let fade_in = self.config.fade_in;
            if let Some(t) = self
                .visible
                .iter_mut()
                .find(|t| t.text == text && t.retire_at.is_none())
            {
                t.count += 1;
                // Restart past the fade-in (it is already opaque).
                if t.age > fade_in {
                    t.age = fade_in;
                }
                self.version += 1;
                return t.id;
            }
            if let Some(w) = self.waiting.iter_mut().find(|w| w.1 == text) {
                w.2 += 1;
                return w.0;
            }
        }
        let id = ToastId(self.next_id);
        self.next_id += 1;
        self.waiting.push_back((id, text, 1));
        while self.waiting.len() > self.config.max_queued.max(1) {
            self.waiting.pop_front();
        }
        self.admit();
        id
    }

    /// Advances time by `dt` seconds: ages toasts, removes finished ones and
    /// shows waiting ones in freed slots.
    pub fn tick(&mut self, dt: f32) {
        let dt = dt.max(0.0);
        for t in &mut self.visible {
            t.age += dt;
        }
        let config = self.config;
        let before = self.visible.len();
        self.visible.retain(|t| t.age < t.end(&config));
        if self.visible.len() != before {
            self.version += 1;
        }
        self.admit();
    }

    /// Fills free slots from the queue; when the queue is non-empty and every
    /// slot is held by a toast that isn't leaving, retires the oldest.
    fn admit(&mut self) {
        let max = self.config.max_visible.max(1);
        while self.visible.len() < max {
            let Some((id, text, count)) = self.waiting.pop_front() else {
                break;
            };
            let hold = self.config.duration(&text);
            self.visible.push(Toast {
                id,
                text,
                count,
                age: 0.0,
                hold,
                retire_at: None,
            });
            self.version += 1;
        }
        if !self.waiting.is_empty() {
            let config = self.config;
            let leaving = self
                .visible
                .iter()
                .any(|t| t.phase(&config) == ToastPhase::FadingOut);
            if !leaving && let Some(oldest) = self.visible.first_mut() {
                oldest.retire_at = Some(oldest.age);
                self.version += 1;
            }
        }
    }

    /// Removes a toast now (visible or waiting).
    pub fn dismiss(&mut self, id: ToastId) {
        self.visible.retain(|t| t.id != id);
        self.waiting.retain(|w| w.0 != id);
        self.version += 1;
        self.admit();
    }

    /// Toasts on screen, oldest first.
    pub fn visible(&self) -> &[Toast] {
        &self.visible
    }

    pub fn waiting(&self) -> usize {
        self.waiting.len()
    }

    /// Changes when toasts appear, disappear or change text (not on
    /// fades: read [`Toast::opacity`] every frame).
    pub fn version(&self) -> u64 {
        self.version
    }
}
