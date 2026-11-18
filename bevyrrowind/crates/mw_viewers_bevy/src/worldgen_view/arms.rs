//! The arm list of `worldgen-view`: the guide folders given on the command line, one per number key.

use std::path::{Path, PathBuf};

/// Number keys 1-9.
pub const MAX_ARMS: usize = 9;

/// One guide folder to show.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Arm {
    /// The folder's name (on screen).
    pub name: String,
    pub dir: PathBuf,
}

/// The arms from `dirs`, in order. A folder with a `meta.json` is one arm; a folder without one stands for its
/// subfolders that have one, sorted by name (so `.../s2` gives `A0`..`A3`). Relative paths are made absolute
/// against the current folder. Errors: no arms, more than [`MAX_ARMS`], or a folder that holds no guide.
pub fn parse(dirs: &[PathBuf]) -> Result<Vec<Arm>, String> {
    let cwd = std::env::current_dir().map_err(|e| e.to_string())?;
    let mut arms = Vec::new();
    for dir in dirs {
        let dir = if dir.is_absolute() { dir.clone() } else { cwd.join(dir) };
        if is_guide(&dir) {
            arms.push(arm(dir));
            continue;
        }
        let mut subs: Vec<PathBuf> = std::fs::read_dir(&dir)
            .map_err(|e| format!("{}: {e}", dir.display()))?
            .filter_map(|e| e.ok().map(|e| e.path()))
            .filter(|p| is_guide(p))
            .collect();
        if subs.is_empty() {
            return Err(format!("{}: no meta.json in it or its subfolders (not a guide v1 folder)", dir.display()));
        }
        subs.sort();
        arms.extend(subs.into_iter().map(arm));
    }
    if arms.is_empty() {
        return Err("worldgen-view: give at least one guide folder".into());
    }
    if arms.len() > MAX_ARMS {
        return Err(format!("worldgen-view: {} arms, at most {MAX_ARMS} (keys 1-9)", arms.len()));
    }
    Ok(arms)
}

fn is_guide(dir: &Path) -> bool {
    dir.join("meta.json").is_file()
}

fn arm(dir: PathBuf) -> Arm {
    let name = dir.file_name().map_or_else(|| dir.display().to_string(), |n| n.to_string_lossy().into_owned());
    Arm { name, dir }
}
