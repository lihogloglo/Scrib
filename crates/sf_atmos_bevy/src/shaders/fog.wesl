// Height fog and ground-fog banks (weather.md "Fog", F2): the uniform and
// the closed-form optical depth, shared by the cloud trace (which marches the
// banks' noise) and the composite (which applies the fog per pixel). The CPU
// twin is `sf_atmos::fog` (change both together). F3's froxels and the
// transparent materials' `sf_atmos::fog` function build on these terms.

// Field order is the layout: it matches `FogUniform` (render.rs).
struct Fog {
    albedo: vec3<f32>,
    phase_g: f32,
    height_density: f32,
    height_falloff: f32,
    height_base: f32,
    bank_density: f32,
    bank_falloff: f32,
    bank_base: f32,
    bank_coverage: f32,
    bank_sharpness: f32,
    bank_off: vec2<f32>,
    detail_off: vec2<f32>,
    warp_off: vec2<f32>,
    bank_tile: f32,
    detail_tile: f32,
    warp_tile: f32,
    warp_amount: f32,
    ambient: f32,
    far: f32,
    sky_distance: f32,
    steps: u32,
    enabled: u32,
};

// Below a layer's base its density stops growing past e^2 × (`sf_atmos::fog::MAX_EXPONENT`).
const MAX_EXPONENT: f32 = 2.0;

// Optical depth of the layer `density · e^(−(h − base) / falloff)` along a
// straight ray from altitude `h0` with vertical cosine `mu`, over `d` meters
// (Unreal's exponential height fog integral, planar). A difference of the
// two end densities, so rays diving far below the layer top never overflow.
fn layer_depth(density: f32, falloff: f32, base: f32, h0: f32, mu: f32, d: f32) -> f32 {
    if density <= 0.0 || d <= 0.0 {
        return 0.0;
    }
    let k = 1.0 / falloff;
    let e0 = exp(min(-(h0 - base) * k, MAX_EXPONENT));
    let x = k * mu * d;
    if abs(x) < 1e-3 {
        return density * d * e0 * (1.0 - 0.5 * x);
    }
    let e1 = exp(min(-(h0 + mu * d - base) * k, MAX_EXPONENT));
    return density * (e0 - e1) / (k * mu);
}

// (height layer, bank layer before the noise) optical depths.
fn fog_depths(f: Fog, h0: f32, mu: f32, d: f32) -> vec2<f32> {
    return vec2(
        layer_depth(f.height_density, f.height_falloff, f.height_base, h0, mu, d),
        layer_depth(f.bank_density, f.bank_falloff, f.bank_base, h0, mu, d),
    );
}

// Fog droplets: a forward Henyey-Greenstein lobe over an isotropic floor
// (multiple scattering spreads the light in dense fog).
fn fog_phase(g: f32, cos_t: f32) -> f32 {
    let g2 = g * g;
    let hg = (1.0 - g2) / (4.0 * 3.14159265 * pow(max(1.0 + g2 - 2.0 * g * cos_t, 1e-4), 1.5));
    return mix(1.0 / (4.0 * 3.14159265), hg, 0.7);
}
