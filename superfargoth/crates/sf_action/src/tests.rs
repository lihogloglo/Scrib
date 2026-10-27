//! Scripted-input tests (action.md "Input buffering and first response: tests").

use glam::{Vec2, Vec3};

use super::*;

const DT: f32 = 1.0 / 60.0;

fn slots() -> SlotTable {
    SlotTable::new()
        .with("loco.move", SlotInfo::looping())
        .with("air.fall", SlotInfo::looping())
        .with("air.jump", SlotInfo::once(0.3))
        .with("air.land.normal", SlotInfo::once(0.25))
        .with("air.land.hard", SlotInfo::once(0.5))
}

fn base() -> RuleSet {
    RuleSet::from_ron(BASE_RULES).expect("base rules load")
}

/// A ballistic stand-in for the kinematic mover (A1 tests the decisions, not the mover): flat
/// ground at y = 0 for z < `edge`, a pit (`floor`) beyond it. Planar velocity follows the intent.
#[derive(Clone, Debug)]
struct Body {
    pos: Vec3,
    vy: f32,
    grounded: bool,
    edge: f32,
    floor: f32,
}

impl Body {
    fn at(y: f32) -> Self {
        Self { pos: Vec3::new(0.0, y, 0.0), vy: 0.0, grounded: y <= 0.0, edge: f32::INFINITY, floor: -50.0 }
    }

    fn ground(&self) -> f32 {
        if self.pos.z < self.edge { 0.0 } else { self.floor }
    }

    fn apply(&mut self, out: &ActionOutput) {
        if let Some(v) = out.launch {
            self.vy = v;
            self.grounded = false;
        }
        self.pos += out.intent.desired_velocity * DT;
        if self.grounded && self.ground() < self.pos.y - 0.01 {
            self.grounded = false;
        }
        if !self.grounded {
            self.vy -= 9.81 * DT;
            self.pos.y += self.vy * DT;
            if self.pos.y <= self.ground() {
                self.pos.y = self.ground();
                self.vy = 0.0;
                self.grounded = true;
            }
        }
    }

    fn sense(&self) -> Sense {
        Sense { grounded: self.grounded, vertical_speed: self.vy, planar_speed: 0.0, yaw: 0.0, focus: None, obstacle: None, ..Sense::default() }
    }
}

struct Sim {
    rules: RuleSet,
    slots: SlotTable,
    params: CharacterParams,
    state: ActionState,
    body: Body,
    out: ActionOutput,
    tick: Tick,
    focus: Option<Focus>,
}

impl Sim {
    fn new(rules: RuleSet, body: Body) -> Self {
        let state = ActionState::new(RuleKey::of("loco"));
        Self { rules, slots: slots(), params: CharacterParams::default(), state, body, out: ActionOutput::default(), tick: Tick(0), focus: None }
    }

    fn base(body: Body) -> Self {
        Self::new(base(), body)
    }

    /// One tick with `cmd` (its tick is overwritten).
    fn run(&mut self, mut cmd: Command) -> &ActionOutput {
        self.tick = self.tick.next();
        cmd.tick = self.tick;
        let ctx = StepCtx { rules: &self.rules, slots: &self.slots, params: &self.params, source: CharacterId(7) };
        let sense = Sense { focus: self.focus, ..self.body.sense() };
        step(&ctx, &mut self.state, &cmd, &sense, &mut self.out);
        self.body.apply(&self.out);
        &self.out
    }

    fn idle(&mut self) -> &ActionOutput {
        self.run(Command::default())
    }

    fn press(&mut self, c: ActionClass) -> &ActionOutput {
        self.run(cmd_press(c))
    }

    fn rule(&self) -> RuleKey {
        self.state.mov
    }
}

fn cmd_press(c: ActionClass) -> Command {
    let mut cmd = Command::default();
    cmd.buttons.pressed.insert(c);
    cmd.buttons.held.insert(c);
    cmd
}

fn started(out: &ActionOutput, id: &str) -> bool {
    out.events.iter().any(|e| e.kind == EventKind::MoveStart(RuleKey::of(id)))
}

// ---- Loading -----------------------------------------------------------------------------

#[test]
fn base_rules_load_and_know_their_slots() {
    let rules = base();
    assert_eq!(rules.len(), 6);
    assert!(rules.check_slots(&slots()).is_empty(), "{:?}", rules.check_slots(&slots()));
    let missing = rules.check_slots(&SlotTable::new());
    assert!(missing.iter().any(|e| e.message.contains("air.land.hard")), "{missing:?}");
    // Priority order: landing before jump before interact before locomotion.
    let order: Vec<&str> = rules.iter().map(CompiledRule::id).collect();
    assert_eq!(order, ["land.hard", "land.normal", "air.jump", "interact.use", "loco", "air.fall"]);
}

#[test]
fn load_errors_name_the_mod_and_rule() {
    let bad = r#"[
        (id: "my.land", kind: Air, category: Landing, slots: ["x"]),
        (id: "my.jump", kind: Air, category: Jump, slots: ["x"], exits: [(on: End, to: Rule("nowhere"))]),
        (id: "my.band", kind: Air, category: Jump, slots: ["x"], entry: (classes: [Jump], speed: Some((3.0, 1.0)))),
        (id: "my.auto", kind: Air, category: Passover, slots: ["x"], entry: (auto_in: [FreeRun])),
        (id: "my.move", kind: Air, category: Jump, slots: ["x"], entry: (classes: [Move])),
    ]"#;
    let errors = RuleSetBuilder::new().add_ron(&Origin::Framework, BASE_RULES).add_ron(&Origin::Mod("Parkour+".into()), bad).build().unwrap_err();
    let text: Vec<String> = errors.iter().map(ToString::to_string).collect();
    assert_eq!(errors.len(), 5, "{text:#?}");
    assert!(text[0].starts_with("mod 'Parkour+', rule 'my.land': category Landing is framework-only"), "{text:#?}");
    assert!(text.iter().any(|t| t.contains("exit to unknown rule 'nowhere'")));
    assert!(text.iter().any(|t| t.contains("speed band")));
    // Unknown fields are load errors too (no silent typos).
    let typo = RuleSetBuilder::new().add_ron(&Origin::Mod("T".into()), r#"[(id: "a", kind: Air, category: Jump, rnak: 3)]"#).build();
    assert!(typo.unwrap_err()[0].to_string().contains("rnak"));
    // A set without an automatic Locomotion rule has no fallback.
    assert!(RuleSet::from_ron(r#"[(id: "j", kind: Air, category: Jump, entry: (classes: [Jump]))]"#).is_err());
}

#[test]
fn a_later_rule_with_the_same_id_replaces_the_earlier() {
    let patch = r#"[(id: "air.jump", kind: Air, category: Jump, slots: ["air.jump"], entry: (classes: [Jump], states: [Grounded]), launch: Some(Speed(2.0)))]"#;
    let rules = RuleSetBuilder::new().add_ron(&Origin::Framework, BASE_RULES).add_ron(&Origin::Mod("Moon".into()), patch).build().unwrap();
    assert_eq!(rules.len(), 6);
    let mut sim = Sim::new(rules, Body::at(0.0));
    assert_eq!(sim.press(ActionClass::Jump).launch, Some(2.0));
}

// ---- First response, jump, coyote, landing ---------------------------------------------

#[test]
fn jump_first_response_on_the_press_tick() {
    let mut sim = Sim::base(Body::at(0.0));
    sim.idle();
    let out = sim.press(ActionClass::Jump).clone();
    assert!(out.started && started(&out, "air.jump"));
    let v = out.launch.expect("launch");
    assert!((v - (2.0f32 * 9.81).sqrt()).abs() < 1e-4, "jump by height: 1 m → {v}");
    // Rises, falls, lands with a normal landing, back to locomotion when it ends.
    let mut land = None;
    for _ in 0..120 {
        let out = sim.idle();
        if out.events.iter().any(|e| e.kind == EventKind::Land) {
            land = Some(started(out, "land.normal"));
        }
    }
    assert_eq!(land, Some(true));
    assert_eq!(sim.rule(), RuleKey::of("loco"));
}

/// Ticks from a jump press to touchdown (inclusive), with the stand-in body.
fn landing_tick() -> u32 {
    let mut sim = Sim::base(Body::at(0.0));
    sim.press(ActionClass::Jump);
    loop {
        if sim.idle().events.iter().any(|e| e.kind == EventKind::Land) {
            return sim.tick.0;
        }
    }
}

#[test]
fn jump_buffer_fires_on_the_landing_tick_within_its_window() {
    let land = landing_tick();
    let n = CharacterParams::default().jump_buffer_ticks;
    for before in [1, u32::from(n), u32::from(n) + 1] {
        let mut sim = Sim::base(Body::at(0.0));
        sim.press(ActionClass::Jump);
        while sim.tick.0 + 1 < land - before {
            sim.idle();
        }
        sim.press(ActionClass::Jump);
        assert!(sim.out.launch.is_none(), "an airborne press doesn't jump");
        while sim.tick.0 + 1 < land {
            sim.idle();
        }
        let out = sim.idle().clone();
        assert_eq!(out.tick.0, land);
        let fired = out.launch.is_some();
        assert_eq!(fired, before <= u32::from(n), "press {before} ticks before landing");
        if fired {
            // Landing first, then the jump in the settle round, on the same tick.
            let kinds: Vec<EventKind> = out.events.iter().map(|e| e.kind).collect();
            let l = kinds.iter().position(|k| *k == EventKind::MoveStart(RuleKey::of("land.normal"))).unwrap();
            let j = kinds.iter().position(|k| *k == EventKind::MoveStart(RuleKey::of("air.jump"))).unwrap();
            assert!(l < j, "{kinds:?}");
        }
    }
}

fn walk_off_edge() -> Sim {
    let mut body = Body::at(0.0);
    body.edge = 0.2;
    let mut sim = Sim::base(body);
    let walk = Command { stick: Vec2::new(0.0, 1.0), ..Command::default() };
    while sim.body.grounded {
        sim.run(walk);
    }
    sim
}

#[test]
fn coyote_jump_counts_airborne_ticks() {
    let n = u32::from(CharacterParams::default().coyote_ticks);
    for late in [0, n - 2, n - 1] {
        let mut sim = walk_off_edge();
        let off = sim.tick.0 + 1; // first tick the step senses the air
        sim.idle();
        assert_eq!(sim.rule(), RuleKey::of("air.fall"), "walking off falls");
        for _ in 1..=late {
            sim.idle();
        }
        let out = sim.press(ActionClass::Jump).clone();
        assert_eq!(out.tick.0 - off, late + 1);
        assert_eq!(out.launch.is_some(), late + 1 < n, "jump on airborne tick {}", late + 1);
    }
}

#[test]
fn hard_landing_commits_and_a_latched_jump_fires_at_its_end() {
    let mut sim = Sim::base(Body::at(6.0));
    sim.state.body.grounded = false;
    let mut t = 0;
    while !started(sim.idle(), "land.hard") {
        t += 1;
        assert!(t < 300);
    }
    let land = sim.tick.0;
    let walk = Command { stick: Vec2::new(0.0, 1.0), ..Command::default() };
    assert_eq!(sim.run(walk).intent.desired_velocity, Vec3::ZERO, "no steering in a hard landing");
    sim.press(ActionClass::Jump);
    let end = land + 30; // 0.5 s
    while sim.tick.0 + 1 < end {
        assert!(sim.idle().launch.is_none(), "committed: nothing cancels it");
    }
    let out = sim.idle().clone();
    assert_eq!(out.tick.0, end);
    assert!(out.launch.is_some() && started(&out, "air.jump"), "the latched jump fires on the end tick");
}

// ---- Two-stage buffer and priorities ---------------------------------------------------

const COMBAT: &str = r#"[
    (id: "light", kind: Combat, category: Attack, slots: ["atk.light"], entry: (classes: [Light], states: [Grounded])),
    (id: "light.alt", kind: Combat, category: Attack, rank: -1, slots: ["atk.light"], entry: (classes: [Light])),
    (id: "dodge", kind: Combat, category: Dodge, slots: ["dodge"], entry: (classes: [Dodge])),
    (id: "guard", kind: Combat, category: Guard, slots: ["guard"], entry: (classes: [Guard])),
    (id: "jump.vault", kind: Traversal, category: Traversal, slots: [], entry: (classes: [Jump], modes: [Normal, FreeRun], need_forward: true)),
]"#;

fn combat() -> Sim {
    let rules = RuleSetBuilder::new().add_ron(&Origin::Framework, BASE_RULES).add_ron(&Origin::Mod("Combat".into()), COMBAT).build().unwrap();
    let mut sim = Sim::new(rules, Body::at(0.0));
    let light = SlotInfo::once(0.8)
        .with(Window::secs(WindowKind::Accept(ClassSet::LATCHABLE), 0.2, 0.8))
        .with(Window::secs(WindowKind::Cancel(ClassSet::of(&[ActionClass::Light, ActionClass::Dodge, ActionClass::Guard])), 0.4, 0.8));
    sim.slots = std::mem::take(&mut sim.slots).with("atk.light", light).with("dodge", SlotInfo::once(0.5)).with("guard", SlotInfo::looping());
    sim
}

/// Starts a light attack; returns its start tick.
fn attack(sim: &mut Sim) -> u32 {
    assert!(started(sim.press(ActionClass::Light), "light"));
    sim.tick.0
}

#[test]
fn press_inside_accept_fires_exactly_on_the_cancel_tick() {
    let mut sim = combat();
    let s = attack(&mut sim);
    while sim.tick.0 < s + 14 {
        sim.idle();
    }
    sim.press(ActionClass::Dodge); // tick s+15: 0.25 s, inside accept
    assert_eq!(sim.state.buffer.latched, ClassSet::of(&[ActionClass::Dodge]));
    while sim.tick.0 + 1 < s + 24 {
        assert_eq!(sim.idle().rule, RuleKey::of("light"));
    }
    let out = sim.idle().clone(); // s+24: 0.4 s, cancel opens
    assert!(started(&out, "dodge"), "{:?}", out.events);
    assert!(sim.state.buffer.latched.is_empty(), "starting a move clears the buffer");
}

#[test]
fn press_before_accept_is_lost() {
    let mut sim = combat();
    let s = attack(&mut sim);
    while sim.tick.0 < s + 5 {
        sim.idle();
    }
    sim.press(ActionClass::Dodge); // 0.1 s: early mashing does nothing
    for _ in 0..60 {
        assert!(!started(sim.idle(), "dodge"));
    }
}

#[test]
fn guard_fires_from_the_buffer_only_if_still_held() {
    for keep_holding in [false, true] {
        let mut sim = combat();
        let s = attack(&mut sim);
        while sim.tick.0 < s + 14 {
            sim.idle();
        }
        sim.press(ActionClass::Guard);
        let hold = |held| {
            let mut c = Command::default();
            if held {
                c.buttons.held.insert(ActionClass::Guard);
            }
            c
        };
        let mut guarded = false;
        while sim.tick.0 < s + 30 {
            guarded |= started(sim.run(hold(keep_holding)), "guard");
        }
        assert_eq!(guarded, keep_holding);
    }
}

#[test]
fn priority_list_dodge_beats_attack_and_rank_orders_a_category() {
    let mut sim = combat();
    let mut both = cmd_press(ActionClass::Light);
    both.buttons.pressed.insert(ActionClass::Dodge);
    assert!(started(sim.run(both), "dodge"));
    // In the air `light` (Grounded) refuses and the lower-ranked `light.alt` wins.
    let mut sim = combat();
    sim.press(ActionClass::Jump);
    sim.idle();
    let out = sim.press(ActionClass::Light).clone();
    assert!(started(&out, "light.alt"));
    assert!(out.refusals.contains(&Refusal { rule: RuleKey::of("light"), reason: Refused::State }));
}

#[test]
fn hit_stop_rate_slows_the_window_clock() {
    let mut sim = combat();
    let s = attack(&mut sim);
    sim.state.clock.rate = RATE_ONE / 2;
    while sim.tick.0 < s + 30 {
        sim.idle();
    }
    sim.press(ActionClass::Dodge); // 0.25 s on the action clock after 30 ticks at half rate
    while sim.tick.0 + 1 < s + 48 {
        assert!(!started(sim.idle(), "dodge"));
    }
    assert!(started(sim.idle(), "dodge"), "cancel at 0.4 s action time = 48 ticks at half rate");
}

#[test]
fn modes_sneak_held_sprint_free_run_and_mode_gated_rules() {
    let mut sim = combat();
    let mut crouch = cmd_press(ActionClass::Sneak);
    sim.run(crouch);
    assert_eq!(sim.state.mode.mode, Mode::Sneak);
    crouch.buttons.pressed = ClassSet::EMPTY;
    let mut sneak_walk = Command { stick: Vec2::new(0.0, 1.0), ..crouch };
    assert_eq!(sim.run(sneak_walk).intent.gait, sf_controller::Gait::Sneak);
    // Sneak never triggers traversal: `jump.vault` is refused, the plain `air.jump` wins.
    let mut jump = cmd_press(ActionClass::Jump);
    jump.buttons.held.insert(ActionClass::Sneak);
    let out = sim.run(jump).clone();
    assert!(out.refusals.contains(&Refusal { rule: RuleKey::of("jump.vault"), reason: Refused::Mode }));
    assert!(started(&out, "air.jump"));
    // Sprint held with the stick: free-run over sneak; back to sneak while it is still held,
    // Normal once it is released too.
    let mut sprint = sneak_walk;
    sprint.buttons.held.insert(ActionClass::Sprint);
    assert_eq!(sim.run(sprint).intent.gait, sf_controller::Gait::Sprint);
    assert_eq!(sim.state.mode.mode, Mode::FreeRun);
    sim.run(sneak_walk);
    assert_eq!(sim.state.mode.mode, Mode::Sneak);
    sneak_walk.buttons.held = ClassSet::EMPTY;
    assert_eq!(sim.run(sneak_walk).intent.gait, sf_controller::Gait::Run);
    assert_eq!(sim.state.mode.mode, Mode::Normal);
}

// ---- Interaction prompt ----------------------------------------------------------------

#[test]
fn press_to_use_prompt_and_activation() {
    let door = Focus { target: TargetId(42), tag: TagId::of("door") };
    let mut sim = Sim::base(Body::at(0.0));
    // No focus: refused, and the stale press doesn't fire when a focus appears later.
    let out = sim.press(ActionClass::Interact).clone();
    assert!(out.prompt.is_none());
    assert!(out.refusals.contains(&Refusal { rule: RuleKey::of("interact.use"), reason: Refused::Focus }));
    sim.focus = Some(door);
    let out = sim.idle().clone();
    assert!(!out.events.iter().any(|e| matches!(e.kind, EventKind::Activate(_))));
    assert_eq!(out.prompt, Some(Prompt { target: TargetId(42), tag: door.tag, rule: RuleKey::of("interact.use"), ready: true }));
    // Press: an instant move activates and settles back to locomotion on the same tick.
    let out = sim.press(ActionClass::Interact).clone();
    let act = out.events.iter().find(|e| matches!(e.kind, EventKind::Activate(_))).expect("activate");
    assert_eq!((act.target, act.source, act.tick), (Some(TargetId(42)), CharacterId(7), out.tick));
    assert!(started(&out, "interact.use") && started(&out, "loco"));
    assert_eq!(out.rule, RuleKey::of("loco"));
    // In the air the prompt is not offered (the rule needs Grounded).
    sim.press(ActionClass::Jump);
    assert!(sim.idle().prompt.is_none());
}

// ---- Input sampling, frame-rate independence, net feed ---------------------------------

#[test]
fn shared_button_taps_dodge_on_release_and_holds_sprint() {
    let mut s = InputSampler::new();
    let b = s.bind_tap_hold(ActionClass::Dodge, ActionClass::Sprint, 10);
    s.press_shared(b);
    let c = s.sample(Tick(1), None);
    assert!(!c.pressed(ActionClass::Dodge) && !c.held(ActionClass::Sprint));
    s.release_shared(b);
    let c = s.sample(Tick(2), None);
    assert!(c.pressed(ActionClass::Dodge) && c.released(ActionClass::Dodge) && !c.held(ActionClass::Dodge));
    // Held past the threshold: Sprint, and no Dodge on release.
    s.press_shared(b);
    let mut sprint_at = None;
    for t in 3..20 {
        let c = s.sample(Tick(t), None);
        assert!(!c.pressed(ActionClass::Dodge));
        if c.held(ActionClass::Sprint) && sprint_at.is_none() {
            sprint_at = Some(t);
            assert!(c.pressed(ActionClass::Sprint));
        }
    }
    assert_eq!(sprint_at, Some(12));
    s.release_shared(b);
    let c = s.sample(Tick(20), None);
    assert!(c.released(ActionClass::Sprint) && !c.pressed(ActionClass::Dodge) && c.hold_ticks[ActionClass::Sprint.index()] == 0);
}

#[test]
fn a_press_and_release_between_ticks_still_counts() {
    let mut s = InputSampler::new();
    s.press(ActionClass::Jump);
    s.release(ActionClass::Jump);
    let c = s.sample(Tick(1), PlayerId::new(1));
    assert!(c.pressed(ActionClass::Jump) && !c.held(ActionClass::Jump) && c.player == PlayerId::new(1));
    assert!(!s.sample(Tick(2), None).pressed(ActionClass::Jump), "consumed by the first tick");
}

/// Runs a scripted session at a render rate: device events at fixed times (µs), frames at
/// `fps`, fixed ticks at 60 Hz run inside frames. Returns (tick, rule, launched) per tick.
fn session(fps: u64) -> Vec<(u32, RuleKey, bool)> {
    // (time µs, press?, class): Jump at 1/3 s, a buffered Jump just before landing, Interact on the ground.
    let script: [(u64, bool, ActionClass); 6] = [
        (333_333, true, ActionClass::Jump),
        (366_667, false, ActionClass::Jump),
        (1_200_000, true, ActionClass::Jump),
        (1_233_333, false, ActionClass::Jump),
        (2_666_666, true, ActionClass::Interact),
        (2_700_000, false, ActionClass::Interact),
    ];
    let mut sim = Sim::base(Body::at(0.0));
    sim.focus = Some(Focus { target: TargetId(1), tag: TagId::of("lever") });
    let mut input = InputSampler::new();
    let mut log = Vec::new();
    let (mut next_event, mut ticks) = (0, 0u64);
    let frame_us = 1_000_000 / fps;
    let mut now = 0u64;
    while now < 3_000_000 {
        while next_event < script.len() && script[next_event].0 <= now {
            let (_, down, c) = script[next_event];
            if down { input.press(c) } else { input.release(c) }
            next_event += 1;
        }
        while (ticks + 1) * 1_000_000 / 60 <= now {
            ticks += 1;
            let cmd = input.sample(Tick(0), None);
            let out = sim.run(cmd);
            log.push((out.tick.0, out.rule, out.launch.is_some()));
        }
        now += frame_us;
    }
    log.truncate(170);
    log
}

#[test]
fn identical_outcomes_at_30_60_144_fps() {
    let a = session(30);
    assert!(a.iter().filter(|e| e.2).count() >= 2, "both jumps (one from the jump buffer) fired");
    assert_eq!(a, session(60));
    assert_eq!(a, session(144));
}

#[test]
fn command_feed_orders_dedupes_and_repeats_then_goes_empty() {
    let mut feed = CommandFeed::new();
    let mut c = Command { player: PlayerId::new(2), stick: Vec2::new(0.0, 1.0), ..cmd_press(ActionClass::Guard) };
    c.tick = Tick(11);
    let mut c10 = c;
    c10.tick = Tick(10);
    feed.insert(c);
    feed.insert(c10);
    feed.insert(c); // redundant resend
    assert_eq!(feed.take(Tick(10)).tick, Tick(10));
    assert_eq!(feed.take(Tick(11)), c);
    feed.insert(c10); // late, already taken
    for t in 12..12 + u32::from(REPEAT_MISSING_TICKS) {
        let r = feed.take(Tick(t));
        assert_eq!((r.tick, r.stick), (Tick(t), c.stick), "repeat the stick and view");
        assert!(r.held(ActionClass::Guard) && !r.pressed(ActionClass::Guard), "held stays, no new presses");
    }
    let r = feed.take(Tick(18));
    assert_eq!((r.stick, r.buttons, r.player), (Vec2::ZERO, ButtonEdges::default(), PlayerId::new(2)));
}

// ---- Net readiness: serde, determinism -------------------------------------------------

#[test]
fn state_command_and_events_round_trip_through_postcard() {
    let mut sim = combat();
    attack(&mut sim);
    sim.press(ActionClass::Dodge);
    sim.state.timers.redo.push((RuleKey::of("dodge"), Tick(99)));
    let bytes = postcard::to_allocvec(&sim.state).unwrap();
    assert!(bytes.len() < 64, "snapshot stays small: {} bytes", bytes.len());
    assert_eq!(postcard::from_bytes::<ActionState>(&bytes).unwrap(), sim.state);
    let cmd = Command { tick: Tick(5), player: PlayerId::new(3), stick: Vec2::new(0.5, -0.5), lock_request: LockRequest::Next(LockSide::Right), ..cmd_press(ActionClass::Up) };
    assert_eq!(postcard::from_bytes::<Command>(&postcard::to_allocvec(&cmd).unwrap()).unwrap(), cmd);
    let ev = sim.out.events.clone();
    assert_eq!(postcard::from_bytes::<Vec<ActionEvent>>(&postcard::to_allocvec(&ev).unwrap()).unwrap(), ev);
    let rule = base().iter().next().unwrap().rule.clone();
    assert_eq!(ron::from_str::<MoveRule>(&ron::to_string(&rule).unwrap()).unwrap(), rule);
}

#[test]
fn same_commands_give_bit_identical_states() {
    let run = || {
        let mut sim = combat();
        let mut snaps = Vec::new();
        for t in 0..200u16 {
            let mut cmd = Command { stick: Vec2::new((f32::from(t) * 0.1).sin(), 1.0) * 0.8, ..Command::default() };
            for (k, c) in [(3, ActionClass::Jump), (7, ActionClass::Light), (11, ActionClass::Dodge), (13, ActionClass::Sneak)] {
                if t % k == 0 {
                    cmd.buttons.pressed.insert(c);
                }
            }
            sim.run(cmd);
            snaps.push(postcard::to_allocvec(&sim.state).unwrap());
        }
        snaps
    };
    assert_eq!(run(), run());
}

#[test]
fn rule_keys_are_stable_fnv1a() {
    // Pinned: keys go on the wire and into saves, so the hash must never change.
    assert_eq!(fnv1a32(b""), 0x811c_9dc5);
    assert_eq!(fnv1a32(b"a"), 0xe40c_292c);
    assert_eq!(ClockTime::from_secs(0.4), ClockTime::from_ticks(24));
}

/// Flat ground at y = 0 for the real mover.
struct FlatGround;

impl sf_controller::HeightSurface for FlatGround {
    fn height(&self, _p: sf_coords::WorldPos) -> Option<f32> {
        Some(0.0)
    }
    fn normal(&self, _p: sf_coords::WorldPos) -> Option<Vec3> {
        Some(Vec3::Y)
    }
    fn raycast(&self, origin: sf_coords::WorldPos, dir: Vec3, max: f32) -> Option<(f32, Vec3)> {
        let t = -sf_coords::WorldPos::ORIGIN.delta_f32(origin).y / dir.y;
        (dir.y.abs() > 1e-6 && (0.0..=max).contains(&t)).then_some((t, Vec3::Y))
    }
}

#[test]
fn a_jump_launch_flies_the_real_mover_to_its_apex() {
    use sf_controller::{KinematicMover, Locomotion, MoveInput, MoverParams, RootMotion, TerrainProbe};
    let mut sim = Sim::base(Body::at(0.0));
    sim.idle();
    let out = sim.press(ActionClass::Jump).clone();
    let (probe, params) = (TerrainProbe(FlatGround), MoverParams::default());
    let mut loco = Locomotion::new(sf_coords::WorldPos::ORIGIN, 0.0);
    let mut mover = KinematicMover::new();
    let mut step = |loco: &mut Locomotion| mover.step(MoveInput { loco, intent: out.intent, params: &params, root: RootMotion::default() }, &probe, DT);
    step(&mut loco);
    assert!(loco.grounded.is_some());
    // The shell's whole mapping of `ActionOutput::launch`.
    loco.launch(out.launch.expect("launch"));
    let (mut apex, mut ticks) = (0.0f32, 0);
    while ticks < 120 {
        step(&mut loco);
        ticks += 1;
        apex = apex.max(sf_coords::WorldPos::ORIGIN.delta_f32(loco.position).y);
        if loco.grounded.is_some() {
            break;
        }
    }
    // Jump by height: the base rule asks for a 1 m apex.
    assert!((apex - 1.0).abs() < 0.02 && loco.grounded.is_some() && ticks > 50, "apex {apex} after {ticks} ticks");
}
