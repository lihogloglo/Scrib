// worldgen-view terrain, main pass (slice WG-V2): Bevy's StandardMaterial fragment (forward) plus contour lines
// of the world height. `refs.rs` `contour_ink` is the CPU mirror of `contour_ink` here: keep them identical.
// The prepass and shadow passes use Bevy's own shaders.

import bevy_pbr::render::{
    forward_io::{VertexOutput, FragmentOutput},
    pbr_fragment::pbr_input_from_standard_material,
    pbr_functions::{alpha_discard, apply_pbr_lighting, main_pass_post_lighting_processing},
};

// (minor interval m, major interval m, minor width px, major width px).
@group(constants::MATERIAL_BIND_GROUP) @binding(100) var<uniform> contour_style: vec4<f32>;
// (ink rgb, linear; strength, 0 = contours off).
@group(constants::MATERIAL_BIND_GROUP) @binding(101) var<uniform> contour_colour: vec4<f32>;

fn contour_line(h: f32, step: f32, dh: f32, width: f32) -> f32 {
    if step <= 0.0 {
        return 0.0;
    }
    let d = abs(fract(h / step + 0.5) - 0.5) * step / dh;
    return clamp(0.5 * width + 0.5 - d, 0.0, 1.0);
}

fn contour_ink(h: f32, dh_per_px: f32) -> f32 {
    let dh = max(dh_per_px, 1e-4);
    let minor = contour_line(h, contour_style.x, dh, contour_style.z) * 0.6;
    return max(minor, contour_line(h, contour_style.y, dh, contour_style.w));
}

@fragment
fn fragment(in: VertexOutput, @builtin(front_facing) is_front: bool) -> FragmentOutput {
    // Derivatives first, in uniform control flow.
    let h = in.world_position.y;
    let dh = fwidth(h);

    var pbr = pbr_input_from_standard_material(in, is_front);
    pbr.material.base_color = alpha_discard(pbr.material.flags, pbr.material.alpha_cutoff, pbr.material.base_color);
    if contour_colour.w > 0.0 {
        let ink = contour_ink(h, dh) * contour_colour.w;
        pbr.material.base_color = vec4<f32>(mix(pbr.material.base_color.rgb, contour_colour.rgb, ink), pbr.material.base_color.a);
    }
    var out: FragmentOutput;
    out.color = apply_pbr_lighting(pbr);
    out.color = main_pass_post_lighting_processing(pbr, out.color);
    return out;
}
