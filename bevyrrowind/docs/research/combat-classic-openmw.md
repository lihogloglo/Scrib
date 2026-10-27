# Combat: how classic Morrowind (via OpenMW) resolves melee, and what the seam must allow

Research note for ROADMAP 3.5. Companion of `superfargoth/docs/research/combat/01-elden-ring-reference.md` (modern feel layer
and the rules seam). Scope: only what the classic `mw_rules` implementation needs from the seam (ARCHITECTURE §1b). GPL
port rules: cite file and commit when porting. Source: OpenMW `master` at `4e718b500ffee6657db90297d7b8a08a57d439bc`
(2026-10-23, via `gh api`). Paths below are in that repo. GMST names are the engine's game settings; default values not
quoted here must be read from the data files (defaults seen: `fCombatArmorMinMult` 0.25, `fKnockDownMult` 0.5, from
`apps/opencs/model/world/defaultgmsts.cpp`). Where OpenMW differs from vanilla on purpose, follow vanilla (§1b).

## 1. The pipeline (one melee swing)

1. **Swing and wind-up, driven by animation text keys** (`apps/openmw/mwmechanics/character.cpp`). A held attack plays up to
   `<weapon>: <attack> max attack`; `calculateWindUp()` (`character.cpp:~1230-1245`) returns
   `(currentTime - "min attack") / ("max attack" - "min attack")` clamped to 0..1. That is the **attack strength**; with no
   usable keys (creatures, random attack animations) it is `min(1, 0.1 + random)` (`prepareHit`, `:~1251-1254`). Release plays
   the rest of the clip from a start point skipped by strength (`startPoint = 1 - strength`, `:~1777`), then the `follow`
   section is picked by strength: small < 0.33, medium < 0.66, else large (`:~1797`).
2. **Hit key.** The damage lands when the clip reaches `"chop hit"`, `"slash hit"`, `"thrust hit"` or a bare `"hit"` in
   `attack1/2/3` groups (`character.cpp:1083-1108`); only one hit key per attack is honoured (`mReadyToHit`). The attack type
   (chop, slash, thrust) comes from the key or group: attack1 = chop, attack2 = slash, attack3 = thrust. The swing is
   *timed by the animation*, there is no separate swing clock. `"min hit"` marks where a minimum-strength swing connects.
3. **Hit chance** is decided at swing prep (`prepareHit`, `:~1256-1262`: `evaluateHit`; a miss sets strength to 0), by
   `getHitChance` (`apps/openmw/mwmechanics/combat.cpp:316-350`):
   `attackTerm = (skill + agility/5 + luck/10) x fatigueTerm + fortifyAttack - blind`;
   `defenseTerm = victim evasion` (0 if the victim is knocked down, paralysed, or an unaware non-combatant target of the
   player) `+ min(100, fCombatInvisoMult x (chameleon + invisibility))`; chance = `round(attackTerm - defenseTerm)` percent,
   rolled against a uniform 0..99 (the roll is in `evaluateHit` of the class, `mwclass/npc.cpp`).
4. **Damage** (`mwclass/npc.cpp:573-678`, `Npc::hit`): weapon damage = `min + (max - min) x attackStrength` for the attack type's
   min/max (chop/slash/thrust bytes in the weapon record, `components/esm3/loadweap.hpp`); then `adjustWeaponDamage`
   (`combat.cpp:452-476`): `x itemCondition/maxCondition`, `x (fDamageStrengthBase + strength x fDamageStrengthMult x 0.1)`;
   then `reduceWeaponCondition`, `resistNormalWeapon` (silver and magical weapon rules), werewolf multiplier. Unarmed uses
   `getHandToHandDamage` (`combat.cpp:478`, skill and strength based; can hit fatigue instead of health). The player earns
   weapon skill use on a successful hit. Unaware victim and player attacker: `x fCombatCriticalStrikeMult` (sneak critical).
   Victim knocked down: `x fCombatKODamageMult`.
5. **Block** (`combat.cpp:71-178`, `blockMeleeAttack`): not when the victim is knocked down, in hit recovery or paralysed; needs
   a shield in the left slot and the victim facing the attacker inside the angle band
   (`fCombatBlockLeftAngle` .. `fCombatBlockRightAngle`, degrees, signed). Block chance = `(blockSkill + 0.2 agility + 0.1 luck)
   x (swing x fSwingBlockMult + fSwingBlockBase)` x `fBlockStillBonus` if standing still `x fatigueTerm`, compared with the
   attacker's own term (rest of the function). A block sets damage to 0, damages the shield, and may stagger the attacker.
6. **Armor** (OpenMW 0.49 moved it to Lua: `files/data-mw/scripts/omw/combat/common.lua`, `adjustDamageForArmor`):
   `x = damage / (damage + armorRating); damage x max(x, fCombatArmorMinMult)`, with armor rating from a skill-adjusted sum of
   worn pieces (`mwclass/npc.cpp:1025-1060`, `getArmorRating`); a random armor slot loses condition (`pickRandomArmor`).
7. **Stagger/knockdown** (`files/data-mw/scripts/omw/combat/local.lua`, `applyStagger`): with `agilityTerm = agility x
   fKnockDownMult` and `knockdownTerm = agility x iKnockDownOddsMult x 0.01 + iKnockDownOddsBase`, a hit with raw health damage
   >= agilityTerm and `knockdownTerm <= roll(0..99)` knocks the victim down; otherwise it only plays hit recovery. So tiers are
   two (hit recovery, knockdown) and are *stat-driven*, not poise-driven.
8. **Fatigue cost of the swing** (`combat.cpp:532-554`, `applyFatigueLoss`): `fFatigueAttackBase + normalizedEncumbrance x
   fFatigueAttackMult + weaponWeight x attackStrength x fWeaponFatigueMult`, deducted at the hit key. Fatigue also scales hit
   chance, block and many formulas through `getFatigueTerm()`. It is a *gradual* penalty, never a gate on starting a swing.
9. **Reach** (`combat.cpp:661-677`): `fCombatDistance x weapon.reach` (hand-to-hand: `fHandToHandReach`); target in reach when
   height difference < reach and distance to bounds < reach. Contact point and victim come from `getHitContact`
   (`combat.cpp:679-780`): a line trace from the actor's eye along facing, tolerance cone `fCombatAngleXY/Z` (stored as
   sine x 90, about 42 degrees at 60), the nearest actor in reach wins; there is no weapon-volume sweep. Done once at swing prep.

## 2. What the seam must allow (requirements on the framework)

- **Timing from clip keys, not our windows.** Classic needs the swing resolved at a clip key (`hit`), with the wind-up measured
  between `min attack` and `max attack`. Our `hit(n)` windows plus a `charge` window can express this: the classic moveset
  sheet derives one `hit` window around the hit key and a `charge` window from min/max attack (the synthesis R6 §12 already
  proposes for these clips). The text-key kinds exist in `bevyrrowind/crates/mw_anim/src/text_keys.rs:32-53`.
- **`Contact.strength` (0..1) from wind-up**, available at both swing start and hit time, and a **pre-roll hook**: classic decides
  hit or miss at swing prep (before the hit key) and zeroes the strength on a miss. The seam needs `begin_swing(attacker,
  target_candidate) -> SwingPlan { will_hit, strength }` as well as `resolve_contact`. Modern leaves it a no-op.
- **Target by reach and cone**, not by sweep: classic resolution takes "the nearest actor in the cone in reach", so the feel layer
  must be able to hand the rules a candidate list (hurtbox overlaps, or a cone query) rather than only sweep hits. Provide a
  `ContactSource {Sweep, Cone}` flag per move.
- **Stats as input, many of them**: skill, attributes, fatigue term, encumbrance, evasion, equipment condition, magic effect
  magnitudes (fortify, blind, chameleon, shield). `Stats` must be an opaque, content-defined bag; the framework never
  reads it. Outcome must carry several pools (health, fatigue, magicka), item damage (condition), and effects.
- **Outcome flows both ways**: block can hurt the attacker (shield, stagger), so `Outcome` has an attacker-side result and a
  per-side reaction hint (hit recovery vs knockdown).
- **Fatigue instead of stamina gates**: classic never refuses to start an action for lack of the resource and scales
  effectiveness instead; the framework's `can_start` must be rules-defined (always true), and regen a rules function.
- **No i-frames, no roll, no poise.** Classic content must be able to leave `iframes`, `armor`, poise pools and dodge empty in its
  sheet without the feel layer misbehaving; stagger tier must be a rules output, not framework poise.
- **Lock-on and hit-stop are optional**: classic mode profile turns lock-on and hit-stop off (the `Combat` profile fields in
  action.md "Mode profiles").
- **Random rolls** must hash (seed, character, tick) for co-op determinism; classic uses uniform 0..99 rolls.
- **Classic rig clips**: attack groups per weapon type (`attack1/2/3`, `swimattack`, `crossbow`, `bowandarrow`, `throwweapon`,
  per-weapon groups such as `onehand`, `twohand`), `block`, `hit1-5`, `knockdown`, `knockout`, `death1-5`; upper-body slot
  playback exists in `mw_graphs` (`upper` slot: attacks, casts, equip, block).

## 3. Not covered here

Projectile and ranged hit rules (`projectileHit`, `combat.cpp:224`), spells and enchant-on-strike, creature attacks and their random
attack animations, crime/aggression on hit (`friendlyHit`, `actorAttacked`), werewolf, skill-gain numbers. Port from the same
files when the classic implementation is built (C5), citing file and commit.
