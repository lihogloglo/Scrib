//! Named test scenes for visual checks (CLAUDE.md rule 5,
//! `docs/visual-checks.md`): `bevyrrowind scene <name>` opens the
//! interactive viewer at a scene from `scenes.ron`, `scene --list` lists them.
//!
//! A scene is a saved `view` command line (cell, camera, hour, flags) plus
//! what to look for; running one parses that command line and takes the
//! `view` path unchanged, so scenes need no rendering code of their own.
//! Runtime A/B keys come from [`mw_devtools_bevy::live_ab`].
//!
//! The registry is read from, in order: `--file`, `scenes.ron` in the
//! current folder, `scenes.ron` next to the exe, and the copy built into
//! the exe (`bevyrrowind/scenes.ron` at build time).

use std::path::{Path, PathBuf};

use clap::Parser;
use mw_game_core::scenes::SceneDef;
use serde::Deserialize;

/// The registry built into the exe.
const BUILT_IN: &str = include_str!("../../../scenes.ron");
const FILE_NAME: &str = "scenes.ron";

/// `scenes.ron`.
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct SceneFile {
    scenes: Vec<SceneDef>,
}

/// `bevyrrowind scene [<name>] [--list] [--file f] [view flags...]`.
pub fn run(
    morrowind: Option<PathBuf>,
    name: Option<&str>,
    list: bool,
    file: Option<&Path>,
    extra: Vec<String>,
) -> crate::Result {
    let (source, scenes) = load(file)?;
    let Some(name) = name.filter(|_| !list) else {
        print_list(&source, &scenes);
        return Ok(());
    };
    let scene = find(&scenes, name)?.clone();
    let args = view_args(&scene, extra);
    println!("scene {} ({source}):\n  bevyrrowind {}", scene.name, quoted(&args));
    if let Some(features) = &scene.build {
        println!("  needs an exe built with `--features {features}`");
    }
    if !scene.look_for.is_empty() {
        println!("  look for: {}", scene.look_for);
    }
    let mut command_line = vec!["bevyrrowind".to_owned()];
    if let Some(m) = morrowind {
        command_line.push("--morrowind".to_owned());
        command_line.push(m.display().to_string());
    }
    command_line.extend(args);
    let cli = crate::Cli::try_parse_from(command_line)?;
    mw_game_core::scenes::set_active(scene);
    crate::run(cli)
}

/// The `view` arguments of a scene, then the caller's extra flags (later flags win in clap).
fn view_args(scene: &SceneDef, extra: Vec<String>) -> Vec<String> {
    let place = match (&scene.level, &scene.cell) {
        (Some(level), _) => format!("{}{level}", mw_exterior_bevy::level::PREFIX),
        (None, Some(cell)) => cell.clone(),
        (None, None) => String::new(),
    };
    let mut args = vec!["view".to_owned(), place];
    if let Some(camera) = &scene.camera {
        args.push("--camera".to_owned());
        args.push(camera.clone());
    }
    if let Some(hour) = scene.hour {
        args.push("--hour".to_owned());
        args.push(hour.to_string());
    }
    args.extend(scene.flags.iter().cloned());
    args.extend(extra);
    args
}

/// Exact name, else a unique prefix.
fn find<'a>(scenes: &'a [SceneDef], name: &str) -> crate::Result<&'a SceneDef> {
    if let Some(s) = scenes.iter().find(|s| s.name.eq_ignore_ascii_case(name)) {
        return Ok(s);
    }
    let lower = name.to_ascii_lowercase();
    let matches: Vec<&SceneDef> = scenes.iter().filter(|s| s.name.to_ascii_lowercase().starts_with(&lower)).collect();
    match matches[..] {
        [one] => Ok(one),
        [] => Err(format!("no scene {name:?}; `bevyrrowind scene --list` lists them").into()),
        _ => Err(format!(
            "scene {name:?} is ambiguous: {}",
            matches.iter().map(|s| s.name.as_str()).collect::<Vec<_>>().join(", ")
        )
        .into()),
    }
}

/// The registry and where it came from.
fn load(file: Option<&Path>) -> crate::Result<(String, Vec<SceneDef>)> {
    let exe_dir = std::env::current_exe().ok().and_then(|p| p.parent().map(Path::to_path_buf));
    let candidates = file
        .map(Path::to_path_buf)
        .into_iter()
        .chain(std::iter::once(PathBuf::from(FILE_NAME)))
        .chain(exe_dir.map(|d| d.join(FILE_NAME)));
    for (i, path) in candidates.enumerate() {
        match std::fs::read_to_string(&path) {
            Ok(text) => return Ok((path.display().to_string(), parse(&text, &path.display().to_string())?)),
            // An explicit `--file` must exist.
            Err(e) if i == 0 && file.is_some() => return Err(format!("{}: {e}", path.display()).into()),
            Err(_) => {}
        }
    }
    Ok(("built-in".to_owned(), parse(BUILT_IN, "built-in scenes.ron")?))
}

fn parse(text: &str, source: &str) -> crate::Result<Vec<SceneDef>> {
    let file: SceneFile = ron::from_str(text).map_err(|e| format!("{source}: {e}"))?;
    let mut seen = std::collections::HashSet::new();
    for s in &file.scenes {
        if !seen.insert(s.name.to_ascii_lowercase()) {
            return Err(format!("{source}: scene {:?} defined twice", s.name).into());
        }
        if s.cell.is_some() == s.level.is_some() {
            return Err(format!("{source}: scene {:?} needs exactly one of `cell` and `level`", s.name).into());
        }
    }
    Ok(file.scenes)
}

fn print_list(source: &str, scenes: &[SceneDef]) {
    println!("{} scenes ({source}); run one with `bevyrrowind scene <name> [view flags]`:\n", scenes.len());
    let width = scenes.iter().map(|s| s.name.len()).max().unwrap_or(0);
    for s in scenes {
        let checks = if s.checks.is_empty() { String::new() } else { format!(" [{}]", s.checks.join(", ")) };
        let build = s.build.as_ref().map_or(String::new(), |b| format!(" (build: {b})"));
        let level = s.level.as_ref().map_or(String::new(), |l| format!("[level {l}] "));
        println!("{:<width$}  {level}{}{checks}{build}", s.name, s.description);
    }
}

fn quoted(args: &[String]) -> String {
    args.iter()
        .map(|a| if a.contains([' ', ',']) { format!("\"{a}\"") } else { a.clone() })
        .collect::<Vec<_>>()
        .join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn built_in_registry_parses() {
        let scenes = parse(BUILT_IN, "built-in").expect("scenes.ron parses");
        assert!(!scenes.is_empty());
        for s in &scenes {
            let args = view_args(s, vec![]);
            assert!(crate::Cli::try_parse_from(std::iter::once("bevyrrowind".to_owned()).chain(args)).is_ok(), "scene {} parses", s.name);
            if let Some(level) = &s.level {
                assert!(mw_exterior_bevy::level::find(level).is_ok(), "scene {}: level {level} loads", s.name);
            }
        }
    }

    /// Flags after the scene name win, also over flags the scene sets itself.
    #[test]
    fn extra_flags_override_the_scene() {
        let scenes = parse(BUILT_IN, "built-in").expect("scenes.ron parses");
        let camera_of = |s: &SceneDef, extra: &[&str]| {
            let args = view_args(s, extra.iter().map(|&a| a.to_owned()).collect());
            match crate::Cli::try_parse_from(std::iter::once("bevyrrowind".to_owned()).chain(args)).expect("parses").command {
                Some(crate::Command::View { flags, .. }) => flags.camera,
                _ => panic!("a scene runs `view`"),
            }
        };
        let level = find(&scenes, "level-robe-walk").expect("scene");
        assert_eq!(camera_of(level, &["--camera", "course"]).as_deref(), Some("course"));
        let placed = scenes.iter().find(|s| s.camera.is_some()).expect("a scene with its own camera");
        assert_eq!(camera_of(placed, &["--camera", "1,2,3,4,5,6"]).as_deref(), Some("1,2,3,4,5,6"));
    }

    /// A scene that places its camera keeps it (no player mode at the start).
    #[test]
    fn fixed_camera_scenes() {
        let scenes = parse(BUILT_IN, "built-in").expect("scenes.ron parses");
        assert!(scenes.iter().find(|s| s.camera.is_some()).expect("a scene with its own camera").fixed_camera());
        let mut free = scenes[0].clone();
        free.camera = None;
        free.flags.clear();
        assert!(!free.fixed_camera());
        free.flags = vec!["--camera".to_owned(), "course".to_owned()];
        assert!(free.fixed_camera());
    }

    #[test]
    fn prefix_lookup() {
        let scenes = parse(BUILT_IN, "built-in").expect("scenes.ron parses");
        let first = &scenes[0];
        assert_eq!(find(&scenes, &first.name.to_ascii_uppercase()).expect("exact").name, first.name);
        assert!(find(&scenes, "no-such-scene-xyz").is_err());
    }
}
