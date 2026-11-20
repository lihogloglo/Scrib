import package::render::{
    mesh_view_types::POINT_LIGHT_FLAGS_SPOT_LIGHT_Y_NEGATIVE,
    mesh_view_bindings as view_bindings,
    shadow_sampling::{
        SPOT_SHADOW_TEXEL_SIZE, sample_shadow_cubemap, sample_shadow_cubemap_pcss,
        sample_shadow_map_hardware, // superfargoth patch (own: lamp shadow filter)
        sample_shadow_map, sample_shadow_map_pcss, sample_shadow_map_directional_pcss,
        random_rotation_matrix, // superfargoth patch (own, CW-13b: 2D point shadow atlas soft PCF)
    }
};

import bevy_render::{
    color_operations::hsv_to_rgb,
    maths::{orthonormalize, PI_2}
};

const flip_z: vec3<f32> = vec3<f32>(1.0, 1.0, -1.0);

// superfargoth patch (own, CW-13b: 2D point shadow atlas)
// Projects light-space direction `frag_ls` onto Bevy's `CUBE_MAP_FACES` (0:+X, 1:-X, 2:+Y, 3:-Y,
// 4:-Z, 5:+Z), returning `(face_ndc.x, face_ndc.y, major_axis_magnitude, f32(face_index))`.
@if(POINT_SHADOW_ATLAS)
fn point_shadow_atlas_project(frag_ls: vec3<f32>) -> vec4<f32> {
    let a = abs(frag_ls);
    if (a.x >= a.y && a.x >= a.z) {
        let inv = 1.0 / max(a.x, 1e-6);
        if (frag_ls.x >= 0.0) {
            return vec4<f32>(frag_ls.z * inv, frag_ls.y * inv, a.x, 0.0);
        } else {
            return vec4<f32>(-frag_ls.z * inv, frag_ls.y * inv, a.x, 1.0);
        }
    } else if (a.y >= a.z) {
        let inv = 1.0 / max(a.y, 1e-6);
        if (frag_ls.y >= 0.0) {
            return vec4<f32>(frag_ls.x * inv, frag_ls.z * inv, a.y, 2.0);
        } else {
            return vec4<f32>(frag_ls.x * inv, -frag_ls.z * inv, a.y, 3.0);
        }
    } else {
        let inv = 1.0 / max(a.z, 1e-6);
        if (frag_ls.z < 0.0) {
            // CUBE_MAP_FACES[4] looks toward Vec3::NEG_Z with up = +Y.
            return vec4<f32>(frag_ls.x * inv, frag_ls.y * inv, a.z, 4.0);
        } else {
            // CUBE_MAP_FACES[5] looks toward Vec3::Z with up = +Y.
            return vec4<f32>(-frag_ls.x * inv, frag_ls.y * inv, a.z, 5.0);
        }
    }
}

@if(POINT_SHADOW_ATLAS)
fn sample_point_shadow_atlas(light_id: u32, frag_ls: vec3<f32>, depth: f32) -> f32 {
    let proj = point_shadow_atlas_project(frag_ls);
    let face_index = u32(proj.w);
    let tile_index = light_id * 6u + face_index;
    if (tile_index >= arrayLength(&view_bindings::point_shadow_atlas_tiles)) {
        return 1.0;
    }
    let tile = view_bindings::point_shadow_atlas_tiles[tile_index];
    let atlas_kind = tile.flags_and_size & 3u;
    if (atlas_kind == 0u) {
        return 1.0;
    }
    let tile_px = max(f32((tile.flags_and_size >> 2u) & 0x3FFFu), 1.0);
    let inv_guard_tan = f32(tile.flags_and_size >> 16u) * (1.0 / 65535.0);
    let tile_uv = proj.xy * inv_guard_tan * vec2<f32>(0.5, -0.5) + vec2<f32>(0.5, 0.5);
    let half_texel = 0.5 * tile.uv_scale / tile_px;
    let uv_min = tile.uv_offset + vec2<f32>(half_texel);
    let uv_max = tile.uv_offset + vec2<f32>(tile.uv_scale - half_texel);
    let atlas_uv = clamp(tile.uv_offset + tile_uv * tile.uv_scale, uv_min, uv_max);
    if (atlas_kind == 2u) {
        return textureSampleCompareLevel(
            view_bindings::point_shadow_atlas_live,
            view_bindings::point_shadow_textures_comparison_sampler,
            atlas_uv,
            depth,
        );
    }
    return textureSampleCompareLevel(
        view_bindings::point_shadow_atlas_static,
        view_bindings::point_shadow_textures_comparison_sampler,
        atlas_uv,
        depth,
    );
}

fn fetch_point_shadow(
    light_id: u32,
    frag_position: vec4<f32>,
    surface_normal: vec3<f32>,
    frag_coord_xy: vec2<f32>,
) -> f32 {
    let light = &view_bindings::clustered_lights.data[light_id];

    // superfargoth patch (own: point shadow fade): `spot_light_tan_angle` carries
    // `PointLightShadowFade` (0..=1) for point lights without growing `GpuClusteredLight`.
    let shadow_fade = (*light).spot_light_tan_angle;
    if (shadow_fade <= 0.0) {
        return 1.0;
    }

    // because the shadow maps align with the axes and the frustum planes are at 45 degrees
    // we can get the worldspace depth by taking the largest absolute axis
    let surface_to_light = (*light).position_radius.xyz - frag_position.xyz;
    let surface_to_light_abs = abs(surface_to_light);
    let distance_to_light = max(surface_to_light_abs.x, max(surface_to_light_abs.y, surface_to_light_abs.z));

    @if(POINT_SHADOW_ATLAS) {
        // Look up this face's tile to get per-tile resolution for normal-bias scaling.
        // Do not early-out on an empty raw face before biasing: a fragment near a 45 deg seam can
        // cross into an adjacent non-empty face after `normal_offset + depth_offset`.
        let raw_proj = point_shadow_atlas_project(-surface_to_light);
        let raw_tile_index = light_id * 6u + u32(raw_proj.w);
        var raw_tile_px = 256.0;
        if (raw_tile_index < arrayLength(&view_bindings::point_shadow_atlas_tiles)) {
            let raw_tile = view_bindings::point_shadow_atlas_tiles[raw_tile_index];
            if ((raw_tile.flags_and_size & 3u) != 0u) {
                raw_tile_px = max(f32((raw_tile.flags_and_size >> 2u) & 0x3FFFu), 1.0);
            }
        }
        // `shadow_normal_bias` carries `raw_bias * 2.0` under POINT_SHADOW_ATLAS; divide by `tile_px`.
        let normal_offset = ((*light).shadow_normal_bias / raw_tile_px) * distance_to_light * surface_normal.xyz;
        let depth_offset = (*light).shadow_depth_bias * normalize(surface_to_light.xyz);
        let offset_position = frag_position.xyz + normal_offset + depth_offset;
        let frag_ls = offset_position.xyz - (*light).position_radius.xyz;
        let abs_position_ls = abs(frag_ls);
        let major_axis_magnitude = max(abs_position_ls.x, max(abs_position_ls.y, abs_position_ls.z));
        let zw = -major_axis_magnitude * (*light).light_custom_data.xy + (*light).light_custom_data.zw;
        let depth = zw.x / zw.y;
        let shadow = sample_point_shadow_atlas(light_id, frag_ls, depth);
        return mix(1.0, shadow, shadow_fade);
    } @else {
        // The normal bias here is already scaled by the texel size at 1 world unit from the light.
        // The texel size increases proportionally with distance from the light so multiplying by
        // distance to light scales the normal bias to the texel size at the fragment distance.
        let normal_offset = (*light).shadow_normal_bias * distance_to_light * surface_normal.xyz;
        let depth_offset = (*light).shadow_depth_bias * normalize(surface_to_light.xyz);
        let offset_position = frag_position.xyz + normal_offset + depth_offset;

        // similar largest-absolute-axis trick as above, but now with the offset fragment position
        let frag_ls = offset_position.xyz - (*light).position_radius.xyz ;
        let abs_position_ls = abs(frag_ls);
        let major_axis_magnitude = max(abs_position_ls.x, max(abs_position_ls.y, abs_position_ls.z));

        // NOTE: These simplifications come from multiplying:
        // projection * vec4(0, 0, -major_axis_magnitude, 1.0)
        // and keeping only the terms that have any impact on the depth.
        // Projection-agnostic approach:
        let zw = -major_axis_magnitude * (*light).light_custom_data.xy + (*light).light_custom_data.zw;
        let depth = zw.x / zw.y;

        var shadow: f32;
        if ((*light).soft_shadow_size > 0.0) {
            shadow = sample_shadow_cubemap_pcss(
                frag_ls * flip_z,
                distance_to_light,
                depth,
                light_id,
                (*light).soft_shadow_size,
                frag_coord_xy,
            );
        } else {
            // Do the lookup, using HW PCF and comparison. Cubemaps assume a left-handed
            // coordinate space, so we have to flip the z-axis when sampling.
            shadow = sample_shadow_cubemap(frag_ls * flip_z, distance_to_light, depth, light_id, frag_coord_xy);
        }
        return mix(1.0, shadow, shadow_fade);
    }
}

fn fetch_spot_shadow(
    light_id: u32,
    frag_position: vec4<f32>,
    surface_normal: vec3<f32>,
    near_z: f32,
    frag_coord_xy: vec2<f32>,
) -> f32 {
    let light = &view_bindings::clustered_lights.data[light_id];

    let surface_to_light = (*light).position_radius.xyz - frag_position.xyz;

    // construct the light view matrix
    var spot_dir = vec3<f32>((*light).light_custom_data.x, 0.0, (*light).light_custom_data.y);
    // reconstruct spot dir from x/z and y-direction flag
    spot_dir.y = sqrt(max(0.0, 1.0 - spot_dir.x * spot_dir.x - spot_dir.z * spot_dir.z));
    if (((*light).flags & POINT_LIGHT_FLAGS_SPOT_LIGHT_Y_NEGATIVE) != 0u) {
        spot_dir.y = -spot_dir.y;
    }

    // view matrix z_axis is the reverse of transform.forward()
    let fwd = -spot_dir;
    let distance_to_light = dot(fwd, surface_to_light);
    let offset_position =
        -surface_to_light
        + ((*light).shadow_depth_bias * normalize(surface_to_light))
        + (surface_normal.xyz * (*light).shadow_normal_bias) * distance_to_light;

    let light_inv_rot = orthonormalize(fwd);

    // because the matrix is a pure rotation matrix, the inverse is just the transpose, and to calculate
    // the product of the transpose with a vector we can just post-multiply instead of pre-multiplying.
    // this allows us to keep the matrix construction code identical between CPU and GPU.
    let projected_position = offset_position * light_inv_rot;

    // divide xy by perspective matrix "f" and by -projected.z (projected.z is -projection matrix's w)
    // to get ndc coordinates
    let f_div_minus_z = 1.0 / ((*light).spot_light_tan_angle * -projected_position.z);
    let shadow_xy_ndc = projected_position.xy * f_div_minus_z;
    // convert to uv coordinates
    let shadow_uv = shadow_xy_ndc * vec2<f32>(0.5, -0.5) + vec2<f32>(0.5, 0.5);

    let depth = near_z / -projected_position.z;

    // If soft shadows are enabled, use the PCSS path.
    let array_index = i32(light_id) + view_bindings::lights.spot_light_shadowmap_offset;
    if ((*light).soft_shadow_size > 0.0) {
        return sample_shadow_map_pcss(
            shadow_uv,
            depth,
            array_index,
            frag_coord_xy,
            SPOT_SHADOW_TEXEL_SIZE,
            (*light).soft_shadow_size,
        );
    }

    // superfargoth patch (own: lamp shadow filter): 2x2 hardware PCF for spot lights, directional keeps the view's filter.
    @if(POINT_SPOT_SHADOW_FILTER_HARDWARE_2X2)
    return sample_shadow_map_hardware(shadow_uv, depth, array_index);
    @else
    return sample_shadow_map(
        shadow_uv,
        depth,
        array_index,
        frag_coord_xy,
        SPOT_SHADOW_TEXEL_SIZE,
    );
}

fn get_cascade_index(light_id: u32, view_z: f32) -> u32 {
    let light = &view_bindings::lights.directional_lights[light_id];

    for (var i: u32 = 0u; i < (*light).num_cascades; i = i + 1u) {
        if (-view_z < (*light).cascades[i].far_bound) {
            return i;
        }
    }
    return (*light).num_cascades;
}

/// Converts from world space to the uv position in the light's shadow map.
///
/// The depth is stored in the return value's z coordinate. If the return value's
/// w coordinate is 0.0, then we landed outside the shadow map entirely.
fn world_to_directional_light_local(
    light_id: u32,
    cascade_index: u32,
    offset_position: vec4<f32>
) -> vec4<f32> {
    let light = &view_bindings::lights.directional_lights[light_id];
    let cascade = &(*light).cascades[cascade_index];

    let offset_position_clip = (*cascade).clip_from_world * offset_position;
    if (offset_position_clip.w <= 0.0) {
        return vec4(0.0);
    }
    let offset_position_ndc = offset_position_clip.xyz / offset_position_clip.w;
    // No shadow outside the orthographic projection volume
    if (any(offset_position_ndc.xy < vec2<f32>(-1.0)) || offset_position_ndc.z < 0.0
            || any(offset_position_ndc > vec3<f32>(1.0))) {
        return vec4(0.0);
    }

    // compute texture coordinates for shadow lookup, compensating for the Y-flip difference
    // between the NDC and texture coordinates
    let flip_correction = vec2<f32>(0.5, -0.5);
    let light_local = offset_position_ndc.xy * flip_correction + vec2<f32>(0.5, 0.5);

    let depth = offset_position_ndc.z;

    return vec4(light_local, depth, 1.0);
}

fn sample_directional_cascade(
    light_id: u32,
    cascade_index: u32,
    frag_position: vec4<f32>,
    surface_normal: vec3<f32>,
    frag_coord_xy: vec2<f32>,
) -> f32 {
    let light = &view_bindings::lights.directional_lights[light_id];
    let cascade = &(*light).cascades[cascade_index];

    // The normal bias is scaled to the texel size.
    let normal_offset = (*light).shadow_normal_bias * (*cascade).texel_size * surface_normal.xyz;
    let depth_offset = (*light).shadow_depth_bias * (*light).direction_to_light.xyz;
    let offset_position = vec4<f32>(frag_position.xyz + normal_offset + depth_offset, frag_position.w);

    let light_local = world_to_directional_light_local(light_id, cascade_index, offset_position);
    if (light_local.w == 0.0) {
        return 1.0;
    }

    let array_index = i32((*light).depth_texture_base_index + cascade_index);
    let texel_size = (*cascade).texel_size;

    // If soft shadows are enabled, use the world-unit directional PCSS path (superfargoth patch, H2).
    if ((*light).soft_shadow_size > 0.0) {
        return sample_shadow_map_directional_pcss(
            light_local.xy,
            light_local.z,
            array_index,
            frag_coord_xy,
            texel_size,
            (*light).soft_shadow_size,
            (*cascade).clip_from_world,
            surface_normal,
        );
    }

    return sample_shadow_map(
        light_local.xy,
        light_local.z,
        array_index,
        frag_coord_xy,
        texel_size,
    );
}

fn fetch_directional_shadow(
    light_id: u32,
    frag_position: vec4<f32>,
    surface_normal: vec3<f32>,
    view_z: f32,
    frag_coord_xy: vec2<f32>,
) -> f32 {
    let light = &view_bindings::lights.directional_lights[light_id];
    let cascade_index = get_cascade_index(light_id, view_z);

    if (cascade_index >= (*light).num_cascades) {
        return 1.0;
    }

    var shadow = sample_directional_cascade(
        light_id,
        cascade_index,
        frag_position,
        surface_normal,
        frag_coord_xy,
    );

    // Blend with the next cascade, if there is one.
    let next_cascade_index = cascade_index + 1u;
    if (next_cascade_index < (*light).num_cascades) {
        let this_far_bound = (*light).cascades[cascade_index].far_bound;
        let next_near_bound = (1.0 - (*light).cascades_overlap_proportion) * this_far_bound;
        if (-view_z >= next_near_bound) {
            let next_shadow = sample_directional_cascade(
                light_id,
                next_cascade_index,
                frag_position,
                surface_normal,
                frag_coord_xy,
            );
            shadow = mix(shadow, next_shadow, (-view_z - next_near_bound) / (this_far_bound - next_near_bound));
        }
    }
    // superfargoth patch (own, CW-8: directional shadow fade): 1 = stock.
    return mix(1.0, shadow, (*light).shadow_fade);
}

fn cascade_debug_visualization(
    output_color: vec3<f32>,
    light_id: u32,
    view_z: f32,
) -> vec3<f32> {
    let overlay_alpha = 0.95;
    let cascade_index = get_cascade_index(light_id, view_z);
    let cascade_color_hsv = vec3(
        f32(cascade_index) / f32(u32(constants::MAX_CASCADES_PER_LIGHT) + 1u) * PI_2,
        1.0,
        0.5
    );
    let cascade_color = hsv_to_rgb(cascade_color_hsv);
    return vec3<f32>(
        (1.0 - overlay_alpha) * output_color.rgb + overlay_alpha * cascade_color
    );
}
