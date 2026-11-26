# Combat adapter: what Bevyrrowind adds to the framework's combat

Status: spec (2026-10-27); built: C1b, C1c, C4, C2a (sparring dummy, below). Framework design: `../../../superfargoth/docs/specs/combat.md` (feel layer, sheets,
`CombatRules` seam, build plan C1-C7). Classic requirements: `../research/combat-classic-openmw.md`. ROADMAP 3.5.
C0d (2026-11-26): Morrowind data onto the C0a-C0c fields (classes, poise, damage packet, ranged, skill, creatures, spells,
allies, sneak, props); numbers `Guess` until their slices; sheet windows wait for the anim lane (AN32 measures, AN33 cooks).

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
- **Windows**: AN33 cooks them from text keys ("Morrowind clips as the fallback layer"), AN32 measures; no window
  numbers here. Missing moves (backstep, 2H guard, parry, riposte, kick) are AN37's key-pose clips.

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

- **Health**: the character's stat sheet (stats.md S5, `mw_player_bevy/src/vitals.rs`): `Combatant` health = sheet
  `health` 1:1 each tick, combat's change goes back as `PoolAdjust` (cause `Combat`); NPCs and creatures from their records
  (NPDT health), the player from the `player` record (chargen skipped), the training dummy set to 100.
- **Stamina (modern)** = sheet fatigue x 0.5 (`mw_combat::STAMINA_PER_FATIGUE`; Morrowind.esm's player: 200 → 100),
  differences back x 2. The HUD's fatigue bar shows the sheet's fatigue (x 2 the stamina); "Fatigue" in the Morrowind
  theme. Stat bands from attributes later (C5).
- **Classic**: fatigue is the Morrowind pool, never gates a start; the fatigue term scales hit chance, block and damage
  inside `mw_rules`.
- **Fists (modern, decision 20)**: hand-to-hand damages health like any weapon (CP2077 close combat), tagged `NonLethal`
  (Q10, pending owner), so a brawl ends in a knockout ("Non-lethal and mercy"). **Classic** (C5, named only, not designed
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

Framework side: combat.md "Non-lethal and mercy" (C20, C21); Q11-Q15 answered as recommended (owner 2026-11-26); Q10 open, recommended answer designed in.
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

## As built: C1b, C2a, C2b

Moved verbatim to [archive/combat-adapter-history.md](archive/combat-adapter-history.md) (C0d, 2026-11-26); still the running code:
- [C1b: game items](archive/combat-adapter-history.md#c1b-game-items): `sword_1h` sheet and UAL slots, the player's `Combatant`, F stance and draw clips, bindings, training dummy, level `arena`, CB-1/CB-2.
- [Sparring dummy (C2a)](archive/combat-adapter-history.md#sparring-dummy-c2a): `DummyKind::Sparring`, `sparring.ron` schedule, non-lethal refill, level `sparring`, CB-3a.
- [Guard (C2b)](archive/combat-adapter-history.md#guard-c2b): sheet `guard`, `GUARD_ABSORB` 0.65, `GUARD_STABILITY` 1, guard clips, classic `blockMeleeAttack`, CB-3b.

## Reuse check (C0d mappings)

Bevy 0.20 and crates.io (2026-11-26): nothing maps game records onto combat numbers. OpenMW @ `4e718b5` (GPL-3.0, cite
file + commit where ported): weapon groups, armour rating and weight class, normal-weapon resistance, on-strike
inflict, projectile and bolt speeds, arrow recovery, siding, sneak critical, difficulty. Veloren (C-R1): poise scale and
stun lengths as `Guess` only. Reused: effects' modern resist, interaction's `DestructionRules`.
Ours: the poise, absorb and skill formulas (modern rules have no Morrowind source).

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

## Build plan (C0d data)

Rows and verification: combat roadmap §4. **C3d** `weapon_classes.ron`, directions,
`greatsword_2h` / `spear_2h` / `fist` sheets once AN33 cooks windows; **C3a** values from "Poise"; **C13** `ModernRules`
(packet, absorb, resistances, on-strike); **C17** record-driven NPC and creature fighters; **C6f** creature files and the
generated default; **C8b** ranged sheets, ammo, speeds, recovery; **C8d** cast slots, the big rule, `launch_spec`;
**C14** prop keys; **C16** crime hooks; **C22** non-lethal tags, essential, downed clips; **C23** downed offences. Tests: exact on synthetic records, vanilla rows `#[ignore = "needs Morrowind.esm"]`.

## Open questions

1. Fists: Morrowind hand-to-hand hurts fatigue unless the victim is down (knockout brawls). Modern: health (ER), or
   fatigue first (non-lethal brawls)? Changes what the player can do: owner. Until answered: health.
   Answered 2026-11-26 (owner, decision 20): modern = health with a non-lethal tag (Q10 pending); fatigue-first = Classic
   only ("Health, stamina, fatigue").
2. If AN32 shows daggers and longswords need distinct timings within `WeaponOneHand`, split the sheet by type (data).
3. Veloren's ramping regen and re-stun buffer need framework fields: only if CB-10 shows stun-locks (C3a).
