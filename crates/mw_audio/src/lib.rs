//! Morrowind sound content for `sf_audio` (superfargoth audio.md "Content adapter"). Bevy-free:
//! records in, `sf_audio` data out; the app's `audio.rs` inserts it and writes `PlaySound`.
//!
//! - Every `SOUN` is a bank entry (its file through the VFS, `DATA` volume / 255) and a
//!   one-variant cue of the same id, positional with the record's range bytes as metres
//!   ([`RangeScale`]: the `fAudio*` game settings, as the original engine and `OpenMW` scale them).
//! - `SNDG` creature sounds are cues `"<creature>/<kind>"` (all generators of one creature and
//!   kind are the cue's variants); `"/<kind>"` holds the generic ones ([`creature_cue`]).
//! - Doors: `DOOR` open / close sound ids ([`DoorSounds`]).
//! - Loop emitters ([`MwSounds::emitters`]): lights with a sound, and activators whose script is
//!   the vanilla "sound activator" idiom (`PlayLoopSound3DVP "Cave Drip", 1.0, 1.0` behind
//!   `GetSoundPlaying` / `CellChanged` guards, [`loop_script`]). Named scaffold: removed when the
//!   script runtime plays `PlayLoopSound3D*` itself.
//! - Footsteps: vanilla picks the sound by the boots' armour weight class ([`Footwear`]).
//! - Music: `Music/Explore` and `Music/Battle` as playlists, listed through the VFS so mods add
//!   tracks ([`playlists`]); the battle rule is [`MusicMood`].
//! - NPC voices ([`voice`]): voice INFOs picked with `mw_rpg`'s filters, greetings and idle
//!   chatter timing, the lip-sync envelope; every INFO sound is a Voice-bus cue.

use std::collections::HashMap;

use sf_audio::{Attenuation, Bus, CueBank, FootstepMap, SoundBank, SoundFile, SoundGroup};
use sf_records::{RecordDb, RecordId};
use tes3::esp::{Activator, Door, GameSetting, GameSettingValue, Light, Script, Sound, SoundGen, SoundGenType};

pub mod voice;

#[cfg(test)]
mod tests;
#[cfg(test)]
mod voice_tests;

/// Metres per Morrowind unit (64 units = 1 yard; `mw_world::convert::UNIT_METERS_F64`).
pub const UNIT_M: f32 = 0.9144 / 64.0;

/// Non-positional sounds of the menus and books (played on the UI bus).
pub const UI_SOUNDS: &[&str] = &["menu click", "menu size", "book open", "book close", "book page", "book page2", "scroll"];

/// The UI click (`Menu Click`).
pub const MENU_CLICK: &str = "menu click";

/// `SOUN` range bytes to an attenuation in metres. Both bytes 0 means the default range; the
/// minimum scales by `fAudioMinDistanceMult`, the maximum by `fAudioMaxDistanceMult`, then units
/// to metres (`OpenMW`'s reading of the same settings, used as behaviour reference). Vanilla
/// values: a sound with no range is full within 1.4 m and gone by 28.6 m; `Fire` (2, 15) 0.6 m
/// and 10.7 m.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RangeScale {
    pub min_mult: f32,
    pub max_mult: f32,
    pub default_min: f32,
    pub default_max: f32,
}

impl Default for RangeScale {
    /// The `Morrowind.esm` values.
    fn default() -> Self {
        Self { min_mult: 20.0, max_mult: 50.0, default_min: 5.0, default_max: 40.0 }
    }
}

impl RangeScale {
    /// The game settings, each falling back to the vanilla value.
    pub fn from_db(db: &RecordDb) -> Self {
        let d = Self::default();
        Self {
            min_mult: gmst(db, "fAudioMinDistanceMult", d.min_mult),
            max_mult: gmst(db, "fAudioMaxDistanceMult", d.max_mult),
            default_min: gmst(db, "fAudioDefaultMinDistance", d.default_min),
            default_max: gmst(db, "fAudioDefaultMaxDistance", d.default_max),
        }
    }

    pub fn attenuation(&self, range: (u8, u8)) -> Attenuation {
        let (min, max) =
            if range == (0, 0) { (self.default_min, self.default_max) } else { (f32::from(range.0), f32::from(range.1)) };
        let min = (min * self.min_mult).max(1.0);
        let max = (max * self.max_mult).max(min);
        Attenuation::new(min * UNIT_M, max * UNIT_M)
    }
}

#[allow(clippy::cast_precision_loss)]
fn gmst(db: &RecordDb, id: &str, fallback: f32) -> f32 {
    match db.get::<GameSetting>(&RecordId::new(id)).map(|g| &g.value) {
        Some(GameSettingValue::Float(v)) => *v,
        Some(GameSettingValue::Integer(v)) => *v as f32,
        _ => fallback,
    }
}

/// `Sound/<path>` with forward slashes (the VFS path of a `SOUN` file before alternates).
pub fn sound_file_path(sound_path: &str) -> String {
    format!("Sound/{}", sound_path.trim_start_matches(['\\', '/']).replace('\\', "/"))
}

/// A door's sound ids (either may be missing).
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct DoorSounds {
    pub open: Option<RecordId>,
    pub close: Option<RecordId>,
}

/// Everything the app inserts.
#[derive(Clone, Debug, Default)]
pub struct MwSounds {
    pub bank: SoundBank,
    pub cues: CueBank,
    /// Door base id -> its sounds.
    pub doors: HashMap<RecordId, DoorSounds>,
    /// Base record id (light, sound activator) -> the cue it loops while loaded.
    pub emitters: HashMap<RecordId, RecordId>,
    pub footsteps: FootstepMap,
    /// How many INFO sound files became Voice-bus cues ([`voice::add_voices`]).
    pub voices: usize,
}

/// Builds the bank and cues from the records. `resolve` maps a raw VFS path to the file the
/// asset server loads (mods' replacements and wav/mp3 alternates), `None` drops the sound.
pub fn build(db: &RecordDb, resolve: impl Fn(&str) -> Option<String>) -> MwSounds {
    let scale = RangeScale::from_db(db);
    let mut out = MwSounds::default();
    let mut ranges = HashMap::new();
    for (id, s) in db.iter::<Sound>() {
        let Some(path) = resolve(&sound_file_path(&s.sound_path)) else { continue };
        out.bank.0.insert(id.clone(), SoundFile { path, volume: f32::from(s.data.volume) / 255.0 });
        let att = scale.attenuation(s.data.range);
        ranges.insert(id.clone(), att);
        let ui = UI_SOUNDS.contains(&id.as_str());
        let group = SoundGroup {
            bus: if ui { Bus::Ui } else { Bus::Sfx },
            attenuation: if ui { None } else { Some(att) },
            ..SoundGroup::of([id.clone()])
        };
        out.cues.insert(id.clone(), group);
    }
    add_creature_cues(db, &ranges, &mut out.cues);
    for (id, d) in db.iter::<Door>() {
        let sound = |s: &str| (!s.is_empty()).then(|| RecordId::new(s));
        let sounds = DoorSounds { open: sound(&d.open_sound), close: sound(&d.close_sound) };
        if sounds != DoorSounds::default() {
            out.doors.insert(id.clone(), sounds);
        }
    }
    add_emitters(db, &ranges, &mut out);
    out.voices = voice::add_voices(db, &resolve, &mut out.bank, &mut out.cues);
    out.footsteps = footstep_map();
    out
}

/// `SNDG` kind as a cue suffix.
pub fn kind_name(kind: SoundGenType) -> &'static str {
    match kind {
        SoundGenType::LeftFoot => "left",
        SoundGenType::RightFoot => "right",
        SoundGenType::SwimLeft => "swimleft",
        SoundGenType::SwimRight => "swimright",
        SoundGenType::Moan => "moan",
        SoundGenType::Roar => "roar",
        SoundGenType::Scream => "scream",
        SoundGenType::Land => "land",
    }
}

fn add_creature_cues(db: &RecordDb, ranges: &HashMap<RecordId, Attenuation>, cues: &mut CueBank) {
    let mut groups: HashMap<String, Vec<RecordId>> = HashMap::new();
    for (_, g) in db.iter::<SoundGen>() {
        if g.sound.is_empty() {
            continue;
        }
        let key = format!("{}/{}", g.creature.to_ascii_lowercase(), kind_name(g.sound_gen_type));
        let sounds = groups.entry(key).or_default();
        let sound = RecordId::new(&g.sound);
        if !sounds.contains(&sound) {
            sounds.push(sound);
        }
    }
    for (key, mut sounds) in groups {
        sounds.sort();
        let attenuation = sounds.first().and_then(|s| ranges.get(s)).copied().or_else(|| Some(RangeScale::default().attenuation((0, 0))));
        cues.insert(RecordId::new(&key), SoundGroup { attenuation, max_instances: 4, ..SoundGroup::of(sounds) });
    }
}

/// The cue for a creature's `kind` sound: its own generators, then those of the creature it
/// takes its sounds from (`CNAM`, `sound_from`), then the generic ones (`SNDG` without a
/// creature). `has` says whether a cue exists.
pub fn creature_cue(creature: &str, sound_from: &str, kind: SoundGenType, has: impl Fn(&RecordId) -> bool) -> Option<RecordId> {
    let kind = kind_name(kind);
    [creature, sound_from]
        .into_iter()
        .filter(|c| !c.is_empty())
        .chain([""])
        .map(|c| RecordId::new(&format!("{}/{kind}", c.to_ascii_lowercase())))
        .find(|id| has(id))
}

/// A sound activator script's loop: `(sound, volume, pitch)`. Only scripts that do nothing else
/// (declarations, `if`s on `CellChanged` / `GetSoundPlaying` / `MenuMode`, one
/// `PlayLoopSound3D[VP]`) qualify, so quest logic is never pre-empted.
pub fn loop_script(text: &str) -> Option<(RecordId, f32, f32)> {
    let mut found = None;
    for raw in text.lines() {
        let line = raw.split(';').next().unwrap_or("").trim().to_ascii_lowercase();
        let Some(word) = line.split([' ', '\t', '(', ',']).next() else { continue };
        match word {
            "" | "begin" | "end" | "endif" | "else" | "return" | "short" | "long" | "float" => {}
            "if" | "elseif" => {
                if !["cellchanged", "getsoundplaying", "menumode"].iter().any(|w| line.contains(w)) {
                    return None;
                }
            }
            "playloopsound3d" | "playloopsound3dvp" => {
                if found.is_some() {
                    return None;
                }
                let mut quoted = line.splitn(3, '"');
                quoted.next();
                let sound = quoted.next()?;
                let mut args = quoted.next().unwrap_or("").split([',', ' ', '\t']).filter_map(|a| a.parse::<f32>().ok());
                let (volume, pitch) = if word == "playloopsound3dvp" {
                    (args.next().unwrap_or(1.0), args.next().unwrap_or(1.0))
                } else {
                    (1.0, 1.0)
                };
                found = Some((RecordId::new(sound), volume, pitch));
            }
            _ => return None,
        }
    }
    found
}

fn add_emitters(db: &RecordDb, ranges: &HashMap<RecordId, Attenuation>, out: &mut MwSounds) {
    let loop_group = |sound: &RecordId, volume: f32, pitch: f32| SoundGroup {
        bus: Bus::Ambience,
        attenuation: ranges.get(sound).copied(),
        volume: [volume; 2],
        pitch: [pitch; 2],
        ..SoundGroup::of([sound.clone()])
    };
    for (id, light) in db.iter::<Light>() {
        if light.sound.is_empty() {
            continue;
        }
        let sound = RecordId::new(&light.sound);
        if !out.bank.0.contains_key(&sound) {
            continue;
        }
        // The light's loop is the sound's own cue on the Ambience bus.
        let cue = RecordId::new(&format!("loop/{}", sound.as_str()));
        out.cues.insert(cue.clone(), loop_group(&sound, 1.0, 1.0));
        out.emitters.insert(id.clone(), cue);
    }
    let mut scripts: HashMap<RecordId, RecordId> = HashMap::new();
    for (id, script) in db.iter::<Script>() {
        let Some((sound, volume, pitch)) = loop_script(&script.text) else { continue };
        if !out.bank.0.contains_key(&sound) {
            continue;
        }
        let cue = RecordId::new(&format!("script/{}", id.as_str()));
        out.cues.insert(cue.clone(), loop_group(&sound, volume, pitch));
        scripts.insert(id.clone(), cue);
    }
    for (id, a) in db.iter::<Activator>() {
        if let Some(cue) = (!a.script.is_empty()).then(|| RecordId::new(&a.script)).and_then(|s| scripts.get(&s)) {
            out.emitters.insert(id.clone(), cue.clone());
        }
    }
}

/// The boots' weight class (vanilla footstep rule); no boots is `Bare`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Footwear {
    #[default]
    Bare,
    Light,
    Medium,
    Heavy,
}

impl Footwear {
    /// The footstep surface tag (`FootstepMap`).
    pub fn surface(self) -> &'static str {
        match self {
            Self::Bare => "bare",
            Self::Light => "light",
            Self::Medium => "medium",
            Self::Heavy => "heavy",
        }
    }
}

/// Footstep event names: `left` / `right` (animation `SoundGen: Left/Right` text keys later).
pub fn footstep_map() -> FootstepMap {
    let mut map = FootstepMap::default();
    for (surface, stem) in [("bare", "bare"), ("light", "light"), ("medium", "med"), ("heavy", "heavy"), ("water", "water")] {
        map.insert("left", Some(surface), RecordId::new(&format!("foot{stem}left")));
        map.insert("right", Some(surface), RecordId::new(&format!("foot{stem}right")));
    }
    map.insert("left", None, RecordId::new("footbareleft"));
    map.insert("right", None, RecordId::new("footbareright"));
    map
}

/// Walk stride 0.7 m at 1.5 m/s or slower, run stride 1.1 m at 4.5 m/s or faster (between: linear).
pub fn stride_m(speed_mps: f32) -> f32 {
    let t = ((speed_mps - 1.5) / 3.0).clamp(0.0, 1.0);
    0.7 + 0.4 * t
}

/// Music playlist tags and their folders.
pub const MUSIC_DIRS: [(&str, &str); 2] = [("explore", "Music/Explore"), ("battle", "Music/Battle")];

/// The playlists: each folder's mp3/wav files from `list` (VFS paths, sorted).
pub fn playlists(list: impl Fn(&str) -> Vec<String>) -> Vec<(&'static str, Vec<String>)> {
    MUSIC_DIRS
        .iter()
        .map(|(tag, dir)| {
            let mut tracks: Vec<String> = list(dir)
                .into_iter()
                .filter(|p| {
                    std::path::Path::new(p)
                        .extension()
                        .is_some_and(|e| e.eq_ignore_ascii_case("mp3") || e.eq_ignore_ascii_case("wav"))
                })
                .collect();
            tracks.sort();
            (*tag, tracks)
        })
        .collect()
}

/// [`playlists`] over a VFS.
pub fn vfs_playlists(vfs: &sf_vfs::Vfs) -> Vec<(&'static str, Vec<String>)> {
    playlists(|dir| {
        vfs.read_dir(dir)
            .into_iter()
            .filter_map(|e| match e {
                sf_vfs::DirEntry::File(p) => Some(p.as_str().to_owned()),
                sf_vfs::DirEntry::Dir(_) => None,
            })
            .collect()
    })
}

/// [`build`] over a VFS (wav or mp3 alternates, as the weather bank).
pub fn vfs_sounds(db: &RecordDb, vfs: &sf_vfs::Vfs) -> MwSounds {
    build(db, |raw| {
        let path = sf_vfs::VfsPath::new(raw).ok()?;
        vfs.resolve_preferring(&path, &["mp3", "wav"]).map(|(p, _)| p.as_str().to_owned())
    })
}

/// The vanilla-like battle music rule: battle while any hostile is in combat with the player,
/// held `battle_hold_s` after the last one stops, else explore. The combat flag is a hook the
/// combat systems set (Phase 3); until then it stays false and music explores.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MusicMood {
    pub battle_hold_s: f32,
    since_combat_s: f32,
}

impl Default for MusicMood {
    fn default() -> Self {
        Self { battle_hold_s: 5.0, since_combat_s: f32::INFINITY }
    }
}

impl MusicMood {
    /// Advances by `dt` with this frame's combat flag; returns the playlist tag to request.
    pub fn update(&mut self, dt: f32, hostile_in_combat: bool) -> &'static str {
        self.since_combat_s = if hostile_in_combat { 0.0 } else { self.since_combat_s + dt.max(0.0) };
        if self.since_combat_s <= self.battle_hold_s { "battle" } else { "explore" }
    }
}
