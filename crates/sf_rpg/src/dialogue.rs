//! Dialogue database, adapter hook and the conversation state machine (rpg.md "Dialogue").

use std::collections::BTreeMap;

use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use sf_records::RecordId;
use thiserror::Error;

use crate::ids::{ActorKey, PlayerId};
use crate::link::{LinkError, LinkRules, TopicLinker};
use crate::quest::Topics;

/// What a topic is for.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TopicKind {
    /// Listed in the topic list and linkable in text.
    Conversation,
    /// Opening lines, evaluated in [`DialogueDb::greetings`] order.
    Greeting,
    /// Persuasion and service replies (admire, bribe, refusal), asked by the rules.
    Service,
    /// Voiced barks outside conversations.
    Bark,
}

/// A topic and its responses in priority order (the adapter resolves load order).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TopicEntry {
    pub kind: TopicKind,
    /// Display name (also what links match).
    pub name: String,
    pub responses: Vec<RecordId>,
}

/// Built by the adapter at load; not saved.
#[derive(Clone, Debug, Default)]
pub struct DialogueDb {
    topics: IndexMap<RecordId, TopicEntry>,
    greetings: Vec<RecordId>,
    linker: Option<TopicLinker>,
}

impl DialogueDb {
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds or replaces a topic. Invalidates the linker (call [`DialogueDb::build_linker`]).
    pub fn insert_topic(&mut self, id: RecordId, entry: TopicEntry) {
        self.topics.insert(id, entry);
        self.linker = None;
    }

    /// Greeting topics in evaluation order.
    pub fn set_greetings(&mut self, greetings: Vec<RecordId>) {
        self.greetings = greetings;
    }

    pub fn greetings(&self) -> &[RecordId] {
        &self.greetings
    }

    pub fn topic(&self, id: &RecordId) -> Option<&TopicEntry> {
        self.topics.get(id)
    }

    pub fn topics(&self) -> impl Iterator<Item = (&RecordId, &TopicEntry)> {
        self.topics.iter()
    }

    /// Builds the link matcher over all `Conversation` topic names.
    pub fn build_linker(&mut self, rules: LinkRules) -> Result<(), LinkError> {
        let names = self
            .topics
            .iter()
            .filter(|(_, t)| t.kind == TopicKind::Conversation)
            .map(|(id, t)| (id.clone(), t.name.as_str()));
        self.linker = Some(TopicLinker::new(names, rules)?);
        Ok(())
    }

    pub fn linker(&self) -> Option<&TopicLinker> {
        self.linker.as_ref()
    }

    /// The first response under `topic` that the rules accept.
    pub fn select(&self, rules: &dyn DialogueRules, ctx: &ConversationCtx, topic: &RecordId) -> Option<RecordId> {
        self.topics.get(topic)?.responses.iter().find(|r| rules.accepts(ctx, topic, r)).cloned()
    }

    /// The first accepted response across the greeting topics, in order.
    pub fn select_greeting(&self, rules: &dyn DialogueRules, ctx: &ConversationCtx) -> Option<(RecordId, RecordId)> {
        self.greetings.iter().find_map(|g| self.select(rules, ctx, g).map(|r| (g.clone(), r)))
    }
}

/// Who is talking to whom, plus the pending choice.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConversationCtx {
    /// The talking player (the script context player).
    pub player: PlayerId,
    pub speaker: ActorKey,
    /// The speaker's base record.
    pub speaker_base: RecordId,
    /// Set only while re-asking after a choice.
    pub choice: Option<i32>,
}

/// Adapter hook: filter semantics and text macros. Implementations hold read access to the
/// world and character state their filters need.
pub trait DialogueRules {
    fn accepts(&self, ctx: &ConversationCtx, topic: &RecordId, response: &RecordId) -> bool;
    /// The response text with macros expanded.
    fn render(&self, ctx: &ConversationCtx, topic: &RecordId, response: &RecordId) -> String;
}

/// What a step produced, in order.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum DialogueOutput {
    /// Show this response (it is also in the session history).
    Line { topic: RecordId, response: RecordId, text: String },
    /// A topic name appeared in the text: learn it (`AddTopic`).
    Learned(RecordId),
    /// Run the response's result script as the speaker (host, context player = `ctx.player`).
    RunResult { topic: RecordId, response: RecordId },
    /// No response passed (`None`: no greeting).
    NoResponse { topic: Option<RecordId> },
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Choice {
    pub label: String,
    pub value: i32,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum SessionState {
    Talking,
    Choosing(Vec<Choice>),
    /// A result asked for goodbye: only ending is allowed.
    Goodbye,
    Ended,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ShownLine {
    pub topic: RecordId,
    pub response: RecordId,
    pub text: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Error)]
pub enum DialogueError {
    #[error("the conversation has ended")]
    Ended,
    #[error("a choice is pending")]
    ChoicePending,
    #[error("only goodbye is allowed")]
    GoodbyeOnly,
    #[error("no choice is pending")]
    NotChoosing,
    #[error("choice {0} was not offered")]
    InvalidChoice(i32),
    #[error("no choices given")]
    NoChoices,
    #[error("unknown topic {0}")]
    UnknownTopic(RecordId),
    #[error("{speaker:?} is talking to {holder:?}")]
    Busy { speaker: ActorKey, holder: PlayerId },
}

/// One conversation: plain data, run on the host.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct DialogueSession {
    ctx: ConversationCtx,
    state: SessionState,
    current: Option<RecordId>,
    history: Vec<ShownLine>,
}

impl DialogueSession {
    /// Opens a conversation with the first accepted greeting.
    pub fn open(db: &DialogueDb, rules: &dyn DialogueRules, ctx: ConversationCtx) -> (Self, Vec<DialogueOutput>) {
        let mut session = Self { ctx, state: SessionState::Talking, current: None, history: Vec::new() };
        let out = match db.select_greeting(rules, &session.ctx) {
            Some((topic, response)) => {
                session.current = Some(topic.clone());
                session.show(db, rules, topic, response)
            }
            None => vec![DialogueOutput::NoResponse { topic: None }],
        };
        (session, out)
    }

    pub fn ctx(&self) -> &ConversationCtx {
        &self.ctx
    }

    pub fn state(&self) -> &SessionState {
        &self.state
    }

    pub fn current_topic(&self) -> Option<&RecordId> {
        self.current.as_ref()
    }

    pub fn history(&self) -> &[ShownLine] {
        &self.history
    }

    fn check_talking(&self) -> Result<(), DialogueError> {
        match self.state {
            SessionState::Talking => Ok(()),
            SessionState::Choosing(_) => Err(DialogueError::ChoicePending),
            SessionState::Goodbye => Err(DialogueError::GoodbyeOnly),
            SessionState::Ended => Err(DialogueError::Ended),
        }
    }

    /// Asks about a topic (conversation or service).
    pub fn ask(
        &mut self,
        db: &DialogueDb,
        rules: &dyn DialogueRules,
        topic: &RecordId,
    ) -> Result<Vec<DialogueOutput>, DialogueError> {
        self.check_talking()?;
        if db.topic(topic).is_none() {
            return Err(DialogueError::UnknownTopic(topic.clone()));
        }
        self.current = Some(topic.clone());
        Ok(self.select_and_show(db, rules, topic.clone()))
    }

    fn select_and_show(&mut self, db: &DialogueDb, rules: &dyn DialogueRules, topic: RecordId) -> Vec<DialogueOutput> {
        match db.select(rules, &self.ctx, &topic) {
            Some(response) => self.show(db, rules, topic, response),
            None => vec![DialogueOutput::NoResponse { topic: Some(topic) }],
        }
    }

    fn show(&mut self, db: &DialogueDb, rules: &dyn DialogueRules, topic: RecordId, response: RecordId) -> Vec<DialogueOutput> {
        let text = rules.render(&self.ctx, &topic, &response);
        let mut out = vec![DialogueOutput::Line { topic: topic.clone(), response: response.clone(), text: text.clone() }];
        if let Some(linker) = db.linker() {
            let mut learned: Vec<RecordId> = Vec::new();
            for span in linker.link(&text) {
                if !learned.contains(&span.topic) {
                    learned.push(span.topic);
                }
            }
            out.extend(learned.into_iter().map(DialogueOutput::Learned));
        }
        out.push(DialogueOutput::RunResult { topic: topic.clone(), response: response.clone() });
        self.history.push(ShownLine { topic, response, text });
        out
    }

    /// Called by a result script (`Choice`): the player must pick one.
    pub fn offer_choices(&mut self, choices: Vec<Choice>) -> Result<(), DialogueError> {
        self.check_talking()?;
        if choices.is_empty() {
            return Err(DialogueError::NoChoices);
        }
        self.state = SessionState::Choosing(choices);
        Ok(())
    }

    /// The player picked `value`: the current topic is asked again with the choice as a
    /// filter input, then the choice is cleared.
    pub fn choose(
        &mut self,
        db: &DialogueDb,
        rules: &dyn DialogueRules,
        value: i32,
    ) -> Result<Vec<DialogueOutput>, DialogueError> {
        let SessionState::Choosing(choices) = &self.state else {
            return Err(match self.state {
                SessionState::Ended => DialogueError::Ended,
                _ => DialogueError::NotChoosing,
            });
        };
        if !choices.iter().any(|c| c.value == value) {
            return Err(DialogueError::InvalidChoice(value));
        }
        self.state = SessionState::Talking;
        let topic = self.current.clone().expect("a choice is offered only after a line");
        self.ctx.choice = Some(value);
        let out = self.select_and_show(db, rules, topic);
        self.ctx.choice = None;
        Ok(out)
    }

    /// Called by a result script (`Goodbye`): only ending remains.
    pub fn force_goodbye(&mut self) {
        if self.state != SessionState::Ended {
            self.state = SessionState::Goodbye;
        }
    }

    /// The player ends the conversation.
    pub fn goodbye(&mut self) {
        self.state = SessionState::Ended;
    }

    /// Known conversation topics this speaker has an accepted response for, by name.
    pub fn available_topics(&self, db: &DialogueDb, rules: &dyn DialogueRules, known: &Topics) -> Vec<RecordId> {
        let mut out: Vec<(&str, RecordId)> = db
            .topics()
            .filter(|(id, t)| t.kind == TopicKind::Conversation && known.knows(id))
            .filter(|(id, _)| db.select(rules, &self.ctx, id).is_some())
            .map(|(id, t)| (t.name.as_str(), id.clone()))
            .collect();
        out.sort_by_cached_key(|(name, _)| name.to_lowercase());
        out.into_iter().map(|(_, id)| id).collect()
    }
}

/// The host's claim table: one conversation per speaker (net.md "Concurrency").
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Conversations {
    claims: BTreeMap<ActorKey, PlayerId>,
}

impl Conversations {
    /// Claims `speaker` for `player`; re-claiming by the same player is fine.
    pub fn claim(&mut self, speaker: ActorKey, player: PlayerId) -> Result<(), DialogueError> {
        match self.claims.get(&speaker) {
            Some(holder) if *holder != player => Err(DialogueError::Busy { speaker, holder: *holder }),
            _ => {
                self.claims.insert(speaker, player);
                Ok(())
            }
        }
    }

    pub fn release(&mut self, speaker: ActorKey) {
        self.claims.remove(&speaker);
    }

    /// Releases every claim a player holds (disconnect).
    pub fn release_player(&mut self, player: PlayerId) {
        self.claims.retain(|_, p| *p != player);
    }

    pub fn holder(&self, speaker: ActorKey) -> Option<PlayerId> {
        self.claims.get(&speaker).copied()
    }
}
