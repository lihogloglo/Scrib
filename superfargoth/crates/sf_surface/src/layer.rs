//! CPU twin of the wetness math in `sf_surface_bevy`'s `layer.wesl` (surface-weather.md
//! "Shader integration", slice W3), for tests and CPU-side readers (W8 character wetness). The
//! constants are the shader's (a `sf_surface_bevy` test reads them out of the WGSL); keep both
//! in step.
//!
//! - Porosity: Lagarde's heuristic `(1 − metallic)·saturate((roughness − 0.3)/0.5)` (S.
//!   Lagarde, "Water drop 3b – Physically based wet surfaces", 2013): metals and polished
//!   surfaces don't drink; rough dielectrics (stone, earth, plaster, wood) do.
//! - Wet material: albedo × lerp(1, 0.2, porosity) at full wetness, a little richer in colour
//!   (Clockwork `WET_SATURATION`), gloss `lerp(1, gloss, lerp(1, factor, 0.5·wet))` (Lagarde),
//!   then a water film past saturation (water roughness, F0 0.02) that porous surfaces mostly
//!   drink. The film's share also flattens the normal-map detail (surface gradient × (1 − film)).
//! - Facing: ground keeps all of it, walls `WALL_WET` along streaks, faces turned down none
//!   (Clockwork `surfaceWater`, files/shaders/lib/light/surface.glsl, gitlab.com/OursCodeur/openmw
//!   branch clockwork-alpha, commit c5e5a9a6362c35512823884753d2d5284af204fa, GPL-3.0).

pub const POROSITY_ROUGH_LOW: f32 = 0.3;
pub const POROSITY_ROUGH_SPAN: f32 = 0.5;
pub const WET_DARK_POROUS: f32 = 0.2;
pub const WET_SATURATION: f32 = 0.3;
pub const WALL_WET: f32 = 0.6;
pub const FILM_START: f32 = 0.35;
pub const FILM_POROUS_HOLD: f32 = 0.6;
pub const FILM_ROUGHNESS: f32 = 0.25;
/// Bevy `reflectance` for F0 0.02 (F0 = 0.16·r²).
pub const WATER_REFLECTANCE: f32 = 0.354;

/// The named shader constants, as `layer.wesl` declares them (`const NAME: f32 = v;`).
pub const SHADER_CONSTANTS: [(&str, f32); 9] = [
    ("POROSITY_ROUGH_LOW", POROSITY_ROUGH_LOW),
    ("POROSITY_ROUGH_SPAN", POROSITY_ROUGH_SPAN),
    ("WET_DARK_POROUS", WET_DARK_POROUS),
    ("WET_SATURATION", WET_SATURATION),
    ("WALL_WET", WALL_WET),
    ("FILM_START", FILM_START),
    ("FILM_POROUS_HOLD", FILM_POROUS_HOLD),
    ("FILM_ROUGHNESS", FILM_ROUGHNESS),
    ("WATER_REFLECTANCE", WATER_REFLECTANCE),
];

const LUMA: [f32; 3] = [0.2126, 0.7152, 0.0722];

fn mix(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t
}

fn smoothstep(e0: f32, e1: f32, x: f32) -> f32 {
    let t = ((x - e0) / (e1 - e0)).clamp(0.0, 1.0);
    t * t * (3.0 - 2.0 * t)
}

/// Lagarde's porosity heuristic (`surface_porosity`).
pub fn porosity(metallic: f32, perceptual_roughness: f32) -> f32 {
    (1.0 - metallic) * ((perceptual_roughness - POROSITY_ROUGH_LOW) / POROSITY_ROUGH_SPAN).clamp(0.0, 1.0)
}

/// Share of the wetness a face keeps by its facing (`surface_facing_wet`), `up` the geometric
/// normal's Y, `streak` the remapped streak noise (0..1).
pub fn facing_wet(up: f32, streak: f32) -> f32 {
    if up < -0.3 {
        return 0.0;
    }
    let wall = WALL_WET * (0.3 + 0.7 * streak);
    mix(wall, 1.0, smoothstep(0.3, 0.8, up)) * smoothstep(-0.3, 0.0, up)
}

/// The water film past saturation (`surface_film`).
pub fn film(wet: f32, porosity: f32) -> f32 {
    wet * smoothstep(FILM_START, 1.0, wet) * (1.0 - FILM_POROUS_HOLD * porosity)
}

/// A wet material (`surface_wet_material`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WetMaterial {
    pub albedo: [f32; 3],
    pub perceptual_roughness: f32,
    pub reflectance: f32,
    /// Film share: the normal-map gradient is scaled by `1 − film`.
    pub film: f32,
}

/// Lagarde's wet surface on a material at wetness `wet` (0..1).
pub fn wet_material(
    albedo: [f32; 3],
    perceptual_roughness: f32,
    metallic: f32,
    reflectance: f32,
    wet: f32,
) -> WetMaterial {
    let p = porosity(metallic, perceptual_roughness);
    let factor = mix(1.0, WET_DARK_POROUS, p);
    let luma: f32 = albedo.iter().zip(LUMA).map(|(c, w)| c * w).sum();
    let sat = 1.0 + WET_SATURATION * wet * p;
    let dark = mix(1.0, factor, wet);
    let albedo = albedo.map(|c| mix(luma, c, sat).max(0.0) * dark);
    let gloss = 1.0 - perceptual_roughness;
    let mut rough = 1.0 - mix(1.0, gloss, mix(1.0, factor, 0.5 * wet));
    let film = film(wet, p);
    rough = mix(rough, rough.min(FILM_ROUGHNESS), film);
    WetMaterial {
        albedo,
        perceptual_roughness: rough,
        reflectance: mix(reflectance, WATER_REFLECTANCE, film * (1.0 - metallic)),
        film,
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::float_cmp)]
    use super::*;

    #[test]
    fn porosity_heuristic() {
        // Metals and polish don't drink; rough dielectrics do; linear between.
        assert_eq!(porosity(1.0, 0.9), 0.0);
        assert_eq!(porosity(0.0, 0.3), 0.0);
        assert_eq!(porosity(0.0, 0.1), 0.0);
        assert_eq!(porosity(0.0, 0.8), 1.0);
        assert_eq!(porosity(0.0, 1.0), 1.0);
        assert!((porosity(0.0, 0.55) - 0.5).abs() < 1e-6);
        assert!((porosity(0.5, 0.8) - 0.5).abs() < 1e-6);
    }

    #[test]
    fn dry_is_identity_and_soaked_follows_lagarde() {
        let stone = [0.4, 0.35, 0.3];
        let dry = wet_material(stone, 0.9, 0.0, 0.5, 0.0);
        assert_eq!(dry.perceptual_roughness, 0.9);
        assert_eq!(dry.reflectance, 0.5);
        assert_eq!(dry.film, 0.0);
        for (a, b) in dry.albedo.iter().zip(stone) {
            assert!((a - b).abs() < 1e-6);
        }
        // Soaked porous stone: albedo luminance down to ~0.2x, glossier, a partial film.
        let soaked = wet_material(stone, 0.9, 0.0, 0.5, 1.0);
        let luma = |c: [f32; 3]| c.iter().zip(LUMA).map(|(c, w)| c * w).sum::<f32>();
        assert!((luma(soaked.albedo) / luma(stone) - WET_DARK_POROUS).abs() < 1e-3);
        assert!(soaked.perceptual_roughness < 0.9);
        assert!((soaked.film - (1.0 - FILM_POROUS_HOLD)).abs() < 1e-6);
        // Polished metal: no darkening, the film alone (whole) sets water roughness.
        let metal = wet_material([0.9, 0.9, 0.9], 0.6, 1.0, 0.5, 1.0);
        assert_eq!(metal.albedo, [0.9, 0.9, 0.9]);
        assert_eq!(metal.film, 1.0);
        assert_eq!(metal.perceptual_roughness, FILM_ROUGHNESS);
        assert_eq!(metal.reflectance, 0.5);
        // Smooth dielectric (glazed): film to water F0.
        let glaze = wet_material([0.5; 3], 0.2, 0.0, 0.5, 1.0);
        assert_eq!(glaze.reflectance, WATER_REFLECTANCE);
        assert!((glaze.perceptual_roughness - 0.2).abs() < 1e-6);
    }

    #[test]
    fn wetter_is_never_lighter_or_rougher() {
        let mut last = wet_material([0.5; 3], 0.85, 0.0, 0.5, 0.0);
        for i in 1..=20 {
            #[allow(clippy::cast_precision_loss)]
            let m = wet_material([0.5; 3], 0.85, 0.0, 0.5, i as f32 / 20.0);
            assert!(m.albedo[0] <= last.albedo[0] + 1e-6);
            assert!(m.perceptual_roughness <= last.perceptual_roughness + 1e-6);
            assert!(m.film >= last.film - 1e-6);
            last = m;
        }
    }

    #[test]
    fn facing_keeps_ground_streaks_walls_dries_ceilings() {
        assert_eq!(facing_wet(1.0, 0.0), 1.0);
        assert!((facing_wet(0.0, 1.0) - WALL_WET).abs() < 1e-6);
        assert!((facing_wet(0.0, 0.0) - WALL_WET * 0.3).abs() < 1e-6);
        assert_eq!(facing_wet(-0.5, 1.0), 0.0);
        assert_eq!(facing_wet(-0.3, 1.0), 0.0);
    }
}
