//! Runtime morph weights of dressed characters (character.md "Morph
//! playback").
//!
//! Two sources write a character's expression targets, by target name:
//!
//! 1. **Animation curves.** A clip curve whose name is a morph target of the
//!    outfit drives it (the glTF / UE convention: morph weights are curves
//!    in the clip, blended by the graph with the pose). Whole-body geometry
//!    morphs that loop with idle and walk play this way.
//! 2. **[`MorphCurvePlayer`]**: curves sampled at a time the game sets each
//!    frame (faces: the game maps "blinking" or "talking" to a time on the
//!    head's own timeline, which is how some legacy content keys faces).
//!    It is applied after the animation, weighted.
//!
//! Untouched targets keep their default weight. Each morph-bearing mesh gets
//! its own targets gathered into `MeshMorphWeights::Value`, written only
//! when a weight changed (no change detection, no re-upload otherwise).
//! Facial LOD: morph meshes exist only on near outfit LODs (assembly), and
//! characters whose animator did not pose this frame (the animation LOD's
//! frozen or culled tiers, which already account for view and distance)
//! keep their last weights.

use std::sync::Arc;

use bevy_ecs::prelude::*;
use bevy_mesh::morph::MeshMorphWeights;
use sf_anim::Curve;
use sf_anim_bevy::Animator;

use crate::{CharacterBody, OutfitAssets};

/// One morph target's weight over time.
#[derive(Clone, Debug, PartialEq)]
pub struct MorphTrack {
    /// The morph target it drives (`MorphTarget::name` of the outfit).
    pub target: String,
    pub curve: Curve<f32>,
}

/// Curves a [`MorphCurvePlayer`] plays, shared by every character with the
/// same part.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct MorphTracks {
    pub tracks: Vec<MorphTrack>,
}

/// Plays [`MorphTracks`] on a character root at an externally driven time.
#[derive(Component, Clone, Debug)]
pub struct MorphCurvePlayer {
    pub tracks: Arc<MorphTracks>,
    /// Seconds on the tracks' timeline (clamped to their keys).
    pub time: f32,
    /// 0: off (animation and defaults show), 1: the tracks replace them.
    pub weight: f32,
}

impl MorphCurvePlayer {
    pub fn new(tracks: Arc<MorphTracks>) -> Self {
        Self { tracks, time: 0.0, weight: 1.0 }
    }
}

/// On a dressed character's root whose outfit has runtime morph targets:
/// the current weight of each, by name. Inserted and refreshed by the
/// outfit spawn; read it to inspect what the character shows.
#[derive(Component, Clone, Debug, Default)]
pub struct CharacterMorphs {
    /// Every runtime morph target of the outfit (first-seen order).
    pub names: Vec<String>,
    pub defaults: Vec<f32>,
    /// What the meshes show (updated by [`drive_character_morphs`]).
    pub weights: Vec<f32>,
    /// Morph-bearing mesh entities and their targets as indices into `names`.
    meshes: Vec<(Entity, Vec<u16>)>,
    /// `(name index, graph curve)` for the bound animation asset.
    anim: Vec<(u16, u16)>,
    anim_asset: usize,
    /// `(name index or none)` per track of the bound player tracks.
    player: Vec<Option<u16>>,
    player_tracks: usize,
}

impl CharacterMorphs {
    /// The outfit's runtime targets bound to its spawned mesh entities
    /// (`meshes[i]` shows `outfit.meshes[i]`); `None` when nothing morphs.
    pub(crate) fn of_outfit(outfit: &OutfitAssets, meshes: &[Entity]) -> Option<Self> {
        let mut out = Self::default();
        for (m, &entity) in outfit.meshes.iter().zip(meshes) {
            if m.morph_targets.is_empty() {
                continue;
            }
            let mut indices = Vec::with_capacity(m.morph_targets.len());
            for (name, &weight) in m.morph_targets.iter().zip(&m.morph_weights) {
                let i = out.names.iter().position(|n| n == name).unwrap_or_else(|| {
                    out.names.push(name.clone());
                    out.defaults.push(weight);
                    out.names.len() - 1
                });
                indices.push(u16::try_from(i).unwrap_or(u16::MAX));
            }
            out.meshes.push((entity, indices));
        }
        out.weights.clone_from(&out.defaults);
        out.anim_asset = usize::MAX;
        out.player_tracks = usize::MAX;
        (!out.meshes.is_empty()).then_some(out)
    }

    /// Index of a target by name.
    pub fn find(&self, name: &str) -> Option<usize> {
        self.names.iter().position(|n| n == name)
    }

    /// Current weight of a target by name.
    pub fn weight(&self, name: &str) -> Option<f32> {
        self.find(name).map(|i| self.weights[i])
    }
}

/// Animation curves, then the player, into each morph mesh's
/// `MeshMorphWeights` (`PostUpdate`, after `AnimSystems::Apply`).
#[allow(clippy::type_complexity)]
pub fn drive_character_morphs(
    mut roots: Query<(&CharacterBody, &mut CharacterMorphs, Option<&MorphCurvePlayer>)>,
    animators: Query<&Animator>,
    mut meshes: Query<&mut MeshMorphWeights>,
    mut next: Local<Vec<f32>>,
) {
    for (body, mut morphs, player) in &mut roots {
        let animator = animators.get(body.body).ok();
        if animator.is_some_and(|a| !a.posed()) {
            continue;
        }
        let morphs = &mut *morphs;
        next.clone_from(&morphs.defaults);
        if let Some(animator) = animator {
            let asset = Arc::as_ptr(&animator.asset) as usize;
            if morphs.anim_asset != asset {
                morphs.anim_asset = asset;
                morphs.anim = bind(&morphs.names, animator.asset.graph.curves.iter().map(String::as_str))
                    .enumerate()
                    .filter_map(|(c, n)| Some((n?, u16::try_from(c).ok()?)))
                    .collect();
                log::debug!("morphs: {} of {} targets follow animation curves", morphs.anim.len(), morphs.names.len());
            }
            for &(n, c) in &morphs.anim {
                if let (Some(slot), Some(&v)) = (next.get_mut(usize::from(n)), animator.output.curves.get(usize::from(c))) {
                    *slot = v;
                }
            }
        }
        if let Some(player) = player.filter(|p| p.weight > 0.0) {
            let tracks = Arc::as_ptr(&player.tracks) as usize;
            if morphs.player_tracks != tracks {
                morphs.player_tracks = tracks;
                morphs.player = bind(&morphs.names, player.tracks.tracks.iter().map(|t| t.target.as_str())).collect();
            }
            let w = player.weight.min(1.0);
            for (track, n) in player.tracks.tracks.iter().zip(&morphs.player) {
                if let Some(slot) = n.and_then(|n| next.get_mut(usize::from(n))) {
                    *slot += (track.curve.sample(player.time) - *slot) * w;
                }
            }
        }
        if *next == morphs.weights {
            continue;
        }
        morphs.weights.clone_from(&next);
        for (entity, indices) in &morphs.meshes {
            if let Ok(mut mesh) = meshes.get_mut(*entity) {
                let weights = indices.iter().map(|&i| next.get(usize::from(i)).copied().unwrap_or(0.0)).collect();
                *mesh = MeshMorphWeights::Value { weights };
            }
        }
    }
}

/// For each of `names_in`, its index in `names`.
fn bind<'a>(names: &'a [String], names_in: impl Iterator<Item = &'a str> + 'a) -> impl Iterator<Item = Option<u16>> + 'a {
    names_in.map(|name| names.iter().position(|n| n == name).and_then(|i| u16::try_from(i).ok()))
}
