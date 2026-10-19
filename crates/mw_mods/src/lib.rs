//! Morrowind content as superfargoth mods (`superfargoth/docs/specs/mods.md`,
//! "Mods directory and profiles").
//!
//! [`MwAdapter`] is Bevyrrowind's [`ContentAdapter`] (`morrowind`). It
//! synthesizes:
//!
//! - the **builtin mod** [`BUILTIN`] (`mw`): layers the host passes in (the
//!   shipped animation graphs, the `mw` Luau runtime under `scripts/mw/`).
//!   [`pin_builtin`] keeps it first in every profile, so it mounts lowest and
//!   any mod overrides its files;
//! - one mod per **content file** (ESM/ESP) in `Data Files`, id from
//!   [`mw_records::mod_id_for`] (`morrowind.esm`): masters are dependencies,
//!   [`mw_records::KNOWN_ORDER`] adds `load_after` hints, `content.records`
//!   lists the file, and its archive (the `Morrowind.ini` BSA with the same
//!   stem, e.g. `Tribunal.bsa`) is its VFS layer;
//! - [`DATA`] (`morrowind-data`): the loose `Data Files` folder, after every
//!   plugin, with the ini archives no plugin claims below it;
//! - [`WEATHER`] (`morrowind-weather`): `mw_weather`'s records (presets,
//!   region climates, precipitation) built from every plugin's `REGN` and
//!   `Morrowind.ini` when its generated record file applies, after the content
//!   and before user mods, so record mods patch them; [`ensure_weather`] keeps
//!   it in every profile.
//!
//! Content files apply through [`mw_records::apply_plugin`], masters resolved
//! through [`RecordCx::source_of`]. User mods live in `<user data>/mods`,
//! profiles in `<user data>/profiles` ([`UserDirs`]); without a
//! `profiles/default.toml`, [`default_profile`] enables the builtin mod, the
//! ini's content files and `morrowind-data`, which loads exactly what the
//! pre-mods load path did.

use std::collections::BTreeMap;
use std::fs;
use std::io::{self, Read as _};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex, PoisonError};
use std::time::{Instant, UNIX_EPOCH};

use mw_install::MorrowindInstall;
use mw_records::{ContentFile, LoadWarning, mod_id_for};
use mw_vfs::BsaSource;
use semver::{Version, VersionReq};
use sf_mods::{
    ContentAdapter, IssueKind, LoadedProfile, ModId, ModIssue, ModManifest, ModRepository, NativeRecords, Profile, ProfileError,
    RecordCx, RecordFileError, load_profile,
};
use sf_records::{RecordDb, SourceId};
use sf_vfs::{DirSource, MemSource, VfsPath, VfsSource};
use tes3::esp::{FileType, Plugin};

/// The adapter's name (`ModManifest::adapter` of its synthesized mods).
pub const ADAPTER: &str = "morrowind";
/// The builtin mod's id; its scripts live under `scripts/mw/`.
pub const BUILTIN: &str = "mw";
/// The loose `Data Files` mod's id.
pub const DATA: &str = "morrowind-data";
/// The synthesized weather mod's id: `mw_weather`'s records (`REGN` + `Morrowind.ini`).
pub const WEATHER: &str = "morrowind-weather";
/// Its one generated record file (applied by [`MwAdapter`], not parsed).
pub const WEATHER_RECORDS: &str = "weather/morrowind-weather.mwweather";
/// The profile used when none is named.
pub const DEFAULT_PROFILE: &str = "default";
/// Environment variable overriding the user data folder.
pub const USER_DATA_ENV: &str = "BEVYRROWIND_USER_DATA";
/// VFS path of the `mw` Luau runtime in the builtin mod (`mw_script::emit::RUNTIME`).
pub const RUNTIME_PATH: &str = "scripts/mw/mw.luau";
/// VFS path of the generated function status table (`mw_script::emit::runtime_functions`).
pub const RUNTIME_FUNCTIONS_PATH: &str = "scripts/mw/functions.luau";

/// Modern formats and the legacy ones they replace (`mw_assets::TEXTURE_ALTERNATES`,
/// the glTF importer); see [`ContentAdapter::modern_alternates`].
pub const MODERN_ALTERNATES: &[(&str, &[&str])] =
    &[("ktx2", &["dds", "tga", "bmp"]), ("png", &["dds", "tga", "bmp"]), ("glb", &["nif"]), ("gltf", &["nif"])];

const VERSION: &str = env!("CARGO_PKG_VERSION");

/// The builtin mod's id.
pub fn builtin_id() -> ModId {
    ModId::new(BUILTIN).expect("valid id")
}

/// The weather mod's id ([`WEATHER`]).
pub fn weather_id() -> ModId {
    ModId::new(WEATHER).expect("valid id")
}

/// The loose `Data Files` mod's id.
pub fn data_id() -> ModId {
    ModId::new(DATA).expect("valid id")
}

/// Where the player's mods and profiles live: `--user-data`, else
/// [`USER_DATA_ENV`], else `<local data dir>/bevyrrowind` (beside the bake cache).
#[derive(Clone, Debug)]
pub struct UserDirs {
    pub root: PathBuf,
}

impl UserDirs {
    pub fn new(root: impl Into<PathBuf>) -> Self {
        Self { root: root.into() }
    }

    pub fn detect(explicit: Option<&Path>) -> Option<Self> {
        explicit
            .map(Path::to_path_buf)
            .or_else(|| std::env::var_os(USER_DATA_ENV).map(PathBuf::from))
            .or_else(|| dirs::data_local_dir().map(|d| d.join("bevyrrowind")))
            .map(Self::new)
    }

    /// `<root>/mods`: one folder per mod, each with a `mod.toml`.
    pub fn mods(&self) -> PathBuf {
        self.root.join("mods")
    }

    /// `<root>/profiles`: `<name>.toml` per profile.
    pub fn profiles(&self) -> PathBuf {
        self.root.join("profiles")
    }
}

/// The profile the game starts with when `profiles/default.toml` doesn't
/// exist: the builtin mod, the ini's content files (masters first, then
/// plugins, each in ini order), then `morrowind-data`.
pub fn default_profile(install: &MorrowindInstall) -> Profile {
    let mut files = install.game_files();
    files.sort_by_key(|f| !Path::new(f).extension().is_some_and(|e| e.eq_ignore_ascii_case("esm")));
    let plugins = files.iter().filter_map(|f| mod_id_for(f).ok());
    let ids = std::iter::once(builtin_id()).chain(plugins).chain([data_id(), weather_id()]);
    Profile::new(DEFAULT_PROFILE, ids)
}

/// Enables the weather mod in `profile` if it's missing, right after
/// `morrowind-data` (else last): its records are core data that record mods
/// patch, so they must exist before any user mod applies. Returns whether
/// the profile changed.
pub fn ensure_weather(profile: &mut Profile) -> bool {
    let id = weather_id();
    if profile.enabled.contains(&id) {
        return false;
    }
    let at = profile.enabled.iter().position(|m| *m == data_id()).map_or(profile.enabled.len(), |i| i + 1);
    profile.enabled.insert(at, id);
    true
}

/// Loads `<profiles>/<name>.toml`; the default profile falls back to
/// [`default_profile`] when its file doesn't exist. The builtin mod is pinned
/// first ([`pin_builtin`]). Returns whether the profile came from a file.
pub fn load_profile_named(user: &UserDirs, name: &str, install: &MorrowindInstall) -> Result<(Profile, bool), ProfileError> {
    let path = Profile::path_in(&user.profiles(), name);
    let (mut profile, from_file) =
        if name == DEFAULT_PROFILE && !path.exists() { (default_profile(install), false) } else { (Profile::load(&path)?, true) };
    pin_builtin(&mut profile);
    ensure_weather(&mut profile);
    Ok((profile, from_file))
}

/// Puts the builtin mod first in `profile` (enabling it if missing): it has
/// no dependencies, so the load order keeps it first and it mounts lowest.
/// Returns whether the profile changed.
pub fn pin_builtin(profile: &mut Profile) -> bool {
    let id = builtin_id();
    if profile.enabled.first() == Some(&id) {
        return false;
    }
    profile.enabled.retain(|m| *m != id);
    profile.enabled.insert(0, id);
    true
}

/// Bevyrrowind's [`ContentAdapter`] (module docs).
pub struct MwAdapter {
    install: MorrowindInstall,
    builtin: Vec<Arc<dyn VfsSource>>,
    apply: bool,
    /// Content file names found by `discover`.
    plugins: Mutex<Vec<String>>,
    /// Applied content files and their warnings, in load order.
    content: Mutex<(Vec<ContentFile>, Vec<LoadWarning>)>,
    /// Archives that failed to open (the mod loads without them).
    mount_warnings: Mutex<Vec<String>>,
}

impl MwAdapter {
    /// `builtin`: the builtin mod's layers, lowest first.
    pub fn new(install: MorrowindInstall, builtin: Vec<Arc<dyn VfsSource>>) -> Self {
        Self {
            install,
            builtin,
            apply: true,
            plugins: Mutex::default(),
            content: Mutex::default(),
            mount_warnings: Mutex::default(),
        }
    }

    /// Mount only: content files are not parsed (VFS-only commands).
    #[must_use]
    pub fn without_records(mut self) -> Self {
        self.apply = false;
        self
    }

    pub fn install(&self) -> &MorrowindInstall {
        &self.install
    }

    /// The content files applied by the last load, in load order, and their warnings.
    pub fn take_content(&self) -> (Vec<ContentFile>, Vec<LoadWarning>) {
        std::mem::take(&mut *self.content.lock().unwrap_or_else(PoisonError::into_inner))
    }

    /// Archives that failed to open during mounting.
    pub fn take_mount_warnings(&self) -> Vec<String> {
        std::mem::take(&mut *self.mount_warnings.lock().unwrap_or_else(PoisonError::into_inner))
    }

    fn warn_mount(&self, message: String) {
        self.mount_warnings.lock().unwrap_or_else(PoisonError::into_inner).push(message);
    }

    /// The ini archives a synthesized mod mounts: a content file's
    /// same-stem archive; for `morrowind-data`, those no content file claims.
    fn archives_of(&self, manifest: &ModManifest) -> Vec<String> {
        let stem = |name: &str| Path::new(name).file_stem().map(|s| s.to_string_lossy().to_ascii_lowercase());
        let archives = self.install.archives();
        if manifest.id.as_str() == DATA {
            let plugins = self.plugins.lock().unwrap_or_else(PoisonError::into_inner);
            let claimed: Vec<_> = plugins.iter().filter_map(|p| stem(p)).collect();
            archives.into_iter().filter(|a| stem(a).is_none_or(|s| !claimed.contains(&s))).collect()
        } else {
            archives.into_iter().filter(|a| stem(a).is_some() && stem(a) == stem(&manifest.name)).collect()
        }
    }

    fn open_archives(&self, manifest: &ModManifest) -> Vec<Arc<dyn VfsSource>> {
        let mut layers: Vec<Arc<dyn VfsSource>> = Vec::new();
        for name in self.archives_of(manifest) {
            let Some(path) = self.install.data_file(&name) else { continue }; // reported by discover
            match BsaSource::open(&path) {
                Ok(bsa) => {
                    if bsa.skipped() > 0 {
                        self.warn_mount(format!("{name}: {} entries with unrepresentable names skipped", bsa.skipped()));
                    }
                    layers.push(Arc::new(bsa));
                }
                Err(e) => self.warn_mount(format!("{name}: {e}")),
            }
        }
        layers
    }

    /// One synthesized mod per content file in `Data Files` (headers only).
    fn discover_plugins(&self, issues: &mut Vec<ModIssue>) -> Vec<ModManifest> {
        let mut names: Vec<String> = fs::read_dir(&self.install.data_files)
            .map(|entries| {
                entries
                    .filter_map(Result::ok)
                    .filter(|e| e.file_type().is_ok_and(|t| t.is_file()))
                    .filter_map(|e| e.file_name().into_string().ok())
                    .filter(|n| {
                        Path::new(n).extension().is_some_and(|e| e.eq_ignore_ascii_case("esm") || e.eq_ignore_ascii_case("esp"))
                    })
                    .collect()
            })
            .unwrap_or_default();
        names.sort_by_key(|n| n.to_ascii_lowercase());
        let issue = |message: String| ModIssue::new(None, IssueKind::Adapter { adapter: ADAPTER.into(), message });
        let mut manifests = Vec::with_capacity(names.len());
        let mut found = Vec::with_capacity(names.len());
        for name in names {
            let Ok(id) = mod_id_for(&name) else {
                issues.push(issue(format!("{name}: no valid mod id")));
                continue;
            };
            let masters = match read_masters(&self.install.data_files.join(&name)) {
                Ok((_, masters)) => masters,
                Err(e) => {
                    issues.push(issue(format!("{name}: unreadable header: {e}")));
                    continue;
                }
            };
            let mut manifest = ModManifest::synthesized(id, name.clone(), Version::new(0, 0, 0));
            manifest.description = format!("content file {name}");
            for master in &masters {
                match mod_id_for(master) {
                    Ok(dep) => {
                        manifest.dependencies.insert(dep, VersionReq::STAR);
                    }
                    Err(_) => issues.push(issue(format!("{name}: master {master:?} has no valid mod id"))),
                }
            }
            for (file, after) in mw_records::KNOWN_ORDER {
                if name.eq_ignore_ascii_case(file)
                    && let Ok(after) = mod_id_for(after)
                {
                    manifest.load_after.push(after);
                }
            }
            manifest.content.records.push(name.clone());
            manifests.push(manifest);
            found.push(name);
        }
        *self.plugins.lock().unwrap_or_else(PoisonError::into_inner) = found;
        manifests
    }
}

/// A content file's type and masters, reading only its header record.
fn read_masters(path: &Path) -> io::Result<(FileType, Vec<String>)> {
    let mut file = fs::File::open(path)?;
    let mut prefix = [0u8; 16];
    file.read_exact(&mut prefix)?;
    let len = mw_records::header_len(&prefix).ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "not a TES3 file"))?;
    let mut bytes = prefix.to_vec();
    bytes.resize(len, 0);
    file.read_exact(&mut bytes[16..])?;
    mw_records::read_header(&bytes)
}

fn stat_bytes(path: &Path) -> (u64, u128) {
    fs::metadata(path).map_or((u64::MAX, 0), |meta| {
        let mtime = meta.modified().ok().and_then(|t| t.duration_since(UNIX_EPOCH).ok());
        (meta.len(), mtime.map_or(0, |d| d.as_nanos()))
    })
}

impl ContentAdapter for MwAdapter {
    fn name(&self) -> &str {
        ADAPTER
    }

    fn discover(&self, issues: &mut Vec<ModIssue>) -> Vec<ModManifest> {
        let mut builtin = ModManifest::synthesized(builtin_id(), "Bevyrrowind builtin", Version::parse(VERSION).expect("crate version"));
        builtin.description = "shipped animation graphs and the mw script runtime; mounted lowest".into();
        let plugins = self.discover_plugins(issues);
        let mut data = ModManifest::synthesized(data_id(), "Data Files (loose)", Version::new(0, 0, 0));
        data.description = "the install's loose Data Files, over its archives".into();
        data.load_after = plugins.iter().map(|m| m.id.clone()).collect();
        for name in self.install.archives() {
            if self.install.data_file(&name).is_none() {
                let message = format!("archive {name} (Morrowind.ini) not found in Data Files");
                issues.push(ModIssue::new(None, IssueKind::Adapter { adapter: ADAPTER.into(), message }));
            }
        }
        // The weather records derive from every plugin's REGN (and the ini), so
        // they come after the content and before user mods, which patch them.
        let mut weather = ModManifest::synthesized(weather_id(), "Morrowind.ini weather", Version::parse(VERSION).expect("crate version"));
        weather.description = "weather presets, region climates and precipitation from REGN and Morrowind.ini (mw_weather)".into();
        weather.load_after = plugins.iter().map(|m| m.id.clone()).chain(std::iter::once(data_id())).collect();
        weather.content.records = vec![WEATHER_RECORDS.into()];
        std::iter::once(builtin).chain(plugins).chain([data, weather]).collect()
    }

    fn mount(&self, manifest: &ModManifest) -> Result<Vec<Arc<dyn VfsSource>>, String> {
        match manifest.id.as_str() {
            BUILTIN => Ok(self.builtin.clone()),
            // The generated record file: a marker the pipeline hands back to `apply_records`.
            WEATHER => Ok(vec![Arc::new(MemSource::new("weather").with(WEATHER_RECORDS, ""))]),
            DATA => {
                let mut layers = self.open_archives(manifest);
                let loose = DirSource::scan("Data Files", &self.install.data_files)
                    .map_err(|e| format!("{}: {e}", self.install.data_files.display()))?;
                if loose.skipped() > 0 {
                    self.warn_mount(format!("Data Files: {} files with unrepresentable names skipped", loose.skipped()));
                }
                layers.push(Arc::new(loose));
                Ok(layers)
            }
            _ => Ok(self.open_archives(manifest)),
        }
    }

    fn record_extensions(&self) -> &[&str] {
        &["esm", "esp", "mwweather"]
    }

    fn register_kinds(&self, db: &mut RecordDb) {
        mw_records::register_kinds(db).expect("TES3 kinds register once");
        mw_weather::register_kinds(db).expect("weather kinds register once");
    }

    fn apply_records(&self, cx: &mut RecordCx<'_>, file: &VfsPath) -> Result<(), RecordFileError> {
        if !self.apply {
            return Ok(());
        }
        if file.extension() == Some("mwweather") {
            // Built from the records so far (every plugin's REGN) and the ini.
            let weather = mw_weather::MwWeather::build(cx.db, self.install.ini.as_ref());
            return weather.define_in(cx.db, cx.source).map_err(|e| RecordFileError::new(e.to_string()));
        }
        let name = file.file_name().to_owned();
        let start = Instant::now();
        let bytes = cx.files.read(file).map_err(|e| RecordFileError::new(format!("read failed: {e}")))?;
        let read_time = start.elapsed();
        let start = Instant::now();
        let mut plugin = Plugin::new();
        plugin.load_bytes(&bytes).map_err(|e| RecordFileError::new(format!("parse failed: {e}")))?;
        let parse_time = start.elapsed();
        let header = plugin.header().ok_or_else(|| RecordFileError::new("no TES3 header"))?;
        let (file_type, masters): (FileType, Vec<String>) =
            (header.file_type, header.masters.iter().map(|(m, _)| m.clone()).collect());
        let master_sources: Vec<Option<SourceId>> =
            masters.iter().map(|m| mod_id_for(m).ok().and_then(|id| cx.source_of(&id))).collect();
        let objects = plugin.objects.len();
        // Slot keys and placement ids name the file by its record source (the mod id).
        let source_name = cx.db.source(cx.source).name.clone();
        let mut warnings = Vec::new();
        let start = Instant::now();
        mw_records::apply_plugin(cx.db, cx.source, &source_name, plugin, &master_sources, &mut warnings)
            .map_err(|e| RecordFileError::new(e.to_string()))?;
        let apply_time = start.elapsed();

        let mut counts: BTreeMap<String, usize> = BTreeMap::new();
        for warning in &warnings {
            let text = format!("{warning:?}");
            *counts.entry(text.split([' ', '{', '(']).next().unwrap_or_default().to_owned()).or_default() += 1;
        }
        for (variant, count) in counts {
            cx.warn(file, format!("{count} x {variant}"));
        }
        let content = ContentFile { name, source: cx.source, file_type, masters, objects, read_time, parse_time, apply_time };
        let mut state = self.content.lock().unwrap_or_else(PoisonError::into_inner);
        state.0.push(content);
        state.1.append(&mut warnings);
        Ok(())
    }

    fn stamp(&self, manifest: &ModManifest) -> Vec<u8> {
        let mut out = format!("{ADAPTER} {VERSION}\n").into_bytes();
        if manifest.id.as_str() == BUILTIN {
            // Generated layers: their bytes (small: graphs and the Luau runtime).
            for layer in &self.builtin {
                let mut paths = Vec::new();
                layer.visit_files(&mut |p| paths.push(p));
                paths.sort_unstable();
                for path in paths {
                    out.extend_from_slice(path.as_str().as_bytes());
                    out.extend(layer.read(&path).unwrap_or_default());
                }
            }
            return out;
        }
        if manifest.id.as_str() == WEATHER {
            // Its records also read Morrowind.ini (the REGN side is in the plugins' stamps).
            let (size, mtime) = stat_bytes(&self.install.root.join("Morrowind.ini"));
            out.extend(format!("Morrowind.ini {size} {mtime}\n").into_bytes());
            return out;
        }
        for name in self.archives_of(manifest) {
            let (size, mtime) = self.install.data_file(&name).map_or((u64::MAX, 0), |p| stat_bytes(&p));
            out.extend(format!("{name} {size} {mtime}\n").into_bytes());
        }
        out
    }

    fn modern_alternates(&self) -> &[(&str, &[&str])] {
        MODERN_ALTERNATES
    }
}

/// The host side of the pipeline: the adapters, the scanned repository and
/// the selected profile (mods.md "Pipeline (startup)" steps 1-4).
pub struct Mods {
    pub adapter: MwAdapter,
    pub native: NativeRecords,
    pub repo: ModRepository,
    pub profile: Profile,
    /// Whether the profile came from its file (else it's the generated default).
    pub profile_from_file: bool,
    pub user: UserDirs,
}

impl Mods {
    /// Scans `user`'s mods folder and the install, and selects `profile`.
    pub fn open(adapter: MwAdapter, user: UserDirs, profile: &str) -> Result<Self, ProfileError> {
        let (profile, profile_from_file) = load_profile_named(&user, profile, adapter.install())?;
        let native = NativeRecords;
        let repo = ModRepository::scan(&[user.mods()], &[&adapter, &native]);
        Ok(Self { adapter, native, repo, profile, profile_from_file, user })
    }

    pub fn adapters(&self) -> [&dyn ContentAdapter; 2] {
        [&self.adapter, &self.native]
    }

    /// Steps 2-4 for the selected profile; `api` is the script API version
    /// (`sf_script::API_VERSION`).
    pub fn load(&self, api: &Version) -> LoadedProfile {
        load_profile(&self.repo, &self.profile, &self.adapters(), api)
    }

    /// The profile fingerprint of a load (bake manifests store it).
    pub fn fingerprint(&self, loaded: &LoadedProfile) -> [u8; 32] {
        loaded.fingerprint(&self.adapters())
    }
}

#[cfg(test)]
mod tests;
