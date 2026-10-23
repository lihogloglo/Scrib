//! Voice lines on a synthetic INFO set (our ids, text and file names), the director's timing,
//! bus routing and the lip-sync envelope. The real-data checks are `tests/voices.rs`.

use std::collections::HashMap;

use mw_rpg::{ActorFacts, GlobalNumber, MwDialogue, MwView};
use sf_audio::{Bus, Ducker, SoundBank};
use sf_records::{RecordDb, RecordId};
use sf_rpg::{ActorKey, ConversationCtx, PlayerId};
use tes3::esp::{Dialogue, DialogueInfo, DialogueType2, Filter, FilterComparison, FilterType, FilterValue, Sex};

use crate::voice::{
    Envelope, Greet, VoiceDirector, VoiceRequest, VoiceRules, VoiceSense, VoiceTopic, add_voices, ducking, select,
    select_greeting, voice_attenuation, voice_cue,
};
use crate::{UNIT_M, build};

fn id(s: &str) -> RecordId {
    RecordId::new(s)
}

fn info(id: &str, sound: &str, text: &str) -> DialogueInfo {
    let mut i = DialogueInfo { id: id.into(), sound_path: sound.into(), text: text.into(), ..DialogueInfo::default() };
    i.data.speaker_rank = -1;
    i.data.player_rank = -1;
    i
}

fn race(mut i: DialogueInfo, race: &str, sex: Sex) -> DialogueInfo {
    i.speaker_race = race.into();
    i.data.speaker_sex = sex;
    i
}

fn quest_at_least(mut i: DialogueInfo, quest: &str, stage: i32) -> DialogueInfo {
    i.filters.push(Filter {
        filter_type: FilterType::Journal,
        id: quest.into(),
        comparison: FilterComparison::GreaterEqual,
        value: FilterValue::Integer(stage),
        ..Filter::default()
    });
    i
}

fn dial(name: &str, kind: DialogueType2) -> Dialogue {
    Dialogue { id: name.into(), dialogue_type: kind, ..Dialogue::default() }
}

/// "Hello" in priority order: one speaker's own line, a quest-gated Dunmer woman's line, then
/// race and sex lines, then a silent catch-all.
fn hello_infos() -> Vec<DialogueInfo> {
    let mut own = info("h_own", "Vo\\Misc\\hlo_tarvyn.mp3", "Ah, %Name greets you.");
    own.speaker_id = "tarvyn".into();
    vec![
        own,
        quest_at_least(race(info("h_df_q", "Vo\\d\\f\\hlo_df_quest.mp3", "You again."), "dark elf", Sex::Female), "q", 10),
        race(info("h_df", "Vo\\d\\f\\hlo_df001.mp3", "Yes?"), "dark elf", Sex::Female),
        race(info("h_dm", "Vo\\d\\m\\hlo_dm001.mp3", "What?"), "dark elf", Sex::Male),
        race(info("h_w", "Vo\\w\\hlo_w001.mp3", "Hm."), "wood elf", Sex::Any),
        info("h_any", "", "..."),
    ]
}

fn dialogue() -> MwDialogue {
    let mut greeting = info("g0", "Vo\\Misc\\greet_tarvyn.mp3", "Welcome, outlander.");
    greeting.speaker_id = "tarvyn".into();
    let idle = race(info("i_dm", "Vo\\d\\m\\idl_dm001.mp3", "Hmph."), "dark elf", Sex::Male);
    MwDialogue::from_topics([
        (dial("Hello", DialogueType2::Voice), hello_infos()),
        (dial("Idle", DialogueType2::Voice), vec![idle]),
        (dial("Greeting 0", DialogueType2::Greeting), vec![greeting, info("g_any", "", "Yes?")]),
    ])
    .unwrap()
}

struct View {
    speaker: ActorFacts,
    player: ActorFacts,
    stages: HashMap<RecordId, i32>,
}

impl View {
    fn npc(id: &str, race: Option<&str>, female: bool) -> Self {
        let speaker = ActorFacts {
            id: RecordId::new(id),
            name: "Tarvyn".into(),
            npc: true,
            race: race.map(RecordId::new),
            female,
            ..ActorFacts::default()
        };
        Self { speaker, player: ActorFacts { npc: true, ..ActorFacts::default() }, stages: HashMap::new() }
    }
}

impl MwView for View {
    fn speaker(&self, _: &ConversationCtx) -> Option<&ActorFacts> {
        Some(&self.speaker)
    }
    fn player(&self, _: PlayerId) -> Option<&ActorFacts> {
        Some(&self.player)
    }
    fn player_rank(&self, _: PlayerId, _: &RecordId) -> Option<i32> {
        None
    }
    fn global(&self, _: &RecordId) -> Option<GlobalNumber> {
        None
    }
    fn local(&self, _: ActorKey, _: &RecordId) -> Option<f32> {
        None
    }
    fn quest_stage(&self, quest: &RecordId) -> i32 {
        self.stages.get(quest).copied().unwrap_or(0)
    }
    fn item_count(&self, _: PlayerId, _: &RecordId) -> u64 {
        0
    }
    fn dead_count(&self, _: &RecordId) -> i32 {
        0
    }
    fn disposition(&self, _: &ConversationCtx) -> i32 {
        50
    }
    fn rank_name(&self, _: &RecordId, _: i32) -> Option<String> {
        None
    }
}

fn ctx(view: &View) -> ConversationCtx {
    ConversationCtx { player: PlayerId::HOST, speaker: ActorKey(7), speaker_base: view.speaker.id.clone(), choice: None }
}

fn hello(view: &View) -> Option<String> {
    select(&dialogue(), view, &ctx(view), VoiceTopic::Hello).map(|l| l.response.as_str().rsplit('\u{1f}').next().unwrap().to_owned())
}

#[test]
fn hello_follows_speaker_then_conditions_then_race_and_sex() {
    // The speaker's own line beats their race's.
    assert_eq!(hello(&View::npc("tarvyn", Some("wood elf"), false)).as_deref(), Some("h_own"));
    // Race and sex pick the file; a condition earlier in the list wins once it holds.
    let mut dunmer_f = View::npc("vendor_f", Some("dark elf"), true);
    assert_eq!(hello(&dunmer_f).as_deref(), Some("h_df"));
    dunmer_f.stages.insert(id("q"), 10);
    assert_eq!(hello(&dunmer_f).as_deref(), Some("h_df_q"));
    assert_eq!(hello(&View::npc("vendor_m", Some("dark elf"), false)).as_deref(), Some("h_dm"));
    // `Any` sex matches both.
    assert_eq!(hello(&View::npc("bosmer_f", Some("wood elf"), true)).as_deref(), Some("h_w"));
    // The catch-all passes but has no sound: silence, not the next voiced INFO.
    assert_eq!(hello(&View::npc("breton", Some("breton"), false)), None);
    // Creatures only pass INFOs naming them.
    let mut crab = View::npc("mudcrab", None, false);
    crab.speaker.npc = false;
    assert_eq!(hello(&crab), None);
}

#[test]
fn a_line_carries_its_cue_and_subtitle() {
    let view = View::npc("tarvyn", Some("wood elf"), false);
    let line = select(&dialogue(), &view, &ctx(&view), VoiceTopic::Hello).unwrap();
    assert_eq!(line.cue, id("voice/vo/misc/hlo_tarvyn.mp3"));
    assert_eq!(line.subtitle, "Ah, Tarvyn greets you.");
    assert_eq!(line.topic, id("hello"));
    let idle = select(&dialogue(), &View::npc("x", Some("dark elf"), false), &ctx(&view), VoiceTopic::Idle).unwrap();
    assert_eq!(idle.cue, voice_cue("vo\\d\\m\\IDL_DM001.mp3"));
    // A voiced greeting opens the conversation with its sound; an unvoiced one is silent.
    let greet = select_greeting(&dialogue(), &view, &ctx(&view)).unwrap();
    assert_eq!((greet.topic, greet.cue), (id("greeting 0"), id("voice/vo/misc/greet_tarvyn.mp3")));
    let other = View::npc("vendor_m", Some("dark elf"), false);
    assert_eq!(select_greeting(&dialogue(), &other, &ctx(&other)), None);
}

fn info_db() -> RecordDb {
    let mut db = RecordDb::new();
    db.register::<DialogueInfo>("INFO").unwrap();
    let esm = db.add_source("Test.esm", &[]);
    for (n, i) in hello_infos().into_iter().enumerate() {
        db.define(esm, id(&format!("hello\u{1f}{n}")), i).unwrap();
    }
    // A second INFO on the same file, and one whose file is missing.
    db.define(esm, id("idle\u{1f}0"), info("i0", "Vo\\d\\m\\hlo_dm001.mp3", "")).unwrap();
    db.define(esm, id("idle\u{1f}1"), info("i1", "Vo\\gone.mp3", "")).unwrap();
    db
}

#[test]
fn info_sounds_are_voice_bus_cues_with_the_voice_range() {
    let db = info_db();
    let mut bank = SoundBank::default();
    let mut cues = sf_audio::CueBank::default();
    let added = add_voices(&db, |raw| (!raw.contains("gone")).then(|| raw.to_ascii_lowercase()), &mut bank, &mut cues);
    assert_eq!(added, 5);
    let cue = id("voice/vo/d/m/hlo_dm001.mp3");
    assert_eq!(bank.get(&cue).unwrap().path, "sound/vo/d/m/hlo_dm001.mp3");
    assert!(bank.get(&voice_cue("Vo\\gone.mp3")).is_none());
    let resolved = cues.resolve(&cue).unwrap();
    assert_eq!((resolved.bus, resolved.max_instances), (Bus::Voice, 1));
    let att = resolved.attenuation.unwrap();
    // Vanilla voice range: 10 x 20 and 80 x 50 units.
    assert!((att.min_m - 200.0 * UNIT_M).abs() < 1e-3 && (att.max_m - 4000.0 * UNIT_M).abs() < 1e-3, "{att:?}");
    assert_eq!(att, voice_attenuation(&RecordDb::new()));
    // `build` includes them.
    let sounds = build(&db, |raw| Some(raw.to_ascii_lowercase()));
    assert_eq!(sounds.voices, 6);
}

#[test]
fn speech_ducks_music_only() {
    let mut ducker = Ducker::new(ducking());
    ducker.tick(0.5, |bus| bus == Bus::Voice);
    assert!((ducker.duck_db(Bus::Music) - 8.0).abs() < 1e-4);
    assert!(ducker.duck_db(Bus::Sfx).abs() < 1e-6 && ducker.duck_db(Bus::Voice).abs() < 1e-6);
    ducker.tick(0.5, |_| false);
    assert!((ducker.duck_db(Bus::Music) - 4.0).abs() < 1e-3, "released halfway after 0.5 s of 1 s");
}

const NPC: ActorKey = ActorKey(1);

fn sense(distance_m: f32) -> VoiceSense {
    VoiceSense { actor: NPC, distance_m, hello: 30, aware: true, free: true, idle: false }
}

fn hellos(d: &mut VoiceDirector, dt: f32, s: VoiceSense) -> usize {
    d.update(dt, &[s]).iter().filter(|r| r.topic == VoiceTopic::Hello).count()
}

#[test]
fn hello_on_approach_once_until_the_player_leaves() {
    let rules = VoiceRules::default();
    // Hello 30 x 6 units = 2.57 m; reset at 512 units = 7.3 m.
    assert!((rules.hello_m(30) - 2.571).abs() < 1e-3 && (rules.greet_reset_m - 7.315).abs() < 1e-3);
    let mut d = VoiceDirector::new(rules, 1);
    assert_eq!(hellos(&mut d, 0.1, sense(4.0)), 0);
    assert_eq!(d.update(0.1, &[sense(2.0)]), vec![VoiceRequest { actor: NPC, topic: VoiceTopic::Hello }]);
    assert!(matches!(d.greet(NPC), Greet::Greeting(_)));
    // Standing there (or stepping just outside the hello range): no second greeting.
    for _ in 0..100 {
        assert_eq!(hellos(&mut d, 0.1, sense(3.0)), 0);
    }
    assert_eq!(d.greet(NPC), Greet::Done);
    assert_eq!(hellos(&mut d, 0.1, sense(7.0)), 0);
    assert_eq!(hellos(&mut d, 0.1, sense(2.0)), 0);
    // Leave past the reset distance, come back: greeted again.
    assert_eq!(hellos(&mut d, 0.1, sense(8.0)), 0);
    assert_eq!(d.greet(NPC), Greet::Ready);
    assert_eq!(hellos(&mut d, 0.1, sense(2.0)), 1);
}

#[test]
fn no_hello_unaware_busy_or_with_hello_zero() {
    let mut d = VoiceDirector::new(VoiceRules::default(), 1);
    assert_eq!(hellos(&mut d, 0.1, VoiceSense { aware: false, ..sense(1.0) }), 0);
    assert_eq!(hellos(&mut d, 0.1, VoiceSense { free: false, ..sense(1.0) }), 0);
    assert_eq!(hellos(&mut d, 0.1, VoiceSense { hello: 0, ..sense(0.1) }), 0);
    // Still ready: the first chance it gets, it greets.
    assert_eq!(hellos(&mut d, 0.1, sense(1.0)), 1);
}

#[test]
fn one_line_per_actor_at_a_time() {
    let mut d = VoiceDirector::new(VoiceRules::default(), 1);
    assert!(d.begin(NPC));
    assert!(!d.begin(NPC), "a second bark while the first plays is dropped");
    assert!(d.speaking(NPC));
    // Speaking: no greeting is requested, and the greeting stays pending.
    assert_eq!(hellos(&mut d, 0.1, sense(1.0)), 0);
    assert_eq!(d.greet(NPC), Greet::Ready);
    d.end(NPC);
    assert!(!d.speaking(NPC) && d.begin(ActorKey(2)), "other actors are independent");
    assert_eq!(hellos(&mut d, 0.1, sense(1.0)), 1);
    assert!(d.begin(NPC));
}

#[test]
fn idle_chatter_follows_the_vanilla_rate() {
    let rules = VoiceRules::default();
    assert!((rules.idle_rate() - 0.006).abs() < 1e-6);
    let mut d = VoiceDirector::new(rules, 42);
    let idle = |i: u128| VoiceSense { actor: ActorKey(i), distance_m: 20.0, hello: 0, aware: true, free: true, idle: true };
    let crowd: Vec<VoiceSense> = (0..100).map(idle).collect();
    let mut lines = 0;
    for _ in 0..1000 {
        lines += d.update(0.1, &crowd).iter().filter(|r| r.topic == VoiceTopic::Idle).count();
    }
    // 100 actors x 100 s x 0.006/s = 60 expected (Poisson, sd ~7.7).
    assert!((35..=85).contains(&lines), "{lines}");
    // Walking, out of range, unseen or busy actors never chatter.
    for s in [
        VoiceSense { idle: false, ..idle(0) },
        VoiceSense { distance_m: 50.0, ..idle(0) },
        VoiceSense { aware: false, ..idle(0) },
        VoiceSense { free: false, ..idle(0) },
    ] {
        let mut d = VoiceDirector::new(rules, 42);
        assert!((0..10_000).all(|_| d.update(0.1, &[s]).is_empty()), "{s:?}");
    }
}

#[test]
fn absent_actors_are_forgotten_unless_speaking() {
    let mut d = VoiceDirector::new(VoiceRules::default(), 1);
    assert_eq!(hellos(&mut d, 0.1, sense(1.0)), 1);
    d.update(0.1, &[]);
    assert_eq!(d.greet(NPC), Greet::Ready, "unloaded: greets again next time");
    assert!(d.begin(NPC));
    d.update(0.1, &[]);
    assert!(d.speaking(NPC), "a playing line outlives its actor's absence until the host ends it");
}

fn tone(amplitude: f32, seconds: f32, rate: u32, channels: u16) -> Vec<f32> {
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss, clippy::cast_precision_loss)]
    let frames = (seconds * rate as f32) as usize;
    (0..frames)
        .flat_map(|i| {
            #[allow(clippy::cast_precision_loss)]
            let v = amplitude * (i as f32 * 440.0 * std::f32::consts::TAU / rate as f32).sin();
            std::iter::repeat_n(v, usize::from(channels))
        })
        .collect()
}

#[test]
fn envelope_follows_loudness_normalised_to_the_loudest_window() {
    let mut samples = tone(0.5, 0.5, 8000, 1);
    samples.extend(tone(0.25, 0.25, 8000, 1));
    samples.extend(std::iter::repeat_n(0.0, 2000));
    let env = Envelope::from_samples(samples, 1, 8000, 20.0);
    assert_eq!(env.values.len(), 20);
    assert!((env.duration_s() - 1.0).abs() < 1e-6);
    assert!((env.at(0.2) - 1.0).abs() < 0.02, "{}", env.at(0.2));
    assert!((env.at(0.6) - 0.5).abs() < 0.02, "{}", env.at(0.6));
    assert!(env.at(0.9).abs() < 1e-6);
    assert!(env.at(-0.1).abs() < 1e-6 && env.at(1.5).abs() < 1e-6);
    // Between window centres the value is interpolated.
    let mid = env.at(0.75);
    assert!(mid > 0.0 && mid < 0.5, "{mid}");
    // Interleaved stereo gives the same timing; silence stays silent.
    let stereo = Envelope::from_samples(tone(0.1, 1.0, 8000, 2), 2, 8000, 20.0);
    assert_eq!(stereo.values.len(), 20);
    assert!(stereo.values.iter().all(|v| (v - 1.0).abs() < 0.02));
    let quiet = Envelope::from_samples(vec![0.0; 800], 1, 8000, 20.0);
    assert!(quiet.values.iter().all(|v| *v == 0.0) && quiet.values.len() == 2);
    assert!(Envelope::default().at(0.0).abs() < 1e-6);
}
