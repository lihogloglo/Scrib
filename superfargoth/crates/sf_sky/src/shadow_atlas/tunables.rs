//! Tunables (`shadow.lamp.atlas.*` and `shadow.lamp.contact_max`) and [`ShadowAtlasConfig`].

use sf_tunables::{AnyDef, Def, Group, RegisterError, Source, Tunables, Unit, def};

/// Valid power-of-two bounds for the static shadow atlas texture.
const STATIC_MIN_PX: u16 = 1024; // tunables: const (layout: valid power-of-two static atlas minimum)
const STATIC_MAX_PX: u16 = 8192; // tunables: const (layout: valid power-of-two static atlas maximum)

/// Valid power-of-two bounds for the live shadow atlas texture.
const LIVE_MIN_PX: u16 = 512; // tunables: const (layout: valid power-of-two live atlas minimum)
const LIVE_MAX_PX: u16 = 4096; // tunables: const (layout: valid power-of-two live atlas maximum)

/// `shadow.lamp.atlas.enabled`: unified cached 2D point shadow atlas (CW-13b).
pub const ENABLED: Def<bool> = def!(bool "shadow.lamp.atlas.enabled" = true)
    .help("Pack lamp shadows into 2D static and live atlases instead of fixed cube-map arrays")
    .source(Source::Owner(
        "2026-11-20: on by default after SH-33 visual check + shadow_a3_atlas interleaved A/B win across all 4 arms",
    ))
    .group(Group::Shadows);

/// `shadow.lamp.atlas.static_size_px`: square side length of the static shadow atlas.
pub const STATIC_SIZE_PX: Def<u32> = def!(count "shadow.lamp.atlas.static_size_px" = 4096)
    .range(1024, 8192)
    .unit(Unit::Pixels)
    .help("Side length of the square 2D shadow atlas holding cached static lamp faces")
    .group(Group::Shadows);

/// `shadow.lamp.atlas.live_size_px`: square side length of the live shadow atlas.
pub const LIVE_SIZE_PX: Def<u32> = def!(count "shadow.lamp.atlas.live_size_px" = 2048)
    .range(512, 4096)
    .unit(Unit::Pixels)
    .help("Side length of the square 2D shadow atlas holding mover and moving-lamp faces")
    .group(Group::Shadows);

/// `shadow.lamp.atlas.guard_texels`: border guard band on every tile edge to prevent PCF bleed.
pub const GUARD_TEXELS: Def<f32> = def!(float "shadow.lamp.atlas.guard_texels" = 2.0)
    .range(0.0, 8.0)
    .unit(Unit::Pixels)
    .help("Guard-band texels padded on each edge of an atlas tile so PCF does not bleed across tiles")
    .group(Group::Shadows);

/// `shadow.lamp.atlas.tier_512_m`: distance threshold for the 512 px shadow tile tier.
pub const TIER_512_M: Def<f32> = def!(float "shadow.lamp.atlas.tier_512_m" = 10.0)
    .range(1.0, 100.0)
    .unit(Unit::Meters)
    .help("Effective viewer distance within which a lamp requests 512 px shadow atlas tiles")
    .group(Group::Shadows);

/// `shadow.lamp.atlas.tier_256_m`: distance threshold for the 256 px shadow tile tier.
pub const TIER_256_M: Def<f32> = def!(float "shadow.lamp.atlas.tier_256_m" = 24.0)
    .range(2.0, 200.0)
    .unit(Unit::Meters)
    .help("Effective viewer distance within which a lamp requests 256 px shadow atlas tiles (128 px beyond)")
    .group(Group::Shadows);

/// `shadow.lamp.atlas.tier_hysteresis`: downgrade multiplier preventing tier oscillation.
pub const TIER_HYSTERESIS: Def<f32> = def!(float "shadow.lamp.atlas.tier_hysteresis" = 1.5)
    .range(1.0, 3.0)
    .unit(Unit::Ratio)
    .help("Distance multiplier applied before downgrading a lamp's shadow atlas tile resolution tier")
    .group(Group::Shadows);

/// `shadow.lamp.atlas.resize_cooldown_s`: minimum seconds between tier resizes for a lamp.
pub const RESIZE_COOLDOWN_S: Def<f32> = def!(float "shadow.lamp.atlas.resize_cooldown_s" = 2.0)
    .range(0.0, 30.0)
    .unit(Unit::Seconds)
    .help("Minimum seconds a lamp keeps its current shadow atlas tile tier before resizing again")
    .group(Group::Shadows);

/// `shadow.lamp.atlas.max_lamps`: maximum simultaneous shadow-atlas lamps.
pub const MAX_LAMPS: Def<u32> = def!(count "shadow.lamp.atlas.max_lamps" = 48)
    .range(1, 128)
    .unit(Unit::Count)
    .help("Maximum simultaneous lamps tracked by the 2D shadow atlas")
    .group(Group::Shadows);

/// `shadow.lamp.atlas.handover_spares`: spare 6-face 256 px lamp budgets reserved for crossfades.
pub const HANDOVER_SPARES: Def<u32> = def!(count "shadow.lamp.atlas.handover_spares" = 2)
    .range(0, 8)
    .unit(Unit::Count)
    .help("Spare 256 px lamp budgets reserved in the static atlas for simultaneous handover crossfades")
    .group(Group::Shadows);

/// `shadow.lamp.atlas.soft_pcf_radius`: 4-tap rotated PCF filter radius in tile texels (`0` = 1-tap 2x2 HW PCF).
pub const SOFT_PCF_RADIUS: Def<f32> = def!(float "shadow.lamp.atlas.soft_pcf_radius" = 1.0)
    .range(0.0, 1.75)
    .unit(Unit::Texels)
    .help("Rotated 4-tap PCF filter radius in tile texels for 2D shadow atlas tiles (0 = crisp 1-tap 2x2 hardware PCF)")
    .source(Source::Industry("UE/Frostbite/Doom 2016: 4-tap IGN-rotated PCF inside tile guard band, resolved by TAA/DLSS"))
    .group(Group::Shadows);

/// `shadow.lamp.contact_max`: maximum nearby lamps contributing screen-space contact shadows.
pub const CONTACT_MAX: Def<u32> = def!(count "shadow.lamp.contact_max" = 8)
    .range(0, 32)
    .unit(Unit::Count)
    .help("Maximum local lamps casting screen-space contact shadows at once")
    .group(Group::Shadows);

/// This module's tunables.
pub const TUNABLES: &[&dyn AnyDef] = &[
    &ENABLED,
    &STATIC_SIZE_PX,
    &LIVE_SIZE_PX,
    &GUARD_TEXELS,
    &TIER_512_M,
    &TIER_256_M,
    &TIER_HYSTERESIS,
    &RESIZE_COOLDOWN_S,
    &MAX_LAMPS,
    &HANDOVER_SPARES,
    &SOFT_PCF_RADIUS,
    &CONTACT_MAX,
];

/// Shadow atlas policy configuration (one field per tunable except [`ENABLED`]).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ShadowAtlasConfig {
    /// Requested side length of the static shadow atlas in pixels.
    pub static_size_px: u32,
    /// Requested side length of the live shadow atlas in pixels.
    pub live_size_px: u32,
    /// Guard-band texels padded on every edge of an atlas tile.
    pub guard_texels: f32,
    /// Effective distance threshold (metres) for the 512 px tile tier.
    pub tier_512_m: f32,
    /// Effective distance threshold (metres) for the 256 px tile tier.
    pub tier_256_m: f32,
    /// Hysteresis multiplier on tier downgrades (`>= 1.0`).
    pub tier_hysteresis: f32,
    /// Minimum seconds between tile-tier resizes for a lamp.
    pub resize_cooldown_s: f32,
    /// Maximum simultaneous lamps tracked by the shadow atlas.
    pub max_lamps: u32,
    /// Number of 6-face 256 px handover spare budgets reserved in the static atlas.
    pub handover_spares: u32,
    /// Rotated 4-tap PCF filter radius in tile texels (`0.0` = 1-tap 2x2 HW PCF).
    pub soft_pcf_radius: f32,
    /// Maximum local lamps casting screen-space contact shadows at once.
    pub contact_max: u32,
}

impl Default for ShadowAtlasConfig {
    /// The defs' defaults.
    fn default() -> Self {
        Self {
            static_size_px: STATIC_SIZE_PX.default_value(),
            live_size_px: LIVE_SIZE_PX.default_value(),
            guard_texels: GUARD_TEXELS.default_value(),
            tier_512_m: TIER_512_M.default_value(),
            tier_256_m: TIER_256_M.default_value(),
            tier_hysteresis: TIER_HYSTERESIS.default_value(),
            resize_cooldown_s: RESIZE_COOLDOWN_S.default_value(),
            max_lamps: MAX_LAMPS.default_value(),
            handover_spares: HANDOVER_SPARES.default_value(),
            soft_pcf_radius: SOFT_PCF_RADIUS.default_value(),
            contact_max: CONTACT_MAX.default_value(),
        }
    }
}

impl ShadowAtlasConfig {
    /// The effective values from a registry holding [`TUNABLES`].
    pub fn from_tunables(t: &Tunables) -> Result<Self, RegisterError> {
        Ok(Self {
            static_size_px: t.read(t.id_of(&STATIC_SIZE_PX)?),
            live_size_px: t.read(t.id_of(&LIVE_SIZE_PX)?),
            guard_texels: t.read(t.id_of(&GUARD_TEXELS)?),
            tier_512_m: t.read(t.id_of(&TIER_512_M)?),
            tier_256_m: t.read(t.id_of(&TIER_256_M)?),
            tier_hysteresis: t.read(t.id_of(&TIER_HYSTERESIS)?),
            resize_cooldown_s: t.read(t.id_of(&RESIZE_COOLDOWN_S)?),
            max_lamps: t.read(t.id_of(&MAX_LAMPS)?),
            handover_spares: t.read(t.id_of(&HANDOVER_SPARES)?),
            soft_pcf_radius: t.read(t.id_of(&SOFT_PCF_RADIUS)?),
            contact_max: t.read(t.id_of(&CONTACT_MAX)?),
        })
    }

    /// Static atlas size snapped to the nearest valid power of two in `[1024, 8192]`.
    pub fn clamped_static_size(&self) -> u16 {
        snap_pow2(self.static_size_px, STATIC_MIN_PX, STATIC_MAX_PX)
    }

    /// Live atlas size snapped to the nearest valid power of two in `[512, 4096]`.
    pub fn clamped_live_size(&self) -> u16 {
        snap_pow2(self.live_size_px, LIVE_MIN_PX, LIVE_MAX_PX)
    }
}

fn snap_pow2(value: u32, min_pow2: u16, max_pow2: u16) -> u16 {
    let min_u32 = u32::from(min_pow2);
    let max_u32 = u32::from(max_pow2);
    if value <= min_u32 {
        return min_pow2;
    }
    if value >= max_u32 {
        return max_pow2;
    }
    let hi = value.next_power_of_two();
    if hi == value {
        return u16::try_from(hi).unwrap_or(max_pow2);
    }
    let lo = hi >> 1;
    let chosen = if hi - value <= value - lo { hi } else { lo };
    u16::try_from(chosen).unwrap_or(max_pow2)
}
