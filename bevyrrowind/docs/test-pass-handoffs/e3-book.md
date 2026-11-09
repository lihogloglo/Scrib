# e3-book (sub-agent, 2026-11-08)

Scope: sf_effects slice E3 (spell book + casting, Request archetype, Cure) per
superfargoth/docs/specs/effects.md "Spell book", "Casting", "Operations", Build plan E3.
Owns: superfargoth/crates/sf_effects/ only (+ effects.md, archive/effects-history.md).
Status: DONE (written 2026-11-08 without compiling: UNTESTED, owner test pass 2026-11-10). Nothing committed.

## Files
- superfargoth/crates/sf_effects/src/book.rs (new): `GameMinutes` (world minutes, `DAY`, `days`), `SourceFlags`,
  `SourceDef` (+ `skill: Option<StatKey>`, `cooldown: u32` ticks, `recharge: Option<GameMinutes>`; `new`,
  `source_ref`, `is_book_source`, `is_castable`, `lives_in_book`), `SourceTable` (+ `SourceTableError::Duplicate`),
  `BookRejection`, `SpellBook` (pub(crate) fields; `known`, `knows`, `selected`, `power_used`, `ready_at`, `recharging`).
- src/def.rs: `Tag` (case-folded string, like `EffectKey`), `CureFilter { Effect(EffectKey), SourceTag(Tag),
  Dispel { harmful_only, chance_from_magnitude } }`, `Archetype::Cure { filter }`, `Archetype::Request { kind: Tag }`,
  `Archetype::stat_target() -> Option<&StatTarget>`; `stat()` kept (returns `&StatTarget::Arg` via a static for
  Cure/Request).
- src/op.rs: `EffectOp::{Cast, Learn, Forget, Select, Cure, Invalidate}`; `RequestPhase`, `Delivery` (+ `inflict()`),
  `EffectRequest::{Effect, Touch, Launch}`; `EffectEvent::{CastSucceeded, CastFailed, PowerUsed, BookChanged,
  BookRejected}`; `EffectOut.requests`.
- src/rules.rs: `CastFailReason::{NotKnown, UnknownSource, NotCastable}`.
- src/step.rs: `EffectEnv` (+ `new(defs, stats, sources, &EffectRules, sheet, time)`), `apply_book(env, set, book,
  ops, ctx, out)`; `apply` now delegates to it with an empty book/table (E1 behaviour unchanged); cast, learn,
  forget, select, cure, invalidate, delivery split; `Request` effects emit requests; `Cure` effects act on landing.
- src/tests_book.rs (new): 9 tests (book sync, select, cast success + delivery by range, refused vs fizzle,
  power recharge on game time, tick cooldown via `StandardEffectRules`, Request start/end/invalidate, cures by
  effect/tag/dispel, book round trip + determinism).
- src/lib.rs: `mod book`, `mod tests_book`, exports, crate doc line.
- superfargoth/docs/specs/effects.md: status line, Build plan rows marked written (E1 row shortened), short
  "As built" E3; E2 and full E3 as-built moved to archive/effects-history.md. Spec 25.5 KB (was 25.9).

## Signature changes
None to existing fn signatures (`apply`, `tick`, `EffectCtx` unchanged: mw_effects tests call them).
Additive but potentially breaking for outside code: new variants on `EffectOp`, `EffectEvent`, `Archetype`,
`CastFailReason` (exhaustive matches; none found in mw_effects); new field `EffectOut.requests` (struct literals;
none found). `Archetype::stat()` no longer reachable-panics for new variants (returns `Arg`).

## Design choices (not in spec)
- No `sf_rpg` dep: `GameMinutes(u64)` (host passes `GameTime::total_minutes()`); avoids a cycle if sf_rpg services
  later read `SpellBook`.
- `Power` stays a unit `SourceClass`; recharge is a `SourceDef` field. Game-time recharge is checked by the step
  (content semantics); tick cooldowns go to the cast model via `CastQuery.ready_at` (TestRules ignores them).
- Cast order: UnknownSource > NotKnown > NotCastable (class or no sheet) > Recharging (power) > plan. Cost spent on
  fizzle; cooldown/power use only on success. Chance roll `roll(seed, 0, 2)`; `always_succeeds` forces chance 1.
- Touch entries without `target` are dropped (no event). Self entries land with the cast's seed.
- Learn of Innate/Affliction: caster = owner, `ignore_resist`, seed `RollSeed(ids.next)`.
- Forget ends only Innate/Affliction instances of the record; landed spells run on. Cooldowns/recharges survive.
- Cures: `Effect` hits temporary sources only; `SourceTag` ends whole instances and drops cured book sources;
  `Dispel` only `Cast`/`Power` instances, per-source roll `roll(seed ^ instance, 0, 3) < magnitude/100`; the
  `Cure` op cures at magnitude 100.
- `Invalidate` on a Request effect: `Ended { Invalid }`, no End request; on another effect: normal end ops.
  Sources with nothing live are pruned (so "still queryable" holds only while siblings live).
- No `Tick` request phase (EffectDef has no `period` yet).

## Unverified guesses (no compiler)
- `static NO_STAT: StatTarget = StatTarget::Arg;` needs `StatTarget: Sync` (StatKey's inner type).
- `&*rules.land` → `&'a dyn LandModel` coercion in `EffectEnv::new`; `cast: &NoCast` promotion in `apply`.
- `&def.effects` / `&own` (SmallVec) coercing to `&[EffectEntry]` in `Landing` literals.
- `split_delivery`'s `delivery` closure param type inferred from the `Delivery.entries` field.
- clippy `too_many_arguments`: `learn`/`cast` have 7 params incl. self (limit 7). `inflict` got
  `#[allow(clippy::too_many_lines)]`.
- Tests: Permanent-layer modifiers show in `modified` (E1 test relies on the same); pools full after
  `Settle::NewCharacter`; standard cast cost with fixed cost 10 and no skill is exactly 10.
- rustfmt: several lines > 120 cols; run `cargo fmt`.

## Owner test commands
```
cargo test -p sf_effects
cargo test -p mw_effects          # uses apply/EffectCtx/Archetype::stat
cargo clippy -p sf_effects --all-targets -- -D warnings
cargo fmt -p sf_effects
```

## Risk spots
- Cooldowns are absolute fixed ticks in the book: E4 saves must rebase them if the tick counter restarts on load.
- Abilities learned on load: E4 re-registration must not re-`Learn` (book already holds them) but re-land
  instances from the saved active set.
- `apply` (book-less) silently discards book state; hosts must call `apply_book`.

## For the orchestrator
- CHANGELOG: `2026-11-08 sf_effects E3 (book + casting): SourceTable, SpellBook, apply_book with Cast/Learn/Forget/Select/Cure/Invalidate, power recharge on game minutes, tick cooldowns, Request archetype and touch/launch delivery requests, 9 tests; written without compiling, UNTESTED (owner pass 2026-11-10).`
- ROADMAP: effects E3 line -> written, untested (owner pass 2026-11-10).
- test-pass-2026-11-10.md: `cargo test -p sf_effects` now includes tests_book; also re-run `cargo test -p mw_effects`.
- DECISIONS (optional): `2026-11-08 effects E3: sf_effects keeps world time as GameMinutes (no sf_rpg dep); apply_book takes an EffectEnv so E1's apply/EffectCtx stay stable; power recharge is checked by the step, tick cooldowns by the cast model.`
- mw_effects owner: `MwSource` can now become `sf_effects::SourceDef` (recharge -> `Some(GameMinutes::days(n))`, affliction kind -> `tags`).
