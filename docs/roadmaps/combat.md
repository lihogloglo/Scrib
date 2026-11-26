# Lane roadmap: Combat (melee, poise and parry, enemy fighters, ranged and magic delivery)

Specs: `superfargoth/docs/specs/{combat,action,character-shapes,effects,net}.md`, `bevyrrowind/docs/specs/combat-adapter.md`,
`ai.md` "Combat AI". Research: `docs/research/gameplay-2026-11-25/combat.md` (cited §n). Ledger `docs/ledger/combat-feel.md`;
ROADMAP 3.5; checks `bevyrrowind/docs/visual-checks/combat.md` (CB-). Kept IDs: combat C1-C7 (spec build plan); new slices
take the next free C number. **Anim's "C2"/"C4" (2026-11-12 plan) are other slices: written "anim C2/C4" here.**

## 1. Player-facing goal

Modern preset, lane done: third person feels like Elden Ring, first person like Dark Messiah [owner 2026-10-27, 2026-11-15].
Swings are Morrowind's own chop, slash and thrust, picked by the movement keys and retimed to commit [1]; a fast spammable
roll; heavy and charged attacks, running and rolling attacks; heavy armour shrugs off light hits (poise) [1]. A guard raised
on time parries and opens a riposte; backstabs and ripostes play a short finisher; a kick shoves enemies into spikes, fire
or off ledges [2]. Damage has types, and armour and resistances matter [3]. Enemies take turns, circle, punish your
recovery, read your attacks by skill, and creatures fight with their own anatomy [4]. Bows, throwing weapons and spells
fly, arc and explode through the same hit rules [5]. Bodies react physically to strong hits (anim lane) and break props [6].
**Classic keeps** Morrowind's rules behind the `CombatRules` seam: hit chance rolled at the swing, fatigue, no i-frames, no
lock-on or hit-stop, swings over walking legs [7].

## 2. Where we are (2026-11-26)

- **Confirmed by the owner:** CB-1 light chain, roll, stamina, hit-stop on the dummy; CB-2 lock-on (strafe, 4-way rolls,
  marker); CB-3 draw/sheathe on Morrowind equip clips; CB-3a sparring partner and hit reactions in both views; **CB-6
  spammable roll** (2026-11-25, 0.58 s cadence, press-to-roll inside moves; uncommitted with the anim batch).
- **Partial:** CB-3b guard: the held guard is a jumpy loop, not a still pose; stamina refills too fast to test the break.
- **Built, awaiting owner:** CB-5 first-person soft aim (3 m, ±30°), CB-5b first-person look limits (45°), FP-1, FP-1b,
  FP-2 (view model, off by default).
- **Built, no check yet:** character-shapes S0 (bench) and S3 (held proxies, PH-15); `sf_effects` E1-E4 and the Morrowind
  cast rules A5a (no cast input, A5b).
- **Owner verdicts against clips:** Quaternius/UAL2 sword chains "very bad", Kimodo swings unusable (MG-C0); no mocap, no
  paid packs. Morrowind attack groups are registered as `upper` clips (`WEAPON_SECTIONS`) but no sheet plays them.
- **Not started:** C2c backstep + knockdown, C3 (heavy, openers, poise), C4 rest (flick switch, shake, per-bone hurt shapes
  S1/S2/S5), C5 classic, C6 NPC combat, C7 net, ranged and magic delivery, kick, hazards, prop targets.
- **Spec non-goals reversed here:** parry, player poise, ranged and magic, paired kills.

## 3. Decisions

1. **Keep the core, build on its seams** (§0; FromSoft's fixed-tick decider + clip windows + swept sweeps + rules layer,
   UE GAS keeps attributes apart from notifies): every addition is a field on `Contact`, `Outcome`, `MoveSpec` or a new
   `source`, never a second pipeline. Reuse: Bevy 0.20 has no combat layer, no maintained crate does (2026-11-25); the
   ER Combat-Rewrite (MIT, Bevy 0.19) has no parry or AI, timing reference only.
2. **One symmetric poise system** (ER stance and hyper-armour [V], §1.1): a pool per `Fighter`, `poise_damage` per hit row,
   `armor` windows clamp the tier to Additive, regen after a delay; a break escalates the tier and sets
   `defender_state.poise_broken` (critical-eligible). Bosses get ER stance from the same code; cast interruption is poise
   damage against cast armour. Replaces the spec's "every unguarded hit staggers".
3. **Parry = a window on guard raise, no posture bar** (Lies of P perfect guard, Mordhau parry + riposte [V], Sekiro's
   payoff without its HUD, §1.2): `guard.parry_ticks`; a hit inside it sets `Outcome.parried`, deals `parry_poise` to the
   attacker and opens `Accept([Riposte])` on the defender. Departs from the spec non-goal: owner direction (Dark Messiah
   has parries, ER has shield parries).
4. **Finishers without paired clips** (§1.4; Warp-aligned like UE Motion Warping): attacker's own thrust + victim's react or
   death clip, aligned by the `Warp` owner (~0.2 s snap, then locked), then `PhysicsTakeover`. Backstab = behind cone or
   unaware; riposte = poise-broken or parried. Criticals multiply in the rules, so crime's sneak-attack multiplier and
   stealth takedowns reuse this path.
5. **Typed damage packet + buildup** (ER damage types and status buildup, §1.3): `Outcome.damage: [(DamageType, f32); ≤3]`,
   `buildup: [(StatusId, f32); ≤2]`; the framework knows ids only; the adapter owns resistances and armour; status ticks
   are `sf_effects`'. `StandardRules` keeps a flat default so a content pack may ignore types.
6. **Clips are Morrowind's own** (owner 2026-11-25; memory clip-sources): the anim lane's Stage 5 turns text keys into sheet
   windows (AN33), retimes per phase (AN34) and builds missing moves (backstep, parry, riposte, kick, 2H guard) as key-pose
   `PoseSeq` clips (AN37). No Quaternius, Kimodo or mocap combat clips; the roll stays UAL `Roll` (CB-6). MG-C1 (Kimodo
   combat pass) is dropped.
7. **Direction by movement keys** (owner 2026-11-12, KCD2 directional, not Morrowind's random pick): chop/slash/thrust from
   the stick at the press. Both views get Mordhau's **feint** (`Cancel` in the last windup ticks) and **morph** (chain
   `next` by look delta) as sheet data (decision 19); no drag/accel (§1).
8. **Enemy combat = Doom 2016 tokens + an ER-style move picker, as data** (§2 [V Doom GDC]): token pools per (target,
   melee/ranged/charged), request, hold, release at recovery start, steal if better placed, counts per difficulty;
   non-holders circle. Moves by distance band × angle × target state × cooldown × last move from a per-species RON table
   (bands from baked windows), anti-spam memory; reactions by `skill` (delay ≥ 200 ms, guard/sidestep/counter odds, aim
   error rising with target speed). A pure function, no crate (`big-brain` stale since 2024-11). Split unchanged (ai.md
   decision 12): C6 = tokens, picker, skill, telegraphs; AI15 = drive, target, engage/reposition/search, ring-slot nav.
9. **No escalation ladder** (ai.md "Drives", owner 2026-11-26): combat never decides *whether* an NPC fights. Hostility
   comes from AI15's combat drive, fed by Fight, faction, crime and being attacked (MA4, crime lane); comfort considerations
   and failed lawful options never raise it. C6 only decides *how* a fighter already engaged fights; morale and flee are
   AI37.
10. **Telegraph floor + lint** (§2.3; human reaction ~250 ms light, ~500 ms heavy): startup before an NPC move's first hit
    window above a tunable floor; the sheet loader warns, never refuses. Delayed swings are authored `Accept` holds.
11. **Framework owns delivery, the adapter owns effects** (Skyrim, ER, DD2 delivery × effect, §3): `source: Sweep | Cone |
    Projectile | Beam | Area`, one contact order. Projectiles: host entities, a swept segment per tick against hurt shapes
    and our rapier query world (no Avian: one physics), stick and persist; clients see a cosmetic copy from a spawn event.
    Spells arrive via `sf_effects`' `EffectRequest::Launch`; VFX `bevy_hanabi` 0.20 (effects lane).
12. **Push and environment contacts** (Dark Messiah kick, owner 2026-11-12 "dedicated kick with physics pushes"): `Kick` class
    and shield bash carry impulse + credit; a ragdolled body touching a hazard volume emits `Contact { source: Environment }`
    credited to the pusher. Hazard volumes are `sf_interaction` data; thrown bodies go through `resolve_contact` too.
13. **Per-bone hurt shapes before ranged** (character-shapes S5): arrows and parries need limbs, not a capsule.
14. **Physical hit reactions are the anim lane's** (anim decision 10, AN40/AN41): combat emits tier, side and impulse;
    Delirium partial drive for Medium+ under a cap, additive fallback.
15. **Co-op: attacker-claimed hits, host plausibility check** (FromSoft co-op [M], §5): host checks tick, range, active move
    id and once-per-swing; the ~250 ms rewind buffer in net.md stays only if the check proves loose. Departs from net.md
    "Hit authority" (spec slice C0e). Projectiles, poise, buildup and tokens are host state.
16. **Difficulty** (Doom): token counts, skill, telegraph time and damage; no rubber-banding; HP last. Classic keeps
    Morrowind's difficulty slider (damage scale).
17. **Tunables:** poise regen delay, parry ticks, token counts, telegraph floors, bands, aim error, projectile drag,
    hit-stop: declared with unit, range, source (`Guess` counted), never bare consts.
18. **Order departs from the spec**: enemy fighters (C6) before ranged (§7 #1); C6 is verified first against a scripted
    **duelist** (a `Command` writer using the C6 picker directly, replacing the sparring beat), so it doesn't wait for AI15.
19. **One rules core, per-view presentation as data** (owner 2026-11-26, Q7-9; research `combat-views.md`, C-R2). Every
    game that ships both views keeps one set of melee rules (Avowed: same timings, 1P master [V]; Chivalry 2, Mordhau:
    third person is a camera option). Contacts, poise, parry, damage, i-frames, windows and tokens are identical for every
    view and actor. **The dodge is one move** (one `MoveSpec`: timing, i-frames, stamina cost, rules), presented per view:
    a roll clip in third person, a side dash in first person (a camera path, no tumble). The per-view profile (input and
    presentation layer, never `sf_combat`) holds targeting (3P lock-on + flick; 1P soft aim), the dodge's presentation,
    camera and feedback weight. **Switching view mid-fight** is instant and changes nothing in the fight. **No view-aware
    AI** (owner: fragile in multiplayer): enemies behave the same whatever the view; first-person fairness comes from
    cues every player gets (audio telegraphs, a hit-direction indicator) and the view-independent token cap on
    simultaneous attackers (decision 8). Feint and morph follow the input scheme, not the view (both views, as Chivalry 2
    and Mordhau). Mixed views in co-op PvE are allowed (a "force first person" host option only if PvP ever exists).

## 4. Stages

Each stage ends in something the player can use. Owner checks run in `level-sparring` / `level-arena` unless named.

### Stage 0: specs (docs only)

| ID | Deliverable | Crate | Depends | Verify |
|---|---|---|---|---|
| C0a | combat.md: Goal/non-goals rewrite; poise, parry, criticals, damage packet, push/environment contacts; "Views" (one rules core, per-view presentation profile, one dodge move); check-ID table | docs | - | doccheck; done 2026-11-26 |
| C0b | `combat-npc.md` (new spec, split from combat.md): tokens, picker table, skill, telegraph lint, threat cues, duelist, creatures; AI15 boundary, `unaware` feed | docs | C-R1 | doccheck; done 2026-11-26 |
| C0c | `combat-delivery.md` (new spec, split from combat.md): projectile/beam/area sources, aim modes, cast slot, `EffectRequest::Launch` contract | docs | C0a | doccheck; done 2026-11-26 |
| C0d | combat-adapter.md: weapon/armour → poise, damage types, resistances; 2H/spear/fist sheets; marksman; creature rigs | docs | C0a, AN32 | doccheck; done 2026-11-26 (windows wait for AN32/AN33) |
| C0e | net.md "Hit authority": attacker-claimed + plausibility default, rewind as fallback | docs | C0a | doccheck; done 2026-11-26 |
| C-R1 | Research (Sonnet): Veloren `common/src/combat.rs` + `server/agent` (poise, tactics), licence headers; Bevy 0.20 check | research | - | `combat-veloren.md`, ledger line; done 2026-11-26 |

### Stage 1: the sword fight feels right on Morrowind clips

Ends with: a duel with the sparring partner on Morrowind swings, a still guard, backstep, knockdown and get-up, heavy
charged attacks, running and rolling attacks, heavy armour shrugging off a light hit; any NPC can be hit and killed.

| ID | Deliverable | Crate | Depends | Verify |
|---|---|---|---|---|
| (anim) | AN32-AN35, anim C4 attack clip pass (chop/slash/thrust by direction, step-in `Warp`) | anim lane | - | owner CB-1 re-check |
| C2b2 | Guard fix: still held pose from `Shield@block`, regen delay so the break can be tested | `mw_combat` data, `sf_combat` | anim C4 | exact regen; owner CB-3b |
| C2c | Backstep + knockdown through `Owner::Physics`, get-up roll-out window, invulnerable while down; same rules in both views | `sf_combat`, `mw_walkers_bevy` | AN37 backstep | exact windows; owner CB-3c |
| C18 | First-person dodge: the same dodge `MoveSpec` (CB-6 timing, i-frames, cost) presented as a side dash camera path, no tumble; view switch mid-move changes nothing | `sf_camera_bevy`, `mw_combat` data | C0a | identical windows in both views; owner CB-5e |
| C3a | Symmetric poise + armour windows, break escalation, `poise_broken` | `sf_combat` | C0a | exact tiers; owner CB-10 |
| C3b | Heavy + charge from the `min attack`..`max attack` window | `sf_combat`, `mw_combat` | AN33 | charge release swaps clip; owner CB-4 |
| C3c | Openers: running, rolling, backstep, jump attacks; typed i-frames | `mw_combat` data, `sf_action` | C3b | exact chains; owner CB-4 |
| C3d | Second classes as data: `greatsword_2h`, `spear_2h`, `fist` on `WeaponTwoHand/TwoWide/HandToHand` | `mw_combat` data | C3b, C0d | no new code; owner CB-4b |
| C17 | NPC and creature fighters without AI: `Fighter`, health from records, reactions, death via walkers ragdoll | `mw_combat`, `mw_walkers_bevy` | C2c | hit NPC reacts and dies; unblocks MA15, DQ-12, C16 |
| C4c | Third-person profile: lock-on flick switching, combat shake (1P keeps soft aim, CB-5) | `sf_combat_bevy`, `sf_camera_bevy` | - | switch picks flicked side; owner CB-5c |

### Stage 2: parry, riposte, finishers, kick and hazards

Ends with: parry the partner's swing and riposte; backstab an unaware dummy; kick it into spikes or off a ledge.

| ID | Deliverable | Crate | Depends | Verify |
|---|---|---|---|---|
| C2d | Parry window, `Outcome.parried`, attacker poise damage, `Accept([Riposte])` | `sf_combat` | C3a, AN37 parry | contact-order tests; owner CB-8 |
| C2e | Criticals: backstab cone/unaware, riposte; `Warp`-aligned finisher, multiplier in rules | `sf_combat`, `mw_combat` | C2d, C2c | exact eligibility; owner CB-8b |
| C9 | `Kick` class + push contact (impulse, credit), knockback via C2c | `sf_combat`, `mw_combat` data | C2c, AN37 kick | exact impulse; owner CB-9 |
| C10 | Hazard volumes → `Contact { source: Environment }` credited to the pusher | `sf_combat`, `sf_interaction` | C9, IN28 | credit test; owner CB-9b |
| C11 | Feint and morph as sheet data, both views (input scheme, not view) | `mw_combat` data, `sf_action` | anim C4 | exact cancel tick; owner CB-5d |

### Stage 3: damage that means something, limbs that get hit

| ID | Deliverable | Crate | Depends | Verify |
|---|---|---|---|---|
| C12 | Damage packet + buildup in `Outcome`; `StandardRules` flat default | `sf_combat` | C0a | exact packet math |
| C13 | Morrowind mapping: chop/slash/thrust ranges, armour rating, resistances, `OnStrike` enchantments | `mw_combat` | C12, C0d | exact vs records |
| S1/S2 | Shape set per rig + posing (character-shapes; physics owns S1-S4) | `sf_ragdoll`, `sf_combat` | - | spec tests |
| S5 | Two-phase sweep against per-bone hurt shapes, region in the contact | `sf_combat` | S1, S2 | blade into an arm hits, beside misses |

### Stage 4: enemies that fight well

Ends with: the duelist punishes your recovery and guards your heavies by skill; a kagouti lunges and sticks on a whiff;
then, with AI15, three guards take turns while others circle (ai.md Stage 6).

| ID | Deliverable | Crate | Depends | Verify |
|---|---|---|---|---|
| C6a | Token pools: request, hold, release, steal, counts per difficulty; view-independent (no view-aware requests); serde host state | `sf_combat` | C0b | exact pool tests |
| C6b | Move picker: bands from baked windows, weighted RON table, cooldowns, anti-spam memory | `sf_combat` | C0b | exact picks per table |
| C6c | Skill reactions from the target's `FightState`; aim error vs target speed | `sf_combat` | C6b | exact probabilities by seed |
| C6d | Telegraph lint at sheet load | `sf_combat` | C0b | warns below floor |
| C19 | Threat cues for every player and view: audio telegraph at windup start, hit-direction indicator | `sf_combat_bevy`, `mw_combat` data | C6d | exact cue tick; owner CB-7b |
| C6e | Duelist: C6 drives a humanoid fighter via `Command` in `level-sparring` | `sf_combat_bevy`, `mw_combat` | C6a-c, C3a | no NaN; owner CB-7 |
| C6f | Creature sheets from `Attack1-3`: anatomy moves, lunge wall-stagger, boss phase swaps | `mw_combat` data | C6e, S1 | exact phase swap; owner CB-11 |
| (ai) | AI15, AI36, AI37, MA10 wrap C6 in the combat drive | ai lane | C6e | owner AI-8 |

### Stage 5: bows, throwing and spells

Ends with: shoot an archer who keeps her distance; cast a fireball that arcs and bursts; dodge a mage's bolt.

| ID | Deliverable | Crate | Depends | Verify |
|---|---|---|---|---|
| C8a | `Projectile` source: host entity, swept segment per tick, gravity/drag, stick, spawn event | `sf_combat`, `sf_combat_bevy` | C0c, S5 | no tunnelling at 100 m/s; stuck arrow persists |
| C8b | Ranged sheet: draw/charge window, aim modes (lock, over-shoulder, crosshair), slowed walk | `sf_combat`, `mw_combat` | C8a | exact charge; owner CB-12 |
| C8c | `Area` and `Beam` sources: ground reticle, overlap with falloff + LOS | `sf_combat` | C8a | exact falloff |
| C8d | Cast action: upper-body Morrowind `SpellCast` clips over locomotion, release tick → `Cast` → C8a/C8c | `sf_combat_bevy`, `mw_combat` | C8c, effects A5b | headless cast lands; owner CB-13 |
| C8e | Ranged fighters: band with LOS, `ranged` token pool, lead solver | `sf_combat` | C8b, C6a | exact band keeping |

### Stage 6: the world takes the hits

| ID | Deliverable | Crate | Depends | Verify |
|---|---|---|---|---|
| C14 | Prop targets in sweeps and projectiles (material × damage type table in the adapter) | `sf_combat`, `mw_combat` | C12, C8a | exact; feeds IN1 |
| C15 | Events for other lanes: `HitLanded` credit, `Died { killer }`, `Knockout`, combat noise | `sf_combat_bevy` | C2c | exact event order |
| C16 | Assault/murder `OffenceAttempt` on hits against non-hostiles (crime adapter) | `mw_combat` | C15, LW2b | exact offences |

### Stage 7: co-op and classic (later)

| ID | Deliverable | Crate | Depends | Verify |
|---|---|---|---|---|
| C7 | Attacker-claimed hits, host plausibility, host projectiles (co-op phase F, net N4) | `sf_combat_bevy` | C0e, N1 | replay determinism |
| C5 | Classic `CombatRules` from OpenMW `combat.cpp` (cite commit), cone contacts, Classic profile | `mw_rules` | C13 | formulas vs source; owner CB-14 |

**Parked:** Brutal Morrowind port (diff vs OpenMW 0.51.0 first), dismemberment, mounted combat (travel), damage numbers,
OpenMW `aicombataction.cpp` ratings (classic AI, after C5).

## 5. Spec changes

C0a-C0e above, before their stages. Also: combat.md build plan gains C2b2, C2d, C2e, C8-C19; the spec's "CB-6 classic swing"
becomes CB-14 (CB-6 is the confirmed roll). anim's "combat-adapter.md Morrowind attack clips" slice (before AN33) stays the
anim lane's. character-shapes.md S5 points at this roadmap.

## 6. Dependencies on other lanes

| Lane | Combat needs | Combat provides |
|---|---|---|
| Anim | AN32-AN37, anim C4 (attack clip pass), anim C2 (locked strafe), AN40/AN41 physics tint | sheets, tiers, impulses, `Warp` targets |
| AI | AI15, AI36, AI37, MA10; MA4 Fight/Flee; perception for alert sharing | C17 NPC fighters (MA15 no longer waits for MA10), C6a-f, C8e, C2c knockdown (AI41 downed flag), C15 noise |
| Interaction | IN28 hazard/prop data, IN1 destruction stages, IN36 thrown props | C10 environment contact, C14 prop targets, poise (C3a) for IN36 |
| Crime | LW2b offences, recognition | C16 assault/murder, C2e sneak-attack multiplier and takedown path |
| Dialogue-quests | - | C17 + C15 hit/death events (DQ-12, `OnPCHitMe`), combat start for DQ-31 |
| Travel | - | C6/C8e for ambushes (TV9); mounted combat parked |
| Nav | ring-slot reachability, LOS (AI36) | - |
| Effects (no lane file) | A5b cast input, `EffectRequest::Launch`, status ticks | C8a/C8c delivery, C12 buildup |
| Physics | character-shapes S1-S3, Delirium takeover | S5 hurt sweep |

## 7. Owner questions (answered 2026-11-26: Q1-6 and Q8 as recommended; Q7, Q9 owner variants)

1. **Parry:** a tight window on raising the guard, riposte on parried or poise-broken foes, in both views. (C2d, C2e)
2. **Player poise:** heavy armour and weapons shrug off light hits mid-swing (Elden Ring). (C3a)
3. **Moving while aiming or casting:** slowed walk while drawing a bow or casting small spells; big spells root. (C8b, C8d)
4. **Friendly fire:** melee passes through allies and companions; arrows, explosions and fire hurt anyone; co-op friendly
   fire off by default. (C8a, C8c, C10, C7)
5. **Finishers:** backstabs and ripostes deal heavy damage with a short warped finisher, never a cutscene. (C2e)
6. **Status buildup:** in the framework (C12); Morrowind content ships none at first (its on-strike enchantments and poisons
   stay instant effects).

**Answered 2026-11-26 (C-R2, decision 19):**

7. **Dodge in first person:** one dodge move (same timing, i-frames, cost, rules) in both views, presented as a roll in
   third person and a side dash in first person (camera path, no tumble). Not the recommendation (two moves). (C18, C0a)
8. **Switching view mid-fight:** yes, instantly, nothing about the fight changes (as recommended). (C18, C0a)
9. **Attackers you can't see in first person:** no view-aware AI (fragile in multiplayer; not the recommendation).
   Enemies behave the same in every view; fairness comes from audio telegraphs, a hit-direction indicator and the
   view-independent token cap. (C6a, C19)
