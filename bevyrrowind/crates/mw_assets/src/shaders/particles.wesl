// `mw_assets::particles::ParticleMaterial`: NIF particles as camera-facing
// quads pulled from a storage buffer (vertex pulling). The CPU simulation
// writes each emitter's particles into its slot of one shared buffer; the
// emitter's `MeshTag` is the slot's first index. The mesh is static: four
// vertices per particle, `uv` the corner, `color.x` the particle's index in
// the slot. Fragment: Bevy's StandardMaterial inputs, lit as a volume (below),
// fogged by the froxel volume like `sf_atmos_bevy::FroxelFogMaterial`
// (render-order.md rule 1).
//
// Lighting (RO-B): smoke and steam are thin volumes, not surfaces, so they
// are lit non-directionally, as UE's "Volumetric NonDirectional"
// translucency and Frostbite's unified volumetric lighting do: every light
// counts as if the puff faced it (no N·L, isotropic phase), through Bevy's
// own light functions on a Lambert face turned toward each light (so the
// sun keeps the atmosphere's transmittance and visible-disc ratio, point and
// spot lights their distance and cone falloff), plus the indirect light
// averaged over up and down. The sum carries Morrowind's light saturation
// (`saturated_light`): the vertex colour drives ambient and diffuse and the
// original clamps `colour × (ambient + Σ lights)` at 1, so a puff near a
// torch shows its plain texture, never a dark veil and never blown out; the
// scene's own light (indirect + sun) stays physical and the lamps fill the
// headroom it leaves. Point and spot lights skip their shadow maps (a lamp's
// own model would shade its smoke); the sun's cascades are sampled.

import sf_render_order_bevy::translucent_fog::{froxel_fog_lookup, froxel_fog_apply, FOG_BLEND, FOG_PREMULTIPLIED, FOG_ADD, FOG_MULTIPLY};
import bevy_pbr::render::mesh_view_bindings::view;
import bevy_pbr::render::mesh_view_bindings as view_bindings;
import bevy_pbr::render::pbr_types::{
    PbrInput,
    STANDARD_MATERIAL_FLAGS_ALPHA_MODE_RESERVED_BITS, STANDARD_MATERIAL_FLAGS_ALPHA_MODE_PREMULTIPLIED,
    STANDARD_MATERIAL_FLAGS_ALPHA_MODE_ADD, STANDARD_MATERIAL_FLAGS_ALPHA_MODE_MULTIPLY,
    STANDARD_MATERIAL_FLAGS_UNLIT_BIT,
};
import bevy_pbr::render::{
    mesh_functions,
    forward_io::{Vertex, VertexOutput, FragmentOutput},
    view_transformations::position_world_to_clip,
    pbr_fragment::pbr_input_from_standard_material,
    pbr_functions::{alpha_discard, main_pass_post_lighting_processing, visibility_range_dither},
    pbr_lighting as lighting,
    pbr_lighting::LAYER_BASE,
    pbr_ambient as ambient,
    clustered_forward as clustering,
    clustered_forward::ClusterableObjectIndexRanges,
    shadows,
    mesh_view_types::DIRECTIONAL_LIGHT_FLAGS_SHADOWS_ENABLED_BIT,
    mesh_types::MESH_FLAGS_SHADOW_RECEIVER_BIT,
};
@if(ENVIRONMENT_MAP) import bevy_pbr::light_probe::environment_map;
@if(IRRADIANCE_VOLUME) import bevy_pbr::light_probe::irradiance_volume;
@if(GI_PROBES) import bevy_pbr::render::gi_probes;

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

const UP: vec3<f32> = vec3<f32>(0.0, 1.0, 0.0);

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
    // The lighting is non-directional (`volume_lighting`); the normal only
    // fills Bevy's PBR input.
    out.world_normal = UP;
    out.uv = vertex.uv;
    out.color = p.color;
@if(VERTEX_OUTPUT_INSTANCE_INDEX) {
    out.instance_index = vertex.instance_index;
}
@if(VISIBILITY_RANGE_DITHER) {
    out.visibility_range_dither = mesh_functions::get_visibility_range_dither_level(vertex.instance_index, world_from_local[3]);
}
    return out;
}

// A rough Lambert face at `P` turned toward `L` and seen head-on (N = V = L):
// Bevy's light functions then return `diffuse / π × E` with no N·L (Burley is
// exactly 1/π there) and no specular (F0 = 0).
fn facing(P: vec3<f32>, L: vec3<f32>, diffuse: vec3<f32>) -> lighting::LightingInput {
    var input: lighting::LightingInput;
    input.layers[LAYER_BASE].NdotV = 1.0;
    input.layers[LAYER_BASE].N = L;
    input.layers[LAYER_BASE].R = L;
    input.layers[LAYER_BASE].perceptual_roughness = 1.0;
    input.layers[LAYER_BASE].roughness = 1.0;
    input.P = P;
    input.V = L;
    input.diffuse_color = diffuse;
    input.metallic = 0.0;
    input.F0_dielectric = vec3<f32>(0.0);
    input.F0_metallic = vec3<f32>(0.0);
    input.F_ab = lighting::F_AB(1.0, 1.0);
    return input;
}

// The direction from `P` to a clustered light (up when on top of it).
fn toward_light(light_id: u32, P: vec3<f32>) -> vec3<f32> {
    let d = view_bindings::clustered_lights.data[light_id].position_radius.xyz - P;
    let len2 = dot(d, d);
    if len2 < 1e-8 {
        return UP;
    }
    return d * inverseSqrt(len2);
}

// Morrowind's light saturation (`particles.rs` `saturated_light`, the CPU
// twin): the factor on the puff's texture × colour, in units of "fully lit".
// `scene` (indirect + sun) and `lamps` (point + spot) are white-face
// luminances in those units, `tint` the vertex colour. The original clamps
// `tint × (ambient + Σ lights)` at 1 per channel, i.e. the factor at
// `1 / tint`; the scene's own light stays physical (by day far past the
// clamp) and the lamps fill the headroom it leaves.
fn saturated_light(scene: vec3<f32>, lamps: vec3<f32>, tint: vec3<f32>) -> vec3<f32> {
    let cap = vec3<f32>(1.0) / max(tint, vec3<f32>(1e-4));
    return max(scene, min(scene + lamps, cap));
}

// Diffuse indirect light (Bevy's priority: GI probes, irradiance volume, then
// the environment map; plus the flat ambient), averaged over up and down: a
// volume's non-directional irradiance.
fn indirect_light(pbr: PbrInput, diffuse: vec3<f32>, ranges: ptr<function, ClusterableObjectIndexRanges>) -> vec3<f32> {
    let P = pbr.world_position.xyz;
    var light = vec3<f32>(0.0);
    var found = false;
    var ambient_scale = 1.0;
    @if(GI_PROBES)
    if gi_probes::gi_probe_inside(P) {
        light += 0.5 * (gi_probes::gi_probe_irradiance(P, UP, pbr.V) + gi_probes::gi_probe_irradiance(P, -UP, pbr.V)) * diffuse;
        found = true;
        ambient_scale = gi_probes::gi_probe_ambient_scale();
    }
    @if(IRRADIANCE_VOLUME)
    if !found {
        light += 0.5 * (irradiance_volume::irradiance_volume_light(P, UP, ranges) + irradiance_volume::irradiance_volume_light(P, -UP, ranges)) * diffuse;
        found = true;
    }
@if(ENVIRONMENT_MAP) {
    var above = facing(P, UP, diffuse);
    var below = facing(P, -UP, diffuse);
    light += 0.5 * (environment_map::environment_map_light(&above, ranges, found).diffuse
        + environment_map::environment_map_light(&below, ranges, found).diffuse);
}
    light += ambient::ambient_light(pbr.world_position, UP, UP, 1.0, diffuse, vec3<f32>(0.0), 1.0, vec3<f32>(1.0)) * ambient_scale;
    return light;
}

// The puff's lit colour (pre-fog), exposed like opaque surfaces. `tint` is
// its vertex colour (already in `base_color`), which sets the saturation.
fn volume_lighting(pbr: PbrInput, tint: vec3<f32>) -> vec4<f32> {
    let base = pbr.material.base_color;
    let diffuse = base.rgb * (1.0 - pbr.material.metallic);
    let P = pbr.world_position.xyz;
    let white = vec3<f32>(1.0);

    let view_z = dot(vec4<f32>(
        view.view_from_world[0].z,
        view.view_from_world[1].z,
        view.view_from_world[2].z,
        view.view_from_world[3].z
    ), pbr.world_position);
    let cluster_index = clustering::view_fragment_cluster_index(pbr.frag_coord.xy, view_z, pbr.is_orthographic);
    var ranges = clustering::unpack_clusterable_object_index_ranges(cluster_index);

    // White-face luminances (nits): the lamps' sum, then the scene's own light.
    var lamps = vec3<f32>(0.0);
    for (var i: u32 = ranges.first_point_light_index_offset; i < ranges.first_spot_light_index_offset; i = i + 1u) {
        let light_id = clustering::get_clusterable_object_id(i);
        var input = facing(P, toward_light(light_id, P), white);
        lamps += lighting::point_light(light_id, &input, true, true);
    }
    for (var i: u32 = ranges.first_spot_light_index_offset; i < ranges.first_rect_light_index_offset; i = i + 1u) {
        let light_id = clustering::get_clusterable_object_id(i);
        var input = facing(P, toward_light(light_id, P), white);
        lamps += lighting::spot_light(light_id, &input, true);
    }
    var scene = vec3<f32>(0.0);
    let n_directional_lights = view_bindings::lights.n_directional_lights;
    for (var i: u32 = 0u; i < n_directional_lights; i = i + 1u) {
        let L = view_bindings::lights.directional_lights[i].direction_to_light;
        var input = facing(P, L, white);
        // Carries the atmosphere's transmittance and visible sun ratio: the
        // raster sun's colour is the top-of-atmosphere one, night included.
        let contrib = lighting::directional_light(i, &input, true);
        var shadow = 1.0;
        if any(contrib != vec3<f32>(0.0)) && (pbr.flags & MESH_FLAGS_SHADOW_RECEIVER_BIT) != 0u
                && (view_bindings::lights.directional_lights[i].flags & DIRECTIONAL_LIGHT_FLAGS_SHADOWS_ENABLED_BIT) != 0u {
            shadow = shadows::fetch_directional_shadow(i, pbr.world_position, L, view_z, pbr.frag_coord.xy) * pbr.directional_shadow_factor;
        }
        scene += contrib * shadow;
    }
    scene += indirect_light(pbr, white, &ranges);

    // "Fully lit" in exposed units (shows the plain texture at any exposure, as
    // Morrowind's display-referred clamp does): equals
    // `mw_assets::emissive_nits` at SCENE_EV100 (EV 7), and follows auto exposure: no
    // ~11x blow-out at the night floor (EV 3.5), no dark plume at day EVs.
    let full = 1.0 / view.exposure;
    let light = diffuse * saturated_light(scene / full, lamps / full, tint) * full;
    let emissive = pbr.material.emissive;
    let emissive_light = emissive.rgb * base.a * mix(1.0, view.exposure, emissive.a);
    return vec4<f32>(view.exposure * light + emissive_light, base.a);
}

@fragment
fn fragment(in: VertexOutput, @builtin(front_facing) is_front: bool) -> FragmentOutput {
@if(VISIBILITY_RANGE_DITHER) {
    visibility_range_dither(in.position, in.visibility_range_dither);
}
    var pbr = pbr_input_from_standard_material(in, is_front);
    pbr.material.base_color = alpha_discard(pbr.material.flags, pbr.material.alpha_cutoff, pbr.material.base_color);
    var out: FragmentOutput;
    if (pbr.material.flags & STANDARD_MATERIAL_FLAGS_UNLIT_BIT) == 0u {
        var tint = vec3<f32>(1.0);
@if(VERTEX_COLORS) {
        tint = in.color.rgb;
}
        out.color = volume_lighting(pbr, tint);
    } else {
        out.color = pbr.material.base_color;
    }

    let uv = (in.position.xy - view.main_pass_viewport.xy) / view.main_pass_viewport.zw;
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
