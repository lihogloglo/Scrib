# Combat adapter: what Bevyrrowind adds to the framework's combat

Status: spec (2026-10-27), nothing built. Framework design: `../../../superfargoth/docs/specs/combat.md` (feel layer, sheets,
`CombatRules` seam, build plan C1-C7). Classic requirements: `../research/combat-classic-openmw.md`. ROADMAP 3.5.

## Crates

- **`mw_combat`** (new, Bevy-free, hakari `final-excludes`): weapon record to sheet mapping, text-key window synthesis for
  Morrowind clips, record stats to `Fighter` numbers; data `data/movesets/*.ron`. C1b.
- **Bevy glue in existing crates**: the player in `mw_player_bevy`, NPC combatants and ragdoll routing in `mw_walkers_bevy`
  (it owns NPC walkers and their physics reactions), the test level in `mw_exterior_bevy` (`level/`), clip slots in
  `mw_graphs/data/animation/sources.ron`, bindings in `mw_controls_bevy`.
- **`mw_rules`** (C5, not before): the classic `CombatRules`, ported from OpenMW (cite file + commit), behind the same seam.

## Weapon records to moveset sheets

The equipped weapon (`mw_actors::held` loadout: weapon type, two-handed) picks `ModeState::moveset`:

| Morrowind weapon type | Sheet | When |
|---|---|---|
| short blade, long blade 1H, blunt 1H, axe 1H | `sword_1h` | C1 |
| none (hand-to-hand) | `fist` (UAL1 `Punch_Jab`, `Punch_Cross`, `Kick`) | C3 |
| long blade 2H, blunt 2H, axe 2H | `greatsword_2h` | C3 (clips missing, combat.md open question 2) |
| spear, staff (two-wide) | `spear_2h` | C3+ |
| bow, crossbow, thrown | ranged, out of scope | later |

Per-weapon numbers go into the `Fighter`, not the sheet: `offense.attack` = the record's max damage for the hit row's
`kind` (`chop`, `slash`, `thrust` tags map to the record's three ranges) x condition / max condition; weapon `reach` scales the
sheet's blade segment (C1: sheet default; later the mesh length along the weapon bone). Creatures get per-rig sheets with
their `Attack1-3` clips (C6).

## Morrowind clips as the fallback layer

Every combat slot ends with a Morrowind clip fallback (`ClipFallback::Clip`) unless the content has none (rolls, dodges):
- Attacks: `WeaponOneHand@chop|slash|thrust`, `WeaponTwoHand@...`, `WeaponTwoWide@...`, `HandToHand@...` (graph clip names,
  `mw_graphs::build::WEAPON_SECTIONS`). Reactions: `Hit1`-`Hit5`, `Knockdown`, `Knockout@tail` (get-up), `Death1`-`Death5`.
  Guard: `Shield@block`.
- **Window synthesis** (`mw_combat::windows`, R6 adapt; text-key kinds already parsed, `mw_anim/src/text_keys.rs:32-53`): one
  `Hit(0)` window from `min hit` (or `hit` - 0.05 s) to `hit` + 0.05 s; `Charge` from `min attack` to `max attack`;
  `Cancel(Dodge)` from `large follow start` (else `follow start`); `Cancel(Move)` at `follow stop`; `Accept(all)` from
  `min attack`. One hit per attack (the original honours one hit key). The sheet's row numbers still apply.
- These clips animate the upper body: in the modern profile they play in the full-body slot (fallback quality, owner accepts);
  the classic profile plays them in the `upper` slot over the movement set.

## Health, stamina, fatigue

- **Health**: NPC and creature records (NPDT health); the player's from the `player` record. `PlayerVitals` in
  `mw_player_bevy/src/hud.rs` is a scaffold whose TODO points here: C1b replaces its health and fatigue with the player
  `Combatant`'s vitals (magicka stays until the stat system owns it).
- **Stamina (modern)** = record fatigue x 0.5 (data, `mw_combat` constants; Morrowind.esm's player: 200 → 100). The HUD's
  fatigue bar shows stamina; the name stays "Fatigue" in the Morrowind theme. Stat bands from attributes later (C5).
- **Classic**: fatigue is the Morrowind pool, never gates a start; the fatigue term scales hit chance, block and damage
  inside `mw_rules`.

## Classic resolution (C5, `mw_rules`)

`ClassicCombat: CombatRules`, ported from OpenMW `4e718b5` (research note §1): `begin_swing` = hit chance at swing prep
(`combat.cpp` `getHitChance`, hashed 0..99 roll), strength from the `Charge` window; `resolve_contact` = `Npc::hit` damage
(min/max by strength, condition, strength term), block (`blockMeleeAttack`: shield, angle band, block chance), armor
(`adjustDamageForArmor`), knockdown odds (`applyStagger`: tier hint `Knockdown` or `Small`); `can_start` always true;
`cost` = `applyFatigueLoss` at the hit; `regen` = fatigue regen. Moves use `source: Cone` (reach x `fCombatDistance`, cone
`fCombatAngleXY`); sheets leave i-frames, armor and poise empty; the Classic mode profile turns lock-on and hit-stop off.
A new game picks the rule set (ARCHITECTURE §1b).

## Ragdoll routing (Knockdown, Death)

The framework's `PhysicsTakeover { impulse, point }` goes, in `mw_walkers_bevy`, to: the Delirium takeover when that
experiment is on and the skeleton is human (`mw_delirium_bevy::DeliriumReaction { hit: Some(DeliriumHit { point, impulse,
part: None }) }`; `DeliriumHandedBack` reports the hand-back), else the walkers' powered-ragdoll knockdown
(`mw_walkers_bevy/src/physics.rs`, get-up from `Knockout@tail`'s first frame). Death: the graph's `dead` state, then limp
ragdoll (`DEATH_HANDOFF`), as the dev kill key does today. Hand-back ends the forced move in `sf_action`. Cosmetic `HitReact`
springs stay on every landing (Additive tier).

## C1b: game items

- **Sheet** `mw_combat/data/movesets/sword_1h.ron` with combat.md's C1b table.
- **Slots** in `sources.ron` (graph `humanoid`, root `Progress`): `attack.sword_1h.light{1,2,3}` (UAL2 `Sword_Regular_A/B/C`,
  then UAL1 `Sword_Attack`, fallback `WeaponOneHand@slash`); `attack.sword_1h.light{1,2}_rec` (UAL2 `Sword_Regular_A_Rec`,
  `_B_Rec`, fallback `Drop`); `dodge.roll.fwd|left|right` (UAL1 `Roll_RM`, `Dodge_Left_RM`, `Dodge_Right_RM`, fallback
  `Drop`); `react.small.front|left|right|back` (UAL1 `Hit_Chest`, `Hit_Shoulder_L`, `Hit_Shoulder_R`, `Hit_Stomach`,
  fallback `Hit1`-`Hit4`); `react.medium.front` (UAL2 `Hit_Knockback_RM`, root x0.25, fallback `Hit5`); `react.large.front`
  (the same clip, root x0.6). No LaFAN1 candidate in C1. Candidate `events` (per-clip windows) are a new manifest field.
- **Player**: `Combatant` on the player (health from the record, stamina as above, `min_tier: Small`, sheet by weapon);
  the ready-weapon key (F, Morrowind's default) toggles the stance; a Light press while sheathed draws instead of attacking.
- **Bindings** (defaults, the player can rebind): `Sprint` becomes **Dodge/Sprint** (tap = roll on release, hold = sprint;
  Shift, pad East); `Attack` (Light): left mouse, pad right bumper; `Lock`: middle mouse, pad right stick click (ToggleView's
  pad binding moves to D-pad up); `Sneak`'s pad binding moves to left stick click; `ReadyWeapon`: F.
- **Training dummy** (named scaffold `TrainingDummy`, removed when C2's sparring dummy lands): a level actor (a plain human
  NPC record) standing still, `Combatant` health 100, hostile, `Lockable`, never attacks; refills at 0 and plays `Large`.
- **Level `arena`** (scene `level-arena`): 40 m flat floor; player start; dummy 3 m ahead; a second dummy 10 m off (C1c
  lock, switch later); a 1 m ledge for the lock break on mantle (C1c). Cameras `chain` (side view of player and dummy) and
  `behind`.
- **Visual checks**: new `docs/visual-checks/combat.md`; CB-1 (C1b): three-hit chain and a roll through the dummy; look for
  hits landing when the blade arrives (not before), hit-stop reading as impact (not a stall), the stamina bar draining per
  hit and refilling after recovery, no slide at the roll's end. CB-2 (C1c): locked strafe and four-way rolls around the dummy.
