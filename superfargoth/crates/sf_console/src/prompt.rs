//! The input line of a console front end: text with a cursor, history browsing, Tab
//! completion and a completion popup with a highlighted candidate, driven by key-agnostic
//! [`Edit`]s (the front end maps keys to them).

use crate::{Completion, Console};

/// One editing action on the [`Prompt`].
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Edit {
    /// Typed text at the cursor (control characters are dropped).
    Insert(String),
    /// Pasted text at the cursor, as one line: line breaks become `; ` (the command
    /// separator, so a pasted block runs line by line once Enter sends it), tabs become
    /// spaces, blank lines and a trailing line break are dropped.
    Paste(String),
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
    /// Empty the line.
    Clear,
    /// Escape on a non-empty line: closes the completion popup if it's shown, else
    /// empties the line.
    Cancel,
    /// Up: while the completion popup is shown, highlights the previous candidate (from
    /// none, the last); else the previous history entry (the unsent draft is kept).
    Older,
    /// Down: while the popup is shown, highlights the next candidate (from none, the
    /// first); else the next history entry, back to the draft past the newest.
    Newer,
    /// Complete the word before the cursor (Tab).
    Complete,
    /// Enter: with a highlighted popup candidate, puts it in the line (as Tab does with a
    /// single match) and sends nothing; else sends the line: [`Prompt::apply`] returns it,
    /// trimmed, and empties the prompt.
    Submit,
}

/// Text and cursor of the line being typed.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Prompt {
    text: String,
    /// Byte offset, always on a char boundary.
    cursor: usize,
    /// The highlighted popup candidate (index into [`Prompt::popup`]'s candidates).
    selected: Option<usize>,
    /// The popup stays closed until the next text edit (after history recall, Escape, Enter).
    popup_closed: bool,
}

impl Prompt {
    pub fn text(&self) -> &str {
        &self.text
    }

    /// Byte offset of the cursor in [`Prompt::text`].
    pub fn cursor(&self) -> usize {
        self.cursor
    }

    /// Replaces the line; the cursor goes to the end and the popup closes.
    pub fn set(&mut self, text: &str) {
        text.clone_into(&mut self.text);
        self.cursor = self.text.len();
        self.selected = None;
        self.popup_closed = true;
    }

    /// Completion candidates for the word before the cursor.
    pub fn completion<C>(&self, console: &Console<C>) -> Completion {
        console.complete(&self.text[..self.cursor])
    }

    /// The completion popup's content when it's shown: after a text edit, on a non-empty
    /// line, with something to offer (not a lone candidate typed in full). History recall,
    /// [`Edit::Cancel`] and [`Edit::Submit`] close it until the next text edit.
    pub fn popup<C>(&self, console: &Console<C>) -> Option<Completion> {
        if self.popup_closed || self.text.is_empty() {
            return None;
        }
        let completion = self.completion(console);
        let typed = &self.text[completion.start.min(self.cursor)..self.cursor];
        match completion.candidates.as_slice() {
            [] => None,
            [only] if only == typed => None,
            _ => Some(completion),
        }
    }

    /// The highlighted popup candidate (index into [`Prompt::popup`]'s candidates).
    pub fn selected(&self) -> Option<usize> {
        self.selected
    }

    /// Applies one edit. Returns the line on [`Edit::Submit`] (the caller submits it, which
    /// records it in the history).
    pub fn apply<C>(&mut self, edit: Edit, console: &mut Console<C>) -> Option<String> {
        // Any edit but a highlight move drops the highlight (the candidates may change).
        let selected = self.selected.take();
        if matches!(
            edit,
            Edit::Older | Edit::Newer | Edit::Submit | Edit::Cancel
        ) && let Some(popup) = self.popup(console)
            && self.on_popup(&edit, &popup, selected)
        {
            return None;
        }
        if matches!(
            edit,
            Edit::Insert(_)
                | Edit::Paste(_)
                | Edit::Backspace
                | Edit::Delete
                | Edit::DeleteWordBack
                | Edit::Complete
        ) {
            self.popup_closed = false;
        }
        match edit {
            Edit::Insert(s) => self.insert(&s),
            Edit::Paste(s) => self.insert(&paste_line(&s)),
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
            Edit::Clear | Edit::Cancel => self.set(""),
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
                self.popup_closed = true;
                // Completion leaves a trailing space; it means nothing to the parser.
                return Some(line.trim().to_owned());
            }
        }
        None
    }

    /// Up, Down, Enter and Escape while the popup is shown. False: Enter with nothing
    /// highlighted, which sends the line as usual.
    fn on_popup(&mut self, edit: &Edit, popup: &Completion, selected: Option<usize>) -> bool {
        let n = popup.candidates.len();
        match (edit, selected) {
            (Edit::Older, Some(i)) => self.selected = Some((i + n - 1) % n),
            (Edit::Older, None) => self.selected = Some(n - 1),
            (Edit::Newer, Some(i)) => self.selected = Some((i + 1) % n),
            (Edit::Newer, None) => self.selected = Some(0),
            (Edit::Submit, Some(i)) => {
                let chosen = Completion {
                    start: popup.start,
                    candidates: vec![popup.candidates[i.min(n - 1)].clone()],
                };
                let head = chosen.apply(&self.text[..self.cursor]);
                self.text.replace_range(..self.cursor, &head);
                self.cursor = head.len();
            }
            (Edit::Submit, None) => return false,
            _ => self.popup_closed = true,
        }
        true
    }

    fn insert(&mut self, s: &str) {
        let s: String = s.chars().filter(|c| !c.is_control()).collect();
        self.text.insert_str(self.cursor, &s);
        self.cursor += s.len();
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

/// A pasted text as one line ([`Edit::Paste`]).
fn paste_line(text: &str) -> String {
    text.lines()
        .map(|line| line.trim_end().replace('\t', " "))
        .filter(|line| !line.trim().is_empty())
        .collect::<Vec<_>>()
        .join("; ")
}
