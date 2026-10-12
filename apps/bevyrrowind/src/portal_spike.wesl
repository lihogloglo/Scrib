// Scaffold `portal_spike` (M8 spike, superfargoth docs/specs/portals.md):
// the door opening shows the portal camera's HDR image, sampled at this
// fragment's screen position (Bevy's `mirror` example technique). The image
// is linear and already exposed by the portal camera; the main camera's
// tonemapping applies once, after this pass.

import bevy_pbr::render::{forward_io::VertexOutput, mesh_view_bindings::view};

@group(constants::MATERIAL_BIND_GROUP) @binding(0) var portal_view: texture_2d<f32>;
@group(constants::MATERIAL_BIND_GROUP) @binding(1) var portal_sampler: sampler;

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    let uv = (in.position.xy - view.viewport.xy) / view.viewport.zw;
    return vec4<f32>(textureSampleLevel(portal_view, portal_sampler, uv, 0.0).rgb, 1.0);
}
