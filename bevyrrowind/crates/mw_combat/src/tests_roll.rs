//! The sword sheet's roll through `sf_action::step` and the shared Dodge/Sprint button
//! (`InputSampler`, fed the last step's `buffering` as the input shell does): a mashed roll
//! chains at its Dodge cancel, a press inside an attack's Dodge cancel rolls on the press tick,
//! a hold from free movement still sprints. Exact ticks; no combat rules (stamina is
//! `sf_combat`'s).

use glam::Vec2;
use sf_action::{
    ActionClass, ActionOutput, ActionState, BASE_RULES, CharacterParams, ClockTime, InputSampler, Mode, Origin, RuleKey, RuleSet, RuleSetBuilder,
    Sense, SlotInfo, SlotTable, StepCtx, TapHold, Tick,
};
use sf_combat::{CompiledSheet, SheetDef, WindowDef};

use super::{SWORD_1H, first_candidates};

struct Rig {
    sheet: CompiledSheet,
    rules: RuleSet,
    slots: SlotTable,
    params: CharacterParams,
    state: ActionState,
    out: ActionOutput,
    input: InputSampler,
    button: usize,
    tick: Tick,
}

impl Rig {
    fn new() -> Self {
        let sheet = sf_combat::load(SWORD_1H, &first_candidates).expect("sword_1h compiles");
        let mut b = RuleSetBuilder::new();
        b.add_ron(&Origin::Framework, BASE_RULES);
        sheet.add_rules(&mut b, &Origin::Mod("mw".into()));
        let rules = b.build().expect("rules build");
        let base = SlotTable::new()
            .with("loco.move", SlotInfo::looping())
            .with("air.fall", SlotInfo::looping())
            .with("air.jump", SlotInfo::once(0.3))
            .with("air.land.normal", SlotInfo::once(0.25))
            .with("air.land.hard", SlotInfo::once(0.5));
        let slots = sheet.fill(base);
        let mut state = ActionState::new(RuleKey::of("loco"));
        state.mode.moveset = Some(sheet.key);
        state.mode.combat = true;
        let mut input = InputSampler::new();
        let button = input.bind(TapHold::DODGE_SPRINT);
        let mut rig = Self { sheet, rules, slots, params: CharacterParams::default(), state, out: ActionOutput::default(), input, button, tick: Tick(0) };
        for _ in 0..5 {
            rig.step();
        }
        assert_eq!(rig.state.mov, RuleKey::of("loco"));
        rig
    }

    fn key(&self, mov: &str) -> RuleKey {
        self.sheet.rule_key(mov)
    }

    /// One fixed tick: the last output's buffer into the sampler, sample, step.
    fn step(&mut self) {
        self.tick = self.tick.next();
        self.input.set_buffering(self.out.buffering);
        let cmd = self.input.sample(self.tick, None);
        let sense = Sense { grounded: true, ..Sense::default() };
        let ctx = StepCtx { rules: &self.rules, slots: &self.slots, params: &self.params, source: sf_action::CharacterId(1) };
        sf_action::step(&ctx, &mut self.state, &cmd, &sense, &mut self.out);
    }

    fn started(&self, mov: &str) -> bool {
        self.out.started && self.state.mov == self.key(mov)
    }

}

/// The first tick (move ticks after its start) a window of `mov` containing `class` opens.
fn opens(mov: &str, class: ActionClass, cancel: bool) -> u32 {
    let ron = ron::Options::default().with_default_extension(ron::extensions::Extensions::IMPLICIT_SOME);
    let def: SheetDef = ron.from_str(SWORD_1H).unwrap();
    let start = def.moves[mov]
        .windows
        .iter()
        .find_map(|w| match w {
            WindowDef::Cancel(c, s, _) if cancel && c.contains(&class) => Some(*s),
            WindowDef::Accept(c, s, _) if !cancel && c.contains(&class) => Some(*s),
            _ => None,
        })
        .unwrap_or_else(|| panic!("{mov}: no window for {class:?}"));
    ClockTime::from_secs(start).0.div_ceil(ClockTime::from_ticks(1).0)
}

/// Mashing the button through a roll (press one tick, release the next) chains a second roll
/// on the roll's Dodge cancel tick, and a third on the second's.
#[test]
fn mashing_dodge_during_a_roll_chains_rolls_at_the_dodge_cancel() {
    let mut r = Rig::new();
    let cancel = opens("roll", ActionClass::Dodge, true);
    let accept = opens("roll", ActionClass::Dodge, false);
    assert!(accept < cancel, "the roll buffers Dodge before its cancel ({accept} < {cancel})");
    // From free movement the tap rolls on release.
    r.input.press_shared(r.button);
    r.step();
    assert_eq!(r.state.mov, RuleKey::of("loco"));
    r.input.release_shared(r.button);
    r.step();
    assert!(r.started("roll"), "tap from loco rolls on release");
    let mut starts = vec![r.tick];
    for k in 0..(3 * cancel) {
        if k % 2 == 0 {
            r.input.press_shared(r.button);
        } else {
            r.input.release_shared(r.button);
        }
        r.step();
        if r.started("roll") {
            starts.push(r.tick);
        }
    }
    let gaps: Vec<u32> = starts.windows(2).map(|w| w[1].since(w[0])).collect();
    assert!(gaps.len() >= 2, "rolls started at {starts:?}");
    assert!(gaps.iter().all(|&g| g == cancel), "roll-to-roll {gaps:?} ticks, Dodge cancel at {cancel}");
}

/// Inside an attack's Dodge cancel the press itself rolls (on its tick, the button still down);
/// inside its accept but before the cancel, the held press is buffered and rolls at the cancel.
#[test]
fn a_press_during_an_attacks_dodge_window_rolls_without_waiting_for_the_release() {
    let cancel = opens("light1", ActionClass::Dodge, true);
    let accept = opens("light1", ActionClass::Dodge, false);
    for (press_at, rolls_at) in [(cancel + 3, cancel + 3), (u32::midpoint(accept, cancel), cancel)] {
        let mut r = Rig::new();
        r.input.press(ActionClass::Light);
        r.step();
        r.input.release(ActionClass::Light);
        assert!(r.started("light1"));
        let light = r.tick;
        while r.tick.since(light) + 1 < press_at {
            r.step();
        }
        r.input.press_shared(r.button);
        let rolled = (0..cancel + 5).find_map(|_| {
            r.step();
            if r.started("roll") {
                return Some(r.tick);
            }
            assert_eq!(r.state.mov, r.key("light1"), "tick {:?}", r.tick);
            None
        });
        let rolled = rolled.expect("rolled with the button held").since(light);
        assert_eq!(rolled, rolls_at, "press on light1 tick {press_at}: rolled on its tick {rolled}");
    }
}

/// From free movement, the button held with the stick sprints (no roll).
#[test]
fn a_hold_from_neutral_still_sprints() {
    let mut r = Rig::new();
    r.input.set_stick(Vec2::Y);
    r.input.press_shared(r.button);
    let ticks = u32::from(TapHold::DODGE_SPRINT.ticks);
    for k in 1..=ticks + 10 {
        r.step();
        assert_ne!(r.state.mov, r.key("roll"), "sample {k}");
        assert_eq!(r.state.mode.mode == Mode::FreeRun, k >= ticks, "sample {k}");
    }
    r.input.release_shared(r.button);
    r.step();
    assert_ne!(r.state.mov, r.key("roll"));
    assert_eq!(r.state.mode.mode, Mode::Normal);
}
