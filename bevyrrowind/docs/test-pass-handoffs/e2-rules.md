# e2-rules (sub-agent, 2026-11-08)

Scope: sf_effects slice E2 (the rules seam) per superfargoth/docs/specs/effects.md "The rules seam" / Build plan E2.
Status: DONE (written 2026-11-08 without compiling: UNTESTED, owner test pass 2026-11-10). Nothing committed.

## Files
- superfargoth/crates/sf_effects/src/rules.rs: extended. New: `Protection`, `ProtectQuery`, `CastFailReason`,
  `CastPlan`, `CastQuery`, `CastModel`, `ItemQuery`, `ItemModel`, `EffectRules` (`standard()`, `from_model`,
  Default, manual Debug). `LandModel::protect` added WITH a default (`Applied`) so other implementors keep compiling.
  `ResistQuery` gained `pub target: Option<SheetRead<'a>>` and lost its derived Debug (SheetRead has no Debug; manual
  impl instead). `TestRules` gained `protect: Protection`, `cast: Option<CastPlan>` (still `Default`) and implements
  `CastModel` + `ItemModel`.
- superfargoth/crates/sf_effects/src/standard.rs (new): pure rules `cast_rule` (sf.effects.cast), `magnitude_rule`
  (sf.effects.magnitude), `protect_rule` (sf.effects.protect), `resist_rule` (sf.effects.resist), `item_charge_rule`
  (sf.effects.item_charge), helper `entry_cost`; In/Out types `CastIn`, `MagnitudeIn`, `ProtectIn`, `ResistIn`,
  `ItemIn`, `ItemCharge`, `CostEntryIn`, `CostConsts`; `StandardEffectRules` (all three traits).
- superfargoth/crates/sf_effects/src/tests_rules.rs (new): 12 tests.
- superfargoth/crates/sf_effects/src/lib.rs: `mod standard`, `mod tests_rules`, exports, crate doc line.
- superfargoth/crates/sf_effects/src/step.rs: ONE edit, the `ResistQuery` literal gets `target: None`.
- superfargoth/docs/specs/effects.md: status line, rules-seam bullet (no sf_rules registry), "As built" (E1 moved,
  E2 added), open questions moved. 25.9 KB.
- superfargoth/docs/specs/archive/effects-history.md (new): E1 as-built + answered open questions.

## E1 signature changes (additions, but literal-constructor breaking)
- `ResistQuery { .., target }`: any other code building a `ResistQuery` literal needs `target: None`. Only step.rs
  did (fixed).
- `ResistQuery` Debug is now manual (same trait, no derive).

## Unverified guesses (no compiler)
- `#[derive(Default)]` + `#[default]` on `Protection` alongside serde derives.
- Closure param inference in `resist`'s `sum` closure (`|keys: &[StatKey]|` called with `.as_slice()`).
- `trivially_copy_pass_by_ref` fires only on `resist_rule` (`ResistIn` 8 B); allowed there. If clippy flags others,
  add the same allow.
- Test sheets: pools full after `Settle::NewCharacter` with `TestModel` (same assumption as E1 tests).
- f32 boundaries in tests: `0.3 * 100.0` rounds above 30 (reflect 30 % not triggered), `0.5 * 100 == 50` (not < 50).
  Costs compared with `near` (1e-4).
- rustfmt: a few test lines exceed 120 cols; run `cargo fmt`.

## Owner test commands
```
cargo test -p sf_effects
cargo clippy -p sf_effects --all-targets -- -D warnings
cargo fmt -p sf_effects
```

## Risk spots
- Modern cost numbers (scale 0.1, area 5 m, 0.5 %/skill point, 10 item uses) are invented defaults, not calibrated.
- The step does not yet call `protect` nor pass sheets to `resist` (E3/E5 must wire both; until then
  `StandardEffectRules::resist` returns 1 from the step).
- Reflect is per source: refused if any entry is unreflectable; needs at least one harmful entry.

## For the orchestrator
- CHANGELOG: `2026-11-08 sf_effects E2 (rules seam): EffectRules with CastModel/LandModel/ItemModel, StandardEffectRules (modern defaults) as pure sf.effects.* rule fns, 12 formula/round-trip tests; written without compiling, UNTESTED (owner pass 2026-11-10).`
- ROADMAP: effects E2 line -> written, untested (owner pass 2026-11-10).
- test-pass-2026-11-10.md: `cargo test -p sf_effects` (E1 + E2 tests together), clippy pedantic on sf_effects.
- DECISIONS (optional): `2026-11-08 effects E2: rules are plain pure fns (fn(&In) -> Out, serde In/Out) with the rule id in the doc; no sf_rules crate until Luau is unparked. Absorb is checked before reflect; reflect is per source.`
- SYSTEMS.md: effects row may add `archive/effects-history.md`.
