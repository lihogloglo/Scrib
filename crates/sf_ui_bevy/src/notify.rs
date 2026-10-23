//! Message boxes and toasts on screen (ui-text.md "Notifications").
//!
//! - [`MessageBoxes`] wraps `sf_ui::MessageBoxQueue`. The front box is drawn
//!   as a modal: a full-screen backdrop that is the root of the
//!   [`MessageBoxConfig::screen`] screen, pushed on the Modal layer while a
//!   box is up (when the registry has that screen), so input routes to it
//!   and focus stays on its buttons. A button's `Activate` (click, Enter,
//!   gamepad confirm) or Back (its cancel button) answers it as a
//!   [`MessageBoxAnswer`] message.
//! - [`Toasts`] wraps `sf_ui::ToastQueue`, ticked on real time (it keeps
//!   counting while the game is paused) and drawn in one `#sf-toasts`
//!   column on the Notification layer, oldest first; opacity follows the
//!   fades (none with the reduce-motion setting).
//!
//! Parts carry `Name`s and [`UiPart`]s for theme sheets; text uses the
//! [`RichTextTheme`] body face at the player's text scale.

use bevy_color::{Alpha, Color};
use bevy_ecs::prelude::*;
use bevy_text::{TextColor, TextLayout, TextSpan};
use bevy_time::{Real, Time};
use bevy_ui::prelude::*;
use bevy_ui_widgets::{Activate, Button};
use sf_ui::{
    MessageBoxChoice, MessageBoxId, MessageBoxQueue, RunStyle, ScreenId, ToastConfig, ToastId,
    ToastQueue,
};

use crate::nav::{UiDefaultFocus, UiFocusable};
use crate::reader::UiPart;
use crate::router::{UiActionEvent, UiHandlesBack, UiScreenRoot};
use crate::text_measure::RichTextTheme;
use crate::{ScreenStackRes, UiAction, UiRegistryRes, UiSettings};

/// Queued message boxes; the front one is on screen.
#[derive(Resource, Debug, Default)]
pub struct MessageBoxes(pub MessageBoxQueue);

impl MessageBoxes {
    /// Queues a box (buttons already localized; empty gets the OK label).
    pub fn show(&mut self, text: impl Into<String>, buttons: Vec<String>, config: &MessageBoxConfig) -> MessageBoxId {
        self.0.show(text, buttons, &config.ok_label)
    }
}

/// How message boxes open.
#[derive(Resource, Clone, Debug)]
pub struct MessageBoxConfig {
    /// The Modal-layer `ui.screen` a box opens as (its input context gets
    /// the box's actions). Without that record the box still draws but input
    /// is not routed to it.
    pub screen: ScreenId,
    /// Label of the single button of a box asked without buttons.
    pub ok_label: String,
}

impl Default for MessageBoxConfig {
    fn default() -> Self {
        Self {
            screen: ScreenId::from("sf.message_box"),
            ok_label: "OK".into(),
        }
    }
}

/// A message box was answered.
#[derive(Message, Clone, Copy, Debug, PartialEq, Eq)]
pub struct MessageBoxAnswer(pub MessageBoxChoice);

/// The drawn box (backdrop root), with the box it shows.
#[derive(Component, Clone, Copy, Debug)]
pub struct MessageBoxView(pub MessageBoxId);

/// A message box button.
#[derive(Component, Clone, Copy, Debug)]
pub struct MessageBoxButton {
    pub id: MessageBoxId,
    pub index: usize,
}

/// Colours of the notification widgets (CSS styles the rest).
#[derive(Resource, Clone, Debug)]
pub struct NotifyColors {
    pub text: Color,
    pub toast_background: Color,
    pub backdrop: Color,
}

impl Default for NotifyColors {
    fn default() -> Self {
        Self {
            text: Color::srgb(0.95, 0.92, 0.85),
            toast_background: Color::srgba(0.05, 0.04, 0.03, 0.8),
            backdrop: Color::srgba(0.0, 0.0, 0.0, 0.45),
        }
    }
}

/// Draw order of the notification widgets above other UI roots.
pub const MESSAGE_BOX_Z: i32 = 1 << 20;
pub const TOAST_Z: i32 = (1 << 20) + 1;

fn part(name: &'static str) -> (Name, UiPart) {
    (Name::new(name), UiPart(name))
}

fn text_font(theme: &RichTextTheme, settings: Option<&UiSettings>) -> bevy_text::TextFont {
    theme.text_font(&RunStyle::default(), settings.map_or(1.0, |s| s.text_scale))
}

/// Draws the front message box when it changes and keeps its screen open.
#[allow(clippy::too_many_arguments)]
pub(crate) fn sync_message_box(
    mut commands: Commands,
    boxes: Res<MessageBoxes>,
    config: Res<MessageBoxConfig>,
    theme: Res<RichTextTheme>,
    colors: Res<NotifyColors>,
    settings: Option<Res<UiSettings>>,
    registry: Option<Res<UiRegistryRes>>,
    mut stack: Option<ResMut<ScreenStackRes>>,
    views: Query<(Entity, &MessageBoxView)>,
    mut seen: Local<Option<u64>>,
) {
    if *seen == Some(boxes.0.version()) {
        return;
    }
    *seen = Some(boxes.0.version());
    let current = boxes.0.current();
    for (entity, view) in &views {
        if current.is_none_or(|b| b.id != view.0) {
            commands.entity(entity).despawn();
        }
    }
    let screen = config.screen.as_str();
    if let Some(stack) = stack.as_mut() {
        let open = stack.0.is_open(screen);
        match (current.is_some(), open, registry.as_ref()) {
            (true, false, Some(registry)) if registry.0.screen(screen).is_some() => {
                let _ = stack.0.push(&registry.0, screen);
            }
            (false, true, _) => {
                stack.0.close(screen);
            }
            _ => {}
        }
    }
    let Some(shown) = current else {
        return;
    };
    if views.iter().any(|(_, v)| v.0 == shown.id) {
        return;
    }
    let font = text_font(&theme, settings.as_deref());
    let root = commands
        .spawn((
            part("sf-message-box"),
            MessageBoxView(shown.id),
            UiScreenRoot(config.screen.clone()),
            UiHandlesBack,
            Node {
                position_type: PositionType::Absolute,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..Node::default()
            },
            BackgroundColor(colors.backdrop),
            GlobalZIndex(MESSAGE_BOX_Z),
        ))
        .id();
    let panel = commands
        .spawn((
            part("sf-message-box-panel"),
            Node {
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                row_gap: Val::Px(16.0),
                padding: UiRect::all(Val::Px(24.0)),
                max_width: Val::Px(560.0),
                ..Node::default()
            },
            BackgroundColor(colors.toast_background),
            ChildOf(root),
        ))
        .id();
    commands.spawn((
        part("sf-message-box-text"),
        Text::new(shown.text.clone()),
        font.clone(),
        TextColor(colors.text),
        TextLayout::justify(bevy_text::Justify::Center),
        ChildOf(panel),
    ));
    let row = commands
        .spawn((
            part("sf-message-box-buttons"),
            Node {
                flex_direction: FlexDirection::Row,
                flex_wrap: FlexWrap::Wrap,
                justify_content: JustifyContent::Center,
                column_gap: Val::Px(12.0),
                row_gap: Val::Px(8.0),
                ..Node::default()
            },
            ChildOf(panel),
        ))
        .id();
    for (index, label) in shown.buttons.iter().enumerate() {
        let mut button = commands.spawn((
            part("sf-message-box-button"),
            Node {
                padding: UiRect::axes(Val::Px(12.0), Val::Px(6.0)),
                ..Node::default()
            },
            Button,
            UiFocusable,
            MessageBoxButton { id: shown.id, index },
            ChildOf(row),
            children![(Text::new(label.clone()), font.clone(), TextColor(colors.text))],
        ));
        if index == 0 {
            button.insert(UiDefaultFocus);
        }
    }
}

/// A message box button was activated: answer the box.
pub(crate) fn on_message_box_button(
    activate: On<Activate>,
    buttons: Query<&MessageBoxButton>,
    mut boxes: ResMut<MessageBoxes>,
    mut out: MessageWriter<MessageBoxAnswer>,
) {
    let Ok(button) = buttons.get(activate.entity) else {
        return;
    };
    if boxes.0.current().is_some_and(|b| b.id == button.id)
        && let Some(choice) = boxes.0.choose(button.index)
    {
        out.write(MessageBoxAnswer(choice));
    }
}

/// Back on the message box screen picks its cancel button.
pub(crate) fn message_box_back(
    mut actions: MessageReader<UiActionEvent>,
    config: Res<MessageBoxConfig>,
    mut boxes: ResMut<MessageBoxes>,
    mut out: MessageWriter<MessageBoxAnswer>,
) {
    for event in actions.read() {
        if event.action == UiAction::Back
            && event.screen == config.screen
            && let Some(choice) = boxes.0.cancel()
        {
            out.write(MessageBoxAnswer(choice));
        }
    }
}

/// Timed notifications. Push with [`Toasts::push`]; tune with
/// [`Toasts::config`] (the reduce-motion setting drops fades on top).
#[derive(Resource, Debug, Default)]
pub struct Toasts {
    queue: ToastQueue,
    config: ToastConfig,
}

impl Toasts {
    pub fn new(config: ToastConfig) -> Self {
        Self {
            queue: ToastQueue::new(config),
            config,
        }
    }

    pub fn push(&mut self, text: impl Into<String>) -> ToastId {
        self.queue.push(text)
    }

    pub fn dismiss(&mut self, id: ToastId) {
        self.queue.dismiss(id);
    }

    pub fn queue(&self) -> &ToastQueue {
        &self.queue
    }

    pub fn config(&self) -> &ToastConfig {
        &self.config
    }

    pub fn set_config(&mut self, config: ToastConfig) {
        self.config = config;
    }
}

/// The toast column.
#[derive(Component, Clone, Copy, Debug)]
pub struct ToastStack;

/// One drawn toast.
#[derive(Component, Clone, Copy, Debug)]
pub struct ToastView(pub ToastId);

/// Advances toasts on real time.
pub(crate) fn tick_toasts(time: Res<Time<Real>>, settings: Option<Res<UiSettings>>, mut toasts: ResMut<Toasts>) {
    let reduce = settings.is_some_and(|s| s.reduce_motion);
    let config = if reduce {
        toasts.config.without_fades()
    } else {
        toasts.config
    };
    toasts.queue.config = config;
    toasts.queue.tick(time.delta_secs());
}

/// Spawns, relabels and despawns toast nodes, and fades them.
#[allow(clippy::too_many_arguments, clippy::type_complexity)]
pub(crate) fn sync_toasts(
    mut commands: Commands,
    toasts: Res<Toasts>,
    theme: Res<RichTextTheme>,
    colors: Res<NotifyColors>,
    settings: Option<Res<UiSettings>>,
    stacks: Query<Entity, With<ToastStack>>,
    mut views: Query<(Entity, &ToastView, &mut BackgroundColor, &Children)>,
    mut texts: Query<(&mut Text, &mut TextColor), Without<TextSpan>>,
    mut seen: Local<Option<u64>>,
) {
    let queue = &toasts.queue;
    let config = queue.config;
    let changed = *seen != Some(queue.version());
    *seen = Some(queue.version());
    if changed {
        let column = stacks.iter().next().unwrap_or_else(|| {
            commands
                .spawn((
                    part("sf-toasts"),
                    ToastStack,
                    Node {
                        position_type: PositionType::Absolute,
                        width: Val::Percent(100.0),
                        bottom: Val::Percent(12.0),
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        row_gap: Val::Px(6.0),
                        ..Node::default()
                    },
                    GlobalZIndex(TOAST_Z),
                ))
                .id()
        });
        for (entity, view, _, children) in &views {
            match queue.visible().iter().find(|t| t.id == view.0) {
                None => commands.entity(entity).despawn(),
                Some(toast) => {
                    for child in children {
                        if let Ok((mut text, _)) = texts.get_mut(*child)
                            && text.0 != toast.label()
                        {
                            text.0 = toast.label();
                        }
                    }
                }
            }
        }
        let font = text_font(&theme, settings.as_deref());
        for toast in queue.visible() {
            if views.iter().any(|(_, v, ..)| v.0 == toast.id) {
                continue;
            }
            let opacity = toast.opacity(&config);
            commands.spawn((
                part("sf-toast"),
                ToastView(toast.id),
                Node {
                    padding: UiRect::axes(Val::Px(12.0), Val::Px(4.0)),
                    ..Node::default()
                },
                BackgroundColor(colors.toast_background.with_alpha(colors.toast_background.alpha() * opacity)),
                ChildOf(column),
                children![(
                    Text::new(toast.label()),
                    font.clone(),
                    TextColor(colors.text.with_alpha(colors.text.alpha() * opacity)),
                )],
            ));
        }
    }
    for (_, view, mut background, children) in &mut views {
        let Some(toast) = queue.visible().iter().find(|t| t.id == view.0) else {
            continue;
        };
        let opacity = toast.opacity(&config);
        background.0 = colors
            .toast_background
            .with_alpha(colors.toast_background.alpha() * opacity);
        for child in children {
            if let Ok((_, mut color)) = texts.get_mut(*child) {
                color.0 = colors.text.with_alpha(colors.text.alpha() * opacity);
            }
        }
    }
}
