# Locks and traps (`sf_crime`, `sf_crime_bevy`, `sf_rpg_ui`)

Status: spec written 2026-11-26 (LW0c: moved out of [crime.md](crime.md), rewritten for the
[crime roadmap](../../../docs/roadmaps/crime.md) decisions 8, 9 and 10); nothing built. Part of the crime system: shares
its crates, `ObjectLaw`, `OffenceAttempt`, co-op rules and the `sf/crime` save section. Meets the activation pipeline
of [interaction.md](interaction.md) "Activation contract" (as built: `sf_interaction_bevy/src/activate.rs`). Morrowind
numbers: [crime-adapter.md](../../../bevyrrowind/docs/specs/crime-adapter.md) "Locks and traps (A5)".

<!-- agent-summary -->
> **Agent summary.** Crime owns the lock (`LockState { level, locked, key, hp, broken, impossible }`), the trap
> (`TrapState`), keys, `LockRules` (pick, probe, force), `LockRequest` / `LockOutcome`, the trespass hook and the
> lock and trap overlay in `sf/crime`. Interaction owns the door. A gate system in `ActivationSystems::Gates` refuses
> an open with `Locked` or `TrapSprung`; `pick` and `probe` verbs become `LockRequest`s. Picking goes through one seam,
> `LockInteraction { Roll | Minigame }`, with one `LockOutcome`. Outcomes make noise through `NoiseEvent`.
<!-- /agent-summary -->

## Goal

A player can open a lock with its key, pick it, force it, or open it with a spell, and can disarm or spring a trap.
Every way in has a cost: tool wear, noise, trespass or vandalism. Rules are pure and seeded; the game supplies levels,
keys, trap spells and the pick formula. Non-goals: the door's swing and queue (IN33), the blow's damage (IN37),
physical noise (IN18), the trap's effect (effects.md `Inflict`), perception (ai.md).

## Reuse check

- **Bevy 0.20:** no lock, key or trap system. We use messages, components, `FixedUpdate` and system sets.
- **Crates (crates.io searched 2026-11-26 for "lockpick", "lockpicking", "lock minigame", "bevy rpg"):** no lock or
  trap crate; the Bevy RPG crates found (`bevy-magic`, `bevy_skills`, `bevy_stat_query`, `mmorpg`) cover magic, skills,
  stats and MMO plumbing, not locks. A lock is a game mechanic of a few hundred lines: we write it in `sf_crime`.
- **Ours:** `sf_interaction_bevy` gates, `sf_ai_bevy` `NoiseEvent`, `sf_effects` `Inflict`, `sf_stats` `UseSkill`,
  `sf_ids` seeded rolls, `sf_save_bevy` sections.
- **Industry** ("Industry comparison"): a sweet-spot minigame scaled by skill (Skyrim, KCD2), an auto option
  (Oblivion), loud failures (KCD2), forced entry as a crime.

## Ownership split (decision 8)

| Who | Owns |
|---|---|
| Crime (`sf_crime`, `sf_crime_bevy`) | `LockState` (with `hp`), `TrapState`, keys, the gate, `LockRules` pick/probe/force, `LockRequest` / `LockOutcome`, trespass and vandalism offences, outcome noise, the lock and trap overlay in `sf/crime` |
| IN33 swing doors | the swing slot (Open/Closed, hinge, doorway queue); reads "Locked" from the gate's refusal |
| IN37 forced entry | the blow: strength, weapon, contents risk, impact noise; writes `Bashed` (crime turns it into `LockRequest::Force`) |
| IN18 noise | physical noise: impacts, doors, breaks |
| DQ-7 world functions | `Lock`, `Unlock`, `GetLocked` opcodes, acting on `LockState` through `CrimeCommand` |

Cores never name each other: `sf_crime` knows no verbs or doors, `sf_interaction` knows no locks. Only the host crate
`sf_crime_bevy` depends on `sf_interaction_bevy` (for `ActivationSystems`, `PendingUses`, `UseApproved`) and
`sf_ai_bevy` (for `NoiseEvent`).

## State

```rust
pub struct LockState { pub level: i32 /* 0 = never locked */, pub locked: bool, pub key: Option<RecordId>,
                       pub hp: u32 /* force damage left */, pub broken: bool, pub impossible: bool /* data flag */ }
pub struct TrapState { pub source: Option<RecordId> }     // an effect SourceDef (effects.md); None = disarmed
pub enum LockTier { Novice, Apprentice, Adept, Expert, Master }
```

Both live on `ObjectLaw` with content defaults and a sparse overlay. Unlocking keeps `level`, so a once-locked object
stays recognisable for trespass. `hp` starts at `LockRules::max_hp(level)` unless content sets it. A broken lock is
unlocked for good; only a script `Lock` repairs it (`broken = false`, `hp` refilled).

## The gate (decision 8, IN31 contract)

`sf_crime_bevy::lock_gate` runs `.in_set(ActivationSystems::Gates)` over `PendingUses`. For each use whose target
has an `ObjectLaw`, it calls the pure, ordered rule:

```rust
pub fn gate_activation(law: &ObjectLaw, actor: &ActorView, verb: VerbClass) -> Gate;
pub enum VerbClass { Open, Pick, Probe, Other }   // the adapter maps verb tags to classes
pub enum Gate { Pass, KeyUsed { key: RecordId, trap_disarmed: bool }, Locked { tier: LockTier, impossible: bool },
                TrapSprung { source: RecordId } }
```

- `Open` class: a held key unlocks and disarms (`KeyUsed`, the use passes, the default opens; no trespass, no noise).
  Then a trap fires before anything opens: `TrapSprung`, the host refuses the use with reason `TrapSprung`, casts the
  trap and clears it. Then a locked object refuses with reason `Locked`; the prompt and IN33 read that reason.
- `Pick` and `Probe` classes pass the gates. Crime registers `pick` and `probe` as verbs (offered while a tool is
  readied; `sf_crime_bevy` mirrors `locked` and `trapped` as `InteractionStateTags` for their `requires`). Crime is
  their default handler: it reads `UseApproved` `.after(ActivationSystems::Default)` and turns the use into a
  `LockRequest`. Picking a trapped lock does not spring the trap.
- No escalation (activate.rs, owner 2026-11-26): a refused open never becomes `pick` or a bash. Forcing is the
  player's own act (an attack or IN37's verb).
- NPC uses pass the same gate; `ActorView` carries their keys.

## Requests and rules

```rust
pub enum LockRequest { Pick { object, tool }, Probe { object, tool }, Force { object, damage: u32 },
                       Open { object, magnitude: i32 } /* spell */, Lock { object, level: i32 } /* spell */ }
pub enum LockOutcome { Unlocked, Failed, Jammed, ToolBroken, Impossible, Disarmed, TrapSprung,
                       Damaged { hp_left: u32 }, Forced, Busy }
```

Requests ride the command stream; the host checks reach, tool and uses. `LockRules` is pure; every roll seeds from (player, object, attempt, tick), splitmix64 as services SV1:

- `pick_term(&PickIn) -> f32`: the adapter's ported skill term (Morrowind `pickLock`'s `x`).
- `pick_chance(&PickIn) -> Chance { Impossible | Percent(f32) }`: the Roll model.
- `window(&PickIn) -> Window { width_deg: f32, jam_s: f32 }`: sweet-spot width and jam time, from `pick_term`.
- `tier(level) -> LockTier` and `snap_chance(LockTier) -> f32`: five labels; break chance per jam.
- `probe_chance(&ProbeIn)`, `force(&LockState, damage) -> (LockState, LockOutcome)`, `open_spell(&LockState, magnitude)`,
  `tool_wear(&Tool) -> u32`, `skill_use(LockOutcome) -> Option<StatOp>`, `max_hp(level)`.

Settlement returns `LockOutcome` plus `RpgCommand`s (tool uses -1, removed at 0) and `StatOp::UseSkill` on success.
Every pick, probe, force and Open spell emits `UnlockAttempted { player, object }`; the crime rules turn it into a
trespass `OffenceAttempt` when the object is owned and `level != 0 || locked || trapped`. A `Forced` outcome on an owned
object also files a vandalism `OffenceAttempt` (offence id from data). Scripts (DQ-7) use `CrimeCommand::{ Lock { object,
level: Option<i32> }, Unlock { object } }`: no roll, no trespass, no noise; the adapter picks the default level.

### Force (IN37)

IN37 computes the blow and writes `Bashed { lock_damage, .. }` (interaction-destruction.md "Bashing (IN37)"); crime's host
turns each one on an `ObjectLaw` target into `LockRequest::Force` (zero damage included). Crime subtracts it from `hp`: above 0 is
`Damaged`, at 0 the lock is `Forced` (`broken`, unlocked). `impossible` locks take no damage. The hit's impact sound is
IN18's; the lock giving way is crime's noise.

## The pick seam (decision 9)

`LockInteraction { Roll | Minigame }`: two front ends, one `LockOutcome`. Classic presets use `Roll`; Modern uses
`Minigame` by default, with an "auto" setting that rolls instead. Probe stays a roll in every preset.

- **Roll** (Classic, auto): `pick_chance` against the seeded roll; one attempt costs `tool_wear`. Classic keeps the
  ported rule's `Impossible` (Morrowind `x <= 0`). Auto in Modern floors the chance at `lock.auto_min_chance`, so only
  the data flag refuses.
- **Minigame** (Modern default; feel owner 2026-11-26 Q1: rotate to a sweet spot, Skyrim and KCD2): the host
  opens a `PickSession { object, tool, seed, window }` and sends the seed and window to the client. The sweet spot is
  an angle from the seed. The client sends a `PickInput { tick, angle, turning }` stream; the host re-simulates it with
  `simulate(&PickSession, &[PickInput]) -> LockOutcome` and its result is final. Turning outside the window for
  `jam_s` jams: one jam costs `tool_wear`, then `snap_chance(tier)` may snap the pick (`ToolBroken`). Inside the window
  the lock turns (`Unlocked`). The ported term only widens or narrows the window and slows jams; a low skill makes a
  lock hard, never refused. Only `impossible` (keyed-only doors) refuses: the prompt says "Requires a key".
- **Prompt** (LW8): the tier label (Novice, Apprentice, Adept, Expert, Master) from `tier(level)`, never the raw level;
  hidden when `level == 0`. It is built from the same rules, so it cannot disagree with the outcome.
- **Net:** solo runs the same path locally. One session per object: a second player gets `Busy`.

Pickpocket reuses this shape (decision 9; crime.md "Stealth and detection").

## Outcome noise (decision 10)

`sf_crime_bevy` writes a `NoiseEvent` (ai.md "Noise", `sf_ai_bevy`) with a `NoiseKind` from data for: a failed or
jammed pick, a snapped pick, a sprung trap, a forced lock. A successful pick, a key use and the Open spell are quiet.
IN18 keeps every physical noise (impacts, doors, breaks). The source is the object; `reveals_source` is true.

## Traps

Stage 1: a trap fires on activation only (the gate), as an effect `Inflict` from the object at the actor. The range
rule is an adapter option (Classic: on the actor within activation reach, else it detonates at the object). Probe
disarms (`Disarmed`) or fails (`Failed`, uses -1); a key disarms. Proximity and tripwire triggers, detection and NPCs
springing traps come with LW22.

## Commands and events

Client to host: `LockRequest`, `PickInput` stream, `CrimeCommand::{ Lock, Unlock }` (scripts). Host to clients (each
with tick, player): `PickStarted { session, seed, window }`, `LockAttempted { object, outcome }`, `LockChanged { object,
lock, trap }`, `TrapSprung { object, actor, source }`, `Refused { reason }`. `GetLocked` reads `LockState.locked`.

## Co-op

Host-authoritative on the fixed tick; lock and trap state is world state, one copy on the host. Clients send requests
and input streams, never state. One `PickSession` per object; one `LockChanged` for all. Offences go to the actor
(crime.md Co-op 6).

## Saves

`sf/crime` `objects: Vec<ObjectLaw>` carries `lock: Option<LockState>` and `trap: Option<Option<RecordId>>` as a sparse
overlay on content defaults: only changed objects, reverting equals base. Open `PickSession`s are not saved (an attempt
in progress is dropped, no wear). Shape golden with crime.md "Saves".

## Tunables

Declared in `sf_crime` (tunables.md); the adapter may override them.

| Key | Unit | Range | Default | Source |
|---|---|---|---|---|
| `lock.window_min_deg` | degrees | 2-30 | 6 | Guess |
| `lock.window_max_deg` | degrees | 10-90 | 40 | Guess |
| `lock.window_term_full` | term (widest window) | 1-200 | 100 | Guess |
| `lock.jam_s` | s | 0.1-2 | 0.6 | Guess |
| `lock.snap_chance` | 0-1 per jam, per tier | 0-1 | 0.05, 0.1, 0.15, 0.25, 0.4 | Guess |
| `lock.tier_levels` | lock level, 4 bounds | 1-100 | 20, 40, 60, 80 | Guess (Skyrim's five tiers) |
| `lock.hp_per_level` | hp per level | 0-20 | 2 | Guess |
| `lock.auto_min_chance` | percent | 0-50 | 5 | Guess |

`NoiseKind` rows are records, not tunables: `lock.pick_fail` 3 m, `lock.pick_snap` 8 m, `lock.trap_sprung` 15 m,
`lock.forced` 20 m (`loudness_m`, Guess).

## Tests

Bevy-free, exact, seeded (`sf_crime`), headless for the host (`sf_crime_bevy`):

- **Gate**: key unlocks and disarms, no trespass; trap fires before open and the use is refused `TrapSprung`; locked and
  trapped without a key is `Locked` and the trap stays; `Pick` on a trapped lock does not spring it.
- **Roll**: `pick_chance` Impossible and Percent; Classic `Impossible` refuses; auto in Modern floors at the minimum.
- **Minigame**: same seed and inputs give the same outcome; the window widens with the term and is never zero; `impossible`
  refuses; a jam costs one use; snap chance per tier; a tampered client result loses to the host's re-simulation.
- **Force**: `Damaged` until 0 hp, then `Forced`, `broken`, unlocked; owned gives vandalism and trespass, unowned
  nothing; `impossible` takes no damage.
- **Trespass**: `UnlockAttempted` yields an offence only when owned and (level or locked or trapped); an
  unlocked-but-once-locked owned object is still trespass; the Open spell emits it.
- **Noise**: one `NoiseEvent` of the right kind per failed, snapped, sprung and forced outcome; none for success, key,
  Open spell.
- **Tool wear**: uses -1 per attempt or jam, removed at 0.
- **Scripts**: `Unlock` keeps the level; `Lock` repairs a broken lock.
- **Co-op** (two `PlayerId`s): one session per object, the second gets `Busy`; one `LockChanged`.
- **Saves**: overlay round trip; revert equals base.

## Industry comparison

| Game | Technique | Us |
|---|---|---|
| Morrowind / OpenMW | one roll per attempt (`security.cpp` `pickLock`), pick uses, refused below a skill term, key disarms, trap on activation, Open spell, trespass on each attempt | **match** in Classic (`Roll`, adapter A5); Modern drops the skill refusal |
| Oblivion | tumbler minigame plus an "Auto Attempt" button; picks break on a fail; Open spell | **match**: auto setting over the same seam |
| Skyrim | rotate to a sweet spot, skill widens it, a jam breaks the pick; five tiers on the prompt; "requires key" locks; no forcing | **match**: `Minigame`, tiers, `impossible`; we add forcing |
| KCD2 | sweet spot glows when aligned, a wrong turn jams and snaps loudly, skill gives margin, opening makes noise | **match**: jam and snap, outcome noise |
| Fallout 4, Starfield | sweet-spot pick, locks above a perk level cannot be tried | **depart**: no skill gate, only the data flag |
| Thief | picking takes time in the open; the risk is being seen or heard | **match**: the minigame runs in the world, failures are loud |
| Forced entry (modern RPGs) | bash a lock, loud, a crime if owned | **match**: IN37 blow, `Force`, vandalism |

## Build plan

Rows and checks: [crime roadmap](../../../docs/roadmaps/crime.md) §4, Stage 1:

- **LW3** `sf_crime`: `LockState` (+hp), `TrapState`, gate, `LockRules` pick/probe/force, `Roll`, trespass.
- **LW6** `sf_crime_bevy`: requests and events, gate in `ActivationSystems::Gates`, `LockChanged`.
- **LW7** `sf/crime` save: lock and trap overlay.
- **LW11** minigame: seeded sweet spot, window from `PickIn`, jam and snap, input stream, re-simulation (owner Q1).
- **LW8** `InteractPrompt`: tier label, trapped.
- **LW13** outcome noises and their `NoiseKind` rows.
- Later: **LW22** proximity and tripwire traps; adapter **A5** ports `pickLock` and `probeTrap`.
