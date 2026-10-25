// `mw_assets::particles::ParticleMaterial`: NIF particles as camera-facing
// quads pulled from a storage buffer (vertex pulling). The CPU simulation
// writes each emitter's particles into its slot of one shared buffer; the
// emitter's `MeshTag` is the slot's first index. The mesh is static: four
// vertices per particle, `uv` the corner, `color.x` the particle's index in
// the slot. Fragment: Bevy's StandardMaterial, fogged by the froxel volume
// like `sf_atmos_bevy::FroxelFogMaterial` (render-order.md rule 1).

import sf_render_order_bevy::translucent_fog::{froxel_fog_lookup, froxel_fog_apply, FOG_BLEND, FOG_PREMULTIPLIED, FOG_ADD, FOG_MULTIPLY};
import bevy_pbr::render::mesh_view_bindings::view;
import bevy_pbr::render::pbr_types::{
    STANDARD_MATERIAL_FLAGS_ALPHA_MODE_RESERVED_BITS, STANDARD_MATERIAL_FLAGS_ALPHA_MODE_PREMULTIPLIED,
    STANDARD_MATERIAL_FLAGS_ALPHA_MODE_ADD, STANDARD_MATERIAL_FLAGS_ALPHA_MODE_MULTIPLY,
};
import bevy_pbr::render::{
    mesh_functions,
    forward_io::{Vertex, VertexOutput, FragmentOutput},
    view_transformations::position_world_to_clip,
    pbr_fragment::pbr_input_from_standard_material,
    pbr_functions::{alpha_discard, apply_pbr_lighting, main_pass_post_lighting_processing},
};

// One particle (`particles.rs` `GpuParticle`): node-space center and radius
// (half the quad width, node units); linear RGBA; in-plane angle (x).
struct Particle {
    center_radius: vec4<f32>,
    color: vec4<f32>,
    angle: vec4<f32>,
}

// (near, far, slices, enabled) of the volume.
@group(constants::MATERIAL_BIND_GROUP) @binding(110) var<uniform> fog_params: vec4<f32>;
@group(constants::MATERIAL_BIND_GROUP) @binding(111) var fog_volume: texture_3d<f32>;
@group(constants::MATERIAL_BIND_GROUP) @binding(112) var fog_sampler: sampler;
@group(constants::MATERIAL_BIND_GROUP) @binding(113) var<storage, read> particles: array<Particle>;

@vertex
fn vertex(vertex: Vertex) -> VertexOutput {
    var out: VertexOutput;
    let world_from_local = mesh_functions::get_world_from_local(vertex.instance_index);
    let p = particles[mesh_functions::get_tag(vertex.instance_index) + u32(vertex.color.x)];

    // NIF node scales are uniform: one factor takes the radius to world.
    let radius = p.center_radius.w * length(world_from_local[0].xyz);
    let center = (world_from_local * vec4<f32>(p.center_radius.xyz, 1.0)).xyz;
    let right = normalize(view.world_from_view[0].xyz);
    let up = normalize(view.world_from_view[1].xyz);
    let c = cos(p.angle.x);
    let s = sin(p.angle.x);
    let r = (right * c + up * s) * radius;
    let u = (up * c - right * s) * radius;
    // uv (0, 0) is the top left corner.
    let world = center + r * (vertex.uv.x * 2.0 - 1.0) + u * (1.0 - vertex.uv.y * 2.0);

    out.world_position = vec4<f32>(world, 1.0);
    out.position = position_world_to_clip(world);
    out.world_normal = normalize(view.world_from_view[2].xyz);
    out.uv = vertex.uv;
    out.color = p.color;
@if(VERTEX_OUTPUT_INSTANCE_INDEX)
    out.instance_index = vertex.instance_index;
@if(VISIBILITY_RANGE_DITHER)
    out.visibility_range_dither = mesh_functions::get_visibility_range_dither_level(vertex.instance_index, world_from_local[3]);
    return out;
}

@fragment
fn fragment(in: VertexOutput, @builtin(front_facing) is_front: bool) -> FragmentOutput {
    var pbr = pbr_input_from_standard_material(in, is_front);
    pbr.material.base_color = alpha_discard(pbr.material.flags, pbr.material.alpha_cutoff, pbr.material.base_color);
    var out: FragmentOutput;
    out.color = apply_pbr_lighting(pbr);

    let uv = (in.position.xy - view.viewport.xy) / view.viewport.zw;
    let fog = froxel_fog_lookup(fog_volume, fog_sampler, fog_params, uv, distance(in.world_position.xyz, view.world_position));
    let bits = pbr.material.flags & STANDARD_MATERIAL_FLAGS_ALPHA_MODE_RESERVED_BITS;
    var mode = FOG_BLEND;
    if bits == STANDARD_MATERIAL_FLAGS_ALPHA_MODE_PREMULTIPLIED {
        mode = FOG_PREMULTIPLIED;
    } else if bits == STANDARD_MATERIAL_FLAGS_ALPHA_MODE_ADD {
        mode = FOG_ADD;
    } else if bits == STANDARD_MATERIAL_FLAGS_ALPHA_MODE_MULTIPLY {
        mode = FOG_MULTIPLY;
    }
    out.color = froxel_fog_apply(out.color, fog, mode);
    out.color = main_pass_post_lighting_processing(pbr, out.color);
    return out;
}
