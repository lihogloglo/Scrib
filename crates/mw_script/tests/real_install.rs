//! W1 corpus test: every script text (SCPT, all files, overridden ones
//! included) and every dialogue result (INFO `BNAM`) of the installed game
//! files parses and resolves. Prints counts per leniency rule and lists
//! unrecoverable scripts; never prints or stores the source text.
//! Skips (passes) when no Morrowind install is detected.

use std::collections::{BTreeMap, HashMap, HashSet};
use std::time::Instant;

use mw_install::MorrowindInstall;
use mw_script::ast::VarType;
use mw_script::{
    Env, FunctionTable, MemberLookup, Parsed, Rule, Severity, SourceKind, parse, parse_and_resolve,
};
use tes3::esp::{EditorId, GlobalValue, Plugin, TES3Object};

struct Corpus {
    /// (file, script id, text, compiled bytecode length), in load order; overridden versions included.
    scripts: Vec<(String, String, String, usize)>,
    /// (file, topic, info id, result text).
    results: Vec<(String, String, String, String)>,
    globals: HashMap<String, VarType>,
    ids: HashSet<String>,
    /// Object ID -> script ID (lowercase).
    object_scripts: HashMap<String, String>,
}

fn script_of(object: &TES3Object) -> Option<&str> {
    use TES3Object as O;
    let s = match object {
        O::Activator(x) => &x.script,
        O::Alchemy(x) => &x.script,
        O::Apparatus(x) => &x.script,
        O::Armor(x) => &x.script,
        O::Book(x) => &x.script,
        O::Clothing(x) => &x.script,
        O::Container(x) => &x.script,
        O::Creature(x) => &x.script,
        O::Door(x) => &x.script,
        O::Ingredient(x) => &x.script,
        O::Light(x) => &x.script,
        O::Lockpick(x) => &x.script,
        O::MiscItem(x) => &x.script,
        O::Npc(x) => &x.script,
        O::Probe(x) => &x.script,
        O::RepairItem(x) => &x.script,
        O::Weapon(x) => &x.script,
        _ => return None,
    };
    (!s.is_empty()).then_some(s.as_str())
}

fn load(install: &MorrowindInstall) -> Corpus {
    let mut c = Corpus {
        scripts: Vec::new(),
        results: Vec::new(),
        globals: HashMap::new(),
        ids: HashSet::from(["player".to_owned()]),
        object_scripts: HashMap::new(),
    };
    for file in install.game_files() {
        let bytes = std::fs::read(install.data_files.join(&file)).unwrap();
        let mut plugin = Plugin::new();
        plugin
            .load_bytes_filtered(&bytes, |tag| !matches!(&tag, b"LAND" | b"PGRD" | b"CELL"))
            .unwrap();
        let mut topic = String::new();
        for object in &plugin.objects {
            match object {
                TES3Object::Script(s) => {
                    c.scripts
                        .push((file.clone(), s.id.clone(), s.text.clone(), s.bytecode.len()));
                }
                TES3Object::Dialogue(d) => topic.clone_from(&d.id),
                TES3Object::DialogueInfo(i) if !i.script_text.is_empty() => {
                    c.results.push((
                        file.clone(),
                        topic.clone(),
                        i.id.clone(),
                        i.script_text.clone(),
                    ));
                }
                TES3Object::GlobalVariable(g) => {
                    let ty = match g.value {
                        GlobalValue::Short(_) => VarType::Short,
                        GlobalValue::Long(_) => VarType::Long,
                        GlobalValue::Float(_) => VarType::Float,
                    };
                    c.globals.insert(g.id.to_lowercase(), ty);
                }
                _ => {}
            }
            c.ids.insert(object.editor_id().to_lowercase());
            if let Some(script) = script_of(object) {
                c.object_scripts
                    .insert(object.editor_id().to_lowercase(), script.to_lowercase());
            }
        }
    }
    c
}

struct CorpusEnv<'a> {
    corpus: &'a Corpus,
    /// Script ID -> (local name -> type), live versions.
    locals: HashMap<String, HashMap<String, VarType>>,
}

impl Env for CorpusEnv<'_> {
    fn global(&self, key: &str) -> Option<VarType> {
        self.corpus.globals.get(key).copied()
    }
    fn object_exists(&self, key: &str) -> Option<bool> {
        Some(self.corpus.ids.contains(key))
    }
    fn member(&self, object: &str, var: &str) -> MemberLookup {
        let script = self
            .corpus
            .object_scripts
            .get(object)
            .map_or(object, String::as_str);
        match self.locals.get(script) {
            Some(vars) => vars
                .get(var)
                .map_or(MemberLookup::NoSuchVar, |t| MemberLookup::Found(*t)),
            None => MemberLookup::NoSuchObject,
        }
    }
}

#[derive(Default)]
struct Tally {
    texts: usize,
    clean: usize,
    inert: Vec<String>,
    rules: BTreeMap<Rule, (usize, usize)>,
}

impl Tally {
    fn add(&mut self, what: String, p: &Parsed, text: &str) {
        // Local investigation only: `MW_SCRIPT_DUMP=<dir>` writes every text with a
        // repair or worse there (the owner's own data; never commit it).
        if let Ok(dir) = std::env::var("MW_SCRIPT_DUMP")
            && p.diagnostics
                .iter()
                .any(|d| d.severity() > Severity::Accepted)
        {
            let name: String = what
                .chars()
                .map(|c| if c.is_alphanumeric() { c } else { '_' })
                .collect();
            let diags: Vec<String> = p.diagnostics.iter().map(ToString::to_string).collect();
            let body = format!("; {what}\n; {}\n{text}", diags.join("\n; "));
            std::fs::write(std::path::Path::new(&dir).join(format!("{name}.txt")), body).unwrap();
        }
        self.texts += 1;
        if p.diagnostics.0.is_empty() {
            self.clean += 1;
        }
        let mut seen = HashSet::new();
        for d in p.diagnostics.iter() {
            let e = self.rules.entry(d.rule).or_default();
            e.0 += 1;
            if seen.insert(d.rule) {
                e.1 += 1;
            }
        }
        if p.is_inert() {
            let errs: Vec<String> = p
                .diagnostics
                .iter()
                .filter(|d| d.severity() == Severity::Error)
                .map(ToString::to_string)
                .collect();
            self.inert.push(format!("{what}: {}", errs.join("; ")));
        }
    }

    fn print(&self, name: &str) {
        eprintln!(
            "{name}: {} texts, {} with no diagnostics, {} unrecoverable",
            self.texts,
            self.clean,
            self.inert.len()
        );
        for (rule, (n, texts)) in &self.rules {
            eprintln!(
                "  {:<22} {:>9?} {n:>6} hits in {texts:>5} texts",
                rule.name(),
                rule.severity()
            );
        }
        for i in &self.inert {
            eprintln!("  INERT {i}");
        }
    }
}

#[test]
#[allow(clippy::cast_precision_loss)]
fn vanilla_corpus_parses() {
    let Ok((install, _)) = MorrowindInstall::detect(None) else {
        eprintln!("no Morrowind install detected; skipping");
        return;
    };
    let table = FunctionTable::vanilla();
    let t0 = Instant::now();
    let corpus = load(&install);
    let load_time = t0.elapsed();

    // Live scripts (the last file's version wins) and their locals.
    let mut live: HashMap<String, &str> = HashMap::new();
    for (_, id, text, _) in &corpus.scripts {
        live.insert(id.to_lowercase(), text);
    }
    let t1 = Instant::now();
    let locals = live
        .iter()
        .map(|(id, text)| {
            let p = parse(text, SourceKind::Script, table);
            (
                id.clone(),
                p.script
                    .locals
                    .iter()
                    .map(|l| (l.name.key.clone(), l.ty))
                    .collect(),
            )
        })
        .collect();
    let env = CorpusEnv {
        corpus: &corpus,
        locals,
    };

    let mut calls: BTreeMap<String, usize> = BTreeMap::new();
    let mut scripts = Tally::default();
    for (file, id, text, bytecode) in &corpus.scripts {
        let p = parse_and_resolve(text, SourceKind::Script, table, &env);
        p.script.walk_calls(&mut |c| {
            *calls
                .entry(table.function(c.function).family.clone())
                .or_default() += 1;
        });
        scripts.add(
            format!("{file} SCPT {id} (original bytecode {bytecode} B)"),
            &p,
            text,
        );
    }
    let mut results = Tally::default();
    for (file, topic, info, text) in &corpus.results {
        let p = parse_and_resolve(text, SourceKind::DialogueResult, table, &env);
        p.script.walk_calls(&mut |c| {
            *calls
                .entry(table.function(c.function).family.clone())
                .or_default() += 1;
        });
        results.add(format!("{file} INFO {topic}/{info}"), &p, text);
    }
    let parse_time = t1.elapsed();

    eprintln!(
        "corpus: {} script texts ({} live), {} dialogue results; {} globals, {} IDs; load {:.2?}, parse+resolve {:.2?}",
        corpus.scripts.len(),
        live.len(),
        corpus.results.len(),
        corpus.globals.len(),
        corpus.ids.len(),
        load_time,
        parse_time
    );
    scripts.print("scripts");
    results.print("dialogue results");
    let mut families: Vec<_> = calls.into_iter().collect();
    families.sort_by_key(|a| std::cmp::Reverse(a.1));
    let families: Vec<String> = families.iter().map(|(f, n)| format!("{f} {n}")).collect();
    eprintln!(
        "call sites by family (scripts + results): {}",
        families.join(", ")
    );

    // The vanilla game files: exact counts (scripting-adapter.md "The corpus").
    let files: Vec<String> = install
        .game_files()
        .iter()
        .map(|f| f.to_lowercase())
        .collect();
    if files == ["morrowind.esm", "tribunal.esm", "bloodmoon.esm"] {
        assert_eq!(
            (corpus.scripts.len(), live.len(), corpus.results.len()),
            (1_231, 1_207, 17_144)
        );
    }

    assert!(corpus.scripts.len() >= 1_000, "{}", corpus.scripts.len());
    assert!(corpus.results.len() >= 10_000, "{}", corpus.results.len());
    let inert = scripts.inert.len() + results.inert.len();
    assert_eq!(inert, 0, "unrecoverable texts (listed above)");
}
