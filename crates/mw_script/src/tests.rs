//! Parser and resolver fixtures (our own scripts, never vanilla text).

use crate::ast::{Arg, Axis, Expr, StmtKind, Target, VarRef, VarType};
use crate::lexer::CmpOp;
use crate::{Env, FunctionTable, MemberLookup, NoEnv, Rule, SourceKind, parse, parse_and_resolve};

fn rules(text: &str, kind: SourceKind) -> Vec<Rule> {
    parse(text, kind, FunctionTable::vanilla())
        .diagnostics
        .iter()
        .map(|d| d.rule)
        .collect()
}

const DOOR: &str = r#"Begin example_door
short button
short asked

if ( MenuMode == 1 )
    return
endif

if ( OnActivate == 1 )
    MessageBox "Go through?" "Yes" "No"
    set asked to 1
endif

if ( asked == 1 )
    set button to GetButtonPressed
    if ( button >= 0 )
        set asked to 0
    endif
    if ( button == 0 )
        Activate
    elseif ( button == 1 )
        "example_guard"->StartCombat player
    else
        player->GetPos z
    endif
endif
End example_door
"#;

#[test]
fn clean_script_parses_without_diagnostics() {
    let t = FunctionTable::vanilla();
    let p = parse(DOOR, SourceKind::Script, t);
    let rules: Vec<_> = p.diagnostics.iter().map(|d| d.rule).collect();
    assert_eq!(rules, [Rule::EndWithName]);
    let s = &p.script;
    assert_eq!(s.name.as_ref().unwrap().key, "example_door");
    assert_eq!(s.locals.len(), 2);
    assert_eq!(s.locals[1].ty, VarType::Short);
    assert_eq!(s.body.len(), 3);
    let StmtKind::If { arms, otherwise } = &s.body[2].kind else {
        panic!()
    };
    assert_eq!((arms.len(), otherwise.len(), s.body[2].line), (1, 0, 14));
    let StmtKind::If { arms, otherwise } = &arms[0].body[2].kind else {
        panic!()
    };
    assert_eq!((arms.len(), otherwise.len()), (2, 1));
    let StmtKind::Call(c) = &arms[1].body[0].kind else {
        panic!()
    };
    assert_eq!(
        c.target,
        Target::Explicit(crate::ast::Ident::new("example_guard"))
    );
    assert_eq!(t.function(c.function).name, "StartCombat");
    let StmtKind::Call(c) = &otherwise[0].kind else {
        panic!()
    };
    assert_eq!(c.args, [Arg::Axis(Axis::Z)]);
    let mut calls = Vec::new();
    s.walk_calls(&mut |c| calls.push(t.function(c.function).name.clone()));
    assert_eq!(
        calls,
        [
            "MenuMode",
            "OnActivate",
            "MessageBox",
            "GetButtonPressed",
            "Activate",
            "StartCombat",
            "GetPos"
        ]
    );
}

#[test]
#[allow(clippy::many_single_char_names)]
fn calls_take_arity_from_the_table() {
    let t = FunctionTable::vanilla();
    let p = parse(
        "if ( player->GetItemCount \"Gold_001\" > Random 100 )\nendif\nAIWander, 0, 0, 0, 40 30 30\n",
        SourceKind::DialogueResult,
        t,
    );
    let StmtKind::If { arms, .. } = &p.script.body[0].kind else {
        panic!()
    };
    let Expr::Cmp(CmpOp::Gt, l, r) = &arms[0].cond else {
        panic!("{:?}", arms[0].cond)
    };
    assert!(matches!(&**l, Expr::Call(c) if c.args.len() == 1));
    assert!(matches!(&**r, Expr::Call(c) if c.args == [Arg::Expr(Expr::Int(100))]));
    let StmtKind::Call(w) = &p.script.body[1].kind else {
        panic!()
    };
    assert_eq!(w.args.len(), 6);
    let commas = p
        .diagnostics
        .iter()
        .filter(|d| d.rule == Rule::CommaBetweenArgs)
        .count();
    assert_eq!(commas, 4);
}

#[test]
fn leniency_rules_fire_and_repair() {
    use Rule::*;
    let s = SourceKind::Script;
    assert_eq!(
        rules("Begin a\nif ( 1 == 1\nendif\nEnd", s),
        [UnbalancedParens]
    );
    assert_eq!(
        rules("Begin a\nif ( 1 == 1 ) )\nendif\nEnd", s),
        [UnbalancedParens]
    );
    assert_eq!(
        rules("Begin a\nif ( 1 == 1 )\nEnd a", s),
        [EndWithName, MissingEndIf]
    );
    assert_eq!(
        rules("Begin a\nshort x\nset x = 2\nset x 3\nEnd", s),
        [NonstandardOperator, SetWithoutTo, SetWithoutTo]
    );
    assert_eq!(
        rules("Begin a\nreturn junk\nendif\nEnd\nmore", s),
        [TrailingTokens, UnmatchedBlockEnd, TextAfterEnd]
    );
    assert_eq!(rules("Begin a\nAddItem \"x\"\nEnd", s), [MissingArgument]);
    assert_eq!(rules("short x\nset x to 1", s), [MissingBegin, MissingEnd]);
    let p = parse(
        "Begin a\nif ( 1 )\nelse if ( 2 )\nendif\nEnd",
        s,
        FunctionTable::vanilla(),
    );
    assert_eq!(p.diagnostics.0[0].rule, ElseIfAsTwoWords);
    let StmtKind::If { arms, .. } = &p.script.body[0].kind else {
        panic!()
    };
    assert_eq!(arms.len(), 2);
    // An arm after Else can never run: dropped.
    let p = parse(
        "Begin a\nif ( 1 )\nelse\nReturn\nelseif ( 2 )\nActivate\nendif\nEnd",
        s,
        FunctionTable::vanilla(),
    );
    let StmtKind::If { arms, otherwise } = &p.script.body[0].kind else {
        panic!()
    };
    assert_eq!((arms.len(), otherwise.len()), (1, 1));
}

#[test]
fn unknown_functions_make_the_script_inert() {
    let p = parse(
        "Begin a\nFrobnicate 3\nEnd",
        SourceKind::Script,
        FunctionTable::vanilla(),
    );
    assert!(p.is_inert());
    assert_eq!(p.diagnostics.0[0].rule, Rule::UnknownFunction);
    assert!(!parse("Begin a\nEnd", SourceKind::Script, FunctionTable::vanilla()).is_inert());
}

struct TestEnv;
impl Env for TestEnv {
    fn global(&self, key: &str) -> Option<VarType> {
        (key == "gamehour").then_some(VarType::Float)
    }
    fn object_exists(&self, key: &str) -> Option<bool> {
        Some(matches!(key, "player" | "guard"))
    }
    fn member(&self, object: &str, var: &str) -> MemberLookup {
        match (object, var) {
            ("guard", "state") => MemberLookup::Found(VarType::Short),
            ("guard", _) => MemberLookup::NoSuchVar,
            _ => MemberLookup::NoSuchObject,
        }
    }
}

#[test]
fn resolution_names_where_state_lives() {
    let t = FunctionTable::vanilla();
    let p = parse_and_resolve(
        "Begin a\nfloat t\nset t to GameHour + guard.state + guard.nope + ghost.x + typo\nnobody->Disable\nEnd",
        SourceKind::Script,
        t,
        &TestEnv,
    );
    let got: Vec<_> = p
        .diagnostics
        .iter()
        .map(|d| (d.rule, d.detail.as_str()))
        .collect();
    assert_eq!(
        got,
        [
            (Rule::UnknownMember, "guard.nope"),
            (Rule::UnknownMemberObject, "ghost"),
            (Rule::UndeclaredVariable, "typo"),
            (Rule::UnknownReference, "nobody"),
        ]
    );
    let StmtKind::Set { target, value } = &p.script.body[0].kind else {
        panic!()
    };
    assert_eq!(*target, VarRef::Local(0));
    let mut vars = Vec::new();
    collect_vars(value, &mut vars);
    assert!(matches!(vars[0], VarRef::Global(_)));
    assert!(matches!(vars[4], VarRef::Undeclared(_)));

    // Dialogue results: bare names that are no globals are the speaker's locals.
    let p = parse_and_resolve(
        "set nolore to 1\nGoodbye",
        SourceKind::DialogueResult,
        t,
        &TestEnv,
    );
    assert!(p.diagnostics.0.is_empty());
    let StmtKind::Set { target, .. } = &p.script.body[0].kind else {
        panic!()
    };
    assert!(matches!(target, VarRef::Speaker(n) if n.key == "nolore"));

    // Without content, unknown names are assumed global and nothing is reported.
    let p = parse_and_resolve("Begin a\nset x to 1\nEnd", SourceKind::Script, t, &NoEnv);
    assert!(p.diagnostics.0.is_empty());
}

fn collect_vars(e: &Expr, out: &mut Vec<VarRef>) {
    match e {
        Expr::Var(v) => out.push(v.clone()),
        Expr::Neg(x) => collect_vars(x, out),
        Expr::Arith(_, a, b) | Expr::Cmp(_, a, b) => {
            collect_vars(a, out);
            collect_vars(b, out);
        }
        _ => {}
    }
}

#[test]
fn corpus_driven_repairs() {
    use Rule::*;
    let t = FunctionTable::vanilla();
    let s = SourceKind::Script;
    // `ref->( call ) == 1`: the reference applies to the call inside.
    let p = parse(
        "Begin a\nif guard->( GetItemCount \"x\" ) == 1\nendif\nEnd",
        s,
        t,
    );
    assert_eq!(p.diagnostics.0[0].rule, ParenthesizedCall);
    let StmtKind::If { arms, .. } = &p.script.body[0].kind else {
        panic!()
    };
    let Expr::Cmp(CmpOp::Eq, call, _) = &arms[0].cond else {
        panic!("{:?}", arms[0].cond)
    };
    assert!(
        matches!(&**call, Expr::Call(c) if matches!(&c.target, Target::Explicit(r) if r.key == "guard"))
    );
    // Extra values after a call are skipped so the comparison survives.
    let p = parse(
        "Begin a\nif ( GetDisabled \"some id\" == 0 )\nendif\nEnd",
        s,
        t,
    );
    assert_eq!(
        p.diagnostics.0.iter().map(|d| d.rule).collect::<Vec<_>>(),
        [ExtraArgument]
    );
    let StmtKind::If { arms, .. } = &p.script.body[0].kind else {
        panic!()
    };
    assert!(matches!(arms[0].cond, Expr::Cmp(CmpOp::Eq, ..)));
    // Hyphenated bare IDs, `<==`, a local read through `x->`, an unknown function in an expression.
    assert_eq!(
        rules("ShowMap Some-town", SourceKind::DialogueResult),
        [HyphenatedId]
    );
    assert_eq!(
        rules("Begin a\nif ( GetDistance player <== 5 )\nendif\nEnd", s),
        [NonstandardOperator]
    );
    assert_eq!(
        rules("Begin a\nshort f\nif ( player->f == 0 )\nendif\nEnd", s),
        [ReferenceOnVariable]
    );
    let p = parse(
        "Begin a\nshort l\nset l to player->GetNothing Level\nEnd",
        s,
        t,
    );
    assert!(!p.is_inert());
    assert_eq!(
        p.diagnostics.0.iter().map(|d| d.rule).collect::<Vec<_>>(),
        [UnknownFunctionInExpression, TrailingTokens]
    );
    // An extra EndIf before ElseIf: the closed block reopens.
    let p = parse(
        "Begin a\nif ( 1 )\nActivate\nendif\nelseif ( 2 )\nDisable\nendif\nEnd",
        s,
        t,
    );
    assert_eq!(
        p.diagnostics.0.iter().map(|d| d.rule).collect::<Vec<_>>(),
        [ExtraEndIf]
    );
    let StmtKind::If { arms, .. } = &p.script.body[0].kind else {
        panic!()
    };
    assert_eq!((p.script.body.len(), arms.len()), (1, 2));
    // At a line start a function wins over a local of the same name.
    let p = parse("Begin a\nshort disable\nDisable\nEnd", s, t);
    assert!(matches!(p.script.body[0].kind, StmtKind::Call(_)));
}
