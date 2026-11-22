// One radial blur pass of the light shaft mask toward the sun's screen position (aerial-shadow.md
// "S3"; the multi-pass form of Mitchell's march, as UE's light shafts): tap `j` samples the
// previous pass at `uv + (sun - uv) * j * step`; each pass's `step` is `taps` times the last
// (`sf_atmos::shafts::pass_step`), so the passes together march `taps^passes` samples with no
// noise. Taps off the screen read its edge (clamped). All four channels: the occlusion mask (R16F)
// and the bloom (RGBA16F, S3b) share this pass.

import bevy_core_pipeline::fullscreen_vertex_shader::FullscreenVertexOutput;
import super::shafts_common::ShaftsPass;

@group(0) @binding(0) var<uniform> shafts: ShaftsPass;
@group(0) @binding(1) var source: texture_2d<f32>;
@group(0) @binding(2) var source_sampler: sampler;

@fragment
fn blur(in: FullscreenVertexOutput) -> @location(0) vec4<f32> {
    let to_sun = (shafts.sun_uv - in.uv) * shafts.step;
    let taps = max(shafts.taps, 1u);
    var sum = vec4(0.0);
    for (var j = 0u; j < taps; j += 1u) {
        sum += textureSampleLevel(source, source_sampler, in.uv + to_sun * f32(j), 0.0);
    }
    return sum / f32(taps);
}
