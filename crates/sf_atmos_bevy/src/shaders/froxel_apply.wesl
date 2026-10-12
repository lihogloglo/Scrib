// Froxel fog for materials (weather.md "Fog", F3 step 4: the `sf_atmos::fog`
// function transparent surfaces call). Bind the froxels' integrated volume
// (`sf_atmos_bevy::FroxelVolume`: rgb in-scattered light, pre-exposed; a
// transmittance) with a filtering sampler and its params (near, far, slices,
// enabled), then fog the lit colour by the fragment's screen position and
// distance. CPU twin of the slice mapping: `sf_atmos::froxel`.

// The integrated fog between the camera and distance `d` through screen `uv`
// (0,0 top left); past the froxels' far end, the fog to the end.
fn froxel_fog_lookup(volume: texture_3d<f32>, samp: sampler, params: vec4<f32>, uv: vec2<f32>, d: f32) -> vec4<f32> {
    if params.w == 0.0 || d <= 0.0 {
        return vec4(0.0, 0.0, 0.0, 1.0);
    }
    let near = params.x;
    let n = params.z;
    var c = 0.0;
    if d > near {
        c = clamp(log(d / near) / log(params.y / near), 0.0, 1.0) * n;
    }
    let v = textureSampleLevel(volume, samp, vec3(uv, max(c - 0.5, 0.5) / n), 0.0);
    // Inside the first slice: from nothing at the camera.
    return mix(vec4(0.0, 0.0, 0.0, 1.0), v, saturate(c));
}

// Blend modes of `froxel_fog_apply`.
const FOG_BLEND: u32 = 0u;
const FOG_PREMULTIPLIED: u32 = 1u;
const FOG_ADD: u32 = 2u;
const FOG_MULTIPLY: u32 = 3u;

// `color` (lit, pre-exposed, alpha in .a) seen through `fog`: dimmed by the
// transmittance, plus the in-scatter in front of it as the blend mode lets it
// count once (additive and multiplicative surfaces only dim, or the fog
// light would be added again over what is behind them).
fn froxel_fog_apply(color: vec4<f32>, fog: vec4<f32>, mode: u32) -> vec4<f32> {
    switch mode {
        case 1u: {
            return vec4(color.rgb * fog.a + fog.rgb * color.a, color.a);
        }
        case 2u: {
            return vec4(color.rgb * fog.a, color.a);
        }
        case 3u: {
            return vec4(mix(vec3(1.0), color.rgb, fog.a), color.a);
        }
        default: {
            return vec4(color.rgb * fog.a + fog.rgb, color.a);
        }
    }
}
