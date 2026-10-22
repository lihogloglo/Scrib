//! Line tokenizer: whitespace-separated words, double quotes, `;` between commands.

/// One word of a command line, with its byte span in the line (quotes included).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Token {
    /// The word with quotes removed and escapes (`\"`, `\\`) resolved.
    pub text: String,
    pub start: usize,
    pub end: usize,
    /// Started with a quote that never closed (fine for completion, an error to run).
    pub unterminated: bool,
}

/// One command of a line: its tokens and the byte where it ends (its `;` or the line end).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Segment {
    pub tokens: Vec<Token>,
    pub end: usize,
}

/// Splits a line into commands (at unquoted `;`) and each command into tokens.
/// Empty commands are kept, so the last entry is always the text after the last `;`.
pub fn split(line: &str) -> Vec<Segment> {
    let mut commands = vec![Segment {
        tokens: Vec::new(),
        end: line.len(),
    }];
    let mut chars = line.char_indices().peekable();
    while let Some(&(start, c)) = chars.peek() {
        if c.is_whitespace() {
            chars.next();
            continue;
        }
        if c == ';' {
            chars.next();
            commands.last_mut().expect("never empty").end = start;
            commands.push(Segment {
                tokens: Vec::new(),
                end: line.len(),
            });
            continue;
        }
        let mut text = String::new();
        let mut end = line.len();
        let mut unterminated = false;
        if c == '"' {
            chars.next();
            unterminated = true;
            while let Some((i, c)) = chars.next() {
                match c {
                    '"' => {
                        end = i + 1;
                        unterminated = false;
                        break;
                    }
                    '\\' => match chars.peek() {
                        Some(&(_, n @ ('"' | '\\'))) => {
                            text.push(n);
                            chars.next();
                        }
                        _ => text.push('\\'),
                    },
                    _ => text.push(c),
                }
            }
        } else {
            while let Some(&(i, c)) = chars.peek() {
                if c.is_whitespace() || c == ';' || c == '"' {
                    end = i;
                    break;
                }
                text.push(c);
                chars.next();
            }
        }
        commands
            .last_mut()
            .expect("never empty")
            .tokens
            .push(Token {
                text,
                start,
                end,
                unterminated,
            });
    }
    commands
}
