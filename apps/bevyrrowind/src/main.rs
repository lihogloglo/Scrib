//! Bevyrrowind command line.
//!
//! Today: inspection tools over the user's Morrowind install. Later, running
//! without a subcommand will launch the game.

mod view;

use std::path::PathBuf;
use std::process::ExitCode;
use std::time::Instant;

use clap::{Parser, Subcommand};
use mw_install::{MorrowindInstall, Origin};
use mw_records::{KindVisitor, LoadedContent};
use sf_records::{RecordDb, RecordId};
use sf_vfs::{DirEntry, Vfs, VfsPath};

#[derive(Parser)]
#[command(version, about)]
struct Cli {
    /// Morrowind install folder (or its Data Files). Overrides detection,
    /// including the `BEVYRROWIND_MORROWIND` environment variable.
    #[arg(long, global = true)]
    morrowind: Option<PathBuf>,
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Show where detection looks and what it finds.
    Detect,
    /// Mount the VFS, load all content, and print a summary.
    Info {
        /// Conflicts to list per record kind.
        #[arg(long, default_value_t = 5)]
        conflicts: usize,
    },
    /// Print one record and the files that touched it.
    Record { kind: String, id: String },
    /// List record IDs of a kind, optionally containing a substring.
    Find {
        kind: String,
        pattern: Option<String>,
        #[arg(long, default_value_t = 50)]
        limit: usize,
    },
    /// List a VFS directory.
    Ls { dir: Option<String> },
    /// Show which sources provide a file, winner last.
    Which { path: String },
    /// Print a NIF model's converted node tree and materials.
    Nif { path: String },
    /// Render a cell (interior name, or exterior grid as "x,y").
    View {
        cell: String,
        /// Save a screenshot here once loaded, then exit.
        #[arg(long)]
        screenshot: Option<PathBuf>,
        /// Seconds to render after loading before the screenshot.
        #[arg(long, default_value_t = 2.0)]
        settle: f32,
        /// Camera eye and target in meters: "ex,ey,ez,tx,ty,tz".
        #[arg(long, value_parser = parse_camera)]
        camera: Option<(bevy::math::Vec3, bevy::math::Vec3)>,
    },
}

fn parse_camera(raw: &str) -> std::result::Result<(bevy::math::Vec3, bevy::math::Vec3), String> {
    let values: Vec<f32> =
        raw.split(',').map(|v| v.trim().parse::<f32>().map_err(|e| e.to_string())).collect::<std::result::Result<_, _>>()?;
    match values[..] {
        [ex, ey, ez, tx, ty, tz] => Ok((bevy::math::Vec3::new(ex, ey, ez), bevy::math::Vec3::new(tx, ty, tz))),
        _ => Err("expected six comma-separated numbers".into()),
    }
}

fn main() -> ExitCode {
    let cli = Cli::parse();
    match run(cli) {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("error: {error}");
            ExitCode::FAILURE
        }
    }
}

type Result<T = ()> = std::result::Result<T, Box<dyn std::error::Error>>;

fn run(cli: Cli) -> Result {
    if let Command::Detect = cli.command {
        return detect(cli.morrowind.as_deref());
    }
    let (install, origin) = MorrowindInstall::detect(cli.morrowind.as_deref())?;
    match cli.command {
        Command::Detect => unreachable!(),
        Command::Info { conflicts } => info(&install, &origin, conflicts),
        Command::Ls { dir } => {
            let vfs = mount(&install, false)?;
            ls(&vfs, dir.as_deref().unwrap_or(""));
            Ok(())
        }
        Command::Which { path } => {
            let vfs = mount(&install, false)?;
            which(&vfs, &path)
        }
        Command::Nif { path } => {
            let vfs = mount(&install, false)?;
            nif(&vfs, &path)
        }
        Command::Record { kind, id } => {
            let (_, content) = load(&install, false)?;
            record(&content.db, &kind, &id)
        }
        Command::Find { kind, pattern, limit } => {
            let (_, content) = load(&install, false)?;
            find(&content.db, &kind, pattern.as_deref(), limit)
        }
        Command::View { cell, screenshot, settle, camera } => {
            let (vfs, content) = load(&install, false)?;
            let found = mw_world::find_cell(&content.db, &cell).ok_or_else(|| format!("no cell {cell:?}"))?;
            let (placements, skipped) = mw_world::cell_placements(&content.db, found);
            if !skipped.is_empty() {
                eprintln!("warning: {} references with unknown base records", skipped.len());
            }
            println!("{}: {} placements", found.name, placements.len());
            view::run(
                std::sync::Arc::new(vfs),
                view::ViewOptions {
                    title: format!("Bevyrrowind - {}", found.name),
                    placements,
                    ambient: found.atmosphere_data.as_ref().map(|a| a.ambient_color),
                    screenshot,
                    settle,
                    camera,
                },
            );
            Ok(())
        }
    }
}

fn detect(explicit: Option<&std::path::Path>) -> Result {
    for candidate in mw_install::candidates(explicit) {
        let verdict = match MorrowindInstall::open(&candidate.path) {
            Ok(_) => "OK",
            Err(_) => "-",
        };
        println!("{verdict:>3}  {:<40} {}", format!("{:?}", candidate.origin), candidate.path.display());
    }
    match MorrowindInstall::detect(explicit) {
        Ok((install, origin)) => {
            println!("\nusing {} ({origin:?})", install.root.display());
            println!("game files: {}", install.game_files().join(", "));
            println!("archives:   {}", install.archives().join(", "));
            Ok(())
        }
        Err(error) => Err(error.into()),
    }
}

fn mount(install: &MorrowindInstall, verbose: bool) -> Result<Vfs> {
    let start = Instant::now();
    let mounted = mw_vfs::mount_install(install, Vfs::builder())?;
    for warning in &mounted.warnings {
        eprintln!("warning: {warning:?}");
    }
    if verbose {
        println!("VFS: {} files in {:.0?}", mounted.vfs.len(), start.elapsed());
        for s in mounted.vfs.stats() {
            println!("  {:<20} {:>7} files, {:>7} winning", s.label, s.files, s.winning);
        }
    }
    Ok(mounted.vfs)
}

fn load(install: &MorrowindInstall, verbose: bool) -> Result<(Vfs, LoadedContent)> {
    let vfs = mount(install, verbose)?;
    let start = Instant::now();
    let content = mw_records::load(&vfs, &install.game_files())?;
    if verbose {
        println!("\ncontent: {} files in {:.0?}", content.files.len(), start.elapsed());
        for f in &content.files {
            println!(
                "  {:<24} {:?} {:>7} objects  read {:>6.0?}  parse {:>6.0?}  apply {:>6.0?}  masters: {}",
                f.name,
                f.file_type,
                f.objects,
                f.read_time,
                f.parse_time,
                f.apply_time,
                if f.masters.is_empty() { "-".to_owned() } else { f.masters.join(", ") }
            );
        }
    }
    let mut counts = std::collections::BTreeMap::<String, usize>::new();
    for warning in &content.warnings {
        let name = format!("{warning:?}");
        let variant = name.split([' ', '{', '(']).next().unwrap_or_default().to_owned();
        *counts.entry(variant).or_default() += 1;
    }
    for (variant, count) in counts {
        eprintln!("warning: {count} x {variant}");
    }
    Ok((vfs, content))
}

fn info(install: &MorrowindInstall, origin: &Origin, conflicts: usize) -> Result {
    println!("install: {} ({origin:?})", install.root.display());
    let (_, content) = load(install, true)?;
    let report = content.db.conflict_report();
    println!("\nrecords: {} live\n", report.total_live());
    print!("{}", report.render(&content.db, conflicts));
    Ok(())
}

fn ls(vfs: &Vfs, dir: &str) {
    for entry in vfs.read_dir(dir) {
        match entry {
            DirEntry::Dir(name) => println!("{name}/"),
            DirEntry::File(path) => {
                let winner = vfs.resolve(&path).map(|s| vfs.source(s).label().to_owned()).unwrap_or_default();
                println!("{:<48} {winner}", path.file_name());
            }
        }
    }
}

fn which(vfs: &Vfs, raw: &str) -> Result {
    let path = VfsPath::new(raw)?;
    let providers = vfs.providers(&path);
    if providers.is_empty() {
        return Err(format!("{path} is not in the VFS").into());
    }
    for source in providers {
        println!("{}", vfs.source(*source).label());
    }
    Ok(())
}

fn nif(vfs: &Vfs, raw: &str) -> Result {
    let path = match mw_world::model_path(raw) {
        Some(p) if !raw.to_ascii_lowercase().replace('\\', "/").starts_with("meshes/") => p,
        _ => raw.to_owned(),
    };
    let model = mw_nif::load(&vfs.read(&VfsPath::new(&path)?)?)?;
    println!("{path}: marker={} no_collision={} had_skin={}", model.marker, model.no_collision, model.had_skin);
    for (i, node) in model.nodes.iter().enumerate() {
        let depth = std::iter::successors(node.parent, |&p| model.nodes[p].parent).count();
        let mesh = node.mesh.map_or(String::new(), |m| {
            let mesh = &model.meshes[m];
            format!("  mesh#{m} {} tris, material#{}", mesh.indices.len() / 3, mesh.material)
        });
        let hidden = if node.visible { "" } else { "  [hidden]" };
        println!("{i:>3} {}{:?} {:?}{mesh}{hidden}", "  ".repeat(depth), node.kind, node.name);
    }
    for (i, m) in model.materials.iter().enumerate() {
        println!("material#{i}: {:?} {:?} alpha={:?}", m.base_texture, m.glow_texture, m.alpha_mode);
    }
    Ok(())
}

fn unknown_kind(kind: &str) -> Box<dyn std::error::Error> {
    format!("unknown record kind {kind:?}; known kinds: {}", mw_records::KINDS.join(" ")).into()
}

fn record(db: &RecordDb, kind: &str, id: &str) -> Result {
    struct Show<'a>(&'a RecordDb, RecordId);
    impl KindVisitor for Show<'_> {
        type Output = Option<String>;
        fn visit<T: std::fmt::Debug + Send + Sync + 'static>(self) -> Option<String> {
            let entry = self.0.entry::<T>(&self.1)?;
            let history: Vec<String> =
                entry.history.iter().map(|t| format!("{} ({:?})", self.0.source(t.source).name, t.op)).collect();
            let value = entry.value.as_ref().map_or_else(|| "<deleted>".to_owned(), |v| format!("{v:#?}"));
            Some(format!("{value}\n\nhistory: {}", history.join(" -> ")))
        }
    }
    let shown = mw_records::visit_kind(kind, Show(db, RecordId::new(id))).ok_or_else(|| unknown_kind(kind))?;
    println!("{}", shown.ok_or_else(|| format!("no {kind} record {id:?}"))?);
    Ok(())
}

fn find(db: &RecordDb, kind: &str, pattern: Option<&str>, limit: usize) -> Result {
    struct Find<'a>(&'a RecordDb, Option<String>, usize);
    impl KindVisitor for Find<'_> {
        type Output = (Vec<String>, usize);
        fn visit<T: std::fmt::Debug + Send + Sync + 'static>(self) -> (Vec<String>, usize) {
            let matches: Vec<String> = self
                .0
                .iter::<T>()
                .map(|(id, _)| id.to_string())
                .filter(|id| self.1.as_deref().is_none_or(|p| id.contains(p)))
                .collect();
            let total = matches.len();
            (matches.into_iter().take(self.2).collect(), total)
        }
    }
    let pattern = pattern.map(str::to_ascii_lowercase);
    let (ids, total) = mw_records::visit_kind(kind, Find(db, pattern, limit)).ok_or_else(|| unknown_kind(kind))?;
    for id in &ids {
        println!("{}", id.replace('\u{1f}', " / "));
    }
    if total > ids.len() {
        println!("... {} more (use --limit)", total - ids.len());
    }
    Ok(())
}
