# NV-R1: navmesh-rs upgrade, HZD flier field, tile rebuild cost (2026-11-26)

## Verdict
1. **navmesh-rs: upgrade 0.2.2 -> 0.2.6 and rebase our patch** (do not drop it: upstream `main` still has `DetachedQuery` as `pub(crate)`). 0.2.3 adds `NavMesh::replace_tile` / `replace_tile_budgeted`, the API NV10b was missing ("no TileCache API").
2. **HZD flier field**: our spec's mip idea is right, but its reading of HZD is partly wrong (per-size fields, not one class-free field; hierarchical capped A*, not a single-mip 8-connected A*; venue was Game AI North, not GDC). Fixes in section 2.
3. **Tile rebuild**: no published per-tile ms exists (searched); our own 3.75 ms/tile mean (19.2 m tile) is plausible. Off-frame worker build + swap at a boundary is what UE does; Unity's async API has the same shape (from memory, not fetched).

## 1. navmesh-rs 0.2.2 -> 0.2.6
Facts (crates.io API, upstream CHANGELOG, `src/detour/mod.rs` and `query.rs` on `main`):
- Releases: 0.2.2 (11-06), 0.2.3 (11-20), 0.2.4 (11-20), 0.2.5 (11-21), 0.2.6 (11-21). Licence MIT OR Apache-2.0 (all versions). Repo `Tavrin/navmesh-rs`. `libm` and `polyref64` features still exist in `detour/mod.rs`. No Bevy mention anywhere (still Bevy-free).
- **Our patch is not upstream.** `DetachedQuery` is still `pub(crate)` (`query.rs:1916` on main; not in the `pub use` list). Upstream's new `resumable` module covers raycast / polys-around / surface-move, not sliced `find_path`. Rebase = re-apply the ~30-line diff (2 files); `query.rs` changed a lot, expect drift. Offer it upstream as planned.
- **0.2.3 (large)**: `replace_tile`, `replace_tile_budgeted`, `replace_cost` (all-or-nothing swap in the same slot, one salt advance; identical content = no change; refused swap keeps the old tile): exactly our NV10b swap. Opt-in `RefLayout::WIDE_SALT` (more salt advances per slot, cap on polys per tile; matters for many rebuilds per slot) and `salt_headroom`. Managed runtime off-mesh connections (stable ids, availability, re-resolved on tile replacement and streaming) = our `LinkId` links. Tile-cache zones with `update_budgeted`; opt-in `dynamic` feature (gates and cost zones over baked polygons, no tile rebuild: an alternative to our `DoorPolys` area switch, not needed). Agent-size bake profiles, goal fields, `CrowdWorld` (we keep our ORCA).
- 0.2.4: `Selector::DetailPrism`, modifier edits. 0.2.5: tile state save/restore no longer re-applies stale gate effects. 0.2.6: opt-in bake extraction of ladders, stair steps, level-gap jumps (useful for NV links later).
- The changelog lists no tiled-mesh bug fixes; changes are additive. **Not verified**: whether 0.2.6 emits byte-identical tile blobs to 0.2.2 for the same input (our spec claims bit-exact for 0.2.2). Check before adopting: bake one tile set with both, compare blake3. If blobs differ, every nav cache invalidates (rebake ~216 s, acceptable).
- Risk: 4 releases in 15 days, one author. Keep the exact `=` pin, upgrade once in its own slice (`test -p sf_nav_recast` + the hash check) instead of waiting for the batched pass, because NV10b needs `replace_tile`.
- **Recommendation**: upgrade + rebase patch, build NV10b on `replace_tile_budgeted` (bounded swap per tick fits `ai.nav.rebuild.max_swaps`). Update `superfargoth/vendor/README.md` and the root `Cargo.toml` comment (both say 0.2.2).

## 2. Horizon Zero Dawn air navigation
Source: Josemans, "Putting the AI back into Air", **Game AI North, Copenhagen, 2017-10-17** (not GDC); slide text and notes read from the official pptx. Slide facts:
- **Rejected**: waypoint graphs (used before), dense voxels (too expensive), sparse voxel octrees (Warframe; "complex, unsure it fits our terrain"), multiple navmesh layers (listed, no reason given).
- **Field**: one height per cell = **highest collision in the cell**; cell size depends on the agent (Glinthawk cell = agent radius = 1.0 m); tiles **64 x 64** cells; **one field per agent size**. Min flying height = max collision + agent radius: physics polygons rasterised (max per cell), then **dilated by the agent radius** (van Herk/Gil-Werman) at generation. Limits: one height per cell (no flying under arches), no upper limit.
- **Generated at runtime**, one job per tile, only inside a "generation bubble" around agents, driven by streaming callbacks.
- **Mips**: max of 4 children down to 1 cell per tile, +1365 cells = +33 %. Stormbird: ~9 tiles resident, 150 KB, 190 KB with mips (float cells).
- **Queries**: safe-height lookup, Bresenham raycast, path.
- **Path**: first plain A* on the regular grid, 4-connected, node = cell + expansion height, cost = XY distance, going up penalised, down free. It scaled badly with distance (notes: 41 vs 801 expanded nodes straight vs diagonal) from equal-cost path symmetry. Fix: **hierarchical planning over mips with a fixed iteration cap (1024)**: plan at mip 3, refine at mip 1 then 0 (mip 2 skipped) only inside the coarse corridor; return the most detailed finished path when iterations run out. Always finds a path, bounded cost, not optimal.
- **Smoothing**: recursive bisection with height-map raycasts, rerun while flying. **Movement**: particle physics, hover vs glide. No fliers' collision avoidance (never needed); area limits = cylinders. **Landing**: navmesh candidates filtered by field height; guided crash on death.
- No query ms figures in the slides.

Spec (`nav.md` "Volumes", ~375-410) gaps:
1. Wrong attribution: "HZD's min_safe_height = floor + class clearance added at query: one field, all classes". HZD bakes a dilated field per agent size. Our query-time 3 x 3 dilation of one field is our own design (valid; ceil + bed go beyond HZD): say so.
2. "rejected octrees and layered navmeshes on memory and query cost" overstates the slides (see above).
3. Cite Game AI North 2017, not GDC.
4. **Missing: iteration cap + coarse-to-fine corridor.** Ours is one 8-connected A* at one mip, replanned past `fine_m`. HZD found plain grid A* scales badly (symmetry), 8-connected is worse. Add a hard node cap returning the best finished level (fits the sliced service) and corridor refinement (HZD 3 -> 1 -> 0).
5. Climb asymmetry is in ours (`climb_cost`), but HZD carries expansion height in the node; ours profiles after the search. Note the difference.
6. Ours bakes `FieldTile`s offline and streams on demand, not runtime generation: cheaper at play time, keep.
7. Missing: landing candidates (navmesh points whose field height is within the radius) and guided crash; ours has levitation landing only.
8. Memory sanity: 64 x 64 x 3 u16 = 24 KB + 33 % = 32 KB per tile, same order as HZD (190 KB for ~9 tiles).

## 3. Tile rebuild cost
- Our tile: humanoid cs 0.15 m, 128 voxels = **19.2 m** (+ border) (nav.md "Tiles"). UE guideline: 32-128 cells per side; ours is at the top.
- Published numbers: **none found.** Epic's guide gives no ms (only "Draw Tile Build Times" and the 32-128 cell range). Mononen's blog and Unity docs were not fetched. Forum anecdotes only (bigger tiles cost more; `addTile` occasionally slow).
- Our measurement: 215.8 s / 57,516 humanoid tiles = **3.75 ms/tile** cold-bake mean (spec "Runtime rebuild"). Caveats: if that bake used `parallel`, 3.75 ms is wall per tile and single-thread cost is higher (about x cores); the mean includes near-empty terrain tiles, so a dense town tile is a multiple. Working range until measured on a dense tile: 4-20 ms single-thread (guess). Frame cost is zero either way when off-frame.
- Practice: UE builds tiles in async tasks (`MaxSimultaneousTileGenerationJobsCount`, bounded by the generator's worker threads) and applies results on the game thread; Unity's `NavMeshBuilder.UpdateNavMeshDataAsync` / `NavMeshSurface` async bake has the same shape (memory, unverified). Our design (worker build, swap at tick boundary capped per tick, coalesced per tile) matches. The costly main-thread part in UE is `addTile`; `replace_tile_budgeted` + `max_swaps` bounds ours.

## Sources
- https://crates.io/api/v1/crates/navmesh-rs
- https://github.com/Tavrin/navmesh-rs (CHANGELOG.md, src/detour/mod.rs, src/detour/query.rs, main)
- https://www.guerrilla-games.com/read/putting-the-ai-back-into-air (abstract; slides: https://www.guerrilla-games.com/media/News/Files/Putting%20the%20AI%20back%20into%20Air.pptx)
- https://dev.epicgames.com/documentation/en-us/unreal-engine/optimizing-navigation-mesh-generation-speed-in-unreal-engine
- https://forums.unrealengine.com/t/optimization-for-detour-addtile/2653590 (anecdote)
- Local: `superfargoth/vendor/README.md`, `superfargoth/docs/specs/nav.md` (Runtime rebuild, Volumes)
