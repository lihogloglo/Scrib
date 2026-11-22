//! Rock class presets (rockgen.md RG-4 "Classes"): each a set of [`BoulderParams`] overrides over the live
//! `rockgen.boulder.*` values, with the geology it imitates. Two families: **eroded** (RG-1/RG-3's classes, smooth
//! weathered rock: river boulders, old granite) and **sharp** (fresh fracture faces and hard edges: talus,
//! volcanic blocks, columnar basalt, scoria, slabs). Numbers are `Guess` (a look loop with the owner tunes them);
//! the shapes follow the named analogue. A game's adapter maps its rock types onto these or its own presets
//! (RG-A1); nothing here names a game.

use sf_tunables::Source;

use crate::BoulderParams;

/// Which look family a class belongs to.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Family {
    /// Rounded, weathered (RG-1 defaults): the edges are smooth-max blends, no snap.
    Eroded,
    /// Fresh fractures: hard edges (feature snap), flat faces.
    Sharp,
}

/// One preset.
#[derive(Clone, Copy, Debug)]
pub struct RockClass {
    pub name: &'static str,
    pub family: Family,
    /// What it imitates.
    pub analogue: &'static str,
    pub source: Source,
    /// Albedo multiplier the viewer applies (scoria is darker than the texture); adapter data at RG-A1.
    pub tint: f32,
    overrides: fn(BoulderParams) -> BoulderParams,
}

impl RockClass {
    /// The class's parameters over `base` (the live tunables).
    pub fn params(&self, base: BoulderParams) -> BoulderParams {
        (self.overrides)(base)
    }
}

/// Every preset, eroded first (RG-3's order), then the sharp family (RG-4).
pub const CLASSES: &[RockClass] = &[
    RockClass {
        name: "block",
        family: Family::Eroded,
        analogue: "weathered joint block: three joint families, rounded edges (the RG-1 defaults)",
        source: Source::Guess,
        tint: 1.0,
        overrides: |b| b,
    },
    RockClass {
        name: "platy",
        family: Family::Eroded,
        analogue: "weathered flag: one joint family (bedding), thin, rounded outline",
        source: Source::Guess,
        tint: 1.0,
        overrides: |b| BoulderParams { aspect: [0.8, 0.25], joint_sets: 1, ..b },
    },
    RockClass {
        name: "cobble",
        family: Family::Eroded,
        analogue: "river cobble / corestone: no flat faces left, round, weathered",
        source: Source::Guess,
        tint: 1.0,
        overrides: |b| BoulderParams { joint_sets: 0, chips: 3, rounding: 0.15, weather_amp: 0.04, ..b },
    },
    RockClass {
        name: "chipped",
        family: Family::Eroded,
        analogue: "weathered block with many spalled corners",
        source: Source::Guess,
        tint: 1.0,
        overrides: |b| BoulderParams { chips: 24, chip_depth: 0.3, ..b },
    },
    RockClass {
        name: "talus",
        family: Family::Sharp,
        analogue: "fresh talus / rockfall block: joint-bounded, fresh fracture faces, crisp edges, a few spalls",
        source: Source::Guess,
        tint: 1.0,
        overrides: |b| BoulderParams {
            joint_sets: 3,
            joint_jitter_deg: 15.0,
            chips: 7,
            chip_depth: 0.2,
            chip_spread: 0.6,
            rounding: 0.0,
            weather_amp: 0.003,
            weather_wavelength: 0.5,
            weather_octaves: 2,
            ..b
        },
    },
    RockClass {
        name: "jagged",
        family: Family::Sharp,
        analogue: "fresh volcanic block (andesite / basalt breccia, aa clinker): many small fracture facets, irregular, rough",
        source: Source::Guess,
        tint: 0.8,
        overrides: |b| BoulderParams {
            aspect: [0.7, 0.5],
            joint_sets: 3,
            joint_jitter_deg: 35.0,
            chips: 40,
            chip_depth: 0.12,
            chip_spread: 1.2,
            rounding: 0.0,
            weather_amp: 0.008,
            weather_wavelength: 0.12,
            weather_octaves: 3,
            resolution: 48,
            ..b
        },
    },
    RockClass {
        name: "columnar",
        family: Family::Sharp,
        analogue: "columnar basalt (Giant's Causeway, Svartifoss): 5-6 sided prisms, open joints, tilted cross-joint tops",
        source: Source::Industry("Goehring & Morris 2009 (columnar jointing: mostly 5-6 sided, mean ~6); numbers Guess"),
        tint: 0.85,
        overrides: |b| BoulderParams {
            aspect: [0.85, 0.75],
            columns: 7,
            chips: 2,
            chip_depth: 0.15,
            rounding: 0.0,
            weather_amp: 0.003,
            weather_wavelength: 0.4,
            weather_octaves: 2,
            resolution: 64,
            ..b
        },
    },
    RockClass {
        name: "scoria",
        family: Family::Sharp,
        analogue: "scoria / vesicular lava clast: irregular, rough, pitted by vesicles, dark",
        source: Source::Guess,
        tint: 0.5,
        overrides: |b| BoulderParams {
            aspect: [0.8, 0.65],
            joint_sets: 0,
            chips: 10,
            chip_depth: 0.2,
            chip_spread: 0.8,
            rounding: 0.03,
            weather_amp: 0.035,
            weather_wavelength: 0.25,
            weather_octaves: 4,
            pit_density: 0.55,
            pit_size: 0.065,
            resolution: 64,
            ..b
        },
    },
    RockClass {
        name: "slab",
        family: Family::Sharp,
        analogue: "fresh flagstone / bedded slab: parallel bedding faces, thin, square-cut joints",
        source: Source::Guess,
        tint: 1.0,
        overrides: |b| BoulderParams {
            aspect: [0.7, 0.14],
            joint_sets: 3,
            joint_jitter_deg: 6.0,
            chips: 3,
            chip_depth: 0.12,
            chip_spread: 0.4,
            rounding: 0.0,
            weather_amp: 0.002,
            weather_wavelength: 0.5,
            weather_octaves: 2,
            ..b
        },
    },
];
