//! Voice lines on the real data (skips, passing, when no Morrowind install is detected; the
//! synthetic checks are `src/voice_tests.rs`). Run on the owner's machine:
//! `cargo test -p mw_audio --test voices -- --nocapture`.

use std::collections::HashMap;

use mw_audio::voice::{BarkView, VoiceRules, VoiceTopic, select, start_globals, voice_attenuation};
use mw_install::MorrowindInstall;
use mw_rpg::{ActorFacts, MwDialogue};
use sf_records::{RecordDb, RecordId};
use sf_rpg::{ActorKey, ConversationCtx, PlayerId};
use sf_vfs::Vfs;
use tes3::esp::{Npc, NpcFlags};

struct Loaded {
    vfs: Vfs,
    db: RecordDb,
}

fn load() -> Option<Loaded> {
    let Ok((install, _)) = MorrowindInstall::detect(None) else {
        eprintln!("no Morrowind install detected; skipping");
        return None;
    };
    let vfs = mw_vfs::mount_install(&install, Vfs::builder()).unwrap().vfs;
    let db = mw_records::load(&vfs, &install.game_files()).unwrap().db;
    Some(Loaded { vfs, db })
}

/// The voice folder letter of a vanilla race (`Sound/Vo/<race>/<sex>/`).
fn race_folder(race: &str) -> Option<char> {
    Some(match race.to_ascii_lowercase().as_str() {
        "argonian" => 'a',
        "breton" => 'b',
        "dark elf" => 'd',
        "high elf" => 'h',
        "imperial" => 'i',
        "khajiit" => 'k',
        "nord" => 'n',
        "orc" => 'o',
        "redguard" => 'r',
        "wood elf" => 'w',
        _ => return None,
    })
}

#[test]
fn game_settings_give_the_vanilla_voice_timing_and_range() {
    let Some(l) = load() else { return };
    let rules = VoiceRules::from_db(&l.db);
    let att = voice_attenuation(&l.db);
    eprintln!("{rules:?}\nvoice range {att:?}");
    assert_eq!(rules, VoiceRules::default(), "Morrowind.esm should match the fallbacks");
    assert_eq!(att, voice_attenuation(&RecordDb::new()));
    let sounds = mw_audio::vfs_sounds(&l.db, &l.vfs);
    eprintln!("{} voice cues", sounds.voices);
    assert!(sounds.voices > 1000, "{}", sounds.voices);
}

#[test]
fn hello_lines_match_the_speakers_race_and_sex() {
    let Some(l) = load() else { return };
    let dialogue = MwDialogue::from_records(&l.db).unwrap();
    let sounds = mw_audio::vfs_sounds(&l.db, &l.vfs);
    let globals = start_globals(&l.db);
    let player = ActorFacts { npc: true, ..ActorFacts::default() };
    let (mut voiced, mut checked, mut wrong, mut missing) = (0, 0, Vec::new(), Vec::new());
    let mut by_topic: HashMap<VoiceTopic, usize> = HashMap::new();
    let npcs: Vec<(&RecordId, &Npc)> = l.db.iter::<Npc>().collect();
    for (n, (id, npc)) in npcs.iter().enumerate() {
        let speaker = ActorFacts::from_npc(&l.db, npc, "");
        let view = BarkView { records: &l.db, speaker, player: &player, globals: &globals, disposition: i32::from(npc.data.disposition) };
        let ctx = ConversationCtx { player: PlayerId::HOST, speaker: ActorKey(n as u128), speaker_base: (*id).clone(), choice: None };
        for topic in VoiceTopic::ALL {
            let Some(line) = select(&dialogue, &view, &ctx, topic) else { continue };
            *by_topic.entry(topic).or_default() += 1;
            if !sounds.bank.0.contains_key(&line.cue) {
                missing.push(line.cue.clone());
            }
            if topic != VoiceTopic::Hello {
                continue;
            }
            voiced += 1;
            // `voice/vo/<race>/<sex>/...`: lines in a race folder belong to that race and sex.
            let parts: Vec<&str> = line.cue.as_str().split('/').collect();
            let (Some(want), [_, _, race, sex, _]) = (race_folder(&npc.race), parts.as_slice()) else { continue };
            checked += 1;
            let female = npc.npc_flags.contains(NpcFlags::FEMALE);
            if !race.starts_with(want) || *sex != if female { "f" } else { "m" } {
                wrong.push(format!("{} ({}, female {female}): {}", id.as_str(), npc.race, line.cue.as_str()));
            }
        }
    }
    eprintln!("{} NPCs, {voiced} with a voiced Hello, {checked} in a race folder; per topic {by_topic:?}", npcs.len());
    eprintln!("mismatched: {wrong:#?}");
    assert!(voiced * 2 > npcs.len(), "most NPCs greet: {voiced} of {}", npcs.len());
    assert!(wrong.len() * 100 <= checked.max(1), "{} of {checked} lines from another race or sex", wrong.len());
    missing.sort();
    missing.dedup();
    assert!(missing.is_empty(), "selected lines whose file is not in the VFS: {missing:?}");
    for topic in [VoiceTopic::Hello, VoiceTopic::Idle, VoiceTopic::Attack, VoiceTopic::Hit, VoiceTopic::Flee] {
        assert!(by_topic.get(&topic).copied().unwrap_or(0) > 0, "no NPC has a {topic:?} line");
    }
}

#[test]
fn fargoth_says_hello() {
    let Some(l) = load() else { return };
    let dialogue = MwDialogue::from_records(&l.db).unwrap();
    let npc: &Npc = l.db.get(&RecordId::new("fargoth")).expect("fargoth");
    let globals = start_globals(&l.db);
    let player = ActorFacts { npc: true, ..ActorFacts::default() };
    let view = BarkView {
        records: &l.db,
        speaker: ActorFacts::from_npc(&l.db, npc, "Seyda Neen"),
        player: &player,
        globals: &globals,
        disposition: i32::from(npc.data.disposition),
    };
    let ctx = ConversationCtx { player: PlayerId::HOST, speaker: ActorKey(1), speaker_base: RecordId::new("fargoth"), choice: None };
    let line = select(&dialogue, &view, &ctx, VoiceTopic::Hello).expect("a voiced Hello");
    eprintln!("{line:?}");
    assert!(line.cue.as_str().starts_with("voice/vo/"), "{}", line.cue.as_str());
}
