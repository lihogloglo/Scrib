# Handoff: gameplay coordinator session, 2026-11-10 (cloud)

Written by the coordinator of a cloud session that ran two to three agent lanes at once: NPC services and magic
(mechanics), first-person combat, and docs-only specs. Branch `ccr-58483ee5-4xe6gy`, pushed. **Everything below
compiled and passed its headless tests and pedantic clippy in a Linux container with no Morrowind install and no
GPU: nothing was run in game, no install-gated test ran, no shader was compiled on a GPU, no clip was baked.** Treat
it like the 2026-11-08/09 laptop work: owner test pass first (`bevyrrowind/docs/test-pass-2026-11-10.md` style).

## Commits (oldest first)

| Commit | Slice | Notes |
|---|---|---|
| 268721f | Crafting spec (alchemy, enchanting, spellmaking) | `superfargoth/docs/specs/crafting.md` + `bevyrrowind/docs/specs/crafting-adapter.md`; GMSTs marked U unverified |
| 0821ca3 | Services TR1: barter in the host | `mw_rpg_bevy/src/dialogue/services.rs`; dialogue `Choice` filter fix (all dialogue) |
| a1c28f2 | FP-1: role layers, head hidden in first person | new spec `superfargoth/docs/specs/first-person.md` |
| 3a42c85 | TR2: trade screen over the dialogue | `dialogue/trade_screen.rs`; PUI-SV3 |
| 4ab29cd | A2b: Mercantile use on a won haggle | OpenMW `tradewindow.cpp` `haggle()` |
| d6043ff | C4 soft aim, first-person melee | `Command::face_view`; CB-5 |
| f279927 | FP-1b: per-worldspace shadow-only layer | `WorldspaceLayers`; FP-1b check |
| 6fdc7e2 | `sf/services` save section (purses) | |
| 54ced4e | FP-2 design: one render, not a second camera | DECISIONS bullet |
| 533a33e | FP-2a: view-model math + view copies | `sf_camera::view_model` |
| 22ef6b7, ef1243a | E4: `sf_effects_bevy` + saves | `sf.character` v4, `sf/effects` section, NPC effects blob |
| e12449b | Effects A2: drinking potions | `mw_rpg_bevy/src/item_use.rs` |
| f1a1acb | FP-2b: view-model material + WESL stages | **also reverted e12449b by a stale index** |
| 7c3b441 | Restore A2 | exact restore; checked: nothing else lost |
| 161338c | `sf_combat_bevy` gizmo test-harness fix | `--all-features` tests pass again (bug from 2026-11-06) |
| 2b7bbd4 | C4b: first-person attack look limits | 45 deg yaw, -45..20 pitch; CB-5b |
| fb222c6 | Effects A3: start spells, abilities | autocalc spells left (A3b) |
| da01e78 | Pool fortify fixes (OpenMW semantics) | temporary Fortify = current over max (OpenMW) |
| 216540f | C2a: sparring dummy + player reactions | scene `level-sparring`; CB-3a; non-lethal scaffold |
| 9ad62a6 | Effects A5a: cast rules + host cast | classic (OpenMW) and modern rules; console `cast` |
| ed9144c | C2b: guard in both views | heavy attack moved to R; CB-3b |
| b20aa56 | Services A3: training | list-service path; 2 h pass moves globals + sky |

Final-hour slices (A5b cast input, services A5 spell purchase, crime spec) are listed at the bottom once their agents
report; a "WIP ... (UNVERIFIED)" title means it was cut off and its commit message says what is left.

## Owner checks waiting (visual/feel; all "awaiting owner")

FP-1, FP-1b, FP-2 (`flip viewmodel` in the console; check the shader-compile log line first), CB-5, CB-5b, CB-3a,
CB-3b (`cargo dev scene level-sparring`), PUI-SV3 (barter, Arrille), PUI-SV4 (training), plus every install-gated
test (`--ignored`) in the crates above, and a release-build measurement of `sf_effects_bevy` (dev profile measured
0.089 ms for 2,000 effects; budget 0.03 ms).

## Owner questions collected (defaults taken in code; overrule any)

1. FP-1: neck hidden with the head (collar line visible looking down) OK, or hide upper torso?
2. FP-1b: ray-traced reflections can show the hidden head. Keep?
3. FP-2: accept the single-pass view model (decided by the coordinator)? Turn `view_model` on by default after FP-2?
   Shoulder seam: accept, blend k near shoulders, or `fov_body = fov_world` (first-person.md open question 4)?
4. Soft aim: body faces the view while walking in first person (strafe), or the stick (today)?
5. C4b: do 45 deg yaw / -45..20 pitch frame the swing? Release limits between chained swings too?
6. C2b: heavy attack on R (right mouse is now guard) OK? Guard arc 80 deg and numbers? Guard walk next? No guard
   while sheathed OK?
7. TR2: "deal done" message after the window closes? Map trade-screen GMST labels as a slice?
8. Training: modern flow (confirm each row, list stays open) vs OpenMW's (closes after training)?
9. Effects: `sf/effects` world section vs `sf/meta`; abilities saved as active instances (not rebuilt); NPC effect
   timers pause while unloaded (built) vs cleared (OpenMW)?
10. Pools: Morrowind temporary Fortify shows current over max (140/100). Keep, or modern max-raising Fortify?
11. Casting (modern rules): Sound cost x(1 + Sound/100)? Silence refuses with nothing spent (classic fizzles)?
    Per-new-game rule-set choice now or with the parked classic option?
12. Crafting spec: 9 questions with defaults in `crafting.md` "Open questions".

## Known bugs and gaps logged (not fixed)

- MWScript `ModMagicka`/`SetMagicka` (and fatigue) on derived pools are overwritten (`stats-adapter.md`).
- Merchants equip the first item per slot, not OpenMW's `autoEquip`; purses of actors gone from content aren't
  dropped on load; no NPC rest during training; no fade for passing time.
- Drink sound not played (no audio dep in `mw_rpg_bevy`); potions don't break invisibility; no quick keys.
- Target spells (Launch) are dropped until projectiles exist.
- Saves from fb222c6 with a Fortify Health ability would double-count on load (no vanilla start spell does this).
- Specs over ~25 KB: `services.md` 26.0, `effects.md` 26.7, `stats.md` 25.7, `crafting.md` 26.6, `combat.md` ~25.3.

## Next slices, in order (each brief: grep the named spec's Build plan and the last agent's handoff)

1. **C2c** backstep + knockdown via `Owner::Physics` (combat.md C2; ed9144c commit message has the starting state).
2. **One game clock** (ROADMAP 1.17): training already moves `SkyTime` and the MWScript globals by hand.
3. **A4 travel**, **A6 merchant repair** (services-adapter.md; A5 spell purchase handoff has their starting state).
4. **A3b autocalc spells**; **A6 enchantments**; **A7 afflictions** (effects-adapter.md Build plan).
5. **HUD active-effect icons** (`HudView.effects`, OpenMW `SpellIcons`), FP-3 held items (torch light at the
   view-model position, spell-hand VFX).
6. **Crime** rules-only slices from the new crime spec; **crafting** CR1 onward.
7. Bigger missing systems (no spec yet): chargen screens, level-up screen, stats screen, encumbrance, ranged
   combat + projectiles, rest/wait, maps, the MWScript long tail (98 of ~225 functions live).

## Services A5 spell purchase: researched, not built (agent stopped at wrap-up before editing)

OpenMW `apps/openmw/mwgui/spellbuyingwindow.cpp` @ca5e2576:
- Listed (112-135): the merchant's own spell list, type `ST_Spell` only (no powers, abilities, diseases, curses); for
  NPC merchants, the merchant race's powers dropped; spells the player knows dropped (`hasSpell` 169-173); no
  "playable" flag; stable case-insensitive sort by name (47-50, 135).
- Price (56-59): `max(1, int(calcSpellCost * fSpellValueMult))`, then the barter offer = existing
  `mw_rpg::spell_price(&SpellPriceIn::new(terms, cost, g))`. Unaffordable rows stay listed, greyed (68-71).
- Buy (175-198): quietly refused if price > gold; else add spell, take gold into the merchant's pool (193), relist,
  sound "Item Gold Up"; no disposition change, no confirmation.
Plan: `mw_rpg` does not depend on `mw_effects` (`spell_cost` is in `mw_effects/src/source.rs` 235), so add two host
methods on `MwTradeHost` (barter.rs ~418, beside `training_sheets`): player knows spell (`Option<bool>`) and spell
cost; `View` implements them (`dialogue/services.rs` ~176) and needs the player's `Effects.book` passed in beside the
sheets through `relist_services` (~1008) and the purchase path. Merchant spells: the NPC's `Effects` book (OpenMW's
live list includes autocalc), else the record list. Rules: `MwService::Spells` arm in barter.rs `entries` (603) and
`buy_entry` (613) through `lists(...)`, rows `EntryDetail::Grant { what }`, effects `[Pay, Grant]`. Host: handle
`ServiceEffect::Grant` in `apply_service_effects` (~990, the "A4-A6" arm) with
`EffectCommand::new(StatOwner::Player(HOST_KEY), vec![EffectOp::Learn { source }])` as `cast.rs` `console_cast` does.
Screen: Grant-row label texts in `service_list_screen.rs` `add_texts`. services-adapter.md is ~25.8 KB: move history
to its archive when writing "As built".
A6 repair: `EntryDetail::Item`, `ServiceEffect::SetAttr`, `mw_rpg::item_health`, `repair_price` exist; `SetAttr` not
applied. A4 travel: `travel_price`, `EntryDetail::Destination`, `ServiceEffect::Teleport { to, with_followers }`
exist; `PassTime` applied (training); `Teleport` and the `GetPCTraveling` flag not applied.

## Lessons for the next cloud coordinator

- Disk is the binding constraint (~40 GB allowance, `target/` grows to ~25 GB with 2-3 agents): trim stale test
  binaries, old incremental caches and duplicate workspace rlibs between slices; never `cargo clean`.
- Agents committing only their hunks of shared files must build their temporary index from the CURRENT HEAD right
  before committing, then check `git show --stat HEAD` (f1a1acb reverted another slice this way).
- System packages for Bevy builds: `libwayland-dev libxkbcommon-dev libudev-dev libasound2-dev`.
- `bevyrrowind/tools/cargo_turn.sh` is Windows-only; on Linux cargo's build lock serialises agents.
