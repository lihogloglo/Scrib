# Combat clips: Morrowind's attack, reaction and block clips as sheet moves

Status: spec (2026-11-26; nothing built). Split from [combat-adapter.md](combat-adapter.md) on 2026-11-26 (size); quoted
section names that are not in this file are sections of [combat-adapter.md](combat-adapter.md). Framework side:
[combat.md](../../../superfargoth/docs/specs/combat.md) (sheets, windows), [action.md](../../../superfargoth/docs/specs/action.md)
(motion owners). Rows: anim roadmap AN33, AN33b, AN34, AN35, AN37; combat roadmap C3b, C4, C6f. Inputs: AN32a (humanoids), AN32b
(creatures).

## Morrowind clips as the fallback layer

Every combat slot ends with a Morrowind clip fallback (`ClipFallback::Clip`) unless the content has none (rolls, dodges):
- Attacks: `WeaponOneHand@chop|slash|thrust`, `WeaponTwoHand@...`, `WeaponTwoWide@...`, `HandToHand@...` (graph clip names,
  `mw_graphs::build::WEAPON_SECTIONS`). Reactions: `Hit1`-`Hit5`, `Knockdown`, `Knockout@tail` (get-up), `Death1`-`Death5`.
  Guard: `Shield@block`.
- **Windows** come from the clips' text keys (`mw_combat::windows`, a new module; kinds parsed in `mw_anim/src/text_keys.rs:32-53`):
  "Morrowind attack clips" below. One hit per attack (the original honours one hit key); the sheet's row numbers apply.
- These clips animate the upper body: in the modern profile they play in the full-body slot (fallback quality, owner accepts);
  the classic profile plays them in the `upper` slot over the movement set.

## Morrowind attack clips (AN33, AN34, AN35; AN37 input)

Owner 2026-11-25 (anim decision 9): Morrowind's clips; UAL, Quaternius, Kimodo combat clips rejected. Input AN32a
(humanoids; creatures: AN32b). Clip s from the section's `Start`; `.` = `mw.combat.clips.`. Today (V): UAL first, Morrowind
as fallback (`sources.ron:335-545`); `<G>@<s>` = `Start`..last follow stop (`mw_graphs/src/source.rs:114-125`); one `rate`
per move (`sf_action/src/step.rs:590-591`); no `Charge` kind (`windows.rs:32-48`); reactions whole, rate 1, no windows
(`sf_combat/src/sheet.rs:646-690`).

**Keys to a move (AN33).** Phases: anticipation `Start`..`Min Attack` (0.07-0.33 s), charge `Min Attack`..`Max Attack`
(0.13-0.33), strike `Max Attack`..`Hit` (0.20-0.40), recovery = ONE `<Size> Follow Start`..`Stop` (0.20-0.53). The follow
windows sit one after another; OpenMW jumps from `Hit` to the chosen one (`character.cpp` 1794-1808 @ `4e718b5`), and the
root at `Hit` equals the root at every follow start (U): root-safe. A move = two slots, `<G>@<s>.strike` (`Start`..`Hit`)
then `<G>@<s>.large|.medium|.small` (new reference forms cut from the keys in `mw_graphs::source`), on the existing
attack + `_rec` chain (`mw_player_bevy/src/avatar.rs:330-372`): no splice, no runtime jump. Light → `.small`, Heavy released
early → `.medium`, Charged → `.large` (C3b picks at release; OpenMW by strength < 0.33 / < 0.66, `character.cpp` 1797).
Cooked moves play `root_scale` 0 (the clips go out and back, 0.02-0.52 m, U); the step-in is C4's `Warp`, `Min
Attack`..`Hit` (owner Q2 2026-11-25). The cook writes times only; `mv`, `tier`, `poise`... stay sheet rows.

| Window | From | To |
|---|---|---|
| `Hit(0)` | `Min Hit` | `Hit` + `.hit_tail_s`, at least `.hit_min_s` played |
| `Charge` (Heavy; C3b adds the kind) | `Min Attack` | `Max Attack`; strength = share played (`character.cpp` 1230-1243) |
| `Accept(Light, Heavy, Dodge, Guard)` | `Min Attack` | End |
| `Cancel(Light, Heavy)` / `(Dodge, Guard)` / `(Move)` | `Hit` + `.cancel.chain_s` / `.dodge_s` / `.move_s` (played, capped at the follow stop) | End |
| `Turn` (sheet default), then `NoTurn` | 0, then `Max Attack` | `Max Attack`, then the follow start |

**Per-phase rate and accents (AN34).** Rate = clip s per played s, per phase, × the sheet `rate`. A monotone
piecewise-linear `TimeWarp` (move time ↔ clip time) replaces the scalar in `Owner::Clip` (`sf_action/src/rule.rs:86-91`)
and `MoveSpec` (`sheet.rs:294`, `slot_at` 309-312, slot lengths 508); `MoveDef.phases: [(clip_s, rate)]`, absent = one
rate. One map feeds root, slot progress, sweeps and both shells; montage slots already take the playhead from the action
clock (`sf_anim_graph/src/instance.rs:756`). Windows are authored in clip time and converted at sheet load to move-time
`ClockTime` (1/60000 s, `sf_action/src/ids.rs:84-104`): the 60 Hz tick sees move time only, a window opens on the first
tick whose `(t0, t1]` reaches it; a rate change reloads the sheet (no bake). Defaults: light hit at 0.589 s (1H slash),
0.378 (1H thrust), 0.567 (2H chop), 0.333 (spear thrust); ER R1 0.37 (dagger) to 0.63 s (spear) ("ER" = `AtomechLLC/
game-animation-library` `frame-data/elden-ring.json`, patch 1.17.3, game files, 30 fps). **Accents** (presentation): the
clip's `Max Attack` pose over its `Start` pose, weight to `.accent.antic` by `Max Attack` (held while charging), 0 at `Hit`;
its `Hit` pose over `Max Attack`, peak `.accent.over` at `Hit`, out over `.accent.over_s`: two one-frame additive clips per
section (`AdditiveBase::Time`, `sf_anim/src/cook.rs:114-119`), the graph's `Additive` node with a `Float` param, upper-body
mask (`sf_anim_graph/src/asset.rs:385-392`).

**Reactions (AN35).** `Hit1-5` have no inner keys, direction or strength (U). Tier = clip + rate + free point (played s
where `Cancel(Move, Dodge, Guard, Light, Heavy)` opens; idle, it plays out); lengths from Veloren's stuns 0.4 / 0.7 / 1.5 /
3.0 s ("Poise (modern)"). Reaction entries become `(slot, rate, free_s)` (`sf_combat`; a bare slot today, `sheet.rs:225`).

| Tier | Clip | Rate | Free | Over it |
|---|---|---|---|---|
| Additive | none | - | - | `HitReact` springs (`asset.rs:1021`) |
| Small | Hit1 front, Hit2 left, Hit3 right, Hit4 back (today's data) | 1.6 | 0.4 | `HitReact` gives the side |
| Medium / Large | Hit5 | 1.2 / 0.75 | 0.7 / end (1.33) | Stagger tint ([physics-tint.md](../../../superfargoth/docs/specs/physics-tint.md) "Hits", AN41a) |
| Knockdown, Death | takeover ("Ragdoll routing"); refused or Classic: `Knockdown` 2.667 s, Death1-5 | 1 | end | get-up `Knockout@tail` |
| Downed (C22), classic knockout | `Knockout`, loop 1.000-1.733 held while down | 1 | tail end | OpenMW loops while fatigue < 0 (`character.cpp` 421) |

Swimming: `SwimKnockdown` 3.0 s, `SwimKnockout` 4.0 s (loop 0.333-3.667), no `SwimHit*`: Hit1-5 (OpenMW the same, 457-461).
A fixed clip per (tier, side): OpenMW's random pick (`chooseRandomGroup`) would need a shared seed.

**Block.** `Shield@block` (1.333 s, `Block Hit` 0.333) is the guard-hit reaction (`guard_hit`, `sheet.rs:228`), from `Block
Hit` at `.block.rate`, left-arm mask over the held guard (OpenMW: `block start`..`block stop` at left-arm priority,
`character.cpp` 443-450). The held shield guard is its `Block Hit` pose (AN37, one key); a weapon guard (1H, 2H) has no
Morrowind pose (AN37). **Ranged:** "Ranged weapons (C8b data)" is confirmed (bow 0.933 / 1.333 / release 1.467, crossbow
0.200 = 0.200 / 0.333, thrown 0.133 / 0.200 / 0.333); `shoot` has one unsized follow and plays at rate 1 until C8b.
**Sets:** one sheet per class. Female uses the male file; beast differs in HandToHand only (slash hit 0.600 vs 0.533; chop
0.067 / 0.267 / 0.533 vs 0.133 / 0.333 / 0.667); first person in WeaponTwoHand chop only (hit 0.867 vs 0.800). Each set's
clip cooks key-aligned: a piecewise retime from its own keys onto the male ones (new `ClipRules.retime`,
`sf_anim/src/cook.rs:231`; identity when they match). Classic plays each set's own keys.

**Missing moves (AN37 input).** AN37 key poses (Morrowind poses first, Kimodo too; owner Q1 2026-11-25): backstep, parry,
riposte, kick, weapon guard 1H / 2H, `parried` (attacker; `Hit5` until then), jumping attack (C3c). Roll: UAL `Roll` (CB-6)
until beaten. Not needed: side dodge (roll + backstep, ER; UAL side hops rejected live 2026-10-27), a recoil on a guarded hit
(ER swings carry through a guard), a directional hit (`HitReact` over Hit1-5), a charged clip (`.rate.charge_heavy` + Large
follow + accents), a running attack (the section from `Max Attack` + a run-in `Warp`, C3c), a get-up (`Knockout@tail`, AN-6).

**Classic seam** (C5, never early): hold plays `Start`..`Max Attack`, strength = the share of `Min Attack`..`Max Attack`
played; release plays `Max Attack`..`Hit` skipping `(1 - strength) × (Min Hit - Max Attack) / (Hit - Max Attack)`, then the
follow by strength, all at the record `speed`; hit recoil picks Hit1-5 at random (OpenMW `apps/openmw/mwmechanics/
character.cpp` 380-479, 1230-1243, 1293-1322, 1763-1808 @ `4e718b5`). **Net-ready:** windows and warp are sheet data in
`ClockTime` on the fixed tick; accents and set retimes are presentation and never feed gameplay.

| Tunable | Unit | Range | Default | Source |
|---|---|---|---|---|
| `.rate.antic` / `.charge` (light) / `.strike` / `.recover` | × | 0.5-3 | 1.2 / 1.5 / 1.5 / 1.0 | strike decision 9 (1.3-1.6); rest Guess |
| `.rate.charge_heavy` | × | 0.2-1 | 0.35 | ER straight sword charged R2 hits frame 36, R1 14: +0.7 s |
| `.hit_tail_s` / `.hit_min_s` | s | 0-0.2 | 0.033 / 0.067 | ER active 2-4 frames |
| `.cancel.chain_s` / `.dodge_s` / `.move_s` | s | 0-1 | 0.133 / 0.367 / 0.533 | ER straight sword R1: hit 14, cancels 18 / 25 / 30 |
| `.charge.hold_s` (then auto-release) | s | 0-1 | 0.15 | Guess (ER auto-releases a full charge) |
| `.accent.antic` / `.over` / `.over_s` | weight, s | 0-1 | 0.25 / 0.2 / 0.12 | Guess |
| `.react.small/medium/large.rate`; `.small/medium.free_s` | ×; s | 0.5-3; 0.1-2 | 1.6 / 1.2 / 0.75; 0.4 / 0.7 | Guess; Veloren |
| `.block.rate` | × | 0.5-3 | 1.5 | Guess |

20 values (11 Guess); rates, windows net `Gameplay`, accents `Presentation`. **Tests:** AN33 (headless, `#[ignore =
"needs Morrowind data"]`): 12 melee sections' windows equal their keys, slots span them (1H slash large 0.867-1.133), root
at the joins < 1 cm, synthetic keys for a missing `Min Hit` or size. AN34: `TimeWarp` round trip within one `ClockTime`
unit, one segment = today's path, the hit times above exact; accents no NaN, weight 0 = the clip. AN35: exact (tier, side)
rows. Sets: aligned keys on the male keys, identity except beast HandToHand and first-person 2H chop.

## Creature attack clips (AN32b; AN33 cooks, C6f uses)

Input AN32b (ledger animation; `mw_bake/tests/it/attack_clips_survey.rs`, ignored test `creature_attack_clips`): 89 creature
rigs, 234 `Attack1-3` groups. Every group has only `Start`, `Hit`, `Stop`: no sections, no min / max attack, no sized
follows. `.` = `mw.combat.clips.`; tactics and the lunge rule: combat-adapter.md "Creatures (C6f)".

- **Move** = one slot, the whole group `Start`..`Stop` (kind `attack1..3`), sheet `rate` only: no phases, no `Charge`
  window, no accents. `Hit(0)` from `Hit` - `.hit_min_s` to `Hit` + `.hit_tail_s` (also the rule for any section without
  `Min Hit`); `Turn` to the window's start, `NoTurn` to its end; `Cancel(Move)` from `Hit` + `.cancel.move_s`, capped at
  `Stop`. `MoveReach.travel_m` = the cooked root travel `Start`..`Hit`.
- **No `Hit` key** (9 moves: `cr_draugr`, `draugrlord`, `ice troll`, all three groups; only `Start` / `Stop`): modern
  puts `Hit` at `Start` + `mw.combat.creature.hit_frac` × (`Stop` - `Start`) (×, 0.2-0.8, 0.5; source AN32b: the keyed
  moves' `Hit` sits at 0.50 of the span, median; p10 0.37, p90 0.67; net `Gameplay`). The cook logs it once per rig;
  `CreatureCombat.hit_s` (per move, clip s) authors it instead. Classic: the hit lands at the `Start` key, as OpenMW
  ("If there is none, the hit happens with the start key", `apps/openmw/mwmechanics/character.cpp`
  `CharacterController::handleTextKey` @ `5f3aafb`).
- **No `Attack1-3`** (11 rigs). Weapon bipeds (dremora, golden saint, skeleton, liche, liche_king, almelexia,
  almelexia_warrior, lordvivec): the humanoid weapon sheet of the equipped weapon ("Weapon classes as data"), its windows
  cooked from the creature file's own weapon groups (OpenMW: `Creature::hasInventoryStore` = the Weapon flag, then the
  NPC weapon path; `getWeaponAnimation` gives `attack1` to non-bipeds only). Werewolf (`wolf/skinnpc`): the `fist` sheet
  from its own `HandToHand` keys (OpenMW: NPC path, `getHandToHandDamage`; claws hurt health). Raven,
  `heart_akulakhan`: no sheet, a target only.
- **Classic seam** (C5): OpenMW picks uniformly among the consecutive `Attack1..N` that exist
  (`chooseRandomGroup("attack")`); attack 1 / 2 / 3 use the record's chop / slash / thrust damage pair. Modern picks by
  `Tactics` rows.
- **Root travel**: open, combat-adapter.md "Open questions" 4.

## Reuse check

Attack clips (2026-11-26): Bevy 0.20's `ActiveAnimation` has one `speed` per clip (`bevy_animation-0.20.0/src/lib.rs:526,
677-682`), no warp inside a clip; our
montage slots take the playhead from the action clock, so the warp is ours, in `sf_action`/`sf_combat`. crates.io: no
crate maps text keys to gameplay windows. OpenMW `character.cpp` @ `4e718b5`: the key semantics (classic seam).
ReAnimation (github.com/fallchildren/OpenMWReAnimation): alternates keep the vanilla keys driving timing, as here.

## Industry comparison

| Mapping | Industry | Ours |
|---|---|---|
| Attack phases | ER: startup, active 2-4 frames, recovery with attack, dodge and move cancels (game-file frame data [V]) | match: anticipation, charge, strike, one follow; cancels after `Hit` |
| Per-phase rate | UE montage Time Stretch Curve [M]; Morrowind: one speed per weapon (`character.cpp` 1322) | piecewise `TimeWarp` per move |
| Anticipation, overshoot | key-pose exaggeration (Overgrowth, GDC 2014 [M]) | additive of the clip's own key poses |
| Hit reactions | ER: tiered stagger, directional by side [M]; OpenMW: random Hit1-5 | tier = clip + rate + free point; side via `HitReact` |
