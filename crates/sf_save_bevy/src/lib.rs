//! Bevy plugin for [`sf_save`] (`docs/specs/save.md`, slice S2): single-player save, load,
//! quicksave and autosave over a registry of section providers.
//!
//! - **Sections**: each owner registers a [`SaveSection`] (key or family, schema version,
//!   migration steps, typed snapshot and apply) with [`SaveAppExt::add_save_section`]. The
//!   framework's sections are built by [`sections`] over the game's resources; a game adds its
//!   own (`mw/globals`) the same way.
//! - **Save**: [`SaveRequest`] → gate ([`SaveGate`]: transient blockers wait, the adapter's
//!   `can_save` refuses) → snapshot of every provider at the frame's end (a tick boundary: after
//!   every `FixedUpdate` of the frame) → encode, write atomically, verify and rotate on the IO
//!   task pool → [`SaveDone`] with sizes per owner and timings.
//! - **Load**: [`LoadRequest`] → read, verify, migrate and decode off-thread → on the main
//!   thread: checks, reset hooks ([`SaveAppExt::on_load_reset`]), then every provider in apply
//!   order (a full replace, never a merge) → [`LoadDone`]. The game fades and runs its streaming
//!   burst on `LoadDone`.
//! - Sections no provider knows (a missing mod's) are kept in [`DormantSections`] and written
//!   back byte for byte (save.md "Dormancy"; the mod diff and `clean` are S3).
//! - [`Autosave`] fires on play time; quicksave/quickload are `SaveRequest::quick()` and
//!   [`LoadRequest::latest`] with [`SaveReason::Quick`].
//! - Counters `save/snapshot_ms`, `save/encode_ms`, `save/write_ms`, `save/bytes/<owner>`,
//!   `load/total_ms` when the app runs `SfDiagPlugin`.
//!
//! Net-ready (net.md "Saves"): the host world save holds one `sf/character/<player key>` per
//! player; guest character files (S4) carry those sections' exact bytes.

mod section;
pub mod sections;

#[cfg(test)]
mod tests;
#[cfg(test)]
mod tests_budget;
#[cfg(test)]
mod tests_formats;

use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use bevy_app::prelude::*;
use bevy_ecs::prelude::*;
use bevy_ecs::system::SystemState;
use bevy_platform::time::Instant;
use bevy_tasks::futures_lite::future;
use bevy_tasks::{IoTaskPool, Task, TaskPool};
use bevy_time::{Time, Virtual};
use sf_diag_bevy::DiagCounters;
use sf_diag_bevy::sf_diag::unit;
pub use sf_save;
use sf_save::{
    ContainerError, FileKind, Limits, MigrateError, ModRef, PartyMember, Registry, RotationPolicy, SaveFile, SaveFs, SaveMeta,
    SaveMode, SaveReason, SaveWriter, SectionFlags, SectionKey, SlotDir, StdFs, TocEntry, write_atomic,
};

pub use section::{SaveSection, read, write};
use section::{Decoded, Encoder, SectionIo};

// ---- configuration and state ---------------------------------------------------------------

/// Where and how saves are written.
#[derive(Resource, Clone, Debug)]
pub struct SaveConfig {
    /// The user-data folder (`<root>/saves/...`, save.md "Slots and files").
    pub root: PathBuf,
    pub mode: SaveMode,
    pub rotation: RotationPolicy,
    pub limits: Limits,
    /// Encode + write and read + decode on the IO task pool (default). `false` runs them inside
    /// the frame (tools, deterministic tests).
    pub background: bool,
}

impl Default for SaveConfig {
    fn default() -> Self {
        Self {
            root: PathBuf::from("."),
            mode: SaveMode::Single,
            rotation: RotationPolicy::default(),
            limits: Limits::default(),
            background: true,
        }
    }
}

/// The file layer (the real file system unless a test injects faults).
#[derive(Resource, Clone)]
pub struct SaveStorage(pub Arc<dyn SaveFs>);

impl Default for SaveStorage {
    fn default() -> Self {
        Self(Arc::new(StdFs))
    }
}

/// The running world's identity and counters, restored from `sf/meta` on load.
#[derive(Resource, Clone, Debug, Default, PartialEq, Eq)]
pub struct SaveSession {
    /// Random per playthrough; 0 until the first save of a new game picks one.
    pub world_id: u128,
    /// The save this state was loaded from or last written as (the next save's parent).
    pub last_save: Option<u128>,
    /// The last sequence number used in this world.
    pub sequence: u64,
    /// Play time (virtual time while the app runs).
    pub play_time: Duration,
    /// Fixed simulation ticks since the world began (`FixedLast` counts them).
    pub sim_tick: u64,
    /// The file last written or loaded.
    pub last_path: Option<PathBuf>,
}

impl SaveSession {
    /// Starts a new playthrough: a fresh world id, counters at zero.
    pub fn new_world(&mut self) {
        *self = Self { world_id: fresh_id(), ..Self::default() };
    }
}

/// Content and profile identity for `sf/meta`; the game fills it once the profile is loaded.
#[derive(Resource, Clone, Debug, Default, PartialEq)]
pub struct SaveIdentity {
    pub content_id: String,
    pub content_version: String,
    pub build_id: String,
    pub profile: String,
    pub load_order: Vec<ModRef>,
    pub profile_fingerprint: [u8; 32],
    pub party: Vec<PartyMember>,
}

/// Load-screen labels for `sf/meta`; the game keeps them current (display only).
#[derive(Resource, Clone, Debug, Default, PartialEq)]
pub struct SaveDisplay {
    pub character_name: String,
    pub character_level: Option<u32>,
    pub location_label: String,
    /// Game time in the clock's days.
    pub game_days: f64,
}

/// The adapter's content rule (Morrowind: no saving with enemies nearby).
pub type CanSave = Arc<dyn Fn(&World, SaveReason) -> Result<(), String> + Send + Sync>;

/// When saving may happen (save.md "Saving" step 1). Blockers (a load, a streaming burst, a door
/// crossing) make requests wait; `can_save` refuses them.
#[derive(Resource, Default, Clone)]
pub struct SaveGate {
    blockers: BTreeSet<String>,
    pub can_save: Option<CanSave>,
}

impl SaveGate {
    pub fn block(&mut self, reason: &str) {
        self.blockers.insert(reason.to_owned());
    }

    pub fn unblock(&mut self, reason: &str) {
        self.blockers.remove(reason);
    }

    pub fn is_blocked(&self) -> bool {
        !self.blockers.is_empty()
    }

    pub fn blockers(&self) -> impl Iterator<Item = &str> {
        self.blockers.iter().map(String::as_str)
    }
}

/// Autosave on play time (save.md open question 4: every 10 minutes by default).
#[derive(Resource, Clone, Debug, PartialEq, Eq)]
pub struct Autosave {
    /// `None` disables the timer (rest, guest-left and exit saves are the game's requests).
    pub interval: Option<Duration>,
    /// Play time since the last save or load.
    pub elapsed: Duration,
}

impl Default for Autosave {
    fn default() -> Self {
        Self { interval: Some(Duration::from_secs(600)), elapsed: Duration::ZERO }
    }
}

/// Sections of the loaded save that no provider knows (a missing mod's), written back unchanged.
#[derive(Resource, Clone, Debug, Default, PartialEq, Eq)]
pub struct DormantSections {
    /// Copied byte for byte (same codec, version and hash).
    stored: Vec<(TocEntry, Vec<u8>)>,
    /// Kept by a provider that couldn't apply them (a script context that isn't running).
    raw: Vec<(SectionKey, u32, Vec<u8>)>,
}

impl DormantSections {
    /// Keeps a section's current-version bytes for the next save.
    pub fn keep_raw(&mut self, key: SectionKey, version: u32, bytes: Vec<u8>) {
        self.raw.retain(|(k, _, _)| *k != key);
        self.raw.push((key, version, bytes));
    }

    pub fn keys(&self) -> impl Iterator<Item = &SectionKey> {
        self.stored.iter().map(|(e, _)| &e.key).chain(self.raw.iter().map(|(k, _, _)| k))
    }

    pub fn is_empty(&self) -> bool {
        self.stored.is_empty() && self.raw.is_empty()
    }
}

type ResetHook = Arc<dyn Fn(&mut World) + Send + Sync>;

/// Every registered provider, in apply order, with the version registry.
#[derive(Resource, Default)]
pub struct SaveSections {
    sections: Vec<SaveSection>,
    registry: Registry,
    resets: Vec<ResetHook>,
}

impl SaveSections {
    /// Registers a provider. Panics on a family registered twice or a gap in its steps
    /// (programming errors).
    pub fn add(&mut self, section: SaveSection) {
        if let Err(e) = self.registry.register(section.spec.clone()) {
            panic!("sf_save_bevy: {e}");
        }
        let at = self.sections.partition_point(|s| s.order <= section.order);
        self.sections.insert(at, section);
    }

    pub fn registry(&self) -> &Registry {
        &self.registry
    }

    pub fn sections(&self) -> &[SaveSection] {
        &self.sections
    }
}

/// Registration on the app.
pub trait SaveAppExt {
    /// Adds a section provider ([`SaveSections::add`]).
    fn add_save_section(&mut self, section: SaveSection) -> &mut Self;
    /// Runs `f` after a load is verified and before any section applies: despawn gameplay
    /// entities, reset resources no section covers.
    fn on_load_reset(&mut self, f: impl Fn(&mut World) + Send + Sync + 'static) -> &mut Self;
}

impl SaveAppExt for App {
    fn add_save_section(&mut self, section: SaveSection) -> &mut Self {
        self.world_mut().get_resource_or_insert_with(SaveSections::default).add(section);
        self
    }

    fn on_load_reset(&mut self, f: impl Fn(&mut World) + Send + Sync + 'static) -> &mut Self {
        self.world_mut().get_resource_or_insert_with(SaveSections::default).resets.push(Arc::new(f));
        self
    }
}

// ---- messages ------------------------------------------------------------------------------

/// Where a save goes.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SaveTarget {
    /// A new file in the world's slot folder (`<sequence>-<reason>.sfsave`), rotated by reason.
    Slot,
    /// This file, replaced atomically; the previous one stays as `.bak` (overwriting a chosen
    /// save, tools).
    Path(PathBuf),
}

/// Asks for a save (UI, autosave timer, rest, a script). Data, so it can come over the net
/// later (net rule 8).
#[derive(Message, Clone, Debug, PartialEq, Eq)]
pub struct SaveRequest {
    pub reason: SaveReason,
    pub target: SaveTarget,
}

impl SaveRequest {
    pub fn slot(reason: SaveReason) -> Self {
        Self { reason, target: SaveTarget::Slot }
    }

    pub fn quick() -> Self {
        Self::slot(SaveReason::Quick)
    }

    pub fn to_path(path: impl Into<PathBuf>) -> Self {
        Self { reason: SaveReason::Manual, target: SaveTarget::Path(path.into()) }
    }
}

/// Where a load comes from.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum LoadSource {
    Path(PathBuf),
    /// The newest readable save of the current world (of `reason` if given): quickload is
    /// `Latest(Some(Quick))`.
    Latest(Option<SaveReason>),
}

#[derive(Message, Clone, Debug, PartialEq, Eq)]
pub struct LoadRequest(pub LoadSource);

impl LoadRequest {
    pub fn path(path: impl Into<PathBuf>) -> Self {
        Self(LoadSource::Path(path.into()))
    }

    pub fn latest(reason: Option<SaveReason>) -> Self {
        Self(LoadSource::Latest(reason))
    }
}

/// A finished save.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SaveReport {
    pub path: PathBuf,
    pub reason: SaveReason,
    pub sequence: u64,
    pub save_id: u128,
    pub bytes: u64,
    /// Stored bytes per section owner (bloat watch, save.md "Corruption safety" 7).
    pub per_owner: BTreeMap<String, u64>,
    pub sections: usize,
    /// Main thread.
    pub snapshot: Duration,
    /// Off-thread.
    pub encode: Duration,
    pub write: Duration,
    /// Older saves removed by rotation.
    pub rotated: Vec<PathBuf>,
    /// The directory sync succeeded.
    pub durable: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, thiserror::Error)]
pub enum SaveError {
    #[error("saving refused: {0}")]
    Refused(String),
    #[error("saving cancelled: {0}")]
    Cancelled(&'static str),
    #[error("snapshot of {section}: {message}")]
    Snapshot { section: String, message: String },
    #[error("encoding: {0}")]
    Encode(String),
    #[error("writing: {0}")]
    Write(String),
}

#[derive(Message, Clone, Debug, PartialEq, Eq)]
pub struct SaveDone(pub Result<SaveReport, SaveError>);

/// A finished load.
#[derive(Clone, Debug, PartialEq)]
pub struct LoadReport {
    pub path: PathBuf,
    pub meta: SaveMeta,
    /// Sections migrated: key, stored version, current version.
    pub migrated: Vec<(SectionKey, u32, u32)>,
    /// Kept dormant (no provider).
    pub dormant: Vec<SectionKey>,
    /// Optional sections skipped as corrupt.
    pub skipped: Vec<(SectionKey, String)>,
    /// Off-thread: read, verify, migrate, decode.
    pub prepare: Duration,
    /// Main thread: checks, resets, apply.
    pub apply: Duration,
}

#[derive(Clone, Debug, PartialEq, Eq, thiserror::Error)]
pub enum LoadError {
    #[error("no save to load")]
    NotFound,
    #[error("reading {0}")]
    Read(String),
    #[error(transparent)]
    Container(#[from] ContainerError),
    #[error("{0} is a character file, not a world save")]
    NotAWorld(String),
    #[error(transparent)]
    Migrate(#[from] MigrateError),
    #[error("section {key}: {message}")]
    Decode { key: SectionKey, message: String },
    #[error("section {key} refused: {message}")]
    Check { key: SectionKey, message: String },
    #[error("applying {section}: {message} (the world may be partly loaded)")]
    Apply { section: String, message: String },
}

#[derive(Message, Clone, Debug, PartialEq)]
pub struct LoadDone(pub Result<LoadReport, LoadError>);

// ---- plugin --------------------------------------------------------------------------------

/// The plugin's systems: one exclusive driver in `Last`.
#[derive(SystemSet, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct SaveSystems;

/// Adds the messages, resources (kept if the app inserted them first) and systems.
#[derive(Default)]
pub struct SfSavePlugin {
    pub config: SaveConfig,
}

impl Plugin for SfSavePlugin {
    fn build(&self, app: &mut App) {
        if !app.world().contains_resource::<SaveConfig>() {
            app.insert_resource(self.config.clone());
        }
        app.add_message::<SaveRequest>()
            .add_message::<LoadRequest>()
            .add_message::<SaveDone>()
            .add_message::<LoadDone>()
            .init_resource::<SaveSections>()
            .init_resource::<SaveStorage>()
            .init_resource::<SaveSession>()
            .init_resource::<SaveIdentity>()
            .init_resource::<SaveDisplay>()
            .init_resource::<SaveGate>()
            .init_resource::<Autosave>()
            .init_resource::<DormantSections>()
            .init_resource::<sections::BlobVersions>()
            .init_resource::<Runner>()
            .add_systems(First, tick_play_time)
            .add_systems(FixedLast, count_tick)
            .add_systems(Last, drive.in_set(SaveSystems));
    }
}

fn tick_play_time(time: Option<Res<Time<Virtual>>>, mut session: ResMut<SaveSession>, mut auto: ResMut<Autosave>, mut out: MessageWriter<SaveRequest>) {
    let Some(dt) = time.map(|t| t.delta()) else { return };
    session.play_time += dt;
    auto.elapsed += dt;
    if auto.interval.is_some_and(|i| auto.elapsed >= i) {
        auto.elapsed = Duration::ZERO;
        out.write(SaveRequest::slot(SaveReason::Auto));
    }
}

fn count_tick(mut session: ResMut<SaveSession>) {
    session.sim_tick += 1;
}

/// A random id (world, save): blake3 over the clock, a process counter and the process id.
fn fresh_id() -> u128 {
    static COUNTER: AtomicU64 = AtomicU64::new(0);
    let mut h = blake3::Hasher::new();
    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default();
    h.update(&now.as_nanos().to_le_bytes());
    h.update(&COUNTER.fetch_add(1, Ordering::Relaxed).to_le_bytes());
    h.update(&std::process::id().to_le_bytes());
    let bytes: [u8; 16] = h.finalize().as_bytes()[..16].try_into().expect("16 bytes");
    u128::from_le_bytes(bytes).max(1)
}

// ---- jobs ------------------------------------------------------------------------------------

/// Background work or its inline result.
enum Job<T> {
    Task(Task<T>),
    Done(Option<T>),
}

impl<T: Send + 'static> Job<T> {
    fn start(background: bool, f: impl FnOnce() -> T + Send + 'static) -> Self {
        if background {
            Self::Task(IoTaskPool::get_or_init(TaskPool::new).spawn(async move { f() }))
        } else {
            Self::Done(Some(f()))
        }
    }

    fn poll(&mut self) -> Option<T> {
        match self {
            Self::Task(t) if t.is_finished() => future::block_on(future::poll_once(t)),
            Self::Task(_) => None,
            Self::Done(r) => r.take(),
        }
    }
}

struct SaveOutcome {
    path: PathBuf,
    sequence: u64,
    save_id: u128,
    bytes: u64,
    per_owner: BTreeMap<String, u64>,
    sections: usize,
    encode: Duration,
    write: Duration,
    rotated: Vec<PathBuf>,
    durable: bool,
}

enum Resolved {
    Slot(SlotDir),
    Path(PathBuf),
}

struct SaveJob {
    fs: Arc<dyn SaveFs>,
    meta: SaveMeta,
    /// The world's last sequence; the job takes the next free one in the folder.
    last_sequence: u64,
    sections: Vec<(SectionKey, u32, SectionFlags, Encoder)>,
    dormant: DormantSections,
    target: Resolved,
}

impl SaveJob {
    fn run(self) -> Result<SaveOutcome, SaveError> {
        let SaveJob { fs, mut meta, last_sequence, sections, dormant, target } = self;
        let t0 = Instant::now();
        meta.sequence = match &target {
            Resolved::Slot(slot) => slot.next_sequence(&*fs).max(last_sequence + 1),
            Resolved::Path(_) => last_sequence + 1,
        };
        let mut writer = SaveWriter::new(FileKind::World, &meta);
        let live: BTreeSet<SectionKey> = sections.iter().map(|(k, ..)| k.clone()).collect();
        for (key, version, flags, encode) in sections {
            writer.add(key, version, flags, &encode()).map_err(|e| SaveError::Encode(e.to_string()))?;
        }
        for (entry, stored) in dormant.stored.iter().filter(|(e, _)| !live.contains(&e.key)) {
            writer.add_stored(entry, stored, entry.flags.with(SectionFlags::DORMANT)).map_err(|e| SaveError::Encode(e.to_string()))?;
        }
        for (key, version, raw) in dormant.raw.into_iter().filter(|(k, ..)| !live.contains(k)) {
            writer.add(key, version, SectionFlags::DORMANT, &raw).map_err(|e| SaveError::Encode(e.to_string()))?;
        }
        let bytes = writer.finish();
        let file = SaveFile::open(bytes).map_err(|e| SaveError::Encode(e.to_string()))?;
        let mut per_owner = BTreeMap::new();
        for e in file.entries() {
            *per_owner.entry(e.key.owner().to_owned()).or_default() += e.stored_len;
        }
        let encode = t0.elapsed();
        let t1 = Instant::now();
        let (written, rotated) = match &target {
            Resolved::Slot(slot) => {
                let w = slot.write(&*fs, meta.sequence, meta.reason, file.bytes()).map_err(|e| SaveError::Write(e.to_string()))?;
                (w.written, w.rotated)
            }
            Resolved::Path(path) => (write_atomic(&*fs, path, file.bytes(), true).map_err(|e| SaveError::Write(e.to_string()))?, Vec::new()),
        };
        Ok(SaveOutcome {
            path: written.path,
            sequence: meta.sequence,
            save_id: meta.save_id,
            bytes: file.len(),
            per_owner,
            sections: file.entries().len(),
            encode,
            write: t1.elapsed(),
            rotated,
            durable: written.durable,
        })
    }
}

/// A verified, migrated and decoded save, ready to apply.
struct Loaded {
    path: PathBuf,
    meta: SaveMeta,
    /// (provider index, key, value), in file order.
    values: Vec<(usize, SectionKey, Decoded)>,
    dormant: Vec<(TocEntry, Vec<u8>)>,
    migrated: Vec<(SectionKey, u32, u32)>,
    skipped: Vec<(SectionKey, String)>,
    prepare: Duration,
}

struct LoadJob {
    fs: Arc<dyn SaveFs>,
    source: LoadSource,
    slot: SlotDir,
    limits: Limits,
    registry: Registry,
    /// The providers' families and decoders, by index into `SaveSections::sections`.
    providers: Vec<(sf_save::Family, Arc<dyn SectionIo>)>,
}

fn bad_key(e: &ContainerError) -> Option<&SectionKey> {
    match e {
        ContainerError::SectionHash { key } | ContainerError::Decompress { key, .. } => Some(key),
        _ => None,
    }
}

impl LoadJob {
    fn run(self) -> Result<Loaded, LoadError> {
        let t0 = Instant::now();
        let path = match &self.source {
            LoadSource::Path(p) => p.clone(),
            LoadSource::Latest(reason) => self.slot.latest(&*self.fs, *reason).ok_or(LoadError::NotFound)?.path,
        };
        let bytes = self.fs.read(&path).map_err(|e| LoadError::Read(format!("{}: {e}", path.display())))?;
        let file = SaveFile::open_with(bytes, &self.limits)?;
        if file.kind() != FileKind::World {
            return Err(LoadError::NotAWorld(path.display().to_string()));
        }
        let meta = file.meta()?;
        let mut skipped = Vec::new();
        for err in file.verify() {
            match bad_key(&err).and_then(|k| file.entry(k)) {
                Some(e) if e.flags.contains(SectionFlags::OPTIONAL) => skipped.push((e.key.clone(), err.to_string())),
                _ => return Err(err.into()),
            }
        }
        let mut out = Loaded { path, meta, values: Vec::new(), dormant: Vec::new(), migrated: Vec::new(), skipped, prepare: Duration::ZERO };
        for e in file.entries().iter().skip(1) {
            if out.skipped.iter().any(|(k, _)| *k == e.key) {
                continue;
            }
            let Some(spec) = self.registry.spec(&e.key) else {
                out.dormant.push((e.clone(), file.stored(&e.key)?.to_vec()));
                continue;
            };
            let (raw, migrated) = self.registry.migrate(&e.key, e.version, file.raw(&e.key)?)?;
            if migrated {
                out.migrated.push((e.key.clone(), e.version, spec.current()));
            }
            let index = self.providers.iter().position(|(f, _)| f == spec.family()).expect("every registered spec has a provider");
            let value = self.providers[index].1.decode(&raw).map_err(|message| LoadError::Decode { key: e.key.clone(), message })?;
            out.values.push((index, e.key.clone(), value));
        }
        out.prepare = t0.elapsed();
        Ok(out)
    }
}

/// A save in flight: the job, its reason and the main-thread snapshot time.
type Saving = (Job<Result<SaveOutcome, SaveError>>, SaveReason, Duration);

/// Queued requests and the job in flight.
#[derive(Resource, Default)]
struct Runner {
    saves: VecDeque<SaveRequest>,
    load: Option<LoadRequest>,
    saving: Option<Saving>,
    loading: Option<Job<Result<Loaded, LoadError>>>,
}

type Requests = SystemState<(MessageReader<'static, 'static, SaveRequest>, MessageReader<'static, 'static, LoadRequest>)>;

/// Polls jobs, applies a finished load, then starts the next load or save.
fn drive(world: &mut World, requests: &mut Requests) {
    let (saves, load) = match requests.get_mut(world) {
        Ok((mut s, mut l)) => (s.read().cloned().collect::<Vec<_>>(), l.read().last().cloned()),
        Err(_) => (Vec::new(), None),
    };
    let mut runner = world.remove_resource::<Runner>().unwrap_or_default();
    if load.is_some() {
        // A load replaces the world: saves queued before it would save the state being left.
        for _ in runner.saves.drain(..) {
            world.write_message(SaveDone(Err(SaveError::Cancelled("a load was requested"))));
        }
        runner.load = load;
    }
    runner.saves.extend(saves);
    finish_save(world, &mut runner);
    finish_load(world, &mut runner);
    if runner.saving.is_none() && runner.loading.is_none() {
        if let Some(req) = runner.load.take() {
            runner.loading = Some(start_load(world, req));
            finish_load(world, &mut runner);
        } else {
            start_save(world, &mut runner);
            finish_save(world, &mut runner);
        }
    }
    world.insert_resource(runner);
}

fn start_save(world: &mut World, runner: &mut Runner) {
    if world.resource::<SaveGate>().is_blocked() {
        return;
    }
    let Some(req) = runner.saves.pop_front() else { return };
    let can_save = world.resource::<SaveGate>().can_save.clone();
    if let Some(Err(why)) = can_save.map(|f| f(world, req.reason)) {
        world.write_message(SaveDone(Err(SaveError::Refused(why))));
        return;
    }
    let t0 = Instant::now();
    if world.resource::<SaveSession>().world_id == 0 {
        // The first save of a playthrough names its world; play time and ticks keep counting.
        world.resource_mut::<SaveSession>().world_id = fresh_id();
    }
    let mut sections = Vec::new();
    {
        let providers = world.resource::<SaveSections>();
        for s in &providers.sections {
            match s.io.snapshot(world) {
                Ok(list) => sections.extend(list.into_iter().map(|(k, enc)| (k, s.spec.current(), s.flags, enc))),
                Err(message) => {
                    let section = s.spec.family().to_string();
                    world.write_message(SaveDone(Err(SaveError::Snapshot { section, message })));
                    return;
                }
            }
        }
    }
    let config = world.resource::<SaveConfig>().clone();
    let session = world.resource::<SaveSession>().clone();
    let target = match req.target {
        SaveTarget::Slot => {
            let mut slot = SlotDir::world(&config.root, config.mode, session.world_id);
            slot.policy = config.rotation;
            Resolved::Slot(slot)
        }
        SaveTarget::Path(p) => Resolved::Path(p),
    };
    let job = SaveJob {
        fs: world.resource::<SaveStorage>().0.clone(),
        meta: build_meta(world, req.reason),
        last_sequence: session.sequence,
        sections,
        dormant: world.resource::<DormantSections>().clone(),
        target,
    };
    let snapshot = t0.elapsed();
    runner.saving = Some((Job::start(config.background, move || job.run()), req.reason, snapshot));
}

fn build_meta(world: &World, reason: SaveReason) -> SaveMeta {
    let session = world.resource::<SaveSession>();
    let id = world.resource::<SaveIdentity>();
    let display = world.resource::<SaveDisplay>();
    SaveMeta {
        world_id: session.world_id,
        save_id: fresh_id(),
        parent: session.last_save,
        sequence: 0,
        reason,
        wall_clock: SystemTime::now().duration_since(UNIX_EPOCH).map_or(0, |d| i64::try_from(d.as_secs()).unwrap_or(i64::MAX)),
        game_days: display.game_days,
        play_time_ms: u64::try_from(session.play_time.as_millis()).unwrap_or(u64::MAX),
        sim_tick: session.sim_tick,
        framework_version: env!("CARGO_PKG_VERSION").to_owned(),
        content_id: id.content_id.clone(),
        content_version: id.content_version.clone(),
        build_id: id.build_id.clone(),
        profile: id.profile.clone(),
        load_order: id.load_order.clone(),
        profile_fingerprint: id.profile_fingerprint,
        grid_cell: sf_coords::GRID_CELL,
        party: id.party.clone(),
        character_name: display.character_name.clone(),
        character_level: display.character_level,
        location_label: display.location_label.clone(),
    }
}

fn counters(world: &World) -> Option<DiagCounters> {
    world.get_resource::<DiagCounters>().cloned()
}

fn ms(d: Duration) -> f64 {
    d.as_secs_f64() * 1000.0
}

fn finish_save(world: &mut World, runner: &mut Runner) {
    let Some((job, reason, snapshot)) = &mut runner.saving else { return };
    let Some(result) = job.poll() else { return };
    let (reason, snapshot) = (*reason, *snapshot);
    runner.saving = None;
    let report = result.map(|o| {
        let mut session = world.resource_mut::<SaveSession>();
        session.sequence = o.sequence;
        session.last_save = Some(o.save_id);
        session.last_path = Some(o.path.clone());
        world.resource_mut::<Autosave>().elapsed = Duration::ZERO;
        if let Some(c) = counters(world) {
            c.0.add_named("save/snapshot_ms", unit::MS, ms(snapshot));
            c.0.add_named("save/encode_ms", unit::MS, ms(o.encode));
            c.0.add_named("save/write_ms", unit::MS, ms(o.write));
            for (owner, bytes) in &o.per_owner {
                #[expect(clippy::cast_precision_loss, reason = "save sizes are far below 2^52")]
                c.0.set_named(&format!("save/bytes/{owner}"), unit::BYTES, *bytes as f64);
            }
        }
        SaveReport {
            path: o.path,
            reason,
            sequence: o.sequence,
            save_id: o.save_id,
            bytes: o.bytes,
            per_owner: o.per_owner,
            sections: o.sections,
            snapshot,
            encode: o.encode,
            write: o.write,
            rotated: o.rotated,
            durable: o.durable,
        }
    });
    if let Err(e) = &report {
        log::error!("sf_save_bevy: {e}");
    }
    world.write_message(SaveDone(report));
}

fn start_load(world: &mut World, req: LoadRequest) -> Job<Result<Loaded, LoadError>> {
    let config = world.resource::<SaveConfig>().clone();
    let providers = world.resource::<SaveSections>();
    let job = LoadJob {
        fs: world.resource::<SaveStorage>().0.clone(),
        source: req.0,
        slot: SlotDir::world(&config.root, config.mode, world.resource::<SaveSession>().world_id),
        limits: config.limits,
        registry: providers.registry.clone(),
        providers: providers.sections.iter().map(|s| (s.spec.family().clone(), s.io.clone())).collect(),
    };
    Job::start(config.background, move || job.run())
}

fn finish_load(world: &mut World, runner: &mut Runner) {
    let Some(job) = &mut runner.loading else { return };
    let Some(result) = job.poll() else { return };
    runner.loading = None;
    let done = result.and_then(|loaded| apply(world, loaded));
    if let Err(e) = &done {
        log::error!("sf_save_bevy: load failed: {e}");
    }
    world.write_message(LoadDone(done));
}

/// Checks, resets, then applies every provider in order (save.md "Loading" step 4).
fn apply(world: &mut World, loaded: Loaded) -> Result<LoadReport, LoadError> {
    let t0 = Instant::now();
    let Loaded { path, meta, values, dormant, migrated, skipped, prepare } = loaded;
    // The tick count is the meta's unless `sf/clock` restores it.
    world.resource_mut::<SaveSession>().sim_tick = meta.sim_tick;
    let providers = world.remove_resource::<SaveSections>().unwrap_or_default();
    let result = apply_with(world, &providers, values, dormant.clone());
    world.insert_resource(providers);
    result?;
    let mut session = world.resource_mut::<SaveSession>();
    session.world_id = meta.world_id;
    session.last_save = Some(meta.save_id);
    session.sequence = meta.sequence;
    session.play_time = Duration::from_millis(meta.play_time_ms);
    session.last_path = Some(path.clone());
    world.resource_mut::<Autosave>().elapsed = Duration::ZERO;
    let apply = t0.elapsed();
    if let Some(c) = counters(world) {
        c.0.add_named("load/total_ms", unit::MS, ms(prepare + apply));
    }
    let dormant = dormant.into_iter().map(|(e, _)| e.key).collect();
    Ok(LoadReport { path, meta, migrated, dormant, skipped, prepare, apply })
}

fn apply_with(
    world: &mut World,
    providers: &SaveSections,
    values: Vec<(usize, SectionKey, Decoded)>,
    dormant: Vec<(TocEntry, Vec<u8>)>,
) -> Result<(), LoadError> {
    for (index, key, value) in &values {
        if let Some(check) = &providers.sections[*index].check {
            check(world, &**value).map_err(|message| LoadError::Check { key: key.clone(), message })?;
        }
    }
    for reset in &providers.resets {
        reset(world);
    }
    *world.resource_mut::<DormantSections>() = DormantSections { stored: dormant, raw: Vec::new() };
    let mut by_provider: Vec<Vec<(SectionKey, Decoded)>> = providers.sections.iter().map(|_| Vec::new()).collect();
    for (index, key, value) in values {
        by_provider[index].push((key, value));
    }
    for (section, values) in providers.sections.iter().zip(by_provider) {
        section.io.apply(world, values).map_err(|message| LoadError::Apply { section: section.spec.family().to_string(), message })?;
    }
    Ok(())
}
