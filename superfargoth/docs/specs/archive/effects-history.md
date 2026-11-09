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
