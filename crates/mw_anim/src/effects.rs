//! Visibility and material controllers of an actor model as clip curves
//! (nif-runtime.md "Actor parts").
//!
//! In an actor's own NIF, these controllers key the same timeline as its
//! animation groups (`dremora.nif` keys visibility over 0-667 s, the length
//! of its group timeline), and the original engine plays them at the
//! current animation's time; `OpenMW` does the same (`Animation::
//! addSingleAnimSource` assigns the whole-body animation time to every
//! controller of the object root that has no source yet). So, like
//! geometry-morph weights, each group's clip carries their cut as float
//! curves, and the game applies them by name after the graph blends them:
//!
//! - `vis:<node>`: 1 shown, 0 hidden (step keys).
//! - `emit:<particle node>`: 1 while the particle controller's emission
//!   window is open at that animation time, else 0 (step keys; see
//!   [`emission_keys`]). A second system on a node of the same name is
//!   `emit:<node>#1`, and so on ([`emission_names`]); systems whose window
//!   covers their controller range emit throughout.
//! - `mat:<material>:<channel>`: `alpha`, `diffuse.r|g|b`, `emissive.r|g|b`
//!   (NIF color values), `uv.u|v` (offset), `tile.u|v` (tiling).
//!   `<material>` is the base texture's file name, which is how `mw_bake`
//!   names cooked materials ([`material_name`]).

use mw_nif::Model;
use mw_nif::anim::{ColorSlot, Curve as NifCurve, Cycle, Target};
use mw_nif::particles::ParticleSystem;

/// Name prefix of visibility curves.
pub const VISIBILITY: &str = "vis:";
/// Name prefix of particle emission curves.
pub const EMISSION: &str = "emit:";
/// Name prefix of material curves.
pub const MATERIAL: &str = "mat:";

/// One material channel a curve drives.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Channel {
    Alpha,
    /// RGB component 0-2.
    Diffuse(usize),
    Emissive(usize),
    /// Texture offset, U (0) or V (1).
    UvOffset(usize),
    /// Texture tiling, U (0) or V (1).
    UvTiling(usize),
}

impl Channel {
    fn name(self) -> &'static str {
        const RGB: [[&str; 3]; 2] = [["diffuse.r", "diffuse.g", "diffuse.b"], ["emissive.r", "emissive.g", "emissive.b"]];
        match self {
            Self::Alpha => "alpha",
            Self::Diffuse(i) => RGB[0][i.min(2)],
            Self::Emissive(i) => RGB[1][i.min(2)],
            Self::UvOffset(i) => ["uv.u", "uv.v"][i.min(1)],
            Self::UvTiling(i) => ["tile.u", "tile.v"][i.min(1)],
        }
    }

    fn parse(name: &str) -> Option<Self> {
        Some(match name {
            "alpha" => Self::Alpha,
            "diffuse.r" => Self::Diffuse(0),
            "diffuse.g" => Self::Diffuse(1),
            "diffuse.b" => Self::Diffuse(2),
            "emissive.r" => Self::Emissive(0),
            "emissive.g" => Self::Emissive(1),
            "emissive.b" => Self::Emissive(2),
            "uv.u" => Self::UvOffset(0),
            "uv.v" => Self::UvOffset(1),
            "tile.u" => Self::UvTiling(0),
            "tile.v" => Self::UvTiling(1),
            _ => return None,
        })
    }
}

/// What a clip curve name drives.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Effect<'a> {
    Visibility {
        node: &'a str,
    },
    /// `name`: the curve name after the prefix, as [`emission_names`] makes it.
    Emission {
        name: &'a str,
    },
    Material {
        material: &'a str,
        channel: Channel,
    },
}

/// The effect a clip curve drives; `None` for other curves (morph weights).
pub fn parse(name: &str) -> Option<Effect<'_>> {
    if let Some(node) = name.strip_prefix(VISIBILITY) {
        return Some(Effect::Visibility { node });
    }
    if let Some(name) = name.strip_prefix(EMISSION) {
        return Some(Effect::Emission { name });
    }
    let (material, channel) = name.strip_prefix(MATERIAL)?.rsplit_once(':')?;
    Some(Effect::Material { material, channel: Channel::parse(channel)? })
}

/// A material's name as `mw_bake` names its cooked material: the base
/// texture's file name. `None` without a base texture.
pub fn material_name(material: &mw_nif::Material) -> Option<&str> {
    let t = material.base_texture.as_deref()?;
    Some(t.rsplit('/').next().unwrap_or(t))
}

/// An actor model's effect curves on its animation timeline.
#[derive(Clone, Debug, Default)]
pub struct Effects {
    /// Material channels (`mat:` names).
    pub curves: Vec<(String, NifCurve<f32>)>,
    /// Visibility (`vis:` names): `(time, 1 shown / 0 hidden)` step keys;
    /// particle emission (`emit:` names): `(time, 1 emitting / 0 not)`.
    pub steps: Vec<(String, Vec<(f32, f32)>)>,
    /// Controllers left out: untextured materials, ambient/specular colors.
    pub skipped: usize,
}

impl Effects {
    pub fn of_model(model: &Model) -> Self {
        let mut out = Self::default();
        for c in &model.controllers {
            let material = |m: usize| model.materials.get(m).and_then(material_name);
            let mut push = |m: usize, channel: Channel, curve: NifCurve<f32>| match material(m) {
                Some(name) if !curve.is_empty() => out.curves.push((format!("{MATERIAL}{name}:{}", channel.name()), curve)),
                Some(_) => {}
                None => out.skipped += 1,
            };
            match &c.target {
                Target::Transform { .. } => {}
                Target::Visibility { node, keys } => {
                    let Some(n) = model.nodes.get(*node) else { continue };
                    let keys = keys.keys().iter().map(|&(t, shown)| (t, if shown { 1.0 } else { 0.0 })).collect();
                    out.steps.push((format!("{VISIBILITY}{}", n.name), keys));
                }
                Target::Alpha { material, curve } => push(*material, Channel::Alpha, curve.clone()),
                Target::MaterialColor { material, slot, curve } => {
                    let channel: fn(usize) -> Channel = match slot {
                        ColorSlot::Diffuse => Channel::Diffuse,
                        ColorSlot::Emissive => Channel::Emissive,
                        ColorSlot::Ambient | ColorSlot::Specular => {
                            out.skipped += 1;
                            continue;
                        }
                    };
                    push(*material, channel(0), curve.clone().map(|v| v.x));
                    push(*material, channel(1), curve.clone().map(|v| v.y));
                    push(*material, channel(2), curve.clone().map(|v| v.z));
                }
                Target::Uv { material, track } => {
                    push(*material, Channel::UvOffset(0), track.offset_u.clone());
                    push(*material, Channel::UvOffset(1), track.offset_v.clone());
                    push(*material, Channel::UvTiling(0), track.tiling_u.clone());
                    push(*material, Channel::UvTiling(1), track.tiling_v.clone());
                }
            }
        }
        // The animation timeline's end: past its last text key, a looping
        // controller's window repeats no further.
        let end = model.text_keys.iter().map(|k| k.0).fold(0.0, f32::max);
        for (system, name) in model.particles.iter().zip(emission_names(model)) {
            out.steps.push((name, emission_keys(system, end)));
        }
        out
    }
}

/// The emission curve name of each of `model`'s particle systems (same
/// order): `emit:<particle node>`, then `emit:<node>#1`, `#2`... for further
/// systems on a node of the same name (exporters name every array node
/// `PArray`: Dagoth Ur has eight).
pub fn emission_names(model: &Model) -> Vec<String> {
    let node = |s: &ParticleSystem| model.nodes.get(s.node).map_or("", |n| n.name.as_str());
    let mut out: Vec<String> = Vec::with_capacity(model.particles.len());
    for (i, system) in model.particles.iter().enumerate() {
        let before = model.particles[..i].iter().filter(|s| node(s) == node(system)).count();
        let node = node(system);
        out.push(if before == 0 { format!("{EMISSION}{node}") } else { format!("{EMISSION}{node}#{before}") });
    }
    out
}

/// A particle system's emission window on the animation timeline, as
/// `(time, 1 emitting / 0 not)` step keys from time 0 to `end` (or the
/// controller's stop, if later). The actor's animation time is the
/// controller's input time (nif-runtime.md "Array emitters, live emitters,
/// emission clocks"), so the window opens where [`ParticleSystem::emits_at`]
/// says, repeated by the controller's cycle. A window covering the controller
/// range ([`ParticleSystem::emits_throughout`]) emits throughout.
pub fn emission_keys(system: &ParticleSystem, end: f32) -> Vec<(f32, f32)> {
    let timing = &system.timing;
    let value = |t: f32| if system.emits_at(t) { 1.0 } else { 0.0 };
    let length = timing.stop - timing.start;
    let end = end.max(timing.stop);
    if system.emits_throughout() {
        // A window ending just short of the range (the flame atronach's body
        // flames stop 1 s before the end of its timeline, in `Idle3`) is an
        // export artifact, not a pause: emit throughout.
        return vec![(0.0, 1.0)];
    }
    if timing.frequency <= 0.0 || length <= 0.0 {
        return vec![(0.0, value(0.0))];
    }
    // Controller times (from its start) where the window opens or closes or
    // the cycle turns, within one period of the cycle.
    let (open, close) = (system.emit_start - timing.start, system.emit_stop - timing.start);
    let (period, edges) = match timing.cycle {
        Cycle::Loop => (length, vec![0.0, open, close]),
        Cycle::PingPong => (2.0 * length, vec![0.0, open, close, length, 2.0 * length - close, 2.0 * length - open]),
        Cycle::Clamp => (f32::INFINITY, vec![0.0, open, close, length]),
    };
    // Animation time of controller time `u` in period `k`.
    // (`k` is 0 for a clamped controller, whose period is infinite.)
    let to_time = |u: f32, k: f32| (timing.start + u + if k == 0.0 { 0.0 } else { k * period } - timing.phase) / timing.frequency;
    let mut times = Vec::new();
    if period.is_finite() {
        // The period holding animation time 0, then on to `end`.
        let mut k = ((timing.phase - timing.start) / period).floor();
        while to_time(0.0, k) < end {
            times.extend(edges.iter().map(|&u| to_time(u, k)));
            k += 1.0;
        }
    } else {
        times.extend(edges.iter().map(|&u| to_time(u, 0.0)));
    }
    times.retain(|&t| t > 0.0 && t < end);
    times.sort_by(f32::total_cmp);
    times.dedup_by(|a, b| *a - *b < 1e-5);
    // Each span between edges takes the value at its middle.
    let mut keys = vec![(0.0, value(0.5 * times.first().copied().unwrap_or(end)))];
    for (i, &t) in times.iter().enumerate() {
        let v = value(t.midpoint(times.get(i + 1).copied().unwrap_or(end)));
        if keys.last().is_some_and(|k| (k.1 - v).abs() > 0.5) {
            keys.push((t, v));
        }
    }
    keys
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn names_round_trip() {
        for channel in [Channel::Alpha, Channel::Diffuse(1), Channel::Emissive(2), Channel::UvOffset(0), Channel::UvTiling(1)] {
            let name = format!("{MATERIAL}ghost.dds:{}", channel.name());
            assert_eq!(parse(&name), Some(Effect::Material { material: "ghost.dds", channel }));
        }
        assert_eq!(parse("vis:Sphere01"), Some(Effect::Visibility { node: "Sphere01" }));
        assert_eq!(parse("emit:Fire#1"), Some(Effect::Emission { name: "Fire#1" }));
        assert_eq!(parse("Tri Head#1"), None);
    }

    fn system(emit: (f32, f32), range: (f32, f32), cycle: Cycle) -> ParticleSystem {
        let timing = mw_nif::anim::Timing { start: range.0, stop: range.1, cycle, ..Default::default() };
        ParticleSystem { emit_start: emit.0, emit_stop: emit.1, timing, ..Default::default() }
    }

    #[test]
    fn emission_window_on_the_animation_timeline() {
        // A death effect: one window near the end of the timeline (dremora dust).
        let keys = emission_keys(&system((667.0, 668.5), (0.0, 669.0), Cycle::Clamp), 669.0);
        assert_eq!(keys, [(0.0, 0.0), (667.0, 1.0), (668.5, 0.0)]);
        // Whole range, stopping 1 s short of its end (atronach body flames).
        assert_eq!(emission_keys(&system((0.0, 27.7), (0.0, 28.7), Cycle::Loop), 28.7), [(0.0, 1.0)]);
        // A short looping controller repeats over a longer timeline.
        let keys = emission_keys(&system((1.0, 2.0), (0.0, 4.0), Cycle::Loop), 10.0);
        assert_eq!(keys, [(0.0, 0.0), (1.0, 1.0), (2.0, 0.0), (5.0, 1.0), (6.0, 0.0), (9.0, 1.0)]);
        // A clamped one stays at its end state past its range.
        let keys = emission_keys(&system((0.0, 2.0), (0.0, 4.0), Cycle::Clamp), 10.0);
        assert_eq!(keys, [(0.0, 1.0), (2.0, 0.0)]);
        // No window: always on.
        assert_eq!(emission_keys(&system((0.0, 0.0), (0.0, 4.0), Cycle::Loop), 10.0), [(0.0, 1.0)]);
    }

    #[test]
    fn systems_on_same_named_nodes_get_numbered_names() {
        let node = |name: &str| mw_nif::Node {
            name: name.into(),
            parent: None,
            transform: mw_nif::coords::Trs { translation: [0.0; 3], rotation: [0.0, 0.0, 0.0, 1.0], scale: 1.0 },
            kind: mw_nif::NodeKind::Particles,
            visible: true,
            mesh: None,
        };
        let model = Model {
            nodes: vec![node("PArray"), node("Body"), node("PArray")],
            particles: vec![
                ParticleSystem { node: 0, ..Default::default() },
                ParticleSystem { node: 1, ..Default::default() },
                ParticleSystem { node: 2, ..Default::default() },
                ParticleSystem { node: 0, ..Default::default() },
            ],
            ..Default::default()
        };
        assert_eq!(emission_names(&model), ["emit:PArray", "emit:Body", "emit:PArray#1", "emit:PArray#2"]);
    }
}
