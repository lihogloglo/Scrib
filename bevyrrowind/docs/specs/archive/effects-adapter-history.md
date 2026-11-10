# effects-adapter history

Dated "As built" blocks moved verbatim from [effects-adapter.md](../effects-adapter.md) (2026-11-10, doc budget).

## As built (A1)

Written 2026-11-08 without compiling: UNTESTED (owner test pass 2026-11-10). Crate `mw_effects` (deps `sf_effects`,
`sf_stats`, `sf_records`, `mw_stats`, `tes3`; no `mw_records`): `mgef.rs` (143-row table of OpenMW `RefId` name,
`sEffect*` GMST label, `HardcodedFlags`; resistance / weakness / shield maps; `archetype(index)`; `MwEffects`),
`stats.rs`, `source.rs`, unit tests in `tests.rs`, `tests/real_install.rs` (`#[ignore]`). Choices:

- Flags as OpenMW's legacy loader: `(record & (spellmaking | enchanting | negative light)) | hardcoded`, not a
  plain OR. `EffectDef` has no `school`, ranges or presentation yet (E1 deferred them): they live in
  `MwEffectInfo` (`school`, `ranges()`, `Presentation`). Label = the GMST name. Stacking: class default.
- Only A1 indices get a def (47: the list above plus Poison, and the three elemental shields as magnitude stats
  because they resist fire / shock / frost damage). Other indices keep key and info; a landing is `Rejected`.
- Magnitude stats: key = effect key (`resistfire`), group `magnitude` (as `mw_stats`'s `fortifymaximummagicka`,
  which Fortify Maximum Magicka writes), `Value { min 0 }`. The 22 defs live in `mw_stats::MAGNITUDE_STATS`,
  appended last to `mw_stats::defs()` (`MwIdx::magnitudes`; older indices unchanged, saves are keyed so no version
  bump); `mw_effects::MAGNITUDE_STAT_EFFECTS` names their MGEF indices, a test checks the two agree (2026-11-08).
- Sources: `MwSource` (the adapter's `SourceDef` until E3) with `recharge` (`GameDays(1)` for powers) and
  `affliction` (disease / blight / curse) beside the flat `SourceClass`; `inflict()` builds the op (abilities
  `ignore_resist`). Entries of indices without an MGEF record are dropped and stray arguments cleared
  (`esmstore.cpp` `getSpellsToReplace`); a skill / attribute effect with no valid argument is kept with none and
  reported (`MapNote`). Areas: feet × 0.3048 m. ALCH: self, no area, no cost. SPEL cost: `calcSpellCost` with
  `fEffectCostMult` (`MwEffectGmst`, vanilla 0.5). ENCH / INGR mapping: A6 / A2.

## As built (A2)

2026-11-10, headless tests (`mw_rpg_bevy::item_use::tests`, 4); not run in game. `mw_rpg_bevy/src/item_use.rs`:
the inventory screen's `InventoryIntent::Use(stack)` on the player's own view → `UseItemRequest { player, stack }`
(`request_item_use`); host `apply_item_use` (Update, before `inventory_ui::sync_open`): host player only, stack in
the loadout, ALCH record, else nothing; `Loadout::take(stack, 1)` (the last piece removes the stack), then
`EffectCommand(Player(HOST_KEY), [map_potion(..).inflict(Some(self), RollSeed(tick << 32 | n))])` and
`ItemUsed { player, item, sound: Drink }`. The MGEF table is kept as `effects::MwEffectsRes` for the mapping.
OpenMW (`inventorywindow.cpp` `useItem` 610-702, `potion.cpp` `use` 122-131, `actor.cpp` `consume` 79-92,
`spellcasting.cpp` 315-328, `activespells.cpp` 484-501): one piece removed before the cast; a stackable source is
always a new instance (second drink = two instances, each expiring on its own); no "cannot drink while" rule.
Differences: OpenMW inflicts self / touch / target entries as up to three instances, we land one instance; the
effect lands on the next fixed tick and a rate's first tick is on the landing tick (Restore Health 10 / 5 s from
70: 70.17 at landing, +20 two seconds later, capped at 100). Leftovers: the `Drink` sound is not routed (audio
glue, as dialogue sounds), drinking does not break Invisibility (A7), potion scripts' `OnPCEquip` /
`PCSkipEquip`, quick keys, MWScript `Equip` (functions.txt "later"). **HUD icon list: not built**: the HUD
(`sf_ui_screens::HudView`) has no effect-icon slot and its game glue is `mw_player_bevy::hud`; next slice
(`HudView.effects` aggregated per effect key as OpenMW `MWGui::SpellIcons`, icons from `Presentation`).
