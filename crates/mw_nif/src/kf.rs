//! Keyframe files (`.kf`): bone animation without a scene graph.
//!
//! A KF's root is an `NiSequenceStreamHelper`. Its extra-data chain starts
//! with the text keys (`NiTextKeyExtraData`), then holds one
//! `NiStringExtraData` per animated bone, naming it; its controller chain
//! holds the matching `NiKeyframeController`s in the same order. Key times
//! are absolute seconds on the file's single timeline; the text keys split
//! that timeline into animation groups.

use tes3::nif::{NiKeyframeController, NiSequenceStreamHelper, NiStream, NiStringExtraData, NiTextKeyExtraData};

use crate::ModelError;
use crate::anim::{RotationTrack, TransformTrack};

/// A parsed `.kf` file, curves in engine space.
#[derive(Clone, Debug, Default)]
pub struct KeyframeFile {
    /// Root text keys `(time, "Group: Event")`, one entry per line, in file order.
    pub text_keys: Vec<(f32, String)>,
    /// `(bone name, track)` in file order; bones without keys are left out.
    pub tracks: Vec<(String, TransformTrack)>,
}

/// Parses and converts `.kf` bytes.
pub fn load_kf(bytes: &[u8]) -> Result<KeyframeFile, ModelError> {
    let stream = crate::parse(bytes)?;
    let helper = stream
        .roots
        .iter()
        .find_map(|root| stream.objects.get(root.key).and_then(|o| <&NiSequenceStreamHelper>::try_from(o).ok()))
        .ok_or(ModelError::NoRoots)?;
    let net = &helper.base;
    let mut file = KeyframeFile::default();
    let mut names = Vec::new();
    for extra in net.extra_datas(&stream) {
        if let Ok(keys) = <&NiTextKeyExtraData>::try_from(extra) {
            file.text_keys.extend(text_key_lines(keys));
        } else if let Ok(name) = <&NiStringExtraData>::try_from(extra) {
            names.push(name.value.clone());
        }
    }
    // Names and controllers pair up by position (the original engine's rule).
    for (name, controller) in names.into_iter().zip(net.controllers(&stream)) {
        let Ok(keyframe) = <&NiKeyframeController>::try_from(controller) else { continue };
        if let Some(track) = transform_track(&stream, keyframe) {
            file.tracks.push((name, track));
        }
    }
    Ok(file)
}

/// The text-key lines of an `NiTextKeyExtraData`: one key may hold several
/// lines (`"Idle: Stop\r\nIdle2: Start"`).
pub(crate) fn text_key_lines(keys: &NiTextKeyExtraData) -> impl Iterator<Item = (f32, String)> + '_ {
    keys.keys.iter().flat_map(|key| key.value.lines().map(str::trim).filter(|l| !l.is_empty()).map(move |line| (key.time, line.to_owned())))
}

/// The engine-space track of a keyframe controller; `None` without keys.
pub(crate) fn transform_track(stream: &NiStream, keyframe: &NiKeyframeController) -> Option<TransformTrack> {
    let data = stream.get(keyframe.data)?;
    let translation = crate::anim::translation_curve(&data.translations.keys);
    let scale = crate::anim::float_curve(&data.scales.keys);
    let track = TransformTrack {
        translation: (!translation.is_empty()).then_some(translation),
        rotation: RotationTrack::from_keys(&data.rotations.keys),
        scale: (!scale.is_empty()).then_some(scale),
    };
    (track != TransformTrack::default()).then_some(track)
}
