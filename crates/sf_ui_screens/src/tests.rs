//! Headless screen tests: a Bevy app with UI layout and text but no window or GPU (a camera
//! with a fixed render target), a synthetic gamepad, and the demo data. They check that
//! view-model changes reach the entity tree without rebuilding it, that focus and gamepad
//! navigation follow the view-models, and that an open screen eats game input.

use std::time::Duration;

use bevy::camera::{ComputedCameraValues, RenderTargetInfo};
use bevy::input::InputPlugin;
use bevy::input::gamepad::{Gamepad, GamepadButton};
use bevy::picking::{InteractionPlugin, PickingPlugin};
use bevy::prelude::*;
use bevy::text::{TextBackgroundColor, TextPlugin, TextSpan};
use bevy::time::TimeUpdateStrategy;
use bevy::ui::UiPlugin;
use bevy::ui::{ComputedNode, UiGlobalTransform};
use bevy_enhanced_input::prelude::*;
use bevy_input_focus::InputFocus;
use bevy_ui_widgets::Activate;
use sf_rpg_ui::{Alphabet, DialogueFocus, DialogueMode, JournalScreen, JournalSeen};
use sf_ui_bevy::{
    ScreenStackRes, SfUiPlugin, UiKeymap, UiRebinder, UiRegistryRes, UiScreenRoot, UiTextPlugin,
};

use crate::demo::{self, DemoInventory, DemoWorld};
use crate::dialogue::DialogueParts;
use crate::hud::HudParts;
use crate::inventory::InventoryParts;
use crate::journal::{JournalFragment, JournalPanel, JournalParts};
use crate::keybinds::KeybindsParts;
use crate::*;

const TARGET: UVec2 = UVec2::new(1600, 1000);

struct Ui {
    app: App,
    pad: Entity,
}

impl Ui {
    fn new() -> Self {
        Self::with(|_| {})
    }

    fn with(extra: impl FnOnce(&mut App)) -> Self {
        let mut app = App::new();
        app.add_plugins((
            MinimalPlugins,
            TransformPlugin,
            AssetPlugin::default(),
            ImagePlugin::default(),
            bevy::image::TextureAtlasPlugin,
            TextPlugin,
            UiPlugin,
            InputPlugin,
            PickingPlugin,
            InteractionPlugin,
            EnhancedInputPlugin,
            SfUiPlugin,
            UiTextPlugin,
        ))
        .add_plugins((SfScreensPlugin, JournalUiPlugin::<DemoWorld>::default()))
        .insert_resource(UiRegistryRes(demo::registry()))
        .insert_resource(TimeUpdateStrategy::ManualDuration(Duration::from_millis(
            16,
        )))
        .add_systems(
            Update,
            (demo::host_dialogue, demo::host_inventory).after(ScreenSystems::Input),
        );
        extra(&mut app);
        app.world_mut().spawn((
            Camera2d,
            Camera {
                computed: ComputedCameraValues {
                    target_info: Some(RenderTargetInfo {
                        physical_size: TARGET,
                        scale_factor: 1.0,
                    }),
                    ..default()
                },
                viewport: Some(bevy::camera::Viewport {
                    physical_size: TARGET,
                    ..default()
                }),
                ..default()
            },
        ));
        app.finish();
        let pad = app.world_mut().spawn(Gamepad::default()).id();
        app.update();
        Self { app, pad }
    }

    fn world(&self) -> &World {
        self.app.world()
    }

    fn world_mut(&mut self) -> &mut World {
        self.app.world_mut()
    }

    /// Spawns a screen root with `bundle` and opens it.
    fn open(&mut self, id: &str, bundle: impl Bundle) -> Entity {
        let root = self
            .world_mut()
            .spawn((UiScreenRoot(id.into()), bundle))
            .id();
        let world = self.world_mut();
        let registry = world.remove_resource::<UiRegistryRes>().unwrap();
        world
            .resource_mut::<ScreenStackRes>()
            .0
            .push(&registry.0, id)
            .unwrap();
        world.insert_resource(registry);
        self.frames(3);
        root
    }

    fn frames(&mut self, n: usize) {
        for _ in 0..n {
            self.app.update();
        }
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

    /// Press, a frame, release, two frames (the host and the sync run after the press).
    fn tap(&mut self, button: GamepadButton) {
        self.set_button(button, true);
        self.app.update();
        self.set_button(button, false);
        self.frames(2);
    }

    fn focus(&self) -> Option<Entity> {
        self.world().resource::<InputFocus>().get()
    }

    fn text_of(&self, entity: Entity) -> String {
        let mut out = self
            .world()
            .get::<Text>(entity)
            .map(|t| t.0.clone())
            .unwrap_or_default();
        if let Some(children) = self.world().get::<Children>(entity) {
            for child in children {
                if let Some(span) = self.world().get::<TextSpan>(*child) {
                    out.push_str(&span.0);
                } else {
                    out.push_str(&self.text_of(*child));
                }
            }
        }
        out
    }

    fn class_of(&self, entity: Entity) -> String {
        self.world()
            .get::<sf_ui_css::UiClass>(entity)
            .map(|c| c.0.clone())
            .unwrap_or_default()
    }
}

// ---------------------------------------------------------------- keyed reconciliation

#[derive(Resource, Default)]
struct Keys(Vec<u32>, Option<KeyedChildren<u32>>, Vec<KeyedSync>);

fn sync_keys(mut commands: Commands, mut keys: ResMut<Keys>) {
    let keys = &mut *keys;
    let Some(list) = keys.1.as_mut() else {
        return;
    };
    let report = list.sync(&mut commands, keys.0.clone(), |commands, k| {
        commands.spawn(Name::new(format!("row {k}"))).id()
    });
    keys.2.push(report);
}

#[test]
fn keyed_children_add_remove_and_reorder_without_rebuilding() {
    let mut app = App::new();
    app.init_resource::<Keys>().add_systems(Update, sync_keys);
    let parent = app.world_mut().spawn_empty().id();
    app.world_mut().resource_mut::<Keys>().1 = Some(KeyedChildren::new(parent));
    let children = |app: &App| -> Vec<Entity> {
        app.world()
            .get::<Children>(parent)
            .map(|c| c.to_vec())
            .unwrap_or_default()
    };
    let set = |app: &mut App, keys: &[u32]| {
        app.world_mut().resource_mut::<Keys>().0 = keys.to_vec();
        app.update();
        app.world_mut().resource_mut::<Keys>().2.pop().unwrap()
    };

    let r = set(&mut app, &[1, 2, 3]);
    assert_eq!((r.spawned, r.despawned, r.reordered), (3, 0, false));
    let first = children(&app);
    // Same keys: nothing happens.
    assert!(set(&mut app, &[1, 2, 3]).is_noop());
    // Insert in the middle: one spawn, the order is rewritten, kept rows keep their entity.
    let r = set(&mut app, &[1, 4, 2, 3]);
    assert_eq!((r.spawned, r.despawned, r.reordered), (1, 0, true));
    let now = children(&app);
    assert_eq!(now.len(), 4);
    assert_eq!((now[0], now[2], now[3]), (first[0], first[1], first[2]));
    // Remove from the middle: one despawn, no reorder needed.
    let r = set(&mut app, &[1, 2, 3]);
    assert_eq!((r.spawned, r.despawned, r.reordered), (0, 1, false));
    assert_eq!(children(&app), first);
    // Append at the end: no reorder.
    let r = set(&mut app, &[1, 2, 3, 5]);
    assert_eq!((r.spawned, r.reordered), (1, false));
    // Reverse: only a reorder.
    let r = set(&mut app, &[5, 3, 2, 1]);
    assert_eq!((r.spawned, r.despawned, r.reordered), (0, 0, true));
    let now = children(&app);
    assert_eq!(&now[1..], &[first[2], first[1], first[0]]);
}

// ---------------------------------------------------------------- dialogue

fn open_dialogue(ui: &mut Ui) -> Entity {
    let mut world = demo::world(6, 12);
    let (talk, view) = demo::open_dialogue(&mut world.world);
    ui.world_mut().insert_resource(world);
    ui.world_mut().insert_resource(talk);
    ui.open("dialogue", DialogueScreen::new(view))
}

fn dialogue(ui: &Ui, root: Entity) -> (&DialogueScreen, &DialogueParts) {
    (
        ui.world().get::<DialogueScreen>(root).unwrap(),
        ui.world().get::<DialogueParts>(root).unwrap(),
    )
}

#[test]
fn dialogue_syncs_entries_topics_and_choices_by_key() {
    let mut ui = Ui::new();
    let root = open_dialogue(&mut ui);
    let (screen, parts) = dialogue(&ui, root);
    // The greeting, with links on topic names.
    let entries: Vec<_> = parts.entries().collect();
    assert_eq!(entries.len(), 1);
    let greeting = entries[0].1;
    assert!(ui.text_of(greeting).starts_with("Rain again."));
    let link = (0..8)
        .find_map(|run| parts.link(entries[0].0, run))
        .expect("a link span");
    assert!(ui.class_of(link).contains("sf-dialogue-link"));
    // Topics sorted by name, one row each.
    let topics: Vec<String> = parts.topics().map(|(t, _)| t.as_str().to_owned()).collect();
    let mut sorted = topics.clone();
    sorted.sort();
    assert_eq!(topics, sorted);
    assert_eq!(topics.len(), screen.view.topics().len());
    let rows: Vec<Entity> = parts.topics().map(|(_, e)| e).collect();

    // Ask "harbor" with the pointer path (Activate on its row).
    let harbor = parts.topic(&"harbor".into()).unwrap();
    ui.world_mut().trigger(Activate { entity: harbor });
    ui.frames(2);
    let (screen, parts) = dialogue(&ui, root);
    let entries: Vec<_> = parts.entries().collect();
    assert_eq!(entries.len(), 2, "one new line");
    assert_eq!(entries[0].1, greeting, "the old line kept its entity");
    assert!(ui.text_of(entries[1].1).contains("About harbor"));
    assert_eq!(
        parts.topics().map(|(_, e)| e).collect::<Vec<_>>(),
        rows,
        "topic rows kept"
    );
    assert!(!ui.class_of(harbor).contains("new"), "asked: no longer new");
    assert_eq!(screen.view.mode(), DialogueMode::Talking);

    // "ferries" offers choices: numbered buttons; the topic list locks.
    let ferries = parts.topic(&"ferries".into()).unwrap();
    ui.world_mut().trigger(Activate { entity: ferries });
    ui.frames(2);
    let (screen, parts) = dialogue(&ui, root);
    assert_eq!(screen.view.mode(), DialogueMode::Choosing);
    let yes = parts.choice(1).unwrap();
    assert_eq!(ui.text_of(yes), "2. Yes, I will carry it");
    assert!(
        ui.world()
            .get::<bevy_ui::InteractionDisabled>(harbor)
            .is_some()
    );
    assert!(ui.class_of(root).contains("choosing"));
    // Back does nothing while choosing.
    ui.tap(GamepadButton::East);
    assert_eq!(dialogue(&ui, root).0.view.mode(), DialogueMode::Choosing);
    // Confirm on the focused choice (focus mirrors into InputFocus).
    let no = dialogue(&ui, root).1.choice(0).unwrap();
    assert_eq!(ui.focus(), Some(no));
    ui.tap(GamepadButton::DPadDown);
    assert_eq!(ui.focus(), Some(yes));
    ui.tap(GamepadButton::South);
    let (screen, parts) = dialogue(&ui, root);
    assert_eq!(screen.view.mode(), DialogueMode::Talking);
    assert!(parts.choice(0).is_none(), "choices gone");
    let kinds: Vec<String> = parts.entries().map(|(_, e)| ui.class_of(e)).collect();
    assert!(kinds.iter().any(|c| c.contains("chosen")), "{kinds:?}");
    assert!(kinds.iter().any(|c| c.contains("notice")), "{kinds:?}");
}

#[test]
fn dialogue_gamepad_focus_and_link_cursor() {
    let mut ui = Ui::new();
    let root = open_dialogue(&mut ui);
    let first_row = dialogue(&ui, root).1.topics().next().unwrap().1;
    assert_eq!(
        ui.focus(),
        Some(first_row),
        "focus starts on the topic list"
    );
    ui.tap(GamepadButton::DPadDown);
    let second_row = dialogue(&ui, root).1.topics().nth(1).unwrap().1;
    assert_eq!(ui.focus(), Some(second_row));

    // Left: into the text, the link cursor lights a span; InputFocus is cleared.
    ui.tap(GamepadButton::DPadLeft);
    let (screen, parts) = dialogue(&ui, root);
    let Some(DialogueFocus::Link { entry, run }) = screen.view.focus().cloned() else {
        panic!("link focus expected, got {:?}", screen.view.focus());
    };
    let span = parts.link(entry, run).unwrap();
    assert!(ui.world().get::<TextBackgroundColor>(span).is_some());
    assert_eq!(ui.focus(), None);
    // Confirm follows the link: a new line, the cursor's background moves on.
    let before = parts.entries().count();
    ui.tap(GamepadButton::South);
    let parts = dialogue(&ui, root).1;
    assert_eq!(parts.entries().count(), before + 1);
    // Right: back to the remembered row.
    ui.tap(GamepadButton::DPadRight);
    assert!(matches!(
        dialogue(&ui, root).0.view.focus(),
        Some(DialogueFocus::Topic(_))
    ));
    assert!(ui.focus().is_some());
    // "wells" forces goodbye; Back says goodbye and the screen closes.
    let wells = dialogue(&ui, root).1.topic(&"wells".into()).unwrap();
    ui.world_mut().trigger(Activate { entity: wells });
    ui.frames(2);
    assert_eq!(dialogue(&ui, root).0.view.mode(), DialogueMode::GoodbyeOnly);
    ui.tap(GamepadButton::East);
    assert!(
        ui.world()
            .resource::<ScreenStackRes>()
            .0
            .input_target()
            .is_none(),
        "closed"
    );
}

// ---------------------------------------------------------------- input routing

#[derive(Component)]
struct Gameplay;

#[derive(InputAction)]
#[action_output(bool)]
struct Jump;

#[derive(Resource, Default)]
struct Jumps(u32);

fn count_jumps(_: On<Start<Jump>>, mut jumps: ResMut<Jumps>) {
    jumps.0 += 1;
}

#[test]
fn an_open_screen_eats_game_input() {
    let mut ui = Ui::with(|app| {
        app.add_input_context::<Gameplay>()
            .init_resource::<Jumps>()
            .add_observer(count_jumps);
    });
    let context = ui.world_mut().spawn(Gameplay).id();
    let jump = ui
        .world_mut()
        .spawn((ActionOf::<Gameplay>::new(context), Action::<Jump>::new()))
        .id();
    ui.world_mut()
        .spawn((Binding::from(GamepadButton::South), BindingOf(jump)));
    ui.world_mut().insert_resource(demo::hud());
    ui.open("hud", HudScreen);
    ui.tap(GamepadButton::South);
    assert_eq!(
        ui.world().resource::<Jumps>().0,
        1,
        "the HUD lets A through"
    );
    let root = open_dialogue(&mut ui);
    ui.tap(GamepadButton::DPadDown);
    let before = dialogue(&ui, root).1.entries().count();
    ui.tap(GamepadButton::South);
    assert_eq!(
        ui.world().resource::<Jumps>().0,
        1,
        "A went to the dialogue"
    );
    assert_eq!(dialogue(&ui, root).1.entries().count(), before + 1);
    // Closed again: gameplay gets A back.
    ui.world_mut()
        .resource_mut::<ScreenStackRes>()
        .0
        .close("dialogue");
    ui.frames(2);
    ui.tap(GamepadButton::South);
    assert_eq!(ui.world().resource::<Jumps>().0, 2);
}

// ---------------------------------------------------------------- inventory

fn open_inventory(ui: &mut Ui, stacks: usize) -> Entity {
    let inventory = DemoInventory::new(stacks);
    let view = inventory.view();
    ui.world_mut().insert_resource(inventory);
    let root = ui.open("inventory", InventoryScreen::new(view));
    ui.frames(3);
    root
}

fn inventory(ui: &Ui, root: Entity) -> (&InventoryScreen, &InventoryParts) {
    (
        ui.world().get::<InventoryScreen>(root).unwrap(),
        ui.world().get::<InventoryParts>(root).unwrap(),
    )
}

#[test]
fn inventory_grid_stays_virtual_and_moves_focus() {
    let mut ui = Ui::new();
    let root = open_inventory(&mut ui, 1000);
    let (screen, parts) = inventory(&ui, root);
    let columns = screen.grid.columns();
    // 560 px panel minus padding and scrollbar: several columns of 68 px.
    assert!(columns >= 5, "columns {columns}");
    let cells = parts.cells().count();
    assert!(cells < 120, "{cells} cells for 1000 stacks");
    let pool = parts.pool_len();
    let first = screen
        .view
        .focus()
        .expect("focus starts on the first stack");
    assert_eq!(ui.focus(), parts.cell_of(first));

    // Right, then down a row.
    ui.tap(GamepadButton::DPadRight);
    ui.tap(GamepadButton::DPadDown);
    let visible = visible(&mut ui, root);
    let (screen, parts) = inventory(&ui, root);
    let focused = screen.view.focus().unwrap();
    assert_eq!(
        visible.iter().position(|k| *k == focused),
        Some(columns + 1)
    );
    assert_eq!(ui.focus(), parts.cell_of(focused));
    assert!(
        ui.class_of(parts.cell_of(focused).unwrap())
            .contains("focused")
    );

    // Hold down: the window scrolls, the pool doesn't grow.
    ui.set_button(GamepadButton::DPadDown, true);
    ui.frames(90);
    ui.set_button(GamepadButton::DPadDown, false);
    ui.frames(2);
    let (screen, parts) = inventory(&ui, root);
    assert!(screen.grid.scroll() > 0.0);
    assert!(
        // Scrolled off the top, the window also keeps its overscan rows above (2).
        parts.pool_len() <= pool + 2,
        "pool {} > {pool} + 2",
        parts.pool_len()
    );
    let focused = screen.view.focus().unwrap();
    assert_eq!(
        ui.focus(),
        parts.cell_of(focused),
        "focused cell in the window"
    );

    // RB: next tab (gear); the tab button shows selected.
    ui.tap(GamepadButton::RightTrigger);
    let (screen, parts) = inventory(&ui, root);
    assert_eq!(screen.view.tab(), 1);
    assert!(ui.class_of(parts.tab(1).unwrap()).contains("selected"));
    assert!(ui.text_of(parts.tab(1).unwrap()).starts_with("Gear ("));
}

#[test]
fn inventory_split_dialog_and_pick_up_merge() {
    let mut ui = Ui::new();
    let root = open_inventory(&mut ui, 12);
    // Focus a bread stack (5 pieces): index 2.
    ui.tap(GamepadButton::DPadRight);
    ui.tap(GamepadButton::DPadRight);
    let (screen, _) = inventory(&ui, root);
    let bread = screen.view.focus().unwrap();
    assert_eq!(screen.view.row(bread).unwrap().stack.count, 5);
    let before = screen.view.rows().len();
    // X: split dialog (the host answers with the picker); right adds one; A confirms.
    ui.tap(GamepadButton::West);
    let (screen, parts) = inventory(&ui, root);
    let (_, picker) = screen.split().expect("split dialog open");
    assert_eq!(picker.value(), 2);
    let dialog = parts.split_dialog();
    assert_eq!(
        ui.world().get::<Node>(dialog).unwrap().display,
        Display::Flex
    );
    ui.tap(GamepadButton::DPadRight);
    assert_eq!(inventory(&ui, root).0.split().unwrap().1.value(), 3);
    ui.tap(GamepadButton::South);
    let (screen, parts) = inventory(&ui, root);
    assert!(screen.split().is_none());
    assert_eq!(screen.view.rows().len(), before + 1, "a new stack");
    let new = screen.view.focus().unwrap();
    assert_ne!(new, bread, "focus on the split-off stack");
    assert_eq!(screen.view.row(new).unwrap().stack.count, 3);
    assert_eq!(
        ui.world().get::<Node>(dialog).unwrap().display,
        Display::None
    );
    let _ = parts;

    // Y picks the new stack up, then A on the source drops it there: merged back.
    ui.tap(GamepadButton::North);
    assert_eq!(inventory(&ui, root).0.held(), Some(new));
    let source_cell = inventory(&ui, root).1.cell_of(bread).unwrap();
    ui.world_mut().trigger(Activate {
        entity: source_cell,
    });
    ui.frames(2);
    let (screen, _) = inventory(&ui, root);
    assert_eq!(screen.view.rows().len(), before);
    assert_eq!(screen.view.row(bread).unwrap().stack.count, 5);
    assert_eq!(screen.held(), None);
}

#[test]
fn inventory_tooltip_follows_focus_with_comparison() {
    let mut ui = Ui::new();
    let root = open_inventory(&mut ui, 12);
    // Index 4 is a second blade: compared with the worn one.
    for _ in 0..4 {
        ui.tap(GamepadButton::DPadRight);
    }
    ui.frames(2);
    let (screen, parts) = inventory(&ui, root);
    let focused = screen.view.focus().unwrap();
    let tooltip = parts.tooltip();
    // Beside the focused cell: same top, just right of it (or left at the screen's edge).
    let rect = |e| {
        let node = ui.world().get::<ComputedNode>(e).unwrap();
        let at = ui.world().get::<UiGlobalTransform>(e).unwrap();
        (
            at.translation - node.size() / 2.0,
            at.translation + node.size() / 2.0,
        )
    };
    let (cell_min, cell_max) = rect(parts.cell_of(focused).unwrap());
    let (tip_min, tip_max) = rect(tooltip);
    assert!(
        (tip_min.y - cell_min.y).abs() < 1.0,
        "{tip_min} vs {cell_min}"
    );
    assert!(
        (tip_min.x - cell_max.x - 4.0).abs() < 1.0 || (cell_min.x - tip_max.x - 4.0).abs() < 1.0,
        "{tip_min}..{tip_max} beside {cell_min}..{cell_max}"
    );
    assert!(screen.compare.is_some());
    let text = ui.text_of(tooltip);
    assert!(text.contains("blade 4"), "{text}");
    assert!(text.contains("damage"), "{text}");
}

fn visible(ui: &mut Ui, root: Entity) -> Vec<sf_rpg::StackId> {
    let mut screen = ui.world_mut().get_mut::<InventoryScreen>(root).unwrap();
    screen.view.visible().to_vec()
}

// ---------------------------------------------------------------- journal

fn open_journal(ui: &mut Ui) -> Entity {
    ui.world_mut().insert_resource(demo::world(14, 30));
    let screen = JournalScreen::new(Alphabet::default(), 2, JournalSeen::default());
    let root = ui.open("journal", JournalUi::new(screen));
    ui.frames(3);
    root
}

fn journal(ui: &Ui, root: Entity) -> (&JournalUi, &JournalParts) {
    (
        ui.world().get::<JournalUi>(root).unwrap(),
        ui.world().get::<JournalParts>(root).unwrap(),
    )
}

fn fragments(ui: &mut Ui) -> Vec<(Entity, usize)> {
    let mut q = ui.world_mut().query::<(Entity, &JournalFragment)>();
    q.iter(ui.world()).map(|(e, f)| (e, f.0)).collect()
}

#[derive(Resource, Default)]
struct SeenEvents(usize);

#[test]
fn journal_paginates_on_real_layout_and_turns_pages() {
    let mut ui = Ui::new();
    ui.app.init_resource::<SeenEvents>().add_systems(
        Update,
        |mut r: MessageReader<JournalSeenChanged>, mut n: ResMut<SeenEvents>| {
            n.0 += r.read().count();
        },
    );
    let root = open_journal(&mut ui);
    let (ui_state, _) = journal(&ui, root);
    let pages = ui_state.screen.page_count();
    assert!(
        pages >= 4,
        "14 long entries make several pages, got {pages}"
    );
    // The journal opens at its last spread (newest entries).
    assert_eq!(
        ui_state.screen.spread_index() + 1,
        ui_state.screen.spread_count()
    );
    let drawn = fragments(&mut ui);
    assert!(!drawn.is_empty());
    // Every drawn slice fits its page body (the measure is the drawing engine's).
    let (_, parts) = journal(&ui, root);
    for body in parts.bodies() {
        let body_node = ui.world().get::<ComputedNode>(body).unwrap();
        let mut content = 0.0;
        for child in ui.world().get::<Children>(body).into_iter().flatten() {
            content += ui.world().get::<ComputedNode>(*child).unwrap().size().y;
        }
        assert!(
            content <= body_node.size().y + 1.0,
            "{content} > {}",
            body_node.size().y
        );
    }

    // LB: previous spread; the new markers of the spread left are cleared (an event out).
    ui.tap(GamepadButton::LeftTrigger);
    let (state, _) = journal(&ui, root);
    assert_eq!(state.screen.spread_index() + 2, state.screen.spread_count());
    assert_eq!(ui.world().resource::<SeenEvents>().0, 1);
    assert!(
        fragments(&mut ui)
            .iter()
            .all(|(e, _)| !drawn.iter().any(|(d, _)| d == e))
    );

    // A smaller page re-paginates and keeps the place (same first block).
    let before = journal(&ui, root).0.screen.visible_pages()[0].slices[0].block;
    let page = ui
        .world()
        .get::<ChildOf>(journal(&ui, root).1.bodies()[0])
        .unwrap()
        .parent();
    ui.world_mut().get_mut::<Node>(page).unwrap().height = Val::Px(380.0);
    ui.frames(3);
    let (state, _) = journal(&ui, root);
    assert!(state.screen.page_count() > pages, "more, shorter pages");
    // The block that started the spread is still on screen.
    let visible: Vec<usize> = state
        .screen
        .visible_pages()
        .iter()
        .flat_map(|p| p.slices.iter().map(|s| s.block))
        .collect();
    assert!(
        visible.contains(&before),
        "place kept: {before} not in {visible:?}"
    );
}

#[test]
fn journal_indexes_open_quests_and_topics() {
    let mut ui = Ui::new();
    let root = open_journal(&mut ui);
    let (state, parts) = journal(&ui, root);
    assert_eq!(state.panel(), JournalPanel::Quests);
    // Active quests: two of three (one finished).
    let quests: Vec<_> = parts.rows().map(|(k, e)| (k.clone(), e)).collect();
    assert_eq!(quests.len(), 2, "{quests:?}");
    let (quest, row) = quests[0].clone();
    ui.world_mut().trigger(Activate { entity: row });
    ui.frames(2);
    let (state, parts) = journal(&ui, root);
    assert_eq!(
        state.screen.content(),
        &sf_rpg_ui::BookContent::Quest(quest.clone())
    );
    assert!(ui.class_of(parts.row(&quest).unwrap()).contains("selected"));
    assert_eq!(parts.row(&quest), Some(row), "row kept");

    // Context1 (X): the topic index; letter tabs, empty ones disabled.
    ui.tap(GamepadButton::West);
    let (state, parts) = journal(&ui, root);
    assert_eq!(state.panel(), JournalPanel::Topics);
    assert_eq!(parts.rows().count(), 30);
    let z = parts.letter("Z").unwrap();
    assert!(ui.world().get::<bevy_ui::InteractionDisabled>(z).is_some());
    let r = parts.letter("R").unwrap();
    ui.world_mut().trigger(Activate { entity: r });
    ui.frames(2);
    let (_, parts) = journal(&ui, root);
    let rows: Vec<String> = parts.rows().map(|(k, _)| k.as_str().to_owned()).collect();
    assert!(rows.iter().all(|t| t.starts_with('r')), "{rows:?}");
    let (topic, row) = parts.rows().next().map(|(k, e)| (k.clone(), e)).unwrap();
    ui.world_mut().trigger(Activate { entity: row });
    ui.frames(2);
    assert_eq!(
        journal(&ui, root).0.screen.content(),
        &sf_rpg_ui::BookContent::Topic(topic)
    );
}

// ---------------------------------------------------------------- keybinds

fn open_keybinds(ui: &mut Ui) -> Entity {
    let root = ui.open("keybinds", KeybindsScreen::new(Some("ui")));
    ui.frames(2);
    root
}

#[test]
fn keybinds_capture_conflict_and_grid_focus() {
    let mut ui = Ui::new();
    let root = open_keybinds(&mut ui);
    let parts = ui.world().get::<KeybindsParts>(root).unwrap();
    let confirm = sf_ui_bevy::UiAction::Confirm.id();
    let confirm_row = parts.row(&confirm).unwrap();
    let up = sf_ui_bevy::UiAction::NavUp.id();
    let up_kbm = parts.slot_cell(&up, sf_ui::Device::Kbm, 0).unwrap();
    assert_eq!(ui.focus(), Some(up_kbm), "first cell of the first row");
    assert!(ui.text_of(up_kbm).contains("Up"), "{}", ui.text_of(up_kbm));
    let up_pad = parts.slot_cell(&up, sf_ui::Device::Gamepad, 0).unwrap();

    // Right x2: past the empty second keyboard slot to the gamepad slot.
    ui.tap(GamepadButton::DPadRight);
    ui.tap(GamepadButton::DPadRight);
    assert_eq!(ui.focus(), Some(up_pad));
    assert_eq!(ui.text_of(up_pad), "D-pad Up");
    // Down x5 (over the stick row, which has no keyboard slots): confirm, same column.
    for _ in 0..5 {
        ui.tap(GamepadButton::DPadDown);
    }
    let pad_confirm = ui
        .world()
        .get::<KeybindsParts>(root)
        .unwrap()
        .slot_cell(&confirm, sf_ui::Device::Gamepad, 0)
        .unwrap();
    assert_eq!(ui.focus(), Some(pad_confirm));
    assert_eq!(ui.text_of(pad_confirm), "A");

    // A starts a capture (the A that started it is not captured); the overlay shows.
    ui.tap(GamepadButton::South);
    assert!(ui.world().resource::<UiRebinder>().0.capturing().is_some());
    let overlay = ui.world().get::<KeybindsParts>(root).unwrap().overlay();
    assert_eq!(
        ui.world().get::<Node>(overlay).unwrap().display,
        Display::Flex
    );
    // LT is free in the UI context: applied at once; the row redraws in place.
    ui.tap(GamepadButton::LeftTrigger2);
    ui.frames(2);
    let parts = ui.world().get::<KeybindsParts>(root).unwrap();
    assert_eq!(parts.row(&confirm), Some(confirm_row), "row entity kept");
    let pad_confirm = parts
        .slot_cell(&confirm, sf_ui::Device::Gamepad, 0)
        .unwrap();
    let lt = sf_ui::glyph("pad:LeftTrigger2", sf_ui::PadFamily::Xbox)
        .unwrap()
        .label;
    assert_eq!(ui.text_of(pad_confirm), lt);
    assert!(ui.class_of(confirm_row).contains("modified"));
    assert_eq!(
        ui.world().get::<Node>(overlay).unwrap().display,
        Display::None
    );
    let keymap = &ui.world().resource::<UiKeymap>().0;
    assert_eq!(
        keymap.bound(&confirm, sf_ui::Device::Gamepad),
        ["pad:LeftTrigger2"]
    );

    // Capture again with B (back's): a conflict question takes focus.
    let cell = ui.focus().unwrap();
    ui.world_mut().trigger(Activate { entity: cell });
    ui.frames(2);
    ui.tap(GamepadButton::East);
    ui.frames(1);
    assert!(ui.world().resource::<UiRebinder>().0.pending().is_some());
    let parts = ui.world().get::<KeybindsParts>(root).unwrap();
    let choices = parts.choice_buttons().to_vec();
    assert_eq!(ui.focus(), Some(choices[0]), "swap focused");
    assert!(
        ui.text_of(parts.overlay())
            .contains("is also bound to back")
    );
    // Right x3: cancel; confirm (on LT now) answers. The binding stays LT, focus returns to
    // the cell.
    for _ in 0..3 {
        ui.tap(GamepadButton::DPadRight);
    }
    assert_eq!(ui.focus(), Some(choices[3]));
    ui.tap(GamepadButton::LeftTrigger2);
    assert!(ui.world().resource::<UiRebinder>().0.pending().is_none());
    assert_eq!(ui.focus(), Some(cell));
    let keymap = &ui.world().resource::<UiKeymap>().0;
    assert_eq!(
        keymap.bound(&confirm, sf_ui::Device::Gamepad),
        ["pad:LeftTrigger2"]
    );
}

// ---------------------------------------------------------------- HUD

#[test]
fn hud_bars_keyed_and_prompt_glyph_follows_the_device() {
    let mut ui = Ui::new();
    demo::register_gameplay(&mut ui.world_mut().resource_mut::<UiKeymap>().0);
    ui.world_mut().insert_resource(demo::hud());
    let root = ui.open("hud", HudScreen);
    let parts = ui.world().get::<HudParts>(root).unwrap();
    let health = parts.bar("health").unwrap();
    let glyph = parts.prompt_glyph();
    assert_eq!(ui.text_of(glyph), "E");
    assert!(
        ui.text_of(ui.world().get::<HudParts>(root).unwrap().prompt())
            .contains("Open Door")
    );

    // New values: the same bar entities; the fill follows.
    let mut view = demo::hud();
    view.bars[0].current = 20.0;
    view.bars.remove(1);
    ui.world_mut().insert_resource(view);
    ui.frames(2);
    let parts = ui.world().get::<HudParts>(root).unwrap();
    assert_eq!(parts.bar("health"), Some(health));
    assert!(parts.bar("magicka").is_none());
    let bar = ui.world().get::<Children>(health).unwrap()[1];
    assert!(ui.class_of(bar).contains("low"), "{}", ui.class_of(bar));
    let fill = ui.world().get::<Children>(bar).unwrap()[0];
    assert_eq!(
        ui.world().get::<Node>(fill).unwrap().width,
        Val::Percent(20.0)
    );

    // A pad press: the prompt shows the pad glyph (X on an Xbox pad for West).
    ui.tap(GamepadButton::DPadLeft);
    assert_eq!(ui.text_of(glyph), "X");
    // Not ready: greyed class.
    let mut view = ui.world().resource::<HudView>().clone();
    view.prompt.as_mut().unwrap().ready = false;
    ui.world_mut().insert_resource(view);
    ui.frames(1);
    let prompt = ui.world().get::<HudParts>(root).unwrap().prompt();
    assert!(ui.class_of(prompt).contains("blocked"));
    assert_eq!(hud::heading_label(48.0), "NE 048");
    assert_eq!(hud::heading_label(-90.0), "W 270");
}

// ---------------------------------------------------------------- idle cost

#[derive(Resource, Default)]
struct Touched(Vec<String>);

type Touch = Or<(
    Changed<Node>,
    Changed<Text>,
    Changed<TextSpan>,
    Changed<sf_ui_css::UiClass>,
    Changed<TextColor>,
)>;

fn count_touched(touched: Query<(Entity, Option<&Name>), Touch>, mut out: ResMut<Touched>) {
    out.0.extend(
        touched
            .iter()
            .map(|(e, n)| n.map_or_else(|| format!("{e}"), ToString::to_string)),
    );
}

#[test]
fn idle_screens_touch_nothing() {
    let mut ui = Ui::with(|app| {
        app.init_resource::<Touched>()
            .add_systems(Last, count_touched);
    });
    demo::register_gameplay(&mut ui.world_mut().resource_mut::<UiKeymap>().0);
    ui.world_mut().insert_resource(demo::hud());
    ui.open("hud", HudScreen);
    let check = |ui: &mut Ui, what: &str| {
        ui.frames(5);
        ui.world_mut().resource_mut::<Touched>().0.clear();
        ui.frames(3);
        let touched = std::mem::take(&mut ui.world_mut().resource_mut::<Touched>().0);
        assert!(touched.is_empty(), "{what} touched {touched:?} while idle");
    };
    check(&mut ui, "hud");
    let root = open_dialogue(&mut ui);
    check(&mut ui, "dialogue");
    ui.world_mut().despawn(root);
    ui.world_mut()
        .resource_mut::<ScreenStackRes>()
        .0
        .close("dialogue");
    let root = open_inventory(&mut ui, 200);
    check(&mut ui, "inventory");
    ui.world_mut().despawn(root);
    ui.world_mut()
        .resource_mut::<ScreenStackRes>()
        .0
        .close("inventory");
    let root = open_journal(&mut ui);
    check(&mut ui, "journal");
    ui.world_mut().despawn(root);
    ui.world_mut()
        .resource_mut::<ScreenStackRes>()
        .0
        .close("journal");
    open_keybinds(&mut ui);
    check(&mut ui, "keybinds");
}

/// Frame times with an open screen (whole app update: input, UI layout, text, screens), on
/// whatever machine runs it. `cargo test -p sf_ui_screens -- --ignored --nocapture`.
#[test]
#[ignore = "timing report, run on demand"]
fn timing_screens() {
    let mut ui = Ui::new();
    let time = |ui: &mut Ui, frames: u32| {
        let t = std::time::Instant::now();
        for _ in 0..frames {
            ui.app.update();
        }
        t.elapsed() / frames
    };
    let root = open_inventory(&mut ui, 1000);
    ui.frames(10);
    let idle = time(&mut ui, 60);
    ui.set_button(GamepadButton::DPadDown, true);
    let scroll = time(&mut ui, 120);
    ui.set_button(GamepadButton::DPadDown, false);
    ui.frames(2);
    let scrolled = ui
        .world()
        .get::<InventoryScreen>(root)
        .unwrap()
        .grid
        .scroll();
    println!(
        "inventory 1000 stacks: idle frame {idle:?}, held-down scroll frame {scroll:?} (scrolled {scrolled} px)"
    );
    ui.world_mut().despawn(root);
    ui.world_mut()
        .resource_mut::<ScreenStackRes>()
        .0
        .close("inventory");
    let root = open_journal(&mut ui);
    ui.frames(5);
    let idle = time(&mut ui, 60);
    let t = std::time::Instant::now();
    ui.tap(GamepadButton::LeftTrigger);
    let turn = t.elapsed();
    println!("journal: idle frame {idle:?}, page turn (3 frames) {turn:?}");
    ui.world_mut().despawn(root);
    ui.world_mut()
        .resource_mut::<ScreenStackRes>()
        .0
        .close("journal");
    open_dialogue(&mut ui);
    let idle = time(&mut ui, 60);
    println!("dialogue: idle frame {idle:?}");
}
