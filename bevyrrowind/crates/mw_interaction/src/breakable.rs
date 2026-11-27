//! Breakables (interaction-adapter.md "Breakables (IN38; bash IN37)", slice `IN38a`): which records
//! break, their material and class, the class's [`DestructibleDef`] and the damage multipliers the
//! shell's `DestructionRules` holds. Pure: the game shell reads the record into [`BreakFacts`] and
//! attaches the def to the placement (`IN38b1`).
//!
//! - **Set** (owner: everything breakable breaks, pottery too). `CONT` ids starting
//!   `contain_crate`, `contain_barrel`, `contain_urn`, `contain_sack`; `MISC` pottery and glass by id
//!   token (`redware`, `pottery`, `pot`, `urn`, `vase`, `jug`, `pitcher`, `bowl`, `cup`, `plate`;
//!   `glass`, `bottle`, `flask`); decorative `STAT` urns and pots by the clay tokens, never
//!   architecture (`ex_*`, `in_*`). A metal, wood, bone or stone token (`metal`, `silverware`,
//!   `dwrv`, `wood`, ...) rules a `MISC` or `STAT` out first, so a silver cup or a wooden bowl stays
//!   whole. Chests, closets, desks, corpses and everything else match nothing: [`Eligibility::Not`].
//! - **Refused** (reported by the cook's counts): an organic container (it respawns its contents),
//!   a record whose script runs on activation or death, or one any script names (quest containers
//!   must not vanish). The shell computes the two script facts from the `mw_script` index.
//! - **Def** ([`destructible_def`]): one stage at health fraction 0 that removes the object (no
//!   broken model yet), the material's debris set, contents spilled for containers only; health
//!   per class from the `mw.rules.interaction.break_*_hp` `Guess` tunables. `ignore_external` stays
//!   off (refused records get no def at all), no loot or salvage (the contents are the loot).
//! - **Damage** ([`damage_rules`]): `(kind, material) -> factor` rows: blunt full, blades x0.5,
//!   arrows x0.25 (a barrel takes a few arrows), hand to hand x0.25 on every breakable material;
//!   fire on wood x1.5. Absent pairs are 1 in the shell.
//! - **Switch off** (`mw.rules.interaction.break`, Classic): no def and no rows, nothing breaks.

use sf_interaction::{BreakAction, DestructibleDef, Stage, TagId};
use sf_records::RecordId;
use smallvec::{SmallVec, smallvec};

use crate::tunables::BreakParams;

/// Material tags (the IN41 material rows' names).
pub mod material {
    use sf_interaction::TagId;

    pub const WOOD: TagId = TagId::of("wood");
    pub const CLAY: TagId = TagId::of("clay");
    pub const GLASS: TagId = TagId::of("glass");
    pub const CLOTH: TagId = TagId::of("cloth");
    /// Every material a breakable can have, for [`super::damage_rules`].
    pub const BREAKABLE: [TagId; 4] = [WOOD, CLAY, GLASS, CLOTH];
}

/// `PropDamage` kind tags the adapter's sources write (combat C14 folds its damage type and hit
/// class into one of these, combat-adapter.md "Prop targets").
pub mod kind {
    use sf_interaction::TagId;

    pub const BLUNT: TagId = TagId::of("blunt");
    pub const BLADE: TagId = TagId::of("blade");
    pub const ARROW: TagId = TagId::of("arrow");
    pub const HAND: TagId = TagId::of("hand");
    pub const FIRE: TagId = TagId::of("fire");
}

/// The debris set record of each material (generic planks, shards, scraps; pack meshes, `IN1c`).
pub mod debris {
    pub const WOOD: &str = "mw_debris_wood_planks";
    pub const CLAY: &str = "mw_debris_clay_shards";
    pub const GLASS: &str = "mw_debris_glass_shards";
    pub const CLOTH: &str = "mw_debris_cloth_scraps";
}

/// Which record type a breakable candidate is.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Source {
    /// `CONT`.
    Container,
    /// `MISC`.
    Misc,
    /// `STAT`.
    Static,
}

/// What the adapter reads from a record (module docs, "Set" and "Refused").
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BreakFacts<'a> {
    pub source: Source,
    /// The record id (any case).
    pub id: &'a str,
    /// `CONT` `ORGANIC` flag.
    pub organic: bool,
    /// The record's own script has an `OnActivate` or `OnDeath`-like block.
    pub activate_script: bool,
    /// Some script names this record.
    pub named_by_script: bool,
}

/// A breakable's class: its health and material.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum BreakClass {
    Crate,
    Barrel,
    Urn,
    Sack,
    Pottery,
    Glass,
}

impl BreakClass {
    /// The class's material tag.
    #[must_use]
    pub const fn material(self) -> TagId {
        match self {
            Self::Crate | Self::Barrel => material::WOOD,
            Self::Urn | Self::Pottery => material::CLAY,
            Self::Glass => material::GLASS,
            Self::Sack => material::CLOTH,
        }
    }

    /// The class's debris set record id.
    #[must_use]
    pub const fn debris(self) -> &'static str {
        match self {
            Self::Crate | Self::Barrel => debris::WOOD,
            Self::Urn | Self::Pottery => debris::CLAY,
            Self::Glass => debris::GLASS,
            Self::Sack => debris::CLOTH,
        }
    }
}

/// A breakable record: its class, and whether it spills contents (containers only).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Breakable {
    pub class: BreakClass,
    pub spills: bool,
}

/// Why a candidate stays unbreakable (module docs, "Refused").
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Refusal {
    Organic,
    ActivateScript,
    NamedByScript,
}

/// The adapter's verdict on one record.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Eligibility {
    /// Not in the breakable set.
    Not,
    /// In the set but kept whole.
    Refused(BreakClass, Refusal),
    Breakable(Breakable),
}

/// Tokens of an id that rule a `MISC` or `STAT` out (metal, wood, bone, stone).
const HARD_TOKENS: &[&str] = &[
    "metal", "silver", "silverware", "gold", "brass", "iron", "steel", "dwrv", "dwemer", "ebony", "wood", "wooden",
    "bone", "stone", "daedric",
];
const GLASS_TOKENS: &[&str] = &["glass", "bottle", "flask"];
const CLAY_TOKENS: &[&str] = &["redware", "pottery", "pot", "urn", "vase", "jug", "pitcher", "bowl", "cup", "plate"];
const CONTAINER_PREFIXES: &[(&str, BreakClass)] = &[
    ("contain_crate", BreakClass::Crate),
    ("contain_barrel", BreakClass::Barrel),
    ("contain_urn", BreakClass::Urn),
    ("contain_sack", BreakClass::Sack),
];

fn has_token(tokens: &[&str], set: &[&str]) -> bool {
    tokens.iter().any(|t| set.contains(t))
}

/// The class of a record by its id alone (module docs, "Set"), before the refusals.
#[must_use]
pub fn class_of(source: Source, id: &str) -> Option<(BreakClass, bool)> {
    let id = id.to_ascii_lowercase();
    if source == Source::Container {
        return CONTAINER_PREFIXES.iter().find(|(p, _)| id.starts_with(p)).map(|&(_, c)| (c, true));
    }
    let tokens: Vec<&str> = id.split('_').collect();
    if has_token(&tokens, HARD_TOKENS) {
        return None;
    }
    let class = match source {
        Source::Misc if has_token(&tokens, GLASS_TOKENS) => BreakClass::Glass,
        Source::Static if id.starts_with("ex_") || id.starts_with("in_") => return None,
        Source::Static if tokens.contains(&"urn") => BreakClass::Urn,
        _ if has_token(&tokens, CLAY_TOKENS) => BreakClass::Pottery,
        _ => return None,
    };
    Some((class, false))
}

/// The adapter's verdict on `facts` (module docs, "Set" and "Refused"); the switch does not change
/// it (the cook's counts stay the same in Classic), [`destructible_def`] does.
#[must_use]
pub fn classify(facts: &BreakFacts) -> Eligibility {
    let Some((class, spills)) = class_of(facts.source, facts.id) else {
        return Eligibility::Not;
    };
    let refusal = if facts.organic && facts.source == Source::Container {
        Some(Refusal::Organic)
    } else if facts.activate_script {
        Some(Refusal::ActivateScript)
    } else if facts.named_by_script {
        Some(Refusal::NamedByScript)
    } else {
        None
    };
    match refusal {
        Some(r) => Eligibility::Refused(class, r),
        None => Eligibility::Breakable(Breakable { class, spills }),
    }
}

/// The def of a breakable (module docs, "Def"); `None` with the switch off.
#[must_use]
pub fn destructible_def(b: Breakable, p: &BreakParams) -> Option<DestructibleDef> {
    p.enabled.then(|| DestructibleDef {
        health: p.health(b.class),
        stages: smallvec![Stage {
            at: 0.0,
            model: None,
            debris: smallvec![RecordId::new(b.class.debris())],
            chunks: None,
            effect: None,
            on_break: if b.spills { BreakAction::SpillContents } else { BreakAction::None },
        }],
        ignore_external: false,
        loot: None,
        salvage: SmallVec::new(),
    })
}

/// The `(kind, material, factor)` rows for the shell's `DestructionRules` (module docs, "Damage"),
/// sorted by kind then material as listed; empty with the switch off.
#[must_use]
pub fn damage_rules(p: &BreakParams) -> Vec<(TagId, TagId, f32)> {
    if !p.enabled {
        return Vec::new();
    }
    let mut rows = Vec::new();
    for (k, f) in [(kind::BLUNT, p.blunt), (kind::BLADE, p.blade), (kind::ARROW, p.arrow), (kind::HAND, p.hand)] {
        rows.extend(material::BREAKABLE.iter().map(|&m| (k, m, f)));
    }
    rows.push((kind::FIRE, material::WOOD, p.fire_wood));
    rows
}
