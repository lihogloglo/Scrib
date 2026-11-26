# Combat adapter: what Bevyrrowind adds to the framework's combat

Status: spec (2026-10-27); built: C1b, C1c, C4, C2a (sparring dummy, below). Framework design: `../../../superfargoth/docs/specs/combat.md` (feel layer, sheets,
`CombatRules` seam, build plan C1-C7). Classic requirements: `../research/combat-classic-openmw.md`. ROADMAP 3.5.
C0d (2026-11-26): Morrowind data onto the C0a-C0c fields (classes, poise, damage packet, ranged, skill, creatures, spells,
allies, sneak, props); numbers `Guess` until their slices; sheet windows wait for the anim lane (AN32 measures, AN33 cooks).
2026-11-26: "Morrowind attack clips" (from AN32a) sets AN33-AN35 and AN37's input.

## Crates

- **`mw_combat`** (new, Bevy-free, hakari `final-excludes`): weapon record to sheet mapping, text-key window synthesis for
  Morrowind clips, record stats to `Fighter` numbers; data `data/movesets/*.ron`. C1b.
- **Bevy glue in existing crates**: the player in `mw_player_bevy`, NPC combatants and ragdoll routing in `mw_walkers_bevy`
  (it owns NPC walkers and their physics reactions), the test level in `mw_exterior_bevy` (`level/`), clip slots in
  `mw_graphs/data/animation/sources.ron`, bindings in `mw_controls_bevy`.
- **`mw_rules`** (C5, not before): the classic `CombatRules`, ported from OpenMW (cite file + commit), behind the same seam.

## Weapon classes as data (C3d)

The equipped weapon's record type picks `ModeState::moveset` through `mw_combat/data/weapon_classes.ron` (type → sheet,
directions). One sheet per Morrowind animation group, because the clips decide the moves (roadmap decision 6); a pack
whose clips add OpenMW's extra groups (`shortbladeonehand`, `bluntonehand`, `blunttwohand`) maps those types to its own
sheets there, no code. Groups: OpenMW `apps/openmw/mwworld/store.cpp` 887-922 (`Store<ESM::WeaponType>::setUp`) @ `4e718b5`.

| WEAP type | Group (`WEAPON_SECTIONS`) | Sheet | Directions |
|---|---|---|---|
| short blade, long blade 1H, blunt 1H, axe 1H | `WeaponOneHand` | `sword_1h` (live) | chop, slash, thrust |
| long blade 2H, blunt 2H close, axe 2H | `WeaponTwoHand` | `greatsword_2h` | chop, slash, thrust |
| spear, blunt 2H wide (staff) | `WeaponTwoWide` | `spear_2h` | by record (below) |
| none | `HandToHand` | `fist` | chop, slash, thrust |
| bow, crossbow, thrown | `BowAndArrow`, `Crossbow`, `ThrowWeapon` | `bow`, `crossbow`, `thrown` | `shoot` ("Ranged weapons") |

- **Directions** (decision 7: the stick picks chop/slash/thrust): a direction is offered when its record max ≥
  `mw.combat.dir_min_frac` × the record's best max (×, 0-1, Guess 0.5); a refused one plays the best (OpenMW's "always
  use best attack" option, as a rule). A spear with small chop and slash ranges offers thrust only: per record, not per class.
- **Per-weapon numbers** live in the rules' stats table (keyed by `CharacterId`, combat.md "The rules seam"), never in the
  sheet: damage per direction (C13), poise by weight ("Poise"), stamina by weight (`cost` = `base × (.stamina.weight_base
  + .stamina.per_weight × weight)`, `mw.combat.stamina.*`, ×, 0-2, Guess 0.8; per unit, 0-0.05, 0.01; the shape of OpenMW
  `applyFatigueLoss`, `combat.cpp` 532-554); `reach` scales the sheet's blade segment. `offense.attack` holds the slash
  value for code that reads one number.
- **Record `speed`**: modern plays every weapon of a sheet at the sheet's timing (one timing per class, ER; AN34 retimes
  per phase); classic (C5) plays at the record speed, as OpenMW.
- **Windows**: AN33 cooks them from text keys, AN34 retimes them per phase ("Morrowind attack clips"); no window
  numbers here. Missing moves (backstep, 2H guard, parry, riposte, kick) are AN37's key-pose clips.

## Morrowind clips as the fallback layer

Every combat slot ends with a Morrowind clip fallback (`ClipFallback::Clip`) unless the content has none (rolls, dodges):
- Attacks: `WeaponOneHand@chop|slash|thrust`, `WeaponTwoHand@...`, `WeaponTwoWide@...`, `HandToHand@...` (graph clip names,
  `mw_graphs::build::WEAPON_SECTIONS`). Reactions: `Hit1`-`Hit5`, `Knockdown`, `Knockout@tail` (get-up), `Death1`-`Death5`.
  Guard: `Shield@block`.
- **Windows** come from the clips' text keys (`mw_combat::windows`, kinds parsed in `mw_anim/src/text_keys.rs:32-53`):
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

## Health, stamina, fatigue

- **Health**: the character's stat sheet (stats.md S5, `mw_player_bevy/src/vitals.rs`): `Combatant` health = sheet
  `health` 1:1 each tick, combat's change goes back as `PoolAdjust` (cause `Combat`); NPCs and creatures from their records
  (NPDT health), the player from the `player` record (chargen skipped), the training dummy set to 100.
- **Stamina (modern)** = sheet fatigue x 0.5 (`mw_combat::STAMINA_PER_FATIGUE`; Morrowind.esm's player: 200 → 100),
  differences back x 2. The HUD's fatigue bar shows the sheet's fatigue (x 2 the stamina); "Fatigue" in the Morrowind
  theme. Stat bands from attributes later (C5).
- **Classic**: fatigue is the Morrowind pool, never gates a start; the fatigue term scales hit chance, block and damage
  inside `mw_rules`.
- **Fists (modern, decision 20)**: hand-to-hand damages health like any weapon (CP2077 close combat), tagged `NonLethal`
  (Q10, owner 2026-11-26), so a brawl ends in a knockout ("Non-lethal and mercy"). **Classic** (C5, named only, not designed
  or built now): Morrowind's fatigue-first rule, fists hurt fatigue unless the victim is knocked down or paralysed, then
  health x `fHandtoHandHealthPer`; werewolf claws always health (OpenMW `apps/openmw/mwmechanics/combat.cpp` 478-530
  `getHandToHandDamage` @ `4e718b5`).

## Poise (modern)

combat.md "Poise" leaves the pool max, row poise and armour windows to content: `mw_combat::poise` computes them at stat
refresh (equip, attribute change) for the rules wrapper. Keys `mw.combat.poise.*`, net `Gameplay`, all `Guess`.
- **Pool max** (player and NPCs alike): `.base + Σ share[slot] × class[piece] + .per_attr × (Strength + Endurance) / 2`.
  Shares are Morrowind's armour-rating weights (cuirass 0.3; shield, helmet, greaves, boots, pauldrons 0.1; gauntlets
  or bracers 0.05; sum 1; OpenMW `mwclass/npc.cpp` 1069-1079 @ `4e718b5`). Weight class as `Armor::getEquipmentSkill`
  (`mwclass/armor.cpp` 114-185): weight ≤ `i<Slot>Weight` × `fLightMaxMod` light, ≤ × `fMedMaxMod` medium, else heavy.
  `.base` (poise, 0-200, 40), `.light` / `.medium` / `.heavy` (poise for a full set, 0-200, 15 / 35 / 60), `.per_attr`
  (poise per point, 0-1, 0.2). Unarmoured average 50, full heavy at 100 Strength/Endurance 120: Veloren's 100-pool
  scale (C-R1, `docs/research/gameplay-2026-11-25/combat-veloren.md` §4), heavy armour about twice a bare body (owner Q2).
- **Hit poise**: rows carry `poise` on that scale; Bevyrrowind sets `combat.poise.per_mv` 20 (`Mod` layer) so a 1H light
  ≈ 20 (a bare body breaks on the third light, full heavy on the sixth). The rules multiply by weapon weight
  `(weight / .ref_weight)^.weight_exp` (`.ref_weight` 20 weight units, a longsword; `.weight_exp` 0-1, 0.5: dagger ≈
  0.45, warhammer ≈ 1.7), direction `.chop` / `.slash` / `.thrust` (×, 0.5-2, 1.2 / 1.0 / 0.8), blunt `.blunt_mult` (×,
  1-2, 1.25; Veloren `Crushing`) and charge `lerp(1, .charged_mult, strength)` on Heavy rows (×, 1-3, 2.0); heavy rows are
  authored at `.heavy_mult` × their light (×, 1-3, 1.5).
- **Framework values** (`Mod` layer, from Veloren's ladder): `combat.poise.regen_delay_s` 1.0 (its 1 s re-stun buffer),
  `.regen_per_s` 0.1 (its 10/s cap on a 100 pool, flat where Veloren ramps), `.broken_s` 1.5 (its `Dazed` stun, the
  deepest standing one and its crit window), `.knockdown_overflow` 0.5. Its thresholds (50/30/15/5 of 100) are not
  modelled (our pool breaks once and escalates by row tier); its stuns 0.4 / 0.7 / 1.5 / 3.0 s are the target lengths of
  the Small / Medium / Large / Knockdown reactions handed to AN35 (reaction map), not tunables here.
- **Creatures** (by size): `.base × (h / h_human)^.size_exp + .per_attr × (Strength + Endurance) / 2`; `h` = the
  skeleton's `Bounding Box` height × record scale (the box the mover capsule uses), `h_human` the same for
  `base_anim.nif`; `.size_exp` (0-3, 2). At `h / h_human ≥ .stance_size` (×, 1-5, 2) `break_tier: Knockdown` (ER stance
  break). Not level (late small creatures would shrug off any blade) nor soul (magic power). Overrides: `poise_mult`.
- **Cast armour**: the cast windup's `Armor(.cast_armor_small)` (scale, 0-1, 1: none) for small spells and
  `Armor(.cast_armor_big)` (0-1, 0.5) for big ones ("Spells"); a break inside it cancels the cast (combat-delivery.md).
- **Classic**: no pool (`poise: None`); knockdowns stay `applyStagger` (C5).

## Damage types, armour and resistances (C13)

- **Types** (`DamageType` ids): `mw.physical`, the default (weapons, fists, creature attacks, thrown bodies); `mw.fire`,
  `mw.frost`, `mw.shock`, `mw.poison`, `mw.magicka` for typed hazards (lava, traps, IN28) and creature rows a pack types.
  No physical subtypes: Morrowind armour has one rating and nothing resists by blade kind; the direction stays the row's
  `kind`. Spell and enchantment damage is never a packet: it lands as effects (effects-adapter.md).
- **Physical packet** (modern `mw_combat::ModernRules`, wrapping `StandardRules`): `max[kind] × condition / max condition
  × str_term × skill_term × mv × strength`. `max[kind]`: the WEAP chop/slash/thrust max; creatures `Attack[n]` max for
  `attack1..3`; fist: Hand To Hand × `fMaxHandToHandMult` (OpenMW `getHandToHandDamage`, `combat.cpp` 478-485).
  `str_term` = `fDamageStrengthBase + Strength × fDamageStrengthMult × 0.1` (`adjustWeaponDamage`, `combat.cpp`
  452-476; 1 at 50); `skill_term` in "Skill mapping". The max, not a roll: modern has no hit roll; charge is `mv` and
  `strength`.
- **Armour → `defense.absorb`**: `.absorb_max × AR / (AR + .ar_half)`, `mw.combat.armor.absorb_max` (0-1, 0.8;
  Reference Skyrim's 80 % cap), `.ar_half` (AR, 10-200, 60; Reference Veloren `p/(60+p)`), on `mw.physical` only. AR is
  Morrowind's skill-adjusted, condition-weighted sum (`Npc::getArmorRating`, `npc.cpp` 1025-1080: piece armour × skill /
  `iBaseArmorSkill`, bare slots `fUnarmoredBase1/2`, plus Shield); creatures: Shield + species `armor` (OpenMW gives them
  Shield only, `creature.cpp` 565-569). Not Morrowind's `damage / (damage + AR)` (`omw/combat/common.lua` 167-171): it cuts
  small hits by up to 75 % and big ones little, so armour's worth depends on the hit; ER negation and Skyrim take a fixed
  share, and heavy armour's job against light hits is poise. Classic keeps OpenMW's.
- **Resistances** (`1 − clamp(resist − weakness, −100, 100) / 100`, effects' modern resist, no roll): Resist/Weakness to
  Normal Weapons on `mw.physical` from a normal weapon (not Silver, Magical or enchanted: `isNormalWeapon`,
  `resistNormalWeapon`, `combat.cpp` 180-207); Resist/Weakness Fire, Frost, Shock, Poison, Magicka on typed entries.
  Silver on werewolves: `fWereWolfSilverWeaponDamageMult` (`combat.cpp` 209-222), both rule sets.
- **On-strike enchantments** (`ItemCast::OnStrike`, instant, owner Q6): at a contact with `Outcome.deliver`, the weapon's
  or ammo's WhenStrikes entries go to `sf_effects` as `Inflict`: Touch and Target entries on the struck target, Self on
  the attacker (on the target for projectiles); never a bolt (OpenMW `spellcasting.cpp` 299-310 with
  `launchProjectile` false, `applyOnStrikeEnchantment` `combat.cpp` 49-69). Elemental shields (`applyElementalShields`,
  `combat.cpp` 352) hit melee attackers the same way. Poisons later, on this path; `Outcome.buildup` stays empty.

## Ranged weapons (C8b data)

- **Draw** (`MoveSpec.draw`, cooked by AN33 from the `shoot` keys, numbers from AN32): `min_s` = `shoot min attack` −
  `shoot start`, `full_s` = `shoot max attack` − `shoot start`, release at `shoot release`. Crossbow: `full_s = min_s`
  (full strength once loaded, ER and Skyrim crossbows). `move_scale` = `combat.delivery.draw.walk_scale`.
- **Ammo**: a bow needs an equipped arrow, a crossbow a bolt (`store.cpp` 914-918); `can_start` refuses without;
  `Released` spends one. Thrown weapons spend the stack itself.
- **Damage**: launcher chop range + ammo chop range, each `min + (max − min) × strength` (`combat.cpp` 253-261); a thrown
  item counts twice, as in OpenMW (projectile = weapon; vanilla thrown records are balanced on it); C13 terms after.
- **Speed** (`ProjectileSpec` per launcher): `fProjectileMinSpeed + (fProjectileMaxSpeed − fProjectileMinSpeed) ×
  strength`, thrown `fThrownWeaponMinSpeed..MaxSpeed` (`mwrender/weaponanimation.cpp` 133-163), units/s ×
  `mw_world::convert::UNIT_METERS_F64` (400-3000 u/s = 5.7-43 m/s). Gravity and drag `Guess` on the spec until C8a.
- **Recovery**: a non-enchanted arrow that hits an actor lands in its inventory at `fProjectileThrownStoreChance` %
  (`combat.cpp` 297-310); one stuck in the world is a pickup while in the `StuckSet` (modern). Enchanted ammo is spent.

## Skill mapping

- **NPCs and creatures → `skill`** (combat-npc.md "Skill reactions"): `clamp((W + 0.2 Agility + 0.1 Luck − .skill_lo) /
  (.skill_hi − .skill_lo), 0, 1)`, W = the equipped weapon's skill (Marksman for ranged; creatures: the record's Combat
  skill), the attack term of OpenMW `getHitChance` (`combat.cpp` 316-350); `mw.combat.skill_lo` / `.skill_hi` (points,
  0-200, Guess 20 / 115: 115 = skill 100, Agility 50, Luck 50). Refreshed with stats, × `DifficultyRow.skill_scale`.
- **Player** (modern feel, existing seams only): weapon skill → `skill_term` = `.skill_dmg_base + .skill_dmg_per × W`
  (Guess 0.75, 0.005: 1 at 50, 1.25 at 100); Block → `defense.guard_stability` = `.block_base + .block_per × Block` (Guess
  0.6, 0.008); armour skills through AR, Strength through `str_term`, Strength and Endurance through poise. Agility, Speed
  and Luck get no combat term (no new framework seams).

## Creatures (C6f)

- **Sheet per rig**: one move per `Attack1-3` group the skeleton has (`kind` `attack1..3`, windows by AN33); damage from
  the record's `Attack[n]` (C13). Tactics in `mw_combat/data/creatures/<skeleton>.ron` (record `CreatureCombat {
  tactics: Tactics, armor, poise_mult, overrides }`): records on one rig share it. Missing file: a generated default, each
  move a `Melee` row of weight 1, and a move whose baked `MoveReach.travel_m` ≥ `mw.combat.creature.lunge_m` (m, 0.5-5,
  Guess 1.5) becomes `Charged` with `whiff: WallStagger`.
- **Overrides** by record id: `skill`, `poise_mult`, `armor`, `tactics` (another file), `phases` (boss phases per
  combat-npc.md, `HealthBelow` / `PoiseBreaks`); content adds them per id, this spec lists no bosses.

Example (kagouti; which attack lunges follows AN32's measured travel):

```ron
Tactics(
  sheet: "kagouti", skill: 0.4,
  react: (guard: (0.0, 0.0), dodge: (0.05, 0.2), parry: (0.0, 0.0), counter: (0.1, 0.3)),
  rows: [
    (move: "attack1", token: Melee, weight: 2.0, bias: [(TargetPhase(Recovery), 2.0)]),
    (move: "attack2", token: Melee, weight: 1.0, cooldown_s: 2.0),
    (move: "attack3", token: Charged, weight: 1.0, band: (2.0, 6.0), cooldown_s: 5.0, whiff: WallStagger),
  ],
  phases: [],
)
```

## Spells (C8d data)

- **Slots**: `SpellCast@self|touch|target` as `cast.<sheet>.<move>`, release at `<range> release` (OpenMW
  `character.cpp` 1164).
- **Big (rooted) or small (slowed)**: big when a Touch or Target entry has area ≥ `mw.combat.cast.big_area` (record area,
  0-50, Guess 10) or the spell's cost (`mw.effects.cost`) ≥ `mw.combat.cast.big_cost` (magicka, 0-500, Guess 60). Area and
  cost are the record's own size of a spell (a fireball against a fire bite); Skyrim roots master-level casts, ER slows
  big incantations. Big: `move_scale` 0 and `Armor(cast_armor_big)`; small: `combat.delivery.cast.small_walk_scale`. A
  `#[ignore = "measurement"]` test prints the vanilla split at C8d.
- **`launch_spec(&Delivery)`**: Target entries make a `ProjectileSpec`, speed `fTargetSpellMaxSpeed` × the mean MGEF
  `speed` of the entries (OpenMW `mwworld/projectilemanager.cpp` 67-79, 145-170) × `UNIT_METERS_F64` (1000 u/s ≈ 14.3
  m/s); per effect, not per school (MGEF carries it); race weight scaling off (OpenMW's normalised race speed option).
  An entry with area adds `AreaSpec { placement: AtImpact, falloff: None, los: true }` at the largest entry radius (unit
  conversion as effects.md "Area"); entries land at their own radii. No beams in vanilla.

## Non-lethal and mercy (C22, C23)

Framework side: combat.md "Non-lethal and mercy" (C20, C21); Q10-Q15 answered as recommended (owner 2026-11-26).
- **Tags** (C22, Q10): `weapon_classes.ron` marks `HandToHand` and the blunt types (blunt 1H, blunt 2H close, blunt 2H
  wide) `NonLethal`; a record-id override list in the same file adds "subdue" items or a pack's own; blades, axes,
  spears, ranged and creature attacks stay `Lethal`. Spell damage is never a packet (effects-adapter.md): it kills.
- **Essential NPCs** (NPC_ flag) get `always_recover`: they always go down, never die, and no Finish is offered. Vanilla
  lets them die with a warning (`sKilledEssential`, OpenMW `apps/openmw/mwmechanics/actors.cpp` 1835 @ `4e718b5`):
  modern departs (Q13), Classic keeps vanilla.
- **Yield** (Q11): humans only (NPC records); creatures, daedra and undead never yield (AI37 sends `Yield`, AI lane).
- **Clips**: Morrowind's `Knockdown` / `Knockout` groups (and their swim variants) are the Knockout down and get-up clips
  through the reaction map (AN35; no numbers here); no Morrowind group fits `react.yield`: a key-pose clip (AN37 kind).
- **Crime** (C23, Q14): a hit that downs a non-hostile is Assault (C16's `actorAttacked`); `Finished` or a lethal hit on a
  Yielded or non-hostile victim with a witness is Murder (`actorKilled`); Yielded counts as non-hostile; foes already
  hostile (bandits, attackers) cost nothing.
- **Quests**: framework quest hooks read `Downed` as defeated; ported MWScript `GetDead` and `OnDeath` stay death-only
  (vanilla semantics), so a vanilla kill quest still needs a kill (the Finish prompt).

## Allies, friendly fire, crime (C16)

- **`allied(a, b)`**: either sides with the other, OpenMW `Actors::getActorsSidingWith` (`actors.cpp` 2149-2240 @
  `4e718b5`): a current Follow or Escort package (or one behind only Wander packages) on the other, transitively through
  followers; companions side with the player. A shared faction is not an alliance (vanilla). Melee spares allies in both
  rule sets; arrows, spells and areas hit them (combat-delivery.md "Friendly fire").
- Followers forgive the player's first three hits outside combat (OpenMW `friendlyHit`, `combat.cpp` 783-803; C16).
- **C16**: `HitLanded` with credit on a non-hostile → crime's `actorAttacked` (Assault), `Died { killer }` → `actorKilled`
  (Murder) (crime-adapter.md "`reportCrime` (A2; MM 1227-1507)"); push credit counts as the attacker.

## Unaware and sneak attacks

- **Feed**: MA5 ports `awarenessCheck` (`mechanicsmanagerimp.cpp` 1603) as `DetectionModel` Classic (ai-adapter.md);
  AI15's glue writes `UnawareOf` from its bands (combat-npc.md). Morrowind's sneak critical needs the victim out of combat
  and failing `awarenessCheck` (`npc.cpp` 643-649): the `Unaware` / `Curious` bands of a victim not engaged.
- **Multiplier**: a player's `Backstab` on an unaware victim uses `fCombatCriticalStrikeMult` (default 4,
  `apps/opencs/model/world/defaultgmsts.cpp` 1654; read from the GMST) through `Contact.critical`; Morrowind gives it to
  player melee only (`npc.cpp` 636-649; `projectileHit` has none). NPC and back-cone backstabs keep
  `combat.crit.backstab_mult`.

## Prop targets (C14)

The table is `mw_interaction`'s `DestructionRules` (interaction-adapter.md "Breakables (IN38; bash IN37)"), keyed
`(material, DamageType, hit class)`; combat supplies the type and a hit class from the record (blunt, blade, arrow,
thrown, fist). Example: `(material: "clay", damage: "mw.physical", hit: Blunt, mult: 1.0)`.

## Difficulty (decision 16)

Modern: `DifficultyRow`s in `mw_combat/data/difficulty.ron` (tokens, skill, telegraph, damage; Guess, Doom shape).
Classic: OpenMW `scaleDamage` (`difficultyscaling.cpp` 10-37, `fDifficultyMult`) on damage to and from the player.

## Classic resolution (C5, `mw_rules`)

`ClassicCombat: CombatRules`, ported from OpenMW `4e718b5` (research note §1): `begin_swing` = hit chance at swing prep
(`combat.cpp` `getHitChance`, hashed 0..99 roll), strength from the `Charge` window; `resolve_contact` = `Npc::hit` damage
(min/max by strength, condition, strength term), block (`blockMeleeAttack`: shield, angle band, block chance), armor
(`adjustDamageForArmor`), knockdown odds (`applyStagger`: tier hint `Knockdown` or `Small`); `can_start` always true;
`cost` = `applyFatigueLoss` at the hit; `regen` = fatigue regen. Moves use `source: Cone` (reach x `fCombatDistance`, cone
`fCombatAngleXY`); sheets leave i-frames, armor and poise empty; the Classic mode profile turns lock-on and hit-stop off.
A new game picks the rule set (ARCHITECTURE §1b).

## Ragdoll routing (Hit, Knockdown, Death)

Design: the framework's `PhysicsTakeover { impulse, point }` goes to the Delirium takeover when the skeleton is human,
else the walkers' powered-ragdoll knockdown (`mw_walkers_bevy/src/physics.rs`, get-up from `Knockout@tail`'s first
frame). Death: the same hook (kind `Death`): Delirium's terminal mode for humans, else the graph's `dead` state, then
limp ragdoll (`DEATH_HANDOFF`). Hand-back ends the forced move in `sf_action`. Cosmetic `HitReact` springs
stay on every landing (Additive tier).

As built 2026-11-05 (hits and deaths of walkers: the `RagdollTakeover` hook, Delirium `HitRules`, measured pushes, corpses;
PH-15, PH-16): [archive/combat-adapter-history.md](archive/combat-adapter-history.md#ragdoll-routing-as-built-2026-11-05).

## Character shapes (Morrowind)

Design 2026-11-05 (nothing built; framework spec `superfargoth/docs/specs/character-shapes.md`, slices S0-S7). Why
balls pass through a Delirium-held body (PH-15): `Held` brings `Dormant`, `sf_physics_bevy` `capsules()` drops the
mover capsule of a `Dormant` mover, and Delirium's parts live in delirium-physics, so no `PhysicsEvent::Hit` is ever
made for it and the `live_only` routing above never runs. The same hole covers frozen corpses (and corpse drag).

- **Source**: Morrowind has no per-bone collision for actors (the root `Bounding Box` node sizes the actor box, OpenMW's
  actor shape; we keep it for the mover capsule and the shape set's `bound`). The shape set is the framework's
  `ragdoll.cook` from the rig profile of `base_anim.nif` / `base_anim_female.nif` and each creature skeleton, radii fit at
  bake to the race's default skinned body parts (armour and clothing ignored: the set is per rig, not per outfit; a
  per-race scale comes from the actor's height/weight as for ragdolls).
- **Delirium as proxy holder** (S3): `mw_delirium_bevy` registers its takeovers as `ProxyHolder::Layer`; proxies follow
  the shown pose after its copy-back (not Delirium's own 21 part boxes), so balls hit what the player sees. An impact's
  shape → joint → Delirium part: the part whose `table.parts[k].joint` is the joint or its nearest ancestor; the push is
  `Takeover::hit(part, at, impulse x HitRules::gain, React)` directly, every contact start of at least 1 N·s, no
  per-frame dedup (rapid balls each push). Proxies leave with the hand-back (the mover capsule returns with `Dormant`
  removed).
- **Animated walkers** (S4): the walkers' `knockdown()` passes the shape's joint in `PhysicsTakeover { joint }`;
  `hit_reaction` uses it as `DeliriumHit::part` (mapped as above) instead of `nearest_part`; `HitReact` gets `joint`.
- **Corpses** (S6, owner question 1 in the framework spec): `DeliriumCorpse` either becomes a sleeping limp
  `sf_ragdoll` at rest (drag and wake as before 3.2b) or keeps static proxies over the frozen pose.
- **Classic rules** (C5): OpenMW's melee hit is a reach and angle test with no per-bone shapes; the classic resolver
  keeps that (bound capsule only); hurt shapes serve the modern rules.

As built C1b, C2a, C2b (still the running code; CB-1/CB-2, CB-3a, CB-3b): [archive](archive/combat-adapter-history.md#as-built-pointers-moved-from-combat-adaptermd-2026-11-26).

## Reuse check (C0d mappings)

Bevy 0.20 and crates.io (2026-11-26): nothing maps game records onto combat numbers. OpenMW @ `4e718b5` (GPL-3.0, cite
file + commit where ported): weapon groups, armour rating and weight class, normal-weapon resistance, on-strike
inflict, projectile and bolt speeds, arrow recovery, siding, sneak critical, difficulty. Veloren (C-R1): poise scale and
stun lengths as `Guess` only. Reused: effects' modern resist, interaction's `DestructionRules`.
Ours: the poise, absorb and skill formulas (modern rules have no Morrowind source).
Attack clips (2026-11-26): Bevy 0.20's `ActiveAnimation` has one `speed` per clip (`bevy_animation-0.20.0/src/lib.rs:526,
677-682`), no warp inside a clip; our
montage slots take the playhead from the action clock, so the warp is ours, in `sf_action`/`sf_combat`. crates.io: no
crate maps text keys to gameplay windows. OpenMW `character.cpp` @ `4e718b5`: the key semantics (classic seam).
ReAnimation (github.com/fallchildren/OpenMWReAnimation): alternates keep the vanilla keys driving timing, as here.

## Industry comparison (C0d mappings)

| Mapping | Industry | Ours |
|---|---|---|
| Weapon classes | ER: class = moveset (dagger, straight sword, greatsword, spear, fist), numbers per weapon [M] | match: sheet per Morrowind group, numbers per record |
| Damage types | ER: physical standard/strike/slash/pierce + magic, fire, lightning, holy [M]; Skyrim: one physical + elemental resists [M] | Skyrim-like: Morrowind data has no physical subtypes |
| Armour curve | Morrowind `d/(d+AR)`; Skyrim about 0.12 % per point, cap 80 % [M]; ER flat defence + % negation [M] | saturating share, cap 80 %, same for every hit |
| Poise | ER: from armour weight, hyper-armour on swings [V]; Veloren: body base + armour | match: weight class × slot shares + attributes |
| Creature stance | ER: authored per enemy, stance break on bosses [M] | size rule + per-species and per-record overrides |
| Sneak critical | Morrowind ×4 player melee on unaware victims; Skyrim ×3 one-handed, ×2 bows, perks up to ×15 [M] | Morrowind's GMST through `Contact.critical` |
| Allies | OpenMW siding set; Skyrim followers spared by melee [M] | match (combat-delivery.md "Friendly fire") |
| Attack phases | ER: startup, active 2-4 frames, recovery with attack, dodge and move cancels (game-file frame data [V]) | match: anticipation, charge, strike, one follow; cancels after `Hit` |
| Per-phase rate | UE montage Time Stretch Curve [M]; Morrowind: one speed per weapon (`character.cpp` 1322) | piecewise `TimeWarp` per move |
| Anticipation, overshoot | key-pose exaggeration (Overgrowth, GDC 2014 [M]) | additive of the clip's own key poses |
| Hit reactions | ER: tiered stagger, directional by side [M]; OpenMW: random Hit1-5 | tier = clip + rate + free point; side via `HitReact` |

## Build plan (C0d data)

Rows and verification: combat roadmap §4. **C3d** `weapon_classes.ron`, directions,
`greatsword_2h` / `spear_2h` / `fist` sheets once AN33 cooks windows; **C3a** values from "Poise"; **C13** `ModernRules`
(packet, absorb, resistances, on-strike); **C17** record-driven NPC and creature fighters; **C6f** creature files and the
generated default; **C8b** ranged sheets, ammo, speeds, recovery; **C8d** cast slots, the big rule, `launch_spec`;
**C14** prop keys; **C16** crime hooks; **C22** non-lethal tags, essential, downed clips; **C23** downed offences. Tests: exact on synthetic records, vanilla rows `#[ignore = "needs Morrowind.esm"]`.

## Open questions

1. Fists: Morrowind hand-to-hand hurts fatigue unless the victim is down (knockout brawls). Modern: health (ER), or
   fatigue first (non-lethal brawls)? Changes what the player can do: owner. Until answered: health.
   Answered 2026-11-26 (owner, decision 20): modern = health with a non-lethal tag (Q10 answered 2026-11-26); fatigue-first = Classic
   only ("Health, stamina, fatigue").
2. If AN32 shows daggers and longswords need distinct timings within `WeaponOneHand`, split the sheet by type (data).
   Decided 2026-11-26 (AN32a): no split. Both share the one `WeaponOneHand` group and its keys; a split could only be
   sheet rate data (ER daggers hit 0.1 s before straight swords), not different clips.
3. Veloren's ramping regen and re-stun buffer need framework fields: only if CB-10 shows stun-locks (C3a).
