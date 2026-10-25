// sf_water_bevy: copies Bevy's per-view aerial-view LUT (log in-scatter) into an image the water
// material binds, so the water gets the aerial perspective render_sky gives opaque geometry.

@group(0) @binding(0) var lut: texture_3d<f32>;
@group(0) @binding(1) var lut_out: texture_storage_3d<rgba16float, write>;

@compute @workgroup_size(4, 4, 4)
fn copy_aerial(@builtin(global_invocation_id) id: vec3<u32>) {
    if any(id >= textureDimensions(lut_out)) {
        return;
    }
    textureStore(lut_out, id, textureLoad(lut, id, 0));
}
