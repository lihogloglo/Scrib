//! The Bevy-free half of `clip-preview --browse <dir>` (motion-generation.md MG10a): the clips
//! under a folder, grouped by activity (the first path component under it), with a cursor that
//! steps clips within an activity and activities, both wrapping around; and the `.prov.json`
//! sidecar's prompt and band for the overlay; and the owner's verdicts (`verdicts.toml` in the
//! browsed folder, motion-generation.md "review"): Y/N/M rate the cursor's clip and advance.
//! The grader's `grades.json` (scrib-motiongen `motiongen grade`) orders each activity (ungated
//! clips first, gated ones, then failing prompt groups last), feeds the overlay, and drives the
//! "candidates only" filter; a view filter (all, unrated, maybe, keep) serves the second pass.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

/// The activity name of clips directly in the browsed folder.
pub const TOP: &str = "(top)";

/// One clip file under the browsed folder.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BrowseClip {
    /// Relative to the browsed folder, `/`-separated.
    pub rel: String,
    pub path: PathBuf,
    /// The `verdicts.toml` key: the file stem (motiongen's `.prov.json` `id`,
    /// `<activity>.<phase>.<band>.<nn>`), or `rel` without `.bvh` when two clips share a stem.
    pub id: String,
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

/// Every activity (sorted by name; with grades, see [`BrowseList::set_grades`]) and the cursor:
/// activity, then clip in it.
#[derive(Clone, Debug, PartialEq)]
pub struct BrowseList {
    /// The browsed folder (where `verdicts.toml` and `grades.json` live).
    pub root: PathBuf,
    pub activities: Vec<Activity>,
    pub activity: usize,
    pub clip: usize,
    /// The folder's `grades.json` (empty without one).
    pub grades: Grades,
    /// Hide gated clips from navigation.
    pub candidates_only: bool,
    /// Which verdicts navigation shows (needs the verdicts; without them every verdict is shown).
    pub view: View,
}

/// The verdict filter navigation follows (cycled all -> unrated -> maybe -> keep).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum View {
    #[default]
    All,
    Unrated,
    Maybe,
    Keep,
}

impl View {
    #[must_use]
    pub fn next(self) -> Self {
        match self {
            Self::All => Self::Unrated,
            Self::Unrated => Self::Maybe,
            Self::Maybe => Self::Keep,
            Self::Keep => Self::All,
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::All => "all",
            Self::Unrated => "unrated",
            Self::Maybe => "maybe",
            Self::Keep => "keep",
        }
    }

    fn admits(self, rating: Option<Rating>) -> bool {
        match self {
            Self::All => true,
            Self::Unrated => rating.is_none(),
            Self::Maybe => rating == Some(Rating::Maybe),
            Self::Keep => rating == Some(Rating::Keep),
        }
    }
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
                let id = stem(&rel).to_owned();
                BrowseClip { rel, path, id }
            })
            .collect();
        clips.sort_by(|a, b| a.rel.cmp(&b.rel));
        let mut seen: BTreeMap<String, usize> = BTreeMap::new();
        for clip in &clips {
            *seen.entry(clip.id.clone()).or_default() += 1;
        }
        for clip in &mut clips {
            if seen[&clip.id] > 1 {
                clip.id = strip_bvh(&clip.rel).to_owned();
            }
        }
        let mut activities: Vec<Activity> = Vec::new();
        for clip in clips {
            let name = clip.rel.split_once('/').map_or(TOP, |(first, _)| first).to_owned();
            match activities.iter_mut().find(|a| a.name == name) {
                Some(a) => a.clips.push(clip),
                None => activities.push(Activity { name, clips: vec![clip] }),
            }
        }
        activities.sort_by(|a, b| a.name.cmp(&b.name));
        (!activities.is_empty()).then_some(Self {
            root: root.to_path_buf(),
            activities,
            activity: 0,
            clip: 0,
            grades: Grades::default(),
            candidates_only: false,
            view: View::All,
        })
    }

    /// Take `grades` and reorder: inside each activity ungated clips first, gated ones after, clips
    /// of failing prompt groups last (path order otherwise); activities whose every clip is in a
    /// failing group go after the others. The cursor goes back to the first clip. Empty grades
    /// change nothing.
    pub fn set_grades(&mut self, grades: Grades) {
        self.grades = grades;
        let g = &self.grades;
        let failing = |c: &BrowseClip| g.group(c).is_some_and(|(_, gr)| gr.failing);
        for act in &mut self.activities {
            act.clips.sort_by_key(|c| (failing(c), g.grade(c).is_some_and(Grade::gated)));
        }
        self.activities.sort_by_key(|a| a.clips.iter().all(failing));
        (self.activity, self.clip) = (0, 0);
    }

    /// Whether navigation stops on `clip`: not gated while candidates-only is on, and its verdict
    /// in the view (the view applies only with `verdicts`).
    pub fn shown(&self, clip: &BrowseClip, verdicts: Option<&Verdicts>) -> bool {
        if self.candidates_only && self.grades.grade(clip).is_some_and(Grade::gated) {
            return false;
        }
        verdicts.is_none_or(|v| self.view.admits(v.rating(&clip.id)))
    }

    /// Clips navigation stops on.
    pub fn shown_count(&self, verdicts: Option<&Verdicts>) -> usize {
        self.cursors().filter(|(_, c)| self.shown(c, verdicts)).count()
    }

    /// After a filter change: if the cursor's clip is hidden, move to the next shown one (it stays
    /// if none is).
    pub fn settle(&mut self, verdicts: Option<&Verdicts>) {
        if !self.shown(self.current(), verdicts) {
            self.advance_in(verdicts);
        }
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
        self.step_clip_in(step, None);
    }

    /// [`Self::step_clip`] over the shown clips ([`Self::shown`]); unchanged if none is.
    pub fn step_clip_in(&mut self, step: isize, verdicts: Option<&Verdicts>) {
        let clips = &self.activities[self.activity].clips;
        let mut at = self.clip;
        for _ in 0..clips.len() {
            at = wrap(at, step, clips.len());
            if self.shown(&clips[at], verdicts) {
                self.clip = at;
                return;
            }
        }
    }

    /// Next or previous activity, wrapping around; starts on its first clip.
    pub fn step_activity(&mut self, step: isize) {
        self.step_activity_in(step, None);
    }

    /// [`Self::step_activity`] over activities with a shown clip, starting on the first shown one;
    /// unchanged if none has one.
    pub fn step_activity_in(&mut self, step: isize, verdicts: Option<&Verdicts>) {
        let mut at = self.activity;
        for _ in 0..self.activities.len() {
            at = wrap(at, step, self.activities.len());
            if let Some(clip) = self.activities[at].clips.iter().position(|c| self.shown(c, verdicts)) {
                (self.activity, self.clip) = (at, clip);
                return;
            }
        }
    }

    /// The next clip in browse order: the next in this activity, else the next activity's first
    /// (wrapping from the last activity to the first).
    pub fn advance(&mut self) {
        self.advance_in(None);
    }

    /// [`Self::advance`] to the next shown clip; unchanged if no other is shown.
    pub fn advance_in(&mut self, verdicts: Option<&Verdicts>) {
        if let Some(at) = self.find_after(|c| self.shown(c, verdicts)) {
            (self.activity, self.clip) = at;
        }
    }

    /// The first clip after the cursor in browse order (wrapping, the cursor's own excluded)
    /// matching `pred`.
    fn find_after(&self, pred: impl Fn(&BrowseClip) -> bool) -> Option<(usize, usize)> {
        let all: Vec<_> = self.cursors().map(|(at, clip)| (at, pred(clip))).collect();
        let here = all.iter().position(|(at, _)| *at == (self.activity, self.clip)).unwrap_or(0);
        (1..all.len()).map(|k| all[(here + k) % all.len()]).find(|(_, hit)| *hit).map(|(at, _)| at)
    }

    /// Every clip with its cursor (activity, clip), in browse order.
    fn cursors(&self) -> impl Iterator<Item = ((usize, usize), &BrowseClip)> {
        self.activities.iter().enumerate().flat_map(|(a, act)| act.clips.iter().enumerate().map(move |(c, clip)| ((a, c), clip)))
    }

    /// Jump to the first unrated shown clip after the cursor (wrapping); `false` (cursor
    /// unchanged) if every other shown clip has a verdict.
    pub fn next_unrated(&mut self, verdicts: &Verdicts) -> bool {
        let found = self.find_after(|c| verdicts.get(&c.id).is_none() && self.shown(c, Some(verdicts)));
        if let Some(at) = found {
            (self.activity, self.clip) = at;
        }
        found.is_some()
    }

    /// Record `verdict` for the cursor's clip, advance to the next clip, then save.
    ///
    /// # Errors
    /// `verdicts.toml` couldn't be written (the verdict stays recorded in memory).
    pub fn rate(&mut self, verdicts: &mut Verdicts, verdict: Verdict) -> Result<(), String> {
        verdicts.set(&self.current().id, verdict);
        self.advance_in(Some(verdicts));
        verdicts.save()
    }

    /// Clear the cursor's clip's verdict and save; the cursor stays. `Ok(false)` if it had none.
    ///
    /// # Errors
    /// `verdicts.toml` couldn't be written.
    pub fn clear_rating(&self, verdicts: &mut Verdicts) -> Result<bool, String> {
        if verdicts.clear(&self.current().id) {
            verdicts.save().map(|()| true)
        } else {
            Ok(false)
        }
    }

    /// The overlay's grade line for `clip`: gates, metrics, mirror flag, prompt group's
    /// gated/total; `None` without a `grades.json`.
    pub fn grade_line(&self, clip: &BrowseClip) -> Option<String> {
        if self.grades.is_empty() {
            return None;
        }
        let Some(grade) = self.grades.grade(clip) else { return Some("grade: (not in grades.json)".to_owned()) };
        let gates = if grade.gated() { format!("gates: {}", grade.gates.join(", ")) } else { "ungated".to_owned() };
        let metrics = metrics_text(&grade.metrics);
        let mirror = if grade.mirror_candidate { "   mirror candidate" } else { "" };
        let group = match self.grades.group(clip) {
            Some((_, g)) => format!("group {}/{} gated{}", g.gated, g.total, if g.failing { " FAILING" } else { "" }),
            None => "group -".to_owned(),
        };
        Some(format!("{gates}   {metrics}{mirror}   {group}   type {}", grade.kind.as_deref().unwrap_or("-")))
    }
}

/// The grader's metrics, in a fixed order with units; absent ones left out.
fn metrics_text(metrics: &BTreeMap<String, f64>) -> String {
    const SHOWN: [(&str, &str, &str); 7] = [
        ("skate_cm_s", "skate ", " cm/s"),
        ("skate_worst_cm", "worst ", " cm"),
        ("jerk_p99", "jerk p99 ", ""),
        ("jerk_max", "max ", ""),
        ("flight_s", "flight ", " s"),
        ("steps", "steps ", ""),
        ("turn_deg", "turn ", " deg"),
    ];
    SHOWN.iter().filter_map(|(k, label, unit)| metrics.get(*k).map(|v| format!("{label}{v}{unit}"))).collect::<Vec<_>>().join("  ")
}

/// The file the grader writes in the browsed folder.
pub const GRADES_FILE: &str = "grades.json";

/// One clip's entry in `grades.json`.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Grade {
    /// Failed gates (`skate`, `jerk`, `turn_mag`, ...); empty = a candidate.
    pub gates: Vec<String>,
    pub metrics: BTreeMap<String, f64>,
    pub mirror_candidate: bool,
    /// `cycle`, `start`, `stop`, `turn`, `pivot`, `other`.
    pub kind: Option<String>,
}

impl Grade {
    pub fn gated(&self) -> bool {
        !self.gates.is_empty()
    }
}

/// One prompt group's `_groups` entry.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct GroupGrade {
    pub failing: bool,
    pub gated: usize,
    pub total: usize,
}

/// `grades.json`: clip stem -> grade, and prompt group (the stem without its take number) -> triage.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Grades {
    clips: BTreeMap<String, Grade>,
    groups: BTreeMap<String, GroupGrade>,
}

impl Grades {
    /// The grader's JSON; `None` if it isn't a JSON object. Entries that aren't objects are skipped.
    pub fn parse(text: &str) -> Option<Self> {
        let value: serde_json::Value = serde_json::from_str(text).ok()?;
        let mut out = Self::default();
        for (key, entry) in value.as_object()? {
            if key == "_groups" {
                for (name, g) in entry.as_object().into_iter().flatten() {
                    let count = |k: &str| g.get(k).and_then(serde_json::Value::as_u64).and_then(|n| usize::try_from(n).ok()).unwrap_or(0);
                    let failing = g.get("failing").and_then(serde_json::Value::as_bool).unwrap_or(false);
                    out.groups.insert(name.clone(), GroupGrade { failing, gated: count("gated"), total: count("total") });
                }
            } else if !key.starts_with('_') && entry.is_object() {
                let gates = entry.get("gates").and_then(serde_json::Value::as_array).into_iter().flatten().filter_map(serde_json::Value::as_str).map(str::to_owned).collect();
                let metrics = entry
                    .get("metrics")
                    .and_then(serde_json::Value::as_object)
                    .into_iter()
                    .flatten()
                    .filter_map(|(k, v)| v.as_f64().map(|v| (k.clone(), v)))
                    .collect();
                let mirror_candidate = entry.get("mirror_candidate").and_then(serde_json::Value::as_bool).unwrap_or(false);
                let kind = entry.get("type").and_then(serde_json::Value::as_str).map(str::to_owned);
                out.clips.insert(key.clone(), Grade { gates, metrics, mirror_candidate, kind });
            }
        }
        Some(out)
    }

    /// `<dir>/grades.json`; empty if it is absent, unreadable or not JSON (browsing works as without one).
    pub fn load(dir: &Path) -> Self {
        std::fs::read_to_string(dir.join(GRADES_FILE)).ok().and_then(|t| Self::parse(&t)).unwrap_or_default()
    }

    pub fn is_empty(&self) -> bool {
        self.clips.is_empty() && self.groups.is_empty()
    }

    /// Graded clips, and how many of them are gated.
    pub fn counts(&self) -> (usize, usize) {
        (self.clips.len(), self.clips.values().filter(|g| g.gated()).count())
    }

    /// `clip`'s grade, joined by its file stem (the grader's key, also when two clips share a stem).
    pub fn grade(&self, clip: &BrowseClip) -> Option<&Grade> {
        self.clips.get(stem(&clip.rel))
    }

    /// `clip`'s prompt group name and triage.
    pub fn group(&self, clip: &BrowseClip) -> Option<(&str, GroupGrade)> {
        self.groups.get_key_value(group_of(stem(&clip.rel))).map(|(k, g)| (k.as_str(), *g))
    }
}

/// The prompt group of a clip stem: the stem without a trailing `.<digits>` take number (the grader's rule).
fn group_of(stem: &str) -> &str {
    match stem.rsplit_once('.') {
        Some((head, take)) if !take.is_empty() && take.bytes().all(|b| b.is_ascii_digit()) => head,
        _ => stem,
    }
}

/// `rel` without a trailing `.bvh` (any case).
fn strip_bvh(rel: &str) -> &str {
    match rel.len().checked_sub(4) {
        Some(at) if rel.is_char_boundary(at) && rel[at..].eq_ignore_ascii_case(".bvh") => &rel[..at],
        _ => rel,
    }
}

/// The file name of `rel` without `.bvh`.
fn stem(rel: &str) -> &str {
    strip_bvh(rel.rsplit('/').next().unwrap_or(rel))
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

/// A keyboard verdict: `keep`, `maybe` or `reject` in `verdicts.toml`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Verdict {
    Keep,
    Maybe,
    Reject,
}

impl Verdict {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Keep => "keep",
            Self::Maybe => "maybe",
            Self::Reject => "reject",
        }
    }
}

/// How the overlay reads one `verdicts.toml` value.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rating {
    Keep,
    Maybe,
    Reject,
    /// `"note: ..."` (written by hand) or any other value.
    Note,
}

/// Verdict counts over some clips.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Tally {
    pub keep: usize,
    pub maybe: usize,
    pub reject: usize,
    pub note: usize,
    pub unrated: usize,
}

/// The file the browser keeps verdicts in, in the browsed folder.
pub const VERDICTS_FILE: &str = "verdicts.toml";

const VERDICTS_HEADER: &str = "# Clip verdicts (motion-generation.md): id = \"keep\" | \"maybe\" | \"reject\" | \"note: ...\".\n\
# Written by `clip-preview --browse` (Y keep, N reject, M maybe, Backspace clear); keys sorted, comments not kept.\n";

/// `verdicts.toml`: clip id -> value. Every entry already in the file is kept on save (sorted;
/// values the browser doesn't write, tables included, pass through unchanged).
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Verdicts {
    pub path: PathBuf,
    entries: BTreeMap<String, toml::Value>,
}

impl Verdicts {
    /// `<dir>/verdicts.toml`, empty if it doesn't exist yet.
    ///
    /// # Errors
    /// The file exists but can't be read or isn't TOML (so a save never overwrites it).
    pub fn load(dir: &Path) -> Result<Self, String> {
        let path = dir.join(VERDICTS_FILE);
        let entries = match std::fs::read_to_string(&path) {
            Ok(text) => toml::from_str::<toml::Table>(&text).map_err(|e| format!("{}: {e}", path.display()))?.into_iter().collect(),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => BTreeMap::new(),
            Err(e) => return Err(format!("{}: {e}", path.display())),
        };
        Ok(Self { path, entries })
    }

    /// The file's text: a header comment, then the entries in key order.
    ///
    /// # Errors
    /// An entry can't be written as TOML.
    pub fn to_toml(&self) -> Result<String, String> {
        Ok(format!("{VERDICTS_HEADER}{}", toml::to_string(&self.entries).map_err(|e| e.to_string())?))
    }

    /// Write the file atomically (`verdicts.toml.tmp`, then rename over it).
    ///
    /// # Errors
    /// The write or the rename failed.
    pub fn save(&self) -> Result<(), String> {
        let text = self.to_toml()?;
        let tmp = self.path.with_extension("toml.tmp");
        std::fs::write(&tmp, text).map_err(|e| format!("{}: {e}", tmp.display()))?;
        std::fs::rename(&tmp, &self.path).map_err(|e| format!("{}: {e}", self.path.display()))
    }

    /// The value for `id` (`"(not a string)"` for a non-string value).
    pub fn get(&self, id: &str) -> Option<&str> {
        self.entries.get(id).map(|v| v.as_str().unwrap_or("(not a string)"))
    }

    pub fn rating(&self, id: &str) -> Option<Rating> {
        self.get(id).map(|v| match v {
            "keep" => Rating::Keep,
            "maybe" => Rating::Maybe,
            "reject" => Rating::Reject,
            _ => Rating::Note,
        })
    }

    pub fn set(&mut self, id: &str, verdict: Verdict) {
        self.entries.insert(id.to_owned(), toml::Value::String(verdict.as_str().to_owned()));
    }

    /// Remove `id`'s entry; `false` if it had none.
    pub fn clear(&mut self, id: &str) -> bool {
        self.entries.remove(id).is_some()
    }

    pub fn tally<'a>(&self, clips: impl IntoIterator<Item = &'a BrowseClip>) -> Tally {
        let mut t = Tally::default();
        for clip in clips {
            *match self.rating(&clip.id) {
                Some(Rating::Keep) => &mut t.keep,
                Some(Rating::Maybe) => &mut t.maybe,
                Some(Rating::Reject) => &mut t.reject,
                Some(Rating::Note) => &mut t.note,
                None => &mut t.unrated,
            } += 1;
        }
        t
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
        let c = BrowseClip { rel: "a/b.ground0.00.bvh".into(), path: PathBuf::from("g/a/b.ground0.00.bvh"), id: "b.ground0.00".into() };
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

    /// A fresh temp folder for one verdict test.
    fn temp(name: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!("browse-verdicts-{name}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        dir
    }

    #[test]
    fn ids_are_stems_unless_two_clips_share_one() {
        let root = Path::new("gen");
        let paths = ["walk/act/walk.act.g.00.bvh", "a/x.bvh", "b/x.BVH"];
        let l = BrowseList::from_paths(root, paths.iter().map(|p| root.join(p))).unwrap();
        let ids: Vec<_> = l.cursors().map(|(_, c)| c.id.as_str()).collect();
        assert_eq!(ids, ["a/x", "b/x", "walk.act.g.00"]);
    }

    #[test]
    fn rating_records_and_advances_wrapping_across_activities() {
        let dir = temp("rate");
        let mut l = list();
        let mut v = Verdicts::load(&dir).unwrap();
        // (top) has one clip: rating it moves to carry.sack's first.
        l.rate(&mut v, Verdict::Keep).unwrap();
        assert_eq!(v.rating("top"), Some(Rating::Keep));
        assert_eq!(l.current().rel, "carry.sack/loop/c.bvh");
        l.rate(&mut v, Verdict::Reject).unwrap();
        assert_eq!(l.current().rel, "sit.chair/enter/a.bvh");
        l.rate(&mut v, Verdict::Maybe).unwrap();
        assert_eq!(l.current().rel, "sit.chair/exit/d.bvh");
        l.step_clip(-1);
        // Re-rating overwrites and still advances.
        l.rate(&mut v, Verdict::Keep).unwrap();
        assert_eq!((v.rating("a"), l.current().rel.as_str()), (Some(Rating::Keep), "sit.chair/exit/d.bvh"));
        l.advance();
        l.rate(&mut v, Verdict::Reject).unwrap();
        // The last clip of the last activity wraps to the first.
        assert_eq!(l.current().rel, "top.bvh");
        let sit = v.tally(&l.activities[2].clips);
        assert_eq!(sit, Tally { keep: 1, maybe: 0, reject: 1, note: 0, unrated: 1 });
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn clear_removes_the_verdict_and_keeps_the_cursor() {
        let dir = temp("clear");
        let mut l = list();
        let mut v = Verdicts::load(&dir).unwrap();
        l.rate(&mut v, Verdict::Maybe).unwrap();
        l.step_activity(-1);
        assert_eq!(l.current().id, "top");
        assert!(l.clear_rating(&mut v).unwrap());
        assert!(!l.clear_rating(&mut v).unwrap());
        assert_eq!((v.rating("top"), l.current().id.as_str()), (None, "top"));
        assert_eq!(Verdicts::load(&dir).unwrap().get("top"), None);
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn verdicts_round_trip_through_the_file_sorted() {
        let dir = temp("roundtrip");
        let mut l = list();
        let mut v = Verdicts::load(&dir).unwrap();
        for verdict in [Verdict::Reject, Verdict::Keep, Verdict::Maybe] {
            l.rate(&mut v, verdict).unwrap();
        }
        let back = Verdicts::load(&dir).unwrap();
        assert_eq!(back, v);
        let text = std::fs::read_to_string(dir.join(VERDICTS_FILE)).unwrap();
        let lines: Vec<_> = text.lines().filter(|l| !l.starts_with('#')).collect();
        assert_eq!(lines, [r#"a = "maybe""#, r#"c = "keep""#, r#"top = "reject""#]);
        assert!(!dir.join("verdicts.toml.tmp").exists());
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn existing_entries_survive_a_save() {
        let dir = temp("preserve");
        let before = "# by hand\n\"loco.walk.cycle.1.act.g.00\" = \"note: left foot skates\"\nzz = \"keep\"\ncount = 3\n\n[meta]\nby = \"owner\"\n";
        std::fs::write(dir.join(VERDICTS_FILE), before).unwrap();
        let mut l = list();
        let mut v = Verdicts::load(&dir).unwrap();
        assert_eq!(v.rating("loco.walk.cycle.1.act.g.00"), Some(Rating::Note));
        l.rate(&mut v, Verdict::Keep).unwrap();
        let back = Verdicts::load(&dir).unwrap();
        assert_eq!(back.get("loco.walk.cycle.1.act.g.00"), Some("note: left foot skates"));
        assert_eq!((back.get("zz"), back.get("top")), (Some("keep"), Some("keep")));
        assert_eq!(back.entries["count"], toml::Value::Integer(3));
        assert_eq!(back.entries["meta"].get("by").and_then(toml::Value::as_str), Some("owner"));
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn a_broken_file_is_not_loaded_so_never_overwritten() {
        let dir = temp("broken");
        std::fs::write(dir.join(VERDICTS_FILE), "top = keep without quotes").unwrap();
        assert!(Verdicts::load(&dir).is_err());
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn next_unrated_skips_rated_clips_and_wraps() {
        let dir = temp("unrated");
        let mut l = list();
        let mut v = Verdicts::load(&dir).unwrap();
        v.set("c", Verdict::Keep);
        v.set("a", Verdict::Reject);
        assert!(l.next_unrated(&v));
        assert_eq!(l.current().id, "d");
        assert!(l.next_unrated(&v));
        assert_eq!(l.current().id, "b");
        assert!(l.next_unrated(&v));
        assert_eq!(l.current().id, "top");
        for id in ["top", "b", "d"] {
            v.set(id, Verdict::Maybe);
        }
        assert!(!l.next_unrated(&v));
        assert_eq!(l.current().id, "top");
        let _ = std::fs::remove_dir_all(&dir);
    }

    /// Two prompt groups in `walk.f` (g0 fine, g1 failing), one failing group filling `run.f`, an ungraded top clip.
    const GRADES: &str = r#"{
        "walk.f.act.g0.00": {"gates": ["skate"], "metrics": {"skate_cm_s": 62.0, "skate_worst_cm": 84.5, "jerk_p99": 1225.0, "steps": 12}, "mirror_candidate": false, "source": "py", "type": "cycle"},
        "walk.f.act.g0.01": {"gates": [], "metrics": {"skate_cm_s": 21.1, "turn_deg": -4.6}, "mirror_candidate": true, "source": "py", "type": "cycle"},
        "walk.f.act.g0.02": {"gates": [], "metrics": {}, "mirror_candidate": false, "source": "py", "type": "cycle"},
        "walk.f.act.g1.00": {"gates": [], "metrics": {}, "mirror_candidate": false, "source": "py"},
        "run.f.act.g0.00": {"gates": ["jerk", "skate"], "metrics": {}, "mirror_candidate": false, "source": "py", "type": "start"},
        "run.f.act.g0.01": {"gates": ["turn_mag"], "metrics": {}, "mirror_candidate": false, "source": "py", "type": "start"},
        "_groups": {
            "walk.f.act.g0": {"failing": false, "gated": 1, "total": 3},
            "walk.f.act.g1": {"failing": true, "gated": 4, "total": 6},
            "run.f.act.g0": {"failing": true, "gated": 2, "total": 2}
        }
    }"#;

    fn graded() -> BrowseList {
        let root = Path::new("gen");
        let paths = [
            "idle.bvh",
            "walk.f/act/walk.f.act.g0.00.bvh",
            "walk.f/act/walk.f.act.g0.01.bvh",
            "walk.f/act/walk.f.act.g0.02.bvh",
            "walk.f/act/walk.f.act.g1.00.bvh",
            "run.f/act/run.f.act.g0.00.bvh",
            "run.f/act/run.f.act.g0.01.bvh",
        ];
        let mut l = BrowseList::from_paths(root, paths.iter().map(|p| root.join(p))).unwrap();
        l.set_grades(Grades::parse(GRADES).unwrap());
        l
    }

    fn order(l: &BrowseList) -> Vec<&str> {
        l.cursors().map(|(_, c)| c.id.as_str()).collect()
    }

    fn find(l: &BrowseList, id: &str) -> BrowseClip {
        l.cursors().find(|(_, c)| c.id == id).unwrap().1.clone()
    }

    #[test]
    fn grades_parse_and_join_by_stem() {
        let l = graded();
        let g = l.grades.grade(&find(&l, "walk.f.act.g0.00")).unwrap();
        assert_eq!(g.gates, ["skate"]);
        assert!(g.gated() && !g.mirror_candidate);
        assert_eq!((g.metrics["skate_cm_s"], g.metrics["steps"], g.kind.as_deref()), (62.0, 12.0, Some("cycle")));
        assert!(l.grades.grade(&find(&l, "walk.f.act.g0.01")).unwrap().mirror_candidate);
        assert_eq!(l.grades.grade(&find(&l, "walk.f.act.g1.00")).unwrap().kind, None);
        assert_eq!(l.grades.grade(&find(&l, "idle")), None);
        assert_eq!(l.grades.group(&find(&l, "run.f.act.g0.01")), Some(("run.f.act.g0", GroupGrade { failing: true, gated: 2, total: 2 })));
        assert_eq!(l.grades.group(&find(&l, "idle")), None);
        assert_eq!(l.grades.counts(), (6, 3));
        assert_eq!((group_of("a.b.00"), group_of("a.b"), group_of("a.b."), group_of("x")), ("a.b", "a.b", "a.b.", "x"));
        assert!(Grades::parse("not json").is_none() && Grades::parse("[]").is_none());
        // Two clips sharing a stem get path ids but both join the stem's grade.
        let root = Path::new("gen");
        let mut dup = BrowseList::from_paths(root, ["a/x.bvh", "b/x.bvh"].iter().map(|p| root.join(p))).unwrap();
        dup.set_grades(Grades::parse(r#"{"x": {"gates": ["jerk"]}}"#).unwrap());
        assert!(dup.cursors().all(|(_, c)| dup.grades.grade(c).is_some_and(Grade::gated)));
        assert_eq!(order(&dup), ["a/x", "b/x"]);
    }

    #[test]
    fn order_puts_gated_then_failing_groups_last() {
        let l = graded();
        let names: Vec<_> = l.activities.iter().map(|a| a.name.as_str()).collect();
        // run.f is all one failing group: after the others, though it sorts first by name.
        assert_eq!(names, [TOP, "walk.f", "run.f"]);
        assert_eq!(order(&l), ["idle", "walk.f.act.g0.01", "walk.f.act.g0.02", "walk.f.act.g0.00", "walk.f.act.g1.00", "run.f.act.g0.00", "run.f.act.g0.01"]);
        assert_eq!((l.activity, l.clip), (0, 0));
    }

    #[test]
    fn candidates_only_hides_gated_clips_from_navigation() {
        let mut l = graded();
        l.candidates_only = true;
        assert_eq!(l.shown_count(None), 4);
        let mut seen = Vec::new();
        for _ in 0..5 {
            l.advance();
            seen.push(l.current().id.clone());
        }
        // run.f has no candidate: advance wraps past it.
        assert_eq!(seen, ["walk.f.act.g0.01", "walk.f.act.g0.02", "walk.f.act.g1.00", "idle", "walk.f.act.g0.01"]);
        l.step_clip(-1);
        assert_eq!(l.current().id, "walk.f.act.g1.00");
        l.step_clip(1);
        assert_eq!(l.current().id, "walk.f.act.g0.01");
        l.step_activity(1);
        assert_eq!(l.current().id, "idle");
        l.step_activity(-1);
        assert_eq!(l.current().id, "walk.f.act.g0.01");
        // Turning it on while on a gated clip moves to the next candidate.
        l.candidates_only = false;
        l.step_activity(1);
        assert_eq!(l.current().id, "run.f.act.g0.00");
        l.candidates_only = true;
        l.settle(None);
        assert_eq!(l.current().id, "idle");
        // next_unrated skips gated clips too.
        let dir = temp("candidates");
        let v = Verdicts::load(&dir).unwrap();
        assert!(l.next_unrated(&v));
        assert_eq!(l.current().id, "walk.f.act.g0.01");
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn view_filter_cycles_and_navigation_stays_inside_it() {
        let views: Vec<_> = std::iter::successors(Some(View::All), |v| Some(v.next())).take(5).collect();
        assert_eq!(views, [View::All, View::Unrated, View::Maybe, View::Keep, View::All]);
        let dir = temp("view");
        let mut l = graded();
        let mut v = Verdicts::load(&dir).unwrap();
        v.set("walk.f.act.g0.01", Verdict::Maybe);
        v.set("walk.f.act.g1.00", Verdict::Maybe);
        v.set("idle", Verdict::Keep);
        l.view = View::Maybe;
        l.settle(Some(&v));
        assert_eq!(l.current().id, "walk.f.act.g0.01");
        l.advance_in(Some(&v));
        assert_eq!(l.current().id, "walk.f.act.g1.00");
        l.advance_in(Some(&v));
        assert_eq!(l.current().id, "walk.f.act.g0.01");
        l.step_clip_in(-1, Some(&v));
        assert_eq!(l.current().id, "walk.f.act.g1.00");
        // No other activity holds a maybe: stepping activities comes back to walk.f's first maybe.
        l.step_activity_in(1, Some(&v));
        assert_eq!(l.current().id, "walk.f.act.g0.01");
        assert!(!l.next_unrated(&v));
        // Re-rating a maybe as keep takes it out of the view and advances inside it.
        l.rate(&mut v, Verdict::Keep).unwrap();
        assert_eq!(l.current().id, "walk.f.act.g1.00");
        assert_eq!(l.shown_count(Some(&v)), 1);
        l.view = View::Keep;
        l.settle(Some(&v));
        assert_eq!(l.current().id, "idle");
        l.advance_in(Some(&v));
        assert_eq!(l.current().id, "walk.f.act.g0.01");
        // The unrated view: rating advances to the next unrated, next_unrated stays inside.
        l.view = View::Unrated;
        l.settle(Some(&v));
        assert_eq!(l.current().id, "walk.f.act.g0.02");
        l.rate(&mut v, Verdict::Reject).unwrap();
        assert_eq!(l.current().id, "walk.f.act.g0.00");
        assert!(l.next_unrated(&v));
        assert_eq!(l.current().id, "run.f.act.g0.00");
        l.advance_in(Some(&v));
        assert_eq!(l.current().id, "run.f.act.g0.01");
        l.advance_in(Some(&v));
        assert_eq!(l.current().id, "walk.f.act.g0.00");
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn grade_line_shows_gates_metrics_mirror_and_group() {
        let l = graded();
        assert_eq!(
            l.grade_line(&find(&l, "walk.f.act.g0.00")).unwrap(),
            "gates: skate   skate 62 cm/s  worst 84.5 cm  jerk p99 1225  steps 12   group 1/3 gated   type cycle"
        );
        assert_eq!(l.grade_line(&find(&l, "walk.f.act.g0.01")).unwrap(), "ungated   skate 21.1 cm/s  turn -4.6 deg   mirror candidate   group 1/3 gated   type cycle");
        assert_eq!(l.grade_line(&find(&l, "walk.f.act.g1.00")).unwrap(), "ungated      group 4/6 gated FAILING   type -");
        assert_eq!(l.grade_line(&find(&l, "idle")).unwrap(), "grade: (not in grades.json)");
        assert_eq!(list().grade_line(list().current()), None);
    }

    #[test]
    fn without_grades_order_and_filters_change_nothing() {
        let dir = temp("nogrades");
        let mut l = list();
        l.set_grades(Grades::load(&dir));
        assert_eq!(l, list());
        std::fs::write(dir.join(GRADES_FILE), "{ broken").unwrap();
        l.set_grades(Grades::load(&dir));
        assert_eq!(l, list());
        // Candidates-only hides nothing ungraded; the view needs verdicts.
        l.candidates_only = true;
        l.view = View::Keep;
        assert_eq!(l.shown_count(None), l.len());
        let _ = std::fs::remove_dir_all(&dir);
    }
}
