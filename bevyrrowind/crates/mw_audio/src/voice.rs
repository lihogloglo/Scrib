//! NPC voices (audio.md "Content adapter", ROADMAP 2.6): Morrowind's voice INFOs, picked with
//! `mw_rpg`'s INFO filters and played positionally on the Voice bus.
//!
//! - **Lines**: the `Voice` dialogue topics ([`VoiceTopic`]: Alarm, Attack, Flee, Hello, Hit,
//!   Idle, Intruder, Thief) and the greetings are ordinary DIAL/INFO; the first INFO the
//!   speaker passes (speaker id, race, sex, class, faction, cell, select conditions, in the
//!   topic's priority order) is the line, its `SNAM` sound field the file ([`select`],
//!   [`select_greeting`]). Every INFO sound is a bank entry and a one-variant cue on the Voice
//!   bus ([`voice_cue`], [`add_voices`]) with the voice range settings.
//! - **When** ([`VoiceDirector`], pure): "Hello" once on approach within the NPC's AI `Hello`
//!   times `iGreetDistanceMultiplier` units, re-armed only after the player leaves
//!   `fGreetDistanceReset` units; for `iGreetDuration` seconds after it the NPC is "greeting"
//!   (the host turns them to the player). Idle chatter on a timer from `fVoiceIdleOdds`, only
//!   while idle, aware of the player and within [`VoiceRules::idle_range_m`]. One line per actor
//!   at a time ([`VoiceDirector::begin`]): a bark while one plays is dropped, as in the original.
//!   Combat and crime barks (Attack, Hit, Flee, Thief, Intruder, Alarm) are host requests: the
//!   Phase 3 combat and crime systems call [`VoiceDirector::begin`] then [`select`].
//! - **Lip-sync hook** ([`Envelope`]): the playing line's loudness (RMS per 50 ms window,
//!   normalised to its loudest window), sampled at the playback time; the app puts the value on
//!   the speaker as a component for the face animation to read.
//!
//! Behaviour reference: `OpenMW` (`DialogueManager::say`, `Actors::updateGreetingState`,
//! `AiWander::playIdleDialogueRandomly`, `Sound_Loudness`) and the game settings' vanilla values.

use std::collections::HashMap;

use mw_rpg::{ActorFacts, GlobalNumber, MwDialogue, MwDialogueRules, MwView};
use sf_audio::{Attenuation, Bus, CueBank, DuckRule, DuckTrigger, Rng, SoundBank, SoundFile, SoundGroup};
use sf_records::{RecordDb, RecordId};
use sf_rpg::{ActorKey, ConversationCtx, PlayerId};
use tes3::esp::{DialogueInfo, GlobalValue, GlobalVariable};

use crate::{RangeScale, UNIT_M, gmst, sound_file_path};

/// The `Voice` dialogue topics (DIAL type 2).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum VoiceTopic {
    Alarm,
    Attack,
    Flee,
    Hello,
    Hit,
    Idle,
    Intruder,
    Thief,
}

impl VoiceTopic {
    pub const ALL: [Self; 8] =
        [Self::Alarm, Self::Attack, Self::Flee, Self::Hello, Self::Hit, Self::Idle, Self::Intruder, Self::Thief];

    /// The DIAL id.
    pub fn name(self) -> &'static str {
        match self {
            Self::Alarm => "alarm",
            Self::Attack => "attack",
            Self::Flee => "flee",
            Self::Hello => "hello",
            Self::Hit => "hit",
            Self::Idle => "idle",
            Self::Intruder => "intruder",
            Self::Thief => "thief",
        }
    }

    pub fn topic(self) -> RecordId {
        RecordId::new(self.name())
    }
}

/// A picked line: the INFO, its cue (bank entry of its sound file) and its subtitle.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct VoiceLine {
    pub topic: RecordId,
    pub response: RecordId,
    /// [`voice_cue`] of the INFO's sound field.
    pub cue: RecordId,
    /// The INFO text with macros expanded (subtitles).
    pub subtitle: String,
}

/// The cue (and bank id) of an INFO sound field: `voice/<path>`, lower case, forward slashes.
pub fn voice_cue(sound_path: &str) -> RecordId {
    RecordId::new(&format!("voice/{}", sound_path.trim_start_matches(['\\', '/']).replace('\\', "/").to_ascii_lowercase()))
}

/// The first INFO under `topic` that `ctx`'s speaker passes, if it has a sound. An INFO that
/// passes without a sound is still the pick (the speaker stays silent), as in the original.
pub fn select(dialogue: &MwDialogue, view: &dyn MwView, ctx: &ConversationCtx, topic: VoiceTopic) -> Option<VoiceLine> {
    let rules = MwDialogueRules::new(dialogue, view);
    let topic = topic.topic();
    let response = dialogue.db.select(&rules, ctx, &topic)?;
    line(dialogue, &rules, ctx, topic, response)
}

/// The greeting a conversation opens with, if its INFO is voiced.
pub fn select_greeting(dialogue: &MwDialogue, view: &dyn MwView, ctx: &ConversationCtx) -> Option<VoiceLine> {
    let rules = MwDialogueRules::new(dialogue, view);
    let (topic, response) = dialogue.db.select_greeting(&rules, ctx)?;
    line(dialogue, &rules, ctx, topic, response)
}

fn line(
    dialogue: &MwDialogue,
    rules: &MwDialogueRules<'_>,
    ctx: &ConversationCtx,
    topic: RecordId,
    response: RecordId,
) -> Option<VoiceLine> {
    let info = dialogue.info(&response)?;
    if info.sound_path.trim().is_empty() {
        return None;
    }
    Some(VoiceLine { cue: voice_cue(&info.sound_path), subtitle: rules.expand(ctx, &info.text), topic, response })
}

/// Named scaffold `BarkView`: the world as barks see it until the game's `MwView` exists
/// (ROADMAP 2.4, dialogue wiring): speaker and player facts, the `GLOB` start values, no
/// script locals (`NotLocal` passes, `Local` fails), every quest at stage 0, no items or
/// deaths, the speaker's base disposition, unimplemented functions neutral (0). Removal path:
/// the host passes its own `MwView` to [`select`] instead.
pub struct BarkView<'a> {
    pub records: &'a RecordDb,
    pub speaker: ActorFacts,
    pub player: &'a ActorFacts,
    pub globals: &'a HashMap<RecordId, GlobalNumber>,
    pub disposition: i32,
}

/// The `GLOB` records' start values ([`BarkView::globals`]).
pub fn start_globals(db: &RecordDb) -> HashMap<RecordId, GlobalNumber> {
    db.iter::<GlobalVariable>()
        .map(|(id, g)| {
            let v = match g.value {
                GlobalValue::Float(f) => GlobalNumber::Float(f),
                GlobalValue::Long(l) => GlobalNumber::Int(l),
                GlobalValue::Short(s) => GlobalNumber::Int(i32::from(s)),
            };
            (id.clone(), v)
        })
        .collect()
}

impl MwView for BarkView<'_> {
    fn speaker(&self, _: &ConversationCtx) -> Option<&ActorFacts> {
        Some(&self.speaker)
    }
    fn player(&self, _: PlayerId) -> Option<&ActorFacts> {
        Some(self.player)
    }
    fn player_rank(&self, _: PlayerId, _: &RecordId) -> Option<i32> {
        None
    }
    fn global(&self, id: &RecordId) -> Option<GlobalNumber> {
        self.globals.get(id).copied()
    }
    fn local(&self, _: ActorKey, _: &RecordId) -> Option<f32> {
        None
    }
    fn quest_stage(&self, _: &RecordId) -> i32 {
        0
    }
    fn item_count(&self, _: PlayerId, _: &RecordId) -> u64 {
        0
    }
    fn dead_count(&self, _: &RecordId) -> i32 {
        0
    }
    fn disposition(&self, _: &ConversationCtx) -> i32 {
        self.disposition
    }
    fn rank_name(&self, faction: &RecordId, rank: i32) -> Option<String> {
        mw_rpg::rank_name(self.records, faction, rank)
    }
}

/// Voice range: `fAudioVoiceDefaultMinDistance` / `fAudioVoiceDefaultMaxDistance` (units, before
/// the `fAudio*DistanceMult` scales of [`RangeScale`]). Vanilla: full within 2.9 m, silent from 43 m (Morrowind.esm: 10 / 60).
pub fn voice_attenuation(db: &RecordDb) -> Attenuation {
    let scale = RangeScale::from_db(db);
    let min = gmst(db, "fAudioVoiceDefaultMinDistance", 10.0);
    let max = gmst(db, "fAudioVoiceDefaultMaxDistance", 60.0);
    let min = (min * scale.min_mult).max(1.0);
    Attenuation::new(min * UNIT_M, (max * scale.max_mult).max(min) * UNIT_M)
}

/// Adds every INFO sound to the bank and as a cue on the Voice bus (positional, voice range,
/// one instance per file). `resolve` as in [`crate::build`]; unresolved files are skipped.
pub fn add_voices(db: &RecordDb, resolve: impl Fn(&str) -> Option<String>, bank: &mut SoundBank, cues: &mut CueBank) -> usize {
    let attenuation = voice_attenuation(db);
    let mut added = 0;
    for (_, info) in db.iter::<DialogueInfo>() {
        if info.sound_path.trim().is_empty() {
            continue;
        }
        let cue = voice_cue(&info.sound_path);
        if bank.0.contains_key(&cue) {
            continue;
        }
        let Some(path) = resolve(&sound_file_path(&info.sound_path)) else { continue };
        bank.0.insert(cue.clone(), SoundFile { path, volume: 1.0 });
        cues.insert(
            cue.clone(),
            SoundGroup { bus: Bus::Voice, attenuation: Some(attenuation), max_instances: 1, ..SoundGroup::of([cue]) },
        );
        added += 1;
    }
    added
}

/// Speech over music: the music bus drops 8 dB while a voice line sounds (0.25 s in, 1 s out).
pub fn ducking() -> Vec<DuckRule> {
    vec![DuckRule { trigger: DuckTrigger::Bus(Bus::Voice), target: Bus::Music, depth_db: 8.0, attack_s: 0.25, release_s: 1.0 }]
}

/// When NPCs speak on their own (metres and seconds).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct VoiceRules {
    /// Hello distance per AI `Hello` point (`iGreetDistanceMultiplier`, 6 units).
    pub greet_m_per_hello: f32,
    /// The player must go this far before the NPC greets again (`fGreetDistanceReset`, 512 units).
    pub greet_reset_m: f32,
    /// How long the NPC keeps facing the player after a greeting (`iGreetDuration`, 4 s).
    pub greet_duration_s: f32,
    /// `fVoiceIdleOdds` (vanilla 10).
    pub idle_odds: f32,
    /// Idle lines only within this distance of the player (3000 units, `OpenMW`'s choice).
    pub idle_range_m: f32,
}

impl Default for VoiceRules {
    /// The `Morrowind.esm` values.
    fn default() -> Self {
        Self {
            greet_m_per_hello: 6.0 * UNIT_M,
            greet_reset_m: 512.0 * UNIT_M,
            greet_duration_s: 4.0,
            idle_odds: 10.0,
            idle_range_m: 3000.0 * UNIT_M,
        }
    }
}

impl VoiceRules {
    pub fn from_db(db: &RecordDb) -> Self {
        let d = Self::default();
        Self {
            greet_m_per_hello: gmst(db, "iGreetDistanceMultiplier", 6.0) * UNIT_M,
            greet_reset_m: gmst(db, "fGreetDistanceReset", 512.0) * UNIT_M,
            greet_duration_s: gmst(db, "iGreetDuration", 4.0),
            idle_odds: gmst(db, "fVoiceIdleOdds", d.idle_odds),
            idle_range_m: d.idle_range_m,
        }
    }

    /// Greeting distance for an AI `Hello` value (0 never greets).
    pub fn hello_m(&self, hello: i16) -> f32 {
        f32::from(hello.max(0)) * self.greet_m_per_hello
    }

    /// Idle lines per second for an idle NPC in range. The original rolled `fVoiceIdleOdds` in
    /// 10000 every frame (frame-rate dependent); this is its rate at 60 fps as `OpenMW`
    /// calibrated it (0.006 per second, a line every ~2.8 minutes on average), as a Poisson rate.
    pub fn idle_rate(&self) -> f32 {
        self.idle_odds.max(0.0) * 6.0 / 10_000.0
    }
}

/// What the host knows about one loaded actor this frame.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct VoiceSense {
    pub actor: ActorKey,
    /// Distance to the nearest player, metres.
    pub distance_m: f32,
    /// AI `Hello` (0..100).
    pub hello: i16,
    /// Perceives the player: line of sight, not sneaking unseen, not invisible.
    pub aware: bool,
    /// Free to talk on its own: alive, not in combat, a conversation or a scripted scene.
    pub free: bool,
    /// Standing idle (idle chatter only then; walkers between pathgrid points are not).
    pub idle: bool,
}

/// The greeting state of one actor.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum Greet {
    /// Will greet on approach.
    #[default]
    Ready,
    /// Greeted; facing the player for the remaining seconds.
    Greeting(f32),
    /// Greeted; waits for the player to leave the reset distance.
    Done,
}

#[derive(Clone, Copy, Debug, Default)]
struct ActorVoice {
    greet: Greet,
    speaking: bool,
}

/// A line the director wants spoken (the host selects it and calls [`VoiceDirector::begin`]).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct VoiceRequest {
    pub actor: ActorKey,
    pub topic: VoiceTopic,
}

/// Per-actor voice state: greetings, idle chatter, one line at a time. Pure; not saved (the
/// original resets greetings with the cell too).
#[derive(Clone, Debug)]
pub struct VoiceDirector {
    pub rules: VoiceRules,
    actors: HashMap<ActorKey, ActorVoice>,
    rng: Rng,
}

impl VoiceDirector {
    pub fn new(rules: VoiceRules, seed: u64) -> Self {
        Self { rules, actors: HashMap::new(), rng: Rng::new(seed) }
    }

    /// Advances by `dt` over this frame's loaded actors and returns the lines to start (Hello on
    /// approach before idle chatter). Actors missing from `actors` are forgotten.
    pub fn update(&mut self, dt: f32, actors: &[VoiceSense]) -> Vec<VoiceRequest> {
        let dt = dt.max(0.0);
        self.actors.retain(|k, v| v.speaking || actors.iter().any(|a| a.actor == *k));
        let idle_p = 1.0 - (-self.rules.idle_rate() * dt).exp();
        let mut out = Vec::new();
        for sense in actors {
            let rules = self.rules;
            let state = self.actors.entry(sense.actor).or_default();
            state.greet = match state.greet {
                Greet::Greeting(left) if left - dt > 0.0 => Greet::Greeting(left - dt),
                Greet::Greeting(_) => Greet::Done,
                Greet::Done if sense.distance_m > rules.greet_reset_m => Greet::Ready,
                g => g,
            };
            if state.speaking || !sense.free {
                continue;
            }
            if state.greet == Greet::Ready && sense.aware && sense.distance_m <= rules.hello_m(sense.hello) {
                state.greet = Greet::Greeting(rules.greet_duration_s);
                out.push(VoiceRequest { actor: sense.actor, topic: VoiceTopic::Hello });
                continue;
            }
            let chatty = sense.idle && sense.aware && sense.distance_m < rules.idle_range_m;
            if chatty && !matches!(state.greet, Greet::Greeting(_)) && self.rng.unit() < idle_p {
                out.push(VoiceRequest { actor: sense.actor, topic: VoiceTopic::Idle });
            }
        }
        out
    }

    /// Marks `actor` speaking; `false` (and nothing changes) while a line of theirs still plays.
    pub fn begin(&mut self, actor: ActorKey) -> bool {
        let state = self.actors.entry(actor).or_default();
        !std::mem::replace(&mut state.speaking, true)
    }

    /// The line ended (finished, dropped or the speaker unloaded).
    pub fn end(&mut self, actor: ActorKey) {
        if let Some(state) = self.actors.get_mut(&actor) {
            state.speaking = false;
        }
    }

    pub fn speaking(&self, actor: ActorKey) -> bool {
        self.actors.get(&actor).is_some_and(|s| s.speaking)
    }

    pub fn greet(&self, actor: ActorKey) -> Greet {
        self.actors.get(&actor).map_or(Greet::Ready, |s| s.greet)
    }
}

/// Window rate of [`Envelope::from_samples`] (50 ms windows, as `OpenMW`'s `Sound_Loudness`).
pub const ENVELOPE_HZ: f32 = 20.0;

/// A line's loudness over time, 0..1, for lip-sync.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Envelope {
    /// Windows per second.
    pub rate_hz: f32,
    /// RMS per window over all channels, divided by the loudest window.
    pub values: Vec<f32>,
}

impl Envelope {
    /// From interleaved samples (any channel count). Silence stays 0; a quiet recording still
    /// opens the mouth fully at its loudest.
    pub fn from_samples(samples: impl IntoIterator<Item = f32>, channels: u16, sample_rate: u32, rate_hz: f32) -> Self {
        #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss, clippy::cast_precision_loss)]
        let window = ((sample_rate as f32 * f32::from(channels.max(1))) / rate_hz.max(1.0)).round().max(1.0) as usize;
        let mut values = Vec::new();
        let (mut sum, mut n) = (0.0_f64, 0_usize);
        for s in samples {
            sum += f64::from(s) * f64::from(s);
            n += 1;
            if n == window {
                values.push(rms(sum, n));
                (sum, n) = (0.0, 0);
            }
        }
        if n > 0 {
            values.push(rms(sum, n));
        }
        let peak = values.iter().copied().fold(0.0_f32, f32::max);
        if peak > 1e-6 {
            for v in &mut values {
                *v /= peak;
            }
        }
        Self { rate_hz: rate_hz.max(1.0), values }
    }

    #[allow(clippy::cast_precision_loss)]
    pub fn duration_s(&self) -> f32 {
        self.values.len() as f32 / self.rate_hz
    }

    /// The loudness at `t` seconds into the line (window centres, linear between; 0 outside).
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    pub fn at(&self, t: f32) -> f32 {
        if !(0.0..self.duration_s()).contains(&t) {
            return 0.0;
        }
        let pos = (t * self.rate_hz - 0.5).max(0.0);
        let last = self.values.len() - 1;
        let first = (pos.floor() as usize).min(last);
        let (from, to) = (self.values[first], self.values[(first + 1).min(last)]);
        from + (to - from) * pos.fract()
    }
}

#[allow(clippy::cast_possible_truncation, clippy::cast_precision_loss)]
fn rms(sum: f64, n: usize) -> f32 {
    (sum / n as f64).sqrt() as f32
}
