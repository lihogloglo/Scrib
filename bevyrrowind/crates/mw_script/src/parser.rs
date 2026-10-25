//! Lenient line-based recursive-descent parser.
//!
//! `MWScript` statements are one line each and calls have no parentheses, so
//! the parser reads each line on its own (recovery point = the next line) and
//! takes argument counts and kinds from the [`FunctionTable`]. Every repair
//! is a named [`Rule`]; anything that cannot be repaired is a
//! [`Severity::Error`](crate::Severity::Error) and makes the script inert.

use std::collections::HashSet;

use crate::ast::{
    Arg, ArithOp, Axis, Call, CondArm, Expr, Ident, Local, Script, SourceKind, Stmt, StmtKind,
    Target, VarRef, VarType,
};
use crate::diag::{Diagnostics, Rule};
use crate::functions::{ArgKind, Arity, FnId, FunctionTable};
use crate::lexer::{Tok, Token};

/// An open block while parsing.
enum Block {
    Root,
    If {
        line: u32,
        arms: Vec<CondArm>,
        cond: Expr,
        cond_line: u32,
        in_else: bool,
        otherwise: Vec<Stmt>,
        dead: bool,
    },
    While {
        line: u32,
        cond: Expr,
    },
}

struct Frame {
    block: Block,
    stmts: Vec<Stmt>,
}

pub(crate) struct Parser<'a> {
    toks: Vec<Token>,
    pos: usize,
    table: &'a FunctionTable,
    diags: &'a mut Diagnostics,
    /// Every local declared anywhere in the text (lowercase), so a bare word
    /// names a local even before its declaration line.
    declared: HashSet<String>,
}

fn word_key(t: &Tok) -> Option<String> {
    match t {
        Tok::Word(w) => Some(w.to_lowercase()),
        _ => None,
    }
}

fn var_type(key: &str) -> Option<VarType> {
    match key {
        "short" => Some(VarType::Short),
        "long" => Some(VarType::Long),
        "float" => Some(VarType::Float),
        _ => None,
    }
}

impl<'a> Parser<'a> {
    pub(crate) fn new(
        toks: Vec<Token>,
        table: &'a FunctionTable,
        diags: &'a mut Diagnostics,
    ) -> Self {
        let mut declared = HashSet::new();
        let mut line_start = true;
        for w in toks.windows(2) {
            if line_start
                && let (Some(k), Tok::Word(name)) = (word_key(&w[0].tok), &w[1].tok)
                && var_type(&k).is_some()
            {
                declared.insert(name.to_lowercase());
            }
            line_start = w[0].tok == Tok::Newline;
        }
        Self {
            toks,
            pos: 0,
            table,
            diags,
            declared,
        }
    }

    fn peek(&self) -> &Tok {
        &self.toks[self.pos].tok
    }

    fn peek_at(&self, n: usize) -> &Tok {
        &self.toks[(self.pos + n).min(self.toks.len() - 1)].tok
    }

    fn line(&self) -> u32 {
        self.toks[self.pos].line
    }

    fn bump(&mut self) -> Tok {
        let t = self.toks[self.pos].tok.clone();
        if self.pos + 1 < self.toks.len() {
            self.pos += 1;
        }
        t
    }

    fn at_line_end(&self) -> bool {
        matches!(self.peek(), Tok::Newline | Tok::Eof)
    }

    /// Skips the rest of the line, reporting leftovers; then eats the newline.
    fn finish_line(&mut self) {
        if !self.at_line_end() {
            let line = self.line();
            let only_parens = (self.pos..self.toks.len())
                .map(|i| &self.toks[i].tok)
                .take_while(|t| !matches!(t, Tok::Newline | Tok::Eof))
                .all(|t| *t == Tok::RParen);
            let first = format!("{:?}", self.peek());
            while !self.at_line_end() {
                self.bump();
            }
            if only_parens {
                self.diags.push(Rule::UnbalancedParens, line, "extra ')'");
            } else {
                self.diags.push(Rule::TrailingTokens, line, first);
            }
        }
        if *self.peek() == Tok::Newline {
            self.bump();
        }
    }

    fn skip_line(&mut self) {
        while !self.at_line_end() {
            self.bump();
        }
        if *self.peek() == Tok::Newline {
            self.bump();
        }
    }

    pub(crate) fn parse(mut self, kind: SourceKind) -> Script {
        let mut script = Script {
            kind,
            name: None,
            locals: Vec::new(),
            body: Vec::new(),
        };
        let mut stack = vec![Frame {
            block: Block::Root,
            stmts: Vec::new(),
        }];
        let mut began = false;
        let mut ended = false;
        let mut statements_seen = false;
        loop {
            while *self.peek() == Tok::Newline {
                self.bump();
            }
            if *self.peek() == Tok::Eof {
                break;
            }
            let line = self.line();
            if ended {
                self.diags.push(Rule::TextAfterEnd, line, "");
                break;
            }
            let head = word_key(self.peek());
            match head.as_deref() {
                Some("begin") => {
                    self.bump();
                    if began {
                        self.diags.push(Rule::DuplicateBegin, line, "");
                    } else if let Some(name) = self.take_id(line) {
                        script.name = Some(Ident::new(&name));
                    }
                    if kind == SourceKind::Script && statements_seen && !began {
                        self.diags.push(Rule::StatementsBeforeBegin, line, "");
                    }
                    began = true;
                    self.finish_line();
                }
                Some("end") if kind == SourceKind::Script || began => {
                    self.bump();
                    if !self.at_line_end() {
                        self.diags.push(Rule::EndWithName, line, "");
                        self.skip_line();
                    }
                    ended = true;
                }
                Some(k)
                    if var_type(k).is_some()
                        && matches!(self.peek_at(1), Tok::Word(_) | Tok::Newline | Tok::Eof) =>
                {
                    let ty = var_type(k).unwrap_or(VarType::Short);
                    self.bump();
                    match self.peek().clone() {
                        Tok::Word(name) => {
                            self.bump();
                            let name = Ident::new(&name);
                            if script.local(&name.key).is_some() {
                                self.diags
                                    .push(Rule::DuplicateLocal, line, name.text.clone());
                            } else {
                                if statements_seen {
                                    self.diags
                                        .push(Rule::LateDeclaration, line, name.text.clone());
                                }
                                script.locals.push(Local { name, ty, line });
                            }
                        }
                        _ => self.diags.push(Rule::EmptyDeclaration, line, ""),
                    }
                    self.finish_line();
                }
                _ => {
                    statements_seen = true;
                    self.statement_line(line, &mut stack);
                }
            }
        }
        if kind == SourceKind::Script {
            if !began {
                self.diags.push(Rule::MissingBegin, 1, "");
            }
            if !ended {
                let line = self.line();
                self.diags.push(Rule::MissingEnd, line, "");
            }
        }
        while stack.len() > 1 {
            let line = self.line();
            self.diags.push(Rule::MissingEndIf, line, "");
            close_block(&mut stack, line);
        }
        script.body = stack.pop().map(|f| f.stmts).unwrap_or_default();
        script
    }

    /// One statement line, including block structure.
    #[allow(clippy::too_many_lines)]
    fn statement_line(&mut self, line: u32, stack: &mut Vec<Frame>) {
        // `x->Set ...` / `x->If`: a reference on a keyword is meaningless.
        if matches!(self.peek(), Tok::Word(_) | Tok::Str(_))
            && *self.peek_at(1) == Tok::Arrow
            && word_key(self.peek_at(2)).is_some_and(|k| is_keyword(&k))
        {
            self.diags.push(Rule::ReferenceOnKeyword, line, "");
            self.bump();
            self.bump();
        }
        let head = word_key(self.peek());
        match head.as_deref() {
            Some("set") => {
                self.bump();
                if let Some(stmt) = self.parse_set(line) {
                    push_stmt(stack, stmt);
                }
                self.finish_line();
            }
            Some("if" | "while") => {
                let is_if = head.as_deref() == Some("if");
                self.bump();
                let cond = self.condition(line);
                self.finish_line();
                let block = if is_if {
                    Block::If {
                        line,
                        arms: Vec::new(),
                        cond,
                        cond_line: line,
                        in_else: false,
                        otherwise: Vec::new(),
                        dead: false,
                    }
                } else {
                    Block::While { line, cond }
                };
                stack.push(Frame {
                    block,
                    stmts: Vec::new(),
                });
            }
            Some("elseif") => {
                self.bump();
                let cond = self.condition(line);
                self.finish_line();
                self.else_branch(stack, line, Some(cond));
            }
            Some("else") => {
                self.bump();
                // `Else If (...)` written as two words: an ElseIf.
                if word_key(self.peek()).as_deref() == Some("if") {
                    self.diags.push(Rule::ElseIfAsTwoWords, line, "");
                    self.bump();
                    let cond = self.condition(line);
                    self.finish_line();
                    self.else_branch(stack, line, Some(cond));
                } else {
                    self.finish_line();
                    self.else_branch(stack, line, None);
                }
            }
            Some(k @ ("endif" | "endwhile")) => {
                let want_if = k == "endif";
                self.bump();
                self.finish_line();
                match stack.last().map(|f| &f.block) {
                    Some(Block::Root) | None => {
                        self.diags.push(Rule::UnmatchedBlockEnd, line, k.to_owned());
                    }
                    Some(Block::If { .. }) if !want_if => {
                        self.diags
                            .push(Rule::MismatchedBlockEnd, line, k.to_owned());
                        close_block(stack, line);
                    }
                    Some(Block::While { .. }) if want_if => {
                        self.diags
                            .push(Rule::MismatchedBlockEnd, line, k.to_owned());
                        close_block(stack, line);
                    }
                    Some(_) => close_block(stack, line),
                }
            }
            Some("return") => {
                self.bump();
                push_stmt(
                    stack,
                    Stmt {
                        kind: StmtKind::Return,
                        line,
                    },
                );
                self.finish_line();
            }
            Some("begin" | "end" | "short" | "long" | "float" | "to") => {
                self.diags
                    .push(Rule::UnknownStatement, line, head.unwrap_or_default());
                self.skip_line();
            }
            _ => {
                match self.peek().clone() {
                    Tok::Word(_) | Tok::Str(_) if *self.peek_at(1) == Tok::Arrow => {
                        let (Tok::Word(r) | Tok::Str(r)) = self.bump() else {
                            unreachable!()
                        };
                        self.bump();
                        let target = Target::Explicit(Ident::new(&r));
                        match self.peek().clone() {
                            Tok::Word(f) => {
                                let Some(id) = self.table.get(&f) else {
                                    self.diags.push(Rule::UnknownFunction, line, f);
                                    self.skip_line();
                                    return;
                                };
                                self.bump();
                                let call = self.parse_call(target, id, line);
                                push_stmt(
                                    stack,
                                    Stmt {
                                        kind: StmtKind::Call(call),
                                        line,
                                    },
                                );
                            }
                            Tok::LParen => {
                                let Target::Explicit(r) = target else {
                                    unreachable!()
                                };
                                if let Expr::Call(call) = self.arrow_target(r, line) {
                                    push_stmt(
                                        stack,
                                        Stmt {
                                            kind: StmtKind::Call(*call),
                                            line,
                                        },
                                    );
                                } else {
                                    self.diags.push(
                                        Rule::UnknownStatement,
                                        line,
                                        "'->' without a call",
                                    );
                                }
                            }
                            _ => {
                                self.diags.push(
                                    Rule::UnknownStatement,
                                    line,
                                    "'->' without a function",
                                );
                                self.skip_line();
                                return;
                            }
                        }
                    }
                    Tok::Word(w) => {
                        let key = w.to_lowercase();
                        // At a line start a function wins over a local of the same name (ShadowedName).
                        if let Some(id) = self.table.get_lower(&key) {
                            self.bump();
                            let call = self.parse_call(Target::Implicit, id, line);
                            push_stmt(
                                stack,
                                Stmt {
                                    kind: StmtKind::Call(call),
                                    line,
                                },
                            );
                        } else if self.declared.contains(&key) {
                            self.diags.push(Rule::UnknownStatement, line, w);
                            self.skip_line();
                            return;
                        } else {
                            self.diags.push(Rule::UnknownFunction, line, w);
                            self.skip_line();
                            return;
                        }
                    }
                    other => {
                        self.diags
                            .push(Rule::UnknownStatement, line, format!("{other:?}"));
                        self.skip_line();
                        return;
                    }
                }
                self.finish_line();
            }
        }
    }

    fn else_branch(&mut self, stack: &mut Vec<Frame>, line: u32, cond: Option<Expr>) {
        self.reopen_if(stack, line);
        let Some(frame) = stack.last_mut() else {
            return;
        };
        let Block::If {
            arms,
            cond: cur,
            cond_line,
            in_else,
            otherwise,
            dead,
            ..
        } = &mut frame.block
        else {
            self.diags.push(
                Rule::UnmatchedBlockEnd,
                line,
                if cond.is_some() { "elseif" } else { "else" },
            );
            return;
        };
        let body = std::mem::take(&mut frame.stmts);
        if *in_else {
            // Statements after `Else` belong to it; a later arm can never run.
            if !*dead {
                otherwise.extend(body);
            }
            self.diags.push(Rule::ElseIfAfterElse, line, "");
            *dead = true;
            return;
        }
        arms.push(CondArm {
            cond: std::mem::replace(cur, Expr::Int(0)),
            line: *cond_line,
            body,
        });
        match cond {
            Some(c) => {
                *cur = c;
                *cond_line = line;
            }
            None => *in_else = true,
        }
    }

    /// `ElseIf`/`Else` with no open `If`, right after an `If` block closed: that
    /// block's `EndIf` was extra (`ExtraEndIf`); reopen the block.
    fn reopen_if(&mut self, stack: &mut Vec<Frame>, line: u32) {
        let Some(frame) = stack.last_mut() else {
            return;
        };
        if matches!(frame.block, Block::If { .. }) {
            return;
        }
        let Some(Stmt {
            kind: StmtKind::If { arms, otherwise },
            line: if_line,
        }) = frame.stmts.last()
        else {
            return;
        };
        if !otherwise.is_empty() || arms.is_empty() {
            return;
        }
        let if_line = *if_line;
        let Some(Stmt {
            kind: StmtKind::If { mut arms, .. },
            ..
        }) = frame.stmts.pop()
        else {
            return;
        };
        let last = arms.pop().expect("checked non-empty");
        self.diags.push(Rule::ExtraEndIf, line, "");
        stack.push(Frame {
            block: Block::If {
                line: if_line,
                arms,
                cond: last.cond,
                cond_line: last.line,
                in_else: false,
                otherwise: Vec::new(),
                dead: false,
            },
            stmts: last.body,
        });
    }

    /// An ID: bare word, string or number; joins `a-b` written without spaces (`HyphenatedId`).
    fn take_id(&mut self, line: u32) -> Option<String> {
        let (mut s, bare) = match self.peek().clone() {
            Tok::Word(s) => (s, true),
            Tok::Int(i) => (i.to_string(), true),
            Tok::Str(s) => (s, false),
            _ => return None,
        };
        self.bump();
        while bare && self.toks[self.pos].tok == Tok::Minus && self.toks[self.pos].glued {
            let next = &self.toks[(self.pos + 1).min(self.toks.len() - 1)];
            let part = match &next.tok {
                Tok::Word(w) if next.glued => w.clone(),
                Tok::Int(i) if next.glued => i.to_string(),
                _ => break,
            };
            self.diags.push(Rule::HyphenatedId, line, "");
            s.push('-');
            s.push_str(&part);
            self.bump();
            self.bump();
        }
        Some(s)
    }

    fn condition(&mut self, line: u32) -> Expr {
        if self.at_line_end() {
            self.diags.push(Rule::MissingCondition, line, "");
            return Expr::Int(0);
        }
        self.expr(line)
    }

    fn parse_set(&mut self, line: u32) -> Option<Stmt> {
        let target = match (
            self.peek().clone(),
            self.peek_at(1).clone(),
            self.peek_at(2).clone(),
        ) {
            (Tok::Word(o) | Tok::Str(o), Tok::Dot, Tok::Word(v)) => {
                self.bump();
                self.bump();
                self.bump();
                VarRef::Member {
                    object: Ident::new(&o),
                    var: Ident::new(&v),
                }
            }
            (Tok::Word(name) | Tok::Str(name), ..) => {
                self.bump();
                VarRef::Name(Ident::new(&name))
            }
            _ => {
                self.diags.push(Rule::BadSetTarget, line, "");
                return None;
            }
        };
        match self.peek() {
            Tok::Word(w) if w.eq_ignore_ascii_case("to") => {
                self.bump();
                if matches!(self.peek(), Tok::Cmp(crate::lexer::CmpOp::Eq)) {
                    self.diags.push(Rule::SetWithoutTo, line, "to =");
                    self.bump();
                }
            }
            Tok::Cmp(crate::lexer::CmpOp::Eq) => {
                self.diags.push(Rule::SetWithoutTo, line, "=");
                self.bump();
            }
            _ => self.diags.push(Rule::SetWithoutTo, line, ""),
        }
        if self.at_line_end() {
            self.diags
                .push(Rule::BadExpression, line, "Set without a value");
            return None;
        }
        let value = self.expr(line);
        Some(Stmt {
            kind: StmtKind::Set { target, value },
            line,
        })
    }

    /// `sum [cmp sum]`.
    fn expr(&mut self, line: u32) -> Expr {
        let lhs = self.sum(line);
        if let Tok::Cmp(op) = *self.peek() {
            self.bump();
            let rhs = self.sum(line);
            return Expr::Cmp(op, Box::new(lhs), Box::new(rhs));
        }
        lhs
    }

    fn sum(&mut self, line: u32) -> Expr {
        let mut lhs = self.term(line);
        loop {
            let op = match self.peek() {
                Tok::Plus => ArithOp::Add,
                Tok::Minus => ArithOp::Sub,
                _ => return lhs,
            };
            self.bump();
            let rhs = self.term(line);
            lhs = Expr::Arith(op, Box::new(lhs), Box::new(rhs));
        }
    }

    fn term(&mut self, line: u32) -> Expr {
        let mut lhs = self.unary(line);
        loop {
            let op = match self.peek() {
                Tok::Star => ArithOp::Mul,
                Tok::Slash => ArithOp::Div,
                _ => return lhs,
            };
            self.bump();
            let rhs = self.unary(line);
            lhs = Expr::Arith(op, Box::new(lhs), Box::new(rhs));
        }
    }

    fn unary(&mut self, line: u32) -> Expr {
        match self.peek() {
            Tok::Minus => {
                self.bump();
                match self.unary(line) {
                    Expr::Int(i) => Expr::Int(-i),
                    Expr::Float(f) => Expr::Float(-f),
                    e => Expr::Neg(Box::new(e)),
                }
            }
            Tok::Plus => {
                self.bump();
                self.unary(line)
            }
            _ => self.primary(line),
        }
    }

    fn primary(&mut self, line: u32) -> Expr {
        match self.peek().clone() {
            Tok::Int(i) => {
                self.bump();
                Expr::Int(i)
            }
            Tok::Float(f) => {
                self.bump();
                Expr::Float(f)
            }
            Tok::LParen => {
                self.bump();
                let e = self.expr(line);
                if *self.peek() == Tok::RParen {
                    self.bump();
                } else {
                    self.diags.push(Rule::UnbalancedParens, line, "missing ')'");
                }
                e
            }
            Tok::Word(_) | Tok::Str(_) if *self.peek_at(1) == Tok::Arrow => {
                let (Tok::Word(r) | Tok::Str(r)) = self.bump() else {
                    unreachable!()
                };
                self.bump();
                self.arrow_target(Ident::new(&r), line)
            }
            Tok::Word(o) | Tok::Str(o)
                if *self.peek_at(1) == Tok::Dot && matches!(self.peek_at(2), Tok::Word(_)) =>
            {
                self.bump();
                self.bump();
                let Tok::Word(v) = self.bump() else {
                    unreachable!()
                };
                Expr::Var(VarRef::Member {
                    object: Ident::new(&o),
                    var: Ident::new(&v),
                })
            }
            Tok::Word(w) => {
                self.bump();
                let key = w.to_lowercase();
                if !self.declared.contains(&key)
                    && let Some(id) = self.table.get_lower(&key)
                {
                    return Expr::Call(Box::new(self.parse_call(Target::Implicit, id, line)));
                }
                Expr::Var(VarRef::Name(Ident::new(&w)))
            }
            Tok::Str(s) => {
                self.bump();
                Expr::Var(VarRef::Name(Ident::new(&s)))
            }
            other => {
                self.diags
                    .push(Rule::BadExpression, line, format!("{other:?}"));
                Expr::Int(0)
            }
        }
    }

    /// After `ref->` in an expression: a call, `( calls )`, or a variable.
    fn arrow_target(&mut self, r: Ident, line: u32) -> Expr {
        match self.peek().clone() {
            Tok::LParen => {
                self.diags.push(Rule::ParenthesizedCall, line, "");
                self.bump();
                let mut e = self.expr(line);
                if *self.peek() == Tok::RParen {
                    self.bump();
                } else {
                    self.diags.push(Rule::UnbalancedParens, line, "missing ')'");
                }
                retarget(&mut e, &r);
                e
            }
            Tok::Word(f) => {
                let key = f.to_lowercase();
                if !self.declared.contains(&key)
                    && let Some(id) = self.table.get_lower(&key)
                {
                    self.bump();
                    return Expr::Call(Box::new(self.parse_call(Target::Explicit(r), id, line)));
                }
                if self.declared.contains(&key) {
                    self.bump();
                    self.diags.push(Rule::ReferenceOnVariable, line, f.clone());
                    return Expr::Var(VarRef::Name(Ident::new(&f)));
                }
                self.bump();
                self.diags.push(Rule::UnknownFunctionInExpression, line, f);
                Expr::Int(0)
            }
            other => {
                self.diags
                    .push(Rule::BadExpression, line, format!("'->' then {other:?}"));
                Expr::Int(0)
            }
        }
    }

    /// Can the next token start an argument?
    fn at_arg_end(&self) -> bool {
        matches!(
            self.peek(),
            Tok::Newline
                | Tok::Eof
                | Tok::RParen
                | Tok::Cmp(_)
                | Tok::Plus
                | Tok::Star
                | Tok::Slash
        ) || word_key(self.peek()).is_some_and(|k| is_keyword(&k))
    }

    fn parse_call(&mut self, target: Target, function: FnId, line: u32) -> Call {
        let specs = self.table.function(function).args.clone();
        let mut args = Vec::with_capacity(specs.len());
        for spec in specs {
            loop {
                if *self.peek() == Tok::Comma {
                    self.diags.push(Rule::CommaBetweenArgs, line, "");
                    self.bump();
                }
                if self.at_arg_end() {
                    if spec.arity == Arity::Required {
                        let name = self.table.function(function).name.clone();
                        self.diags.push(Rule::MissingArgument, line, name);
                        args.push(Arg::Missing);
                    }
                    break;
                }
                let arg = match spec.kind {
                    ArgKind::Id => self
                        .take_id(line)
                        .map_or(Arg::Missing, |s| Arg::Id(Ident::new(&s))),
                    ArgKind::Text => match self.peek().clone() {
                        Tok::Word(s) | Tok::Str(s) => {
                            self.bump();
                            Arg::Str(s)
                        }
                        _ => Arg::Missing,
                    },
                    ArgKind::Axis => match self.peek().clone() {
                        Tok::Word(s) | Tok::Str(s) => match s.to_ascii_lowercase().as_str() {
                            "x" => Some(Axis::X),
                            "y" => Some(Axis::Y),
                            "z" => Some(Axis::Z),
                            _ => None,
                        }
                        .map_or(Arg::Missing, |a| {
                            self.bump();
                            Arg::Axis(a)
                        }),
                        _ => Arg::Missing,
                    },
                    ArgKind::Num => Arg::Expr(self.unary(line)),
                    ArgKind::Any => match self.peek().clone() {
                        Tok::Str(s) => {
                            self.bump();
                            Arg::Str(s)
                        }
                        _ => Arg::Expr(self.unary(line)),
                    },
                };
                if arg == Arg::Missing {
                    if spec.arity == Arity::Required {
                        let name = self.table.function(function).name.clone();
                        self.diags.push(Rule::MissingArgument, line, name);
                        args.push(Arg::Missing);
                    }
                    break;
                }
                args.push(arg);
                if spec.arity != Arity::Rest {
                    break;
                }
            }
        }
        // Values after the last argument (`GetDisabled "id" == 0`, `Disable "id"`): skipped, so
        // an expression keeps its operator (`ExtraArgument`).
        let mut extra = false;
        while matches!(self.peek(), Tok::Str(_) | Tok::Int(_) | Tok::Float(_))
            || word_key(self.peek()).is_some_and(|k| !is_keyword(&k))
        {
            if !extra {
                let detail = self.table.function(function).name.clone();
                self.diags.push(Rule::ExtraArgument, line, detail);
                extra = true;
            }
            self.bump();
        }
        Call {
            target,
            function,
            args,
            line,
        }
    }
}

fn is_keyword(k: &str) -> bool {
    matches!(
        k,
        "begin"
            | "end"
            | "if"
            | "elseif"
            | "else"
            | "endif"
            | "while"
            | "endwhile"
            | "set"
            | "to"
            | "return"
    )
}

/// Gives the implicit calls in `e` the explicit reference `r` (`ParenthesizedCall`).
fn retarget(e: &mut Expr, r: &Ident) {
    match e {
        Expr::Call(c) if c.target == Target::Implicit => c.target = Target::Explicit(r.clone()),
        Expr::Neg(x) => retarget(x, r),
        Expr::Arith(_, a, b) | Expr::Cmp(_, a, b) => {
            retarget(a, r);
            retarget(b, r);
        }
        _ => {}
    }
}

fn push_stmt(stack: &mut [Frame], stmt: Stmt) {
    if let Some(f) = stack.last_mut() {
        if let Block::If { dead: true, .. } = f.block {
            return;
        }
        f.stmts.push(stmt);
    }
}

/// Closes the innermost block into its parent.
fn close_block(stack: &mut Vec<Frame>, _line: u32) {
    let Some(frame) = stack.pop() else { return };
    let stmt = match frame.block {
        Block::Root => {
            stack.push(frame);
            return;
        }
        Block::If {
            line,
            mut arms,
            cond,
            cond_line,
            in_else,
            mut otherwise,
            dead,
        } => {
            if in_else {
                if !dead {
                    otherwise.extend(frame.stmts);
                }
            } else {
                arms.push(CondArm {
                    cond,
                    line: cond_line,
                    body: frame.stmts,
                });
            }
            Stmt {
                kind: StmtKind::If { arms, otherwise },
                line,
            }
        }
        Block::While { line, cond } => Stmt {
            kind: StmtKind::While {
                cond,
                body: frame.stmts,
            },
            line,
        },
    };
    push_stmt(stack, stmt);
}
