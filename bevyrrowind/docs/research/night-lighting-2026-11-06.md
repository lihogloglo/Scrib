# Night lighting: why the sheen, and what others do (2026-11-06)

Research only. "[V]" = verified this session from the linked source; "[U]" = from memory, unverified.

## Why we see a sheen

Our night ambient (4 nits, bluish) is applied by `pbr_ambient.wesl` to diffuse AND specular. Specular ambient with no
visible source is a uniform "reflection of nothing", so every smooth surface glows blue under a black sky. The sky and
the ground disagree: the ground is lit by a light the sky does not contain.

## 1. OpenMW

- Weather colours are keyed per time of day: `TimeOfDayInterpolator<T>::getValue(hour, timeSettings, prefix)` blends
  sunrise / day / sunset / night values with prefixes "Sky", "Ambient", "Fog", "Sun", "Stars". Night value is returned
  before `nightEnd - preSunrise` and after `nightStart + postSunset`. Sky, fog, ambient and sun colours each have their
  own Morrowind.ini night colour; the sky and the ambient are authored together, not derived from one another. [V]
  ([weather.cpp](https://github.com/OpenMW/openmw/blob/master/apps/openmw/mwworld/weather.cpp), master; commit not pinned,
  cite the SHA before porting anything)
- Sun at night: `sunDisable()` at/after `mNightStart` and before sunrise; `getSunVisibility` returns 0. The sun colour is
  still interpolated, and the sun orbit continues below the horizon (`orbit = 2t-1` at night) so the light direction
  keeps moving. There is NO moonlight directional light: moons (Masser, Secunda) are sky sprites only, with phase and
  alpha. [V]
- Renderer: `RenderingManager::setAmbientColour` stores the weather ambient; `updateAmbient` adds a grey `0.7 *
  nightEyeFactor` (the Night Eye spell). Sun light diffuse/specular come from `setSunColour(diffuse, specular, sunVis)`;
  specular alpha is scaled by sunVis, so specular from the sun vanishes at night. [V]
  ([renderingmanager.cpp](https://github.com/OpenMW/openmw/blob/master/apps/openmw/mwrender/renderingmanager.cpp))
- Shading: fixed-function-style Blinn/Phong. Ambient is multiplied by the ambient material colour and added to
  diffuse only; specular uses the sun/lights' specular colour times material shininess (alpha of a specular map, or
  material). No ambient specular exists. [V for objects.frag structure]
  ([objects.frag](https://github.com/OpenMW/openmw/blob/master/files/shaders/compatibility/objects.frag))
- Conclusion: OpenMW at night = diffuse-only coloured ambient (no sheen possible) + vanilla's bluish fog/sky colours
  chosen to be in the same hue family. The sky colour is dark but not black ("Sky Night Color" is a dark blue), stars
  fade in, moons are visible. Ground and sky agree because the same hue is used and fog fades distance to the sky colour.

## 2. Other engines and games

- **Ambient is diffuse-only or sky-derived, never a fake specular.** In PBR engines ambient comes from a sky capture
  (UE SkyLight with real-time capture, Unity HDRP sky reflection probe, Frostbite sky/light probes); its specular is
  that capture's convolution, so it reflects the actual sky colour. Where there is no visible blue sky, there is no blue
  sheen. [U] (Lagarde and de Rousiers, "Moving Frostbite to PBR", SIGGRAPH 2014:
  [course notes](https://www.ea.com/frostbite/news/moving-frostbite-to-physically-based-rendering)) covers IBL, exposure
  and light units; I did not verify any night-specific content.
- **UE5:** moonlight is a second directional light assigned as "Atmosphere Sun Light Index 1", offset 180 degrees from the
  sun; a SkyLight with real-time capture then carries the moon-lit sky glow to ambient. Community practice confirms this
  setup and its issues (moon disc glow, stars through clouds). [V]
  ([forum thread](https://forums.unrealengine.com/t/how-do-you-light-a-night-time-exterior-scene-with-directional-moonlight-for-a-day-night-cycle-system-in-ue5/242443))
  Night readability then comes from auto-exposure min/max and Local Exposure. [U]
- **Unity HDRP:** physically-based sky with moon as a second directional light, sky is a visible star/space cubemap,
  exposure curve with a scene-dependent compensation; HDRP has no scotopic mode by default. [U]
- **Skyrim / Creation Engine, Witcher 3, Elden Ring, RDR2, Ghost of Tsushima:** all [U], not verified.
  - Skyrim: directional ambient (hemisphere/cube ambient with up/down colours) is diffuse-only; night is lit by a
    weather-defined sun-colour moonlight directional light plus bright moons and stars. Nights are clearly readable and
    blue.
  - Witcher 3 / RDR2 / Tsushima: moon directional light plus a sky-derived ambient; the sky is never black (moon glow,
    airglow, stars, a blue-shifted tint). RDR2 and Tsushima shift the grade blue at night. Elden Ring: strongly
    artistic night grading, moon-coloured fill, visible sky.
  - Common to all of them: a visible, non-black night sky whose hue matches the fill light.
- **Scotopic / Purkinje:** a blue-shifted, desaturated, brighter grade for low luminance. Used as a post-grade in
  many games and film "day for night" practice; in engines it is usually a tone-map/LUT blend keyed on exposure, not
  physical. [U]

## 3. Recommendation for Bevy 0.20 raster

Options and verdicts:

- (a) **Diffuse-only ambient.** What OpenMW and Skyrim do, and kills the sheen completely. Bevy lacks the switch; needs a
  small vendored patch to `pbr_ambient.wesl` (skip the specular `F0*A+B` term for the flat ambient, keep it for env
  maps), consistent with "patch Bevy, don't inject". Cost: zero GPU, one shader edit. Con: nothing reflects at night, so
  wet surfaces and metals look dead unless a moon/sky provides specular.
- (b) **Glowing night sky** (moonlit airglow gradient, stars, bluish horizon matching the ambient hue, fog colour = sky
  horizon). Makes the owner's discrepancy go away at its root, and then an env map has something to reflect. Cost: sky
  shader tweak, nearly free. Con alone: if the fill stays a fake uniform specular it is still a uniform sheen.
- (c) **Moon directional light** carrying the light (Masser/Secunda, phase and colour). Gives directional shape and real
  specular highlights from a visible source (a moon highlight on wet ground reads as correct). Cost: one extra
  directional light; shadows optional (low-res/off for the moon). Con: it needs a visible moon in the sky and must be
  bright enough (about 0.05 nit now, a tuned few tenths to 1 nit with exposure opening); ties to the exposure range.
- (d) **Scotopic/Purkinje shift** in the tonemap: blue-shift and desaturate below a luminance. Cheap (a LUT or a few ALU
  in the tonemap). It supplies the "bluish" mood from the grade, not from fake specular. Con: it is a look, so owner
  sign-off needed.
- (e) **Combination (recommended).** Together they match what shipped titles do [U]:
  1. (a) Patch the flat ambient to diffuse-only. This is the actual fix for the sheen and costs nothing. Keep specular
     occlusion/IBL for the env map only.
  2. (b) Raise night sky to a dark-blue moonlit gradient plus stars, with the horizon and fog colour matching the
     weather's "Sky/Fog Night Color" and the ambient hue (OpenMW's data already has these authored together).
     Env-map capture then reflects this sky, so any remaining specular is honest.
  3. (c) Add the moon(s) as a directional light that carries a share of the readable light (phase scales it), with soft
     or no shadows. Masser/Secunda should light the world in a modern take: it gives direction, silhouettes and real
     highlights, and lets the flat ambient shrink. OpenMW does not do this (sprites only); Skyrim, Witcher 3 and UE
     practice do. Keep a fallback lerp so a new moon phase never goes fully dark (owner prefers lit nights).
  4. (d) Optional later: a mild Purkinje blue-shift in the tonemap for the mood, as a visual-check item.

Stepwise order (cheapest and safest first): (a) alone should remove the sheen; (b) and (c) then make it feel
intentional; (d) is the optional polish. All items are within the 11 ms budget (a, b, d are effectively free; c is one
light without shadows).

Open question for the owner: after (a), nights will be flatter and less "readable" by the specular lift; is a brighter
moon (c) with phases acceptable as the replacement lift?

## Not verified / gaps
- Pinned OpenMW commit SHA (read from master via raw file; sandbox had no git checkout).
- All Skyrim, Witcher 3, RDR2, Tsushima, Elden Ring, HDRP and Frostbite night details are from memory.
- No Bevy source check of `pbr_ambient.wesl` was done here (taken from the brief).
