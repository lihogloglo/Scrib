# Visual checks: streaming

Open visual checks: streaming spikes, size-based draw distance, door links (ST, SC, DR). Index and how-to: [`../visual-checks.md`](../visual-checks.md).

## Streaming spikes (superfargoth `streaming.md`, "Spikes"; `.sessions/spikes.md`)

### ST-2: No pop at origin shifts in flight
- Kind: game location. Run: `.\bevyrrowind view "Seyda Neen" --bench benchmarks/exterior_flight.json --bench-ground`
  (60 m/s), watch the whole flight.
- Changed: at a floating-origin shift (every 256 m) terrain patches move in place
  instead of respawning, terrain clipmap uploads wait two frames, and object
  streaming (spawn, reveal, despawn) holds for up to 6 frames around the shift.
- Look for: no terrain flicker or one-frame holes when crossing 256 m borders; no
  objects arriving visibly late (pop-in in front of the camera) compared with before.
- A/B: none at runtime (`StreamingConfig::origin_shift_guard_s = 0` in code).
- Owner 2026-11-12 (Bitter Coast flight, also seen before): a little flicker in distant terrain every ~200 m of travel.
  Owner suspects distant shadows mishandling the origin shift (every 256 m); not diagnosed, check later.
  Cause (FLICKER-DIAG + owner bisect 2026-11-12): DLSS/TAA history is reset at every origin shift (`sync_origin`);
  seen on terrain and water, never when still, gone with F4 Off.
- Fix FLICKER-1 (2026-11-12): shifts keep TAA/DLSS history, motion vectors rebased by the origin delta (`OriginRebase`).
  Launch `bash bevyrrowind/tools/owner_launch.sh view "Seyda Neen" --hour 10`, fly west over the Bitter Coast high and
  fast. Look for: no periodic pop on distant terrain, shadows or water. Must not: smears or ghost trails for one frame at
  shifts. A/B `--off shiftrebase` brings the pop back. Log: `origin shift A -> B: rebase ... (history kept)` per shift.
  Water's own motion vectors get the rebase after W-GERSTNER lands (until then water may still pop/smear at shifts).
- Status: confirmed (owner 2026-11-12, Seyda Neen 10:00 flight west: "the flicker is gone"; `--off shiftrebase` A/B not run).
- Source: spikes.

### ST-3: Paced model loads arrive in time
- Kind: game location. Run: `.\bevyrrowind view "Seyda Neen" --bench benchmarks/exterior_flight.json --bench-ground`
  (60 m/s), then the 7 m/s run (`--bench benchmarks/exterior_run.json`).
- Changed: models load only while the meshes of the models already loading stay under 64
  (`StreamingConfig::max_loading_meshes`, estimated from a running mean of meshes per model; never
  capped in burst mode); the rest queue, so a cell's objects can be revealed a few frames later than before.
- Look for: no objects popping in near the camera (inside ~100 m) that didn't before;
  far objects fading in at their usual distance.
- A/B: `$env:BEVYRROWIND_LOADING_MESHES=0` before the run restores unpaced loads.
- Status: deferred 2026-10-19 (phase0): recheck in the Phase 1 perf pass.
- Source: spikes2. Renamed by spikes4 (the budget counts meshes, not models).

## Size-based draw distance (superfargoth `industry-audit.md` J1; `.sessions/sizecull.md`)

### SC-2: The cutoff fades, never pops (Low preset)
- Kind: game location. Run: `.\bevyrrowind scene sizecull-balmora-low`, then
  back away from a market stall or a pile of crates (S, or Shift+S).
- Look for: small props dither out over the last 10 % of their draw distance
  (a 6 cm cup ~23 m, a 45 cm barrel ~170 m, at Low); nothing pops; buildings,
  doors and NPCs never go.
- A/B: add `--off sizecull` (everything to its ring or the 300 m handoff).
- Status: deferred 2026-10-19 (phase0): recheck in the Phase 1 perf pass.
- Source: sizecull.

## Door links (superfargoth `streaming.md`, "Worldspaces at runtime"; `.sessions/interiors.md`)

### DR-1: Through a door into an interior
- Kind: game location. Run: `.\bevyrrowind scene door-balmora-guild`, press
  Space (Morrowind's activate key) facing the door, within 4 m.
- Look for: a short fade (0.3 s out, 0.3 s in; longer only if the interior
  wasn't prefetched); you stand at the arrival marker inside the guild hall,
  level, facing into the room (the marker's heading); the cell's ambient light
  and dark clear color, no sky, sun, clouds, terrain or sea showing through;
  the window title names the interior. `door-balmora-guild-auto` does the
  same by itself.
- Mouse after arrival (fixed 2026-10-12, doorfix): move the mouse right after
  the fade lifts, and also keep moving it during the fade. The view must turn
  smoothly from where you arrived facing, never snap back to the heading you
  had before the door. Before the fix, Bevy's `FreeCamera` kept its own
  yaw/pitch, so the first mouse motion jumped to the pre-door direction (the
  "sudden turn", and the "wrong facing" when the mouse moved during the fade).
- Height: the camera arrives 0.5 m above the marker (scaffold until the
  character controller grounds the player). Most markers float ~1.2 m above
  the sill, so that's eye height; a marker authored low can still leave the
  camera near the floor (the Census Office's other door, DR-3).
- Status: partial, owner 2026-10-19 (round 8, several round trips): travel works, but see DR-4 (proxy shapes first, fade not black, arriving while the interior still loads, crash).
- Source: interiors, doorfix.

### DR-2: Back out
- Kind: game location. Run: as DR-1, then turn to the door you came in by and
  press Space. Hands-off: `.\bevyrrowind view Balmora --camera "-316.6,10.4,200.5,-312.6,10.0,200.35" --hour 10 --use-door 2`.
- Look for: you arrive in front of the guild door outside, facing away from
  it; sky, clouds, terrain and the town are back at once; lighting and
  exposure match the start; mouse look as in DR-1.
- What changed (doorfix): the exterior you left stays resident (hidden) while
  you stand within 32 m of a door back to it, and for 5 s after leaving it
  (`SchedulerConfig::linger_secs`), so the way out no longer reloads the near
  ring under the fade. Measured: out 0.64 s (was 1.02 s), in 0.63 s. Also try
  staying inside a minute near the door, then leaving: still fast. Walk more
  than 32 m from the door in a large interior and wait 5 s: the exterior
  streams out, and leaving takes about 1 s again.
- Status: partial, owner 2026-10-19 (round 8): works, see DR-4.
- Source: interiors, doorfix.

### RG-1: Nothing pops in after a reveal (reveal gate, pipeline-warmup W1)
- Kind: game location, cold (delete `%LOCALAPPDATA%\NVIDIA\DXCache` and `\GLCache` first) then warm. Build: `cargo build -p bevyrrowind --features dev`.
- Run: (1) release start, New game into Seyda Neen; (2) `.\bevyrrowind view "Seyda Neen"`, walk into "Seyda Neen, Census and Excise Office"; (3) back out. High preset, then Low.
- Look for: when the fade (or the loading screen) lifts, every object, texture and shadow near you is already there: nothing appears one by one. Holds may be longer than before, never over ~10 s.
- Log: one `reveal: door|loading held X s (cells, textures, assets, pipelines, gates ...)` line per reveal (which term held), its `+10.0 s: after_reveal N` line (target 0); any `reveal gate: ... not ready after` warning names what was pending (send it).
- Status: awaiting owner (2026-11-11). Source: warmup-w1.
