//! The user's animation libraries on disk (`docs/specs/animation-sources.md`):
//! the manifest ([`mw_graphs::sources`]) plus the local folder holding the
//! files. Bring your own, like the Morrowind install: nothing is
//! downloaded, and every slot falls back when its library is missing.
//!
//! The bake names library files by virtual path, `animsrc/<source>/<file>`
//! ([`AnimSources::virtual_path`]), so bake keys don't depend on where the
//! folder is; [`crate::VfsInputs::with_anim_sources`] serves those paths
//! from the folder (`<dir>/<file>`, else `<dir>/<source>/<file>`).
//!
//! The folder: `BEVYRROWIND_ANIM_SOURCES` ([`LIBRARY_ENV`]), else
//! `<repo>/animation packs` (gitignored), as the install path is an env
//! override over a default. After it, the bundled folder
//! ([`bundled_dir`]: the CC0 Quaternius packs, tracked with Git LFS), so
//! those resolve with no library at all and a library copy wins.

use std::fmt;
use std::path::{Path, PathBuf};

use mw_graphs::sources::{SourceRef, SourcesManifest};

/// Environment variable naming the library folder.
pub const LIBRARY_ENV: &str = "BEVYRROWIND_ANIM_SOURCES";

/// Prefix of the bake's virtual library paths.
pub const VIRTUAL_ROOT: &str = "animsrc/";

/// The default library folder: `<repo>/animation packs`.
pub fn default_dir() -> PathBuf {
    let crate_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    crate_dir.ancestors().nth(3).unwrap_or(crate_dir).join("animation packs")
}

/// The repo's bundled library files: `<repo>/bevyrrowind/assets/animation/quaternius`
/// (`PROVENANCE.md` there).
pub fn bundled_dir() -> PathBuf {
    let crate_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    crate_dir.ancestors().nth(2).unwrap_or(crate_dir).join("assets").join("animation").join("quaternius")
}

/// Where the folder setting came from.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DirOrigin {
    Env,
    Default,
    Explicit,
}

/// The manifest and the folder holding the user's library files.
#[derive(Clone, Debug)]
pub struct AnimSources {
    pub manifest: SourcesManifest,
    pub dir: PathBuf,
    pub origin: DirOrigin,
    /// Searched after [`Self::dir`] ([`bundled_dir`] for [`Self::local`]).
    pub bundled: Option<PathBuf>,
}

impl AnimSources {
    /// The shipped manifest over the local folder ([`LIBRARY_ENV`], else
    /// [`default_dir`]), then the bundled one.
    pub fn local() -> Self {
        let (dir, origin) = match std::env::var_os(LIBRARY_ENV) {
            Some(dir) => (PathBuf::from(dir), DirOrigin::Env),
            None => (default_dir(), DirOrigin::Default),
        };
        Self { manifest: SourcesManifest::builtin().clone(), dir, origin, bundled: Some(bundled_dir()) }
    }

    /// A manifest over an explicit folder (tests, tools), no bundled folder.
    pub fn new(manifest: SourcesManifest, dir: impl Into<PathBuf>) -> Self {
        Self { manifest, dir: dir.into(), origin: DirOrigin::Explicit, bundled: None }
    }

    /// No libraries: every slot takes its fallback.
    pub fn none() -> Self {
        Self::new(SourcesManifest::default(), PathBuf::new())
    }

    /// The bake's path for a source file.
    pub fn virtual_path(source: &str, file: &str) -> String {
        format!("{VIRTUAL_ROOT}{source}/{file}")
    }

    /// The file on disk behind a virtual path, when the manifest lists it
    /// and it exists.
    pub fn real_path(&self, path: &str) -> Option<PathBuf> {
        let (source, file) = path.strip_prefix(VIRTUAL_ROOT)?.split_once('/')?;
        self.manifest.sources.get(source)?.files.iter().find(|f| f.name == file)?;
        [Some(self.dir.join(file)), Some(self.dir.join(source).join(file)), self.bundled.as_ref().map(|b| b.join(file))]
            .into_iter()
            .flatten()
            .find(|p| p.is_file())
    }

    /// Every candidate reference with its virtual file path.
    pub fn references(&self) -> Vec<(String, SourceRef)> {
        self.manifest.references().unwrap_or_default().into_iter().map(|r| (Self::virtual_path(&r.source, &r.file), r)).collect()
    }

    /// Which files are present and which slot resolves to what, from the
    /// folder alone (the bake also checks that each file holds the clip:
    /// [`crate::anim::AnimBakeReport::slots`]).
    pub fn check(&self) -> SourcesCheck {
        let mut files = Vec::new();
        for (id, source) in &self.manifest.sources {
            for f in &source.files {
                let real = self.real_path(&Self::virtual_path(id, &f.name));
                let size = real.as_ref().and_then(|p| std::fs::metadata(p).ok()).map(|m| m.len());
                files.push(FileCheck { source: id.clone(), name: f.name.clone(), license: source.license.tag(), path: real, size });
            }
        }
        let mut slots = Vec::new();
        for (name, slot) in &self.manifest.slots {
            let present = |c: &mw_graphs::sources::Candidate| {
                let file = c.file.clone().or_else(|| self.manifest.sources.get(&c.source)?.files.first().map(|f| f.name.clone()));
                file.is_some_and(|f| self.real_path(&Self::virtual_path(&c.source, &f)).is_some())
            };
            let from = slot.candidates.iter().find(|c| present(c)).map(mw_graphs::sources::Candidate::reference);
            slots.push(SlotCheck { slot: name.clone(), graph: slot.graph.clone(), from, fallback: format!("{:?}", slot.fallback) });
        }
        SourcesCheck { dir: self.dir.clone(), origin: self.origin, bundled: self.bundled.clone(), files, slots }
    }
}

/// One expected library file.
#[derive(Clone, Debug)]
pub struct FileCheck {
    pub source: String,
    pub name: String,
    pub license: &'static str,
    pub path: Option<PathBuf>,
    pub size: Option<u64>,
}

/// One slot: the first candidate whose file is present, else the fallback.
#[derive(Clone, Debug)]
pub struct SlotCheck {
    pub slot: String,
    pub graph: String,
    pub from: Option<String>,
    pub fallback: String,
}

/// [`AnimSources::check`]: `bevyrrowind anim-sources`.
#[derive(Clone, Debug)]
pub struct SourcesCheck {
    pub dir: PathBuf,
    pub origin: DirOrigin,
    pub bundled: Option<PathBuf>,
    pub files: Vec<FileCheck>,
    pub slots: Vec<SlotCheck>,
}

impl fmt::Display for SourcesCheck {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let origin = match self.origin {
            DirOrigin::Env => format!("from {LIBRARY_ENV}"),
            DirOrigin::Default => format!("default; set {LIBRARY_ENV} to change"),
            DirOrigin::Explicit => "explicit".to_owned(),
        };
        writeln!(f, "library folder: {} ({origin})", self.dir.display())?;
        if let Some(b) = &self.bundled {
            writeln!(f, "bundled folder: {}", b.display())?;
        }
        for file in &self.files {
            match (&file.path, file.size) {
                (Some(p), size) => writeln!(f, "  present  {} {} ({}, {} bytes) {}", file.source, file.name, file.license, size.unwrap_or(0), p.display())?,
                (None, _) => writeln!(f, "  missing  {} {} ({})", file.source, file.name, file.license)?,
            }
        }
        for slot in &self.slots {
            match &slot.from {
                Some(r) => writeln!(f, "  slot {} ({}): {r}", slot.slot, slot.graph)?,
                None => writeln!(f, "  slot {} ({}): fallback {}", slot.slot, slot.graph, slot.fallback)?,
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn virtual_paths_map_into_the_folder() {
        let dir = tempfile::tempdir().unwrap();
        let sources = AnimSources::new(SourcesManifest::builtin().clone(), dir.path());
        let v = AnimSources::virtual_path("quaternius_ual1", "UAL1_RM.glb");
        assert_eq!(v, "animsrc/quaternius_ual1/UAL1_RM.glb");
        assert!(sources.real_path(&v).is_none());
        assert!(sources.check().slots.iter().all(|s| s.from.is_none()));

        std::fs::create_dir(dir.path().join("quaternius_ual1")).unwrap();
        std::fs::write(dir.path().join("quaternius_ual1/UAL1_RM.glb"), b"x").unwrap();
        assert!(sources.real_path(&v).is_some(), "a per-source subfolder works too");
        assert!(sources.real_path("animsrc/quaternius_ual1/other.glb").is_none(), "only manifest files");
        assert!(sources.real_path("animsrc/nope/UAL1_RM.glb").is_none());
        let check = sources.check();
        let climb = check.slots.iter().find(|s| s.slot == "traverse.climb.up").unwrap();
        assert_eq!(climb.from.as_deref(), Some("quaternius_ual1:ClimbLedge"));
        assert!(check.slots.iter().find(|s| s.slot == "traverse.mantle.waist").unwrap().from.is_none(), "UAL1 serves no mantle");
        assert!(default_dir().ends_with("animation packs") && default_dir().parent().unwrap().join("bevyrrowind").is_dir());
        assert!(check.to_string().contains("present  quaternius_ual1"));

        // The bundled folder comes after the library: a library copy wins.
        let bundled = tempfile::tempdir().unwrap();
        std::fs::write(bundled.path().join("UAL1_RM.glb"), b"y").unwrap();
        let both = AnimSources { bundled: Some(bundled.path().to_owned()), ..sources.clone() };
        assert_eq!(both.real_path(&v), sources.real_path(&v));
        let only = AnimSources { bundled: Some(bundled.path().to_owned()), ..AnimSources::new(sources.manifest.clone(), "") };
        assert_eq!(only.real_path(&v), Some(bundled.path().join("UAL1_RM.glb")));
        assert!(only.real_path(&AnimSources::virtual_path("quaternius_ual2", "UAL2_RM.glb")).is_none());
        assert!(bundled_dir().ends_with("bevyrrowind/assets/animation/quaternius"));
        assert_eq!(AnimSources::local().bundled, Some(bundled_dir()));
    }
}
