//! Big-world tunables (`bigworld.*`, spec `docs/specs/tunables.md`; big-world-rendering.md "Tiers"). The tile edge
//! moves every instance, so it applies on restart; the tier switch sizes and the crossfade band are read each frame
//! by the band maths (`crate::Bands`) and apply live.

use sf_tunables::{AnyDef, Def, Group, Lifetime, Source, Unit, def};

pub const TILE_M: Def<f32> = def!(float "bigworld.tile_m" = 256.0)
    .range(64.0, 1024.0)
    .unit(Unit::Meters)
    .help("Edge of a square world tile on the ground: one instance block, one aggregate; changing it moves every instance")
    .source(Source::Reference("sf_worldgen scatter.tile_m"))
    .lifetime(Lifetime::Restart)
    .group(Group::Foliage);

pub const MESH_SWITCH_PX: Def<f32> = def!(float "bigworld.mesh.switch_px" = 80.0)
    .range(20.0, 400.0)
    .unit(Unit::Pixels)
    .help("A tree is drawn as a full mesh while it is at least this many pixels tall on screen, then as an impostor")
    .source(Source::Industry("screen-size LOD, UE/Fortnite"))
    .lifetime(Lifetime::Live)
    .group(Group::Foliage);

pub const AGGREGATE_SWITCH_PX: Def<f32> = def!(float "bigworld.aggregate.switch_px" = 3.0)
    .range(1.0, 16.0)
    .unit(Unit::Pixels)
    .help("A tree keeps its own impostor while its crown is at least this many pixels wide, then only the tile aggregate draws it")
    .source(Source::Industry("Chlorophyll tile proxies past ~1 km, SmartImposter ~3 km"))
    .lifetime(Lifetime::Live)
    .group(Group::Foliage);

pub const FADE_BAND: Def<f32> = def!(float "bigworld.fade_band" = 0.1)
    .range(0.0, 0.5)
    .unit(Unit::Ratio)
    .help("Share of each switch distance over which two tiers crossfade (dithered) so there is never a hole; 0 is a hard switch")
    .source(Source::Guess)
    .lifetime(Lifetime::Live)
    .group(Group::Foliage);

pub const TUNABLES: &[&dyn AnyDef] = &[&TILE_M, &MESH_SWITCH_PX, &AGGREGATE_SWITCH_PX, &FADE_BAND];
