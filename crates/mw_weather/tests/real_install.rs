//! The weather records from the real game data and `Morrowind.ini`.
//! Skips (passes) when no Morrowind install is detected.

use mw_install::MorrowindInstall;
use mw_weather::{MwWeather, WeatherType, climate_id};
use sf_vfs::Vfs;
use sf_weather::{Climate, WeatherBook};

#[test]
fn regions_and_ini_become_weather_records() {
    let Ok((install, _)) = MorrowindInstall::detect(None) else {
        eprintln!("no Morrowind install detected; skipping");
        return;
    };
    let vfs = mw_vfs::mount_install(&install, Vfs::builder()).unwrap().vfs;
    let mut content = mw_records::load(&vfs, &install.game_files()).unwrap();
    let weather = MwWeather::build(&content.db, install.ini.as_ref());
    eprintln!(
        "{} climates, timescale {}, fingerprint {:016x}",
        weather.climates.len(),
        weather.timescale,
        weather.fingerprint
    );
    for (id, c) in &weather.climates {
        eprintln!(
            "  {}: {:?} transition {:.2} h",
            id.as_str(),
            c.weights,
            c.transition_hours
        );
    }
    assert!(weather.climates.len() >= 9, "{}", weather.climates.len());

    let weight = |c: &Climate, ty: WeatherType| {
        c.weights
            .iter()
            .find(|(id, _)| *id == ty.preset_id())
            .map_or(0.0, |(_, w)| *w)
    };
    let get = |region: &str| {
        &weather
            .climates
            .iter()
            .find(|(id, _)| *id == climate_id(region))
            .unwrap()
            .1
    };
    let red = get("Red Mountain Region");
    assert!(
        weight(red, WeatherType::Ashstorm) + weight(red, WeatherType::Blight) > 50.0,
        "{:?}",
        red.weights
    );
    let bitter = get("Bitter Coast Region");
    assert!(
        weight(bitter, WeatherType::Rain) > 0.0,
        "{:?}",
        bitter.weights
    );
    let storm = &weather.presets[WeatherType::Thunderstorm.index()].1;
    assert!(
        storm
            .lightning
            .as_ref()
            .is_some_and(|l| l.per_hour > 0.0 && !l.thunder.is_empty())
    );

    let last = content
        .files
        .last()
        .map(|f| f.source)
        .into_iter()
        .collect::<Vec<_>>();
    weather.define_into(&mut content.db, &last).unwrap();
    let book = WeatherBook::from_db(&content.db);
    assert!(book.climate(&climate_id("Bitter Coast Region")).is_some());
}

/// `RED_MOUNTAIN_SUMMIT` is the highest `LAND` vertex near Red Mountain.
#[test]
fn red_mountain_summit_is_the_highest_land_vertex() {
    let Ok((install, _)) = MorrowindInstall::detect(None) else {
        eprintln!("no Morrowind install detected; skipping");
        return;
    };
    let vfs = mw_vfs::mount_install(&install, Vfs::builder()).unwrap().vfs;
    let content = mw_records::load(&vfs, &install.game_files()).unwrap();
    let [sx, sy, _] = mw_weather::visual::RED_MOUNTAIN_SUMMIT;
    let mut best = [0.0f32, 0.0, f32::MIN];
    for (_, land) in content.db.iter::<tes3::esp::Landscape>() {
        let data = mw_world::terrain::LandData::decode(land);
        let (gx, gy) = data.grid;
        if (gx - 2).abs() > 6 || (gy - 7).abs() > 6 {
            continue;
        }
        for (i, &h) in data.heights.iter().enumerate() {
            if h > best[2] {
                #[allow(clippy::cast_precision_loss)]
                let (x, y) = ((i % 65) as f32 * 128.0, (i / 65) as f32 * 128.0);
                #[allow(clippy::cast_precision_loss)]
                let corner = (gx as f32 * 8192.0, gy as f32 * 8192.0);
                best = [corner.0 + x, corner.1 + y, h];
            }
        }
    }
    eprintln!("highest LAND vertex near Red Mountain: {best:?}");
    let d = (best[0] - sx).hypot(best[1] - sy);
    assert!(d < 1024.0, "summit {best:?} is {d} units from the constant");
}
