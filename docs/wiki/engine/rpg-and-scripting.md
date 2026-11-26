# RPG, stats and scripting

## What it does
Dialogue, journal, quests, inventory, trade and stats run on a Bevy-free framework core with the Morrowind rules
plugged in through adapter hooks. MWScript is read by a native interpreter that today serves dialogue results and
the console. The sandboxed Luau runtime exists in the framework and is parked.

## How it works
- **Framework RPG** (`superfargoth/crates/sf_rpg`): item stacks, `Loadout` (`equip.rs:92`), `QuestLog`
  (`quest.rs:34`), `Journal` (`:129`), `DialogueSession` state machine (`dialogue.rs:182`), services (`service/`).
  No rules or formats: hooks `DialogueRules`, `EquipRules`, `LinkRules`, `ServiceRules`. All saved data is plain serde
  keyed by stable ids.
- **View-models** (`sf_rpg_ui`): dialogue window, journal, trade, service list; screens never read `sf_rpg`. Widgets
  live in `sf_ui_screens` (see ui-and-menus).
- **Stats** (`sf_stats`): `StatTable` (content defines stats), `Sheet`, `StatOp` run by pure `apply`/`tick`
  (`ops.rs:158`, `:187`); layered source-tracked modifiers, never mutate-and-undo. Content rules: `DerivedModel`,
  `SkillUseModel`, `LevelingModel`. `sf_stats_bevy` schedules it (`Stats` component, owner by `StatOwner`, no
  `Entity` in saves).
- **Morrowind RPG** (`mw_rpg`, Bevy-free, ported from OpenMW): dialogue DB from DIAL/INFO, filters and text macros
  (`MwDialogueRules`), result scripts (`MwResults`, `runner.rs:113`), barter, disposition, persuasion, training,
  services, equip slots. `mw_stats`: stat table from attributes and SKIL, GMST, sheet init, `openmw_rules`, faction
  ranks.
- **In the game** (`mw_rpg_bevy`): `dialogue::add` (`exterior.rs:316`): `Rpg` resource is the `MwView`/`MwWorld`
  over live state; E activates the focused NPC, an unhandled activation opens dialogue; `J` toggles the journal.
  `stats::add` (`exterior.rs:318`): player sheet on its own entity, NPC/creature sheets attached on spawn,
  write-back on unload. `inventory`, `inventory_ui`, `item_use` (potions), `effects`, `cast` (spells).
- **MWScript** (`mw_script`): lexer, lenient parser, resolver; each leniency is a named `Rule`, a script with an
  Error is inert. Native interpreter `interp.rs` walks the AST over a `Host`; step-limited. A Luau emitter (`emit.rs`)
  exists, superseded (DQ-1 2026-11-25: interpreter + one game API; deletion is its own slice).
- **Global MWScripts** (DQ-4, 2026-11-25): `mw_rpg_bevy/src/dialogue/global_scripts.rs` `GlobalScripts` parses every
  SCPT once at new game, starts `main` then SSCR in record order, ticks every `FixedUpdate` through `mw_rpg::ScriptHost`
  (`script_host.rs`), applies ops with `Rpg::apply`. Saved as `mw/scripts` (`mw_saves_bevy`). `scriptid.var` reads and
  writes go through a copy in `Rpg` (`ScriptLocals`): a script id wins over an actor id (OpenMW `getMemberType`).
- **Luau** (`sf_script`, `sf_script_bevy`): one sandboxed state per mod, memory/CPU limits, fault isolation,
  timers, snapshots; `ScriptSlot` systems `Input`/`Update`/`Ui` (`sf_script_bevy/src/lib.rs`). `mw_scripts_bevy::add`
  (`exterior.rs:551`) creates contexts per mod, script handles for placements (`HANDLE_CLASSES`: statics, items, doors, lights, actors, containers, activators), and door activation.

## Bevy features used
Resources, events, exclusive systems for script slots. No Bevy scripting crate.

## Switches and counters
No `--off` part. Unknown MWScript functions are reported once per session by `MwResults`.

## Known limits
Object (local) MWScripts do not run yet (DQ-5); globals do, with ~63 % of call sites bound (DQ-2): unbound calls are
reported once. Step limit is the tunable's default, not the store's. Luau beyond L3 is `[p]` (ROADMAP). Crime, crafting and services have adapter
specs; check each ROADMAP line for status before relying on them.

## Specs
`superfargoth/docs/specs/rpg.md`, `stats.md`, `scripting.md`, `rule-hooks.md`, `services.md`, `crime.md`;
`bevyrrowind/docs/specs/rpg-adapter.md`, `stats-adapter.md`, `scripting-adapter.md`, `services-adapter.md`.
