//! The adapter's tunables (CONTRIBUTING "No hardcoded tunables", tunables.md): declared here
//! (Bevy-free), registered and bound by the game shell (`IN32b`/`IN32c`), read by the pure code through
//! [`FurnitureParams`] (defaults straight from the declarations, so no value is written twice).
//!
//! Switches: only `mw.rules.interaction.sit` so far; the other `mw.rules.interaction.*` switches
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

/// The crate's tunables, for the shell to register.
pub const TUNABLES: &[&dyn AnyDef] =
    &[&SIT, &BENCH_SEAT_M, &APPROACH_M, &EDGE_SEAT_INSET_M, &COUNTER_SPACING_M, &COUNTER_STAND_M];

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
        }
    }
}
