//! Surface weather W1 (surface-weather.md "Data model", "A/B switches and console"): the
//! deterministic [`sf_surface`] state stepped on the game clock with the weather
//! ([`WorldSurface`], saved in `sf/weather`), and the console command
//! `surface set <wet|puddle|snow|ash> <0..1> [here|<climate>] | dump | clear`.

use std::fmt::Write as _;

use bevy_app::App;
use bevy_ecs::prelude::*;
use bevy_platform::time::Instant;
use sf_console_bevy::ConsoleAppExt;
use sf_console_bevy::sf_console::{ArgKind, CommandSpec, Param};
use sf_diag_bevy::DiagCounters;
use sf_records::RecordId;
use sf_sky_bevy::GameClock;
use sf_surface::view::DebugView;
use sf_surface::{CELL_M, CellGrid, Channel, RateSource, SurfaceSave, SurfaceState, SurfaceTuning, WeatherRates, cell_of};
use sf_weather::{ClimateIx, WeatherBook};

use crate::{WeatherRecords, WeatherSite, WorldWeather, WorldWeatherMap};

/// The surface state, its tuning and the cover cells of the world weather map's worldspace
/// (built once from its raster, when the map names its worldspace).
#[derive(Resource, Default)]
pub struct WorldSurface {
    pub state: SurfaceState,
    pub tuning: SurfaceTuning,
    grid: Option<CellGrid>,
    generation: u64,
}

impl WorldSurface {
    pub fn grid(&self) -> Option<&CellGrid> {
        self.grid.as_ref()
    }

    /// Bumps on every change the GPU copy must follow (a step, a new grid, the console, a
    /// restore): `sf_surface_bevy` rewrites its cell window when it moves. Code that writes
    /// `state` directly calls [`Self::touch`].
    pub fn generation(&self) -> u64 {
        self.generation
    }

    pub fn touch(&mut self) {
        self.generation = self.generation.wrapping_add(1);
    }

    /// The saved form (climates by id).
    pub fn save(&self, book: &WeatherBook) -> SurfaceSave {
        self.state.to_save(book)
    }

    /// Restores a save's state; `None` (a save from before surface weather) starts over, and
    /// the next step warms up from the weather over the catch-up window.
    pub fn restore(&mut self, save: Option<&SurfaceSave>, book: &WeatherBook) {
        self.state = save.map_or_else(SurfaceState::default, |s| SurfaceState::from_save(s, book));
        self.touch();
    }
}

/// `surface debug <off|exposure|cells|porosity|trails>`: the debug view `sf_surface_bevy` shows.
#[derive(Resource, Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct SurfaceDebug(pub DebugView);

/// Game hours since the clock's epoch.
fn game_hours(time: &GameClock) -> f64 {
    time.now.hours()
}

/// `WeatherSystems::Evaluate`: steps the state to the clock (none most frames: one step per
/// 10 game minutes).
#[allow(clippy::too_many_arguments)]
pub(crate) fn advance(
    time: Res<GameClock>,
    records: Res<WeatherRecords>,
    world: Res<WorldWeather>,
    map: Option<Res<WorldWeatherMap>>,
    mut surface: ResMut<WorldSurface>,
    counters: Option<Res<DiagCounters>>,
) {
    let WorldSurface { state, tuning, grid, generation } = &mut *surface;
    if let Some(m) = map.as_deref()
        && let Some(ws) = &m.worldspace
        && grid.as_ref().is_none_or(|g| g.worldspace() != ws)
    {
        *grid = Some(CellGrid::new(ws.clone(), m.map.raster()));
        *generation = generation.wrapping_add(1);
    }
    let t0 = Instant::now();
    let rates = WeatherRates { book: &records.0, state: &world.0 };
    let grids: Vec<&CellGrid> = grid.iter().collect();
    let steps = sf_surface::advance(state, game_hours(&time), &rates, &grids, tuning);
    if steps > 0 {
        *generation = generation.wrapping_add(1);
    }
    if steps > 0
        && let Some(c) = counters
    {
        c.add_named("surface/advance_cpu_ms", sf_diag_bevy::sf_diag::unit::MS, t0.elapsed().as_secs_f64() * 1000.0);
        #[expect(clippy::cast_precision_loss, reason = "cell counts fit f64")]
        c.add_named("surface/active_cells", "count", state.active_cells() as f64);
        c.add_named("surface/steps", "count", f64::from(steps));
    }
}

pub(crate) fn register_console(app: &mut App) {
    app.add_console_command(
        CommandSpec::new("surface")
            .help(
                "Surface weather (host): `surface dump`, `surface clear`, `surface set <wet|puddle|snow|ash> <0..1> [here|<climate>]` \
                 (here: the camera's climates; snow/ash set every cell those climates weigh); \
                 `surface debug <off|exposure|cells|porosity|trails>` (tint: shelter exposure, 32 m cells, porosity, trail ring)",
            )
            .param(Param::new("action", ArgKind::choice(&["set", "dump", "clear", "debug"])))
            .param(Param::new("channel", ArgKind::choice(&["wet", "puddle", "snow", "ash", "off", "exposure", "cells", "porosity", "trails"])).optional())
            .param(Param::new("value", ArgKind::Float { min: 0.0, max: 1.0 }).optional())
            .param(Param::rest("target").optional()),
        |inv| {
            let world = &mut *inv.ctx;
            match inv.args.str("action").unwrap_or("dump") {
                "clear" => {
                    let mut s = world.resource_mut::<WorldSurface>();
                    s.state.clear();
                    s.touch();
                    inv.log.info("surface: cleared (dry and bare)");
                }
                "debug" => {
                    let view = inv.args.str("channel").and_then(DebugView::parse).ok_or("surface debug needs a view: off, exposure, cells, porosity or trails")?;
                    world.insert_resource(SurfaceDebug(view));
                    inv.log.info(&format!("surface: debug view {}", view.name()));
                }
                "set" => {
                    let channel = inv.args.str("channel").and_then(Channel::parse).ok_or("surface set needs a channel: wet, puddle, snow or ash")?;
                    #[expect(clippy::cast_possible_truncation, reason = "a 0..1 amount")]
                    let value = inv.args.float("value").ok_or("surface set needs a value in 0..1")? as f32;
                    let target = inv.args.str("target").map_or("here", |t| t.trim().trim_matches('"'));
                    let n = set(world, channel, value, target)?;
                    inv.log.info(&format!("surface: {} = {value:.3} on {n} {}", channel.name(), if channel.per_climate() { "climates" } else { "cells" }));
                }
                _ => {
                    for line in dump(world) {
                        inv.log.info(&line);
                    }
                }
            }
            Ok(())
        },
    );
}

/// `surface set`: forces `channel` on the target's climates (wet, puddle) or on every cell those
/// climates weigh (snow, ash). Returns how many climates or cells changed.
pub fn set(world: &mut World, channel: Channel, value: f32, target: &str) -> Result<usize, String> {
    let book = world.resource::<WeatherRecords>().0.clone();
    let climates: Vec<ClimateIx> = if target.is_empty() || target.eq_ignore_ascii_case("here") {
        let site = world.get_resource::<WeatherSite>().map(|s| s.climates.iter().filter(|c| c.1 > 0.0).map(|c| c.0).collect::<Vec<_>>()).unwrap_or_default();
        if site.is_empty() {
            return Err("surface: no camera site yet (name a climate)".into());
        }
        site
    } else {
        vec![book.climate(&RecordId::new(target)).ok_or_else(|| format!("surface: unknown climate {target:?}"))?]
    };
    let mut surface = world.resource_mut::<WorldSurface>();
    surface.touch();
    let WorldSurface { state, grid, .. } = &mut *surface;
    if channel.per_climate() {
        for &c in &climates {
            state.set_climate(c, channel, value);
        }
        return Ok(climates.len());
    }
    let grid = grid.as_ref().ok_or("surface: no cover cells (the world weather map names no worldspace)")?;
    let mut n = 0;
    for ix in 0..grid.len() {
        if grid.site(ix).iter().any(|e| climates.contains(&e.climate)) {
            state.set_cell(grid.worldspace(), grid.cell(ix), channel, value);
            n += 1;
        }
    }
    Ok(n)
}

/// `surface dump`: the time, the camera's site and the surface there, the climates' current
/// rates and non-zero state, and the cell counts.
pub fn dump(world: &World) -> Vec<String> {
    let Some(surface) = world.get_resource::<WorldSurface>() else { return vec!["surface: off".into()] };
    let book = &world.resource::<WeatherRecords>().0;
    let weather = &world.resource::<WorldWeather>().0;
    let s = &surface.state;
    let mut out = Vec::new();
    let grid = surface.grid.as_ref().map_or_else(
        || "no cell grid".to_owned(),
        |g| {
            let (_, size) = g.extent();
            format!("grid {}x{} cells of {CELL_M} m over {}", size.x, size.y, g.worldspace())
        },
    );
    out.push(format!(
        "surface t {:.2} h (step {:.3} h), {} active cells, {grid}",
        s.t_h,
        surface.tuning.step_h,
        s.active_cells()
    ));
    let rates = WeatherRates { book, state: weather };
    if let Some(site) = world.get_resource::<WeatherSite>()
        && !site.climates.is_empty()
    {
        let mut line = "site:".to_owned();
        for &(c, w) in &site.climates {
            let cs = s.climate(c);
            let r = rates.rates(c, s.t_h);
            let _ = write!(
                line,
                " {} {w:.2} [wet {:.3} puddle {:.3}; rates wet {:.2} puddle {:.2} snow {:.2} ash {:.2} dry {:.2}]",
                book.climate_id(c),
                cs.wet,
                cs.puddle,
                r.wet,
                r.puddle,
                r.snow,
                r.ash,
                surface.tuning.drying(&r)
            );
        }
        out.push(line);
    }
    if let (Some(map), Some(grid)) = (world.get_resource::<WorldWeatherMap>(), surface.grid.as_ref())
        && let Some(cam) = map.camera
    {
        let p = sf_surface::sample(s, map.map.raster(), grid.worldspace(), cam);
        let cell = cell_of(cam);
        out.push(format!(
            "here: cell ({}, {}) wet {:.3} puddle {:.3} snow {:.3} ash {:.3}",
            cell.x, cell.y, p.wet, p.puddle, p.snow, p.ash
        ));
    }
    let mut wet = "climates:".to_owned();
    let mut any = false;
    for (i, c) in s.climates.iter().enumerate() {
        if c.wet > 0.0 || c.puddle > 0.0 {
            any = true;
            let id = book.climate_id(ClimateIx(u32::try_from(i).expect("climate index fits u32")));
            let _ = write!(wet, " {id} wet {:.3} puddle {:.3};", c.wet, c.puddle);
        }
    }
    out.push(if any { wet } else { "climates: all dry".into() });
    for (ws, cells) in &s.cells {
        let (snow, ash) = cells.values().fold((0, 0), |(a, b), c| (a + usize::from(c.snow > 0.0), b + usize::from(c.ash > 0.0)));
        out.push(format!("cells {ws}: {} active ({snow} snow, {ash} ash)", cells.len()));
    }
    out
}
