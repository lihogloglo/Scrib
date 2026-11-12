// superfargoth patch (own, P1b: combined medium LUTs on the GPU). Writes the stock combined
// medium LUTs from the per-term unit LUTs (`medium.rs` `MediumLutData`) and the live
// coefficients (`Atmosphere::medium_scattering` / `medium_absorption`, term x scale), so the
// raymarching shaders sample one texel per lookup like stock Bevy (`functions.wesl`
// `sample_density_lut`, `sample_scattering_lut`). Runs only with the static LUTs (`node.rs`,
// keyed on the medium and its scales), not every frame.
//
// Outputs, stock layouts (bevy_pbr 0.20 `medium.rs`):
// - density: `falloff x 2`, row 0 the absorption sum, row 1 the scattering sum.
// - scattering: `falloff x phase`, sum over terms of scattering x falloff x phase.
// Texel centres match the unit LUTs' (both are baked at `(i + 0.5) / resolution`), so
// `textureLoad` gives what the stock CPU bake gave.

import package::atmosphere::types::Atmosphere;

@group(0) @binding(0) var<uniform> atmosphere: Atmosphere;
@group(0) @binding(1) var unit_falloff_lut: texture_2d<f32>;
@group(0) @binding(2) var unit_phase_lut: texture_2d<f32>;
@group(0) @binding(3) var density_lut_out: texture_storage_2d<rgba32float, write>;
@group(0) @binding(4) var scattering_lut_out: texture_storage_2d<rgba32float, write>;

@compute
@workgroup_size(16, 16, 1)
fn main(@builtin(global_invocation_id) idx: vec3<u32>) {
    let falloff_size = textureDimensions(unit_falloff_lut).x;
    let phase_size = textureDimensions(unit_phase_lut).x;
    let x = idx.x;
    let y = idx.y;
    if x >= falloff_size {
        return;
    }
    let terms = atmosphere.medium_term_count;
    let write_density = y < 2u;
    let write_scattering = y < phase_size;
    if !write_density && !write_scattering {
        return;
    }
    var absorption = vec3(0.0);
    var scattering_density = vec3(0.0);
    var scattering = vec3(0.0);
    var falloff = vec4(0.0);
    for (var t = 0u; t < terms; t++) {
        if t % 4u == 0u {
            falloff = textureLoad(unit_falloff_lut, vec2(x, t / 4u), 0);
        }
        let f = falloff[t % 4u];
        let coefficient = atmosphere.medium_scattering[t].xyz;
        absorption += atmosphere.medium_absorption[t].xyz * f;
        scattering_density += coefficient * f;
        if write_scattering && any(coefficient > vec3(0.0)) {
            let phase = textureLoad(unit_phase_lut, vec2(y, t), 0).xyz;
            scattering += coefficient * f * phase;
        }
    }
    if write_density {
        textureStore(density_lut_out, vec2(x, y), vec4(select(absorption, scattering_density, y == 1u), 0.0));
    }
    if write_scattering {
        textureStore(scattering_lut_out, vec2(x, y), vec4(scattering, 0.0));
    }
}
