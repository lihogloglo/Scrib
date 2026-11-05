# Visual checks: global illumination (`superfargoth/docs/specs/gi.md`)

### GI-1: Dynamic probe GI, Census office (slices 1-2)
- Kind: game location, High preset, raster. Run: `.\bevyrrowind view "Seyda Neen, Census and Excise Office"`;
  A/B live with the console (backquote) `flip gi` (or start with `--off gi`). Log: `gi: volume AxBxC probes at S m (worldspace ...)`,
  then `gi: probe_update N ms, ...; lights 21 (0 changed), sun no` once per second.
- Look for: warm bounce on walls, ceiling and under tables near the candles and lamps (stronger than `--off gi`);
  a carried torch's bounce follows it within a few frames; no black or flat-coloured box over the room, no flicker
  or crawl; after a door into another interior, that room lights without a dark flash once the fade lifts. Known limits: light
  leaks through thin walls, a volume fitted to the cell bounds (outside it: today's ambient).
- Status: partial (owner 2026-11-02): toggling GI barely changes the room; with `--ambient 0 --range 1.5` the difference exists but is mild (a bit more in dark areas). Diagnosis: `.sessions/gi-weak.md` (cell ambient + unshadowed long-range lamps swamp a physically small bounce). Exterior fps drop is not the GI (SH-22).

### GI-2: DDGI visibility sampling, Census office (slice 3)
- Kind: game location, High preset, raster. Run (dev exe): `powershell -File bevyrrowind/tools/run_dev.ps1 view "Seyda Neen, Census and Excise Office" --resolution 1280x720`;
  A/B with `flip gi` in the console or a second run with `--off gi`. Log: `gi: volume traced, sampled by the PBR shader (DDGI probes)`,
  then `gi: probe_update N ms, 512 probes, 32768 rays, atlas 105x21 tiles; ...` once per second; no `naga`/`wgsl`/validation error.
- Look for: no light leaking through the floor or walls of the office (no glow under walls, no bright patches in the
  rooms next to a lamp's room); bounce reaches the corners and under tables; no black blotches or blocky 2 m steps;
  compare `--off gi`. Known limits: hard switch to today's ambient at the volume's edge, no specular occlusion yet.
- Status: partial (owner 2026-11-02): no blotches or flicker seen, but the GI is too weak to judge leaks; revisit once it is visible.

### GI-3: GI replaces the flat ambient inside its volume, Census office
- Kind: game location, High preset, raster. Run (dev exe): `powershell -File bevyrrowind/tools/run_dev.ps1 view "Seyda Neen, Census and Excise Office"`;
  A/B with `flip gi` in the console (on: probe bounce, no cell ambient inside the volume; off: today's cell ambient).
  Log: `gi: volume traced, sampled by the PBR shader (DDGI probes)`; no `naga`/`wgsl`/validation error.
- Look for: with GI on, rooms read darker in unlit corners and warmer near lamps than before (GI-1/2 runs); no hard
  box edge visible inside the office; nothing black or flickering. If corners go too dark, say so: the game's
  `INTERIOR_AMBIENT_SCALE` (0) can take a small floor. Known limit: dark probe patches (no relocation yet).
- Tune live: `render.gi_intensity 2` (try 1-4), `render.gi_ambient 0.1`; report the value you like.
- Status: confirmed (owner 2026-11-02: looks good; defaults kept, `render.gi_intensity` 1, `render.gi_ambient` 0).

### GI-lampbody: lamp bodies don't light twice at GI hits, ambient floor 0.1, Census office
- Kind: game location, High preset, raster. Run (dev exe): `powershell -File bevyrrowind/tools/run_dev.ps1 view "Seyda Neen, Census and Excise Office"`;
  stand with the chandelier (or a lantern) in view and A/B with `flip gi` in the console. Log: no `naga`/`wgsl`/validation error.
- Look for, GI on: the warm bounce on the ceiling and walls right around a lamp is less hot than before (the flame mesh
  no longer adds to its own point light in the probes), while the room fill elsewhere stays; the lamps themselves still
  glow exactly as with GI off. Unlit corners are dim but not pitch black (interior ambient floor 0.1).
- Tune live: `render.gi_ambient 0` / `0.1` / `0.2` compares the floor; report the value you like.
- Status: awaiting owner.
