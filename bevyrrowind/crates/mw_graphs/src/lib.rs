//! Morrowind animation graphs for `sf_anim_graph` (Bevy-free).
//!
//! - [`build`]: the templates ([`Template`]: humanoid, first person,
//!   creature) built in code and shipped as RON under `data/` ([`BUILTIN`],
//!   VFS paths as `mw_actors::ActorConfig` names them), so mods replace or
//!   patch them like any graph asset.
//! - [`AnimSet`]: one skeleton with its KF/NIF animation groups, the
//!   `CompileSource` that resolves the templates' clip references
//!   (`Group`, `Group@section`, `Group@tail`) and cooks them.
//! - [`validate`]: compile a template for a skeleton and report how every
//!   clip resolved; [`coverage`]: which groups each creature model has.
//! - [`sim::Sim`]: headless simulation (parameter scripts → states, clips,
//!   events), for tests and tools.
//! - [`script`]: `PlayGroup` / `LoopGroup` → montage slot requests.
//!
//! Spec: `docs/specs/actors-adapter.md` "Graph assets as built".

pub mod build;
pub mod script;
pub mod sim;
mod source;

use std::fmt::Write as _;

pub use build::{Stance, Template, asset, to_ron};
use sf_anim_graph::CompiledGraph;
pub use source::{AnimSet, HUMANOID_SETS, HumanoidSet, creature_animations};

/// The shipped graph assets: `(VFS path, RON text)`.
pub const BUILTIN: [(&str, &str); 3] = [
    ("animation/graphs/mw_humanoid.animgraph.ron", include_str!("../data/animation/graphs/mw_humanoid.animgraph.ron")),
    ("animation/graphs/mw_first_person.animgraph.ron", include_str!("../data/animation/graphs/mw_first_person.animgraph.ron")),
    ("animation/graphs/mw_creature.animgraph.ron", include_str!("../data/animation/graphs/mw_creature.animgraph.ron")),
];

/// Label of the [`builtin_source`] layer.
pub const BUILTIN_LABEL: &str = "builtin:mw_graphs";

/// The shipped graph assets as a VFS layer. Mount it first (lowest
/// priority, under the archives and `Data Files`): the graphs then
/// resolve at the `mw_actors::ActorConfig` paths like any content, and a
/// mod file at the same path replaces them (everything is a mod).
pub fn builtin_source() -> sf_vfs::MemSource {
    BUILTIN.iter().fold(sf_vfs::MemSource::new(BUILTIN_LABEL), |source, (path, text)| source.with(path, *text))
}

/// The shipped RON of a template.
pub fn builtin(t: Template) -> &'static str {
    BUILTIN.iter().find(|(p, _)| *p == t.path()).map(|(_, text)| *text).expect("every template ships")
}

/// The template a creature plays, from its CREA flags: biped creatures
/// (dremora, golden saints, skeletons, liches) share the humanoid
/// skeleton layout and animation (`xbase_anim.kf` layered under their own,
/// [`creature_animations`]), so they take the humanoid graph with its
/// weapon stances; every other creature takes the creature graph.
pub fn creature_template(flags: u32) -> Template {
    if flags & mw_anim::rig::crea_flags::BIPED != 0 { Template::Humanoid } else { Template::Creature }
}

/// How a template compiled for one skeleton.
#[derive(Clone, Debug)]
pub struct Validation {
    pub graph: CompiledGraph,
    /// Clips that took a fallback: `(name, missing references, resolved to)`.
    pub fell_back: Vec<(String, Vec<String>, Option<String>)>,
    /// Optional clips that dropped (with what only they fed).
    pub dropped: Vec<String>,
    /// States of the `locomotion` machine that survived.
    pub states: Vec<String>,
}

/// Compiles the shipped template for `set` and reports clip resolutions.
pub fn validate(set: &mut AnimSet, t: Template) -> Result<Validation, String> {
    let asset = sf_anim_graph::asset::parse_graph(builtin(t))?;
    let graph = set.compile(&format!("{}:{}", set.name, t.path()), &asset)?;
    let mut fell_back = Vec::new();
    let mut dropped = Vec::new();
    for r in graph.resolutions.iter().filter(|r| r.fell_back()) {
        match r.slot {
            Some(s) => fell_back.push((r.name.clone(), r.missing.clone(), Some(graph.clips[usize::from(s)].reference.clone()))),
            None => dropped.push(r.name.clone()),
        }
    }
    let states = graph.node(build::LOCOMOTION).map(|n| graph.state_names(n).into_iter().map(str::to_owned).collect()).unwrap_or_default();
    Ok(Validation { graph, fell_back, dropped, states })
}

/// One creature model's row in the coverage report.
#[derive(Clone, Debug)]
pub struct Coverage {
    pub model: String,
    /// CREA ids using it.
    pub creatures: Vec<String>,
    pub mover: mw_anim::rig::CreatureMover,
    pub template: Template,
    pub groups: usize,
    /// `Err`: the template didn't compile (or the model didn't load).
    pub result: Result<Validation, String>,
}

/// A table: per model, mover, template, group count, states kept, clips dropped.
pub fn coverage(rows: &[Coverage]) -> String {
    let mut out = String::new();
    let ok = rows.iter().filter(|r| r.result.is_ok()).count();
    let _ = writeln!(out, "{} creature models, {ok} compile with their template", rows.len());
    for r in rows {
        let _ = write!(out, "{:40} {:7} {:?} {:3} groups  ", r.model, format!("{:?}", r.mover), r.template, r.groups);
        match &r.result {
            Ok(v) => {
                let _ = writeln!(out, "{} states, dropped: {}", v.states.len(), v.dropped.join(" "));
            }
            Err(e) => {
                let _ = writeln!(out, "ERROR {e}");
            }
        }
    }
    out
}

#[cfg(test)]
mod tests;
