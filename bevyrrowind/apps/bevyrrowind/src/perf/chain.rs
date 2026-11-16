//! A bench chain's hold on the machine (perf-verdict audit 2026-11-16 §5 fixes 2 and 6): the cross-session
//! bench lock (`<repo>/.sessions/bench.lock`, next to `cargo_turn.sh`'s `build-queue/`) held for every arm and
//! repeat of one `perf suite` / `perf sweep`, and the power plan for the chain ([`PowerGuard`]). A bench killed
//! mid-chain leaves its lock behind; the next bench finds its pid gone, takes over, and first restores the
//! power plan the dead one had switched away from.

use std::path::{Path, PathBuf};
use std::time::Duration;

use super::power::{self, PowerGuard};
use sf_diag_bevy::sf_diag::lock::{self, Attempt, BenchLock, Holder};
use sf_diag_bevy::sf_diag::suite;

/// The repo root (the folder holding `CONTRIBUTING.md`, searched upward from the working directory).
pub fn repo_root() -> Option<PathBuf> {
    let cwd = std::env::current_dir().ok()?;
    cwd.ancestors().find(|d| d.join("CONTRIBUTING.md").is_file() && d.join("Cargo.toml").is_file()).map(Path::to_path_buf)
}

/// Where the lock lives.
pub fn lock_path() -> PathBuf {
    repo_root().unwrap_or_else(|| PathBuf::from(".")).join(".sessions").join("bench.lock")
}

/// The machine's state for one chain. Fields drop in order: the power plan is restored, then the lock released.
pub struct Chain {
    power: Option<PowerGuard>,
    lock: Option<BenchLock>,
    /// Summary lines (lock, power plan, throttling).
    pub notes: Vec<String>,
}

impl Chain {
    /// A dry run: no lock, nothing changed.
    pub fn dry() -> Self {
        println!("dry run: no bench lock, power plan untouched");
        Self { power: None, lock: None, notes: Vec::new() }
    }

    /// Takes the bench lock (waiting while another live bench holds it; `no_lock` skips it), then sets the
    /// power plan for the chain. `what` is shown to whoever waits.
    pub fn begin(what: &str, exe: &Path, no_lock: bool) -> Result<Self, Box<dyn std::error::Error>> {
        let mut notes = Vec::new();
        let lock = if no_lock {
            println!("bench lock: skipped (--no-lock)");
            notes.push("bench lock: **not held** (`--no-lock`): another bench may have overlapped".to_owned());
            None
        } else {
            let (lock, waited) = acquire(what)?;
            notes.push(format!(
                "bench lock: held for the whole chain (`{}`){}",
                lock.path().display(),
                if waited.is_zero() { String::new() } else { format!(", after waiting {:.0} min for another bench", waited.as_secs_f64() / 60.0) }
            ));
            Some(lock)
        };
        let power = PowerGuard::begin(exe);
        notes.extend(power.notes.iter().cloned());
        let mut chain = Self { power: Some(power), lock, notes };
        chain.record_undo();
        Ok(chain)
    }

    /// Writes what a taker must undo if this process dies (power plan, throttling exemption) into the lock.
    fn record_undo(&mut self) {
        let (Some(lock), Some(power)) = (self.lock.as_mut(), self.power.as_ref()) else { return };
        let mut h = lock.holder().clone();
        h.notes.extend(power.restore.iter().map(|g| ("power_restore".to_owned(), g.clone())));
        h.notes.extend(power.set.iter().map(|g| ("power_set".to_owned(), g.clone())));
        h.notes.extend(power.unthrottled.iter().map(|p| ("throttle_reset".to_owned(), p.display().to_string())));
        if let Err(e) = lock.update(h) {
            eprintln!("warning: could not record the power undo in the bench lock: {e}");
        }
    }
}

/// This process as a lock holder.
fn me(what: &str) -> Holder {
    let name = std::env::current_exe().ok().and_then(|e| e.file_name().map(|n| n.to_string_lossy().into_owned())).unwrap_or_default();
    Holder { pid: std::process::id(), name, since_unix_s: suite::unix_now(), what: what.to_owned(), notes: Vec::new() }
}

/// Takes the lock, polling every 5 s while a live bench holds it; returns how long it waited.
fn acquire(what: &str) -> Result<(BenchLock, Duration), Box<dyn std::error::Error>> {
    let path = lock_path();
    let me = me(what);
    let started = std::time::Instant::now();
    let mut said = String::new();
    loop {
        let running = suite::processes()?;
        let alive = |h: &Holder| h.alive(running.iter().map(|p| (p.pid, p.name.as_str())));
        match lock::try_acquire(&path, &me, alive)? {
            Attempt::Acquired { lock, stale } => {
                if let Some(dead) = stale {
                    println!("bench lock: took over from pid {} ({}), which is gone", dead.pid, dead.name);
                    undo_dead(&dead);
                }
                return Ok((lock, started.elapsed()));
            }
            Attempt::Held(h) => {
                let msg = format!("pid {} ({}) since {}: {}", h.pid, h.name, super::ab::utc(h.since_unix_s), h.what);
                if msg != said {
                    println!("bench lock: waiting for {msg}");
                    said = msg;
                }
                std::thread::sleep(Duration::from_secs(5));
            }
        }
    }
}

/// Undoes what a dead holder had changed: its power plan (only while its plan is still the active one, so
/// a change the owner made since is kept) and its throttling exemption.
fn undo_dead(dead: &Holder) {
    if let (Some(restore), Some(set)) = (dead.note("power_restore"), dead.note("power_set")) {
        let active = power::schemes().into_iter().find(|s| s.active).map(|s| s.guid);
        if active.as_deref() == Some(set) {
            println!("bench lock: restoring the power plan it had switched away from ({restore})");
            power::set_active(restore);
        }
    }
    if let Some(exe) = dead.note("throttle_reset") {
        power::reset_throttling(Path::new(exe));
    }
}
