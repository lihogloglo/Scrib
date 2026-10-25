//! Ambience layers (audio.md "Ambience"): named layers of looping beds, each heard outdoors,
//! indoors or both. The listener's eased outdoor exposure weighs them, so walking through a
//! door crossfades the exterior bed into the interior one. `reconcile_loops` turns the wanted
//! set into start/retarget/stop changes for the host's loop voices.

use std::collections::BTreeMap;

use sf_records::RecordId;

/// Where a layer is heard.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum Space {
    Outdoor,
    Indoor,
    #[default]
    Any,
}

impl Space {
    /// The layer's weight at outdoor exposure `outdoor` (1 = open sky, 0 = inside).
    pub fn weight(self, outdoor: f32) -> f32 {
        let o = outdoor.clamp(0.0, 1.0);
        match self {
            Space::Outdoor => o,
            Space::Indoor => 1.0 - o,
            Space::Any => 1.0,
        }
    }
}

/// Layer tag (host-chosen: "region", "cell", "weather-bed", ...) -> looping sounds.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct AmbienceLayers {
    layers: BTreeMap<String, Vec<(RecordId, f32, Space)>>,
}

impl AmbienceLayers {
    /// Replaces a layer's loops: `(sound, volume, space)`.
    pub fn set_layer(&mut self, tag: &str, loops: Vec<(RecordId, f32, Space)>) {
        self.layers.insert(tag.to_owned(), loops);
    }

    pub fn clear_layer(&mut self, tag: &str) {
        self.layers.remove(tag);
    }

    /// The loops to hear at exposure `outdoor`, with their target volumes; a sound in several
    /// layers plays once at its loudest. Silent entries are left out. Sorted by id.
    pub fn desired(&self, outdoor: f32) -> Vec<(RecordId, f32)> {
        let mut out: BTreeMap<&RecordId, f32> = BTreeMap::new();
        for (sound, volume, space) in self.layers.values().flatten() {
            let v = volume.max(0.0) * space.weight(outdoor);
            if v > 0.0 {
                let e = out.entry(sound).or_insert(0.0);
                *e = e.max(v);
            }
        }
        out.into_iter().map(|(s, v)| (s.clone(), v)).collect()
    }
}

/// A change to the host's loop voices.
#[derive(Clone, Debug, PartialEq)]
pub enum LoopChange {
    /// Start a loop at silence and fade it toward `target`.
    Start { sound: RecordId, target: f32 },
    /// Fade a playing loop toward a new `target`.
    Retarget { sound: RecordId, target: f32 },
    /// Fade a loop out (target 0); the host drops it once silent.
    Stop { sound: RecordId },
}

/// Diffs the playing loops (`sound`, current target) against the wanted set. Targets within
/// `1e-3` count as unchanged; loops already fading out (target 0) and not wanted are left alone.
pub fn reconcile_loops(playing: &[(RecordId, f32)], wanted: &[(RecordId, f32)]) -> Vec<LoopChange> {
    let mut out = Vec::new();
    for (sound, target) in wanted {
        match playing.iter().find(|(s, _)| s == sound) {
            None => out.push(LoopChange::Start { sound: sound.clone(), target: *target }),
            Some((_, t)) if (t - target).abs() > 1e-3 => {
                out.push(LoopChange::Retarget { sound: sound.clone(), target: *target });
            }
            Some(_) => {}
        }
    }
    for (sound, t) in playing {
        if *t > 0.0 && !wanted.iter().any(|(s, _)| s == sound) {
            out.push(LoopChange::Stop { sound: sound.clone() });
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn id(s: &str) -> RecordId {
        RecordId::new(s)
    }

    #[test]
    fn exposure_crossfades_outdoor_and_indoor_layers() {
        let mut a = AmbienceLayers::default();
        a.set_layer("region", vec![(id("wind"), 0.8, Space::Outdoor)]);
        a.set_layer("cell", vec![(id("cave_drips"), 0.5, Space::Indoor), (id("fire"), 0.4, Space::Any)]);
        assert_eq!(a.desired(1.0), vec![(id("fire"), 0.4), (id("wind"), 0.8)]);
        assert_eq!(a.desired(0.0), vec![(id("cave_drips"), 0.5), (id("fire"), 0.4)]);
        let half = a.desired(0.5);
        assert_eq!(half.len(), 3);
        assert!(half.iter().any(|(s, v)| *s == id("wind") && (v - 0.4).abs() < 1e-6));
        a.clear_layer("cell");
        assert_eq!(a.desired(0.0), vec![]);
    }

    #[test]
    fn same_sound_in_two_layers_plays_once_at_its_loudest() {
        let mut a = AmbienceLayers::default();
        a.set_layer("x", vec![(id("water"), 0.3, Space::Any)]);
        a.set_layer("y", vec![(id("water"), 0.6, Space::Any)]);
        assert_eq!(a.desired(1.0), vec![(id("water"), 0.6)]);
    }

    #[test]
    fn reconcile_starts_retargets_and_stops() {
        let playing = vec![(id("a"), 0.5), (id("b"), 0.5), (id("c"), 0.0), (id("d"), 0.7)];
        let wanted = vec![(id("a"), 0.5), (id("b"), 0.2), (id("e"), 1.0)];
        let ch = reconcile_loops(&playing, &wanted);
        assert_eq!(
            ch,
            vec![
                LoopChange::Retarget { sound: id("b"), target: 0.2 },
                LoopChange::Start { sound: id("e"), target: 1.0 },
                LoopChange::Stop { sound: id("d") },
            ]
        );
    }
}
