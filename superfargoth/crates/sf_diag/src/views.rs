//! Per-view render counters (static-scene.md slice S0, diag.md "Render
//! views"): what each view (main camera, sun cascade, lamp face, ...) queued,
//! binned and batched in a frame.
//!
//! The renderer fills a [`ViewFrame`] once per frame; [`ViewFrame::record`]
//! turns it into gauges (totals per phase and sums per view kind), and a
//! [`ViewTable`] accumulates the measured frames into the per-view breakdown
//! of the JSON report ([`crate::Report::views`]).
//!
//! Gauges (all `count`, set every frame the renderer reports):
//! - `views/<kind>/count`: views of that kind; `<kind>` is `main`, `cascade`,
//!   `lamp_face`, `other`, or `all` (every view).
//! - `views/<kind>/items|bins|batch_sets|work_items|pipelines`: summed over
//!   the kind's views and their phases. `pipelines` sums each view's distinct
//!   pipelines (a pipeline used by two views counts twice).
//! - `views/<kind>/instance_batches|instance_draws|instance_slots`: the
//!   GPU-authored instance batches (static-scene.md S3a2) inside those sums.
//! - `views/all/pipelines_distinct`: distinct pipelines across all views.
//! - `phases/<phase>/<metric>` for every metric in [`PHASE_METRICS`]: one
//!   phase summed over all views.
//! - `preprocess/bind_groups`: every preprocessing bind group created this
//!   frame, when the renderer reports it, with
//!   `preprocess/bind_groups_created|_ranges|_materialize` as its parts
//!   ([`BindGroups`]).
//!
//! Static materials (slice S2, [`StaticMaterials`], [`StaticBindGroups`]; names in [`statics`]),
//! over the live static mesh parts, every frame while the engine counts them:
//! - `statics/materials`, `statics/material_classes`, `statics/materials_unclassed`;
//! - `statics/material_keys` (distinct `StandardMaterialKey`s) and `statics/material_pipelines`
//!   (key + alpha mode + render method);
//! - `statics/material_bind_groups` (distinct material bind groups holding them),
//!   `statics/materials_bound` (of those prepared so far), `statics/bindless` (1 or 0).

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::counters::{Counters, unit};

/// What a view renders for.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ViewKind {
    /// The main camera.
    Main,
    /// A directional light's shadow cascade.
    Cascade,
    /// A point light's cube face or a spot light's shadow view.
    LampFace,
    /// Anything else with binned phases (other cameras, probes).
    Other,
}

impl ViewKind {
    pub const ALL: [ViewKind; 4] = [Self::Main, Self::Cascade, Self::LampFace, Self::Other];

    /// The kind's segment in counter names.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Main => "main",
            Self::Cascade => "cascade",
            Self::LampFace => "lamp_face",
            Self::Other => "other",
        }
    }
}

/// Metrics reported per phase: the length of [`PHASE_METRICS`].
pub const PHASE_METRIC_COUNT: usize = 7;

/// Metric names of [`PhaseCounts`], in the order of [`PhaseCounts::values`]
/// and of the arrays [`ViewFrame::phase_totals`] returns.
pub const PHASE_METRICS: [&str; PHASE_METRIC_COUNT] = [
    "items",
    "bins",
    "batch_sets",
    "pipelines",
    "instance_batches",
    "instance_draws",
    "instance_slots",
];

/// One binned phase of one view.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct PhaseCounts {
    /// Short phase name (`opaque3d`, `shadow`, ...), a counter-name segment.
    pub phase: &'static str,
    /// Entities queued in the phase.
    pub items: u32,
    /// Bins (one per mesh/material key within a batch set).
    pub bins: u32,
    /// Batch sets built this frame: one (multi-)draw each. Includes the
    /// GPU-authored instance batches ([`Self::instance_batches`]), which draw
    /// from their own list and register their batch sets straight on the
    /// indirect buffers.
    pub batch_sets: u32,
    /// Distinct render pipelines among the phase's batch sets.
    pub pipelines: u32,
    /// GPU-authored instance batches drawn in the phase this frame
    /// (static-scene.md S3a; one draw record per batch entity). A subset of
    /// [`Self::batch_sets`]: subtract it for the entity path alone.
    pub instance_batches: u32,
    /// Indirect sub-draws inside those batches' batch sets: one per geometry
    /// range a batch draws. Equals [`Self::instance_batches`] until a batch
    /// can carry more than one range (S3a2 step 2).
    pub instance_draws: u32,
    /// Instance slots those batches reserved in the view's output
    /// (`MeshUniform`) buffer. **Reserved, pre-cull**: the number that
    /// survived the GPU cull exists only in `early_instance_count` on the GPU
    /// and is not reported.
    pub instance_slots: u32,
}

impl PhaseCounts {
    /// The phase's metrics in [`PHASE_METRICS`] order.
    pub fn values(&self) -> [u32; PHASE_METRIC_COUNT] {
        [
            self.items,
            self.bins,
            self.batch_sets,
            self.pipelines,
            self.instance_batches,
            self.instance_draws,
            self.instance_slots,
        ]
    }
}

/// One view in one frame.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ViewCounts {
    /// Stable name across frames (`main`, `cascade0`, `lamp.face3`, ...):
    /// views sharing a label are summed in the breakdown.
    pub label: String,
    pub kind: ViewKind,
    pub phases: Vec<PhaseCounts>,
    /// Mesh preprocessing work items written for the view, all phases:
    /// **reserved, not surviving**. A GPU-authored instance batch reserves one
    /// slot per instance slot of its capacity (the GPU fills them in), so this
    /// counts the dispatch width, not what the cull let through.
    pub work_items: u32,
    /// Distinct render pipelines across the view's phases.
    pub pipelines: u32,
}

impl ViewCounts {
    fn phase_sum(&self, f: impl Fn(&PhaseCounts) -> u32) -> u64 {
        self.phases.iter().map(|p| u64::from(f(p))).sum()
    }

    /// The view's metrics by name: `items`, `bins`, `batch_sets`,
    /// `instance_batches`, `instance_draws`, `instance_slots`, `work_items`,
    /// `pipelines`, then `<phase>/<metric>` per phase (every name in
    /// [`PHASE_METRICS`]).
    pub fn metrics(&self) -> Vec<(String, u64)> {
        let mut out = vec![
            ("items".to_owned(), self.phase_sum(|p| p.items)),
            ("bins".to_owned(), self.phase_sum(|p| p.bins)),
            ("batch_sets".to_owned(), self.phase_sum(|p| p.batch_sets)),
            ("instance_batches".to_owned(), self.phase_sum(|p| p.instance_batches)),
            ("instance_draws".to_owned(), self.phase_sum(|p| p.instance_draws)),
            ("instance_slots".to_owned(), self.phase_sum(|p| p.instance_slots)),
            ("work_items".to_owned(), u64::from(self.work_items)),
            ("pipelines".to_owned(), u64::from(self.pipelines)),
        ];
        for p in &self.phases {
            for (metric, v) in PHASE_METRICS.into_iter().zip(p.values()) {
                out.push((format!("{}/{metric}", p.phase), u64::from(v)));
            }
        }
        out
    }
}

/// Preprocessing bind groups created in one render frame, by owner. Summed
/// into `preprocess/bind_groups`, and each is its own gauge as well. Without
/// GPU-authored instance batches `ranges` and `materialize` are 0 and the
/// total equals `created`, so the S0 baseline stays comparable.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct BindGroups {
    /// Mesh preprocessing, indirect-parameter building, bin unpacking and
    /// uniform allocation (the renderer rebuilds all of them every frame).
    pub created: u32,
    /// Range unpacking for the instance batches: one per view, phase and
    /// indexed/non-indexed buffer that has ranges. Rebuilt every frame.
    pub ranges: u32,
    /// Instance-batch materialization: one per live batch, but cached on the
    /// buffers it binds, so normally 0 after a batch's first frame and
    /// non-zero again when its output buffer is resized.
    pub materialize: u32,
}

impl BindGroups {
    /// Every preprocessing bind group created this frame.
    pub fn total(self) -> u32 {
        self.created
            .saturating_add(self.ranges)
            .saturating_add(self.materialize)
    }
}

/// Every view the renderer drew in one frame.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ViewFrame {
    pub views: Vec<ViewCounts>,
    /// Distinct render pipelines across all views.
    pub distinct_pipelines: u32,
    /// Preprocessing bind groups created this frame, if known.
    pub bind_groups: Option<BindGroups>,
}

/// Per-kind (or total) sums of one frame.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct KindTotals {
    pub count: u64,
    pub items: u64,
    pub bins: u64,
    pub batch_sets: u64,
    pub work_items: u64,
    pub pipelines: u64,
    /// GPU-authored instance batches drawn, summed over the kind's views and
    /// phases (static-scene.md S3a2); a subset of `batch_sets`.
    pub instance_batches: u64,
    /// Indirect sub-draws inside those batches.
    pub instance_draws: u64,
    /// Output-buffer instance slots those batches reserved (pre-cull).
    pub instance_slots: u64,
}

impl KindTotals {
    fn add(&mut self, v: &ViewCounts) {
        self.count += 1;
        self.items += v.phase_sum(|p| p.items);
        self.bins += v.phase_sum(|p| p.bins);
        self.batch_sets += v.phase_sum(|p| p.batch_sets);
        self.work_items += u64::from(v.work_items);
        self.pipelines += u64::from(v.pipelines);
        self.instance_batches += v.phase_sum(|p| p.instance_batches);
        self.instance_draws += v.phase_sum(|p| p.instance_draws);
        self.instance_slots += v.phase_sum(|p| p.instance_slots);
    }

    fn fields(&self) -> [(&'static str, u64); 9] {
        [
            ("count", self.count),
            ("items", self.items),
            ("bins", self.bins),
            ("batch_sets", self.batch_sets),
            ("work_items", self.work_items),
            ("pipelines", self.pipelines),
            ("instance_batches", self.instance_batches),
            ("instance_draws", self.instance_draws),
            ("instance_slots", self.instance_slots),
        ]
    }
}

#[allow(clippy::cast_precision_loss)] // counts far below 2^52
fn f(v: u64) -> f64 {
    v as f64
}

impl ViewFrame {
    /// Sums for one kind, or for every view with `None`.
    pub fn totals(&self, kind: Option<ViewKind>) -> KindTotals {
        let mut t = KindTotals::default();
        for v in self.views.iter().filter(|v| kind.is_none_or(|k| v.kind == k)) {
            t.add(v);
        }
        t
    }

    /// One phase summed over every view, the metrics in [`PHASE_METRICS`]
    /// order; phases in first-seen order.
    pub fn phase_totals(&self) -> Vec<(&'static str, [u64; PHASE_METRIC_COUNT])> {
        let mut out: Vec<(&'static str, [u64; PHASE_METRIC_COUNT])> = Vec::new();
        for p in self.views.iter().flat_map(|v| &v.phases) {
            let found = out.iter().position(|(n, _)| *n == p.phase);
            let i = if let Some(i) = found {
                i
            } else {
                out.push((p.phase, [0; PHASE_METRIC_COUNT]));
                out.len() - 1
            };
            let sums = &mut out[i].1;
            for (s, v) in sums.iter_mut().zip(p.values()) {
                *s += u64::from(v);
            }
        }
        out
    }

    /// Sets the gauges of the module docs. Every kind is written, zero when
    /// it has no view this frame, so the columns are continuous.
    pub fn record(&self, counters: &Counters) {
        let kinds = ViewKind::ALL.map(|k| (k.as_str(), Some(k)));
        for (name, kind) in kinds.into_iter().chain([("all", None)]) {
            for (metric, v) in self.totals(kind).fields() {
                counters.set_named(&format!("views/{name}/{metric}"), unit::COUNT, f(v));
            }
        }
        counters.set_named(
            "views/all/pipelines_distinct",
            unit::COUNT,
            f(u64::from(self.distinct_pipelines)),
        );
        for (phase, sums) in self.phase_totals() {
            for (metric, v) in PHASE_METRICS.into_iter().zip(sums) {
                counters.set_named(&format!("phases/{phase}/{metric}"), unit::COUNT, f(v));
            }
        }
        if let Some(b) = self.bind_groups {
            for (name, v) in [
                ("preprocess/bind_groups", b.total()),
                ("preprocess/bind_groups_created", b.created),
                ("preprocess/bind_groups_ranges", b.ranges),
                ("preprocess/bind_groups_materialize", b.materialize),
            ] {
                counters.set_named(name, unit::COUNT, f(u64::from(v)));
            }
        }
    }
}

/// Gauge names of [`StaticMaterials`] and [`StaticBindGroups`].
pub mod statics {
    pub const MATERIALS: &str = "statics/materials";
    pub const CLASSES: &str = "statics/material_classes";
    pub const KEYS: &str = "statics/material_keys";
    pub const PIPELINES: &str = "statics/material_pipelines";
    pub const UNCLASSED: &str = "statics/materials_unclassed";
    pub const BIND_GROUPS: &str = "statics/material_bind_groups";
    pub const BOUND: &str = "statics/materials_bound";
    pub const BINDLESS: &str = "statics/bindless";
}

/// What the spawned statics' materials fold into (static-scene.md slice S2), counted by the
/// engine over the live static mesh parts.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct StaticMaterials {
    /// Distinct material assets.
    pub materials: u32,
    /// Distinct material classes among them.
    pub classes: u32,
    /// Distinct `StandardMaterialKey`s (the material's shader-variant bits).
    pub keys: u32,
    /// Distinct material pipeline keys: the key plus alpha-mode bucket and render method.
    pub pipelines: u32,
    /// Materials with no class (blended kinds, front culling).
    pub unclassed: u32,
}

impl StaticMaterials {
    /// Sets the `statics/*` material gauges.
    pub fn record(&self, counters: &Counters) {
        for (name, v) in [
            (statics::MATERIALS, self.materials),
            (statics::CLASSES, self.classes),
            (statics::KEYS, self.keys),
            (statics::PIPELINES, self.pipelines),
            (statics::UNCLASSED, self.unclassed),
        ] {
            counters.set_named(name, unit::COUNT, f(u64::from(v)));
        }
    }
}

/// Where the live static materials sit in the renderer's material bind groups (render world).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct StaticBindGroups {
    /// Distinct bind groups (bindless slabs, or one per material without bindless).
    pub bind_groups: u32,
    /// Static materials prepared on the GPU so far.
    pub bound: u32,
    /// Whether the material type uses bindless resources on this device.
    pub bindless: bool,
}

impl StaticBindGroups {
    /// Sets the `statics/*` bind-group gauges (`statics/bindless` is 1 or 0).
    pub fn record(&self, counters: &Counters) {
        counters.set_named(statics::BIND_GROUPS, unit::COUNT, f(u64::from(self.bind_groups)));
        counters.set_named(statics::BOUND, unit::COUNT, f(u64::from(self.bound)));
        counters.set_named(statics::BINDLESS, unit::COUNT, f(u64::from(self.bindless)));
    }
}

/// One metric of one view label over the measured frames.
#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct MetricSummary {
    /// Mean per frame in which the label was drawn (all views of the label).
    pub mean: f64,
    /// Largest single frame (all views of the label).
    pub max: f64,
}

/// One view label in the report: how often it was drawn and its metrics.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ViewSummary {
    pub kind: ViewKind,
    /// Measured frames in which the label had at least one view.
    pub frames: u64,
    /// Views per such frame, on average (lamp faces share a label).
    pub views_per_frame: f64,
    pub metrics: BTreeMap<String, MetricSummary>,
}

#[derive(Clone, Debug, Default)]
struct LabelAcc {
    kind: Option<ViewKind>,
    frames: u64,
    views: u64,
    /// metric → (sum over frames, max frame).
    metrics: BTreeMap<String, (u64, u64)>,
}

/// Accumulates [`ViewFrame`]s by view label for the report.
#[derive(Clone, Debug, Default)]
pub struct ViewTable {
    labels: BTreeMap<String, LabelAcc>,
    frames: u64,
}

impl ViewTable {
    pub fn new() -> Self {
        Self::default()
    }

    /// Frames added so far.
    pub fn frames(&self) -> u64 {
        self.frames
    }

    /// Adds one frame: views sharing a label are summed first.
    pub fn add(&mut self, frame: &ViewFrame) {
        self.frames += 1;
        let mut per_label: BTreeMap<&str, (ViewKind, u64, BTreeMap<String, u64>)> = BTreeMap::new();
        for v in &frame.views {
            let e = per_label.entry(v.label.as_str()).or_insert((v.kind, 0, BTreeMap::new()));
            e.1 += 1;
            for (metric, x) in v.metrics() {
                *e.2.entry(metric).or_default() += x;
            }
        }
        for (label, (kind, views, metrics)) in per_label {
            let acc = self.labels.entry(label.to_owned()).or_default();
            acc.kind.get_or_insert(kind);
            acc.frames += 1;
            acc.views += views;
            for (metric, x) in metrics {
                let m = acc.metrics.entry(metric).or_default();
                m.0 += x;
                m.1 = m.1.max(x);
            }
        }
    }

    /// The per-label breakdown.
    pub fn summary(&self) -> BTreeMap<String, ViewSummary> {
        self.labels
            .iter()
            .map(|(label, acc)| {
                let frames = f(acc.frames.max(1));
                let metrics = acc
                    .metrics
                    .iter()
                    .map(|(name, &(sum, max))| {
                        (name.clone(), MetricSummary { mean: f(sum) / frames, max: f(max) })
                    })
                    .collect();
                let summary = ViewSummary {
                    kind: acc.kind.unwrap_or(ViewKind::Other),
                    frames: acc.frames,
                    views_per_frame: f(acc.views) / frames,
                    metrics,
                };
                (label.clone(), summary)
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn phase(phase: &'static str, items: u32, bins: u32, batch_sets: u32, pipelines: u32) -> PhaseCounts {
        PhaseCounts { phase, items, bins, batch_sets, pipelines, ..PhaseCounts::default() }
    }

    /// A phase whose counts include `batches` GPU-authored instance batches
    /// (`draws` indirect sub-draws over `slots` reserved instance slots).
    fn phase_with_batches(
        phase: &'static str,
        items: u32,
        bins: u32,
        batch_sets: u32,
        pipelines: u32,
        (instance_batches, instance_draws, instance_slots): (u32, u32, u32),
    ) -> PhaseCounts {
        PhaseCounts { phase, items, bins, batch_sets, pipelines, instance_batches, instance_draws, instance_slots }
    }

    fn view(label: &str, kind: ViewKind, phases: Vec<PhaseCounts>, work_items: u32, pipelines: u32) -> ViewCounts {
        ViewCounts { label: label.to_owned(), kind, phases, work_items, pipelines }
    }

    fn bind_groups(created: u32, ranges: u32, materialize: u32) -> BindGroups {
        BindGroups { created, ranges, materialize }
    }

    /// Main view with two phases, two cascades, two faces of two lamps
    /// sharing the `lamp.face0` label.
    fn frame() -> ViewFrame {
        ViewFrame {
            views: vec![
                view("main", ViewKind::Main, vec![phase("opaque3d", 100, 40, 10, 3), phase("alpha_mask3d", 20, 8, 2, 1)], 130, 4),
                view("cascade0", ViewKind::Cascade, vec![phase("shadow", 50, 30, 5, 2)], 50, 2),
                view("cascade1", ViewKind::Cascade, vec![phase("shadow", 70, 35, 6, 2)], 70, 2),
                view("lamp.face0", ViewKind::LampFace, vec![phase("shadow", 7, 5, 2, 1)], 7, 1),
                view("lamp.face0", ViewKind::LampFace, vec![phase("shadow", 3, 2, 1, 1)], 3, 1),
            ],
            distinct_pipelines: 5,
            bind_groups: Some(bind_groups(12, 0, 0)),
        }
    }

    /// The same frame once the static scene draws: the main view's opaque
    /// phase gained 3 instance batches (4 sub-draws, 9000 reserved slots) on
    /// top of its 10 entity-path batch sets, and every lamp face one batch.
    fn frame_with_statics() -> ViewFrame {
        ViewFrame {
            views: vec![
                view(
                    "main",
                    ViewKind::Main,
                    vec![
                        phase_with_batches("opaque3d", 103, 43, 13, 3, (3, 4, 9000)),
                        phase("alpha_mask3d", 20, 8, 2, 1),
                    ],
                    9130,
                    4,
                ),
                view("lamp.face0", ViewKind::LampFace, vec![phase_with_batches("shadow", 8, 6, 3, 1, (1, 1, 3000))], 3007, 1),
                view("lamp.face0", ViewKind::LampFace, vec![phase_with_batches("shadow", 4, 3, 2, 1, (1, 1, 3000))], 3003, 1),
            ],
            distinct_pipelines: 5,
            bind_groups: Some(bind_groups(12, 4, 3)),
        }
    }

    #[test]
    fn totals_per_kind_and_all() {
        let f = frame();
        let main = f.totals(Some(ViewKind::Main));
        assert_eq!(
            main,
            KindTotals { count: 1, items: 120, bins: 48, batch_sets: 12, work_items: 130, pipelines: 4, ..KindTotals::default() }
        );
        let cascades = f.totals(Some(ViewKind::Cascade));
        assert_eq!((cascades.count, cascades.items, cascades.batch_sets), (2, 120, 11));
        let lamps = f.totals(Some(ViewKind::LampFace));
        assert_eq!((lamps.count, lamps.items, lamps.work_items), (2, 10, 10));
        assert_eq!(f.totals(Some(ViewKind::Other)), KindTotals::default());
        let all = f.totals(None);
        assert_eq!((all.count, all.items, all.bins, all.batch_sets, all.pipelines), (5, 250, 120, 26, 10));
    }

    /// static-scene.md S3a2 (`gs-patch-plan.md` step 3): the instance-batch
    /// counts reach the per-kind sums, and `batch_sets` keeps counting them
    /// too (it is the budget metric, so it must see the statics).
    #[test]
    fn instance_batch_totals_per_kind_and_all() {
        let f = frame_with_statics();
        let main = f.totals(Some(ViewKind::Main));
        assert_eq!(
            (main.instance_batches, main.instance_draws, main.instance_slots),
            (3, 4, 9000),
            "one view, one phase with batches"
        );
        assert_eq!(main.batch_sets, 15, "13 opaque (10 entity path + 3 batches) + 2 alpha mask");
        let lamps = f.totals(Some(ViewKind::LampFace));
        assert_eq!((lamps.instance_batches, lamps.instance_draws, lamps.instance_slots), (2, 2, 6000));
        let all = f.totals(None);
        assert_eq!((all.instance_batches, all.instance_draws, all.instance_slots), (5, 6, 15000));
        // Without statics every instance counter is zero, so an S0 baseline
        // taken before S3b stays a valid "before" measurement.
        let before = frame().totals(None);
        assert_eq!((before.instance_batches, before.instance_draws, before.instance_slots), (0, 0, 0));
    }

    #[test]
    fn phase_totals_sum_over_views() {
        let totals = frame().phase_totals();
        assert_eq!(
            totals,
            vec![
                ("opaque3d", [100, 40, 10, 3, 0, 0, 0]),
                ("alpha_mask3d", [20, 8, 2, 1, 0, 0, 0]),
                ("shadow", [130, 72, 14, 6, 0, 0, 0]),
            ]
        );
        let statics = frame_with_statics().phase_totals();
        assert_eq!(
            statics,
            vec![
                ("opaque3d", [103, 43, 13, 3, 3, 4, 9000]),
                ("alpha_mask3d", [20, 8, 2, 1, 0, 0, 0]),
                ("shadow", [12, 9, 5, 2, 2, 2, 6000]),
            ]
        );
    }

    /// `PHASE_METRICS` and [`PhaseCounts::values`] stay in lockstep: the
    /// gauge names and the arrays `phase_totals` returns are zipped.
    #[test]
    fn phase_metric_names_match_the_values() {
        let p = phase_with_batches("opaque3d", 1, 2, 3, 4, (5, 6, 7));
        assert_eq!(p.values(), [1, 2, 3, 4, 5, 6, 7]);
        assert_eq!(PHASE_METRICS.len(), p.values().len());
        let by_name: BTreeMap<&str, u32> = PHASE_METRICS.into_iter().zip(p.values()).collect();
        assert_eq!(by_name["batch_sets"], 3);
        assert_eq!(by_name["instance_batches"], 5);
        assert_eq!(by_name["instance_draws"], 6);
        assert_eq!(by_name["instance_slots"], 7);
    }

    #[test]
    fn bind_group_parts_sum_to_the_total() {
        assert_eq!(bind_groups(12, 4, 3).total(), 19);
        assert_eq!(BindGroups::default().total(), 0);
        // No instance batches: the total is exactly the S0 number.
        assert_eq!(bind_groups(12, 0, 0).total(), 12);
        assert_eq!(bind_groups(u32::MAX, 1, 1).total(), u32::MAX, "saturating, never a panic");
    }

    #[test]
    fn record_sets_gauges_for_every_kind() {
        let counters = Counters::new();
        frame().record(&counters);
        let mut values = BTreeMap::new();
        counters.drain(|_, def, v| {
            values.insert(def.name.clone(), v);
        });
        let get = |n: &str| values.get(n).copied().flatten();
        assert_eq!(get("views/main/items"), Some(120.0));
        assert_eq!(get("views/cascade/count"), Some(2.0));
        assert_eq!(get("views/other/count"), Some(0.0));
        assert_eq!(get("views/all/batch_sets"), Some(26.0));
        assert_eq!(get("views/all/pipelines_distinct"), Some(5.0));
        assert_eq!(get("phases/shadow/items"), Some(130.0));
        assert_eq!(get("preprocess/bind_groups"), Some(12.0));
        // Written every frame, zero without statics, so the columns are continuous.
        assert_eq!(get("views/all/instance_batches"), Some(0.0));
        assert_eq!(get("phases/opaque3d/instance_slots"), Some(0.0));
        assert_eq!(get("preprocess/bind_groups_ranges"), Some(0.0));
    }

    #[test]
    fn record_sets_the_instance_batch_gauges() {
        let counters = Counters::new();
        frame_with_statics().record(&counters);
        let mut values = BTreeMap::new();
        counters.drain(|_, def, v| {
            values.insert(def.name.clone(), v);
        });
        let get = |n: &str| values.get(n).copied().flatten();
        assert_eq!(get("views/main/instance_batches"), Some(3.0));
        assert_eq!(get("views/main/instance_draws"), Some(4.0));
        assert_eq!(get("views/main/instance_slots"), Some(9000.0));
        assert_eq!(get("views/lamp_face/instance_batches"), Some(2.0));
        assert_eq!(get("views/all/instance_slots"), Some(15000.0));
        assert_eq!(get("phases/opaque3d/instance_draws"), Some(4.0));
        assert_eq!(get("phases/shadow/instance_slots"), Some(6000.0));
        assert_eq!(get("views/all/batch_sets"), Some(20.0), "15 main + 3 + 2 lamp faces");
        assert_eq!(get("preprocess/bind_groups"), Some(19.0));
        assert_eq!(get("preprocess/bind_groups_created"), Some(12.0));
        assert_eq!(get("preprocess/bind_groups_ranges"), Some(4.0));
        assert_eq!(get("preprocess/bind_groups_materialize"), Some(3.0));
    }

    #[test]
    fn static_material_gauges() {
        let counters = Counters::new();
        StaticMaterials { materials: 900, classes: 4, keys: 3, pipelines: 6, unclassed: 2 }.record(&counters);
        StaticBindGroups { bind_groups: 1, bound: 880, bindless: true }.record(&counters);
        let mut values = BTreeMap::new();
        counters.drain(|_, def, v| {
            values.insert(def.name.clone(), v);
        });
        let get = |n: &str| values.get(n).copied().flatten();
        assert_eq!(get(statics::MATERIALS), Some(900.0));
        assert_eq!(get(statics::CLASSES), Some(4.0));
        assert_eq!(get(statics::KEYS), Some(3.0));
        assert_eq!(get(statics::PIPELINES), Some(6.0));
        assert_eq!(get(statics::UNCLASSED), Some(2.0));
        assert_eq!(get(statics::BIND_GROUPS), Some(1.0));
        assert_eq!(get(statics::BOUND), Some(880.0));
        assert_eq!(get(statics::BINDLESS), Some(1.0));
    }

    #[test]
    fn table_merges_labels_and_frames() {
        let mut table = ViewTable::new();
        table.add(&frame());
        let mut second = frame();
        second.views.truncate(1);
        second.views[0].phases[0].items = 200;
        table.add(&second);
        assert_eq!(table.frames(), 2);
        let s = table.summary();
        let main = &s["main"];
        assert_eq!((main.kind, main.frames), (ViewKind::Main, 2));
        assert_eq!(main.metrics["opaque3d/items"], MetricSummary { mean: 150.0, max: 200.0 });
        assert_eq!(main.metrics["items"], MetricSummary { mean: 170.0, max: 220.0 });
        let lamp = &s["lamp.face0"];
        assert_eq!(lamp.frames, 1);
        assert!((lamp.views_per_frame - 2.0).abs() < 1e-9);
        assert_eq!(lamp.metrics["shadow/items"], MetricSummary { mean: 10.0, max: 10.0 });
        let json = serde_json::to_string(&s).unwrap();
        let back: BTreeMap<String, ViewSummary> = serde_json::from_str(&json).unwrap();
        assert_eq!(back, s);
    }
}
