//! Journal search (journal-ui.md "Search"): whitespace-separated terms, all required,
//! case-insensitive by `char::to_lowercase` (the `sf_ui::ListFilter` rule), with highlight
//! ranges into the original text.

use std::ops::Range;

/// A parsed search: folded terms, all of which must occur.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct SearchQuery {
    terms: Vec<String>,
}

impl SearchQuery {
    pub fn parse(text: &str) -> Self {
        let mut terms: Vec<String> = text.split_whitespace().map(fold).collect();
        terms.sort();
        terms.dedup();
        Self { terms }
    }

    pub fn is_empty(&self) -> bool {
        self.terms.is_empty()
    }

    pub fn terms(&self) -> &[String] {
        &self.terms
    }

    /// Highlight ranges (sorted, merged) of every term in `text` when all terms occur; `None`
    /// when one is missing or the query is empty.
    pub fn find(&self, text: &str) -> Option<Vec<Range<usize>>> {
        if self.terms.is_empty() {
            return None;
        }
        let folded = Folded::new(text);
        let mut hits = Vec::new();
        for term in &self.terms {
            let before = hits.len();
            hits.extend(folded.find_all(term));
            if hits.len() == before {
                return None;
            }
        }
        Some(merge(hits))
    }

    /// Whether every term occurs somewhere in `texts` taken together.
    pub fn matches_any_of<'a>(&self, texts: impl IntoIterator<Item = &'a str> + Clone) -> bool {
        !self.terms.is_empty()
            && self.terms.iter().all(|t| {
                texts
                    .clone()
                    .into_iter()
                    .any(|s| fold(s).contains(t.as_str()))
            })
    }
}

fn fold(s: &str) -> String {
    s.chars().flat_map(char::to_lowercase).collect()
}

/// Lowercased text with a map back to the original characters.
struct Folded {
    text: String,
    /// Per folded byte: the original character's byte range.
    origin: Vec<Range<usize>>,
}

impl Folded {
    fn new(original: &str) -> Self {
        let mut text = String::with_capacity(original.len());
        let mut origin = Vec::with_capacity(original.len());
        for (at, c) in original.char_indices() {
            let span = at..at + c.len_utf8();
            for lower in c.to_lowercase() {
                text.push(lower);
                origin.extend(std::iter::repeat_n(span.clone(), lower.len_utf8()));
            }
        }
        Self { text, origin }
    }

    /// Original byte ranges of the non-overlapping occurrences of `term` (already folded).
    fn find_all<'a>(&'a self, term: &'a str) -> impl Iterator<Item = Range<usize>> + 'a {
        self.text.match_indices(term).map(|(at, m)| {
            let start = self.origin[at].start;
            let end = self.origin[at + m.len() - 1].end;
            start..end
        })
    }
}

fn merge(mut ranges: Vec<Range<usize>>) -> Vec<Range<usize>> {
    ranges.sort_by_key(|r| (r.start, r.end));
    let mut out: Vec<Range<usize>> = Vec::with_capacity(ranges.len());
    for r in ranges {
        match out.last_mut() {
            Some(last) if r.start <= last.end => last.end = last.end.max(r.end),
            _ => out.push(r),
        }
    }
    out
}

/// Clips sorted highlight ranges to a page slice (`range` of the same text).
pub fn highlights_in(highlights: &[Range<usize>], range: &Range<usize>) -> Vec<Range<usize>> {
    highlights
        .iter()
        .filter(|h| h.start < range.end && h.end > range.start)
        .map(|h| h.start.max(range.start)..h.end.min(range.end))
        .collect()
}
