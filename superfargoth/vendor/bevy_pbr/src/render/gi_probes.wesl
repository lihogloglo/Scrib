// superfargoth patch (own, GI): DDGI probe-volume sampling (Majercik et al. 2019, "Dynamic
// Diffuse Global Illumination with Ray-Traced Irradiance Fields", JCGT 8(2)), written from the
// paper. The atlases come from superfargoth sf_gi_bevy (probe_update.wesl); the octahedral
// mapping, tile layout and Chebyshev weight mirror superfargoth sf_gi::ddgi (tested there).

import package::render::mesh_view_bindings::{
    gi_probe_irradiance_atlas,
    gi_probe_visibility_atlas,
    gi_probe_sampler,
    gi_probe_volume,
};

fn gi_sign_not_zero(v: vec2<f32>) -> vec2<f32> {
    return select(vec2(-1.0), vec2(1.0), v >= vec2(0.0));
}

// Octahedral encoding of a unit direction to [-1, 1]^2 (sf_gi::oct_encode).
fn gi_oct_encode(n: vec3<f32>) -> vec2<f32> {
    let p = n.xy / (abs(n.x) + abs(n.y) + abs(n.z));
    if (n.z < 0.0) {
        return (1.0 - abs(p.yx)) * gi_sign_not_zero(p);
    }
    return p;
}

// Atlas UV of octahedral point `oct` in probe `c`'s tile (sf_gi::AtlasLayout::uv).
fn gi_atlas_uv(c: vec3<u32>, oct: vec2<f32>, inner: f32, atlas_size: vec2<f32>) -> vec2<f32> {
    let tile = vec2<f32>(vec2(c.x + gi_probe_volume.dims.x * c.y, c.z));
    let texel = tile * (inner + 2.0) + 1.0 + (oct * 0.5 + 0.5) * inner;
    return texel / atlas_size;
}

// Whether a world point lies inside the probe volume's box.
fn gi_probe_inside(world_position: vec3<f32>) -> bool {
    let local = (world_position - gi_probe_volume.origin_spacing.xyz) / gi_probe_volume.origin_spacing.w;
    return all(local >= vec3(0.0)) && all(local <= vec3<f32>(gi_probe_volume.dims.xyz));
}

// Diffuse irradiance / pi at a surface point (unit normal N, unit direction to the eye V), in the
// units of Bevy's environment map diffuse (multiply by the diffuse colour): the eight
// surrounding probes, weighted trilinear x smooth back-face x Chebyshev visibility, with the
// paper's self-shadow bias (0.2 N + 0.8 V) and its perceptual (square-root) blend.
fn gi_probe_irradiance(world_position: vec3<f32>, N: vec3<f32>, V: vec3<f32>) -> vec3<f32> {
    let volume = gi_probe_volume;
    let spacing = volume.origin_spacing.w;
    let dims = volume.dims.xyz;
    let irr_inner = f32(volume.dims.w);
    let vis_inner = volume.bias_intensity.z;
    let irr_size = vec2<f32>(textureDimensions(gi_probe_irradiance_atlas));
    let vis_size = vec2<f32>(textureDimensions(gi_probe_visibility_atlas));

    let biased = world_position + (N * 0.2 + V * 0.8) * volume.bias_intensity.x;
    // Probe space: probe (0,0,0) at 0.
    let local = clamp((biased - volume.origin_spacing.xyz) / spacing - 0.5, vec3(0.0), vec3<f32>(dims - 1u));
    let base = min(vec3<u32>(local), dims - 1u);
    let alpha = local - vec3<f32>(base);
    let irr_oct = gi_oct_encode(N);

    var sum = vec3(0.0);
    var weight_sum = 0.0;
    for (var i = 0u; i < 8u; i++) {
        let offset = vec3(i & 1u, (i >> 1u) & 1u, (i >> 2u) & 1u);
        let c = min(base + offset, dims - 1u);
        let probe_position = volume.origin_spacing.xyz + (vec3<f32>(c) + 0.5) * spacing;

        // Smooth back-face: probes behind the surface fade, never fully (DDGI's (x+1)/2 squared + 0.2).
        let true_dir = normalize(probe_position - world_position);
        var w = (dot(true_dir, N) + 1.0) * 0.5;
        w = w * w + 0.2;

        // Chebyshev visibility from the probe's distance moments towards the biased point.
        let to_point = biased - probe_position;
        let distance = max(length(to_point), 0.0001);
        let moments = textureSampleLevel(
            gi_probe_visibility_atlas,
            gi_probe_sampler,
            gi_atlas_uv(c, gi_oct_encode(to_point / distance), vis_inner, vis_size),
            0.0,
        ).rg;
        if (distance > moments.x) {
            let variance = abs(moments.x * moments.x - moments.y);
            let d = distance - moments.x;
            let chebyshev = variance / (variance + d * d);
            w *= max(chebyshev * chebyshev * chebyshev, 0.0);
        }
        w = max(w, 0.000001);
        // Crush tiny weights (light leaking through a tiny visibility is very visible).
        let crush = 0.2;
        if (w < crush) {
            w *= w * w / (crush * crush);
        }
        let tri = mix(1.0 - alpha, alpha, vec3<f32>(offset));
        w *= tri.x * tri.y * tri.z;

        let irradiance = textureSampleLevel(
            gi_probe_irradiance_atlas,
            gi_probe_sampler,
            gi_atlas_uv(c, irr_oct, irr_inner, irr_size),
            0.0,
        ).rgb;
        sum += sqrt(max(irradiance, vec3(0.0))) * w;
        weight_sum += w;
    }
    if (weight_sum <= 0.0) {
        return vec3(0.0);
    }
    let s = sum / weight_sum;
    return s * s * volume.bias_intensity.y;
}
