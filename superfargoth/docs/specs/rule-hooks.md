# Rule hooks: Luau overrides for game formulas (`sf_rules`)

Status: draft 2026-11-08 (rule-hooks spec session, docs only; owner: "Yes, spec the hooks"). Nothing built.
**Owner 2026-11-08, later: everything starts in Rust; the Luau part comes later.** In force now: the house rule that
every event-rate formula is a pure Rust `fn(&In) -> Out` over one input struct holding every number it reads ("Declaring
a rule", minus the registry). Deferred until the owner unparks Luau: `sf_rules` as a crate, `@sf/rules`, R1-R3. Because
the formulas already have this shape, wrapping them later changes no caller.
Builds on [scripting.md](scripting.md) (contexts, watchdog, `@sf/interfaces`), [stats.md](stats.md) "The rules seam",
[combat.md](combat.md) "The rules seam" and ARCHITECTURE §1b (feel systems behind seams). First consumer outside the
framework: Bevyrrowind's ROADMAP 2.7 mechanics (barter, persuasion, services, crafting).

<!-- agent-summary -->
> **Agent summary.** Every *event-rate* formula behind a feel-system seam is a **rule**: a pure Rust function of one
> explicit input struct, declared once with an id (`sf.stats.skill_gain`, `mw.barter.price`). Luau mods wrap a rule with
> `rules.wrap(id, function(input, next) ... end)`; wraps chain in load order around the Rust default. No override = the
> Rust call directly. Rules are pure (no world access, no random unless the input carries a seed), host-only, run
> synchronously at the call site under the mod's watchdog, and fall back to the inner chain on any failure. Per-tick
> formulas (regen, derived maxima) are not hookable: they are tuned through records. Read "Design", then "Build plan".
<!-- /agent-summary -->

## Goal

A mod changes *how a game computes* something (a price, a hit's damage, how fast a skill grows) without forking Rust,
and several such mods compose. Morrowind's modding culture lives here (MCP and MWSE formula fixes, OpenMW's Lua
handlers), and so will any other pack's.

Requirements, in priority order:

1. **Unchanged without mods.** No override: the caller runs the Rust default directly, at the cost of one branch.
2. **Pure.** A rule's result depends only on its input (and read-only config). No world access, no side effects, no
   hidden randomness: same input, same output. Callers may call rules for UI previews as often as they like.
3. **Composable.** Several mods may wrap one rule; the order is the load order; each sees the next one inward.
4. **Safe.** A failing or slow override never breaks the game: the call falls back and the override is disabled after
   repeated faults (scripting.md "Error isolation and limits").
5. **Net-ready.** Rules run where gameplay runs (the host); results are what replicates (net.md).
6. **Typed and documented.** Each rule's input and output are declared once in Rust and generate the Luau types
   (`.d.luau`) and docs.

Non-goals: hooks on per-tick or per-entity-per-frame formulas (regeneration, fatigue term, carry capacity, movement);
hooks that change control flow of systems (what happens on a hit, AI decisions: those are events and goals); a rule
mutating the world; script-defined rules in v1 (only Rust declares rules); WASM overrides (later, same registry).

## Canonical technique

**A fixed, documented set of named hook points with declared inputs and one typed result, chosen by data, wrapped in
load order around a native default.** Sources (research brief 2026-11-08):

- **OpenMW Lua** (0.49/0.50): mechanics moved into built-in Lua scripts are exposed as *interfaces*. An override
  receives the previous implementation (`onInterfaceOverride(base)`) and should call it; `I.SkillProgression`
  (`addSkillUsedHandler`, `addSkillLevelUpHandler`) runs handlers in registration order over a shared options table,
  `return false` skips the rest and the default, and an empty handler list returns early.
  `I.Combat.addOnHitHandler` does the same for hits, and the armour getters the UI reads must stay side-effect free.
  Issue 9104 (attack strength not passed on misses) is the lesson that **a hook that lacks one of the formula's inputs
  cannot be overridden fully**. https://openmw.readthedocs.io/en/latest/reference/lua-scripting/interface_skill_progression.html ,
  https://openmw.readthedocs.io/en/latest/reference/lua-scripting/interface_combat.html , https://gitlab.com/OpenMW/openmw/-/issues/9104
- **Unreal GAS**: `UGameplayModMagnitudeCalculation` / `UGameplayEffectExecutionCalculation`: a calculation class
  selected by data, with a declared capture list of inputs and a typed result. https://dev.epicgames.com/documentation/en-us/unreal-engine/gameplay-effects-for-the-gameplay-ability-system-in-unreal-engine
- **Factorio**: numbers are prototype data, changed in the data stage (`data-updates`, `data-final-fixes`); runtime
  scripts only react to events and cannot change prototypes ("the game was built around them not being changeable").
  So *numbers go in data, logic goes in hooks*. https://lua-api.factorio.com/latest/auxiliary/data-lifecycle.html
- **Creation Engine**: formulas are hardcoded with GMST constants; perk entry points ("Mod Attack Damage") are a fixed
  list of named stages modders can't extend without native SKSE plugins. Our rule ids are that list, made extensible
  by Rust code and overridable by scripts. https://ck.uesp.net/wiki/Perk_Entry_Point

## Where rules sit: systems, rules, content

| Tier | Examples | Language | Mods change it by |
|---|---|---|---|
| Systems | physics, animation, combat timing and reactions, AI planning, replication, saves | Rust | not at all (new adapters, PRs) |
| **Rules** (this spec) | skill gain, level-up gains, hit damage, move costs, barter price, haggle, persuasion, service prices, spell chance, crafting output | Rust default, Luau wrap | numbers: record/GMST patches; logic: `rules.wrap` |
| Content | quests, dialogue results, scripted events, UI mods | Luau, data (MWScript translated) | scripts, records |

Rule of the house: **if a mod only needs different numbers, it patches records** (sf_records field patches, GMSTs);
a hook is for changing the logic. A rule's default reads its constants from records, so both routes stay open.

**Rate test** for "is this a rule?": evaluated per *event* (an attack lands, a skill is used, an offer is made, a potion
is brewed or previewed), at most hundreds of times a second in a busy scene. Per-tick-per-entity formulas fail the test
and stay `DerivedModel`/`CombatRules::regen`-style Rust, tunable by data only.

## Design

### Declaring a rule (Rust, Bevy-free `sf_rules`, no mlua)

```rust
/// A rule's stable name: `<owner>.<system>.<what>` (`sf.stats.skill_gain`, `mw.barter.price`).
pub struct RuleId(&'static str);

pub trait Rule: Send + Sync + 'static {
    const ID: RuleId;
    /// Bumped on an incompatible change of In/Out (see "Versioning").
    const VERSION: u16;
    /// Every number the formula reads (OpenMW 9104 lesson), plain serde data.
    type In: Serialize + DeserializeOwned + RuleType;
    type Out: Serialize + DeserializeOwned + RuleType;
}

/// The Luau type of a rule's input or output (derive), for `.d.luau` and output validation.
pub trait RuleType { fn luau_type() -> &'static str; }
```

- The **crate that owns the seam declares the rule** next to the seam (`sf_stats` declares `sf.stats.skill_gain`;
  `mw_rpg` declares `mw.barter.price`). Rules from different owners never share a prefix.
- **The input is the formula's whole input.** If the default reads a GMST, the GMST's value is a field of the input
  (or of a config struct the default captured at registration, also passed to Luau as `input.config`). A mod sees
  exactly what the default sees; the default is a pure `fn(&In) -> Out` and so is every override.
- **Outputs are numbers or small records, never ops or world changes.** The seam turns the result into `StatOp`s,
  combat outcomes or offers in Rust. Example: `SkillUseModel::use_skill` asks `sf.stats.skill_gain` for `gain` and
  keeps the progress-counter and `RaiseSkill` plumbing itself.

### Evaluating a rule (the call site)

```rust
pub struct Rules { /* per-rule override chains, resolved at start; counters */ }

impl Rules {
    /// The default when nothing wraps `R` (one indexed load and a branch), else the chain.
    pub fn eval<R: Rule>(&self, input: &R::In, default: impl Fn(&R::In) -> R::Out) -> R::Out;
}
```

- Seams receive `&Rules` beside their model (stats: in `ApplyCtx`/`TickCtx`; combat: a `CombatRules` impl holds an
  `Arc<Rules>`). Models stay testable without scripts: `Rules::none()` always takes the default.
- `Rules` holds, per declared rule, `Option<Arc<dyn RuleChain>>`. `RuleChain` is the trait the script runtime
  implements (`sf_script::rules`), so cores depend on `sf_rules` only, never on mlua (build-perf.md rule 1).
- Calls are **synchronous** and may come from any system on any thread (fixed tick, UI, dialogue). That is safe
  because rules are pure: an override never touches the world, so it needs no script slot or world guard
  (scripting.md "Execution model"). Calls into one mod's state serialise on mlua's state lock (`send` feature); two
  systems calling rules in parallel cost contention, never deadlock (a rule call can't happen inside a script slot of
  the same state, and rules can't call other rules' chains re-entrantly, below).

### Overriding a rule (Luau, `@sf/rules`)

```luau
local rules = require("@sf/rules")

-- Haggling mod: merchants pay 10% more for anything you sell, and never below the item's base value.
rules.wrap("mw.barter.price", function(input, next)
    local out = next(input)              -- the next rule inward: an earlier mod's wrap, or the Rust default
    if not input.buying then
        out.price = math.max(input.base_value, math.floor(out.price * 1.1))
    end
    return out
end)
```

- **One verb, `wrap(id, fn)`.** `fn(input, next)` may change the input before `next`, change the output after, or
  **replace** the rule by not calling `next`. Same power as OpenMW's handlers (`return false`) and interface
  overrides (`base`), one shape.
- **Registration happens while the context starts** (top level or `on_load`); `wrap` after start is an error. Chains
  are built once after every context started, in load order: the **last-loaded mod is outermost**. Hot reload of a
  mod rebuilds the chains (as `@sf/interfaces` republishes exports).
- `input` is a read-only copy (a table built from `R::In`, `table.freeze`); `next` accepts a modified copy. Values
  cross states as plain data (`StateValue`), as `@sf/interfaces` does.
- **Output validation:** the returned table is decoded into `R::Out`; a missing or ill-typed field is an error (below).
- An unknown rule id or a `VERSION` the mod doesn't know is a load diagnostic for that mod (the wrap is skipped).

### Purity inside a rule call

While a rule function runs, its context is in **rule mode** (a flag in the context's app data, set by the chain):

| Available | Not available (raises "not available in a rule") |
|---|---|
| `math`, `string`, `table`, `buffer`, `bit32` | `@sf/world`, `@sf/ui`, `@sf/anim`, `@sf/physics` (already: "outside a script slot") |
| `@sf/core` `api_version`, `mod_id`, `settings` (read), `log` | `@sf/events` `emit`, `@sf/timers`, `@sf/interfaces` calls |
| `@sf/core` `storage()` **read-only** (state the mod's own handlers wrote) | `math.random`, `sf.random` (use `input.seed` when a rule needs chance) |

Randomness: a rule that rolls (hit chance, persuasion success, alchemy failure) gets `seed: u64` in its input,
derived by the caller from (owner, tick, call kind) as `sf_stats` `Inputs::seed` already is (net rule 4), and the
default and overrides draw from `sf_rules::Rng::new(input.seed)` (Rust) / `rules.rng(input.seed)` (Luau, same PCG).

### Failure and limits

- Each link runs under its context's watchdog with a **per-call limit** (default 2 ms dev, 1 ms release; settable per
  rule in its declaration for heavier ones such as an alchemy preview over many ingredients).
- On an error, a timeout or an invalid output, **that call falls back to what the failing link's `next(input)` returns
  with the unmodified input** (the chain inward of it), so one broken mod costs its own change only. Logged once per
  mod and rule with the traceback, counted in `sf_diag`.
- 3 faults within 60 s disable **that link** for the session (the mod's other wraps and scripts keep running); the
  player is told as for a suspended context. A context suspended or failed for other reasons drops all its links.

### Versioning

- Additive changes (a new optional input field, a new optional output field) keep `VERSION`. A rename, removal or
  meaning change bumps it; the owner keeps the old version as a thin adapter for one framework major where cheap
  (scripting.md "Curated API and versioning").
- `rules.wrap` takes `"id"` (any version) or `"id@N"` (exact); type definitions are generated per version.

### Types and docs

`sf_rules` collects every declaration and emits `rules.d.luau`: one overload of `wrap` per rule, so `luau-lsp` checks
`input.buying` and the returned table, plus a generated page listing each rule (owner, version, inputs with their
meaning, output, rate, default formula source). Owners document each field once, on the Rust struct.

### Diagnostics

- Console `rules`: each rule with its chain (`mw.barter.price: rust <- better_haggle <- merchant_tweaks`), calls this
  second, mean and max µs, faults. `rules off <mod>` (dev) drops a mod's links live for A/B checks.
- Mod manager: a mod's "Overrides" list. A rule wrapped by two or more links where an outer one never called `next`
  during the session is flagged as a **conflict** (the outer mod replaced the inner one), the OpenMW last-loaded-wins
  pitfall made visible.

### Rule sets (modern, classic) and hooks

The rule set chosen per new game (stats.md, combat.md: modern or classic profile) decides which Rust default sits at
the bottom of each chain. Rule ids don't change with the profile; `input.profile` (a string id) tells a mod which
default it wraps. A mod that only makes sense for one profile checks it and calls `next` otherwise.

## Co-op (net-ready rules)

- Rules are evaluated on the **host only** (rule 1, server authority), where the fixed-tick gameplay runs. Results
  travel as the state they produce (stat ops, damage, offers). Guests never evaluate gameplay rules, so their mods'
  scripts don't need to match beyond the content fingerprint, which already covers every mod (net.md).
- **Never from predicted code.** Prediction covers the local character's movement only (net.md); a rule call there
  would let a guest's override differ from the host's. Rule-evaluating APIs are not available to prediction systems
  (lint once prediction exists).
- UI previews on a guest (a barter list) read prices from the host's service session, not a local evaluation.
- Pure functions of explicit inputs: replays and host migration re-evaluate identically (same build, same mods).

## Saves

Rules hold no state, so nothing is saved for them. Results that persist (a brewed potion's magnitude, a skill's base)
are stored as values: removing a mod changes future evaluations only, never existing items or sheets.

## Performance

| Item | Budget (reference laptop, release) | Basis |
|---|---|---|
| `eval` with no override | ≤ 20 ns over the bare default | an indexed `Option` load and a branch |
| One link, 10-field input and 3-field output | ≤ 3 µs | scripting.md L1: ~0.8 µs to build a 3-field payload with handles (2.0 µs with the memory limit, lifted for host-built payloads), 148 ns guarded call; decode of the output |
| All rule calls per frame, busy combat scene (20 hits/s, 50 skill uses/s, all wrapped twice) | ≤ 0.05 ms average | ~140 links/s × 3 µs |
| Barter screen open, 200 items priced, wrapped twice | ≤ 1.5 ms once per refresh | not per frame |

Levers if a measured case fails: cache per (rule, input hash) per frame for UI previews; `@native` on hot wraps;
batch rules over lists (`mw.barter.prices` taking all items) where a call site evaluates many at once.

## First rules

Declared by their seam owners when each seam is (re)built; the 2.7 mechanics are written as rules from day one.

| Id | Owner (seam) | Event | Input (summary) | Output |
|---|---|---|---|---|
| `sf.stats.skill_gain` | `sf_stats` (`SkillUseModel::use_skill`) | a skill use | skill key, use kind, scale, base, progress, class use, specialization match, use value, config | `gain` |
| `sf.stats.level_up_offer` | `sf_stats` (`LevelingModel::offer`) | level-up screen | level, tallies per attribute, picks | gain per attribute, pick count |
| `sf.stats.rest` | `sf_stats` (`DerivedModel::rest`) | rest or wait | hours, sleep, attributes it reads, pools | restored per pool |
| `sf.combat.damage` | `sf_combat` (`CombatRules::resolve_contact`) | a contact | attack, move value, strength, absorb, guarded, poise inputs, profile | health and stamina damage (reaction stays Rust) |
| `sf.combat.cost` | `sf_combat` (`CombatRules::cost`) | a move start, a hit window | base, strength, class | pool deltas |
| `mw.barter.price` | `mw_rpg` (`PriceModel`, 2.7) | an offer row | base value, buying, mercantile and personality and luck (both sides), disposition, fatigue terms, GMSTs | price |
| `mw.barter.haggle` | `mw_rpg` (2.7) | an offer | offered vs asked, both sides' terms, seed | accepted, disposition change |
| `mw.persuasion` | `mw_rpg` (2.7) | admire, intimidate, taunt, bribe | kind, amount, both sides' terms, disposition, seed | success, disposition change |
| `mw.service.price` | `mw_rpg` (2.7) | training, travel, spell, repair offer | service kind, base cost, terms | price |
| `mw.spell.chance` | effect runtime (planned crate) | a cast, the spell list | spell cost, school skill, attributes, fatigue term, sound/silence | chance |
| `mw.alchemy.potion` | crafting (2.7, planned crate) | brew, preview | ingredients' effects, alchemy skill, apparatus quality, attributes, seed | effects with magnitude and duration, success |
| `mw.enchant.result` | crafting (2.7, planned crate) | enchant, preview | effects, soul, item capacity, enchant skill, seed | charge, cost, chance |

`sf.*` rows are framework seams any pack gets; `mw.*` rows are Bevyrrowind's and follow its adapter specs once
written. Not rules (per-tick, data only): `DerivedModel::pool_base`/`derived`/`regen`, `CombatRules::regen` and
`can_start`, disposition while filtering dialogue (evaluated per INFO; a cached `mw.disposition` per conversation is
an open question).

## Industry comparison

| Technique | Who | Us |
|---|---|---|
| Native default, script wrapper gets the previous implementation | OpenMW `onInterfaceOverride(base)` | Match (`next`) |
| Ordered handler chain, any handler may skip the default | OpenMW `I.SkillProgression`, `I.Combat` | Match: not calling `next` |
| Fast path with no handlers | OpenMW (`#handlers == 0` early return) | Match (≤ 20 ns) |
| Declared inputs, typed single result, chosen by data | UE GAS magnitude and execution calculations | Match: `Rule::In`/`Out`; chosen by the rule set + mods |
| Fixed named list of hook points | Creation Engine perk entry points | Match, but extensible by Rust and overridable by scripts (Bethesda needs SKSE) |
| Numbers in data, logic in code | Factorio data stage vs control stage | Match: records first, hooks for logic |
| Overrides of UI-read getters must be side-effect free | OpenMW combat docs | Stronger: purity enforced (rule mode) |
| Last-loaded override silently replaces others | OpenMW interfaces (pitfall) | Differ: chains compose, replacement flagged as a conflict |
| Formula inputs missing at the hook | OpenMW issue 9104 (pitfall) | Avoided by rule: the input is the formula's whole input |
| Scripts on per-actor (local) vs global VMs | OpenMW local vs global scripts | Differ: per-mod states, rules callable from any system because they are pure |

## Tests

Exact (logic): chain order follows load order; `next` with a modified input reaches the default with it; not calling
`next` replaces; an error, a timeout and an invalid output each fall back to the inner chain with the original input;
3 faults disable the link only; rule mode refuses `@sf/world`, `emit`, timers, interfaces and `math.random`; `wrap`
after start errors; unknown id and unknown version are load diagnostics; same input gives the same output twice; hot
reload rebuilds the chain. Each declared rule: its default against the ported formula's own tests (exact). Measurement
(`#[ignore = "measurement: ..."]`): `eval` without overrides, one and three links, the 200-item barter refresh.

## Build plan

| Slice | Deliverable | Verify |
|---|---|---|
| **R0** | This spec, owner answers | owner |
| **R1** | `sf_rules` (Bevy-free, no mlua): `RuleId`, `Rule`, `RuleType` (+ derive or macro), `Rules::eval` fast path, `RuleChain` trait, `Rng`, counters | unit tests with a fake chain |
| **R2** | `sf_script::rules` + `@sf/rules` (`wrap`, `rng`), chain build in load order, rule mode, per-link limits and fallback, `rules.d.luau` | tests in `sf_script` (Luau, no Bevy) |
| **R3** | Bevy wiring (`Rules` resource built after script start, rebuilt on reload), console `rules`, `sf_diag` | headless app test |
| **R4** | First seam: `sf.stats.skill_gain` through `SkillUseModel`, plus an example mod (`mods/examples/fast_learner`) | exact stat tests unchanged without the mod; with it, gain × 2 |
| **R5** | `sf.combat.damage` / `cost` | combat tests unchanged; an example wrap |
| **R6+** | Each 2.7 mechanic declares its rule in its own slice (house rule from R1 on) | per mechanic |

R1 is useful before Luau unparks: it fixes the shape every 2.7 formula is written in (pure function of one input).

## Open questions (owner)

1. **Unpark Luau for `@sf/rules` only** (R2-R3) while the rest of the scripting lane stays parked? Recommended: yes,
   because it's small and it's what makes 2.7 moddable.
2. **Read-only `storage()` inside rules**: lets "prices rise after you sell a lot" mods work, at the cost of rules being
   pure in (input, mod state) rather than input alone. Recommended: yes (mod state is saved and host-side).
3. **Per-call limit**: 1 ms release by default?
4. **Replace detection**: warn in the mod manager when an outer wrap never calls `next` (recommended), or require an
   explicit `rules.replace` verb?
5. **Disposition** (evaluated per dialogue INFO): cache a `mw.disposition` rule per conversation, or keep it data-only?
