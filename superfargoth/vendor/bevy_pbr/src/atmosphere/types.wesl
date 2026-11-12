struct Atmosphere {
    ground_albedo: vec3<f32>,
    /// Radius of the planet
    /// Units: m
    inner_radius: f32,
    /// Radius at which we consider the atmosphere to 'end' for out calculations (from center of planet)
    /// Units: m
    outer_radius: f32,
    /// Transform from world space to atmosphere space, inverse of the atmosphere entity's transform
    world_to_atmosphere: mat4x4<f32>,
    // superfargoth patch (own, P1: live medium scales): the medium's terms (at most
    // `MAX_MEDIUM_TERMS` = 8, `medium.rs`) and each term's coefficients (m^-1, xyz) times its
    // live `ScatteringMediumScales`, multiplied into the unit falloff and phase LUTs.
    medium_term_count: u32,
    medium_scattering: array<vec4<f32>, 8>,
    medium_absorption: array<vec4<f32>, 8>,
}

struct AtmosphereSettings {
    transmittance_lut_size: vec2<u32>,
    multiscattering_lut_size: vec2<u32>,
    sky_view_lut_size: vec2<u32>,
    aerial_view_lut_size: vec3<u32>,
    transmittance_lut_samples: u32,
    multiscattering_lut_dirs: u32,
    multiscattering_lut_samples: u32,
    sky_view_lut_samples: u32,
    aerial_view_lut_samples: u32,
    aerial_view_lut_max_distance: f32,
    sky_max_samples: u32,
    rendering_method: u32,
    // superfargoth patch (own, N2: sky glow): zenith radiance (cd/m², linear) and
    // horizon/zenith ratio of the sky's own glow (`functions::airglow`).
    airglow: vec3<f32>,
    airglow_horizon: f32,
}

/// "Atmosphere space" uses local up for the zenith so the horizon-detail
/// parameterization concentrates texels at the viewer's horizon. Azimuth uses a
/// world-fixed reference so the terminator stays stable when tilting the camera.
struct AtmosphereTransforms {
    world_from_atmosphere: mat4x4<f32>,
    atmosphere_from_world: mat4x4<f32>,
}
