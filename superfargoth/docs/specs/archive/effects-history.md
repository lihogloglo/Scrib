# effects.md history (moved out of the spec to keep it under budget)

## As built (E1)

Not yet compiled. `apply(defs, stats, land, set, ops, ctx, out)`, `tick(defs, stats, set, ctx,
out)`: no sheet or book yet. Choices: `Inflict` carries `class` (no source table before E3);
stacking per effect (same key and arg, the class's `StackKey`; `def.stack: None` = class default; `Stack` at its
limit drops the new one; `EndReason::Replaced`); durations `max(1, round(s/dt))` ticks; instants deliver rates once;
temporary pool fortify = modifier + `above_max` adjust, current reverted first (exact on `KeepCurrent`); absorb
inverts damage/restore, hurt/heal, drain/fortify; splitmix64 `roll`;
`no_magnitude` resisted when `rolls[1] >= mult`; unknown effect or stat, no caster: `Rejected`. Deferred:
`school`, presentation, range/crafting flags, `period`; `ItemKey(u64)` placeholder.

## Open questions (owner, all answered 2026-11-08)

1. Modern casting never fizzles, the chance roll classic. Owner 2026-11-08: default taken.
2. Modern resist deterministic, the roll classic. Owner 2026-11-08: default taken.
3. Magnitude rolled once per effect. Owner 2026-11-08: default taken.
4. NPC effects kept and resumed across unload. Owner 2026-11-08: default taken.
5. Crate split `sf_effects` + `sf_effects_bevy`. Owner 2026-11-08: default taken.

## As built (E2)

`rules.rs`, `standard.rs`, `tests_rules.rs`. `EffectRules` (`standard()`, `from_model`); `CastQuery`, `ProtectQuery`,
`ItemQuery` carry `SheetRead`s, content-resolved stat indices (school skill) and host state (`ready_at`, `tick`,
cost); `ResistQuery` gained `target` (E1 step passes `None`); `LandModel::protect` defaults to `Applied`. Pure rules
with serde In/Out: `cast_rule`, `magnitude_rule`, `protect_rule`, `resist_rule`, `item_charge_rule` (`{ cost, max }`).
Defaults (formulas in `standard.rs` docs): entry cost `base · max(1, avg mag) · max(1, s) · (1 + area/5) · 0.1`,
0.5 % off per skill point (cap 100), powers free, refusals recharging > silenced > no mana, absorb before reflect
(gain = cost), item max = cost · 10. Deferred: the step calling `protect` with sheets (E5).

## As built (E3)

`book.rs`, `tests_book.rs` (not yet compiled):
`SourceDef`/`SourceTable` (+ `skill`, `cooldown` ticks, `recharge: Option<GameMinutes>`; `Power` stays a unit class);
`SpellBook` (private fields, accessors). `apply_book(env: &EffectEnv, set, book, ops, ctx, out)`: `EffectEnv` holds
defs, stats, sources, land and cast models, the target's `SheetRead` and `time: GameMinutes` (world minutes; no
`sf_rpg` dependency), so E1's `apply` and `EffectCtx` are unchanged (`apply` = no book, empty table). Cast: refusals
`UnknownSource`, `NotKnown`, `NotCastable`, `Recharging` (power, by the step) before the plan; cost = `PoolAdjust` on
`Role::Mana`; `roll(seed, 0, 2) < chance`; cooldown and power use on success only. Touch/target entries leave as
`EffectRequest::Touch`/`Launch(Delivery)`; `Request` effects as `EffectRequest::Effect { Start | End }` (no `Tick`
until `period`), answered by `EffectOp::Invalidate`. `CureFilter::Effect` (the spec's `Archetype`), `SourceTag`
(book sources leave the book), `Dispel` (`Cast`/`Power` only, chance `magnitude/100`; the `Cure` op uses 100).

## As built (E4a)

Crate `sf_effects_bevy` (deps `sf_effects`, `sf_stats_bevy`): `Effects { owner,
active, book }` component, `EffectIndex` (observers), resources `EffectTableRes` (content), `SourceTableRes` (empty),
`EffectRulesRes` (standard), `EffectIds` (the host counter), `EffectClock`, `PendingEffectOut`; messages `EffectCommand`
in, `EffectMessage` and `EffectRequestMessage` out; `EffectSystems::{Commands, Tick, Requests, Out}` chained before
`StatSystems::Apply`. Stat ops leave as `StatCommand`s, one per run of ops with one owner and cause
(`stat_commands`), so they land the same tick. Ticks only timed sets, in owner order. Game glue
`mw_rpg_bevy::effects` (`add` from `stats::add`; `Effects` beside every `Stats`; `feed_clock`). Tests: 3 headless +
1 game (synthetic records); measurement 500 characters / 2 000 effects / 50 rate: effect tick 0.089 ms, whole fixed
tick with stats 0.55 ms (dev profile, workspace crates at opt-level 1; the 0.03 ms budget is for release on the
target laptop). Known issue (E1, logged): a temporary `Pool Fortify` on a `KeepRatio` pool (Morrowind magicka,
fatigue) counts twice: the modifier rebases the current by ratio, then `PoolAdjust` adds the magnitude.

## As built (E4b)

`sf_effects::save` (`EffectsSave { tick, active, book }` with `capture`, `is_empty`, `restore(defs, sources, now,
ids)`: `started` rebased by age, cooldowns by ticks left (an expired one dropped), unknown effects and book sources
dropped into `EffectsLoadReport`, the selection cleared with its source, `ids.next` moved past restored instances,
`timed` recomputed; `reregister` keeps the `AddModifier` ops of `step::start_ops`, an absorb's caster side under the
caster). `sf_effects_bevy`: `Effects::to_save(tick)` (`None` when empty), `Effects::restore(owner, save, LoadTables,
now, ids)` (warns drift, returns the modifiers). `sf_save_bevy`: `CharacterSave.effects`, `CHARACTER_VERSION` 4,
`legacy::v3` + typed step, `sections::effects` / `EffectsWorldSave`, goldens `sf.character.v4.json`,
`sf.effects.v1.json`, `character-v4.sfsave` (the current golden; `character-v3.sfsave` loads through the step).
Game: `mw_rpg_bevy::effects` (`EFFECTS_BLOB`, `EffectLoad` system param, `restore_in_world`, `saved_effects`,
`write_effects`, `player_effects_save`); `stats::load_sheet(.., modifiers)` applies the owner's before settle,
`restore_player_sheet(.., effects)`, attach and unload/persist read and write the blob; `mw_saves_bevy` declares the
blob, registers `sf/effects` over `EffectIds`, fills `CharacterSave.effects`, clears `PendingEffectOut` on load.
Tests: sf_effects 2 (round trip with fortify + drain + pool fortify: sheet bytes, values and remaining durations
equal, same expiry; drift and rebase), sf_save_bevy 3 (codec round trip, v3 golden through the step, pre-effects
save loads with none), mw_saves_bevy 1 (game path, synthetic records: player and creature).
An NPC's empty effects are written over an older blob (the overlay has no blob removal), so ended effects do not
come back.
