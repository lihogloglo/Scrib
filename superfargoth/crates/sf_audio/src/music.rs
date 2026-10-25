//! Music player (audio.md "Music"): playlists keyed by a generic mood tag ("explore", "battle",
//! whatever the content defines), shuffle without repeats, crossfade on a mood change and the
//! next track when one ends. Pure state machine: the host plays the commands and reports ends.

use std::collections::HashMap;

use crate::{Rng, fade_toward};

/// Seconds of a mood-change crossfade (old track out, new track in).
pub const MUSIC_FADE_S: f32 = 3.0;

/// Identifies one playing track instance.
pub type MusicVoiceId = u32;

/// What the host must do with its music voices.
#[derive(Clone, Debug, PartialEq)]
pub enum MusicCommand {
    /// Start `path` (non-positional, Music bus) at gain `equal_power(level)` from `voices()`.
    Start { voice: MusicVoiceId, path: String },
    /// Stop and drop the voice.
    Stop { voice: MusicVoiceId },
}

#[derive(Clone, Debug)]
struct Playing {
    id: MusicVoiceId,
    level: f32,
    target: f32,
}

#[derive(Clone, Debug, Default)]
struct Playlist {
    tracks: Vec<String>,
    bag: Vec<usize>,
    last: Option<usize>,
}

/// Mood-driven music state machine.
#[derive(Clone, Debug)]
pub struct MusicPlayer {
    playlists: HashMap<String, Playlist>,
    wanted: Option<String>,
    playing: Option<String>,
    voices: Vec<Playing>,
    next_id: MusicVoiceId,
    since_switch_s: f32,
    rng: Rng,
    out: Vec<MusicCommand>,
    /// Crossfade length on a mood change.
    pub crossfade_s: f32,
    /// A mood plays at least this long before another may replace it (stops music flicking
    /// between moods; 0 = switch at once). Silence (`request(None)`) is not held.
    pub min_hold_s: f32,
}

impl MusicPlayer {
    pub fn new(seed: u64) -> Self {
        Self {
            playlists: HashMap::new(),
            wanted: None,
            playing: None,
            voices: Vec::new(),
            next_id: 0,
            since_switch_s: 0.0,
            rng: Rng::new(seed),
            out: Vec::new(),
            crossfade_s: MUSIC_FADE_S,
            min_hold_s: 0.0,
        }
    }

    /// Sets a mood's tracks (VFS paths). Replacing a playlist does not stop what is playing.
    pub fn set_playlist(&mut self, tag: &str, tracks: Vec<String>) {
        self.playlists.insert(tag.to_owned(), Playlist { tracks, bag: Vec::new(), last: None });
    }

    /// The mood the game wants now (`None` = silence). Cheap to call every frame.
    pub fn request(&mut self, tag: Option<&str>) {
        if self.wanted.as_deref() != tag {
            self.wanted = tag.map(str::to_owned);
        }
    }

    /// The mood currently playing (or fading in).
    pub fn playing(&self) -> Option<&str> {
        self.playing.as_deref()
    }

    /// Voices and their linear fade levels (apply `equal_power` for the output gain).
    pub fn voices(&self) -> impl Iterator<Item = (MusicVoiceId, f32)> + '_ {
        self.voices.iter().map(|v| (v.id, v.level))
    }

    /// The host reports that `voice` reached its end: the next track of the same mood starts.
    pub fn finished(&mut self, voice: MusicVoiceId) {
        let Some(i) = self.voices.iter().position(|v| v.id == voice) else { return };
        let was_current = self.voices[i].target > 0.0;
        self.voices.remove(i);
        self.out.push(MusicCommand::Stop { voice });
        if was_current && let Some(tag) = self.playing.clone() {
            self.start(&tag, 1.0);
        }
    }

    /// Advances fades and mood switches by `dt` seconds; returns the commands to play.
    pub fn tick(&mut self, dt: f32) -> Vec<MusicCommand> {
        self.since_switch_s += dt.max(0.0);
        let held = self.playing.is_none() || self.wanted.is_none() || self.since_switch_s >= self.min_hold_s;
        if self.wanted != self.playing && held {
            for v in &mut self.voices {
                v.target = 0.0;
            }
            self.playing.clone_from(&self.wanted);
            self.since_switch_s = 0.0;
            if let Some(tag) = self.wanted.clone() {
                self.start(&tag, 0.0);
            }
        }
        let fade = self.crossfade_s;
        let mut stops = Vec::new();
        self.voices.retain_mut(|v| {
            v.level = fade_toward(v.level, v.target, dt, fade);
            let done = v.level <= 0.0 && v.target <= 0.0;
            if done {
                stops.push(MusicCommand::Stop { voice: v.id });
            }
            !done
        });
        self.out.extend(stops);
        std::mem::take(&mut self.out)
    }

    fn start(&mut self, tag: &str, level: f32) {
        let Some(list) = self.playlists.get_mut(tag) else { return };
        let n = list.tracks.len();
        if n == 0 {
            return;
        }
        if list.bag.is_empty() {
            // Shuffle bag (Fisher-Yates), never opening with the track that just played.
            list.bag = (0..n).collect();
            for i in (1..n).rev() {
                let j = self.rng.index(i + 1);
                list.bag.swap(i, j);
            }
            if n > 1 && list.bag.last().copied() == list.last {
                list.bag.swap(0, n - 1);
            }
        }
        let Some(track) = list.bag.pop() else { return };
        list.last = Some(track);
        let id = self.next_id;
        self.next_id = self.next_id.wrapping_add(1);
        self.voices.push(Playing { id, level, target: 1.0 });
        self.out.push(MusicCommand::Start { voice: id, path: list.tracks[track].clone() });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn player() -> MusicPlayer {
        let mut p = MusicPlayer::new(5);
        p.set_playlist("explore", vec!["e1.mp3".into(), "e2.mp3".into(), "e3.mp3".into()]);
        p.set_playlist("battle", vec!["b1.mp3".into(), "b2.mp3".into()]);
        p
    }

    fn started(cmds: &[MusicCommand]) -> Vec<(MusicVoiceId, &str)> {
        cmds.iter()
            .filter_map(|c| match c {
                MusicCommand::Start { voice, path } => Some((*voice, path.as_str())),
                MusicCommand::Stop { .. } => None,
            })
            .collect()
    }

    #[test]
    fn mood_change_crossfades_old_out_and_new_in() {
        let mut p = player();
        p.request(Some("explore"));
        let c = p.tick(0.0);
        let s = started(&c);
        assert_eq!(s.len(), 1);
        assert!(s[0].1.starts_with('e'));
        let explore = s[0].0;
        p.tick(MUSIC_FADE_S);
        assert_eq!(p.voices().collect::<Vec<_>>(), vec![(explore, 1.0)]);

        p.request(Some("battle"));
        let c = p.tick(0.0);
        let s = started(&c);
        assert!(s.len() == 1 && s[0].1.starts_with('b'));
        let battle = s[0].0;
        assert_eq!(p.playing(), Some("battle"));
        p.tick(MUSIC_FADE_S / 2.0);
        let levels: HashMap<_, _> = p.voices().collect();
        assert!((levels[&explore] - 0.5).abs() < 1e-5 && (levels[&battle] - 0.5).abs() < 1e-5);
        let c = p.tick(MUSIC_FADE_S / 2.0);
        assert!(c.contains(&MusicCommand::Stop { voice: explore }));
        assert_eq!(p.voices().collect::<Vec<_>>(), vec![(battle, 1.0)]);
    }

    #[test]
    fn track_end_starts_the_next_without_repeats() {
        let mut p = player();
        p.request(Some("explore"));
        let mut cmds = p.tick(0.0);
        let mut order = Vec::new();
        for _ in 0..9 {
            let (voice, path) = started(&cmds)[0];
            order.push(path.to_owned());
            p.finished(voice);
            cmds = p.tick(0.1);
            assert!(cmds.contains(&MusicCommand::Stop { voice }));
        }
        for w in order.windows(2) {
            assert_ne!(w[0], w[1], "{order:?}");
        }
        // Each bag of three holds every track once.
        for bag in order.chunks(3) {
            let mut b = bag.to_vec();
            b.sort();
            assert_eq!(b, ["e1.mp3", "e2.mp3", "e3.mp3"]);
        }
    }

    #[test]
    fn min_hold_delays_a_switch_and_silence_fades_out() {
        let mut p = player();
        p.min_hold_s = 10.0;
        p.request(Some("battle"));
        p.tick(0.0);
        p.request(Some("explore"));
        assert!(started(&p.tick(5.0)).is_empty());
        assert_eq!(p.playing(), Some("battle"));
        assert_eq!(started(&p.tick(5.0)).len(), 1);
        assert_eq!(p.playing(), Some("explore"));
        p.request(None);
        p.tick(0.0);
        assert_eq!(p.playing(), None);
        p.tick(MUSIC_FADE_S);
        assert_eq!(p.voices().count(), 0);
    }

    #[test]
    fn unknown_or_empty_mood_is_silence() {
        let mut p = player();
        p.request(Some("tavern"));
        assert!(p.tick(1.0).is_empty());
        assert_eq!(p.voices().count(), 0);
    }
}
