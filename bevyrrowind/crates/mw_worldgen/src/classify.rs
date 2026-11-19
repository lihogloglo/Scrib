//! Exterior references as human-made / nature / other (spec `worldgen-adapter.md` "Anchors: what counts as
//! human-made"; WG-X2).
//!
//! Human-made references form anchors (they keep their place at 1:1 in a scaled world); nature is dropped and
//! regenerated; "other" neither seeds an anchor nor is guessed silently: the census prints those classes (the
//! WG-R3 lesson from Procedural Tamriel: mesh-path classes misfire, so the ambiguous ones go to the owner).

use std::fmt;

/// One exterior reference, reduced to what the classification and the anchors need.
#[derive(Clone, Debug, PartialEq)]
pub struct RefInfo {
    /// The stable reference key a placement is matched by (`Placement.id`, 32 hex digits).
    pub key: String,
    /// The base record's kind tag (`STAT`, `DOOR`, `LIGH`, `CONT`, `ACTI`, `NPC_`, `CREA`, `LEVC`, `BOOK`, ...).
    pub kind: String,
    /// The base record id.
    pub base: String,
    /// Model VFS path, lower case with `/` (`meshes/x/ex_hlaalu_b_01.nif`).
    pub model: Option<String>,
    /// Source metres: x east, y north, z up.
    pub pos: [f64; 3],
    /// The reference's uniform scale.
    pub scale: f32,
    /// Model-space bounds at unit scale, engine axes (x east, y up, z south), metres.
    pub bounds: Option<([f32; 3], [f32; 3])>,
}

impl RefInfo {
    /// Horizontal radius around the reference's origin that covers its model's bounds (scaled), if known.
    pub fn bounds_radius_m(&self) -> Option<f64> {
        let (lo, hi) = self.bounds?;
        let x = f64::from(lo[0].abs().max(hi[0].abs()));
        let z = f64::from(lo[2].abs().max(hi[2].abs()));
        Some(x.hypot(z) * f64::from(self.scale))
    }

    /// Largest edge of the scaled bounds, metres.
    pub fn bounds_extent_m(&self) -> Option<f64> {
        let (lo, hi) = self.bounds?;
        let e = (0..3).map(|i| f64::from(hi[i] - lo[i])).fold(0.0, f64::max);
        Some(e * f64::from(self.scale))
    }

    /// The model's file stem, lower case (`ex_hlaalu_b_01`).
    pub fn stem(&self) -> Option<&str> {
        let m = self.model.as_deref()?;
        let file = m.rsplit('/').next().unwrap_or(m);
        Some(file.strip_suffix(".nif").unwrap_or(file))
    }

    /// The census group of the model: its folder below `meshes/` plus the stem's first two `_` words
    /// (`x/ex_hlaalu`), or `(no model) <KIND>`.
    pub fn folder(&self) -> String {
        let Some(m) = self.model.as_deref() else { return format!("(no model) {}", self.kind) };
        let m = m.strip_prefix("meshes/").unwrap_or(m);
        let dir = m.rsplit_once('/').map_or("", |(d, _)| d);
        let stem = self.stem().unwrap_or("");
        let prefix: Vec<&str> = stem.split('_').take(2).collect();
        format!("{dir}/{}", prefix.join("_"))
    }
}

/// The three classes of the spec.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Class {
    HumanMade,
    Nature,
    Other,
}

impl fmt::Display for Class {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::HumanMade => "human-made",
            Self::Nature => "nature",
            Self::Other => "other",
        })
    }
}

/// The rule that classified a reference (the census groups by it).
pub type Rule = &'static str;

/// Whether a model path is Morrowind flora (same rule as `mw_bake::far::is_flora`, `mw_bake/src/far.rs`).
pub fn is_flora(model: &str) -> bool {
    let m = model.to_ascii_lowercase();
    m.contains("/flora_") || m.contains("/f/flora")
}

/// Classifies one reference. `rock_min_m`: rocks whose largest bounds edge reaches this are terrain (nature).
pub fn classify(r: &RefInfo, rock_min_m: f64) -> (Class, Rule) {
    if mw_world::EDITOR_MARKERS.iter().any(|m| r.base.eq_ignore_ascii_case(m)) || r.stem() == Some("editormarker") {
        return (Class::Other, "editor marker");
    }
    // Harvestable plants are containers (and a few activators or lights) with flora meshes: nature, scattered again
    // with the flora.
    let harvest = |n: &str| n.starts_with("flora_") || n.starts_with("contain_trama") || n.starts_with("furn_shell");
    if harvest(&r.base.to_ascii_lowercase()) || r.stem().is_some_and(harvest) || r.model.as_deref().is_some_and(is_flora) {
        return (Class::Nature, if r.kind == "STAT" { "flora" } else { "flora (harvestable)" });
    }
    match r.kind.as_str() {
        "DOOR" => return (Class::HumanMade, "door"),
        "CONT" => return (Class::HumanMade, "container"),
        // Model-less lights are ambient sources (over lava, in the sea): they would chain the wilds into anchors.
        "LIGH" if r.model.is_none() => return (Class::Other, "light without model"),
        "LIGH" => return (Class::HumanMade, "light"),
        "ACTI" => return (Class::HumanMade, "activator"),
        "NPC_" => return (Class::HumanMade, "npc"),
        "BOOK" => return (Class::HumanMade, "book or note"),
        // Wild creatures and levelled spawns would make one-reference anchors all over the wilds: they ride
        // along inside footprints but don't seed anchors (an ambiguous class the census prints).
        "CREA" | "LEVC" => return (Class::Other, "creature or levelled spawn"),
        "STAT" => {}
        _ => return (Class::Other, "loose item"),
    }
    let Some(stem) = r.stem() else { return (Class::Other, "static without model") };
    if stem.starts_with("terrain_rock") {
        return (Class::Nature, "terrain rock");
    }
    // The Ghostfence ring is regenerated along a path at 10×, not stamped (spec "Hard cases").
    if stem.contains("ghostfence") || stem.starts_with("ex_gg_fence") || stem.starts_with("ex_gg_pylon") {
        return (Class::Other, "ghostfence (path)");
    }
    // Menhirs: the rock spires of Azura's Coast (thousands), rocks by shape.
    if stem.contains("rock") || stem.contains("boulder") || stem.contains("menhir") {
        return match r.bounds_extent_m() {
            Some(e) if e >= rock_min_m => (Class::Nature, "large rock"),
            Some(_) => (Class::Other, "small rock"),
            None => (Class::Other, "rock without bounds"),
        };
    }
    // Path markers line the roads: re-placed with the roads, and they would chain whole roads into one anchor.
    if stem.contains("pathspear") {
        return (Class::Other, "path marker (road)");
    }
    // Shore and cave rock dressing: barnacle clusters, cave pieces used as exterior cliffs and arches.
    if stem.starts_with("ex_barnacle") {
        return (Class::Other, "barnacles (shore dressing)");
    }
    if stem.starts_with("in_cave") {
        return (Class::Other, "cave piece in the exterior");
    }
    if stem.contains("bone") || stem.contains("skull") || stem.contains("skeleton") {
        return (Class::Other, "bones");
    }
    if stem.starts_with("terrain_") {
        return (Class::Other, "terrain (not rock)");
    }
    (Class::HumanMade, "structure")
}
