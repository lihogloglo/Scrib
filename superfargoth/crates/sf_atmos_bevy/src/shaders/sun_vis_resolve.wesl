// Sun-visibility temporal resolve (aerial-shadow.md "Architecture review", FG-12 A2;
// `sun_visibility.rs`): this frame's march (`sun_vis_march.wesl`: V_atmo, V_fog, scene distance)
// blended with last frame's result, reprojected through the pixel's world position (sky: its
// direction only) with last frame's clip-from-world and clipped to this frame's 3×3 min/max (the
// froxels' Karis 2014 rule). Off screen, the first frame or a reset: this frame alone. Output
// (Rg16Float): R = V_atmo (what `render_sky` reads, `bevy_pbr::AtmosphereLightShafts`), G = V_fog
// (the composite's fog sun term). CPU twin: `sf_atmos::sun_visibility::resolve`.

import bevy_core_pipeline::fullscreen_vertex_shader::FullscreenVertexOutput;
import bevy_pbr::atmosphere::functions::uv_to_ray_direction;
import super::sun_vis_common::{SunVisPass, clip_history2};

@group(1) @binding(0) var<uniform> sv: SunVisPass;
@group(1) @binding(1) var current: texture_2d<f32>;
@group(1) @binding(2) var history: texture_2d<f32>;
@group(1) @binding(3) var linear_sampler: sampler;

@fragment
fn resolve(in: FullscreenVertexOutput) -> @location(0) vec4<f32> {
    let px = vec2<i32>(floor(in.position.xy));
    let size_max = vec2<i32>(sv.half_size) - 1;
    let c = textureLoad(current, px, 0);
    var value = c.xy;
    if sv.history_valid != 0u && sv.history > 0.0 {
        let rd = uv_to_ray_direction(in.uv);
        // Geometry: its world point in last frame's render space; sky: the direction.
        var clip = sv.prev_clip_from_world * vec4(rd, 0.0);
        if c.z > 0.0 {
            clip = sv.prev_clip_from_world * vec4(sv.camera + rd * c.z + sv.origin_shift, 1.0);
        }
        if clip.w > 0.0 {
            let puv = clip.xy / clip.w * vec2(0.5, -0.5) + 0.5;
            if all(puv >= vec2(0.0)) && all(puv <= vec2(1.0)) {
                var lo = value;
                var hi = value;
                for (var dy = -1; dy <= 1; dy += 1) {
                    for (var dx = -1; dx <= 1; dx += 1) {
                        let n = textureLoad(current, clamp(px + vec2(dx, dy), vec2(0), size_max), 0).xy;
                        lo = min(lo, n);
                        hi = max(hi, n);
                    }
                }
                let prev = clip_history2(textureSampleLevel(history, linear_sampler, puv, 0.0).xy, lo, hi);
                value = mix(value, prev, clamp(sv.history, 0.0, 1.0));
            }
        }
    }
    return vec4(value, 0.0, 1.0);
}
