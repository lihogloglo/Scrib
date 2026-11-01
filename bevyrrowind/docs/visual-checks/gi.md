# Visual checks: global illumination (`superfargoth/docs/specs/gi.md`)

### GI-1: Dynamic probe GI, Census office (slices 1-2)
- Kind: game location, High preset, raster. Run: `.\bevyrrowind view "Seyda Neen, Census and Excise Office"`;
  A/B live with the console (backquote) `flip gi` (or start with `--off gi`). Log: `gi: volume AxBxC probes at S m (worldspace ...)`,
  then `gi: probe_update N ms, ...; lights 21 (0 changed), sun no` once per second.
- Look for: warm bounce on walls, ceiling and under tables near the candles and lamps (stronger than `--off gi`);
  a carried torch's bounce follows it within a few frames; no black or flat-coloured box over the room, no flicker
  or crawl; after a door into another interior, that room lights without a dark flash once the fade lifts. Known limits: light
  leaks through thin walls, a volume fitted to the cell bounds (outside it: today's ambient).
- Status: partial (owner 2026-11-02): no errors, but toggling GI barely changes the room; the exterior fps drop seen after leaving is not the GI (SH-22).

### GI-2: DDGI visibility sampling, Census office (slice 3)
- Kind: game location, High preset, raster. Run (dev exe): `powershell -File bevyrrowind/tools/run_dev.ps1 view "Seyda Neen, Census and Excise Office" --resolution 1280x720`;
  A/B with `flip gi` in the console or a second run with `--off gi`. Log: `gi: volume traced, sampled by the PBR shader (DDGI probes)`,
  then `gi: probe_update N ms, 512 probes, 32768 rays, atlas 105x21 tiles; ...` once per second; no `naga`/`wgsl`/validation error.
- Look for: no light leaking through the floor or walls of the office (no glow under walls, no bright patches in the
  rooms next to a lamp's room); bounce reaches the corners and under tables; no black blotches or blocky 2 m steps;
  compare `--off gi`. Known limits: hard switch to today's ambient at the volume's edge, no specular occlusion yet.
- Status: partial (owner 2026-11-02): no blotches or flicker seen, but the GI is too weak to judge leaks; revisit once it is visible.
