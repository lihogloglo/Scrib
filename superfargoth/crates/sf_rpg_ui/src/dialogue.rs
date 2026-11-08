//! The dialogue window view-model (ui.md "Dialogue window view-model").
//!
//! Data flow, one player step:
//!
//! 1. Input: [`DialogueView::nav`] (keyboard/gamepad) or [`DialogueView::activate`]
//!    (pointer) returns a [`DialogueRequest`].
//! 2. The host applies it ([`DialogueRequest::apply`] on its `DialogueSession`) and feeds
//!    the outputs back: [`DialogueView::apply_outputs`]. Result scripts run; their
//!    `RpgEvent`s go to [`DialogueView::apply_events`] (journal notices).
//! 3. [`DialogueView::refresh`] (or [`DialogueView::refresh_from`]) syncs mode, choices and
//!    the topic list with the session and the known topics.
//!
//! Action buttons (services, content menus) are host data: [`DialogueView::set_actions`].
//! Activating one yields [`DialogueRequest::Action`], which the host routes itself (it is not
//! a session step).

use std::collections::vec_deque;
use std::collections::{BTreeSet, VecDeque};

use serde::{Deserialize, Serialize};
use sf_rpg::service::{ServiceOffer, ServiceRequest};
use sf_rpg::{
    Choice, DialogueDb, DialogueError, DialogueOutput, DialogueRules, DialogueSession, RecordId,
    RpgEvent, SessionState, TopicKind, Topics,
};
use sf_ui::{ListFilter, ListModel, SortSpec, UiValue, ViewModel};

/// Scrollback length kept by [`DialogueView::new`] (oldest entries drop first).
pub const DEFAULT_MAX_ENTRIES: usize = 256;

/// What the player asked for. The host applies it to the session (a client sends it).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum DialogueRequest {
    Ask(RecordId),
    Choose(i32),
    Goodbye,
    /// An action button (`option: None`) or an option of its submenu. Not a session step:
    /// the host routes it ([`DialogueRequest::service_open`] for services, its own handler
    /// for content menus) and feeds any lines it produces back as usual.
    Action {
        action: ActionId,
        option: Option<ActionId>,
    },
}

impl DialogueRequest {
    /// Applies the request to a session (host side). [`DialogueRequest::Action`] is not a
    /// session step: no outputs, the session is untouched.
    pub fn apply(
        &self,
        session: &mut DialogueSession,
        db: &DialogueDb,
        rules: &dyn DialogueRules,
    ) -> Result<Vec<DialogueOutput>, DialogueError> {
        match self {
            Self::Ask(topic) => session.ask(db, rules, topic),
            Self::Choose(value) => session.choose(db, rules, *value),
            Self::Goodbye => {
                session.goodbye();
                Ok(Vec::new())
            }
            Self::Action { .. } => Ok(Vec::new()),
        }
    }

    /// The service request a pressed service button stands for: an action without an option
    /// whose id is the kind of one of `offers` (buttons made by [`DialogueAction::from_offer`]).
    pub fn service_open(&self, offers: &[ServiceOffer]) -> Option<ServiceRequest> {
        let Self::Action {
            action,
            option: None,
        } = self
        else {
            return None;
        };
        let offer = offers.iter().find(|o| o.kind.0 == action.0)?;
        Some(ServiceRequest::Open {
            provider: offer.provider,
            kind: offer.kind.clone(),
        })
    }
}

/// A host-chosen id for an action button or a submenu option (opaque to the view).
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct ActionId(pub RecordId);

impl ActionId {
    /// An id from its text (lowercased, as [`RecordId::new`]).
    pub fn new(id: &str) -> Self {
        Self(RecordId::new(id))
    }
}

/// One option of an action's submenu (one level: options have no submenu).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ActionOption {
    pub id: ActionId,
    /// Display text, already localised by the host.
    pub label: String,
    /// Disabled options are shown greyed and take no focus.
    pub enabled: bool,
}

impl ActionOption {
    /// An enabled option.
    pub fn new(id: &str, label: impl Into<String>) -> Self {
        Self {
            id: ActionId::new(id),
            label: label.into(),
            enabled: true,
        }
    }

    /// Sets [`ActionOption::enabled`].
    #[must_use]
    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }
}

impl ViewModel for ActionOption {
    const KIND: &'static str = "dialogue.action_option";

    fn fields() -> &'static [&'static str] {
        &["label", "enabled"]
    }

    fn field(&self, name: &str) -> Option<UiValue> {
        match name {
            "label" => Some(self.label.as_str().into()),
            "enabled" => Some(self.enabled.into()),
            _ => None,
        }
    }
}

/// An action button above the topic list, supplied by the host (services first, then content
/// menus, in the order given). With a non-empty `submenu`, activating it opens the submenu
/// instead of sending a request.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct DialogueAction {
    pub id: ActionId,
    /// Display text, already localised by the host.
    pub label: String,
    /// Disabled buttons are shown greyed and take no focus.
    pub enabled: bool,
    pub submenu: Vec<ActionOption>,
}

impl DialogueAction {
    /// An enabled button without a submenu.
    pub fn new(id: &str, label: impl Into<String>) -> Self {
        Self {
            id: ActionId::new(id),
            label: label.into(),
            enabled: true,
            submenu: Vec::new(),
        }
    }

    /// The button of a service offer: its id is the offer's kind, so
    /// [`DialogueRequest::service_open`] maps the press back. `label` is the offer's
    /// `LabelRef` resolved by the host.
    pub fn from_offer(offer: &ServiceOffer, label: impl Into<String>) -> Self {
        Self {
            id: ActionId(offer.kind.0.clone()),
            label: label.into(),
            enabled: true,
            submenu: Vec::new(),
        }
    }

    /// Sets [`DialogueAction::enabled`].
    #[must_use]
    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }

    /// Sets the submenu options.
    #[must_use]
    pub fn with_submenu(mut self, options: Vec<ActionOption>) -> Self {
        self.submenu = options;
        self
    }

    /// Can take focus and be pressed (while talking): enabled, and a submenu, if any, has an
    /// enabled option. Screens grey the button otherwise.
    pub fn is_live(&self) -> bool {
        self.enabled && (self.submenu.is_empty() || self.submenu.iter().any(|o| o.enabled))
    }

    fn option(&self, id: &ActionId) -> Option<&ActionOption> {
        self.submenu.iter().find(|o| o.id == *id)
    }
}

impl ViewModel for DialogueAction {
    const KIND: &'static str = "dialogue.action";

    fn fields() -> &'static [&'static str] {
        &["label", "enabled", "has_submenu"]
    }

    /// `enabled` is [`DialogueAction::is_live`].
    fn field(&self, name: &str) -> Option<UiValue> {
        match name {
            "label" => Some(self.label.as_str().into()),
            "enabled" => Some(self.is_live().into()),
            "has_submenu" => Some((!self.submenu.is_empty()).into()),
            _ => None,
        }
    }
}

/// What the window allows, from the session state.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DialogueMode {
    /// Topics, links and goodbye are live.
    Talking,
    /// Only the offered choices are live.
    Choosing,
    /// A result forced goodbye: only goodbye is live.
    GoodbyeOnly,
    /// The conversation is over; the screen closes.
    Ended,
}

impl DialogueMode {
    fn of(state: &SessionState) -> Self {
        match state {
            SessionState::Talking => Self::Talking,
            SessionState::Choosing(_) => Self::Choosing,
            SessionState::Goodbye => Self::GoodbyeOnly,
            SessionState::Ended => Self::Ended,
        }
    }

    /// Binding value (`dialogue.mode`), also a CSS attribute value.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Talking => "talking",
            Self::Choosing => "choosing",
            Self::GoodbyeOnly => "goodbye",
            Self::Ended => "ended",
        }
    }
}

/// A scrollback entry's stable id (monotonic; survives trimming of older entries).
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EntryId(pub u64);

/// How a topic link looks and whether it can be followed.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LinkState {
    /// In the topic list and not asked yet.
    New,
    /// In the topic list and already asked (here or with anyone).
    Known,
    /// This speaker has nothing on it: styled, not followable.
    Unavailable,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TopicLink {
    pub topic: RecordId,
    pub state: LinkState,
}

/// A piece of a line: plain text or a topic hyperlink (one `TextSpan` on the Bevy side).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TextRun {
    pub text: String,
    pub link: Option<TopicLink>,
}

/// A shown response.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ViewLine {
    pub topic: RecordId,
    pub response: RecordId,
    /// The topic name for conversation topics; `None` for greetings and service replies.
    pub heading: Option<String>,
    /// The rendered text in order; concatenated they give the full text.
    pub runs: Vec<TextRun>,
}

impl ViewLine {
    pub fn text(&self) -> String {
        self.runs.iter().map(|r| r.text.as_str()).collect()
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum EntryKind {
    Line(ViewLine),
    /// The player's pick, echoed so the scrollback reads as a conversation.
    Chosen {
        label: String,
    },
    /// "Your journal has been updated": the screen formats it (Fluent) and may also toast
    /// it. Lists the quests of the entries added (none for quest-less notes).
    JournalUpdated {
        quests: Vec<RecordId>,
    },
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Entry {
    pub id: EntryId,
    pub kind: EntryKind,
}

/// A row of the topic list.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TopicRow {
    pub name: String,
    /// Not asked yet: no line heard under it in the known topics and not asked in this
    /// conversation (the "unexplored option" highlight).
    pub new: bool,
    /// Only while talking (choices and forced goodbye lock the list).
    pub enabled: bool,
}

impl ViewModel for TopicRow {
    const KIND: &'static str = "dialogue.topic";

    fn fields() -> &'static [&'static str] {
        &["name", "new", "enabled"]
    }

    fn field(&self, name: &str) -> Option<UiValue> {
        match name {
            "name" => Some(self.name.as_str().into()),
            "new" => Some(self.new.into()),
            "enabled" => Some(self.enabled.into()),
            _ => None,
        }
    }
}

/// The keyboard/gamepad focus. Pointer hover and clicks move it too.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DialogueFocus {
    Topic(RecordId),
    /// A link run in a scrollback line (the link cursor).
    Link {
        entry: EntryId,
        run: usize,
    },
    /// An index into [`DialogueView::choices`].
    Choice(usize),
    Goodbye,
    /// An action button ([`DialogueView::actions`]), above the topics in the list region.
    Action(ActionId),
    /// An option of the open submenu ([`DialogueView::submenu`]).
    SubAction(ActionId),
}

/// Semantic navigation input. A screen maps its UI actions to these (stock mapping:
/// up/down = `Prev`/`Next`, left = `PrevRegion` (into the text's links), right =
/// `NextRegion` (back to the topic list), confirm, back).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DialogueNav {
    /// Previous item in the focused region (clamped, no wrap).
    Prev,
    Next,
    /// Into the text: the link cursor starts on the newest line that has a live link.
    PrevRegion,
    /// Back to the topic list (remembered row, or the row now at its index).
    NextRegion,
    /// Activates the focused item.
    Confirm,
    /// Closes an open submenu; else goodbye while talking or when goodbye is forced; nothing
    /// while choosing.
    Back,
}

/// The dialogue window's state. Plain data; see the module docs for the update order.
pub struct DialogueView {
    speaker: String,
    mode: DialogueMode,
    entries: VecDeque<Entry>,
    next_entry: u64,
    max_entries: usize,
    topics: ListModel<RecordId, TopicRow>,
    /// The topic list's keys as of the last refresh (the session's available topics).
    available: BTreeSet<RecordId>,
    /// Available topics with a heard line in the known topics, as of the last refresh.
    heard: BTreeSet<RecordId>,
    /// Asked in this conversation (marks rows known before the host's `HearLine` lands).
    asked: BTreeSet<RecordId>,
    choices: Vec<Choice>,
    /// Host-supplied buttons above the topic list.
    actions: Vec<DialogueAction>,
    /// The action whose submenu is open (modal: only its options are live).
    submenu: Option<ActionId>,
    focus: Option<DialogueFocus>,
    /// The focus was placed by the view (a fallback), not by the player: it follows the best
    /// target until the player moves it (the greeting arrives before the topic list).
    focus_auto: bool,
    /// Last focus the player put in the topic list (a topic or goodbye) and its row index.
    list_focus: Option<(DialogueFocus, usize)>,
    scroll_to: Option<EntryId>,
    version: u64,
}

impl DialogueView {
    /// An empty window for a conversation with `speaker` (display name). Feed it the
    /// outputs of `DialogueSession::open`, then refresh.
    pub fn new(speaker: impl Into<String>) -> Self {
        let mut topics = ListModel::new();
        topics.set_sort(Some(SortSpec {
            field: "name".into(),
            descending: false,
        }));
        Self {
            speaker: speaker.into(),
            mode: DialogueMode::Talking,
            entries: VecDeque::new(),
            next_entry: 0,
            max_entries: DEFAULT_MAX_ENTRIES,
            topics,
            available: BTreeSet::new(),
            heard: BTreeSet::new(),
            asked: BTreeSet::new(),
            choices: Vec::new(),
            actions: Vec::new(),
            submenu: None,
            focus: None,
            focus_auto: true,
            list_focus: None,
            scroll_to: None,
            version: 0,
        }
    }

    /// Keeps at most `max` scrollback entries (at least 1).
    #[must_use]
    pub fn with_max_entries(mut self, max: usize) -> Self {
        self.max_entries = max.max(1);
        self
    }

    pub fn speaker(&self) -> &str {
        &self.speaker
    }

    pub fn mode(&self) -> DialogueMode {
        self.mode
    }

    /// The screen should close.
    pub fn is_closed(&self) -> bool {
        self.mode == DialogueMode::Ended
    }

    pub fn can_goodbye(&self) -> bool {
        matches!(self.mode, DialogueMode::Talking | DialogueMode::GoodbyeOnly)
    }

    /// Links and topics can be followed (talking).
    pub fn links_active(&self) -> bool {
        self.mode == DialogueMode::Talking
    }

    /// Changes on every change to the view, its topic list included (both counters only grow).
    pub fn version(&self) -> u64 {
        self.version + self.topics.version()
    }

    /// Scrollback, oldest first.
    pub fn entries(&self) -> vec_deque::Iter<'_, Entry> {
        self.entries.iter()
    }

    pub fn entry(&self, id: EntryId) -> Option<&Entry> {
        let first = self.entries.front()?.id.0;
        let index = usize::try_from(id.0.checked_sub(first)?).ok()?;
        self.entries.get(index)
    }

    /// The topic list (rows by key, per-row versions for binding).
    pub fn topics(&self) -> &ListModel<RecordId, TopicRow> {
        &self.topics
    }

    /// Topic keys in display order (sorted by name, filtered by the search).
    pub fn topic_order(&mut self) -> &[RecordId] {
        self.topics.view()
    }

    /// Filters the topic list by name (PC-dense search box); empty shows all.
    pub fn set_topic_search(&mut self, search: &str) {
        self.topics.set_filter(ListFilter {
            search: search.to_owned(),
            search_fields: vec!["name".into()],
            ..ListFilter::default()
        });
        self.settle_focus();
    }

    pub fn choices(&self) -> &[Choice] {
        &self.choices
    }

    /// The action buttons, in display order (above the topic list).
    pub fn actions(&self) -> &[DialogueAction] {
        &self.actions
    }

    /// Replaces the action buttons (the host's offers and content menus; none by default).
    /// They are live only while talking ([`DialogueView::links_active`]). An open submenu
    /// whose action is gone or no longer live closes.
    pub fn set_actions(&mut self, actions: Vec<DialogueAction>) {
        if actions == self.actions {
            return;
        }
        self.actions = actions;
        self.version += 1;
        self.settle_focus();
    }

    /// The action whose submenu is open. While open, only its enabled options take focus or
    /// activate; `Back` (or [`DialogueView::close_submenu`]) closes it.
    pub fn submenu(&self) -> Option<&DialogueAction> {
        let open = self.submenu.as_ref()?;
        self.action(open)
    }

    /// Closes the open submenu, focus back on its button; `false` when none was open.
    pub fn close_submenu(&mut self) -> bool {
        let Some(open) = self.submenu.take() else {
            return false;
        };
        self.version += 1;
        let parent = DialogueFocus::Action(open);
        if self.focus_valid(&parent) {
            self.move_focus(Some(parent), false);
        } else {
            self.settle_focus();
        }
        true
    }

    pub fn focus(&self) -> Option<&DialogueFocus> {
        self.focus.as_ref()
    }

    /// The entry the screen should scroll into view (a new line from its top, or the line
    /// under the link cursor); taken once.
    pub fn take_scroll_request(&mut self) -> Option<EntryId> {
        self.scroll_to.take()
    }

    // ------------------------------------------------------------ session → view

    /// Appends the shown lines of a session step. Other outputs are the host's business.
    pub fn apply_outputs(&mut self, db: &DialogueDb, outputs: &[DialogueOutput]) {
        for out in outputs {
            let DialogueOutput::Line {
                topic,
                response,
                text,
            } = out
            else {
                continue;
            };
            let heading = db
                .topic(topic)
                .filter(|t| t.kind == TopicKind::Conversation)
                .map(|t| t.name.clone());
            if heading.is_some() {
                self.mark_asked(topic);
            }
            let runs = self.split_runs(db, text);
            let id = self.push(EntryKind::Line(ViewLine {
                topic: topic.clone(),
                response: response.clone(),
                heading,
                runs,
            }));
            self.scroll_to = Some(id);
        }
    }

    /// Journal notices from the events of a step (consecutive additions share one notice).
    pub fn apply_events(&mut self, events: &[RpgEvent]) {
        for event in events {
            let RpgEvent::JournalAdded { quest, .. } = event else {
                continue;
            };
            if let Some(Entry {
                kind: EntryKind::JournalUpdated { quests },
                ..
            }) = self.entries.back_mut()
            {
                if let Some(q) = quest.as_ref().filter(|q| !quests.contains(q)) {
                    quests.push(q.clone());
                    self.version += 1;
                }
            } else {
                self.push(EntryKind::JournalUpdated {
                    quests: quest.iter().cloned().collect(),
                });
            }
        }
    }

    /// Syncs mode, choices and topics. `available` is the session's topic list
    /// (`DialogueSession::available_topics`, sent by the host to a client).
    pub fn refresh(
        &mut self,
        db: &DialogueDb,
        state: &SessionState,
        available: &[RecordId],
        known: &Topics,
    ) {
        let mode = DialogueMode::of(state);
        let choices = match state {
            SessionState::Choosing(choices) => choices.clone(),
            _ => Vec::new(),
        };
        if mode != self.mode || choices != self.choices {
            self.mode = mode;
            self.choices = choices;
            self.version += 1;
        }
        let now: BTreeSet<RecordId> = available
            .iter()
            .filter(|t| db.topic(t).is_some())
            .cloned()
            .collect();
        for gone in self.available.difference(&now) {
            self.topics.remove(gone);
        }
        self.heard = now
            .iter()
            .filter(|t| known.get(t).is_some_and(|k| !k.heard.is_empty()))
            .cloned()
            .collect();
        self.available = now;
        let enabled = mode == DialogueMode::Talking;
        for topic in available {
            let Some(entry) = db.topic(topic) else {
                continue;
            };
            let row = TopicRow {
                name: entry.name.clone(),
                new: self.is_new(topic),
                enabled,
            };
            self.topics.upsert(topic.clone(), row);
        }
        self.restyle_links();
        self.settle_focus();
    }

    /// [`DialogueView::refresh`] straight from a host-side session.
    pub fn refresh_from(
        &mut self,
        session: &DialogueSession,
        db: &DialogueDb,
        rules: &dyn DialogueRules,
        known: &Topics,
    ) {
        let available = session.available_topics(db, rules, known);
        self.refresh(db, session.state(), &available, known);
    }

    // ------------------------------------------------------------ input → requests

    pub fn nav(&mut self, nav: DialogueNav) -> Option<DialogueRequest> {
        match nav {
            DialogueNav::Confirm => {
                let focus = self.focus.clone()?;
                self.activate(focus)
            }
            DialogueNav::Back => {
                if self.close_submenu() {
                    return None;
                }
                self.can_goodbye().then_some(DialogueRequest::Goodbye)
            }
            DialogueNav::PrevRegion | DialogueNav::NextRegion if self.submenu.is_some() => None,
            DialogueNav::Prev => {
                self.step(false);
                None
            }
            DialogueNav::Next => {
                self.step(true);
                None
            }
            DialogueNav::PrevRegion => {
                self.enter_text();
                None
            }
            DialogueNav::NextRegion => {
                if matches!(self.focus, Some(DialogueFocus::Link { .. })) {
                    let target = self.list_fallback();
                    self.move_focus(Some(target), false);
                }
                None
            }
        }
    }

    /// Activates `target` (a click, or confirm on the focus): moves the focus there and
    /// returns the request. `None` when the target isn't live in this mode, and for an
    /// action with a submenu, which opens it (focus on its first enabled option). An option
    /// closes the submenu (focus back on its button) and returns
    /// `Action { action, option: Some(..) }`.
    pub fn activate(&mut self, target: DialogueFocus) -> Option<DialogueRequest> {
        if !self.focus_valid(&target) {
            return None;
        }
        let request = match &target {
            DialogueFocus::Action(action) => {
                let action = action.clone();
                self.move_focus(Some(target.clone()), false);
                if self.action(&action).is_some_and(|a| !a.submenu.is_empty()) {
                    self.open_submenu(action);
                    return None;
                }
                return Some(DialogueRequest::Action {
                    action,
                    option: None,
                });
            }
            DialogueFocus::SubAction(option) => {
                let option = option.clone();
                let action = self.submenu.take()?;
                self.version += 1;
                self.move_focus(Some(DialogueFocus::Action(action.clone())), false);
                return Some(DialogueRequest::Action {
                    action,
                    option: Some(option),
                });
            }
            DialogueFocus::Topic(topic) => DialogueRequest::Ask(topic.clone()),
            DialogueFocus::Link { entry, run } => {
                DialogueRequest::Ask(self.link_at(*entry, *run)?.topic.clone())
            }
            DialogueFocus::Choice(index) => DialogueRequest::Choose(self.choices[*index].value),
            DialogueFocus::Goodbye => DialogueRequest::Goodbye,
        };
        self.move_focus(Some(target), false);
        match &request {
            DialogueRequest::Ask(topic) => self.mark_asked(topic),
            DialogueRequest::Choose(value) => {
                let label = self
                    .choices
                    .iter()
                    .find(|c| c.value == *value)
                    .map(|c| c.label.clone());
                self.push(EntryKind::Chosen {
                    label: label.unwrap_or_default(),
                });
            }
            DialogueRequest::Goodbye | DialogueRequest::Action { .. } => {}
        }
        Some(request)
    }

    /// Moves the focus (pointer hover); `false` when the target isn't live.
    pub fn set_focus(&mut self, target: DialogueFocus) -> bool {
        let valid = self.focus_valid(&target);
        if valid {
            self.move_focus(Some(target), false);
        }
        valid
    }

    // ------------------------------------------------------------ internals

    fn push(&mut self, kind: EntryKind) -> EntryId {
        let id = EntryId(self.next_entry);
        self.next_entry += 1;
        self.entries.push_back(Entry { id, kind });
        while self.entries.len() > self.max_entries {
            self.entries.pop_front();
        }
        self.version += 1;
        self.settle_focus();
        id
    }

    fn split_runs(&self, db: &DialogueDb, text: &str) -> Vec<TextRun> {
        let plain = |s: &str| TextRun {
            text: s.to_owned(),
            link: None,
        };
        let Some(linker) = db.linker() else {
            return vec![plain(text)];
        };
        let mut runs = Vec::new();
        let mut at = 0;
        for span in linker.link(text) {
            if span.range.start > at {
                runs.push(plain(&text[at..span.range.start]));
            }
            runs.push(TextRun {
                text: text[span.range.clone()].to_owned(),
                link: Some(TopicLink {
                    state: self.link_state(&span.topic),
                    topic: span.topic,
                }),
            });
            at = span.range.end;
        }
        if at < text.len() || runs.is_empty() {
            runs.push(plain(&text[at..]));
        }
        runs
    }

    fn is_new(&self, topic: &RecordId) -> bool {
        is_new(&self.asked, &self.heard, topic)
    }

    fn link_state(&self, topic: &RecordId) -> LinkState {
        link_state(&self.available, &self.asked, &self.heard, topic)
    }

    fn mark_asked(&mut self, topic: &RecordId) {
        if !self.asked.insert(topic.clone()) {
            return;
        }
        if let Some(row) = self.topics.get(topic).filter(|r| r.new) {
            let row = TopicRow {
                new: false,
                ..row.clone()
            };
            self.topics.upsert(topic.clone(), row);
        }
        self.restyle_links();
    }

    fn restyle_links(&mut self) {
        let mut changed = false;
        for entry in &mut self.entries {
            let EntryKind::Line(line) = &mut entry.kind else {
                continue;
            };
            for link in line.runs.iter_mut().filter_map(|r| r.link.as_mut()) {
                let state = link_state(&self.available, &self.asked, &self.heard, &link.topic);
                if link.state != state {
                    link.state = state;
                    changed = true;
                }
            }
        }
        if changed {
            self.version += 1;
        }
    }

    fn link_at(&self, entry: EntryId, run: usize) -> Option<&TopicLink> {
        let EntryKind::Line(line) = &self.entry(entry)?.kind else {
            return None;
        };
        line.runs.get(run)?.link.as_ref()
    }

    /// Followable links in reading order.
    fn link_items(&self) -> Vec<DialogueFocus> {
        let mut items = Vec::new();
        for entry in &self.entries {
            let EntryKind::Line(line) = &entry.kind else {
                continue;
            };
            for (run, r) in line.runs.iter().enumerate() {
                if r.link
                    .as_ref()
                    .is_some_and(|l| l.state != LinkState::Unavailable)
                {
                    items.push(DialogueFocus::Link {
                        entry: entry.id,
                        run,
                    });
                }
            }
        }
        items
    }

    /// Live action buttons, then topic rows in display order, then goodbye.
    fn list_items(&mut self) -> Vec<DialogueFocus> {
        let mut items: Vec<DialogueFocus> =
            self.live_actions().into_iter().map(DialogueFocus::Action).collect();
        items.extend(self.topics.view().iter().cloned().map(DialogueFocus::Topic));
        items.push(DialogueFocus::Goodbye);
        items
    }

    fn action(&self, id: &ActionId) -> Option<&DialogueAction> {
        self.actions.iter().find(|a| a.id == *id)
    }

    /// Ids of the live action buttons, in display order.
    fn live_actions(&self) -> Vec<ActionId> {
        self.actions
            .iter()
            .filter(|a| a.is_live())
            .map(|a| a.id.clone())
            .collect()
    }

    /// The enabled options of `action`'s submenu, as focus targets.
    fn option_items(&self, action: &ActionId) -> Vec<DialogueFocus> {
        self.action(action)
            .into_iter()
            .flat_map(|a| a.submenu.iter())
            .filter(|o| o.enabled)
            .map(|o| DialogueFocus::SubAction(o.id.clone()))
            .collect()
    }

    /// Opens `action`'s submenu, focus on its first enabled option.
    fn open_submenu(&mut self, action: ActionId) {
        let first = self.option_items(&action).into_iter().next();
        self.submenu = Some(action);
        self.version += 1;
        if first.is_some() {
            self.move_focus(first, false);
        }
    }

    /// Closes the submenu when it can't stay open: not talking, or its action is gone, has
    /// no submenu any more, or isn't live.
    fn check_submenu(&mut self) {
        let Some(open) = &self.submenu else {
            return;
        };
        let keep = self.mode == DialogueMode::Talking
            && self
                .action(open)
                .is_some_and(|a| !a.submenu.is_empty() && a.is_live());
        if !keep {
            self.submenu = None;
            self.version += 1;
        }
    }

    fn focus_valid(&mut self, focus: &DialogueFocus) -> bool {
        if let Some(open) = &self.submenu {
            return self.mode == DialogueMode::Talking
                && matches!(focus, DialogueFocus::SubAction(option)
                    if self.action(open).and_then(|a| a.option(option)).is_some_and(|o| o.enabled));
        }
        match (self.mode, focus) {
            (DialogueMode::Talking, DialogueFocus::Action(action)) => {
                self.action(action).is_some_and(DialogueAction::is_live)
            }
            (DialogueMode::Talking, DialogueFocus::Topic(topic)) => {
                self.topics.view().contains(topic)
            }
            (DialogueMode::Talking, DialogueFocus::Link { entry, run }) => self
                .link_at(*entry, *run)
                .is_some_and(|l| l.state != LinkState::Unavailable),
            (DialogueMode::Talking | DialogueMode::GoodbyeOnly, DialogueFocus::Goodbye) => true,
            (DialogueMode::Choosing, DialogueFocus::Choice(index)) => *index < self.choices.len(),
            _ => false,
        }
    }

    /// Keeps the focus on something live after any change.
    fn settle_focus(&mut self) {
        self.check_submenu();
        if !self.focus_auto
            && let Some(focus) = self.focus.clone()
            && self.focus_valid(&focus)
        {
            return;
        }
        let next = match self.mode {
            DialogueMode::Ended => None,
            DialogueMode::GoodbyeOnly => Some(DialogueFocus::Goodbye),
            DialogueMode::Choosing => {
                (!self.choices.is_empty()).then_some(DialogueFocus::Choice(0))
            }
            DialogueMode::Talking => match self.submenu.clone() {
                Some(open) => self.option_items(&open).into_iter().next(),
                None => Some(self.list_fallback()),
            },
        };
        self.move_focus(next, true);
    }

    /// Where focus lands in the topic list: the remembered row, else the row now at its
    /// index (clamped), else the first row, else goodbye. A remembered action button that is
    /// no longer live gives the live button now at its index (clamped), else the first row.
    fn list_fallback(&mut self) -> DialogueFocus {
        let remembered = self.list_focus.clone();
        let live = self.live_actions();
        let order = self.topics.view();
        let at = |index: usize| {
            order
                .get(index)
                .or(order.last())
                .map_or(DialogueFocus::Goodbye, |t| DialogueFocus::Topic(t.clone()))
        };
        match remembered {
            Some((DialogueFocus::Topic(topic), _)) if order.contains(&topic) => {
                DialogueFocus::Topic(topic)
            }
            Some((DialogueFocus::Topic(_), index)) => at(index),
            Some((DialogueFocus::Action(action), _)) if live.contains(&action) => {
                DialogueFocus::Action(action)
            }
            Some((DialogueFocus::Action(_), index)) => live
                .get(index)
                .or(live.last())
                .map_or_else(|| at(0), |a| DialogueFocus::Action(a.clone())),
            Some((focus, _)) => focus,
            None => at(0),
        }
    }

    fn move_focus(&mut self, focus: Option<DialogueFocus>, auto: bool) {
        self.focus_auto = auto;
        if self.focus == focus {
            return;
        }
        match &focus {
            _ if auto => {}
            Some(DialogueFocus::Topic(topic)) => {
                let index = self
                    .topics
                    .view()
                    .iter()
                    .position(|t| t == topic)
                    .unwrap_or(0);
                self.list_focus = Some((DialogueFocus::Topic(topic.clone()), index));
            }
            Some(DialogueFocus::Action(action)) => {
                let index = self
                    .live_actions()
                    .iter()
                    .position(|a| a == action)
                    .unwrap_or(0);
                self.list_focus = Some((DialogueFocus::Action(action.clone()), index));
            }
            Some(DialogueFocus::Goodbye) if self.mode == DialogueMode::Talking => {
                self.list_focus = Some((DialogueFocus::Goodbye, self.topics.view().len()));
            }
            Some(DialogueFocus::Link { entry, .. }) => self.scroll_to = Some(*entry),
            _ => {}
        }
        self.focus = focus;
        self.version += 1;
    }

    fn step(&mut self, forward: bool) {
        let items = if let Some(open) = self.submenu.clone() {
            self.option_items(&open)
        } else {
            match (self.mode, &self.focus) {
                (DialogueMode::Talking, Some(DialogueFocus::Link { .. })) => self.link_items(),
                (DialogueMode::Talking, _) => self.list_items(),
                (DialogueMode::Choosing, _) => {
                    (0..self.choices.len()).map(DialogueFocus::Choice).collect()
                }
                (DialogueMode::GoodbyeOnly, _) => vec![DialogueFocus::Goodbye],
                (DialogueMode::Ended, _) => Vec::new(),
            }
        };
        if items.is_empty() {
            return;
        }
        let target = match self
            .focus
            .as_ref()
            .and_then(|f| items.iter().position(|i| i == f))
        {
            Some(at) if forward => (at + 1).min(items.len() - 1),
            Some(at) => at.saturating_sub(1),
            None => 0,
        };
        self.move_focus(Some(items[target].clone()), false);
    }

    /// Link cursor into the text: the first live link of the newest line that has one.
    fn enter_text(&mut self) {
        if self.mode != DialogueMode::Talking
            || matches!(self.focus, Some(DialogueFocus::Link { .. }))
        {
            return;
        }
        let target = self.entries.iter().rev().find_map(|entry| {
            let EntryKind::Line(line) = &entry.kind else {
                return None;
            };
            let run = line.runs.iter().position(|r| {
                r.link
                    .as_ref()
                    .is_some_and(|l| l.state != LinkState::Unavailable)
            })?;
            Some(DialogueFocus::Link {
                entry: entry.id,
                run,
            })
        });
        if target.is_some() {
            self.move_focus(target, false);
        }
    }
}

fn is_new(asked: &BTreeSet<RecordId>, heard: &BTreeSet<RecordId>, topic: &RecordId) -> bool {
    !asked.contains(topic) && !heard.contains(topic)
}

fn link_state(
    available: &BTreeSet<RecordId>,
    asked: &BTreeSet<RecordId>,
    heard: &BTreeSet<RecordId>,
    topic: &RecordId,
) -> LinkState {
    if !available.contains(topic) {
        LinkState::Unavailable
    } else if is_new(asked, heard, topic) {
        LinkState::New
    } else {
        LinkState::Known
    }
}

impl ViewModel for DialogueView {
    const KIND: &'static str = "dialogue";

    fn fields() -> &'static [&'static str] {
        &["speaker", "mode", "can_goodbye", "links_active"]
    }

    fn field(&self, name: &str) -> Option<UiValue> {
        match name {
            "speaker" => Some(self.speaker.as_str().into()),
            "mode" => Some(self.mode.as_str().into()),
            "can_goodbye" => Some(self.can_goodbye().into()),
            "links_active" => Some(self.links_active().into()),
            _ => None,
        }
    }
}
