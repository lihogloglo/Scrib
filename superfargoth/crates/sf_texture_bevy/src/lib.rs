//! Bevy glue for `sf_texture`: the KTX2 loader with a load-time level-0 size cap (textures.md §6
//! "Pool per preset", slice S1).
//!
//! [`TextureCapPlugin`] registers [`CappedKtx2Loader`] for `.ktx2` after Bevy's `ImageLoader`, so
//! it is the one that loads them: Bevy 0.20 picks the last loader registered for an extension
//! among the asset type's loaders (`bevy_asset` `server/loaders.rs` `find`). It takes the same
//! settings type ([`ImageLoaderSettings`]), so callers' `with_settings` closures don't change.
//!
//! Per file it reads the header and level index, plans the cap ([`sf_texture::level_cap`]), reads
//! only the bytes the kept levels need (KTX2 stores the top levels last), rewrites the file without
//! the top levels and hands it to Bevy's `ImageLoader`, which builds the [`Image`] exactly as it
//! would have (sampler, sRGB from the settings, usages, Zstandard levels). Under no cap, or for a
//! texture already within it, the file is passed on unchanged.
//!
//! Counters ([`TextureCapStats`]): images loaded, capped, uncappable, GPU bytes loaded and skipped;
//! logged once each loading burst settles (`texture.cap: ...`).
//!
//! Known: with `bevy_image/ktx2` on (it is: `tonemapping_luts` needs it), registering a second
//! `Image` loader for `ktx2` logs Bevy's one startup `warn!` `Duplicate AssetLoader preregistered
//! ... ["ktx2"]`. Bevy 0.20 has no API to replace a loader; selection is still deterministic
//! (last registered wins). Removal path: an upstream API to override an extension's loader.

use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Duration;

use bevy_app::{App, Plugin, Update};
use bevy_asset::io::{Reader, VecReader};
use bevy_asset::{AssetApp, AssetLoader, LoadContext};
use bevy_ecs::prelude::*;
use bevy_image::{CompressedImageFormatSupport, CompressedImageFormats, Image, ImageLoader, ImageLoaderError, ImageLoaderSettings};
use bevy_log::{info, warn};
use bevy_reflect::TypePath;
use bevy_time::{Real, Time};
use futures_lite::AsyncReadExt;
use sf_texture::level_cap::{CapError, CapPlan, HEADER_LEN, Layout, level_index_len, trim};

/// Loads KTX2 levels from the cap down (see the crate docs). `cap`: largest side of level 0 in
/// texels, `None` for the full chain.
#[derive(Clone, Copy, Debug, Default)]
pub struct TextureCapPlugin {
    pub cap: Option<u32>,
}

/// The counters of the capped loader, shared with it (read with [`CapCounters::snapshot`]).
#[derive(Resource, Clone, Debug, Default)]
pub struct TextureCapStats {
    pub cap: Option<u32>,
    pub counters: Arc<CapCounters>,
}

/// Running totals since start.
#[derive(Debug, Default)]
pub struct CapCounters {
    loaded: AtomicU64,
    capped: AtomicU64,
    uncappable: AtomicU64,
    bytes_loaded: AtomicU64,
    bytes_skipped: AtomicU64,
}

/// One reading of [`CapCounters`]. Bytes are GPU bytes (uncompressed level lengths).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct CapCounts {
    /// KTX2 images through the loader.
    pub loaded: u64,
    /// Of those, started below level 0.
    pub capped: u64,
    /// Over the cap but served whole (`sf_texture::level_cap::Uncappable`).
    pub uncappable: u64,
    pub bytes_loaded: u64,
    pub bytes_skipped: u64,
}

impl CapCounters {
    /// Counts one loaded file.
    pub fn record(&self, plan: &CapPlan) {
        self.loaded.fetch_add(1, Ordering::Relaxed);
        if plan.skip > 0 {
            self.capped.fetch_add(1, Ordering::Relaxed);
        }
        if plan.uncappable.is_some() {
            self.uncappable.fetch_add(1, Ordering::Relaxed);
        }
        self.bytes_loaded.fetch_add(plan.bytes_kept, Ordering::Relaxed);
        self.bytes_skipped.fetch_add(plan.bytes_skipped, Ordering::Relaxed);
    }

    pub fn snapshot(&self) -> CapCounts {
        CapCounts {
            loaded: self.loaded.load(Ordering::Relaxed),
            capped: self.capped.load(Ordering::Relaxed),
            uncappable: self.uncappable.load(Ordering::Relaxed),
            bytes_loaded: self.bytes_loaded.load(Ordering::Relaxed),
            bytes_skipped: self.bytes_skipped.load(Ordering::Relaxed),
        }
    }
}

/// The `.ktx2` loader: trims the file under the cap, then Bevy's [`ImageLoader`] builds the image.
#[derive(TypePath)]
pub struct CappedKtx2Loader {
    inner: ImageLoader,
    cap: Option<u32>,
    counters: Arc<CapCounters>,
}

impl CappedKtx2Loader {
    pub fn new(formats: CompressedImageFormats, cap: Option<u32>, counters: Arc<CapCounters>) -> Self {
        Self { inner: ImageLoader::new(formats), cap, counters }
    }
}

/// Why a `.ktx2` didn't load.
#[derive(Debug)]
pub enum CappedKtx2Error {
    Io(std::io::Error),
    Cap(CapError),
    Image(ImageLoaderError),
}

impl std::fmt::Display for CappedKtx2Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(e) => write!(f, "ktx2: {e}"),
            Self::Cap(e) => e.fmt(f),
            Self::Image(e) => write!(f, "ktx2: {e}"),
        }
    }
}

impl std::error::Error for CappedKtx2Error {}

impl From<std::io::Error> for CappedKtx2Error {
    fn from(e: std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<CapError> for CappedKtx2Error {
    fn from(e: CapError) -> Self {
        Self::Cap(e)
    }
}

impl AssetLoader for CappedKtx2Loader {
    type Asset = Image;
    type Settings = ImageLoaderSettings;
    type Error = CappedKtx2Error;

    async fn load(
        &self,
        reader: &mut dyn Reader,
        settings: &ImageLoaderSettings,
        load_context: &mut LoadContext<'_>,
    ) -> Result<Image, CappedKtx2Error> {
        let (bytes, plan) = read_capped(reader, self.cap).await?;
        self.counters.record(&plan);
        let mut trimmed = VecReader::new(bytes);
        self.inner.load(&mut trimmed, settings, load_context).await.map_err(CappedKtx2Error::Image)
    }

    fn extensions(&self) -> &[&str] {
        &["ktx2"]
    }
}

/// Reads a KTX2 file under `cap`: header and level index first, then only the prefix the kept
/// levels need, trimmed ([`sf_texture::level_cap::trim`]); the whole file when nothing is dropped.
///
/// # Errors
/// IO errors and files the cap can't parse ([`CapError`]).
pub async fn read_capped(reader: &mut dyn Reader, cap: Option<u32>) -> Result<(Vec<u8>, CapPlan), CappedKtx2Error> {
    let mut bytes = vec![0; HEADER_LEN];
    reader.read_exact(&mut bytes).await?;
    let head: &[u8; HEADER_LEN] = bytes[..].try_into().expect("HEADER_LEN bytes");
    let index_len = level_index_len(head)?;
    bytes.resize(HEADER_LEN + index_len, 0);
    reader.read_exact(&mut bytes[HEADER_LEN..]).await?;
    let layout = Layout::parse(&bytes)?;
    let plan = layout.plan(cap);
    let Some(read_len) = plan.read_len else {
        Reader::read_to_end(reader, &mut bytes).await?;
        return Ok((bytes, plan));
    };
    let read_len = usize::try_from(read_len).map_err(|_| CapError::Truncated)?;
    let start = bytes.len();
    if read_len > start {
        bytes.resize(read_len, 0);
        reader.read_exact(&mut bytes[start..]).await?;
    }
    let trimmed = trim(&bytes, &layout, &plan)?;
    Ok((trimmed, plan))
}

impl Plugin for TextureCapPlugin {
    fn build(&self, app: &mut App) {
        // Loads of `.ktx2` issued before `finish` wait for this loader, not Bevy's.
        app.preregister_asset_loader::<CappedKtx2Loader>(&["ktx2"]);
        app.insert_resource(TextureCapStats { cap: self.cap, counters: Arc::default() });
        app.add_systems(Update, log_cap_counts);
    }

    fn finish(&self, app: &mut App) {
        let formats = app.world().get_resource::<CompressedImageFormatSupport>().map_or_else(
            || {
                warn!("texture.cap: no CompressedImageFormatSupport (no render plugin?); BCn textures won't load");
                CompressedImageFormats::NONE
            },
            |support| support.0,
        );
        let counters = app.world().resource::<TextureCapStats>().counters.clone();
        app.register_asset_loader(CappedKtx2Loader::new(formats, self.cap, counters));
    }
}

/// How long the counters must stay still before the line is logged (one line per loading burst).
pub(crate) const SETTLE: Duration = Duration::from_secs(2);

/// `texture.cap: ...` once each loading burst has settled for [`SETTLE`].
fn log_cap_counts(
    stats: Res<TextureCapStats>,
    time: Res<Time<Real>>,
    mut seen: Local<(CapCounts, Duration)>,
    mut logged: Local<CapCounts>,
) {
    let now = stats.counters.snapshot();
    if now != seen.0 {
        *seen = (now, time.elapsed());
        return;
    }
    if now != *logged && time.elapsed().saturating_sub(seen.1) >= SETTLE {
        *logged = now;
        info!("{}", cap_line(stats.cap, now));
    }
}

/// The log line: `texture.cap: 2048 px, 812 ktx2 loaded, 140 capped, 0 uncappable, 1234.5 MiB loaded, 567.8 MiB skipped`.
pub fn cap_line(cap: Option<u32>, c: CapCounts) -> String {
    #[expect(clippy::cast_precision_loss, reason = "MiB for a log line")]
    let mib = |b: u64| b as f64 / (1024.0 * 1024.0);
    let cap = cap.map_or_else(|| "no cap".to_owned(), |px| format!("{px} px"));
    format!(
        "texture.cap: {cap}, {} ktx2 loaded, {} capped, {} uncappable, {:.1} MiB loaded, {:.1} MiB skipped",
        c.loaded,
        c.capped,
        c.uncappable,
        mib(c.bytes_loaded),
        mib(c.bytes_skipped)
    )
}

#[cfg(test)]
mod tests;

pub mod rebind;
pub use rebind::{ImageRebindPlugin, ImageRebindStats, ImageUsers};

#[cfg(test)]
mod rebind_tests;
