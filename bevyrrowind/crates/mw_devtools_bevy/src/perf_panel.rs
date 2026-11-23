//! The live perf panel (ROADMAP 1.25 CW-17, graphical since CW-17b; `OpenMW`'s F3 profiler, UE's
//! `stat unitgraph` + `stat unit` + `stat gpu`, RTSS / `PresentMon` frame-time graphs): F3 cycles
//! off / compact / full in every interactive run, release builds included (it rides on
//! [`crate::live_ab::add`]); console and BRP `flip perfpanel` ([`crate::live_ab::Switch::PerfPanel`]).
//! Wiki: `docs/wiki/engine/app-and-dev-tools.md` "Perf panel".
//!
//! Compact: the frame line (mean, median, p99, max), the bound, a frame-time graph of the last
//! `diag.panel.graph_frames` frames (each column the worst frame it covers, coloured by what bound
//! it; main / render / GPU lines; dashed target lines at `diag.panel.target_ms`), and main /
//! render / GPU bars against the frame. Full adds the top GPU passes and main stages as bars,
//! the counts with submit-time and view-count sparklines, the preset and the tunables above it.
//! Layout and tree: [`ui`]; pixels and rules: [`graph`] and [`crate::perf_panel_model`].
//!
//! It measures nothing of its own: it shows the gauges benchmarks already record, the numbers
//! smoothed over `diag.panel.window_s` and redrawn every `diag.panel.refresh_s`, the graph from
//! per-frame samples redrawn every `diag.panel.graph_refresh_s`:
//! - frame: `Time<Real>` delta (mean, median, p99, max over the window; per frame in the graph);
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
//! nothing that reports into it (scene, static-material and script gauges) runs; the panel's own
//! systems are skipped by a run condition, no node exists and no frame is recorded. Shown, the
//! gauges run as in a benchmark; the panel drains ~100 counters a frame, keeps one small sample per
//! frame, redraws the graph image (~400 x 110 texels at 1080p: a CPU fill and one `write_texture`,
//! Bevy reuses the texture while its size stays) at the graph rate and mutates ~80 UI nodes at the
//! refresh rate.

#[path = "perf_panel_graph.rs"]
pub mod graph;
#[path = "perf_panel_ui.rs"]
mod ui;

use std::collections::HashMap;
use std::time::Instant;

use bevy_app::prelude::*;
use bevy_diagnostic::DiagnosticsStore;
use bevy_ecs::prelude::*;
use bevy_light::{PointLight, SpotLight};
use bevy_render::diagnostic::RenderDiagnosticsPlugin;
use bevy_time::TimeSystems;
use bevy_time::prelude::*;
use bevy_ui::UiScale;
use bevy_window::{PrimaryWindow, Window};
use sf_diag_bevy::sf_diag::CounterKind;
use sf_diag_bevy::sf_diag::pacing::{MAIN_BUSY, RENDER_BUSY};
use sf_diag_bevy::{DiagCounters, DiagViews, FramePhasePlugin, RenderViewsPlugin};
use sf_tunables_bevy::sf_tunables::{Def, Lifetime, Source, Unit, def};
use sf_tunables_bevy::{AnyDef, Id, SetBy, Tunables, TunablesAppExt};

use self::graph::{FrameSample, GraphSpec, History, Palette, bar_fraction, draw_graph, draw_sparkline, rgba, series_columns};
use self::ui::{Look, PanelUi, UiWrite, View};
pub use crate::perf_panel_model::PanelMode;
use crate::perf_panel_model::{self as model, Bound, EncodeLine, Series, Snapshot, ViewsLine, Windows};

/// `diag.panel.refresh_s`: how often the numbers and bars are rewritten.
pub const REFRESH_S: Def<f32> = def!(float "diag.panel.refresh_s" = 0.25)
    .range(0.05, 5.0)
    .unit(Unit::Seconds)
    .help("Perf panel (F3): seconds between text and bar refreshes (4 Hz keeps digits readable)")
    .source(Source::Guess);

/// `diag.panel.window_s`: the smoothing window.
pub const WINDOW_S: Def<f32> = def!(float "diag.panel.window_s" = 1.0)
    .range(0.1, 10.0)
    .unit(Unit::Seconds)
    .help("Perf panel (F3): every number is the mean over this many seconds; median, p99 and max of the frame too")
    .source(Source::Guess);

/// `diag.panel.bound_ratio`: when the busiest of main / render / GPU counts as the bound.
pub const BOUND_RATIO: Def<f32> = def!(float "diag.panel.bound_ratio" = 0.85)
    .range(0.1, 1.0)
    .unit(Unit::Ratio)
    .help("Perf panel (F3): the busiest of main, render and GPU is the bound when it takes at least this share of the frame (the window's means, and each frame in the graph); else 'none' (vsync, a cap, untimed GPU)")
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

/// `diag.panel.graph_frames`: the frame graph's history.
pub const GRAPH_FRAMES: Def<u32> = def!(count "diag.panel.graph_frames" = 600)
    .range(30, 4000)
    .unit(Unit::Count)
    .help("Perf panel (F3): frames in the frame-time graph (600: 5 s at 120 fps, 10 s at 60); a column shows the worst frame it covers, so no hitch is lost on a narrower graph")
    .source(Source::Industry("RTSS / PresentMon overlays and UE stat unitgraph keep a few seconds of per-frame samples"));

/// `diag.panel.graph_refresh_s`: how often the graph image is redrawn.
pub const GRAPH_REFRESH_S: Def<f32> = def!(float "diag.panel.graph_refresh_s" = 0.05)
    .range(0.0, 1.0)
    .unit(Unit::Seconds)
    .help("Perf panel (F3): seconds between graph and sparkline redraws (0: every frame); each redraw fills the image on the CPU and uploads it")
    .source(Source::Guess);

/// `diag.panel.graph_top_ms`: the graph's scale.
pub const GRAPH_TOP_MS: Def<f32> = def!(float "diag.panel.graph_top_ms" = 25.0)
    .range(5.0, 1000.0)
    .unit(Unit::Millis)
    .help("Perf panel (F3): frame time at the top of the graph; slower frames are clipped and marked on top (the max shows their value)")
    .source(Source::Guess);

/// `diag.panel.target_ms`: the owner's frame target, drawn as the graph's dashed lines.
pub const TARGET_MS: Def<[f32; 2]> = def!(float2 "diag.panel.target_ms" = [8.3, 11.0])
    .range([1.0, 1.0], [1000.0, 1000.0])
    .unit(Unit::Millis)
    .help("Perf panel (F3): the graph's target lines: median target, p99 target")
    .source(Source::Owner("2026-11-15: <= 8.3 ms median, <= 11 ms p99 (High, 1440p + DLSS Quality, RTX 4060 laptop)"));

/// `diag.panel.graph_px`: the graph's size at 1080p.
pub const GRAPH_PX: Def<[f32; 2]> = def!(float2 "diag.panel.graph_px" = [400.0, 110.0])
    .range([160.0, 40.0], [2000.0, 800.0])
    .unit(Unit::Pixels)
    .help("Perf panel (F3): frame graph width, height at 1080p (the panel scales with the screen, diag.panel.scale)")
    .source(Source::Guess);

/// `diag.panel.scale`: the panel's size multiplier.
pub const SCALE: Def<f32> = def!(float "diag.panel.scale" = 0.0)
    .range(0.0, 4.0)
    .unit(Unit::Ratio)
    .help("Perf panel (F3): size multiplier; 0: auto (window height / 1080, at least diag.panel.min_scale). Bevy's UiScale multiplies it")
    .source(Source::Guess);

/// `diag.panel.min_scale`: the auto scale's floor.
pub const MIN_SCALE: Def<f32> = def!(float "diag.panel.min_scale" = 0.9)
    .range(0.25, 4.0)
    .unit(Unit::Ratio)
    .help("Perf panel (F3): smallest auto scale (keeps text readable at 720p)")
    .source(Source::Guess);

/// `diag.panel.alpha`: the panel's background opacity.
pub const ALPHA: Def<f32> = def!(float "diag.panel.alpha" = 0.8)
    .range(0.0, 1.0)
    .unit(Unit::Ratio)
    .help("Perf panel (F3): background opacity")
    .source(Source::Guess);

/// Declares a panel colour (sRGB 0..1).
macro_rules! colour_def {
    ($name:literal = $rgb:expr, $help:literal) => {
        def!(float3 $name = $rgb).range([0.0, 0.0, 0.0], [1.0, 1.0, 1.0]).help($help).source(Source::Guess)
    };
}

/// The panel's colours: lines and bars, the frame columns by bound, the target lines, the clip mark.
pub const COLOUR_MAIN: Def<[f32; 3]> = colour_def!("diag.panel.colour.main" = [1.0, 0.62, 0.22], "Perf panel (F3): main thread colour (sRGB)");
pub const COLOUR_RENDER: Def<[f32; 3]> = colour_def!("diag.panel.colour.render" = [0.36, 0.68, 1.0], "Perf panel (F3): render thread colour (sRGB)");
pub const COLOUR_GPU: Def<[f32; 3]> = colour_def!("diag.panel.colour.gpu" = [0.42, 0.9, 0.38], "Perf panel (F3): GPU colour (sRGB)");
pub const COLOUR_WAIT: Def<[f32; 3]> = colour_def!("diag.panel.colour.none" = [0.5, 0.52, 0.56], "Perf panel (F3): colour of frames bound by none (vsync, a cap, untimed GPU) (sRGB)");
pub const COLOUR_TARGET: Def<[f32; 3]> = colour_def!("diag.panel.colour.target" = [0.92, 0.92, 0.92], "Perf panel (F3): median target line (sRGB)");
pub const COLOUR_TARGET_P99: Def<[f32; 3]> = colour_def!("diag.panel.colour.target_p99" = [1.0, 0.36, 0.36], "Perf panel (F3): p99 target line (sRGB)");
pub const COLOUR_CLIP: Def<[f32; 3]> = colour_def!("diag.panel.colour.clip" = [1.0, 0.25, 0.85], "Perf panel (F3): mark on frames above the graph's top (sRGB)");

/// This module's tunables.
pub const TUNABLES: &[&dyn AnyDef] = &[
    &REFRESH_S,
    &WINDOW_S,
    &BOUND_RATIO,
    &TOP_PASSES,
    &MAX_TUNABLES,
    &GAUGES,
    &GPU_TIMERS,
    &GRAPH_FRAMES,
    &GRAPH_REFRESH_S,
    &GRAPH_TOP_MS,
    &TARGET_MS,
    &GRAPH_PX,
    &SCALE,
    &MIN_SCALE,
    &ALPHA,
    &COLOUR_MAIN,
    &COLOUR_RENDER,
    &COLOUR_GPU,
    &COLOUR_WAIT,
    &COLOUR_TARGET,
    &COLOUR_TARGET_P99,
    &COLOUR_CLIP,
];

/// The panel's mode: F3, `flip perfpanel` (console, BRP). Off at start.
#[derive(Resource, Default, Debug)]
pub struct PerfPanel {
    pub mode: PanelMode,
}

/// Another tool wants the panel's per-frame samples while it is hidden (the flip delta probe,
/// [`crate::switch_probe`]): the gauges run and [`PerfFrameFeed`] fills, with no tree and no
/// windows or graph kept. Off: a hidden panel costs nothing, as before.
#[derive(Resource, Default, Debug, PartialEq, Eq)]
pub struct PerfGaugeDemand(pub bool);

/// The last frame's sample, written each frame the panel samples (shown, or [`PerfGaugeDemand`]);
/// `None` on warm-up frames. A reader `take`s it, so a stale sample is never read twice.
#[derive(Resource, Default, Debug)]
pub struct PerfFrameFeed(pub Option<FrameSample>);

/// Counter-name prefixes the panel keeps windows for (the rest is drained and dropped).
const KEPT: [&str; 6] = ["main/busy/", "render/busy/", "render/thread/", "stage/main/", "encode/", "views/"];

/// The views counted in the full panel's sparkline.
const VIEW_COUNTS: [&str; 3] = ["views/main/count", "views/cascade/count", "views/lamp_face/count"];

/// Frames dropped after the gauges go live: the render thread reports one frame behind, and the
/// first live frame folds the queue counts piled up while hidden.
// tunables: const (a pipeline depth, not a trade-off)
const WARMUP_FRAMES: u32 = 3;

/// A GPU span's last timing: when Bevy measured it, the value (ms), when the panel saw it (s).
type GpuSeen = (Instant, f64, f64);

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
    /// Per-frame samples for the graph (recorded only while shown).
    history: History,
    gpu_seen: HashMap<String, GpuSeen>,
    skip: u32,
    last_refresh: Option<f64>,
    last_graph: Option<f64>,
    /// The sparklines' largest values (submit ms, views), from the last graph redraw.
    spark_max: [Option<f32>; 2],
    /// Bevy's GPU pass timers run (ours or the GI's), known at `finish`.
    gpu_timers: bool,
    ui: Option<PanelUi>,
}

#[derive(Clone, Copy)]
struct Ids {
    refresh: Id<f32>,
    window: Id<f32>,
    ratio: Id<f32>,
    top: Id<u32>,
    max_tunables: Id<u32>,
    gauges: Id<bool>,
    graph_frames: Id<u32>,
    graph_refresh: Id<f32>,
    graph_top: Id<f32>,
    targets: Id<[f32; 2]>,
    graph_px: Id<[f32; 2]>,
    scale: Id<f32>,
    min_scale: Id<f32>,
    alpha: Id<f32>,
    colours: [Id<[f32; 3]>; 7],
}

/// Marks the panel's root node.
#[derive(Component)]
struct PanelRoot;

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
            graph_frames: app.add_tunable(&GRAPH_FRAMES).id(),
            graph_refresh: app.add_tunable(&GRAPH_REFRESH_S).id(),
            graph_top: app.add_tunable(&GRAPH_TOP_MS).id(),
            targets: app.add_tunable(&TARGET_MS).id(),
            graph_px: app.add_tunable(&GRAPH_PX).id(),
            scale: app.add_tunable(&SCALE).id(),
            min_scale: app.add_tunable(&MIN_SCALE).id(),
            alpha: app.add_tunable(&ALPHA).id(),
            colours: [&COLOUR_MAIN, &COLOUR_RENDER, &COLOUR_GPU, &COLOUR_WAIT, &COLOUR_TARGET, &COLOUR_TARGET_P99, &COLOUR_CLIP]
                .map(|d| app.add_tunable(d).id()),
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
            .init_resource::<PerfGaugeDemand>()
            .init_resource::<PerfFrameFeed>()
            .insert_resource(PanelState {
                ids,
                owned: None,
                live: false,
                windows: Windows::default(),
                frame: Series::default(),
                history: History::default(),
                gpu_seen: HashMap::new(),
                skip: 0,
                last_refresh: None,
                last_graph: None,
                spark_max: [None; 2],
                gpu_timers: false,
                ui: None,
            })
            .add_systems(First, sample.after(TimeSystems).run_if(sampling))
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

/// Run condition of [`sample`]: shown, or the gauges run for [`PerfGaugeDemand`].
fn sampling(panel: Res<PerfPanel>, state: Res<PanelState>) -> bool {
    panel.mode != PanelMode::Off || state.live
}

/// Run condition of [`apply_mode`]: the mode or `diag.panel.gauges` changed (true on its first run).
fn mode_or_gauges_changed(
    panel: Res<PerfPanel>,
    demand: Res<PerfGaugeDemand>,
    tunables: Res<Tunables>,
    state: Res<PanelState>,
    mut seen: Local<Option<u64>>,
) -> bool {
    let generation = tunables.var(state.ids.gauges.untyped()).generation();
    let gauges = *seen != Some(generation);
    *seen = Some(generation);
    panel.is_changed() || demand.is_changed() || gauges
}

/// Turns the gauges and the counters on or off with the panel; hiding despawns the tree and drops
/// every sample (the next show builds it, [`refresh`]).
fn apply_mode(mut commands: Commands, panel: Res<PerfPanel>, demand: Res<PerfGaugeDemand>, mut state: ResMut<PanelState>, tunables: Res<Tunables>) {
    let on = panel.mode != PanelMode::Off;
    state.last_refresh = None;
    let live = (on || demand.0) && tunables.read(state.ids.gauges) && state.owned.is_some();
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
    let was_on = state.ui.is_some();
    if on == was_on {
        return; // compact <-> full: the look changed, refresh rebuilds
    }
    state.windows.clear();
    state.frame.clear();
    state.history.clear();
    state.gpu_seen.clear();
    state.spark_max = [None; 2];
    if on {
        state.skip = WARMUP_FRAMES;
    } else if let Some(ui) = state.ui.take() {
        commands.entity(ui.root).despawn();
    }
}

/// Each frame while shown: the last frame's time, the counters it reported, new GPU timings; one
/// [`FrameSample`] for the graph and [`PerfFrameFeed`]. Hidden with [`PerfGaugeDemand`]: only the
/// sample (no windows, no graph).
#[allow(clippy::cast_possible_truncation, reason = "ms values to f32 for the graph")]
fn sample(
    real: Res<Time<Real>>,
    panel: Res<PerfPanel>,
    mut state: ResMut<PanelState>,
    mut feed: ResMut<PerfFrameFeed>,
    tunables: Res<Tunables>,
    store: Option<Res<DiagnosticsStore>>,
) {
    let shown = panel.mode != PanelMode::Off;
    feed.0 = None;
    let state = &mut *state;
    let window = f64::from(tunables.read(state.ids.window));
    let capacity = usize::try_from(tunables.read(state.ids.graph_frames)).unwrap_or(usize::MAX);
    if state.history.capacity() != capacity {
        state.history.set_capacity(capacity);
    }
    let now = real.elapsed_secs_f64();
    let mut frame = FrameSample::default();
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
                let name = def.name.as_str();
                match name {
                    MAIN_BUSY => frame.main_ms = Some(value as f32),
                    RENDER_BUSY => frame.render_ms = Some(value as f32),
                    "encode/submit" => frame.submit_ms = Some(value as f32),
                    _ if VIEW_COUNTS.contains(&name) => frame.views = Some(frame.views.unwrap_or(0.0) + value as f32),
                    _ => {}
                }
                if shown {
                    windows.push(name, now, value, window);
                }
            });
        }
    }
    if state.skip > 0 {
        state.skip -= 1;
        return;
    }
    let frame_ms = real.delta_secs_f64() * 1000.0;
    if frame_ms > 0.0 && shown {
        state.frame.push(now, frame_ms, window);
    }
    if let Some(store) = store {
        for d in store.iter() {
            let path = d.path().as_str();
            if model::gpu_span(path).is_none() {
                continue;
            }
            let Some(m) = d.measurement() else { continue };
            match state.gpu_seen.get_mut(path) {
                Some((last, _, _)) if *last >= m.time => continue,
                Some(seen) => *seen = (m.time, m.value, now),
                None => {
                    state.gpu_seen.insert(path.to_owned(), (m.time, m.value, now));
                }
            }
            if shown {
                state.windows.push(path, now, m.value, window);
            }
        }
        // The GPU frame of the latest timings (spans not timed for a window are gone).
        let recent = state.gpu_seen.iter().filter(|(_, (_, _, at))| now - at <= window);
        frame.gpu_ms = model::gpu_frame(recent.filter_map(|(p, (_, v, _))| model::gpu_span(p).map(|s| (s, *v)))).map(|v| v as f32);
    }
    if frame_ms > 0.0 {
        frame.frame_ms = frame_ms as f32;
        feed.0 = Some(frame);
        if shown {
            state.history.push(frame);
        }
    }
}

/// What the tree should look like now.
#[allow(clippy::cast_possible_truncation, clippy::cast_precision_loss, reason = "small counts")]
fn look(mode: PanelMode, tunables: &Tunables, ids: &Ids, window: Option<&Window>, ui_scale: f32) -> Look {
    let (scale, px_scale) = graph::panel_scale(
        tunables.read(ids.scale),
        tunables.read(ids.min_scale),
        window.map_or(1080, Window::physical_height),
        window.map_or(1.0, Window::scale_factor),
        ui_scale,
    );
    let [main, render, gpu, wait, target, target_p99, clip] = ids.colours.map(|id| rgba(tunables.read(id), 1.0));
    Look {
        mode,
        scale,
        px_scale,
        graph: tunables.read(ids.graph_px),
        top_ms: tunables.read(ids.graph_top),
        targets: tunables.read(ids.targets),
        palette: Palette { background: rgba([0.05, 0.055, 0.065], 0.85), main, render, gpu, wait, target, target_p99, clip },
        panel_alpha: tunables.read(ids.alpha),
        passes: usize::try_from(tunables.read(ids.top)).unwrap_or(usize::MAX),
    }
}

/// Every frame while shown: (re)builds the tree when its look changed, redraws the graph at the
/// graph rate, rewrites numbers and bars at the refresh rate.
#[allow(clippy::needless_pass_by_value, clippy::too_many_arguments)]
fn refresh(
    mut commands: Commands,
    real: Res<Time<Real>>,
    panel: Res<PerfPanel>,
    mut state: ResMut<PanelState>,
    tunables: Res<Tunables>,
    window: Query<&Window, With<PrimaryWindow>>,
    ui_scale: Option<Res<UiScale>>,
    world: (Query<()>, Query<&PointLight>, Query<&SpotLight>),
    mut w: UiWrite,
) {
    let state = &mut *state;
    let now = real.elapsed_secs_f64();
    let look = look(panel.mode, &tunables, &state.ids, window.single().ok(), ui_scale.map_or(1.0, |s| s.0));
    if state.ui.as_ref().is_none_or(|ui| ui.look != look) {
        if let Some(old) = state.ui.take() {
            commands.entity(old.root).despawn();
        }
        let ui = ui::spawn(&mut commands, w.images.as_deref_mut(), look);
        commands.entity(ui.root).insert(PanelRoot);
        state.ui = Some(ui);
        state.last_refresh = None;
        state.last_graph = None;
        return; // the nodes exist from the next frame
    }
    let Some(ui) = state.ui.as_ref() else { return };
    let ratio = tunables.read(state.ids.ratio);
    let graph_period = f64::from(tunables.read(state.ids.graph_refresh));
    if state.last_graph.is_none_or(|t| now - t >= graph_period) {
        state.last_graph = Some(now);
        if let Some(images) = w.images.as_deref_mut() {
            redraw(ui, &state.history, ratio, images, &mut state.spark_max);
        }
    }
    let period = f64::from(tunables.read(state.ids.refresh));
    if state.last_refresh.is_some_and(|t| now - t < period) {
        return;
    }
    state.last_refresh = Some(now);
    let mut snap = snapshot(state, &tunables);
    if panel.mode == PanelMode::Full {
        let (entities, points, spots) = &world;
        snap.entities = entities.count();
        let shadowed = points.iter().filter(|l| l.shadow_maps_enabled).count() + spots.iter().filter(|l| l.shadow_maps_enabled).count();
        snap.lights = (points.count() + spots.count(), shadowed);
    }
    let passes = bars(&snap.gpu_passes, snap.frame_ms);
    let stages = bars(&snap.main_stages, snap.frame_ms);
    let view = view(&snap, &state.history, state.spark_max, &look.palette, &passes, &stages);
    ui::update(ui, &mut w, &view);
}

/// Redraws the graph and the sparklines into their images.
fn redraw(ui: &PanelUi, history: &History, ratio: f32, images: &mut bevy_asset::Assets<bevy_image::Image>, spark_max: &mut [Option<f32>; 2]) {
    let look = &ui.look;
    if let Some(handle) = &ui.graph
        && let Some(mut image) = images.get_mut(handle)
    {
        let spec = GraphSpec { width: ui.graph_px[0], height: ui.graph_px[1], top_ms: look.top_ms, targets: look.targets, ratio };
        draw_graph(history, &spec, &look.palette, image.data.get_or_insert_with(Vec::new));
    }
    let picks: [fn(&FrameSample) -> Option<f32>; 2] = [|s| s.submit_ms, |s| s.views];
    for ((spark, pick), (max, colour)) in ui.sparks.iter().zip(picks).zip(spark_max.iter_mut().zip([look.palette.render, look.palette.main])) {
        let Some(handle) = &spark.image else { continue };
        let Some(mut image) = images.get_mut(handle) else { continue };
        let values = series_columns(history, spark.size[0] as usize, pick);
        *max = draw_sparkline(&values, spark.size[0], spark.size[1], colour, look.palette.background, image.data.get_or_insert_with(Vec::new));
    }
}

/// `(ms, share of the frame, name)` rows for the bar lists.
fn bars(items: &[(String, f64)], frame: Option<f64>) -> Vec<(f64, f32, String)> {
    items.iter().map(|(n, v)| (*v, bar_fraction(Some(*v), frame), n.clone())).collect()
}

/// The tree's content from a snapshot.
fn view<'a>(s: &Snapshot, history: &History, spark_max: [Option<f32>; 2], p: &Palette, passes: &'a [(f64, f32, String)], stages: &'a [(f64, f32, String)]) -> View<'a> {
    let bound = s.bound.unwrap_or(Bound::Unknown);
    let thread = |v: Option<f64>, me: Bound| {
        let is = bound == me;
        let text = match (v, s.frame_ms) {
            (Some(v), Some(f)) if f > 0.0 => format!("{v:6.2} ms {:4.0}%{}", v / f * 100.0, if is { "  < bound" } else { "" }),
            (Some(v), _) => format!("{v:6.2} ms"),
            (None, _) => "  n/a".to_owned(),
        };
        (bar_fraction(v, s.frame_ms), text, is)
    };
    let max = |m: Option<f32>, digits: usize| m.map_or_else(String::new, |m| format!(" (max {m:.digits$})"));
    let submit = s.encode.and_then(|e: EncodeLine| e.submit_ms).map_or_else(|| "n/a".to_owned(), |v| format!("{v:.2} ms{}", max(spark_max[0], 2)));
    let views = s.views.map_or_else(|| "n/a".to_owned(), |v: ViewsLine| format!("{:.0}{}", v.main + v.cascades + v.lamp_faces, max(spark_max[1], 0)));
    View {
        header: model::frame_line(s),
        bound: format!("bound by: {}", s.bound.map_or("n/a", Bound::label)),
        bound_colour: ui::colour(p.bound(bound)),
        span: format!("last {:.1} s, {} frames", history.span_s(), history.len()),
        threads: [thread(s.main_ms, Bound::Main), thread(s.render_ms, Bound::Render), thread(s.gpu_ms, Bound::Gpu)],
        notes: model::notes(s),
        passes,
        stages,
        counts: model::counts_text(s),
        sparks: [submit, views],
        footer: model::footer_text(s),
    }
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
        frame_median: state.frame.percentile(50.0),
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
        main_stages: model::top_n(w.means(is_main_stage).filter_map(|(n, v)| n.strip_prefix("stage/main/").map(|s| (s, v))), ui::STAGES)
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
