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
//! - `mat:<material>:<channel>`: `alpha`, `diffuse.r|g|b`, `emissive.r|g|b`
//!   (NIF color values), `uv.u|v` (offset), `tile.u|v` (tiling).
//!   `<material>` is the base texture's file name, which is how `mw_bake`
//!   names cooked materials ([`material_name`]).

use mw_nif::Model;
use mw_nif::anim::{ColorSlot, Curve as NifCurve, Target};

/// Name prefix of visibility curves.
pub const VISIBILITY: &str = "vis:";
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
    Visibility { node: &'a str },
    Material { material: &'a str, channel: Channel },
}

/// The effect a clip curve drives; `None` for other curves (morph weights).
pub fn parse(name: &str) -> Option<Effect<'_>> {
    if let Some(node) = name.strip_prefix(VISIBILITY) {
        return Some(Effect::Visibility { node });
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
    /// Visibility (`vis:` names): `(time, 1 shown / 0 hidden)` step keys.
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
        out
    }
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
        assert_eq!(parse("Tri Head#1"), None);
    }
}
