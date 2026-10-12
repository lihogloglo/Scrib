//! NPC faces: blinking and talking on the head's geometry morph (character.md
//! "Morph playback", Morrowind rules).
//!
//! A Morrowind head NIF keys its morph targets on one timeline, and text
//! keys bound two segments of it: `Talk: Start`/`Talk: Stop` (mouth shapes
//! by loudness) and `Blink: Start`/`Blink: Stop`. The game picks a time on
//! that timeline (observed in Morrowind; `OpenMW`'s head animation time does
//! the same): blinking plays the blink segment after a random 2-8 s pause;
//! talking maps the voice loudness (doubled, clamped to 1) onto the talk
//! segment, and wins over blinking. The framework's `MorphCurvePlayer`
//! samples the head's curves at that time.
//!
//! **Scaffold "talk key"**: until dialogue and voice exist, `Y` toggles a fake
//! loudness on every NPC. Removal: the dialogue system sets [`Talking`]
//! loudness from the playing voice's envelope.

use std::collections::HashMap;
use std::sync::Arc;

use bevy::prelude::*;
use mw_anim::Animation;
use sf_character_bevy::{Character, CharacterMorphs, Characters, MorphCurvePlayer, MorphTrack, MorphTracks};
use sf_vfs::{Vfs, VfsPath};

/// A head's morph curves and its face segments.
#[derive(Debug)]
pub struct Face {
    tracks: Arc<MorphTracks>,
    talk: Option<(f32, f32)>,
    blink: Option<(f32, f32)>,
}

impl Face {
    /// The face of a head NIF; `None` without morph curves.
    pub fn from_nif(bytes: &[u8]) -> Option<Self> {
        let model = mw_nif::load(bytes).ok()?;
        let animation = Animation::from_model(&model);
        let tracks: Vec<MorphTrack> = animation
            .morphs
            .iter()
            .filter(|(_, c)| !c.is_empty())
            .map(|(name, c)| {
                let end = c.keys().map(|k| k.0).fold(0.0f32, f32::max);
                let curve = mw_anim::convert::float(name, c, 0.0, end);
                MorphTrack { target: curve.name, curve: curve.curve }
            })
            .collect();
        if tracks.is_empty() {
            return None;
        }
        let key = |name: &str| {
            animation
                .text_keys
                .iter()
                .find(|(_, text)| text.lines().any(|l| l.trim().eq_ignore_ascii_case(name)))
                .map(|(t, _)| *t)
        };
        let segment = |a: &str, b: &str| key(a).zip(key(b)).filter(|(a, b)| b > a);
        Some(Self {
            tracks: Arc::new(MorphTracks { tracks }),
            talk: segment("talk: start", "talk: stop"),
            blink: segment("blink: start", "blink: stop"),
        })
    }
}

/// Heads by model path, and the talk-key scaffold's switch.
#[derive(Resource)]
pub struct MwFaces {
    vfs: Arc<Vfs>,
    cache: HashMap<VfsPath, Option<Arc<Face>>>,
    /// Scaffold "talk key": every NPC talks (fake loudness).
    pub talking: bool,
}

/// An NPC's face clock.
#[derive(Component, Debug)]
pub struct FaceState {
    face: Arc<Face>,
    /// Seconds into the blink segment; negative: the pause before it.
    blink: f32,
    clock: f32,
    seed: u32,
}

/// Voice loudness (0-1) while an NPC speaks.
#[derive(Component, Debug, Default)]
pub struct Talking(pub f32);

/// No head morph to drive (looked up once).
#[derive(Component)]
struct NoFace;

pub fn add(app: &mut App, vfs: Arc<Vfs>) {
    app.insert_resource(MwFaces { vfs, cache: HashMap::new(), talking: false })
        .add_systems(Update, (attach_faces, talk_key, drive_faces).chain());
}

/// Next pause before a blink: 2-8 s (xorshift per face).
fn next_pause(seed: &mut u32) -> f32 {
    *seed ^= *seed << 13;
    *seed ^= *seed >> 17;
    *seed ^= *seed << 5;
    #[allow(clippy::cast_precision_loss)] // 24 bits fit an f32 mantissa
    let unit = (*seed >> 8) as f32 / (1u32 << 24) as f32;
    2.0 + 6.0 * unit
}

#[allow(clippy::needless_pass_by_value, clippy::type_complexity)]
fn attach_faces(
    mut commands: Commands,
    mut faces: ResMut<MwFaces>,
    characters: Option<Res<Characters>>,
    fresh: Query<(Entity, &Character), (With<CharacterMorphs>, Without<FaceState>, Without<NoFace>)>,
) {
    let Some(characters) = characters else { return };
    for (entity, character) in &fresh {
        let face = head_model(&characters, character).and_then(|model| {
            let vfs = Arc::clone(&faces.vfs);
            faces.cache.entry(model.clone()).or_insert_with(|| vfs.read(&model).ok().and_then(|b| Face::from_nif(&b)).map(Arc::new)).clone()
        });
        let Some(face) = face else {
            commands.entity(entity).insert(NoFace);
            continue;
        };
        #[allow(clippy::cast_possible_truncation)] // a seed: mixing, not a value
        let mut seed = ((entity.to_bits() ^ (entity.to_bits() >> 29)) as u32).wrapping_mul(0x9E37_79B9) | 1;
        let blink = -next_pause(&mut seed);
        commands.entity(entity).insert((MorphCurvePlayer::new(Arc::clone(&face.tracks)), FaceState { face, blink, clock: 0.0, seed }));
    }
}

/// The head part's model of an NPC record.
fn head_model(characters: &Characters, character: &Character) -> Option<VfsPath> {
    let Character::Record { kind, id, state } = character else { return None };
    if !kind.eq_ignore_ascii_case(mw_actors::NPC_KIND) {
        return None;
    }
    let content = characters.content();
    let recipe = content.source().recipe(kind, id, state).ok()?;
    Some(content.parts().part(recipe.head.as_ref()?)?.model.clone())
}

#[allow(clippy::needless_pass_by_value)]
fn talk_key(keys: Res<ButtonInput<KeyCode>>, mut faces: ResMut<MwFaces>) {
    if keys.just_pressed(KeyCode::KeyY) {
        faces.talking = !faces.talking;
        info!("faces: talk key (scaffold) {}", if faces.talking { "on: every NPC talks" } else { "off" });
    }
}

#[allow(clippy::needless_pass_by_value)]
fn drive_faces(time: Res<Time>, faces: Res<MwFaces>, mut q: Query<(&mut FaceState, &mut MorphCurvePlayer, Option<&Talking>)>) {
    let dt = time.delta_secs();
    for (mut st, mut player, talking) in &mut q {
        let t = st.step(dt, talking.map(|t| t.0), faces.talking);
        if player.time.to_bits() != t.to_bits() {
            player.time = t;
        }
    }
}

impl FaceState {
    /// Advances the face by `dt` and returns the head timeline's time.
    /// `loudness`: a voice playing; `fake_talk`: the talk-key scaffold.
    fn step(&mut self, dt: f32, loudness: Option<f32>, fake_talk: bool) -> f32 {
        self.clock += dt;
        // Fake syllables for the scaffold: two beating sines, 0-0.5.
        #[allow(clippy::cast_precision_loss)]
        let phase = (self.seed % 1000) as f32 * 0.01;
        let clock = self.clock;
        let fake = || 0.25 * (1.0 + (clock * 19.0 + phase).sin()) * (0.5 + 0.5 * (clock * 4.4 + phase).sin());
        match (loudness.or_else(|| fake_talk.then(fake)), self.face.talk, self.face.blink) {
            (Some(loud), Some((a, b)), _) => a + (b - a) * (loud * 2.0).clamp(0.0, 1.0),
            (_, _, Some((a, b))) => {
                self.blink += dt;
                if self.blink > b - a {
                    self.blink = -next_pause(&mut self.seed);
                }
                if self.blink >= 0.0 { a + self.blink } else { 0.0 }
            }
            _ => 0.0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The dark elf head's keys: talk 0-1.33 s, blink 1.4-2.0 s.
    fn state(seed: u32) -> FaceState {
        let face = Arc::new(Face { tracks: Arc::default(), talk: Some((0.0, 1.333)), blink: Some((1.4, 2.0)) });
        let mut seed = seed | 1;
        FaceState { face, blink: -next_pause(&mut seed), clock: 0.0, seed }
    }

    #[test]
    fn blinks_fire_every_two_to_eight_seconds_and_talking_maps_loudness() {
        for seed in [1, 77, 4242, 0xDEAD] {
            let mut st = state(seed);
            let (dt, mut last_blink, mut blinks) = (1.0 / 60.0, 0.0f32, 0);
            let mut was_blinking = false;
            for frame in 0..60 * 60 {
                #[allow(clippy::cast_precision_loss)]
                let now = frame as f32 * dt;
                let t = st.step(dt, None, false);
                let blinking = t > 0.0;
                assert!(t == 0.0 || (1.4..=2.0).contains(&t), "time {t} outside the blink segment");
                if blinking && !was_blinking {
                    let gap = now - last_blink;
                    assert!(blinks == 0 || (2.0..=8.0 + 0.7).contains(&gap), "seed {seed}: blink gap {gap} s");
                    assert!(blinks > 0 || gap <= 8.1, "seed {seed}: first blink after {gap} s");
                    (last_blink, blinks) = (now, blinks + 1);
                }
                was_blinking = blinking;
            }
            assert!((7..=30).contains(&blinks), "seed {seed}: {blinks} blinks a minute");
        }
        let mut st = state(5);
        assert!((st.step(0.016, Some(0.25), false) - 0.6665).abs() < 1e-3, "loudness 0.25 opens half the talk segment");
        assert!((st.step(0.016, Some(0.9), false) - 1.333).abs() < 1e-3, "loud voices clamp to talk stop");
        let fake = st.step(0.016, None, true);
        assert!((0.0..=1.333).contains(&fake), "the talk key plays in the talk segment ({fake})");
    }
}
