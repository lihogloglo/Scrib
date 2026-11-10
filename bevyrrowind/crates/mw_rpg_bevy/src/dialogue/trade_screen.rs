//! The trade screen over the dialogue (slice TR2, services.md "Screens"): the stock
//! `sf_ui_screens` [`TradeScreen`] on the barter session [`Rpg`] opened, as the container
//! window is opened over a container (`inventory_ui.rs`).
//!
//! - **Open**: a `ServiceOpened` event with a session ([`ServiceEventMessage`], the Barter
//!   button) for the conversation's speaker pushes [`TRADE_SCREEN`] over the dialogue (both are
//!   Menu screens: the trade window is the input target, the dialogue waits under it). Its
//!   panes are the speaker's store and the player's loadout, its price columns and the
//!   merchant's money [`Rpg::trade_quotes`].
//! - **Requests in** ([`host_trade`]): the screen's [`ServiceScreenEvent`] requests go to
//!   [`Rpg::service_request`] with the conversation, as `host_dialogue` routes the dialogue's.
//! - **Events out** ([`trade_events`]): `TradeQuoted`, `TradeRefused`, `TradeDone` and
//!   `PurseRestocked` feed the view (`TradeView::apply_event`); both panes re-sync and are
//!   re-priced when the RPG state changes (staging, a settled deal, a script).
//! - **Close**: an accepted deal closes the session (`OpenMW` `TradeWindow::onOfferButtonClicked`
//!   closes the barter window, `apps/openmw/mwgui/tradewindow.cpp`, commit
//!   `ca5e2576bfeb8fa64b53fdc8a31fefc360decc2f`), so `TradeDone` closes the screen back to the
//!   dialogue; Cancel and Back close it through the screen (the host drops the session on its
//!   `Cancel`); a closed dialogue (goodbye, which drops the sessions) closes it too
//!   ([`close_trade`]).
//!
//! Net-ready (services.md "Co-op"): the screen only sends requests, the host owns the session,
//! and the view changes only from the host's events.

use bevy_app::prelude::*;
use bevy_ecs::prelude::*;
use bevy_log::prelude::*;
use sf_rpg::service::{ServiceEventKind, ServiceShape, SessionId};
use sf_rpg::{ActorKey, PlayerId};
use sf_rpg_ui::{TradePane, TradeView};
use sf_stats_bevy::StatTick;
use sf_ui::Layer;
use sf_ui_bevy::{ScreenStackRes, UiRegistryRes, UiScreenRoot};
use sf_ui_screens::{
    DialogueScreen, ScreenSystems, ServiceIntent, ServiceScreenEvent, TradeScreen,
};

use super::services::{ServiceEventMessage, TradeQuotes, send_service_events, trade_config};
use super::{ActiveTalk, DialogueSheets, Rpg, host_dialogue};
use crate::inventory::ItemRecords;
use crate::inventory_ui::{PcSlots, container_view, player_view};

/// The trade screen's id (`SCREENS` in `dialogue.rs`).
pub const TRADE_SCREEN: &str = "trade";

/// On a trade screen's root: the session it shows and its provider.
#[derive(Component, Clone, Copy, Debug, PartialEq, Eq)]
pub struct OpenTrade {
    pub session: SessionId,
    pub provider: ActorKey,
}

/// The trade screen's hosting, from `inventory_ui::add` (it needs the item records and the
/// player's slots).
pub fn add(app: &mut App) {
    app.add_systems(
        Update,
        (
            host_trade
                .after(ScreenSystems::Input)
                .after(host_dialogue)
                .before(send_service_events),
            (trade_events, close_trade)
                .chain()
                .after(send_service_events),
        ),
    );
}

/// The simulation tick the requests apply at (seeds haggle rolls), 0 without the stats plugin.
fn tick_of(tick: Option<&StatTick>) -> u64 {
    tick.map_or(0, |t| u64::from(t.0))
}

/// The screen's requests → [`Rpg::service_request`] with the open conversation.
#[allow(clippy::needless_pass_by_value)] // system parameters
fn host_trade(
    mut events: MessageReader<ServiceScreenEvent>,
    mut rpg: ResMut<Rpg>,
    active: Option<ResMut<ActiveTalk>>,
    trades: Query<(), With<OpenTrade>>,
    mut dialogues: Query<&mut DialogueScreen>,
    sheets: DialogueSheets,
    tick: Option<Res<StatTick>>,
) {
    let Some(mut active) = active else {
        events.clear();
        return;
    };
    let tick = tick_of(tick.as_deref());
    let Ok(mut window) = dialogues.get_mut(active.screen) else {
        events.clear();
        return;
    };
    for event in events.read() {
        match &event.intent {
            ServiceIntent::Request(request) if trades.contains(event.screen) => {
                rpg.service_request(&mut active.talk, &mut window.view, request, &sheets, tick);
            }
            // The list screen's requests and Close: `service_list_screen.rs`.
            ServiceIntent::Request(_) | ServiceIntent::Close => {}
        }
    }
}

/// A new trade view over session `session` with `provider`, synced and priced; `None` without
/// the provider's store or the session.
pub fn trade_view(
    rpg: &Rpg,
    records: &ItemRecords,
    slots: &PcSlots,
    session: SessionId,
    provider: ActorKey,
    quotes: TradeQuotes,
) -> Option<TradeView> {
    let store = rpg.state.inventories.get(&provider)?;
    let stock = container_view(records, &slots.0, provider, store);
    let mine = player_view(records, &slots.0, &rpg.state.pc);
    let mut view = TradeView::new(session, stock, mine, &trade_config(), quotes.money);
    view.set_prices(TradePane::Provider, quotes.provider);
    view.set_prices(TradePane::Player, quotes.player);
    Some(view)
}

/// Re-syncs both panes of `screen` from the RPG state and re-sends their prices.
fn refresh(
    screen: &mut TradeScreen,
    open: OpenTrade,
    rpg: &Rpg,
    records: &ItemRecords,
    slots: &PcSlots,
    sheets: &DialogueSheets,
    tick: u64,
) {
    if let Some(store) = rpg.state.inventories.get(&open.provider) {
        screen
            .view
            .sync(TradePane::Provider, &store.loadout, &slots.0, records);
    }
    screen
        .view
        .sync(TradePane::Player, &rpg.state.pc.loadout, &slots.0, records);
    if let Some(quotes) = rpg.trade_quotes(open.session, sheets, tick) {
        screen.view.set_prices(TradePane::Provider, quotes.provider);
        screen.view.set_prices(TradePane::Player, quotes.player);
    }
}

/// Host events → the trade screens: `ServiceOpened` with a session opens one over the
/// dialogue, the session's events feed its view, `TradeDone` closes it; panes and prices
/// follow the RPG state.
#[allow(clippy::needless_pass_by_value, clippy::too_many_arguments)] // system parameters
fn trade_events(
    mut commands: Commands,
    mut events: MessageReader<ServiceEventMessage>,
    rpg: Res<Rpg>,
    active: Option<Res<ActiveTalk>>,
    records: Res<ItemRecords>,
    slots: Res<PcSlots>,
    mut stack: ResMut<ScreenStackRes>,
    registry: Res<UiRegistryRes>,
    mut screens: Query<(&mut TradeScreen, &OpenTrade)>,
    sheets: DialogueSheets,
    tick: Option<Res<StatTick>>,
) {
    let tick = tick_of(tick.as_deref());
    let mut changed = rpg.is_changed();
    for ServiceEventMessage(event) in events.read().filter(|e| e.0.player == PlayerId::HOST) {
        match &event.kind {
            ServiceEventKind::ServiceOpened {
                shape: ServiceShape::Trade,
                session: Some(session),
                ..
            } => {
                let speaker = active.as_deref().map(|a| a.talk.speaker);
                if speaker != Some(event.provider) {
                    warn!(
                        "trade: {session:?} with {:?} outside its conversation",
                        event.provider
                    );
                    continue;
                }
                if stack
                    .0
                    .top(Layer::Menu)
                    .is_none_or(|top| top.id.as_str() != "dialogue")
                {
                    info!("trade: {session:?} not shown, another menu is over the dialogue");
                    continue;
                }
                let view = rpg
                    .trade_quotes(*session, &sheets, tick)
                    .and_then(|q| trade_view(&rpg, &records, &slots, *session, event.provider, q));
                let Some(view) = view else {
                    warn!(
                        "trade: {session:?} with {:?} has no session or store",
                        event.provider
                    );
                    continue;
                };
                let title = rpg
                    .actor_name(event.provider)
                    .unwrap_or_default()
                    .to_owned();
                let open = OpenTrade {
                    session: *session,
                    provider: event.provider,
                };
                commands.spawn((
                    UiScreenRoot(TRADE_SCREEN.into()),
                    TradeScreen::new(view).with_title(title),
                    open,
                ));
                if let Err(e) = stack.0.push(&registry.0, TRADE_SCREEN) {
                    warn!("trade: {e:?}");
                }
            }
            kind => {
                for (mut screen, open) in &mut screens {
                    if open.provider != event.provider || !screen.view.apply_event(kind) {
                        continue;
                    }
                    if matches!(kind, ServiceEventKind::TradeDone { session, .. } if *session == open.session)
                    {
                        info!("trade: deal done with {:?}", open.provider);
                        stack.0.close(TRADE_SCREEN);
                    }
                    changed = true;
                }
            }
        }
    }
    if changed {
        for (mut screen, open) in &mut screens {
            refresh(&mut screen, *open, &rpg, &records, &slots, &sheets, tick);
        }
    }
}

/// Trade screens follow their dialogue: a closed dialogue (goodbye) closes the trade screen,
/// a closed view (Cancel, a veto) closes it, and a root off the stack despawns.
#[allow(clippy::needless_pass_by_value)] // system parameters
fn close_trade(
    mut commands: Commands,
    mut stack: ResMut<ScreenStackRes>,
    roots: Query<(Entity, &TradeScreen), With<OpenTrade>>,
) {
    for (e, screen) in &roots {
        if !stack.0.is_open("dialogue") || screen.view.is_closed() {
            stack.0.close(TRADE_SCREEN);
        }
        if !stack.0.is_open(TRADE_SCREEN) {
            commands.entity(e).despawn();
        }
    }
}

#[cfg(test)]
mod tests {
    //! The trade screen over the dialogue, headless: the host's dialogue and trade systems
    //! over synthetic records (`services.rs` tests), exact assertions. No sheets: prices are
    //! the items' base values.
    use std::sync::Arc;

    use mw_rpg::NoSheets;
    use sf_records::RecordId;
    use sf_rpg::service::{ServiceRequest, Side};
    use sf_rpg::{Holder, StackId};
    use sf_rpg_ui::DialogueRequest;
    use sf_ui_screens::{DialogueRequestEvent, DialogueScreen};

    use super::super::services::tests::{
        MERCHANT, PLAYER, SELLER, barter, count, records, stack_of,
    };
    use super::super::{close_screens, registry};
    use super::*;
    use crate::inventory::GOLD;

    /// An app with the dialogue's and the trade screen's host systems, the player (100 gold, a
    /// spoon) talking to the merchant.
    fn app() -> (App, Entity) {
        let db = Arc::new(records(None));
        let mut rpg = Rpg::new(db.clone()).unwrap();
        assert!(rpg.add_actor(MERCHANT, &RecordId::new("arrille"), "Balmora"));
        for (item, n) in [(GOLD, 100), ("misc_spoon", 1)] {
            rpg.state
                .pc
                .loadout
                .add(RecordId::new(item), n, sf_rpg::Attrs::new())
                .unwrap();
        }
        let (talk, view) = rpg.open(MERCHANT, &NoSheets).unwrap();
        let mut app = App::new();
        app.add_message::<ServiceEventMessage>()
            .add_message::<ServiceScreenEvent>()
            .add_message::<DialogueRequestEvent>()
            .insert_resource(rpg)
            .insert_resource(ItemRecords(db))
            .insert_resource(UiRegistryRes(registry()))
            .init_resource::<ScreenStackRes>()
            .init_resource::<PcSlots>()
            .add_systems(
                Update,
                (host_dialogue, close_screens, send_service_events).chain(),
            );
        add(&mut app);
        let screen = app
            .world_mut()
            .spawn((UiScreenRoot("dialogue".into()), DialogueScreen::new(view)))
            .id();
        app.insert_resource(ActiveTalk { talk, screen });
        app.world_mut()
            .resource_mut::<ScreenStackRes>()
            .0
            .push(&registry(), "dialogue")
            .unwrap();
        (app, screen)
    }

    fn say(app: &mut App, screen: Entity, request: DialogueRequest) {
        app.world_mut()
            .write_message(DialogueRequestEvent { screen, request });
        app.update();
    }

    fn trade(app: &mut App) -> Option<(Entity, OpenTrade)> {
        let mut q = app.world_mut().query::<(Entity, &OpenTrade)>();
        q.iter(app.world()).map(|(e, o)| (e, *o)).next()
    }

    fn view(app: &mut App, screen: Entity) -> &mut TradeView {
        &mut app
            .world_mut()
            .get_mut::<TradeScreen>(screen)
            .unwrap()
            .into_inner()
            .view
    }

    fn request(app: &mut App, screen: Entity, request: ServiceRequest) {
        app.world_mut().write_message(ServiceScreenEvent {
            screen,
            intent: ServiceIntent::Request(request),
        });
        app.update();
    }

    fn is_open(app: &App, id: &str) -> bool {
        app.world().resource::<ScreenStackRes>().0.is_open(id)
    }

    fn stack(app: &App, holder: Holder, item: &str) -> StackId {
        stack_of(app.world().resource::<Rpg>(), holder, item)
    }

    /// (item, count, price) of a pane's shown rows.
    fn rows(view: &mut TradeView, pane: TradePane) -> Vec<(String, u32, Option<i64>)> {
        let rows = view.rows(pane);
        let item = |stack| {
            view.pane(pane)
                .row(stack)
                .map(|r| r.stack.item.as_str().to_owned())
                .unwrap_or_default()
        };
        rows.into_iter()
            .map(|r| (item(r.stack), r.available, r.price))
            .collect()
    }

    /// Barter, then the trade screen on top of the dialogue.
    fn open_trade(app: &mut App, dialogue: Entity) -> (Entity, OpenTrade) {
        say(app, dialogue, barter());
        app.update();
        let opened = trade(app).expect("Barter opened the trade screen");
        let layer = app
            .world()
            .resource::<ScreenStackRes>()
            .0
            .layer(Layer::Menu)
            .iter()
            .map(|s| s.id.as_str().to_owned())
            .collect::<Vec<_>>();
        assert_eq!(layer, ["dialogue", TRADE_SCREEN], "over the dialogue");
        opened
    }

    #[test]
    fn barter_opens_the_trade_screen_with_both_panes_priced() {
        let (mut app, dialogue) = app();
        let (screen, open) = open_trade(&mut app, dialogue);
        assert_eq!(
            open,
            OpenTrade {
                session: SessionId(0),
                provider: MERCHANT
            }
        );
        let root = app.world().get::<UiScreenRoot>(screen).unwrap().0.clone();
        assert_eq!(root.as_str(), TRADE_SCREEN);
        assert_eq!(
            app.world().get::<TradeScreen>(screen).unwrap().title,
            "Arrille"
        );
        let view = view(&mut app, screen);
        assert_eq!(view.session(), SessionId(0));
        // The merchant's seeded stock at base prices; the player's items without the gold.
        let mut stock = rows(view, TradePane::Provider);
        stock.sort();
        assert_eq!(
            stock,
            [
                ("misc_bottle".into(), 1, Some(10)),
                ("misc_candle".into(), 5, Some(15))
            ]
        );
        assert_eq!(
            rows(view, TradePane::Player),
            [("misc_spoon".into(), 1, Some(4))]
        );
        assert_eq!((view.player_gold(), view.merchant_gold()), (100, Some(500)));
        assert!(
            view.rows(TradePane::Player)
                .iter()
                .all(|r| r.blocked.is_none())
        );
    }

    #[test]
    fn a_stage_request_from_the_screen_moves_the_deal_balance() {
        let (mut app, dialogue) = app();
        let (screen, open) = open_trade(&mut app, dialogue);
        let bottle = stack(&app, SELLER, "misc_bottle");
        let stage = ServiceRequest::Stage {
            session: open.session,
            side: Side::Buy,
            source: SELLER,
            stack: bottle,
            count: 1,
        };
        request(&mut app, screen, stage);
        let session = app
            .world()
            .resource::<Rpg>()
            .trade_session(open.session)
            .unwrap();
        assert_eq!((session.lines.len(), session.balance), (1, -10));
        let deal = view(&mut app, screen).deal();
        assert_eq!(
            (
                deal.buy.len(),
                deal.buy_total,
                deal.balance,
                deal.player_after
            ),
            (1, 10, -10, 90)
        );
        // Staging moved nothing: the bottle is on the bar, not in the player's pane.
        assert_eq!(
            view(&mut app, screen).available(TradePane::Provider, bottle),
            0
        );
        assert_eq!(
            count(app.world().resource::<Rpg>(), PLAYER, "misc_bottle"),
            0
        );
    }

    #[test]
    fn accept_closes_the_screen_back_to_the_dialogue_with_the_goods_moved() {
        let (mut app, dialogue) = app();
        let (screen, open) = open_trade(&mut app, dialogue);
        let (bottle, spoon) = (
            stack(&app, SELLER, "misc_bottle"),
            stack(&app, PLAYER, "misc_spoon"),
        );
        let session = open.session;
        request(
            &mut app,
            screen,
            ServiceRequest::Stage {
                session,
                side: Side::Buy,
                source: SELLER,
                stack: bottle,
                count: 1,
            },
        );
        request(
            &mut app,
            screen,
            ServiceRequest::Stage {
                session,
                side: Side::Sell,
                source: PLAYER,
                stack: spoon,
                count: 1,
            },
        );
        assert_eq!(view(&mut app, screen).deal().balance, -6);
        request(&mut app, screen, ServiceRequest::Accept { session });
        app.update();
        assert!(
            !is_open(&app, TRADE_SCREEN),
            "the deal closed the trade screen"
        );
        assert!(trade(&mut app).is_none(), "its root despawned");
        assert!(is_open(&app, "dialogue"), "back to the dialogue");
        assert!(
            app.world().get_resource::<ActiveTalk>().is_some(),
            "the conversation goes on"
        );
        let rpg = app.world().resource::<Rpg>();
        assert!(rpg.trade_session(session).is_none());
        let items = [GOLD, "misc_bottle", "misc_spoon"];
        assert_eq!(items.map(|i| count(rpg, PLAYER, i)), [94, 1, 0]);
        assert_eq!(items.map(|i| count(rpg, SELLER, i)), [0, 0, 1]);
        assert_eq!(rpg.services.gold(MERCHANT), Some(506));
        // Barter again: a new session over the new stock.
        let (screen, open) = open_trade(&mut app, dialogue);
        assert_eq!(open.session, SessionId(1));
        assert_eq!(
            rows(view(&mut app, screen), TradePane::Player),
            [("misc_bottle".into(), 1, Some(10))]
        );
    }

    #[test]
    fn goodbye_closes_the_trade_screen_and_the_dialogue() {
        let (mut app, dialogue) = app();
        let (_, open) = open_trade(&mut app, dialogue);
        say(&mut app, dialogue, DialogueRequest::Goodbye);
        app.update();
        assert!(!is_open(&app, TRADE_SCREEN) && !is_open(&app, "dialogue"));
        assert!(trade(&mut app).is_none(), "the trade root despawned");
        assert!(
            app.world().get_entity(dialogue).is_err(),
            "the dialogue root despawned"
        );
        assert!(app.world().get_resource::<ActiveTalk>().is_none());
        assert!(
            app.world()
                .resource::<Rpg>()
                .trade_session(open.session)
                .is_none(),
            "goodbye dropped the session"
        );
        assert_eq!(
            count(app.world().resource::<Rpg>(), PLAYER, GOLD),
            100,
            "nothing traded"
        );
    }
}
