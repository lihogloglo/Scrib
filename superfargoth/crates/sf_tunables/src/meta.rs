//! What a tunable is beyond its value: unit, provenance, preset group, scope, net role and
//! what a change costs. Spec: `docs/specs/tunables.md` "Definition", "Lifetimes".

use std::fmt;

use serde::Serialize;

/// The unit a number is in. Part of the name too (`_m`, `_s`), so the dump and console show it.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Unit {
    /// No unit (switches, choices, factors with no physical meaning).
    #[default]
    None,
    Meters,
    Seconds,
    Millis,
    Pixels,
    Degrees,
    Lux,
    /// A number of things (lights, rays, steps).
    Count,
    /// A unitless fraction or multiplier.
    Ratio,
    Texels,
}

impl Unit {
    /// Short symbol: `m`, `s`, `ms`, `px`, `deg`, `lux`, `count`, `ratio`, `texels`; `""` for none.
    pub const fn symbol(self) -> &'static str {
        match self {
            Unit::None => "",
            Unit::Meters => "m",
            Unit::Seconds => "s",
            Unit::Millis => "ms",
            Unit::Pixels => "px",
            Unit::Degrees => "deg",
            Unit::Lux => "lux",
            Unit::Count => "count",
            Unit::Ratio => "ratio",
            Unit::Texels => "texels",
        }
    }
}

/// Where a default comes from (provenance). `Guess` is legal and honest: the dump counts
/// guesses, and a sweep turns one into `Measured`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Serialize)]
#[serde(tag = "kind", content = "detail", rename_all = "snake_case")]
pub enum Source {
    /// The Bevy field it mirrors (`bevy_pbr::DirectionalLightShadowMap::size`).
    BevyDefault(&'static str),
    /// An engine doc, talk or paper.
    Industry(&'static str),
    /// `OpenMW`, a game's data, another shipped game.
    Reference(&'static str),
    /// A ledger line (`docs/ledger/<system>.md` date): a sweep or A/B chose it.
    Measured(&'static str),
    /// An owner decision (its date).
    Owner(&'static str),
    /// Nobody measured it yet.
    #[default]
    Guess,
}

impl Source {
    /// `industry`, `measured`, ... (the dump's provenance column and guess counts).
    pub const fn kind(self) -> &'static str {
        match self {
            Source::BevyDefault(_) => "bevy",
            Source::Industry(_) => "industry",
            Source::Reference(_) => "reference",
            Source::Measured(_) => "measured",
            Source::Owner(_) => "owner",
            Source::Guess => "guess",
        }
    }

    /// The text after the kind (empty for `Guess`).
    pub const fn detail(self) -> &'static str {
        match self {
            Source::BevyDefault(s)
            | Source::Industry(s)
            | Source::Reference(s)
            | Source::Measured(s)
            | Source::Owner(s) => s,
            Source::Guess => "",
        }
    }
}

impl fmt::Display for Source {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.detail() {
            "" => f.write_str(self.kind()),
            d => write!(f, "{}: {d}", self.kind()),
        }
    }
}

/// The preset (scalability) group a tunable belongs to: one `sg.*` driver each.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Group {
    ViewDistance,
    Shadows,
    Lighting,
    Post,
    Textures,
    Effects,
    Characters,
    Water,
    Sky,
    Foliage,
}

impl Group {
    /// The group driver's name (`sg.shadows`).
    pub const fn driver(self) -> &'static str {
        match self {
            Group::ViewDistance => "sg.view_distance",
            Group::Shadows => "sg.shadows",
            Group::Lighting => "sg.lighting",
            Group::Post => "sg.post",
            Group::Textures => "sg.textures",
            Group::Effects => "sg.effects",
            Group::Characters => "sg.characters",
            Group::Water => "sg.water",
            Group::Sky => "sg.sky",
            Group::Foliage => "sg.foliage",
        }
    }
}

/// Who may see and change it.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Scope {
    /// Console, BRP, command line and mod/profile files only; never an options-menu row
    /// (owner 2026-11-16: fine-tuned values are dev-only).
    #[default]
    Dev,
    /// Has an options-menu row (preset and quality groups).
    Player,
}

/// Its role in co-op.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Net {
    /// Each peer its own; never feeds gameplay.
    #[default]
    Presentation,
    /// Host-authoritative: on a client only the host's value counts, a change at run time is a
    /// gameplay command applied on the fixed tick.
    Gameplay,
}

/// What a change costs.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Lifetime {
    /// Next frame, through the bound resource or component.
    #[default]
    Live,
    /// Applies to things spawned after the change.
    Respawn,
    /// Live, but changes a shader def: pipelines re-specialise.
    Recompile,
    /// Stored, read at the next start.
    Restart,
    /// Part of the bake key; needs a re-bake.
    Bake,
}

impl Lifetime {
    /// What the console says after a change (`None` for `Live`).
    pub const fn notice(self) -> Option<&'static str> {
        match self {
            Lifetime::Live => None,
            Lifetime::Respawn => Some("applies to new spawns"),
            Lifetime::Recompile => Some("recompiles pipelines"),
            Lifetime::Restart => Some("applies on restart"),
            Lifetime::Bake => Some("needs a re-bake"),
        }
    }

    pub const fn name(self) -> &'static str {
        match self {
            Lifetime::Live => "live",
            Lifetime::Respawn => "respawn",
            Lifetime::Recompile => "recompile",
            Lifetime::Restart => "restart",
            Lifetime::Bake => "bake",
        }
    }
}

/// Everything about a tunable except its name, kind, default and help.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct Meta {
    pub unit: Unit,
    pub source: Source,
    pub group: Option<Group>,
    pub scope: Scope,
    pub net: Net,
    pub lifetime: Lifetime,
    /// Where it is declared (`file!()`, `line!()`), for the dump.
    pub location: Option<(&'static str, u32)>,
    /// A deprecated environment switch that still sets it (`BEVYRROWIND_SSAO`): read once when
    /// the tunable is registered, at the `CommandLine` layer (`sf_tunables_bevy`). Kept one
    /// release, then removed in favour of `--set name=value`.
    pub env_alias: Option<&'static str>,
}
