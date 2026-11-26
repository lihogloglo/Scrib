# Combat: NPC fighters (C6, C19)

Status (2026-11-26): spec C0b, nothing built. Split from [combat.md](combat.md) (size); it uses that spec's types and adds
none to its pipeline. Order and slices: [combat roadmap](../../../docs/roadmaps/combat.md) Stage 4 (decisions 8, 9, 10, 16,
18, 19). Research: `../../../docs/research/gameplay-2026-11-25/combat.md` §2, C-R1
`../../../docs/research/gameplay-2026-11-25/combat-veloren.md` §2, §4. Ledger `../../../docs/ledger/combat-feel.md`.

## Goal

An engaged NPC or creature fights well and fairly through the same `Command`s as the player: a few attackers at a time per
target while the rest circle (tokens), moves chosen by reach, angle and the target's state from a per-species table
(picker), defensive reactions by skill after a human-like delay, telegraphs long enough to read, cues every player gets, and
creatures with their own anatomy and boss phases. One pure, deterministic module, `sf_combat::npc` (Bevy-free, no new crate).

Non-goals: *whether* and *whom* to fight (AI15 combat drive, MA4 Fight rating; no escalation ladder, decision 9); morale,
yield and flee (AI37); ring-slot choice and navigation (AI36); alert sharing (AI36, perception); **view-aware behaviour**
(rejected by the owner 2026-11-26, decision 19: nothing here reads the camera, the view mode or a target's `Command.view`).

## The C6 ↔ AI15 boundary

- **AI15** (`sf_ai`, ai.md "Combat AI", AI roadmap decision 12): the combat drive, target choice, the HTN engage /
  reposition / search tasks, ring slots and their nav checks (AI36), morale (AI37). It owns locomotion whenever C6 says
  `Close` or `Wait`.
- **C6** (`sf_combat::npc`): for one fighter already engaged on one target, `decide` returns what to press this tick (attack,
  reaction or nothing) and the token request; `tokens::step` arbitrates; `npc::command` turns an intent into `Command` bits.
  Called by AI15's engage executor and by the duelist (C6e). `sf_combat` never depends on `sf_ai`.

## Awareness input (`unaware`, the backstab hook)

combat.md "Criticals and finishers" leaves `unaware` to the content. It is fed here, read-only for combat:
- Component `UnawareOf(SmallVec<[CharacterId; 4]>)` on a defender (`sf_combat_bevy`), sorted ids of attackers it is unaware
  of, written in a new set `CombatSystems::Sense` before `Gate` by whoever owns perception. No writer: never unaware (the
  framework default; a player body is never unaware).
- The AI-side writer (AI15 glue, `sf_ai_bevy`): attacker absent from the defender's `Awareness` entries, or its band
  `Unaware` or `Curious` (ai.md "Awareness"). Combat never writes awareness: a hit reaches perception as the C15 event and
  raises awareness on the next tick, so only the first strike can be a sneak backstab.
- Fills `Sense.critical` eligibility and `Contact.defender_state.unaware`. Host state in co-op (awareness is per player).

## Tokens (C6a)

Doom 2016's attack tokens [V GDC]. **View-independent**: no term reads the target's camera, view mode or facing.
- **Pools** keyed `(target: CharacterId, kind: TokenKind)`, `TokenKind { Melee, Ranged, Charged }` (Charged: lunges,
  leaps, charged heavies, big areas; Ranged: designed in C0c, used by C8e). Any fighter can be a target; a remote player
  is just another target. Each table row names its kind (or `None`: guard, dodge, step). One token per attack.
- **Request** before an attack's press; granted if the pool has room. Full: **steal** from the worst uncommitted holder
  when `placement(requester) > placement(holder) + steal_margin`. `placement = w_dist × clamp(1 - d / reach_max, 0, 1) +
  w_wait × min(waited_s / wait_cap_s, 1)` (`d` centre distance, `reach_max` of the requester's best legal move, `waited_s`
  since its last release or engagement). Committed = the holder's attack move has started; committed holders are never
  robbed. A counter (C6c) adds `w_counter`.
- **Hold and release**: held through the attack; released at **recovery start** (first tick after the move's last hit
  window) unless a chain continuation is already committed, capped at `max_chain` moves per hold (Doom releases at attack
  end; ER strings are one hold). Also released on: a `Medium`+ reaction or poise break, death, `PhysicsTakeover`, target
  lost or dead (pool dropped), no attack started within `grant_timeout_s`, held past `hold_max_s`. After a release the fighter
  waits `cooldown_s` before requesting the same pool (rotation, Arkham/AC queue shape).
- **Non-holders**: `decide` returns `Wait { ring_m }` with `ring_m = own reach_max + ring_margin_m`; AI15/AI36 put the
  fighter on a reachable ring slot and circle (no attack press). Reactions (guard, dodge) need no token.
- **Counts per difficulty**: `DifficultyRow.tokens { melee, ranged, charged }` (decision 16: tokens, skill, telegraph,
  damage; no rubber-banding).
- **Net and determinism**: `TokenBook` is host state, serde, not saved (rebuilt like AI plans). `tokens::step(book,
  requests, releases, tick)` runs once per fixed tick in `CombatSystems::Tokens` (before `Gate`): releases first, then
  requests sorted by `(target, kind, placement desc, requester id)`; ties to the lower id. Grants are visible to `decide`
  the same tick. Events `TokenEvent { Granted, Denied, Stolen { by }, Released(Reason), Expired }` go to AI15 and the debug
  trace.

## Move picker (C6b)

ER-style selection [M] with Veloren's gate fields (C-R1 §4): filter by legality, then a weighted, seeded choice.
- **Inputs** (`PickInput`): own fighter, `FightState`, `ActionState` (current move, clock, open `Accept`/`Cancel` windows),
  vitals; the delayed target view (C6c) `TargetView { dist_m, bearing_deg, height_m, speed_mps, phase, poise_broken,
  behind, guard_up }` with `TargetPhase { Idle, Startup { move, ticks_to_hit }, Active, Recovery { ticks_left }, Guarding,
  Dodging, Reacting(Tier), Down, Casting }` (from the target's `FightState`/`ActionState` and its compiled sheet); own
  cooldowns; `PickMemory`; token status; difficulty row; tick and fighter id.
- **Baked reach** per move at sheet compile (`MoveReach`): `startup_ticks` (start to first `Hit`), `active_ticks`,
  `recovery_ticks`, `travel_m` (root-motion travel to the last hit window, lunge included), `sweep_m` (farthest horizontal
  weapon point inside hit windows, socket track), `arc_deg` (swept yaw span plus `Turn` rate × startup). At pick time:
  `reach_max = sweep_m + travel_m + target hurt radius`, `reach_min` = the row's override or 0; legal when `reach_min ≤ d ≤
  reach_max` and `|bearing| ≤ arc_deg / 2`.
- **Legality**: startable now (an opener from `Loco`, or a chain `from` the running move inside its `Accept` window), off
  cooldown, `rules.can_start` true, row gates pass, token held or kind `None`, phase gate (boss phases).
- **Weight** = `weight × Π bias(gate) × repeat_mult^(times in memory) × punish`. **Punish rule**: target in `Recovery {
  ticks_left }`: rows with `startup_ticks + travel time ≤ ticks_left` × `punish_mult`, slower rows × `punish_slow_mult`.
  **Anti-spam** (ER adaptation): `PickMemory.habit { dodge, guard, parry }` counts the target's answers to this fighter's
  attacks (decaying over `habit_decay_s`); rows bias on them (`TargetHabit(Dodge)`: a delayed or sweeping swing).
- **Choice**: `sf_combat::roll(seed ^ SALT_PICK, fighter, tick)` over the summed weights (salts per stream: pick,
  react, aim; seed from the world's net seed). No legal row: `Close { to_m }` (the band of the best-weighted row out of
  reach) or `Wait`. Chains: a row's `chain` lists continuations, `on_hit` ones only after the previous move landed (ER).
- **Output** `NpcIntent { Attack { move, class, dir, hold_ticks }, React(Reaction), Close { to_m }, Wait { ring_m }, Hold }`;
  `npc::command` presses `class` with `stick` = the swing direction (decision 7), `hold_ticks` for charges, `view` at the
  aim point and `face_view` set (no lock-on needed).

Table shape (content RON, record kind `Tactics`, one per species or sheet; patchable like `Moveset`):

```ron
Tactics(
  sheet: "sword_1h", skill: 0.5,
  react: (guard: (0.1, 0.6), dodge: (0.05, 0.3), parry: (0.0, 0.25), counter: (0.1, 0.5)),
  rows: [
    (move: "light1", token: Melee, weight: 3.0, bias: [(TargetPhase(Recovery), 2.0)]),
    (move: "light2", token: Melee, weight: 2.0, chain_only: true),
    (move: "heavy", token: Melee, weight: 1.0, cooldown_s: 4.0, gates: [Not(TargetGuarding)]),
    (move: "thrust_lunge", token: Charged, weight: 1.0, band: (2.5, 5.0), cooldown_s: 6.0,
     bias: [(TargetHabit(Dodge), 2.0)], whiff: WallStagger),
  ],
  phases: [(when: HealthBelow(0.5), tactics: "sword_1h_enraged", enter: Some("roar"))],
)
```

## Skill reactions (C6c)

The NPC reads the player's `FightState`, which is the telegraph itself, so reading it is fair [research §2.4].
- **Delay**: `TargetTrace` keeps the last 32 tick snapshots of each target view; a fighter reads the snapshot `delay` ticks
  old, `delay = lerp(delay_max_s, delay_floor_s, skill)`, never under `delay_floor_s` (≥ 200 ms). Picker and reactions both
  read the delayed view; own state is current.
- **One roll per incoming swing** (`(attacker, swing start tick)`, remembered in `NpcState.reacted_to`) when the delayed view
  shows a swing whose `reach_max` and arc cover this fighter: `guard`, `dodge`, `parry`, or none; each odds pair `(lo, hi)`
  lerped by skill, partitioning [0, 1). Gates: `Unblockable` → no guard/parry (its share goes to dodge); `NoParry` → no
  parry; the sheet lacks the class or `can_start` refuses → none. Timing: guard pressed at once; parry pressed so
  `parry_ticks` covers the predicted hit tick ± `parry_jitter` (lerped `parry_jitter_max_s` → `_min_s`); dodge toward the
  side away from the swing.
- **Counter**: after a guarded, parried or dodged swing, `counter` odds request a token with `w_counter`; a parried or
  poise-broken target makes the riposte move legal (combat.md `Sense.critical`).
- **Aim error** per swing (sampled at its press): `yaw_err = min(aim.base_deg × (1 - skill) + aim.per_mps_deg × speed_mps,
  aim.max_deg)`, signed by roll; applied to the command's `view` (Doom: accuracy falls as player speed rises [V]). C8e reuses it.
- `skill` (0..1) is the content's (the stat mapping is combat-adapter.md, C0d) × `DifficultyRow.skill_scale`, clamped.

## Telegraph lint (C6d)

- At load, for every move a `Tactics` row names: `startup_s` = ticks from start to the first `Hit` window (authored
  `Accept` holds count: that is how delayed swings are made, decision 10). Floor `telegraph.light_s` (Light, Offhand),
  `telegraph.heavy_s` (Heavy, `Charged` token, `Unblockable` or `Grab` rows).
- Below the floor: `SheetWarning::TelegraphBelowFloor { tactics, move, startup_s, floor_s }` in the load log; the sheet still
  loads (warns, never refuses). Player-only moves are not linted.
- Difficulty's telegraph lever: `DifficultyRow.startup_rate` (≤ 1) scales the action clock in an NPC move's startup ticks
  only, the same for every player (one clock, so windows and sweeps stay consistent).

## Threat cues (C19)

Event contract only; presentation (sound, indicator, haptics) is `sf_combat_bevy`'s and the content's.
- `ThreatCue { attacker, target: Option<CharacterId>, move_key, kind: Windup { ticks_to_hit, heavy, unblockable } | Release,
  tick }`: `Windup` at an attack move's first tick, `Release` when an authored `Accept` hold ends (a delayed swing sounds
  twice). Emitted for every fighter's attacks; presentation filters on the local player.
- `HitLanded` gains `direction: Vec3` (world unit, attacker to defender, from `Contact.direction`) for the hit-direction
  indicator. Sheet moves gain `cue: Option<TagId>` (default per class, content data).
- Co-op: emitted on the host from host state, replicated as events; the telegraph floor absorbs the client's latency.

## Duelist (C6e)

Verifies C6 before AI15 (decision 18). `Duelist { target, tactics }` replaces `SparSchedule` on the sparring kind in
`level-sparring`: a system in `ActionSystems::Command` builds `PickInput`, requests tokens on the player's pool, calls
`decide` and writes the `Command`. Steering is a named scaffold, `DuelistSteer` (stick toward `Close.to_m`, sidestep at
`Wait.ring_m`), removed when AI15/AI36 drive fighters. It uses combat.md as written: it guards and parries by skill (C2d),
breaks the player's poise (C3a) and ripostes a poise-broken or parried player (C2e); the non-lethal refill stays. The beat
(`SparSchedule`, `sparring.ron`) is deleted when the duelist lands.

## Creatures (C6f)

- Anatomy moves are `Tactics` rows on per-rig sheets built from the content's attack groups (Bevyrrowind: `Attack1-3`,
  combat-adapter.md C0d; tables in `mw_combat` data): bite/claw short and fast, lunge/charge `Charged` and committed, tail
  sweep wide arc, spit/breath via C8.
- **Lunge whiff → wall stagger**: a row with `whiff: WallStagger`: if no contact landed in its hit windows and the mover
  reported over `lunge.wall_block_frac` of the planned root delta blocked during its `Lunge` window, the fighter gets a forced
  `Large` front reaction (self-stagger, a punish window); a clean whiff plays the authored recovery.
- **Boss phases**: `phases` swap the `Tactics` (and optionally the sheet) when `HealthBelow(f)` or `PoiseBreaks(n)` first
  holds; `NpcState.phase` only rises; `enter` plays a transition move forced at the swap tick.
- **Stance** is combat.md "Poise" (large pool, `break_tier: Knockdown`): no new code.

## Data model and interfaces

- `Tactics { sheet, skill, react: ReactOdds, rows: Vec<TacticRow>, phases: Vec<Phase> }`; `TacticRow { move, token:
  Option<TokenKind>, weight, cooldown_s, band: Option<(f32, f32)>, gates, bias: Vec<(Gate, f32)>, chain: Vec<(MoveKey, f32,
  on_hit)>, chain_only, whiff: Option<Whiff> }`; `Gate { TargetPhase(_), TargetGuarding, TargetBehind, TargetPoiseBroken,
  TargetHabit(_), SelfHealthBelow(f32), Not(_) }`; `Phase { when: HealthBelow(f32) | PoiseBreaks(u8), tactics, sheet, enter }`.
- `MoveReach` per move in `CompiledSheet` (C6b). State (serde, host, beside `FightState`): `NpcState { memory: PickMemory {
  last: [MoveKey; 4], len, habit }, cooldowns, phase, reacted_to, aim_err_deg }`; `TargetTrace` (32 ticks, a capacity);
  `TokenBook { pools: BTreeMap<(CharacterId, TokenKind), TokenPool { cap, holders: SmallVec<[Holder { who, since,
  committed, moves }; 4]> }> }`.
- Content: `DifficultyRow { tokens { melee, ranged, charged }, skill_scale, startup_rate, damage_scale }` (damage applied
  by the rules); `skill` per fighter.
- Functions: `npc::decide(&PickInput, &Tactics, &mut NpcState) -> (NpcIntent, Option<TokenRequest>)`,
  `tokens::step(&mut TokenBook, &[TokenRequest], &[TokenRelease], Tick) -> SmallVec<[TokenEvent; 8]>`,
  `npc::command(&NpcIntent, …) -> Command`, `lint::telegraph(&Tactics, &CompiledSheet, &floors) -> Vec<SheetWarning>`.

## Tunables (`combat.npc.*`, net `Gameplay`)

All Guess unless noted; difficulty rows scale the Normal values.
- Tokens: `.tokens.melee` (count, 1-6, 2; Reference Doom shape), `.tokens.ranged` (count, 0-6, 2), `.tokens.charged`
  (count, 0-3, 1), `.tokens.grant_timeout_s` (s, 0.5-5, 2), `.tokens.hold_max_s` (s, 2-15, 6), `.tokens.cooldown_s` (s,
  0-3, 0.8), `.tokens.steal_margin` (score, 0-1, 0.2), `.tokens.max_chain` (moves, 1-6, 3), `.tokens.w_dist` (0-2, 1),
  `.tokens.w_wait` (0-2, 0.5), `.tokens.w_counter` (0-2, 0.5), `.tokens.wait_cap_s` (s, 1-10, 4), `.ring_margin_m` (m,
  0.5-4, 1.5).
- Picker: `.pick.memory_len` (moves, 0-4, 3; capacity 4), `.pick.repeat_mult` (0-1, 0.5), `.pick.punish_mult` (1-5, 3),
  `.pick.punish_slow_mult` (0-1, 0.25), `.pick.habit_decay_s` (s, 2-30, 10).
- Reactions: `.react.delay_floor_s` (s, 0.2-0.5, 0.2; Reference research §2 human reaction), `.react.delay_max_s` (s,
  0.2-0.5, 0.45; ≤ trace capacity), `.react.parry_jitter_max_s` / `_min_s` (s, 0-0.2, 0.08 / 0.02), `.skill.default` (0-1, 0.5).
- Aim: `.aim.base_deg` (deg, 0-15, 6), `.aim.per_mps_deg` (deg per m/s, 0-5, 1.5; Reference Doom shape), `.aim.max_deg`
  (deg, 0-30, 15).
- Telegraph: `.telegraph.light_s` (s, 0.15-0.6, Reference 0.25), `.telegraph.heavy_s` (s, 0.3-1, Reference 0.5).
- Creatures: `.lunge.wall_block_frac` (0-1, 0.5).
- Poise numbers are combat.md "Poise"'s (C0d fills them); C-R1's ladder (stuns 0.4/0.7/1.5/3.0 s, 1 s re-stun buffer,
  ramping regen) is a candidate for those, not redefined here.

## Tests

Exact (Bevy-free, `sf_combat`), fixtures built in code:
- **C6a**: cap 2: three requests → two grants, one `Denied`; steal: better placement + margin wins, committed holder kept;
  release at recovery start, kept through a committed chain, forced at `max_chain`; timeouts; cooldown blocks re-request;
  stagger and death release; same inputs in shuffled order → identical book (sorted processing); serde round trip.
- **C6b**: band and arc from a synthetic `MoveReach`; cooldown and gate filters; fixed seed → exact pick sequence over 100
  ticks; repeat penalty halves weight; punish rule picks the fast row against `Recovery { 20 }`; `on_hit` chain only after
  a landed hit; no legal row → `Close` with the right band.
- **C6c**: delay = floor at skill 1, never below 200 ms; one roll per swing; by-seed outcome counts over 10 000 seeded swings
  match the lerped odds within the binomial bound; `Unblockable` moves the guard share to dodge; aim error formula exact.
- **C6d**: a move under the floor warns with the exact fields and still loads; an `Accept` hold lifts it over the floor.
- **C19**: `Windup` on the move's first tick, `Release` on the hold's end, `HitLanded.direction` equals the contact's.
- **C6e/C6f** (headless, sanity): 60 s seeded duel, no NaN, tokens never above cap; phase swap on the exact threshold
  tick; wall-blocked lunge whiff forces `Large`, clean whiff doesn't. Owner checks CB-7, CB-7b, CB-11.

## Reuse check

Bevy 0.20: no combat or combat-AI layer. Crates (crates.io 2026-11-26): `big-brain` 0.22.0 (stale, 2024-11-30),
`bevy_behave` 0.5.0, `seldom_state` 0.16.0, `moonshine-behavior` 0.6.0: decision frameworks only (AI15 has its own planner),
none does tokens, pickers, reaction delay or poise. Veloren (GPL-3.0-or-later, C-R1): per-species Rust handlers, uniform
pick, none of those either: gate fields taken as ideas, no code. So: our own pure module in `sf_combat`.

## Industry comparison

| Technique | Industry | Ours |
|---|---|---|
| Attack tokens | Doom 2016: pools per attack type, steal by better placement, counts per difficulty [V GDC] | match (view-independent score: no facing term) |
| Attacker queue | Arkham, Assassin's Creed: rotating attackers, others circle; Arkham attacker icon [V Sulman] | match (wait term, cooldown; ring via AI36; cue C19) |
| Aggression tokens | God of War 2018: static per difficulty and enemy type [secondhand] | match (`DifficultyRow`) |
| Move selection | Elden Ring: distance band × angle × cooldown × weighted random, anti-spam, delayed swings, on-hit combos [M] | match, as data |
| Reach gates | Veloren `could_use`: range + travel + body radius, angle, target state | match, baked from windows |
| Reaction by skill | Doom accuracy vs player speed [V]; Veloren none | ahead of Veloren (delay floor, seeded odds) |
| Telegraph floor | ER/Souls readable windups, authored by hand; no shipped lint known | ahead (load lint) |
| Unseen attackers | Doom/Arkham token caps; Dead Island 2 cuts unseen damage | tokens + cues; no view-aware rule (owner Q9) |
| Boss phases | ER, God of War: health-threshold phase swaps | match |

## Build plan

Rows and verification: combat roadmap Stage 4 (and C8e in Stage 5).
- **C6a** tokens; **C6b** picker + baked `MoveReach`; **C6c** delayed view, reactions, aim error; **C6d** telegraph lint;
  **C19** threat cues + `HitLanded.direction`; **C6e** duelist in `level-sparring` (`UnawareOf` set and `CombatSystems::Sense`
  land here); **C6f** creature tactics, wall stagger, boss phases. Then AI15/AI36/AI37/MA10 wrap C6 (AI lane).

## Open questions

1. NPC-on-NPC fights: same token caps as for a player target, or uncapped? Default: same pools, decide at AI15.
2. Where AI15's engage executor calls C6: `sf_ai_bevy → sf_combat` (Bevy-free leaf, cheap) is the expected edge; AI15 decides.
3. The `ranged` pool's sharing between bows and spells, and its band and LOS rules: C0c.
4. Morrowind skill → `skill` mapping and creature sheets from `Attack1-3`: C0d (combat-adapter.md).
