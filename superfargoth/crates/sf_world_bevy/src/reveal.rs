//! [`RevealGate`]: the one readiness contract a reveal waits on (pipeline-warmup.md §5.4, W1).
//!
//! A *reveal* is new content becoming visible after a cover: the door fade lifting
//! ([`crate::FadeTransition`]), or the app's `Loading` → `InGame`. Nothing is drawn late on
//! purpose: the reveal waits until, in order,
//! - **cells**: the client's streaming readiness (the fade: arrival cells spawned; loading:
//!   the start area resident);
//! - **textures**: every mesh within [`RevealGate::radius_m`] of the focus has its
//!   `StandardMaterial` textures loaded on the CPU side (or failed: they never will be);
//! - **assets**: resident, shown meshes within the radius whose mesh or material (with its
//!   textures) is not prepared in the render world, plus visible meshes with a material but no
//!   pipeline yet ([`sf_diag_bevy::PendingSample::near_unprepared`] and `unprepared`);
//! - **pipelines**: visible main-pass, shadow-view and prepass draws whose pipeline is queued or
//!   compiling, plus any pipeline still compiling (`PipelineCache::waiting_pipelines`, which
//!   covers the non-material ones), plus the shown set's pending specimens
//!   ([`crate::ResidentWarmup::pending`], W4b3: the resident meshes within the radius, requested
//!   under [`crate::RESIDENT_TAG`]), plus the readiness holds ([`RevealHolds`], W4b1-fix: any crate or game names a source not
//!   ready yet, e.g. the GI volume before its first trace, whose arrival changes the view key and
//!   re-specialises every material). The catalog ([`crate::CatalogWarmup`]) does not hold a
//!   reveal (W4b3, UE5 PSO precaching: the loading screen waits for the shown components' PSOs,
//!   the bundled precache keeps compiling in the background): it is a background precache tag,
//!   admitted after the resident set's and paused once only compiles are left, so
//!   `waiting_pipelines()` drains; it shows in the cap warning and the `warmup: reveal` line;
//! - **gates**: external holds ([`crate::FadeTransition::hold`]: lamp shadow captures, the GI
//!   volume, ground collision) for at most [`RevealGate::gates_max_secs`] after cells and
//!   textures, as before W1; the last frames settling ([`SETTLE_FRAMES`]) count here too.
//!
//! The render-world terms arrive one frame late through [`sf_diag_bevy::PendingDraws`]
//! (`sf_diag_bevy::PendingDrawsPlugin`, public render resources only); while a hold runs the gate
//! sets a [`sf_diag_bevy::RevealProbe`] (focus and radius) so the render world counts the near
//! meshes and the shadow views. All terms must stay clear for [`SETTLE_FRAMES`] frames.
//!
//! Caps (a broken shader or a texture that never uploads must not hang the game): the textures,
//! assets, pipelines and gates terms get [`RevealGate::max_secs`] (10 s, owner 2026-11-11) after the
//! cells; a client may give up earlier (the fade's whole-hold `max_hold_secs`). Hitting a cap
//! logs a `warn!` naming what is pending (terms, counts, a few mesh names), never silently.
//! Industry practice: Unreal's loading screen waits for streaming and the PSO precache
//! (`FShaderPipelineCache` / `PSOPrecache`) before it lifts; Bevy's `loading_screen` example
//! waits on `PipelineCache`. [`RevealGate::progress`] is a 0..1 value a loading UI can show.
//!
//! One hold at a time: a client calls [`RevealCheck::step`] every frame of its cover; a hold no
//! one stepped this frame is dropped in `Last` (and the probe cleared).

use bevy_camera::{Camera, Camera3d};
use bevy_ecs::name::Name;
use bevy_ecs::prelude::*;
use bevy_ecs::system::SystemParam;
use bevy_time::{Real, Time};
use sf_diag_bevy::{PendingDraws, PendingSample, Reveal, RevealProbe, RevealTerm};

use crate::catalog_warm::CatalogWarmup;
use crate::fade::NearTextures;
use crate::precache::Precache;
use crate::resident_warm::ResidentWarmup;

/// Frames every term must stay clear before the reveal (the render world's sample is a frame
/// late, and content spawned by the last CPU step needs a frame to be extracted).
pub const SETTLE_FRAMES: u32 = 3;

/// Term names, in the order the hold attributes its time.
const TERMS: [&str; 5] = ["cells", "textures", "assets", "pipelines", "gates"];

/// The reveal gate's settings and the hold in progress (see the module docs).
#[derive(Resource, Debug)]
pub struct RevealGate {
    /// Longest the textures, assets, pipelines and gates terms hold after the cells, seconds.
    pub max_secs: f32,
    /// Longest an external gate holds after cells and textures, seconds.
    pub gates_max_secs: f32,
    /// Meshes this close to the focus must have their textures, mesh and material, meters.
    pub radius_m: f32,
    hold: Option<Hold>,
    stepped: bool,
    last: Option<Reveal>,
    /// The hold's focus (world space), as its client last saw it.
    focus: Option<[f32; 3]>,
}

impl Default for RevealGate {
    fn default() -> Self {
        Self { max_secs: 10.0, gates_max_secs: 1.5, radius_m: 48.0, hold: None, stepped: false, last: None, focus: None }
    }
}

#[derive(Debug, Clone)]
struct Hold {
    kind: &'static str,
    place: String,
    waited: f32,
    terms: [f32; 5],
    cells: bool,
    textures: bool,
    /// Seconds since cells and textures were ready.
    since_cpu: Option<f32>,
    /// Seconds since the cells were ready (the cap's clock).
    since_cells: Option<f32>,
    clear_frames: u32,
    peaks: RevealPending,
    progress: f32,
}

/// One frame's readiness, as the client and the render world saw it.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
#[allow(clippy::struct_excessive_bools)] // one independent flag per term
pub struct RevealInputs {
    /// The client's streaming readiness (sticky: once true the hold remembers it).
    pub cells: bool,
    /// Textures within the radius loaded (CPU side; sticky, counted only once `cells` is).
    pub textures: bool,
    /// The render world's latest sample.
    pub sample: PendingSample,
    /// An external gate asked to stay covered this frame.
    pub gated: bool,
    /// The client's own cap was hit: reveal now (capped `max_hold`).
    pub give_up: bool,
    /// The shown set's specimens still pending ([`ResidentWarmup::pending`]: textures loading,
    /// then the [`crate::RESIDENT_TAG`] assets, queued and compiling); part of the pipelines term.
    pub resident: usize,
    /// Catalog specimens still pending (`CatalogWarmup::pending`), for the log only: the catalog
    /// compiles in the background and never holds a reveal (W4b3).
    pub catalog: usize,
    /// Readiness holds pending ([`RevealHolds::count`]); part of the pipelines term.
    pub holds: usize,
}

impl RevealInputs {
    /// Only compiles are left (cells, textures, assets, visible and shadow draws, the shown set
    /// and the holds all clear): the gate then pauses background precache tags so the compiles
    /// in flight (`waiting_pipelines()`, which counts the catalog's too) drain (W4b3).
    #[must_use]
    pub fn drains_background(&self) -> bool {
        let s = &self.sample;
        self.cells && self.textures && s.near_unprepared + s.unprepared + s.skipped + s.skipped_shadow + self.resident + self.holds == 0
    }
}

/// Readiness holds (W4b1-fix): named sources any crate or game feeds, each pending or not, that
/// keep a reveal covered (pipelines term, under [`RevealGate::max_secs`]; the cap warning names
/// them). For work whose completion changes what the first frames draw, such as the GI volume's
/// first full trace (it adds a view key bit, so every visible material re-specialises): unlike
/// [`crate::FadeTransition::hold`] (fade only, capped at [`RevealGate::gates_max_secs`]) it holds
/// both reveals, the fade and `Loading`. A source sets its state when it changes (persistent, no
/// per-frame order with the gate's clients); a source that never reports never holds.
#[derive(Resource, Debug, Default, Clone)]
pub struct RevealHolds {
    holds: Vec<(&'static str, bool)>,
}

impl RevealHolds {
    /// Sets `name` pending (holds every reveal) or ready.
    pub fn set(&mut self, name: &'static str, pending: bool) {
        match self.holds.iter_mut().find(|(n, _)| *n == name) {
            Some((_, p)) => *p = pending,
            None => self.holds.push((name, pending)),
        }
    }

    /// Whether `name` is pending.
    #[must_use]
    pub fn is_pending(&self, name: &str) -> bool {
        self.holds.iter().any(|&(n, p)| p && n == name)
    }

    /// The pending sources' names.
    pub fn pending(&self) -> impl Iterator<Item = &'static str> + '_ {
        self.holds.iter().filter(|(_, p)| *p).map(|&(n, _)| n)
    }

    /// How many sources are pending.
    #[must_use]
    pub fn count(&self) -> usize {
        self.pending().count()
    }
}

/// The render-world work a reveal waits for.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct RevealPending {
    /// Near meshes not prepared + visible meshes with no pipeline yet.
    pub assets: usize,
    /// Main, shadow and prepass draws waiting for a pipeline + pipelines compiling + the shown
    /// set's specimens pending + readiness holds pending.
    pub pipelines: usize,
}

impl RevealPending {
    pub fn of(s: &PendingSample, resident: usize, holds: usize) -> Self {
        Self { assets: s.near_unprepared + s.unprepared, pipelines: s.skipped + s.skipped_shadow + s.waiting + resident + holds }
    }

    pub fn is_clear(&self) -> bool {
        self.assets == 0 && self.pipelines == 0
    }
}

/// What one [`RevealGate::step`] decided.
#[derive(Clone, Debug, PartialEq)]
pub enum RevealStep {
    Holding,
    /// Revealed this frame; `warning` names what was still pending when a cap ended the hold.
    Open { reveal: Reveal, warning: Option<String> },
}

impl RevealGate {
    /// A hold is in progress.
    pub fn is_holding(&self) -> bool {
        self.hold.is_some()
    }

    /// The hold's progress, 0 to 1 (monotonic; `None` when no hold runs): cells and textures a
    /// quarter each, then the share of the peak assets and pipelines prepared since.
    pub fn progress(&self) -> Option<f32> {
        self.hold.as_ref().map(|h| h.progress)
    }

    /// The last reveal this gate opened.
    pub fn last_reveal(&self) -> Option<&Reveal> {
        self.last.as_ref()
    }

    /// Cells and textures are ready in the hold in progress.
    pub fn cpu_ready(&self) -> bool {
        self.hold.as_ref().is_some_and(|h| h.cells && h.textures)
    }

    /// One frame of a hold (begins one if none runs). Pure: the caller passes the frame's `dt`
    /// and readiness ([`RevealCheck::step`] gathers them).
    #[allow(clippy::cast_precision_loss)] // counts
    pub fn step(&mut self, kind: &'static str, place: &str, dt: f32, inputs: &RevealInputs) -> RevealStep {
        self.stepped = true;
        let hold = self.hold.get_or_insert_with(|| Hold {
            kind,
            place: place.to_owned(),
            waited: 0.0,
            terms: [0.0; 5],
            cells: false,
            textures: false,
            since_cpu: None,
            since_cells: None,
            clear_frames: 0,
            peaks: RevealPending::default(),
            progress: 0.0,
        });
        hold.waited += dt;
        hold.cells |= inputs.cells;
        hold.textures |= hold.cells && inputs.textures;
        let cpu = hold.cells && hold.textures;
        let pending = RevealPending::of(&inputs.sample, inputs.resident, inputs.holds);
        if cpu {
            hold.since_cpu = Some(hold.since_cpu.map_or(0.0, |s| s + dt));
        }
        let since = hold.since_cpu.unwrap_or(0.0);
        if hold.cells {
            hold.since_cells = Some(hold.since_cells.map_or(0.0, |s| s + dt));
        }
        let since_cells = hold.since_cells.unwrap_or(0.0);
        let clear = cpu && pending.is_clear();
        hold.clear_frames = if clear { hold.clear_frames + 1 } else { 0 };
        let term = if !hold.cells {
            0
        } else if !hold.textures {
            1
        } else if pending.assets > 0 {
            2
        } else if pending.pipelines > 0 {
            3
        } else {
            4
        };
        hold.terms[term] += dt;
        if cpu {
            hold.peaks.assets = hold.peaks.assets.max(pending.assets);
            hold.peaks.pipelines = hold.peaks.pipelines.max(pending.pipelines);
        }
        let share = |n: usize, peak: usize| if peak == 0 { 1.0 } else { 1.0 - n as f32 / peak as f32 };
        let progress = if cpu {
            0.5 + 0.25 * (share(pending.assets, hold.peaks.assets) + share(pending.pipelines, hold.peaks.pipelines))
        } else {
            0.25 * f32::from(u8::from(hold.cells))
        };
        hold.progress = hold.progress.max(progress);

        let gate_held = inputs.gated && since < self.gates_max_secs;
        let ready = clear && hold.clear_frames >= SETTLE_FRAMES && !gate_held;
        let capped = if ready {
            inputs.gated.then_some("gate")
        } else if inputs.give_up {
            Some("max_hold")
        } else if hold.cells && since_cells >= self.max_secs {
            Some("reveal_cap")
        } else {
            return RevealStep::Holding;
        };
        let hold = self.hold.take().expect("held");
        self.focus = None;
        let warning = match capped {
            Some("gate") => Some(format!("a gate (shadow captures, GI, collision, ...) held past {:.1} s", self.gates_max_secs)),
            Some(cap) => Some(format!("not ready after {:.1} s ({cap}): pending {}", hold.waited, pending_text(&hold, inputs))),
            None => None,
        };
        let reveal = Reveal {
            kind: hold.kind,
            place: hold.place,
            held_secs: hold.waited,
            terms: TERMS.iter().zip(hold.terms).map(|(name, secs)| RevealTerm { name, secs: Some(secs) }).collect(),
            capped,
        };
        self.last = Some(reveal.clone());
        RevealStep::Open { reveal, warning }
    }

    /// Drops a hold no client stepped since the last call (an abandoned cover); returns whether it
    /// did. Runs in `Last`.
    fn end_frame(&mut self) -> bool {
        let abandoned = !std::mem::take(&mut self.stepped) && self.hold.is_some();
        if abandoned {
            self.hold = None;
            self.focus = None;
        }
        abandoned
    }

    /// The focus of the hold in progress (world space, as its client last saw it); `None` when no
    /// hold runs or its client has no focus.
    #[must_use]
    pub fn focus(&self) -> Option<[f32; 3]> {
        self.hold.as_ref().and(self.focus)
    }

    /// Sets the hold's focus (its client does, each frame).
    pub fn set_focus(&mut self, focus: Option<[f32; 3]>) {
        self.focus = focus;
    }
}

/// `cells, textures, assets 3 (near 2, visible 1), pipelines 6 (draws 1, shadow/prepass 2, compiling 2, resident 1, holds 1); catalog 9 (background)`
/// (`resident`, `holds` and `catalog` only when some are pending; [`RevealCheck`] names the holds).
fn pending_text(hold: &Hold, inputs: &RevealInputs) -> String {
    let (s, resident, holds) = (&inputs.sample, inputs.resident, inputs.holds);
    let mut parts = Vec::new();
    if !hold.cells {
        parts.push("cells".to_owned());
    }
    if !hold.textures {
        parts.push("textures".to_owned());
    }
    let p = RevealPending::of(s, resident, holds);
    if p.assets > 0 {
        parts.push(format!("assets {} (near {}, visible {})", p.assets, s.near_unprepared, s.unprepared));
    }
    if p.pipelines > 0 {
        let resident = if resident > 0 { format!(", resident {resident}") } else { String::new() };
        let holds = if holds > 0 { format!(", holds {holds}") } else { String::new() };
        parts.push(format!(
            "pipelines {} (draws {}, shadow/prepass {}, compiling {}{resident}{holds})",
            p.pipelines, s.skipped, s.skipped_shadow, s.waiting
        ));
    }
    let text = if parts.is_empty() { "nothing (settling)".to_owned() } else { parts.join(", ") };
    if inputs.catalog > 0 { format!("{text}; catalog {} (background)", inputs.catalog) } else { text }
}

/// A client's frame of a hold ([`RevealCheck::step`]).
#[derive(Clone, Copy, Debug)]
pub struct RevealAsk<'a> {
    /// `door`, `loading`, ... (the `reveal:` line's kind).
    pub kind: &'static str,
    /// Worldspace or cell name, for the log.
    pub place: &'a str,
    /// Where the textures and near assets are checked; `None`: the first active 3D camera.
    pub focus: Option<Entity>,
    /// The client's streaming readiness.
    pub cells: bool,
    /// An external gate asked to stay covered this frame.
    pub gated: bool,
    /// The client's own cap was hit.
    pub give_up: bool,
}

/// What a client's frame of a hold returned.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct RevealStatus {
    /// Cells and textures are ready (the CPU side of the hold).
    pub cpu_ready: bool,
    /// The reveal happened this frame: the cover may lift.
    pub open: bool,
}

/// The gate plus what it reads, for a client system (the fade, the app's `Loading`). Writes the
/// [`Reveal`] message (`sf_diag_bevy::RevealWatchPlugin` logs the `reveal:` line) and the cap
/// warning. In an app without a [`RevealGate`] (no `WorldStreamingPlugin`: unit-test apps) the
/// client's own readiness decides.
#[derive(SystemParam)]
pub struct RevealCheck<'w, 's> {
    gate: Option<ResMut<'w, RevealGate>>,
    pending: Option<Res<'w, PendingDraws>>,
    near: NearTextures<'w, 's>,
    cameras: Query<'w, 's, (Entity, &'static Camera), With<Camera3d>>,
    names: Query<'w, 's, &'static Name>,
    time: Option<Res<'w, Time<Real>>>,
    reveals: Option<ResMut<'w, Messages<Reveal>>>,
    warmup: Option<Res<'w, CatalogWarmup>>,
    resident: Option<Res<'w, ResidentWarmup>>,
    precache: Option<Res<'w, Precache>>,
    holds: Option<Res<'w, RevealHolds>>,
}

impl RevealCheck<'_, '_> {
    /// The gate (settings, progress).
    pub fn gate(&self) -> Option<&RevealGate> {
        self.gate.as_deref()
    }

    /// One frame of the client's hold (see [`RevealAsk`]).
    pub fn step(&mut self, ask: RevealAsk) -> RevealStatus {
        let Some(gate) = self.gate.as_deref_mut() else {
            return RevealStatus { cpu_ready: ask.cells, open: ask.cells || ask.give_up };
        };
        let focus = ask.focus.or_else(|| self.cameras.iter().filter(|(_, c)| c.is_active).map(|(e, _)| e).next());
        let radius = gate.radius_m;
        let cells = ask.cells || gate.hold.as_ref().is_some_and(|h| h.cells);
        let textures_known = gate.hold.as_ref().is_some_and(|h| h.textures);
        let textures = cells && (textures_known || focus.is_none_or(|f| self.near.ready(f, radius)));
        let sample = self.pending.as_ref().map(|p| p.sample()).unwrap_or_default();
        let precache = self.precache.as_deref();
        let catalog = self.warmup.as_ref().map_or(0, |w| w.pending(precache));
        let resident = self.resident.as_ref().map_or(0, |r| r.pending(precache));
        let holds = self.holds.as_ref().map_or(0, |h| h.count());
        let inputs = RevealInputs { cells, textures, sample, gated: ask.gated, give_up: ask.give_up, resident, catalog, holds };
        let dt = self.time.as_ref().map_or(0.0, |t| t.delta_secs());
        let center = focus.and_then(|f| self.near.at(f));
        let step = gate.step(ask.kind, ask.place, dt, &inputs);
        let cpu_ready = gate.cpu_ready() || matches!(step, RevealStep::Open { .. });
        match step {
            RevealStep::Holding => {
                gate.set_focus(center);
                if let Some(pending) = &self.pending {
                    pending.set_probe(center.map(|c| RevealProbe { center: c, radius }));
                }
                // Only compiles left: let the background tags' compiles in flight drain (W4b3).
                if let Some(precache) = precache {
                    precache.pause_background(gate.cpu_ready() && inputs.drains_background());
                }
                RevealStatus { cpu_ready, open: false }
            }
            RevealStep::Open { reveal, warning } => {
                if let Some(precache) = precache {
                    precache.pause_background(false);
                }
                let shown = self.resident.as_ref().map_or(0, |r| r.requested());
                log::info!(
                    "warmup: reveal {} \"{}\": resident {shown} specimens ({resident} pending), catalog {catalog} pending (background)",
                    reveal.kind,
                    reveal.place
                );
                let examples = self.pending.as_ref().map(|p| {
                    p.set_probe(None);
                    p.examples()
                });
                if let Some(warning) = warning {
                    let names: Vec<String> = examples
                        .unwrap_or_default()
                        .into_iter()
                        .map(|e| self.names.get(e).map_or_else(|_| format!("{e}"), |n| format!("\"{n}\"")))
                        .collect();
                    let near = if names.is_empty() { String::new() } else { format!("; near meshes: {}", names.join(", ")) };
                    let pending_holds: Vec<&str> = self.holds.as_ref().map(|h| h.pending().collect()).unwrap_or_default();
                    let holds =
                        if pending_holds.is_empty() { String::new() } else { format!("; holds: {}", pending_holds.join(", ")) };
                    log::warn!("sf_world_bevy: reveal gate: {} \"{}\" {warning}{holds}{near}; revealing", reveal.kind, reveal.place);
                }
                if let Some(reveals) = self.reveals.as_mut() {
                    reveals.write(reveal);
                }
                RevealStatus { cpu_ready, open: true }
            }
        }
    }
}

/// Drops an abandoned hold and clears the probe once no hold runs (`Last`).
pub(crate) fn end_reveal_frame(mut gate: ResMut<RevealGate>, pending: Option<Res<PendingDraws>>, precache: Option<Res<Precache>>) {
    if gate.end_frame() {
        log::info!("sf_world_bevy: reveal gate: a hold was abandoned (its cover ended without a reveal)");
    }
    if let Some(precache) = precache
        && !gate.is_holding()
        && precache.is_background_paused()
    {
        precache.pause_background(false);
    }
    if let Some(pending) = pending
        && !gate.is_holding()
        && pending.probe().is_some()
    {
        pending.set_probe(None);
    }
}

#[cfg(test)]
#[path = "reveal_tests.rs"]
mod tests;
