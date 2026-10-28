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
  (github.com/PVCPC/delirium-physics, GPL-3.0-only) as a Cargo git dependency.

## OpenMW Clockwork (sunsoft directional PCSS)

- Source: https://github.com/OursCodeur/openmw branch `clockwork-alpha`, commit `c5e5a9a6362c35512823884753d2d5284af204fa`:
  `files/shaders/lib/light/sunsoft.glsl` (author: Thomas Portal).
- Licence: GPL-3.0-or-later (the full text is identical to the GNU GPL v3 at https://www.gnu.org/licenses/gpl-3.0.txt).
- Ported into: `superfargoth/vendor/bevy_pbr/src/render/shadow_sampling.wesl` (`sample_shadow_map_directional_pcss`, H2 world-unit sun PCSS with analytic receiver slope and world-meter penumbra scale across cascades), combined with the rotated blocker search kernel from `bevyengine/bevy#22272`.
