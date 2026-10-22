//! Quest list and topic index rows (journal-ui.md "Quests", "Topics"): `sf_ui` view-models,
//! held in `sf_ui::ListModel`s by the screen.

use std::collections::HashMap;

use sf_rpg::{RecordId, WorldRpg};
use sf_ui::{UiValue, ViewModel};

use crate::labels::{self, JournalLabels};
use crate::seen::JournalSeen;

/// A quest with journal entries.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct QuestRow {
    pub id: RecordId,
    pub name: String,
    pub finished: bool,
    pub stage: i32,
    /// Journal entries written for it.
    pub entries: u32,
    /// Sequence number of its latest entry ("recent" sort).
    pub last_seq: u32,
    pub last_day: u32,
    /// Has an entry the player hasn't seen.
    pub new: bool,
}

impl ViewModel for QuestRow {
    const KIND: &'static str = "journal.quest";

    fn fields() -> &'static [&'static str] {
        &[
            "id", "name", "finished", "stage", "entries", "last_seq", "last_day", "new",
        ]
    }

    fn field(&self, name: &str) -> Option<UiValue> {
        Some(match name {
            "id" => self.id.as_str().into(),
            "name" => self.name.as_str().into(),
            "finished" => self.finished.into(),
            "stage" => self.stage.into(),
            "entries" => self.entries.into(),
            "last_seq" => self.last_seq.into(),
            "last_day" => self.last_day.into(),
            "new" => self.new.into(),
            _ => return None,
        })
    }
}

/// One row per quest with entries, in first-entry order.
pub fn quest_rows(
    world: &WorldRpg,
    labels: &dyn JournalLabels,
    seen: &JournalSeen,
) -> Vec<QuestRow> {
    let mut rows: Vec<QuestRow> = Vec::new();
    let mut index: HashMap<&RecordId, usize> = HashMap::new();
    for e in world.journal.entries() {
        let Some(quest) = &e.quest else { continue };
        let i = *index.entry(quest).or_insert_with(|| {
            let state = world.quests.get(quest);
            rows.push(QuestRow {
                id: quest.clone(),
                name: labels::quest_name(labels, quest),
                finished: state.finished,
                stage: state.stage,
                entries: 0,
                last_seq: e.seq,
                last_day: e.time.day,
                new: false,
            });
            rows.len() - 1
        });
        let row = &mut rows[i];
        row.entries += 1;
        row.last_seq = e.seq;
        row.last_day = e.time.day;
        row.new |= !seen.entry_seen(e.seq);
    }
    rows
}

/// A known topic.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TopicRow {
    pub id: RecordId,
    pub name: String,
    /// Letter tab key ([`Alphabet::tab_of`]).
    pub initial: String,
    /// Learning order.
    pub learned: u32,
    /// Lines heard under it.
    pub heard: u32,
    pub new: bool,
}

impl ViewModel for TopicRow {
    const KIND: &'static str = "journal.topic";

    fn fields() -> &'static [&'static str] {
        &["id", "name", "initial", "learned", "heard", "new"]
    }

    fn field(&self, name: &str) -> Option<UiValue> {
        Some(match name {
            "id" => self.id.as_str().into(),
            "name" => self.name.as_str().into(),
            "initial" => self.initial.as_str().into(),
            "learned" => self.learned.into(),
            "heard" => self.heard.into(),
            "new" => self.new.into(),
            _ => return None,
        })
    }
}

/// One row per known topic, in learning order (the screen sorts by name).
pub fn topic_rows(
    world: &WorldRpg,
    labels: &dyn JournalLabels,
    seen: &JournalSeen,
    alphabet: &Alphabet,
) -> Vec<TopicRow> {
    world
        .topics
        .in_learned_order()
        .into_iter()
        .filter_map(|id| {
            let known = world.topics.get(id)?;
            let name = labels::topic_name(labels, id);
            Some(TopicRow {
                id: id.clone(),
                initial: alphabet.tab_of(&name).to_owned(),
                name,
                learned: known.learned,
                heard: u32::try_from(known.heard.len()).unwrap_or(u32::MAX),
                new: seen.topic_new(id, known.heard.len()),
            })
        })
        .collect()
}

/// Letter tabs of the topic index: ordered labels, aliases (`Ä` → `A`) and a catch-all tab
/// for names that start with anything else.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Alphabet {
    letters: Vec<String>,
    aliases: Vec<(char, usize)>,
    other: String,
}

impl Default for Alphabet {
    /// Latin A–Z, other initials under `#`.
    fn default() -> Self {
        Self::new(('A'..='Z').map(String::from), "#")
    }
}

impl Alphabet {
    /// Tabs in display order; each label's first character (uppercased) is its initial.
    pub fn new(letters: impl IntoIterator<Item = String>, other: impl Into<String>) -> Self {
        Self {
            letters: letters.into_iter().collect(),
            aliases: Vec::new(),
            other: other.into(),
        }
    }

    /// Files names starting with `initial` (any case) under the tab `letter`. Unknown tab
    /// labels are ignored.
    #[must_use]
    pub fn with_alias(mut self, initial: char, letter: &str) -> Self {
        if let Some(i) = self.letters.iter().position(|l| l == letter) {
            for c in initial.to_uppercase() {
                self.aliases.push((c, i));
            }
        }
        self
    }

    /// Tab labels in display order, then the catch-all.
    pub fn labels(&self) -> impl Iterator<Item = &str> {
        self.letters
            .iter()
            .map(String::as_str)
            .chain(std::iter::once(self.other.as_str()))
    }

    /// The tab a name files under.
    pub fn tab_of(&self, name: &str) -> &str {
        let Some(c) = name.chars().find(|c| !c.is_whitespace()) else {
            return &self.other;
        };
        let upper: String = c.to_uppercase().collect();
        if let Some(l) = self.letters.iter().find(|l| first_upper(l) == upper) {
            return l;
        }
        let mut chars = upper.chars();
        if let (Some(u), None) = (chars.next(), chars.next())
            && let Some(&(_, i)) = self.aliases.iter().find(|(a, _)| *a == u)
        {
            return &self.letters[i];
        }
        &self.other
    }
}

fn first_upper(label: &str) -> String {
    label
        .chars()
        .next()
        .map(char::to_uppercase)
        .into_iter()
        .flatten()
        .collect()
}

/// A letter tab with what it holds.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LetterTab {
    pub label: String,
    /// Topics filed under it (0: shown disabled).
    pub count: usize,
    /// Holds a new topic.
    pub new: bool,
}

/// Every tab of `alphabet` with its counts, in display order.
pub fn letter_tabs<'a>(
    alphabet: &Alphabet,
    rows: impl IntoIterator<Item = &'a TopicRow>,
) -> Vec<LetterTab> {
    let mut tabs: Vec<LetterTab> = alphabet
        .labels()
        .map(|l| LetterTab {
            label: l.to_owned(),
            count: 0,
            new: false,
        })
        .collect();
    for row in rows {
        if let Some(tab) = tabs.iter_mut().find(|t| t.label == row.initial) {
            tab.count += 1;
            tab.new |= row.new;
        }
    }
    tabs
}
