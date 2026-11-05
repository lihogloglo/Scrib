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

- **Health**: the character's stat sheet (stats.md S5, `mw_player_bevy/src/vitals.rs`): `Combatant` health = sheet
  `health` 1:1 each tick, combat's change goes back as `PoolAdjust` (cause `Combat`); NPCs and creatures from their records
  (NPDT health), the player from the `player` record (chargen skipped), the training dummy set to 100.
- **Stamina (modern)** = sheet fatigue x 0.5 (`mw_combat::STAMINA_PER_FATIGUE`; Morrowind.esm's player: 200 → 100),
  differences back x 2. The HUD's fatigue bar shows the sheet's fatigue (x 2 the stamina); "Fatigue" in the Morrowind
  theme. Stat bands from attributes later (C5).
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

## Ragdoll routing (Hit, Knockdown, Death)

Design: the framework's `PhysicsTakeover { impulse, point }` goes to the Delirium takeover when the skeleton is human,
else the walkers' powered-ragdoll knockdown (`mw_walkers_bevy/src/physics.rs`, get-up from `Knockout@tail`'s first
frame). Death: the same hook (kind `Death`): Delirium's terminal mode for humans, else the graph's `dead` state, then
limp ragdoll (`DEATH_HANDOFF`). Hand-back ends the forced move in `sf_action`. Cosmetic `HitReact` springs
stay on every landing (Additive tier).

As built (2026-11-05, hits on walkers; `sf_action`'s `PhysicsTakeover` for action characters is not built). Owner
verdict on the first cut (every impact >= 15 N·s forced `catchFall`: "limp instantly"): hits must push along their
direction. Canonical technique: Euphoria / RDR2 hit reactions (the impulse on the struck part, the balancer recovers
with steps, the fall is the balance failing; a scripted fall only when the game decides). The walkers' `knockdown()`
asks the adapter hook `RagdollTakeover` (resource, `mw_walkers_bevy::physics`) first for every impact >=
`HIT_REACT_IMPULSE` (1 N·s) with `PhysicsTakeover { root, skeleton, mover, held, point, impulse, kind: Hit }`, `Held`
bodies included (not `KnockedDown`: the powered ragdoll takes its impacts); a claim inserts its own request and owns the
character (`Held`, get-up, the root's move) until its hand-back. A decline (or no hook, a `--no-default-features` app):
`HitReact` under `KNOCKDOWN_IMPULSE`, the powered ragdoll above; a held body gets nothing. The host (`mw_exterior_bevy`,
feature `delirium`) maps `Hit` to `mw_delirium_bevy::hit_reaction` with `HitRules` (human skeleton on a mover only):
push = impact x `gain` (1: our props have no restitution, the body takes their incoming momentum; a bouncing source
would bring `1 + e`); a takeover from `takeover` = 10 N·s (where the push first reads past the takeover's own settle
sway); a knockdown after the push from `certain_fall` = 100 N·s (Delirium's balancer stays up to 120 N·s on its own,
1.5 m/s on 80 kg, beyond a person's step recovery); `held` -> `live_only` (a further push on Delirium's own takeover,
dropped otherwise: frozen corpses, limp ragdolls). The takeover (`mw_delirium::Takeover::hit(part, at, impulse,
Outcome)`, the entry point combat's hits use later with weapon mass x speed) pushes the part nearest the point with
`bodyBalance` on; `Outcome::KnockDown` (also kind `Knockdown`, game-decided: `knockdown_reaction`) keeps the balancer
`Rules::knock_down_after` = 0.3 s then `catchFall` (0.5 s left a still body standing). Measured (`mw_delirium`
`hit_reactions_vs_impulse`, chest, pelvis travel along the hit at 1 s): 36 N·s (a `V` ball head-on) 0.31-0.45 m, 45 N·s
0.44-0.62, 80 N·s 0.59-0.98, never down under 120; knockdowns fall at 0.5-0.85 s, 36 N·s 0.28-0.91 m along. Walkers
resume their AI when the hand-back removes `Held`/`Dormant`. Tests: `mw_walkers_bevy` `physics::tests`,
`mw_delirium_bevy` `physics_hits_become_pushes`, `mw_delirium` `hits_push_along_their_direction`,
`a_held_body_takes_a_second_hit`, `knockdown_impulses_hand_back` (real install). Visual check PH-15.

As built (2026-11-05, deaths of walkers): `physics::kill()` asks the same hook with `kind: Death` (the blow as
`point`/`impulse`, zero for none) unless the character is a powered-ragdoll knockdown (`KnockedDown`: it dies in its
ragdoll); a claim inserts `Dead` (requires `Held`) and no `Ragdoll`. A character a layer already holds (`Held` without
`KnockedDown`) is killable: the hook turns it terminal; a decline leaves it alive. The host maps `Death` to
`mw_delirium_bevy::death_reaction` (`DeliriumReaction { kill: true }`): with a blow, `Takeover::kill_by_hit` pushes and
keeps a weakened balancer for the dying window (`Rules::dying` = 0.6 s at `bodyStiffness` 4, Euphoria's `shot` +
`shotRelax`; falls at ~1.1 s, 36 N·s carries the pelvis 0.37 m along the hit in 1 s); without (console `kill`)
`Takeover::kill` collapses at once; either way `catchFall`, then near-limp `bodyRelax` once down (`Rules::dead_relax`),
no get-up, never a hand-back; at rest
(`Rules::corpse_secs`) it is `settled`, the host drops the takeover and the body keeps its last `PhysicsPose` (frozen),
root `Held` + `DeliriumCorpse` (later requests refused). A kill on a live takeover turns it terminal (a down body relaxes
at once). A claimed death Delirium then can't take (no rig) falls back to `Ragdoll { Death }`. The dev `I` key is gone:
console `kill` (`physics::kill_in_view`, walkers only). Gaps: corpse drag (Delirium parts aren't Rapier bodies) and
save/streaming persistence of the corpse pose. Tests: `physics::tests` (death routing), `mw_delirium_bevy`
`deaths_of_human_walkers_become_terminal_takeovers`, `mw_delirium` `killed_takeover_falls_and_settles`. Visual check PH-16.

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

## C1b: game items

- **Sheet** `mw_combat/data/movesets/sword_1h.ron` with combat.md's C1b table.
- **Slots** in `sources.ron` (graph `humanoid`, root `Progress`): `attack.sword_1h.light{1,2,3}` (UAL2 `Sword_Regular_A/B/C`,
  then UAL1 `Sword_Attack`, fallback `WeaponOneHand@slash`); `attack.sword_1h.light{1,2}_rec` (UAL2 `Sword_Regular_A_Rec`,
  `_B_Rec`, fallback `Drop`); `dodge.roll.fwd|left|right` (UAL1 `Roll_RM`, `Dodge_Left_RM`, `Dodge_Right_RM`, fallback
  `Drop`); `react.small.front|left|right|back` (UAL1 `Hit_Chest`, `Hit_Shoulder_L`, `Hit_Shoulder_R`, `Hit_Stomach`,
  fallback `Hit1`-`Hit4`); `react.medium.front` (UAL2 `Hit_Knockback_RM`, root x0.25, fallback `Hit5`); `react.large.front`
  (the same clip, root x0.6). No LaFAN1 candidate in C1. Candidate `events` (per-clip windows) are a new manifest field.
- **Player**: `Combatant` on the player (health from the record, stamina as above, `min_tier: Small`, sheet by weapon);
  the ready-weapon key (F, Morrowind's default) toggles the stance; sheathed, attack presses do nothing (owner 2026-11-02:
  first the stance, then the fight; the implicit draw is gone). As built (CB-3, 2026-11-05): the avatar's
  `CharacterReadiness` follows `Stance`, so its held weapon mounts on `Weapon Bone` while drawn and hides while sheathed
  (the original shows no sheaths) and `feed_stance` sets the graph's one-handed set. The player never auto-equips
  (`mw_actors::held::equipped`: first wieldable weapon, first shield; `EquippedHeld` on the avatar); levels name its
  items in `player_held` (`arena`, `anim-lab`: `iron longsword`). Real cells: no weapon until an equip screen writes
  `EquippedHeld`. As built (draw clips, 2026-11-05): F plays `base_anim`'s `WeaponOneHand@equip` / `@unequip`
  (`mw_combat::SWORD_1H_DRAW`/`_SHEATHE`, already graph slot clips) in the humanoid graph's masked `upper` slot over
  locomotion, playhead = a fixed-tick `sf_character::ReadyChange` (gameplay clock, net-ready) timed from the baked clip's
  `equip.attach` / `unequip.detach` events (Morrowind's `Equip Attach` / `Unequip Detach` keys; UE attach notify): the
  readiness (prop mount) flips at the key; attacks stay withheld until the draw ends and from a sheathe's first tick;
  F during a change is ignored; no baked clip = instant (pop). Not yet: other weapon groups (C3 maps records to
  sheets), the grip IK still treats the right hand as free during a change (`grip.rs` checks `Stance::Drawn`).
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
