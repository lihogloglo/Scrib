//! Biovision Hierarchy (`.bvh`) skeleton and clip importer, our own
//! (animation.md "Bake steps": `bvh_anim` 0.4 dates from 2019 on
//! unmaintained dependencies, `bvh_anim_parser` 1.0 pulls in `cargo` and
//! `regex` as runtime dependencies; the format is a page of grammar).
//!
//! - **Skeleton**: one joint per `ROOT` / `JOINT`, rest translation =
//!   `OFFSET`, rest rotation identity (BVH's zero pose). Each `End Site`
//!   becomes a leaf joint `<parent>_End` with no track: it gives the
//!   head, hands and toes a length for auto-map and auto-align. Names are
//!   the rest of the `ROOT`/`JOINT` line, so 3ds Max's `Bip01 Pelvis`
//!   works; a name used twice gets `#<index>` appended.
//! - **Clip**: one per file, named after the file stem, keyed at every
//!   frame (`Frame Time` gives the rate, kept exactly: 30, 60, 120 fps
//!   sources stay uniformly keyed, so `anim.cook` sees their native rate).
//!   Rotation channels compose in their listed order
//!   (`Zrotation Xrotation Yrotation` is `Rz·Rx·Ry`), degrees. Position
//!   channels replace the `OFFSET` (on any joint, not only the root).
//!   Missing frames at the end of a truncated file are dropped.
//! - **Space**: BVH has no unit or axis metadata. Each file's
//!   [`SourceSpace`] comes from [`BvhAnimImporter::space_for`] (default
//!   centimeters, Y up, facing +Z), and every transform is converted
//!   into the framework's space.

use glam::{Quat, Vec3};

use crate::{
    AnimImporter, AnimSource, Curve, ImportError, SourceClip, SourceJoint, SourceTrack, Trs, detect_root_motion_joint, import::SourceSpace, math,
};

/// Imports `.bvh` files, converting each from its [`SourceSpace`].
#[derive(Clone, Debug, PartialEq)]
pub struct BvhAnimImporter {
    /// The space of files no override matches.
    pub space: SourceSpace,
    /// `(path prefix, space)`: the longest matching prefix wins, so one
    /// importer serves sources in different units (`byo/cmu/` in inches,
    /// `byo/100style/` in centimeters).
    pub overrides: Vec<(String, SourceSpace)>,
}

impl Default for BvhAnimImporter {
    fn default() -> Self {
        Self { space: SourceSpace::centimeters(), overrides: Vec::new() }
    }
}

impl BvhAnimImporter {
    pub fn new(space: SourceSpace) -> Self {
        Self { space, overrides: Vec::new() }
    }

    /// Adds a per-prefix space.
    #[must_use]
    pub fn with_override(mut self, prefix: impl Into<String>, space: SourceSpace) -> Self {
        self.overrides.push((prefix.into(), space));
        self
    }

    /// The space `path` is read in.
    pub fn space_for(&self, path: &str) -> SourceSpace {
        self.overrides.iter().filter(|(p, _)| path.starts_with(p.as_str())).max_by_key(|(p, _)| p.len()).map_or(self.space, |(_, s)| *s)
    }
}

impl AnimImporter for BvhAnimImporter {
    fn extensions(&self) -> &[&str] {
        &["bvh"]
    }

    fn version(&self) -> u32 {
        1
    }

    fn settings_key(&self, path: &str) -> Vec<u8> {
        postcard::to_allocvec(&self.space_for(path)).expect("a space serializes")
    }

    fn import(&self, path: &str, bytes: &[u8]) -> Result<AnimSource, ImportError> {
        let err = |message: String| ImportError { path: path.to_owned(), message };
        let text = std::str::from_utf8(bytes).map_err(|e| err(format!("not UTF-8 text: {e}")))?;
        let stem = path.rsplit(['/', '\\']).next().unwrap_or(path);
        let name = stem.rsplit_once('.').map_or(stem, |(s, _)| s);
        parse(text, name, self.space_for(path)).map_err(err)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Channel {
    Position(usize),
    Rotation(usize),
}

impl Channel {
    fn parse(s: &str) -> Option<Self> {
        let axis = match s.as_bytes().first()?.to_ascii_lowercase() {
            b'x' => 0,
            b'y' => 1,
            b'z' => 2,
            _ => return None,
        };
        match s[1..].to_ascii_lowercase().as_str() {
            "position" => Some(Self::Position(axis)),
            "rotation" => Some(Self::Rotation(axis)),
            _ => None,
        }
    }
}

struct Joint {
    name: String,
    parent: Option<usize>,
    offset: Vec3,
    channels: Vec<Channel>,
}

/// Hierarchy tokens: words, except that a `ROOT`/`JOINT` name is the rest
/// of its line (minus a trailing `{`).
fn tokens(hierarchy: &str) -> Vec<&str> {
    let mut out = Vec::new();
    for line in hierarchy.lines() {
        let line = line.trim();
        let (word, rest) = line.split_once(char::is_whitespace).map_or((line, ""), |(w, r)| (w, r.trim()));
        if word.eq_ignore_ascii_case("ROOT") || word.eq_ignore_ascii_case("JOINT") {
            out.push(word);
            match rest.strip_suffix('{') {
                Some(name) => out.extend([name.trim_end(), "{"]),
                None => out.push(rest),
            }
        } else {
            out.extend(line.split_whitespace());
        }
    }
    out
}

fn parse(text: &str, clip_name: &str, space: SourceSpace) -> Result<AnimSource, String> {
    let conv = space.converter().ok_or("the source space's up and forward axes are parallel")?;
    let motion_at = text.find("MOTION").ok_or("no MOTION section")?;
    let (hierarchy, motion) = text.split_at(motion_at);
    let joints = parse_hierarchy(hierarchy)?;

    // MOTION: `Frames: n`, `Frame Time: t`, then n rows of channel values.
    let mut words = motion.split_whitespace().skip(1);
    expect_word(&mut words, "Frames:")?;
    let declared: usize = words.next().and_then(|t| t.parse().ok()).ok_or("Frames: expected a count")?;
    expect_word(&mut words, "Frame")?;
    expect_word(&mut words, "Time:")?;
    let frame_time: f64 =
        words.next().and_then(|t| t.parse().ok()).filter(|t: &f64| *t > 0.0 && t.is_finite()).ok_or("Frame Time: expected a positive number")?;
    let values: Vec<f32> = words.map(|w| w.parse::<f32>().map_err(|_| format!("bad motion value {w:?}"))).collect::<Result<_, _>>()?;
    let stride: usize = joints.iter().map(|j| j.channels.len()).sum();
    let frames = values.len().checked_div(stride).map_or(0, |n| declared.min(n));
    let names = unique_names(&joints);
    let skeleton: Vec<SourceJoint> = joints
        .iter()
        .zip(&names)
        .map(|(j, name)| SourceJoint { name: name.clone(), parent: j.parent, rest: conv.trs(Trs::from_translation(j.offset)) })
        .collect();
    let mut clips = Vec::new();
    if frames > 0 {
        #[allow(clippy::cast_precision_loss, clippy::cast_possible_truncation)] // Frame indices and times fit.
        let times: Vec<f32> = (0..frames).map(|f| (f as f64 * frame_time) as f32).collect();
        let mut tracks = Vec::new();
        let mut base = 0;
        for (j, name) in joints.iter().zip(&names) {
            let has_pos = j.channels.iter().any(|c| matches!(c, Channel::Position(_)));
            let has_rot = j.channels.iter().any(|c| matches!(c, Channel::Rotation(_)));
            if !has_pos && !has_rot {
                continue;
            }
            let (mut ts, mut rs) = (Vec::with_capacity(frames), Vec::<Quat>::with_capacity(frames));
            for f in 0..frames {
                let row = &values[f * stride + base..f * stride + base + j.channels.len()];
                let (mut t, mut r) = (j.offset, Quat::IDENTITY);
                for (c, &v) in j.channels.iter().zip(row) {
                    match *c {
                        Channel::Position(a) => t[a] = v,
                        Channel::Rotation(a) => r *= Quat::from_axis_angle(Vec3::AXES[a], v.to_radians()),
                    }
                }
                let r = conv.rotation(r);
                rs.push(rs.last().map_or(r, |&prev| math::align_hemisphere(r, prev)));
                ts.push(conv.translation(t));
            }
            tracks.push(SourceTrack {
                joint: name.clone(),
                translation: has_pos.then(|| Curve::linear(times.clone(), ts)),
                rotation: has_rot.then(|| Curve::linear(times.clone(), rs)),
                scale: None,
            });
            base += j.channels.len();
        }
        let duration = times.last().copied().unwrap_or(0.0);
        clips.push(SourceClip { name: clip_name.to_owned(), duration, tracks, ..SourceClip::default() });
    }
    let root_motion = detect_root_motion_joint(&skeleton, &clips).map(|j| skeleton[j].name.clone());
    Ok(AnimSource { skeleton: Some(skeleton), clips, root_motion })
}

fn expect_word<'a>(words: &mut impl Iterator<Item = &'a str>, word: &str) -> Result<(), String> {
    if words.next().is_some_and(|t| t.eq_ignore_ascii_case(word)) { Ok(()) } else { Err(format!("expected {word:?}")) }
}

fn unique_names(joints: &[Joint]) -> Vec<String> {
    let mut seen = std::collections::HashSet::new();
    joints
        .iter()
        .enumerate()
        .map(|(i, j)| {
            let name = if seen.contains(j.name.as_str()) { format!("{}#{i}", j.name) } else { j.name.clone() };
            seen.insert(name.clone());
            name
        })
        .collect()
}

fn parse_hierarchy(hierarchy: &str) -> Result<Vec<Joint>, String> {
    let toks = tokens(hierarchy);
    let mut it = toks.into_iter();
    if !it.next().is_some_and(|t| t.eq_ignore_ascii_case("HIERARCHY")) {
        return Err("expected HIERARCHY".into());
    }
    let mut joints: Vec<Joint> = Vec::new();
    // Open joints; `None` is an End Site.
    let mut stack: Vec<Option<usize>> = Vec::new();
    let num = |it: &mut std::vec::IntoIter<&str>, what: &str| -> Result<f32, String> {
        it.next().and_then(|t| t.parse::<f32>().ok()).ok_or_else(|| format!("{what}: expected a number"))
    };
    while let Some(tok) = it.next() {
        match tok.to_ascii_uppercase().as_str() {
            "ROOT" | "JOINT" => {
                let name = it.next().filter(|n| !n.is_empty() && *n != "{").ok_or("joint without a name")?;
                let parent = stack.iter().rev().find_map(|s| *s);
                if tok.eq_ignore_ascii_case("JOINT") && parent.is_none() {
                    return Err(format!("JOINT {name:?} outside a ROOT"));
                }
                joints.push(Joint {
                    name: name.to_owned(),
                    parent: if tok.eq_ignore_ascii_case("ROOT") { None } else { parent },
                    offset: Vec3::ZERO,
                    channels: Vec::new(),
                });
                if it.next() != Some("{") {
                    return Err(format!("{name:?}: expected {{"));
                }
                stack.push(Some(joints.len() - 1));
            }
            "END" => {
                if !it.next().is_some_and(|t| t.eq_ignore_ascii_case("Site")) || it.next() != Some("{") {
                    return Err("expected End Site {".into());
                }
                let parent = stack.last().copied().flatten().ok_or("End Site outside a joint")?;
                let name = format!("{}_End", joints[parent].name);
                joints.push(Joint { name, parent: Some(parent), offset: Vec3::ZERO, channels: Vec::new() });
                stack.push(None);
            }
            "OFFSET" => {
                let v = Vec3::new(num(&mut it, "OFFSET")?, num(&mut it, "OFFSET")?, num(&mut it, "OFFSET")?);
                // An End Site's joint is the last one pushed.
                let j = stack.last().copied().flatten().unwrap_or(joints.len().saturating_sub(1));
                joints.get_mut(j).ok_or("OFFSET outside a joint")?.offset = v;
            }
            "CHANNELS" => {
                let j = stack.last().copied().flatten().ok_or("CHANNELS outside a joint")?;
                let n: usize = it.next().and_then(|t| t.parse().ok()).ok_or("CHANNELS: expected a count")?;
                for _ in 0..n {
                    let c = it.next().ok_or("CHANNELS: too few names")?;
                    joints[j].channels.push(Channel::parse(c).ok_or_else(|| format!("unknown channel {c:?}"))?);
                }
            }
            "}" => {
                stack.pop().ok_or("unbalanced }")?;
            }
            _ => return Err(format!("unexpected {tok:?} in HIERARCHY")),
        }
    }
    if !stack.is_empty() {
        return Err("unclosed joint at MOTION".into());
    }
    if joints.is_empty() {
        return Err("no joints".into());
    }
    Ok(joints)
}
