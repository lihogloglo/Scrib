//! The Bevy-free half of `clip-preview --browse <dir>` (motion-generation.md MG10a): the clips
//! under a folder, grouped by activity (the first path component under it), with a cursor that
//! steps clips within an activity and activities, both wrapping around; and the `.prov.json`
//! sidecar's prompt and band for the overlay.

use std::path::{Path, PathBuf};

/// The activity name of clips directly in the browsed folder.
pub const TOP: &str = "(top)";

/// One clip file under the browsed folder.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BrowseClip {
    /// Relative to the browsed folder, `/`-separated.
    pub rel: String,
    pub path: PathBuf,
}

impl BrowseClip {
    /// `<clip>.prov.json` next to `<clip>.bvh`.
    pub fn sidecar(&self) -> PathBuf {
        self.path.with_extension("prov.json")
    }
}

/// The clips of one activity, in path order.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Activity {
    pub name: String,
    pub clips: Vec<BrowseClip>,
}

/// Every activity (sorted by name) and the cursor: activity, then clip in it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BrowseList {
    pub activities: Vec<Activity>,
    pub activity: usize,
    pub clip: usize,
}

/// Clip files the browser lists.
fn is_clip(path: &Path) -> bool {
    path.extension().is_some_and(|e| e.eq_ignore_ascii_case("bvh"))
}

/// Folders and files skipped while scanning (`_cache`, `.git`, ...).
fn hidden(name: &str) -> bool {
    name.starts_with('_') || name.starts_with('.')
}

impl BrowseList {
    /// Every `.bvh` under `dir` (recursively; `_`- and `.`-prefixed entries skipped).
    ///
    /// # Errors
    /// The folder can't be read, or holds no clip.
    pub fn scan(dir: &Path) -> Result<Self, String> {
        let mut found = Vec::new();
        let mut stack = vec![dir.to_path_buf()];
        while let Some(at) = stack.pop() {
            let entries = std::fs::read_dir(&at).map_err(|e| format!("{}: {e}", at.display()))?;
            for entry in entries.filter_map(Result::ok) {
                let path = entry.path();
                if hidden(&entry.file_name().to_string_lossy()) {
                    continue;
                }
                if path.is_dir() {
                    stack.push(path);
                } else if is_clip(&path) {
                    found.push(path);
                }
            }
        }
        Self::from_paths(dir, found).ok_or_else(|| format!("{}: no .bvh clips", dir.display()))
    }

    /// The list of `paths` (files under `root`) grouped by activity; `None` if empty.
    pub fn from_paths(root: &Path, paths: impl IntoIterator<Item = PathBuf>) -> Option<Self> {
        let mut clips: Vec<BrowseClip> = paths
            .into_iter()
            .map(|path| {
                let rel = path.strip_prefix(root).unwrap_or(&path).components().map(|c| c.as_os_str().to_string_lossy().into_owned()).collect::<Vec<_>>().join("/");
                BrowseClip { rel, path }
            })
            .collect();
        clips.sort_by(|a, b| a.rel.cmp(&b.rel));
        let mut activities: Vec<Activity> = Vec::new();
        for clip in clips {
            let name = clip.rel.split_once('/').map_or(TOP, |(first, _)| first).to_owned();
            match activities.iter_mut().find(|a| a.name == name) {
                Some(a) => a.clips.push(clip),
                None => activities.push(Activity { name, clips: vec![clip] }),
            }
        }
        activities.sort_by(|a, b| a.name.cmp(&b.name));
        (!activities.is_empty()).then_some(Self { activities, activity: 0, clip: 0 })
    }

    pub fn current(&self) -> &BrowseClip {
        &self.activities[self.activity].clips[self.clip]
    }

    pub fn current_activity(&self) -> &Activity {
        &self.activities[self.activity]
    }

    /// Total clips across activities.
    pub fn len(&self) -> usize {
        self.activities.iter().map(|a| a.clips.len()).sum()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Next (`step` 1) or previous (`step` -1) clip in the current activity, wrapping around.
    pub fn step_clip(&mut self, step: isize) {
        self.clip = wrap(self.clip, step, self.current_activity().clips.len());
    }

    /// Next or previous activity, wrapping around; starts on its first clip.
    pub fn step_activity(&mut self, step: isize) {
        self.activity = wrap(self.activity, step, self.activities.len());
        self.clip = 0;
    }
}

fn wrap(at: usize, step: isize, len: usize) -> usize {
    let len = isize::try_from(len.max(1)).unwrap_or(isize::MAX);
    let at = isize::try_from(at).unwrap_or(0);
    usize::try_from((at + step).rem_euclid(len)).unwrap_or(0)
}

/// What the overlay shows from a clip's `.prov.json` (motiongen's provenance sidecar).
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Prov {
    pub prompt: Option<String>,
    pub band: Option<String>,
}

impl Prov {
    /// The sidecar's `prompt` and `band` strings; `None` if it isn't a JSON object.
    pub fn parse(text: &str) -> Option<Self> {
        let value: serde_json::Value = serde_json::from_str(text).ok()?;
        let field = |k: &str| value.get(k).and_then(serde_json::Value::as_str).map(str::to_owned);
        value.is_object().then(|| Self { prompt: field("prompt"), band: field("band") })
    }

    /// The clip's sidecar, if present and readable.
    pub fn read(clip: &BrowseClip) -> Option<Self> {
        Self::parse(&std::fs::read_to_string(clip.sidecar()).ok()?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn list() -> BrowseList {
        let root = Path::new("gen");
        let paths = ["sit.chair/loop/b.bvh", "sit.chair/enter/a.bvh", "carry.sack/loop/c.bvh", "top.bvh", "sit.chair/exit/d.bvh"];
        BrowseList::from_paths(root, paths.iter().map(|p| root.join(p))).unwrap()
    }

    #[test]
    fn groups_by_first_component_sorted() {
        let l = list();
        let names: Vec<_> = l.activities.iter().map(|a| a.name.as_str()).collect();
        assert_eq!(names, [TOP, "carry.sack", "sit.chair"]);
        let sit: Vec<_> = l.activities[2].clips.iter().map(|c| c.rel.as_str()).collect();
        assert_eq!(sit, ["sit.chair/enter/a.bvh", "sit.chair/exit/d.bvh", "sit.chair/loop/b.bvh"]);
        assert_eq!(l.len(), 5);
        assert_eq!(l.current().rel, "top.bvh");
    }

    #[test]
    fn clips_and_activities_wrap() {
        let mut l = list();
        l.step_activity(-1);
        assert_eq!(l.current_activity().name, "sit.chair");
        l.step_clip(-1);
        assert_eq!(l.current().rel, "sit.chair/loop/b.bvh");
        l.step_clip(1);
        assert_eq!(l.current().rel, "sit.chair/enter/a.bvh");
        l.step_clip(1);
        l.step_activity(1);
        assert_eq!((l.current_activity().name.as_str(), l.clip), (TOP, 0));
        l.step_clip(1);
        assert_eq!(l.current().rel, "top.bvh");
    }

    #[test]
    fn empty_is_none_and_sidecar_sits_next_to_the_clip() {
        assert!(BrowseList::from_paths(Path::new("x"), Vec::new()).is_none());
        let c = BrowseClip { rel: "a/b.ground0.00.bvh".into(), path: PathBuf::from("g/a/b.ground0.00.bvh") };
        assert_eq!(c.sidecar(), PathBuf::from("g/a/b.ground0.00.prov.json"));
    }

    #[test]
    fn prov_reads_prompt_and_band() {
        let p = Prov::parse(r#"{"prompt": "A person sits.", "band": "seat45", "fps": 30.0}"#).unwrap();
        assert_eq!(p, Prov { prompt: Some("A person sits.".into()), band: Some("seat45".into()) });
        assert_eq!(Prov::parse("{}").unwrap(), Prov::default());
        assert!(Prov::parse("not json").is_none());
    }

    #[test]
    fn scans_skipping_hidden_folders() {
        let root = std::env::temp_dir().join(format!("mg10a-browse-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&root);
        for f in ["walk/loop/w.bvh", "walk/loop/w.prov.json", "_cache/x.bvh", "idle.bvh"] {
            let p = root.join(f);
            std::fs::create_dir_all(p.parent().unwrap()).unwrap();
            std::fs::write(&p, "{}").unwrap();
        }
        let l = BrowseList::scan(&root).unwrap();
        let rels: Vec<_> = l.activities.iter().flat_map(|a| a.clips.iter().map(|c| c.rel.as_str())).collect();
        assert_eq!(rels, ["idle.bvh", "walk/loop/w.bvh"]);
        let _ = std::fs::remove_dir_all(&root);
    }
}
