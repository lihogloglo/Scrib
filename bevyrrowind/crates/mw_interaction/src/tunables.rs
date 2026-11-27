//! The adapter's tunables (CONTRIBUTING "No hardcoded tunables", tunables.md): declared here
//! (Bevy-free), registered and bound by the game shell (`IN32b`/`IN32c`), read by the pure code through
//! [`FurnitureParams`] (defs) and [`MeasureParams`] (the bake's measuring, `IN32b1`), defaults straight
//! from the declarations, so no value is written twice.
//!
//! Switches: `mw.rules.interaction.sit` and `.snuff` so far; the other `mw.rules.interaction.*` switches
//! (interaction-adapter.md "Morrowind has none of this") land with their slices.

use sf_tunables::{AnyDef, Def, Lifetime, Net, Scope, Source, Unit, def};

/// A gameplay length (net `Gameplay`, scope `Dev`, source `Guess`, metres): defs are built when
/// cells load, so a change applies to defs built after it.
const fn length(d: Def<f32>) -> Def<f32> {
    d.source(Source::Guess).scope(Scope::Dev).net(Net::Gameplay).lifetime(Lifetime::Respawn).unit(Unit::Meters)
}

/// `mw.rules.interaction.sit`: census furniture (seats, beds, tables, counters) gets pose slots.
/// Off = no def, so the furniture is inert as in vanilla (the Classic preset turns it off).
pub const SIT: Def<bool> = def!(bool "mw.rules.interaction.sit" = true)
    .help("Census furniture (seats, beds, tables, counters) can be sat, lain or leant on; off = inert as in vanilla (Classic)")
    .source(Source::Owner("modern first, classic behind a seam (CONTRIBUTING \"Design constraints\")"))
    .scope(Scope::Dev)
    .net(Net::Gameplay)
    .lifetime(Lifetime::Respawn);

/// `mw.rules.interaction.snuff`: fire lights can be snuffed and relit (`crate::light`, `IN19c`). Off =
/// no light verbs, inert as in vanilla (the Classic preset turns it off); the lights stay gameplay
/// lights for the AI.
pub const SNUFF: Def<bool> = def!(bool "mw.rules.interaction.snuff" = true)
    .help("Candles, lanterns and other fire lights can be snuffed and relit; off = inert as in vanilla (Classic)")
    .source(Source::Owner("modern first, classic behind a seam (CONTRIBUTING \"Design constraints\")"))
    .scope(Scope::Dev)
    .net(Net::Gameplay)
    .lifetime(Lifetime::Respawn);

/// `mw.interact.bench_seat_m`: the width one sitter takes on a bench.
pub const BENCH_SEAT_M: Def<f32> = length(def!(float "mw.interact.bench_seat_m" = 0.55)
    .range(0.4, 1.0)
    .help("Width one sitter takes on a seat: a seat this many times wider gets that many places"));

/// `mw.interact.approach_m`: where a user stands to start an enter clip, out from the furniture's edge.
pub const APPROACH_M: Def<f32> = length(def!(float "mw.interact.approach_m" = 0.3)
    .range(0.1, 1.0)
    .help("Approach point of a furniture slot's entrance, out from the furniture's edge"));

/// `mw.interact.edge_seat_inset_m`: a bed-edge seat's root, in from the bed's front edge.
pub const EDGE_SEAT_INSET_M: Def<f32> = length(def!(float "mw.interact.edge_seat_inset_m" = 0.2)
    .range(0.0, 0.5)
    .help("Root of the seat on a bed's edge, in from the front edge"));

/// `mw.interact.counter_spacing_m`: room one user takes along a table or counter edge.
pub const COUNTER_SPACING_M: Def<f32> = length(def!(float "mw.interact.counter_spacing_m" = 0.6)
    .range(0.4, 1.5)
    .help("Room one standing user takes along a table or counter edge"));

/// `mw.interact.counter_stand_m`: a table or counter station's root, out from the edge.
pub const COUNTER_STAND_M: Def<f32> = length(def!(float "mw.interact.counter_stand_m" = 0.3)
    .range(0.1, 0.6)
    .help("Root of a standing or leaning user at a table or counter, out from its edge"));

/// `mw.interact.floor_seat_band_m`: a seat whose top is lower than this above the floor is sat on
/// from the floor (clip family `seat.floor`, cross-legged or kneeling), whatever its shape.
pub const FLOOR_SEAT_BAND_M: Def<f32> = length(def!(float "mw.interact.floor_seat_band_m" = 0.3)
    .range(0.1, 0.5)
    .help("A seat whose top is lower than this above the floor is sat on from the floor (clip family seat.floor)"));

/// A bake-time measuring threshold (`IN32b1`, [`crate::measure()`]): net `Gameplay` (the defs feed
/// gameplay), scope `Dev`, source `Guess`, lifetime `Bake` (part of the bake key).
const fn measuring(d: Def<f32>) -> Def<f32> {
    d.source(Source::Guess).scope(Scope::Dev).net(Net::Gameplay).lifetime(Lifetime::Bake)
}

/// `mw.interact.measure.backrest_above_m`: geometry this far above a seat's top is backrest (or
/// arms), m.
pub const BACKREST_ABOVE_M: Def<f32> = measuring(def!(float "mw.interact.measure.backrest_above_m" = 0.2)
    .range(0.05, 0.5)
    .unit(Unit::Meters)
    .help("Geometry this far above a seat's top counts as its backrest (or arms) when the bake finds the seat's front"));

/// `mw.interact.measure.backrest_offset`: how far off-centre the backrest geometry must sit, as a
/// share of the half footprint, for its side to be the back.
pub const BACKREST_OFFSET: Def<f32> = measuring(def!(float "mw.interact.measure.backrest_offset" = 0.3)
    .range(0.1, 0.9)
    .unit(Unit::Ratio)
    .help("Backrest geometry this far off a seat's centre (share of the half footprint) makes that side the back; less = no backrest"));

/// `mw.interact.measure.upright_min_area`: least area of backrest or headboard geometry, as a
/// share of the footprint area, for it to count.
pub const UPRIGHT_MIN_AREA: Def<f32> = measuring(def!(float "mw.interact.measure.upright_min_area" = 0.1)
    .range(0.01, 1.0)
    .unit(Unit::Ratio)
    .help("Backrest or headboard geometry needs this much area (share of the footprint area) to count; less = the fallback side"));

/// `mw.interact.measure.bench_aspect`: a seat with no backrest this many times longer than deep is
/// a bench, used from a long side.
pub const BENCH_ASPECT: Def<f32> = measuring(def!(float "mw.interact.measure.bench_aspect" = 1.5)
    .range(1.1, 4.0)
    .unit(Unit::Ratio)
    .help("A backless seat this many times longer than deep is a bench and faces a long side; below it, a stool facing +Z"));

/// `mw.interact.measure.headboard_above_m`: geometry this far above a bed's top is a head- or
/// footboard, m.
pub const HEADBOARD_ABOVE_M: Def<f32> = measuring(def!(float "mw.interact.measure.headboard_above_m" = 0.15)
    .range(0.05, 0.6)
    .unit(Unit::Meters)
    .help("Geometry this far above a bed's top counts as a head- or footboard when the bake finds the head end"));

/// `mw.interact.measure.bed_end_share`: each end zone of a bed, as a share of its length.
pub const BED_END_SHARE: Def<f32> = measuring(def!(float "mw.interact.measure.bed_end_share" = 0.25)
    .range(0.1, 0.5)
    .unit(Unit::Ratio)
    .help("The head and foot zones of a bed, each this share of its length, where the boards are measured"));

/// `mw.interact.measure.head_share`: the head end's least share of the two ends' board area.
pub const HEAD_SHARE: Def<f32> = measuring(def!(float "mw.interact.measure.head_share" = 0.6)
    .range(0.5, 0.95)
    .unit(Unit::Ratio)
    .help("The end with at least this share of the two ends' board area is a bed's head; neither = the fallback (+ long axis)"));

/// The crate's tunables, for the shell to register.
pub const TUNABLES: &[&dyn AnyDef] = &[
    &SIT,
    &SNUFF,
    &BENCH_SEAT_M,
    &APPROACH_M,
    &EDGE_SEAT_INSET_M,
    &COUNTER_SPACING_M,
    &COUNTER_STAND_M,
    &FLOOR_SEAT_BAND_M,
    &BACKREST_ABOVE_M,
    &BACKREST_OFFSET,
    &UPRIGHT_MIN_AREA,
    &BENCH_ASPECT,
    &HEADBOARD_ABOVE_M,
    &BED_END_SHARE,
    &HEAD_SHARE,
];

/// The values [`crate::defs::def_for`] reads, as plain fields (the shell binds them from
/// [`TUNABLES`]).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FurnitureParams {
    /// [`SIT`].
    pub sit: bool,
    /// [`BENCH_SEAT_M`], m.
    pub bench_seat_m: f32,
    /// [`APPROACH_M`], m.
    pub approach_m: f32,
    /// [`EDGE_SEAT_INSET_M`], m.
    pub edge_seat_inset_m: f32,
    /// [`COUNTER_SPACING_M`], m.
    pub counter_spacing_m: f32,
    /// [`COUNTER_STAND_M`], m.
    pub counter_stand_m: f32,
    /// [`FLOOR_SEAT_BAND_M`], m.
    pub floor_seat_band_m: f32,
}

impl Default for FurnitureParams {
    fn default() -> Self {
        Self {
            sit: SIT.default_value(),
            bench_seat_m: BENCH_SEAT_M.default_value(),
            approach_m: APPROACH_M.default_value(),
            edge_seat_inset_m: EDGE_SEAT_INSET_M.default_value(),
            counter_spacing_m: COUNTER_SPACING_M.default_value(),
            counter_stand_m: COUNTER_STAND_M.default_value(),
            floor_seat_band_m: FLOOR_SEAT_BAND_M.default_value(),
        }
    }
}

/// The thresholds [`crate::measure()`] reads, as plain fields (the bake uses the defaults; a dev
/// override applies at the next bake).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MeasureParams {
    /// [`BACKREST_ABOVE_M`], m.
    pub backrest_above_m: f32,
    /// [`BACKREST_OFFSET`], share of the half footprint.
    pub backrest_offset: f32,
    /// [`UPRIGHT_MIN_AREA`], share of the footprint area.
    pub upright_min_area: f32,
    /// [`BENCH_ASPECT`], length over depth.
    pub bench_aspect: f32,
    /// [`HEADBOARD_ABOVE_M`], m.
    pub headboard_above_m: f32,
    /// [`BED_END_SHARE`], share of the length.
    pub bed_end_share: f32,
    /// [`HEAD_SHARE`], share of the two ends' board area.
    pub head_share: f32,
}

impl Default for MeasureParams {
    fn default() -> Self {
        Self {
            backrest_above_m: BACKREST_ABOVE_M.default_value(),
            backrest_offset: BACKREST_OFFSET.default_value(),
            upright_min_area: UPRIGHT_MIN_AREA.default_value(),
            bench_aspect: BENCH_ASPECT.default_value(),
            headboard_above_m: HEADBOARD_ABOVE_M.default_value(),
            bed_end_share: BED_END_SHARE.default_value(),
            head_share: HEAD_SHARE.default_value(),
        }
    }
}
