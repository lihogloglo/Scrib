# Water bodies: rivers, lakes, joins, waterfalls, swimming in a current, wading (FL-1..FL-7)

Status: specced 2026-11-26 (this file); nothing built. Parent spec: [water.md](water.md) (data model, query, budgets,
"Water bodies at any height", "Rivers: baked flow to the sea"). Underwater: [water-underwater.md](water-underwater.md).
Research: `superfargoth/docs/research/water-types-2026-11-26.md` (causes C1-C8, industry, owner answers §8).
Ledger: `docs/ledger/water.md` (G-W5/6/7 river speed). Wiki: `docs/wiki/engine/terrain-and-water.md`.
Adapter: `../../../bevyrrowind/docs/specs/water-adapter.md` "Generated water (FL)".
Crates: `sf_water` (Bevy-free), `sf_water_bevy`, `sf_worldgen` (guide reader), `sf_controller` (current drift);
game: `mw_environment_bevy` (`worldgen_water.rs`, `water.rs`, new `swim.rs`). Lanes: FL-4, FL-5 = worldgen-2.

<!-- agent-summary -->
> **Agent summary (2026-11-26).** Generated (guide) rivers and lakes are meshes only today: the CPU query does not know
> them, the river ripples are the ocean's self-moving FFT cascades advected by one vector per cross-section, joins are
> dropped, waterfalls do not exist. Slices: FL-1 bank shear, FL-2 a river flow normal texture (no ocean waves on
> rivers), FL-3 a baked solved flow field, FL-4 joins (worldgen-2), FL-5 guide bodies in the query + swimming carried
> by the current (worldgen-2, first), FL-6 waterfalls at drops > 2 m, FL-7 wading rings + a short wake (flow unchanged).
> Read next: "Build rows", then your slice's section, "Tunables", "Performance".
<!-- /agent-summary -->

## Goal

Rivers read as carried water: streaks drifting downstream, slow banks, flow bending round bars, no sea waves; joins
without seams at confluences, lake inlets and river mouths; waterfalls wherever the levels drop; a swimmer in a river is
pushed downstream, a wader leaves rings and a short trail. Owner answers (2026-11-26, research §8): **rivers carry no
ocean waves** (cascades off on river bodies, flow ripples and foam only); **FL-5 first** (swim + current in guide lakes
and rivers); **waterfalls at every drop > 2 m** (a tunable); **wading = visual rings + a short wake, the river's flow is
never changed by the wader**.

## Today (verified 2026-11-26)

| What | Where |
|---|---|
| Guide reader keeps rivers `id, kind, points [x,y,w,level]`, lakes `id, level_m, outline, holes`; `joins`, `outlet`, `mu_id`, `clipped`, lake `kind` ignored | `sf_worldgen/src/water.rs:7-8, 23-45` |
| MU writes those keys: `RiverOut { kind, mu_id, joins, outlet, clipped, points }`, `LakeOut { kind open/closed, from_course }` | `scrib-worldgen/crates/water/src/run.rs:28-61`; MU `Line.joins/outlet` `vendor/magna-urbis/rust/crates/terrain/src/vector_water/rivers.rs:46-58` |
| Speed per point: Manning, Jarrett on steep reaches, window average, 3 m/s cap (G-W7) | `mw_environment_bevy/src/worldgen_water.rs:147-197` |
| Bake: lake `lake_mesh` (no flow), river `river_mesh`; spawned as `WaterMeshSurface`; **no `WaterBody`** | `worldgen_water.rs:256-290, 313-320` |
| River ribbon: `flow = s.tangent * s.flow_speed`, the same on every column | `sf_water/src/body_mesh.rs:443-461` (params 194-232) |
| Mesh bodies are flat (no vertex displacement); flow rotated by the transform | `sf_water_bevy/src/shaders/water_material.wesl:85-104` |
| `WATER_FLOW` two-phase advection of the **ocean slope cascades**; slope gain rises with speed; speed foam | `water_material.wesl:256-298, 372-374`; tunables `sf_water_bevy/src/flow.rs:18-78`, bound `mw_environment_bevy/src/water.rs:190` |
| Query: `Polygon` and `Ribbon` supported, flow = segment direction × lerped speed, no cross profile; ocean dry where bed ≥ surface, other bodies not | `sf_water/src/query.rs:302-320, 414-437, 361-371` |
| River waves already flat on the CPU: `WaveTerms::of(River) = FLAT` | `sf_water/src/records.rs:72` |
| Query bodies built once: `source.bodies(..)` then `WorldspaceWater::new(bodies)` | `mw_environment_bevy/src/water.rs:262-267` |
| Swim: `MoverMode::Swim { surface }` surface clamp; set from the water query by `mw_environment_bevy/src/swim.rs` since FL-5c (2026-11-26; creature scaffold `Fly` for pure swimmers still there) | `sf_controller/src/mover.rs:225-229, 1083-1089`; `mw_walkers_bevy/src/walkers.rs:443-447` |
| Nav swim level from `fSwimHeightScale` (OpenMW `getSwimLevel`) | `mw_bake/src/nav.rs:17-20, 73` |
| Graph `swimming` parameter: declared, written by no system (grep) | `mw_graphs/src/build.rs:100-101` |
| x10s-b2w `water.json` (2026-11-26): 95 rivers (max 1490 points, ~5 m apart, 18 516 segments), 186 lakes, 77 rivers with `joins`, 49 with `outlet`; steep runs (grade > 0.5) grouped: 142 drops > 2 m, p10/50/90 3.8/19.4/182 m, max 1127 m | node count over the file |

## Canonical technique

Body model as UE5 Water (lake = polygon at one level, river = spline with per-point width, height, velocity; one data
query for gameplay and render). River shading as Vlachos 2010 / Naughty Dog: a **dedicated tileable flow normal map**
advected in two phases half a cycle apart with phase noise, normal strength **falling** with speed, foam from the same
field; the flow itself from a field solved against the channel geometry (Vlachos: Houdini combing; ours: potential flow
on the water mask). Joins as UE transition zones (river flow and look fade over a mixing zone into the receiver).
Waterfalls as an authored-style curtain mesh with stretched scrolling noise, plunge foam and mist cards. Characters in a
current: velocity relative to the water (the swimmer drifts with it). Wakes: a dynamic ripple grid fed by emitters
(Crest), the river's own flow untouched.

## Reuse check (Bevy 0.20, crates, ours)

| Item | Finding | Verdict |
|---|---|---|
| Bevy 0.20 | no water, flow map, fluid or particle system in `bevy_pbr`/`bevy_render` | nothing to reuse |
| `bevy_water` | newest Bevy 0.18, ocean tiles only | no |
| `bevy-aqua` 0.1.3 (MIT/Apache, Bevy 0.19) | rivers = 3 analytic sines along a baked current; tilted water rejected; no joins/falls | reference only |
| `bevy_hanabi` | no Bevy 0.20 release found | mist uses our own instanced cards (FL-6b); revisit on a 0.20 release |
| `salva` and SPH crates | particle fluids, wrong tool for a 2D height grid | no |
| Flow-map generators | Houdini, Maya tools; no Rust crate | own bake (FL-3): Jacobi/multigrid Laplace, ~200 lines |
| Character current | `sf_controller` mover already sweeps a horizontal displacement | extend (FL-5b), no new crate |

## Industry comparison

| Topic | Reference | Ours after the slices | Verdict |
|---|---|---|---|
| Body model + data query | UE5 Water bodies + water data queries | guide lakes/rivers as `WaterBody` records (FL-5a) | match (gap today) |
| Rivers without sea waves | UE5 rivers carry no waves; Vlachos | `WATER_FLOW` stops reading the cascades (FL-2); CPU already flat | match |
| River normals | Vlachos 2010, Naughty Dog (GDC 2012): flow normal map, two phases, strength down with speed | FL-2 | match (gap today: C1, C8) |
| Cross-section shear | per-point velocity + profile (UE spline points, ND colour maps) | analytic profile (FL-1), solved field (FL-3) | match after FL-3 |
| Solved field, obstacles | Vlachos Houdini combing; Peytavie 2019 blend-flow tree | potential flow on the mask, terrain islands as obstacles (FL-3) | match; differ: no placed rocks in v1 |
| Joins | UE River-to-Lake / River-to-Ocean transition materials | coverage + flow fade over a mixing zone, tributary snap (FL-4) | partial: no dedicated transition material |
| Waterfalls | UE / ND authored mesh + scroll + particles + mist | generated from MU level drops (FL-6) | match on technique; differ: placement generated |
| Current on characters | UE `CharacterMovement` swimming in a fluid volume; UE Buoyancy river forces (recalled, unverified) | relative-velocity drift on the fixed tick (FL-5b/c) | match |
| Wading wake | Crest dynamic waves (recalled), UE Niagara Fluids | R2 grid + CPU-drifted wake points (FL-7) | differ by design: one-way, flow unchanged (owner) |
| RDR2, Witcher 3, Horizon FW, Sea of Thieves rivers | no public technique source found (research §4) | - | not cited |

## Slices

### FL-1: cross-section speed profile

`sf_water::RiverProfile { bank_floor, exponent }` and `profile(u) = max(floor, 1 − |2u − 1|^p)` (u = 0..1 bank to bank).
`river_mesh` (`body_mesh.rs:443-461`) writes `flow = tangent * speed * profile(u)` per column (direction unchanged);
`contains()` for `Ribbon` (`query.rs:421-436`) scales the speed by the same profile from the lateral offset, through
`WorldspaceWater::set_river_profile` (like `set_bed`), so the swimmer feels what is drawn. Test (exact): centre = v, edge
= floor × v, all columns parallel to the tangent, monotone in |u − ½|; query at the centre and at 0.9 of the half width
returns the mesh's values; `body_mesh_tests` pass. Crate `sf_water`. Cost 0 GPU, 0 bake.

### FL-2: river flow normal texture, no ocean waves on rivers

`sf_water::flow_normals(seed, px, streak) -> Vec<[u8; 4]>`: a tileable RGBA8 texture (XY slope, Z foam break-up mask),
two noise octaves stretched along +V (the flow), generated once at start (< 1 s, deterministic). `sf_water_bevy` binds it
for `WATER_FLOW` bodies: the tap loop (`water_material.wesl:281-291`) reads this texture (two taps × two octaves at
`normal_tile_m` and `normal_tile_m / 3.7`) instead of the slope cascades; no whitecap term; slope gain becomes
`1 / (1 + normal_fall · speed / foam_full)` (replaces `water.flow.normal_gain`, `flow.rs:50`, Vlachos); speed foam ×
the texture's foam mask. Lakes keep the short cascades (`WaveTerms::of(Lake)`). Switch `--off waterflowtex` (game
`mw_game_core/src/perf.rs` part list, `perf.rs:305-320` style) falls back to today's path. Test: generator exact
(tileable: row 0 continues row px−1 within one gradient step, mean slope 0 ± 1/255, same seed same bytes); naga compile of
both defs (`shader_tests.rs`); CPU twin of the phases (`flow_tests.rs`) unchanged. Crates `sf_water`, `sf_water_bevy`
(+ the switch row in `mw_game_core`, bind in `mw_environment_bevy/src/water.rs:190`). Cost on river pixels: 4 fetches
instead of 2 × n cascades (8 on High): −0.03..+0.02 ms.

### FL-3: baked solved flow field

`sf_water::flow_field` (Bevy-free, feature `bake`): per river, rasterise its ribbon (and the lake margins it touches) at
`texel_m` on a world grid; obstacles = terrain above the local surface inside the mask; solve a potential flow (Laplace
of φ, φ = 1 at the inflow edge, 0 at the outflow, Neumann at banks and obstacles; multigrid, tolerance tunable);
velocity = −∇φ normalised × the point speed × FL-1's profile from the bank distance. Store per river resampled in
**ribbon space** (along × across, the mesh UVs `body_mesh.rs:459`) as RG8 snorm + R8 speed in one atlas, so the shader
reads it by UV (no water-level clipmap needed) and `contains()` reads the same texels. Cache product `water.flow`
(water.md), fingerprinted on `water.json` + terrain; built on a worker at first load, never in play. Speed source stays
Manning/Jarrett (owner: speed good after G-W7). Switch `--off flowfield` (vertex flow). Test (exact, synthetic): straight
channel uniform ±2 %, 90° bend inner/outer speed ratio > 1, island splits and re-merges with divergence < 2 %; bake ms on
x10s-b2w printed by an `#[ignore = "measurement: ..."]` test (est. 20-60 s, 1.3 M texels). Crates `sf_water`,
`sf_water_bevy`, `mw_environment_bevy`. Cost +0.05-0.10 ms (one fetch per tap).

### FL-4: joins (worldgen-2)

FL-4a and FL-4b are written for the worldgen-2 lane in "Build rows". Design: the reader keeps `mu_id`, `joins`, `outlet`,
`clipped` (rivers) and `kind`, `from_course` (lakes); `sf_water::joins` resolves each river end: **tributary** (`joins`
names another river's `mu_id`): last point projected onto the receiver's centreline if within `snap_max_m`, level =
min(own, receiver's there); **mouth** (end inside a lake polygon, or at sea level over the ocean): mixing zone;
**inlet** (first point inside a lake, `lake_link` or an open lake's outflow): mixing zone from the lake edge.
`RiverPoint` gains `river_ness: f32` (serde default 1): 1 → 0 over `mix_widths × width` toward the join, smoothstep.
**As built (FL-4a, 2026-11-26, `sf_water::joins::resolve`):** the zone lies only where the receiver's water already
is, so a river never thins over its own dry bed (UE's Water plugin blends river into lake or ocean over the overlap of
the two bodies): the *overlap run* is the trailing run of points inside the lake at its level, within the receiver
river's half width, or at sea level (leading run for an inlet); `river_ness` is 1 at the run's first point and falls
by smoothstep to 0 over `min(mix_widths × width, run length)`, 0 beyond. A point counts as in a lake only within
`water.join.level_tol_m` of its level (the real file has courses running 2-48 m under lake outlines: not mouths). End
order: lake, then tributary (snapped when within `snap_max_m`, else reported unsnapped and left alone), then sea. The
reader keeps the keys as written (`River { mu_id, joins, outlet, clipped }`, `Lake { kind, from_course }`); several
rivers can share one `mu_id` (a cut line), so a tributary's receivers are every piece of the named line.
The same value scales the point's `flow_speed` (mesh and query agree) and, as a vertex attribute
`ATTRIBUTE_WATER_RIVERNESS`, the ribbon's coverage and flow normals, so the receiver (lake, sea, main river) shows
through and no two flows cross-fade by alpha. Priorities: ocean 0, guide lakes 20, guide rivers 30 (rivers own their
inlet/outlet zones; constants, an overlap rule). Bed carving under the ribbon stays the worldgen tool's (ROADMAP [p]
"generated rivers refactor"), not FL-4.

### FL-5: guide bodies in the query, swimming carried by the current (worldgen-2, first)

- **FL-5a** (built 2026-11-26, worldgen-3: 844 ns per river sample on x10s-b2w at chunk 64, 2296 ns unchunked):
  `worldgen_water::guide_bodies(&GuideWater, &RiverFlow, ribbon_chunk) -> Vec<WaterBody>` (pure): lakes `Polygon` (kind
  Lake, `WaveTerms::for_kind(Lake)`: `first_cascade` 2, so the CPU twin, cascade 0 only, leaves them flat, `query.rs:328`),
  rivers `Ribbon` in chunks of `water.query.ribbon_chunk` points with a one-point overlap (a 1490-point river in one
  body costs a 1490-segment scan per sample; chunks keep the BVH tight and a sample ≤ 2 µs), kind River, `FLAT` waves,
  ids `guide:lake:<id>`, `guide:river:<id>:<k>`. `water.rs:262-264` appends them before `WorldspaceWater::new`; speeds
  from the same `river_speeds` as the mesh bake (identical inputs, identical values).
- **FL-5b** (built 2026-11-26, worldgen-3): `Locomotion::current: Vec3` (m/s, world, host-written each tick, `#[serde(skip)]`
  like `inherited`: zero on load, save shape unchanged): the step adds
  `current.xz · dt` to the horizontal displacement before the sweep in every mode (walls and banks stop the drift); the
  intent stays relative to the water.
- **FL-5c** (built 2026-11-26, worldgen-3 rule, worldgen-4 system; `enter_fraction` = `fSwimHeightScale` itself, the share of the height `OpenMW` swims at; the `Mod` layer carries it): `sf_water::swim::medium(&Immersion, swimming: bool, &SwimParams) -> Medium { swim, surface, current }`
  (pure: enter at `swim.enter_fraction`, leave below `swim.exit_fraction`, current = horizontal sample velocity × gain,
  × `wade_gain × fraction` when wading, clamped) and the game system `mw_environment_bevy/src/swim.rs` in `FixedUpdate`,
  `ControllerSystems::Drive`: for every awake `Mover` (capsule from `MoverParams::radius/height`), `immersion()` →
  `Ground` ↔ `Swim { surface }` and `loco.current`; movers in `Fly` (creature scaffold) are left alone. The adapter maps
  `fSwimHeightScale` to `enter_fraction` with the nav bake's rule (`mw_bake/src/nav.rs:17-20`). Applies to every body in
  the query, one rule: the player swims wherever `WaterQuery` has water, the classic sea and interior pools included
  (meta 2026-11-26 from vanilla behaviour and the industry standard, owner informed; DECISIONS). Dive, breath,
  strokes, the graph's `swimming` parameter (anim lane) stay ROADMAP 3.6.

### FL-6: waterfalls at drops > 2 m

- **FL-6a**: `sf_water::waterfall::detect(&[RiverPoint], &FallParams) -> Vec<Fall { lip, foot, top, bottom, width,
  speed }>`: consecutive segments with grade > `min_grade` grouped into one fall, kept when the summed drop >
  `min_drop_m`; also river-end-to-receiver steps from FL-4 (end level above the receiver by > `min_drop_m`).
  `river_mesh` takes the falls and leaves their steep strips undrawn (no ramp). Test (exact, synthetic profiles): a 3 m
  step is one fall, a 1.5 m step none, three steep segments in a row one fall with the summed drop, a gentle 30 m descent
  over 1 km none; the guide-file test prints the count on x10s-b2w (142 by the 2026-11-26 file).
- **FL-6b**: per fall a curtain mesh lip → foot (`sf_water`, Bevy-free): ballistic arc from the lip speed where it clears
  the terrain, else hugging the slope `offset_m` above it; `WATER_FALL` def of the water material: FL-2's texture
  stretched along the fall, scrolling at `min(√(2 g h), cap)`, aerated white rising with the drop, thin alpha edges;
  plunge foam disc on the receiver (`plunge_radius_k × width`); mist: our instanced soft cards for the nearest
  `max_full` falls within `far_m`, others curtain only. Switch `--off waterfalls`. Test: curtain sanity (no NaN, monotone
  downward, ends at the foot level ± 1 cm), naga compile; look → WA-14. Crates `sf_water`, `sf_water_bevy`,
  `mw_environment_bevy`.

### FL-7: wading rings and a short wake (flow unchanged)

Depends on water.md R2 (iWave grid) and R3 (emitters). FL-7 adds wake points: a wading or swimming actor drops one every
`wake_spacing_m`; each drifts with the query's flow at its position (sampled at `sample_hz`) and re-injects into the R2
grid with amplitude decaying to 0 over `wake_s`, so the rings trail downstream. Leg rings: R3 emitters scaled by
`ring_gain`. Presentation only (`Update`, reads positions and the query, writes nothing gameplay reads). The flow field,
the mesh flow and the query are never modified. Switch `--off wadewake`. Test (exact, CPU): a point in a 1 m/s river is
1 m downstream after 1 s ± 1 cm and expires at `wake_s`; sanity on the grid: energy < 1 % after 2 × `wake_s`, no NaN.
Crates `sf_water` (wake points), `sf_water_bevy`. Cost inside the R2 row (+0.005 ms splats); CPU ≤ 0.02 ms.

## Build rows

| Slice | Deliverable | Crates (claims) | Depends | Test | Verify |
|---|---|---|---|---|---|
| FL-1 | bank shear in mesh + query | `sf_water` | - | exact profile + query agree | WA-13 |
| FL-2 | river flow normal texture, cascades off on rivers | `sf_water`, `sf_water_bevy`, switch row `mw_game_core`, bind `mw_environment_bevy` | - | generator exact, naga | WA-13, `perf ab --off waterflowtex` |
| FL-3 | solved flow field product | `sf_water` (bake), `sf_water_bevy`, `mw_environment_bevy` | FL-1 | synthetic exact, bake ms printed | WA-13, `--off flowfield` |
| FL-4a | **worldgen-2.** Reader keeps join keys; `sf_water::joins` (snap, mixing zones, `river_ness` on `RiverPoint`) | `sf_worldgen` (`src/water.rs:23-45`, doc 7-8), `sf_water` (`records.rs:19-29`, new `joins.rs`): claim `sf_water` from meta | - | exact: Y confluence ends on the receiver's centreline at equal level; mouth/inlet `river_ness` 1→0 monotone over the zone; reader round-trips the keys; the guide-file test (`worldgen_water_tests.rs:150`, skips without a guide) prints resolved/unresolved joins (77 `joins`, 49 `outlet` expected) | `cargo test -p sf_worldgen -p sf_water` |
| FL-4b | **worldgen-2.** Mesh attribute + shader fade + wiring | `sf_water` (`body_mesh.rs` attribute), `sf_water_bevy` (`mesh.rs` `ATTRIBUTE_WATER_RIVERNESS`, `material.rs:194-227` layout, `water_material.wesl`), `mw_environment_bevy` (`worldgen_water.rs:256-290` calls `joins`, priorities) | FL-4a | naga both defs; mesh test: attribute per vertex = its point's `river_ness` | WA-13 joins part (owner) |
| FL-5a | **worldgen-2, first.** Guide bodies in `WaterQuery` | `mw_environment_bevy` (`worldgen_water.rs` new `guide_bodies`, `water.rs:262-264`) | - | exact (`worldgen_water_tests.rs`, flat `BedHeights` stub at 117 m): river point → kind River, velocity = segment direction × lerped speed (± 1e-4), height = lerped level; lake (level 120) → kind Lake, depth 3.0; land above sea outside both → `None`; capsule immersion fraction exact; sample ns on x10s-b2w printed (`#[ignore = "measurement: ..."]`, ≤ 2 µs) | `cargo test -p mw_environment_bevy` |
| FL-5b | **worldgen-2.** Current drift in the mover | `sf_controller` (`mover.rs:281-305` field, step displacement near 1083): check `.sessions` heads for a claim | - | exact: a `Swim` mover with zero intent, current (1,0,0) moves 1.000 m in 60 ticks of 1/60 s; against a wall it stops at skin; intent adds; serde default 0 | `cargo test -p sf_controller` |
| FL-5c | **worldgen-2.** Swim medium + game system | `sf_water` (new `swim.rs`, tunable defs), `mw_environment_bevy` (new `swim.rs`; direct dep `sf_controller_bevy`, already in its graph via `sf_physics_bevy`: `cargo hakari generate --diff`, `tools/bevy_variants.sh`) | FL-5a, FL-5b | exact: hysteresis enter 0.9 / stay 0.85 / leave 0.6; current clamp; headless app (MinimalPlugins + `Water` with one guide river + one `Mover`): after 60 ticks the mover is `Swim` and drifted speed × 1 s ± 2 cm downstream; a `Fly` mover untouched | `cargo test -p sf_water -p mw_environment_bevy`; WA-16 |
| FL-6a | fall detection, ribbon split | `sf_water` | FL-4a (join steps) | synthetic exact, count printed | - |
| FL-6b | curtain, plunge foam, mist | `sf_water`, `sf_water_bevy`, `mw_environment_bevy` | FL-6a, FL-2 | curtain sanity, naga | WA-14, `perf ab --off waterfalls` |
| FL-7 | wake points + leg rings into R2 | `sf_water`, `sf_water_bevy` | R2, R3, FL-5a | wake drift exact, grid sanity | WA-15, `--off wadewake` |

Order (research §7 with the owner's FL-5 first): FL-5a, FL-5b, FL-5c, FL-1, FL-2, FL-4a, FL-4b, FL-3, FL-6a, FL-6b, FL-7.
Worldgen-2 rows read only guide data (`water.json` through `sf_worldgen::water`) and never Morrowind records; the
scene for checks is the TN-2 guide `world:C:/cargo-target/scrib/worldgen/x10s-b2w`.

## Net-ready

The current is a pure function of position and the bodies (rivers and lakes have no time-varying CPU waves), computed
on the fixed tick in `ControllerSystems::Drive` before the mover step: host-authoritative, a client predicts the same
value. A remote player is just another `Mover`. Rings, wake points, curtains, plunge foam and mist are presentation
(`Update`, render world) and feed nothing back.

## Tunables

Declared with `def!` (unit, range, source) as `sf_water_bevy/src/flow.rs:18-78`; `Guess` counted; census must pass.
Look values stay `Dev` (no own options row: tunables.md open question 1, groups only); `water.fall.max_full` and
`water.fall.mist` are `sg.water` members; `water.fall.mist` is `Player` and gets a graphics-page row "Waterfall mist"
once per-feature rows are allowed. Switches are game `--off` parts: `waterflowtex`, `flowfield`, `waterfalls`, `wadewake`.

| Name | Unit | Range | Default | Source | Slice |
|---|---|---|---|---|---|
| `water.river.bank_speed_floor` | ratio | 0-1 | 0.15 | Guess (research rank 1) | FL-1 |
| `water.river.profile_exp` | ratio | 1-8 | 2 | Industry (parabolic channel profile) | FL-1 |
| `water.flow.normal_tex_px` | px | 128-1024 | 512 | Guess | FL-2 |
| `water.flow.normal_tile_m` | m | 0.5-16 | 4 | Guess | FL-2 |
| `water.flow.streak` | ratio | 1-4 | 2 | Guess | FL-2 |
| `water.flow.normal_fall` | ratio | 0-2 | 0.5 | Industry (Vlachos 2010: strength down with speed) | FL-2 |
| `water.flow.foam_breakup` | ratio | 0-1 | 0.6 | Guess | FL-2 |
| `water.flow.texel_m` | m | 0.5-8 | 2 | water.md `water.flow` (2 m texels) | FL-3 |
| `water.flow.solve_tolerance` | ratio | 1e-5-1e-2 | 1e-3 | Guess | FL-3 |
| `water.join.mix_widths` | ratio | 0.5-8 | 2.5 | Guess (research: 2-3 widths) | FL-4a |
| `water.join.snap_max_m` | m | 0-50 | 20 | Guess (farther ends unsnapped, logged) | FL-4a |
| `water.join.level_tol_m` | m | 0-5 | 0.5 | Guess (x10hs-b2w: real mouths within 0.4 m of the lake level, courses under a lake 2-48 m below) | FL-4a |
| `water.query.ribbon_chunk` | count | 8-512 | 64 | Guess (keeps a sample ≤ 2 µs) | FL-5a |
| `swim.enter_fraction` | ratio | 0-1 | 0.7 | Guess (adapter: from `fSwimHeightScale`) | FL-5c |
| `swim.exit_fraction` | ratio | 0-1 | 0.6 | Guess (hysteresis) | FL-5c |
| `swim.current.gain` | ratio | 0-2 | 1.0 | Industry (velocity relative to the water) | FL-5c |
| `swim.current.wade_gain` | ratio | 0-1 | 0.3 | Guess | FL-5c |
| `swim.current.max_mps` | m/s | 0-5 | 3.0 | G-W7 river speed cap | FL-5c |
| `water.fall.min_drop_m` | m | 0.5-20 | 2 | Owner 2026-11-26 ("> 2 m") | FL-6a |
| `water.fall.min_grade` | ratio | 0.1-5 | 0.5 | water.md (2 m over 4 m) | FL-6a |
| `water.fall.max_full` | count | 0-8 | 2 (High) | Guess (budget) | FL-6b |
| `water.fall.mist` | bool | - | on | Owner 2026-11-26 (falls + mist) | FL-6b |
| `water.fall.mist_density` | ratio | 0-4 | 1 | Guess | FL-6b |
| `water.fall.plunge_radius_k` | ratio | 0.5-4 | 1.5 | Guess | FL-6b |
| `water.fall.far_m` | m | 500-10 000 | 3000 | Guess | FL-6b |
| `water.fall.offset_m` | m | 0-2 | 0.3 | Guess | FL-6b |
| `water.wade.wake_s` | s | 0.2-5 | 1.5 | Owner 2026-11-26 ("short wake") | FL-7 |
| `water.wade.wake_spacing_m` | m | 0.1-2 | 0.4 | Guess | FL-7 |
| `water.wade.max_points` | count | 8-256 | 64 | Guess | FL-7 |
| `water.wade.ring_gain` | ratio | 0-2 | 1 | Guess | FL-7 |
| `water.wade.sample_hz` | Hz | 2-60 | 10 | Guess | FL-7 |

Retired by FL-2: `water.flow.normal_gain`. Constants (maths/layout): g = 9.81, priorities 0/20/30, the 3.7 octave ratio.

## Performance

Estimates (research §6, none measured), RTX 4060 laptop, High, 1440p + DLSS Q; each lands with an `sf_diag` counter and a
`perf ab` arm on its switch. Against water.md "Performance budgets": rivers and lakes sit in the "Water draw" row (High
0.5), wakes in "Water-level clipmap + dynamic ripples" (0.08), waterfalls in the new "Waterfalls (when visible)" row.

| Slice | GPU ms (High) | CPU | Row |
|---|---|---|---|
| FL-1 | 0 | 0 | water draw |
| FL-2 | −0.03..+0.02 (river pixels) | start-up texture < 1 s | water draw |
| FL-3 | +0.05-0.10 | bake 20-60 s once, cached | water draw |
| FL-4b | +0.02 | 0 | water draw |
| FL-5a-c | 0 | ≤ 2 µs per sample; ≤ 32 movers in water ≤ 0.07 ms per tick (typ. < 0.01) | CPU queries |
| FL-6b | 0.1-0.15 per full fall, curtain-only ≤ 0.02 each; ≤ 0.3 total via `max_full` | detection at mesh build | waterfalls |
| FL-7 | +0.005 (splats) | ≤ 0.02 ms (64 points at 10 Hz) | ripples |

Steady rivers + lakes after FL-1..FL-4: +0.04-0.14 ms, inside the 0.5 ms draw row; no allocation or bake in play, no p99
effect expected.

## Verification (owner visual checks)

Planned in `bevyrrowind/docs/visual-checks/terrain-water.md`: **WA-13** river without sea waves, shear, joins (after
FL-1, FL-2; joins part after FL-4b); **WA-14** waterfall (after FL-6b); **WA-15** wading rings + wake (after FL-7);
**WA-16** swimming in the current (after FL-5c).

## Moved from water.md "Rivers: baked flow to the sea" (2026-11-26)

**Generated rivers** (`Generate`): trace from each cell where accumulation crosses the threshold down
D8 to a body; smooth to a spline (Catmull-Rom, simplified to 4 m error); width by hydraulic geometry
(`w = a·Q^0.5`); surface height per point monotone non-increasing (min of bank − depth along the
channel); ribbon mesh + flow map; join at confluences; end in the receiving body with a mixing zone.
Waterfalls where the surface drops more than 2 m over 4 m: a marker for an effect mesh (content
supplies it), not simulated.

**Shading** (Vlachos 2010): two-phase flow-advected normal maps (and foam where speed or shore
contact is high), wave cascades × openness (rivers get little), ripples. Buoyant bodies drift with
`velocity`.

Superseded for guide rivers by FL-1..FL-6 above: rivers get no wave cascades (owner 2026-11-26), waterfalls are built
(FL-6) instead of a marker, joins follow FL-4.

## Later

Speed from discharge continuity (MU's per-point `area`, not in `water.json` yet), placed rocks as flow obstacles, a
foam wedge at confluences, a low swell on very wide rivers, per-region optics for guide water (today all coastal,
`worldgen_water.rs:10-13`), waterfall sound cue, floaters drifting in guide rivers (W6 reads the same bodies).
