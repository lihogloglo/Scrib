//! Input history (shell style) and the output log.

use std::collections::VecDeque;

/// Submitted lines, oldest first, with up/down navigation that keeps the unsent draft.
#[derive(Clone, Debug)]
pub struct History {
    entries: VecDeque<String>,
    capacity: usize,
    /// Index into `entries` while browsing; `None` at the draft.
    cursor: Option<usize>,
    draft: String,
}

impl History {
    pub fn new(capacity: usize) -> Self {
        Self {
            entries: VecDeque::new(),
            capacity: capacity.max(1),
            cursor: None,
            draft: String::new(),
        }
    }

    /// Records a submitted line: blank lines and repeats of the last line are skipped.
    /// Browsing restarts at the draft.
    pub fn push(&mut self, line: &str) {
        self.cursor = None;
        self.draft.clear();
        let line = line.trim();
        if line.is_empty() || self.entries.back().is_some_and(|last| last == line) {
            return;
        }
        if self.entries.len() == self.capacity {
            self.entries.pop_front();
        }
        self.entries.push_back(line.to_owned());
    }

    /// One step back (older). `current` is the input text, saved as the draft on the first step.
    /// Stays on the oldest entry; `None` when the history is empty.
    pub fn older(&mut self, current: &str) -> Option<&str> {
        let i = match self.cursor {
            None if self.entries.is_empty() => return None,
            None => {
                current.clone_into(&mut self.draft);
                self.entries.len() - 1
            }
            Some(i) => i.saturating_sub(1),
        };
        self.cursor = Some(i);
        Some(&self.entries[i])
    }

    /// One step forward (newer); past the newest entry it returns the draft.
    /// `None` when not browsing.
    pub fn newer(&mut self) -> Option<&str> {
        let i = self.cursor?;
        if i + 1 < self.entries.len() {
            self.cursor = Some(i + 1);
            Some(&self.entries[i + 1])
        } else {
            self.cursor = None;
            Some(&self.draft)
        }
    }

    /// Oldest first (to save between runs).
    pub fn entries(&self) -> impl ExactSizeIterator<Item = &str> {
        self.entries.iter().map(String::as_str)
    }

    /// Replaces the history with saved entries, oldest first (blank lines and repeats dropped).
    pub fn load<'a>(&mut self, entries: impl IntoIterator<Item = &'a str>) {
        self.entries.clear();
        for e in entries {
            self.push(e);
        }
    }
}

/// Severity of a log line.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Level {
    /// A submitted line, echoed back.
    Echo,
    Info,
    Warn,
    Error,
}

/// One output line. `seq` increases by one per line for the console's lifetime, so a front
/// end can draw only what it hasn't seen ([`Log::since`]), even after old lines were dropped.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Line {
    pub seq: u64,
    pub level: Level,
    pub text: String,
}

/// Bounded output log: the oldest lines drop when full.
#[derive(Clone, Debug)]
pub struct Log {
    lines: VecDeque<Line>,
    capacity: usize,
    next_seq: u64,
}

impl Log {
    pub fn new(capacity: usize) -> Self {
        Self {
            lines: VecDeque::new(),
            capacity: capacity.max(1),
            next_seq: 0,
        }
    }

    /// Appends text; each `\n`-separated line becomes its own [`Line`].
    pub fn push(&mut self, level: Level, text: &str) {
        for part in text.split('\n') {
            if self.lines.len() == self.capacity {
                self.lines.pop_front();
            }
            self.lines.push_back(Line {
                seq: self.next_seq,
                level,
                text: part.to_owned(),
            });
            self.next_seq += 1;
        }
    }

    pub fn info(&mut self, text: &str) {
        self.push(Level::Info, text);
    }

    pub fn warn(&mut self, text: &str) {
        self.push(Level::Warn, text);
    }

    pub fn error(&mut self, text: &str) {
        self.push(Level::Error, text);
    }

    /// Lines still held, oldest first.
    pub fn lines(&self) -> impl ExactSizeIterator<Item = &Line> {
        self.lines.iter()
    }

    /// Held lines with `seq >= seq`.
    pub fn since(&self, seq: u64) -> impl Iterator<Item = &Line> {
        let skip = self
            .lines
            .front()
            .map_or(0, |first| seq.saturating_sub(first.seq));
        self.lines
            .iter()
            .skip(usize::try_from(skip).unwrap_or(usize::MAX))
    }

    /// The `seq` the next line will get.
    pub fn next_seq(&self) -> u64 {
        self.next_seq
    }

    /// Drops every line (sequence numbers keep counting).
    pub fn clear(&mut self) {
        self.lines.clear();
    }
}
