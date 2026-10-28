//! The Bevy-free half of `clip-preview`: which file, clip and source a job names, and the cook
//! onto the actor's skeleton, in process, through the bake's own call
//! ([`mw_bake::anim::cook_library_clip`]: the `skeleton.cook`, `rig.profile`, `anim.cook` and
//! `anim.retarget` steps' calls and rules, MG3). No store, no bake round trip: a new clip
//! previews in seconds.

use std::path::{Path, PathBuf};

use mw_bake::anim::HumanoidTarget;
use mw_bake::sources::AnimSources;
use mw_graphs::sources::SourcesManifest;
use sf_anim::{Clip, ClipRules, Looping, RootMotionMode, RootMotionRules};

/// One clip to preview, as the command line or a `--batch` line names it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ClipJob {
    /// A path, or a file name in the animation sources folder (`UAL1_RM.glb`).
    pub file: String,
    /// The clip in the file (a GLB pack holds many); `None`: the file's first clip.
    pub name: Option<String>,
    /// `sources.ron` source id (unit, up, rest frame, root rules); `None`: the source whose
    /// file list names this file.
    pub source: Option<String>,
}

impl ClipJob {
    /// A `--batch` line: `path[|name[|source]]`; empty fields take the defaults given.
    /// `None` for blank lines and `#` comments.
    pub fn parse_line(line: &str, defaults: &ClipJob) -> Option<Self> {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            return None;
        }
        let mut fields = line.split('|').map(str::trim);
        let file = fields.next().unwrap_or_default().to_owned();
        let field = |f: Option<&str>, d: &Option<String>| f.filter(|s| !s.is_empty()).map(str::to_owned).or_else(|| d.clone());
        let name = field(fields.next(), &defaults.name);
        let source = field(fields.next(), &defaults.source);
        Some(Self { file, name, source })
    }
}

/// A job checked against the disk and the manifest.
#[derive(Clone, Debug)]
pub struct Resolved {
    /// File stem, then `.name` for a pack clip: the output name.
    pub id: String,
    pub path: PathBuf,
    pub source: String,
    pub name: Option<String>,
}

/// The file on disk: the path as given, else in the sources folder (`<dir>/<file>`,
/// `<dir>/<source>/<file>`, then the bundled Quaternius folder).
pub fn locate(file: &str, source: Option<&str>) -> Option<PathBuf> {
    let given = PathBuf::from(file);
    if given.is_file() {
        return Some(given);
    }
    let sources = AnimSources::local();
    let mut tries = vec![sources.dir.join(file)];
    tries.extend(source.map(|s| sources.dir.join(s).join(file)));
    tries.extend(sources.manifest.sources.keys().map(|s| sources.dir.join(s).join(file)));
    tries.extend(sources.bundled.iter().map(|b| b.join(file)));
    tries.into_iter().find(|p| p.is_file())
}

/// The source whose manifest file list names `path` (its path ends with a listed name).
pub fn infer_source(path: &Path) -> Option<String> {
    let p = path.to_string_lossy().replace('\\', "/");
    SourcesManifest::builtin().sources.iter().find(|(_, s)| s.files.iter().any(|f| p.ends_with(&f.name))).map(|(id, _)| id.clone())
}

/// Output name: `<stem>` or `<stem>.<clip>`, filesystem-safe.
fn clip_id(path: &Path, name: Option<&str>) -> String {
    let stem = path.file_stem().map_or_else(|| "clip".into(), |s| s.to_string_lossy().into_owned());
    let raw = match name {
        Some(n) => format!("{stem}.{n}"),
        None => stem,
    };
    raw.chars().map(|c| if c.is_ascii_alphanumeric() || matches!(c, '.' | '-' | '_') { c } else { '_' }).collect()
}

/// Finds the job's file and source (before the window opens, so a typo fails at once).
///
/// # Errors
/// No such file, or no source (none given and the manifest lists no such file), or an unknown one.
pub fn resolve(job: &ClipJob) -> Result<Resolved, String> {
    let path = locate(&job.file, job.source.as_deref()).ok_or_else(|| format!("{}: no such file (path, or in the animation sources folder)", job.file))?;
    let manifest = SourcesManifest::builtin();
    let known = || manifest.sources.keys().map(String::as_str).collect::<Vec<_>>().join(", ");
    let source = job
        .source
        .clone()
        .or_else(|| infer_source(&path))
        .ok_or_else(|| format!("{}: no sources.ron source lists this file; pass --source ({})", path.display(), known()))?;
    if !manifest.sources.contains_key(&source) {
        return Err(format!("unknown source {source:?} (sources.ron has {})", known()));
    }
    Ok(Resolved { id: clip_id(&path, job.name.as_deref()), path, source, name: job.name.clone() })
}

/// The preview's cook rules: the whole clip, played once, root motion driven (horizontal travel
/// and yaw onto the root track, height left in the pose), as a `Drive` slot's.
pub fn preview_rules() -> ClipRules {
    ClipRules { looping: Looping::Once, root_motion: RootMotionRules { mode: RootMotionMode::Drive, ..RootMotionRules::default() }, ..ClipRules::default() }
}

/// The resolved clip cooked and retargeted onto `target`.
///
/// # Errors
/// As [`mw_bake::anim::cook_library_clip`]: import, skeleton map, missing clip, retarget.
pub fn cook(job: &Resolved, target: &HumanoidTarget) -> Result<Clip, String> {
    mw_bake::anim::cook_library_clip(&job.source, &job.path, job.name.as_deref(), &preview_rules(), target)
}

/// Frames rendered for a clip of `duration` seconds at `fps`: one per `1/fps` step from 0,
/// the last on the clip's end (a 2 s clip at 30 fps: 61 frames).
pub fn frame_count(duration: f32, fps: u32) -> u32 {
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss, clippy::cast_precision_loss, reason = "a clip is seconds long at tens of fps")]
    let steps = (duration.max(0.0) * fps as f32).round() as u32;
    steps + 1
}

/// `--at x,y,z,yaw` (MG6): where the furniture stands in the preview scene's frame (meters, Y up,
/// the actor at the origin facing +Z) and its turn about Y in degrees (counter-clockwise seen
/// from above).
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct At {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub yaw_deg: f32,
}

impl std::str::FromStr for At {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, String> {
        let fields = s.split(',').map(|f| f.trim().parse::<f32>().map_err(|e| format!("--at {s:?}: {f:?}: {e}"))).collect::<Result<Vec<_>, _>>()?;
        if !fields.iter().all(|v| v.is_finite()) {
            return Err(format!("--at {s:?}: not finite"));
        }
        match fields[..] {
            [x, y, z] => Ok(Self { x, y, z, yaw_deg: 0.0 }),
            [x, y, z, yaw_deg] => Ok(Self { x, y, z, yaw_deg }),
            _ => Err(format!("--at {s:?}: x,y,z[,yaw] (meters, degrees)")),
        }
    }
}

/// `--prop <record id>[@<bone>]` (MG7a): an item whose model the actor holds, attached to a
/// skeleton joint (`None`: the game's right-hand attach bone, `Weapon Bone`).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PropSpec {
    pub id: String,
    pub bone: Option<String>,
}

impl std::str::FromStr for PropSpec {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, String> {
        let (id, bone) = match s.split_once('@') {
            Some((id, bone)) => (id.trim(), Some(bone.trim())),
            None => (s.trim(), None),
        };
        if id.is_empty() {
            return Err(format!("--prop {s:?}: <record id>[@<bone>]"));
        }
        if bone.is_some_and(str::is_empty) {
            return Err(format!("--prop {s:?}: empty bone after '@'"));
        }
        Ok(Self { id: id.to_owned(), bone: bone.map(str::to_owned) })
    }
}

/// `--prop-offset x,y,z[,rx,ry,rz]` (MG7a): the prop's transform under its bone, in the bone's
/// frame: meters, then degrees about X, Y, Z (applied in that order: `EulerRot::XYZ`).
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PropOffset {
    pub translation: [f32; 3],
    pub rotation_deg: [f32; 3],
}

impl std::str::FromStr for PropOffset {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, String> {
        let fields = s.split(',').map(|f| f.trim().parse::<f32>().map_err(|e| format!("--prop-offset {s:?}: {f:?}: {e}"))).collect::<Result<Vec<_>, _>>()?;
        if !fields.iter().all(|v| v.is_finite()) {
            return Err(format!("--prop-offset {s:?}: not finite"));
        }
        match fields[..] {
            [x, y, z] => Ok(Self { translation: [x, y, z], rotation_deg: [0.0; 3] }),
            [x, y, z, rx, ry, rz] => Ok(Self { translation: [x, y, z], rotation_deg: [rx, ry, rz] }),
            _ => Err(format!("--prop-offset {s:?}: x,y,z[,rx,ry,rz] (meters, degrees)")),
        }
    }
}

/// Up to `n` ids from `ids` closest to `query` (case-insensitive edit distance, ids that contain
/// the query first), for an unknown-id error.
pub fn near_matches<'a>(query: &str, ids: impl IntoIterator<Item = &'a str>, n: usize) -> Vec<String> {
    let q = query.to_ascii_lowercase();
    let mut scored: Vec<(bool, usize, String)> = ids
        .into_iter()
        .map(|id| {
            let lower = id.to_ascii_lowercase();
            (!lower.contains(&q), edit_distance(&q, &lower), id.to_owned())
        })
        .collect();
    scored.sort_unstable();
    scored.dedup_by(|a, b| a.2 == b.2);
    scored.into_iter().take(n).map(|(.., id)| id).collect()
}

/// Levenshtein distance over bytes (ids are ASCII).
fn edit_distance(a: &str, b: &str) -> usize {
    let (a, b) = (a.as_bytes(), b.as_bytes());
    let mut row: Vec<usize> = (0..=b.len()).collect();
    for (i, &ca) in a.iter().enumerate() {
        let mut diagonal = row[0];
        row[0] = i + 1;
        for (j, &cb) in b.iter().enumerate() {
            let next = (diagonal + usize::from(ca != cb)).min(row[j] + 1).min(row[j + 1] + 1);
            diagonal = row[j + 1];
            row[j + 1] = next;
        }
    }
    row[b.len()]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn at_parses_meters_and_yaw() {
        assert_eq!("0.1,-0.2,0.35,90".parse::<At>(), Ok(At { x: 0.1, y: -0.2, z: 0.35, yaw_deg: 90.0 }));
        assert_eq!(" 1, 2 ,3".parse::<At>(), Ok(At { x: 1.0, y: 2.0, z: 3.0, yaw_deg: 0.0 }));
        assert!("1,2".parse::<At>().is_err());
        assert!("1,2,3,4,5".parse::<At>().is_err());
        assert!("1,x,3".parse::<At>().is_err());
        assert!("1,2,3,inf".parse::<At>().is_err());
    }

    #[test]
    fn prop_parses_id_and_bone() {
        assert_eq!("misc_com_tankard_01".parse::<PropSpec>(), Ok(PropSpec { id: "misc_com_tankard_01".into(), bone: None }));
        assert_eq!("misc_com_broom_01@Bip01 L Hand".parse::<PropSpec>(), Ok(PropSpec { id: "misc_com_broom_01".into(), bone: Some("Bip01 L Hand".into()) }));
        assert!("".parse::<PropSpec>().is_err());
        assert!("@Bip01 R Hand".parse::<PropSpec>().is_err());
        assert!("misc_com_tankard_01@".parse::<PropSpec>().is_err());
    }

    #[test]
    fn prop_offset_parses_meters_and_degrees() {
        assert_eq!("0.01,-0.02,0.03".parse::<PropOffset>(), Ok(PropOffset { translation: [0.01, -0.02, 0.03], rotation_deg: [0.0; 3] }));
        assert_eq!("0,0,0.1, 90,0,-45".parse::<PropOffset>(), Ok(PropOffset { translation: [0.0, 0.0, 0.1], rotation_deg: [90.0, 0.0, -45.0] }));
        assert!("1,2".parse::<PropOffset>().is_err());
        assert!("1,2,3,4".parse::<PropOffset>().is_err());
        assert!("1,2,3,4,5,6,7".parse::<PropOffset>().is_err());
        assert!("1,2,nan".parse::<PropOffset>().is_err());
    }

    #[test]
    fn near_matches_rank_containing_ids_first() {
        let ids = ["furn_de_p_chair_01", "furn_de_p_chair_02", "furn_com_r_chair_01", "ex_hlaalu_bridge_01"];
        let m = near_matches("de_p_chair", ids, 2);
        assert_eq!(m, ["furn_de_p_chair_01", "furn_de_p_chair_02"]);
        assert_eq!(near_matches("furn_de_p_chiar_01", ids, 1), ["furn_de_p_chair_01"]);
    }

    #[test]
    fn frames_cover_the_clip_end_to_end() {
        assert_eq!(frame_count(2.0, 30), 61);
        assert_eq!(frame_count(0.0, 30), 1);
        assert_eq!(frame_count(1.0 / 30.0, 30), 2);
    }

    #[test]
    fn batch_lines() {
        let d = ClipJob { file: String::new(), name: Some("Idle".into()), source: None };
        assert_eq!(ClipJob::parse_line("  # x", &d), None);
        assert_eq!(ClipJob::parse_line("", &d), None);
        assert_eq!(ClipJob::parse_line("a b.glb", &d), Some(ClipJob { file: "a b.glb".into(), name: Some("Idle".into()), source: None }));
        assert_eq!(
            ClipJob::parse_line("x.bvh||generated", &d),
            Some(ClipJob { file: "x.bvh".into(), name: Some("Idle".into()), source: Some("generated".into()) })
        );
        assert_eq!(ClipJob::parse_line("x.glb|Walk", &d).and_then(|j| j.name), Some("Walk".into()));
    }

    /// A bundled pack by file name: found in the bundled folder, its source from the manifest.
    #[test]
    fn bundled_pack_resolves_by_name() {
        let job = ClipJob { file: "UAL1_RM.glb".into(), name: Some("Idle_Talking_Loop".into()), source: None };
        if locate(&job.file, None).is_none() {
            eprintln!("skipped: UAL1_RM.glb not found (Git LFS not pulled?)");
            return;
        }
        let r = resolve(&job).unwrap();
        assert_eq!(r.source, "quaternius_ual1");
        assert_eq!(r.id, "UAL1_RM.Idle_Talking_Loop");
        assert!(resolve(&ClipJob { source: Some("nope".into()), ..job }).is_err());
    }
}
