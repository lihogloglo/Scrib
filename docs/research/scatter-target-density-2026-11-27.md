# Scatter: driving the point process from a target density, and a load cut by area (WG-E2o research)

Date 2026-11-27. Input for the WG-E2o SPEC row (`superfargoth/docs/specs/worldgen-scatter.md` Design 3, 4, 10, "Counts
per biome class", Tunables) and WG-E2p. Docs only. Marks: **V** = read in the source (page, docs, or our code at the
cited line), **U** = only seen through a search summary or abstract, not read in full; check before relying on it.

## The problem (from WG-E2n, V: `docs/ledger/worldgen.md` 2026-11-27)

- Canopy intensity today is per species: `kappa = parent_per_ha x su x D x prior` at the parent, then
  `Poisson(offspring_mean x su)` offspring (`sf_worldgen/src/scatter.rs:510-513`, V). The ceiling is
  `parent_per_ha x offspring_mean` = 48/ha for every class; `class.<name>.stems_per_ha` (5-300) and
  `canopy_stems_per_ha_max` (400) are parsed and never read.
- `su` enters three times: in `D` (noise x best canopy su, `scatter_stand.rs:68`), in the parent thinning and in the
  offspring mean. So the expected count goes as `E[su^2 D]`, not `E[su]`, with noise mean 0.50 and a clearing cut that
  removes 35-68 % of land.
- Competition keeps 63-73 %; canopy sizes 1-40 m, crowns up to 8.75 m.
- Result: winners 4.3-11.4/ha in every class, against declared 10-250.

The question has three parts: (1) how shipped tools turn "N per area" into points, (2) how to account for what
competition removes, (3) how a radius-limited load cut is done.

## 1. Target density -> points: what shipped tools do

| Tool | How density is given | What happens after | Mark |
|---|---|---|---|
| UE5 PCG Surface Sampler | `Points Per Squared Meter` sets the sample grid cell size (floor at the point extents); each point carries a `Density` attribute in 0..1 | Density Filter keeps points by density bounds (optionally normalised); Self Pruning / Difference remove overlaps. The expected count after pruning is not computed by the engine: artists iterate | U (API reference summaries; no published formula from count to kept count, a forum thread asks for one and gets none) |
| UE Procedural Foliage Spawner | per species `Initial Seed Density` (seeds per 10 m span, squared over 10 x 10 m), `Num Steps` of ageing and seeding, `Collision Radius` and `Shade Radius` decide overlaps by priority | an ecosystem simulation: the final density is an outcome, artists raise seed density and steps when collision cuts too much | U (Epic property reference, community reports) |
| Horizon Zero Dawn (Decima), GDC 2017 | per layer density maps from paintable/generated 2D world maps (~4 MB/km2, streamed in sections), ecotopes pick assets | collision between layers by **layered dithering**: a layer's density reduces the ones below, so a map value is directly the output coverage (no competition to invert); deterministic and locally stable; 100k+ objects around the player, ~250 us GPU | U (slide summaries; the PDF at guerrilla-games.com was not read in full) |
| EcoBrush (Gain, Long, Cordonnier, Cani 2017) | per terrain condition, distributions learned from ecosystem simulation; the user paints density and variability | a suitability map shows consistency with the terrain; a healing brush moves the result to the nearest consistent state. Density is the authored quantity, the distribution is resampled to meet it | U (abstract and summary; the attribution of EcoBrush to "Gain, Peytavie, Galin" in our earlier notes is wrong: Galin and Peytavie co-wrote Cordonnier 2017, not EcoBrush) |
| Cordonnier et al. 2017 (TOG 36(4) 134) | vegetation simulated jointly with erosion | density is a simulation outcome | U (abstract only; nothing found on how density is parameterised) |
| spatstat `rThomas` (reference implementation of the Thomas process) | parents Poisson(`kappa`), offspring Poisson(`mu`), Gaussian spread `scale`; intensity = `kappa x mu`. Inhomogeneity: `kappa` an image (inhomogeneous parents) or `mu` an image, then "the reference density for offspring points, in the sense of Waagepetersen (2007)" | the intensity is set by construction; no post-filter | V (R documentation page text) for `kappa x mu` and the two inhomogeneity forms; U for Waagepetersen 2007 itself |

**Finding 1.** Every tool with a "density" knob sets the intensity *before* removal steps and lets the artist iterate;
none of them inverts pruning automatically. The two approaches that meet a target by construction are (a) HZD's
dithering, where density is the output and there is no competition, and (b) the point-process approach: put the target
into the intensity once, with a known relation between intensity before and after the removal steps. We keep
competition (Design 6: crowns must not intersect, winners have sizes), so (b) is our case, with the keep ratio
accounted for explicitly (part 2).

**Finding 2 (the su-once form).** The standard inhomogeneous Neyman-Scott/Thomas construction puts the covariate into
*one* place. Waagepetersen's form (spatstat's `mu` image, U for the paper): homogeneous parents, offspring intensity
`mu(x)` proportional to the covariate; equivalently independent thinning of offspring by `w(x) / w_max`. Independent
thinning of a process by `p(x)` multiplies its intensity by exactly `p(x)` (the defining property of independent
thinning, D). So for a class target `T` and a weight `w(x) = su(x) x stand(x)` in 0..1:

```
lambda_pre(x) = T(class(x)) x w(x) / mean_class(w) / keep(T)        (stems per ha, before competition)
```

keeps the class mean at `T / keep(T)` before competition and `T` after, whatever the field's shape, if `w` enters
once. `mean_class(w)` is the area mean of `w` over the class's land; it must be computed from the same fields the pass
reads (a census over the guide at bake time) so it is deterministic and goes into the product hash. Today's product
`su^2 x D x prior` would have to be replaced by this single `w`; where `w` is put (parent thinning or offspring
thinning) does not change the mean, only the clustering: offspring thinning keeps clusters whole near edges of good
ground, parent thinning thins whole clusters. Both are valid; parent thinning is what the code does today
(Lewis-Shedler at the parent, `scatter.rs:510`, V), so the smallest change is "`su x D x prior` at the parent, plain
`offspring_mean` at the offspring, and `kappa_max` from the target".

The region prior (mean 1.0, clamp 0-2 %) multiplies `w`; if it stays inside `w` the class mean includes it, so it is
neutral on average (E2n measured mean 1.0).

## 2. The competition keep ratio

Competition (Design 6) is Matern II in round 1, then re-admits losers free of winners for `competition_rounds` rounds
(`worldgen-scatter.md` Design 6, V).

**Matern II closed form (Poisson input, fixed conflict distance R).** Intensity after thinning:

```
lambda_kept = (1 - exp(-lambda pi R^2)) / (pi R^2),   keep = lambda_kept / lambda = (1 - exp(-x)) / x,  x = lambda pi R^2
```

V-equivalent: the 2026 arXiv paper 2601.06396 gives `(1 - e^(-lambda0 V_d(rho)))/V_d(rho)` with `V_2 = pi R^2` and the
retention probability `(1 - e^(-lambda0 V))/(lambda0 V)`; arXiv 1103.1598 states the same planar form (both U, search
extracts). Its ceiling is `1 / (pi R^2)` as `lambda -> inf`. Inverting: `lambda = -ln(1 - T pi R^2) / (pi R^2)`, which
exists only for `T < 1/(pi R^2)` and blows up near it.

**With random radii** (our `R_ij = crown_k (r_i + r_j)`, size-ranked priority): Mansson and Rudemo 2002 give the
retention of a point of radius r as an integral over the radius distribution (U: only cited through a 2018 paper; the
formula was not read). No closed form for our case; and priority by size (big wins) makes small trees' keep lower than
big trees'.

**With clustering** (Thomas offspring at sigma 4-12 m): the local intensity inside a cluster is about
`mu / (2 pi sigma^2)`, far above the mean, so the keep ratio is *lower* than the Poisson closed form at the same mean.
(Derived, D.)

**With rounds > 1:** re-admission raises the keep toward random sequential adsorption. RSA of equal disks jams at
coverage 0.547 (Wang 1994, arXiv cond-mat/9402066: 0.547069; Hinrichsen et al. 1986: 0.547 +- 0.003; U, abstracts).
For hard-core distance R (disks of radius R/2) that is `2.19 / (pi R^2)`, 2.19x the Matern II ceiling. (Derived, D.)

**Finding 3.** No closed form covers clustered + random radii + size priority + k rounds. Two standard ways out:

1. **Bake-time calibration** (recommended): run the pass on a sample of tiles per class (or on every tile, since the
   bake already runs the whole world: E2m 57.8 s), measure `keep_class = winners / candidates`, and correct the
   intensity once, `lambda_pre := T / keep_class`. Because keep falls as intensity rises, one correction undershoots;
   a fixed-point iteration `lambda_{n+1} = lambda_n x T / winners_n` converges in 2-4 steps when `T` is well below the
   ceiling (D: keep is smooth and monotone in lambda). Deterministic: same inputs, same result, into the product hash.
   This is the "artist iterates" loop of UE PCG and the Foliage Spawner, done by the bake.
2. **Closed form as a first guess** (Matern II with the class's mean `R`), then one calibration step. Cheaper for
   tests; inaccurate near the ceiling.

## 3. Feasibility: the target must fit the crowns

The ceiling depends on the crown sizes, so a class target is reachable only if the sizes allow it. With today's canopy
row (`scatter_species.toml:71-73`, V: `crown_a_m 0.75`, `crown_b 0.2`, `crown_k 0.8`), two trees of height h conflict
at `R = 0.8 x 2 x (0.75 + 0.2 h) = 1.2 + 0.32 h` m. Ceilings (D):

| h (m) | R (m) | Matern II ceiling `1/(pi R^2)` (/ha) | RSA jamming `2.19/(pi R^2)` (/ha) |
|---|---|---|---|
| 5 | 2.8 | 406 | 889 |
| 10 | 4.4 | 164 | 360 |
| 20 | 7.6 | 55 | 121 |
| 40 | 14.0 | 16 | 36 |

Taiga and rainforest at 250-300/ha are reachable only if the class's typical canopy is about 10-13 m or smaller (k
rounds), or 7 m or smaller (round 1 alone). A 1-40 m reverse-J size range with the same mark everywhere caps dense
classes. Forestry says the same thing from the other side: Reineke's self-thinning rule `N = k d^-1.605` (stems per
area against quadratic mean diameter, fully stocked even-aged stands; slopes -1.42 to -1.79 by species, Pretzsch and
Biber 2005; U, abstracts) ties stem density to tree size: dense stands are stands of smaller trees.

**Finding 4.** The spec must state, per class, that `T` is below the ceiling of its size distribution, and pick what
gives: either the size distribution depends on the target (e.g. `size_max` or the reverse-J rate per class so that
`T` sits at a set fraction of the ceiling), or `T` is clamped to the ceiling with a warning in the bake log. The
calibration (Finding 3) cannot reach a target above the ceiling; it must detect it (winners stop rising with lambda)
and report, not loop. `canopy_stems_per_ha_max` then becomes either the clamp or is deleted as redundant.

**Lower layers.** Understorey and ground flora live under canopy light (Design 7) and have no class stem targets in
the spec (only canopy has "Counts per biome class"); their density is a cover, closer to HZD's density-as-output.
Keeping them per species is consistent with every tool above (per-layer density, artist-set). (D.)

## 4. Load cut by area

| Engine | Technique | Mark |
|---|---|---|
| UE5 PCG runtime generation | the domain is partitioned into grid cells (grid size per graph, on the PCG World Actor; the Grid Size node before samplers); a cell is generated when it is within the **Generation Radius** of a generation source (streaming source, PCG generation source component, or the editor viewport), one radius per grid size, increasing with size; cleaned up beyond radius x Cleanup Radius Multiplier | U (Epic docs summary; the page was not read in full) |
| UE World Partition | streaming sources load cells within a radius per grid | U |
| Horizon Zero Dawn | world maps streamed in sections; placement on the GPU around the player | U |
| Ours | `big-world-rendering.md` BW-3: `InstanceSource::tile(key, tile_m, ...)` per ring tile (`:181`, V); BW-2 measured the stand case over budget, BW-3 did not start ("Tripped", `:292-297`, V) | V |

**Finding 5.** The standard is "full density within a radius of the source, nothing (or a coarser LOD) beyond", with
the radius per cell size so big objects reach further. Our interim (WG-E2p) is the same idea at load time: keep the
whole kept set within `scatter.full_radius_m` of one focus (the start position) and apply today's rank budget beyond
it. It is a named scaffold: removal path BW-3's `InstanceSource` per ring tile around the moving camera. A per-group
radius (canopy further than ground flora) follows UE's radius per grid size. Budget arithmetic for the spec (D, from
E2n: x10s class targets ask 8.89 M canopy at 81/ha mean): at 81/ha a 1 km radius is 3.14 km2 = ~25k canopy, 2 km
~100k; the canopy group's share of `scatter.budget` (200k today) decides the radius, not the reverse.

## What the SPEC row must decide (from these findings)

1. `w = su x D_noise x prior` once, at the parent; `D` keeps the noise mosaic and loses `su` (or the reverse), plain
   `offspring_mean` (Finding 2).
2. `mean_class(w)` from a guide census at bake, in the product hash.
3. `keep` by bake-time fixed-point calibration per class, capped iterations, closed form as the first guess
   (Finding 3).
4. A ceiling rule per class: size distribution follows the target, or clamp + bake warning; fate of
   `canopy_stems_per_ha_max` (Finding 4).
5. WG-E2p: `scatter.full_radius_m` (unit m, Guess, source: UE PCG generation radius idea), focus = start position,
   rank cut beyond; per-group radius optional (Finding 5).

## Sources

- Epic, PCG Surface Sampler settings (Python/C++ API): https://dev.epicgames.com/documentation/en-us/unreal-engine/python-api/class/PCGSurfaceSamplerSettings (U)
- Epic, PCG Density Filter: https://dev.epicgames.com/documentation/unreal-engine/API/Plugins/PCG/UPCGDensityFilterSettings (U)
- Forum, surface sampler max points: https://forums.unrealengine.com/t/max-number-of-point-s-in-surface-sampler/1841227 (U)
- Epic, PCG generation modes: https://dev.epicgames.com/documentation/unreal-engine/using-pcg-generation-modes-in-unreal-engine (U)
- Epic, runtime hierarchical generation: https://dev.epicgames.com/documentation/unreal-engine/runtime-hierarchical-generation (U)
- Epic, Open World Tools property reference (Procedural Foliage): https://dev.epicgames.com/documentation/unreal-engine/open-world-tools-property-reference-in-unreal-engine (U)
- van Muijden, GPU-Based Run-Time Procedural Placement in Horizon Zero Dawn, GDC 2017: https://www.guerrilla-games.com/media/News/Files/GDC2017_VanMuijden_GPUBasedProceduralPlacementInHorizonZeroDawn.pdf (U)
- Gain, Long, Cordonnier, Cani, EcoBrush, CGF 2017: https://people.cs.uct.ac.za/~Jgain/wp-content/papercite-data/pdf/gain2017.pdf (U)
- Cordonnier et al., Authoring landscapes by combining ecosystem and terrain erosion simulation, TOG 2017: https://cs.purdue.edu/homes/bbenes/papers/Cordonnier17ToG.pdf (U)
- spatstat.random `rThomas`: https://search.r-project.org/CRAN/refmans/spatstat.random/html/rThomas.html (V, doc text)
- Matern II intensity: https://arxiv.org/pdf/2601.06396 , https://arxiv.org/pdf/1103.1598 ; generalisations and Mansson-Rudemo random radii: https://arxiv.org/pdf/1209.2566 , https://arxiv.org/pdf/1709.05868 (U)
- RSA disk jamming 0.547: https://arxiv.org/pdf/cond-mat/9402066 ; https://en.wikipedia.org/wiki/Random_sequential_adsorption (U)
- Reineke's rule re-evaluated (Pretzsch and Biber 2005): https://www.waldwachstum.wzw.tum.de/fileadmin/publications/576.pdf ; https://en.wikipedia.org/wiki/Stand_density_index (U)
