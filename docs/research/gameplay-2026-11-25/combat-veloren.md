# C-R1: what Veloren does for NPC combat, poise and agent tactics

Date 2026-11-26. Source: gitlab.com/veloren/veloren, `--depth 1` clone, commit `6c99b292e0c3cbb5ab9c76e2c2f83a3ce73b295c`.
Paths below are at that commit. Compared against the planned C0b design: tokens, move-picker table, reaction skill, telegraph lint, poise.

## 0. Verdict

- Licence is GPL-3.0-or-later: code may be ported into Scrib with attribution. But Veloren has nothing we plan to build in reusable form.
- Poise: take the **ideas** (state ladder with fixed stun lengths, damage multiplier while stunned, 1 s re-stun buffer, block costs poise,
  armour-based poise reduction). Port nothing: 317 lines tied to specs, `CharacterState` and armour items.
- NPC tactics: **hand-coded Rust per species and weapon** (about 80 `handle_*_attack` fns, 9000 lines). Our RON table is the better shape; do not port.
- No attack tokens, no reaction delay, no telegraph/fairness logic, no aim error in Veloren. Those C0b parts are ours.
- Damage pipeline: take the idea of damage kinds that act on a second resource (Crushing -> poise, Slashing -> energy, Piercing -> armour penetration).

## 1. Licence

Root `LICENSE` is the GNU GPL **version 3** text. Workspace `Cargo.toml:35`: `license = "GPL-3.0-or-later"`; `server/agent` and `common`
inherit it (`license.workspace = true`). README ("How is this game licensed") says GPL v3.0. The two source files checked (`combat.rs`,
`attack.rs`) carry no per-file header. So the grant is "3 or later", the same as Scrib: porting is allowed and no `-only` taint arises
(unlike Delirium). Obligations: cite file + commit in module docs and add a `superfargoth/THIRD_PARTY_NOTICES.md` entry. Take `.rs` only,
never assets (README mentions CC-BY-SA-4.0 for some docs; asset terms not checked). Veloren is a voxel, specs-ECS, networked MMO: any
port is a rewrite to Bevy anyway, so licence is not the limiting factor.

## 2. Findings by topic

### (a) Poise (`common/src/comp/poise.rs`, 317 lines)
- Pool: `Poise { current, base_max, maximum, regen_rate, last_stun_time, previous_state }`, u32 x256 fixed point; base from body (`Poise::new` -> `body.base_poise()`).
- One pool counting **down**; state is a pure function of `current` via `POISE_THRESHOLDS = [50, 30, 15, 5]` (`poise_state()`):
  >50 Normal, >30 Interrupted, >15 Stunned, >5 Dazed, else KnockedDown. One big hit can skip tiers.
- Stagger states (`PoiseState::poise_effect`): buildup+recover ms of the `Stunned` state: Interrupted 200+200, Stunned 350+350, Dazed 750+750, KnockedDown 1500+1500 (+ impulse 10). Move-speed factor 0.8/0.5/0.2/0, turn-rate 0.8/0.5/0/0.
- `damage_multiplier()` 0.1/0.25/0.5/1.0: while the target is stunned, poise damage is also applied as **health damage** at that factor (`combat.rs:514-530`). This is the crit-eligible window.
- Anti-chain: `POISE_BUFFER_TIME = 1.0` s: `change()` ignores poise changes for 1 s after `reset()`; `reset()` refills the pool when the stun starts.
- Regen: `regen(accel, dt, now)`, `regen_rate` ramps by `accel*dt` up to a cap of 10/s: accelerating, not a flat delay. The caller (which resets the ramp on hit) is outside the checkout (section 5).
- Reduction: `compute_poise_damage_reduction` = armour `dr/(60+|dr|)` combined multiplicatively with the `POISE_RESISTANT` ability capability (x0.5) and a stats modifier; armour may give full immunity (`None`).
- Block: `combat.rs:186-247`: a blocked hit costs `min(damage/block_strength,1) * MAX_BLOCK_POISE_COST (25)` (`:79`); parry has its own `poise_multiplier` (`:217`).

### (b) Damage pipeline (`common/src/combat.rs`)
- `DamageKind` (`:1722`): Piercing (removes 0.75 x damage of protection, `:1735`), Slashing (drains 0.5 of damage as energy), Crushing (poise damage = armour-prevented damage x `CRUSHING_POISE_FRACTION` 1.0, `:488-500`), Energy (catch-all).
- Armour DR `p/(60+|p|)` (50 % at 60), combined with stat DR; block subtracts a flat decrement (`:1748-1808`).
- Crit = "precision" (`precision_mult`, `:122,317-380`), hit-location based, not a random roll.
- Knockback (`:1866-1890`): `50 x strength x (0.5 if KNOCKBACK_RESISTANT) x attacker mult`, direction Away/Towards/Up/TowardsUp slerped toward +Z.
- Hit resolution only emits events (`HealthChangeEvent`, `PoiseChangeEvent`, `KnockbackEvent`), applied elsewhere: the seam we want too.

### (c) Agent tactics (`server/agent/src/`)
- `action_nodes.rs:1200 attack()` picks a `Tactic` enum from the weapon's `ability_spec` **string** (`:1283-1315`, e.g. "Quad Med Jump", "Iron Golem") or weapon kind, then calls one `handle_<species>_attack` in `attack.rs` (`handle_simple_melee :54`, `handle_sword_attack :952`, `handle_minotaur_attack :4781`, `handle_hydra :8255` ...).
- So tactics are **hand-written Rust per species/weapon**, with `agent.combat_state` arrays `timers[]`, `conditions[]`, `int_counters[]` indexed by handler-local enums (`attack.rs:752,1199`), hardcoded floats and `rng.random_bool(0.5)` (about 315 rng uses in the file).
- Move picking for skilled weapons (`:952-1090` sword, `:795-935` hammer): pick a random tactic tier from owned skills once, then whenever idle pick a random ability slot (else 50/50 primary/secondary) and gate it with `AbilityData::could_use` (`data.rs:876`): distance band = ability range + forced-movement travel + body radius, angle, energy, combo, line of sight, and target state (`attack_kind_check` on the target's current attack kind `:921,1065`; sword aux gates "target stunned / blocking / behind / lacks buff", `attack.rs:802-840`). Ranges and angles come from the ability data, not the handler. The choice among legal moves is uniform random: no weights, no last-move memory.
- Cooldowns: no per-move cooldown table; limits are energy cost, combo counters and handler timers (`AttackTimer`, `attack.rs:150-153`).
- Circling: ad hoc per handler: backstab strafe `attack.rs:402-445` (STRAFE_DIST 4.5, x0.75 speed, spiral-in 0.8, goes for the back), ranged circle `:2382,2763`, `CircleCharge{radius,circle_time}` for charging quadrupeds.
- Attacker-count limiting: **none**. No token, max-attackers or surround code in the agent crate; only `SEPARATION_DIST 10 / SEPARATION_BIAS 0.8` (`consts.rs`, `action_nodes.rs:113`) spreads allies. Groups all attack.
- Other: `flee_health` per body (`comp/agent.rs:254-290`), `FLEE_DURATION 3`, `RETARGETING_THRESHOLD_SECONDS 10`, `DAMAGE_MEMORY_DURATION 0.25`, `ActionMode` Reckless/Guarded/Fleeing (`data.rs:170`).

### (d) Reaction and skill
- Reaction delay: none; agents read the current tick's target state. The only delay is the `Awareness` gauge for noticing the player (`comp/agent.rs:651-690`).
- Block/dodge odds: no model. Humanoid simple melee rolls 2 % per tick within 16 m (`attack.rs:75-80`); skilled handlers block/roll through `blocked_attacks` filters and `RollingBreakThrough` flags (`:752,2299`). Difficulty = which skill tier the NPC owns.
- Aim: ballistic solve only (`common/src/comp/projectile.rs:771 aim_projectile`), **no error term**, no dependence on target speed.

### (e) Telegraphs and fairness
- Nothing in agent code. Readability comes from hand-set ability buildup durations and animations; no lint or floor. Bosses rely on poise/stun for counterplay.

## 3. Bevy/crate check (crates.io API, 2026-11-26)
- `big-brain` 0.22.0, last release 2024-11-30 (stale). `bevy_observed_utility` 0.2.0, 2025-03-05 (stale). `bevy_behave` 0.5.0, 2026-02-06 (behaviour trees). `seldom_state` 0.16.0, 2026-04-02 (state machines). `moonshine-behavior` 0.6.0, 2026-10-11 (behaviour stack). None is a combat layer.
- Searches "attack token" and "bevy combat": only `bevy_combat` 0.1.0 (2021), `bevy_battle` 0.1.0 (2026-05), and tiny hobby crates. No maintained crate does attack tokens, move pickers or poise. Bevy 0.20 has no combat or AI-combat layer. These stay ours (Bevy-free cores).

## 4. What C0b should take
1. **Poise ladder as starting tunables** (`Guess`): 4 stagger tiers, total stun 0.4/0.7/1.5/3.0 s with move-speed factors, a 1 s post-stun immunity buffer, pool refill on stagger, regen that ramps after a hit instead of a flat rate. Armour windows are ours (Veloren has none; its per-ability `POISE_RESISTANT` x0.5 is a cheap per-move flag to copy).
2. **Stunned-target damage conversion**: poise damage on a staggered target also hits health at x0.1/0.25/0.5/1.0: our crit-eligible window, tiered by stagger depth.
3. **Block costs poise** by damage/guard strength, capped; parry changes the multiplier: fits the shipped guard-break (C2b).
4. **Move-filter inputs** from `could_use`: reach = range + root-motion/lunge travel + body radius, angle gate, target-state gates (stunned, blocking, behind). Our baked clip windows should supply the numbers; add body radius and root-motion reach to band maths.
5. **Keep our design for the rest**: a RON table with weighted pick and last-move memory beats 80 hand-coded handlers on uniform random. Tokens, reaction delay, aim error and telegraph lint have no precedent in Veloren; the Doom 2016 / Elden Ring sources in the spec remain the references. For creature anatomy moves only the shape (a tactic = gated abilities + per-species timers) is shared.

## 5. Not verified
- Poise regen caller and `accel` value (`server/src/sys/poise.rs`) and how a poise state becomes a character state: not in the sparse checkout.
- `body.base_poise()` per species, ability RON assets (`assets/common/abilities/`), any asset-level attacker limiting (only the agent crate was grepped).
- Per-file licence headers beyond two files; asset licences.
- Crate dates are crates.io only; Bevy-version compatibility and GitHub activity of `bevy_behave`/`seldom_state` not checked.
