//! Stack split and merge: the quantity picker and the commands the UI sends.
//!
//! The UI never changes a loadout (rpg.md "Commands and events"): it builds an
//! [`RpgCommand`] the host applies on its tick, which emits `StackSplit` / `StacksMerged`.
//! The builders below check the same rules as [`Loadout::split`] / [`Loadout::merge`], so
//! an action the screen offers doesn't fail on the host (the host still checks).

use sf_rpg::{InventoryError, Loadout, RpgCommand, StackId};

use crate::view::InventoryView;

/// A count chooser (split, and later drop, give, sell): a slider or spinner over
/// `min..=max`. Gamepad: left/right ±1, shoulders ±10, triggers to the ends.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct QuantityPicker {
    min: u32,
    max: u32,
    value: u32,
}

impl QuantityPicker {
    /// `None` when the range is empty.
    pub fn new(min: u32, max: u32, value: u32) -> Option<Self> {
        (min <= max).then(|| Self {
            min,
            max,
            value: value.clamp(min, max),
        })
    }

    /// Splitting a stack of `count`: 1..=count-1, starting at half (our default; the
    /// common choices are 1 and half). `None` for a single piece.
    pub fn for_split(count: u32) -> Option<Self> {
        Self::new(1, count.checked_sub(1)?, count / 2)
    }

    pub fn min(&self) -> u32 {
        self.min
    }

    pub fn max(&self) -> u32 {
        self.max
    }

    pub fn value(&self) -> u32 {
        self.value
    }

    pub fn set(&mut self, value: u32) {
        self.value = value.clamp(self.min, self.max);
    }

    /// Moves by `delta`, clamped (no wrap).
    pub fn step(&mut self, delta: i64) {
        let v = (i64::from(self.value) + delta).clamp(i64::from(self.min), i64::from(self.max));
        self.value = u32::try_from(v).unwrap_or(self.max);
    }

    /// Slider position in 0..=1.
    pub fn fraction(&self) -> f32 {
        if self.max == self.min {
            return 1.0;
        }
        #[allow(clippy::cast_precision_loss)] // a slider position
        let f = f64::from(self.value - self.min) / f64::from(self.max - self.min);
        #[allow(clippy::cast_possible_truncation)]
        let f = f as f32;
        f
    }

    /// Sets the value from a slider position (mouse drag), rounded to the nearest count.
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)] // clamped to the range
    pub fn set_fraction(&mut self, fraction: f32) {
        let span = f64::from(self.max - self.min);
        let v = f64::from(self.min) + (f64::from(fraction.clamp(0.0, 1.0)) * span).round();
        self.set(v as u32);
    }
}

/// What the action bar may offer for a stack.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct StackActions {
    pub split: bool,
    /// The first stack it may merge into.
    pub merge_into: Option<StackId>,
}

impl InventoryView {
    /// Split and merge availability for a stack (greyed-out actions, controller prompts).
    pub fn actions(&self, loadout: &Loadout, stack: StackId) -> StackActions {
        StackActions {
            split: self.split_picker(loadout, stack).is_some(),
            merge_into: loadout.merge_targets(stack).next(),
        }
    }

    /// A picker for splitting `stack`; `None` when it can't split (one piece, worn).
    pub fn split_picker(&self, loadout: &Loadout, stack: StackId) -> Option<QuantityPicker> {
        if loadout.is_locked(stack) {
            return None;
        }
        QuantityPicker::for_split(loadout.inventory().get(stack)?.count)
    }

    /// The command splitting `count` pieces off `stack`.
    pub fn split_command(
        &self,
        loadout: &Loadout,
        stack: StackId,
        count: u32,
    ) -> Result<RpgCommand, InventoryError> {
        let source = loadout
            .inventory()
            .get(stack)
            .ok_or(InventoryError::NoStack(stack))?;
        if loadout.is_locked(stack) {
            return Err(InventoryError::Equipped(stack));
        }
        if count == 0 {
            return Err(InventoryError::ZeroCount);
        }
        if count >= source.count {
            return Err(InventoryError::NotEnough {
                item: source.item.clone(),
                have: u64::from(source.count),
                want: count.saturating_add(1),
            });
        }
        Ok(RpgCommand::SplitStack {
            holder: self.holder(),
            stack,
            count,
        })
    }

    /// The command merging `from` into `into`, or into its first merge target when `into`
    /// is `None` (the "merge" action; dropping a stack onto another names the target).
    pub fn merge_command(
        &self,
        loadout: &Loadout,
        from: StackId,
        into: Option<StackId>,
    ) -> Result<RpgCommand, InventoryError> {
        let into = if let Some(into) = into {
            loadout.can_merge(from, into)?;
            into
        } else {
            loadout
                .inventory()
                .get(from)
                .ok_or(InventoryError::NoStack(from))?;
            let why = if loadout.is_locked(from) {
                InventoryError::Equipped(from)
            } else {
                InventoryError::NoMergeTarget(from)
            };
            loadout.merge_targets(from).next().ok_or(why)?
        };
        Ok(RpgCommand::MergeStacks {
            holder: self.holder(),
            from,
            into,
        })
    }
}
