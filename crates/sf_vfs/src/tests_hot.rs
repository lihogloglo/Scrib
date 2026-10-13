//! Hot reload (mods.md "Hot reload", ROADMAP M5 A4): rebuild, diff, swap, watcher.

use std::path::Path;
use std::sync::Arc;
use std::time::{Duration, Instant};

use super::*;

fn p(s: &str) -> VfsPath {
    VfsPath::new(s).unwrap()
}

fn write(path: &Path, text: &str) {
    std::fs::create_dir_all(path.parent().unwrap()).unwrap();
    std::fs::write(path, text).unwrap();
}

/// Two directory layers (`low` < `high`) over an archive, and the rebuild after edits.
#[test]
fn rebuild_changes_winners_and_ignores_shadowed_edits() {
    let tmp = tempfile::tempdir().unwrap();
    let (low, high) = (tmp.path().join("low"), tmp.path().join("high"));
    write(&low.join("a.txt"), "a");
    write(&low.join("shadowed.txt"), "low");
    write(&high.join("shadowed.txt"), "high");
    write(&low.join("gone.txt"), "x");
    write(&low.join("over.txt"), "low");
    let vfs = Vfs::builder()
        .mount(MemSource::new("archive").with("a.txt", "archived"))
        .mount(DirSource::scan("low", &low).unwrap())
        .mount(DirSource::scan("high", &high).unwrap())
        .build();
    assert_eq!(vfs.watch_roots(), [low.clone(), high.clone()]);

    write(&low.join("a.txt"), "edited, longer");
    write(&low.join("shadowed.txt"), "edited below the winner");
    std::fs::remove_file(low.join("gone.txt")).unwrap();
    write(&low.join("new/b.txt"), "b");
    write(&high.join("over.txt"), "high wins now");

    let (only_low, changes) = vfs.rebuild(&|r: &Path| r == low).unwrap();
    assert!(only_low.contains(&p("new/b.txt")));
    assert_eq!(only_low.read(&p("over.txt")).unwrap(), b"low", "high is not rescanned");
    assert!(changes.iter().all(|c| c.path != p("over.txt")));

    let (next, changes) = vfs.rebuild(&|_| true).unwrap();
    let got: Vec<(&str, ChangeKind)> = changes.iter().map(|c| (c.path.as_str(), c.kind)).collect();
    assert_eq!(
        got,
        [
            ("a.txt", ChangeKind::Modified),
            ("gone.txt", ChangeKind::Removed),
            ("new/b.txt", ChangeKind::Added),
            ("over.txt", ChangeKind::Modified),
        ],
        "the edit under high's shadowed.txt is no change"
    );
    assert_eq!(next.read(&p("over.txt")).unwrap(), b"high wins now");
    assert_eq!(next.stats().len(), 3, "same layers in the same positions");
    assert_eq!(vfs.read(&p("a.txt")).unwrap(), b"edited, longer", "old snapshots read loose files from disk");

    let (_, none) = next.rebuild(&|_| true).unwrap();
    assert!(none.is_empty(), "nothing changed since: {none:?}");
}

/// Readers load the current snapshot while a writer swaps; neither waits for
/// the other, and a reader holding an old snapshot keeps it intact.
#[test]
fn snapshot_swap_is_visible_and_never_blocks_readers() {
    use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
    let snapshot = |n: usize| Arc::new(Vfs::builder().mount(MemSource::new("m").with("n.txt", n.to_string())).build());
    let shared = SharedVfs::new(snapshot(0));
    let held = shared.load();
    let previous = shared.store(snapshot(1));
    assert!(Arc::ptr_eq(&held, &previous));
    assert_eq!(held.read(&p("n.txt")).unwrap(), b"0", "a held snapshot stays as it was");
    assert_eq!(shared.load().read(&p("n.txt")).unwrap(), b"1", "new reads see the swap");

    let done = Arc::new(AtomicBool::new(false));
    let reads = Arc::new(AtomicUsize::new(0));
    let readers: Vec<_> = (0..4)
        .map(|_| {
            let (shared, done, reads) = (shared.clone(), done.clone(), reads.clone());
            std::thread::spawn(move || {
                let (mut last, mut slowest) = (0, Duration::ZERO);
                while !done.load(Ordering::Relaxed) {
                    let start = Instant::now();
                    let vfs = shared.load();
                    slowest = slowest.max(start.elapsed());
                    let n: usize = String::from_utf8(vfs.read(&p("n.txt")).unwrap()).unwrap().parse().unwrap();
                    assert!(n >= last, "snapshots only move forward");
                    last = n;
                    reads.fetch_add(1, Ordering::Relaxed);
                }
                slowest
            })
        })
        .collect();
    for n in 2..=2000 {
        shared.store(snapshot(n));
    }
    while reads.load(Ordering::Relaxed) < 10_000 {
        std::thread::yield_now();
    }
    done.store(true, Ordering::Relaxed);
    for reader in readers {
        let slowest = reader.join().unwrap();
        // Lock-free loads; a generous bound for a busy machine, far below a lock convoy.
        assert!(slowest < Duration::from_millis(50), "a load took {slowest:?}");
    }
    assert_eq!(shared.load().read(&p("n.txt")).unwrap(), b"2000");
}

#[cfg(feature = "watch")]
#[test]
fn watcher_rebuilds_swaps_and_reports_winning_changes() {
    use crate::watch::{VfsWatcher, WatchEvent};
    let tmp = tempfile::tempdir().unwrap();
    let (low, high) = (tmp.path().join("low"), tmp.path().join("high"));
    write(&low.join("tex/a.png"), "a");
    write(&low.join("tex/hidden.png"), "low");
    write(&high.join("tex/hidden.png"), "high");
    let vfs =
        Vfs::builder().mount(DirSource::scan("low", &low).unwrap()).mount(DirSource::scan("high", &high).unwrap()).build();
    let shared = SharedVfs::new(Arc::new(vfs));
    let (tx, rx) = std::sync::mpsc::channel();
    let watcher = VfsWatcher::start(shared.clone(), Duration::from_millis(50), move |e| {
        let _ = tx.send(match e {
            WatchEvent::Updated(u) => Ok(u.changes),
            WatchEvent::Failed(e) => Err(e),
        });
    })
    .unwrap();
    assert_eq!(watcher.roots().len(), 2);
    let wait = Duration::from_secs(10);

    // A shadowed edit alone reports nothing; the winning edit after it does.
    write(&low.join("tex/hidden.png"), "edited under the winner");
    std::thread::sleep(Duration::from_millis(400));
    write(&low.join("tex/a.png"), "edited a");
    let changes = rx.recv_timeout(wait).expect("watcher event").expect("no watcher error");
    assert_eq!(changes, [VfsChange { path: p("tex/a.png"), kind: ChangeKind::Modified }]);
    assert_eq!(shared.load().read(&p("tex/a.png")).unwrap(), b"edited a");

    write(&high.join("tex/b.png"), "b");
    let changes = rx.recv_timeout(wait).expect("watcher event").expect("no watcher error");
    assert_eq!(changes, [VfsChange { path: p("tex/b.png"), kind: ChangeKind::Added }]);
    assert!(shared.load().contains(&p("tex/b.png")), "new reads see the added file");
    drop(watcher);
}
