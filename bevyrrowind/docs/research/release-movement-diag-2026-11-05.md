# Release start: "new movement seems absent" (diagnosis, 2026-11-05)

ROADMAP 3.1, owner 2026-10-30. Code read only: no build, no run. Delirium set aside (default feature since 2026-11-05).

## Findings

1. **No code-path split (high confidence).** The release start (`play`, main.rs:896), `view "<cell>"` (main.rs:933) and
   `view level:` / `scene level-*` (apps/bevyrrowind/src/level.rs:15) all end in `mw_exterior_bevy::exterior::run`, which
   loads the same baked humanoid clips (`player_clips`, exterior.rs:1447) and calls the same `mw_player_bevy::add`
   (exterior.rs:504). Same body (`spawn_body`, mw_player_bevy lib.rs:349), same rules, same `Physics` probe, no movement
   code behind `dev` / `debug` / `debug_assertions` (only combat/physics gizmos). The owner's own 2026-11-01 release
   new-game log (`%TEMP%\bw-owner-2026-11-01\last-run.log`) shows `player clips: 23 of 23 slot clips baked` and
   `combat: sheet sword_1h with 21 slots, 15 moves`. The only level-only systems are `mw_exterior_bevy/src/level.rs`
   `spawn` (greybox, dummies) and `equip_player`.
2. **Most real obstacles are classed `Vault`, which no rule takes (medium-high; most likely cause for mantles).**
   The traversal rules accept only `Mantle` (sf_action traversal_rules.ron:27 band 0.3-0.85 h, :53 band 0.85-1.25 h,
   i.e. 0.54-2.25 m for 1.8 m). The context probe classes an obstacle at most `vault_max` 0.7 h (1.26 m) high and at
   most `vault_depth` 0.33 h (0.59 m) deep with lower ground behind as `Vault` (sf_controller context.rs:64-65, 451-457);
   `Vault` maps to `ObstacleClass::Vault` (sf_action_bevy lib.rs:345-351) and nothing matches, so Space is a plain jump.
   Test ledges are 1.5 m deep boxes (levels/anim-lab.ron:71) and always class as `Mantle`. Barrels, fences, railings,
   thin walls, narrow crates and tables in Vvardenfell are shallow and never mantle; under 0.54 m nothing triggers either.
3. **Combat and lock-on have no targets or weapon outside levels (high, by design until wired).** `Lockable` is only on
   training dummies (mw_player_bevy dummy.rs:80), spawned only from a level's `dummy*` spawns (level.rs:247-252); NPCs get
   no `Lockable` / `Combatant`, so Q locks nothing and swings hit nothing. The weapon shown on F comes only from a level's
   `player_held` (level.rs:268-276; `EquippedHeld` has no other writer), so the game draws an empty hand.
4. **Timing (low).** Wall brace (B2c), the head-on wall stop and the palm frame were built 2026-10-30..11-05; the
   10-30 owner copy (`bw-owner-2026-10-30`, exe 12:37) may predate part of B2c. Re-check the brace with a current build.
5. **Unverified (low).** Morrowind collision on rocks and clutter (sloped faces read as walkable slopes, missing
   collision on small props) could also refuse; slice A's log would show it.

## Fix slices (one each)

- **A. Log what the probe saw (diagnosis, small).** `sf_action_bevy` `step_actions` / `sense_obstacle` (lib.rs:295-342):
  on a Jump/Up press, `debug!`/`info!` once per press the `ContextHit` kind, height, depth and whether a traversal
  started (mw_player_bevy can log from `ActionCharacter::output`). Test: `tests_mantle.rs` asserts the logged/returned
  kind for a 1.0 m high 0.4 m deep box with floor behind (`Vault`). Owner check: `cargo dev view "Seyda Neen"`, W+Space
  at a barrel, a crate and the dock railing; read the log lines.
- **B. Shallow obstacles mantle until a vault exists (build).** Named scaffold: let `traverse.mantle.waist` accept
  `classes: [Mantle, Vault]` (traversal_rules.ron:27) with a removal note ("until a vault rule + clip"), or add a vault
  rule if a UAL2 vault clip is in the sources. Test in `sf_action_bevy/src/tests_mantle.rs`: 1.0 m x 0.4 m box with
  floor behind starts the waist mantle (today a plain jump); add a 0.4 m deep ledge to `levels/anim-lab.ron`. Owner
  check: `scene level-anim-ledges` (new shallow box) and `cargo dev view "Seyda Neen"`: mantle a crate and a barrel,
  brace on the Census office wall by running into it.
- **C. Game-side combat (ROADMAP 3.x, ask the owner first; not a fix).** Player weapon from the inventory's equipped
  weapon as `EquippedHeld` (replaces level-only `player_held`), and `Lockable` + `Combatant` on NPCs (the AI fight goal).
