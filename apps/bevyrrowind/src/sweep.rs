//! `bevyrrowind nif-sweep`: converts every NIF in the VFS and reports
//! failures and the object types the converter doesn't understand, ranked
//! by how many files use them.

use std::collections::BTreeMap;
use std::sync::Mutex;
use std::time::{Duration, Instant};

use mw_nif::census::{self, Support};
use sf_vfs::{Vfs, VfsPath};

#[derive(Default)]
struct Tally {
    files: usize,
    converted: usize,
    failures: Vec<(String, String)>,
    /// Type name -> how much content uses it.
    types: BTreeMap<&'static str, TypeUse>,
    parse_time: Duration,
}

#[derive(Default)]
struct TypeUse {
    files: usize,
    objects: usize,
    examples: Vec<String>,
}

impl Tally {
    fn merge(&mut self, other: Self, examples: usize) {
        self.files += other.files;
        self.converted += other.converted;
        self.failures.extend(other.failures);
        self.parse_time += other.parse_time;
        for (name, used) in other.types {
            let entry = self.types.entry(name).or_default();
            entry.files += used.files;
            entry.objects += used.objects;
            entry.examples.extend(used.examples);
            entry.examples.sort();
            entry.examples.truncate(examples);
        }
    }
}

pub fn run(vfs: &Vfs, prefix: &str, examples: usize, show_all: bool) {
    let start = Instant::now();
    let prefix = prefix.to_ascii_lowercase().replace('\\', "/");
    let mut paths: Vec<&VfsPath> =
        vfs.paths().filter(|p| p.extension() == Some("nif") && p.as_str().starts_with(&prefix)).collect();
    paths.sort();

    let workers = std::thread::available_parallelism().map_or(4, usize::from);
    let chunk = paths.len().div_ceil(workers).max(1);
    let total = Mutex::new(Tally::default());
    std::thread::scope(|scope| {
        for part in paths.chunks(chunk) {
            let total = &total;
            scope.spawn(move || {
                let mut tally = Tally::default();
                for path in part {
                    sweep_one(vfs, path, examples, &mut tally);
                }
                total.lock().expect("no worker panicked").merge(tally, examples);
            });
        }
    });
    let mut tally = total.into_inner().expect("no worker panicked");
    tally.failures.sort();

    println!(
        "{} NIFs under {:?}: {} converted, {} failed ({:.1?} total, {:.1?} parse+convert CPU)\n",
        tally.files,
        if prefix.is_empty() { "/" } else { &prefix },
        tally.converted,
        tally.failures.len(),
        start.elapsed(),
        tally.parse_time,
    );
    for (path, error) in &tally.failures {
        println!("FAIL {path}: {error}");
    }
    if !tally.failures.is_empty() {
        println!();
    }

    // Rank by files affected: that's what a missing feature costs.
    let mut rows: Vec<(&&str, &TypeUse, Support)> =
        tally.types.iter().map(|(name, row)| (name, row, census::support(name))).collect();
    rows.sort_by(|a, b| (a.2 == Support::Converted).cmp(&(b.2 == Support::Converted)).then(b.1.files.cmp(&a.1.files)));
    println!("{:<30} {:>6} {:>8}  status", "type", "files", "objects");
    for (name, used, support) in rows {
        let status = match support {
            Support::Converted if !show_all => continue,
            Support::Converted => "ok".to_owned(),
            Support::Partial(note) => format!("partial: {note}"),
            Support::Ignored(note) => format!("ignored: {note}"),
            Support::Unsupported => "UNSUPPORTED".to_owned(),
        };
        println!("{name:<30} {:>6} {:>8}  {status}", used.files, used.objects);
        if show_all || support == Support::Unsupported {
            for path in &used.examples {
                println!("{:<30} {:>6} {:>8}    e.g. {path}", "", "", "");
            }
        }
    }
}

fn sweep_one(vfs: &Vfs, path: &VfsPath, examples: usize, tally: &mut Tally) {
    tally.files += 1;
    let bytes = match vfs.read(path) {
        Ok(bytes) => bytes,
        Err(error) => {
            tally.failures.push((path.to_string(), format!("read: {error}")));
            return;
        }
    };
    let start = Instant::now();
    let stream = match mw_nif::parse(&bytes) {
        Ok(stream) => stream,
        Err(error) => {
            tally.failures.push((path.to_string(), error.to_string()));
            return;
        }
    };
    for (name, count) in census::object_types(&stream) {
        let entry = tally.types.entry(name).or_default();
        entry.files += 1;
        entry.objects += count;
        if entry.examples.len() < examples {
            entry.examples.push(path.to_string());
        }
    }
    match mw_nif::convert(stream) {
        Ok(_) => tally.converted += 1,
        Err(error) => tally.failures.push((path.to_string(), format!("convert: {error}"))),
    }
    tally.parse_time += start.elapsed();
}
