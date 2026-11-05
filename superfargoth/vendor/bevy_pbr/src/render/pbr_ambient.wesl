import package::render::{
    pbr_lighting::{EnvBRDFApprox, F_AB},
    mesh_view_bindings::lights,
    mesh_view_types::AMBIENT_LIGHT_FLAGS_DIFFUSE_ONLY_BIT, // superfargoth patch (own, N1)
};

/// A precomputed `NdotV` is provided because it is computed regardless,
/// but `world_normal` and the view vector `V` are provided separately for more advanced uses.
fn ambient_light(
    world_position: vec4<f32>,
    world_normal: vec3<f32>,
    V: vec3<f32>,
    NdotV: f32,
    diffuse_color: vec3<f32>,
    specular_color: vec3<f32>,
    perceptual_roughness: f32,
    occlusion: vec3<f32>,
) -> vec3<f32> {
    let diffuse_ambient = EnvBRDFApprox(diffuse_color, F_AB(1.0, NdotV));
    let specular_ambient = EnvBRDFApprox(specular_color, F_AB(perceptual_roughness, NdotV));

    // No real world material has specular values under 0.02, so we use this range as a
    // "pre-baked specular occlusion" that extinguishes the fresnel term, for artistic control.
    // See: https://google.github.io/filament/Filament.md.html#specularocclusion
    var specular_occlusion = saturate(dot(specular_color, vec3(50.0 * 0.33)));
    // superfargoth patch (own, N1: ambient specular toggle): a flat fill has no visible source,
    // so with `AmbientLightSpecular(false)` it lights the diffuse lobe only (no sheen of nothing).
    if (lights.ambient_light_flags & AMBIENT_LIGHT_FLAGS_DIFFUSE_ONLY_BIT) != 0u {
        specular_occlusion = 0.0;
    }

    return (diffuse_ambient + specular_ambient * specular_occlusion) * lights.ambient_color.rgb * occlusion;
}
