//! The service list screen over the dialogue (slice A3, services.md "Screens"): the stock
//! `sf_ui_screens` [`ServiceListScreen`] on a list service [`Rpg`] opened (training first;
//! travel, spells and repair reuse it in A4-A6), as `trade_screen.rs` opens the trade screen.
//!
//! - **Open**: a `ServiceOpened` event of shape `List` for the conversation's speaker pushes
//!   [`SERVICE_SCREEN`] over the dialogue (both Menu screens). Its rows are the host's open list
//!   ([`Rpg::service_list`]: the `EntriesListed` sent with the opening), its title the service's
//!   button GMST (`sServiceTrainingTitle`), the player's gold from the loadout.
//! - **Labels**: row labels, refusal reasons and the title are GMST keys; their texts go into
//!   [`ServiceLabels`] as rows arrive (the key itself when the content has no such GMST).
//! - **Requests in** ([`host_list`]): the screen's `BuyEntry` requests go to
//!   [`Rpg::service_request`] with the conversation; its Close (Back, the Close button) closes the
//!   host's list ([`Rpg::close_service_list`]) and the screen, back to the dialogue.
//! - **Events out** ([`list_events`]): `EntriesListed`, `EntryBought` and `EntryRefused` feed the
//!   view (`ServiceListView::apply_event`); the gold follows the RPG state.
//! - **Confirmation** ([`confirm_time_passing`]): rows that pass game time ask first (training's
//!   2 hours, travel), services.md "Screens" (modern; Morrowind buys on click).
//! - **Close**: a closed dialogue (goodbye, which also closes the host's lists) closes the
//!   screen ([`close_list`]).
//!
//! Net-ready (services.md "Co-op"): the screen only sends requests, the host owns the list and
//! its rows, and the view changes only from the host's events.

use bevy_app::prelude::*;
use bevy_ecs::prelude::*;
use bevy_log::prelude::*;
use mw_rpg::MwService;
use sf_records::RecordId;
use sf_rpg::service::{
    EntryDetail, EntryState, LabelRef, ServiceEntry, ServiceEventKind, ServiceKind, ServiceShape,
};
use sf_rpg::{ActorKey, PlayerId};
use sf_rpg_ui::ServiceListView;
use sf_stats_bevy::StatTick;
use sf_ui::Layer;
use sf_ui_bevy::{ScreenStackRes, UiRegistryRes, UiScreenRoot};
use sf_ui_screens::{
    DialogueScreen, ScreenSystems, ServiceIntent, ServiceLabels, ServiceListScreen,
    ServiceScreenEvent,
};

use super::persuasion::gmst_text;
use super::services::{ServiceEventMessage, label_fallback, relist_services, send_service_events};
use super::{ActiveTalk, DialogueSheets, Rpg, host_dialogue, id};
use crate::inventory::GOLD;

/// The service list screen's id (`SCREENS` in `dialogue.rs`).
pub const SERVICE_SCREEN: &str = "service";

/// Row height and viewport of the list, in rows (a trainer's 3 skills, a guide's destinations).
const ROW_EXTENT: f32 = 1.0;
const VIEWPORT_ROWS: f32 = 8.0;

/// On a service list screen's root: the provider and the service it shows.
#[derive(Component, Clone, Debug, PartialEq, Eq)]
pub struct OpenServiceList {
    pub provider: ActorKey,
    pub kind: ServiceKind,
}

/// Rows that pass game time ask "are you sure?" first: training (a skill row, 2 hours) and
/// travel (a destination). services.md "Screens": `PassTime` over an hour, `Teleport`.
pub fn confirm_time_passing(entry: &ServiceEntry) -> bool {
    matches!(
        entry.detail,
        EntryDetail::Stat { .. } | EntryDetail::Destination { .. }
    )
}

/// The list screen's hosting, from `dialogue::add`.
pub fn add(app: &mut App) {
    app.add_systems(
        Update,
        (
            host_list
                .after(ScreenSystems::Input)
                .after(host_dialogue)
                .before(relist_services)
                .before(send_service_events),
            (list_events, close_list).chain().after(send_service_events),
        ),
    );
}

/// The screen's requests → [`Rpg::service_request`] with the open conversation; Close → the
/// host's list closes.
#[allow(clippy::needless_pass_by_value)] // system parameters
fn host_list(
    mut events: MessageReader<ServiceScreenEvent>,
    mut rpg: ResMut<Rpg>,
    active: Option<ResMut<ActiveTalk>>,
    lists: Query<&OpenServiceList>,
    mut dialogues: Query<&mut DialogueScreen>,
    sheets: DialogueSheets,
    tick: Option<Res<StatTick>>,
) {
    let Some(mut active) = active else {
        events.clear();
        return;
    };
    let tick = tick.map_or(0, |t| u64::from(t.0));
    let Ok(mut window) = dialogues.get_mut(active.screen) else {
        events.clear();
        return;
    };
    for event in events.read() {
        // The trade screen's events: `trade_screen.rs`.
        let Ok(open) = lists.get(event.screen) else {
            continue;
        };
        match &event.intent {
            ServiceIntent::Request(request) => {
                rpg.service_request(&mut active.talk, &mut window.view, request, &sheets, tick);
            }
            ServiceIntent::Close => {
                let player = active.talk.session.ctx().player;
                rpg.close_service_list(player, open.provider);
                debug!("service list: {:?} closed", open.kind);
            }
        }
    }
}

/// The texts of a list's label keys (row labels, a skill row's stat, refusal reasons): their
/// GMSTs, or the key itself.
fn add_texts(labels: &mut ServiceLabels, rpg: &Rpg, entries: &[ServiceEntry]) {
    let db = &rpg.content.records;
    for e in entries {
        let label = gmst_text(db, e.label.0.as_str(), e.label.0.as_str());
        if let EntryDetail::Stat { stat, .. } = &e.detail {
            labels.texts.insert(stat.clone(), label.clone());
        }
        labels.texts.insert(e.label.0.clone(), label);
        if let EntryState::Refused(why) = &e.state {
            let text = gmst_text(db, why.0.as_str(), why.0.as_str());
            labels.texts.insert(why.0.clone(), text);
        }
    }
}

/// A new list view for `kind` with `provider`: its title, the host's rows, the player's gold.
fn list_view(rpg: &Rpg, provider: ActorKey, kind: &ServiceKind) -> ServiceListView {
    let service = MwService::from_kind(kind);
    let title = LabelRef(RecordId::new(
        service.map_or(kind.0.as_str(), |s| s.label_gmst()),
    ));
    let mut view = ServiceListView::new(provider, kind.clone(), title, ROW_EXTENT, VIEWPORT_ROWS)
        .with_confirm(confirm_time_passing);
    view.set_player_gold(rpg.state.pc.loadout.count(&id(GOLD)));
    if let Some(list) = rpg.service_list(PlayerId::HOST, provider) {
        view.set_entries(list.entries.clone());
    }
    view
}

/// Host events → the list screens: `ServiceOpened` of shape `List` opens one over the dialogue,
/// the list's events feed its view; the gold follows the RPG state.
#[allow(clippy::needless_pass_by_value, clippy::too_many_arguments)] // system parameters
fn list_events(
    mut commands: Commands,
    mut events: MessageReader<ServiceEventMessage>,
    rpg: Res<Rpg>,
    active: Option<Res<ActiveTalk>>,
    mut stack: ResMut<ScreenStackRes>,
    registry: Res<UiRegistryRes>,
    mut screens: Query<(&mut ServiceListScreen, &OpenServiceList)>,
    mut labels: Option<ResMut<ServiceLabels>>,
) {
    for ServiceEventMessage(event) in events.read().filter(|e| e.0.player == PlayerId::HOST) {
        match &event.kind {
            ServiceEventKind::ServiceOpened {
                kind,
                shape: ServiceShape::List,
                ..
            } => {
                let speaker = active.as_deref().map(|a| a.talk.speaker);
                if speaker != Some(event.provider) {
                    warn!(
                        "service list: {kind:?} with {:?} outside its conversation",
                        event.provider
                    );
                    continue;
                }
                if stack
                    .0
                    .top(Layer::Menu)
                    .is_none_or(|top| top.id.as_str() != "dialogue")
                {
                    info!("service list: {kind:?} not shown, another menu is over the dialogue");
                    continue;
                }
                let view = list_view(&rpg, event.provider, kind);
                if let Some(labels) = labels.as_deref_mut() {
                    let title = view.title().0.clone();
                    let fallback =
                        MwService::from_kind(kind).map_or(title.as_str(), |s| label_fallback(s));
                    let text = gmst_text(&rpg.content.records, title.as_str(), fallback);
                    labels.texts.insert(title.clone(), text);
                    if let Some(list) = rpg.service_list(PlayerId::HOST, event.provider) {
                        add_texts(labels, &rpg, &list.entries);
                    }
                }
                commands.spawn((
                    UiScreenRoot(SERVICE_SCREEN.into()),
                    ServiceListScreen::new(view),
                    OpenServiceList {
                        provider: event.provider,
                        kind: kind.clone(),
                    },
                ));
                if let Err(e) = stack.0.push(&registry.0, SERVICE_SCREEN) {
                    warn!("service list: {e:?}");
                }
            }
            kind => {
                if let (ServiceEventKind::EntriesListed { entries, .. }, Some(labels)) =
                    (kind, labels.as_deref_mut())
                {
                    add_texts(labels, &rpg, entries);
                }
                for (mut screen, open) in &mut screens {
                    if open.provider == event.provider {
                        screen.view.apply_event(kind);
                    }
                }
            }
        }
    }
    if rpg.is_changed() {
        let gold = rpg.state.pc.loadout.count(&id(GOLD));
        for (mut screen, _) in &mut screens {
            if screen.view.player_gold() != gold {
                screen.view.set_player_gold(gold);
            }
        }
    }
}

/// List screens follow their dialogue: a closed dialogue (goodbye) or a closed view (Back)
/// closes the screen, and a root off the stack despawns.
#[allow(clippy::needless_pass_by_value)] // system parameters
fn close_list(
    mut commands: Commands,
    mut stack: ResMut<ScreenStackRes>,
    roots: Query<(Entity, &ServiceListScreen), With<OpenServiceList>>,
) {
    for (e, screen) in &roots {
        if !stack.0.is_open("dialogue") || screen.view.is_closed() {
            stack.0.close(SERVICE_SCREEN);
        }
        if !stack.0.is_open(SERVICE_SCREEN) {
            commands.entity(e).despawn();
        }
    }
}

#[cfg(test)]
mod tests {
    //! Training over the dialogue, headless: the host's dialogue, list and stat systems over
    //! synthetic records and sheets, exact assertions (prices against `mw_rpg`'s A1 rule with the
    //! host's own barter terms).
    use std::sync::Arc;

    use bevy_time::Time;
    use mw_game_core::player::HOST_KEY;
    use mw_rpg::{MwTradeHost, NoSheets, TrainingIn, skill_entry, training_price};
    use mw_stats::{MwGmst, MwTable, SkillInfo, class_skill, openmw_rules};
    use sf_records::RecordDb;
    use sf_rpg::Attrs;
    use sf_rpg::service::{EntryId, EntryRefusal, ServiceEvent, ServiceRequest};
    use sf_rpg_ui::{ActionId, DialogueRequest, ListNav, ListOutcome, RowReason};
    use sf_sky_bevy::GameClock;
    use sf_sky_bevy::sf_ids::clock::MS_PER_HOUR;
    use sf_stats_bevy::sf_stats::{Inputs, Sheet, StatOp, StatOwner};
    use sf_stats_bevy::{SfStatsPlugin, StatRules, StatTableRes, Stats};
    use sf_ui_screens::DialogueRequestEvent;
    use tes3::esp::{
        GameSetting, GameSettingValue, GlobalValue, GlobalVariable, Npc, ServiceFlags,
    };

    use super::super::services::{OpenList, advance_sky, sync_clock};
    use super::super::{PERSUASION_ACTION, View, close_screens, registry, send_stat_commands};
    use super::*;
    use crate::stats::MwStats;

    const TRAINER: ActorKey = ActorKey(5);
    const LONG_BLADE: usize = 5;
    const DESTRUCTION: usize = 10;
    const SPEECHCRAFT: usize = 25;

    /// `trainer` (trains, 100 gold), the five calendar globals at 9:00 on 16 Last Seed 427,
    /// `sSkillLongblade` = "Long Blade".
    fn records() -> RecordDb {
        let mut db = RecordDb::new();
        mw_records::register_kinds(&mut db).unwrap();
        let src = db.add_source("test", &[]);
        let mut trainer = Npc {
            id: "trainer".into(),
            name: "Trainer".into(),
            ..Default::default()
        };
        trainer.ai_data.services = ServiceFlags::OFFERS_TRAINING;
        trainer.data.gold = 100;
        trainer.data.disposition = 50;
        db.define(src, id("trainer"), trainer).unwrap();
        db.define(src, id(GOLD), tes3::esp::MiscItem::default())
            .unwrap();
        for (name, value) in [
            ("GameHour", GlobalValue::Float(9.0)),
            ("DaysPassed", GlobalValue::Short(1)),
            ("Day", GlobalValue::Short(16)),
            ("Month", GlobalValue::Short(7)),
            ("Year", GlobalValue::Short(427)),
        ] {
            let g = GlobalVariable {
                id: name.into(),
                value,
                ..Default::default()
            };
            db.define(src, id(&name.to_lowercase()), g).unwrap();
        }
        let label = GameSetting {
            id: "sSkillLongblade".into(),
            value: GameSettingValue::String("Long Blade".into()),
            ..Default::default()
        };
        db.define(src, id("sSkillLongblade"), label).unwrap();
        db
    }

    /// Every skill governed by Strength.
    fn table() -> MwTable {
        MwTable::with_skills(std::array::from_fn(|_| SkillInfo {
            governing: 0,
            specialization: 0,
            use_values: [1.0, 0.0, 0.0, 0.0],
        }))
        .unwrap()
    }

    /// Trainer: Long Blade, Destruction, Speechcraft 60, Personality 50. Player: Strength 30,
    /// Personality 50, Long Blade 20 (a major skill), Destruction 30 (= Strength), Speechcraft 60.
    fn sheets(t: &MwTable) -> (Sheet, Sheet) {
        let rules = openmw_rules(t, &MwGmst::vanilla());
        let skill = |n: usize, value| StatOp::SetBase {
            stat: t.idx.skills[n],
            value,
        };
        let personality = StatOp::SetBase {
            stat: t.idx.attributes[6],
            value: 50.0,
        };
        let make = |owner, ops: &[StatOp]| {
            let inputs = Inputs {
                is_player: matches!(owner, StatOwner::Player(_)),
                ..Inputs::default()
            };
            let (sheet, rejected) = mw_stats::new_sheet(t, &rules, owner, &inputs, ops);
            assert_eq!(rejected, 0);
            sheet
        };
        let trainer = make(
            StatOwner::Actor(TRAINER),
            &[
                personality.clone(),
                skill(LONG_BLADE, 60.0),
                skill(DESTRUCTION, 60.0),
                skill(SPEECHCRAFT, 60.0),
            ],
        );
        let player = make(
            StatOwner::Player(HOST_KEY),
            &[
                personality,
                StatOp::SetBase {
                    stat: t.idx.attributes[0],
                    value: 30.0,
                },
                StatOp::SetBase {
                    stat: t.idx.class_skills[LONG_BLADE],
                    value: class_skill::MAJOR,
                },
                skill(LONG_BLADE, 20.0),
                skill(DESTRUCTION, 30.0),
                skill(SPEECHCRAFT, 60.0),
            ],
        );
        (player, trainer)
    }

    /// An app with the dialogue's, the list's and the stats' host systems: the player (`gold`
    /// gold) talking to the trainer, a sky clock at its default.
    fn app(gold: u32) -> (App, Entity) {
        let db = Arc::new(records());
        let mut rpg = Rpg::new(db.clone()).unwrap();
        assert!(rpg.add_actor(TRAINER, &id("trainer"), "Balmora"));
        rpg.state
            .pc
            .loadout
            .add(id(GOLD), gold, Attrs::new())
            .unwrap();
        let (talk, view) = rpg.open(TRAINER, &NoSheets).unwrap();
        let t = table();
        let (player, trainer) = sheets(&t);
        let rules = openmw_rules(&t, &MwGmst::vanilla());
        let mut app = App::new();
        app.add_plugins(SfStatsPlugin)
            .add_message::<ServiceEventMessage>()
            .add_message::<ServiceScreenEvent>()
            .add_message::<DialogueRequestEvent>()
            .insert_resource(StatTableRes(Arc::new(t.table.clone())))
            .insert_resource(StatRules(Arc::new(rules)))
            .insert_resource(MwStats {
                table: Arc::new(t),
                gmst: MwGmst::vanilla(),
                db: db.clone(),
            })
            .insert_resource(Time::<()>::default())
            .insert_resource(GameClock::default())
            .insert_resource(rpg)
            .insert_resource(UiRegistryRes(registry()))
            .init_resource::<ScreenStackRes>()
            .init_resource::<ServiceLabels>()
            .add_systems(
                Update,
                (
                    sync_clock,
                    host_dialogue,
                    close_screens,
                    send_stat_commands,
                    relist_services,
                    send_service_events,
                    advance_sky,
                )
                    .chain(),
            );
        add(&mut app);
        app.world_mut().spawn(Stats::new(
            StatOwner::Player(HOST_KEY),
            player,
            Inputs {
                is_player: true,
                ..Inputs::default()
            },
        ));
        app.world_mut().spawn(Stats::new(
            StatOwner::Actor(TRAINER),
            trainer,
            Inputs::default(),
        ));
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

    fn training() -> DialogueRequest {
        DialogueRequest::Action {
            action: ActionId::new(MwService::Training.id()),
            option: None,
        }
    }

    fn say(app: &mut App, screen: Entity, request: DialogueRequest) {
        app.world_mut()
            .write_message(DialogueRequestEvent { screen, request });
        app.update();
    }

    fn is_open(app: &App, id: &str) -> bool {
        app.world().resource::<ScreenStackRes>().0.is_open(id)
    }

    fn list(app: &mut App) -> Option<Entity> {
        let mut q = app
            .world_mut()
            .query_filtered::<Entity, With<OpenServiceList>>();
        q.iter(app.world()).next()
    }

    fn view(app: &mut App, screen: Entity) -> &mut ServiceListView {
        &mut app
            .world_mut()
            .get_mut::<ServiceListScreen>(screen)
            .unwrap()
            .into_inner()
            .view
    }

    fn send(app: &mut App, screen: Entity, outcome: ListOutcome) {
        let intent = match outcome {
            ListOutcome::Request(r) => ServiceIntent::Request(r),
            ListOutcome::Close => ServiceIntent::Close,
        };
        app.world_mut()
            .write_message(ServiceScreenEvent { screen, intent });
        app.update();
    }

    /// One stats fixed tick: the queued stat commands apply.
    fn stat_tick(app: &mut App) {
        app.world_mut().run_schedule(FixedFirst);
        app.world_mut().run_schedule(FixedUpdate);
    }

    fn sheet(app: &mut App, owner: StatOwner) -> Sheet {
        let mut q = app.world_mut().query::<&Stats>();
        q.iter(app.world())
            .find(|s| s.owner == owner)
            .unwrap()
            .sheet
            .clone()
    }

    fn gold(app: &App) -> u64 {
        app.world()
            .resource::<Rpg>()
            .state
            .pc
            .loadout
            .count(&id(GOLD))
    }

    fn global(app: &App, name: &str) -> Option<f32> {
        app.world()
            .resource::<Rpg>()
            .state
            .globals
            .get(&id(name))
            .copied()
    }

    /// Training, then the list screen over the dialogue.
    fn open_list(app: &mut App, dialogue: Entity) -> Entity {
        say(app, dialogue, training());
        app.update();
        let screen = list(app).expect("Training opened the list screen");
        let layer: Vec<_> = app
            .world()
            .resource::<ScreenStackRes>()
            .0
            .layer(Layer::Menu)
            .iter()
            .map(|s| s.id.as_str().to_owned())
            .collect();
        assert_eq!(layer, ["dialogue", SERVICE_SCREEN], "over the dialogue");
        screen
    }

    /// The price `mw.services.training` gives for a player base skill with the host's terms.
    fn price(app: &mut App, pc_base: f32, trainer: f32) -> i64 {
        let t = table();
        let (player, npc) = (
            sheet(app, StatOwner::Player(HOST_KEY)),
            sheet(app, StatOwner::Actor(TRAINER)),
        );
        let sheets = OneSheets {
            table: t,
            player,
            npc,
        };
        let rpg = app.world().resource::<Rpg>();
        let v = View {
            c: &rpg.content,
            s: &rpg.state,
            sheets: &sheets,
        };
        let terms = v.barter_terms(PlayerId::HOST, TRAINER).unwrap();
        let input = TrainingIn::new(terms, pc_base, trainer, 30.0, &MwGmst::vanilla());
        i64::from(training_price(&input).price)
    }

    struct OneSheets {
        table: MwTable,
        player: Sheet,
        npc: Sheet,
    }

    impl mw_rpg::MwSheets for OneSheets {
        fn table(&self) -> Option<&MwTable> {
            Some(&self.table)
        }

        fn player(&self, _player: PlayerId) -> Option<&Sheet> {
            Some(&self.player)
        }

        fn actor(&self, _actor: ActorKey) -> Option<&Sheet> {
            Some(&self.npc)
        }
    }

    fn label(s: &str) -> LabelRef {
        LabelRef(id(s))
    }

    #[test]
    fn a_trainer_shows_a_training_button() {
        let (app, dialogue) = app(1000);
        let actions: Vec<_> = app
            .world()
            .get::<DialogueScreen>(dialogue)
            .unwrap()
            .view
            .actions()
            .iter()
            .map(|a| (a.id.clone(), a.label.clone()))
            .collect();
        assert_eq!(
            actions,
            [
                (ActionId::new(PERSUASION_ACTION), "Persuasion".into()),
                (ActionId::new("mw.training"), "Training".into())
            ]
        );
    }

    #[test]
    fn the_list_shows_the_trainers_best_three_with_openmw_prices() {
        let (mut app, dialogue) = app(1000);
        let screen = open_list(&mut app, dialogue);
        let prices = [
            price(&mut app, 20.0, 60.0),
            price(&mut app, 30.0, 60.0),
            price(&mut app, 60.0, 60.0),
        ];
        assert!(prices.iter().all(|p| *p > 0));
        let v = view(&mut app, screen);
        assert_eq!(v.title(), &label("sServiceTrainingTitle"));
        assert_eq!(v.player_gold(), 1000);
        let order = v.order().to_vec();
        assert_eq!(
            order,
            [
                skill_entry(LONG_BLADE),
                skill_entry(DESTRUCTION),
                skill_entry(SPEECHCRAFT)
            ]
        );
        let rows: Vec<_> = order
            .iter()
            .map(|e| {
                let r = v.row(e).unwrap();
                (r.entry.price, r.reason.clone())
            })
            .collect();
        assert_eq!(
            rows,
            [
                (prices[0], None),
                (
                    prices[1],
                    Some(RowReason::Refused(label("sNotifyMessage17")))
                ),
                (
                    prices[2],
                    Some(RowReason::Refused(label("sServiceTrainingWords")))
                ),
            ]
        );
        assert_eq!(
            v.row(&order[0]).unwrap().entry.detail,
            EntryDetail::Stat {
                stat: id("longblade"),
                current: 20,
                next: 21
            }
        );
        let labels = app.world().resource::<ServiceLabels>();
        assert_eq!(labels.text(&id("sServiceTrainingTitle")), "Training");
        assert_eq!(
            (
                labels.text(&id("sSkillLongblade")),
                labels.text(&id("longblade"))
            ),
            ("Long Blade".into(), "Long Blade".into())
        );
        let host = app.world().resource::<Rpg>();
        assert_eq!(
            host.service_list(PlayerId::HOST, TRAINER)
                .map(|l| l.kind.clone()),
            Some(MwService::Training.kind())
        );
    }

    #[test]
    fn training_raises_the_skill_moves_the_gold_and_passes_two_hours() {
        let (mut app, dialogue) = app(1000);
        let screen = open_list(&mut app, dialogue);
        let long_blade = skill_entry(LONG_BLADE);
        let paid = view(&mut app, screen).row(&long_blade).unwrap().entry.price;
        let pool = app.world().resource::<Rpg>().services.gold(TRAINER);
        assert_eq!(
            pool,
            Some(100),
            "the trainer's purse opened at its base gold"
        );
        let before = app.world().resource::<GameClock>().now;
        // A asks first (2 hours pass), yes buys.
        assert_eq!(view(&mut app, screen).activate(&long_blade), None);
        assert_eq!(view(&mut app, screen).asking(), Some(&long_blade));
        let buy = view(&mut app, screen).confirm().expect("yes buys");
        send(&mut app, screen, buy);
        assert_eq!(gold(&app), 1000 - u64::try_from(paid).unwrap());
        assert_eq!(
            app.world().resource::<Rpg>().services.gold(TRAINER),
            Some(100 + paid),
            "the price goes into the trainer's pool"
        );
        // 9:00 on 16 Last Seed + 2 h: the MWScript calendar and the game clock.
        assert_eq!(
            ["gamehour", "dayspassed", "day", "month", "year"].map(|g| global(&app, g)),
            [Some(11.0), Some(1.0), Some(16.0), Some(7.0), Some(427.0)]
        );
        let now = app.world().resource::<GameClock>().now;
        assert_eq!(now.0 - before.0, 2 * MS_PER_HOUR, "the game clock skipped exactly 2 h");
        // The stamps read the clock (synced before the next dialogue host run).
        app.update();
        assert_eq!(app.world().resource::<Rpg>().game_time(), sf_rpg::GameTime::from(now));
        // The raise applies on the next stats tick: Long Blade 21, one major-skill level-up
        // point (`iLevelUpMajorMult` 1), as a trainer level-up.
        let t = table();
        let before = sheet(&mut app, StatOwner::Player(HOST_KEY));
        stat_tick(&mut app);
        let after = sheet(&mut app, StatOwner::Player(HOST_KEY));
        let read = |s: &Sheet, stat| s.view(&t.table).base(stat);
        assert_eq!(
            (
                read(&before, t.idx.skills[LONG_BLADE]),
                read(&after, t.idx.skills[LONG_BLADE])
            ),
            (20.0, 21.0)
        );
        assert_eq!(
            (
                read(&before, t.idx.level_progress),
                read(&after, t.idx.level_progress)
            ),
            (0.0, 1.0)
        );
        // The list re-lists from the raised sheet, still open over the dialogue.
        app.update();
        app.update();
        let now = gold(&app);
        let v = view(&mut app, screen);
        assert_eq!(
            v.row(&long_blade).unwrap().entry.detail,
            EntryDetail::Stat {
                stat: id("longblade"),
                current: 21,
                next: 22
            }
        );
        assert_eq!(v.player_gold(), now);
        assert!(matches!(
            v.toast(),
            Some(sf_rpg_ui::ListToast::Bought { price, .. }) if *price == paid
        ));
        assert!(is_open(&app, SERVICE_SCREEN) && is_open(&app, "dialogue"));
    }

    /// `BuyEntry` for `entry` straight from the screen's root; the events it produced.
    fn buy(app: &mut App, screen: Entity, entry: EntryId) -> Vec<ServiceEvent> {
        let request = ServiceRequest::BuyEntry {
            provider: TRAINER,
            kind: MwService::Training.kind(),
            entry,
        };
        let mut seen = app
            .world_mut()
            .resource_mut::<Messages<ServiceEventMessage>>()
            .get_cursor_current();
        send(app, screen, ListOutcome::Request(request));
        let messages = app.world().resource::<Messages<ServiceEventMessage>>();
        seen.read(messages).map(|m| m.0.clone()).collect()
    }

    fn refused(events: &[ServiceEvent]) -> Vec<EntryRefusal> {
        events
            .iter()
            .filter_map(|e| match &e.kind {
                ServiceEventKind::EntryRefused { reason, .. } => Some(reason.clone()),
                _ => None,
            })
            .collect()
    }

    #[test]
    fn training_is_refused_when_the_trainer_is_not_better_or_the_attribute_caps_it() {
        let (mut app, dialogue) = app(1000);
        let screen = open_list(&mut app, dialogue);
        // Disabled rows don't buy from the screen...
        assert_eq!(
            view(&mut app, screen).activate(&skill_entry(SPEECHCRAFT)),
            None
        );
        assert_eq!(view(&mut app, screen).asking(), None);
        // ...and the host refuses a forged request with the same reasons.
        let not_better = buy(&mut app, screen, skill_entry(SPEECHCRAFT));
        let capped = buy(&mut app, screen, skill_entry(DESTRUCTION));
        assert_eq!(
            (refused(&not_better), refused(&capped)),
            (
                vec![EntryRefusal::Refused(label("sServiceTrainingWords"))],
                vec![EntryRefusal::Refused(label("sNotifyMessage17"))]
            )
        );
        let not_offered = buy(&mut app, screen, skill_entry(0));
        assert_eq!(refused(&not_offered), [EntryRefusal::Unknown]);
        assert_eq!(gold(&app), 1000, "nothing paid");
        assert_eq!(global(&app, "gamehour"), None, "no time passed");
        assert!(
            app.world()
                .resource::<Messages<sf_stats_bevy::StatCommand>>()
                .is_empty()
        );
    }

    #[test]
    fn training_is_refused_when_the_gold_is_short() {
        let (mut app, dialogue) = app(10);
        let screen = open_list(&mut app, dialogue);
        let long_blade = skill_entry(LONG_BLADE);
        let v = view(&mut app, screen);
        assert!(v.row(&long_blade).unwrap().entry.price > 10);
        assert_eq!(
            v.row(&long_blade).unwrap().reason,
            Some(RowReason::TooExpensive)
        );
        assert_eq!(v.activate(&long_blade), None, "a greyed row");
        let events = buy(&mut app, screen, long_blade);
        assert_eq!(refused(&events), [EntryRefusal::TooExpensive]);
        assert_eq!(gold(&app), 10);
    }

    #[test]
    fn back_closes_the_list_back_to_the_dialogue() {
        let (mut app, dialogue) = app(1000);
        let screen = open_list(&mut app, dialogue);
        let close = view(&mut app, screen)
            .nav(ListNav::Back)
            .expect("Back closes");
        send(&mut app, screen, close);
        app.update();
        assert!(!is_open(&app, SERVICE_SCREEN) && is_open(&app, "dialogue"));
        assert!(list(&mut app).is_none(), "its root despawned");
        assert!(
            app.world().get_resource::<ActiveTalk>().is_some(),
            "the conversation goes on"
        );
        assert_eq!(
            app.world()
                .resource::<Rpg>()
                .service_list(PlayerId::HOST, TRAINER),
            None::<&OpenList>
        );
        // Training again opens a fresh list.
        let again = open_list(&mut app, dialogue);
        assert_eq!(view(&mut app, again).order().len(), 3);
    }

    #[test]
    fn goodbye_closes_the_list_and_the_dialogue() {
        let (mut app, dialogue) = app(1000);
        open_list(&mut app, dialogue);
        say(&mut app, dialogue, DialogueRequest::Goodbye);
        app.update();
        assert!(!is_open(&app, SERVICE_SCREEN) && !is_open(&app, "dialogue"));
        assert!(list(&mut app).is_none(), "the list root despawned");
        assert!(app.world().get_resource::<ActiveTalk>().is_none());
        assert_eq!(
            app.world()
                .resource::<Rpg>()
                .service_list(PlayerId::HOST, TRAINER),
            None::<&OpenList>,
            "goodbye closed the host's list"
        );
    }
}
