# Brief: scene-core A5 and A6 (`SceneSpace` / `CellPlan` / `batch_ranges`, and the v3 bake)

Written 2026-11-10 by the agent that finished A3 and A4 on `static-scene/scene-core-v3`. A1-A4 are done, tested and
pushed; the whole of `sf_scene` is Bevy-free, so a full `cargo test -p sf_scene --all-features` is under 10 s. Run
every cargo command through `bash bevyrrowind/tools/cargo_turn.sh` (it **always exits 0** — judge by output:
`^error`, `could not compile`, `test result:`, `warning:`, `Finished`). Never build a Bevy-dependent crate.

Baselines to beat, not regress (branch head `4ff2160`):

    cargo_turn.sh test   -p sf_scene --all-targets           -> 71 passed; 0 failed; 1 ignored
    cargo_turn.sh test   -p sf_scene --features bake --all-targets -> 85 passed; 0 failed; 1 ignored
    cargo_turn.sh clippy -p sf_scene --features bake --all-targets -- -D warnings  -> clean

Workspace lints put `clippy::pedantic` at warn already, so `-D warnings` denies it. Do **not** pass
`-W clippy::pedantic` (it re-enables three lints the owner set to allow). The repo is **not** `cargo fmt`-clean and
fmt is not in the gate: match the surrounding style (~120 columns, single-line struct literals), never run
`cargo fmt`.

---

## 1. Format v3 as built — the contract A6's writer must satisfy

Read out of `superfargoth/crates/sf_scene/src/table.rs` (module docs `:39-82`, `parse` `:433`, `validate` `:487`,
`validate_models` `:573`, writer `StaticCellTable::write` `:318`). All little-endian, no compression, no padding
beyond what is listed. Sections follow the header in this order and the file is **exactly** their total size.
**v3 has no reserved bytes**: v2's three spare header words and six spare chain bytes became the model section,
the layout version and the chain source.

Constants: `MAGIC = b"SFSCENE\0"`, `FORMAT_VERSION = 3`, `MAX_LOD_LEVELS = 16`, `MAX_MODELS = 65536`;
private sizes `HEADER = 80`, `INSTANCE = 80`, `CHAIN = 32`, `LEVEL = 12`, `MODEL_OFFSET = 4`, `INSTANCE_CHAIN = 68`.

### Header, 80 B

| off | size | field |
|---|---|---|
| 0 | 8 | magic `SFSCENE\0` |
| 8 | 4 | version `u32` (3) |
| 12 | 4 | instance count `u32` |
| 16 | 4 | chain count `u32` |
| 20 | 4 | level count `u32` (total across all chains) |
| 24 | 12 | origin cell `i32 × 3` (written as `u32`, two's complement) |
| 36 | 4 | land minimum `f32`, cell-relative Y |
| 40 | 12 | AABB min `f32 × 3` |
| 52 | 12 | AABB max `f32 × 3` |
| 64 | 4 | layer mask `u32` |
| 68 | 4 | model count `u32` |
| 72 | 4 | model bytes `u32` (byte length of the names blob) |
| 76 | 4 | chain layout version `u32` (= `chain::LAYOUT_VERSION`, currently 1) |

80 is a multiple of 16, so the instance section of a 16-aligned buffer is 16-aligned.

### Instance record, 80 B each — this **is** the GPU layout

| off | size | field |
|---|---|---|
| 0 | 48 | transform rows `f32 × 12` (three rows of 4; `StaticInstance::rows`) |
| 48 | 16 | sphere centre `f32 × 3` + radius `f32`, cell-relative, bounds **this part's** LOD0 vertices |
| 64 | 4 | placement id `u32`, cell-local |
| 68 | 4 | chain index `u32`, **cell-local** — the only field `write_remapped` rewrites |
| 72 | 4 | material override `u32`, **always 0** in a baked table |
| 76 | 4 | flags `u32` (`InstanceFlags`) |

WGSL equivalent: `array<vec4<f32>, 3>`, `vec4<f32>`, four `u32`, 16-byte aligned. Stream-in uploads
`write_remapped`'s copy (`table.rs:645`), which differs from `instance_bytes` (`:636`) only at offset 68.

### Chain record, 32 B each

| off | size | field |
|---|---|---|
| 0 | 4 | first level `u32` (index into the level section) |
| 4 | 4 | level count `u32`, `1..=MAX_LOD_LEVELS` |
| 8 | 2 | material class `u16` (`MaterialClass` repr) |
| 10 | 2 | **model index `u16`** into the model section (v3) |
| 12 | 4 | **part `u32`** — the material index inside that model (v3) |
| 16 | 8 | mesh key `u64` |
| 24 | 8 | material key `u64` |

### Level record, 12 B each

`first_index u32`, `index_count u32`, `error f32` (metres, model space, finest level first).

### Model section — exact encoding

Two sections, in this order, after the levels:

- **model offsets**: `model count` × `u32` byte offsets into the names blob, **strictly** ascending, the first
  exactly 0, each equal to where the previous name ended, the last name running to `model bytes`.
- **model names**: the models' `mesh.cook` subjects, UTF-8, **concatenated with no separators and no terminators**,
  in index order. The names are **sorted and unique**, so one table has exactly one encoding.

A name is therefore never empty, and every model must be named by some chain.

### Every validation rule the reader enforces

`parse` (in order): length ≥ 12 and magic → `BadMagic`; version ≠ 3 → `Version`; length < 80 → `Size`; total size
≠ header + 80·instances + 32·chains + 12·levels + 4·models + model bytes → `Size`; layout version ≠
`LAYOUT_VERSION` → `Layout`. Then `validate`:

1. `layers == 0` → `NoLayers` (a whole worldspace baked invisible).
2. AABB not finite, or `min > max` on any axis → `Aabb`.
3. land minimum NaN or `+inf` → `LandMin` (`-inf` is the intended "no land / holes").
4. Model section (`validate_models`): more than `MAX_MODELS` → `TooLarge`; count 0 with a non-empty blob →
   `ModelOffsets`; an offset that is not where the previous name ended, or an empty name, or a name past the blob →
   `ModelOffsets`; invalid UTF-8 → `ModelUtf8(index)`; not sorted or not unique → `ModelOrder(index)`.
5. Chains, in index order: mesh keys **strictly** ascending → else `ChainOrder(mesh)`; model index past the model
   count → `MissingModel`; a repeated `(model, part)` → `DuplicateChainSource { first, second }` (one geometry
   product in two chain slots would draw it twice); `level_count == 0`, `> MAX_LOD_LEVELS`, or
   `first_level + level_count` past the level section → `ChainLevels(index)`; a level error that is not finite or is
   negative → `LevelError(index)`; an unknown `MaterialClass` value → `MaterialClass { chain, class }`.
6. A model no chain names → `UnusedModel(index)`.
7. Instances, in order: placement ids never decrease → else `PlacementOrder`; chain index past the chain count →
   `MissingChain`; `material_override != 0` → `MaterialOverride`; a non-finite transform → `Transform`; a
   non-finite or negative sphere → `Sphere`; unknown, runtime-only or not-yet-implemented flag bits
   (`InstanceFlags::NOT_BAKED`, which includes the runtime-only `INACTIVE`) → `Flags`.
8. A chain no instance uses → `UnusedChain(index)`.

Not enforced by `parse` (it would need the baker): **the mesh key must equal `bake::mesh_key(model, part)`**.
`StaticCellTable::verify_chain_sources` (`table.rs:399`, feature-gated on `bake` through `bake::mesh_key`) checks
it and returns `TableError::MeshKey(index)`. A6's writer must satisfy it; the bake-side test
`mesh_key_matches_the_chain_source` is the one that proves it.

Versions 1 and 2 are **not** read and need no migration: `bake_scene` has never been called, so no table of either
was ever written.

## 2. The public API A5 and A6 build on

### `sf_scene::product` (always compiled, no `sf_bake`)

```rust
pub const CELLS_STEP: &str = "scene.cells";
pub const CELLS_EXT: &str = "sfscene";
pub const CHAINS_STEP: &str = "scene.chains";
pub const CHAINS_EXT: &str = "sfmesh";
pub fn cell_subject(worldspace: &str, cell: IVec3) -> String;          // "<ws>/<x>,<y>,<z>"
pub fn parse_cell_subject(subject: &str) -> Option<(&str, IVec3)>;     // splits at the LAST '/'
pub fn chain_subject(model: &str, part: u32) -> String;                // "<model>/<part>"
pub fn parse_chain_subject(subject: &str) -> Option<(&str, u32)>;      // splits at the LAST '/'
```

`sf_scene::bake` re-exports `CELLS_STEP`/`CHAINS_STEP` and its own `chain_subject` wrapper (`bake.rs:64`, `:109`).

### `sf_scene::chains` (A3, new)

```rust
pub type ChainKey = u64;    // LodChain::mesh
pub type ChainSlot = u32;   // what write_remapped writes into the instances

pub struct ChainSource { pub model: String, pub part: u32 }
impl ChainSource { pub fn subject(&self) -> String; }               // "<model>/<part>"

pub struct ChainEntry {                 // PartialEq, Clone
    pub key: ChainKey,
    pub source: ChainSource,
    pub class: MaterialClass,
    pub material_key: u64,
    pub levels: Vec<LodLevel>,
}
impl ChainEntry {
    pub fn from_table(view: &StaticCellTableView<'_>, index: u32) -> Option<Self>;
}

pub enum ChainError {
    Disagreement { key: ChainKey, field: &'static str },
    NotResident { key: ChainKey },
    Slot(AllocError),                   // #[from]; only AllocError::Overflow is reachable
}

pub enum Acquired { Added(ChainSlot), Shared(ChainSlot) }
impl Acquired { pub fn slot(self) -> ChainSlot; pub fn is_new(self) -> bool; }

pub struct ChainStats { pub live: u32, pub pending: u32, pub high_water: u32 }

pub struct ChainRegistry { /* slots: Vec<Option<Resident>>, by_key: BTreeMap, alloc: SlotAllocator */ }
impl ChainRegistry {
    pub fn new(frames_in_flight: u32) -> Self;
    pub fn acquire(&mut self, entry: ChainEntry) -> Result<Acquired, ChainError>;
    pub fn release(&mut self, key: ChainKey, frame: u64) -> Result<Option<ChainSlot>, ChainError>;
    pub fn advance(&mut self, frame: u64);
    pub fn slot(&self, key: ChainKey) -> Option<ChainSlot>;
    pub fn entry(&self, slot: ChainSlot) -> Option<&ChainEntry>;
    pub fn refs(&self, key: ChainKey) -> Option<u32>;
    pub fn resident(&self) -> impl Iterator<Item = (ChainSlot, &ChainEntry)>;   // slot order
    pub fn stats(&self) -> ChainStats;
}
```

Semantics A5 must not contradict:

- `acquire` returns `Added` on the **first** reference (the Bevy half then starts the asset load) and `Shared`
  afterwards; the slot is the same in both cases.
- Slots are **lowest-free-first**: `SlotAllocator::alloc(1)`'s first fit over a free list sorted by start. The same
  sequence of cells gives the same slots on every machine — the determinism test in `scene-runtime-plan.md` §11
  depends on it.
- `release` returns `Some(slot)` only on the **last** reference; the slot returns to the free list at
  `advance(frame + frames_in_flight)`, not before.
- A chain **re-acquired inside the deferral window takes a new slot**. Deliberate: the Bevy half drops the mesh
  handles when the last cell releases, so resurrecting the old slot would point at geometry already on its way out.
  If a cell-unload/reload churn ever shows up as a cost, the fix is to keep a short grace list of entries whose
  handles are still alive, not to reuse slots inside the window.
- A `Disagreement` leaves the registry **exactly** as it was (asserted field by field). Levels compare **bit for
  bit**, errors included, so two NaN level errors from a stale product are a disagreement, not an accidental equal.
- `refs` is the registry's own, not a field of the caller-supplied `ChainEntry`.

### `sf_scene::alloc` (A4 adds the second half)

```rust
pub struct SlotRange { pub start: u32, pub len: u32 }   impl SlotRange { pub fn end(self) -> u32; }
pub enum AllocError { ZeroLength, Overflow, NotAllocated(SlotRange), Exhausted { want: u32, capacity: u32 } }

// unchanged (the per-space instance table: one contiguous range per cell, first fit, coalesced)
pub struct SlotAllocator;
impl SlotAllocator {
    pub fn new(frames_in_flight: u32) -> Self;
    pub fn alloc(&mut self, len: u32) -> Result<SlotRange, AllocError>;   // alloc.rs:79-108
    pub fn release(&mut self, range: SlotRange, frame: u64) -> Result<(), AllocError>;  // :110
    pub fn advance(&mut self, frame: u64);                                 // :121-134
    pub fn live_range(&self, start: u32) -> Option<SlotRange>;
    pub fn stats(&self) -> SlotStats;   // high_water, live_slots/ranges, free_slots/ranges,
}                                       // largest_free, pending_slots, fragmentation

// new (one per batch, over that batch's fixed output slots)
pub const BLOCK_GRAIN: u32 = 64;
pub const CLASSES: usize = 32;
pub const CLASS_SIZES: [u32; CLASSES];      // 64,128,..,512, 640, 832, .., 129_088
pub fn size_class(len: u32) -> Option<(u8, u32)>;   // (class, block slots); None for 0 or > 129_088

pub struct ClassStats {
    pub capacity: u32, pub bump: u32, pub live_slots: u32, pub peak_live_slots: u32,
    pub live_blocks: u32, pub free_blocks: u32, pub pending_blocks: u32, pub waste: u32,
}
pub struct ClassAllocator;
impl ClassAllocator {
    pub fn new(capacity: u32, frames_in_flight: u32) -> Self;
    pub fn capacity(&self) -> u32;
    pub fn alloc(&mut self, len: u32, tag: u64) -> Result<SlotRange, AllocError>;   // never splits
    pub fn release(&mut self, range: SlotRange, frame: u64) -> Result<(), AllocError>;
    pub fn advance(&mut self, frame: u64);
    pub fn blocks(&self) -> impl Iterator<Item = (SlotRange, u64)> + '_;   // address order, with tags
    pub fn block(&self, start: u32) -> Option<(SlotRange, u64)>;
    pub fn free_blocks_in(&self, class: u8) -> usize;
    pub fn stats(&self) -> ClassStats;
}
```

`alloc` takes a `u64` **tag** (the plan said `u32`): `sf_scene_bevy` wants `(chain slot, level)` in it, which does
not fit 32 bits once chain slots are a full `u32`. `blocks()` hands it back, so `batch_ranges` reads
`(first_slot, slot_count, chain, level)` straight off the block list. The returned `SlotRange` is the `len` the
caller asked for, **not** the block: the rest of the block is the allocator's and goes back to its class on
`release`. `release` must be given the range `alloc` returned (same `start` **and** `len`), else `NotAllocated`.

## 3. The A4 fragmentation result — read this before sizing a batch

**The plan's §4.4 "Verdict: bounded" is wrong, measured.** `scene-runtime-plan.md:211-222` claims
`bump < 1.25·Σ_c peak_c·len_c + 64·CLASSES` — "≤ 1.25 × the worst resident footprint plus one block of slack per
class" — and that it "*converges* once the worst neighbourhood has been walked; it is not a creep". On a synthetic
5×5 ring (25 resident cells, 3 frames in flight) over 40 cell footprints of 7..=638 instances (nine classes in
play), `bump_is_bounded_under_a_city_walk` and the ignored `class_allocator_high_water_curve` measure:

| cycles | 10 k | 20 k | 50 k | 80 k | 150 k | 330 k | 1.0 M | 2.0 M |
|---|---|---|---|---|---|---|---|---|
| `bump` | 30 848 | 32 128 | 33 472 | 34 560 | 35 072 | 38 720 | 39 104 | 40 704 |
| `bump / peak_live` | 2.79 | 2.85 | 2.97 | 2.95 | 3.00 | 3.28 | 3.24 | **3.33** |

It is **3.0× the peak live slots at 200 000 cycles and still creeping at 2 M** (≈ 9 h of play at 60 fps), not
1.25×, and it does not converge: `peak(c)` is a record process whose per-class records keep being beaten, ever
more rarely. And the contrast the plan expected to lose: the **same** sequence through the first-fit
`SlotAllocator` settles at `high_water / peak_live = 1.17` with six coalesced free ranges
(`first_fit_high_water_on_the_same_walk`). Robson's ≈ 11× is an adversarial bound; a streaming ring with a
content-bounded size set is not an adversary, so first fit is **2.6× tighter** here.

What *does* hold, and is what the tests assert:

- the identity `bump = Σ_c size(c)·peak(c)`, **exactly**, re-checked at twenty stages of the walk, with `peak(c)`
  the most class-`c` blocks ever *occupied* at once (live, or still waiting for the frames in flight). Blocks are
  never split and never merged, so nothing else can make the allocator grow: external fragmentation inside a class
  is zero.
- `bump ≤ (resident cells + frames in flight) · Σ_{reachable c} size(c)` — an **a-priori** ceiling computable from
  the ring configuration before a cell streams in, independent of session length. 82 432 slots (7.0× peak live) for
  the walk above.
- internal waste: `size(c) < 1.25·len + 64`, measured under 1 000 slots across 25 live cells.

**Consequences for A5 and the `sf_scene_bevy` slices.** The case for `ClassAllocator` over a batch's output slots
is the a-priori ceiling, `BLOCK_GRAIN`-aligned block starts for the materialize dispatch, and O(1) allocation —
**not** a smaller high-water mark. So:

1. Size `SceneConfig::batch_capacity` from the occupancy ceiling, never from `1.25 × the live set`. The plan's
   default of 4096 holds only ~1 300 slots of live instances per batch at the measured 3.0×; whether that is right
   is a *measurement* for the owner's machine, and `Exhausted` → entity fallback must therefore be exercised, not
   treated as a corner case. Put a `scene/slots/*` counter and an `#[ignore = "measurement"]` note in
   `bevyrrowind/docs/visual-checks.md` rather than guessing.
2. The **scene buffer** keeps plain `SlotAllocator` (the plan is right about that, for the wrong reason: it is the
   better policy here, not the worse one tolerated for its simplicity).
3. Do **not** "fix" the allocator to meet the plan's bound: the owner's rule is no quick fixes, and the design is
   the owner's. `bump_is_bounded_under_a_city_walk` deliberately asserts that the plan's bound is **violated** —
   if a later change makes that assertion fail *low*, the plan was right after all and §4.4 can stand as written.
4. §4.4's prose needs correcting in `scene-runtime-plan.md` (it lives on `static-scene/gs-slice-b1`, not on this
   branch, so this slice could not edit it). `alloc.rs`'s `ClassAllocator` doc comment already carries the
   corrected version verbatim — copy it.

## 4. Wrong in `scene-runtime-plan.md` about A5 and A6

File references are to `C:/.../repo/superfargoth/docs/specs/scene-runtime-plan.md` (branch
`static-scene/gs-slice-b1`); `sf_scene` line numbers are branch head `4ff2160`.

- `:282` — `plan_in` "acquire a chain slot per `chains()` entry (`table.rs:506`)": `chains()` is now
  **`table.rs:763`**. Prefer `ChainEntry::from_table(view, index)` (A3) over `chains()` + `chain_source()`.
- `:283` — "`write_remapped(&chain_slots, &mut scene_bytes)` (`table.rs:425`)": now **`table.rs:645`**. Note it
  **panics** if `slots.len() != chain_count()` (test `remap_needs_one_slot_per_chain`), so `plan_in` must build
  `chain_slots` for *every* cell-local chain, in index order, before calling it — including chains whose batch
  overflowed.
- `:287` — "the table is sorted by `(placement, part)` (`table.rs:9-13`)": the docs are now at **`table.rs:7-12`**.
- `:289` — "`WorkItem::slot = cell start + instance` (`cull.rs:264-266`)": the field is **`WorkItem::index`**, and
  the "slot = the cell's range start + this" note is at **`cull.rs:304-311`**. There is no `WorkItem::slot`.
- `:173` and `:174` (§4.2, A3's own references, for the record) — first fit is `alloc.rs:79-108`, deferral is
  `alloc.rs:110-134`, not `:76-104` / `:107-130`.
- `:120` (§3.2) — `CookedSubmesh::encode` is `pub(crate)` at **`sf_model/src/cooked.rs:107`**, not `:106`.
- `:195` (§4.4) — `size_class` returns **`Option<(u8, u32)>`**, not `(u8, u32)`: `len == 0` and a `len` above the
  largest class (129 088) have no class.
- `:203`, `:207` (§4.4) — `ClassAllocator::alloc` takes `(len, tag: u64)` and `blocks()` yields `(SlotRange, u64)`, not
  `u32` (see §2 above).
- §4.2's `ChainSource { model: String, material: u32, levels: u32 }` became `{ model: String, part: u32 }`: the
  plan's `material` *is* the part (the material index inside the model, `LodChain::part`), and `levels: u32`
  duplicated `ChainEntry::levels.len()`. `ChainEntry` has no `refs` field (§2).
- §4.2's `ChainError` has two variants; the built one has three — `Slot(AllocError)`, because
  `SlotAllocator::alloc` returns a `Result` and nothing in this crate may panic on it.

## 5. A6: what it still has to build, and what the adapter must supply

Present already (A1/A2): `product.rs`'s step ids and subject grammar, `bake::mesh_key(model, material)`
(`bake.rs:120`), `bake::chain_subject` (`:109`), `part_spheres` (`:180`), the `SceneSource` adapter trait
(`:236`), `CellModel` (`:293`), `SceneBake` (`:517`) and `SceneBake::register` (`:550`, registers `CellsStep`
only), and the v3 writer with the model section (`table.rs:318`). `cargo test -p sf_scene --features bake` is
green, so A2's v3 and `bake.rs` already agree.

Absent, and A6's job: **`ChainsStep`**, **`SceneBake::chain_keys`**, **`SceneBake::bake_chains`**, and
`sf_model::CookedSubmesh::encode` → `pub` (`sf_model/src/cooked.rs:107`, one line, Bevy-free).

**The adapter question the plan does not answer.** `scene-runtime-plan.md:128-130` says `chain_keys` "derives its
subjects from the baked tables' **model sections**", and that is exactly right and already possible: a v3 table's
model names *are* `mesh.cook` subjects (`StaticCellTableView::models()`, `table.rs:736`; `chain_source()`, `:747`),
and `bake.rs` builds the section from `CellModel::Static { path }`. So **A6 needs nothing new from
`mw_bake::scene` or from `MwSceneSource`**: no new trait method, no Morrowind name in `sf_scene`. The model string
already flows baked-table → `chain_keys` → `chain_subject` → `ChainsStep::plan`'s
`cx.product(MeshCookStep::ID, model)`. The plan is not wrong here, only silent — and the silence is the right
answer. Flag it in the A6 commit message so C1 (`mw_bake/src/scene.rs`, `mw_boot_bevy/src/bake.rs`) knows it has
only the two call sites and the two report lines to add, per §3.3 (`:121-135`).

The one thing to confirm while building `ChainsStep`: §3.2 says a chain product is an `sf_model` `.sfmesh` with
**one LOD level per chain level, one submesh each**, which makes `LodLevel::first_index` "describe a concatenation
nothing builds". That is already logged as a spec correction (`:119`, §12). A5's `batch_ranges` and the GPU side
therefore use `{ mesh: meshes[level], first_index: 0, index_count: 0 }`. **`LodLevel::first_index` stays in the
table** (the reader validates against it and `chain_layout` produces it); it is simply not what the runtime
indexes with. Do not "simplify" it away without the owner.

## 6. Open questions, with the recommended default

1. **`ChainRegistry` slot resurrection inside the deferral window** (§2). Default: keep the current behaviour (new
   slot). Revisit only with a measured cost from a real unload/reload churn.
2. **`batch_capacity`** (§3). Default: leave the plan's 4096 and make `Exhausted` → entity fallback a first-class
   tested path; record the real ceiling from an owner-machine walk before changing the number.
3. **`ClassAllocator` for the scene buffer too?** Default: no. First fit measured 2.6× tighter, and the scene
   buffer has no workgroup-alignment requirement.
4. **`ChainStats::high_water` is the Bevy-side side-table length.** A5 should surface it in `SpaceStats` so B3 can
   size `Vec<Option<ChainAssets>>` once instead of growing it per acquire.
