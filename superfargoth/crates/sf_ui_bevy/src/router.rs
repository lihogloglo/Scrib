//! The input router (ui.md "Interfaces": "the top active screen of the highest
//! input-taking layer gets actions", Unreal Common UI's rule).
//!
//! `sf_ui::ScreenStack` decides the input target; here its input context id
//! becomes one `bevy_enhanced_input` context entity (`UiScreenInput`). Only the
//! target's entity is active, at [`UI_CONTEXT_PRIORITY`], and its actions
//! consume their inputs, so lower screens and gameplay contexts are shadowed
//! for exactly the inputs the UI binds. Games attach extra per-screen actions
//! to that entity ([`UiInputContexts::get`]).

use bevy_camera::visibility::Visibility;
use bevy_ecs::prelude::*;
use bevy_enhanced_input::prelude::*;
use bevy_input_focus::{FocusCause, InputFocus};
use bevy_ui::GlobalZIndex;
use bevy_ui::auto_directional_navigation::AutoDirectionalNavigation;
use bevy_ui_widgets::Activate;
use sf_ui::{InputContextId, Layer, ScreenId};

use crate::ScreenStackRes;
use crate::actions::{UiAction, UiActionTag, UiBindings, spawn_actions};
use crate::nav::{UiDefaultFocus, UiFocusable, UiOwnsFocus};

/// Evaluation priority of UI contexts: above any gameplay context.
pub const UI_CONTEXT_PRIORITY: usize = 1 << 16;

/// The `bevy_enhanced_input` context of a screen input context id.
#[derive(Component, Clone, Debug)]
pub struct UiScreenInput {
    pub id: InputContextId,
}

/// Marks the root UI node of an open screen. Focusable descendants navigate
/// only while their screen is the input target.
#[derive(Component, Clone, Debug)]
pub struct UiScreenRoot(pub ScreenId);

/// On a screen root: the screen handles `Back` itself (the router then does
/// not close it).
#[derive(Component, Clone, Copy, Debug, Default)]
pub struct UiHandlesBack;

/// A UI action that started this frame, for the input target screen.
#[derive(Message, Clone, Debug, PartialEq, Eq)]
pub struct UiActionEvent {
    pub action: UiAction,
    /// The input target when the action fired.
    pub screen: ScreenId,
}

/// UI context entities by input context id, with the action set each carries.
#[derive(Resource, Debug, Default)]
pub struct UiInputContexts {
    contexts: Vec<(InputContextId, Entity, &'static [UiAction])>,
    active: Option<Entity>,
}

impl UiInputContexts {
    /// The context entity of an input context id (spawned when a screen using
    /// it first became the input target).
    pub fn get(&self, id: &str) -> Option<Entity> {
        self.contexts
            .iter()
            .find(|(c, ..)| c.as_str() == id)
            .map(|(_, e, _)| *e)
    }

    /// The active UI context entity, if any.
    pub fn active(&self) -> Option<Entity> {
        self.active
    }

    pub fn entities(&self) -> impl Iterator<Item = (Entity, &'static [UiAction])> + '_ {
        self.contexts.iter().map(|(_, e, set)| (*e, *set))
    }
}

/// On a stack change, activate the input target's context (spawning it on
/// first use) and deactivate every other UI context.
pub(crate) fn route_stack(
    mut commands: Commands,
    stack: Res<ScreenStackRes>,
    bindings: Res<UiBindings>,
    mut contexts: ResMut<UiInputContexts>,
    mut seen: Local<Option<u64>>,
) {
    let version = stack.0.version();
    if *seen == Some(version) {
        return;
    }
    *seen = Some(version);

    let target = stack.0.input_target().map(|open| {
        if let Some(entity) = contexts.get(open.input.as_str()) {
            return entity;
        }
        let set: &'static [UiAction] = if open.layer == Layer::Hud {
            &UiAction::HUD
        } else {
            &UiAction::ALL
        };
        let entity = commands
            .spawn((
                UiScreenInput {
                    id: open.input.clone(),
                },
                ContextPriority::<UiScreenInput>::new(UI_CONTEXT_PRIORITY),
                ContextActivity::<UiScreenInput>::INACTIVE,
            ))
            .id();
        spawn_actions(&mut commands, entity, set, &bindings);
        contexts.contexts.push((open.input.clone(), entity, set));
        entity
    });
    if target == contexts.active {
        return;
    }
    if let Some(old) = contexts.active {
        commands
            .entity(old)
            .insert(ContextActivity::<UiScreenInput>::INACTIVE);
    }
    if let Some(new) = target {
        commands
            .entity(new)
            .insert(ContextActivity::<UiScreenInput>::ACTIVE);
    }
    contexts.active = target;
}

/// The screen a UI entity belongs to: its nearest [`UiScreenRoot`] ancestor
/// (or itself).
fn screen_of<'a>(
    entity: Entity,
    parents: &Query<&ChildOf>,
    roots: &'a Query<(Entity, &UiScreenRoot, Has<UiHandlesBack>)>,
) -> Option<(Entity, &'a ScreenId)> {
    let mut current = entity;
    loop {
        if let Ok((root, screen, _)) = roots.get(current) {
            return Some((root, &screen.0));
        }
        current = parents.get(current).ok()?.parent();
    }
}

/// Global z-index step per layer: screen roots stack by layer (HUD under menus under modals),
/// then by position in their layer; every value stays below the message boxes and toasts
/// ([`crate::MESSAGE_BOX_Z`]).
pub const SCREEN_Z_STEP: i32 = 1 << 12;

/// The global z-index of an open screen's root.
pub fn screen_z(layer: Layer, position: usize) -> i32 {
    let layer = i32::try_from(layer.index()).unwrap_or(0) + 1;
    layer * SCREEN_Z_STEP + i32::try_from(position).unwrap_or(0).min(SCREEN_Z_STEP - 1)
}

/// Draw order follows the stack: each open screen's root gets [`screen_z`] (a later screen
/// of a layer above an earlier one, any menu above the HUD).
pub(crate) fn order_screen_roots(
    mut commands: Commands,
    stack: Res<ScreenStackRes>,
    roots: Query<(Entity, &UiScreenRoot, Option<&GlobalZIndex>)>,
    added: Query<(), Added<UiScreenRoot>>,
    mut seen: Local<Option<u64>>,
) {
    if *seen == Some(stack.0.version()) && added.is_empty() {
        return;
    }
    *seen = Some(stack.0.version());
    for (entity, root, z) in &roots {
        let place = Layer::ALL.iter().find_map(|layer| {
            let position = stack.0.layer(*layer).iter().position(|s| s.id == root.0)?;
            Some(screen_z(*layer, position))
        });
        if let Some(place) = place
            && z.is_none_or(|z| z.0 != place)
        {
            commands.entity(entity).insert(GlobalZIndex(place));
        }
    }
}

/// On a screen root: the router leaves it alive when its screen closes (the app re-pushes the
/// same root later, or despawns it itself). Without it, a closed screen's root despawns.
#[derive(Component, Clone, Copy, Debug, Default)]
pub struct UiKeepWhenClosed;

/// On a screen root the router has seen open on the stack: only such roots despawn when their
/// screen closes, so a root spawned ahead of its push (the HUD at startup, a screen pushed a
/// frame later) is never killed before it opens.
#[derive(Component, Clone, Copy, Debug, Default)]
pub struct UiSeenOpen;

/// Closing a screen despawns its root (and, Bevy's despawn being recursive, its tree): every
/// [`UiScreenRoot`] once seen open whose screen is no longer on the stack, except roots
/// marked [`UiKeepWhenClosed`]. Runs before [`hide_covered_roots`] in the same pass, so the
/// closed root goes and the root it uncovers shows in one frame.
pub(crate) fn despawn_closed_roots(
    mut commands: Commands,
    stack: Res<ScreenStackRes>,
    roots: Query<(Entity, &UiScreenRoot, Has<UiSeenOpen>), Without<UiKeepWhenClosed>>,
    added: Query<(), Added<UiScreenRoot>>,
    mut seen: Local<Option<u64>>,
) {
    if *seen == Some(stack.0.version()) && added.is_empty() {
        return;
    }
    *seen = Some(stack.0.version());
    for (entity, root, was_open) in &roots {
        let open = stack.0.is_open(root.0.as_str());
        if open && !was_open {
            commands.entity(entity).insert(UiSeenOpen);
        } else if !open && was_open {
            commands.entity(entity).despawn();
        }
    }
}

/// On a screen root the router hid because a screen above it covers it
/// (`sf_ui::Covers::Below`); its visibility is restored when it is uncovered.
#[derive(Component, Clone, Copy, Debug, Default)]
pub struct UiCovered;

/// Hides covered screens (menus.md §7): a root whose screen is covered on its
/// layer gets `Visibility::Hidden` and [`UiCovered`], and `Visibility::Inherited`
/// back once uncovered or closed. Hidden UI nodes are neither drawn nor picked
/// (`bevy_ui`'s picking backend skips nodes whose `InheritedVisibility` is
/// false), so no `Pickable` is needed. Writes only on change; a root the game
/// already hid is left alone (never marked, never shown by the router).
pub(crate) fn hide_covered_roots(
    mut commands: Commands,
    stack: Res<ScreenStackRes>,
    roots: Query<(Entity, &UiScreenRoot, Option<&Visibility>, Has<UiCovered>)>,
    added: Query<(), Added<UiScreenRoot>>,
    mut seen: Local<Option<u64>>,
) {
    if *seen == Some(stack.0.version()) && added.is_empty() {
        return;
    }
    *seen = Some(stack.0.version());
    for (entity, root, visibility, hidden) in &roots {
        let covered = stack.0.is_covered(root.0.as_str());
        if covered && !hidden && visibility != Some(&Visibility::Hidden) {
            commands
                .entity(entity)
                .insert((UiCovered, Visibility::Hidden));
        } else if !covered && hidden {
            commands
                .entity(entity)
                .remove::<UiCovered>()
                .insert(Visibility::Inherited);
        }
    }
}

/// Focus remembered per screen,/// Focus remembered per screen, restored when the screen is the input target
/// again (a closed modal returns focus to the menu button that opened it).
#[derive(Resource, Debug, Default)]
pub(crate) struct FocusMemory(Vec<(ScreenId, Entity)>);

/// A focusable or screen root spawned since the last run.
type NewNavigable = Or<(Added<UiFocusable>, Added<UiScreenRoot>, Added<UiOwnsFocus>)>;

/// Keeps directional navigation inside the input target: only its
/// [`UiFocusable`]s carry `AutoDirectionalNavigation` (Bevy's navigation is
/// layer-agnostic). Moves focus into the target when it is elsewhere.
#[allow(clippy::too_many_arguments)]
pub(crate) fn sync_navigable(
    mut commands: Commands,
    stack: Res<ScreenStackRes>,
    focusables: Query<
        (Entity, Has<AutoDirectionalNavigation>, Has<UiDefaultFocus>),
        With<UiFocusable>,
    >,
    added: Query<(), NewNavigable>,
    parents: Query<&ChildOf>,
    roots: Query<(Entity, &UiScreenRoot, Has<UiHandlesBack>)>,
    owners: Query<&UiScreenRoot, With<UiOwnsFocus>>,
    mut focus: ResMut<InputFocus>,
    mut memory: ResMut<FocusMemory>,
    mut seen: Local<Option<u64>>,
) {
    let target = stack.0.input_target().map(|s| s.id.clone());
    // A screen that owns its focus takes no part in directional navigation.
    let owned = target
        .as_ref()
        .is_some_and(|t| owners.iter().any(|r| r.0 == *t));
    let changed = *seen != Some(stack.0.version()) || !added.is_empty();
    *seen = Some(stack.0.version());

    let mut candidates = Vec::new();
    for (entity, navigable, default) in &focusables {
        let in_target = target.as_ref().is_some_and(|t| {
            screen_of(entity, &parents, &roots).is_some_and(|(_, screen)| screen == t)
        });
        if changed {
            if in_target && !owned && !navigable {
                commands
                    .entity(entity)
                    .insert(AutoDirectionalNavigation::default());
            } else if (!in_target || owned) && navigable {
                commands
                    .entity(entity)
                    .remove::<AutoDirectionalNavigation>();
            }
        }
        if in_target {
            candidates.push((entity, default));
        }
    }

    // Focus must be on a focusable of the target screen (or anywhere inside a screen that
    // owns its focus).
    let current = focus.get();
    if current.is_some_and(|f| candidates.iter().any(|(e, _)| *e == f)) {
        return;
    }
    if owned
        && current.is_some_and(|f| {
            screen_of(f, &parents, &roots).is_some_and(|(_, s)| Some(s) == target.as_ref())
        })
    {
        return;
    }
    if let Some(old) = current
        && focusables.contains(old)
        && let Some((_, screen)) = screen_of(old, &parents, &roots)
    {
        memory.0.retain(|(s, _)| s != screen);
        memory.0.push((screen.clone(), old));
    }
    let remembered = target.as_ref().and_then(|t| {
        memory
            .0
            .iter()
            .find(|(s, e)| s == t && candidates.iter().any(|(c, _)| c == e))
            .map(|(_, e)| *e)
    });
    let next = if owned {
        None
    } else {
        remembered
            .or_else(|| candidates.iter().find(|(_, d)| *d).map(|(e, _)| *e))
            .or_else(|| candidates.first().map(|(e, _)| *e))
    };
    match next {
        Some(entity) => focus.set(entity, FocusCause::Navigated),
        None if current.is_some() => focus.clear(),
        None => {}
    }
}

/// Entities activated this frame (by the widgets' own keyboard handling or by
/// us), so one press never activates twice.
#[derive(Resource, Debug, Default)]
pub(crate) struct ActivatedThisFrame(pub Vec<Entity>);

pub(crate) fn record_activation(activate: On<Activate>, mut seen: ResMut<ActivatedThisFrame>) {
    seen.0.push(activate.entity);
}

pub(crate) fn clear_activations(mut seen: ResMut<ActivatedThisFrame>) {
    seen.0.clear();
}

/// Forwards the button-like UI actions that started this frame as
/// [`UiActionEvent`]s. Confirm activates the focused entity (`Activate`, the
/// headless widgets' contract); Back closes a menu or modal input target
/// unless its root has [`UiHandlesBack`].
pub(crate) fn dispatch_actions(
    mut commands: Commands,
    mut stack: ResMut<ScreenStackRes>,
    actions: Query<(&UiActionTag, &ActionEvents)>,
    roots: Query<(Entity, &UiScreenRoot, Has<UiHandlesBack>)>,
    focus: Res<InputFocus>,
    activated: Res<ActivatedThisFrame>,
    mut out: MessageWriter<UiActionEvent>,
) {
    let Some(target) = stack.0.input_target().cloned() else {
        return;
    };
    let mut close = false;
    for (tag, events) in &actions {
        let action = tag.0;
        if !events.contains(ActionEvents::START)
            || matches!(
                action,
                UiAction::NavUp
                    | UiAction::NavDown
                    | UiAction::NavLeft
                    | UiAction::NavRight
                    | UiAction::NavStick
            )
        {
            continue;
        }
        out.write(UiActionEvent {
            action,
            screen: target.id.clone(),
        });
        match action {
            UiAction::Confirm => {
                if let Some(entity) = focus.get()
                    && !activated.0.contains(&entity)
                {
                    commands.trigger(Activate { entity });
                }
            }
            UiAction::Back if target.layer != Layer::Hud => {
                let handled = roots
                    .iter()
                    .any(|(_, r, handles)| r.0 == target.id && handles);
                close |= !handled;
            }
            _ => {}
        }
    }
    if close {
        stack.0.close(target.id.as_str());
    }
}
