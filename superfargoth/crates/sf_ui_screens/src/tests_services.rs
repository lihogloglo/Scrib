//! Headless sanity tests of the service screens (trade, service list): they spawn, bind
//! their view-models, and a pad step yields the expected `ServiceRequest`. Same app shape as
//! `tests.rs` (UI layout and text, no window or GPU, a synthetic gamepad, the demo data).

use std::time::Duration;

use bevy::camera::{ComputedCameraValues, RenderTargetInfo};
use bevy::input::InputPlugin;
use bevy::input::gamepad::{Gamepad, GamepadButton};
use bevy::picking::{InteractionPlugin, PickingPlugin};
use bevy::prelude::*;
use bevy::text::{TextPlugin, TextSpan};
use bevy::time::TimeUpdateStrategy;
use bevy::ui::UiPlugin;
use bevy_enhanced_input::prelude::*;
use bevy_input_focus::InputFocus;
use sf_rpg::service::{
    Currency, EntryDetail, EntryId, EntryState, LabelRef, MerchantMoney, ServiceEntry,
    ServiceEventKind, ServiceKind, ServiceRequest, SessionId, Side, TradeConfig, TradeLine,
};
use sf_rpg::{ActorKey, Attrs, RecordId, StackId};
use sf_rpg_ui::{ServiceListView, TradePane, TradeView};
use sf_ui_bevy::{ScreenStackRes, SfUiPlugin, UiRegistryRes, UiScreenRoot, UiTextPlugin};

use crate::demo::{self, DemoContainer, DemoInventory};
use crate::*;

const TARGET: UVec2 = UVec2::new(1600, 1000);
const SESSION: SessionId = SessionId(4);

#[derive(Resource, Default)]
struct Intents(Vec<ServiceIntent>);

fn collect(mut events: MessageReader<ServiceScreenEvent>, mut seen: ResMut<Intents>) {
    seen.0.extend(events.read().map(|e| e.intent.clone()));
}

struct Ui {
    app: App,
    pad: Entity,
}

impl Ui {
    fn new() -> Self {
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
        .add_plugins(SfScreensPlugin)
        .insert_resource(UiRegistryRes(demo::registry()))
        .insert_resource(TimeUpdateStrategy::ManualDuration(Duration::from_millis(
            16,
        )))
        .init_resource::<Intents>()
        .add_systems(Update, collect.after(ScreenSystems::Sync));
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

    /// Spawns a screen root with `bundle` and opens it under `id` (a demo registry screen).
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

    fn tap(&mut self, button: GamepadButton) {
        for down in [true, false] {
            let mut pad = self.app.world_mut().get_mut::<Gamepad>(self.pad).unwrap();
            if down {
                pad.digital_mut().press(button);
            } else {
                pad.digital_mut().release(button);
            }
            pad.analog_mut().set(button, if down { 1.0 } else { 0.0 });
            self.app.update();
        }
        self.app.update();
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

    fn take_intents(&mut self) -> Vec<ServiceIntent> {
        std::mem::take(&mut self.world_mut().resource_mut::<Intents>().0)
    }

    /// Whether the screen `id` still takes input.
    fn is_open(&self, id: &str) -> bool {
        self.world()
            .resource::<ScreenStackRes>()
            .0
            .input_target()
            .is_some_and(|t| t.id.as_str() == id)
    }
}

// ---------------------------------------------------------------- trade

/// The demo chest as the provider (every stack priced 10), the demo player (every stack 5).
fn open_trade() -> (Ui, Entity) {
    let mut ui = Ui::new();
    let inventory = DemoInventory::new(8);
    let chest = DemoContainer::default();
    let config = TradeConfig {
        currency: Currency {
            item: RecordId::new("gold"),
        },
        haggle: true,
    };
    let mut view = TradeView::new(
        SESSION,
        chest.view(&inventory),
        inventory.view(),
        &config,
        MerchantMoney::Pool(500),
    );
    for (pane, price) in [(TradePane::Provider, 10), (TradePane::Player, 5)] {
        let stacks: Vec<StackId> = view.pane(pane).rows().iter().map(|(k, _)| *k).collect();
        view.set_prices(pane, stacks.into_iter().map(|s| (s, Some(price))));
    }
    let screen = TradeScreen::new(view).with_title("Trader");
    let root = ui.open("container", screen);
    (ui, root)
}

fn shown(ui: &mut Ui, root: Entity, pane: TradePane) -> Vec<StackId> {
    ui.world_mut()
        .get_mut::<TradeScreen>(root)
        .unwrap()
        .view
        .shown(pane)
}

fn trade_parts(ui: &Ui, root: Entity) -> &TradeParts {
    ui.world().get::<TradeParts>(root).unwrap()
}

#[test]
fn trade_screen_binds_both_panes_and_stages_by_pad() {
    let (mut ui, root) = open_trade();
    let provider = shown(&mut ui, root, TradePane::Provider);
    let player = shown(&mut ui, root, TradePane::Player);
    assert!(provider.len() >= 2 && !player.is_empty());
    let parts = trade_parts(&ui, root);
    assert_eq!(parts.rows(TradePane::Provider).count(), provider.len());
    assert_eq!(parts.rows(TradePane::Player).count(), player.len());
    // The price column shows the host's quote.
    let first = parts.row_of(TradePane::Provider, provider[0]).unwrap();
    assert!(ui.text_of(first).contains("10"), "{}", ui.text_of(first));
    // The pad starts on the provider's first stack; down moves to the second.
    assert_eq!(ui.focus(), Some(first));
    ui.tap(GamepadButton::DPadDown);
    let second = trade_parts(&ui, root)
        .row_of(TradePane::Provider, provider[1])
        .unwrap();
    assert_eq!(ui.focus(), Some(second));
    // A stages it whole on the buy side.
    ui.tap(GamepadButton::South);
    let intents = ui.take_intents();
    assert!(
        matches!(
            intents.as_slice(),
            [ServiceIntent::Request(ServiceRequest::Stage {
                session,
                side: Side::Buy,
                source,
                stack,
                ..
            })] if *session == SESSION && *source == demo::CHEST && *stack == provider[1]
        ),
        "{intents:?}"
    );
}

#[test]
fn trade_screen_shows_the_quoted_bar_and_cancels_on_back() {
    let (mut ui, root) = open_trade();
    let provider = shown(&mut ui, root, TradePane::Provider);
    {
        let mut screen = ui.world_mut().get_mut::<TradeScreen>(root).unwrap();
        let item = screen
            .view
            .pane(TradePane::Provider)
            .row(provider[0])
            .unwrap()
            .stack
            .item
            .clone();
        let quoted = ServiceEventKind::TradeQuoted {
            session: SESSION,
            lines: vec![TradeLine {
                side: Side::Buy,
                source: demo::CHEST,
                stack: provider[0],
                item,
                attrs: Attrs::new(),
                count: 1,
                price: 10,
            }],
            merchant_offer: -10,
            balance: -10,
        };
        assert!(screen.view.apply_event(&quoted));
    }
    ui.frames(2);
    let parts = trade_parts(&ui, root);
    let line = parts.line_of(0).expect("the staged line is on the bar");
    assert!(ui.text_of(line).contains("10"), "{}", ui.text_of(line));
    assert!(ui.text_of(parts.totals()).contains("10"));
    // B on a pane cancels the trade and the screen closes itself.
    ui.tap(GamepadButton::East);
    assert_eq!(
        ui.take_intents(),
        vec![ServiceIntent::Request(ServiceRequest::Cancel { session: SESSION })]
    );
    assert!(!ui.is_open("container"));
}

#[test]
fn trade_screen_asks_how_many() {
    let (mut ui, root) = open_trade();
    // A provider stack of more than one piece.
    let (stack, count) = {
        let mut screen = ui.world_mut().get_mut::<TradeScreen>(root).unwrap();
        let stacks = screen.view.shown(TradePane::Provider);
        let found = stacks
            .into_iter()
            .map(|s| (s, screen.view.available(TradePane::Provider, s)))
            .find(|(_, n)| *n > 1)
            .expect("the demo chest has a stack of several");
        screen.view.focus_row(TradePane::Provider, found.0);
        found
    };
    ui.frames(2);
    // X opens the count dialog (all pieces preselected), left takes one off, A stages.
    ui.tap(GamepadButton::West);
    assert!(
        ui.world()
            .get::<TradeScreen>(root)
            .unwrap()
            .view
            .count_dialog()
            .is_some()
    );
    let dialog = trade_parts(&ui, root).count_dialog();
    assert_ne!(
        ui.world().get::<Node>(dialog).unwrap().display,
        Display::None
    );
    ui.tap(GamepadButton::DPadLeft);
    ui.tap(GamepadButton::South);
    let intents = ui.take_intents();
    assert!(
        matches!(
            intents.as_slice(),
            [ServiceIntent::Request(ServiceRequest::Stage { stack: s, count: n, .. })]
                if *s == stack && *n == count - 1
        ),
        "{intents:?}"
    );
}

// ---------------------------------------------------------------- service list

fn kind() -> ServiceKind {
    ServiceKind(RecordId::new("train"))
}

fn entry(id: &str, price: i64, detail: EntryDetail) -> ServiceEntry {
    ServiceEntry {
        id: EntryId(RecordId::new(id)),
        label: LabelRef(RecordId::new(id)),
        detail,
        price,
        state: EntryState::Available,
    }
}

fn entries() -> Vec<ServiceEntry> {
    vec![
        entry("cheap", 10, EntryDetail::Text(LabelRef(RecordId::new("note")))),
        entry("dear", 999, EntryDetail::Text(LabelRef(RecordId::new("note")))),
        entry(
            "trip",
            20,
            EntryDetail::Destination {
                place: LabelRef(RecordId::new("town")),
                hours: 2,
            },
        ),
    ]
}

#[test]
fn service_list_buys_skips_disabled_rows_and_confirms_travel() {
    let mut ui = Ui::new();
    let mut view = ServiceListView::new(
        ActorKey(7),
        kind(),
        LabelRef(RecordId::new("trainer")),
        1.0,
        10.0,
    );
    view.set_player_gold(100);
    view.set_entries(entries());
    let root = ui.open("inventory", ServiceListScreen::new(view));
    let id = |s: &str| EntryId(RecordId::new(s));
    let row = |ui: &Ui, s: &str| {
        ui.world()
            .get::<ServiceListParts>(root)
            .unwrap()
            .row(&id(s))
            .unwrap()
    };
    assert_eq!(
        ui.world().get::<ServiceListParts>(root).unwrap().rows().count(),
        3
    );
    assert!(ui.class_of(row(&ui, "dear")).contains("disabled"));
    assert!(!ui.class_of(row(&ui, "cheap")).contains("disabled"));
    assert!(ui.text_of(row(&ui, "cheap")).contains("10"));

    // A on the first row buys it.
    assert_eq!(ui.focus(), Some(row(&ui, "cheap")));
    ui.tap(GamepadButton::South);
    let buy = |s: &str| {
        ServiceIntent::Request(ServiceRequest::BuyEntry {
            provider: ActorKey(7),
            kind: kind(),
            entry: id(s),
        })
    };
    assert_eq!(ui.take_intents(), vec![buy("cheap")]);
    // The host re-lists (which also ends the purchase in flight).
    ui.world_mut()
        .get_mut::<ServiceListScreen>(root)
        .unwrap()
        .view
        .apply_event(&ServiceEventKind::EntriesListed {
            kind: kind(),
            entries: entries(),
        });
    ui.frames(1);

    // The disabled row does nothing.
    ui.tap(GamepadButton::DPadDown);
    assert_eq!(ui.focus(), Some(row(&ui, "dear")));
    ui.tap(GamepadButton::South);
    assert!(ui.take_intents().is_empty());

    // Travel asks first: the question takes focus, yes buys.
    ui.tap(GamepadButton::DPadDown);
    ui.tap(GamepadButton::South);
    assert!(ui.take_intents().is_empty());
    let yes = ui
        .world()
        .get::<ServiceListParts>(root)
        .unwrap()
        .confirm_buttons()[0];
    assert_eq!(ui.focus(), Some(yes));
    ui.tap(GamepadButton::South);
    assert_eq!(ui.take_intents(), vec![buy("trip")]);

    // B closes the list.
    ui.tap(GamepadButton::East);
    assert_eq!(ui.take_intents(), vec![ServiceIntent::Close]);
    assert!(!ui.is_open("inventory"));
}
