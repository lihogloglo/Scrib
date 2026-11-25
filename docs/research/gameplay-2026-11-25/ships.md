# Ships: Sea of Thieves-style crewed vessels, buoyancy, hull masking, sea effects (research, 2026-11-25)

Research only (no code). It feeds a new framework spec and amendments to `water.md`, `interaction.md` (c) and
`docs/roadmaps/travel.md`. Godotwind is an anecdote here, never a design (CONTRIBUTING rule 3).

## 1. Summary

- **What to build:** a generic **crewed vessel**. Activators (wheel, sail-length cleats, sail-angle pulleys, capstan,
  bucket, oars) set a few control parameters. A Bevy-free model turns those parameters plus wind and water into forces
  on one rapier dynamic body. Characters walk the deck with based movement (IN20/IN11) and push no force on it.
- **Order:** (1) spec. (2) Water exclusion volumes in the CPU `WaterQuery`, then in the water draw: no water inside any
  hull, rowboats included. (3) Buoyancy 3.7 (pontoons, already specified), then hero-hull buoyancy (Kerner triangles).
  (4) The control model (sails, rudder, anchor), then the activators. (5) Loose objects resting on the deck. (6)
  Flooding and bailing, with the interior water drawn as a local body. (7) Hull effects: bow spray, wake foam.
- **Sea look:** the Sea of Thieves "light on the wave lip" is **not SSR**. Rare used no SSR, only planar reflections. The
  look comes from strong subsurface scattering on the wave sides plus Jacobian foam. Crest SSS, crest foam and crest spray
  belong to the separate storm sea lane. Ships add only hull-driven effects and share that lane's spray renderer.

## 2. Sea of Thieves: how ships work

**Main source.** SIGGRAPH 2018 Talks, *The Technical Art of Sea of Thieves*, by Nigel Ang, Andrew Catling, Francesco
Cifariello Ciardi and Valentine Kozin (Rare). DOI 10.1145/3214745.3214820. Record: https://history.siggraph.org/?p=84316.
Video: https://www.youtube.com/watch?v=y9BOz2dFZzs. Transcript: `Godotwind/.../docs/reference/transcript_sea_of_thieves.md`
(line numbers below are "T:n"). The art side is Ryan Stevenson's GDC 2018 talk (T:29). Rare gave no public talk on ship
physics or ship netcode. What follows is all the public record holds:

- **Controls → parameters.** Players "adjust the height of the sails, the rotation of the sails" through the rigging
  (T:251-253). The wiki lists the stations: sail-length cleats and sail-angle pulleys per mast, the wheel (helm), the
  capstan for the anchor, buckets, and planks (https://seaofthieves.wiki.gg/wiki/Ships). Speed depends on sail length
  and on the sail angle against the wind. The anchor must be up before sailing and stops the ship almost at once. More
  crew on the capstan raise the anchor faster (https://www.windowscentral.com/sea-thieves-ship-guide). AI ships compute
  "movement speed ... based on the potential it can achieve based on wind direction" (Tommy Thompson,
  https://www.gamedeveloper.com/design/how-megaladons-krakens-and-skeleton-ships-work-in-sea-of-thieves-part-3-of-4-).
- **Damage and water.** Hits open holes. Repeated hits grow a hole, and a larger hole floods faster and takes longer to
  fix. Water fills the lower deck, buckets bail it out, and the ship sinks when fully flooded. Planks repair the hull,
  wheel, masts and capstan (each takes 3 hits). Source: wiki, above.
- **Physics cost.** A Rare programmer said "the ship is one great big physics object containing lots of other little
  ones!" and called physics updates "one of the most expensive things we do"
  (https://wccftech.com/rare-big-ambitions-ai-sea-of-thieves/). The project ran on UE4 and was CPU- and memory-bound, so
  work moved to the GPU (T:36-46). The rope rigging runs on the GPU from a CPU-precomputed catenary (T:256-326).
- **Crew AI on the deck.** The ship's navmesh does not move. It sits at a fixed place, and "each coordinate ... is
  transposed ... onto the ships deck and hull", accounting for the ship's orientation (Thompson, above). This is
  IN22's deck-local nav.
- **Networking.** Waves are kept "synchronised for each player so everyone sees the same level of waves, particularly
  during naval battles" (MCV, 2016, https://mcvuk.com/?p=61071). No official source states the server authority model.
  A community post says physics and waves run server-side (unofficial). Our net-ready rules already give the answer: the
  host owns the ship state, and each client evaluates the waves itself (water.md "Determinism").
- **The deck.** Rare runs a two-pass shallow-water heightfield on the deck (512×1024), biased by the deck normal so
  water rolls as the ship rocks. Bow sensors that "trigger the bow waves when you crash into a wave" feed it. Cost: 0.2 ms
  per ship, run for one ship at a time (T:90-113).

## 3. Buoyancy and ship physics

**Canonical techniques.** (a) **Sample spheres** ("pontoons"): UE5's Buoyancy component uses spheres as "a simplistic
volumetric approximation ... a low-cost solution" (https://dev.epicgames.com/documentation/en-us/unreal-engine/water-buoyancy-component-in-unreal-engine).
Crest leaves boat physics to third-party assets (https://crest.readthedocs.io/en/stable/user/clipping.html). (b)
**Per-triangle hull clipping**: Jacques Kerner (Avalanche, Just Cause 3) clips the hull triangles at the waterline and
applies hydrostatic pressure per submerged triangle, plus viscous resistance, pressure drag and slamming. He targets
under 1 ms per boat across all sea states
(https://www.gamedeveloper.com/programming/water-interaction-model-for-boats-in-video-games and its Part 2). (c)
Voxelised hulls sit between the two. Shipped games use (a) for props and (b) for hero boats. Every one of them evaluates
the waves on the CPU from the same spectrum as the GPU, on the physics tick. GPU readback adds latency and stalls.
`bevy-aqua-query` reads back from the GPU with about one frame of latency and requires Bevy ^0.19
(https://docs.rs/crate/bevy-aqua-query/latest).

**What exists (water.md).** `WaterQuery::sample_many` is already shaped for pontoons. The CPU twin evaluates cascade 0
(cascade 1 when the sea is rough) on the 60 Hz physics step. It agrees with the GPU within 1 cm, and each sample costs
≤ 2 µs. Buoyancy is specified but **not built** (ROADMAP 3.7 `[ ]`): pontoons with a spherical-cap fraction, drag
against the water velocity, `add_force_at`, a `Floating` flag, a `water.pontoons` bake (1-8 spheres), and the Kerner
hull as a per-body flag at 0.02-0.05 ms per boat. Budget: 32 floaters ≤ 0.1 ms. Physics is rapier3d 0.36.1 behind
`sf_physics` (root `Cargo.toml:131`). No avian.

**Missing for ships:**
1. **Wavelength by hull size.** The twin drops the short cascades because "they don't move a boat" (water.md:141).
   That holds for a 30 m ship but not for a 3 m rowboat, which rides 4-16 m waves (cascades 1-2). WA-10 also logged a
   visible gap between the CPU surface and the drawn surface. The standard fix is a per-query minimum wavelength:
   Crest's height query takes a minimum spatial length, and HDRP has its per-surface "evaluate ripples" switch. A
   floater asks for the cascades whose waves are longer than about half its hull length.
2. **A force model for ships** beyond buoyancy: keel lateral resistance, sail thrust from the apparent wind (`sf_wind`
   minus the ship's velocity), rudder lift that grows with speed through the water, and the anchor (strong drag plus a
   bow pivot, which gives the SoT anchor turn). Pressure drag and slamming come from Kerner Part 2. These terms make a
   ship feel like a ship. Plain pontoons only make it float.
3. **Hull volume and mass** from a dedicated low-poly physics hull, not the render mesh. Flood water adds mass at the
   floodwater's centroid.

**Stability.** Buoyancy acts as a stiff spring. Apply it on the fixed tick with the wave state at that tick time, damp
heave, pitch and roll with linear plus quadratic terms, and keep the body awake while the sea moves (`Floating`). Ships
use the triangle hull. Debris and rowboats use 4-8 pontoons. Godotwind taught two lessons here. It had two parallel
implementations: box cells with per-cell gravity, and probes tuned with a non-physical "force multiplier" and power
curve. It also had a delta-scaling bug in `apply_force` (`docs/plans/ocean_quality_tracker_2026_05_07.md:70,497`). Its
synchronous GPU readback stalled the render device, so it moved to a CPU spectrum evaluator (`ocean_manager.gd:57-60`).
The most useful tool it built was a debug grid that draws the CPU surface over the mesh (`tests/visual/test_buoyancy_debug.gd`).
We want the same as a dev overlay.

## 4. Deck stability (no jitter)

**Technique.** UE's Character Movement Component replicates and simulates a character relative to its
`MovementBase`, with `bImpartBaseAngularVelocity` on leaving
(https://dev.epicgames.com/documentation/en-us/unreal-engine/understanding-networked-movement-in-the-character-movement-component-for-unreal-engine;
forum detail https://forums.unrealengine.com/t/use-relative-character-replication-when-attached-resolved/1913738).
Jolt's `CharacterVirtual` takes a ground velocity. Star Citizen gives each ship a local physics grid
(https://starcitizen.tools/Comm-Link:BTS_-_Multicrew_%26_Physics_Grids). Travel decision 4 already chose based
movement. The jitter comes from three places, and each one maps to a slice:
- **Mover vs deck timing** (IN11). The mover runs in `Update` at frame rate (locomotion.md:118), while the deck moves on
  the fixed tick. The mover must keep a base-local pose, apply the base's interpolated pose for the frame, collide in
  that frame, and leave with the point velocity. IN11's "30/144 fps alike" test checks this.
- **Drawing and camera** (IN21). Both are composed as `interp(ship pose) × base-local pose`, with the same
  interpolation alpha as the hull. Without that, a character drifts by a tick's motion against the deck.
- **No feedback.** Characters put no force on the deck (travel decision 3). The kinematic capsule must not contact the
  ship's collider as a body. The mover queries the ship's collider as static geometry in ship space.
- **Loose objects** (new, SHP-7). A crate resting on a rolling dynamic hull jitters through friction and the solver.
  Rule: a dynamic body whose ground is a vessel, with relative speed below ε for N ticks, becomes **deck-resting**. It
  is kinematically attached at a ship-local pose (no solver contacts) and is released when hit, when the deck tilt passes
  its friction angle (cargo slides in a storm), or when picked up. This is Star Citizen's local-grid idea at the cost of
  one transform. It also makes saves and replication base-relative for free (IN24).
- **Crew NPCs** (IN22). They path in ship-local space, as SoT's transposed navmesh does.

## 5. Hull interior water masking

**What others do:**
- **SoT:** no public source.
- **AC IV Black Flag:** a displacement mask pushes the water down under the ship, and an invisible "lid" is drawn "only
  into the depth buffer", so the water fails the depth test inside. A stencil variant exists. Commenters list its limit:
  it assumes a flat water level (https://simonschreibt.de/gat/black-flag-waterplane/).
- **Unity HDRP:** a Water Excluder, a simplified interior mesh childed to the boat, "marks pixels on screen that should
  not receive a water surface". It "only affects the water surface from above", and the issue tracker marked the
  waterline glitches "by design"
  (https://docs.unity3d.com/Packages/com.unity.render-pipelines.high-definition@15.0/manual/WaterSystem-waterexcluder.html).
- **Crest:** a clip-surface map (0-1, a hole above 0.5) fed by SDF primitives or by a vessel's **convex hull** rendered
  top-down. Overlapping meshes misbehave. *Portals & Volumes* also clip the underwater volume, "a window on a boat" (URL
  in §3).
- **UE5:** `WaterBodyExclusionVolume` makes "underwater zones where gameplay can act as if it was not under water"
  (https://dev.epicgames.com/documentation/unreal-engine/water-body-actors-in-unreal-engine). Its rendering exclusion
  is buggy (UE-170684: the surface stays visible).
- **Godotwind:** never masked hulls. Its stencil portal for interiors needed a stencil-read copy of every material and
  had depth-sorting issues (`docs/systems/interior_transitions.md:7,120-130`).

**What Bevy 0.20 gives us.** The main depth target is `Depth32Float` with no stencil
(`bevy_core_pipeline/src/core_3d/mod.rs:5`). A stencil mask would need a vendored patch of the core depth format
(rule 4 allows that) and a stencil write on every hull material. Bevy has no exclusion feature of its own.

**Recommendation: exclusion volumes tested per fragment in the water shader, and the same volumes in the CPU query.**
- Each vessel carries 1-8 convex volumes (oriented boxes or capsules, or convex plane sets) in ship space. Together they
  cover the dry air inside the hull, capped at the gunwale or deck. A bake (`hull.cook`) derives them from the physics
  hull: voxelise it, flood-fill from outside, take the cells under the cap plane, and fit convex pieces. Morrowind
  meshes have no authored excluders, and packs are data (memory: generic renderer).
- **GPU:** a small storage buffer of the volumes in view, culled to the screen. The water fragment shader tests its
  displaced world position first and discards if the position is inside a volume. The test does not depend on the
  camera, so it works from inside the hull, on any wave, and for a 3 m rowboat. Discard costs the water pass its
  early-Z, so measure it. If that cost shows, gate the test by a per-tile volume flag.
- **CPU:** `WaterQuery::sample` returns the vessel's interior body (its flood level), or no water, for points inside a
  volume. That one change keeps loose objects from floating inside a dry hold and keeps swimming off. It also gives the
  U1 medium `Above` in the hold below sea level (U1 already resolves "pool over sea"), and lets AI and footsteps follow.
  UE's `IsWorldLocationInExclusionVolume` is the same seam.
- **Other consumers** of the water-level clipmap must respect the volumes too: caustics and the wet band in the opaque
  hook, rain rings and splashes, and the froxel water medium.
- **Flooded hull:** the interior water is a local **pool-kind body** (water.md "Water bodies at any height") attached to
  the vessel. Its level comes from the flood model, it tilts with the ship, and it is clipped to the same volumes.
  Underwater inside a flooded hold, the U1 priority rule picks the interior body. Known v1 limit: looking out from a dry
  hold through a hole below the waterline shows the outer sea surface without underwater fog. That needs Crest-style
  volume-aware underwater, a later slice.

## 6. Sea effects

**SoT's recipe (T:53-89).** The waves are a plain Tessendorf FFT.
- **Subsurface scattering:** "we use the choppiness vertex offsets ... to generate a mask for where the sides of the
  waves are and then we just use a DOT product of the light and view vector" (T:65-67). They also boosted SSS beyond
  realistic (T:60).
- **Foam:** the Jacobian determinant, biased down to get more foam, is injected into a tiling render target and blurred
  progressively frame by frame. Sharp crests dissipate into soft foam, blended from a high-frequency texture at the
  crest to a low-frequency one (T:68-80).
- **Spray:** "we also use the same [crest] where we generate foam to generate some particles off the top of the waves".
  Storms just raise the amplitude and bias the Jacobian further (T:80-83).
- **Reflections:** no SSR, only planar reflections for the ocean (T:40-41).
- **Cost:** the surface draw is pixel-bound, near 8 ms on Xbox One when the sea fills the screen (T:84-89).
- **Wakes and intersection foam:** an intersection map from scene depth in a render-target window that travels with the
  player, progressively blurred, gives foam trails behind moving objects and around islands. Under 0.1 ms (T:114-141).

HDRP names the same terms: height or displacement scattering (more horizontal displacement, more scattering) and a
"direct light tip" term at wave tips
(https://docs.unity3d.com/Packages/com.unity.render-pipelines.high-definition@14.0/manual/WaterSystem-Properties.html).

**Ours today.**
- Whitecaps: Jacobian foam with decay (Dupuy-Bruneton).
- Shore and breaker foam: SF-1.
- Contact foam from scene depth.
- Reflections: SSPR (confirmed WA-3) plus a planned dynamic probe. The hull is opaque, so SSPR already reflects it.
- Crest light: only `body *= 1.0 + 0.4 * saturate(in.height)` (`water_material.wesl:360-361`), a height boost with no
  light or view term and no choppiness mask.
- Spray: none. The owner's WA-10 note: "storm sea still meek (no crest foam, no spray)".
- iWave dynamic ripples R2-R4: planned, not built.
- Bevy SSR is deferred-only (`bevy_pbr/src/ssr/mod.rs:54-80`), so it would never touch our transmissive water. Nothing
  is lost, since SoT did not use SSR either.
- `bevy_hanabi` 0.20.0 (MIT/Apache, `bevy ^0.20`) shipped on 2026-11-01. That unblocks the ROADMAP:151 "update all"
  pass and makes it a candidate spray renderer. `sf_precip_bevy` already draws GPU splash sprites around the camera.

**Split between lanes:**
- **Storm sea lane** (owner 2026-11-17, SoT reference): the whole-ocean look. A crest SSS term (choppiness mask ×
  back-light phase × sun, shadowed), crest foam (biased Jacobian, progressive blur, crest-to-body texture blend),
  storm swell, and **the spray renderer** with its crest emitter (GPU billboards in a camera window, spawned from the
  FFT foam/slope signal). Godotwind's spray pass tried this: foam as the primary spawn signal plus a slope filter,
  weather-driven energy, its own off switch (`ocean_quality_tracker:772-815`).
- **Ships lane:** hull-driven effects only. Bow and hull **splash emitters** from waterline sensors (the relative
  normal speed of the water at the bow), going through R4's `WaterSplash` into that spray renderer. **Wake foam**
  from hull-waterline intersection injected into a persistent foam window (SoT T:136-141), sharing R3's emitter path.
  Later, an optional deck wetness sim (SoT T:90-113).

## 7. Other big vehicles later (airships)

Everything except the medium stays generic in the framework:
- activators → control parameters → force model (`sf_vessel`);
- a dynamic hull body with based movement on its deck;
- deck-resting loose objects;
- deck-local navmesh;
- seat links and passenger state (IN7, IN24);
- base-relative save and replication.

An airship swaps buoyancy-in-water for lift and gas buoyancy, and keels and sails for props and envelopes. The
exclusion volumes become cloud or fog exclusion inside the gondola, which follows the same "volume list tested in the
medium's shader and query" pattern. Nothing in the framework may name ships, sails or Morrowind: the control set is data.

## 8. Proposed slices

Spec first: **SHP-0** writes `superfargoth/docs/specs/vessels.md`. It covers controls and parameters, the force model,
the flood model, the deck-resting rule, net and save, an industry comparison and the reuse check (all of the above). It
also amends water.md (exclusion volumes in the query and the render, per-query minimum wavelength, hull-effect
emitters, a flooded-hull body), interaction.md (c) (kind 5 "crewed vehicle": stations as interaction points, not seats),
and the travel roadmap (TV13 boats run on the vessel model, kinematic until SHP-5 lands). Docs only.

| ID | Deliverable (one + its test) | Crate | Depends | Verify |
|---|---|---|---|---|
| SHP-1 | Exclusion volumes in `WaterQuery`: inside a vessel volume → interior body or no water | `sf_water` | SHP-0 | exact: points inside, outside, on the cap; pool-over-sea priority; ≤ 2 µs |
| SHP-2 | Water draw discards inside the volumes; the opaque hook, rain rings and the medium respect them | `sf_water_bevy` | SHP-1 | shader compile + CPU reference; GPU ms; owner SHP-2 (`level:` hull greybox, a 30 m and a 3 m hull, camera in the hold) |
| SHP-3 | `hull.cook` bake: interior volumes from the physics hull (voxel flood-fill under the cap) | `sf_water` (bake) | SHP-1 | exact on a synthetic open box and a U-hull; log ms |
| SHP-4 | Per-query minimum wavelength in the twin (short hulls get cascades 1-2) | `sf_water` | - | twin vs GPU within 1 cm per cascade set; µs |
| (3.7) | Pontoon buoyancy, as specified (ROADMAP 3.7) | `sf_physics`, `sf_water_bevy` | SHP-4 | as water.md |
| SHP-5 | Kerner hull buoyancy: triangle clipping, hydrostatics, pressure drag, slamming | `sf_water` | 3.7 | exact: box hull rests at the analytic draft; sanity: no NaN over sea states; ms per boat |
| SHP-6 | Vessel model: sail length and angle × apparent wind, keel, rudder, anchor (with the pivot) → forces | new `sf_vessel` (Bevy-free) | SHP-0 | exact: no wind → no thrust, anchor stops in tolerance, mirror symmetry; tick-pure |
| SHP-7 | Deck-resting loose objects (attach on rest, release on hit, tilt or pick-up) | `sf_physics` | IN20 | sanity: a crate on a rolling deck has 0 drift over 10 min, slides past its friction angle |
| SHP-8 | Stations: wheel, cleats, pulleys, capstan (crew count), oars as interaction points → `VesselControls` commands | new `sf_vessel_bevy` | SHP-6, IN0, IN11 | exact: command → parameter at the tick; 2 crew raise the anchor faster |
| SHP-9 | Flooding: holes from damage, Torricelli inflow, bailing, added mass, sinking; the interior body published to SHP-1 | `sf_vessel` | SHP-1, SHP-6 | exact: inflow vs analytic, a bucket removes its volume, sinks at full |
| SHP-10 | Flooded-hull water drawn: a vessel-attached pool body, tilted, clipped | `sf_water_bevy` | SHP-2, SHP-9 | owner SHP-10 (half-flooded hold, camera above and below) |
| SHP-11 | Hull effects: bow sensors → `WaterSplash`; waterline wake foam injection | `sf_water_bevy` | R3/R4, storm-lane spray renderer | counts in log; owner SHP-11 |
| SHP-12 | Morrowind ships and rowboats as vessels: hull data, station slots, an adapter profile | `mw_*` data | SHP-3, SHP-8 | owner SHP-12 (a player-crewed ship off Ebonheart) |

Later and not sliced yet: replication of vessel state (net lane), the SoT deck wetness sim, volume-aware underwater
through hull holes, airships.

## 9. Owner questions (design only)

1. **Who crews the stations when the player sails?** Recommended: followers and hired crew take stations on command
   (SoT-style manual stations for players and remote co-op players; NPCs fill the empty ones). Solo play stays possible
   on small ships.
2. **Can ships sink and be lost?** Recommended: yes, through holes, flooding and bailing as in SoT. A sunk owned ship
   returns to its home dock after a delay, its cargo goes down with the wreck, and the wreck can be looted.
3. **Can any rowboat placed in the world be used?** Recommended: yes, where the content marks it usable. Taking an
   owned one is theft (`crime`).
4. **Classic preset?** Recommended: player sailing is Modern only. Classic keeps fare travel and ride boats (vanilla
   option).
