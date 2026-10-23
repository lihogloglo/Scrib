//! Headless router tests: a Bevy app without window or renderer, driven by a
//! synthetic gamepad (a `Gamepad` entity whose buttons we press) and keys.

use bevy::input::ButtonState;
use bevy::input::InputPlugin;
use bevy::input::gamepad::{Gamepad, GamepadButton};
use bevy::prelude::*;
use bevy_enhanced_input::prelude::*;
use bevy_input_focus::InputFocus;
use bevy_math::CompassOctant;
use bevy_ui_widgets::Activate;
use sf_records::{RecordDb, RecordFormat, apply_record_file};
use sf_ui::{Layer, UiRegistry, register_kinds};

use crate::*;

const SCREENS: &str = r#"
[[define]]
kind = "ui.screen"
id = "hud"
value = { layer = "Hud", implementation = { Template = "hud" } }

[[define]]
kind = "ui.screen"
id = "menu"
value = { layer = "Menu", pause = "WhileOpen", implementation = { Template = "menu" } }

[[define]]
kind = "ui.screen"
id = "confirm"
value = { layer = "Modal", input = "modal", implementation = { Template = "confirm" } }
"#;

fn registry() -> UiRegistry {
    let mut db = RecordDb::new();
    register_kinds(&mut db).unwrap();
    let base = db.add_source("base", &[]);
    let outcome = apply_record_file(&mut db, base, SCREENS, RecordFormat::Toml).unwrap();
    assert_eq!(outcome.errors, []);
    UiRegistry::build(&db).0
}

#[derive(Resource, Default)]
struct Seen(Vec<UiActionEvent>);

fn collect(mut reader: MessageReader<UiActionEvent>, mut seen: ResMut<Seen>) {
    seen.0.extend(reader.read().cloned());
}

#[derive(Resource, Default)]
struct Activated(Vec<Entity>);

struct Harness {
    app: App,
    pad: Entity,
}

impl Harness {
    fn new() -> Self {
        let mut app = App::new();
        app.add_plugins((MinimalPlugins, InputPlugin, EnhancedInputPlugin, SfUiPlugin))
            .insert_resource(UiRegistryRes(registry()))
            .init_resource::<Seen>()
            .init_resource::<Activated>()
            .add_systems(Update, collect.after(UiSystems::Dispatch))
            .add_observer(|a: On<Activate>, mut seen: ResMut<Activated>| seen.0.push(a.entity));
        app.finish();
        let pad = app.world_mut().spawn(Gamepad::default()).id();
        Self { app, pad }
    }

    fn push(&mut self, id: &str) {
        let world = self.app.world_mut();
        let registry = world.remove_resource::<UiRegistryRes>().unwrap();
        world
            .resource_mut::<ScreenStackRes>()
            .0
            .push(&registry.0, id)
            .unwrap();
        world.insert_resource(registry);
    }

    fn set_button(&mut self, button: GamepadButton, down: bool) {
        let mut pad = self.app.world_mut().get_mut::<Gamepad>(self.pad).unwrap();
        if down {
            pad.digital_mut().press(button);
        } else {
            pad.digital_mut().release(button);
        }
        pad.analog_mut().set(button, if down { 1.0 } else { 0.0 });
    }

    /// Press, run a frame, release, run a frame.
    fn tap(&mut self, button: GamepadButton) {
        self.set_button(button, true);
        self.app.update();
        self.set_button(button, false);
        self.app.update();
    }

    fn take_seen(&mut self) -> Vec<(UiAction, String)> {
        std::mem::take(&mut self.app.world_mut().resource_mut::<Seen>().0)
            .into_iter()
            .map(|e| (e.action, e.screen.0))
            .collect()
    }

    fn active_context(&self) -> Option<String> {
        let world = self.app.world();
        let active = world.resource::<UiInputContexts>().active()?;
        Some(world.get::<UiScreenInput>(active)?.id.0.clone())
    }

    fn top(&self) -> Option<String> {
        let stack = &self.app.world().resource::<ScreenStackRes>().0;
        stack.input_target().map(|s| s.id.0.clone())
    }
}

#[test]
fn router_activates_the_input_target_context_only() {
    let mut h = Harness::new();
    h.app.update();
    assert_eq!(h.active_context(), None);

    h.push("hud");
    h.app.update();
    assert_eq!(h.active_context().as_deref(), Some("hud"));
    // The HUD context carries only the screen-opening actions.
    let world = h.app.world_mut();
    let hud = world.resource::<UiInputContexts>().get("hud").unwrap();
    assert_eq!(
        world.get::<Actions<UiScreenInput>>(hud).unwrap().len(),
        UiAction::HUD.len()
    );

    h.push("menu");
    h.push("confirm");
    h.app.update();
    // The modal's input context id is "modal" (record field), not the screen id.
    assert_eq!(h.active_context().as_deref(), Some("modal"));
    let world = h.app.world();
    let contexts = world.resource::<UiInputContexts>();
    for (entity, _) in contexts.entities() {
        let active = **world.get::<ContextActivity<UiScreenInput>>(entity).unwrap();
        assert_eq!(active, Some(entity) == contexts.active());
    }

    h.app
        .world_mut()
        .resource_mut::<ScreenStackRes>()
        .0
        .pop(Layer::Modal);
    h.app.update();
    assert_eq!(h.active_context().as_deref(), Some("menu"));
}

#[test]
fn gamepad_actions_reach_only_the_target_and_back_closes() {
    let mut h = Harness::new();
    h.push("hud");
    h.app.update();

    // Start on the HUD: open menu. The d-pad is not a HUD action.
    h.tap(GamepadButton::DPadDown);
    h.tap(GamepadButton::Start);
    assert_eq!(h.take_seen(), [(UiAction::OpenMenu, "hud".to_owned())]);

    // The game opens the menu while Start is still held: the menu context
    // must not see that held press (require_reset).
    h.set_button(GamepadButton::Start, true);
    h.app.update();
    h.push("menu");
    h.app.update();
    h.app.update();
    h.set_button(GamepadButton::Start, false);
    h.app.update();
    assert_eq!(
        h.take_seen(),
        [(UiAction::OpenMenu, "hud".to_owned())],
        "one press, one event"
    );
    assert_eq!(h.top().as_deref(), Some("menu"));

    h.tap(GamepadButton::RightTrigger);
    h.tap(GamepadButton::West);
    assert_eq!(
        h.take_seen(),
        [
            (UiAction::TabNext, "menu".to_owned()),
            (UiAction::Context1, "menu".to_owned())
        ]
    );

    // Modal on top: B closes it, then B closes the menu, then the HUD keeps B unbound.
    h.push("confirm");
    h.app.update();
    h.tap(GamepadButton::East);
    assert_eq!(h.take_seen(), [(UiAction::Back, "confirm".to_owned())]);
    assert_eq!(h.top().as_deref(), Some("menu"));
    h.app.update();
    h.tap(GamepadButton::East);
    assert_eq!(h.top().as_deref(), Some("hud"));
    h.app.update();
    h.tap(GamepadButton::East);
    let seen = h.take_seen();
    assert_eq!(seen, [(UiAction::Back, "menu".to_owned())]);
}

#[test]
fn rebinding_applies_to_open_contexts() {
    let mut h = Harness::new();
    h.push("hud");
    h.app.update();
    // Rebind "open menu" to Y (menu open is rebindable too).
    let mut keymap = h.app.world_mut().resource_mut::<UiKeymap>();
    let menu = UiAction::OpenMenu.id();
    keymap
        .0
        .set(&menu, Device::Gamepad, &["pad:North"])
        .unwrap();
    keymap.0.set(&menu, Device::Kbm, &["key:Tab"]).unwrap();
    h.app.update();
    h.tap(GamepadButton::Start);
    assert_eq!(h.take_seen(), []);
    h.tap(GamepadButton::North);
    assert_eq!(h.take_seen(), [(UiAction::OpenMenu, "hud".to_owned())]);
    let keymap = h.app.world().resource::<UiKeymap>();
    let label = |device, family| glyph(keymap, UiAction::OpenMenu, device, family).map(|g| g.label);
    assert_eq!(
        label(Device::Gamepad, PadFamily::Xbox).as_deref(),
        Some("Y")
    );
    assert_eq!(
        label(Device::Gamepad, PadFamily::PlayStation).as_deref(),
        Some("Triangle")
    );
    assert_eq!(label(Device::Kbm, PadFamily::Xbox).as_deref(), Some("Tab"));
}

#[test]
fn focus_follows_the_target_and_confirm_activates() {
    let mut h = Harness::new();
    let world = h.app.world_mut();
    let menu = world.spawn(UiScreenRoot("menu".into())).id();
    let first = world.spawn((UiFocusable, ChildOf(menu))).id();
    let second = world
        .spawn((UiFocusable, UiDefaultFocus, ChildOf(menu)))
        .id();
    let modal = world.spawn(UiScreenRoot("confirm".into())).id();
    let ok = world.spawn((UiFocusable, ChildOf(modal))).id();

    h.push("menu");
    h.app.update();
    let focus = |h: &Harness| h.app.world().resource::<InputFocus>().get();
    assert_eq!(focus(&h), Some(second), "default focus");
    let navigable = |h: &Harness, e| {
        h.app
            .world()
            .get::<bevy_ui::auto_directional_navigation::AutoDirectionalNavigation>(e)
            .is_some()
    };
    assert!(navigable(&h, first) && navigable(&h, second) && !navigable(&h, ok));

    h.app
        .world_mut()
        .resource_mut::<InputFocus>()
        .set(first, bevy_input_focus::FocusCause::Navigated);
    h.push("confirm");
    h.app.update();
    assert_eq!(focus(&h), Some(ok));
    assert!(!navigable(&h, first) && navigable(&h, ok));

    h.tap(GamepadButton::South);
    assert_eq!(h.app.world().resource::<Activated>().0, [ok]);

    // Closing the modal restores the menu's remembered focus.
    h.app
        .world_mut()
        .resource_mut::<ScreenStackRes>()
        .0
        .close("confirm");
    h.app.update();
    assert_eq!(focus(&h), Some(first));
}

#[derive(Resource, Default)]
struct NavSeen(Vec<UiNavEvent>);

#[test]
fn a_screen_owning_its_focus_gets_directions_instead() {
    let mut h = Harness::new();
    h.app.init_resource::<NavSeen>().add_systems(
        Update,
        (|mut r: MessageReader<UiNavEvent>, mut seen: ResMut<NavSeen>| {
            seen.0.extend(r.read().cloned())
        })
        .after(UiSystems::Dispatch),
    );
    let world = h.app.world_mut();
    let menu = world.spawn(UiScreenRoot("menu".into())).id();
    let button = world.spawn((UiFocusable, ChildOf(menu))).id();
    let modal = world
        .spawn((UiScreenRoot("confirm".into()), UiOwnsFocus))
        .id();
    let cell = world.spawn((UiFocusable, ChildOf(modal))).id();
    let focus = |h: &Harness| h.app.world().resource::<InputFocus>().get();
    let navigable = |h: &Harness, e| {
        h.app
            .world()
            .get::<bevy_ui::auto_directional_navigation::AutoDirectionalNavigation>(e)
            .is_some()
    };

    h.push("menu");
    h.app.update();
    assert_eq!(focus(&h), Some(button));
    // The owning screen on top: no directional navigation, no focus picked, the menu's cleared.
    h.push("confirm");
    h.app.update();
    assert_eq!(focus(&h), None);
    assert!(!navigable(&h, cell) && !navigable(&h, button));
    h.tap(GamepadButton::DPadDown);
    let seen = std::mem::take(&mut h.app.world_mut().resource_mut::<NavSeen>().0);
    assert_eq!(
        seen,
        [UiNavEvent {
            direction: CompassOctant::South,
            screen: "confirm".into()
        }]
    );
    // Focus the screen sets itself stays, and confirm activates it.
    h.app
        .world_mut()
        .resource_mut::<InputFocus>()
        .set(cell, bevy_input_focus::FocusCause::Navigated);
    h.app.update();
    assert_eq!(focus(&h), Some(cell));
    h.tap(GamepadButton::South);
    assert_eq!(h.app.world().resource::<Activated>().0, [cell]);
    // Closed: the menu gets its focus back.
    h.app
        .world_mut()
        .resource_mut::<ScreenStackRes>()
        .0
        .close("confirm");
    h.app.update();
    assert_eq!(focus(&h), Some(button));
    assert!(navigable(&h, button));
}

#[test]
fn held_direction_repeats_after_delay() {
    let mut r = NavRepeat::default();
    let s = Some(CompassOctant::South);
    assert_eq!(r.update(s, 0.016), s, "fires at once");
    assert_eq!(r.update(s, 0.2), None);
    assert_eq!(r.update(s, 0.21), s, "after the delay");
    assert_eq!(r.update(s, 0.05), None);
    assert_eq!(r.update(s, 0.05), s, "then every interval");
    assert_eq!(r.update(None, 0.016), None);
    assert_eq!(r.update(s, 0.016), s, "a new press fires again");

    assert_eq!(
        held_direction((0, 1), Vec2::ZERO),
        Some(CompassOctant::North)
    );
    assert_eq!(
        held_direction((0, 0), Vec2::new(0.2, -0.9)),
        Some(CompassOctant::South)
    );
    assert_eq!(
        held_direction((0, 0), Vec2::new(0.3, 0.1)),
        None,
        "dead zone"
    );
}

#[test]
fn last_used_device_switches_input_mode() {
    let mut h = Harness::new();
    h.app.update();
    assert_eq!(h.app.world().resource::<InputMode>().0, Device::Kbm);
    h.set_button(GamepadButton::South, true);
    h.app.update();
    assert_eq!(h.app.world().resource::<InputMode>().0, Device::Gamepad);
    // A still-held pad button is not new use: the key press below wins.
    h.app
        .world_mut()
        .write_message(bevy::input::keyboard::KeyboardInput {
            key_code: KeyCode::KeyA,
            logical_key: bevy::input::keyboard::Key::Character("a".into()),
            state: ButtonState::Pressed,
            text: None,
            repeat: false,
            window: Entity::PLACEHOLDER,
        });
    h.app.update();
    assert_eq!(h.app.world().resource::<InputMode>().0, Device::Kbm);
}

#[derive(Resource, Default)]
struct Rebinds(Vec<sf_ui::RebindEvent>);

#[test]
fn capture_rebinds_without_firing_ui_actions() {
    use sf_ui::{RebindEvent, Resolution, SlotRef};
    let mut h = Harness::new();
    h.app.init_resource::<Rebinds>().add_systems(
        Update,
        |mut r: MessageReader<UiRebindEvent>, mut seen: ResMut<Rebinds>| {
            seen.0.extend(r.read().map(|e| e.0.clone()));
        },
    );
    let take_rebinds =
        |h: &mut Harness| std::mem::take(&mut h.app.world_mut().resource_mut::<Rebinds>().0);
    let start = |h: &mut Harness, action: UiAction| {
        let world = h.app.world_mut();
        let keymap = world.resource::<UiKeymap>().clone();
        let slot = SlotRef {
            action: action.id(),
            device: Device::Gamepad,
            slot: 0,
        };
        world
            .resource_mut::<UiRebinder>()
            .start(&keymap, slot)
            .unwrap();
    };
    h.push("hud");
    h.push("menu");
    h.app.update();

    // The player picks the "open menu" row with A; capture starts while A is held.
    h.set_button(GamepadButton::South, true);
    h.app.update();
    start(&mut h, UiAction::OpenMenu);
    h.take_seen();
    h.set_button(GamepadButton::South, false);
    h.app.update();
    h.tap(GamepadButton::LeftTrigger2);
    assert_eq!(
        take_rebinds(&mut h),
        [RebindEvent::Applied {
            slot: SlotRef {
                action: UiAction::OpenMenu.id(),
                device: Device::Gamepad,
                slot: 0
            },
            input: "pad:LeftTrigger2".into()
        }]
    );
    assert_eq!(
        h.take_seen(),
        [],
        "the A that opened the capture didn't land"
    );

    // Capturing B for context 1 collides with back; the menu stays open.
    start(&mut h, UiAction::Context1);
    h.app.update();
    h.set_button(GamepadButton::East, true);
    h.app.update();
    assert!(
        matches!(&take_rebinds(&mut h)[..], [RebindEvent::Conflict(p)] if p.conflicts[0].other == UiAction::Back.id())
    );
    assert!(
        h.app.world().resource::<UiRebinder>().0.blocks_ui(),
        "B still held"
    );
    h.set_button(GamepadButton::East, false);
    h.app.update();
    assert_eq!(h.top().as_deref(), Some("menu"));
    assert_eq!(h.take_seen(), []);
    let world = h.app.world_mut();
    world.resource_scope(|world, mut rebinder: Mut<UiRebinder>| {
        let mut keymap = world.resource_mut::<UiKeymap>();
        assert!(matches!(
            rebinder.resolve(&mut keymap, Resolution::Swap),
            Some(RebindEvent::Applied { .. })
        ));
    });
    h.app.update();
    let keymap = &h.app.world().resource::<UiKeymap>().0;
    assert_eq!(
        keymap.bound(&UiAction::Context1.id(), Device::Gamepad),
        ["pad:East"]
    );
    assert_eq!(
        keymap.bound(&UiAction::Back.id(), Device::Gamepad),
        ["pad:West"]
    );

    // The new bindings drive the engine: X backs out, the left trigger opens the menu.
    h.tap(GamepadButton::LeftTrigger2);
    h.tap(GamepadButton::Start);
    h.tap(GamepadButton::West);
    assert_eq!(
        h.take_seen(),
        [
            (UiAction::OpenMenu, "menu".to_owned()),
            (UiAction::Back, "menu".to_owned())
        ]
    );
    assert_eq!(h.top().as_deref(), Some("hud"));
}
