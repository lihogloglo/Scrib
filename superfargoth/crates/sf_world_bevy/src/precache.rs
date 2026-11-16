//! Pipeline warm-up W3 (`docs/specs/pipeline-warmup.md` §5.2): the main-world surface of the
//! precache (vendored `bevy_pbr` patch `own, W3`, `bevy_pbr::precache`).
//!
//! A client asks [`Precache::request`] for a set of [`Specimen`]s under a tag ("arrival",
//! "catalog") and polls [`Precache::status`]; the render world specialises each specimen for every
//! known view kind (camera main pass and prepass, directional and perspective shadows) with the same
//! key code as the draw path, so an entity drawn later gets the precached pipeline (a hash hit).
//! Counters (`sf_diag`): `precache/requested`, `/hit`, `/missed` (distinct pipelines the draw path
//! needed that a specimen had / had not predicted, UE's validation set) and `/pending_assets`.
//! Low priority (W4b1-fix, W4b1-throughput): [`Precache::set_limits`] caps the precache pipelines
//! queued or compiling at once, lower while a draw-path pipeline is queued or compiling
//! (`compile_pool::precache_limit`; held-back specimens count as queued), counters
//! `precache/throttled`, `/outstanding`, `/draw_waiting` and `/limit` (the cap applied).
//! Each draw pipeline the precache missed is logged once ("precache missed: ...").
//! W4 builds the catalog and wires the status into the reveal gate.

use std::sync::{Arc, Mutex, MutexGuard};

use bevy_app::{App, Plugin};
use bevy_asset::{Asset, Handle};
use bevy_camera::visibility::VisibilityRange;
use bevy_ecs::prelude::*;
use bevy_mesh::Mesh;
use bevy_pbr::Lightmap;
use bevy_render::{
    ExtractSchedule, Render, RenderApp, RenderSystems,
    render_resource::{CachedPipelineState, PipelineCache},
};
use bevy_shader::ShaderCacheError;
use sf_diag_bevy::DiagCounters;

pub use bevy_pbr::precache::{EntityKeyFlags as SpecimenFlags, PipelinePrecache, PrecacheLimit, PrecacheResolved, PrecacheStats, PrecacheWaiting, Specimen};

/// Where one specimen (or one pipeline) stands.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Progress {
    /// Its mesh or material is not prepared in the render world yet.
    PendingAssets,
    /// Waiting for a camera view, or a pipeline is queued (or waits for a shader to load).
    Queued,
    /// A pipeline is compiling.
    Compiling,
    /// Every pipeline is ready.
    Ready,
    /// A specialisation or a pipeline failed.
    Failed,
}

/// The counts of one tag's specimens.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct PrecacheStatus {
    pub pending_assets: u32,
    pub queued: u32,
    pub compiling: u32,
    pub ready: u32,
    pub failed: u32,
}

impl PrecacheStatus {
    pub fn add(&mut self, progress: Progress) {
        let slot = match progress {
            Progress::PendingAssets => &mut self.pending_assets,
            Progress::Queued => &mut self.queued,
            Progress::Compiling => &mut self.compiling,
            Progress::Ready => &mut self.ready,
            Progress::Failed => &mut self.failed,
        };
        *slot += 1;
    }

    #[must_use]
    pub fn total(&self) -> u32 {
        self.pending_assets + self.queued + self.compiling + self.ready + self.failed
    }

    /// Nothing left to wait for (failures included: they never finish).
    #[must_use]
    pub fn is_settled(&self) -> bool {
        self.pending_assets + self.queued + self.compiling == 0
    }
}

/// A pipeline's progress from its cache state. A shader still loading is retried by the cache, so
/// it counts as queued, not failed.
#[must_use]
pub fn pipeline_progress(state: &CachedPipelineState) -> Progress {
    match state {
        CachedPipelineState::Queued
        | CachedPipelineState::Err(ShaderCacheError::ShaderNotLoaded(_) | ShaderCacheError::ShaderImportNotYetAvailable) => {
            Progress::Queued
        }
        CachedPipelineState::Creating(_) => Progress::Compiling,
        CachedPipelineState::Ok(_) => Progress::Ready,
        CachedPipelineState::Err(_) => Progress::Failed,
    }
}

/// A resolved specimen's progress: failed if any specialisation or pipeline failed, else its least
/// advanced pipeline (none: ready).
#[must_use]
pub fn specimen_progress(pipelines: impl IntoIterator<Item = Progress>, errors: u32) -> Progress {
    let mut worst = Progress::Ready;
    let mut failed = errors > 0;
    for p in pipelines {
        if p == Progress::Failed {
            failed = true;
        } else {
            worst = worst.min(p);
        }
    }
    if failed { Progress::Failed } else { worst }
}

/// The status of every tag: waiting specimens (assets or a camera view) then resolved ones, rated
/// with `pipeline` (the render world passes the pipeline cache's states).
#[must_use]
pub fn tag_status(
    tags: usize,
    waiting: &[PrecacheWaiting],
    resolved: &[PrecacheResolved],
    mut pipeline: impl FnMut(bevy_render::render_resource::CachedRenderPipelineId) -> Progress,
) -> Vec<PrecacheStatus> {
    let mut status = vec![PrecacheStatus::default(); tags];
    for w in waiting {
        if let Some(s) = status.get_mut(w.tag as usize) {
            s.add(if w.assets_ready { Progress::Queued } else { Progress::PendingAssets });
        }
    }
    for r in resolved {
        if let Some(s) = status.get_mut(r.tag as usize) {
            s.add(specimen_progress(r.pipelines.iter().map(|&id| pipeline(id)), r.errors));
        }
    }
    status
}

/// The entity-level key bits of an entity with these components (what the draw path would read).
/// `previous_skin` / `previous_morph`: skinned / morphed (the precache also covers the first frame,
/// before the previous-frame data exists).
#[must_use]
pub fn specimen_flags(
    shadow_caster: bool,
    visibility_range: Option<&VisibilityRange>,
    lightmap: Option<&Lightmap>,
    previous_skin: bool,
    previous_morph: bool,
) -> SpecimenFlags {
    let mut flags = SpecimenFlags::empty();
    flags.set(SpecimenFlags::SHADOW_CASTER, shadow_caster);
    flags.set(SpecimenFlags::VISIBILITY_RANGE_DITHER, visibility_range.is_some_and(|r| !r.is_abrupt()));
    if let Some(lightmap) = lightmap {
        flags |= SpecimenFlags::LIGHTMAPPED;
        flags.set(SpecimenFlags::LIGHTMAP_BICUBIC_SAMPLING, lightmap.bicubic_sampling);
    }
    flags.set(SpecimenFlags::HAS_PREVIOUS_SKIN, previous_skin);
    flags.set(SpecimenFlags::HAS_PREVIOUS_MORPH, previous_morph);
    flags
}

/// A specimen of `mesh` drawn with `material` (a `MaterialPlugin` material).
#[must_use]
pub fn specimen<M: Asset>(mesh: &Handle<Mesh>, material: &Handle<M>, flags: SpecimenFlags) -> Specimen {
    Specimen { mesh: mesh.id(), material: material.id().untyped(), flags }
}

#[derive(Default)]
struct Shared {
    tags: Vec<String>,
    inbox: Vec<(u32, Specimen)>,
    releases: Vec<u32>,
    status: Vec<PrecacheStatus>,
    /// Specimens per tag sent to the render world since its last status report (W4b1: without
    /// them a tag read between the extract and the report would look settled).
    in_flight: Vec<u32>,
    /// The render-world limit on precache pipelines queued or compiling (W4b1-fix, W4b1-throughput).
    limit: Option<PrecacheLimit>,
}

/// The precache, main world (added by [`PrecachePlugin`]). Cheap to clone (shared with the render
/// world).
#[derive(Resource, Clone, Default)]
pub struct Precache(Arc<Mutex<Shared>>);

impl Precache {
    fn lock(&self) -> MutexGuard<'_, Shared> {
        self.0.lock().unwrap_or_else(std::sync::PoisonError::into_inner)
    }

    fn tag_id(shared: &mut Shared, tag: &str) -> u32 {
        let index = shared.tags.iter().position(|t| t == tag).unwrap_or_else(|| {
            shared.tags.push(tag.to_owned());
            shared.tags.len() - 1
        });
        u32::try_from(index).unwrap_or(u32::MAX)
    }

    /// Queues `specimens` under `tag`; returns how many. A tag may be requested again (adds).
    pub fn request(&self, specimens: impl IntoIterator<Item = Specimen>, tag: &str) -> usize {
        let mut shared = self.lock();
        let id = Self::tag_id(&mut shared, tag);
        let before = shared.inbox.len();
        shared.inbox.extend(specimens.into_iter().map(|s| (id, s)));
        shared.inbox.len() - before
    }

    /// Where `tag`'s specimens stand (as of the last rendered frame; ones not yet sent to the render
    /// world count as queued).
    #[must_use]
    pub fn status(&self, tag: &str) -> PrecacheStatus {
        let shared = self.lock();
        let Some(id) = shared.tags.iter().position(|t| t == tag) else { return PrecacheStatus::default() };
        let mut status = shared.status.get(id).copied().unwrap_or_default();
        let unsent = shared.inbox.iter().filter(|(t, _)| *t as usize == id).count();
        status.queued += u32::try_from(unsent).unwrap_or(u32::MAX) + shared.in_flight.get(id).copied().unwrap_or(0);
        status
    }

    /// Forgets `tag`'s specimens (their pipelines stay compiled).
    pub fn release(&self, tag: &str) {
        let mut shared = self.lock();
        if let Some(id) = shared.tags.iter().position(|t| t == tag) {
            if let Some(n) = shared.in_flight.get_mut(id) {
                *n = 0;
            }
            let id = u32::try_from(id).unwrap_or(u32::MAX);
            shared.inbox.retain(|(t, _)| *t != id);
            shared.releases.push(id);
        }
    }

    /// Precache at low priority (W4b1-throughput): at most `limit.draw_waiting` precache pipelines
    /// queued or compiling at once in the render world while a draw-path pipeline is, `limit.idle`
    /// otherwise (`None`: no limit). Specimens held back count as queued. The compile pool plugin
    /// sets it every frame from the compile threads (`precache_limit`).
    pub fn set_limits(&self, limit: Option<PrecacheLimit>) {
        self.lock().limit = limit;
    }

    /// The same limit whether a draw waits or not ([`Self::set_limits`]).
    pub fn set_limit(&self, limit: Option<usize>) {
        self.set_limits(limit.map(PrecacheLimit::uniform));
    }

    /// The limit last set ([`Self::set_limits`]).
    #[must_use]
    pub fn limit(&self) -> Option<PrecacheLimit> {
        self.lock().limit
    }

    /// The new requests and releases, for the render world (they count as queued, in flight, until
    /// the next [`Self::report`]).
    pub(crate) fn take(&self) -> (Vec<(u32, Specimen)>, Vec<u32>) {
        let mut shared = self.lock();
        let inbox = std::mem::take(&mut shared.inbox);
        let releases = std::mem::take(&mut shared.releases);
        let tags = shared.tags.len();
        shared.in_flight.resize(tags, 0);
        for (tag, _) in &inbox {
            if let Some(n) = shared.in_flight.get_mut(*tag as usize) {
                *n += 1;
            }
        }
        (inbox, releases)
    }

    /// The render world's per-tag status, covering everything [`Self::take`] handed it.
    pub(crate) fn report(&self, status: Vec<PrecacheStatus>) {
        let mut shared = self.lock();
        shared.status = status;
        shared.in_flight.iter_mut().for_each(|n| *n = 0);
    }
}

/// The render world's handle on the main-world [`Precache`].
#[derive(Resource, Clone)]
struct PrecacheLink(Precache);

/// Adds [`Precache`] and the render-world queue. `WorldStreamingPlugin` adds it; inert until a
/// request.
#[derive(Default)]
pub struct PrecachePlugin;

impl Plugin for PrecachePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Precache>();
    }

    fn finish(&self, app: &mut App) {
        let precache = app.world().resource::<Precache>().clone();
        let counters = app.world().get_resource::<DiagCounters>().cloned();
        let Some(render) = app.get_sub_app_mut(RenderApp) else { return };
        if let Some(counters) = counters
            && !render.world().contains_resource::<DiagCounters>()
        {
            render.insert_resource(counters);
        }
        render
            .insert_resource(PrecacheLink(precache))
            .init_resource::<PipelinePrecache>()
            .add_systems(ExtractSchedule, take_requests)
            .add_systems(
                Render,
                (
                    // After the main and shadow draw specialisers (it sees this frame's draws).
                    bevy_pbr::precache::precache_specimens_systems(),
                    report_status.in_set(RenderSystems::Cleanup),
                ),
            );
    }
}

/// Moves new requests and releases into the render-world queue, and the limit.
fn take_requests(link: Res<PrecacheLink>, mut precache: ResMut<PipelinePrecache>) {
    let limit = link.0.limit();
    if precache.limit() != limit.map(PrecacheLimit::normalized) {
        precache.set_limits(limit);
    }
    let (inbox, releases) = link.0.take();
    if inbox.is_empty() && releases.is_empty() {
        return;
    }
    for tag in releases {
        precache.release(tag);
    }
    for (tag, specimen) in inbox {
        precache.request(tag, specimen);
    }
}

/// Per-tag status back to the main world, and the counters.
fn report_status(
    link: Res<PrecacheLink>,
    precache: Res<PipelinePrecache>,
    cache: Res<PipelineCache>,
    counters: Option<Res<DiagCounters>>,
    mut last: Local<PrecacheStats>,
) {
    let stats = precache.stats();
    if stats.requested == 0 {
        return;
    }
    let tags = link.0.lock().tags.len();
    let per_tag = tag_status(tags, precache.waiting(), precache.resolved(), |id| {
        pipeline_progress(cache.get_render_pipeline_state(id))
    });
    link.0.report(per_tag);
    if let Some(c) = counters {
        let count = sf_diag_bevy::sf_diag::unit::COUNT;
        #[allow(clippy::cast_precision_loss)]
        {
            c.add_named("precache/requested", count, (stats.requested - last.requested) as f64);
            c.add_named("precache/hit", count, (stats.hit - last.hit) as f64);
            c.add_named("precache/missed", count, (stats.missed - last.missed) as f64);
        }
        c.set_named("precache/pending_assets", count, f64::from(stats.pending_assets));
        c.set_named("precache/throttled", count, f64::from(stats.throttled));
        c.set_named("precache/outstanding", count, f64::from(stats.outstanding));
        c.set_named("precache/draw_waiting", count, f64::from(stats.draw_waiting));
        c.set_named("precache/limit", count, f64::from(stats.limit));
    }
    *last = stats;
}
