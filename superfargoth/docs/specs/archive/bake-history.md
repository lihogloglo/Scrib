# bake history

Dated or survey blocks moved verbatim from [bake.md](../bake.md) (doc cleanup S12c, 2026-10-26).

## Industry comparison

J9 (industry-audit.md), 2026-10-12, docs research only (`.sessions/j9rest.md`).
The skinned/morph part has its own comparison above ("Skinned and morphed
models"); this covers the pipeline as a whole. "Unverified" marks claims
from memory not re-read this session.

| Technique | Industry | Ours | Verdict |
|---|---|---|---|
| Derived-data cache keyed by all inputs + tool version | UE Derived Data Cache (keys from source hash + format/version GUIDs; local, shared and Zen-server backends); Unity Asset Database v2 (import artifacts keyed by dependency hashes, Unity Accelerator as a shared cache); Bazel/Buck2 CAS | trace-based steps over a content-addressed store, step versions in keys | **match** |
| Who bakes | UE/Unity: developers cook, players download cooked data; Bethesda: mod authors run offline tools (Creation Kit precombines/previs, LOD generation, FaceGen; xLODGen/DynDOLOD for LOD); OpenMW: no bake (runtime loading), except an optional navmesh disk cache (`openmw-navmeshtool`) | the player's machine bakes once per mod set, ≤ a few minutes | **differ, by design**: a mod list changes the world, so a per-load-order bake replaces the Bethesda "every LOD/precombine mod ships its own generated files" conflict problem. Shared/prebuilt caches (UE shared DDC) are unnecessary for one player |
| Mesh LOD generation | QEM simplification: UE's built-in reduction (and Simplygon in many AAA pipelines); Unity 6 automatic Mesh LOD (meshoptimizer-based, unverified); meshoptimizer is the common open choice | meshopt, attribute-aware, absolute error per level | **match** |
| Vertex/index optimization + compression | meshoptimizer vertex cache/overdraw/fetch + `EXT_meshopt_compression` / `KHR_mesh_quantization` (glTF); UE Nanite/static mesh compression | same, in `.sfmesh` | **match** |
| Texture cooking (BCn + mips, per-platform) | universal (UE texture compression settings, Unity import platforms, BC7 for color, BC5 for normals) | terrain arrays and far textures cook BC7/BC5; object textures load as shipped (M7: `CompressedImageSaver` route) | **partial**, already planned (M7); no new job |
| Shader/pipeline precompile | UE PSO precaching + bundled caches; Unity variant collections / PSO tracing | warm-up screen planned, not built | known gap (audit row 22, J7) |
| Packing many small products | UE pak/IoStore containers, Zen store; Unity `Library/Artifacts` with an index; git pack files | one file per object (open question 2) | **differ**; fine until file counts hurt (question 2) |
| Deterministic, parallel builds | Bazel hermeticity; UE/Unity parallel importers (Unity parallel import, unverified scope) | rayon steps, determinism tests, no blocking on in-flight work ("Concurrency") | **match** |

No new gap: the differences are planned (M7 texture cooking), known (J7) or
deliberate (per-player bake).

**Sources:** UE Derived Data Cache docs
(https://dev.epicgames.com/documentation/en-us/unreal-engine/derived-data-cache-in-unreal-engine,
not re-read); Unity Asset Database and Accelerator manual pages (not re-read);
OpenMW `openmw-navmeshtool` docs (not re-read); meshoptimizer README
(cited above). All four from memory, unverified on detail.

