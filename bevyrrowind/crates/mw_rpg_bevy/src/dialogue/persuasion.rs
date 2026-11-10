//! Persuasion in the game host (ROADMAP 2.7, rpg-adapter.md "Persuasion"): `OpenMW`'s
//! `DialogueManager::persuade`, `goodbyeSelected` and `updateOriginalDisposition`
//! (`apps/openmw/mwdialogue/dialoguemanagerimp.cpp` at `ca5e2576bfeb8fa64b53fdc8a31fefc360decc2f`)
//! over `mw_rpg`'s pure formula ([`persuasion_change`]) and session ([`mw_rpg::PersuasionSession`]).
//!
//! Host-side and net-ready: the roll, the disposition change, the gold and the Speechcraft use
//! are decided and applied here, for the conversation's player (`ctx.player`). The roll is
//! [`persuasion_roll`] over (speaker, player, the caller's tick, the session's attempt count),
//! so every peer and a replay agree. The temporary change goes on the speaker's base at once
//! (`NpcStates`, as `OpenMW`'s `setBaseDisposition(mCurrentDisposition)`); [`Rpg::end_talk`]
//! (goodbye, or the window closed) keeps only the permanent part.
//!
//! The menu (`OpenMW`'s `PersuasionDialog` in `apps/openmw/mwgui/dialogue.cpp` and
//! `files/data/mygui/openmw_persuasion_dialog.layout`, same commit): a "Persuasion" action
//! button on the dialogue window, NPCs only and first (services follow it), whose submenu holds
//! the six attempts labelled from their GMSTs; a bribe the player can't pay is disabled
//! (`onOpen`: `playerGold >= 10`, ...). [`Rpg::refresh_actions`] rebuilds it after every
//! conversation step, so gold an attempt or a result script moved shows at once;
//! [`Rpg::route_action`] sends a pressed option to [`Rpg::persuade_and_reply`].

use bevy_log::prelude::*;
use mw_rpg::{
    MwSheets, PersuasionKind, PersuasionOutcome, Target, persuasion_change, persuasion_input, persuasion_roll,
    speechcraft_use,
};
use sf_records::{RecordDb, RecordId};
use sf_rpg::{ActorKey, Attrs, RpgCommand, SessionState};
use sf_rpg_ui::{ActionId, ActionOption, DialogueAction, DialogueRequest, DialogueView};
use sf_stats_bevy::StatCommand;
use sf_stats_bevy::sf_stats::{CauseKind, StatOwner};
use tes3::esp::{GameSetting, GameSettingValue};

use super::{Rpg, Talk, id, player_key};
use crate::inventory::GOLD;

/// The Persuasion button's [`ActionId`] (its options' ids are in [`PERSUASION_OPTIONS`]).
pub const PERSUASION_ACTION: &str = "persuasion";

/// The Persuasion submenu in `OpenMW`'s order (`openmw_persuasion_dialog.layout`): option id,
/// attempt, label GMST, the label without that GMST (Morrowind.esm's English value).
pub const PERSUASION_OPTIONS: [(&str, PersuasionKind, &str, &str); 6] = [
    ("admire", PersuasionKind::Admire, "sAdmire", "Admire"),
    ("intimidate", PersuasionKind::Intimidate, "sIntimidate", "Intimidate"),
    ("taunt", PersuasionKind::Taunt, "sTaunt", "Taunt"),
    ("bribe10", PersuasionKind::Bribe10, "sBribe 10 Gold", "Bribe 10 Gold"),
    ("bribe100", PersuasionKind::Bribe100, "sBribe 100 Gold", "Bribe 100 Gold"),
    ("bribe1000", PersuasionKind::Bribe1000, "sBribe 1000 Gold", "Bribe 1000 Gold"),
];

/// The attempt a Persuasion submenu option stands for.
pub fn persuasion_kind(option: &ActionId) -> Option<PersuasionKind> {
    PERSUASION_OPTIONS.iter().find(|&&(key, ..)| ActionId::new(key) == *option).map(|&(_, kind, ..)| kind)
}

/// A string GMST, trimmed (`fallback` when missing, not a string, or blank).
pub(super) fn gmst_text(db: &RecordDb, name: &str, fallback: &str) -> String {
    match db.get::<GameSetting>(&RecordId::new(name)).map(|g| &g.value) {
        Some(GameSettingValue::String(s)) if !s.trim().is_empty() => s.trim().to_owned(),
        _ => fallback.to_owned(),
    }
}

/// One attempt's result.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Persuaded {
    pub outcome: PersuasionOutcome,
    /// The roll (0..=99) drawn for it.
    pub roll: u8,
    /// The Persuasion DIAL the speaker answers with ("admire success", ...), a service topic:
    /// ask it through the dialogue path ([`Rpg::persuade_and_reply`] does).
    pub topic: RecordId,
}

/// Why the host refused an attempt (nothing changed).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PersuadeRefused {
    /// The conversation takes no topics now (a choice is pending, goodbye only, ended).
    NotTalking,
    /// The speaker is no registered NPC.
    NotNpc,
    /// No stat table, or the player's or the speaker's sheet is missing.
    NoStats,
    /// A bribe the player can't pay (`OpenMW`'s persuasion menu disables it).
    NotEnoughGold,
}

impl Rpg {
    /// One persuasion attempt by the conversation's player (`DialogueManager::persuade`): the
    /// roll, the speaker's disposition change (temporary now, permanent at [`Rpg::end_talk`]),
    /// the player's Speechcraft use (success: use 0, fail: use 1; a stat command), a successful
    /// bribe's gold from the player to the speaker, an Intimidate or Taunt success's Fight/Flee
    /// nudge. `tick`: the host's simulation tick at the request (seeds the roll with the speaker,
    /// the player and the attempt count). Returns the reply topic to show; nothing changes on a
    /// refusal.
    pub fn persuade(
        &mut self,
        talk: &mut Talk,
        kind: PersuasionKind,
        sheets: &dyn MwSheets,
        tick: u64,
    ) -> Result<Persuaded, PersuadeRefused> {
        if *talk.session.state() != SessionState::Talking {
            return Err(PersuadeRefused::NotTalking);
        }
        let ctx = talk.session.ctx().clone();
        let speaker = ctx.speaker;
        let actor = self.content.actors.get(&speaker).filter(|a| a.facts.npc).ok_or(PersuadeRefused::NotNpc)?;
        let price = u32::try_from(kind.gold()).unwrap_or(u32::MAX);
        // TODO(co-op): `RpgState::pc` is the host's loadout; a guest's needs its own.
        if self.state.pc.loadout.count(&id(GOLD)) < u64::from(price) {
            return Err(PersuadeRefused::NotEnoughGold);
        }
        let record_base = actor.base_disposition;
        let ai = self.ai_settings.get(&speaker).copied().unwrap_or(actor.ai);
        let disposition = {
            let cx = self.content.disposition_ctx(&self.state, sheets, ctx.player, speaker, actor);
            self.content.disposition.disposition(&cx)
        };
        let roll = persuasion_roll(speaker, ctx.player, tick, talk.persuasion.attempts);
        let input =
            persuasion_input(sheets, ctx.player, speaker, kind, disposition, ai, roll).ok_or(PersuadeRefused::NoStats)?;
        let outcome = persuasion_change(&input, &self.content.gmst);

        let base = record_base + self.state.npcs.modifier(speaker);
        let new_base = talk.persuasion.attempt(base, &outcome);
        self.state.npcs.set(speaker, record_base, new_base);
        if let (Some(t), Some(pk)) = (sheets.table(), player_key(ctx.player)) {
            let ops = vec![speechcraft_use(t, outcome.success)];
            self.stat_commands.push(StatCommand::new(StatOwner::Player(pk), CauseKind::Dialogue, ops));
        }
        if outcome.gold > 0 {
            let count = u32::try_from(outcome.gold).unwrap_or(0);
            if let Some((holder, inv)) = self.character(&ctx, &Target::Player) {
                let _ = inv.apply(RpgCommand::RemoveItem { holder, item: id(GOLD), count });
            }
            if let Some((holder, inv)) = self.character(&ctx, &Target::Speaker) {
                let _ = inv.apply(RpgCommand::AddItem { holder, item: id(GOLD), count, attrs: Attrs::new() });
            }
        }
        if let Some(nudge) = outcome.ai {
            debug!(
                "persuasion: {speaker:?}'s base Flee {} / Fight {} (TODO(AI settings): kept unsaved, nothing reads them yet)",
                nudge.flee, nudge.fight
            );
            self.ai_settings.insert(speaker, nudge);
        }
        Ok(Persuaded { outcome, roll, topic: id(kind.topic(outcome.success)) })
    }

    /// [`Rpg::persuade`], then the speaker's reply ("Admire Success", ...) through the
    /// conversation, as `persuade`'s `executeTopic`: shown in `view`, its result script run.
    pub fn persuade_and_reply(
        &mut self,
        talk: &mut Talk,
        view: &mut DialogueView,
        kind: PersuasionKind,
        sheets: &dyn MwSheets,
        tick: u64,
    ) -> Result<Persuaded, PersuadeRefused> {
        let persuaded = self.persuade(talk, kind, sheets, tick)?;
        self.request(talk, view, &DialogueRequest::Ask(persuaded.topic.clone()), sheets);
        Ok(persuaded)
    }

    /// The Persuasion button with its six options; `None` unless `speaker` is a registered NPC
    /// (`DialogueWindow` adds `sPersuasion` for NPCs only). Labels from `sPersuasion`,
    /// `sAdmire`, `sIntimidate`, `sTaunt`, `sBribe 10 Gold`, ...; a bribe is disabled while the
    /// player has less gold than it offers (`PersuasionDialog::onOpen`).
    /// TODO(co-op): the gold is the host's (`RpgState::pc`), as in [`Rpg::persuade`].
    pub fn persuasion_action(&self, speaker: ActorKey) -> Option<DialogueAction> {
        if !self.content.actors.get(&speaker).is_some_and(|a| a.facts.npc) {
            return None;
        }
        let db = &self.content.records;
        let gold = self.state.pc.loadout.count(&id(GOLD));
        let options = PERSUASION_OPTIONS
            .iter()
            .map(|&(option, kind, gmst, vanilla)| {
                let price = u64::try_from(kind.gold()).unwrap_or(u64::MAX);
                ActionOption::new(option, gmst_text(db, gmst, vanilla)).enabled(gold >= price)
            })
            .collect();
        let label = gmst_text(db, "sPersuasion", "Persuasion");
        Some(DialogueAction::new(PERSUASION_ACTION, label).with_submenu(options))
    }

    /// The dialogue window's action buttons for `talk`'s speaker: Persuasion (NPCs only), then
    /// one button per service offer, as `OpenMW`'s `updateTopicsPane` lists them
    /// ([`Rpg::service_actions`]). Called after every conversation step and every routed
    /// action, so a bribe's enabled state follows the player's gold.
    pub fn refresh_actions(&self, talk: &Talk, view: &mut DialogueView) {
        let mut actions: Vec<DialogueAction> = self.persuasion_action(talk.speaker).into_iter().collect();
        actions.extend(self.service_actions(talk));
        view.set_actions(actions);
    }

    /// A pressed action button or submenu option ([`DialogueRequest::Action`]): a Persuasion
    /// option runs [`Rpg::persuade_and_reply`] at `tick` and returns its result; a service
    /// button goes to the service host ([`Rpg::service_request`], its outcome in
    /// [`Rpg::take_service_events`]) and returns `None`, as does anything else (the Persuasion
    /// button alone, unknown ids: a logged no-op). The buttons are refreshed either way.
    pub fn route_action(
        &mut self,
        talk: &mut Talk,
        view: &mut DialogueView,
        action: &ActionId,
        option: Option<&ActionId>,
        sheets: &dyn MwSheets,
        tick: u64,
    ) -> Option<Result<Persuaded, PersuadeRefused>> {
        let kind = if *action == ActionId::new(PERSUASION_ACTION) { option.and_then(persuasion_kind) } else { None };
        let result = if let Some(kind) = kind {
            let result = self.persuade_and_reply(talk, view, kind, sheets, tick);
            match &result {
                Ok(p) => debug!("persuasion: {kind:?} on {:?}: roll {}, {:?}", talk.speaker, p.roll, p.outcome),
                Err(e) => info!("persuasion: {kind:?} on {:?} refused: {e:?}", talk.speaker),
            }
            Some(result)
        } else {
            let request = DialogueRequest::Action { action: action.clone(), option: option.cloned() };
            if !self.route_service_action(talk, view, &request, sheets, tick) {
                debug!("dialogue: action {action:?} (option {option:?}) has no handler");
            }
            None
        };
        self.refresh_actions(talk, view);
        result
    }

    /// The conversation ends (`goodbyeSelected`; also when its window closes without goodbye):
    /// the player's trade sessions close, and the speaker's base becomes the session's original plus its permanent changes, clamped so
    /// the derived disposition stays in 0..=100. A no-op without persuasion, and when called
    /// again.
    pub fn end_talk(&mut self, talk: &mut Talk, sheets: &dyn MwSheets) {
        let speaker = talk.speaker;
        // The service windows close with the conversation; staged items never moved.
        self.sessions.close_player(talk.session.ctx().player);
        self.close_service_lists(talk.session.ctx().player);
        let Some(actor) = self.content.actors.get(&speaker).filter(|a| a.facts.npc) else { return };
        let record_base = actor.base_disposition;
        let zero = {
            let cx = self.content.disposition_ctx(&self.state, sheets, talk.session.ctx().player, speaker, actor);
            self.content.disposition.without_base(&cx)
        };
        let base = record_base + self.state.npcs.modifier(speaker);
        if let Some(new_base) = talk.persuasion.goodbye(base, zero) {
            self.state.npcs.set(speaker, record_base, new_base);
        }
    }
}

#[cfg(test)]
mod tests {
    //! Persuasion through the host, exact against `mw_rpg`'s pure functions (real data; skips
    //! when no Morrowind install is detected), the disposition model's fallback and the
    //! Persuasion menu (headless).
    use std::sync::Arc;

    use mw_game_core::player::HOST_KEY;
    use mw_install::MorrowindInstall;
    use mw_rpg::{
        ActorFacts, DispositionExtras, MwView, NoSheets, PersuasionSession, derived_disposition,
        derived_disposition_unclamped, disposition_input, session_perm_change,
    };
    use mw_stats::{MwGmst, MwTable};
    use sf_rpg::PlayerId;
    use sf_stats_bevy::sf_stats::{Inputs, Sheet};
    use tes3::esp::Npc;

    use super::super::{ContentDisposition, DerivedDisposition, DispositionCtx, DispositionModel, View};
    use super::*;
    use crate::stats::MwStats;

    const FARGOTH: ActorKey = ActorKey(1);

    /// A headless host over records with one NPC (`fargoth`, no script, no dialogue) and one
    /// label GMST (`sAdmire` = " Flatter ", trimmed on use; the others take their fallback),
    /// talking to Fargoth with no stats.
    fn headless() -> (Rpg, Talk, DialogueView) {
        let mut db = RecordDb::new();
        mw_records::register_kinds(&mut db).unwrap();
        let src = db.add_source("test", &[]);
        let npc = Npc { id: "fargoth".into(), name: "Fargoth".into(), ..Default::default() };
        db.define(src, id("fargoth"), npc).unwrap();
        let admire = GameSetting { id: "sAdmire".into(), value: GameSettingValue::String(" Flatter ".into()), ..Default::default() };
        db.define(src, id("sAdmire"), admire).unwrap();
        let mut rpg = Rpg::new(Arc::new(db)).unwrap();
        assert!(rpg.add_actor(FARGOTH, &id("fargoth"), "Seyda Neen"));
        let (talk, view) = rpg.open(FARGOTH, &NoSheets).unwrap();
        (rpg, talk, view)
    }

    /// The Persuasion button as the host should build it, bribes enabled as given.
    fn menu([b10, b100, b1000]: [bool; 3]) -> DialogueAction {
        DialogueAction::new(PERSUASION_ACTION, "Persuasion").with_submenu(vec![
            ActionOption::new("admire", "Flatter"),
            ActionOption::new("intimidate", "Intimidate"),
            ActionOption::new("taunt", "Taunt"),
            ActionOption::new("bribe10", "Bribe 10 Gold").enabled(b10),
            ActionOption::new("bribe100", "Bribe 100 Gold").enabled(b100),
            ActionOption::new("bribe1000", "Bribe 1000 Gold").enabled(b1000),
        ])
    }

    fn press(option: Option<&str>) -> DialogueRequest {
        DialogueRequest::Action { action: ActionId::new(PERSUASION_ACTION), option: option.map(ActionId::new) }
    }

    #[test]
    fn persuasion_menu_offers_the_six_attempts_to_an_npc() {
        let (rpg, _talk, view) = headless();
        // The new-game player has no gold: every bribe disabled, the button still live.
        let want = menu([false, false, false]);
        assert_eq!(view.actions(), std::slice::from_ref(&want));
        assert!(view.actions()[0].is_live());
        assert_eq!(rpg.persuasion_action(FARGOTH), Some(want));
        assert_eq!(rpg.persuasion_action(ActorKey(99)), None, "no menu for an unregistered speaker");
        let kinds: Vec<_> = PERSUASION_OPTIONS.iter().map(|&(key, ..)| persuasion_kind(&ActionId::new(key))).collect();
        assert_eq!(
            kinds,
            [
                PersuasionKind::Admire,
                PersuasionKind::Intimidate,
                PersuasionKind::Taunt,
                PersuasionKind::Bribe10,
                PersuasionKind::Bribe100,
                PersuasionKind::Bribe1000
            ]
            .map(Some)
        );
        assert_eq!(persuasion_kind(&ActionId::new("flirt")), None);
    }

    #[test]
    fn persuasion_bribes_follow_the_players_gold() {
        let (mut rpg, mut talk, mut view) = headless();
        rpg.state.pc.loadout.add(id(GOLD), 100, Attrs::new()).unwrap();
        assert_eq!(rpg.persuasion_action(FARGOTH), Some(menu([true, true, false])), "100 gold pays 10 and 100");
        // The window's buttons are refreshed after an attempt (refused here: no stats).
        let r = rpg.route_action(&mut talk, &mut view, &ActionId::new(PERSUASION_ACTION), Some(&ActionId::new("admire")), &NoSheets, 0);
        assert_eq!(r, Some(Err(PersuadeRefused::NoStats)));
        assert_eq!(view.actions(), std::slice::from_ref(&menu([true, true, false])));
        rpg.state.pc.loadout.add(id(GOLD), 900, Attrs::new()).unwrap();
        rpg.refresh_actions(&talk, &mut view);
        assert_eq!(view.actions(), std::slice::from_ref(&menu([true, true, true])), "1000 gold pays every bribe");
    }

    #[test]
    fn persuasion_menu_requests_land_in_persuade() {
        let (mut rpg, mut talk, mut view) = headless();
        let persuasion = ActionId::new(PERSUASION_ACTION);
        let route = |rpg: &mut Rpg, talk: &mut Talk, view: &mut DialogueView, action: &ActionId, option: Option<&str>| {
            rpg.route_action(talk, view, action, option.map(ActionId::new).as_ref(), &NoSheets, 0)
        };
        // Each refusal names a check inside `persuade`: the request got there.
        assert_eq!(route(&mut rpg, &mut talk, &mut view, &persuasion, Some("bribe10")), Some(Err(PersuadeRefused::NotEnoughGold)));
        assert_eq!(route(&mut rpg, &mut talk, &mut view, &persuasion, Some("admire")), Some(Err(PersuadeRefused::NoStats)));
        // The button alone, an unknown option, a service button: logged no-ops.
        assert_eq!(route(&mut rpg, &mut talk, &mut view, &persuasion, None), None);
        assert_eq!(route(&mut rpg, &mut talk, &mut view, &persuasion, Some("flirt")), None);
        assert_eq!(route(&mut rpg, &mut talk, &mut view, &ActionId::new("barter"), None), None);
        // Through the window's request path: no session step, nothing changed.
        let lines = talk.session.history().len();
        rpg.request_at(&mut talk, &mut view, &press(Some("taunt")), &NoSheets, 5);
        rpg.request(&mut talk, &mut view, &press(None), &NoSheets);
        assert_eq!((talk.session.history().len(), talk.persuasion.attempts), (lines, 0));
        assert_eq!(*talk.session.state(), SessionState::Talking);
        assert_eq!(rpg.state.npcs.modifier(FARGOTH), 0);
        // After goodbye the attempt is refused by the session check.
        rpg.request(&mut talk, &mut view, &DialogueRequest::Goodbye, &NoSheets);
        assert_eq!(route(&mut rpg, &mut talk, &mut view, &persuasion, Some("admire")), Some(Err(PersuadeRefused::NotTalking)));
    }

    #[test]
    fn derived_disposition_falls_back_to_content_without_stats() {
        let records = RecordDb::new();
        let facts = ActorFacts { npc: true, ..ActorFacts::default() };
        let cx = |base, modifier| DispositionCtx {
            sheets: &NoSheets,
            records: &records,
            speaker: FARGOTH,
            facts: &facts,
            player: PlayerId::HOST,
            player_race: None,
            base,
            modifier,
        };
        let model = DerivedDisposition { gmst: MwGmst::vanilla() };
        assert_eq!(model.disposition(&cx(40, -5)), 35);
        assert_eq!(model.disposition(&cx(40, 70)), 100);
        assert_eq!(model.disposition(&cx(10, -30)), 0);
        assert_eq!(model.without_base(&cx(40, 0)), 0);
        assert_eq!(ContentDisposition.disposition(&cx(40, -5)), 35);
    }

    /// The host player's and Fargoth's new sheets.
    struct TestSheets {
        table: Arc<MwTable>,
        player: Sheet,
        npc: Sheet,
    }

    impl MwSheets for TestSheets {
        fn table(&self) -> Option<&MwTable> {
            Some(&self.table)
        }

        fn player(&self, player: PlayerId) -> Option<&Sheet> {
            (player == PlayerId::HOST).then_some(&self.player)
        }

        fn actor(&self, actor: ActorKey) -> Option<&Sheet> {
            (actor == FARGOTH).then_some(&self.npc)
        }
    }

    fn test_sheets(db: &Arc<RecordDb>) -> TestSheets {
        let (gmst, _) = MwGmst::from_db(db);
        let stats = MwStats { table: Arc::new(MwTable::from_db(db).unwrap()), gmst, db: db.clone() };
        let rules = mw_stats::openmw_rules(&stats.table, &stats.gmst);
        let pc = Inputs { is_player: true, ..Inputs::default() };
        let player = mw_stats::new_sheet(&stats.table, &rules, StatOwner::Player(HOST_KEY), &pc, &stats.player_ops()).0;
        let npc_ops = stats.actor_ops(&id("fargoth")).unwrap();
        let npc = mw_stats::new_sheet(&stats.table, &rules, StatOwner::Actor(FARGOTH), &Inputs::default(), &npc_ops).0;
        TestSheets { table: stats.table, player, npc }
    }

    #[test]
    fn persuasion_through_the_host_matches_the_formula() {
        let Ok((install, _)) = MorrowindInstall::detect(None) else {
            eprintln!("no Morrowind install detected; skipping");
            return;
        };
        let vfs = mw_vfs::mount_install(&install, sf_vfs::Vfs::builder()).unwrap().vfs;
        let content = mw_records::load(&vfs, &install.game_files()).unwrap();
        let db = Arc::new(content.db);
        let sheets = test_sheets(&db);
        let gmst = MwGmst::from_db(&db).0;
        let mut rpg = Rpg::new(db.clone()).unwrap();
        assert!(rpg.add_actor(FARGOTH, &id("fargoth"), "Seyda Neen"));
        let input_at = |rpg: &Rpg, base: i32, modifier: i32| {
            let facts = &rpg.content.actors[&FARGOTH].facts;
            let extras = DispositionExtras { base, modifier, ..DispositionExtras::default() };
            disposition_input(&sheets, &db, PlayerId::HOST, None, facts, extras).unwrap()
        };
        let (mut talk, mut view) = rpg.open(FARGOTH, &sheets).unwrap();
        let record_base = rpg.content.actors[&FARGOTH].base_disposition;
        let modifier = rpg.state.npcs.modifier(FARGOTH);
        let disposition = derived_disposition(&input_at(&rpg, record_base, modifier), &gmst);
        eprintln!("Fargoth: record {record_base}, modifier {modifier}, derived {disposition}");
        // The filters read the derived value.
        let ctx = talk.session.ctx().clone();
        assert_eq!(MwView::disposition(&View { c: &rpg.content, s: &rpg.state, sheets: &sheets }, &ctx), disposition);

        // The new-game player has no gold.
        assert_eq!(rpg.persuade(&mut talk, PersuasionKind::Bribe10, &sheets, 7), Err(PersuadeRefused::NotEnoughGold));

        let tick = 1234;
        let roll = persuasion_roll(FARGOTH, PlayerId::HOST, tick, 0);
        let ai = rpg.content.actors[&FARGOTH].ai;
        let i = persuasion_input(&sheets, PlayerId::HOST, FARGOTH, PersuasionKind::Admire, disposition, ai, roll).unwrap();
        let expected = persuasion_change(&i, &gmst);
        let commands = rpg.stat_commands.len();
        let p = rpg.persuade(&mut talk, PersuasionKind::Admire, &sheets, tick).unwrap();
        eprintln!("admire: roll {roll}, {expected:?}");
        assert_eq!((p.outcome, p.roll), (expected, roll));
        assert_eq!(p.topic, id(if expected.success { "admire success" } else { "admire fail" }));
        assert_eq!(rpg.state.npcs.modifier(FARGOTH), modifier + expected.temp_change, "the temporary change at once");
        assert_eq!(rpg.stat_commands.len(), commands + 1, "one Speechcraft use");
        assert_eq!(talk.persuasion.attempts, 1);
        // The reply goes through the conversation as a service topic.
        let lines = talk.session.history().len();
        rpg.request(&mut talk, &mut view, &DialogueRequest::Ask(p.topic.clone()), &sheets);
        assert_eq!(talk.session.history().len(), lines + 1, "{:?}", talk.session.history().last());

        // Goodbye keeps the permanent part only, clamped by the zero-base derived value.
        let original = record_base + modifier;
        let perm = session_perm_change(expected.temp_change, expected.perm_change, original, 0);
        let zero = derived_disposition_unclamped(&input_at(&rpg, 0, 0), &gmst);
        rpg.request(&mut talk, &mut view, &DialogueRequest::Goodbye, &sheets);
        let want = if perm != 0 || expected.temp_change != 0 {
            (original + perm).clamp(-zero, 100 - zero) - record_base
        } else {
            modifier
        };
        assert_eq!(rpg.state.npcs.modifier(FARGOTH), want);
        assert_eq!(talk.persuasion, PersuasionSession { attempts: 1, ..PersuasionSession::default() });

        // A bribe the player can pay: the gold moves on success only.
        rpg.state.pc.loadout.add(id(GOLD), 100, Attrs::new()).unwrap();
        let (mut talk, mut view) = rpg.open(FARGOTH, &sheets).unwrap();
        let enabled = |view: &DialogueView| view.actions()[0].submenu.iter().map(|o| o.enabled).collect::<Vec<_>>();
        eprintln!("persuasion menu: {:?}", view.actions());
        assert_eq!(enabled(&view), [true, true, true, true, true, false]);
        let p = rpg.persuade(&mut talk, PersuasionKind::Bribe100, &sheets, tick).unwrap();
        let paid: u32 = if p.outcome.success { 100 } else { 0 };
        assert_eq!(p.outcome.gold, i32::try_from(paid).unwrap());
        assert_eq!(rpg.state.pc.loadout.count(&id(GOLD)), 100 - u64::from(paid));
        assert_eq!(rpg.state.inventories.get(&FARGOTH).map_or(0, |i| i.loadout.count(&id(GOLD))), u64::from(paid));

        // The menu's Admire through the window's request path: one more attempt, its reply
        // shown, the bribes following the gold left.
        let lines = talk.session.history().len();
        let admire = DialogueRequest::Action { action: ActionId::new(PERSUASION_ACTION), option: Some(ActionId::new("admire")) };
        rpg.request_at(&mut talk, &mut view, &admire, &sheets, tick + 1);
        assert_eq!(talk.persuasion.attempts, 2);
        assert_eq!(talk.session.history().len(), lines + 1, "{:?}", talk.session.history().last());
        assert_eq!(enabled(&view), [true, true, true, paid == 0, paid == 0, false]);
    }
}
