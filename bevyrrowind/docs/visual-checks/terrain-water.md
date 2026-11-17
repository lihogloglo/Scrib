# Visual checks: terrain-water

Open visual checks: terrain and water (TR-*, WA-*). Index and how-to: [`../visual-checks.md`](../visual-checks.md).

## Haze and dust (superfargoth `weather/distant-shadows.md` "Aerial perspective and haze" N1/N2; ROADMAP M6 F1; `.sessions/m6-f1.md`)

The `--clouds` weather now also sets the atmosphere's aerosol haze and an absorbing dust/ash term (table in
`weather-adapter.md` "Haze and dust"); `--haze <x>` and `--dust <x>` fix them for A/B. The C key's genera use the
fair-weather look. Near fog, ground fog banks and god rays are F2/F3, not this check.

### WA-1: Open sea, W2 FFT ocean (`scene wa-seyda-noon`, `wa-vivec-waterfront`, `wa-bittercoast`, `wa-sunset-sea`, `wa-night-moons`; A/B `--water scaffold`, `--water off`, `--water-wind 2` / `14`; example `water_ocean`)

Status: partial, owner 2026-10-19 (round 12): one sea, no seams, nice transparency and glitter. Issues: (1) moiré in the distance — fixed 2026-10-28 (render-opt: grazing-angle slope mip footprint + full horizon normal-to-roughness variance transfer in `water_material.wesl`), awaiting owner; (2) shore foam band artificial, near pure white, fixed width, 'offendingly ugly'; (3) shore damping wrong: water goes 100 % flat in the last centimetres; want swash (water moving up and down at the shore), wavelets over the first ~20 m rising gradually to full open-sea waves; (4) unsure wave height follows the weather (spec: h0 blend from WindState; verify); (5) no whitecaps on wave crests (spec: Jacobian foam, not visible); (6) owner asks for crest subsurface scattering (light through wave tips). (2)-(6) Phase 4.
Look for: waves read as one sea (swell + chop) and move; sand visible through shallow water, colour deepening
offshore; a thin contact-foam line where water meets sand, rocks, piers and hulls; sun glitter widening with distance,
no sparkle noise; far water hazed like the land (aerial perspective, morning froxel fog); no square seams, cracks or
steps (clipmap rings) and no gap at the horizon; a sunset path on the water; at night moon glints, sea not black.
Known: reflections are the environment map only (SSPR/probe at W4), no shore wave damping yet (W3), the water may read
too milky turquoise (W1 placeholder optics, tuned at W3), whitecaps at storm winds untuned.
- Owner 2026-10-30 (`scene wa-seyda-noon`): distant moiré gone (fixed). Still missing: rain ripples on the water and
  ripples from moving bodies (Phase 4 water polish / PR2 splashes).
- Owner 2026-11-12 (Seyda Neen thunderstorm): (2) shore foam still bad, (3) water fully flat next to the shore: wants
  wavelets from the normals even where vertex displacement stops (fix W-SHORE, check WA-4). Code: the shading normal uses the same shore scales as
  the displacement (`water_material.wesl:207-209`, `shore_scales` chop -> 0 at the line, `water_common.wesl:52`). Rain
  goes over the water but makes no ripples; floating or wading bodies make none either.

### WA-2: Shore and region optics, W3 (`scene wa-seyda-noon`, `wa-bittercoast`, `wa-vivec-waterfront`, `wa-sunset-sea`, `wa-night-moons`)

Status: **region colour confirmed by owner (2026-10-18)**: muddy water at Seyda Neen, bluish toward Suran.
**Unfinished (owner 2026-10-18):** at `wa-vivec-waterfront` the shore foam is a uniform white band everywhere
(not thin and broken); at `wa-sunset-sea` the swell is very small, even with `--water-wind 14` and `20`. Parked:
no more work on these now (owner); pick up with the foam/shore and weather-driven waves work (SU1, WE3). Shots: `shots/w3/` (compare with `shots/w2/`).
Addresses the owner's 6a notes: water no longer milky turquoise (body colour sized like real backscatter, 2-5 %);
colour by region from the cell map (Seyda Neen / Bitter Coast swamp olive-brown, Ascadian lagoons teal, open Inner Sea
deep blue, ash grey-brown, the north steel blue; ≈ 117 m blend between cells); shore foam per region and fainter; waves
calm toward shores and in enclosed water (`water.shore`: signed shore distance + openness, baked at start in ~0.5 s).
Look for: the hue changes between regions without a visible seam; swamp water murky and calm near Seyda Neen; the
shore line is a thin broken foam band, not a white outline; swell dies on beaches, open sea keeps it; no water drawn
under land. Known: from above (`wa-bittercoast`) the swamp reads grey (sky reflection dominates at that angle; SSPR at
W4); at night the water reads lighter than the land (env-map reflection, W4); the shore foam band shape is still the
generic travelling band (wet sand band waits for SU1); waves by weather are WE3.



### WA-5: Gerstner shore waves (W-GERSTNER; `scene wa-seyda-noon`, `wa-bittercoast`; A/B `--off shorewaves`)

Owner 2026-11-12 (`scene wa-seyda-noon`): hard to judge, the waves are never big; asked whether waves follow the weather.
They don't yet: the ocean wind is a fixed 7 m/s scaffold (`mw_environment_bevy/src/water.rs:72` `DEFAULT_WIND`, WA-1 item 4),
and near the shore the 1:20 stand-in slope caps breakers at ~0.15-0.18 m. Owner 2026-11-12 (`--water-wind 14`, then `30`): the open sea responds
and looks nice at 30 (wants whitecap foam on crests, and contact foam around objects reworked); the Gerstner shore waves
are not visible at all (owner suspects the damping). Status: failed so far (not visible); diagnosis W-GERSTNER-DIAG. Status: awaiting owner. Kind: real game locations, High preset, the scenes' own time and weather; windy variant `--water-wind 14`.
Launch: `bash bevyrrowind/tools/owner_launch.sh scene wa-seyda-noon`, then `scene wa-bittercoast`, then `scene wa-seyda-noon --water-wind 14`;
A/B: the same with `--off shorewaves` (the FFT alone fades to flat at the shore). Answers WA-1 swash wish.
Look for: waves rolling in parallel to the beach and washing up and back over the sand (a few metres of run-up, faster up
than down), rising from ripples at the waterline to the open swell over the last ~40 m, crests bending and sets varying
along the shore, foam riding the breaking crests; stronger swash with `--water-wind 14`. Bad: conveyor-belt regularity,
loops or spikes on crests, a seam where they meet the FFT swell, smearing when the camera moves (motion vectors), water
showing through the beach behind the swash, W-SHORE ripples gone.
Fix W-GERSTNER-DEPTH (2026-11-13): the shore waves shoal over the real bed slope baked from `LAND` (was a 1:20 proxy that
capped crests at ≤ 0.19 m). Rerun `scene wa-seyda-noon`, then `--water-wind 14` and `--water-wind 30`, then `scene wa-bittercoast`;
A/B each with `--off shoredepth` (the old proxy). Log line to check: `water: shore bed slope in the wave band p10 .. p50 .. p90 ..`.
Look for: visible crests rolling in (tens of cm at 14 m/s, more on steep shores, gentler on flats), longer shallow wavelengths
(~20-30 m), swash several times stronger on steep beaches. Bad: water flooding over the beach far past the waterline (storm
swash reaches ±2 m on β 0.2 at 30 m/s), crest loops/spikes, crests kinking where the bed changes. Status: awaiting owner.
- Owner 2026-11-15 (`scene wa-seyda-noon --water-wind 14`, after W-GERSTNER-DEPTH): shore waves are there now but faint, a little bulge, not a shore-wave shape; crests do not bend; open-sea waves small even at 14 m/s (owner: too little wind or a wrong sim factor; compare the 2026-11-12 "nice at 30"); contact foam on terrain/objects still looks terrible (WA-1 foam item); FPS very low in this scene (not measured: needs a `--bench` run before any guess); no crash. Status: partial.

### WA-6: The sea follows the weather (W-WIND; `scene wa-seyda-noon`; A/B `--off seawind`)

Status: partial (owner 2026-11-13, `scene wa-seyda-noon`, clear -> thunderstorm over BRP, clock running): the change is too tame to
  see, "waves are still super tame" in a storm (wind map 0-1 -> 1-15 m/s; `--water-wind 30` looked nice, so storms need a
  stronger sea state, or a scale problem). New bug: a white band animated toward the shore in the calmest water
  (rivers, lakes): the shore foam / shore-wave bands are not gated off for enclosed water.
Launch: `bash bevyrrowind/tools/owner_launch.sh scene wa-seyda-noon`; then `bevyrrowind/set {"weather":"thunderstorm","transition_hours":0.2}`
(add `"time_scale":120` to see it sooner), wait ~1-2 min, then `bevyrrowind/set {"weather":"clear","transition_hours":0.2}`.
The sea lags the weather by design (30 s low-pass + 6 s steps). A/B: the same with `--off seawind` (fixed 7 m/s sea).
Look for: a calm, small-waved sea in clear weather; the sea roughening smoothly into the storm (longer, higher swell,
more whitecaps, stronger shore swash) and calming back the same way. Bad: a pop or a re-roll of the wave pattern at any
moment, waves jumping at the shore, floating things (CPU water) bobbing out of step with the drawn waves.
- Owner 2026-11-15 (repeated): the white bands on inland water shores (rivers, lakes) are still there.

### WA-7: Water chain gated by visible water (CW-10a; `view "Balmora"`; switch `water.gate.visible`)

Status: awaiting owner (built 2026-11-17, cw10a-water; not measured).
- Owner 2026-11-17 (dev build, `set water.gate.visible on`): nothing broke, no visible change (expected: it only skips passes). The
  F3 GPU water-pass numbers could not be read: the F3 panel shows no GPU data at all (logged, rendering). Status: look
  confirmed (no regression); perf not seen.
Launch: release build, `bash bevyrrowind/tools/owner_launch.sh view "Balmora"` at noon, F3 GPU passes open. In a street
facing away from the Odai: note `water_velocity`, `water_sspr`, `water_snapshot`, then `set water.gate.visible on`: they should
drop to ~0 (a small `water_gate` probe appears; `water_fft` stays unless you look at the sky or straight down).
Then walk to the river bank: all passes back, same values as off. Turn toward and away from the water repeatedly, and walk
out of a street onto the bank. Look for: no pop, flicker or one-frame flat/frozen water when it comes into view; reflections
(SSPR) and refraction valid on the first frame; no smear behind waves (DLSS motion) on re-entry; foam unchanged. A/B: `set water.gate.visible off`.

### WA-8: Shore foam follows the wave energy, none on calm inland water (W-FOAM-GATE; `view "Balmora"`, `scene wa-seyda-noon`)

Status: awaiting owner (built 2026-11-17, water-inland). Fixes the WA-6 white band on rivers and lakes.
Launch: `bash bevyrrowind/tools/owner_launch.sh view "Balmora"` (the Odai banks at noon), then `scene wa-seyda-noon`.
Look for: on the Odai and inland pools no white band creeping toward the bank; the largest lakes at most a faint band; on the
Seyda Neen beach the band still rides the breaking crests and pulses with the sets (a little weaker than before on small
sets: if too weak, `ShoreSettings::foam_full_breaking` 0.6 goes down). Contact foam and the band's look are unchanged
(the surf + foam rework comes after the research pass).

### WA-9: Surf: breaker foam (SF-1) and breaking wave shape (SF-2) (`scene wa-seyda-noon --water-wind 14`; A/B `--off surffoam`, `--off surfskew`)

Status: failed (owner 2026-11-17, profiling build, `--water-wind 14`): (1) no surf, the water rises and falls gently with a Gerstner bulge (Seyda Neen likely too gentle: SF-2b + a steeper test beach); (2) no patchy breaking seen; (3) foam "frankly quite bad": looks painted, wrong scale; no turquoise glow seen; Seyda Neen water reads murky green; (4) no wet sand trace at the waterline (the swash lace is on the water; the terrain wet band is not built); (5) contact foam = the painted, wrong-scale one. Owner: "I have seen projects doing foam quite well": research the foam LOOK (textured foam), not only where it is born.
Launch: `bash bevyrrowind/tools/owner_launch.sh scene wa-seyda-noon --water-wind 14`, then `--water-wind 30`; A/B the same with `--off surffoam,surfskew`.
Look for: waves steepening with a forward-leaning front as they come in, breaking in patches (not one ruler line), dense white foam on the breaking crest fading behind it into lace with holes, aged foam greyish and see-through, a faint turquoise glow under the breaking crest; lacy foam at the waterline on the uprush; thin foam lines (not halos) at rocks and piers. Bad: white paint bands, foam that slides over still water, spikes or loops at the crest, shimmering lace far away.
Known: on the gentle 1:20 parts the waves stay small (they are faded where they break; SF-2b next).

### WA-10: Storm sea, underwater post, rain rings (W-SEASTATE, U3, R1; `scene wa-seyda-noon --weather thunderstorm`, profiling build)

Status: failed (owner 2026-11-17, first launch): (1) storm sea still not big, "definitely not 2 m"; (2) no rain rings on the
water at all; (3) rain falls underwater; (4) underwater far too murky: visibility a few tens of cm; (5) a gap between the
head going under and the effect starting (Split state draws nothing until all near-plane corners are under; U4 not built);
(6) no waterline "middle line" (U4, expected, owner wants it). Next: one read-only diagnosis of 1-4, then fixes + U4.
Recheck (owner 2026-11-17 22:15, fixes in): rain rings visible (the first launch may have been hidden by wavelets); no rain
underwater, pass. Colour regression: the Bitter Coast fix made the sea bluer, but murky green is right for Seyda Neen (bluer
elsewhere, as before); only the visibility range was wrong. Next: give Bitter Coast its green tint back with coastal
visibility. Storm sea still too meek, with no big crest foam and no sea spray: that goes to a separate storm-sea session
inspired by Sea of Thieves. Open: U4 waterline.
