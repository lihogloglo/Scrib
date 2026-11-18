# World generation: history (`../worldgen.md`)

Superseded handoffs and as-built history moved out of the spec, newest first.

## Handoff (2026-11-18, end of the first worldgen session)

State: the owner judges looks in `bevyrrowind worldgen-view <guide dirs> --markers <markers.json>` (launched through
`owner_launch.sh`; keys 1-9 switch arms). Current best: WG-VO's volcano on S4 C1 (`target/worldgen/vo/V1-V4`, params
`bevyrrowind/worldgen/red-mountain-vo.toml`), owner "very promising". Open look gripes (later): the volcano reads as
"different cones merged together" (artificial); the littoral is smooth and undeveloped; relief gets dull down the
slopes; landscape arms have too little detail below 300 m and too much 1-3 km relief vs real Etna/Teide (WG-M,
`target/worldgen/m/README.md`), real Iceland rejected as a reference. Metrics only pick knobs; the owner judges.

**Screenshot milestone (owner 2026-11-18: "get people excited")**, the next session's work, three slices in the game
(not the viewer: it has no textures, clouds or Morrowind meshes):
1. **Whole 10x island as game terrain** (owner: no crop). WG-4 + WG-5 minimal: `sf_terrain` stores from a coarser
   finest level (~2 m; 0.5 m everywhere would be ~58 GB) and the levels below come from the WG-F filter at runtime
   (GPU fill + CPU twin), fed through a `TerrainSource` over a guide v1 grid (`--world <guide dir>`).
2. **Settlements stamped in** (WG-8 + WG-A1/A3 minimal): named settlement clusters (`markers.json`) placed at 10x
   positions at 1:1 size with their source ground stamped and blended; their references streamed at the new positions.
   Doors and scripted coordinates (WG-A3b) wait. Careful: some settlements sit on geology (Vivec in a bay, Balmora on
   both banks of the Odai): a stamp is enough for now (adapter "Hard cases").
3. **Ground textures and one tree species** (WG-N1/N2 minimal): layers from slope/altitude/region mapped to Morrowind
   LTEX; one Morrowind tree mesh scattered by simple rules on the static scene + far field.
Also: raise the volumetric cloud base (`sf_atmos` `CloudSettings::base`) so the 5.5 km summit can pierce the layer.
The 3D viewer lives in `mw_viewers_bevy` for now; a standalone tool would move it to its own crate.
