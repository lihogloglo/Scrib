# Rock typology per geology unit and climate (research, 2026-11-27)

Owner ask (2026-11-26): "add more typology of rocks if needed, depending on the biome / geology; our mesh vocab is
very limited". This doc answers three questions for the SPEC item that follows it (`.ralph/fix_plan.md`):

1. Which rock forms occur in nature on each unit of `bevyrrowind/worldgen/geology.toml` (basalt_flow, tuff, tephra,
   capped_sediment, coastal_basalt, alluvium), and how each climate class of
   `superfargoth/crates/sf_worldgen/src/climate_classes.toml` changes them.
2. How shipped games and engines organise a rock vocabulary per biome.
3. For each form: can `sf_rockgen` make it now from data, does it need a new parameter, or a new generator (or is
   it not a rock mesh at all: a material, a placement rule, a landform).

Marks: **V** = source read (page or slides opened, or the code in this repo); **U** = known only from a search
snippet, a talk title or general knowledge, not read in full. Numbers here are inputs to the spec, not decisions.

## 1. Summary

- Nature's rock vocabulary is set by **lithology first, climate second**: the unit decides the fracture pattern
  (joints, columns, vesicles, beds), the climate decides how far edges round, what cavities form and what grows on
  top. That splits our work cleanly: unit -> shape class (`sf_rockgen` geometry), climate -> a modifier on the class
  (rounding, chips, pits) plus a **surface layer** (moss, lichen, varnish), which is a material job.
- `sf_rockgen` already covers most forms as data: its 9 classes and 5 formations (section 4) span fresh blocks,
  slabs, clinker, scoria, columns, cobbles, hoodoos, stacks, necks and arches. The gaps are few and specific:
  **(a)** cavities on the sides of a rock (tafoni / honeycomb), **(b)** bedding grooves on a boulder (sediment and
  tuff blocks), **(c)** a lumpy welded clump (spatter / agglutinate), **(d)** a stacked-corestone tor, **(e)** a
  second material layer by up-facing normal and cavity (moss, lichen, ash dust), and **(f)** irregular, curving
  entablature columns.
- Games organise rock vocabulary **per biome as a kit in size tiers** (large formation / cliff, medium boulder,
  small debris), shared base materials with biome variants, and placement rules per biome (ecotope) that pick from
  the kit. That is exactly the shape of our `[rock_units]` table (unit x context -> class or pack model entries),
  with climate as a second key we do not have yet.

## 2. Rock forms in nature, per unit

Scales: S = pebble-cobble (< 0.3 m, terrain detail), M = boulder (0.3-3 m, scatter), L = block / outcrop (3-10 m),
X = formation (> 10 m, hero or landform).

| Unit | Forms that occur | Scale | Sources |
|---|---|---|---|
| basalt_flow | **'A'a clinker**: very rough, spiny, irregular blocks over a massive core (flow top and base rubble); **slabby / ropy pahoehoe** crust plates broken and tilted; **fresh joint blocks** (rockfall from flow fronts and scarps); **columnar prisms** in thick ponded flows (colonnade straight, entablature irregular and hackly above it); **tumuli** (domes < 10 m) and **pressure ridges** with cracked crust; **squeeze-ups, hornitos** (spatter chimneys on pahoehoe); **lava tubes** | S-X | NPS lava flow forms + surface features U; USGS Bulletin 994 (a'a core between clinker) U; Walker 1991 Bull. Volc. (tumuli vs pressure ridges) U; NPS / Oregon State columnar jointing U |
| tuff (welded / bedded ash, ignimbrite) | **rounded, soft-edged blocks** (weak rock rounds fast); **tafoni and honeycomb** cavities (salt and wet-dry cycles; cm cells to m alcoves); **hoodoos / fairy chimneys / tent rocks** under a harder cap (Cappadocia, Arizona ignimbrite); **bedded blocks** showing welded vs unwelded beds as grooves; columnar jointing in thick welded ignimbrite (coarser, less regular than basalt) | M-X | Wikipedia Tafoni, Honeycomb weathering U; NPS tafoni U; Dorn 2023 fairy chimneys (ASU) U; Wikipedia Hoodoo U; ignimbrite columns: general knowledge U |
| tephra (loose ash, lapilli, scoria) | **scoria clasts** (vesicular, dark, irregular, light); **volcanic bombs** (spindle, ribbon, cylindrical from fluid magma; **breadcrust** bombs cracked after landing; cored bombs); **blocks** (ejected solid, angular); **spatter / agglutinate** (welded blobs near vents, spatter cones vs loose cinder cones); grain size fines away from the vent (bombs near, ash far) | S-M (X: cones are landforms) | NPS pyroclasts U; SDSU How Volcanoes Work U; Wikipedia Volcanic bomb, Lapilli U; UCSB volcanic particles U |
| capped_sediment (hard cap over soft beds) | **cap-rock slabs** fallen from mesa and cuesta rims (tabular, square-jointed); **flagstones / platy blocks** from the soft beds; **bedded blocks** with recessed soft layers; **talus aprons** of cap blocks over the soft slope; **hoodoos / pedestal rocks** where the cap breaks into small pieces; **arches** in jointed caprock | M-X | Wikipedia Hoodoo U; landforms spec `landforms.md` (mesas, cuestas) V; research `terrain-sharp-features-2026-11-19.md` §4 (joint families + bedding) V |
| coastal_basalt (columnar, flat-lying) | **column segments** (fallen prisms, often with a cross-joint "ball and socket" end) on boulder beaches, **wave-rounded** to subrounded columns and cobbles; **sea stacks** and **arches** (Reynisdrangar: basalt stacks, remains of sea cliffs); **wave-cut notches** at the foot; **black basalt boulder beaches** (cobbles rounded by wave abrasion); stepped column "pavements" (Giant's Causeway) | S-X | Wikipedia Reynisdrangar / Reynisfjara U; Arctic Portal 2026 Reynisfjara erosion (boulders rolled along the beach) U; Goehring & Morris 2008 JGR (column order, 5-6 sides) U, cited in `classes.rs` V |
| alluvium (river and swamp deposits) | **rounded to well-rounded river boulders and cobbles** (rounding rises with transport and with softness of the rock); **imbricated, flattened cobbles** in channels; few large rocks except **corestones and erratics** let down from upstream; in swamps almost no surface rock (rock = buried) | S-M | Wikipedia Roundness (geology) (rounding vs transport, hardness, size) U; desert pavement / EBSCO (water-laid pavements are rounded, mixed origin, near channels) U |

Two cross-unit facts the spec should keep: (1) **rounding is a property of history, not of the unit alone**:
fresh fall blocks are angular on every unit, transported ones round (fluvial, wave), weak rock rounds faster
(Roundness, U). (2) **Columnar jointing has two styles in one flow**: regular colonnade below, irregular entablature
(often 2/3 of the flow) above (NPS columnar jointing, Oregon State, U). Our `columnar` class is the colonnade style.

## 3. Climate modifiers

The 12 climate classes fall into five weathering regimes (Peltier's 1950 morphogenetic regions are the classic
grouping, U; the per-regime processes below are from the cited sources). Shape effects go on the class as parameter
deltas; surface effects are material.

| Regime | Climate classes | Shape | Surface (material) | Placement |
|---|---|---|---|---|
| Frost | tundra, taiga | **angular, frost-shattered** blocks, many fresh facets, little rounding (angular boulders = frost shattering) | crustose **lichen** (lichen on rock in 5 +- 1 yr, Tatra), moss in taiga shade | **blockfields (felsenmeer)** on plateaus and gentle slopes: dense fields of angular blocks |
| Arid | desert, cold desert | subangular; **tafoni / honeycomb** (salt weathering), **ventifacts** (wind-cut facets, mostly in situ) | **rock varnish** (dark Mn-Fe-clay coat, decades to millennia) on stable tops; no growth | **desert pavement**: packed pebbles and cobbles on fans |
| Semi-arid | shrubland, grassland, savanna, cold-desert edge | **exfoliation sheets, spheroidal boulders, tors** of stacked corestones (granite-like weathering also seen on massive basalt and welded tuff, U) | sparse lichen, varnish on old surfaces | scattered boulders, outcrop tors |
| Temperate humid | woodland, temperate forest, seasonal forest | rounded edges, few chips (chemical + frost) | **moss on tops and shaded faces** under canopy, lichen in the open (shade and litter smother lichen, favour bryophytes) | boulders under canopy hold more moss species (deciduous > spruce) |
| Wet | temperate rainforest, rainforest | strong chemical weathering: **rounded corestones**, deep regolith, few fresh loose blocks | **heavy moss / epiphyte cover**, dark wet albedo | fewer loose rocks, most buried |

Sources: Britannica felsenmeer, Wikipedia Blockfield (frost, plateaus; a Neogene chemical-weathering alternative
exists) U; wits.ac.za thesis (angular = frost shattering) U; Wikipedia Tafoni / Honeycomb (salt, wet-dry; polygenetic)
U; Iranian ventifact study (in situ, lose shape in transport) U; Wikipedia Rock veneer / EPA (varnish = clay, Fe, Mn,
microbes) U; Glos. eprints boulder grazing refugia (shade, litter vs lichen; bryophytes) U; Tatra lichenometry
(Springer s11629-020-6130-3) U; SLU thesis (deciduous vs spruce bryophytes) U.

Vvardenfell is a game world, not a climate record: which classes its regions get comes from the climate rasters
(WG-E1c3, not built). The table is written for any game; the adapter picks contexts per unit.

## 4. What `sf_rockgen` can make today (code read, V)

- **Boulder SDF** (`boulder.rs`, `sdf.rs`): a joint-set block (0-3 families, `joint_jitter_deg`) or a column
  cluster (`columns` 1-40: Voronoi cells of a jittered hex lattice, `column_jitter`, `column_groove`, `column_step`,
  `column_tilt_deg`, `column_fuse`), cut by `chips` half-spaces (`chip_depth`, `chip_spread`), smooth-max
  `rounding`, fbm `weather_*` noise, minus **vesicle pits** (`pit_density`, `pit_size`: one pit per lattice cell
  with that chance, kept where it opens on the surface, so **uniform over the surface**, `sdf.rs:230-235`).
  `aspect` + `aspect_spread` set the axes.
- **9 classes** (`classes.rs`): eroded `block`, `platy`, `cobble`, `chipped`; sharp `talus`, `jagged` (aa
  clinker), `columnar`, `scoria`, `slab`. Each carries a `tint` (albedo multiplier).
- **5 formations** (`formations.rs`): `Stack` (strata bands with hardness recess, caprock, twist, lean, drift,
  flare skirt, wave notch) as `hoodoo`, `pinnacle`, `sea_stack`; `Neck` (tapered column cluster) as
  `volcanic_neck`; `Arch` (rib on flared legs). The SDF has unions of convex pieces and pits, **no general
  subtraction** (the arch comment says so, `formations.rs:12`).
- **Material** (`sf_rockgen_bevy/src/material.rs`): one triplanar texture set (albedo, normal, ORM) per rock
  material with hex tiling, plus the class tint. **No second layer** (moss / lichen / dust by up-normal), and the
  spec's vertex-colour cavity and strata index (`rockgen.md` "Materials") are **not built** (no colour attribute in
  `sf_rockgen_bevy`, grep V).

## 5. Each form: data, new parameter, new generator, or not ours

| Form | Verdict | How |
|---|---|---|
| Fresh fall block (any hard unit) | **Data** | `talus` |
| Frost-shattered angular block | **Data** | `talus` with more chips, higher `joint_jitter_deg`, `chip_spread` up |
| 'A'a clinker | **Data** | `jagged` (already the analogue) |
| Scoria clast | **Data** | `scoria` |
| Slabby pahoehoe crust plate | **Data** | `slab` with `weather_*` up, darker tint |
| Ropy pahoehoe surface | **Material** | a ropy normal/height texture on the slab; geometry noise is isotropic (no directional fbm) |
| Volcanic bomb, spindle / cylindrical | **Data** | `cobble` with a long `aspect` ([0.45, 0.4]) and light pits |
| Breadcrust bomb (crack net) | **New parameter** | surface crack grooves (a few shallow slab cuts in a Voronoi pattern); or material only |
| Spatter / agglutinate clump | **New generator (small)** | union of 3-8 rounded blobs flattened by impact (`Solid` already unions pieces; a recipe like `Neck`) |
| Columnar prism, colonnade | **Data** | `columnar` |
| Single fallen column segment | **Data** | `columnar` with `columns` 1-3 and a long aspect (count range 0-40 allows it; check closure in a test) |
| Wave-rounded column (beach) | **Data** | `columnar` with `rounding` up and `weather_amp` up |
| Entablature (curving, hackly columns) | **New parameter** | column axis bend / fan per column (today straight with `column_tilt_deg`) |
| Soft tuff block | **Data** | `block` with high `rounding`, few chips, light pits |
| Tafoni (side alcoves) | **New parameter** | pit placement biased to side and underside faces + a larger size spread (pits are uniform today) |
| Honeycomb (cm cells) | **Data (approximate)** | `pit_density` high, `pit_size` small; becomes exact with the tafoni bias |
| Bedded block with recessed beds (sediment, tuff) | **New parameter** | bed bands on the boulder: `Stack`'s band recess applied to a joint block (beds along the platy axis) |
| Cap-rock slab, flagstone | **Data** | `slab`, `platy` |
| Ventifact | **Data (approximate)** | `block` with 2-4 deep chips on one side and low weathering (wind facets); polish is material |
| Spheroidal / corestone boulder | **Data** | `cobble` |
| Exfoliation sheets (onion skin) | **New parameter** | concentric shell offsets (nested rounded solids with gaps); low priority |
| Tor (stacked corestones) | **New generator (small)** | a `Stack` variant whose bands are rounded blocks offset by joints; formation recipe |
| River cobble / boulder, imbricated | **Data** | `cobble` flattened (`aspect` [0.75, 0.4]); imbrication = pose rule in the scatter (tilt toward flow), not geometry |
| Hoodoo, fairy chimney, tent rock | **Data** | `hoodoo` formation (tent rock = cone, small cap) |
| Sea stack, arch, wave notch | **Data** | `sea_stack`, `arch`, `Stack.notch_*` |
| Volcanic neck | **Data** | `volcanic_neck` |
| Hornito / spatter chimney | **New generator** | hollow needs subtraction (none in the SDF); a solid spatter cone works as the clump recipe scaled up |
| Tumulus, pressure ridge, lava tube, scoria cone | **Not a rock mesh** | landforms (`landforms.md`) and caves (RG-7) |
| Blockfield, desert pavement, talus apron | **Not a rock mesh** | placement: density and size law per unit x context in `[rock_units]` (`field_stone_per_ha`, size ranges) |
| Moss, lichen, varnish, ash dust, salt crust | **Material** | a second triplanar layer blended by up-normal, cavity and a per-context amount; needs the vertex-colour cavity of `rockgen.md` "Materials" |
| Grain size fining away from vents | **Not a rock mesh** | size law driven by distance to vent (a field the scatter would read; later) |

Count: of 31 forms, 18 are data now (2 approximate), 5 need a parameter (breadcrust cracks, entablature bend,
tafoni bias, bed bands, exfoliation shells), 3 need a generator recipe (clump, tor, hornito), 2 are material
(ropy surface; the moss / lichen / varnish layer), 3 are not rock meshes. **The vocabulary gap is mostly preset data and one material
layer, not new generators.**

## 6. Industry: how games organise a rock vocabulary

| Who | What they do | Mark |
|---|---|---|
| Horizon Zero Dawn (van Muijden, GDC 2017) | An **ecotope** describes an environment and determines asset types, distribution, colourisation, weather, effects, sound, wildlife; per-layer density map -> generate -> place on the GPU; 500+ asset types | ecotope list U (slides summary, PDF not opened this session); placement method V in `rockgen.md` Industry (research §4) |
| Ghost of Tsushima (Sucker Punch; B. Mead ArtStation) | A small team owned rocks, cliffs and peaks; a **base rock material** plus **rock biome variants** (biome materials credited to Mead, Lamb, Jenne); procedural tools to build the world fast | U (ArtStation and PlayStation Blog snippets) |
| Megascans / Fab (Quixel) | Packs sold **per biome**: Sandstone = "complete arid desert biome" with rock formations, cliffs and scattered rocks; Moab / Quarry Collections = 75 set-dressed **assemblies** per ecosystem; Mountain Slope = 136 rock meshes | U (listing snippets) |
| UE5 PCG third party (KK Design AutoEnv) | **Data-asset biomes** with control over 7 rock layers each | U (forum snippet) |
| Photogrammetry kits (DICE Battlefront GDC 2016, Embark, 80.lv) | Scanned rocks in size tiers, modular pieces plus "gap hider" medium rocks | U |
| RDR2, Witcher 3, Elden Ring | No public talk on their rock vocabulary found this session | U (nothing found) |
| Morrowind (vanilla, this repo's data) | **Regional terrain rock sets**: `terrain_rock_<region>_NN` statics (e.g. `terrain_rock_ac_01` Azura's Coast, `terrain_rock_wg_01`/`_05` West Gash), used as fake cliffs; thousands of **menhirs** on Azura's Coast; 33,017 rock references learned by `mw_worldgen::flora` | V (`classify.rs:118-131`, `worldgen-adapter.md:67,216`, `worldgen-scatter.md` Adapter data); the full region-code list waits on the LF-A1 census |
| Tamriel Rebuilt / Tamriel_Data | Shared asset repository; **region rock and cliff packs** (e.g. Cathnoquey `pi_terr_cq_clfNN` cliffs, `pi_terr_cq_rockNN` rocks; Bloodmoon rock collection) | U (tamriel-rebuilt.org assets page snippet; UESP Tamriel Data) |
| OpenMW | Engine only: renders whatever statics the content places, no rock vocabulary of its own | general knowledge U |

**Pattern:** per biome a kit in size tiers (formation / cliff, large, medium, small debris), shared base
material with biome variants, and placement rules per biome (ecotope) choosing from the kit. Morrowind does the
same by region (one rock set per region). Our design maps to it as: kit = classes + pack models in
`[rock_units]`, size tiers = the scatter's size law, ecotope = unit x context, biome material variant = texture
layer per unit + the moss/lichen layer per climate context. The missing key is **climate context**: today
`[rock_units]` is keyed by unit and by placement context (talus, outcrop, field) only.

## 7. Inputs for the SPEC item

- New generic classes, as data over existing parameters: `frost_block` (talus, more chips), `bomb` (long cobble),
  `column_segment` (1-3 columns, long), `beach_column` (rounded columnar), `soft_block` (tuff), `ventifact`,
  `river_cobble` (flattened cobble), `crust_plate` (pahoehoe slab). Names generic, numbers `Guess`.
- New parameters, one row each: tafoni pit bias (`pit_side_bias`, `pit_size_spread`), boulder bed bands
  (`beds`, `bed_recess`), column bend (entablature), crack grooves (breadcrust); exfoliation shells last.
- New recipes, one row each: `clump` (spatter / agglutinate), `tor` (stacked corestones).
- Material row: second triplanar layer (moss / lichen / dust) blended by up-normal and cavity, amount per climate
  context; it depends on the vertex-colour cavity of `rockgen.md` "Materials" (a row of its own, first).
- Adapter data: `[rock_units]` gains an optional climate-regime key (frost, arid, semi-arid, temperate, wet) over
  the placement contexts; per unit the class lists of section 2; the source's pack models stay (owner 2026-11-26).
- Not ours: tumuli, pressure ridges, cones (landforms); lava tubes (RG-7); blockfields and pavements (scatter
  density data); vent-distance fining (later field).

## Sources

- NPS, Lava flow forms / Lava flow surface features / Inflation structures / Basaltic lava flows: https://www.nps.gov/articles/000/lava-flow-forms.htm, https://www.nps.gov/articles/000/lava-flow-surface-features.htm, https://www.nps.gov/articles/000/inflation-structures-lava-rise-plateaus-inflation-pits.htm (U)
- USGS Bulletin 994 (a'a structure): https://pubs.usgs.gov/bul/0994/report.pdf (U)
- Walker 1991, Bull. Volcanol. (tumuli vs pressure ridges): https://link.springer.com/doi/10.1007/BF00298155 (U)
- Wikipedia, Pressure ridge (lava): https://en.wikipedia.org/wiki/Pressure_ridge_(lava) (U)
- NPS, Columnar jointing: https://www.nps.gov/subjects/volcanoes/columnar-jointing.htm; Oregon State: https://volcano.oregonstate.edu/columnar-jointing; Michigan Tech Keweenaw: https://www.geo.mtu.edu/KeweenawGeoheritage/BlackLavas/Columnar_Joints/Columnar_Joints.html; USGS CRBG flow interior: https://or.water.usgs.gov/projs_dir/crbg/flowinterior.html (U)
- Goehring & Morris 2008, JGR (column scaling): https://www.physics.utoronto.ca/~nonlin/preprints/GM_JGR08.pdf (U)
- Wikipedia, Tafoni: https://en.wikipedia.org/wiki/Tafoni; Honeycomb weathering: https://en.wikipedia.org/wiki/Honeycomb_weathering; NPS tafoni: https://www.nps.gov/articles/tafoni.htm (U)
- Dorn 2023, Arizona ignimbrite fairy chimneys: https://www.public.asu.edu/~atrid/Dorn_FairyChineys_2023.pdf (U)
- Wikipedia, Hoodoo: https://en.wikipedia.com/wiki/Hoodoos (U)
- NPS Pyroclasts: https://www.nps.gov/subjects/volcanoes/pyroclasts.htm; SDSU Tephra: https://volcanoes.sdsu.edu/Tephra.html; Wikipedia Volcanic bomb: https://en.wikipedia.com/wiki/Lava_bomb; Lapilli: https://en.wikipedia.org/wiki/Lapilli; UCSB volcanic particles: https://volcanology.geol.ucsb.edu/frags.htm (U)
- Wikipedia, Reynisdrangar: https://en.wikipedia.com/wiki/Reynisdrangar; Reynisfjara: https://www.Wikipedia.org/wiki/Reynisfjara; Arctic Portal 2026 erosion: https://arcticportal.org/ap-library/news/4027-the-famous-reynisfjara-beach-dramatically-transformed-by-coastal-erosion (U)
- Wikipedia, Roundness (geology): https://en.wikipedia.org/wiki/Roundness_(geology) (U)
- Desert pavement: https://www.ebsco.com/research-starters/environmental-sciences/desert-pavement, https://en.wikipedia.com/wiki/Gibber; Rock veneer: https://www.Wikipedia.org/wiki/Rock_veneer; EPA: https://semspub.epa.gov/work/09/2065237.pdf; ventifacts (Iran): https://journals.ui.ac.ir/article_28045_081e2119c1f88a098867e69191f96a7a.pdf (U)
- Britannica, Felsenmeer: https://www.britannica.com/science/felsenmeer; Wikipedia Blockfield: https://www.wikiwand.com/en/articles/Blockfield; Wits thesis: https://wiredspace.wits.ac.za/bitstreams/7aba524d-43ba-46f1-9f90-db4e88eeb193/download (U)
- Boulder lichen / bryophytes: https://eprints.glos.ac.uk/4336/3/The%20natural%20exclusion%20of%20red%20deer%20from%20large%20boulder%20grazing%20refugia%20and%20the%20consequences%20for%20saxicolous%20bryophyte%20and%20lichen%20ecology.pdf; Tatra lichenometry: https://link.springer.com/article/10.1007/s11629-020-6130-3; SLU: https://pub.epsilon.slu.se/13/ (U)
- Peltier 1950, "The geographic cycle in periglacial regions..." (morphogenetic regions): not opened (U)
- Horizon Zero Dawn, van Muijden GDC 2017: https://www.guerrilla-games.com/media/News/Files/GDC2017_VanMuijden_GPUBasedProceduralPlacementInHorizonZeroDawn.pdf (V via `rockgen.md`; ecotope list U)
- Ghost of Tsushima: https://blakeovitch.artstation.com/projects/0nl2mE, https://tsmith3d.artstation.com/projects/yk42A3, https://blog.playstation.com/?p=333111 (U)
- Megascans / Fab: https://www.unrealengine.com/marketplace/en-US/product/megascans-sandstone, https://www.unrealengine.com/marketplace/en-US/product/moab-desert-collections, https://www.unrealengine.com/marketplace/en-US/product/quarry-collections; KK Design AutoEnv: https://forums.unrealengine.com/t/kk-design-autoenv-procedural-rock-cliffs/2646777 (U)
- Photogrammetry: https://80.lv/articles/building-rocks-for-games-with-photogrammetry, https://80.lv/articles/modular-photogrammetry-in-environment-design/, https://es.slideshare.net/DICEStudio/photogrammetry-and-star-wars-battlefront (U)
- Tamriel Rebuilt assets: https://tamriel-rebuilt.org/assets/terrain?page=9; Tamriel Data: https://en.m.uesp.net/wiki/Tamriel_Data:Main_Page (U)
- This repo (V): `superfargoth/crates/sf_rockgen/src/{boulder.rs,classes.rs,formations.rs,columns.rs,sdf.rs}`,
  `sf_rockgen/src/boulder/tunables.rs:163`, `sf_rockgen_bevy/src/material.rs`, `superfargoth/docs/specs/rockgen.md`,
  `worldgen-scatter.md` "Adapter data", `bevyrrowind/worldgen/geology.toml`, `mw_worldgen/src/classify.rs:118-131`,
  `bevyrrowind/docs/specs/worldgen-adapter.md:67,216`.
