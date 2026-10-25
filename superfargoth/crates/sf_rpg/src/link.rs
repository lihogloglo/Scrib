//! Topic hyperlinks in response text (rpg.md "Topic links").

use std::ops::Range;

use aho_corasick::{AhoCorasick, MatchKind};
use serde::{Deserialize, Serialize};
use sf_records::RecordId;
use thiserror::Error;

/// Word-boundary policy (adapter data). A boundary is the text edge or a
/// non-alphanumeric character.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct LinkRules {
    /// A match must start at a boundary ("ring" not inside "bring").
    pub word_start: bool,
    /// A match must end at a boundary ("ring" not inside "rings").
    pub word_end: bool,
}

/// A linked topic in a text: byte range plus topic id.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct TopicSpan {
    pub range: Range<usize>,
    pub topic: RecordId,
}

#[derive(Debug, Error)]
#[error("topic linker: {0}")]
pub struct LinkError(#[from] aho_corasick::BuildError);

/// Multi-pattern matcher over topic names: ASCII case-insensitive, leftmost-longest,
/// non-overlapping, with boundary checks.
#[derive(Clone, Debug)]
pub struct TopicLinker {
    automaton: AhoCorasick,
    topics: Vec<RecordId>,
    rules: LinkRules,
}

impl TopicLinker {
    /// `topics`: (id, display name). Empty names are skipped.
    pub fn new<'a>(topics: impl IntoIterator<Item = (RecordId, &'a str)>, rules: LinkRules) -> Result<Self, LinkError> {
        let (ids, names): (Vec<RecordId>, Vec<&str>) = topics.into_iter().filter(|(_, n)| !n.is_empty()).unzip();
        // Standard semantics so every overlapping candidate is seen; longest-first selection
        // happens after the boundary filter (a longer match failing a boundary must not hide a
        // shorter one that passes).
        let automaton =
            AhoCorasick::builder().ascii_case_insensitive(true).match_kind(MatchKind::Standard).build(&names)?;
        Ok(Self { automaton, topics: ids, rules })
    }

    pub fn rules(&self) -> LinkRules {
        self.rules
    }

    pub fn len(&self) -> usize {
        self.topics.len()
    }

    pub fn is_empty(&self) -> bool {
        self.topics.is_empty()
    }

    pub fn link(&self, text: &str) -> Vec<TopicSpan> {
        let boundary_before = |at: usize| text[..at].chars().next_back().is_none_or(|c| !c.is_alphanumeric());
        let boundary_after = |at: usize| text[at..].chars().next().is_none_or(|c| !c.is_alphanumeric());
        let mut found: Vec<(usize, usize, usize)> = self
            .automaton
            .find_overlapping_iter(text)
            .filter(|m| !self.rules.word_start || boundary_before(m.start()))
            .filter(|m| !self.rules.word_end || boundary_after(m.end()))
            .map(|m| (m.start(), m.end(), m.pattern().as_usize()))
            .collect();
        // Leftmost first, longest first at the same start, then pattern order.
        found.sort_by(|a, b| a.0.cmp(&b.0).then(b.1.cmp(&a.1)).then(a.2.cmp(&b.2)));
        let mut spans = Vec::new();
        let mut cursor = 0;
        for (start, end, pattern) in found {
            if start < cursor {
                continue;
            }
            spans.push(TopicSpan { range: start..end, topic: self.topics[pattern].clone() });
            cursor = end;
        }
        spans
    }
}
