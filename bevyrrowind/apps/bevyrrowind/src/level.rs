//! `view level:<name>` (the CLI entry, which takes the clap `ViewFlags`); the levels themselves
//! are `mw_exterior_bevy::level` (TS1, `docs/specs/test-levels.md`).

use std::sync::Arc;
use std::time::Instant;

use mw_exterior_bevy::level::{LevelSetup, STARTED, actors, find, pick_camera, worldspace};
use mw_install::MorrowindInstall;
use sf_level::Level;

/// `view level:<name>`: content (actors and records stay available), then the
/// streamed viewer in the level's worldspace. `--camera <name>` picks one of
/// the level's cameras; raw "ex,ey,ez,tx,ty,tz" still works.
pub(crate) fn view(install: &MorrowindInstall, name: &str, mut flags: crate::ViewFlags) -> crate::Result {
    let started = *STARTED.get_or_init(Instant::now);
    let (source, mut level) = find(name)?;
    lab_overrides(&mut level, &flags)?;
    let camera = pick_camera(&level, &mut flags.camera);
    println!(
        "level {} ({source}): {} objects, cameras {}",
        level.name,
        level.objects.len(),
        level.cameras.iter().map(|c| c.name.as_str()).collect::<Vec<_>>().join(", ")
    );
    let (vfs, content) = mw_boot_bevy::load(install, false)?;
    println!("content loaded in {:.2} s", started.elapsed().as_secs_f32());
    let ws = worldspace(&level.name);
    let start = crate::StreamStart::Interior { worldspace: ws.clone(), eye: camera.eye.as_dvec3(), target: camera.target.as_dvec3() };
    // The level's own hour and weather (anim-lab: clear noon) unless the command line sets them.
    flags.hour = flags.hour.or(level.hour);
    if flags.weather.is_none()
        && let Some(w) = &level.weather
    {
        flags.weather = Some(mw_environment_bevy::weather::parse_weather_type(w)?);
    }
    let mut options = flags.exterior_options(install, &content.db, format!("Bevyrrowind - level {}", level.name), start)?;
    let actors = actors::resolve(&level, &content.db);
    options.actors = Some(Arc::new(content.db));
    options.level = Some(LevelSetup { level: Arc::new(level), worldspace: ws, actors: actors.into() });
    mw_exterior_bevy::exterior::run(Arc::new(vfs), options)
}

/// The light-lab flags (`--lab-lights`, `--lab-ring`, `--lab-radius`, `--lab-orbit`) edit the
/// level's [`sf_level::LightRig`]: a perf sweep steps one of them per run.
fn lab_overrides(level: &mut Level, flags: &crate::ViewFlags) -> crate::Result<()> {
    let lab = &flags.lab;
    if lab.lights.is_none() && lab.ring.is_none() && lab.radius.is_none() && lab.orbit.is_none() {
        return Ok(());
    }
    let Some(rig) = level.lights.as_mut() else {
        return Err(format!("level {}: --lab-* flags need a level with `lights`", level.name).into());
    };
    rig.count = lab.lights.unwrap_or(rig.count);
    if let Some(radius) = lab.ring {
        rig.layout = sf_level::LightLayout::Ring { radius };
    }
    rig.radius = lab.radius.unwrap_or(rig.radius);
    rig.orbit = lab.orbit.unwrap_or(rig.orbit);
    let problems = level.problems();
    if problems.is_empty() { Ok(()) } else { Err(format!("level {}: {problems:?}", level.name).into()) }
}
