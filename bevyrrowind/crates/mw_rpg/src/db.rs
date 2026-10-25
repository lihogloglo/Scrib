//! Dialogue and journal tables built from the record db (rpg-adapter.md "Building the
//! `DialogueDb`").

use std::collections::{BTreeMap, HashMap};

use sf_records::{RecordDb, RecordId};
use sf_rpg::{
    DialogueDb, EntryDraft, GameTime, LinkError, LinkRules, RpgCommand, StageFlags, TopicEntry,
    TopicKind, WorldRpg,
};
use tes3::esp::{Dialogue, DialogueInfo, DialogueType2, QuestState};

/// The persuasion topic shown when only a disposition check failed.
pub const INFO_REFUSAL: &str = "info refusal";

/// Link boundaries (`OpenMW` `KeywordSearch`: a link must start after a separator, may end
/// inside a word, so "ring" links in "rings" but not in "bring").
pub const LINK_RULES: LinkRules = LinkRules {
    word_start: true,
    word_end: false,
};

/// A response's id: the `mw_records` INFO key, `<topic>\u{1f}<info id>`.
pub fn response_id(topic: &str, info: &str) -> RecordId {
    RecordId::from(format!("{topic}\u{1f}{info}"))
}

/// One journal stage (a journal INFO with that index).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct JournalStage {
    pub response: RecordId,
    /// Raw text (macros expanded when the entry is written).
    pub text: String,
    pub flags: StageFlags,
}

/// A journal topic: the quest's display name (`QSTN` INFO) and its stages by index.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct MwQuest {
    pub name: Option<String>,
    pub stages: BTreeMap<i32, JournalStage>,
}

/// Journal topic id → quest.
#[derive(Clone, Debug, Default)]
pub struct MwJournalDb {
    quests: HashMap<RecordId, MwQuest>,
}

impl MwJournalDb {
    pub fn quest(&self, id: &RecordId) -> Option<&MwQuest> {
        self.quests.get(id)
    }

    pub fn stage(&self, quest: &RecordId, index: i32) -> Option<&JournalStage> {
        self.quests.get(quest)?.stages.get(&index)
    }

    pub fn len(&self) -> usize {
        self.quests.len()
    }

    pub fn is_empty(&self) -> bool {
        self.quests.is_empty()
    }

    fn insert(&mut self, topic: &Dialogue, infos: &[DialogueInfo]) {
        let mut quest = MwQuest::default();
        for info in infos {
            if info.quest_state == Some(QuestState::Name) {
                quest.name.get_or_insert_with(|| info.text.clone());
                continue;
            }
            let flags = StageFlags {
                finish: info.quest_state == Some(QuestState::Finished),
                restart: info.quest_state == Some(QuestState::Restart),
            };
            // Journal INFOs store their index in the disposition field.
            quest
                .stages
                .entry(info.data.disposition)
                .or_insert_with(|| JournalStage {
                    response: response_id(&topic.id, &info.id),
                    text: info.text.clone(),
                    flags,
                });
        }
        self.quests.insert(RecordId::new(&topic.id), quest);
    }
}

/// Morrowind's dialogue data: the framework `DialogueDb`, the journal table and the INFO
/// records the rules read. Built once at load; not saved.
#[derive(Clone, Debug)]
pub struct MwDialogue {
    pub db: DialogueDb,
    pub journal: MwJournalDb,
    infos: HashMap<RecordId, DialogueInfo>,
}

impl MwDialogue {
    /// From the merged record db: INFO order per topic rebuilt from `prev_id` links.
    pub fn from_records(records: &RecordDb) -> Result<Self, LinkError> {
        let mut per_topic: HashMap<&str, Vec<&DialogueInfo>> = HashMap::new();
        // `iter` is first-definition order, i.e. load order then file order.
        for (key, info) in records.iter::<DialogueInfo>() {
            if let Some((topic, _)) = mw_records::keys::split_info(key) {
                per_topic.entry(topic).or_default().push(info);
            }
        }
        let topics = records.iter::<Dialogue>().map(|(id, dial)| {
            let infos = per_topic.remove(id.as_str()).unwrap_or_default();
            (dial.clone(), order_infos(&infos))
        });
        Self::from_topics(topics)
    }

    /// From topics whose INFOs are already in priority order (tests, tools).
    pub fn from_topics(
        topics: impl IntoIterator<Item = (Dialogue, Vec<DialogueInfo>)>,
    ) -> Result<Self, LinkError> {
        let mut db = DialogueDb::new();
        let mut journal = MwJournalDb::default();
        let mut infos = HashMap::new();
        for (dial, list) in topics {
            let kind = match dial.dialogue_type {
                DialogueType2::Topic => TopicKind::Conversation,
                DialogueType2::Greeting => TopicKind::Greeting,
                DialogueType2::Persuasion => TopicKind::Service,
                DialogueType2::Voice => TopicKind::Bark,
                DialogueType2::Journal => {
                    journal.insert(&dial, &list);
                    continue;
                }
            };
            let responses: Vec<RecordId> =
                list.iter().map(|i| response_id(&dial.id, &i.id)).collect();
            for (id, info) in responses.iter().zip(list) {
                infos.insert(id.clone(), info);
            }
            db.insert_topic(
                RecordId::new(&dial.id),
                TopicEntry {
                    kind,
                    name: dial.id.clone(),
                    responses,
                },
            );
        }
        let greetings = (0..10)
            .map(|n| RecordId::from(format!("greeting {n}")))
            .filter(|g| db.topic(g).is_some_and(|t| t.kind == TopicKind::Greeting))
            .collect();
        db.set_greetings(greetings);
        db.build_linker(LINK_RULES)?;
        Ok(Self { db, journal, infos })
    }

    pub fn info(&self, response: &RecordId) -> Option<&DialogueInfo> {
        self.infos.get(response)
    }

    /// The `Journal quest index` command (`OpenMW` `Journal::addEntry`): adds the stage's entry
    /// unless the journal already has one for this quest at this index; the stage only ever
    /// rises through `Journal`. An index with no INFO just raises the stage. `None`: nothing
    /// to do. `render` expands macros in the entry text (snapshot at write time).
    pub fn journal_command(
        &self,
        world: &WorldRpg,
        quest: &RecordId,
        index: i32,
        time: GameTime,
        render: impl FnOnce(&str) -> String,
    ) -> Option<RpgCommand> {
        let current = world.quests.stage(quest);
        let raise = |flags| {
            (current < index).then(|| RpgCommand::SetStage {
                quest: quest.clone(),
                stage: index,
                flags,
                entry: None,
            })
        };
        let Some(stage) = self.journal.stage(quest, index) else {
            return raise(StageFlags::default());
        };
        if world
            .journal
            .for_quest(quest)
            .any(|e| e.stage == Some(index))
        {
            return raise(StageFlags::default());
        }
        let entry = (!stage.text.is_empty()).then(|| EntryDraft {
            source: Some(stage.response.clone()),
            text: render(&stage.text),
            time,
        });
        Some(RpgCommand::SetStage {
            quest: quest.clone(),
            stage: current.max(index),
            flags: stage.flags,
            entry,
        })
    }

    /// `SetJournalIndex quest index`: sets the stage (up or down), no entry, no flags.
    pub fn set_journal_index(quest: &RecordId, index: i32) -> RpgCommand {
        RpgCommand::SetStage {
            quest: quest.clone(),
            stage: index,
            flags: StageFlags::default(),
            entry: None,
        }
    }
}

/// `OpenMW` `InfoOrder::insertInfo` over the merged records: each INFO goes right after its
/// `prev_id`, at the front when it has none, at the end when its `prev_id` isn't there yet.
/// `NNAM` is ignored (as in `OpenMW`). Records arrive in first-definition order; a later
/// plugin's override uses its own `prev_id` at the first definition's turn (differs from
/// `OpenMW` only when an override moves an INFO; rpg-adapter.md "As built").
fn order_infos(infos: &[&DialogueInfo]) -> Vec<DialogueInfo> {
    let mut list: Vec<&DialogueInfo> = Vec::with_capacity(infos.len());
    for info in infos {
        let at = if info.prev_id.is_empty() {
            0
        } else {
            list.iter()
                .position(|i| i.id.eq_ignore_ascii_case(&info.prev_id))
                .map_or(list.len(), |p| p + 1)
        };
        list.insert(at, info);
    }
    list.into_iter().cloned().collect()
}

#[cfg(test)]
pub(crate) fn info(id: &str, prev: &str, text: &str) -> DialogueInfo {
    let mut info = DialogueInfo {
        id: id.into(),
        prev_id: prev.into(),
        text: text.into(),
        ..DialogueInfo::default()
    };
    // -1 is "no rank filter" in the data (the type defaults to 0).
    info.data.speaker_rank = -1;
    info.data.player_rank = -1;
    info
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn order_follows_prev_links() {
        let a = info("a", "", "");
        let c = info("c", "a", "");
        let b = info("b", "a", ""); // inserted after a, ahead of c
        let z = info("z", "missing", "");
        let first = info("f", "", "");
        let order = order_infos(&[&a, &c, &b, &z, &first]);
        let ids: Vec<&str> = order.iter().map(|i| i.id.as_str()).collect();
        assert_eq!(ids, ["f", "a", "b", "c", "z"]);
    }
}
