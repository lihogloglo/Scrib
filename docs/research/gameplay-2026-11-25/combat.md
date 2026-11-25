# Combat research: complete modern ARPG combat (2026-11-25)

Player and enemies, melee + ranged + magic, net-ready. Read: ledger `combat-feel`, `superfargoth/docs/specs/combat.md`, `bevyrrowind/docs/specs/combat-adapter.md`, `ai.md` Combat AI + AI15, `net.md` "Hit authority", `interaction.md`. Tags: **[V]** verified this session (URL), **[M]** from memory of shipped games (not re-fetched), **[S]** our spec/code.

## 0. Summary

Our core (fixed 60 Hz, sheet windows, swept weapon segment vs hurt capsules, contact order i-frames > guard > rules, tiers x sides, hit-stop, host-authoritative net sketch) matches what Souls-likes do internally. The gaps are what sits on top: poise for the player, parry/riposte/critical, typed damage and status buildup, an enemy decision layer (tokens, spacing, skill reactions), ranged/magic delivery, environment kills. All fit existing seams (`Contact`, `Outcome`, `Command`, `MoveRule`); no rewrite.

## 1. Player combat vs ours

| Topic | Standard | Ours [S] | Verdict |
|---|---|---|---|
| Poise / stagger | ER hidden poise per actor, attacks carry poise damage, broken = stagger tier; **hyper armor** windows inside attacks; heavies/parries break stance [V: eldenring.wiki.gg/wiki/Stance, /Hyperarmor] | `Vitals.poise` option; player `min_tier` = every unguarded hit staggers; poise numbers open (Q3) | **Change**: one symmetric poise system now (1.1) |
| Posture / deflect (Sekiro) | Posture bar from hits and guards; deflect turns a hit into posture damage on the attacker; full = deathblow [M] | guard only | Take the idea as parry (1.2), no second HUD bar |
| Guard / parry | ER guard drains stamina, break at 0; Lies of P perfect guard in first frames of raise [M]; Mordhau parry + queued riposte [V: pcgamer.com/mordhau-tips] | guard built (C2b); **parry is a non-goal** | **Change**: parry window on guard raise |
| Stamina | spend on attack/roll, regen blocked in recovery, one overdraw | same | Match. Roll (owner: fast, spammable): cost ~8% of pool, attack-cancel from ~0.5 of the clip, i-frames ~0.35-0.4 s (ours (0.02, 0.40), cost 12). Tunables |
| Buffer / cancels | Sekiro two-stage buffer, cancel windows | built | Match |
| Chains / heavy / skills | light chains, charged heavy, run/roll/jump/backstep openers, guard counter, one weapon skill per weapon [M] | C3 plans heavy, openers | Add guard counter and weapon skill (data) to C3 |
| Movesets | ER ~20 classes; KCD2 five-direction + masterstrike [M] | one sheet, second class is data | Match. Morrowind chop/slash/thrust chosen by input direction (owner 2026-11-12), not RNG |
| First-person directional | Mordhau/Chivalry: look picks swing, feint (cancel in last windup ~50 ms), morph, drag/accel, chamber [V: mordhau.fandom.com/wiki/Combat] | sheets only | Add feint = `Cancel` in windup and morph = chain `next` by look delta (data). Skip drag/accel |
| Kick / physics (Dark Messiah) | kick = push contact, knockback, hazards kill [M] | owner wants dedicated kick; not built | `Kick` class + push contact (section 4) |
| Lock-on / camera | sticky lock, flick switch | built; flick + shake left (C4) | Finish C4 |
| Hit-stop / feedback | 2-6 frames both sides by weight, shake, sparks, haptics, sound layers [M] | hit-stop built | Add haptics/trail tags to `HitLanded` |
| Hit detection | swept shape per tick, once per target per swing (UE5 notify-state traces, FromSoft active hitboxes) [M] | 5-substep segment sweep between baked socket samples, `(hit, defender)` pairs | Match, better than a per-frame trace (no tunnelling). Open: per-bone shapes, weak points |
| Damage layers | ER: base + stat scaling per type (physical split, magic, fire, lightning, holy), flat + % defense per type, status buildup (bleed, poison, frost) [M] | `attack x mv x strength x (1 - absorb)` | **Change**: typed packet + buildup (1.3) |
| Backstab / riposte | behind cone or poise-broken/unaware = critical x2-3.5, paired anim [M] | `defender_state.unaware/down` exist | Adopt (1.4) |

**1.1 Poise.** One pool per `Fighter`; `poise_damage` per hit row; `armor` windows = "tier clamped to Additive while open"; regen after a tunable delay. Break escalates the tier (Large/Knockdown by overflow) and sets `defender_state.poise_broken` briefly (critical-eligible). Replaces "every hit staggers"; gives bosses ER-style stance with the same code. Cast interruption = poise damage vs cast armor (section 3).

**1.2 Parry as data.** `guard: (raise, arc, parry_ticks, stability)`. A hit inside `parry_ticks`: `Outcome.parried`, attacker takes `parry_poise`, defender opens `Accept([Riposte])`. Stamina damage and guard break stay as built. The Sekiro payoff is the attacker's poise break, so no new bar.

**1.3 Damage packet.** `Outcome` today = pool deltas + tags. Add `damage: SmallVec<[(DamageType, f32); 3]>` and `buildup: SmallVec<[(StatusId, f32); 2]>`; the framework knows ids only, content rules own resistances; `StandardRules` keeps the flat default so classic stays behind the seam. Status ticks themselves belong to effects.md.

**1.4 Paired actions without paired clips.** Play the attacker's own thrust and the victim's prepared react/death clip, aligned by the `Warp` owner (snap over ~0.2 s, then locked), then `PhysicsTakeover`. No new clips; the look goes to the owner as a visual check.

## 2. Enemy combat AI

**Verified** (Doom 2016 [V: gamedeveloper.com/design/cyber-demons-the-ai-of-doom-2016-]): attack tokens per attack type (melee, ranged, charged), limited pool, request/release, **token stealing** by a better-placed demon, token counts per difficulty, no new enemy attack while the player is mid-glory-kill, accuracy falls as player speed rises, pain/twitch/stagger ladder, ranged demons keep distance, melee demons stage to rush, hierarchical FSM. God of War 2018: aggression tokens static per difficulty and type (secondhand notes only; primary talks gdcvault.com/play/1026423, /1025768, /1029286 not viewed). ER [M]: move picked by distance band x angle x cooldown x weighted random, anti-spam adaptation (repeated rolls bias to grabs), delayed swings, combos conditional on the first hit landing.

Mapped to our model (AI writes the same `Command`s as the player):
1. **Tokens**: Doom's shape: pools per `(target, attack kind)`, request, hold, release at recovery start, steal if closer, counts per difficulty. Non-holders **circle at a ring band**. Host state, serde, keyed by `ActorKey`.
2. **Spacing bands** from baked reach/startup/recovery (derived from windows). Pick the move by `(distance, angle, target state, cooldown, last move)` from a per-species weight table (RON). Keep the anti-spam memory.
3. **Telegraphs**: startup before the first hit window above a human-reaction floor (~250 ms light, ~500 ms heavy, [M], tunable); a load-time sheet lint warns below it. Delayed swings are authored `Accept` holds.
4. **Reaction by skill**: AI reads the player's `FightState` (move, clock), which is fair because the same data is the telegraph. A `skill` scalar per actor/difficulty sets reaction delay (>= 200 ms), P(guard/sidestep/backstep/counter), ranged aim error. Doom's accuracy-vs-speed is the cheapest fairness lever [V].
5. **Group tactics**: ring slots by angle, rotation of non-holders, nav query for reachable slots, alert sharing (ai.md perception). Roles: shield, archer, bruiser. No GOAP/HTN first.
6. **Morale/flee**: health fraction, allies dead nearby, repeated poise breaks, via the *Flee* drive. Grudge/Nemesis = later hooks.
7. **Creatures**: per-rig sheets with anatomy moves (bite/claw short, lunge/charge committed with wall-stuck stagger on whiff, tail sweep all-round, spit/breath ranged); boss phases = health-threshold swaps of the species table; per-part hurt volumes (character-shapes S1+).
8. **Archers/casters**: hold a band (8-20 m, tunable) with LOS, strafe, retreat when closed below a floor; token holders of the `ranged` pool so volleys don't stack [V Doom].
9. **Difficulty**: token counts, skill, telegraph time, damage; no rubber-banding; HP last.

Decision layer: pure `decide` as ai.md; weighted table inside it, hierarchical FSM as Doom [V]. Crates (crates.io API 2026-11-25): `big-brain` 0.22.0 last release 2024-11-30 (stale), `bevy_behave` 0.5.0 (2026-02-06), `beet` 0.0.9-rc.1. None needed: a data table plus a pure function.

## 3. Ranged and magic

Standard [M] (Skyrim, ER, DD2, Avowed): **delivery x effect**. Delivery: projectile (ballistic/homing), beam (channelled), cone, ground-targeted area, self/aura, touch, summon. Casting = upper-body clip over locomotion at reduced speed (ER, Skyrim); chants/charge are the cost, with vulnerability (DD2: interrupt by stagger).
- **Framework owns delivery, adapter owns effects** (Morrowind magic effects ported into `mw_*`). `sf_combat` gains `source: Projectile | Beam | Area` beside `Sweep | Cone`, all through the same contact order and `Outcome`.
- **Projectiles**: host entity on the fixed tick, swept segment per tick (same primitive as weapons) vs hurt shapes and the static query world (`rapier3d` 0.36.1 direct, no Avian). Arrows: gravity + drag, stick into world/actor, persist as props. Client sees a cosmetic copy from a spawn event (tick, origin, velocity).
- **Aiming**: third person = lock-on or over-shoulder mode, reticle, mild assist (soft-aim exists), lead solver for AI; first person = crosshair. Draw/charge = a `Heavy`-like charge window on a ranged sheet. Cast while moving = slowed (tunable); big spells root (sheet field).
- **Area**: ground reticle (range-clamped ray), overlap query with falloff and LOS; reuse effects.md area queries; explosions hit props (interaction.md:218).
- **Resources**: mana is a content-side pool (`Vitals` allows it). AI casters use the same `Command` stream with aim error by `skill`.
- **Crates**: nothing gameplay-side. `bevy_hanabi` 0.20.0 (Bevy ^0.20, MIT/Apache, 2026-11-01) for spell VFX (effects lane). `avian3d` 0.8.0 (Bevy ^0.20, MIT/Apache, 2026-10-20) has shape-casts but we own a rapier query world; no second physics.

## 4. Physical reactions and environment

Existing [S]: tiers x sides, Knockdown/Death via `PhysicsTakeover` (Delirium/powered ragdoll), thrown bodies hurt via `PhysicsEvent::Hit`.
- **Partial-ragdoll hit reactions** for Medium+ [M] (UE5 Physical Animation, RDR2, Cyberpunk): drive the hit chain toward the animation with a decaying weight instead of only clips. Variety without authored clips (the weak point). Delirium has the drive; this is routing + a weight curve.
- **Pushes and hazards**: a `push` contact (kick, shield bash, shockwave) carries impulse and credit; a ragdolled body touching a hazard volume (spikes, lava, fire, deep water, ledge) emits `Contact { source: Environment }` crediting the pusher. Hazards are `sf_interaction` data; combat only sees the contact.
- **Bodies as weapons** go through the same `resolve_contact` so poise/armor apply.
- **Props**: swings and projectiles get prop targets (interaction.md:218: stages; material x damage-type table in the adapter); debris local, destruction event replicated.
- **Friendly fire**: melee skips same-faction; AoE, explosions, projectiles, environment hurt anyone; NPC infighting by faction damage as Doom [V]; co-op players: world setting, off by default [S net.md].

## 5. Net-ready combat

net.md [S]: host decides every hit; ~250 ms NPC rewind for player hits; defender-favoured up to ~150 ms for NPC attacks; own-start prediction; hit-stop is presentation. This is the Valve/Overwatch lag-compensation model [M].
- **Co-op PvE needs less.** FromSoft-style co-op lets each player's game resolve their attacks (widely described, unverified [M]). Recommend **attacker-claimed hits, host plausibility-checked** (tick, range, active move id, once per swing instance) as the N default; keep the rewind buffer only if the check proves too loose. It removes most of the machinery.
- Already net-friendly: integer clocks, serde `FightState`, hashed RNG, host-only AI. Add: projectiles/areas are host entities spawned by tick; poise, buildup, tokens are host state.
- Crates (crates.io API 2026-11-25): `bevy_replicon` 0.45.8 (2026-11-24, Bevy ^0.20), `lightyear` 0.31.0 (2026-10-21, bevy_ecs ^0.20), both MIT/Apache. Choice stays in net.md N1.

## 6. Prior art

- **Funny-Bones/ELDEN-RING-Combat-Rewrite** [V: README]: Bevy 0.19, MIT, pure 60 Hz `src/sim/`, lock-on, four hit reactions, **no parry, no enemy AI** (one dummy, four fixed attacks). A timing reference only; we are beyond it.
- **Veloren** (Rust, GitHub SPDX GPL-3.0, active 2026-11-24 [V: API]): poise/stagger, energy, projectiles, abilities, NPC tactics [M]. Own ECS: ideas and data shapes. Licence may be -only: check headers before porting. Candidate slice: read `common/src/combat.rs` and `server/agent`.
- **Brutal Morrowind** (GPL-3.0, OpenMW 0.51.0 fork): kicks, parries, ragdolls, dismemberment; Morrowind side, parked. **OpenMW** `combat.cpp`, `aicombataction.cpp`: classic rules, last.
- **Bevy ecosystem**: no maintained ARPG combat crate found (search 2026-11-25). `bevy_tnua` 0.33.2 irrelevant (own mover). Build, per CONTRIBUTING rule 2.

## 7. Gaps ranked by player impact

| # | Gap | Recommendation | Where |
|---|---|---|---|
| 1 | No enemy decision layer (AI15 = "tokens respected") | Write C6 now: tokens (melee/ranged/charged), ring slots, band picker from sheet reach, skill reactions, anti-spam, morale. Before ranged | combat.md C6, ai.md AI15 |
| 2 | Player poise deferred, every hit staggers | Symmetric poise + armor windows in C3 (1.1) | C3 |
| 3 | No parry/riposte/critical | Parry window, riposte, critical state, `Warp` finishers (1.2, 1.4) | new C2d |
| 4 | No ranged/magic | Delivery layer, cast-while-moving slot, charge window; adapter ports effects | new C8 |
| 5 | Single-number damage | Typed packet + buildup (1.3) | rules seam |
| 6 | Kick, hazards, props | push contact, hazard volumes, prop targets | C2c/C3 + interaction (b) |
| 7 | No telegraph lint | load-time sheet lint for NPC moves | sheet loader |
| 8 | Net plan heavier than co-op needs | attacker-claimed + plausibility default | net.md |
| 9 | Clip-only hit reactions | partial-ragdoll blend for Medium+ | adapter routing |

Change: "every unguarded hit staggers" and "parry non-goal". Drop: nothing; Brutal Morrowind port stays parked (sheets already cover directional/charged attacks as data). Classic rules stay last.

## 8. Candidate owner questions (each with recommendation)

1. Should a well-timed guard be a Sekiro-style parry with riposte, or a plain block? Recommend parry: tight window, same button, riposte on staggered or parried targets.
2. Should heavy weapons and armor let the player shrug off light hits mid-swing (Elden Ring poise), or does every hit interrupt? Recommend Elden Ring poise.
3. May the player walk while drawing a bow or casting? Recommend slowed walk for bows and small spells, root for big spells.
4. Are kicks into hazards (spikes, fire, ledges) and breakable props a core option in both views? Recommend yes, hazards placed by level/pack data.
5. Do explosions, fire and arrows hurt allies and companions (melee not)? Recommend AoE and projectiles hurt anyone; co-op friendly fire off by default.
6. Do backstabs and ripostes deal big damage with a short finisher animation? Recommend yes, short warped finisher, no cutscene.


## 9. Missing

Memory only, not fetched: Sekiro/Lies of P/KCD2/DD2/Avowed specifics, lag-compensation talks, FromSoft co-op authority, ER AI behaviour. Not done: Veloren source read and licence headers, the God of War AI talks, Bevy 0.20 source check (no engine combat feature is known to exist).
