// Froxel integration (weather.md "Fog", F3, step 3): front to back along each
// tile's column of slices, Hillaire 2015's energy-conserving step (the
// scattering integrated analytically over each slab). CPU twin:
// `sf_atmos::froxel::integrate_step`.

import super::froxel_common::{Froxels, slice_distance};

@group(0) @binding(0) var<uniform> fx: Froxels;
// rgb: σs × incoming light (pre-exposed); a: σt (1/m).
@group(0) @binding(1) var volume: texture_3d<f32>;
// rgb: in-scattered light to the slice's far boundary; a: transmittance.
@group(0) @binding(2) var integrated: texture_storage_3d<rgba16float, write>;

@compute @workgroup_size(8, 8, 1)
fn integrate(@builtin(global_invocation_id) id: vec3<u32>) {
    let size = vec3<u32>(fx.size);
    if id.x >= size.x || id.y >= size.y {
        return;
    }
    var light = vec3(0.0);
    var t = 1.0;
    var d0 = 0.0;
    for (var s = 0u; s < size.z; s += 1u) {
        let d1 = slice_distance(fx, f32(s + 1u) / f32(size.z));
        let dz = d1 - d0;
        let v = textureLoad(volume, vec3<i32>(vec2<i32>(id.xy), i32(s)), 0);
        let tr = exp(-v.a * dz);
        let k = select(dz, (1.0 - tr) / max(v.a, 1e-7), v.a > 1e-7);
        light += t * v.rgb * k;
        t *= tr;
        textureStore(integrated, vec3<i32>(vec2<i32>(id.xy), i32(s)), vec4(light, t));
        d0 = d1;
    }
}
