//! The dev Features panel (slice FS-UI1; spec `superfargoth/docs/specs/feature-switches.md` "Dev
//! Features panel"; UE's show flags menu, Unity's Rendering Debugger): every registered feature
//! switch by category, each row a toggle. Ctrl+F3 in every interactive run, release included (it
//! rides on [`crate::live_ab::add`] like the F3 panel); console and BRP `flip featurespanel`
//! ([`crate::live_ab::Switch::FeaturesPanel`]).
//!
//! - **An overlay, not a menu screen:** the game keeps running (the clock, the frame, the F3
//!   graph), so a flip shows at once. Open, it suspends the player's controls and frees the cursor
//!   through the menus' seam (`PlayerMode::paused`: the body stands, `mw_player_bevy` frees the
//!   cursor on the edge and grabs it again after the close); without a player (viewers) it frees
//!   the cursor itself and restores it on close. Not a `ui.screen`: a Menu-layer screen pauses the
//!   same way but takes a record, a template and the router's covered-screen rules for a dev
//!   overlay that needs none of them.
//! - **Writes the `Console` layer only** (never saved): a row's toggle is
//!   `sf_console_bevy::features::set_feature` (the console's `toggle`), "reset all" peels the
//!   console layer of every switch set there ([`PeelTunable`]), so the preset look comes back.
//!   The answer line shows under the header. A switch not built this run answers why and changes
//!   nothing.
//! - **Rebuilt only on change:** the tab strip and rows are respawned when a [`FeatureToggled`]
//!   arrives, the tunables' revision or the switch count moved, or the tab changed; never per
//!   frame. Closed, no node exists and only the run condition runs.
//! - **Cost beside the switch (slice FS-UI2):** each row shows its last flip delta from
//!   [`SwitchProbe`] (frame ms, render and GPU smaller, `moved` / `superseded` / `measuring...`,
//!   greyed under the noise floor), rewritten in place when the probe's generation, its measuring
//!   flag or the switches waiting on it change. The header shows the live mean frame, main, render
//!   and GPU ms of the F3 feed ([`PerfFrameFeed`], read before the probe takes it), rewritten in
//!   place every `diag.panel.refresh_s` of frames. The probe keeps the gauges running while the
//!   panel is open ([`crate::perf_panel::PerfGaugeDemand`]).
//!
//! Pure rules: [`crate::features_panel_model`]; tree: [`ui`].

#[path = "features_panel_ui.rs"]
mod ui;

use bevy_app::prelude::*;
use bevy_ecs::prelude::*;
use bevy_log::info;
use bevy_text::TextColor;
use bevy_time::{Real, Time};
use bevy_ui::prelude::Text;
use bevy_ui_widgets::Activate;
use bevy_window::{CursorGrabMode, CursorOptions, PrimaryWindow};
use bevy_color::Color;
use mw_game_core::player::PlayerMode;
use sf_tunables_bevy::{
    Category, FeatureSwitches, FeatureToggled, Id, PeelTunable, SetBy, TunableSystems, Tunables, TunablesAppExt,
};

pub use self::ui::PanelAction;
use self::ui::{DeltaTexts, Look, PanelUi};
use crate::features_panel_model::{self as model, AbCell, DeltaCell, LiveMeans};
use crate::perf_panel::PerfFrameFeed;
use crate::switch_probe::{QuickAb, SwitchProbe, start_quick_ab};

/// Whether the panel is open (Ctrl+F3, `flip featurespanel`).
#[derive(Resource, Default, Debug, Clone, Copy, PartialEq, Eq)]
pub struct FeaturesPanel {
    pub open: bool,
}

/// The F3 panel's tunables the panel reads its look from.
#[derive(Clone, Copy, Debug)]
struct LookIds {
    alpha: Id<f32>,
    on: Id<[f32; 3]>,
    off: Id<[f32; 3]>,
    warn: Id<[f32; 3]>,
}

#[derive(Resource, Debug)]
struct PanelState {
    ids: LookIds,
    ui: Option<PanelUi>,
    look: Option<Look>,
    /// The selected tab (kept across closes).
    tab: Option<Category>,
    tabs: Vec<Category>,
    /// What the rows were built from: tunables revision, switch count, tab, an A/B running.
    built: Option<(u64, usize, Option<Category>, bool)>,
    /// The shown rows' delta texts.
    cells: Vec<DeltaTexts>,
    /// What the delta and A/B texts show: probe generation, measuring, A/B generation.
    delta_key: Option<(u64, bool, u64)>,
    /// `diag.panel.refresh_s` (the F3 panel's).
    refresh: Id<f32>,
    /// The header's means since its last redraw.
    live: LiveMeans,
    /// The last action's answer, and whether it is still to be shown.
    note: String,
    note_dirty: bool,
    /// `PlayerMode::paused` before the panel opened.
    saved_paused: Option<bool>,
    /// The cursor before the panel opened, when there is no player to restore it.
    saved_cursor: Option<(bool, CursorGrabMode)>,
}

/// Marks the panel's root node.
#[derive(Component)]
pub struct FeaturesPanelRoot;

/// Adds the panel (the key and console reach it through [`crate::live_ab`]).
pub struct FeaturesPanelPlugin;

impl Plugin for FeaturesPanelPlugin {
    fn build(&self, app: &mut App) {
        use crate::perf_panel::{ALPHA, COLOUR_GPU, COLOUR_MAIN, COLOUR_TARGET_P99, REFRESH_S};
        let ids = LookIds {
            alpha: app.add_tunable(&ALPHA).id(),
            on: app.add_tunable(&COLOUR_GPU).id(),
            off: app.add_tunable(&COLOUR_TARGET_P99).id(),
            warn: app.add_tunable(&COLOUR_MAIN).id(),
        };
        let refresh_s = app.add_tunable(&REFRESH_S).id();
        app.init_resource::<FeaturesPanel>()
            .insert_resource(PanelState {
                ids,
                ui: None,
                look: None,
                tab: None,
                tabs: Vec::new(),
                built: None,
                cells: Vec::new(),
                delta_key: None,
                refresh: refresh_s,
                live: LiveMeans::default(),
                note: String::new(),
                note_dirty: false,
                saved_paused: None,
                saved_cursor: None,
            })
            .add_message::<FeatureToggled>()
            .add_message::<PeelTunable>()
            .add_observer(on_activate)
            // Before the probe (after `TunableSystems::Apply`) takes the feed.
            .add_systems(PreUpdate, live.run_if(is_open).before(TunableSystems::Apply))
            .add_systems(
                Update,
                (apply_open.run_if(resource_changed::<FeaturesPanel>), refresh.run_if(is_open)).chain(),
            );
    }
}

fn is_open(panel: Res<FeaturesPanel>) -> bool {
    panel.open
}

fn srgb(c: [f32; 3]) -> Color {
    Color::srgb(c[0], c[1], c[2])
}

fn read_look(tunables: &Tunables, ids: LookIds) -> Look {
    Look {
        alpha: tunables.read(ids.alpha),
        on: srgb(tunables.read(ids.on)),
        off: srgb(tunables.read(ids.off)),
        warn: srgb(tunables.read(ids.warn)),
    }
}

/// Spawns or despawns the tree; suspends the player's controls and frees the cursor while open.
#[allow(clippy::needless_pass_by_value)] // system parameters
fn apply_open(
    mut commands: Commands,
    panel: Res<FeaturesPanel>,
    mut state: ResMut<PanelState>,
    tunables: Res<Tunables>,
    mode: Option<ResMut<PlayerMode>>,
    mut cursors: Query<&mut CursorOptions, With<PrimaryWindow>>,
) {
    if panel.open && state.ui.is_none() {
        let look = read_look(&tunables, state.ids);
        let ui = ui::spawn(&mut commands, look);
        commands.entity(ui.root).insert(FeaturesPanelRoot);
        state.ui = Some(ui);
        state.look = Some(look);
        state.built = None;
        state.tabs.clear();
        state.cells.clear();
        state.delta_key = None;
        state.live = LiveMeans::default();
        state.note_dirty = true;
        if let Some(mut mode) = mode {
            state.saved_paused = Some(mode.paused);
            mode.paused = true;
        }
        if let Ok(mut cursor) = cursors.single_mut() {
            state.saved_cursor = Some((cursor.visible, cursor.grab_mode));
            cursor.visible = true;
            cursor.grab_mode = CursorGrabMode::None;
        }
    } else if !panel.open
        && let Some(ui) = state.ui.take()
    {
        commands.entity(ui.root).despawn();
        let saved_cursor = state.saved_cursor.take();
        match (mode, state.saved_paused.take()) {
            // The player grabs the cursor again on its own edge.
            (Some(mut mode), Some(paused)) => mode.paused = paused,
            _ => {
                if let (Ok(mut cursor), Some((visible, grab_mode))) = (cursors.single_mut(), saved_cursor) {
                    cursor.visible = visible;
                    cursor.grab_mode = grab_mode;
                }
            }
        }
    }
}

/// A switch's delta cell: `measuring...` while the probe waits on it, else its last result.
fn delta_of(probe: Option<&SwitchProbe>, name: &'static str) -> DeltaCell {
    let measuring = probe.is_some_and(|p| p.waiting(name));
    model::delta_cell(probe.and_then(|p| p.get(name)), measuring)
}

/// A switch's A/B button: its progress while it runs, else its last result.
fn ab_of(ab: Option<&QuickAb>, name: &'static str, can_ab: bool) -> AbCell {
    model::ab_cell(name, ab.and_then(QuickAb::running), ab.and_then(|a| a.get(name)), can_ab)
}

/// Rebuilds the tabs and rows when something they show changed; rewrites the delta texts in place
/// when only a probe or A/B result, or the measuring state, moved.
#[allow(clippy::needless_pass_by_value, clippy::too_many_arguments)] // system parameters
fn refresh(
    mut commands: Commands,
    mut state: ResMut<PanelState>,
    switches: Option<Res<FeatureSwitches>>,
    tunables: Res<Tunables>,
    probe: Option<Res<SwitchProbe>>,
    ab: Option<Res<QuickAb>>,
    mut toggled: MessageReader<FeatureToggled>,
    mut texts: Query<(&mut Text, &mut TextColor)>,
) {
    let state = &mut *state;
    let Some(ui) = state.ui else { return };
    let look = state.look.unwrap_or_else(|| read_look(&tunables, state.ids));
    let toggled = toggled.read().count() > 0;
    let probe = probe.as_deref();
    let ab = ab.as_deref();
    let busy = ab.is_some_and(|a| a.running().is_some());
    let measuring = probe.is_some_and(SwitchProbe::measuring);
    let delta_key = (probe.map_or(0, SwitchProbe::generation), measuring, ab.map_or(0, QuickAb::generation));
    if state.note_dirty {
        state.note_dirty = false;
        if let Ok((mut text, _)) = texts.get_mut(ui.note) {
            text.0.clone_from(&state.note);
        }
    }
    let Some(switches) = switches else {
        if state.built.is_none() {
            state.cells = ui::fill_rows(&mut commands, &ui, &[], look, busy, |_| AbCell::default());
            state.built = Some((0, 0, None, busy));
        }
        return;
    };
    let tabs = model::tabs(&switches);
    let tab = model::select_tab(&tabs, state.tab);
    let key = (tunables.revision(), switches.len(), tab, busy);
    if !toggled && state.built == Some(key) {
        if state.delta_key.as_ref() != Some(&delta_key) {
            for &cell in &state.cells {
                ui::set_delta(&mut texts, cell, &delta_of(probe, cell.name));
                ui::set_ab(&mut texts, cell, &ab_of(ab, cell.name, cell.can_ab));
            }
            state.delta_key = Some(delta_key);
        }
        return;
    }
    if tabs != state.tabs || tab != state.tab || state.built.is_none_or(|(_, _, t, _)| t != tab) {
        ui::fill_tabs(&mut commands, &ui, &tabs, tab);
    }
    let rows = tab
        .map(|t| model::rows(&switches, &tunables, t, |name| delta_of(probe, name)))
        .unwrap_or_default();
    state.cells = ui::fill_rows(&mut commands, &ui, &rows, look, busy, |row| ab_of(ab, row.name, row.can_ab()));
    state.delta_key = Some(delta_key);
    state.tabs = tabs;
    state.tab = tab;
    state.built = Some(key);
}

/// Feeds the header's live means from the F3 feed (the bare `Time<Real>` frame when this frame has
/// no sample) and rewrites its text in place every `diag.panel.refresh_s` of frames.
#[allow(clippy::needless_pass_by_value)] // system parameters
fn live(
    mut state: ResMut<PanelState>,
    feed: Option<Res<PerfFrameFeed>>,
    real: Res<Time<Real>>,
    tunables: Res<Tunables>,
    mut texts: Query<&mut Text>,
) {
    let state = &mut *state;
    let Some(ui) = state.ui else { return };
    match feed.filter(DetectChanges::is_changed).and_then(|f| f.0) {
        Some(f) => state.live.push(f.frame_ms, f.main_ms, f.render_ms, f.gpu_ms),
        None => state.live.push(real.delta_secs() * 1000.0, None, None, None),
    }
    if !state.live.due(tunables.read(state.refresh)) {
        return;
    }
    if let Ok(mut text) = texts.get_mut(ui.live) {
        text.0 = state.live.text();
    }
    state.live = LiveMeans::default();
}

/// A panel button was activated (click, or the router's Confirm on the focused one).
fn on_activate(
    activate: On<Activate>,
    actions: Query<&PanelAction>,
    mut commands: Commands,
    mut panel: ResMut<FeaturesPanel>,
    mut state: ResMut<PanelState>,
    ab: Option<Res<QuickAb>>,
) {
    let Ok(action) = actions.get(activate.entity) else { return };
    // One A/B at a time, and nothing else flips while it runs.
    if let Some((busy, _)) = ab.as_deref().and_then(QuickAb::running)
        && matches!(action, PanelAction::Toggle(_) | PanelAction::Ab(_) | PanelAction::ResetAll)
    {
        state.note = format!("the A/B of {busy} runs: wait for it, or close the panel to stop it");
        state.note_dirty = true;
        return;
    }
    match *action {
        PanelAction::Ab(name) => {
            commands.queue(move |world: &mut World| {
                let note = match start_quick_ab(world, name) {
                    Ok(line) | Err(line) => line,
                };
                set_note(world, note);
            });
        }
        PanelAction::Toggle(name) => {
            commands.queue(move |world: &mut World| toggle(world, name));
        }
        PanelAction::Tab(category) => {
            if state.tab != Some(category) {
                state.tab = Some(category);
                state.built = None;
            }
        }
        PanelAction::ResetAll => {
            commands.queue(reset_all);
        }
        PanelAction::Close => panel.open = false,
    }
}

fn set_note(world: &mut World, note: String) {
    info!("[features] {note}");
    let mut state = world.resource_mut::<PanelState>();
    state.note = note;
    state.note_dirty = true;
}

/// Flips switch `name` at the console layer (the console's `toggle`).
fn toggle(world: &mut World, name: &'static str) {
    let answer = if world.contains_resource::<Tunables>() {
        world.resource_scope(|world, tunables: Mut<Tunables>| {
            sf_console_bevy::features::set_feature(world, &tunables.0, name, None)
        })
    } else {
        None
    };
    let note = match answer {
        Some(Ok(line) | Err(line)) => line,
        None => format!("{name}: no such feature switch"),
    };
    set_note(world, note);
}

/// Peels the console layer of every switch set there: each comes back to its preset value.
fn reset_all(world: &mut World) {
    let names = match (world.get_resource::<FeatureSwitches>(), world.get_resource::<Tunables>()) {
        (Some(switches), Some(tunables)) => model::console_set(switches, tunables),
        _ => Vec::new(),
    };
    for name in &names {
        world.write_message(PeelTunable { name: (*name).to_owned(), layer: Some(SetBy::Console) });
    }
    let note = match names.len() {
        0 => "reset all: no switch is set at the console layer".to_owned(),
        n => format!("reset all: {n} switch(es) back to the preset ({})", names.join(", ")),
    };
    set_note(world, note);
}

#[cfg(test)]
#[path = "features_panel_tests.rs"]
mod tests;
