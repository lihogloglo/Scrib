//! The Features panel's pure half ([`crate::features_panel`], slice FS-UI1; spec
//! `superfargoth/docs/specs/feature-switches.md` "Dev Features panel"): which category tabs exist,
//! the rows of one tab and their badges, from the switch registry and the tunables store; the flip
//! delta cell (slice FS-UI2, from [`crate::switch_probe`]'s [`SwitchDelta`]) and the header's live
//! means. No ECS, so every rule has an exact test (`features_panel_tests.rs`).

use sf_tunables_bevy::sf_tunables::{
    Availability, Category, Effect, FeatureSwitch, FeatureSwitches, Lifetime, SetBy, Tunables,
};

use crate::switch_probe_model::{AbOutcome, SwitchDelta};

/// A note beside a row: what flipping it does beyond the obvious, or why it can't run.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Badge {
    /// The tunable is read at start: the flip is stored and applies on the next run.
    Restart,
    /// The tunable is part of the bake key: the flip needs a re-bake.
    Bake,
    /// Off hides the result only; the work still runs ([`Effect::Hides`]).
    Hides,
    /// The app did not build the feature this run; the switch can't bring it back.
    NotBuilt(String),
    /// A required switch does not run.
    BlockedBy(&'static str),
}

impl Badge {
    /// The badge's text (`restart`, `blocked by sky.fog.enabled`).
    pub fn text(&self) -> String {
        match self {
            Badge::Restart => "restart".to_owned(),
            Badge::Bake => "re-bake".to_owned(),
            Badge::Hides => "hides".to_owned(),
            Badge::NotBuilt(reason) => format!("not built ({reason})"),
            Badge::BlockedBy(by) => format!("blocked by {by}"),
        }
    }

    /// Not built and blocked: the feature does not run whatever the switch says.
    pub fn is_warning(&self) -> bool {
        matches!(self, Badge::NotBuilt(_) | Badge::BlockedBy(_))
    }
}

/// One switch as the panel shows it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Row {
    pub name: &'static str,
    pub label: &'static str,
    /// The tunable's effective value is not the off value.
    pub on: bool,
    /// On and available: the feature runs.
    pub running: bool,
    /// The layer the effective value comes from (`console` after a click).
    pub layer: SetBy,
    pub badges: Vec<Badge>,
    /// The last measured flip ([`delta_cell`]).
    pub delta: DeltaCell,
}

impl Row {
    /// A click can change it: everything but a feature not built this run.
    pub fn can_toggle(&self) -> bool {
        !self.badges.iter().any(|b| matches!(b, Badge::NotBuilt(_)))
    }

    /// A quick A/B can run on it: a live switch that runs or can (not restart, re-bake, not
    /// built or blocked; the `part.*` rows are restart rows).
    pub fn can_ab(&self) -> bool {
        !self
            .badges
            .iter()
            .any(|b| matches!(b, Badge::Restart | Badge::Bake | Badge::NotBuilt(_) | Badge::BlockedBy(_)))
    }

    /// The toggle's word: `on`, `off`.
    pub fn state_word(&self) -> &'static str {
        if self.on { "on" } else { "off" }
    }
}

/// The categories that have at least one switch, in panel order ([`Category::ALL`]).
pub fn tabs(switches: &FeatureSwitches) -> Vec<Category> {
    Category::ALL
        .into_iter()
        .filter(|c| switches.by_category(*c).next().is_some())
        .collect()
}

/// The tab to show: `current` while it still exists, else the first.
pub fn select_tab(tabs: &[Category], current: Option<Category>) -> Option<Category> {
    current
        .filter(|c| tabs.contains(c))
        .or_else(|| tabs.first().copied())
}

/// A tab's title: `view_distance` reads `View distance`.
pub fn tab_label(category: Category) -> String {
    let name = category.name().replace('_', " ");
    let mut chars = name.chars();
    chars.next().map_or_else(String::new, |first| {
        first.to_ascii_uppercase().to_string() + chars.as_str()
    })
}

/// The rows of one category, in registration order. `store` is the one the switches were
/// registered against; `delta` gives each switch's flip delta cell.
pub fn rows(
    switches: &FeatureSwitches,
    store: &Tunables,
    category: Category,
    delta: impl Fn(&'static str) -> DeltaCell,
) -> Vec<Row> {
    switches
        .by_category(category)
        .filter_map(|switch| {
            let state = switches.state(store, switch.name())?;
            let lifetime = switches
                .tunable(switch.name())
                .map_or(Lifetime::Live, |id| store.var(id).meta().lifetime);
            let mut badges = Vec::new();
            match lifetime {
                Lifetime::Restart => badges.push(Badge::Restart),
                Lifetime::Bake => badges.push(Badge::Bake),
                Lifetime::Live | Lifetime::Respawn | Lifetime::Recompile => {}
            }
            if switch.effect_of() == Effect::Hides {
                badges.push(Badge::Hides);
            }
            match state.availability {
                Availability::Available => {}
                Availability::NotBuilt { reason } => badges.push(Badge::NotBuilt(reason.to_owned())),
                Availability::Blocked { by } => badges.push(Badge::BlockedBy(by)),
            }
            Some(Row {
                name: switch.name(),
                label: switch.label_text(),
                on: state.on,
                running: state.running(),
                layer: state.layer,
                badges,
                delta: delta(switch.name()),
            })
        })
        .collect()
}

/// Every switch whose value comes from the console layer: what "reset all" peels.
pub fn console_set(switches: &FeatureSwitches, store: &Tunables) -> Vec<&'static str> {
    switches
        .iter()
        .filter(|s| {
            switches
                .state(store, s.name())
                .is_some_and(|state| state.layer == SetBy::Console)
        })
        .map(FeatureSwitch::name)
        .collect()
}

/// The delta column's heading: the change reads as "after the flip minus before".
pub const DELTA_HEADING: &str = "frame change when flipped";

/// A row's flip delta, as the panel writes it: `value` (the frame ms change), `detail` (render and
/// GPU changes, smaller) and `flag` (`moved`, `superseded`, `measuring...`). All blank: never
/// measured, or no before window.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct DeltaCell {
    pub value: String,
    pub detail: String,
    pub flag: &'static str,
    /// Under `diag.switch.noise_ms`: greyed.
    pub noise: bool,
}

impl DeltaCell {
    /// The smaller text beside the value: detail, then flag.
    pub fn small(&self) -> String {
        match (self.detail.is_empty(), self.flag.is_empty()) {
            (_, true) => self.detail.clone(),
            (true, false) => self.flag.to_owned(),
            (false, false) => format!("{} {}", self.detail, self.flag),
        }
    }
}

/// `-1.3`, `+0.4`, `+0.0` (never `-0.0`).
fn signed(ms: f32) -> String {
    let tenths = (ms * 10.0).round() / 10.0;
    let tenths = if tenths.abs() < 0.05 { 0.0 } else { tenths };
    format!("{tenths:+.1}")
}

/// The cell for one switch: `measuring` while its after window is still open (any earlier result
/// is hidden), else its last result. Delta = after - before: turning off a feature that costs
/// 1.3 ms reads `-1.3 ms`.
pub fn delta_cell(result: Option<&SwitchDelta>, measuring: bool) -> DeltaCell {
    if measuring {
        return DeltaCell { flag: "measuring...", ..DeltaCell::default() };
    }
    let Some(d) = result else { return DeltaCell::default() };
    if d.superseded {
        return DeltaCell { flag: "superseded", ..DeltaCell::default() };
    }
    let Some(frame) = d.frame_ms else { return DeltaCell::default() };
    let detail = [("r", d.render_ms), ("g", d.gpu_ms)]
        .into_iter()
        .filter_map(|(tag, v)| v.map(|v| format!("{tag} {}", signed(v))))
        .collect::<Vec<_>>()
        .join(" ");
    DeltaCell {
        value: format!("{} ms", signed(frame)),
        detail,
        flag: if d.moved { "moved" } else { "" },
        noise: d.noise,
    }
}

#[derive(Clone, Copy, Debug, Default)]
struct Mean {
    sum: f64,
    n: u32,
}

impl Mean {
    fn push(&mut self, v: Option<f32>) {
        if let Some(v) = v {
            self.sum += f64::from(v);
            self.n += 1;
        }
    }

    fn text(self) -> String {
        if self.n == 0 {
            "n/a".to_owned()
        } else {
            format!("{:.1}", self.sum / f64::from(self.n))
        }
    }
}

/// The header's live means of the frames since the last redraw (frame, main, render, GPU ms);
/// time is counted in frame time, so a redraw comes every `refresh_s` of frames.
#[derive(Clone, Copy, Debug, Default)]
pub struct LiveMeans {
    frame: Mean,
    main: Mean,
    render: Mean,
    gpu: Mean,
    elapsed_s: f32,
}

impl LiveMeans {
    /// One frame; a thread without a number this frame (GPU timers off) is left out of its mean.
    pub fn push(&mut self, frame_ms: f32, main_ms: Option<f32>, render_ms: Option<f32>, gpu_ms: Option<f32>) {
        self.frame.push(Some(frame_ms));
        self.main.push(main_ms);
        self.render.push(render_ms);
        self.gpu.push(gpu_ms);
        self.elapsed_s += frame_ms / 1000.0;
    }

    /// `refresh_s` of frames went by since the last redraw.
    pub fn due(&self, refresh_s: f32) -> bool {
        self.frame.n > 0 && self.elapsed_s >= refresh_s
    }

    /// `ms: frame 9.8  main 6.1  render 4.0  gpu n/a` (`n/a`: no number in the window).
    pub fn text(&self) -> String {
        format!(
            "ms: frame {}  main {}  render {}  gpu {}",
            self.frame.text(),
            self.main.text(),
            self.render.text(),
            self.gpu.text()
        )
    }
}

/// A row's quick A/B button: `A/B`, `A/B 2/6...` while it runs, `cost 1.4 ms (r 0.9, 1.1..1.7)`
/// after (the median frame cost, the render one, the spread of the flips), or why it stopped.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct AbCell {
    pub text: String,
    /// A click starts one: a live row, and no A/B running.
    pub enabled: bool,
    /// The cost is under `diag.switch.noise_ms`: greyed.
    pub noise: bool,
}

/// `1.4`, `-0.3`, `0.0` (never `-0.0`).
fn plain(ms: f32) -> String {
    let tenths = (ms * 10.0).round() / 10.0;
    let tenths = if tenths.abs() < 0.05 { 0.0 } else { tenths };
    format!("{tenths:.1}")
}

/// Switch `name`'s A/B button, from the A/B running (its switch and progress) and its last
/// outcome; `can_ab` from [`Row::can_ab`].
pub fn ab_cell(
    name: &str,
    running: Option<(&'static str, (u32, u32))>,
    outcome: Option<&AbOutcome>,
    can_ab: bool,
) -> AbCell {
    let enabled = can_ab && running.is_none();
    if let Some((busy, (done, total))) = running
        && busy == name
    {
        return AbCell { text: format!("A/B {done}/{total}..."), enabled, noise: false };
    }
    match outcome {
        Some(AbOutcome::Done(c)) => {
            let render = c.render_ms.map(|r| format!("r {}, ", plain(r))).unwrap_or_default();
            AbCell {
                text: format!("cost {} ms ({render}{}..{})", plain(c.frame_ms), plain(c.min_ms), plain(c.max_ms)),
                enabled,
                noise: c.noise,
            }
        }
        Some(AbOutcome::Aborted(why)) => AbCell { text: format!("A/B: {}", why.text()), enabled, noise: false },
        None => AbCell { text: "A/B".to_owned(), enabled, noise: false },
    }
}
