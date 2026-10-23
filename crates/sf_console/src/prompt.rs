//! The input line of a console front end: text with a cursor, history browsing and
//! Tab completion, driven by key-agnostic [`Edit`]s (the front end maps keys to them).

use crate::{Completion, Console};

/// One editing action on the [`Prompt`].
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Edit {
    /// Typed text at the cursor (control characters are dropped).
    Insert(String),
    /// Delete the character before the cursor.
    Backspace,
    /// Delete the character after the cursor.
    Delete,
    /// Delete the word before the cursor (Ctrl+Backspace, Ctrl+W).
    DeleteWordBack,
    Left,
    Right,
    WordLeft,
    WordRight,
    Home,
    End,
    /// Empty the line (Escape on a non-empty line).
    Clear,
    /// Previous history entry (Up); the unsent draft is kept.
    Older,
    /// Next history entry (Down), back to the draft past the newest.
    Newer,
    /// Complete the word before the cursor (Tab).
    Complete,
    /// Send the line (Enter): [`Prompt::apply`] returns it, trimmed, and empties the prompt.
    Submit,
}

/// Text and cursor of the line being typed.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Prompt {
    text: String,
    /// Byte offset, always on a char boundary.
    cursor: usize,
}

impl Prompt {
    pub fn text(&self) -> &str {
        &self.text
    }

    /// Byte offset of the cursor in [`Prompt::text`].
    pub fn cursor(&self) -> usize {
        self.cursor
    }

    /// Replaces the line; the cursor goes to the end.
    pub fn set(&mut self, text: &str) {
        text.clone_into(&mut self.text);
        self.cursor = self.text.len();
    }

    /// Completion candidates for the word before the cursor (the popup's content).
    pub fn completion<C>(&self, console: &Console<C>) -> Completion {
        console.complete(&self.text[..self.cursor])
    }

    /// Applies one edit. Returns the line on [`Edit::Submit`] (the caller submits it, which
    /// records it in the history).
    pub fn apply<C>(&mut self, edit: Edit, console: &mut Console<C>) -> Option<String> {
        match edit {
            Edit::Insert(s) => {
                let s: String = s.chars().filter(|c| !c.is_control()).collect();
                self.text.insert_str(self.cursor, &s);
                self.cursor += s.len();
            }
            Edit::Backspace => {
                if let Some(prev) = self.prev_boundary(self.cursor) {
                    self.text.replace_range(prev..self.cursor, "");
                    self.cursor = prev;
                }
            }
            Edit::Delete => {
                if let Some(next) = self.next_boundary(self.cursor) {
                    self.text.replace_range(self.cursor..next, "");
                }
            }
            Edit::DeleteWordBack => {
                let start = self.word_start(self.cursor);
                self.text.replace_range(start..self.cursor, "");
                self.cursor = start;
            }
            Edit::Left => self.cursor = self.prev_boundary(self.cursor).unwrap_or(0),
            Edit::Right => {
                self.cursor = self.next_boundary(self.cursor).unwrap_or(self.text.len());
            }
            Edit::WordLeft => self.cursor = self.word_start(self.cursor),
            Edit::WordRight => self.cursor = self.word_end(self.cursor),
            Edit::Home => self.cursor = 0,
            Edit::End => self.cursor = self.text.len(),
            Edit::Clear => self.set(""),
            Edit::Older => {
                if let Some(line) = console.history_mut().older(&self.text) {
                    let line = line.to_owned();
                    self.set(&line);
                }
            }
            Edit::Newer => {
                if let Some(line) = console.history_mut().newer() {
                    let line = line.to_owned();
                    self.set(&line);
                }
            }
            Edit::Complete => {
                let head = self.completion(console).apply(&self.text[..self.cursor]);
                self.text.replace_range(..self.cursor, &head);
                self.cursor = head.len();
            }
            Edit::Submit => {
                let line = std::mem::take(&mut self.text);
                self.cursor = 0;
                // Completion leaves a trailing space; it means nothing to the parser.
                return Some(line.trim().to_owned());
            }
        }
        None
    }

    fn prev_boundary(&self, at: usize) -> Option<usize> {
        self.text[..at].char_indices().next_back().map(|(i, _)| i)
    }

    fn next_boundary(&self, at: usize) -> Option<usize> {
        self.text[at..].chars().next().map(|c| at + c.len_utf8())
    }

    /// Start of the word before `at` (skipping spaces first), shell style.
    fn word_start(&self, at: usize) -> usize {
        let head = self.text[..at].trim_end();
        head.char_indices()
            .rev()
            .find(|(_, c)| c.is_whitespace())
            .map_or(0, |(i, c)| i + c.len_utf8())
    }

    /// End of the word after `at` (skipping spaces first).
    fn word_end(&self, at: usize) -> usize {
        let tail = &self.text[at..];
        let skipped = tail.len() - tail.trim_start().len();
        let rest = &tail[skipped..];
        at + skipped + rest.find(char::is_whitespace).unwrap_or(rest.len())
    }
}
