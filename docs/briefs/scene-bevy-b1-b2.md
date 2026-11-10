# Brief: scene runtime B1 and B2 (`sf_scene_bevy` the crate, and the `CellScene` seam in `sf_world_bevy`)

Written 2026-11-10 by a research agent on `static-scene/brief-b1-b2`. Every `file:line` below was read at
**`main` = `e3112be`** (worktree `../worktrees/brief-b1-b2`, clean). No cargo command was run: this machine's
single build slot belongs to the coordinator.

This brief exists because `docs/briefs/static-scene-s3b.md` was written from the plan and audited *unbuildable*.
So: the plan's §5/§6/§10 line references are **mostly stale**, two of its claims are **structurally wrong on the
as-built tree** (§3 below — read that first), and the three §11 tests land in a harness the plan does not name.

Companion brief, same standard: `docs/briefs/scene-core-a5-a6.md` (the as-built `sf_scene` API; §2 of it is the
API B3 will build on and B1 must not contradict).

Run every cargo command through `bash bevyrrowind/tools/cargo_turn.sh`. It **always exits 0** — judge by output
(`^error`, `could not compile`, `test result:`, `warning:`, `Finished`), never by `$?`. Never `--workspace`.

---

## 1. Where it stands

| Step | State at `e3112be` | Evidence |
|---|---|---|
| A1 `product.rs` out of `feature = "bake"` | **done** | `sf_scene/src/product.rs:22-57`, ungated `pub mod product;` at `lib.rs:44`, re-exported `lib.rs:61-64` |
| A2 format v3 | **done** | `StaticCellTable::write` `table.rs:318`, `StaticCellTableView::parse` `table.rs:433`, one `cfg(feature="bake")` in the file (`table.rs:398`) |
| A3 `ChainRegistry` | **done** | `sf_scene/src/chains.rs`, exported `lib.rs:55` |
| A4 `ClassAllocator` / `size_class` | **done** | `sf_scene/src/alloc.rs`, exported `lib.rs:49-52` |
| A5 `SceneSpace`/`CellPlan`/`batch_ranges` | **done** (`daf9e38`) | `sf_scene/src/space.rs`, exported `lib.rs:66-69` |
| A6 `scene.chains` bake step | **done** (`bb0baa7`) | merge `34b7452` |
| patch GS (`GpuBatchedMesh3d` + N ranges) | **done** (`8047c44`) | `sf_world_bevy/src/gpu_batch_tests.rs` |
| **B1** | **this brief** | create `superfargoth/crates/sf_scene_bevy`: manifest, `SceneTableAsset` + loader, `baked_extensions()`, `SceneConfig`, an empty `ScenePlugin` nobody adds |
| **B2** | **this brief** | `sf_world_bevy/src/scene.rs`: `CellScene` + `SceneCells`, wired at **seven** call sites in `stream.rs` (not six) |
| B3-B6, C1-C2 | not started | §10 of the plan |

`sf_scene_bevy` does **not** exist (`ls superfargoth/crates/` — 72 crates, no `sf_scene_bevy`). `SceneConfig`
does not exist anywhere in the tree (the only hit is a doc comment, `sf_scene/src/alloc.rs:218`).

---

## 2. The corrections — `scene-runtime-plan.md` §5/§6/§10 against `e3112be`

`stream.rs` = `superfargoth/crates/sf_world_bevy/src/stream.rs` (1467 lines).

### 2.1 The §5.1 call-site table

| Plan says | Real | Verdict |
|---|---|---|
| `CellProxies` is `far.rs:33-57` | `far.rs:33-57` | **CONFIRMED** (exactly) |
| `CellLoaded`/`CellUnloaded` `stream.rs:198-212` | `198-212` | **CONFIRMED** |
| `start_fetch` `:695-726` ⇒ `acquire`, `work.scene = true` | `start_fetch` is **695-727** (`Work` inserted at **727**) | CONFIRMED as a location — but see §3.1: this is *not* where `CellProxies::acquire` runs, so "exactly as `w.proxy` does" is false |
| `poll_fetches` `:779-792` ⇒ union `covered` into `work.members` | `poll_fetches` is **729-786**; `work.members` is assigned at **774-777** | **WRONG** (`:779-792` is `scheduler.fetched` + the counter and the start of `upload_estimate`) |
| the model gather at `:760-770` | **759-768** (`let mut models = BTreeSet::new();` at 759, `models.insert` at 766) | **WRONG** by one |
| readiness `:496-510` | the `ready` closure is **523-526**; the proxy arm is **525** | **WRONG** (`:496-510` is the `missing_worldspaces` / `cancelled` / `start` loops) |
| `reveal` (`Stage::Upload`) `:1262-1275` | **1265-1275** | **WRONG** by three — and see §3.2: a resumable `upload` is not expressible there as written |
| the Upload stage's cost is `request.upload_bytes` (`:816`) | **814-815** | CONFIRMED as fact, **WRONG** as a budget argument (§3.2) |
| `despawn_item` `:1346-1352` ⇒ `release` | the far release is **1348-1353**, guarded `request.from == Level::Far` | **WRONG** by two; the guard is right |
| `upload_estimate` `:787-798` | **787-800** | CONFIRMED |
| `work.members` skips the spawn at `:1232-1236` | **1227-1229** | **WRONG** by five |
| `activate_item` `:1276-1307` runs for every placement | **1278-1308**, and it has no `members` filter at all | CONFIRMED (the *conclusion* is right; see §2.4 for the real reason) |
| the precombine's `valid_members` fallback `:783-789` | `fn valid_members` is **1409-1423**; its call site + the fallback counter is **752-757** | **WRONG** |
| "streaming despawns immediately and has no frames-in-flight deferral" `:1311-1355` | `despawn_item` is **1311-1354**; the claim is true (one entity per stage item, then `release_models`, then the hook) | **CONFIRMED** |
| the precombine "draws the same placements" `:1176-1207` | `spawn_precombine` is **1125-1176**; the gate that even builds a precombine handle is **713-723**. `:1176-1207` is `spawn_item`'s head and the *far-proxy* spawn block | **WRONG** |
| R2 spawns far-eligible placements as coarse entities `:1398-1410` | `Ends::of` **1392-1403** | **WRONG** by six |
| the hook's methods run inside `drive_streaming` (exclusive) `:466-477` | **466-477** | **CONFIRMED** |
| the shift guard `:596-616`, `SHIFT_COOLDOWN_FRAMES` `:57` | `SHIFT_COOLDOWN_FRAMES` **57** CONFIRMED. The guard that holds the stages is **546-566**; the detection that sets `shift_soon` is **604-619** | **WRONG** |
| `RenderAssetBytesPerFrame` disabled, `lib.rs:136-141` | `sf_world_bevy/src/lib.rs:138-142` (the `use` is `:57`) | WRONG by two; the fact CONFIRMED |
| `matclass::track_static_materials` in `Last` behind the counters, `lib.rs:131-133` | `lib.rs:125` | **WRONG** |
| `build_template` materials `:1437-1457` | `fn build_template` **1437-1467** (end of file) | CONFIRMED |

### 2.2 The §6 references

| Plan says | Real | Verdict |
|---|---|---|
| template `sf_lod_bevy/src/lib.rs:1-16` | `1-16` | **CONFIRMED** |
| loader template `sf_lod_bevy/src/assets.rs:41-66` | `impl AssetLoader for FarNodeLoader` is **40-66** — but that one builds labeled sub-assets. The one-field template is **`FarDirectoryLoader`, `assets.rs:71-89`** | **WRONG template** (use `FarDirectoryLoader`) |
| `baked_extensions()` copies `sf_lod_bevy/src/lib.rs:129-131` | **128-131** | CONFIRMED |
| joins the map at `mw_exterior_bevy/src/exterior.rs:205-207` | **204-206** | WRONG by one |
| `get_recursive_dependency_load_state` as `sf_lod_bevy/src/runtime.rs:226-233` | the `match` is **226-232**, inside `FarHook::ready` **218-233** | **CONFIRMED** |
| `BakedReader` returns `NotFound` `sf_model_bevy/src/source.rs:65-79` | `read_bytes` **64-78**; the manifest miss is **74** | CONFIRMED (one line out) |
| `cell_offset` at `sf_scene/src/lib.rs:52-55` / `50-55` | **`sf_scene/src/lib.rs:81`** | **WRONG** |
| the gated items at `sf_scene/src/lib.rs:26-27` | **ABSENT** — A1 moved them; `product.rs:22-29` (consts) and `:32-57` (subjects), ungated | correctly predicted by §12, now done |
| `sf_lod_bevy/src/lib.rs:138-141` (R1 past the handoff) | `far_streaming_config` **136-146**, `rings.near` at **140** | CONFIRMED |
| "no new external dependency" | **CONFIRMED** — see §4.2 | |
| `SceneConfig`, `SceneSpaces`, `SceneFrame`, `SceneUploads`, `SceneTableAsset` | **all ABSENT**; only `SpaceConfig` (`sf_scene/src/space.rs:279-293`) exists | B1 defines `SceneConfig` + the asset; the rest is B3/B4 |

### 2.3 §10's "B1 … root `Cargo.toml`, `Cargo.lock`, `workspace-hack`"

Root `Cargo.toml:5` is `members = ["superfargoth/crates/*", "bevyrrowind/crates/*", "bevyrrowind/apps/*"]` — a
**glob**. Creating the directory makes it a member; there is **no member list to edit**. The only root edit B1
needs is one line in `[workspace.dependencies]` (`sf_scene_bevy = { path = "superfargoth/crates/sf_scene_bevy",
version = "0.1.0" }`, alphabetically between `sf_save_bevy` and `sf_script`, near `Cargo.toml:207`) and it is
needed only so C2 can write `sf_scene_bevy.workspace = true`. That line is in a **shared file**: per the owner's
rule the implementer must add *only* that line and commit it with an explicit path, or hand it to the
coordinator.

### 2.4 One claim that is right for the wrong reason

§5.1: "`work.members` already means … still activate (`activate_item` runs for every placement), so colliders,
scripts and AI are untouched." The conclusion holds, but not because `activate_item` ignores `members`: the
**Activate stage is a different request** (`Level::Active`, `stream.rs:527`: `let spawning = r.kind == Load &&
r.to != Level::Active`), and when it runs un-activation it builds a **fresh `Work`** from the cell's retained
placements with `members` empty (`stream.rs:1281-1284`). A `covered` union into the *near* request's `members`
therefore cannot reach activation at all. Good news for B2 — and it means no test can prove the property by
inspecting `members`; it must assert on spawner `activate` calls (§6.2).

---

## 3. Two things §5 asks for that the as-built tree does not express

Neither is fatal. Both change what B2 must write, so neither may be discovered by the implementer mid-slice.

### 3.1 `ready` cannot gate `covered` — the order in §5.1/§5.3 is inverted

§5.1 documents the order as `acquire -> ready until true -> covered -> upload -> release`, and §5.3 leans on it:
"`covered` is consulted after `ready`, so the decision is always made on a resolved table."

On the tree, `covered` must feed the **model gather** (`stream.rs:759-768`), which runs inside `poll_fetches`
(**729-786**) — the completion of the **Fetch** stage. `ready` is polled in the loop at **521-540**, over
requests whose `stage == Stage::Assets`, i.e. strictly *after* `poll_fetches` has already reported
`scheduler.fetched(id, now, assets, upload)` (**779**) with the model count. `covered` therefore runs **before**
`ready`, always. The plan's order is unachievable, and §5.3's guarantee does not hold as written.

**The fix is already in the file, and it is the industry-standard one** (asset-gated stage completion, exactly
how Bethesda/UE gate a streaming cell's publish on its dependent loads): the precombine has the same problem
and solves it by making the *fetch itself* wait. `poll_fetches`' `settled` closure (**733-737**) holds a near
load's fetch until its `Handle<CellPrecombine>` leaves `NotLoaded`/`Loading`; the handle is started in
`start_fetch` (**713-723**) and parked in `Work::near` (**268-270**).

So B2's shape is **two gates, not one**:

- `Work::scene_table: Option<UntypedHandle>` (or a typed handle; see §5.3), started by `CellScene::acquire` in
  `start_fetch` and returned to streaming so it can be parked beside `near`. Extend `settled` (**733-737**) to
  also require the table handle out of `NotLoaded`/`Loading`. Now `covered` *is* consulted on a resolved table
  and §5.3's table of failure behaviours is true.
- `CellScene::ready` keeps its place at **523-526** (`&& (!w.scene || scene.ready(..))`) and covers what only
  the resolved table can name: the **chain `CookedModelAsset`s**, via
  `AssetServer::get_recursive_dependency_load_state` as `sf_lod_bevy/src/runtime.rs:226-232` does.

That is one honest change to the trait: `acquire` returns the handle (or `SceneCells` grows a
`table_handle(&World, &WorldspaceId, IVec3) -> Option<UntypedHandle>`). **Recommended**: give `acquire` the
return type — `fn acquire(&self, world: &World, ws: &WorldspaceId, cell: IVec3) -> Option<UntypedHandle>` —
because streaming already owns a slot for exactly this (`Work::near`) and a second lookup through the hook per
`settled` call (which runs for every in-flight fetch every frame) is wasted work. `CellProxies::acquire`
returns `()` only because a far proxy has nothing the *fetch* must wait for.

### 3.2 A resumable `upload` is not expressible in `reveal`, and the Upload budget would be charged in full every frame

§5.1 puts `scene.upload(world, &cx)` in `reveal` and §5.2 says "`upload` returns `done == false` when the byte
budget cut it short, so the scheduler re-invokes it next frame."

`reveal` (**1265-1275**) is a single, non-resumable stage item:

```rust
fn reveal(&mut self, world: &mut World, request: &Request) -> bool {
    let cell_entity = self.cell_entity(world, &request.worldspace, request.cell);
    let Some(mut work) = self.work.remove(&request.id) else { return true };   // :1267
    ...
    cell.groups.insert(request.to, Group { ... });                              // :1273
    true                                                                        // :1274
}
```

It **removes the `Work`** at 1267 and returns `true` unconditionally. Returning `false` from it would leave the
group already revealed (1269-1271) and the next invocation would hit the `else { return true }` and silently
drop the rest of the upload. `run_stage` (**802-820**) *does* support a multi-frame item (`StageStep { cost,
done }`), so the mechanism exists; the state does not.

Second, independent problem: the item's cost is `let cost = if stage == Stage::Upload { request.upload_bytes as
f64 }` (**814-815**) — the whole estimate, charged on **every** invocation. A three-frame upload would spend
3× the cell's byte budget against `Stage::Upload`'s per-frame allowance, starving every other cell's upload.

**What B2 must do** (and what the implementer must not improvise): restructure `reveal` so the `Work` survives
an unfinished upload, and make the cost truthful:

```rust
fn reveal(&mut self, world: &mut World, request: &Request) -> bool {
    let cell_entity = self.cell_entity(world, &request.worldspace, request.cell);
    if !self.work.contains_key(&request.id) { return true; }            // already revealed
    if let Some(scene) = self.scene.0.clone()
        && self.work.get(&request.id).is_some_and(|w| w.scene)
    {
        let cx = SceneCellCx { .. };
        let (bytes, done) = scene.upload(world, &cx);
        self.stats.scene_upload_bytes += bytes;
        if !done { return false; }          // Work still in the map, group not yet revealed
    }
    let Some(mut work) = self.work.remove(&request.id) else { return true };
    ... // unchanged from 1268-1274
}
```

Note the upload now runs **before** the reveal (which §5.1 also asks for: "before revealing the group"), so the
`!done` early return leaves the group `Visibility::Hidden` — reveal-at-once is preserved, which the current
shape could not guarantee.

For the cost, the honest form is to charge what was actually queued. `StageStep.cost` is computed in
`run_stage`'s closure (**806-817**) from `request.upload_bytes`; B2 should carry the per-invocation bytes out of
`reveal` instead. **Recommended default**: have `reveal` stash the bytes it queued this invocation in
`Runtime` (one `u64` field, written by `reveal`, read and zeroed by the closure) and use it as the Upload cost
when non-zero, falling back to `request.upload_bytes` for cells with no scene. That is ~6 lines, is not a
kludge (it makes an existing estimate exact), and keeps the far/precombine paths byte-identical. If the
implementer judges it out of scope, the fallback is **`upload` must always return `done == true`** in M1 and
`SceneConfig` must have no per-frame upload budget — say so in the commit message and open a ROADMAP line,
rather than shipping a budget that triple-charges.

---

## 4. B1, step by step

### 4.1 Files

```
superfargoth/crates/sf_scene_bevy/Cargo.toml
superfargoth/crates/sf_scene_bevy/src/lib.rs        # ScenePlugin, SceneConfig, baked_extensions()
superfargoth/crates/sf_scene_bevy/src/assets.rs     # SceneTableAsset + SceneTableLoader
superfargoth/crates/sf_scene_bevy/src/tests.rs      # one real test (§6.1)
```
Plus one line in root `Cargo.toml` `[workspace.dependencies]` (§2.3) and the `workspace-hack.workspace = true`
line in the new manifest (which is what `cargo hakari manage-deps` would add; write it by hand so the
dry-run is clean).

**Template: `sf_lod_bevy`, not `sf_model_bevy`.** `sf_model_bevy` is glue (loaders + an asset source) with no
`Plugin` over a streaming hook and no `sf_world_bevy` dependency; `sf_lod_bevy` is the same *role* —
a Bevy plugin over per-cell baked products that implements a `sf_world_bevy` streaming hook, with its own
`baked_extensions()` and its asset loaders in `src/assets.rs` (`sf_lod_bevy/src/lib.rs:1-16`, `:128-131`,
`assets.rs:71-89`, `runtime.rs:205-233`). Copy its manifest's comment discipline (one comment per Bevy crate
saying which type it is for — `sf_lod_bevy/Cargo.toml:8-47`), not its dependency list.

### 4.2 The exact manifest

B1's code is: an asset + loader, an empty plugin, a config struct, `baked_extensions()`. **Nothing else.** The
dep set follows from that, and is a strict subset of §6's list — §6's list is sized for B3/B4:

```toml
[package]
name = "sf_scene_bevy"
description = "Bevy plugin for the superfargoth static scene: baked cell tables as assets, retained GPU instance batches, chain geometry residency"
version.workspace = true
edition.workspace = true
license.workspace = true

# Minimal Bevy (ARCHITECTURE §8b rule 6). The scene logic (table format, chain
# registry, slot allocators, batch ranges, CPU cull) is Bevy-free in sf_scene.
[dependencies]
# App, Plugin, schedules.
bevy_app = { workspace = true, features = ["std"] }
# Asset, AssetApp, AssetLoader: the `.sfscene` cell table.
bevy_asset.workspace = true
bevy_ecs = { workspace = true, features = ["std"] }
bevy_math = { workspace = true, features = ["std"] }
# TypePath for the asset and its loader.
bevy_reflect = { workspace = true, features = ["std"] }
log.workspace = true
sf_scene.workspace = true
sf_world.workspace = true
thiserror.workspace = true
workspace-hack.workspace = true

[dev-dependencies]
bevy = { workspace = true, features = ["3d", "png", "tonemapping_luts"] }

[lints]
workspace = true
```

Deferred to B3/B4 **with the step that needs them**, so B1's own `cargo check` has no unused weight:
`bevy_camera`, `bevy_mesh`, `bevy_pbr`, `bevy_platform`, `bevy_render`, `bevy_transform`, `glam`, `sf_coords`,
`sf_diag_bevy`, `sf_model_bevy`, `sf_world_bevy`. Adding them later costs nothing (§4.3: they are all already
unified), and B1 is the slice that must not drag an unexplained dependency through review.

Two notes if the implementer prefers to declare the full §6 set up front (acceptable, no hakari cost):
- `sf_world_bevy` must be `{ path = "../sf_world_bevy", version = "0.1.0", default-features = false }` — its
  `default = ["debug"]` pulls `bevy_gizmos` (`sf_world_bevy/Cargo.toml:8-11`); `sf_lod_bevy/Cargo.toml:45` does
  exactly this.
- `sf_scene` must stay on default features (`default = []`, `sf_scene/Cargo.toml:9`). Plain
  `sf_scene.workspace = true` is correct and **does not** enable `bake`.

### 4.3 The hakari obligation, and the cold-rebuild question — answered

**Answer: the dep set above adds nothing to the unified feature set, so it cannot force a cold Bevy rebuild.**
Evidence:

- Every Bevy crate is declared `default-features = false` at the root (`Cargo.toml:19-53`), so a member's
  `features = [...]` list is exactly what it contributes.
- The hack already unifies, with **defaults on** (no `default-features = false` on these lines) and a superset
  of what B1 asks for (`superfargoth/crates/workspace-hack/Cargo.toml`):
  `bevy_app:23` `["reflect_auto_register"]`, `bevy_asset:24` `["multi_threaded"]`, `bevy_ecs:30`
  `["multi_threaded","reflect_auto_register","serialize"]`, `bevy_math:38`
  `["approx","bevy_reflect","nostd-libm","serialize"]`, `bevy_platform:42`, `bevy_mesh:39`, `bevy_pbr:40`,
  `bevy_render:45`, `bevy_transform:51`, `glam:77`, `log:87`. `std` is a default feature of these crates and the
  hack lines keep defaults, so `features = ["std"]` is already inside the union.
- The union is computed over the whole workspace, so a new member asking for a **subset** leaves it byte-identical.
- `sf_scene_bevy` is a Bevy crate: it **takes** the hack line and must **not** go in
  `.config/hakari.toml` `final-excludes.workspace-members` (`:38-103`; that list is Bevy-free cores —
  `sf_scene` is on it at `:66`, `sf_scene_bevy` must not be). No hakari.toml edit at all.

**Order the implementer must follow (this is the whole of the cold-rebuild protection):**

1. Write the manifest and the sources. Touch no other manifest except the one `[workspace.dependencies]` line.
2. `bash bevyrrowind/tools/cargo_turn.sh hakari generate --diff` — **metadata only, compiles nothing.**
   Required output: no diff for `superfargoth/crates/workspace-hack/Cargo.toml`. A non-empty diff means the dep
   set added a feature: **stop, do not build**, and remove the offending feature or report it.
3. `bash bevyrrowind/tools/cargo_turn.sh hakari manage-deps --dry-run` — expect it to want only
   `workspace-hack.workspace = true` in the new manifest (write it by hand in step 1 and this is clean too).
4. Only then `cargo_turn.sh check -p sf_scene_bevy --all-targets --locked`. `Cargo.lock` should gain exactly
   one `[[package]]` (`sf_scene_bevy`, path, no new external crate). `git diff --stat Cargo.lock` before
   committing: more than that means step 2 lied and the build is already cold.

(The coordinator owns `Cargo.lock`. Report the one-package diff; do not reformat the file.)

### 4.4 `SceneTableAsset` + loader, and `baked_extensions()`

**Reuse check, in order.** Bevy: no generic "bytes asset" loader exists for a custom extension — a `Handle` must
have a type, so `AssetLoader` is the trait and there is no way around implementing it. Crates: none applicable
(the format is ours). Our own: **there is no generic baked-product loader**, and the brief must say so plainly
so nobody hunts for one. `sf_model_bevy::baked_source` (`source.rs:105-112`) is the asset **source** (an
`AssetReader` over the bake `Store`, keyed `step -> extension`, `source.rs:44-49`); the **loader** is per
extension and per product type, one `impl AssetLoader` each: `SfMeshLoader` (`loader.rs:112-147`, `.sfmesh`),
`FarNodeLoader` (`assets.rs:40-66`, `.sfnode`), `FarDirectoryLoader` (`assets.rs:71-89`, `.sffar`),
`ImpostorAtlasLoader`, `NearCombineLoader`. So: **copy `FarDirectoryLoader`, `sf_lod_bevy/src/assets.rs:71-89`**
— a one-field asset, `Settings = ()`, `read_to_end` then one constructor, then `extensions()`.

```rust
// sf_scene_bevy/src/assets.rs
#[derive(Debug, thiserror::Error)]
pub enum SceneLoadError {
    #[error("reading a scene product: {0}")] Io(#[from] std::io::Error),
    #[error(transparent)] Table(#[from] sf_scene::TableError),
}

/// One cell's baked instance table, bytes verbatim: `StaticCellTableView` borrows them, so the
/// asset owns the `Vec` (audit §C).
#[derive(bevy_asset::Asset, bevy_reflect::TypePath, Debug)]
pub struct SceneTableAsset(pub Vec<u8>);

impl SceneTableAsset {
    pub fn view(&self) -> Result<sf_scene::StaticCellTableView<'_>, sf_scene::TableError> {
        sf_scene::StaticCellTableView::parse(&self.0)
    }
}

#[derive(Default, bevy_reflect::TypePath)]
pub struct SceneTableLoader;
// AssetLoader: read_to_end -> validate with StaticCellTableView::parse -> Ok(SceneTableAsset(bytes))
// extensions() -> &[sf_scene::CELLS_EXT]   (= "sfscene")
```

**Recommendation (improves §5.3, costs nothing): validate in the loader.** `StaticCellTableView::parse`
(`table.rs:433`) checks the header, the size and every cross-reference. Calling it in the loader and discarding
the view turns a corrupt table into an ordinary `LoadState::Failed` on Bevy's loader task, off the main thread
— which is exactly what §5.3's second row wants ("table fails to parse ⇒ fall back, `error!` once per cell"),
delivered by machinery that already exists instead of a second error path inside `covered`. It also makes the
§5.3 rows 1 and 2 collapse into one code path in B3: `ready` true, `covered` `None`.

**`extensions()` must return a `&[&str]`**, so `&[sf_scene::CELLS_EXT]` works only because `CELLS_EXT` is a
`&'static str` const (`product.rs:24`) — it is. Do not write the literal.

```rust
/// File extensions of static-scene products for `sf_model_bevy::baked_source`.
/// (`sf_lod_bevy::baked_extensions`, `sf_lod_bevy/src/lib.rs:128-131`.)
pub fn baked_extensions() -> impl Iterator<Item = (String, String)> {
    [(CELLS_STEP, CELLS_EXT), (CHAINS_STEP, CHAINS_EXT)].into_iter().map(|(s, e)| (s.to_owned(), e.to_owned()))
}
```

`sf_lod_bevy` can iterate a `sf_lod::products::EXTENSIONS` table; **`sf_scene::product` has no `EXTENSIONS`
const** (the four consts at `product.rs:22-29` are all there is), so B1 builds the pairs from the consts. Do
not add `EXTENSIONS` to `sf_scene` in B1: that is a second crate in the slice and a second test slot.

Two facts the implementer should not have to re-derive:
- `CHAINS_EXT == "sfmesh"` (`product.rs:29`) and `SfMeshLoader::extensions()` returns `&["sfmesh"]`
  (`loader.rs:145-147`), so chain products need **no loader of their own** — §3.2's reuse is real. The
  `scene.chains` step still must be in the extensions map, because `BakedReader::read_bytes` strips the
  extension using the `step -> ext` map before the manifest lookup (`source.rs:66-74`); without the entry the
  subject would keep its `.sfmesh` suffix and the manifest lookup would miss.
- Two steps mapping to the same extension is fine: the map is keyed by step (`source.rs:48`), and
  `mw_exterior_bevy` already inserts `mesh.cook -> sfmesh` (`exterior.rs:204`).

### 4.5 `ScenePlugin` and `SceneConfig` — the gap, not a design

Shape (§6, lines 404-462) vs tree:

| §6 names | Where it is | B1 |
|---|---|---|
| `SceneSpaces`, `SpaceState`, `ChainAssets`, `SceneFrame`, `SceneUploads`, `SceneSystems`, the six systems, the batch entities | nowhere | **B3/B4**, not B1 |
| `SceneSpace`, `CellPlan`, `BatchId`, `BatchKey`, `batch_ranges`, `SlotWrite`, `SpaceStats` | `sf_scene::space`, exported `sf_scene/src/lib.rs:66-69` | reuse, define nothing |
| `SpaceConfig { space, granularity, frames_in_flight, batch_capacity }` + `Default` (`PerClass`, 3, 4096) | `sf_scene/src/space.rs:279-293` | reuse |
| `SceneConfig` — referenced for `batch_capacity` (§6), `frames_in_flight` (§6, §13 Q5), `worldspaces` (§12), `shift_rewrite_bytes` (§5.4, default 1 MiB), `enabled` (§8) | **ABSENT everywhere** | **B1 defines it** |

B1's `ScenePlugin` is `{ pub config: SceneConfig }` with a `build` that does exactly two things:
`app.init_asset::<SceneTableAsset>()` and `app.register_asset_loader(SceneTableLoader)`, plus
`app.insert_resource(self.config.clone())` if `SceneConfig` is the resource. **Do not design new fields.** The
gap to report to the owner, with the recommended default:

- `SceneConfig` should **hold** a `sf_scene::SpaceConfig` rather than re-declare `granularity` /
  `frames_in_flight` / `batch_capacity`: one source of truth, and A5 already chose the defaults. Recommended:
  `pub struct SceneConfig { pub enabled: bool, pub space: SpaceConfig, pub worldspaces: Option<Vec<WorldspaceId>>, pub shift_rewrite_bytes: u64 }`
  with `enabled: false`, `worldspaces: None` ("those with a `scene.cells` product", §12), `shift_rewrite_bytes:
  1 << 20`. `SpaceConfig::space: SpaceId` is per space, so the plugin overrides it per worldspace in B3.
- `upload_bytes_per_frame`: **do not add it in B1** — see §3.2. If the cost fix lands, B3 adds it; if not,
  there is no budget to configure.

### 4.6 What "leaves the tree compiling" means for a plugin nobody adds

B1 is additive and unreferenced: `ScenePlugin` is in no `add_plugins` call, `baked_extensions()` is in no
extensions map, `SceneTableAsset` has no user. "Compiling" therefore means all four of:

1. `cargo_turn.sh check -p sf_scene_bevy --all-targets --locked` → `Finished`, no `^error`, no `warning:`.
2. `cargo_turn.sh clippy -p sf_scene_bevy --all-targets --no-deps -- -D warnings` → clean. Workspace lints
   already put `clippy::pedantic` at warn, so `-D warnings` denies it; **do not** pass `-W clippy::pedantic`.
3. `cargo_turn.sh test -p sf_scene_bevy --locked` → the §6.1 test passes (not zero tests collected).
4. `cargo_turn.sh check -p sf_world_bevy -p mw_exterior_bevy --all-targets --locked` → unchanged. B1 changes no
   existing crate, so this is the proof that the new workspace member and the lock entry broke nothing.

There is **no** `--workspace` run in this slice. The repo is not `cargo fmt`-clean and fmt is not in the gate:
match the surrounding style (~120 columns) and never run `cargo fmt`.

---

## 5. B2, step by step

### 5.1 The new module

`sf_world_bevy/src/scene.rs` (new, ~110 lines) + `mod scene;` beside `mod far;` (`lib.rs:26`) + a `pub use` line
beside `pub use far::{CellProxies, FarProxies, ProxySpawnCx};` (`lib.rs:39`). `mod far` is private and
re-exported; follow that exactly.

```rust
//! The static-scene hook: what a cell's retained GPU instance table covers.
//! ... (mirror far.rs:1-9's framing: this keeps sf_world_bevy free of sf_scene_bevy's types)

pub struct SceneCellCx<'a> {
    pub worldspace: &'a WorldspaceId,
    pub cell: IVec3,
    pub cell_origin: WorldPos,
    pub level: Level,
}

/// Order for one cell: `acquire` -> (the fetch waits for the handle it returned) -> `covered`
/// -> `ready` until true -> `upload` until done -> `release` (also after a cancelled load).
pub trait CellScene: Send + Sync + 'static {
    /// The cell's near load started its fetch: start loading its table. The returned handle,
    /// if any, holds the fetch open until it is Loaded or Failed (see `Work::near`).
    fn acquire(&self, world: &World, ws: &WorldspaceId, cell: IVec3) -> Option<UntypedHandle>;
    /// Whether the chain geometry the resolved table names is loaded (or failed).
    fn ready(&self, world: &World, ws: &WorldspaceId, cell: IVec3) -> bool;
    /// Placements the table draws: their models are not loaded and they do not spawn.
    fn covered(&self, world: &World, ws: &WorldspaceId, cell: IVec3) -> Option<Arc<HashSet<PlacementId>>>;
    /// Queues this cell's uploads; returns (bytes queued, done) — resumable.
    fn upload(&self, world: &mut World, cx: &SceneCellCx<'_>) -> (u64, bool);
    fn release(&self, world: &mut World, ws: &WorldspaceId, cell: IVec3);
    fn bytes(&self, _ws: &WorldspaceId, _cell: IVec3) -> u64 { 0 }
}

#[derive(Resource, Clone, Default)]
pub struct SceneCells(pub Option<Arc<dyn CellScene>>);
```

`release` **must be documented and implemented as idempotent** — see call site 6 below.

`Arc<HashSet<PlacementId>>` matches `Work::members`'s type exactly (`stream.rs:271-273`:
`members: Arc<HashSet<PlacementId>>`), so the no-precombine case shares the `Arc` with no clone.

### 5.2 `Runtime` and `Work` fields

- `Runtime` (`stream.rs:315-336`): add `scene: SceneCells` beside `far: FarProxies` (`:328`), refreshed in
  `drive` beside `self.rt.far = ...` (**488**): `self.rt.scene = world.get_resource::<SceneCells>().cloned()
  .unwrap_or_default();`
- `Work` (`stream.rs:251-275`): add `scene: bool` (beside `proxy`, `:258`) and `scene_table:
  Option<UntypedHandle>` (beside `near`, `:270`). `Work` already carries
  `#[allow(clippy::struct_excessive_bools)]` (`:250`), so one more bool needs no new allow.
- `StreamStats` (`stream.rs:220-242`): add `scene_covered: u64` and `scene_upload_bytes: u64` beside
  `precombined` (`:232`) / `precombine_fallbacks` (`:235`). These are the only exact handles the tests get
  (§6), so they are not optional.

### 5.3 The seven call sites

| # | `file:line` at `e3112be` | What it must do |
|---|---|---|
| 1 | `stream.rs:695-727`, `start_fetch` (`&World`) — in the `Work { near, .. }` insert at **727** | If the hook is installed **and** `request.kind == Load` **and** `request.to == Level::Near` **and** (`SceneConfig::worldspaces` admits the worldspace — that test lives in the hook, not here): `let h = scene.acquire(world, &request.worldspace, request.cell);` then `Work { near, scene: true, scene_table: h, ..default() }`. Mirror the structure of the `near` match at **713-723**, including its guards' *placement* (not its conditions). |
| 2 | `stream.rs:733-737`, `poll_fetches`' `settled` closure | Also require `work.scene_table` out of `NotLoaded`/`Loading`. The existing closure is the template verbatim — extend the `is_none_or` chain, do not add a second closure. **This is what makes §3.1 true.** |
| 3 | `stream.rs:750-777`, `poll_fetches` | After `members` is computed (**752-757**) and **before** the model gather (**759-768**): take `covered(world, &request.worldspace, request.cell)` for `w.scene` work, and fold it into the set the gather tests. Then at **774-777** replace the `match members` with a three-way: precombine-only ⇒ `Arc::new(m)`; scene-only ⇒ the hook's `Arc` **cloned, not rebuilt**; both ⇒ a union (and `self.rt.stats.precombine_fallbacks` semantics unchanged). `self.rt.stats.scene_covered += covered.len() as u64`. Note the existing `None => work.near = None` arm must keep firing on a precombine fallback independently of `covered`. |
| 4 | `stream.rs:523-526`, the `ready` closure in `drive` | `&& (!w.scene \|\| self.rt.scene.0.as_ref().is_none_or(\|s\| s.ready(world_ref, &r.worldspace, r.cell)))` — one more conjunct, exactly the shape of the `w.proxy` arm at **525**. |
| 5 | `stream.rs:787-800`, `upload_estimate` (`&self`) | Add the hook's bytes to the `Level::Near` arms of the `match (r.to, &self.rt.far.proxies)` at **791-799**. The match is on `(to, proxies)`; the scene is orthogonal to the far field, so add `+ scene_bytes` to both `Level::Near` arms rather than widening the match to a 2×2. |
| 6 | `stream.rs:1311-1354`, `despawn_item` — beside the far release at **1348-1353** | `if request.from == Level::Near && let Some(scene) = self.scene.0.clone() { scene.release(world, &request.worldspace, request.cell); }`. **Hazard the plan does not mention:** the `request.to == Level::Unloaded` branch (**1327-1332**) takes `std::mem::take(&mut cell.groups)` and tears down *every remaining level's* group without a per-level despawn request. A cell going Near → Unloaded in one step arrives with `from == Level::Near` (fine), but a cell at Near whose unload is reported with `from == Level::Far` would never see the Near release. So: call `release` from **both** the `from == Near` guard and the `to == Unloaded` branch, and **specify `release` idempotent** (a no-op when the cell has no scene residency). Cheaper and safer than proving the scheduler can never do it. |
| 7 | `stream.rs:1049-1072`, `Runtime::cancel` (`&mut World`) — beside the far release at **1059-1063** | `if work.scene && let Some(scene) = self.scene.0.clone() { scene.release(world, &request.worldspace, request.cell); }`. **The §5.1 table omits this site entirely** — yet §11 names `release_runs_on_the_near_unload_and_after_a_cancelled_load`. Without it a cancelled near load leaks the cell's chain refs and its scene-buffer range forever. |

Plus `reveal` (`stream.rs:1265-1275`) restructured per §3.2, which is site 8 in practice; it is listed separately
because it is a change to the function's control flow, not a line inserted into it.

**Origin shift (§5.4).** No hook method and no `stream.rs` call site. The plan says "the plugin reacts to the
shift the streaming guard already detects (`stream.rs:596-616`)" — `Streaming::shift_soon` (**348**) and
`guarded` (**546**) are **private with no accessor** (`Streaming`'s public surface is `stream.rs:362-464`:
`scheduler`, `scheduler_mut`, `config`, `begin_burst`, `in_burst`, `is_idle`, `stats`, `now`, `has_directory`,
`reload_resident`, `cell_entity`). **B2 needs no new API**: `sf_scene_bevy` can watch the public
`WorldOrigin` resource's `cell` field (`sf_world_bevy/src/origin.rs:35-37`, exported `lib.rs:40`), which changes
exactly on a shift. §14 defers the M1 shift rewrite past S3b anyway. Do **not** add a `shift_guarded()`
accessor speculatively.

### 5.4 The precombine interlock

**Expressible exactly where §8 says.** `StreamingConfig::precombine` is a plain `pub bool`
(`stream.rs:95`, default `true` at `:116`) consulted once, in `start_fetch`'s guard at **715**
(`if self.rt.config.precombine && self.rt.far.is_installed() && request.kind == Load && request.to == Near &&
...`). So `streaming.precombine &= !scene` at `mw_exterior_bevy/src/exterior.rs` **before `add_plugins`** (as
`--off matclass` is resolved) makes `Work::near` permanently `None` and `spawn_precombine`
(**1125-1176**) unreachable. Nothing in `sf_world_bevy` needs to know about the interlock, and `--off
matclass` keeps working. That is C2's line, not B2's, but B2's `covered` union (site 3) is only safe *because*
of it: `spawn_precombine` reads `work.members` to build `by_id` (**1133-1136**) and would otherwise take bucket
LOD ends from placements that are not its own. **B2 must say this in the module docs of `scene.rs`** so C2
cannot land the plugin without the interlock.

Correction for the spec file: the plan's citation for "the precombine draws the same placements" is
`stream.rs:1176-1207`; the right one is `spawn_precombine` **1125-1176** and the gate **713-723**.

---

## 6. Tests

Our rule: logic and data get exact assertions; look and feel gets sanity only. All of B1/B2 is logic, so
**every test below asserts something exact.** The repo carries ~100 install-gated tests that pass having
asserted nothing; none of these may join them, and each one below names the value it reads.

### 6.1 B1 — `sf_scene_bevy/src/tests.rs`, one test

`the_cells_loader_parses_a_baked_table` — **exact.** Build a synthetic v3 table with
`sf_scene::StaticCellTable::write()` (`table.rs:318`, ungated, no `bake` feature, no `sf_bake`), register it in a
`Manifest` under `(CELLS_STEP, cell_subject("ws", cell))`, build the app with `register_asset_source(BAKED_SOURCE,
baked_source(store, BakedManifest::new(manifest), BTreeMap::from_iter(sf_scene_bevy::baked_extensions())))` then
`MinimalPlugins + AssetPlugin::default()` + `ScenePlugin::default()`, `load(baked_path(CELLS_STEP, &subject,
Some(CELLS_EXT)))`, pump until `Loaded`, and assert `asset.0.len() == bytes.len()` **and**
`asset.view().unwrap().instances().len() == n` (`table.rs:663`, an `ExactSizeIterator`) — i.e. the whole path (extensions map → `BakedReader` →
extension-keyed loader → owned `Vec` → borrowing view) round-trips. Harness template:
`sf_world_bevy/src/tests.rs:185-214` (`streaming_app`, which already does `baked_source` + a `Manifest` with a
`store.put`) and `sf_model_bevy`'s own loader tests. A second test asserting a **corrupt** table yields
`LoadState::Failed` is worth 10 lines if §4.4's validate-in-loader recommendation is taken; skip it otherwise
(it would assert nothing).

Note on dev-deps: the test needs `sf_bake` (Store/Manifest), `sf_model_bevy` (`baked_source`, `baked_path`) and
`tempfile`. If the full §6 dep set is not declared (§4.2), these go in `[dev-dependencies]` — all three are
already in the workspace and in the unified set (`sf_bake` is `final-excludes`d, Bevy-free).

### 6.2 B2 — `sf_world_bevy/src/tests.rs`, three tests

**The plan's harness citation is wrong.** `gpu_batch_tests.rs:178-190` is `wgsl_struct` / `wgsl_size` — naga
reflection helpers, nothing to do with apps. The no-render-app `App` builder in that file is `fn app()` at
**`gpu_batch_tests.rs:378-390`**, and it has no streaming at all. The right harness for all three is
**`sf_world_bevy/src/tests.rs:185-229`**: `streaming_app(dir)` (`MinimalPlugins + AssetPlugin + MeshPlugin`,
`WorldStreamingPlugin::default()`, a `MemoryIndex` over 1602 synthetic placements, and — decisively — a
`baked_source` with an extensions map at **204-207** that a `scene.cells` entry drops straight into), with
`settle(app)` (**215-225**) and `count::<F>(app)` (**226-229**). `streams_in_around_an_anchor_and_out_when_it_leaves`
(**231-299**) is the worked example: `WorldspaceRegistry::ensure_in`, a `FloatingOrigin` anchor, `begin_burst()`,
`settle`, then assertions on `Streaming::stats()` and `count::<With<Mesh3d>>`.

A **test-local `CellScene` impl** (a `struct TestScene` holding `Mutex<TestSceneLog>`: acquires, readies,
covered sets, uploads, releases per cell) plus `insert_resource(SceneCells(Some(Arc::new(..))))` is all three
tests' instrument. It must not need `sf_scene_bevy` — `sf_world_bevy` must not depend on it (framework layering:
the hook exists so the dependency goes one way, `far.rs:5-9`).

| §11 name | File | Exact assertion |
|---|---|---|
| `scene_covered_placements_load_no_models_and_spawn_nothing` | `tests.rs`, new | **Exact.** `TestScene::covered` returns the ids of every `plate.mdl` placement in one cell. After `settle`: `stats.models_loaded` counts no slot for that cell's models (use a model name unique to the covered cell so the assertion is unambiguous — the existing `placements()` at **152-183** shares `plate.mdl` across all cells, so the test must build its own placement set or assert on `stats.precombined`/`scene_covered` instead), `stats.scene_covered == covered.len()`, and `count::<With<Mesh3d>>` is lower by exactly the covered count versus a run with `SceneCells::default()`. Also assert the §2.4 property properly: a test `PlacementSpawner` (template `tests.rs:296-311`) counts `activate` calls and they must be **unchanged** — covered placements still activate. |
| `a_cell_without_a_scene_hook_streams_as_before` | `tests.rs`, new | **Exact, and it is the regression gate for all seven call sites.** Run `streaming_app` with no `SceneCells` resource and assert the full `StreamStats` tuple and `count::<With<Mesh3d>>` equal the values `streams_in_around_an_anchor_and_out_when_it_leaves` already asserts. Cheapest correct form: `TestScene` installed but with `covered -> None`, `acquire -> None`, asserted byte-identical to no hook at all — that also proves `None` from `covered` is a true no-op (§5.3 row 1). |
| `release_runs_on_the_near_unload_and_after_a_cancelled_load` | `tests.rs`, new | **Split it.** Near-unload half: **exact** — move the anchor out as `streams_in_around_an_anchor_and_out_when_it_leaves` does (**272-299**), `settle`, assert `log.releases == log.acquires` per cell and that every cell that was ever acquired was released exactly once. Cancelled-load half: **reachable, and deterministically so** — `WorldIndexSource(Arc<dyn IndexReader>)` is public and `IndexReader` is a two-method sync trait (`sf_world/src/reader.rs:22-27`), so a test-local `struct Gate(MemoryIndex, Arc<AtomicBool>)` that spins in `page()` until released holds every fetch in flight (`load_page` runs it on an `IoTaskPool` task, `stream.rs:704-712`); move the anchor past R1 while gated, release the gate, `settle`, and assert `log.releases == log.acquires` **and** `log.releases > 0`. Do **not** try to force a cancellation by frame timing: on Windows that is flaky, and a flaky test is worse than none. |

**Vacuity flags.** Two shapes to refuse: (a) any assertion of the form `assert!(log.releases <= log.acquires)`
— true of an empty log, so it must be paired with `> 0`; (b) a cancelled-load test that silently passes when no
cancellation happened — the `log.releases > 0` conjunct above is what prevents it, and the gated reader is what
makes it reliable. If the gated reader turns out not to produce a cancellation (the scheduler may complete the
step rather than cancel it), the right outcome is to **report that** and ship the near-unload half alone, not to
weaken the assertion.

`sf_scene_bevy`'s own six §11 tests (`a_resident_cell_spawns_its_batches_and_despawns_them_on_unload`, …) are
**B3/B4's**, not this slice's. B1 ships one test; B2 ships three.

---

## 7. Build economy — the exact runs, cheapest first

Every line through `bash bevyrrowind/tools/cargo_turn.sh` from the repo root. It **always exits 0**; judge each
by the output lines named. Never `--workspace`.

| # | Command | Judge by | When |
|---|---|---|---|
| 1 | `hakari generate --diff` | **no diff** on `workspace-hack/Cargo.toml`. Compiles nothing | after writing B1's manifest, **before any build** |
| 2 | `hakari manage-deps --dry-run` | wants only the hack line you already wrote | same moment |
| 3 | `check -p sf_scene_bevy --all-targets --locked` | `Finished`; no `^error`, no `warning:`; `git diff --stat Cargo.lock` = one package | B1's first build |
| 4 | `clippy -p sf_scene_bevy --all-targets --no-deps -- -D warnings` | no `^error`, no `warning:` | after 3 |
| 5 | `test -p sf_scene_bevy --locked` | `test result: ok. 1 passed` (not `0 passed`) | after 4 |
| 6 | `check -p sf_world_bevy --all-targets --locked` | `Finished` | B2's first build |
| 7 | `clippy -p sf_world_bevy --all-targets --no-deps -- -D warnings` | clean | after 6 |
| 8 | `test -p sf_world_bevy --locked` | `test result: ok. 28 passed; 0 failed` — the `e3112be` baseline is **25** (11 in `tests.rs`, 14 in `gpu_batch_tests.rs`) **+ 3 new**; never fewer | after 7 |
| 9 | `check -p sf_lod_bevy -p mw_exterior_bevy --all-targets --locked` | `Finished` | last — `sf_lod_bevy` is `sf_world_bevy`'s other hook user and the only crate that can be broken by a `Work`/`Runtime` change leaking into the public API |

If run 1 shows a diff, **stop and report**; a build at that point is a 20-40 minute cold vendored-Bevy rebuild
and the coordinator owns the slot. Shared-target-dir hazard from the S3a handoff: artifacts of
`superfargoth/vendor/bevy_{render,pbr}` are named relative to the workspace root, so a build from another
worktree can look fresh here. If run 6 or 8 behaves inexplicably,
`touch superfargoth/vendor/bevy_{render,pbr}/src/lib.rs` and rerun before concluding anything.

---

## 8. Open questions, with the recommended default

1. **Does `CellScene::acquire` return the table handle?** (§3.1) — the plan's single `ready` cannot gate
   `covered`. **Default: yes**, `-> Option<UntypedHandle>`, and extend `poll_fetches`' `settled` closure. The
   alternative (a second trait method called from `settled` every frame per in-flight fetch) is slower and no
   simpler. **This changes the trait §5.1 publishes, so it needs a one-line spec correction.**
2. **Who fixes the `Stage::Upload` cost so a resumable upload is not triple-charged?** (§3.2) — **Default: B2**,
   ~6 lines in `run_stage`/`Runtime`, because B2 is the slice that makes a multi-frame Upload item possible. If
   the implementer judges it out of scope: `upload` returns `done == true` always in M1, `SceneConfig` grows no
   upload budget, and it goes in the ROADMAP. **Do not ship the resumable path on the current cost model.**
3. **`release` idempotent?** (§5.3 site 6) — **Default: yes, specified in the trait docs**, and called from both
   the `from == Near` guard and the `to == Unloaded` sweep in `despawn_item`. Proving the scheduler can never
   reach the second path is more work than making the call safe.
4. **Full §6 dependency set in B1's manifest, or only what B1 uses?** — **Default: only what B1 uses** (§4.2).
   Both are hakari-free; the narrow set is what passes review and keeps `-D warnings` honest. B3 adds the rest
   with the code that needs them.
5. **Validate the table in the loader?** (§4.4) — **Default: yes.** It gives §5.3's rows 1 and 2 one code path
   using Bevy's own `LoadState::Failed`, off the main thread, with no new error channel.
6. **Add `sf_scene::product::EXTENSIONS` so `baked_extensions()` iterates a table like `sf_lod`'s?** —
   **Default: no, not in B1.** It is a second crate and a second test slot in the slice; B1 builds the pairs
   from the four consts. Worth doing in whatever `sf_scene` slot comes next.

### For the owner

- **Nothing here reverses an owner decision.** Delirium stays default; no `--no-default-features` anywhere;
  nothing in `superfargoth/` named a Morrowind format; the interlock and `--scene` live in `bevyrrowind/`.
- The two places the "no quick fixes" rule made this *larger* than the plan, said out loud: the second
  readiness gate (§3.1) and the Upload cost (§3.2). Both are the plan's own design meeting the as-built
  scheduler; neither is new scope, and the alternative in each case is a kludge that would be found by audit.
- **Spec corrections requested** (the spec is owned elsewhere; the coordinator applies them): §5.1's call-site
  table (every line, §2.1), its trait order and §5.3's "`covered` is consulted after `ready`" (§3.1), the
  `reveal` entry and its budget claim (§3.2), the missing **seventh** call site `Runtime::cancel`
  (`stream.rs:1049-1072`), §5.4's "`stream.rs:596-616`" and the private-field assumption behind it (§5.3),
  §6's loader template (`FarDirectoryLoader`, `assets.rs:71-89`), §10's "root `Cargo.toml`" (member globbing,
  §2.3), §11's harness (`tests.rs:185-229`, not `gpu_batch_tests.rs:178-190`), and `cell_offset` at
  `sf_scene/src/lib.rs:81`.

---

## 9. Dispatch

B1 and B2 are **one slice** (`e3112be` → one branch), as §10 and §14 say: B1 alone is a crate nobody adds and
cannot be judged by a test that exercises it; B2 alone has no asset to load. One deliverable — "the scene seam
exists and nothing streams differently without it" — plus its four tests. The slice touches two crates and one
shared line (§2.3), and all of it is behind `SceneCells::default()`, so the tree ships identically until C2.

Hand off at ~80-100k tokens with the tree compiling. The next slice is **B3+B4** (`sf_scene_bevy`'s residency
and render halves) and it will need `docs/briefs/scene-core-a5-a6.md` §2 for the `sf_scene` API and the S3a
handoff (`bevyrrowind/docs/test-pass-handoffs/s3a-gpu-batch.md`, "For S3b") for `GpuBatchedMesh3d`'s contract.
