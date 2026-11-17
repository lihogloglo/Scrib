# Surf and shore foam: how shipped games do it, and what we build (2026-11-17)

Research only. Scope: `water.md` "Shoreline and depth colour", `water_common.wesl` `shore_trains`,
`water_material.wesl` foam block, ledger W-GERSTNER / -DIAG / -DEPTH, owner checks WA-1 and WA-5.
Sources are listed at the end; claims marked (inferred) are not from a primary source.

## Summary and recommendation

**Two causes, one fix.** Our waves are tame for two reasons. (1) Physics caps them: crest ≤ ½·0.78·h, so a
1 m swell is only allowed to stand up at ~25 m from the shore on a 1:20 bed, right where the vertex LOD and the
`swell_ramp` (40 m) hide it. (2) Their shape is symmetric: a Gerstner crest with no skew, held loop-free
(Σ Q·k·A·coth ≤ 0.75). A real breaker is a steep-fronted sawtooth that leaves a white roller behind it. The
foam band is ugly because it is not tied to breaking: a fixed 6 m band with a sine, a fixed 0.8 albedo and
`max()` with a constant 0.7 contact foam.

**Recommended technique: "breaker-state foam and skew", analytic first, a small foam buffer second.**
Every industry system we could verify uses the same three ideas: a depth or shore field decides where waves
shoal and break (Crest, UE5 Water, Outerra, Horizon); the breaking wave has its own shape, not a taller sine
(Outerra skewed trochoid, Peachey 1986, Horizon's baked cross-section); foam is born from the breaking state and
then persists and fades (Crest foam sim, Atlas Jacobian inject + blur + fade, Sea of Thieves blur feedback).
We already have the field (shore distance, bed slope β, shoreward direction) and the trains, so the breaking
state is free: ratio `b = H/(γh)` per train, already implied by `env`/`breaker` in `shore_trains`.

Build slices, smallest first. Each one is one deliverable with one switch; judged by the owner in a
visual-check scene, never by screenshots (CONTRIBUTING). GPU ms are estimates for 1440p + DLSS Q on the 4060
laptop, to be measured by an interleaved A/B (`perf suite --arm`).

| # | Slice | What | Est. GPU | Switch |
|---|---|---|---|---|
| 1 | **SF-1 Breaker foam** | Replace the fixed band by bore foam from `b`, swash lace, reworked contact foam, new foam look | +0.03-0.06 ms (fragment ALU, 1-2 noise taps) | `--off surffoam` (old band) |
| 2 | **SF-2 Surf shape** | Break-point profile: skew + surf gain + steeper front + bore saw-tooth inside the surf zone | +0.02-0.05 ms (vertex, velocity redraw, fragment: one `pow`/`sin` per train) | `--off surfskew` |
| 3 | **SF-3 Foam buffer** | Camera-centred R8 foam texture: inject from SF-1's birth term, advect by swash/current, decay, so foam lingers and drifts on the sand and in the lee of objects | +0.05-0.10 ms (one 1024² pass + one tap per fragment) | `--off foamsim` |
| 4 | SF-4 (parked, owner's go) | True overhanging lip: a dedicated surf-strip mesh along the break line with baked cross-section deformation (Horizon style) | +0.1-0.3 ms, new mesh path | own switch |

Slices 1-3 together: about 0.10-0.21 ms, in budget (≤ 0.2 ms goal; slice 3 is the one to skip or shrink if
the sum lands high). Order: 1 first because it answers the owner's loudest complaint ("bad whitish thing")
and needs no new pass. Slice 2 gives the foam of slice 1 a wave to sit on. Slice 3 only if slice 1 shows the
foam still looks "painted on" (no memory); its injection term already exists then.

## Surf: breaking shape, asymmetry, amplitude, run-up

**What real waves do.** Waves shoal (Green's law, height ∝ h^-¼), steepen with a skewed, peaked crest and flat
trough, then break at H ≈ γh (γ 0.78 on a plane beach). The type follows the Iribarren number
ξ = β/√(H/L0): spilling ξ < 0.5, plunging 0.5-3.3, surging above. With our baked β 0.05-0.3 and 0.5-1 m swell,
most of our coast is plunging to spilling. Inside the surf zone the wave is a bore: a near-sawtooth with a
white roller, height about 0.4-0.78·h. So most of what the player sees on a beach is not tall waves, it is the
**roller and the steep front**. That is why foam and shape must be built together.

**What games do.**
- *Outerra (2011):* beach waves are a skewed trochoid (extra skew parameter, skew 1 = plain Gerstner). Several
  skews are precomputed in a small helper texture, indexed by depth; a time-varying mask stops waves running
  continuously all along the shore. Author's own caveat: real breaking needs more geometry.
- *Peachey 1986:* a phase function that gives refraction, plus a profile that changes with steepness and depth;
  targets a sloping beach. Fournier-Reeves 1986: Gerstner orbits that become elliptical in shallows, and can
  place breakers. We already have the phase function (Eckart closed form); the profile is what we lack.
- *Horizon Forbidden West (Malan, SIGGRAPH 2022):* one baked animated cross-section of a breaking wave (a
  deformation texture: rows = animation time, RGB = XYZ offset), swept along artist-drawn wavefront curves; an
  "animation parameter" (0.27 rising, 0.4 steepening, 0.5 curling and turning to foam) per point; a variation
  texture makes gaps in the wave; foam placement and strength come through vertex colour. Cost they report:
  tessellation never dense enough (the inner loop stretches triangles 2-3×; foam popped on the front face);
  future work is one vertex per pixel or splats. Wave particles (Yuksel) were their stated inspiration.
- *Crest (MIT):* says outright that realistic shoreline waves are an open problem; it only attenuates waves by
  depth (action below depth λ/2) and tells users to use spline Gerstner inputs.
- *Thürey et al. 2007 / Chentanez-Müller:* detect steep fronts in a shallow-water height field, spawn a particle
  sheet for the lip. Heavy, needs a sim, and US patent 8,204,725 covers it. **Avoid.**
- *Jeschke et al. 2020 ("Boundary-aware procedural waves"):* waves obey terrain boundaries, particles for
  breaking, foam, spray. Binary only, no source or licence: reference, not a base.

**Recommendation for us (SF-2), in order of cost:**
1. **Break-point lifecycle per train, from the existing `b`.** Three phases along shore distance: *shoaling*
   (b < 0.6: Green's law, symmetric-ish), *steepening* (0.6-1: skew grows, face steepens), *bore* (b ≥ 1:
   sawtooth, height set by the saturated surf zone ≈ 0.5-0.7·γ·h, decays to the swash). One scalar, `b`, drives
   the skew `s`, the foam birth (below) and the normal steepening. Today `env` soft-mins shoal and breaker, so
   `b` is `shoal/breaker` before the soft-min: free.
2. **Skewed phase.** Warp the label phase, `θ' = θ − s·sin θ`, and use θ' for both the height and the along
   (horizontal) term, so the surface stays a valid Lagrangian parametrisation (the swash already uses this
   warp: `SW_SKEW`). The CPU twin test must check the Jacobian stays positive except by design.
   Raise the loop cap from 0.75 to about 1.0 only inside the bore phase, where a vertical front is wanted.
3. **Surf gain.** The caps are physically right but look tame. Add a tunable `surf_height_gain` (Guess 1.4-1.8)
   on `Hs`, and let γ be tunable (0.78-1.0). The wave then stands up taller before breaking and the break
   line moves out and (with a larger H0) toward 15-25 m, where the owner's "rising over ~20 m" wish lives.
   Also a tunable minimum visible crest at the break line.
4. **Sets and gaps.** We already have world-space set noise; add Horizon's "gap" idea: along-crest break mask
   (noise on the crest-parallel coordinate) so a wave breaks in patches, not as a ruler line.
5. **Run-up.** Stockdon swash is in. Add: the uprush front is a thin bore (steep, lace foam), and the
   backwash leaves a thin film (darker wet band, already in `sf_surface`). The waterline must move with the
   swash phase so contact foam moves too.

**The curl.** A real overhang needs vertex spacing of ~5-10 cm at the break line. Our clipmap gives ~d/48 (0.4 m
at 20 m). Horizon, with a dedicated mesh and extra tessellation, still lists "not enough triangles" as its top
regret. So do not promise a tube from the clipmap. Fake the lip with a steep front, a foam-capped crest and a
dark hollow behind it (normal steepening plus the crest scatter boost we have), and park SF-4 (a surf strip
mesh + baked cross-section) for the owner's call. The Horizon cross-section is also a clean way to get an
art-directed curl later, which fits our "data/adapter-driven" rule (a deformation texture per coast type).

## Foam: sources, persistence, look, contact, whitecaps

**Sources** (each a separate birth term; foam = max or soft-add of all):
1. *Whitecaps (open sea).* FFT Jacobian < threshold (Tessendorf). Atlas: Jacobian per band, if above threshold
   inject "turbulent energy", dissipate by blur + fade, sum over bands at render, modulate a foam texture by the
   energy. This is what `displacement.w` is. Owner at wind 30 says the open sea looks nice but wants foam on
   crests: confirm the persistence/decay is really running (CONTRIBUTING: check a component runs before building
   on it) before changing it. Not part of SF-1.
2. *Breaking foam (new).* Birth = `smoothstep(0.85, 1.1, b) · break_mask`. Stateless version (SF-1): foam
   density at a fragment = birth at the crest × a trailing profile in phase behind the crest:
   `exp(−Δφ / bore_decay)`, with Δφ the phase distance behind the main crest (`gw.crest` is sin of the phase;
   use θ' directly). Crest front is dense and sharp; the tail is eroded by a noise threshold that rises with Δφ
   (lace and holes). This is the roller and its whitewater wake, one per train.
3. *Swash foam.* A thin lacy leading edge at the uprush front plus a residue that fades over the backwash,
   from the swash phase `theta0` (exists) and `swash`.
4. *Contact foam.* Existing snapshot-depth test, reworked (below).
5. *Wakes, objects, steep flow (later).* Crest/Sea of Thieves: depth-buffer comparison in a camera-centred
   window plus feedback blur; Crest input shaders (add from texture, vertex colour). Same buffer as SF-3.

**Persistence and advection.** Three levels, cheapest first:
- *Stateless trail (SF-1).* The trains are analytic in (x, t), so "foam born at the crest and left behind" is a
  function of phase behind the crest. No memory, no pass. Limits: foam cannot drift off the wave or sit on the
  wet sand after the wave has gone, and is always phase-locked.
- *Foam buffer (SF-3).* Crest's foam sim: one scrolling texture, each update `foam = foam·decay + births`,
  Crest default 30 updates/s, "Foam Fade Rate", wave foam strength/coverage, shoreline foam max depth
  (a depth threshold, not a distance), input shaders. Atlas and Sea of Thieves do the same with blur feedback.
  Ours: camera-centred R8 at 1024² over ~100-130 m (10-12 cm texels), snapped to texel steps so it does not
  swim; advect by the swash velocity and the water current (we have both); decay 4-10 s (Guess); 30 Hz update
  (a fixed tick, so a remote player sees the same state, net-ready; presentation only). Blur is the only
  cross-texel read, so use a 3×3 tap or a separable half.
- *Particles (a 300K GPU-particle foam renderer exists in one indie coastal renderer; texture advected by the
  flow field before).* Heavier, needs a particle pipeline. Not for us now; revisit only if SF-3 looks too smooth.

**Look.** The ugliness is in the shading, not only the mask:
- Fresh foam dense and bright (albedo ~0.85); aged foam thinner, translucent, tinted by the water body (albedo
  ~0.5-0.6, alpha-like blend to the body colour), so it reads as bubbles on water, not paint. Use age (phase
  behind crest, or buffer decay) as the control. The current `mix(body, 0.8, foam)` with a fixed 0.8 and
  `foam = 1` reads pure white in sun.
- Shape the mask with a noise threshold plus a feather (Crest "Foam Feather"; a hobbyist engine uses cellular
  noise thresholds with the wave-phase vector for density: dense toward the crest, gradual toward the trough).
  Noise advects with the wave (scrolls with the train) so the lace moves with the water, not with the screen.
- Foam normals from the foam value or noise gradient (Crest "Foam 3D lighting") so it catches light; roughness
  to 0.9 is already there.
- Under-foam bubbles: fresh breaking injects air, which lights the water body turquoise just under the roller
  (Crest "Foam Bubbles", the glow in a real breaker). Cheap: raise `body` scattering by the fresh-foam term.
- Naughty Dog (2012): foam as a threshold on a gradient field, plus a "churn" mix of depth colour modulated by a
  foam texture channel. Same idea: foam changes colour of the body too, not only a white overlay.

**Contact foam rework.** Keep the depth test (UE community recipe, Sea of Thieves, Crest all use depth
differences). Replace the constant 0.7 with: `vertical` depth in cm drives a noise threshold (foam only where
noise > vertical / contact), a bright thin line at the very edge, a soft lace outward, modulated by the swash
phase and by the local turbulence (`b`, swash speed), and with no hard max with the band. Thin lines at object
contact, not a halo. On a beach the waterline is the swash edge: one coherent system with source 3.

**Whitecaps.** Keep Jacobian + persistence for the FFT. For the Gerstner shore zone, whitecaps are the bore
foam of source 2, so no second system.

## Reuse check (Bevy, crates, open source)

- **Bevy 0.20:** released 2026-10-08; its release notes mention no water, foam or wave feature (checked the
  news page and release-notes search). Solari is the headline. Nothing to reuse.
- **Crates:** `bevy_water` (Bevy ≤ 0.16): open-sea waves, no shore model. `bevy-aqua` 0.1.x (Bevy 0.19): FFT,
  persistent whitecaps, shoreline foam, shallow optics; we already port its spectrum and coastal foam (entry in
  `THIRD_PARTY_NOTICES.md`), but it has no breaking or shore-directed trains. No other maintained Bevy crate.
- **Open source elsewhere:** Crest (Unity, **MIT**, verified): foam sim, depth cache, dynamic waves; useful as a
  *design* reference (parameters, sim frequency, fade rate), nothing to link. Godot ocean addons (FFT, foam from
  Jacobian) and the owner's own Godotwind swash (MIT) are mechanism-only prior art already used. OpenMW has no
  water shore model. Jeschke 2020: binary only; Thürey 2007: patented; wave particles: academic, not needed.
- **Verdict:** build our own, as small additions in our existing shore trains and fragment foam block; no new
  dependency, no licence issue.

## Industry comparison

| System | Breaking shape | Foam source and memory | Cost / notes |
|---|---|---|---|
| Horizon FW (Decima) | Baked animated cross-section, swept along artist wavefronts; variation texture | Vertex colour from the break parameter; wished for Weta-style bubbles | Mesh density the limit; per-coast authoring |
| Crest (Unity) | None built in (open problem); depth attenuation; spline Gerstner | Foam sim buffer: pinched crests + shallow depth; fade rate; input shaders | 30 Hz sim texture; MIT |
| Sea of Thieves | Stylised FFT; calm vs stormy | Peak mask from FFT chop, depth comparison around objects, blur feedback, artist textures | Stylised, cheap |
| Atlas | FFT bands, no shore break | Jacobian per band: inject, blur + fade, sum; wakes via eWave grid | Wind sea 0.5 ms (2080) |
| Uncharted (2012) | Artist big waves | Threshold on a gradient field; flow-advected; churn mix | Authored flow grid |
| Outerra | Skewed trochoid by depth (helper texture), time mask | Foam from the same | Cheap; they list "needs geometry" |
| UE5 Water | Wave attenuation by depth/terrain curve | Foam from water-info / distance field; community recipes | Limited out of the box |
| AC IV Black Flag | (inferred) wave sets + foam sim | Foam sim controls for sand vs rock; ship wake displacement | No primary source found |
| RDR2 / Ghost of Tsushima / Cyberpunk | No primary source found | No primary source found | Not claimed |
| **Ours now** | Symmetric Gerstner, breaker-capped | Fixed 6 m band, constant contact foam | ~0 |
| **Ours after SF-1..3** | Skewed, break lifecycle, taller; lip faked | Born at breaking, trailing, lace, buffer | ≤ ~0.2 ms |

Gap to log in `industry-audit.md`: nobody ships an overhanging tube from a plain clipmap; Horizon needed a baked
cross-section and finer tessellation.

## Net-readiness and tunables

Presentation only: inputs are the baked shore field, β, wind/sea state and the shared time; the foam buffer
steps on a fixed tick and never feeds gameplay. Remote players see the same analytic waves. Declare as tunables
(`tunables.md`, source `Guess` allowed): `surf_height_gain`, `breaker_gamma`, `skew_max`, `bore_decay_phase`,
`break_gap_noise`, `foam_albedo_fresh/aged`, `foam_noise_scale`, `foam_feather`, `contact_depth_cm`,
`foam_decay_s`, `foam_buffer_res`, `foam_update_hz`.

## What each slice needs

- **SF-1:** `b` exported from `shore_trains` (ShoreWave gets `bore: vec2(b, phase-behind)` and a swash-edge term);
  foam block rewrite in `water_material.wesl`; a tileable cellular noise (256² R8; check for an existing foam
  or noise texture first); CPU twin unchanged. Test: exact asserts on `b` monotone with depth; sanity on foam in
  [0, 1]. Scene: `view` of a beach cell at wind 7 / 14 / 30, camera at the waterline and from the hill.
- **SF-2:** profile function in `shore_trains` and its CPU twin `sf_water::shore_waves`; the existing Jacobian /
  no-fold tests extended (positive Jacobian except designed fronts); tunables; ledger line with crest heights
  per β (the W-GERSTNER-DEPTH table, re-measured).
- **SF-3:** one compute or fragment pass in `sf_water_bevy`, an R8 texture, scroll by texel snapping, swash +
  current advection; a "foam buffer" row in the water wiki page; A/B per CONTRIBUTING (untraced, 3+ runs,
  interleaved).

## Sources

- Malan, "Rendering Water in Horizon Forbidden West", SIGGRAPH 2022 Advances: https://advances.realtimerendering.com/s2022/SIGGRAPH2022-Advances-Water-Malan.pdf
- Crest docs, shorelines and shallows: https://crest.readthedocs.io/en/stable/user/shallows-and-shorelines.html
- Crest docs, water appearance (foam simulation and shading): https://crest.readthedocs.io/en/4.17/user/water-appearance.html
- Crest licence (MIT): https://raw.githubusercontent.com/wave-harmonic/crest/master/LICENSE
- Atlas, GDC 2019 (Mihelich, Tcheblokov): https://gpuopen.com/gdc-presentations/2019/gdc-2019-agtd6-interactive-water-simulation-in-atlas.pdf
- Ang et al., "The Technical Art of Sea of Thieves", SIGGRAPH 2018 Talks: https://history.siggraph.org/wp-content/uploads/2022/09/2018-Talks-Ang_The-Technical-Art-of-Sea-of-Thieves.pdf
- Gonzalez-Ochoa, "Water Technology of Uncharted", GDC 2012: https://www.slideshare.net/slideshow/water-technologyofunchartedgdc2012/12977478
- Outerra, "Ocean rendering" (2011): https://outerra.blogspot.com/2011/02/ocean-rendering.html
- Peachey, "Modeling Waves and Surf", SIGGRAPH 1986: https://history.siggraph.org/?p=116024
- Fournier and Reeves, "A Simple Model of Ocean Waves", SIGGRAPH 1986: https://history.siggraph.org/?p=116025
- Yuksel, House, Keyser, "Wave Particles", 2007: https://cemyuksel.com/research/waveparticles/
- Jeschke et al., "Water Surface Wavelets", 2018: https://pub.ista.ac.at/group_wojtan/projects/2018_Jeschke_WaterSurfaceWavelets/WaterSurfaceWavelets.pdf
- Jeschke et al., "Making Procedural Water Waves Boundary-aware", 2020: https://visualcomputing.ist.ac.at/publications/2020/MPWWBa
- Thürey et al., "Real-time breaking waves for shallow water simulations", 2007 (patent US 8,204,725): https://matthias-research.github.io/pages/publications/breakingWaves.pdf
- Frozen Fractal, "Making better waves" (foam from wave phase, cellular noise): https://frozenfractal.com/blog/2026/3/27/around-the-world-30-making-better-waves/
- Coastal renderer with GPU-particle foam (80.lv): https://80.lv/articles/real-time-shoreline-simulation-in-custom-c-coastal-renderer
- bevy-aqua: https://docs.rs/crate/bevy-aqua/0.1.1 ; bevy_water: https://github.com/Neopallium/bevy_water
- Bevy 0.20 release: https://bevy.org/news/bevy-0-20/
- UE community notes on Water foam (inverted foam, depth-difference mask): https://forums.unrealengine.com/t/foam-shore-foam-in-4-26-water-system/154611
- Tessendorf, "Simulating Ocean Water" (2001); Stockdon et al. 2006; Battjes 1974 (already cited in `water.md`).

Not found despite searching: primary technical sources for shore foam or breakers in AC IV, RDR2, Ghost of
Tsushima, Cyberpunk, UE5 Water internals. Those rows are not claimed.
