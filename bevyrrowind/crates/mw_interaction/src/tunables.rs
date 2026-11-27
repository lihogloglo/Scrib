//! The adapter's tunables (CONTRIBUTING "No hardcoded tunables", tunables.md): declared here
//! (Bevy-free), registered and bound by the game shell (`IN32b`/`IN32c`), read by the pure code through
//! [`FurnitureParams`] (defs) and [`MeasureParams`] (the bake's measuring, `IN32b1`), defaults straight
//! from the declarations, so no value is written twice.
//!
//! Switches: `mw.rules.interaction.sit`, `.snuff` and `.break` so far; the other `mw.rules.interaction.*` switches
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

/// `mw.rules.interaction.break`: crates, barrels, urns, sacks, pottery and glass break
/// (`crate::breakable`, `IN38a`). Off = no destructible def and no damage rows: nothing breaks, as in
/// vanilla (the Classic preset turns it off).
pub const BREAK: Def<bool> = def!(bool "mw.rules.interaction.break" = true)
    .help("Crates, barrels, urns, sacks, pottery and glass can be broken; off = unbreakable as in vanilla (Classic)")
    .source(Source::Owner("everything breakable breaks, pottery too (interaction-adapter.md \"Breakables\")"))
    .scope(Scope::Dev)
    .net(Net::Gameplay)
    .lifetime(Lifetime::Respawn);

/// A breakable's health or damage factor (net `Gameplay`, scope `Dev`, source `Guess`): defs are
/// built when cells load, so a change applies to defs built after it; the rows apply when the shell
/// refills its rules.
const fn breaking(d: Def<f32>) -> Def<f32> {
    d.source(Source::Guess).scope(Scope::Dev).net(Net::Gameplay).lifetime(Lifetime::Respawn)
}

/// A breakable class's health, hit points.
const fn break_hp(d: Def<f32>) -> Def<f32> {
    breaking(d.range(0.5, 500.0))
}

pub const BREAK_CRATE_HP: Def<f32> = break_hp(def!(float "mw.rules.interaction.break_crate_hp" = 15.0)
    .help("Health of a breakable crate (hit points, before the damage factors)"));
pub const BREAK_BARREL_HP: Def<f32> = break_hp(def!(float "mw.rules.interaction.break_barrel_hp" = 20.0)
    .help("Health of a breakable barrel (hit points): a few arrows at x0.25"));
pub const BREAK_URN_HP: Def<f32> = break_hp(def!(float "mw.rules.interaction.break_urn_hp" = 10.0)
    .help("Health of a clay urn, container or decorative (hit points)"));
pub const BREAK_SACK_HP: Def<f32> = break_hp(def!(float "mw.rules.interaction.break_sack_hp" = 8.0)
    .help("Health of a breakable sack (hit points)"));
pub const BREAK_POTTERY_HP: Def<f32> = break_hp(def!(float "mw.rules.interaction.break_pottery_hp" = 2.0)
    .help("Health of loose pottery (redware, pots, plates, cups; hit points)"));
pub const BREAK_GLASS_HP: Def<f32> = break_hp(def!(float "mw.rules.interaction.break_glass_hp" = 1.0)
    .help("Health of loose glass (bottles, flasks, glassware; hit points)"));

/// A damage kind's factor on breakables.
const fn break_factor(d: Def<f32>) -> Def<f32> {
    breaking(d.range(0.0, 4.0).unit(Unit::Ratio))
}

pub const BREAK_BLUNT: Def<f32> = break_factor(def!(float "mw.rules.interaction.break_blunt" = 1.0)
    .help("Blunt weapon damage factor on breakables (wood, clay, glass, cloth)"));
pub const BREAK_BLADE: Def<f32> = break_factor(def!(float "mw.rules.interaction.break_blade" = 0.5)
    .help("Blade damage factor on breakables"));
pub const BREAK_ARROW: Def<f32> = break_factor(def!(float "mw.rules.interaction.break_arrow" = 0.25)
    .help("Arrow and bolt damage factor on breakables (a barrel takes a few arrows)"));
pub const BREAK_HAND: Def<f32> = break_factor(def!(float "mw.rules.interaction.break_hand" = 0.25)
    .help("Hand-to-hand damage factor on breakables"));
pub const BREAK_FIRE_WOOD: Def<f32> = break_factor(def!(float "mw.rules.interaction.break_fire_wood" = 1.5)
    .help("Fire damage factor on wooden breakables (crates, barrels)"));

/// A spilled container's contents (`crate::breakable` "Spill", `IN38b3c`; interaction-destruction.md
/// "Spill"): read when a break releases, so a change applies to the next break.
const fn spilling(d: Def<f32>) -> Def<f32> {
    d.source(Source::Guess).scope(Scope::Dev).net(Net::Gameplay).lifetime(Lifetime::Live)
}

pub const BREAK_SPILL_LOSS_KNEE: Def<f32> = spilling(def!(float "mw.rules.interaction.break_spill_loss_knee" = 0.5)
    .range(0.0, 4.0)
    .unit(Unit::Ratio)
    .help("Overkill (damage past zero / the breakable's health) below which a spill loses nothing: an ordinary smash"));
pub const BREAK_SPILL_LOSS_SPAN: Def<f32> = spilling(def!(float "mw.rules.interaction.break_spill_loss_span" = 2.0)
    .range(0.1, 8.0)
    .unit(Unit::Ratio)
    .help("Overkill past the knee over which a spill's loss ramps up to its maximum"));
pub const BREAK_SPILL_LOSS_MAX: Def<f32> = spilling(def!(float "mw.rules.interaction.break_spill_loss_max" = 0.9)
    .range(0.0, 1.0)
    .unit(Unit::Ratio)
    .help("Largest share of each spilled stack destroyed by a vaporising hit (gold never breaks)"));
pub const BREAK_SPILL_HEALTH_PER_OVERKILL: Def<f32> =
    spilling(def!(float "mw.rules.interaction.break_spill_health_per_overkill" = 0.5)
        .range(0.0, 2.0)
        .unit(Unit::Ratio)
        .help("Share of a spilled weapon's or armour's full condition lost per unit of overkill"));
pub const BREAK_SPILL_HEALTH_MAX: Def<f32> = spilling(def!(float "mw.rules.interaction.break_spill_health_max" = 0.75)
    .range(0.0, 1.0)
    .unit(Unit::Ratio)
    .help("Largest share of a spilled weapon's or armour's full condition one break takes"));
pub const BREAK_SPILL_RADIUS_M: Def<f32> = spilling(def!(float "mw.rules.interaction.break_spill_radius_m" = 0.25)
    .range(0.0, 1.0)
    .unit(Unit::Meters)
    .help("Radius around the broken container's origin its contents land in"));
pub const BREAK_SPILL_LIFT_M: Def<f32> = spilling(def!(float "mw.rules.interaction.break_spill_lift_m" = 0.3)
    .range(0.0, 1.0)
    .unit(Unit::Meters)
    .help("Height above the container's origin the first spilled stack starts falling from"));
pub const BREAK_SPILL_LAYER_M: Def<f32> = spilling(def!(float "mw.rules.interaction.break_spill_layer_m" = 0.05)
    .range(0.0, 0.3)
    .unit(Unit::Meters)
    .help("Extra height per spilled stack, so stacks do not start inside each other"));

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
    &BREAK,
    &BREAK_CRATE_HP,
    &BREAK_BARREL_HP,
    &BREAK_URN_HP,
    &BREAK_SACK_HP,
    &BREAK_POTTERY_HP,
    &BREAK_GLASS_HP,
    &BREAK_BLUNT,
    &BREAK_BLADE,
    &BREAK_ARROW,
    &BREAK_HAND,
    &BREAK_FIRE_WOOD,
    &BREAK_SPILL_LOSS_KNEE,
    &BREAK_SPILL_LOSS_SPAN,
    &BREAK_SPILL_LOSS_MAX,
    &BREAK_SPILL_HEALTH_PER_OVERKILL,
    &BREAK_SPILL_HEALTH_MAX,
    &BREAK_SPILL_RADIUS_M,
    &BREAK_SPILL_LIFT_M,
    &BREAK_SPILL_LAYER_M,
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

/// The values [`crate::breakable`] reads, as plain fields (the shell binds them from [`TUNABLES`]).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BreakParams {
    /// [`BREAK`].
    pub enabled: bool,
    /// [`BREAK_CRATE_HP`].
    pub crate_hp: f32,
    /// [`BREAK_BARREL_HP`].
    pub barrel_hp: f32,
    /// [`BREAK_URN_HP`].
    pub urn_hp: f32,
    /// [`BREAK_SACK_HP`].
    pub sack_hp: f32,
    /// [`BREAK_POTTERY_HP`].
    pub pottery_hp: f32,
    /// [`BREAK_GLASS_HP`].
    pub glass_hp: f32,
    /// [`BREAK_BLUNT`].
    pub blunt: f32,
    /// [`BREAK_BLADE`].
    pub blade: f32,
    /// [`BREAK_ARROW`].
    pub arrow: f32,
    /// [`BREAK_HAND`].
    pub hand: f32,
    /// [`BREAK_FIRE_WOOD`].
    pub fire_wood: f32,
    /// The spill's loss, condition and scatter.
    pub spill: SpillParams,
}

/// The spill's values (`crate::breakable` "Spill"), part of [`BreakParams`].
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SpillParams {
    /// [`BREAK_SPILL_LOSS_KNEE`].
    pub loss_knee: f32,
    /// [`BREAK_SPILL_LOSS_SPAN`].
    pub loss_span: f32,
    /// [`BREAK_SPILL_LOSS_MAX`].
    pub loss_max: f32,
    /// [`BREAK_SPILL_HEALTH_PER_OVERKILL`].
    pub health_per_overkill: f32,
    /// [`BREAK_SPILL_HEALTH_MAX`].
    pub health_max: f32,
    /// [`BREAK_SPILL_RADIUS_M`].
    pub radius_m: f32,
    /// [`BREAK_SPILL_LIFT_M`].
    pub lift_m: f32,
    /// [`BREAK_SPILL_LAYER_M`].
    pub layer_m: f32,
}

impl Default for SpillParams {
    fn default() -> Self {
        Self {
            loss_knee: BREAK_SPILL_LOSS_KNEE.default_value(),
            loss_span: BREAK_SPILL_LOSS_SPAN.default_value(),
            loss_max: BREAK_SPILL_LOSS_MAX.default_value(),
            health_per_overkill: BREAK_SPILL_HEALTH_PER_OVERKILL.default_value(),
            health_max: BREAK_SPILL_HEALTH_MAX.default_value(),
            radius_m: BREAK_SPILL_RADIUS_M.default_value(),
            lift_m: BREAK_SPILL_LIFT_M.default_value(),
            layer_m: BREAK_SPILL_LAYER_M.default_value(),
        }
    }
}

impl BreakParams {
    /// The health of `class`.
    #[must_use]
    pub const fn health(&self, class: crate::breakable::BreakClass) -> f32 {
        use crate::breakable::BreakClass as C;
        match class {
            C::Crate => self.crate_hp,
            C::Barrel => self.barrel_hp,
            C::Urn => self.urn_hp,
            C::Sack => self.sack_hp,
            C::Pottery => self.pottery_hp,
            C::Glass => self.glass_hp,
        }
    }
}

impl Default for BreakParams {
    fn default() -> Self {
        Self {
            enabled: BREAK.default_value(),
            crate_hp: BREAK_CRATE_HP.default_value(),
            barrel_hp: BREAK_BARREL_HP.default_value(),
            urn_hp: BREAK_URN_HP.default_value(),
            sack_hp: BREAK_SACK_HP.default_value(),
            pottery_hp: BREAK_POTTERY_HP.default_value(),
            glass_hp: BREAK_GLASS_HP.default_value(),
            blunt: BREAK_BLUNT.default_value(),
            blade: BREAK_BLADE.default_value(),
            arrow: BREAK_ARROW.default_value(),
            hand: BREAK_HAND.default_value(),
            fire_wood: BREAK_FIRE_WOOD.default_value(),
            spill: SpillParams::default(),
        }
    }
}
