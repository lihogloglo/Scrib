//! The live perf panel (ROADMAP 1.25 CW-17; `OpenMW`'s F3 profiler, UE's `stat unit` + `stat gpu`):
//! F3 cycles off / compact / full in every interactive run, release builds included (it rides on
//! [`crate::live_ab::add`]); console and BRP `flip perfpanel` ([`crate::live_ab::Switch::PerfPanel`]).
//! Wiki: `docs/wiki/engine/app-and-dev-tools.md` "Perf panel".
//!
//! It measures nothing of its own: it shows the gauges benchmarks already record, smoothed over
//! `diag.panel.window_s` and redrawn every `diag.panel.refresh_s`:
//! - frame: `Time<Real>` delta (mean, p99, max over the window);
//! - main / render thread busy: `sf_diag_bevy::FramePhasePlugin` (`main/busy/ms`,
//!   `render/busy/ms`), main stages (`stage/main/*`), encoders and submits (`encode/*`, CW-5b);
//! - views by kind and lamp faces culled: `sf_diag_bevy::RenderViewsPlugin` (`views/*`);
//! - GPU: Bevy's `RenderDiagnosticsPlugin` (`render/<pass>/elapsed_gpu`), only when it runs
//!   (`diag.panel.gpu_timers`, or the GI's timer); the GPU frame is the sum of the top-level timed
//!   passes (partial: untimed passes are missing);
//! - lights, entities, the graphics preset (`sf.graphics_preset`; when `custom`, the groups that differ)
//!   and the tunables set above the preset: read at refresh from the world.
//!
//! Cost: the two gauge plugins are added to every interactive run and the panel owns them. Hidden
//! (or `diag.panel.gauges` off), they are off (`sf_diag_bevy::set_gauges_live(false)`: one
//! run-condition check per marker system, the encode hook returns at once; the vendored encoder
//! timer still stamps each `finish`) and the shared `DiagCounters` is out of the main world, so
//! nothing that reports into it (scene, static-material and script gauges) runs. Shown, they run
//! as in a benchmark; the panel's own sampling drains ~100 counters a frame and the text is
//! rebuilt only at the refresh rate.

use std::collections::HashMap;
use std::time::Instant;

use bevy_app::prelude::*;
use bevy_color::prelude::*;
use bevy_diagnostic::DiagnosticsStore;
use bevy_ecs::prelude::*;
use bevy_light::{PointLight, SpotLight};
use bevy_render::diagnostic::RenderDiagnosticsPlugin;
use bevy_text::prelude::*;
use bevy_time::prelude::*;
use bevy_time::TimeSystems;
use bevy_ui::prelude::*;
use sf_diag_bevy::sf_diag::CounterKind;
use sf_diag_bevy::sf_diag::pacing::{MAIN_BUSY, RENDER_BUSY};
use sf_diag_bevy::{DiagCounters, DiagViews, FramePhasePlugin, RenderViewsPlugin};
use sf_tunables_bevy::sf_tunables::{Def, Lifetime, Source, Unit, def};
use sf_tunables_bevy::{AnyDef, Id, SetBy, Tunables, TunablesAppExt};

pub use crate::perf_panel_model::PanelMode;
use crate::perf_panel_model::{self as model, EncodeLine, Series, Snapshot, ViewsLine, Windows};

/// `diag.panel.refresh_s`: how often the text is rebuilt.
pub const REFRESH_S: Def<f32> = def!(float "diag.panel.refresh_s" = 0.25)
    .range(0.05, 5.0)
    .unit(Unit::Seconds)
    .help("Perf panel (F3): seconds between text refreshes (4 Hz keeps digits readable)")
    .source(Source::Guess);

/// `diag.panel.window_s`: the smoothing window.
pub const WINDOW_S: Def<f32> = def!(float "diag.panel.window_s" = 1.0)
    .range(0.1, 10.0)
    .unit(Unit::Seconds)
    .help("Perf panel (F3): every number is the mean over this many seconds; p99 and max of the frame too")
    .source(Source::Guess);

/// `diag.panel.bound_ratio`: when the busiest of main / render / GPU counts as the bound.
pub const BOUND_RATIO: Def<f32> = def!(float "diag.panel.bound_ratio" = 0.85)
    .range(0.1, 1.0)
    .unit(Unit::Ratio)
    .help("Perf panel (F3): the busiest of main, render and GPU is the bound when it takes at least this share of the frame; else 'none' (vsync, a cap, untimed GPU)")
    .source(Source::Guess);

/// `diag.panel.top_passes`: GPU passes listed in the full panel.
pub const TOP_PASSES: Def<u32> = def!(count "diag.panel.top_passes" = 6)
    .range(1, 32)
    .unit(Unit::Count)
    .help("Perf panel (F3, full): how many GPU passes to list, slowest first")
    .source(Source::Guess);

/// `diag.panel.max_tunables`: non-default tunables listed in the full panel.
pub const MAX_TUNABLES: Def<u32> = def!(count "diag.panel.max_tunables" = 24)
    .range(0, 256)
    .unit(Unit::Count)
    .help("Perf panel (F3, full): how many non-default tunables to list")
    .source(Source::Guess);

/// `diag.panel.gauges`: the frame-phase and view gauges while the panel is shown.
pub const GAUGES: Def<bool> = def!(bool "diag.panel.gauges" = true)
    .help("Perf panel (F3): main/render busy, stages, encoders and views while shown (sf_diag_bevy FramePhasePlugin + RenderViewsPlugin, as in benchmarks); always off while hidden; off: frame time only")
    .source(Source::Guess);

/// `diag.panel.gpu_timers`: Bevy's GPU pass timers in interactive runs. Plugins are added before
/// the command line applies (`--set` lands in `finish`), so this one is read at registration: its
/// default, or `BEVYRROWIND_PANEL_GPU_TIMERS=1`.
pub const GPU_TIMERS: Def<bool> = def!(bool "diag.panel.gpu_timers" = false)
    .lifetime(Lifetime::Restart)
    .env_alias("BEVYRROWIND_PANEL_GPU_TIMERS")
    .help("Perf panel (F3): add Bevy's RenderDiagnosticsPlugin for GPU pass times. Always on once added, shown or not (timestamp queries per pass, a resolve and a readback per frame; CW-5b: may cost submit time)")
    .source(Source::Guess);

/// This module's tunables.
pub const TUNABLES: &[&dyn AnyDef] = &[&REFRESH_S, &WINDOW_S, &BOUND_RATIO, &TOP_PASSES, &MAX_TUNABLES, &GAUGES, &GPU_TIMERS];

/// The panel's mode: F3, `flip perfpanel` (console, BRP). Off at start.
#[derive(Resource, Default, Debug)]
pub struct PerfPanel {
    pub mode: PanelMode,
}

/// Counter-name prefixes the panel keeps windows for (the rest is drained and dropped).
const KEPT: [&str; 6] = ["main/busy/", "render/busy/", "render/thread/", "stage/main/", "encode/", "views/"];

/// Frames dropped after the gauges go live: the render thread reports one frame behind, and the
/// first live frame folds the queue counts piled up while hidden.
// tunables: const (a pipeline depth, not a trade-off)
const WARMUP_FRAMES: u32 = 3;

#[derive(Resource)]
struct PanelState {
    ids: Ids,
    /// The gauges' counters, owned by the panel (in the main world only while live). `None`: the
    /// run already had them (a benchmark owns the gauges).
    owned: Option<DiagCounters>,
    /// The gauges run (shown, `diag.panel.gauges` on, and owned).
    live: bool,
    windows: Windows,
    frame: Series,
    gpu_seen: HashMap<String, Instant>,
    skip: u32,
    last_refresh: Option<f64>,
    /// Bevy's GPU pass timers run (ours or the GI's), known at `finish`.
    gpu_timers: bool,
}

#[derive(Clone, Copy)]
struct Ids {
    refresh: Id<f32>,
    window: Id<f32>,
    ratio: Id<f32>,
    top: Id<u32>,
    max_tunables: Id<u32>,
    gauges: Id<bool>,
}

#[derive(Component)]
struct PanelText;

/// Adds the panel (keys and console reach it through [`crate::live_ab`]).
pub struct PerfPanelPlugin;

impl Plugin for PerfPanelPlugin {
    fn build(&self, app: &mut App) {
        app.register_tunables(TUNABLES);
        let ids = Ids {
            refresh: app.add_tunable(&REFRESH_S).id(),
            window: app.add_tunable(&WINDOW_S).id(),
            ratio: app.add_tunable(&BOUND_RATIO).id(),
            top: app.add_tunable(&TOP_PASSES).id(),
            max_tunables: app.add_tunable(&MAX_TUNABLES).id(),
            gauges: app.add_tunable(&GAUGES).id(),
        };
        let gpu_id = app.add_tunable(&GPU_TIMERS).id();
        let gpu_timers = app.world().resource::<Tunables>().read(gpu_id);
        // A run that already has the shared counters (a benchmark) owns the gauges itself. Else the
        // gauge plugins are always added: gated off while hidden, they cost a run-condition check.
        if !app.world().contains_resource::<DiagCounters>() {
            if !app.is_plugin_added::<FramePhasePlugin>() {
                app.add_plugins(FramePhasePlugin);
            }
            if !app.is_plugin_added::<RenderViewsPlugin>() {
                app.add_plugins(RenderViewsPlugin);
            }
        }
        if gpu_timers && !app.is_plugin_added::<RenderDiagnosticsPlugin>() {
            app.add_plugins(RenderDiagnosticsPlugin);
        }
        app.init_resource::<PerfPanel>()
            .insert_resource(PanelState {
                ids,
                owned: None,
                live: false,
                windows: Windows::default(),
                frame: Series::default(),
                gpu_seen: HashMap::new(),
                skip: 0,
                last_refresh: None,
                gpu_timers: false,
            })
            .add_systems(First, sample.after(TimeSystems).run_if(shown))
            .add_systems(
                Update,
                (apply_mode.run_if(mode_or_gauges_changed), refresh.run_if(shown)).chain(),
            );
    }

    /// The gauge plugins (added in `build`, so they finish after this) take the counters here.
    fn finish(&self, app: &mut App) {
        let gpu_timers = app.is_plugin_added::<RenderDiagnosticsPlugin>();
        app.world_mut().resource_mut::<PanelState>().gpu_timers = gpu_timers;
        if app.world().contains_resource::<DiagCounters>() {
            return; // someone else's
        }
        let counters = DiagCounters::default();
        app.insert_resource(counters.clone());
        if !app.world().contains_resource::<DiagViews>() {
            app.insert_resource(DiagViews::default());
        }
        app.world_mut().resource_mut::<PanelState>().owned = Some(counters);
    }

    /// Hidden at start: the gauges off, the counters out of the main world.
    fn cleanup(&self, app: &mut App) {
        if app.world().resource::<PanelState>().owned.is_some() {
            app.world_mut().remove_resource::<DiagCounters>();
            sf_diag_bevy::set_gauges_live(false);
        }
    }
}

fn shown(panel: Res<PerfPanel>) -> bool {
    panel.mode != PanelMode::Off
}

/// Run condition of [`apply_mode`]: the mode or `diag.panel.gauges` changed (true on its first run).
fn mode_or_gauges_changed(panel: Res<PerfPanel>, tunables: Res<Tunables>, state: Res<PanelState>, mut seen: Local<Option<u64>>) -> bool {
    let generation = tunables.var(state.ids.gauges.untyped()).generation();
    let gauges = *seen != Some(generation);
    *seen = Some(generation);
    panel.is_changed() || gauges
}

/// Shows or hides the text node, and turns the gauges and the counters on or off with it.
fn apply_mode(mut commands: Commands, panel: Res<PerfPanel>, mut state: ResMut<PanelState>, tunables: Res<Tunables>, text: Query<Entity, With<PanelText>>) {
    let on = panel.mode != PanelMode::Off;
    state.last_refresh = None;
    let live = on && tunables.read(state.ids.gauges) && state.owned.is_some();
    if live != state.live {
        state.live = live;
        state.windows.clear();
        if let Some(counters) = state.owned.clone() {
            counters.clear();
            if live {
                state.skip = WARMUP_FRAMES;
                commands.insert_resource(counters);
            } else {
                commands.remove_resource::<DiagCounters>();
            }
            sf_diag_bevy::set_gauges_live(live);
        }
    }
    if on != text.is_empty() {
        return; // compact <-> full: the next refresh redraws
    }
    state.windows.clear();
    state.frame.clear();
    state.gpu_seen.clear();
    if on {
        state.skip = WARMUP_FRAMES;
        commands.spawn((
            PanelText,
            Text::new("perf panel: measuring..."),
            TextFont::from_font_size(13.0),
            TextColor(Color::srgb(0.85, 1.0, 0.85)),
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.6)),
            GlobalZIndex(1000),
            Node {
                position_type: PositionType::Absolute,
                top: Val::Px(6.0),
                right: Val::Px(6.0),
                padding: UiRect::all(Val::Px(6.0)),
                ..Node::default()
            },
        ));
    } else {
        for e in &text {
            commands.entity(e).despawn();
        }
    }
}

/// Each frame while shown: the last frame's time, the counters it reported, new GPU timings.
fn sample(real: Res<Time<Real>>, mut state: ResMut<PanelState>, tunables: Res<Tunables>, store: Option<Res<DiagnosticsStore>>) {
    let state = &mut *state;
    let window = f64::from(tunables.read(state.ids.window));
    let now = real.elapsed_secs_f64();
    if state.live
        && let Some(counters) = &state.owned
    {
        if state.skip > 0 {
            counters.clear();
        } else {
            let windows = &mut state.windows;
            counters.drain(|_, def, value| {
                if !KEPT.iter().any(|p| def.name.starts_with(p)) {
                    return;
                }
                let value = match (def.kind, value) {
                    (_, Some(v)) => v,
                    (CounterKind::Sum, None) => 0.0,
                    (CounterKind::Gauge, None) => return,
                };
                windows.push(&def.name, now, value, window);
            });
        }
    }
    if state.skip > 0 {
        state.skip -= 1;
        return;
    }
    let frame_ms = real.delta_secs_f64() * 1000.0;
    if frame_ms > 0.0 {
        state.frame.push(now, frame_ms, window);
    }
    let Some(store) = store else { return };
    for d in store.iter() {
        let path = d.path().as_str();
        if model::gpu_span(path).is_none() {
            continue;
        }
        let Some(m) = d.measurement() else { continue };
        match state.gpu_seen.get_mut(path) {
            Some(last) if *last >= m.time => continue,
            Some(last) => *last = m.time,
            None => {
                state.gpu_seen.insert(path.to_owned(), m.time);
            }
        }
        state.windows.push(path, now, m.value, window);
    }
}

/// At the refresh rate: the snapshot and the text.
#[allow(clippy::needless_pass_by_value, clippy::too_many_arguments)]
fn refresh(
    real: Res<Time<Real>>,
    panel: Res<PerfPanel>,
    mut state: ResMut<PanelState>,
    tunables: Res<Tunables>,
    entities: Query<()>,
    points: Query<&PointLight>,
    spots: Query<&SpotLight>,
    mut text: Query<&mut Text, With<PanelText>>,
) {
    let now = real.elapsed_secs_f64();
    let period = f64::from(tunables.read(state.ids.refresh));
    if state.last_refresh.is_some_and(|t| now - t < period) {
        return;
    }
    state.last_refresh = Some(now);
    let Ok(mut text) = text.single_mut() else { return };
    let mut snap = snapshot(&state, &tunables);
    let full = panel.mode == PanelMode::Full;
    if full {
        snap.entities = entities.count();
        let shadowed = points.iter().filter(|l| l.shadow_maps_enabled).count() + spots.iter().filter(|l| l.shadow_maps_enabled).count();
        snap.lights = (points.count() + spots.count(), shadowed);
    }
    text.0 = format!("perf ({}, F3)  {}", panel.mode.label(), model::format(&snap, full));
}

/// The numbers from the windows (lights, entities: the caller).
fn snapshot(state: &PanelState, tunables: &Tunables) -> Snapshot {
    let w = &state.windows;
    let gpu: Vec<(&str, f64)> = w.means(|n| model::gpu_span(n).is_some()).filter_map(|(n, v)| model::gpu_span(n).map(|s| (s, v))).collect();
    let gpu_ms = model::gpu_frame(gpu.iter().copied());
    let (main_ms, render_ms) = (w.mean(MAIN_BUSY), w.mean(RENDER_BUSY));
    let frame_ms = state.frame.mean();
    let top = usize::try_from(tunables.read(state.ids.top)).unwrap_or(usize::MAX);
    let is_main_stage = |n: &str| n.starts_with("stage/main/");
    let encoders: f64 = w.means(|n| n.starts_with("encode/") && !n.starts_with("encode/queue/") && n.ends_with("/count")).map(|(_, v)| v).sum();
    let encode = w.get("encode/submit").map(|_| EncodeLine {
        encoders,
        command_buffers: w.mean("encode/queue/command_buffers"),
        submits: w.mean("encode/queue/submits"),
        submit_ms: w.mean("encode/submit"),
        poll_ms: w.mean("encode/poll"),
    });
    let views = w.get("views/main/count").map(|_| ViewsLine {
        main: w.mean("views/main/count").unwrap_or_default(),
        cascades: w.mean("views/cascade/count").unwrap_or_default(),
        lamp_faces: w.mean("views/lamp_face/count").unwrap_or_default(),
        lamp_culled: w.mean("views/lamp_face/culled"),
        items: w.mean("views/all/items").unwrap_or_default(),
        batch_sets: w.mean("views/all/batch_sets").unwrap_or_default(),
    });
    let max_tunables = usize::try_from(tunables.read(state.ids.max_tunables)).unwrap_or(usize::MAX);
    let changed: Vec<(String, String, &'static str)> = tunables
        .iter()
        .filter(|(_, t)| t.set_by() > SetBy::Preset)
        .map(|(_, t)| (t.name().to_owned(), t.value().to_string(), t.set_by().describe()))
        .collect();
    let preset = preset_label(tunables);
    Snapshot {
        frame_ms,
        frame_p99: state.frame.percentile(99.0),
        frame_max: state.frame.max(),
        main_ms,
        render_ms,
        gpu_ms,
        bound: Some(model::bound_by(frame_ms, main_ms, render_ms, gpu_ms, f64::from(tunables.read(state.ids.ratio)))),
        cpu_note: match (state.live, state.owned.is_some()) {
            (true, _) => None,
            (false, true) => Some("main/render: diag.panel.gauges off"),
            (false, false) => Some("main/render: gauges owned by a benchmark"),
        },
        gpu_note: gpu.is_empty().then_some(if state.gpu_timers { "GPU: no timings yet (or no timestamp queries)" } else { "GPU: diag.panel.gpu_timers off" }),
        main_stages: model::top_n(w.means(is_main_stage).filter_map(|(n, v)| n.strip_prefix("stage/main/").map(|s| (s, v))), 3)
            .into_iter()
            .map(|(n, v)| (n.to_owned(), v))
            .collect(),
        gpu_passes: model::top_n(gpu, top).into_iter().map(|(n, v)| (n.to_owned(), v)).collect(),
        views,
        lights: (0, 0),
        encode,
        entities: 0,
        preset,
        tunables_more: changed.len().saturating_sub(max_tunables),
        tunables: changed.into_iter().take(max_tunables).collect(),
    }
}

/// The overall graphics preset's level (`sf.graphics_preset`); when it reads `custom`, the most common
/// group level and the `sg.*` groups that differ from it (`custom (high; shadows custom, post low)`).
pub(crate) fn preset_label(tunables: &Tunables) -> Option<String> {
    use sf_tunables_bevy::sf_tunables::presets::{CUSTOM, DRIVERS, GRAPHICS_PRESET};
    let level = |name: &str| tunables.id(name).map(|id| tunables.text(id).to_owned());
    let overall = level(GRAPHICS_PRESET.name())?;
    if overall != CUSTOM {
        return Some(overall);
    }
    let groups: Vec<(&str, String)> = DRIVERS[1..].iter().filter_map(|d| level(d.name()).map(|l| (d.name(), l))).collect();
    let mut counts: Vec<(&str, usize)> = Vec::new();
    for (_, l) in &groups {
        match counts.iter_mut().find(|(c, _)| c == l) {
            Some((_, n)) => *n += 1,
            None => counts.push((l.as_str(), 1)),
        }
    }
    let common = counts.iter().max_by_key(|(_, n)| *n).map_or(CUSTOM, |(l, _)| *l);
    let differ: Vec<String> =
        groups.iter().filter(|(_, l)| l != common).map(|(n, l)| format!("{} {l}", n.trim_start_matches("sg."))).collect();
    Some(format!("custom ({common}; {})", differ.join(", ")))
}

#[cfg(test)]
#[path = "perf_panel_tests.rs"]
mod tests;
