use tes3::esp::SoundData;

use super::*;

fn id(s: &str) -> RecordId {
    RecordId::new(s)
}

fn sound(name: &str, path: &str, volume: u8, range: (u8, u8)) -> Sound {
    Sound { id: name.into(), sound_path: path.into(), data: SoundData { volume, range }, ..Sound::default() }
}

fn define<T: Send + Sync + 'static>(db: &mut RecordDb, esm: sf_records::SourceId, key: &str, record: T) {
    db.define(esm, id(key), record).unwrap();
}

const DRIP: &str = "begin Sound_Cave_Drip\r\n\r\nif ( CellChanged == 0 )\r\n\tif ( GetSoundPlaying \"Cave Drip\" == 0 )\r\n\t\tPlayLoopSound3DVP \"Cave Drip\", 0.5, 1.2\r\n\tendif\r\nendif\r\n\r\nend";

fn db() -> RecordDb {
    let mut db = RecordDb::new();
    db.register::<Sound>("SOUN").unwrap();
    db.register::<SoundGen>("SNDG").unwrap();
    db.register::<Door>("DOOR").unwrap();
    db.register::<Light>("LIGH").unwrap();
    db.register::<Script>("SCPT").unwrap();
    db.register::<Activator>("ACTI").unwrap();
    db.register::<GameSetting>("GMST").unwrap();
    let esm = db.add_source("Test.esm", &[]);
    define(&mut db, esm, "Fire", sound("Fire", "Fx\\envrn\\fire.wav", 204, (2, 15)));
    define(&mut db, esm, "Door Stone Open", sound("Door Stone Open", "Fx\\trans\\drstn_opn.wav", 255, (0, 0)));
    define(&mut db, esm, "Menu Click", sound("Menu Click", "Fx\\menu click.wav", 127, (0, 0)));
    define(&mut db, esm, "Cave Drip", sound("Cave Drip", "Fx\\envrn\\drip.wav", 255, (1, 15)));
    define(&mut db, esm, "mudcrab moan", sound("mudcrab moan", "Cr\\mdcrb\\moan.wav", 255, (0, 0)));
    define(&mut db, esm, "mudcrab moan2", sound("mudcrab moan2", "Cr\\mdcrb\\moan2.wav", 255, (0, 0)));
    define(&mut db, esm, "default land", sound("default land", "Fx\\land.wav", 255, (0, 0)));
    define(&mut db, esm, "missing", sound("missing", "Fx\\nope.wav", 255, (0, 0)));
    for (key, creature, kind, s) in [
        ("mudcrab0004", "mudcrab", SoundGenType::Moan, "mudcrab moan"),
        ("mudcrab0005", "mudcrab", SoundGenType::Moan, "mudcrab moan2"),
        ("defaultland", "", SoundGenType::Land, "default land"),
    ] {
        define(&mut db, esm, key, SoundGen { id: key.into(), creature: creature.into(), sound_gen_type: kind, sound: s.into(), ..SoundGen::default() });
    }
    define(&mut db, esm, "door_stone", Door { id: "door_stone".into(), open_sound: "Door Stone Open".into(), ..Door::default() });
    define(&mut db, esm, "light_fire", Light { id: "light_fire".into(), sound: "Fire".into(), ..Light::default() });
    define(&mut db, esm, "light_quiet", Light { id: "light_quiet".into(), ..Light::default() });
    define(&mut db, esm, "Sound_Cave_Drip", Script { id: "Sound_Cave_Drip".into(), text: DRIP.into(), ..Script::default() });
    define(&mut db, esm, "sound_cave_drip00", Activator { id: "sound_cave_drip00".into(), script: "Sound_Cave_Drip".into(), ..Activator::default() });
    db
}

fn sounds() -> MwSounds {
    build(&db(), |raw| (!raw.contains("nope")).then(|| raw.to_ascii_lowercase()))
}

fn close(a: f32, b: f32) -> bool {
    (a - b).abs() < 1e-3
}

#[test]
fn range_bytes_scale_to_metres_with_the_vanilla_settings() {
    let s = RangeScale::default();
    let fire = s.attenuation((2, 15));
    assert!(close(fire.min_m, 40.0 * UNIT_M) && close(fire.max_m, 750.0 * UNIT_M), "{fire:?}");
    assert!(close(fire.max_m, 10.716), "{fire:?}");
    let default = s.attenuation((0, 0));
    assert!(close(default.min_m, 1.429) && close(default.max_m, 28.575), "{default:?}");
    // min never below one unit, max never below min.
    let odd = s.attenuation((0, 0)).max_m.min(s.attenuation((3, 0)).max_m);
    assert!(close(odd, 60.0 * UNIT_M));
}

#[test]
fn every_sound_is_a_bank_entry_and_a_cue() {
    let mut s = sounds();
    let fire = s.bank.get(&id("fire")).unwrap();
    assert_eq!(fire.path, "sound/fx/envrn/fire.wav");
    assert!(close(fire.volume, 0.8));
    assert!(s.bank.get(&id("missing")).is_none());
    let cue = s.cues.resolve(&id("fire")).unwrap();
    assert_eq!((cue.sound.as_str(), cue.bus), ("fire", Bus::Sfx));
    assert!(close(cue.attenuation.unwrap().max_m, 10.716));
    let click = s.cues.resolve(&id("menu click")).unwrap();
    assert_eq!((click.bus, click.attenuation), (Bus::Ui, None));
}

#[test]
fn creature_generators_group_by_creature_and_kind() {
    let mut s = sounds();
    let moan = &s.cues.groups[&id("mudcrab/moan")];
    assert_eq!(moan.variants.len(), 2);
    assert!(s.cues.resolve(&id("mudcrab/moan")).is_some());
    let has = |c: &RecordId| s.cues.groups.contains_key(c);
    assert_eq!(creature_cue("mudcrab_unique", "mudcrab", SoundGenType::Moan, has), Some(id("mudcrab/moan")));
    assert_eq!(creature_cue("mudcrab", "", SoundGenType::Land, has), Some(id("/land")));
    assert_eq!(creature_cue("mudcrab", "", SoundGenType::Roar, has), None);
}

#[test]
fn doors_lights_and_sound_activators() {
    let mut s = sounds();
    assert_eq!(s.doors[&id("door_stone")], DoorSounds { open: Some(id("door stone open")), close: None });
    let light = s.emitters[&id("light_fire")].clone();
    let cue = s.cues.resolve(&light).unwrap();
    assert_eq!((cue.sound.as_str(), cue.bus), ("fire", Bus::Ambience));
    assert!(!s.emitters.contains_key(&id("light_quiet")));
    let drip = s.emitters[&id("sound_cave_drip00")].clone();
    let cue = s.cues.resolve(&drip).unwrap();
    assert_eq!(cue.sound.as_str(), "cave drip");
    assert!(close(cue.volume, 0.5) && close(cue.pitch, 1.2));
}

#[test]
fn loop_scripts_only_match_the_pure_idiom() {
    assert_eq!(loop_script(DRIP).map(|(s, ..)| s), Some(id("cave drip")));
    assert_eq!(loop_script("begin x\nPlayLoopSound3D \"Fire\"\nend"), Some((id("fire"), 1.0, 1.0)));
    // Quest logic or a second sound: not a sound activator.
    assert_eq!(loop_script("begin x\nif ( GetJournalIndex \"A\" > 10 )\nPlayLoopSound3D \"Fire\"\nendif\nend"), None);
    assert_eq!(loop_script("begin x\nPlayLoopSound3D \"Fire\"\nPlayLoopSound3D \"Wind\"\nend"), None);
    assert_eq!(loop_script("begin x\nDisable\nend"), None);
}

#[test]
fn footsteps_follow_the_boots_weight_class() {
    let s = sounds();
    assert_eq!(s.footsteps.resolve("left", Some(Footwear::Heavy.surface())), Some(&id("footheavyleft")));
    assert_eq!(s.footsteps.resolve("Right", Some("medium")), Some(&id("footmedright")));
    assert_eq!(s.footsteps.resolve("left", Some("grass")), Some(&id("footbareleft")));
    assert!(close(stride_m(1.0), 0.7) && close(stride_m(6.0), 1.1) && close(stride_m(3.0), 0.9));
}

#[test]
fn playlists_take_audio_files_from_the_music_folders() {
    let lists = playlists(|dir| match dir {
        "Music/Explore" => vec!["music/explore/b.mp3".into(), "music/explore/a.mp3".into(), "music/explore/readme.txt".into()],
        "Music/Battle" => vec!["music/battle/mod_track.wav".into()],
        _ => vec![],
    });
    assert_eq!(lists[0], ("explore", vec!["music/explore/a.mp3".to_owned(), "music/explore/b.mp3".to_owned()]));
    assert_eq!(lists[1], ("battle", vec!["music/battle/mod_track.wav".to_owned()]));
}

#[test]
fn battle_music_holds_after_combat() {
    let mut mood = MusicMood::default();
    assert_eq!(mood.update(0.1, false), "explore");
    assert_eq!(mood.update(0.1, true), "battle");
    assert_eq!(mood.update(4.0, false), "battle");
    assert_eq!(mood.update(1.5, false), "explore");
}
