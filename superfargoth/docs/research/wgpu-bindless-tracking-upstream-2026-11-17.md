# wgpu bindless tracking and Bevy shadow material cost: upstream state (2026-11-17)

Research only. "fetched" = page opened and read; "snippet" = search snippet only. No memory citations.

## 1. Verdict

- The cost is known upstream. wgpu's own bindless design doc says binding a bindless bind group is "far too expensive" because of tracking (fetched, https://hackmd.io/@cwfitzgerald/wgpu-bindless).
- The exact `flush_bindings_helper` / `register_init_action` loop we found was reported in #9738 (wgpu 29.0.3, 87% of time, 30 -> 120 fps when the loop is removed). The issue is closed with NO linked fix PR; the only remedy offered is a bind group ordering workaround.
- Nothing found that makes a many-texture bind group O(1) per `set_bind_group` or at submit. No flag or unsafe opt-out found. Open perf issue #8452 (submit validation over bind groups only) is unmerged.
- Bevy: nothing merged that gives shadow/depth passes a material-less pipeline. #16077 open, #11647 open (merge conflicts), #26149 and #26161 closed unmerged 2026-10-30.
- Best lever is on our side: bind group order (rebind start index) and fewer encoders or lower slab limit; see section 4.

## 2. wgpu findings

| Item | Link | State | Release | What it changes | Fetched |
|---|---|---|---|---|---|
| cwfitzgerald "wgpu bindless" design doc | https://hackmd.io/@cwfitzgerald/wgpu-bindless | proposal, undated | none | Says tracking is the biggest complaint; proposes read-only usage pinning so tracked resources drop "10,000+ to 100 or less"; Round 2 moves validation to bind group validity | fetched |
| #3637 Bindless Tracking Issue | https://github.com/gfx-rs/wgpu/issues/3637 | open (opened 2023-04-02) | n/a | Umbrella; `Buffer/Texture::set_usages` idea to skip per-resource barrier checks. No measurements on large arrays. "GPU Validation" and "Sparse Bind Groups" unchecked | fetched |
| #9738 87% of time in `flush_bindings_helper` | https://github.com/gfx-rs/wgpu/issues/9738 | closed "completed" 2026-07-02 by reporter, no fix PR linked | none | 840k `register_init_action` calls/frame. Maintainer: there is a fast path when nothing was rebound; workaround: put static material bind group at index 0, per-mesh group at index 1 | fetched |
| #8418 Defer bind group processing to draw/dispatch | https://github.com/gfx-rs/wgpu/pull/8418 | merged 2025-10-23 | not stated on page | Moved init-action registration to draw time; source of the regression | fetched |
| #8499 Massive perf regression binding large bind groups | https://github.com/gfx-rs/wgpu/issues/8499 | closed 2025-11-14 | v28 milestone per page | Bindless benchmark 4-5 orders of magnitude slower after #8418 | fetched |
| #8519 Fix perf regression in bind group processing | https://github.com/gfx-rs/wgpu/pull/8519 | merged 2025-11-14 | not stated | Init actions only when bindings changed; reuse usage scope in compute pass. Most speed recovered, ~10% left in compute encode | fetched |
| #8452 Iterate only bind groups when validating a submission | https://github.com/gfx-rs/wgpu/pull/8452 | open (2025-10-31), no PR/branch | none | Would stop `validate_command_buffer` visiting every buffer and texture. This is the submit-side item that matches our 2.4 ms submit | fetched |
| #9653 Fix partially-bound texture binding arrays (DX12) | https://github.com/gfx-rs/wgpu/pull/9653 | merged 2026-06-29 | v30.0.0 changelog | DX12 correctness only; no perf change | fetched (page) |
| #10003 validate all command buffers before updating init trackers | https://raw.githubusercontent.com/gfx-rs/wgpu/trunk/CHANGELOG.md | merged, in "Unreleased" section | after 30.0.0 | Correctness; changelog says no expected perf impact | fetched (first 100k chars only) |

Queue submit scaling / per-command-buffer Transit encoder: no dedicated issue found. Only #8452 (above) addresses submit-time iteration. Trunk changelog Unreleased perf entries (#10913 GLES empty command buffers, #10319 coalesced queue-write barriers) do not touch Vulkan submit of many encoders.

## 3. Bevy findings

| Item | Link | State | What it changes | Fetched |
|---|---|---|---|---|
| #16077 Batch shadow draws for different materials with same depth shader | https://github.com/bevyengine/bevy/issues/16077 | open, S-Ready-For-Implementation, no PR | Idea only; prior art is #11647 | fetched |
| #11647 shadow performance (robtfm) | https://github.com/bevyengine/bevy/pull/11647 | open, S-Merge-Conflicts, last activity 2026-08-22 | `Material::shadow_material_key` so opaque StandardMaterials share one shadow material; sorts shadow items by mesh id. JMS55 called the key "hacky", suggested separate shadow bindings. Reported 50 -> 64 fps in a real scene | fetched |
| #26149 Run point/spot shadow views in own schedule (`Core3dShadows`) | https://github.com/bevyengine/bevy/pull/26149 | closed unmerged 2026-10-30 | 16 point lights 11.51 -> 10.54 ms (RTX 5070 Ti, Vulkan). Reviewer: single-threaded executor could save ~15% | fetched |
| #26161 Skip re-rendering unchanged point/spot shadow maps | https://github.com/bevyengine/bevy/pull/26161 | closed unmerged 2026-10-30 | Static 16 lights 11.50 -> 2.51 ms. Depends on #26149 | fetched |
| #26432 Correct tracking of bindless binding entry insertion | https://github.com/bevyengine/bevy/pull/26432 | open (2026-11-05), S-Needs-Review | Fixes slab over-counting resources when one material binds the same texture in several slots (issue #26429) | fetched |
| `AUTO_BINDLESS_SLAB_RESOURCE_LIMIT` | https://docs.rs/bevy_render/latest/src/bevy_render/render_resource/bindless.rs.html | n/a | 64 on macOS/iOS, 2048 elsewhere; doc text only says "default number of resources in a slab". A search snippet (not a page read) attributes the low limit to wgpu performance with many bindless resources | fetched (constant) / snippet (reason) |
| #23436 skip unused bindless sampler/texture arrays (holg) | https://vat.pages.dev/bevyengine/bevy/pull/23436 | state unknown | Fewer binding arrays per layout | snippet only |
| #21961 prepass skipping logic (pcwalton) | https://backiee.wasmer.app/https_git_codeproxy_net/bevyengine/bevy/pull/21961 | state unknown | Prepass specialisation gating on `prepass_enabled`; not about the bind group | snippet only |

(a) Material bind group in depth-only/shadow passes: no issue or PR found that removes `SetMaterialBindGroup` for opaque materials. (c) Nothing on `main` after 0.20 found that reduces per-view encoder cost for shadows; the two candidates were closed.

## 4. Portable to wgpu 30.0.1 / Bevy 0.20 as a vendored patch

1. **Reorder bind groups so the slab sits at the lowest index** (no wgpu patch). Per maintainer in #9738, `flush_bindings_helper` re-registers every texture of every bind group from the rebind start index; a high-index static slab is re-walked whenever a lower group changes. Bevy fixes material at group 3 (snippet: MATERIAL_BIND_GROUP_INDEX = 3, not verified at source). Test in our vendored bevy_pbr by moving the material group to index 0 or 1. Verify the fast path exists in 30.0.1 `command/pass.rs` before trusting it.
2. **PR #8452 (iterate only bind groups at submit)** is the one to port or finish: it targets the submit validation walk, but it is only a proposal, so it is new work, not a cherry-pick. Needs bind groups invalidated when a contained resource is destroyed.
3. **PR #11647 (Bevy shadow material batching)** is portable in spirit to cut batch sets and could also let shadows use a shared small material; it conflicts with current main, so expect a rebase.
4. **Bevy #26149 and #26161** are closed but their diffs are readable; #26161 (skip unchanged shadow maps) is the biggest published win for many lamp faces and is the same idea as our lamp cache.
5. Slab limit: upstream default is 2048 and its stated reason is wgpu cost with many resources. Our 8192 limit goes against that; an A/B at 2048 or 4096 is cheap.

## 5. Not found / open

- Any published ms measurement of wgpu tracking in a real Bevy scene. Only the #9738 callgrind numbers (outside Bevy).
- How #9738 was resolved (closed with no PR); whether the fast path is in 30.0.1; whether trunk after 30.0.0 changed `flush_bindings_helper` (only first 100k chars of the changelog were read, 30.0.0 and Unreleased covered).
- wgpu flag or unsafe opt-out for tracking: none seen in changelog sections read.
- Transit encoder cost: no issue found.
- Searches run: wgpu binding array tracking set_bind_group bindless; bevy bindless shadow prepass material bind group; wgpu flush_bindings_helper register_init_action; bevy slab limit; bevy prepass skip material bind group; bevy bindless render thread slow; GitHub issue list query for bindless tracking. Discord and blogs: nothing found.
