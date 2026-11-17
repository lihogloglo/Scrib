# Performance architecture review, 2026-11-17

Read-only review asked by the owner: why the 120 fps target is still missed, and whether the cause is the
architecture, outdated solutions, tuning or the hardware. No code change, no build, no bench run, no game launch.
Sources: `docs/ledger/perf.md`, the three 2026-11-16 audits in this folder, per-frame CSVs of saved runs under
`bevyrrowind/target/perf/`, wgpu-core 30.0.1 and our vendored Bevy 0.20 source, and two research notes written today
(`superfargoth/docs/research/wgpu-bindless-tracking-upstream-2026-11-17.md`,
`superfargoth/docs/research/depth-only-no-material-2026-11-17.md`). Labels: **code read** = I read the lines cited;
**measured** = a number from a saved run; **estimated** = my reading, not measured.

## 1. Short answer

- The design is not wrong. Streaming rings, precombine, far field, GPU preprocessing, bindless materials, the cascade
  hold and pipelined rendering are what shipped engines do. Nothing here needs a rewrite.
- The frame is held by **three lanes that all sit above the target at once** (measured, `f_on`): walk main 9.41 /
  render 9.75 / GPU 7.31 ms; standing main 7.50 / render 7.95 / GPU 8.97 ms; target 8.3 ms. Fixing one lane exposes
  the next, which is why single fixes kept reading "parity".
- One cost was misattributed. About half the render thread (finish 2.3 + submit 2.4 ms) is wgpu's resource tracking,
  and it scales with **the number of textures in the bindless material slab**, not with batch sets (§2). This is new.
- Six fixes built on 2026-11-16 are all off by default and unmeasured (§4). The shipped frame has barely changed.
- The hardware is part of it (a 2022 laptop CPU), but the GPU was not held back by a power cap (§3, corrected).

## 2. New finding: the material slab is re-walked by wgpu, texture by texture

**Mechanism (code read, wgpu-core 30.0.1).** Every `StandardMaterial` lives in one bindless bind group of about 2,700
texture views (`superfargoth/vendor/bevy_pbr/src/pbr_material.rs:25-34`; arrays are not padded, `PARTIALLY_BOUND` is
on: `vendor/bevy_render/src/material_bind_groups.rs:1662-1669`). wgpu visits every one of those views:
- on each `set_bind_group` in a render pass (`command/pass.rs:111-117`, loop in `track/texture.rs:363-370`);
- again when bindings are flushed after a lower-index group or the pipeline layout changed (`command/pass.rs:144-166`,
  one `register_init_action` per view; the material group is index 3, so a change of the mesh group at 2 re-walks it;
  rebind rule in `command/bind.rs:207-251`);
- again per command buffer at finish and at submit (`docs/reviews/submit-cost-diagnosis-2026-11-16.md` §4).

**Upstream knows (fetched by the research agent, see its note).** wgpu issue #9738 reports this exact loop at 87 % of
frame time in another app (30 -> 120 fps with the loop removed); closed without a linked fix. The maintainers' own
bindless design note calls binding a bindless group "far too expensive". No fix, flag or opt-out exists in wgpu 30.

**Evidence in our own runs (measured, walk means, no new run).**

| pair (same exe each) | render graph ms | batch sets | implied ms per batch set |
|---|---:|---:|---:|
| PBR maps on -> off (`lean_base` -> `lean_pbr`) | 7.64 -> 5.40 | 163 -> 116 | 0.048 |
| 2 slabs -> 1 slab (`slab_off` -> `slab_on`) | 7.97 -> 7.07 | 168 -> 134 | 0.026 |
| flat normal map (`b2_off` -> `b2_on`) | 6.63 -> 6.55 | 97 -> 91 | 0.013 |

A batch set cannot cost 13, 26 and 48 us in the same engine: batch sets are not the unit. The first row is the one
where the slab's texture count fell (maps off took the materials from two 2048-slabs to one). The ledger's "PBR maps
cost 2.7 ms because variants split batches" is better read as "PBR maps tripled the textures wgpu walks".

**A wasted bind (code read).** Bevy 0.20 already has a material-less depth path for opaque casters
(`DrawDepthOnlyPrepass`, empty layout at group 3: `vendor/bevy_pbr/src/prepass/mod.rs:413-416, 456-463, 1651-1658`;
shadow use at `render/light.rs:3660-3668`). `StandardMaterial` never takes it, because it declares a prepass fragment
shader (`pbr_material.rs:1464`) and `prepass_reads_material()` then returns true (`material.rs:1903-1912`). So every
cascade and every lamp face binds the whole slab to draw opaque geometry that samples nothing.

**Not known.** The size in ms. No trace so far can see it: every trace was a span trace (Tracy), which stops at
`encoder.finish()` and `queue.submit`. A sampling profile sees inside wgpu and settles it in one minute.

## 3. Hardware and settings (read today with `nvidia-smi`, `powercfg`, WMI)

- Laptop: ASUS TUF A15 FA507NVR, Ryzen 7 7435HS (Zen 3+, 8 cores), RTX 4060 Laptop. Windows plan: Balanced.
- GPU power (corrected the same day; my first reading was wrong): the driver reports a default limit of 80 W and a
  maximum of 140 W, but the per-run `gpu.csv` of `f_on` shows the GPU was not capped at 80 W. Standing (94 % busy): mean
  87.6 W, max 94.8 W, 2531 MHz. Walk (77 % busy): mean 75.9 W, max 122 W, 2555 MHz. The "77 W" of the frame-waste audit
  is the mean of a CPU-bound run. Flag `0x400` on 98 % of loaded samples: not decoded (not one of the documented bits I
  know). The owner confirms G-Helper Turbo, plugged in. No free GPU gain to expect from the power mode.
- CPU: a 2022 laptop core. The same exe on a current desktop CPU would run each CPU lane roughly 1.5x faster
  (estimated, from public single-thread scores, not measured here) and would already pass 120 fps.
- Honest reading: 120 fps on this laptop is reachable with a thin margin. 140 fps is not, without dropping features.

## 4. Built, off, unmeasured (ROADMAP 1.25)

CW-2 selection gates (expected -0.5 ms main), CW-4 lamp cache, CW-6 material controller gate (-0.2 to -0.5 render),
CW-7 skin skip (-0.3 to -0.6 render, -0.2 to -0.4 main), CW-9 interiors park the exterior, CW-10a water gate (-0.3 to
-0.6 GPU). All opt-in, all "pending A/B". `tunables/cw-gates-on.toml` (PP-5) turns them on together. Until they are
on, the frame the owner feels is the 2026-11-15 one.

## 5. What is structural (Bevy and wgpu, not our design)

1. wgpu sits between the engine and Vulkan and tracks every resource of every bind group. Shipped engines have no
   such layer. Measured share: finish + submit = 4.7 of 9.75 ms on the render thread.
2. Every view is a full schedule and its own encoder: 33 views and 39 encoders on a day walk, 30 of them lamp faces.
3. Visibility is tested per entity per view on the main thread (2.5-3.0 ms, ledger 2026-11-15).

None of the three is fixed by another gate on a 0.2 ms GPU pass. 1 is §2; 2 is the lamp work already planned
(1.23b day gate, CW-4); 3 is 1.22 F6.

## 6. Plan, ranked

| # | step | lane | size | cost |
|---|---|---|---|---|
| 1 | Turn on the built gates in normal play (`cw-gates-on.toml`), keep what looks right | all | est. 1.5-3 ms summed over lanes | owner, play session |
| 2 | One 60 s sampling profile of the walk (samply; needs an elevated shell) | render | sizes §2 exactly | 10 minutes, one run |
| 3 | `StandardMaterial` takes Bevy's material-less depth path for opaque casters, opt-in switch | render | unknown until 2; removes the slab from every shadow view with no alpha-tested caster | 15-30 lines in vendored `bevy_pbr`, 1 test (design: research note) |
| 4 | Lamp shadows gated by day contribution (1.23b, CW-3) | render, main | removes ~30 views and the 1.16 ms caster scan by day | planned, not built |
| 5 | From the profile: fewer re-walks in the main pass (bin order, or a vendored wgpu-core patch on the flush loop, #9738) | render | unknown until 2 | M; wgpu vendoring is the last resort |
| 6 | Main thread: 1.22 F6 (one camera + cascade scan, cached lamp caster lists) | main | est. 0.9-1.4 ms | planned |

Do not start: a one-pass lamp atlas, a static scene revival or more small GPU gates before step 2 has been read.
Step 3 touches vendored `bevy_pbr`, so it waits until the owner's visual-check session is over (a half-done edit
there would break that session's builds).

## 7. Not verified

- The ms size of §2 (needs step 2). The table in §2 is consistent with it and does not prove it.
- How many times per pass the slab is re-walked (depends on bin order; not counted).
- That maps-off shrinks the slab to about a third: inferred from the slab count falling 2 -> 1, not read from a counter.
- What throttle flag `0x400` means in the bench GPU samples.
- Step 3 leaves the slab bound in any shadow view that draws an alpha-tested caster (trees, grates). Its share is
  unknown; cascades probably keep it, lamp faces in town probably drop it.
- Research note risks for step 3: where to store the flag (`MaterialProperties` is in the unvendored
  `bevy_material`), a pipeline-catalog re-bake, surface-weather prepass keys.
