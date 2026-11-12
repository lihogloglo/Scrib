# Saves

## What it does
One self-contained file per save, made of owner-keyed, versioned sections. The framework writes atomically off the
main thread; each owner supplies snapshot, apply and migration. The game has quicksave/quickload, a save/load
screen and Continue.

## How it works
- **File** (Bevy-free `superfargoth/crates/sf_save`): 64-byte preamble, table of contents, sections postcard-encoded,
  LZ4 when it helps, blake3-hashed (`container.rs:300` `SaveWriter`, `:410` `SaveFile`). `sf/meta` first and raw so a
  list reads a few KiB (`read_meta` `:512`). `Registry` (`registry.rs:149`): current/oldest versions, stepwise
  `v -> v+1` migrations; newer versions refused by name.
- **Write**: `write_atomic` (`fs.rs:196`): temp, fsync, read-back verify, optional `.bak`, rename. Slots: `SlotDir`
  (`slots.rs:88`), `<n>-<reason>.sfsave`, automatic saves rotate. CLI `sf_save info|verify|dump`.
- **Plugin** (`sf_save_bevy/src/lib.rs`): `SaveRequest` (`:301`) -> `SaveGate` (`:161`; transient blockers wait, a
  refused autosave retries every `AUTOSAVE_RETRY` 1 s `:786`) -> snapshot at frame end after `FixedUpdate` -> encode and
  write on the IO pool -> `SaveDone`. `LoadRequest` (`:330`): read/verify/migrate off-thread, then replace every
  provider in apply order (never merge) -> `LoadDone`. A `LoadTransition` defers apply to the fade's black cut
  (`apply_pending_load` `:1017`).
- **Sections** are added with `add_save_section`; sections no provider knows stay in `DormantSections` (`:212`) and are
  written back byte for byte. `Autosave` (`:197`): 10 min of play time by default.
- **Game sections** (`mw_saves_bevy/src/lib.rs:1-60`): `sf/overlay` (taken/removed placements by `PlacementId`),
  `sf/rpg`, `sf/clock`, `sf/weather` (seed, overrides, surface), `sf/character/<key>` (position, `ActionState`,
  `Locomotion`, inventory, effects), `sf/effects`, `sf/services`, `mw/rpg` (dialogue state, containers, script
  locals/globals). Stable ids only, never entity bits.
- **Files**: `<user data>/saves/single/<world id>/<n>-quick.sfsave`; two quicksaves rotate. F5/F9, rebindable
  (`mw_controls_bevy::Control::QuickSave`).
- **Quickload** runs in the live game behind a blackout: sections replaced at the cut, `StartTravel { reload: true }`
  moves the player and respawns resident cells from the overlay. A fresh run takes the newest quicksave of any world.
- **Screens**: `mw_saves_bevy/src/menu.rs` (`OpenSaveLoad`, `LoadChosen`, `DeferLoads`) serves the pause and main
  menus; Continue = `newest_save`. Toasts via `sf_ui_bevy::Toasts`.
- **New game baseline**: `sf_save_bevy/src/baseline.rs` (`apply_baseline`), captured on first Frontend entry.

## Bevy features used
`AsyncComputeTaskPool`/IO task pool, `Time<Fixed>` tick boundary, resources and events. No Bevy scene format.

## Switches and counters
Counters (with `SfDiagPlugin`): `save/snapshot_ms`, `save/encode_ms`, `save/write_ms`, `save/bytes/<owner>`,
`load/total_ms`. No `--off` part.

## Known limits
No load-side identity check or mod diff (save.md S3); `sf/records` unused; no save during dialogue; host `PlayerKey` is
a constant; guest character files (S4) not built; the game does not set an autosave policy beyond the default timer
(grep `Autosave` in `bevyrrowind/`: no game use).

## Specs
`superfargoth/docs/specs/save.md`, `net.md` ("Saves").
