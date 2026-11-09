// superfargoth patch (bevyengine/bevy#25978): new file.
import bevy_render::occlusion_culling::mesh_preprocess_types::{MeshInput, MeshCullingData};
import package::gpu_instance_batch::gpu_instance::GpuMeshInstance;
import package::render::mesh_types::MESH_FLAGS_SIGN_DETERMINANT_MODEL_3X3_BIT;

struct Metadata {
    mesh_input: MeshInput,
    bounds: MeshCullingData,
    input_base: u32,
    capacity: u32,
    // superfargoth patch (own, GS: ranges): 0 = no range table, i.e. the whole
    // batch draws `mesh_input`'s own geometry (upstream #25978).
    range_count: u32,
    pad: u32,
}

// superfargoth patch (own, GS: ranges): mirror of Rust `GpuBatchGeometryRange`,
// field for field and in order. 64 B, 16-aligned in both languages
// (`aabb_center` at 32, `aabb_half_extents` at 48); asserted by
// `sf_world_bevy`'s `gpu_batch_tests`.
struct GeometryRange {
    first_slot: u32,
    first_vertex_index: u32,
    first_index_index: u32,
    index_count: u32,
    metadata_index: u32,
    material_and_lightmap_bind_group_slot: u32,
    flags: u32,
    pad0: u32,
    aabb_center: vec3<f32>,
    pad1: f32,
    aabb_half_extents: vec3<f32>,
    pad2: f32,
}

@group(0) @binding(0) var<uniform> metadata: Metadata;
@group(0) @binding(1) var<storage> instances: array<GpuMeshInstance>;
@group(0) @binding(2) var<storage, read_write> inputs: array<MeshInput>;
@group(0) @binding(3) var<storage, read_write> bounds: array<MeshCullingData>;
// superfargoth patch (own, GS: ranges): never empty, but only the first
// `metadata.range_count` records may be read.
@group(0) @binding(4) var<storage> ranges: array<GeometryRange>;

// superfargoth patch (own, GS: ranges)
// Ranges are sorted by `first_slot` and partition `0..capacity`, so the range
// of a slot is the last one whose `first_slot <= slot`. Mirrors Rust
// `gpu_instance_batch::range_of`, which is the copy the tests check; keep the
// two in lockstep. `count` must be non-zero.
fn range_of(slot: u32, count: u32) -> u32 {
    var low = 0u;
    var high = count;
    while (low + 1u < high) {
        let mid = low + (high - low) / 2u;
        if (ranges[mid].first_slot <= slot) { low = mid; } else { high = mid; }
    }
    return low;
}

@compute @workgroup_size(64)
fn main(@builtin(global_invocation_id) id: vec3<u32>) {
    if id.x >= metadata.capacity { return; }
    let output = metadata.input_base + id.x;
    let instance = instances[id.x];
    var input = metadata.mesh_input;
    var culling = metadata.bounds;
    // superfargoth patch (own, GS: ranges): per-instance geometry. Every
    // instance of one range shares these fields, and a range is a contiguous
    // span of slots, so the "first work item of a draw writes `mesh_index`"
    // rule in `mesh_preprocess.wesl` stays correct without any sorting.
    if (metadata.range_count > 0u) {
        let r = range_of(id.x, metadata.range_count);
        input.first_vertex_index = ranges[r].first_vertex_index;
        input.first_index_index = ranges[r].first_index_index;
        input.index_count = ranges[r].index_count;
        input.metadata_index = ranges[r].metadata_index;
        input.material_and_lightmap_bind_group_slot =
            ranges[r].material_and_lightmap_bind_group_slot;
        input.flags = input.flags | ranges[r].flags;
        culling.aabb_center = ranges[r].aabb_center;
        culling.aabb_half_extents = ranges[r].aabb_half_extents;
    }
    input.world_from_local = instance.world_from_local;
    // The affine transpose stores the linear matrix in the first three lanes.
    let linear = mat3x3<f32>(instance.world_from_local[0].xyz,
        instance.world_from_local[1].xyz, instance.world_from_local[2].xyz);
    input.flags = (input.flags & ~MESH_FLAGS_SIGN_DETERMINANT_MODEL_3X3_BIT)
        | select(0u, MESH_FLAGS_SIGN_DETERMINANT_MODEL_3X3_BIT, determinant(linear) >= 0.0);
    input.tag = instance.tag;
    inputs[output] = input;
    culling.is_active = instance.is_active;
    bounds[output] = culling;
}
