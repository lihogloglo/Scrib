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
//! - Puddles (W4): Lagarde's accumulated water ("Water drop 2b - Dynamic rain and its effects",
//!   2013): a water level against a height map with a soft shoreline, and the surface under it
//!   turns to water (mirror gloss, F0 0.02, flat normal). The level comes from the puddle state
//!   and a pool site score (noise, plus terrain concavity), in the form of Clockwork's
//!   `surfaceWater` height branch (`1 + (pool - 1 + reach) / PUDDLE_DEPTH` against the height);
//!   only level faces hold water (≤ 2° full, none from 4°, on the true surface: a mesh's face
//!   normal, the terrain's height field; Clockwork's 0.94..0.99 ramp let ~15° slopes flood).
//! - Rain rings (W4): the ring profile of `files/shaders/lib/water/rain_ripples.glsl` (`blip`,
//!   `blipDerivative`, `circle`: a ring running out from a drop, trailing rings, energy fading
//!   over the cycle) as Clockwork uses it on puddles (same commit), on drop grids that turn on
//!   quarter by quarter of rain intensity (Lagarde's ripple layers).

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

/// Puddles: the puddle state's reach into the pool score at 1, the pool span the height map
/// covers, the shoreline (height units), the level-face ramp (`N.y` of the true surface: none
/// from cos 4°, full from cos 2°; water runs off anything steeper), terrain concavity at full
/// (m) and its weight in the pool score, the water's perceptual roughness (Bevy clamps it to
/// 0.089), the pool noise tile (m; divides the 256 m origin grid).
pub const PUDDLE_REACH: f32 = 0.3;
pub const PUDDLE_DEPTH: f32 = 0.1;
pub const PUDDLE_EDGE: f32 = 0.1;
pub const PUDDLE_FLAT_LOW: f32 = 0.997_564;
pub const PUDDLE_FLAT_HIGH: f32 = 0.999_391;
pub const BASIN_FULL_M: f32 = 0.1;
pub const BASIN_POOL: f32 = 0.35;
pub const PUDDLE_ROUGHNESS: f32 = 0.06;
pub const POOL_TILE_M: f32 = 64.0;
/// Object height from albedo luminance: dark grout and cracks fill first (Clockwork).
pub const HEIGHT_FROM_LUMA: f32 = 2.0;
/// Rain rings: drop cell (m, divides 256), ring radius (cell share), ring breadth (cycle share),
/// cycles a second, the water's tilt per unit of ring slope, the pixel footprint (m) where rings
/// start to fade and are gone (no shimmer under DLSS), the animation clock's wrap (s; the rate
/// times it is whole, so the wrap doesn't pop).
pub const RIPPLE_CELL_M: f32 = 1.6;
pub const RIPPLE_RADIUS: f32 = 0.2;
pub const RIPPLE_RING: f32 = 0.166_666_67;
pub const RIPPLE_RATE: f32 = 1.2;
pub const RIPPLE_TILT: f32 = 0.5;
pub const RIPPLE_FADE_NEAR_M: f32 = 0.02;
pub const RIPPLE_FADE_FAR_M: f32 = 0.06;
pub const RIPPLE_TIME_WRAP_S: f32 = 1000.0;
/// Drop grids; grid `k` turns on over rain `k/4 .. (k+1)/4`.
pub const RIPPLE_GRIDS: usize = 4;
/// Cell scale of each drop grid: each grid's cell (`RIPPLE_CELL_M` x scale) divides 256 m.
pub const RIPPLE_SCALES: [f32; RIPPLE_GRIDS] = [1.0, 1.25, 0.8, 0.625];

/// The named shader constants, as `layer.wesl` declares them (`const NAME: f32 = v;`).
pub const SHADER_CONSTANTS: [(&str, f32); 31] = [
    ("POROSITY_ROUGH_LOW", POROSITY_ROUGH_LOW),
    ("POROSITY_ROUGH_SPAN", POROSITY_ROUGH_SPAN),
    ("WET_DARK_POROUS", WET_DARK_POROUS),
    ("WET_SATURATION", WET_SATURATION),
    ("WALL_WET", WALL_WET),
    ("FILM_START", FILM_START),
    ("FILM_POROUS_HOLD", FILM_POROUS_HOLD),
    ("FILM_ROUGHNESS", FILM_ROUGHNESS),
    ("WATER_REFLECTANCE", WATER_REFLECTANCE),
    ("PUDDLE_REACH", PUDDLE_REACH),
    ("PUDDLE_DEPTH", PUDDLE_DEPTH),
    ("PUDDLE_EDGE", PUDDLE_EDGE),
    ("PUDDLE_FLAT_LOW", PUDDLE_FLAT_LOW),
    ("PUDDLE_FLAT_HIGH", PUDDLE_FLAT_HIGH),
    ("BASIN_FULL_M", BASIN_FULL_M),
    ("BASIN_POOL", BASIN_POOL),
    ("PUDDLE_ROUGHNESS", PUDDLE_ROUGHNESS),
    ("POOL_TILE_M", POOL_TILE_M),
    ("HEIGHT_FROM_LUMA", HEIGHT_FROM_LUMA),
    ("RIPPLE_CELL_M", RIPPLE_CELL_M),
    ("RIPPLE_RADIUS", RIPPLE_RADIUS),
    ("RIPPLE_RING", RIPPLE_RING),
    ("RIPPLE_RATE", RIPPLE_RATE),
    ("RIPPLE_TILT", RIPPLE_TILT),
    ("RIPPLE_FADE_NEAR_M", RIPPLE_FADE_NEAR_M),
    ("RIPPLE_FADE_FAR_M", RIPPLE_FADE_FAR_M),
    ("RIPPLE_TIME_WRAP_S", RIPPLE_TIME_WRAP_S),
    ("RIPPLE_SCALE_0", RIPPLE_SCALES[0]),
    ("RIPPLE_SCALE_1", RIPPLE_SCALES[1]),
    ("RIPPLE_SCALE_2", RIPPLE_SCALES[2]),
    ("RIPPLE_SCALE_3", RIPPLE_SCALES[3]),
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

/// How level a face is for standing water (`surface_puddle_flat`), `up` the true surface
/// normal's Y (`SurfaceInput.level_up`: a mesh's triangle face normal, the terrain's height
/// field), never a smoothed vertex normal.
pub fn puddle_flat(up: f32) -> f32 {
    smoothstep(PUDDLE_FLAT_LOW, PUDDLE_FLAT_HIGH, up)
}

/// The normal's Y of a height field with gradient `g` (rise per meter along X and Z;
/// `surface_up_of_gradient`): `1 / sqrt(1 + |g|²)`.
pub fn up_of_gradient(g: [f32; 2]) -> f32 {
    1.0 / (1.0 + g[0] * g[0] + g[1] * g[1]).sqrt()
}

/// Puddle inputs of a height field from heights `r` meters out along +X, −X, +Z, −Z and at the
/// centre (`terrain_ground` in `sf_terrain_bevy`): the central-difference gradient over `2r`
/// and the concavity (mean around minus centre, ≥ 0, meters). Returns (gradient, basin).
pub fn ground(around: [f32; 4], centre: f32, r: f32) -> ([f32; 2], f32) {
    let [px, nx, pz, nz] = around;
    let g = [(px - nx) / (2.0 * r), (pz - nz) / (2.0 * r)];
    (g, (0.25 * (px + nx + pz + nz) - centre).max(0.0))
}

/// The pool site score (`surface_pool_of`): `noise` the pool noise (0..1), `basin` the terrain's
/// concavity in meters (0 off terrain).
pub fn pool(noise: f32, basin: f32) -> f32 {
    (noise + BASIN_POOL * (basin / BASIN_FULL_M).clamp(0.0, 1.0)).clamp(0.0, 1.0)
}

/// An object's height signal from its albedo luminance (`surface_height_of`).
pub fn height_of(luma: f32) -> f32 {
    (HEIGHT_FROM_LUMA * luma).clamp(0.0, 1.0)
}

/// Water depth over the height map, in height units (`surface_puddle`): the water level
/// `(pool - 1 + reach) / PUDDLE_DEPTH` minus the height; `p` the puddle state (0..1, sheltered).
pub fn puddle_depth(p: f32, pool: f32, height: f32) -> f32 {
    (pool - 1.0 + PUDDLE_REACH * p) / PUDDLE_DEPTH - height
}

/// Standing water (0..1) on a face: level x the soft shoreline over the depth.
pub fn puddle(p: f32, pool: f32, height: f32, up: f32) -> f32 {
    puddle_flat(up) * smoothstep(0.0, PUDDLE_EDGE, puddle_depth(p, pool, height))
}

/// The surface under standing water (`surface_puddle_material`): water's roughness and F0 on a
/// dielectric top. Returns (perceptual roughness, reflectance, metallic).
pub fn puddle_material(perceptual_roughness: f32, reflectance: f32, metallic: f32, puddle: f32) -> (f32, f32, f32) {
    (
        mix(perceptual_roughness, PUDDLE_ROUGHNESS, puddle),
        mix(reflectance, WATER_REFLECTANCE, puddle),
        mix(metallic, 0.0, puddle),
    )
}

/// Drop grid weights for rain (0..1) at the point: grid `k` fades in over `k/4 .. (k+1)/4`
/// (Lagarde's ripple layers), so drizzle rings sparsely and a storm densely.
pub fn ripple_weights(rain: f32) -> [f32; RIPPLE_GRIDS] {
    #[allow(clippy::cast_precision_loss)]
    let n = RIPPLE_GRIDS as f32;
    #[allow(clippy::cast_precision_loss)]
    std::array::from_fn(|k| ((rain - k as f32 / n) * n).clamp(0.0, 1.0))
}

/// Rings fade as a pixel covers more than a ring's breadth (`footprint` m per pixel).
pub fn ripple_fade(footprint: f32) -> f32 {
    1.0 - smoothstep(RIPPLE_FADE_NEAR_M, RIPPLE_FADE_FAR_M, footprint)
}

/// The ring's height bump (`rain_ripples.glsl` `blip`).
pub fn blip(x: f32) -> f32 {
    let x = (1.0 - x * x).max(0.0);
    x * x * x
}

/// Its derivative (`blipDerivative`).
pub fn blip_derivative(x: f32) -> f32 {
    let x = x.clamp(-1.0, 1.0);
    let n = x * x - 1.0;
    -6.0 * x * n * n
}

/// One drop's ring slope (`surface_ring`): `to` the point minus the drop (cell units), `phase`
/// the drop's cycle (0..1). Zero outside the ring's band `R (phase - 2 RING) .. R phase`.
pub fn ring(to: [f32; 2], phase: f32) -> [f32; 2] {
    let d = (to[0] * to[0] + to[1] * to[1]).sqrt();
    let follower = (phase - d / RIPPLE_RADIUS) / RIPPLE_RING - 1.0;
    if !(-1.0..=1.0).contains(&follower) {
        return [0.0, 0.0];
    }
    let energy = 1.0 - phase;
    let range = blip(follower.min(0.0));
    let k = -blip_derivative(follower) * 5.0 * energy * energy;
    let v = [to[0] * k, to[1] * k, 0.5];
    let s = energy * range / (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt();
    [v[0] * s, v[1] * s]
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

    #[test]
    fn puddles_fill_from_the_lows_and_dry_back() {
        // No puddle state, no water, whatever the site.
        assert_eq!(puddle(0.0, 1.0, 0.0, 1.0), 0.0);
        // Full state: the reach submerges pools past 1 - reach (+ depth over the height map).
        let line = 1.0 - PUDDLE_REACH;
        assert_eq!(puddle(1.0, line, 0.0, 1.0), 0.0);
        assert!(puddle_depth(1.0, line + 0.05, 0.5).abs() < 1e-5);
        assert_eq!(puddle(1.0, line + 0.07, 0.5, 1.0), 1.0);
        // Shoreline: half way through the edge, smoothstep's half.
        let half = line + PUDDLE_DEPTH * (0.5 + 0.5 * PUDDLE_EDGE);
        assert!((puddle(1.0, half, 0.5, 1.0) - 0.5).abs() < 1e-3);
        // Lows (cracks, dark grout) fill first; a smaller state reaches fewer sites.
        assert!(puddle(1.0, line + 0.04, 0.2, 1.0) > puddle(1.0, line + 0.04, 0.6, 1.0));
        assert!(puddle(0.5, line + 0.07, 0.5, 1.0) < puddle(1.0, line + 0.07, 0.5, 1.0));
        // Only level faces: none on a 30 degree slope or a wall (more in `no_puddles_on_slopes`).
        assert_eq!(puddle(1.0, 1.0, 0.0, 0.85), 0.0);
        assert_eq!(puddle(1.0, 1.0, 0.0, 0.0), 0.0);
        assert_eq!(puddle_flat(PUDDLE_FLAT_HIGH), 1.0);
        // Terrain concavity raises the pool score, capped at BASIN_FULL_M; bumps (negative) don't.
        assert_eq!(pool(0.5, 0.0), 0.5);
        assert!((pool(0.5, BASIN_FULL_M) - (0.5 + BASIN_POOL)).abs() < 1e-6);
        assert_eq!(pool(0.5, 1.0), pool(0.5, BASIN_FULL_M));
        assert_eq!(pool(0.5, -1.0), 0.5);
        assert_eq!(pool(0.9, 1.0), 1.0);
        assert_eq!(height_of(0.25), 0.5);
        assert_eq!(height_of(0.9), 1.0);
    }

    #[test]
    fn no_puddles_on_slopes() {
        // Owner SW-4 (2026-11-05): water lies on level ground only. A full puddle state, the
        // highest pool score and the lowest height: everything but the slope says "water".
        let r = 2.0_f32;
        for deg in [5.0_f32, 10.0, 20.0] {
            let rise = deg.to_radians().tan();
            // Terrain: a plane rising along X, Z and the diagonal, sampled 2 m out as the shader
            // does; a hollow (+0.1 m all round) on the slope is still a slope.
            for dir in [[1.0_f32, 0.0], [0.0, 1.0], [std::f32::consts::FRAC_1_SQRT_2; 2]] {
                for hollow in [0.0, 0.1] {
                    let h = |x: f32, z: f32| rise * (dir[0] * x + dir[1] * z) + hollow;
                    let (g, basin) = ground([h(r, 0.0), h(-r, 0.0), h(0.0, r), h(0.0, -r)], 0.0, r);
                    let up = up_of_gradient(g);
                    assert!((up - deg.to_radians().cos()).abs() < 1e-5, "{deg}: up {up}");
                    assert_eq!(puddle(1.0, pool(1.0, basin), 0.0, up), 0.0, "terrain at {deg} degrees");
                }
            }
            // Objects: the triangle's face normal.
            assert_eq!(puddle(1.0, 1.0, 0.0, deg.to_radians().cos()), 0.0, "face at {deg} degrees");
        }
        // Level ground with a hollow: the basin lifts the pool score into a full puddle where
        // the same ground without one stays dry.
        let (g, basin) = ground([0.1; 4], 0.0, r);
        assert!(g == [0.0, 0.0] && (basin - 0.1).abs() < 1e-6);
        assert_eq!(up_of_gradient(g), 1.0);
        assert_eq!(puddle(1.0, pool(0.5, basin), 0.2, up_of_gradient(g)), 1.0);
        assert_eq!(puddle(1.0, pool(0.5, 0.0), 0.2, 1.0), 0.0);
        // A bump is no basin.
        assert_eq!(ground([-0.1; 4], 0.0, r).1, 0.0);
        // Up to 1.5 degrees counts as level; the ramp ends at 4 (cos 4°, rounded).
        assert_eq!(puddle_flat(up_of_gradient([1.5_f32.to_radians().tan(), 0.0])), 1.0);
        assert_eq!(puddle_flat(4.1_f32.to_radians().cos()), 0.0);
    }

    #[test]
    fn standing_water_is_a_mirror_with_water_f0() {
        assert_eq!(puddle_material(0.9, 0.5, 1.0, 0.0), (0.9, 0.5, 1.0));
        let (rough, reflectance, metallic) = puddle_material(0.9, 0.5, 1.0, 1.0);
        assert!((rough - PUDDLE_ROUGHNESS).abs() < 1e-6);
        assert!((reflectance - WATER_REFLECTANCE).abs() < 1e-6);
        assert_eq!(metallic, 0.0);
        // F0 = 0.16 r^2 = 0.02.
        assert!((0.16 * WATER_REFLECTANCE * WATER_REFLECTANCE - 0.02).abs() < 1e-3);
    }

    #[test]
    fn rings_follow_rain_and_stay_in_their_band() {
        assert_eq!(ripple_weights(0.0), [0.0; RIPPLE_GRIDS]);
        assert_eq!(ripple_weights(1.0), [1.0; RIPPLE_GRIDS]);
        assert_eq!(ripple_weights(0.125), [0.5, 0.0, 0.0, 0.0]);
        assert_eq!(ripple_weights(0.5), [1.0, 1.0, 0.0, 0.0]);
        assert_eq!(ripple_fade(0.0), 1.0);
        assert_eq!(ripple_fade(RIPPLE_FADE_FAR_M), 0.0);
        // Every grid's cell and the pool tile divide the 256 m origin grid (no jump on an origin
        // shift), and the clock wraps on whole cycles.
        for s in RIPPLE_SCALES {
            let n = 256.0 / (RIPPLE_CELL_M * s);
            assert!((n - n.round()).abs() < 1e-3, "{s}");
        }
        assert_eq!((256.0 / POOL_TILE_M).fract(), 0.0);
        let turns = RIPPLE_RATE * RIPPLE_TIME_WRAP_S;
        assert!((turns - turns.round()).abs() < 1e-3);
        // Ring profile: zero at the drop before the ring passes and outside the band.
        assert_eq!(ring([0.0, 0.0], 0.0), [0.0, 0.0]);
        assert_eq!(ring([0.15, 0.0], 0.1), [0.0, 0.0]);
        assert_eq!(blip(0.0), 1.0);
        assert_eq!(blip(1.0), 0.0);
        assert_eq!(blip_derivative(0.0), 0.0);
        // The ring runs out with the phase: nothing behind its trailing edge
        // d = R (phase - 2 RING), a slope just past it.
        let r = |d: f32, phase: f32| ring([d, 0.0], phase)[0];
        let trailing = |phase: f32| RIPPLE_RADIUS * (phase - 2.0 * RIPPLE_RING);
        assert_eq!(r(trailing(0.5) - 0.002, 0.5), 0.0);
        assert!(r(trailing(0.5) + 0.01, 0.5).abs() > 0.0);
        // Symmetric about the drop, finite, at most a unit vector's share.
        for i in 0..=40 {
            #[allow(clippy::cast_precision_loss)]
            let d = i as f32 * 0.005;
            for phase in [0.4, 0.7, 0.95] {
                let a = ring([d, 0.0], phase);
                let b = ring([-d, 0.0], phase);
                assert!(a[0].is_finite() && a[0].abs() <= 1.0 && a[1] == 0.0);
                assert!((a[0] + b[0]).abs() < 1e-6);
            }
        }
        // Old drops ring weaker (energy fades over the cycle).
        let peak = |phase: f32| {
            (0..=200)
                .map(|i| {
                    #[allow(clippy::cast_precision_loss)]
                    let d = i as f32 * 0.001;
                    r(d, phase).abs()
                })
                .fold(0.0, f32::max)
        };
        assert!(peak(0.9) < peak(0.5));
    }
}
