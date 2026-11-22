//! Rock material and LOD tunables (`rockgen.material.*`, `rockgen.lod.*`; spec `docs/specs/tunables.md`,
//! rockgen.md "Tunables"). The material values are live (applied to every [`crate::RockMaterial`] when they
//! change); the LOD distances apply when a rock spawns. The hex-tiling defaults are the terrain's
//! (`terrain.hex_tiling.*`, Mikkelsen 2022), so rocks and ground break up their repeats the same way.

use sf_tunables::{AnyDef, Def, Lifetime, Source, Unit, def};

pub const BLEND_SHARPNESS: Def<f32> = def!(float "rockgen.material.blend_sharpness" = 6.0)
    .range(1.0, 16.0)
    .unit(Unit::Ratio)
    .help("Triplanar blend exponent on the normal (higher = narrower seams between the three projections)")
    .source(Source::Industry("terrain sample_layer_triplanar exponent 6; Golus 2017 triplanar blending"))
    .lifetime(Lifetime::Live);

pub const HEX_TILING: Def<bool> = def!(bool "rockgen.material.hex_tiling" = true)
    .help("Rock texture anti-tiling with Mikkelsen 2022 hex-tiling on each triplanar projection (off: plain repeats)")
    .source(Source::Industry("Mikkelsen 2022, Practical Real-Time Hex-Tiling, JCGT"))
    .lifetime(Lifetime::Live);

pub const HEX_ROTATION: Def<f32> = def!(float "rockgen.material.hex_rotation" = 1.0)
    .range(0.0, 1.0)
    .unit(Unit::Ratio)
    .help("Hex-tiling per-cell random rotation strength (0 = offset only, 1 = full +-pi rotation)")
    .source(Source::Industry("Mikkelsen 2022, Practical Real-Time Hex-Tiling, JCGT"))
    .lifetime(Lifetime::Live);

pub const HEX_CONTRAST: Def<f32> = def!(float "rockgen.material.hex_contrast" = 7.0)
    .range(1.0, 16.0)
    .unit(Unit::Ratio)
    .help("Hex-tiling weight contrast exponent (higher keeps sharper cell seams and prunes more secondary taps)")
    .source(Source::Industry("Mikkelsen 2022, Practical Real-Time Hex-Tiling, JCGT: g_exp = 7"))
    .lifetime(Lifetime::Live);

pub const HEX_SCALE: Def<f32> = def!(float "rockgen.material.hex_scale" = 1.0)
    .range(0.25, 4.0)
    .unit(Unit::Ratio)
    .help("Hex-tiling cells per texture repeat (1 = one hex edge per texture tile)")
    .source(Source::Guess)
    .lifetime(Lifetime::Live);

pub const LOD1_DISTANCE: Def<f32> = def!(float "rockgen.lod.lod1_distance" = 8.0)
    .range(1.0, 200.0)
    .unit(Unit::Ratio)
    .help("Camera distance, in rock sizes, where LOD1 replaces LOD0 (a 2 m rock switches at 16 m)")
    .source(Source::Guess)
    .lifetime(Lifetime::Respawn);

pub const LOD_GROWTH: Def<f32> = def!(float "rockgen.lod.growth" = 2.0)
    .range(1.1, 8.0)
    .unit(Unit::Ratio)
    .help("Each further LOD switch is this many times farther than the previous (halved triangles per level)")
    .source(Source::Industry("UE auto-LOD halves screen size per level for 50 % triangle steps"))
    .lifetime(Lifetime::Respawn);

pub const LOD_FADE: Def<f32> = def!(float "rockgen.lod.fade" = 0.1)
    .range(0.0, 0.5)
    .unit(Unit::Ratio)
    .help("Dithered crossfade half-width at each LOD switch, as a fraction of the switch distance (0 = hard pop)")
    .source(Source::Guess)
    .lifetime(Lifetime::Respawn);

/// Every rock-material and rock-LOD tunable (the game lists it in its catalog).
pub const TUNABLES: &[&dyn AnyDef] =
    &[&BLEND_SHARPNESS, &HEX_TILING, &HEX_ROTATION, &HEX_CONTRAST, &HEX_SCALE, &LOD1_DISTANCE, &LOD_GROWTH, &LOD_FADE];
