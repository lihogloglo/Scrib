//! Lenient `MWScript` lexer. `MWScript` is line-based: every statement is one
//! line, so newlines are tokens. Keywords and names are case-insensitive;
//! words keep their source text and the parser compares lowercase.

use crate::diag::{Diagnostics, Rule};

/// Comparison operators.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum CmpOp {
    Eq,
    Ne,
    Lt,
    Le,
    Gt,
    Ge,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Tok {
    /// A bare word: keyword, name, ID or function (source text, any case).
    Word(String),
    /// A quoted string (quotes removed).
    Str(String),
    /// A number literal without a fraction.
    Int(i64),
    /// A number literal with a fraction.
    Float(f64),
    Arrow,
    Dot,
    Comma,
    LParen,
    RParen,
    Plus,
    Minus,
    Star,
    Slash,
    Cmp(CmpOp),
    Newline,
    Eof,
}

/// A token and its 1-based source line.
#[derive(Clone, Debug, PartialEq)]
pub struct Token {
    pub tok: Tok,
    pub line: u32,
    /// No whitespace between this token and the previous one.
    pub glued: bool,
}

fn is_word_char(c: char) -> bool {
    c.is_alphanumeric() || c == '_' || c == '\'' || c == '`' || !c.is_ascii()
}

/// Splits `text` into tokens, recording lexical leniencies.
#[allow(clippy::too_many_lines)]
// `push!` resets `ws` after every token; the last reset is never read.
#[allow(unused_assignments)]
pub fn lex(text: &str, diags: &mut Diagnostics) -> Vec<Token> {
    let chars: Vec<char> = text.chars().collect();
    let mut out = Vec::with_capacity(chars.len() / 3);
    let mut line = 1u32;
    let mut i = 0;
    let mut ws = true;
    macro_rules! push {
        ($tok:expr) => {{
            out.push(Token {
                tok: $tok,
                line,
                glued: !ws,
            });
            ws = false;
        }};
    }
    while i < chars.len() {
        let c = chars[i];
        match c {
            '\n' => {
                push!(Tok::Newline);
                line += 1;
                ws = true;
                i += 1;
            }
            ';' => {
                ws = true;
                while i < chars.len() && chars[i] != '\n' {
                    i += 1;
                }
            }
            c if c.is_whitespace() => {
                ws = true;
                i += 1;
            }
            '"' => {
                let start = i + 1;
                let mut j = start;
                while j < chars.len() && chars[j] != '"' && chars[j] != '\n' {
                    j += 1;
                }
                let s: String = chars[start..j].iter().collect();
                if j < chars.len() && chars[j] == '"' {
                    i = j + 1;
                } else {
                    diags.push(Rule::UnterminatedString, line, "");
                    i = j;
                }
                push!(Tok::Str(s));
            }
            '-' if chars.get(i + 1) == Some(&'>') => {
                push!(Tok::Arrow);
                i += 2;
            }
            '(' | ')' | ',' | '+' | '-' | '*' | '/' => {
                let tok = match c {
                    '(' => Tok::LParen,
                    ')' => Tok::RParen,
                    ',' => Tok::Comma,
                    '+' => Tok::Plus,
                    '-' => Tok::Minus,
                    '*' => Tok::Star,
                    _ => Tok::Slash,
                };
                push!(tok);
                i += 1;
            }
            '=' | '<' | '>' | '!' => {
                // Gather operator characters, spaces between them included (`> =`).
                let mut ops = String::new();
                let mut j = i;
                let mut spaced = false;
                loop {
                    match chars.get(j) {
                        Some(&ch @ ('=' | '<' | '>' | '!')) => {
                            ops.push(ch);
                            j += 1;
                        }
                        Some(' ' | '\t') => {
                            let mut k = j;
                            while matches!(chars.get(k), Some(' ' | '\t')) {
                                k += 1;
                            }
                            if matches!(chars.get(k), Some('='))
                                && (!ops.ends_with('=') || ops == "=")
                            {
                                spaced = true;
                                j = k;
                            } else {
                                break;
                            }
                        }
                        _ => break,
                    }
                }
                i = j;
                let (lt, gt, bang) = (ops.contains('<'), ops.contains('>'), ops.contains('!'));
                let eq = ops.contains('=');
                let op = match (lt, gt, bang, eq) {
                    (true, true, _, _) | (false, false, true, true) => CmpOp::Ne,
                    (true, false, false, false) => CmpOp::Lt,
                    (true, false, false, true) => CmpOp::Le,
                    (false, true, false, false) => CmpOp::Gt,
                    (false, true, false, true) => CmpOp::Ge,
                    (false, false, false, true) => CmpOp::Eq,
                    _ => {
                        diags.push(Rule::StrayCharacter, line, ops);
                        continue;
                    }
                };
                let canonical = match op {
                    CmpOp::Eq => "==",
                    CmpOp::Ne => "!=",
                    CmpOp::Lt => "<",
                    CmpOp::Le => "<=",
                    CmpOp::Gt => ">",
                    CmpOp::Ge => ">=",
                };
                if ops != canonical {
                    diags.push(Rule::NonstandardOperator, line, ops);
                } else if spaced {
                    diags.push(Rule::SpacedOperator, line, ops);
                }
                push!(Tok::Cmp(op));
            }
            '.' if !chars.get(i + 1).is_some_and(char::is_ascii_digit) => {
                push!(Tok::Dot);
                i += 1;
            }
            c if is_word_char(c) || c == '.' => {
                let start = i;
                // A number: digits with at most one '.'.
                let mut j = i;
                let mut dot = false;
                while j < chars.len() && (chars[j].is_ascii_digit() || (chars[j] == '.' && !dot)) {
                    dot |= chars[j] == '.';
                    j += 1;
                }
                let numeric = j > start && chars[start..j].iter().any(char::is_ascii_digit);
                if numeric && !chars.get(j).is_some_and(|&c| is_word_char(c)) {
                    let s: String = chars[start..j].iter().collect();
                    let tok = if dot {
                        Tok::Float(s.parse().unwrap_or(0.0))
                    } else {
                        s.parse()
                            .map_or_else(|_| Tok::Float(s.parse().unwrap_or(0.0)), Tok::Int)
                    };
                    push!(tok);
                    i = j;
                    continue;
                }
                // A word (IDs may start with digits: `1st_foo`).
                let mut j = start;
                while j < chars.len() && is_word_char(chars[j]) {
                    j += 1;
                }
                if j == start {
                    // A lone '.' followed by a digit was handled above; a lone non-word char:
                    diags.push(Rule::StrayCharacter, line, c.to_string());
                    i += 1;
                    continue;
                }
                push!(Tok::Word(chars[start..j].iter().collect()));
                i = j;
            }
            other => {
                diags.push(Rule::StrayCharacter, line, other.to_string());
                ws = true;
                i += 1;
            }
        }
    }
    push!(Tok::Newline);
    push!(Tok::Eof);
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn toks(s: &str) -> (Vec<Tok>, Diagnostics) {
        let mut d = Diagnostics::default();
        let t = lex(s, &mut d).into_iter().map(|t| t.tok).collect();
        (t, d)
    }

    #[test]
    fn basics() {
        let (t, d) = toks("if ( player->GetItemCount \"Gold_001\" >= 10.5 ) ; c\nset a.b to -1");
        assert!(d.0.is_empty(), "{d:?}");
        assert_eq!(
            t,
            vec![
                Tok::Word("if".into()),
                Tok::LParen,
                Tok::Word("player".into()),
                Tok::Arrow,
                Tok::Word("GetItemCount".into()),
                Tok::Str("Gold_001".into()),
                Tok::Cmp(CmpOp::Ge),
                Tok::Float(10.5),
                Tok::RParen,
                Tok::Newline,
                Tok::Word("set".into()),
                Tok::Word("a".into()),
                Tok::Dot,
                Tok::Word("b".into()),
                Tok::Word("to".into()),
                Tok::Minus,
                Tok::Int(1),
                Tok::Newline,
                Tok::Eof,
            ]
        );
    }

    #[test]
    fn lenient_operators_and_strings() {
        let (t, d) = toks("x > = 1 =< \"open");
        assert_eq!(t[1], Tok::Cmp(CmpOp::Ge));
        assert_eq!(t[3], Tok::Cmp(CmpOp::Le));
        assert_eq!(t[4], Tok::Str("open".into()));
        let rules: Vec<_> = d.iter().map(|d| d.rule).collect();
        assert_eq!(
            rules,
            [
                Rule::SpacedOperator,
                Rule::NonstandardOperator,
                Rule::UnterminatedString
            ]
        );
        let (t, _) = toks("1st_id .5 7");
        assert_eq!(
            t[..3],
            [Tok::Word("1st_id".into()), Tok::Float(0.5), Tok::Int(7)]
        );
    }
}
