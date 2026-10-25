use super::*;

/// A test context: what the commands did.
#[derive(Default)]
struct Ctx {
    spawned: Vec<(String, i64)>,
}

fn console() -> (Console<Ctx>, VarId, VarId, VarId) {
    let mut c = Console::<Ctx>::new();
    let shadows = c
        .add_var(VarSpec::bool("render.shadows", true).help("Shadow maps"))
        .unwrap();
    let scale = c
        .add_var(VarSpec::float("time.scale", 1.0).float_range(0.0, 10.0))
        .unwrap();
    let mode = c
        .add_var(VarSpec::choice(
            "render.mode",
            &["Raster", "Solari"],
            "raster",
        ))
        .unwrap();
    c.add_var(VarSpec::bool("render.water", true)).unwrap();
    c.add_var(VarSpec::int("lod.bias", 0).int_range(-4, 4))
        .unwrap();
    c.add_command(
        CommandSpec::new("spawn")
            .alias("sp")
            .help("Spawn things")
            .param(Param::new("what", ArgKind::Str))
            .param(Param::new("count", ArgKind::Int { min: 1, max: 100 }).optional()),
        |inv| {
            let what = inv.args.str("what").unwrap().to_owned();
            if what == "nothing" {
                return Err("can't spawn nothing".into());
            }
            inv.ctx
                .spawned
                .push((what, inv.args.int("count").unwrap_or(1)));
            Ok(())
        },
    )
    .unwrap();
    (c, shadows, scale, mode)
}

fn last_line<C>(c: &Console<C>) -> &str {
    &c.log().lines().last().unwrap().text
}

#[test]
fn tokenizer_quotes_escapes_and_chains() {
    let segs = split(r#"echo "a b" c\d "q\"x\\";  set x 1"#);
    assert_eq!(segs.len(), 2);
    let words: Vec<&str> = segs[0].tokens.iter().map(|t| t.text.as_str()).collect();
    assert_eq!(words, ["echo", "a b", r"c\d", r#"q"x\"#]);
    assert_eq!(segs[1].tokens.len(), 3);
    assert!(split(r#"say "open"#)[0].tokens[1].unterminated);
    // Semicolons inside quotes don't split.
    assert_eq!(split(r#"echo "a;b""#).len(), 1);
    assert_eq!(split("").len(), 1);
}

#[test]
fn values_parse_by_kind() {
    for on in ["on", "TRUE", "yes", "1"] {
        assert_eq!(ArgKind::Bool.parse(on), Ok(Value::Bool(true)));
    }
    assert!(ArgKind::Bool.parse("maybe").is_err());
    assert_eq!(
        ArgKind::Int { min: 0, max: 3 }.parse("3"),
        Ok(Value::Int(3))
    );
    assert!(ArgKind::Int { min: 0, max: 3 }.parse("4").is_err());
    assert!(ArgKind::int().parse("1.5").is_err());
    assert!(ArgKind::float().parse("nan").is_err());
    assert!(ArgKind::float().parse("inf").is_err());
    assert_eq!(
        ArgKind::choice(&["A", "b"]).parse("a"),
        Ok(Value::Str("a".into()))
    );
    assert!(ArgKind::choice(&["a"]).parse("c").is_err());
}

#[test]
fn bare_variable_prints_and_sets() {
    let (mut c, shadows, scale, mode) = console();
    let ctx = &mut Ctx::default();
    c.exec(ctx, "render.shadows").unwrap();
    assert_eq!(last_line(&c), "render.shadows = on [on|off]: Shadow maps");
    c.exec(ctx, "Render.Shadows off").unwrap();
    assert!(!c.vars().flag(shadows));
    assert_eq!(
        last_line(&c),
        "render.shadows = off (default on) [on|off]: Shadow maps"
    );
    c.exec(ctx, "set time.scale 2.5; set render.mode SOLARI")
        .unwrap();
    assert_eq!(c.vars().get(scale), &Value::Float(2.5));
    assert_eq!(c.vars().get(mode), &Value::Str("solari".into()));
}

#[test]
fn invalid_values_are_refused_and_logged() {
    let (mut c, _, scale, _) = console();
    let ctx = &mut Ctx::default();
    let e = c.exec(ctx, "time.scale 11").unwrap_err();
    assert_eq!(
        e.to_string(),
        r#"time.scale: <value> expected float 0..=10, got "11""#
    );
    assert_eq!(c.log().lines().last().unwrap().level, Level::Error);
    assert_eq!(c.vars().get(scale), &Value::Float(1.0));
    assert!(matches!(
        c.exec(ctx, "time.scale 1 2"),
        Err(ExecError::TooManyArgs { .. })
    ));
    assert!(matches!(
        c.exec(ctx, "set render.mode vulkan"),
        Err(ExecError::BadArg { .. })
    ));
    // From code: kind checked, ints widen to float.
    assert!(c.vars_mut().set(scale, Value::Bool(true)).is_err());
    assert_eq!(c.vars_mut().set(scale, Value::Int(3)), Ok(true));
    assert_eq!(c.vars().get(scale), &Value::Float(3.0));
}

#[test]
fn toggle_and_reset() {
    let (mut c, shadows, scale, _) = console();
    let ctx = &mut Ctx::default();
    c.exec(ctx, "toggle render.shadows").unwrap();
    assert!(!c.vars().flag(shadows));
    c.exec(ctx, "toggle render.shadows").unwrap();
    assert!(c.vars().flag(shadows));
    let e = c.exec(ctx, "toggle time.scale").unwrap_err();
    assert!(matches!(e, ExecError::Unknown { ref what, .. } if what == "on/off variable"));
    c.exec(ctx, "render.shadows off; time.scale 5").unwrap();
    c.exec(ctx, "reset time.scale").unwrap();
    assert_eq!(c.vars().get(scale), &Value::Float(1.0));
    assert!(!c.vars().flag(shadows));
    c.exec(ctx, "reset").unwrap();
    assert_eq!(last_line(&c), "reset 1 variable(s)");
    assert!(c.vars().flag(shadows));
}

#[test]
fn commands_bind_typed_args() {
    let (mut c, ..) = console();
    let ctx = &mut Ctx::default();
    c.exec(ctx, "spawn crab 3; SP \"mud crab\"").unwrap();
    assert_eq!(
        ctx.spawned,
        [("crab".to_owned(), 3), ("mud crab".to_owned(), 1)]
    );
    let e = c.exec(ctx, "spawn").unwrap_err();
    assert_eq!(
        e.to_string(),
        "spawn: missing <what>; usage: spawn <what:text> [count:int 1..=100]"
    );
    assert!(matches!(
        c.exec(ctx, "spawn crab 0"),
        Err(ExecError::BadArg { .. })
    ));
    assert!(matches!(
        c.exec(ctx, "spawn crab 1 2"),
        Err(ExecError::TooManyArgs { .. })
    ));
    assert!(matches!(
        c.exec(ctx, "spawn \"crab"),
        Err(ExecError::UnterminatedQuote)
    ));
    // A failing handler stops the chain.
    let e = c.exec(ctx, "spawn nothing; spawn rat").unwrap_err();
    assert_eq!(
        e,
        ExecError::Failed {
            command: "spawn".into(),
            message: "can't spawn nothing".into()
        }
    );
    assert_eq!(ctx.spawned.len(), 2);
}

#[test]
fn rest_parameter_takes_raw_text() {
    let mut c: Console = Console::new();
    let got = std::sync::Arc::new(std::sync::Mutex::new(String::new()));
    let sink = got.clone();
    c.add_command(
        CommandSpec::new("lua").param(Param::rest("code")),
        move |inv| {
            inv.args
                .str("code")
                .unwrap()
                .clone_into(&mut sink.lock().unwrap());
            Ok(())
        },
    )
    .unwrap();
    c.exec(&mut (), r#"lua  print("a  b", 'c')   ; echo done"#)
        .unwrap();
    assert_eq!(*got.lock().unwrap(), r#"print("a  b", 'c')"#);
    assert_eq!(last_line(&c), "done");
}

#[test]
fn unknown_names_suggest() {
    let (mut c, ..) = console();
    let ctx = &mut Ctx::default();
    let e = c.exec(ctx, "render.shadow off").unwrap_err();
    assert_eq!(
        e.to_string(),
        r#"unknown command or variable "render.shadow"; did you mean render.shadows?"#
    );
    // The last dotted part, and substrings, are matched too.
    assert_eq!(c.suggestions("shadows"), ["render.shadows"]);
    assert_eq!(
        c.suggestions("render"),
        ["render.mode", "render.shadows", "render.water"]
    );
    assert_eq!(c.suggestions("tgogle"), ["toggle"]);
    assert!(c.suggestions("zzzzzz").is_empty());
}

#[test]
fn registration_is_checked() {
    let (mut c, ..) = console();
    assert_eq!(
        c.add_var(VarSpec::bool("Render.Shadows", false)),
        Err(RegisterError::Duplicate("render.shadows".into()))
    );
    assert_eq!(
        c.add_var(VarSpec::bool("help", false)),
        Err(RegisterError::Duplicate("help".into()))
    );
    assert!(matches!(
        c.add_var(VarSpec::bool("1st", false)),
        Err(RegisterError::InvalidName(_))
    ));
    assert!(matches!(
        c.add_var(VarSpec::bool("a b", false)),
        Err(RegisterError::InvalidName(_))
    ));
    assert!(matches!(
        c.add_var(VarSpec::int("x", 9).int_range(0, 5)),
        Err(RegisterError::BadDefault { .. })
    ));
    assert!(matches!(
        c.add_var(VarSpec::choice("y", &["a"], "b")),
        Err(RegisterError::BadDefault { .. })
    ));
    let ok = |_: Invocation<'_, Ctx>| Ok(());
    let bad = |spec| matches!(spec, Err(RegisterError::BadParams { .. }));
    assert!(bad(c.add_command(
        CommandSpec::new("a")
            .param(Param::new("x", ArgKind::Str).optional())
            .param(Param::new("y", ArgKind::Str)),
        ok
    )));
    assert!(bad(c.add_command(
        CommandSpec::new("b")
            .param(Param::rest("r"))
            .param(Param::new("y", ArgKind::Str)),
        ok
    )));
    assert!(bad(c.add_command(
        CommandSpec::new("c")
            .param(Param::new("x", ArgKind::Str))
            .param(Param::new("x", ArgKind::Str)),
        ok
    )));
    assert_eq!(
        c.add_command(CommandSpec::new("d").alias("sp"), ok),
        Err(RegisterError::Duplicate("sp".into()))
    );
    assert_eq!(
        c.add_command(CommandSpec::new("e").alias("e"), ok),
        Err(RegisterError::Duplicate("e".into()))
    );
    // Nothing half-registered.
    assert_eq!(c.add_command(CommandSpec::new("a"), ok), Ok(()));
    assert_eq!(c.add_command(CommandSpec::new("d"), ok), Ok(()));
}

#[test]
fn change_tracking() {
    let (mut c, shadows, scale, _) = console();
    let ctx = &mut Ctx::default();
    assert!(c.vars_mut().take_changes().is_empty());
    c.exec(
        ctx,
        "time.scale 2; render.shadows off; time.scale 3; render.shadows off",
    )
    .unwrap();
    assert_eq!(c.vars_mut().take_changes(), [scale, shadows]);
    assert!(c.vars_mut().take_changes().is_empty());
    assert_eq!(c.vars().var(scale).generation(), 2);
    assert_eq!(c.vars().var(shadows).generation(), 1);
    assert_eq!(c.vars().revision(), 3);
    // Setting the same value is not a change.
    assert_eq!(c.vars_mut().set(scale, Value::Float(3.0)), Ok(false));
    assert!(c.vars_mut().take_changes().is_empty());
}

#[test]
fn complete_names_then_args() {
    let (c, ..) = console();
    let names = |line| c.complete(line).candidates;
    assert_eq!(
        names("ren"),
        ["render.mode", "render.shadows", "render.water"]
    );
    assert_eq!(names("TOG"), ["toggle"]);
    // Aliases run but aren't offered.
    assert!(!names("s").contains(&"sp".to_owned()));
    assert_eq!(names("toggle "), ["render.shadows", "render.water"]);
    assert_eq!(names("set render.m"), ["render.mode"]);
    assert_eq!(names("set render.mode "), ["raster", "solari"]);
    assert_eq!(names("render.mode s"), ["solari"]);
    assert_eq!(names("render.shadows "), ["off", "on"]);
    assert_eq!(names("help tog"), ["toggle"]);
    assert_eq!(names("echo ren"), Vec::<String>::new());
    assert_eq!(names("time.scale "), Vec::<String>::new());
    assert_eq!(names("render.shadows on "), Vec::<String>::new());
    assert_eq!(names("spawn x 1 "), Vec::<String>::new());
    // After `;` a new command starts.
    assert_eq!(names("echo hi; tog"), ["toggle"]);
}

#[test]
fn completion_apply() {
    let (c, ..) = console();
    let tab = |line: &str| c.complete(line).apply(line);
    assert_eq!(tab("tog"), "toggle ");
    assert_eq!(tab("ren"), "render.");
    assert_eq!(tab("render."), "render.");
    assert_eq!(tab("toggle render.s"), "toggle render.shadows ");
    assert_eq!(tab("echo hi; set lod"), "echo hi; set lod.bias ");
    assert_eq!(tab("zzz"), "zzz");
    assert_eq!(c.complete("set render.mode ").common_prefix(), "");
}

#[test]
fn submit_records_history_and_echo() {
    let (mut c, ..) = console();
    let ctx = &mut Ctx::default();
    c.submit(ctx, "render.shadows off").unwrap();
    c.submit(ctx, "render.shadows off").unwrap();
    let _ = c.submit(ctx, "bogus");
    c.exec(ctx, "time.scale 2").unwrap();
    assert_eq!(
        c.history().entries().collect::<Vec<_>>(),
        ["render.shadows off", "bogus"]
    );
    let echoes = c.log().lines().filter(|l| l.level == Level::Echo).count();
    assert_eq!(echoes, 3);
    c.exec(ctx, "history").unwrap();
    assert_eq!(last_line(&c), "   2  bogus");
}

#[test]
fn history_navigation_keeps_draft() {
    let mut h = History::new(3);
    assert_eq!(h.older("x"), None);
    for l in ["a", "b", " ", "c", "d"] {
        h.push(l);
    }
    // Capacity 3: "a" dropped, blank skipped.
    assert_eq!(h.entries().collect::<Vec<_>>(), ["b", "c", "d"]);
    assert_eq!(h.newer(), None);
    assert_eq!(h.older("draft"), Some("d"));
    assert_eq!(h.older("ignored"), Some("c"));
    assert_eq!(h.older(""), Some("b"));
    assert_eq!(h.older(""), Some("b"));
    assert_eq!(h.newer(), Some("c"));
    assert_eq!(h.newer(), Some("d"));
    assert_eq!(h.newer(), Some("draft"));
    assert_eq!(h.newer(), None);
    h.load(["x", "x", "y"]);
    assert_eq!(h.entries().collect::<Vec<_>>(), ["x", "y"]);
}

#[test]
fn log_is_bounded_with_stable_seq() {
    let mut log = Log::new(3);
    log.info("a\nb");
    log.warn("c");
    log.error("d");
    assert_eq!(
        log.lines().map(|l| l.text.as_str()).collect::<Vec<_>>(),
        ["b", "c", "d"]
    );
    assert_eq!(log.next_seq(), 4);
    assert_eq!(
        log.since(3).map(|l| l.text.as_str()).collect::<Vec<_>>(),
        ["d"]
    );
    assert_eq!(log.since(0).count(), 3);
    assert_eq!(log.since(9).count(), 0);
    log.clear();
    log.info("e");
    assert_eq!(log.lines().next().unwrap().seq, 4);
}

#[test]
fn help_vars_search_echo_clear() {
    let (mut c, ..) = console();
    let ctx = &mut Ctx::default();
    c.exec(ctx, "help spawn").unwrap();
    let help: Vec<&str> = c.log().lines().map(|l| l.text.as_str()).collect();
    assert_eq!(
        help,
        [
            "usage: spawn <what:text> [count:int 1..=100]",
            "Spawn things",
            "aliases: sp"
        ]
    );
    c.exec(ctx, "? time.scale").unwrap();
    assert_eq!(last_line(&c), "time.scale = 1 [float 0..=10]");
    c.exec(ctx, "help").unwrap();
    assert_eq!(
        last_line(&c),
        "5 variable(s): `vars` lists them, `help <name>` describes one"
    );
    c.exec(ctx, "help nope").unwrap();
    assert_eq!(last_line(&c), "nothing matches \"nope\"");
    c.exec(ctx, "help rendr.water").unwrap();
    assert_eq!(
        last_line(&c),
        "nothing matches \"rendr.water\"; did you mean render.water?"
    );

    c.exec(ctx, "clear; render.water off; clear; vars render")
        .unwrap();
    let vars: Vec<&str> = c.log().lines().map(|l| l.text.as_str()).collect();
    assert_eq!(
        vars,
        [
            "3 variable(s)",
            "  render.mode = raster",
            "  render.shadows = on",
            "* render.water = off  (console)"
        ]
    );
    c.exec(ctx, "cls; help shadow").unwrap();
    assert_eq!(last_line(&c), "render.shadows = on [on|off]: Shadow maps");
    c.exec(ctx, "echo  hello   \"world\" ").unwrap();
    assert_eq!(last_line(&c), "hello   \"world\"");
}

#[test]
fn display_round_trips_through_parse() {
    for (kind, v) in [
        (ArgKind::Bool, Value::Bool(false)),
        (ArgKind::float(), Value::Float(0.25)),
        (ArgKind::int(), Value::Int(-7)),
        (ArgKind::Str, Value::Str("two words".into())),
        (ArgKind::Str, Value::Str(String::new())),
    ] {
        let text = v.to_string();
        let tokens = &split(&text)[0].tokens;
        let word = tokens.first().map_or("", |t| t.text.as_str());
        assert_eq!(kind.parse(word), Ok(v));
    }
}

#[test]
fn set_by_priority() {
    let (mut c, shadows, scale, _) = console();
    let ctx = &mut Ctx::default();
    // Command line (`--off`) first, then a config file that says otherwise: the switch holds.
    assert_eq!(
        c.vars_mut()
            .set_by(shadows, Value::Bool(false), SetBy::CommandLine),
        Ok(SetOutcome::Changed)
    );
    c.exec_as(ctx, "render.shadows on; time.scale 2", SetBy::Config)
        .unwrap();
    assert!(!c.vars().flag(shadows));
    assert_eq!(
        c.vars().get(scale),
        &Value::Float(2.0),
        "the rest of the config line applies"
    );
    let kept = c
        .log()
        .lines()
        .find(|l| l.text.starts_with("render.shadows kept"))
        .unwrap();
    assert_eq!(
        kept.text,
        "render.shadows kept: set by command line, which outranks config"
    );
    assert_eq!(
        c.vars_mut()
            .set_by(shadows, Value::Bool(true), SetBy::Config),
        Ok(SetOutcome::Outranked(SetBy::CommandLine))
    );
    // Code outranks the command line; the console outranks code.
    assert_eq!(c.vars_mut().set(shadows, Value::Bool(true)), Ok(true));
    c.submit(ctx, "toggle render.shadows").unwrap();
    assert_eq!(c.vars().var(shadows).set_by(), SetBy::Console);
    assert_eq!(c.vars_mut().set(shadows, Value::Bool(true)), Ok(false));
    assert!(!c.vars().flag(shadows));
    assert_eq!(
        last_line(&c),
        "render.shadows = off (default on) [on|off]: Shadow maps"
    );
    // Same value from a higher source still takes ownership.
    assert_eq!(
        c.vars_mut()
            .set_by(scale, Value::Float(2.0), SetBy::CommandLine),
        Ok(SetOutcome::Unchanged)
    );
    assert_eq!(c.vars().var(scale).set_by(), SetBy::CommandLine);
    assert_eq!(
        c.vars().var(scale).describe(),
        "time.scale = 2 (default 1) [float 0..=10] (set by command line)"
    );
    // `reset` gives the variable back to every source; `list` shows who set what.
    c.exec(ctx, "reset render.shadows; clear; list time")
        .unwrap();
    assert_eq!(c.vars().var(shadows).set_by(), SetBy::Default);
    assert_eq!(last_line(&c), "* time.scale = 2  (command line)");
    c.exec_as(ctx, "render.shadows off", SetBy::Config).unwrap();
    assert!(!c.vars().flag(shadows));
}

#[test]
fn prompt_editing_history_and_completion() {
    let (mut c, ..) = console();
    let mut p = Prompt::default();
    let type_text = |p: &mut Prompt, c: &mut Console<Ctx>, t: &str| {
        p.apply(Edit::Insert(t.into()), c);
    };
    type_text(&mut p, &mut c, "rendér.sh");
    p.apply(Edit::Left, &mut c);
    p.apply(Edit::Backspace, &mut c);
    assert_eq!((p.text(), p.cursor()), ("rendér.h", 8));
    p.apply(Edit::Home, &mut c);
    p.apply(Edit::Delete, &mut c);
    p.apply(Edit::Clear, &mut c);
    // Tab completes the word before the cursor; the popup lists candidates.
    type_text(&mut p, &mut c, "render.");
    assert_eq!(
        p.completion(&c).candidates,
        ["render.mode", "render.shadows", "render.water"]
    );
    type_text(&mut p, &mut c, "s");
    p.apply(Edit::Complete, &mut c);
    assert_eq!(p.text(), "render.shadows ");
    assert_eq!(p.completion(&c).candidates, ["off", "on"]);
    p.apply(Edit::Complete, &mut c);
    assert_eq!(p.text(), "render.shadows o", "extends to the common prefix");
    type_text(&mut p, &mut c, "f");
    p.apply(Edit::Complete, &mut c);
    assert_eq!(p.text(), "render.shadows off ");
    // Completion in the middle of a line keeps the tail.
    p.set("tog render.water");
    p.apply(Edit::WordLeft, &mut c);
    p.apply(Edit::Left, &mut c);
    p.apply(Edit::Complete, &mut c);
    assert_eq!((p.text(), p.cursor()), ("toggle  render.water", 7));
    p.apply(Edit::WordRight, &mut c);
    assert_eq!(p.cursor(), p.text().len());
    p.apply(Edit::DeleteWordBack, &mut c);
    assert_eq!(p.text(), "toggle  ");
    // Submit hands the line back; history browsing keeps the draft.
    p.set("echo one");
    let line = p.apply(Edit::Submit, &mut c).unwrap();
    c.submit(&mut Ctx::default(), &line).unwrap();
    assert_eq!(p.text(), "");
    c.submit(&mut Ctx::default(), "echo two").unwrap();
    type_text(&mut p, &mut c, "draft");
    p.apply(Edit::Older, &mut c);
    assert_eq!(p.text(), "echo two");
    p.apply(Edit::Older, &mut c);
    p.apply(Edit::Older, &mut c);
    assert_eq!(p.text(), "echo one");
    p.apply(Edit::Newer, &mut c);
    p.apply(Edit::Newer, &mut c);
    assert_eq!((p.text(), p.cursor()), ("draft", 5));
    // Control characters (the toggle key's text, Enter's "\r") never land in the line.
    type_text(&mut p, &mut c, "\r\t!");
    assert_eq!(p.text(), "draft!");
}

#[test]
fn fallback_runs_unclaimed_commands_and_completes() {
    let mut c: Console<Vec<String>> = Console::new();
    let mut seen = Vec::new();
    // Without a fallback an unknown word is an error.
    assert!(matches!(
        c.exec(&mut seen, "player->additem x 1"),
        Err(ExecError::Unknown { .. })
    ));
    c.set_fallback("script", |u| {
        if u.text.contains("bad") {
            return Err(format!("no {}", u.text));
        }
        u.ctx.push(u.text.to_owned());
        Ok(())
    });
    c.set_fallback_completion(|p| {
        ["player->additem", "additem"]
            .iter()
            .filter(|n| n.starts_with(p))
            .map(|n| (*n).to_owned())
            .collect()
    });
    c.exec(&mut seen, r#"player->AddItem "gold_001" 5 ; echo hi"#)
        .unwrap();
    assert_eq!(seen, [r#"player->AddItem "gold_001" 5"#]);
    // A registered command still wins.
    c.exec(&mut seen, "echo additem").unwrap();
    assert_eq!(seen.len(), 1);
    let err = c.exec(&mut seen, "bad thing").unwrap_err();
    assert_eq!(err.to_string(), "script: no bad thing");
    assert_eq!(c.complete("player->ad").candidates, ["player->additem"]);
    assert!(c.complete("add").candidates.contains(&"additem".to_owned()));
}
