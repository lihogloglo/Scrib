# Third-party notices

Code ported into superfargoth (and, where marked, into the game under `bevyrrowind/`) from other projects,
with their licences. Dependencies pulled through Cargo carry their own licence files and are not listed here.

## bevy-aqua

- Source: https://github.com/sayhisam1/bevy-aqua, version 0.1.3, commit `98d755ff1ff13c509d8616bd41afeab403ae1409`.
- Licence: MIT OR Apache-2.0 (used here under MIT). The upstream `LICENSE-MIT` has no copyright line;
  the holder is recorded as "bevy-aqua contributors" (maintainer: sayhisam1).
- Ported into: `crates/sf_water/src/spectrum.rs` (JONSWAP peak and alpha relations, integer hash,
  Box-Muller pair), with the changes listed in that file's header.

```text
MIT License

Copyright (c) bevy-aqua contributors

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
```

## skyrim-delirium (game only)

- Source: https://github.com/YoucefNabil/skyrim-delirium, commit `e44cb38d54bd79d8f2b54cc863a0df36df7e0542`:
  `fit-test/examples/make_character.rs` (character generator) and `tools/skel_joints.py` (the Spine3 joint rule).
- Licence: GPL-3.0-only (the full text is the repository's `LICENSE`, identical to the GNU GPL v3 at
  https://www.gnu.org/licenses/gpl-3.0.txt). No copyright line upstream; the holder is recorded as
  "skyrim-delirium contributors" (maintainer: YoucefNabil).
- Ported into: `bevyrrowind/crates/mw_delirium/src/body.rs` (the game, not the framework; the crate is
  GPL-3.0-only), with the changes listed in that file's header. The crate also links delirium-physics
  (github.com/PVCPC/delirium-physics, GPL-3.0-only) as a Cargo git dependency. The game app's `delirium` feature
  (on by default since 2026-11-05, release builds included) links both, so default game binaries are GPL-3.0-only;
  the GPU-vendor permission (`LICENSE-EXCEPTION.md`) doesn't extend to this code.

## OpenMW Clockwork (sunsoft directional PCSS)

- Source: https://github.com/OursCodeur/openmw branch `clockwork-alpha`, commit `c5e5a9a6362c35512823884753d2d5284af204fa`:
  `files/shaders/lib/light/sunsoft.glsl` (author: Thomas Portal).
- Licence: GPL-3.0-or-later (the full text is identical to the GNU GPL v3 at https://www.gnu.org/licenses/gpl-3.0.txt).
- Ported into: `superfargoth/vendor/bevy_pbr/src/render/shadow_sampling.wesl` (`sample_shadow_map_directional_pcss`, H2 world-unit sun PCSS with analytic receiver slope and world-meter penumbra scale across cascades), combined with the rotated blocker search kernel from `bevyengine/bevy#22272`.

## OpenMW Clockwork (surface weather)

- Source: https://gitlab.com/OursCodeur/openmw branch `clockwork-alpha`, commit `c5e5a9a6362c35512823884753d2d5284af204fa`:
  `files/shaders/lib/light/surface.glsl` (`surfaceExposure`, `surfaceWater`).
- Licence: GPL-3.0-or-later (the full text is identical to the GNU GPL v3 at https://www.gnu.org/licenses/gpl-3.0.txt).
- Ported into: `superfargoth/crates/sf_surface_bevy/src/layer.wesl` (shelter exposure: 6-texel tent over nine gathers,
  cover ramp, slope rise; wall streaks, wall and down-facing wetness, wet saturation; constants re-calibrated in meters)
  and its CPU twin `superfargoth/crates/sf_surface/src/layer.rs` (surface-weather.md W3).
- Also W2b (technique, no code): partial leaf shelter (`roofLeaves`, `LEAF_HOLD`) re-done as a separate canopy depth
  map in `layer.wesl`; rain washing ash (`sWashRate`) and quickening snow melt (`apps/openmw/mwrender/clockwork/scene/
  surfaceweather.cpp`) as linear wash rates in `superfargoth/crates/sf_surface/src/lib.rs` (`SurfaceTuning`).
- Also W4: the rain ring profile of `files/shaders/lib/water/rain_ripples.glsl` (`blip`, `blipDerivative`, `circle`,
  trailing rings; GPL-3.0 OpenMW code as Clockwork carries it, same commit) and the shape of `surfaceWater`'s puddle
  height branch, rewritten in WGSL in `layer.wesl` (`surface_ring`, `surface_drops`, `surface_puddle`) with the CPU
  twin in `layer.rs` (`ring`, `blip`, `puddle`); hash, grids and constants are ours, in meters.

## openmw-rtx (albedo de-lighting)

- Source: https://github.com/xorza/openmw-rtx, commit `f2f8b2d5b2cb142d1b454b3bb1815f4552980562`:
  `components/rtx/image/shadingmap.hpp` / `.cpp`, `components/rtx/shaders/shadingmap.h`,
  `components/rtxvulkan/shaders/texture/shadingsum.comp` / `shadingmap.comp`, `components/rtxvulkan/shaders/lib/ground.glsl`
  (`delitTexel`), constants from `components/rtx/shaders/look.h`, `scene.h`, `colour.h`; tests from
  `apps/components_tests/rtx/image/shadingmap.cpp`.
- Licence: GPL-3.0 ("GPLv3, as upstream" OpenMW; the full text is the repository's `LICENSE`, identical to the GNU GPL v3
  at https://www.gnu.org/licenses/gpl-3.0.txt). Holder: the openmw-rtx and OpenMW contributors (maintainer: xorza).
- Ported into: `superfargoth/crates/sf_texture/src/delight.rs` (the shading grid estimate and its apply step) and its
  tests, with the changes listed in that file's header.

## OpenMW (character stats)

- Source: https://github.com/OpenMW/openmw, commit `ca5e2576bfeb8fa64b53fdc8a31fefc360decc2f`:
  `apps/openmw/mwclass/npc.cpp` (`autoCalculateAttributes`, `autoCalculateSkills`, `round_ieee_754`,
  `Npc::ensureCustomData`, `getCapacity`), `apps/openmw/mwclass/creature.cpp` (`ensureCustomData`, `getSkill`,
  `getCapacity`), `apps/openmw/mwmechanics/mechanicsmanagerimp.cpp` (`buildPlayer`), `apps/openmw/mwmechanics/
  npcstats.cpp` (`updateHealth`), `apps/openmw/mwmechanics/creaturestats.cpp` (`recalculateMagicka`, fatigue in
  `setAttribute`, `getFatigueTerm`), `apps/openmw/mwmechanics/stat.cpp` (`DynamicStat::setCurrent`) and
  `apps/openmw/mwmechanics/spelleffects.cpp` (`damageAttribute`/`damageSkill` cap); stats S3:
  `apps/openmw/mwmechanics/actors.cpp` (`calculateRestoration`, `getRestorationPerHourOfSleep`,
  `restoreDynamicStats`), `apps/openmw/mwworld/class.cpp` (`getNormalizedEncumbrance`),
  `files/data-mw/scripts/omw/playerskillhandlers.lua` (skill progression and level-up options),
  `files/data/scripts/omw/skillhandlers.lua` (`skillUsed` gain), `apps/openmw/mwmechanics/npcstats.cpp` (`levelUp`,
  `getLevelupAttributeMultiplier`, `hasSkillsForRank`), `apps/openmw/mwgui/levelupdialog.cpp` (picks, cap 100),
  `apps/openmw/mwdialogue/filter.cpp` (faction rank skill/reputation requirements, `RankRequirement`); stats S6: `filter.cpp`
  (`getSelectStructInteger`, `testSelectStructNumeric`, `getSelectStructBoolean`, `testPlayer`: the PC and speaker stat
  functions), `apps/openmw/mwclass/armor.cpp` and `clothing.cpp` (`getValue`, for `PcClothingModifier`); stats S7
  (2026-11-08): `apps/openmw/mwscript/statsextensions.cpp` (attribute, skill, dynamic, level, crime and faction
  opcodes, `modStat`, `getDialogueActorFaction`), `apps/openmw/mwscript/dialogueextensions.cpp` (`Op*Reputation`),
  `npcstats.cpp` (`setFactionRank`, `setReputation`), `stat.cpp` (`DynamicStat::getRatio`); disposition and persuasion
  (2026-11-08): `mechanicsmanagerimp.cpp` (persuasion ratings, `getDerivedDisposition`,
  `getPersuasionDispositionChange`), `apps/openmw/mwdialogue/dialoguemanagerimp.cpp` (`persuade`), and GMST defaults
  from `apps/opencs/model/world/defaultgmsts.cpp`.
- Licence: GPL-3.0 (OpenMW: "GNU General Public License version 3"; the full text is the repository's `LICENSE`,
  identical to the GNU GPL v3 at https://www.gnu.org/licenses/gpl-3.0.txt). Holder: the OpenMW contributors.
- Ported into: `bevyrrowind/crates/mw_stats/src/{init,derived,progression,faction,gmst}.rs`, `mw_rpg/src/{stats,equip}.rs` (S6),
  `mw_rpg/src/script_stats.rs` (S7), `mw_rpg/src/{disposition,persuasion}.rs` (the game; f32 arithmetic in OpenMW's order,
  differences listed in the module docs) and, for the two stat semantics only, `superfargoth/crates/sf_stats/src/ops.rs`
  (`set_current`, capped `StatOp::Damage`).

## OpenMW (weather ambient by time of day)

- Source: https://github.com/OpenMW/openmw, commit `ca5e2576bfeb8fa64b53fdc8a31fefc360decc2f`:
  `apps/openmw/mwworld/weather.cpp` (`TimeOfDayInterpolator::getValue`, `lerp`, the `TimeOfDaySettings` set up in
  `WeatherManager::WeatherManager`, the ambient blend in `calculateTransitionResult`) and
  `apps/openmw/mwworld/weather.hpp` (`TimeOfDaySettings::addSetting`).
- Licence: GPL-3.0 (OpenMW: "GNU General Public License version 3"; the full text is the repository's `LICENSE`,
  identical to the GNU GPL v3 at https://www.gnu.org/licenses/gpl-3.0.txt). Holder: the OpenMW contributors.
- Ported into: `bevyrrowind/crates/mw_weather/src/ambient.rs` (the game; differences in the module docs: linear decode and
  luminance normalisation after OpenMW's gamma-space interpolation).

## OpenMW (book markup)

- Source: https://github.com/OpenMW/openmw, commit `ca5e2576bfeb8fa64b53fdc8a31fefc360decc2f`:
  `apps/openmw/mwgui/formatting.cpp` (`BookTextParser::next`, `parseTag`, `BookFormatter::markupToWidget`, `handleDiv`,
  `handleFont`), `components/misc/resourcehelpers.cpp` (`correctBookartPath`, `correctResourcePath`) and
  `components/fontloader/fontloader.cpp` (`getFontForFace`).
- Licence: GPL-3.0 (OpenMW: "GNU General Public License version 3"; the full text is the repository's `LICENSE`,
  identical to the GNU GPL v3 at https://www.gnu.org/licenses/gpl-3.0.txt). Holder: the OpenMW contributors.
- Ported into: `bevyrrowind/crates/mw_ui/src/book.rs` (the game; deviations in the module docs: `<FONT>` restyles inline
  instead of breaking the line, malformed tags are repaired instead of throwing).

## OpenMW (magic effects)

- Source: https://github.com/OpenMW/openmw, commit `ca5e2576bfeb8fa64b53fdc8a31fefc360decc2f`:
  `components/esm3/loadmgef.cpp` (HardcodedFlags, effect names, GMST ids, resistance and weakness maps),
  `apps/openmw/mwmechanics/spellutil.cpp` (`calcEffectCost`, `getTotalCost`, `calcSpellCost`),
  `apps/openmw/mwmechanics/spellresistance.cpp` (`getEffectResistanceAttribute`) and `apps/openmw/mwworld/esmstore.cpp`
  (effect-list validation).
- Licence: GPL-3.0, as for "OpenMW (book markup)". Holder: the OpenMW contributors.
- Ported into: `bevyrrowind/crates/mw_effects` (the game; written 2026-11-08, untested).

## OpenMW (barter and service prices)

- Source: https://github.com/OpenMW/openmw, commit `ca5e2576bfeb8fa64b53fdc8a31fefc360decc2f`:
  `apps/openmw/mwmechanics/mechanicsmanagerimp.cpp` (`getBarterOffer`, 563-590), `apps/openmw/mwgui/tradewindow.cpp`
  (38-114, 447-457, 608-645), `trainingwindow.cpp` (118-121, 172-191), `travelwindow.cpp` (50-83, 203-213),
  `spellbuyingwindow.cpp` (54-59), `merchantrepair.cpp` (64-83); GMST defaults from
  `apps/opencs/model/world/defaultgmsts.cpp`.
- Licence: GPL-3.0, as for "OpenMW (book markup)". Holder: the OpenMW contributors.
- Ported into: `bevyrrowind/crates/mw_rpg/src/services.rs` and GMST defaults in `bevyrrowind/crates/mw_stats/src/gmst.rs`
  (the game; written 2026-11-08, untested).

## OpenMW (MessageBox formatting)

- Source: https://github.com/OpenMW/openmw, commit `ca5e2576bfeb8fa64b53fdc8a31fefc360decc2f`:
  `components/misc/messageformatparser.cpp`, `components/interpreter/miscopcodes.hpp`,
  `components/compiler/lineparser.cpp`, `components/compiler/generator.cpp`; test vectors from
  `apps/openmw_tests/mwscript/testscripts.cpp`.
- Licence: GPL-3.0, as for "OpenMW (book markup)". Holder: the OpenMW contributors.
- Ported into: `bevyrrowind/crates/mw_ui/src/message.rs` (the game; written 2026-11-08, untested).
