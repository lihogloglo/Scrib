//! Joint-name heuristics for the auto-mapper: tokens, side, and a coarse
//! body-part class. Names only *classify*; which humanoid bone a joint
//! becomes is decided by the hierarchy (automap.rs), so ambiguous words
//! ("Leg" is the lower leg in one convention and the whole leg in another)
//! never decide alone.

use crate::humanoid::{Finger, Side};

/// A coarse body-part class from a joint name.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Class {
    /// End sites, nubs, leaves, tips: never mapped.
    End,
    /// Twist, roll, IK targets, poles, props, sockets, correctives: never in limb chains.
    Helper,
    Finger(Option<Finger>),
    Toe,
    Foot,
    Hand,
    LowerArm,
    UpperArm,
    Shoulder,
    LowerLeg,
    UpperLeg,
    Eye,
    Jaw,
    Head,
    Neck,
    Hips,
    Spine,
    Root,
    Other,
}

/// A classified joint name.
#[derive(Clone, Debug)]
pub(crate) struct NameInfo {
    pub class: Class,
    pub side: Option<Side>,
    /// Lowercase word tokens with digits kept, side tokens removed: the
    /// side-neutral key of a joint ("bip|01|fore|twist").
    pub neutral: String,
    /// Word tokens without noise prefixes, sides and digits, joined ("upperarm").
    pub core: String,
    /// All lowercase tokens.
    pub tokens: Vec<String>,
}

/// Lowercase tokens split at non-alphanumerics, camelCase and digit runs.
pub(crate) fn tokenize(name: &str) -> Vec<String> {
    let mut tokens = Vec::new();
    let mut cur = String::new();
    let mut prev: Option<char> = None;
    for c in name.chars() {
        let boundary = match prev {
            None => false,
            Some(p) => {
                !c.is_alphanumeric() || (c.is_uppercase() && p.is_lowercase()) || (c.is_ascii_digit() != p.is_ascii_digit() && p.is_alphanumeric())
            }
        };
        if boundary && !cur.is_empty() {
            tokens.push(std::mem::take(&mut cur));
        }
        if c.is_alphanumeric() {
            cur.extend(c.to_lowercase());
        }
        prev = Some(c);
    }
    if !cur.is_empty() {
        tokens.push(cur);
    }
    tokens
}

/// Exporter and rig-tool prefixes that carry no anatomy.
const NOISE: &[&str] =
    &["def", "org", "mch", "mixamorig", "bip", "biped", "valvebiped", "armature", "rig", "cc", "base", "j", "jnt", "joint", "bone", "b", "c"];

fn side_of(token: &str) -> Option<Side> {
    match token {
        "l" | "left" | "lf" | "lft" => Some(Side::Left),
        "r" | "right" | "rt" | "rgt" => Some(Side::Right),
        _ => None,
    }
}

fn finger_word(token: &str) -> Option<Finger> {
    match token {
        "thumb" => Some(Finger::Thumb),
        "index" | "pointer" => Some(Finger::Index),
        "middle" => Some(Finger::Middle),
        "ring" => Some(Finger::Ring),
        "pinky" | "little" | "small" => Some(Finger::Little),
        _ => None,
    }
}

pub(crate) fn classify(name: &str) -> NameInfo {
    let tokens = tokenize(name);
    let mut side = None;
    let mut neutral = Vec::new();
    let mut words: Vec<&str> = Vec::new();
    for t in &tokens {
        if let Some(s) = side_of(t) {
            side = Some(s);
            continue;
        }
        neutral.push(t.as_str());
        if t.chars().all(|c| c.is_ascii_digit()) || NOISE.contains(&t.as_str()) {
            continue;
        }
        // Rigify's `f_` finger prefix and other one-letter fragments.
        if t.len() > 1 {
            words.push(t);
        }
    }
    let core: String = words.concat();
    let has = |w: &[&str]| words.iter().any(|t| w.contains(t));
    let class = class_of(&words, &core, &has, &tokens);
    NameInfo { class, side, neutral: neutral.join("|"), core, tokens }
}

fn class_of(words: &[&str], core: &str, has: &dyn Fn(&[&str]) -> bool, tokens: &[String]) -> Class {
    if has(&["end", "nub", "tip", "leaf", "top", "endsite"]) {
        return Class::End;
    }
    if has(&[
        "twist",
        "roll",
        "ik",
        "fk",
        "pole",
        "target",
        "helper",
        "ctrl",
        "control",
        "ctl",
        "prop",
        "socket",
        "attach",
        "weapon",
        "shield",
        "corrective",
        "corr",
        "piston",
        "goal",
        "nonaccum",
        "accum",
    ]) {
        return Class::Helper;
    }
    if let Some(f) = words.iter().find_map(|t| finger_word(t)) {
        return Class::Finger(Some(f));
    }
    if has(&["finger", "fingers"]) {
        // 3ds Max Biped: `Finger<f><segment>`, f = 0 (thumb) ..= 4.
        let after = tokens.iter().skip_while(|t| !t.starts_with("finger")).nth(1);
        let digit = after.filter(|t| t.chars().all(|c| c.is_ascii_digit())).and_then(|t| t.chars().next());
        let finger = digit.and_then(|d| d.to_digit(10)).and_then(|d| Finger::ALL.get(d as usize).copied());
        return Class::Finger(finger);
    }
    if has(&["toe", "toes", "ball"]) || core.contains("toebase") {
        return Class::Toe;
    }
    if has(&["foot", "ankle"]) {
        return Class::Foot;
    }
    if has(&["hand", "wrist", "palm"]) {
        return Class::Hand;
    }
    if core.contains("forearm") || core.contains("lowerarm") || has(&["elbow", "ulna", "radius"]) {
        return Class::LowerArm;
    }
    if core.contains("upperarm") || core.contains("uparm") || has(&["arm", "humerus"]) {
        return Class::UpperArm;
    }
    if has(&["clavicle", "shoulder", "collar", "scapula"]) {
        return Class::Shoulder;
    }
    if core.contains("upleg") || core.contains("upperleg") || has(&["thigh", "femur"]) {
        return Class::UpperLeg;
    }
    if core.contains("lowerleg") || has(&["calf", "shin", "knee", "leg", "tibia", "crus"]) {
        return Class::LowerLeg;
    }
    if has(&["eye", "eyeball"]) {
        return Class::Eye;
    }
    if has(&["eyelid", "lid", "brow", "eyebrow", "lash", "cheek", "lip", "tongue", "teeth", "nose", "ear", "mouth"]) {
        return Class::Other;
    }
    if has(&["jaw", "chin", "mandible"]) {
        return Class::Jaw;
    }
    if has(&["head", "skull", "cranium"]) {
        return Class::Head;
    }
    if has(&["neck"]) {
        return Class::Neck;
    }
    if has(&["hips", "pelvis", "hip"]) {
        return Class::Hips;
    }
    if has(&["spine", "chest", "torso", "abdomen", "waist", "belly", "back", "ribs", "ribcage", "upperchest", "stomach"]) {
        return Class::Spine;
    }
    if has(&["root"]) {
        return Class::Root;
    }
    Class::Other
}

/// Whether a name marks a finger metacarpal (UE5's `index_metacarpal_l`).
pub(crate) fn is_metacarpal(info: &NameInfo) -> bool {
    info.tokens.iter().any(|t| matches!(t.as_str(), "metacarpal" | "meta" | "carpal"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classifies_common_conventions() {
        let cases: &[(&str, Class, Option<Side>)] = &[
            ("mixamorig:LeftUpLeg", Class::UpperLeg, Some(Side::Left)),
            ("mixamorig:LeftLeg", Class::LowerLeg, Some(Side::Left)),
            ("mixamorig:RightArm", Class::UpperArm, Some(Side::Right)),
            ("mixamorig:RightForeArm", Class::LowerArm, Some(Side::Right)),
            ("mixamorig:LeftHandThumb1", Class::Finger(Some(Finger::Thumb)), Some(Side::Left)),
            ("mixamorig:HeadTop_End", Class::End, None),
            ("Bip01 L Finger21", Class::Finger(Some(Finger::Middle)), Some(Side::Left)),
            ("Bip01 R Finger0Nub", Class::End, Some(Side::Right)),
            ("Bip01 L Calf", Class::LowerLeg, Some(Side::Left)),
            ("Bip01 Pelvis", Class::Hips, None),
            ("Bip01 L ForeTwist", Class::Helper, Some(Side::Left)),
            ("DEF-upper_arm.L", Class::UpperArm, Some(Side::Left)),
            ("DEF-f_pinky.02.R", Class::Finger(Some(Finger::Little)), Some(Side::Right)),
            ("DEF-shin.L", Class::LowerLeg, Some(Side::Left)),
            ("upperarm_twist_01_l", Class::Helper, Some(Side::Left)),
            ("thumb_04_leaf_l", Class::End, Some(Side::Left)),
            ("index_metacarpal_r", Class::Finger(Some(Finger::Index)), Some(Side::Right)),
            ("ik_foot_l", Class::Helper, Some(Side::Left)),
            ("J_Bip_C_Hips", Class::Hips, None),
            ("J_Bip_L_UpperArm", Class::UpperArm, Some(Side::Left)),
            ("LeftLowerArm", Class::LowerArm, Some(Side::Left)),
            ("spine_03", Class::Spine, None),
            ("ball_l", Class::Toe, Some(Side::Left)),
            ("clavicle_r", Class::Shoulder, Some(Side::Right)),
        ];
        for (name, class, side) in cases {
            let info = classify(name);
            assert_eq!((info.class, info.side), (*class, *side), "{name}");
        }
        assert_eq!(classify("Bip01 L ForeTwist").neutral, classify("Bip01 R ForeTwist").neutral);
    }
}
