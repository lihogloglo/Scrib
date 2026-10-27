//! Moveset sheets (combat.md "Moveset sheet"): one RON file per weapon class and rig, compiled
//! into generated [`MoveRule`]s (`<sheet>.<move>`, Clip owner, `moveset` and `from` predicates),
//! the slots' windows ([`SlotInfo`]) and a combat [`MoveTable`] (costs, hits, i-frames).
//!
//! Times in a sheet are move time (after `rate`), from the move's start across its slot chain;
//! `End` is the chain's end. Slots whose clip the content doesn't have are dropped from the chain
//! at compile (a warning); a move with every slot dropped is not loaded.

use std::collections::{BTreeMap, HashMap};

use glam::Vec3;
use serde::de::{self, Deserializer};
use serde::{Deserialize, Serialize, Serializer};
use sf_action::{
    ActionClass, Category, ClassSet, ClockTime, Entry, StickSide, MoveKind, MoveRule, MovesetKey, Origin, Owner, Prev, RuleKey, RuleSetBuilder, SlotInfo, SlotKey,
    SlotTable, SquareUp, TagId, Window, WindowKind, WindowPolicy,
};

pub use sf_action::HitMask;

use crate::react::{ReactionTable, Side, Tier};
use crate::shape::WeaponShape;

/// A window end: seconds, or the end of the move.
/// Authored bare: `0.32` or `End`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum At {
    Secs(f32),
    End,
}

/// `At` as read: an untagged number or name. Untagged goes through serde's buffered content, for
/// which RON keeps a bare identifier's name (a plain `deserialize_any` gets a nameless unit).
#[derive(Deserialize)]
#[serde(untagged)]
enum AtRepr {
    Secs(f32),
    Name(String),
}

impl<'de> Deserialize<'de> for At {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        match AtRepr::deserialize(d)? {
            AtRepr::Secs(s) => Ok(Self::Secs(s)),
            AtRepr::Name(n) if n == "End" => Ok(Self::End),
            AtRepr::Name(n) => Err(de::Error::invalid_value(de::Unexpected::Str(&n), &"seconds or End")),
        }
    }
}

impl Serialize for At {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        match *self {
            Self::Secs(v) => s.serialize_f32(v),
            Self::End => s.serialize_unit_variant("At", 1, "End"),
        }
    }
}

/// An i-frame mask in a sheet: `All` or bits.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum MaskDef {
    All,
    Bits(u8),
}

impl MaskDef {
    pub fn mask(self) -> HitMask {
        match self {
            Self::All => HitMask::ALL,
            Self::Bits(b) => HitMask(b),
        }
    }
}

/// An authored window (move time).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum WindowDef {
    Accept(Vec<ActionClass>, f32, At),
    Cancel(Vec<ActionClass>, f32, At),
    /// deg/s.
    Turn(f32, f32, At),
    NoTurn(f32, At),
}

/// A hit row.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct HitDef {
    pub window: (f32, At),
    /// Motion value.
    pub mv: f32,
    /// Stamina charged when the window opens.
    #[serde(default)]
    pub stamina: f32,
    /// Content tag for the rules (`"slash"`).
    pub kind: String,
    /// Unguarded reaction tier.
    pub tier: Tier,
    /// Hit-stop hold, s (`None`: the sheet default).
    #[serde(default)]
    pub stop: Option<f32>,
    #[serde(default)]
    pub flags: u8,
}

fn one() -> f32 {
    1.0
}

/// One move of a sheet.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MoveDef {
    pub category: Category,
    pub classes: Vec<ActionClass>,
    /// Moves it may start from (sheet-local names or global rule ids); empty = only through
    /// another move's `next`, or any move if none chains to it.
    #[serde(default)]
    pub from: Vec<Prev>,
    #[serde(default)]
    pub rank: i32,
    pub slots: Vec<String>,
    #[serde(default = "one")]
    pub rate: f32,
    #[serde(default = "one")]
    pub root_scale: f32,
    #[serde(default)]
    pub square_up: SquareUp,
    /// Stamina at start (roll, backstep).
    #[serde(default)]
    pub cost: f32,
    #[serde(default)]
    pub iframes: Vec<(f32, At, MaskDef)>,
    #[serde(default)]
    pub windows: Vec<WindowDef>,
    #[serde(default)]
    pub hits: Vec<HitDef>,
    /// Chains: a press of the class inside this move's windows starts the named move.
    #[serde(default)]
    pub next: BTreeMap<ActionClass, String>,
    /// Lock-on variants by side (C1c).
    #[serde(default)]
    pub locked: BTreeMap<Side, String>,
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct HitStopDef {
    pub hold: f32,
    pub ease: f32,
}

impl Default for HitStopDef {
    fn default() -> Self {
        Self { hold: 0.06, ease: 0.10 }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Defaults {
    /// deg/s, for content tools (sheet `Turn` windows name their own rate).
    pub turn: f32,
    pub hit_stop: HitStopDef,
    /// Slot blend-in, s (presentation).
    pub blend_in: f32,
}

impl Default for Defaults {
    fn default() -> Self {
        Self { turn: 360.0, hit_stop: HitStopDef::default(), blend_in: 0.10 }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct WeaponDef {
    pub socket: String,
    pub segment: ((f32, f32, f32), (f32, f32, f32)),
    pub radius: f32,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Costs {
    pub jump: f32,
    pub sprint_per_s: f32,
}

/// A sheet file.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SheetDef {
    pub id: String,
    /// The graph family whose slots it names.
    pub rig: String,
    #[serde(default)]
    pub defaults: Defaults,
    pub weapon: WeaponDef,
    pub moves: BTreeMap<String, MoveDef>,
    /// (tier, side) -> the reaction's slot.
    #[serde(default)]
    pub reactions: BTreeMap<(Tier, Side), String>,
    #[serde(default)]
    pub costs: Costs,
}

/// A sheet problem, naming the sheet and move.
#[derive(Clone, Debug, PartialEq, Eq, thiserror::Error)]
#[error("moveset '{sheet}'{}: {message}", .mov.as_ref().map(|m| format!(", move '{m}'")).unwrap_or_default())]
pub struct SheetError {
    pub sheet: String,
    pub mov: Option<String>,
    pub message: String,
}

/// One hit row, compiled (move time).
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct HitRow {
    pub start: ClockTime,
    pub end: ClockTime,
    pub mv: f32,
    pub stamina: f32,
    pub kind: TagId,
    pub tier: Tier,
    /// Hit-stop hold, s.
    pub stop: f32,
    pub flags: HitMask,
}

/// A present slot of a move's chain, in move time.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct SlotSpan {
    pub key: SlotKey,
    pub start: ClockTime,
    pub len: ClockTime,
}

/// A move's combat data.
#[derive(Clone, Debug, PartialEq)]
pub struct MoveSpec {
    pub id: String,
    pub category: Category,
    pub classes: ClassSet,
    pub rate: f32,
    /// Stamina at start.
    pub cost: f32,
    pub slots: Vec<SlotSpan>,
    pub length: ClockTime,
    pub hits: Vec<HitRow>,
    pub iframes: Vec<(ClockTime, ClockTime, HitMask)>,
    /// Regeneration opens here: the first `Cancel(Move)` (else the end).
    pub regen_from: ClockTime,
}

impl MoveSpec {
    /// The slot playing at move time `t` and the clip time inside it (seconds, after `rate`).
    pub fn slot_at(&self, t: ClockTime) -> Option<(SlotKey, f32)> {
        let s = self.slots.iter().find(|s| t < ClockTime(s.start.0 + s.len.0)).or(self.slots.last())?;
        Some((s.key, ClockTime(t.0.saturating_sub(s.start.0).min(s.len.0)).secs() * self.rate))
    }

    /// The i-frame mask open at `t`.
    pub fn iframes_at(&self, t: ClockTime) -> HitMask {
        HitMask(self.iframes.iter().filter(|(a, b, _)| *a <= t && t < *b).fold(0, |m, (_, _, k)| m | k.0))
    }
}

/// Combat data by generated rule key.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct MoveTable {
    pub moves: HashMap<RuleKey, MoveSpec>,
}

/// A compiled sheet.
#[derive(Clone, Debug, PartialEq)]
pub struct CompiledSheet {
    pub id: String,
    pub key: MovesetKey,
    pub rig: String,
    pub rules: Vec<MoveRule>,
    /// Slot name and its timing with the sheet's windows (length in move time).
    pub slots: Vec<(String, SlotInfo)>,
    pub moves: MoveTable,
    pub reactions: ReactionTable,
    pub socket: String,
    pub weapon: WeaponShape,
    pub hit_stop: HitStopDef,
    pub costs: Costs,
    pub warnings: Vec<String>,
}

impl CompiledSheet {
    /// Adds the generated rules (after the base rules).
    pub fn add_rules(&self, builder: &mut RuleSetBuilder, origin: &Origin) {
        for r in &self.rules {
            builder.add(origin.clone(), r.clone());
        }
    }

    /// Inserts the sheet's slots into a slot table.
    #[must_use]
    pub fn fill(&self, mut table: SlotTable) -> SlotTable {
        for (name, info) in &self.slots {
            table = table.with(name, info.clone());
        }
        table
    }

    pub fn rule_key(&self, mov: &str) -> RuleKey {
        RuleKey::of(&format!("{}.{mov}", self.id))
    }
}

/// Every loaded sheet, by key.
#[derive(Clone, Debug, Default)]
pub struct Movesets {
    pub sheets: HashMap<MovesetKey, CompiledSheet>,
}

impl Movesets {
    pub fn insert(&mut self, sheet: CompiledSheet) {
        self.sheets.insert(sheet.key, sheet);
    }

    pub fn spec(&self, moveset: MovesetKey, mov: RuleKey) -> Option<&MoveSpec> {
        self.sheets.get(&moveset)?.moves.moves.get(&mov)
    }
}

/// Parses and compiles a sheet; `clip_len(slot)` is the slot's clip length in seconds, `None` when
/// the content has no clip for it (dropped).
pub fn load(text: &str, clip_len: &dyn Fn(&str) -> Option<f32>) -> Result<CompiledSheet, Vec<SheetError>> {
    // Optional fields read bare (`stop: 0.05`, as authored in the spec), not `Some(0.05)`.
    let ron = ron::Options::default().with_default_extension(ron::extensions::Extensions::IMPLICIT_SOME);
    let def: SheetDef = ron.from_str(text).map_err(|e| vec![SheetError { sheet: "?".into(), mov: None, message: e.to_string() }])?;
    compile(&def, clip_len)
}

/// A move that may open from Loco or anything (`from` has `Any` or `Loco`).
fn opens(m: &MoveDef) -> bool {
    m.from.iter().any(|p| matches!(p, Prev::Any | Prev::Loco))
}

/// Chain ranks: a chain-only target (no `Any`/`Loco`) ranks above its sources and above every
/// opener taking the chained class, transitively (light3 > light2 > light1), so the chooser's
/// priority list picks the chain over a fresh opener. Bounded passes: a chain loop of chain-only
/// moves stops climbing after one pass per move.
fn chain_ranks<'a>(def: &'a SheetDef, chained: &BTreeMap<&'a str, Vec<(&'a str, ActionClass)>>) -> BTreeMap<&'a str, i32> {
    let mut ranks: BTreeMap<&str, i32> = def.moves.iter().map(|(n, m)| (n.as_str(), m.rank)).collect();
    for _ in 0..def.moves.len() {
        let mut changed = false;
        for (&target, sources) in chained {
            let Some(t) = def.moves.get(target).filter(|t| !opens(t)) else { continue };
            let mut r = t.rank;
            for &(src, class) in sources {
                r = r.max(ranks[src] + 1);
                for (n, _) in def.moves.iter().filter(|(_, o)| opens(o) && o.classes.contains(&class)) {
                    r = r.max(ranks[n.as_str()] + 1);
                }
            }
            if ranks.insert(target, r) != Some(r) {
                changed = true;
            }
        }
        if !changed {
            break;
        }
    }
    ranks
}

/// A sheet `locked` side as the chooser's [`StickSide`].
const fn lock_side(s: Side) -> StickSide {
    match s {
        Side::Front => StickSide::Front,
        Side::Back => StickSide::Back,
        Side::Left => StickSide::Left,
        Side::Right => StickSide::Right,
    }
}

fn secs(a: At, end: ClockTime) -> ClockTime {
    match a {
        At::Secs(s) => ClockTime::from_secs(s),
        At::End => end,
    }
}

/// Compiles a parsed sheet (see [`load`]).
#[allow(clippy::too_many_lines)]
pub fn compile(def: &SheetDef, clip_len: &dyn Fn(&str) -> Option<f32>) -> Result<CompiledSheet, Vec<SheetError>> {
    let mut errors = Vec::new();
    let mut warnings = Vec::new();
    let err = |m: Option<&str>, message: String| SheetError { sheet: def.id.clone(), mov: m.map(str::to_owned), message };
    let full = |name: &str| format!("{}.{name}", def.id);
    if def.id.is_empty() {
        errors.push(err(None, "empty id".into()));
    }
    // Chain sources per target, with the chained class.
    let mut chained: BTreeMap<&str, Vec<(&str, ActionClass)>> = BTreeMap::new();
    for (name, m) in &def.moves {
        for (class, target) in &m.next {
            match def.moves.get(target) {
                None => errors.push(err(Some(name), format!("next {class:?} names unknown move '{target}'"))),
                Some(t) if !t.classes.contains(class) => errors.push(err(Some(name), format!("next {class:?}: '{target}' doesn't take {class:?}"))),
                Some(_) => chained.entry(target.as_str()).or_default().push((name.as_str(), *class)),
            }
        }
    }
    // Lock-on variants (`locked`): a variant starts where its base starts, ranked above it, only
    // while locked on with the stick on its side ([`Entry::lock`]); other sides keep the base.
    let mut variants: BTreeMap<&str, (&str, StickSide)> = BTreeMap::new();
    for (name, m) in &def.moves {
        for (side, target) in &m.locked {
            match def.moves.get(target) {
                None => errors.push(err(Some(name), format!("locked {side:?} names unknown move '{target}'"))),
                Some(_) if target == name => errors.push(err(Some(name), format!("locked {side:?} names the move itself"))),
                Some(t) if !m.classes.iter().all(|c| t.classes.contains(c)) => {
                    errors.push(err(Some(name), format!("locked {side:?}: '{target}' doesn't take every class of this move")));
                }
                Some(_) => {
                    if variants.insert(target.as_str(), (name.as_str(), lock_side(*side))).is_some() {
                        errors.push(err(Some(name), format!("locked {side:?}: '{target}' is already another lock-on variant")));
                    }
                }
            }
        }
    }
    let mut ranks = chain_ranks(def, &chained);
    for (&v, &(base, _)) in &variants {
        let above = ranks[base] + 1;
        ranks.entry(v).and_modify(|r| *r = (*r).max(above));
    }
    let mut rules = Vec::new();
    let mut slots = Vec::new();
    let mut moves = MoveTable::default();
    for (name, m) in &def.moves {
        let numbers_ok = m.rate.is_finite() && m.rate > 0.0 && m.root_scale.is_finite() && m.root_scale >= 0.0;
        if !numbers_ok {
            errors.push(err(Some(name), "rate must be > 0 and root_scale >= 0".into()));
            continue;
        }
        if m.slots.is_empty() {
            errors.push(err(Some(name), "no slots".into()));
            continue;
        }
        let mut spans = Vec::new();
        let mut t = ClockTime::ZERO;
        let mut present = Vec::new();
        for s in &m.slots {
            match clip_len(s) {
                Some(len) => {
                    let len = ClockTime::from_secs(len / m.rate);
                    spans.push(SlotSpan { key: SlotKey::of(s), start: t, len });
                    present.push(s.clone());
                    t = ClockTime(t.0 + len.0);
                }
                None => warnings.push(format!("moveset '{}', move '{name}': slot '{s}' has no clip; dropped from the chain", def.id)),
            }
        }
        if spans.is_empty() {
            warnings.push(format!("moveset '{}', move '{name}': every slot dropped; move not loaded", def.id));
            continue;
        }
        let length = t;
        // Windows in move time.
        let mut windows = Vec::new();
        for w in &m.windows {
            let (kind, a, b) = match w {
                WindowDef::Accept(c, a, b) => (WindowKind::Accept(ClassSet::of(c)), *a, *b),
                WindowDef::Cancel(c, a, b) => (WindowKind::Cancel(ClassSet::of(c)), *a, *b),
                // Saturating float-to-int cast: rates above u16 are no turn limit anyway.
                #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
                WindowDef::Turn(rate, a, b) => (WindowKind::Turn(rate.max(0.0) as u16), *a, *b),
                WindowDef::NoTurn(a, b) => (WindowKind::NoTurn, *a, *b),
            };
            windows.push(Window { kind, start: ClockTime::from_secs(a), end: secs(b, length) });
        }
        let mut hits = Vec::new();
        for (i, h) in m.hits.iter().enumerate() {
            let (start, end) = (ClockTime::from_secs(h.window.0), secs(h.window.1, length));
            if start >= end || !h.mv.is_finite() || !h.stamina.is_finite() {
                errors.push(err(Some(name), format!("hit {i}: empty window or bad numbers")));
                continue;
            }
            if end > length {
                warnings.push(format!("moveset '{}', move '{name}': hit {i} runs past the move's end", def.id));
            }
            let n = u8::try_from(i).unwrap_or(u8::MAX);
            windows.push(Window { kind: WindowKind::Hit(n), start, end });
            hits.push(HitRow {
                start,
                end,
                mv: h.mv,
                stamina: h.stamina,
                kind: TagId::of(&h.kind),
                tier: h.tier,
                stop: h.stop.unwrap_or(def.defaults.hit_stop.hold),
                flags: HitMask(h.flags),
            });
        }
        if hits.len() > 32 {
            errors.push(err(Some(name), "more than 32 hit rows".into()));
        }
        let iframes: Vec<_> = m.iframes.iter().map(|&(a, b, k)| (ClockTime::from_secs(a), secs(b, length), k.mask())).collect();
        for &(start, end, k) in &iframes {
            windows.push(Window { kind: WindowKind::IFrames(k), start, end });
        }
        // Split the move's windows into its slots' local windows.
        for (span, slot) in spans.iter().zip(&present) {
            let (s0, s1) = (span.start.0, span.start.0 + span.len.0);
            let mut info = SlotInfo { length: Some(span.len), windows: Vec::new() };
            for w in &windows {
                let (a, b) = (w.start.0.max(s0), w.end.0.min(s1));
                // A window open to the end of the move stays open to the end of its last slot.
                if a < b || (a == b && w.end == length && b == s1) {
                    info.windows.push(Window { kind: w.kind, start: ClockTime(a - s0), end: ClockTime(b - s0) });
                }
            }
            slots.push((slot.clone(), info));
        }
        let regen_from = windows
            .iter()
            .filter(|w| matches!(w.kind, WindowKind::Cancel(c) if c.contains(ActionClass::Move)))
            .map(|w| w.start)
            .min()
            .unwrap_or(length);
        // `from`: authored (sheet names prefixed) plus chain sources; a lock-on variant with none
        // authored takes its base's.
        let variant = variants.get(name.as_str()).copied();
        let mut origins = vec![(name.as_str(), m)];
        if let Some((base, _)) = variant.filter(|_| m.from.is_empty()) {
            origins.push((base, &def.moves[base]));
        }
        let mut from: Vec<Prev> = Vec::new();
        for (o, om) in origins {
            from.extend(om.from.iter().map(|p| match p {
                Prev::Rule(id) if def.moves.contains_key(id) => Prev::Rule(full(id)),
                p => p.clone(),
            }));
            let sources = chained.get(o).map(Vec::as_slice).unwrap_or_default();
            from.extend(sources.iter().map(|(src, _)| Prev::Rule(full(src))));
        }
        let rank = ranks.get(name.as_str()).copied().unwrap_or(m.rank);
        let id = full(name);
        rules.push(MoveRule {
            id: id.clone(),
            kind: MoveKind::Combat,
            category: m.category,
            rank,
            slots: present,
            owner: Owner::Clip { root_scale: m.root_scale, square_up: m.square_up, rate: m.rate },
            entry: Entry { classes: m.classes.clone(), moveset: Some(def.id.clone()), from, lock: variant.map(|(_, s)| s), ..Entry::default() },
            windows: WindowPolicy::Committed,
            exits: Vec::new(),
            launch: None,
            steer: false,
            redo_ticks: 0,
        });
        moves.moves.insert(
            RuleKey::of(&id),
            MoveSpec { id, category: m.category, classes: ClassSet::of(&m.classes), rate: m.rate, cost: m.cost, slots: spans, length, hits, iframes, regen_from },
        );
    }
    let mut reactions = ReactionTable::default();
    for (&(tier, side), slot) in &def.reactions {
        let Some(len) = clip_len(slot) else {
            warnings.push(format!("moveset '{}': reaction slot '{slot}' has no clip; dropped", def.id));
            continue;
        };
        let id = full(&format!("react.{tier:?}.{side:?}").to_lowercase());
        let len = ClockTime::from_secs(len);
        slots.push((slot.clone(), SlotInfo { length: Some(len), windows: Vec::new() }));
        rules.push(MoveRule {
            id: id.clone(),
            kind: MoveKind::Forced,
            category: Category::Forced,
            rank: 0,
            slots: vec![slot.clone()],
            owner: Owner::Clip { root_scale: 1.0, square_up: SquareUp::Keep, rate: 1.0 },
            entry: Entry { moveset: Some(def.id.clone()), ..Entry::default() },
            windows: WindowPolicy::Committed,
            exits: Vec::new(),
            launch: None,
            steer: false,
            redo_ticks: 0,
        });
        let key = RuleKey::of(&id);
        reactions.moves.insert((tier, side), key);
        let span = SlotSpan { key: SlotKey::of(slot), start: ClockTime::ZERO, len };
        let spec =
            MoveSpec { id, category: Category::Forced, classes: ClassSet::EMPTY, rate: 1.0, cost: 0.0, slots: vec![span], length: len, hits: Vec::new(), iframes: Vec::new(), regen_from: len };
        moves.moves.insert(key, spec);
    }
    if !errors.is_empty() {
        return Err(errors);
    }
    let ((ax, ay, az), (bx, by, bz)) = def.weapon.segment;
    Ok(CompiledSheet {
        id: def.id.clone(),
        key: MovesetKey::of(&def.id),
        rig: def.rig.clone(),
        rules,
        slots,
        moves,
        reactions,
        socket: def.weapon.socket.clone(),
        weapon: WeaponShape { a: Vec3::new(ax, ay, az), b: Vec3::new(bx, by, bz), radius: def.weapon.radius },
        hit_stop: def.defaults.hit_stop,
        costs: def.costs,
        warnings,
    })
}
