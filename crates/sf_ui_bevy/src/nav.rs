//! Directional navigation with held-direction repeat (ui.md "Input": no
//! virtual cursor, held directions repeat).
//!
//! Bevy's `AutoDirectionalNavigator` picks the neighbour from screen
//! positions; the router keeps its candidate set to the input target screen.
//! Bevy has no repeat for gamepad directions, so [`NavRepeat`] adds the usual
//! delay-then-interval repeat for d-pad, arrow keys and stick alike.

use bevy_ecs::prelude::*;
use bevy_enhanced_input::prelude::*;
use bevy_math::CompassOctant;
use bevy_time::Time;
use bevy_ui::auto_directional_navigation::AutoDirectionalNavigator;

use crate::ScreenStackRes;
use crate::actions::{UiAction, UiActionTag};
use crate::router::UiScreenRoot;

/// A UI entity that can take focus. Navigable while its screen
/// ([`crate::UiScreenRoot`] ancestor) is the input target.
#[derive(Component, Clone, Copy, Debug, Default)]
pub struct UiFocusable;

/// The focusable that gets focus when its screen becomes the input target
/// and no remembered focus applies.
#[derive(Component, Clone, Copy, Debug, Default)]
pub struct UiDefaultFocus;

/// On a screen root: the screen moves focus itself (a view-model owns it: a virtualized grid,
/// text links that are spans, not nodes). While it is the input target, its focusables are left
/// out of Bevy's directional navigation, the router doesn't pick a focus for it (focus elsewhere
/// is cleared), and held directions arrive as [`UiNavEvent`]s, with the same repeat. Confirm
/// still activates whatever the screen focused.
#[derive(Component, Clone, Copy, Debug, Default)]
pub struct UiOwnsFocus;

/// A navigation step for a [`UiOwnsFocus`] screen (the input target): a direction now, then
/// held-direction repeats.
#[derive(Message, Clone, Debug, PartialEq, Eq)]
pub struct UiNavEvent {
    pub direction: CompassOctant,
    pub screen: sf_ui::ScreenId,
}

/// Stick deflection that counts as a direction.
pub const STICK_THRESHOLD: f32 = 0.5;

/// Delay-then-interval repeat of a held direction.
#[derive(Clone, Debug, PartialEq)]
pub struct NavRepeat {
    /// Seconds before the first repeat.
    pub delay: f32,
    /// Seconds between repeats after that.
    pub interval: f32,
    held: Option<CompassOctant>,
    elapsed: f32,
    next: f32,
}

impl Default for NavRepeat {
    fn default() -> Self {
        Self {
            delay: 0.4,
            interval: 0.08,
            held: None,
            elapsed: 0.0,
            next: 0.0,
        }
    }
}

impl NavRepeat {
    /// Feeds this frame's held direction; returns the direction to move now:
    /// at once on a new direction, then after `delay`, then every `interval`.
    pub fn update(&mut self, direction: Option<CompassOctant>, dt: f32) -> Option<CompassOctant> {
        if direction != self.held {
            self.held = direction;
            self.elapsed = 0.0;
            self.next = self.delay;
            return direction;
        }
        let direction = direction?;
        self.elapsed += dt;
        if self.elapsed >= self.next {
            self.next += self.interval;
            return Some(direction);
        }
        None
    }
}

#[derive(Resource, Debug, Default)]
pub(crate) struct NavState {
    pub repeat: NavRepeat,
}

/// Held navigation direction from the digital actions and the stick
/// (four-way: game menus are grids and lists).
/// `digital` is the net of the digital directions (x: right - left, y: up -
/// down, each in -1..=1); a single digital axis wins over the stick.
pub fn held_direction(digital: (i8, i8), stick: bevy_math::Vec2) -> Option<CompassOctant> {
    let (x, y) = digital;
    match (x, y) {
        (0, 1) => return Some(CompassOctant::North),
        (0, -1) => return Some(CompassOctant::South),
        (1, 0) => return Some(CompassOctant::East),
        (-1, 0) => return Some(CompassOctant::West),
        _ => {}
    }
    if stick.length() < STICK_THRESHOLD {
        return None;
    }
    Some(if stick.x.abs() > stick.y.abs() {
        if stick.x > 0.0 {
            CompassOctant::East
        } else {
            CompassOctant::West
        }
    } else if stick.y > 0.0 {
        CompassOctant::North
    } else {
        CompassOctant::South
    })
}

pub(crate) fn navigate(
    time: Res<Time>,
    mut state: ResMut<NavState>,
    actions: Query<(&UiActionTag, &ActionValue)>,
    screens: Res<ScreenStackRes>,
    owners: Query<&UiScreenRoot, With<UiOwnsFocus>>,
    mut events: MessageWriter<UiNavEvent>,
    mut navigator: AutoDirectionalNavigator,
) {
    let (mut up, mut down, mut left, mut right) = (false, false, false, false);
    let mut stick = bevy_math::Vec2::ZERO;
    for (tag, value) in &actions {
        match tag.0 {
            UiAction::NavUp => up |= value.as_bool(),
            UiAction::NavDown => down |= value.as_bool(),
            UiAction::NavLeft => left |= value.as_bool(),
            UiAction::NavRight => right |= value.as_bool(),
            UiAction::NavStick => stick += value.as_axis2d(),
            _ => {}
        }
    }
    let digital = (
        i8::from(right) - i8::from(left),
        i8::from(up) - i8::from(down),
    );
    let held = held_direction(digital, stick);
    let Some(direction) = state.repeat.update(held, time.delta_secs()) else {
        return;
    };
    let target = screens.0.input_target().map(|s| &s.id);
    if let Some(screen) = target.filter(|t| owners.iter().any(|r| r.0 == **t)) {
        events.write(UiNavEvent {
            direction,
            screen: screen.clone(),
        });
    } else {
        // No neighbour that way (an edge) is fine: focus stays.
        let _ = navigator.navigate(direction);
    }
}
