# Vvardenfell natural landmarks gazetteer (2026-11-18)

For the 10x regeneration lane (`sf_worldgen` / `mw_worldgen`): the natural features the generator must keep.

**Sources.** UESP wikitext (`en.uesp.net/wiki/Morrowind:<page>?action=raw`, fetched 2026-11-18): Places, Regions, Foyada
and each foyada page, the lake/river/bay pages, region pages (Ascadian Isles, Ashlands, Azura's Coast, Bitter Coast,
Grazelands, Molag Amur, Red Mountain, Sheogorad, West Gash), Ghostfence, and settlement pages. Cross-check:
`target/worldgen/vvardenfell/markers.json` (named cells from the game data) and `heights.f32` (sampled for altitude,
metres above sea level in the dump: sea 0, max 270.8 m at cell (3,9)). UESP place pages carry `coords=[x,y]` only for
some places; foyadas, rivers' courses and mountains' extents have no cells, so those are given from waypoint places
whose cells are confirmed. No map image was read; nothing is estimated from maps. Where a location is blank, UESP
gives none. "hm" = measured from heightmap sampled over the cell (mean, min..max m), a cross-check not a UESP claim.

Cells are (x, y) exterior grid. Vvardenfell in the dump spans cells x -28..23, y -19..28.

Key: imp = importance 1-3. U = UESP page `https://en.uesp.net/wiki/Morrowind:<name>`.

## Volcano and Red Mountain features

| Name | Kind | Cell(s) | Region | Shape notes | Imp | Source |
|---|---|---|---|---|---|---|
| Red Mountain | volcano (stratovolcano) | Heightmap summit cell (3,9) hm max 270.8 m (global max). Dagoth Ur facility (2,8) hm mean 212; Odrosal (3,7); Tureynulal (4,9); Vemynal (0,10) hm 138; Kogoruhn (0,14) on N foot. Bounding cells by Ghostfence not given | Red Mountain | Dominates central island, "second highest mountain in Tamriel"; steep climbs, deep ravines (foyadas) and lava pools inside the Ghostfence; crater holds the Dwemer citadels and Dagoth Ur's facility; foyadas leave from its crater lip (SE, NW, S). Island's original name was Vvardenfell | 3 | U:Red_Mountain; markers (Dagoth Ur, Odrosal, Tureynulal, Vemynal, Kogoruhn) |
| Crater of Red Mountain (Dagoth Ur crater) | crater/caldera | around (2,8)-(3,9) (location from markers + heightmap; UESP gives no cell) | Red Mountain | Blight-haze filled crater, ringed by the Ghostfence; rim "lips" in SE, NW and S where foyadas start | 3 | U:Red_Mountain, U:Ghostfence, U:Foyada_Ashur-Dan |
| Ghostfence | ridge/wall (artificial rim) | Ghostgate (2,4) on the S gap; Kogoruhn (0,14) is the N entrance. Full ring: not given | Red Mountain | Shield-like wall around the whole crater slope; only two ways in (Ghostgate S, Kogoruhn tunnel N). Marks the Red Mountain / Ashlands boundary | 2 | U:Ghostfence |
| Ashlands ash desert | ash field | region-wide, no cell | Ashlands | Largest contiguous region: dry ash desert, stone obelisks/menhirs, "bubbling ash mires", ash storms; slopes level off to Molag Amur lava fields in SE | 2 | U:Ashlands |
| Valley of the Wind | valley/foyada (J-shaped) | Cavern of the Incarnate (6,13) is its end; Dareleth Ancestral Tomb there (unverified cell) | Ashlands (NE) | "J-shaped" ravine leading to the Cavern of the Incarnate, NE of Red Mountain | 2 | U:Valley_of_the_Wind, U:Cavern_of_the_Incarnate |
| Crater next to Caldera | crater (closed basin) | centre (-2.8, 1.3), just SW of Caldera (-2,2) (found in `heights.f32` by priority-flood fill depth, 2026-11-18; UESP names none) | West Gash | ~170 m across, 29 m deep, floor 4.4 m, spills at 33.5 m: at 10x ~1.7 km across and ~290 m deep | 3 | heightmap; owner |

## Foyadas (lava channels / ravines)

UESP: "deep, ash-dark ravines" running from Red Mountain; good footing, clear of brush, long and used as roads. No cells
for any foyada; courses below are from waypoints UESP names (cells confirmed by markers or UESP `coords`).

| Name | Cells (waypoints) | Region | Shape notes | Imp | Source |
|---|---|---|---|---|---|
| Foyada Mamaea | Start: S lip of crater; passes through Ghostgate (2,4) (built in its centre); Assarnatamat shrine in its middle (0,0); Moonmoth Legion Fort (-1,-3) beside it (Dwemer bridge, Pelagiad-Balmora road crosses SE of fort); end "a bit east of Hla Oad" (Hla Oad (-6,-5)); end cell blank. UESP also says "ends just below Balmora" (inconsistent) | Ashlands / West Gash / Ascadian Isles boundary (Ashlands' S border follows it) | Longest foyada; runs S then SSW, ~14 cells; also forms the west edge of the Ascadian Isles. Marked red on the paper map | 3 | U:Foyada_Mamaea, U:Foyada |
| Foyada Ashur-Dan | From SE lip of crater into Molag Amur; no end cell. Dunirai Caverns lie between it and Esannudan near Ghostgate (SE of (2,4)) | Red Mountain -> Molag Amur | Orange on map; runs SE from crater. Its course also bounds Molag Amur's north edge ("turns north after the Foyada Ashur-Dan") | 2 | U:Foyada_Ashur-Dan, U:Molag_Amur |
| Foyada Bani-Dad | From NW lip of crater, passes just N of Maar Gan (-3,12), to the Sea of Ghosts coast N of Khuul (-9,16); Derelict Shipwreck (-7,18) at the sea mouth; Assurnabitashpi (-5,18); Urshilaku Camp (-4,18) "in a low hollow" east of it | Ashlands -> Sheogorad coast | Yellow on map; ravine to the northern sea, ends in a mouth with a landmark wreck. Longest N-going channel | 3 | U:Foyada_Bani-Dad |
| Foyada Esannudan | Branches off Mamaea N of Ghostgate, runs S/E, ends SE of Ghostgate (2,4); crosses the Ghostfence; cave Sharapli just S of the fence crossing. Conflicting in-game text calls Mamaea "Esannudan" | Red Mountain/Ashlands | Short (~3-4 cells), green on map. Between Mamaea and Ashur-Dan | 1 | U:Foyada_Esannudan |
| Foyada Zabirbael | Semicircular around Mount Kand (11,-5), separates it from Mount Assarnibibi (14,-4); per the Foyada page, "northeast from Lake Nabia (4,-2) past Mount Kand to Mount Assarnibibi" | Molag Amur | Purple on map; semi-circular path | 2 | U:Foyada_Zabirbael, U:Foyada |
| Foyada Ilibaal | From Molag Mar (12..13,-8) to Mount Kand (11,-5); cave of Assu (11,-5) along it | Molag Amur | Light blue; pilgrim route, "hardened lava river". Not on in-game maps (UESP) | 2 | U:Foyada_Ilibaal |
| Foyada Nadanat | Runs through Molag Amur to Erabenimsun Camp (13,-1) | Molag Amur | Dark blue; UESP stub, no more detail | 1 | U:Foyada_Nadanat; marker Erabenimsun Camp |

## Mountains and peaks

| Name | Cell(s) | Region | Shape notes | Imp | Source |
|---|---|---|---|---|---|
| Mount Kand | (11,-5) (UESP coords; marker matches). hm mean 94 m, max 112 | Molag Amur | Peak NW of Molag Mar; cliff racers on peak; Mount Kand Cavern at base; cavern of Assu (Staff of Magnus) "near the summit". Pilgrimage site; ringed by Foyada Zabirbael | 3 | U:Mount_Kand |
| Mount Assarnibibi | (14,-4) (UESP coords; marker matches). hm mean 70 m, max 96 | Molag Amur | Peak N of Molag Mar; Assarnibibi Shrine (Ebony Mail); NE of Kand across Zabirbael | 3 | U:Mount_Assarnibibi |
| West Gash mountain ranges | no cells | West Gash | "Between two mountain ranges, west of the Ashlands and east of the Bitter Coast"; rocky scrubland in S, "unique natural rock bridges" inland in the S and on the coast, sparse forest in N | 2 | U:West_Gash |
| Ashlands mountain wall (W) | no cells | Ashlands | "Forbidding steepness of the mountains of the Ashlands" bounds Grazelands on the W | 2 | U:Grazelands |
| Hills N of Lake Nabia | around (4,-2) (unverified extent) | Molag Amur | Molag Amur's northern boundary runs along these; steep hills, steam vents and lava pools near Uvirith's Grave | 1 | U:Molag_Amur, U:Uvirith's_Grave |
| Hill of Governor's Hall / Caldera hill | Caldera (-2,2) | West Gash | Town on hillside; Governor's Hall on a hill N of town | 1 | U:Caldera |
| Hill of Balmora High Town | Balmora (-4..-2,-3..-2 in markers) | West Gash | High Town on the hill W of the river | 1 | U:Balmora |

## Rivers

| Name | Cells | Region | Shape notes | Imp | Source |
|---|---|---|---|---|---|
| Odai River | UESP coords (-4,-4). Source just N of Balmora (Balmora cells (-4,-2),(-3,-3),(-3,-2),(-2,-2)); through Balmora S to the Odai Plateau (-5,-5); mouth at Inner Sea "just south-southeast of Hla Oad" (Hla Oad (-6,-5)); mouth cell blank. Shulk Egg Mine (-4,-4) beside it | West Gash -> Ascadian Isles/Bitter Coast boundary | Mountainous, mine-dotted terrain around; has rope bridge; flows S through a valley between Balmora and the sea; "Odai valley/gorge" is a player-facing feature but UESP does not call it a gorge | 3 | U:Odai_River, U:Shulk_Egg_Mine |
| Nabia River | UESP coords (5,-4). Flows S from Lake Nabia (4,-2) to Lake Masobi (5,-7); Bal Ur (6,-5) and Vandus Ancestral Tomb on E bank; Suran (6,-7) at mouth. hm mean 28 m, up to 75 m | Molag Amur -> Ascadian Isles | "Very steep banks"; N-S, ~5 cells; marked flowing into Masobi's NE corner | 2 | U:Nabia_River |
| River Samsi (Ouada Samsi) | UESP coords (-10,10). Borders the peninsula of Gnisis (-11,10..11) and Ald Velothi (-11,15); bridges upstream; Gnisis Eggmine enterable from it | West Gash | Large waterway, "very steep banks", too shallow for boats; flows N to the Sea of Ghosts. Endpoints not given | 2 | U:River_Samsi |
| Channel Hairan-Masobi | between (2,-8) and (5,-7) | Ascadian Isles | River channel joining Lake Hairan's NE end to Lake Masobi | 1 | U:Lake_Hairan |

## Lakes

Heights from the heightmap (hm) cross-check: every Ascadian lake is at sea level; Nabia is slightly above.

| Name | Cell | Region | Altitude | Shape notes | Imp | Source |
|---|---|---|---|---|---|---|
| Lake Amaya | UESP coords (2,-5). hm mean 0, min -16 m | Ascadian Isles | sea level (lowland) | "Large lake" NE of Pelagiad (0,-8..-7); N shore Fields of Kummu (1,-5); S shore Dren Plantation (2,-7), Desolate Shipwreck (1,-6); Arvel Plantation (2,-6) across; joins Lake Masobi to the E per Masobi page; small island with mucksponges | 3 | U:Lake_Amaya, U:Lake_Masobi |
| Lake Hairan | UESP coords (2,-8). hm mean 3 | Ascadian Isles | sea level | Small, between Pelagiad and Vivec; Dren Plantation on N bank; NE end narrows to a river channel to Masobi; S end "merges with the vast lagoon encompassing Vivec" (i.e. open to sea, brackish) | 2 | U:Lake_Hairan |
| Lake Masobi | UESP coords (5,-7). hm mean 4 | Ascadian Isles | sea level | Small, W of Suran (6,-7); Nabia River enters NE corner; joins Amaya on W; Ules Manor (5,-7) on N shore | 2 | U:Lake_Masobi |
| Lake Nabia | UESP coords (4,-2). hm mean 7 m, up to 36 | Molag Amur | slightly above sea level, in uplands (hills to N) | Small lake N of Suran; Marandus (4,-3) on shore; source of the Nabia River; roads W to Balmora, N to Sulipund, E into Molag Amur | 2 | U:Lake_Nabia |

Not a lake on Vvardenfell: Lake Fjalding is on Solstheim (marker at (-21,23), outside scope).

## Bays, seas, coves, lagoons, coast

| Name | Cell(s) | Region | Shape notes | Imp | Source |
|---|---|---|---|---|---|
| Vivec's bay (Norvayn Bay; "the vast lagoon") | Vivec markers (2..5,-14..-9); Vivec (city) at (2,-10),(3,-9),(5,-10); Foreign Quarter (3..4,-10); Redoran (3,-11), St Delyn (3,-12), St Olms (4,-12), Telvanni (5,-11), Hlaalu (2,-11), Arena (4,-11) | Ascadian Isles | City on nine artificial islands in "Norvayn Bay"; bridges to mainland between Ebonheart peninsula (Hlaalu side) and N/E bay regions. Bay merges N into Lake Hairan | 3 | U:Vivec_(city), U:Lake_Hairan; markers |
| Zafirbel Bay | UESP coords (15,11). hm at (15,11) mean 2 m | Azura's Coast | "Large body of water" in the centre of the E coast; ~35 islands (some bare rock), Sadrith Mora (17..18,4..5) on largest island, Tel Aruhn (15,5), Tel Mora (13,14), Indoranyon (14,9); kollop beds in shallows | 3 | U:Zafirbel_Bay |
| Inner Sea | no cell | -- | Water between Vvardenfell and the mainland (S and W); "coast rugged and treacherous, shipwrecks scattered among menhirs"; ports: Ebonheart, Hla Oad, Gnaar Mok, Molag Mar, Tel Branora, Seyda Neen | 3 | U:Inner_Sea |
| Sea of Ghosts | no cell | -- | Ocean N and NE of the island; icebergs; meets Inner Sea along N mainland | 3 | U:Sea_of_Ghosts |
| Seyda Neen harbour / Grand Pharos | Seyda Neen (-2,-10..-9) | Bitter Coast | Harbour mouth guarded by the Grand Pharos lighthouse | 2 | U:Seyda_Neen; marker |
| Khartag Point | UESP coords (-9,4); marker matches | Bitter Coast (boundary; also listed West Gash) | Sea cliff NW of Gnaar Mok (-8,3); orcs cliff-jump; the northern boundary of the Bitter Coast. hm mean -3, min -13: mostly sea in the cell | 2 | U:Khartag_Point |
| Ebonheart island / peninsula | Ebonheart (1..2,-13) | Ascadian Isles | Castle on an island/peninsula W of Vivec; underground caves entered underwater at its N tip | 2 | U:Ebonheart |
| Inlet E of Molag Mar | Molag Mar (12..13,-8) | Molag Amur | UESP notes "an inlet" E of Molag Mar bounding the region (location only in prose) | 1 | U:Molag_Amur |
| Bitter Coast smuggler coves | no cells | Bitter Coast | "secluded coves and islands... hidden caves"; fog, rain | 1 | U:Bitter_Coast |

## Islands and island chains

| Name | Cell(s) | Region | Shape notes | Imp | Source |
|---|---|---|---|---|---|
| Sheogorad island group | Dagon Fel (7,22); Sanctus Shrine (1,21) on "a medium-sized island ... due west of Dagon Fel"; Ald Redaynia (-4,21); Vas (0,22); Ashmelech (2,19); Rotheran (6,18) (stronghold SW of Dagon Fel) | Sheogorad | ~28 islands off the N coast; "high pillar-like menhirs", hundreds of exposed/submerged rocks make navigation almost impossible; the only port is Dagon Fel on the main island; separated from the mainland by a sea channel (Derelict Shipwreck (-7,18)) | 3 | U:Sheogorad, U:Dagon_Fel, U:Sanctus_Shrine |
| Azura's Coast isles / Zafirbel Bay | see Zafirbel Bay; Shrine of Azura (19,-8); Holamayan (19,-4); Tel Fyr/Corprusarium (15,1); Telvanni Council House (17,5) | Azura's Coast | "Numerous islands S and E of Vvardenfell", incl. all southern islands E of Ald Sotha (6,-9), N up the east coast; three small peninsulas SE with the Shrine of Azura | 3 | U:Azura's_Coast |
| Sadrith Mora island | Sadrith Mora (17,4),(17,5),(18,4) | Azura's Coast | "The only really substantial island" in the bay; town accessible by sea or teleport only | 2 | U:Zafirbel_Bay, U:Sadrith_Mora |
| Tel Fyr island | (15,1) | Azura's Coast | Island just off the mainland S of Tel Aruhn; can swim from Tel Aruhn/Sadrith Mora; also walkable from Molag Mar? ("through Molag Amur") (unverified) | 1 | U:Tel_Fyr |
| Tel Branora island | (14..15,-13) | Azura's Coast | One of the southernmost, "on an island far to the southeast" | 1 | U:Tel_Branora; marker |
| Tel Aruhn / Tel Mora / Vos coast | (15,5) / (13,14) / (11..12,13..14) | Azura's Coast / Grazelands | Telvanni tower islands on the E coast (Tel Mora, Tel Aruhn); Vos harbour downhill from village | 1 | markers |
| Firemoth Region islands | Fort Firemoth (plugin, no cell) | Firemoth (Siege at Firemoth plugin) | Island chain off SW coast; plugin only (not in vanilla dump) (unverified location) | 1 | U:Vvardenfell; U:Fort_Firemoth |
| Ald Sotha island/peninsula | (6,-9) | Ascadian Isles | Daedric ruin marks the boundary between Ascadian Isles and Azura's Coast | 1 | U:Ald_Sotha; marker |
| Mzahnch / Zaintiraris coast | (8,-10), (12,-10) | Ascadian Isles | S coast islets with ruins (location only) | 1 | markers |
| Solstheim (Bloodmoon) | cells (-27..-17, 20..27) in dump | -- | Separate island NW of Vvardenfell; out of scope (markers present: Mortrag Glacier, Hvitkald Peak, Lake Fjalding...) | -- | markers |

## Swamps, marshes, mud and ash

| Name | Cell(s) | Region | Shape notes | Imp | Source |
|---|---|---|---|---|---|
| Bitter Coast swamps | Seyda Neen (-2,-10..-9) ... Hla Oad (-6,-5) ... Gnaar Mok (-8,3) ... Khartag Point (-9,4) are the stated points; bounding cells not given | Bitter Coast | "Humid, putrid swamps and salt marshes" along the SW coast; stagnant pools under ragged trees; big shallow coastal waters; fog and rain. Region runs from Khartag Point S/E to Seyda Neen, merging into Ascadian terrain W of Vivec | 3 | U:Bitter_Coast |
| Molag Amur lava pools and steam vents | region from Balmora E to the coast, N of Molag Mar; Uvirith's Grave (10,1) "steep hills, steam vents and large lava pools" | Molag Amur | "Distinguished by lava pools and rivers on the surface"; dark volcanic rock with ash overlay; irregular triangle with the point at Arkngthand (0,-2) | 3 | U:Molag_Amur, U:Uvirith's_Grave |
| Ashlands ash mires | region-wide | Ashlands | "Bubbling ash mires" and stone obelisks | 2 | U:Ashlands |

UESP does not name any individual mud pit, geyser or fumarole beyond the above.

## Plateaus, plains, fields

| Name | Cell | Region | Shape notes | Imp | Source |
|---|---|---|---|---|---|
| Odai Plateau | UESP coords (-5,-5); marker matches. hm mean 21 m, max 42 | Bitter Coast | W bank of the Odai River, SW of Balmora and E of Hla Oad; Rethan Manor (Hlaalu stronghold) on top; Shulk Egg Mine at (-4,-4) | 3 | U:Odai_Plateau |
| Fields of Kummu | UESP coords (1,-5); marker matches. hm mean -1 m | Ascadian Isles | N shore of Lake Amaya; shrine beside the road; saltrice field and dock to the E | 2 | U:Fields_of_Kummu |
| Grazelands (Sunna Guradan) | Falensarano (9,6) S edge; N to coast above Tel Vos (10,14); E to the coast; Ahemmusa Camp (11,16), Zainab Camp (9,10); Nchuleft (8,12); Kushtashpi (9,16); Indoranyon (14,9) | Grazelands | Pastoral plains in the NE, wickwheat grasslands, sparse trees, bounded on W by steep Ashlands mountains | 3 | U:Grazelands |
| Ascadian Isles lowlands | region from Ebonheart (1,-13) N to past Fields of Kummu (1,-5), W to Foyada Mamaea, E to Ald Sotha (6,-9) | Ascadian Isles | "Fertile, lush lowlands"; scattered islands and peninsulas; Lake Amaya the main obstacle | 2 | U:Ascadian_Isles |
| Balmora basin / hills | (-4..-2,-3..-2) | West Gash | Town straddles the Odai River; hill W of the river is High Town | 2 | U:Balmora |
| Caldera town | (-2,2) | West Gash | Mining town on hillside; Caldera Mine nearby; hill with Governor's Hall to N. "Caldera" is a name; no crater is attested (unverified) | 2 | U:Caldera |
| Ghostgate pass | Ghostgate (2,4); Tower of Dawn (E), Tower of Dusk (W) with temple between | Ashlands | Fortress built in the middle of Foyada Mamaea where it crosses the Ghostfence (i.e. a gap in the fence at the foot of the mountain). hm mean 32 m, max 70 | 3 | U:Ghostgate, U:Foyada_Mamaea; marker |
| Gnisis peninsula / bluff | Gnisis (-11,10..11), Ald Velothi (-11,15) | West Gash | Village "on a small plain on a hillside" overlooking the Samsi; peninsula bordered by the river | 2 | U:Gnisis, U:River_Samsi |
| Ald'ruhn slopes | (-3,6),(-2,6),(-2,7) | Ashlands | "Nestled against the southwest slopes of Red Mountain, just outside the Ghostfence"; manor district inside a giant emperor-crab shell | 2 | U:Ald'ruhn |
| Maar Gan | (-3,12) | Ashlands | Walled village closest to the Ghostfence besides Ghostgate | 1 | U:Maar_Gan |

## Cliffs, bridges, rock formations

| Name | Cell | Region | Shape notes | Imp | Source |
|---|---|---|---|---|---|
| Natural rock bridges (West Gash) | inland S and on the coast, no cells | West Gash | "unique, natural rock bridges"; location blank | 2 | U:West_Gash |
| Menhirs and sea stacks | Sheogorad coast; Inner Sea shipwreck zones | Sheogorad / Inner Sea | "High pillar-like menhirs" protruding from water | 2 | U:Sheogorad, U:Inner_Sea |
| Stone obelisks | Ashlands | Ashlands | "clusters of stone obelisks" | 1 | U:Ashlands |
| Cliff at Khartag Point | (-9,4) | Bitter Coast | Cliff above sea; an orc was dared to jump from it | 1 | U:Khartag_Point |
| Waterfalls | -- | -- | none found in UESP | -- | -- |

## Matches against markers.json

Gazetteer entries whose cell UESP agrees with a marker of the same name: Odai Plateau (-5,-5); Fields of Kummu (1,-5);
Mount Kand (11,-5); Mount Assarnibibi (14,-4); Khartag Point (-9,4); Ghostgate (2,4); Caldera (-2,2); Arkngthand
(0,-2: not a marker); Kogoruhn (0,14); Holamayan (19,-4); Tel Fyr (15,1); Ald Sotha (6,-9); Sanctus Shrine (1,21);
Ald Redaynia (-4,21); Urshilaku Camp (-4,18); Uvirith's Grave (10,1); Zergonipal (5,15); Nchuleft Ruin (8,12) /
Nchuleftingth (10,-3); Mzahnch Ruin (8,-10); Ashurnibibi (-7,-4) (a marker only; UESP lake/river pages do not use
it). Vivec cantons and Balmora, Ebonheart, Seyda Neen, Pelagiad, Molag Mar, Sadrith Mora, Dagon Fel etc. match by name
(see the tables).

Not in markers (UESP coords only, no cell marker): Lake Amaya (2,-5), Lake Hairan (2,-8), Lake Masobi (5,-7), Lake
Nabia (4,-2), Nabia River (5,-4), Odai River (-4,-4), River Samsi (-10,10), Zafirbel Bay (15,11), Cavern of the
Incarnate (6,13), Assarnatamat (0,0), Shrine of Azura (19,-8), Derelict Shipwreck (-7,18), Indoranyon (14,9).
Markers confirmed these UESP cells have game-data names only where the table says "marker matches".

## Gaps

- **Odai "gorge/valley":** UESP calls it a river with mountainous terrain; steepness and depth are not stated. The
  heightmap shows relief (cells (-4,-4), (-5,-5): max 29-42 m, min -4..-1), confirming a notable cut by the plateau.
- **Foyada end cells and widths:** no UESP cells for any foyada; Mamaea's southern end contradicts itself (below
  Balmora vs east of Hla Oad). Resolve from the heightmap (lowest linear trench) in the generator.
- **Red Mountain crater rim:** no UESP rim cells or depth; use the heightmap (summit (3,9), 270.8 m).
- **Waterfalls, geysers, named mud pits, arches/spires, natural bridges' cells:** none named on UESP; West Gash
  bridges and Sheogorad menhirs exist as descriptions only.
- **Odai River mouth, Samsi mouth/source, Nabia/Hairan channel cells.**
- **Unnamed lakes/ponds** in the Ashlands and Grazelands; small coastal lagoons on the Bitter Coast.
- **Other peaks:** UESP names only Kand and Assarnibibi; Ashlands/West Gash ranges unnamed. The Grazelands' western
  mountain wall and the Ashlands N coast cliffs have no names.
- **Sheogorad island names:** individual islands (beyond Sanctus Shrine's) are unnamed.
- **Official paper-map labels (Foyada Mamaea, Mount Kand, Mount Assarnibibi, etc.):** not independently read; UESP
  says Kand and Assarnibibi are on the paper map and Esannudan is not labelled.
- **Firemoth Region:** plugin content, absent from the dump.
- Solstheim and Mournhold are out of scope.
