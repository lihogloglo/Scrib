//! A cross-process lock file for benchmark chains: one bench at a time on the machine, so two never
//! measure each other.
//!
//! The file holds `key=value` lines ([`Holder`]): who holds it (pid, process name), since when, what it runs,
//! and anything the holder must undo if it dies (a power plan to restore). It is created complete and
//! atomically (written to a private temp file, then hard-linked into place, which fails when the lock
//! exists), so a reader never sees a half-written holder. A holder whose process is gone (pid not running,
//! or now running under another name: pid reuse) is stale and gets taken over; the taker reads its undo
//! notes first. Dropping a [`BenchLock`] removes the file.

use std::fmt::Write as _;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

/// Who holds the lock.
#[derive(Clone, Debug, PartialEq)]
pub struct Holder {
    pub pid: u32,
    /// The holder's process name (`bevyrrowind.exe`), to tell pid reuse from the holder.
    pub name: String,
    /// When it took the lock (Unix seconds).
    pub since_unix_s: f64,
    /// What it runs (a command line), shown to whoever waits.
    pub what: String,
    /// Extra lines (`key=value`) the holder keeps for whoever takes over a stale lock.
    pub notes: Vec<(String, String)>,
}

impl Holder {
    /// The file contents.
    pub fn to_text(&self) -> String {
        let mut s = format!("pid={}\nname={}\nsince={:.0}\nwhat={}\n", self.pid, self.name, self.since_unix_s, one_line(&self.what));
        for (k, v) in &self.notes {
            let _ = writeln!(s, "{k}={}", one_line(v));
        }
        s
    }

    /// Parses [`Holder::to_text`]; `None` when `pid` is missing (not a lock file).
    pub fn parse(text: &str) -> Option<Self> {
        let mut h = Self { pid: 0, name: String::new(), since_unix_s: 0.0, what: String::new(), notes: Vec::new() };
        let mut has_pid = false;
        for line in text.lines() {
            let Some((k, v)) = line.split_once('=') else { continue };
            match k.trim() {
                "pid" => {
                    h.pid = v.trim().parse().ok()?;
                    has_pid = true;
                }
                "name" => v.trim().clone_into(&mut h.name),
                "since" => h.since_unix_s = v.trim().parse().unwrap_or(0.0),
                "what" => v.trim().clone_into(&mut h.what),
                k => h.notes.push((k.to_owned(), v.trim().to_owned())),
            }
        }
        has_pid.then_some(h)
    }

    /// A note's value.
    pub fn note(&self, key: &str) -> Option<&str> {
        self.notes.iter().find(|(k, _)| k == key).map(|(_, v)| v.as_str())
    }

    /// Whether the holder still runs, given the running processes as `(pid, name)`: the same pid under the
    /// same name (case-insensitive; an empty recorded name matches any).
    pub fn alive<'a>(&self, running: impl IntoIterator<Item = (u32, &'a str)>) -> bool {
        running.into_iter().any(|(pid, name)| pid == self.pid && (self.name.is_empty() || name.eq_ignore_ascii_case(&self.name)))
    }
}

fn one_line(s: &str) -> String {
    s.replace(['\r', '\n'], " ")
}

/// What [`try_acquire`] found.
#[derive(Debug)]
pub enum Attempt {
    /// The lock is ours; `stale` is the dead holder it replaced, if any (its notes may need undoing).
    Acquired { lock: BenchLock, stale: Option<Holder> },
    /// Someone alive holds it.
    Held(Holder),
}

/// A held lock; dropping it removes the file (when it is still ours).
#[derive(Debug)]
pub struct BenchLock {
    path: PathBuf,
    holder: Holder,
}

impl BenchLock {
    pub fn path(&self) -> &Path {
        &self.path
    }

    pub fn holder(&self) -> &Holder {
        &self.holder
    }

    /// Rewrites the file with `holder` (e.g. after adding an undo note).
    pub fn update(&mut self, holder: Holder) -> io::Result<()> {
        let tmp = temp_path(&self.path, holder.pid);
        fs::write(&tmp, holder.to_text())?;
        // Replaces the old contents in one step (rename over an existing file).
        fs::rename(&tmp, &self.path)?;
        self.holder = holder;
        Ok(())
    }
}

impl Drop for BenchLock {
    fn drop(&mut self) {
        let ours = fs::read_to_string(&self.path).ok().and_then(|t| Holder::parse(&t)).is_some_and(|h| h.pid == self.holder.pid);
        if ours {
            let _ = fs::remove_file(&self.path);
        }
    }
}

fn temp_path(path: &Path, pid: u32) -> PathBuf {
    let mut name = path.file_name().map(std::ffi::OsStr::to_os_string).unwrap_or_default();
    name.push(format!(".{pid}.tmp"));
    path.with_file_name(name)
}

/// One attempt to take the lock at `path` for `me`. `alive` says whether a recorded holder still runs (see
/// [`Holder::alive`]); a dead holder's lock is taken over and returned as `stale`.
pub fn try_acquire(path: &Path, me: &Holder, alive: impl Fn(&Holder) -> bool) -> io::Result<Attempt> {
    if let Some(dir) = path.parent() {
        fs::create_dir_all(dir)?;
    }
    let tmp = temp_path(path, me.pid);
    fs::write(&tmp, me.to_text())?;
    let result = acquire_from(path, &tmp, me, alive);
    let _ = fs::remove_file(&tmp);
    result
}

fn acquire_from(path: &Path, tmp: &Path, me: &Holder, alive: impl Fn(&Holder) -> bool) -> io::Result<Attempt> {
    let mut stale = None;
    for _ in 0..8 {
        // A hard link fails when the lock exists: create-if-absent with the contents already in place.
        match fs::hard_link(tmp, path) {
            Ok(()) => return Ok(Attempt::Acquired { lock: BenchLock { path: path.to_owned(), holder: me.clone() }, stale }),
            Err(e) if e.kind() == io::ErrorKind::AlreadyExists => {}
            Err(e) => return Err(e),
        }
        let text = match fs::read_to_string(path) {
            Ok(t) => t,
            Err(e) if e.kind() == io::ErrorKind::NotFound => continue, // released meanwhile
            Err(e) => return Err(e),
        };
        // An unreadable file can't name a live holder: treat it as stale (it was never written by this code).
        let holder = Holder::parse(&text);
        if let Some(h) = holder.as_ref().filter(|h| alive(h)) {
            return Ok(Attempt::Held(h.clone()));
        }
        // Take over: move the dead holder's file aside, check it is the one we judged (another taker may have
        // replaced it meanwhile), then loop to create ours.
        let aside = temp_path(path, me.pid).with_extension("stale");
        match fs::rename(path, &aside) {
            Ok(()) => {
                let moved = fs::read_to_string(&aside).unwrap_or_default();
                if moved == text {
                    stale = holder.or(stale);
                    let _ = fs::remove_file(&aside);
                } else {
                    // Not the file we judged: put it back (unless someone created a new one meanwhile).
                    let _ = fs::hard_link(&aside, path);
                    let _ = fs::remove_file(&aside);
                }
            }
            Err(e) if e.kind() == io::ErrorKind::NotFound => {}
            Err(e) => return Err(e),
        }
    }
    Err(io::Error::other(format!("could not take {} (contended)", path.display())))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn holder(pid: u32, name: &str) -> Holder {
        Holder { pid, name: name.into(), since_unix_s: 1000.0, what: "perf suite --tag a\nb".into(), notes: vec![("power_restore".into(), "381b4222".into())] }
    }

    fn scratch(name: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!("sf_diag_lock_{}_{name}", std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap();
        dir.join("bench.lock")
    }

    #[test]
    fn holder_round_trips_and_checks_pid_reuse() {
        let h = holder(42, "bevyrrowind.exe");
        let back = Holder::parse(&h.to_text()).unwrap();
        assert_eq!(back.what, "perf suite --tag a b", "one line");
        assert_eq!(back.note("power_restore"), Some("381b4222"));
        assert_eq!(Holder::parse("garbage\n"), None);
        assert!(back.alive([(1, "x"), (42, "BEVYRROWIND.EXE")]));
        assert!(!back.alive([(42, "notepad.exe")]), "pid reused by another program");
        assert!(!back.alive([(43, "bevyrrowind.exe")]));
    }

    #[test]
    fn second_taker_waits_then_takes_over_a_dead_holder() {
        let path = scratch("takeover");
        let a = holder(1, "a.exe");
        let b = holder(2, "b.exe");
        let Attempt::Acquired { lock, stale: None } = try_acquire(&path, &a, |_| false).unwrap() else { panic!("first take") };
        // A is alive: B waits and learns who holds it.
        let Attempt::Held(h) = try_acquire(&path, &b, |h| h.pid == 1).unwrap() else { panic!("must be held") };
        assert_eq!(h.name, "a.exe");
        // A died without dropping (killed): B takes over and gets A's undo notes.
        std::mem::forget(lock);
        let Attempt::Acquired { lock, stale: Some(dead) } = try_acquire(&path, &b, |_| false).unwrap() else { panic!("takeover") };
        assert_eq!(dead.note("power_restore"), Some("381b4222"));
        assert_eq!(Holder::parse(&fs::read_to_string(&path).unwrap()).unwrap().pid, 2);
        drop(lock);
        assert!(!path.exists(), "drop releases");
        let leftovers: Vec<_> = fs::read_dir(path.parent().unwrap()).unwrap().collect();
        assert!(leftovers.is_empty(), "no temp files left");
    }

    #[test]
    fn drop_leaves_someone_elses_lock_alone_and_update_rewrites() {
        let path = scratch("update");
        let Attempt::Acquired { mut lock, .. } = try_acquire(&path, &holder(1, "a.exe"), |_| false).unwrap() else { panic!() };
        let mut h = lock.holder().clone();
        h.notes.push(("throttle_reset".into(), "C:\\x.exe".into()));
        lock.update(h).unwrap();
        assert_eq!(Holder::parse(&fs::read_to_string(&path).unwrap()).unwrap().note("throttle_reset"), Some("C:\\x.exe"));
        // Someone took it over (we were judged dead): our drop must not remove theirs.
        fs::write(&path, holder(9, "c.exe").to_text()).unwrap();
        drop(lock);
        assert!(path.exists());
    }
}
