//! Albedo de-lighting (`superfargoth/docs/specs/bake.md` "De-lighting"; owner decision 2026-11-05:
//! on by default, strength 0.5, `--off delight` for the A/B).
//!
//! Products:
//! - `texture.roles` (per model): every texture slot of the NIF's materials with its role
//!   ([`Role`]), the base slot's clamp flags and alpha-test cutoff.
//! - `texture.delight` (per winning texture file): the de-lit texture as BC7 sRGB KTX2 with mips
//!   (`sf_texture`), or a constant skip marker for a texture found unfit once decoded (too small,
//!   not block-aligned, undecodable). Strength, clamp and cutoff are parameters, the algorithm and
//!   encoder version is the step version, the source is a planned input: a re-bake redoes only
//!   textures whose source or settings changed.
//! - `texture.delight.index` (one): `(source, source fingerprint, ktx2 object)` per de-lit texture,
//!   read from digests only (no product bytes), plus the step-time skip counts.
//! - `texture.alpha_class` (per texture file a cutout-candidate material uses) and
//!   `texture.alpha_class.index` (one, reference → [`AlphaClass`]): is this texture's alpha a
//!   cutout or translucency ([`alpha_is_cutout`])? Not a de-lighting fact — the static-scene bake
//!   reads it to classify candidate materials the way the loader does (`mw_bake::scene`) — but
//!   baked here, where every referenced model's texture uses are already resolved.
//!
//! Runtime ([`crate::cook::CookedTextureLayer`], shared with the texture cook): a VFS source
//! mounted above every other layer that serves
//! `<stem>.ktx2` for an index entry only while its source is still the winning file with the
//! fingerprint the bake saw and the only variant of its stem ([`sole_variant`]). A mod's newer
//! `.dds`/`.ktx2`/`.tga` changes the winner or the fingerprint, so the stale entry is not served
//! and the mod's file wins; `bevyrrowind bake` then de-lights the new winner (or skips it).
//!
//! Selection policy ([`plan_jobs`], counts per [`Skip`]): base-slot textures of referenced models;
//! skipped when any use is not plain albedo (glow, detail/dark/decal slots, base colour glowing
//! through emissive or unlit materials, particles and effect models, translucent blends), when a
//! `_n`/`_nh`/`_spec` companion comes from the albedo's own VFS source (a PBR replacer shipping both:
//! leave it as authored; a normals-only pack over another source's albedo still de-lights it,
//! textures-adapter.md §3), for sky and UI
//! names, a winner already in KTX2, more than one variant of the stem, and at decode time for
//! sides under [`MIN_SIDE`], non-4-aligned sizes and undecodable files. Terrain layer textures are
//! not model textures and are not de-lit yet (their arrays bake from the originals).

use std::collections::{BTreeMap, BTreeSet, HashSet};
use std::fmt;
use std::sync::Arc;
use std::time::{Duration, Instant};

use mw_nif::{AlphaMode, Cutout, VertexColorMode};
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use sf_bake::{BakeError, Baker, Digest, Manifest, PlanCx, ProductKey, RunCx, Step, Store};
use sf_texture::ktx2_out::is_block_aligned;
use sf_texture::{Bc7Quality, bc7_ktx2_cutout, delight, shading_grid};
use sf_vfs::{Vfs, VfsPath};

use crate::textures::{TEXTURE_ALTERNATES, alpha_is_cutout, decode_rgba};

/// `texture.roles`: a model's texture uses.
pub const ROLES_STEP: &str = "texture.roles";
/// `texture.delight`: one de-lit texture.
pub const DELIGHT_STEP: &str = "texture.delight";
/// `texture.delight.index`: what the runtime layer serves.
pub const INDEX_STEP: &str = "texture.delight.index";
/// The index's only subject.
pub const INDEX_SUBJECT: &str = "all";
/// The owner's strength (DECISIONS 2026-11-05): half the estimated shading divided out.
pub const DEFAULT_STRENGTH: f32 = delight::DEFAULT_STRENGTH;
/// Smallest side de-lit: small detailed textures (icons, trims) don't come back neutral (D-1).
pub const MIN_SIDE: u32 = 64;
/// Bump when `sf_texture`'s de-lighting, mips or encode change (2: coverage-preserving cutout mips, D-2c).
pub const ALGORITHM_VERSION: u32 = 2;
/// Every extension a texture reference may resolve to, and its source formats.
pub const VARIANTS: [&str; 5] = ["dds", "tga", "bmp", "png", "ktx2"];
/// Companion maps whose presence marks a PBR replacer.
pub const COMPANIONS: [&str; 3] = ["_n", "_nh", "_spec"];

/// What a texture slot does in a material.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum Role {
    /// Base colour lit by the scene: the only role de-lit.
    Albedo,
    /// Glow map slot.
    Glow,
    /// Detail, dark (light map) or decal slots.
    Detail,
    /// Base colour that glows through an emissive colour, emissive vertex colours or an unlit material.
    Emissive,
    /// Particles, additive blends and spell effect models (`meshes/e/`).
    Effect,
    /// Alpha-blended surfaces that aren't foliage cutouts (glass, water, webs).
    Translucent,
}

impl Role {
    fn name(self) -> &'static str {
        match self {
            Role::Albedo => "albedo",
            Role::Glow => "glow",
            Role::Detail => "detail",
            Role::Emissive => "emissive",
            Role::Effect => "effect",
            Role::Translucent => "translucent",
        }
    }
}

/// One texture slot of one material.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TextureUse {
    /// The reference as the NIF names it (normalised VFS path, e.g. `textures/tx_wood.tga`).
    pub texture: String,
    pub role: Role,
    /// `(clamp_s, clamp_t)` of the base slot.
    pub clamp: [bool; 2],
    /// Alpha-test cutoff (0..=1) when the surface may draw alpha-tested.
    pub cutoff: Option<f32>,
    /// The cutoff applies only if the texture's alpha has holes (`mw_nif::Cutout::Candidate`).
    pub candidate: bool,
}

/// The texture uses of a converted model. `model` is its VFS path (effect models live in `meshes/e/`).
pub fn roles_of(model: &str, nif: &mw_nif::Model) -> Vec<TextureUse> {
    let effect_model = model.starts_with("meshes/e/");
    let particle_materials: HashSet<usize> = nif.particles.iter().map(|p| p.material).collect();
    let mut uses = Vec::new();
    let mut push = |texture: &str, role: Role, clamp: [bool; 2], cutoff: Option<f32>, candidate: bool| {
        uses.push(TextureUse { texture: texture.to_owned(), role, clamp, cutoff, candidate });
    };
    for (i, m) in nif.materials.iter().enumerate() {
        if let Some(glow) = &m.glow_texture {
            push(glow, Role::Glow, [false; 2], None, false);
        }
        for detail in [&m.detail_texture, &m.dark_texture].into_iter().flatten() {
            push(detail, Role::Detail, [false; 2], None, false);
        }
        let Some(base) = &m.base_texture else { continue };
        let (cutoff, candidate) = match m.alpha_mode {
            AlphaMode::Mask { cutoff } => (Some(cutoff), false),
            AlphaMode::Blend { additive: false } if m.cutout == Cutout::Candidate => (Some(mw_nif::CUTOUT_CUTOFF), true),
            _ => (None, false),
        };
        let role = if effect_model || particle_materials.contains(&i) || m.alpha_mode == (AlphaMode::Blend { additive: true }) {
            Role::Effect
        } else if matches!(m.alpha_mode, AlphaMode::Blend { .. }) && !candidate {
            Role::Translucent
        } else if m.unlit_lighting
            || m.vertex_colors == VertexColorMode::Emissive
            || (m.glow_texture.is_none() && m.emissive.iter().sum::<f32>() > 0.0)
        {
            Role::Emissive
        } else {
            Role::Albedo
        };
        push(base, role, m.base_clamp, cutoff, candidate);
    }
    uses
}

/// Why a texture is not de-lit.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum Skip {
    /// The reference resolves to no file.
    Missing,
    /// More than one variant of the stem exists (`foo.tga` and `foo.dds`): references could
    /// resolve differently, so one baked `foo.ktx2` can't stand for all of them.
    Ambiguous,
    /// The winner is already a KTX2 (authored by a mod).
    AuthoredKtx2,
    /// Sky or UI texture by name.
    SkyUi,
    /// Used in a role other than albedo by some material.
    Role(Role),
    /// A `_n`/`_nh`/`_spec` companion from the albedo's own VFS source exists.
    Companion,
    /// A side under [`MIN_SIDE`].
    Small,
    /// Not a multiple of 4 (BC7 blocks, wgpu).
    Unaligned,
    /// The file doesn't decode.
    Undecodable,
}

impl fmt::Display for Skip {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Skip::Missing => "missing",
            Skip::Ambiguous => "ambiguous",
            Skip::AuthoredKtx2 => "authored ktx2",
            Skip::SkyUi => "sky/ui",
            Skip::Role(role) => role.name(),
            Skip::Companion => "pbr companion",
            Skip::Small => "small",
            Skip::Unaligned => "unaligned",
            Skip::Undecodable => "undecodable",
        })
    }
}

/// The `texture.delight` product of a texture skipped at decode time: a constant per reason, so
/// the index recognises it by digest without reading products.
fn skip_marker(skip: Skip) -> Vec<u8> {
    format!("sf.delight.skip:{skip}").into_bytes()
}

const STEP_SKIPS: [Skip; 3] = [Skip::Small, Skip::Unaligned, Skip::Undecodable];

/// What `texture.delight` needs to bake one winning file.
#[derive(Clone, Debug, PartialEq)]
pub struct TextureJob {
    /// Repeats along u / v (any use that doesn't clamp the axis).
    pub wrap: [bool; 2],
    /// Alpha-test cutoff the mips keep coverage for: the most common among the uses (ties: the
    /// lowest). A texture used at several cutoffs keeps coverage for that one only.
    pub cutoff: Option<f32>,
    /// Every alpha-tested use is a cutout candidate: the cutoff applies only if alpha has holes.
    pub candidate: bool,
}

/// Sky, moon, star and UI textures by name.
pub fn is_sky_or_ui(path: &str) -> bool {
    const UI_DIRS: [&str; 6] = ["textures/menu", "textures/icons/", "textures/bookart/", "textures/levelup/", "textures/splash/", "textures/scroll"];
    const SKY_NAMES: [&str; 7] = ["tx_moon", "tx_masser", "tx_secunda", "tx_sun", "tx_stars", "tx_night", "tx_menu"];
    let name = path.rsplit('/').next().unwrap_or(path);
    name.contains("sky") || SKY_NAMES.iter().any(|p| name.starts_with(p)) || UI_DIRS.iter().any(|d| path.starts_with(d))
}

/// `path` without its extension.
pub(crate) fn stem(path: &str) -> &str {
    match path.rsplit_once('.') {
        Some((stem, ext)) if !ext.contains('/') => stem,
        _ => path,
    }
}

fn extension(path: &str) -> &str {
    path.rsplit_once('.').map_or("", |(_, e)| e)
}

/// Whether `source` is the only variant of its stem that `exists` ([`VARIANTS`]): then every
/// reference to the stem resolves to it, and a `<stem>.ktx2` above it stands for all of them.
pub fn sole_variant(source: &str, exists: impl Fn(&str) -> bool) -> bool {
    let (stem, ext) = (stem(source), extension(source));
    exists(source) && VARIANTS.iter().filter(|v| **v != ext).all(|v| !exists(&format!("{stem}.{v}")))
}

/// Whether a `_n`/`_nh`/`_spec` companion of `source` exists in any variant format in the same VFS
/// source as `source` (`source_of`: the winning source of an exact path, `None` when absent): a
/// pack that ships both was authored together (textures-adapter.md §3).
pub fn has_companion<S: PartialEq>(source: &str, source_of: impl Fn(&str) -> Option<S>) -> bool {
    let stem = stem(source);
    let Some(own) = source_of(source) else { return false };
    COMPANIONS
        .iter()
        .any(|c| VARIANTS.iter().any(|v| source_of(&format!("{stem}{c}.{v}")).is_some_and(|s| s == own)))
}

/// The textures to de-lit, by winning file, and the skip counts (per texture, by first reason in
/// [`Skip`]'s order). `resolve` maps a reference to its winning file (the runtime's rule,
/// [`TEXTURE_ALTERNATES`]); `source_of` gives the winning VFS source of an exact path (`None`: no
/// such file).
pub fn plan_jobs<S: PartialEq>(
    uses: impl IntoIterator<Item = TextureUse>,
    resolve: impl Fn(&str) -> Option<String>,
    source_of: impl Fn(&str) -> Option<S>,
) -> (BTreeMap<String, TextureJob>, BTreeMap<Skip, u32>) {
    let exists = |p: &str| source_of(p).is_some();
    let mut by_ref: BTreeMap<String, Vec<TextureUse>> = BTreeMap::new();
    for u in uses {
        by_ref.entry(u.texture.clone()).or_default().push(u);
    }
    let mut skipped: BTreeMap<Skip, u32> = BTreeMap::new();
    let mut by_winner: BTreeMap<String, Vec<TextureUse>> = BTreeMap::new();
    for (reference, uses) in by_ref {
        let base_slot = uses.iter().any(|u| matches!(u.role, Role::Albedo | Role::Emissive | Role::Effect | Role::Translucent));
        match resolve(&reference) {
            Some(winner) => by_winner.entry(winner).or_default().extend(uses),
            None if base_slot => *skipped.entry(Skip::Missing).or_default() += 1,
            None => {}
        }
    }
    let mut jobs = BTreeMap::new();
    for (winner, uses) in by_winner {
        if !uses.iter().any(|u| matches!(u.role, Role::Albedo | Role::Emissive | Role::Effect | Role::Translucent)) {
            continue; // Only glow/detail slots: not a base-colour texture.
        }
        let other_role = uses.iter().map(|u| u.role).filter(|r| *r != Role::Albedo).min();
        let skip = if !sole_variant(&winner, exists) {
            Some(Skip::Ambiguous)
        } else if extension(&winner) == "ktx2" {
            Some(Skip::AuthoredKtx2)
        } else if is_sky_or_ui(&winner) {
            Some(Skip::SkyUi)
        } else if let Some(role) = other_role {
            Some(Skip::Role(role))
        } else if has_companion(&winner, &source_of) {
            Some(Skip::Companion)
        } else {
            None
        };
        if let Some(skip) = skip {
            *skipped.entry(skip).or_default() += 1;
            continue;
        }
        let wrap = [uses.iter().any(|u| !u.clamp[0]), uses.iter().any(|u| !u.clamp[1])];
        let mut cutoffs: BTreeMap<u32, u32> = BTreeMap::new();
        for c in uses.iter().filter_map(|u| u.cutoff) {
            *cutoffs.entry(c.to_bits()).or_default() += 1;
        }
        // Most common; on a tie the lowest (positive floats order like their bits).
        let cutoff = cutoffs.iter().max_by_key(|(bits, n)| (**n, std::cmp::Reverse(**bits))).map(|(bits, _)| f32::from_bits(*bits));
        let candidate = uses.iter().filter(|u| u.cutoff.is_some()).all(|u| u.candidate);
        jobs.insert(winner, TextureJob { wrap, cutoff, candidate: candidate && cutoff.is_some() });
    }
    (jobs, skipped)
}

/// `texture.roles`: [`roles_of`] a model, postcard.
pub struct RolesStep;

impl Step for RolesStep {
    fn id(&self) -> &'static str {
        ROLES_STEP
    }

    fn version(&self) -> u32 {
        1
    }

    fn plan(&self, subject: &str, cx: &mut PlanCx) -> Result<(), BakeError> {
        cx.file(subject);
        Ok(())
    }

    fn run(&self, subject: &str, cx: &mut RunCx<'_>) -> Result<Vec<u8>, BakeError> {
        let key = || ProductKey::new(ROLES_STEP, subject);
        let bytes = cx.read_file(subject)?;
        let nif = mw_nif::load(&bytes).map_err(|e| BakeError::step(&key(), e))?;
        postcard::to_stdvec(&roles_of(subject, &nif)).map_err(|e| BakeError::step(&key(), e))
    }
}

/// `texture.delight`: decode, de-light, mips, BC7 sRGB KTX2.
pub struct DelightStep {
    pub strength: f32,
    pub jobs: Arc<BTreeMap<String, TextureJob>>,
}

impl DelightStep {
    fn job(&self, subject: &str) -> Result<&TextureJob, BakeError> {
        self.jobs.get(subject).ok_or_else(|| BakeError::step(&ProductKey::new(DELIGHT_STEP, subject), "not a planned texture"))
    }
}

impl Step for DelightStep {
    fn id(&self) -> &'static str {
        DELIGHT_STEP
    }

    fn version(&self) -> u32 {
        ALGORITHM_VERSION
    }

    fn plan(&self, subject: &str, cx: &mut PlanCx) -> Result<(), BakeError> {
        let job = self.job(subject)?;
        cx.file(subject)
            .param("strength", self.strength.to_bits().to_le_bytes())
            .param("wrap", [u8::from(job.wrap[0]), u8::from(job.wrap[1])])
            .param("cutoff", job.cutoff.map_or_else(Vec::new, |c| c.to_bits().to_le_bytes().to_vec()))
            .param("candidate", [u8::from(job.candidate)]);
        Ok(())
    }

    fn run(&self, subject: &str, cx: &mut RunCx<'_>) -> Result<Vec<u8>, BakeError> {
        let job = self.job(subject)?.clone();
        let bytes = cx.read_file(subject)?;
        Ok(delight_texture(subject, &bytes, &job, self.strength).unwrap_or_else(skip_marker))
    }

    fn compress(&self) -> bool {
        false
    }
}

/// One texture's `texture.delight` output: the KTX2 file, or why it is skipped.
pub fn delight_texture(path: &str, bytes: &[u8], job: &TextureJob, strength: f32) -> Result<Vec<u8>, Skip> {
    let mut image = decode_rgba(path, bytes).map_err(|_| Skip::Undecodable)?;
    let (w, h) = (image.width, image.height);
    if w < MIN_SIDE || h < MIN_SIDE {
        return Err(Skip::Small);
    }
    if !is_block_aligned(w, h) {
        return Err(Skip::Unaligned);
    }
    let cutoff = job.cutoff.filter(|_| !job.candidate || alpha_is_cutout(&image));
    let grid = shading_grid(&image.rgba, w, h, job.wrap[0], job.wrap[1]);
    delight(&mut image.rgba, w, h, &grid, strength);
    Ok(bc7_ktx2_cutout(&image.rgba, w, h, true, Bc7Quality::Basic, cutoff))
}

/// One de-lit texture in the index.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct IndexEntry {
    /// The winning source file it was made from.
    pub source: String,
    /// The source's bake fingerprint (`Inputs::file_fingerprint`) when it was made.
    pub fingerprint: Digest,
    /// The KTX2 object in the store.
    pub ktx2: Digest,
}

/// `texture.delight.index`.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct DelightIndex {
    pub entries: Vec<IndexEntry>,
    /// Textures skipped at decode time, by reason.
    pub skipped: BTreeMap<Skip, u32>,
}

/// `texture.delight.index`: plans every job's source and product, reads only their digests.
pub struct IndexStep {
    pub jobs: Arc<BTreeMap<String, TextureJob>>,
}

impl Step for IndexStep {
    fn id(&self) -> &'static str {
        INDEX_STEP
    }

    fn version(&self) -> u32 {
        // 2: a companion skips de-lighting only from the albedo's own source (T4, adapter §3).
        2
    }

    fn plan(&self, _subject: &str, cx: &mut PlanCx) -> Result<(), BakeError> {
        for source in self.jobs.keys() {
            cx.file(source.as_str()).product(DELIGHT_STEP, source.as_str());
        }
        Ok(())
    }

    fn run(&self, subject: &str, cx: &mut RunCx<'_>) -> Result<Vec<u8>, BakeError> {
        let markers: Vec<(Digest, Skip)> = STEP_SKIPS.iter().map(|s| (Digest::of(&skip_marker(*s)), *s)).collect();
        let mut index = DelightIndex::default();
        for source in self.jobs.keys() {
            let fingerprint = cx.file_fingerprint(source)?;
            let ktx2 = cx.product_digest(DELIGHT_STEP, source)?;
            match markers.iter().find(|(d, _)| *d == ktx2) {
                Some((_, skip)) => *index.skipped.entry(*skip).or_default() += 1,
                None => index.entries.push(IndexEntry { source: source.clone(), fingerprint, ktx2 }),
            }
        }
        postcard::to_stdvec(&index).map_err(|e| BakeError::step(&ProductKey::new(INDEX_STEP, subject), e))
    }
}

/// What a texture's alpha channel is, as [`alpha_is_cutout`] reads it: the fact that decides
/// whether a cutout candidate (`mw_nif::Cutout::Candidate`: an authored blend with no alpha test)
/// draws alpha-tested or blended.
///
/// Classified once per texture file in the bake ([`AlphaClassStep`]) instead of per load, because
/// the static-scene bake needs the same answer the cooked-model loader reaches
/// (`mw_bake::scene::runtime_alpha`, `mw_assets::texture_is_cutout`) and cannot decode textures.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum AlphaClass {
    /// Holes plus solid texels: a cutout (leaf cards, fences, grates). Draws alpha-tested.
    Cutout,
    /// Broad partial alpha, or no alpha at all: translucency (glass, webs, potions), or opaque.
    /// Draws as authored.
    Soft,
    /// The file is there but did not decode ([`decode_rgba`]): no answer, so not a cutout.
    Undecodable,
}

impl AlphaClass {
    /// The classifier's answer for `bytes` (the texture file's contents).
    pub fn of(path: &str, bytes: &[u8]) -> Self {
        match decode_rgba(path, bytes) {
            Ok(image) if alpha_is_cutout(&image) => Self::Cutout,
            Ok(_) => Self::Soft,
            Err(_) => Self::Undecodable,
        }
    }
}

/// `texture.alpha_class`: one texture file's [`AlphaClass`].
pub const ALPHA_CLASS_STEP: &str = "texture.alpha_class";
/// `texture.alpha_class.index`: every cutout candidate's reference and its class.
pub const ALPHA_INDEX_STEP: &str = "texture.alpha_class.index";
/// The index's only subject.
pub const ALPHA_INDEX_SUBJECT: &str = "all";
/// Bump when [`alpha_is_cutout`] can answer differently: it is this step's version, so every
/// class product (and the index, and through the scene source's fingerprint every `scene.cells`
/// table) re-bakes.
pub const ALPHA_CLASSIFIER_VERSION: u32 = 1;

/// `texture.alpha_class`: decode one texture file's top mip and classify its alpha.
///
/// Subject: the **winning file** (as `texture.delight`), so two references that resolve to the
/// same file are classified once. Keyed on that file and the classifier version, nothing else:
/// the class is a property of the file alone, and the step is independent of de-lighting (it is
/// baked here only because this is where texture uses are resolved, and it covers candidates
/// de-lighting skips, [`plan_jobs`]).
pub struct AlphaClassStep;

impl Step for AlphaClassStep {
    fn id(&self) -> &'static str {
        ALPHA_CLASS_STEP
    }

    fn version(&self) -> u32 {
        ALPHA_CLASSIFIER_VERSION
    }

    fn plan(&self, subject: &str, cx: &mut PlanCx) -> Result<(), BakeError> {
        cx.file(subject);
        Ok(())
    }

    fn run(&self, subject: &str, cx: &mut RunCx<'_>) -> Result<Vec<u8>, BakeError> {
        let bytes = cx.read_file(subject)?;
        postcard::to_stdvec(&AlphaClass::of(subject, &bytes))
            .map_err(|e| BakeError::step(&ProductKey::new(ALPHA_CLASS_STEP, subject), e))
    }

    fn compress(&self) -> bool {
        false
    }
}

/// `texture.alpha_class.index`: every cutout candidate's **reference** (as a material names it)
/// and the class of the file it resolves to. One product, so a later bake stage reads one thing.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct AlphaClassIndex {
    pub classes: BTreeMap<String, AlphaClass>,
}

/// `texture.alpha_class.index`: plans every candidate's class product, then collects them.
pub struct AlphaIndexStep {
    /// Reference → winning file ([`alpha_class_jobs`]).
    pub textures: Arc<BTreeMap<String, String>>,
}

impl Step for AlphaIndexStep {
    fn id(&self) -> &'static str {
        ALPHA_INDEX_STEP
    }

    fn version(&self) -> u32 {
        1
    }

    fn plan(&self, _subject: &str, cx: &mut PlanCx) -> Result<(), BakeError> {
        // The classes themselves are products (so a changed texture re-bakes this), and the
        // reference → winner map is a parameter: the references appear only in the output, so a
        // reference that moves to another file would otherwise not change this key.
        let mut map = Vec::new();
        for (reference, winner) in &*self.textures {
            cx.product(ALPHA_CLASS_STEP, winner.as_str());
            map.extend_from_slice(reference.as_bytes());
            map.push(0);
            map.extend_from_slice(winner.as_bytes());
            map.push(0);
        }
        cx.param("textures", Digest::of(&map).0.to_vec());
        Ok(())
    }

    fn run(&self, subject: &str, cx: &mut RunCx<'_>) -> Result<Vec<u8>, BakeError> {
        let key = || ProductKey::new(ALPHA_INDEX_STEP, subject);
        let mut index = AlphaClassIndex::default();
        for (reference, winner) in &*self.textures {
            // Both sides of the lookup come from `mw_nif::texture_path` (lowercased, `/`), so the
            // scene bake's material texture and this key are the same string by construction.
            debug_assert_eq!(*reference, mw_nif::texture_path(reference), "a reference is already normalised");
            let bytes = cx.product(ALPHA_CLASS_STEP, winner.as_str())?;
            let class: AlphaClass = postcard::from_bytes(&bytes).map_err(|e| BakeError::step(&key(), e))?;
            index.classes.insert(reference.clone(), class);
        }
        postcard::to_stdvec(&index).map_err(|e| BakeError::step(&key(), e))
    }

    fn compress(&self) -> bool {
        false
    }
}

/// Reference → winning file for every texture a cutout-candidate material uses
/// ([`TextureUse::candidate`]): the only textures whose alpha class anything reads, so the only
/// ones classified. A reference that resolves to no file is left out, and the rule's answer for a
/// missing class is "not a cutout" (`mw_bake::scene::runtime_alpha`).
pub fn alpha_class_jobs<'a>(
    uses: impl IntoIterator<Item = &'a TextureUse>,
    resolve: impl Fn(&str) -> Option<String>,
) -> BTreeMap<String, String> {
    let mut out = BTreeMap::new();
    for use_ in uses.into_iter().filter(|u| u.candidate) {
        if let Some(winner) = resolve(&use_.texture) {
            out.insert(use_.texture.clone(), winner);
        }
    }
    out
}

/// The baked [`AlphaClassIndex`], or `None` when the texture bake has not run in this store.
pub fn load_alpha_classes(store: &Store, manifest: &Manifest) -> Option<AlphaClassIndex> {
    let digest = manifest.get(ALPHA_INDEX_STEP, ALPHA_INDEX_SUBJECT)?;
    postcard::from_bytes(&store.get(&digest).ok()?).ok()
}

/// What [`bake_delight`] did.
#[derive(Debug, Default)]
pub struct DelightReport {
    /// Textures de-lit (served by the layer).
    pub textures: usize,
    /// Skips by reason (selection and decode time).
    pub skipped: BTreeMap<Skip, u32>,
    /// `texture.delight` products built this session (the rest were cache hits).
    pub built: u32,
    /// Cutout-candidate textures with an [`AlphaClass`] in the index.
    pub alpha_classes: usize,
    /// `texture.alpha_class` products built this session.
    pub alpha_built: u32,
    /// Products that failed (roles of models that don't parse, unreadable textures).
    pub failures: Vec<(ProductKey, String)>,
    pub elapsed: Duration,
    /// Every texture use of the referenced models (`texture.roles`): the texture cook plans from
    /// them ([`crate::cook::bake_cook`]).
    pub uses: Vec<TextureUse>,
    /// The sources the de-light index serves: the cook leaves them to de-light.
    pub delit: BTreeSet<String>,
}

impl fmt::Display for DelightReport {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let reasons: Vec<String> = self.skipped.iter().map(|(s, n)| format!("{s} {n}")).collect();
        writeln!(
            f,
            "delight: {} textures, skipped by reason {{{}}}, {} built, {} failed, {:.1} s\n\
             alpha classes: {} cutout candidates, {} built",
            self.textures,
            reasons.join(", "),
            self.built,
            self.failures.len(),
            self.elapsed.as_secs_f64(),
            self.alpha_classes,
            self.alpha_built
        )
    }
}

/// Registers the de-lighting steps and bakes every model's roles, then every albedo texture and
/// the index. `models` are the referenced model paths; `vfs` the bake's VFS (no baked layer).
pub fn bake_delight(baker: &mut Baker, vfs: &Vfs, models: &[String], strength: f32) -> DelightReport {
    let started = Instant::now();
    let mut report = DelightReport::default();
    baker.register(RolesStep);
    // NIF models that exist (the model bake reports the missing ones).
    let role_keys: Vec<ProductKey> = models
        .iter()
        .filter(|m| extension(m).eq_ignore_ascii_case("nif") && VfsPath::new(m.as_str()).is_ok_and(|p| vfs.contains(&p)))
        .map(|m| ProductKey::new(ROLES_STEP, m.as_str()))
        .collect();
    let roles = baker.bake(&role_keys);
    let store = baker.store().clone();
    let digests: Vec<Digest> = roles
        .products
        .iter()
        .filter_map(|(key, result)| match result {
            Ok(d) => Some(*d),
            Err(e) => {
                report.failures.push((key.clone(), e.to_string()));
                None
            }
        })
        .collect();
    let uses: Vec<TextureUse> = digests
        .par_iter()
        .filter_map(|d| store.get(d).ok().and_then(|bytes| postcard::from_bytes::<Vec<TextureUse>>(&bytes).ok()))
        .flatten()
        .collect();
    let resolve = |r: &str| {
        let path = VfsPath::new(r).ok()?;
        vfs.resolve_preferring(&path, TEXTURE_ALTERNATES).map(|(p, _)| p.as_str().to_owned())
    };
    // Cutout candidates' alpha classes (`AlphaClassStep`), the fact `scene.cells` reads. Not a
    // de-lighting product: baked here because this is where every model's texture uses are
    // resolved, and it must cover candidates de-lighting skips.
    let textures = Arc::new(alpha_class_jobs(&uses, resolve));
    baker.register(AlphaClassStep).register(AlphaIndexStep { textures: textures.clone() });
    let classes = baker.bake(&[ProductKey::new(ALPHA_INDEX_STEP, ALPHA_INDEX_SUBJECT)]);
    for (key, error) in classes.failures() {
        report.failures.push((key.clone(), error.to_string()));
    }
    report.alpha_classes = textures.len();
    report.alpha_built = classes.steps.get(ALPHA_CLASS_STEP).map_or(0, |s| s.built);
    let source_of = |p: &str| VfsPath::new(p).ok().and_then(|p| vfs.resolve(&p));
    let (jobs, skipped) = plan_jobs(uses.iter().cloned(), resolve, source_of);
    report.skipped = skipped;
    let jobs = Arc::new(jobs);
    baker.register(DelightStep { strength, jobs: jobs.clone() }).register(IndexStep { jobs });
    let result = baker.bake(&[ProductKey::new(INDEX_STEP, INDEX_SUBJECT)]);
    for (key, error) in result.failures() {
        report.failures.push((key.clone(), error.to_string()));
    }
    report.built = result.steps.get(DELIGHT_STEP).map_or(0, |s| s.built);
    let index = result
        .products
        .iter()
        .find(|(k, _)| k.step == INDEX_STEP)
        .and_then(|(_, r)| r.as_ref().ok())
        .and_then(|d| store.get(d).ok())
        .and_then(|bytes| postcard::from_bytes::<DelightIndex>(&bytes).ok())
        .unwrap_or_default();
    report.textures = index.entries.len();
    report.delit = index.entries.iter().map(|e| e.source.clone()).collect();
    for (skip, n) in index.skipped {
        *report.skipped.entry(skip).or_default() += n;
    }
    report.uses = uses;
    report.elapsed = started.elapsed();
    report
}

/// The baked index of a profile, if its manifest has one.
pub fn load_index(store: &Store, manifest: &Manifest) -> Option<DelightIndex> {
    let digest = manifest.get(INDEX_STEP, INDEX_SUBJECT)?;
    postcard::from_bytes(&store.get(&digest).ok()?).ok()
}

/// The runtime layer moved to [`crate::cook`]: one layer serves the de-lit and the cooked textures
/// (textures.md §5 "Serving"). Kept here for the callers that ask about the de-lit layer.
pub use crate::cook::{LAYER_LABEL, layer_index};

#[cfg(test)]
mod tests;
