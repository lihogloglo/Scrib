//! The texture cook (`superfargoth/docs/specs/textures.md` §5 "Part A"; adapter
//! `bevyrrowind/docs/specs/textures-adapter.md` §4 "T3 cook"): every texture a referenced model
//! uses, except those the de-light index serves, as `BCn` KTX2 for its role.
//!
//! Products:
//! - `texture.cook` (per winning texture file, keyed by its role): what [`cook_texture`] made, as
//!   one object: the KTX2 file (empty when nothing is served) followed by its [`CookMeta`]
//!   (postcard) and the meta's length (`u32` LE). Stored without LZ4 (`BCn` barely compresses).
//! - `texture.cook.meta` (per winning file): the [`CookMeta`] alone, read from the cook product, so
//!   the index reads a few bytes per texture and a changed texture re-reads only its own product.
//! - `texture.cook.index` (one): a [`CookEntry`] per texture the cook didn't skip (source, its
//!   fingerprint, the served object if any, format, size, level table) and the step-time skips.
//!
//! What each texture becomes ([`CookPath`], textures.md §5 "Rewrap rule", "Authored beats
//! derived"): a block-compressed source whose format suits the role and whose chain is full is
//! **indexed in place** (no copy: the shipped DDS or KTX2 keeps loading; the entry records its
//! level table for the streamer, adapter §5 "index untransformed DDS in place"); one with
//! missing levels or a remap is **rewrapped** (shipped blocks unchanged, levels filled); anything
//! else (TGA, BMP, PNG, uncompressed DDS) is decoded and **encoded** (BC7 sRGB for colour).
//!
//! Roles ([`plan_cook`]): Morrowind's model textures are all colour (normal-map companions are
//! T4). A texture used in a blend, effect, glow or detail slot keeps its alpha; one whose every
//! alpha-tested use is a cutout candidate is tested only if its alpha has holes
//! ([`alpha_is_cutout`], as the loader decides); opaque albedo drops alpha.
//!
//! Runtime ([`CookedTextureLayer`]): one VFS source above every mod serving `<stem>.ktx2` for a
//! de-light or cook entry (de-light first) only while its source is still the winning file with
//! the fingerprint the bake saw and the only variant of its stem ([`sole_variant`]). A changed
//! winner makes the entry stale: the mod's file loads as shipped and the next bake cooks it.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;
use std::sync::Arc;
use std::time::{Duration, Instant};

use serde::{Deserialize, Serialize};
use sf_bake::{BakeError, Baker, Digest, Inputs, Manifest, PlanCx, ProductKey, RunCx, Step, Store};
use sf_texture::ktx2_out::is_block_aligned;
use sf_texture::{
    AlphaUse, Bc7Quality, BlockFormat, ChannelMap, PackProfile, PackRole, Rewrap, Role as TexRole, SkipReason, SourceY, SuffixRule, cook_levels,
    format_for, level_len, read_source, rewrap_mapped, write_ktx2,
};
use sf_vfs::{Vfs, VfsPath, VfsSource};

use crate::delight::{DelightIndex, DelightReport, Role, TextureUse, is_sky_or_ui, sole_variant, stem};
use crate::textures::{TEXTURE_ALTERNATES, alpha_is_cutout, decode_rgba};

/// `texture.cook`: one texture cooked for its role.
pub const COOK_STEP: &str = "texture.cook";
/// `texture.cook.meta`: one cooked texture's [`CookMeta`].
pub const META_STEP: &str = "texture.cook.meta";
/// `texture.cook.index`: what the runtime layer serves and the streamer plans from.
pub const INDEX_STEP: &str = "texture.cook.index";
/// The index's only subject.
pub const INDEX_SUBJECT: &str = "all";
/// The runtime layer's VFS label.
pub const LAYER_LABEL: &str = "cooked textures (bake)";
/// The file the layer serves with the current normal-map companions (postcard
/// `BTreeMap<base stem, companion file>`, [`read_companions`]).
pub const COMPANIONS_PATH: &str = "cooked/texture_normals.postcard";
/// Bump when the cook's decisions or `sf_texture`'s rewrap, mips or encoders change.
pub const VERSION: u32 = 2;
/// The BC7 profile the cook encodes with (textures.md §5: `quality: Basic | Fast`).
pub const QUALITY: Bc7Quality = Bc7Quality::Basic;

/// What a colour texture's alpha is for, merged over every use of it ([`plan_cook`]).
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum CookAlpha {
    /// Every use is opaque: alpha is dropped (cooked as 255).
    Opaque,
    /// Alpha-tested at this cutoff: mips keep coverage.
    Test(f32),
    /// Blended or otherwise read (effects, glow, detail): alpha kept as filtered.
    Blend,
}

impl CookAlpha {
    /// The `sf_texture` role it cooks as.
    pub const fn role(self) -> TexRole {
        TexRole::Color {
            alpha: match self {
                Self::Opaque => AlphaUse::None,
                Self::Test(cutoff) => AlphaUse::Test(cutoff),
                Self::Blend => AlphaUse::Blend,
            },
        }
    }

    fn name(self) -> &'static str {
        match self {
            Self::Opaque => "opaque",
            Self::Test(_) => "cutout",
            Self::Blend => "blend",
        }
    }
}

/// What `texture.cook` needs to cook one winning file.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct CookJob {
    pub alpha: CookAlpha,
    /// The cutoff applies only if the texture's alpha has holes (every alpha-tested use is a cutout
    /// candidate); otherwise the texture blends, as the loader draws it.
    pub candidate: bool,
    /// Set for a normal-map companion (T4): cooked as a linear normal in Bevy's +Y convention.
    pub normal: Option<NormalJob>,
}

/// Why a texture is not cooked (served as shipped).
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum CookSkip {
    /// More than one variant of the stem exists: one `<stem>.ktx2` can't stand for all of them.
    Ambiguous,
    /// Sky or UI texture by name (role `Ui`: sampled 1:1, out of scope).
    SkyUi,
    /// Level 0 isn't a whole number of 4x4 blocks.
    Unaligned,
    /// Cubemap, volume, array, BC6H, signed or Basis texture.
    Unsupported,
    /// A supercompressed KTX2: already a runtime file.
    Supercompressed,
    /// The file doesn't parse as a block-compressed texture, or doesn't decode.
    Undecodable,
}

impl fmt::Display for CookSkip {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::Ambiguous => "ambiguous",
            Self::SkyUi => "sky/ui",
            Self::Unaligned => "unaligned",
            Self::Unsupported => "unsupported",
            Self::Supercompressed => "supercompressed",
            Self::Undecodable => "undecodable",
        })
    }
}

/// How a texture reached its cooked form.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum CookPath {
    /// Already right: the shipped file is the cooked texture, nothing is copied. `dds_payload`:
    /// the byte offset of level 0 in a DDS (levels follow back to back); `None` for a KTX2, whose
    /// own level index says where.
    InPlace { dds_payload: Option<u32> },
    /// Shipped blocks rewrapped into KTX2: `filled` levels added, `remapped` through a
    /// block-exact channel remap.
    Rewrapped { filled: u32, remapped: bool },
    /// Decoded and encoded (BC7 for colour).
    Encoded,
    /// Not cooked.
    Skipped(CookSkip),
}

/// One cooked texture's facts: the trailer of its `texture.cook` product and its
/// `texture.cook.meta` product.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CookMeta {
    pub path: CookPath,
    /// The alpha use it was cooked for (a candidate resolved to `Test` or `Blend`).
    pub alpha: CookAlpha,
    /// [`format_name`] of the cooked levels; empty when skipped.
    pub format: String,
    pub width: u32,
    pub height: u32,
    /// Bytes of each level, level 0 first (its GPU size).
    pub level_bytes: Vec<u64>,
    /// GPU bytes of the texture as shipped: its shipped levels for a block-compressed source,
    /// level 0 as RGBA8 for a decoded one (no mips are generated at load).
    pub shipped_vram: u64,
    /// A normal-map companion, cooked linear in +Y ([`CookJob::normal`]).
    pub normal: bool,
}

impl CookMeta {
    fn skipped(skip: CookSkip, alpha: CookAlpha) -> Self {
        Self {
            path: CookPath::Skipped(skip),
            alpha,
            format: String::new(),
            width: 0,
            height: 0,
            level_bytes: Vec::new(),
            shipped_vram: 0,
            normal: false,
        }
    }

    /// GPU bytes of the cooked chain.
    pub fn cooked_vram(&self) -> u64 {
        self.level_bytes.iter().sum()
    }
}

/// A stable name for a block format (stats and the index).
pub const fn format_name(format: BlockFormat) -> &'static str {
    match format {
        BlockFormat::Bc1 => "bc1",
        BlockFormat::Bc1Srgb => "bc1-srgb",
        BlockFormat::Bc2 => "bc2",
        BlockFormat::Bc2Srgb => "bc2-srgb",
        BlockFormat::Bc3 => "bc3",
        BlockFormat::Bc3Srgb => "bc3-srgb",
        BlockFormat::Bc4 => "bc4",
        BlockFormat::Bc5 => "bc5",
        BlockFormat::Bc7 => "bc7",
        BlockFormat::Bc7Srgb => "bc7-srgb",
    }
}

/// The textures to cook by winning file, the plan-time skips, and how many winners de-light serves.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct CookPlan {
    pub jobs: BTreeMap<String, CookJob>,
    pub skipped: BTreeMap<CookSkip, u32>,
    /// References that resolve to no file.
    pub missing: u32,
    /// Winners left to de-light (its index serves them).
    pub delit: u32,
    /// Normal-map companions ([`plan_normals`]): base reference stem → the companion's winning file.
    pub normals: BTreeMap<String, String>,
}

/// The merged alpha use of one texture's uses (module docs).
pub fn job_for<'a>(uses: impl IntoIterator<Item = &'a TextureUse>) -> CookJob {
    let uses: Vec<&TextureUse> = uses.into_iter().collect();
    if uses.iter().any(|u| matches!(u.role, Role::Effect | Role::Translucent | Role::Glow | Role::Detail)) {
        return CookJob { alpha: CookAlpha::Blend, candidate: false, normal: None };
    }
    let mut cutoffs: BTreeMap<u32, u32> = BTreeMap::new();
    for c in uses.iter().filter_map(|u| u.cutoff) {
        *cutoffs.entry(c.clamp(0.0, 1.0).to_bits()).or_default() += 1;
    }
    // Most common; on a tie the lowest (positive floats order like their bits), as de-light.
    match cutoffs.iter().max_by_key(|(bits, n)| (**n, std::cmp::Reverse(**bits))) {
        Some((bits, _)) => {
            let candidate = uses.iter().filter(|u| u.cutoff.is_some()).all(|u| u.candidate);
            CookJob { alpha: CookAlpha::Test(f32::from_bits(*bits)), candidate, normal: None }
        }
        None => CookJob { alpha: CookAlpha::Opaque, candidate: false, normal: None },
    }
}

/// Plans the cook over every texture use: by winning file (`resolve`, the runtime's rule), minus
/// the winners de-light serves (`delit`), sky/UI names and stems with more than one variant
/// (`exists` says whether a VFS path exists).
pub fn plan_cook<'a>(
    uses: impl IntoIterator<Item = &'a TextureUse>,
    resolve: impl Fn(&str) -> Option<String>,
    exists: impl Fn(&str) -> bool,
    delit: &BTreeSet<String>,
) -> CookPlan {
    let mut by_ref: BTreeMap<&str, Vec<&TextureUse>> = BTreeMap::new();
    for u in uses {
        by_ref.entry(u.texture.as_str()).or_default().push(u);
    }
    let mut plan = CookPlan::default();
    let mut by_winner: BTreeMap<String, Vec<&TextureUse>> = BTreeMap::new();
    for (reference, uses) in by_ref {
        match resolve(reference) {
            Some(winner) => by_winner.entry(winner).or_default().extend(uses),
            None => plan.missing += 1,
        }
    }
    for (winner, uses) in by_winner {
        let skip = if delit.contains(&winner) {
            plan.delit += 1;
            continue;
        } else if !sole_variant(&winner, &exists) {
            CookSkip::Ambiguous
        } else if is_sky_or_ui(&winner) {
            CookSkip::SkyUi
        } else {
            plan.jobs.insert(winner, job_for(uses));
            continue;
        };
        *plan.skipped.entry(skip).or_default() += 1;
    }
    plan
}

/// How a normal-map companion is laid out (its pack profile's rule, textures-adapter.md §7).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct NormalJob {
    /// Canonical X, Y, Z ← shipped channel (the profile's `_n` / `_nh` layout).
    pub layout: ChannelMap,
    /// Which way the shipped green channel points.
    pub source_y: SourceY,
}

impl CookJob {
    /// The job of a normal-map companion.
    pub const fn normal(layout: ChannelMap, source_y: SourceY) -> Self {
        Self { alpha: CookAlpha::Opaque, candidate: false, normal: Some(NormalJob { layout, source_y }) }
    }
}

/// The normal-map companion of base texture `reference` under `profile` (textures-adapter.md §2):
/// the first `Normal` / `NormalHeight` suffix rule (`_nh` before `_n`, then `_nm`) whose
/// `<stem><suffix>` resolves (`resolve`: the runtime's rule with [`TEXTURE_ALTERNATES`], given a
/// `.tga` reference so TGA, KTX2, DDS and PNG files all count). Returns the winning file and rule.
/// The bake's one companion lookup: the texture cook and `mesh.cook` both ask it.
pub fn normal_companion<'p>(
    profile: &'p PackProfile,
    reference: &'p str,
    mut resolve: impl FnMut(&str) -> Option<String>,
) -> Option<(String, &'p SuffixRule)> {
    let base = stem(reference);
    profile
        .companions(base)
        .filter(|(_, rule)| matches!(rule.role, PackRole::Normal | PackRole::NormalHeight))
        .find_map(|(companion, rule)| resolve(&format!("{companion}.tga")).map(|winner| (winner, rule)))
}

/// `mesh.cook`'s tangent rule ([`sf_model::steps::TangentRule`], textures-adapter.md §2): a lit
/// material gets `MikkTSpace` tangents when its base texture has a normal companion
/// ([`normal_companion`], default profile). Every candidate file it looks at goes through
/// [`RunCx::file_exists`], so it becomes a dependency: a pack that adds or removes a companion
/// re-cooks only the models whose materials use that base texture. Existence only (any extension
/// in [`TEXTURE_ALTERNATES`] counts), which is what [`Vfs::resolve_preferring`] needs to answer
/// `Some`. A companion the texture cook later skips (ambiguous) still gets tangents: harmless,
/// the runtime binds no normal map for it.
pub fn tangent_rule() -> sf_model::steps::TangentRule {
    sf_model::steps::TangentRule {
        version: 1,
        wants: Arc::new(|material: &sf_model::Material, cx: &mut RunCx<'_>| {
            let Some(reference) = material.base_color_texture.as_deref().filter(|_| !material.unlit) else { return Ok(false) };
            let mut error = None;
            let found = normal_companion(crate::texture_profiles::default_profile(), reference, |candidate| {
                let path = VfsPath::new(candidate).ok()?;
                let tries = std::iter::once(path.clone()).chain(TEXTURE_ALTERNATES.iter().map(|ext| path.with_extension(ext)));
                for p in tries {
                    match cx.file_exists(p.as_str()) {
                        Ok(true) => return Some(p.as_str().to_owned()),
                        Ok(false) => {}
                        Err(e) => {
                            error.get_or_insert(e);
                            return None;
                        }
                    }
                }
                None
            });
            match error {
                Some(e) => Err(e),
                None => Ok(found.is_some()),
            }
        }),
    }
}

/// Adds the normal-map companions of every base-colour reference in `uses` to `plan`: a
/// [`CookJob::normal`] per companion that is the sole variant of its stem (else counted
/// [`CookSkip::Ambiguous`]), and the base stem → companion map the index publishes
/// ([`CookPlan::normals`]). Glow, detail and effect slots get no normal map.
pub fn plan_normals<'a>(
    plan: &mut CookPlan,
    uses: impl IntoIterator<Item = &'a TextureUse>,
    profile: &PackProfile,
    resolve: impl Fn(&str) -> Option<String>,
    exists: impl Fn(&str) -> bool,
) {
    let bases: BTreeSet<&str> = uses.into_iter().filter(|u| u.role == Role::Albedo).map(|u| u.texture.as_str()).collect();
    let mut companions: BTreeMap<String, NormalJob> = BTreeMap::new();
    for reference in bases {
        let Some((winner, rule)) = normal_companion(profile, reference, &resolve) else { continue };
        plan.normals.insert(stem(reference).to_owned(), winner.clone());
        companions.insert(winner, NormalJob { layout: rule.layout, source_y: profile.normal_y });
    }
    for (winner, normal) in companions {
        if sole_variant(&winner, &exists) {
            plan.jobs.insert(winner, CookJob::normal(normal.layout, normal.source_y));
        } else {
            *plan.skipped.entry(CookSkip::Ambiguous).or_default() += 1;
        }
    }
}

/// Byte offset of level 0 in a DDS file (after the `DX10` extension header when present).
fn dds_payload(bytes: &[u8]) -> Option<u32> {
    (bytes.get(..4)? == b"DDS ").then(|| if bytes.get(84..88) == Some(b"DX10") { 148 } else { 128 })
}

/// Bytes of each level of a `width` x `height` chain of `levels` levels in `format`.
fn level_table(format: BlockFormat, width: u32, height: u32, levels: u32) -> Vec<u64> {
    (0..levels.min(31)).map(|i| level_len(format, (width >> i).max(1), (height >> i).max(1)) as u64).collect()
}

/// One texture's `texture.cook` output: the KTX2 file to serve (empty when the shipped file is
/// served: in place or skipped) and its [`CookMeta`]. `path` decides how it decodes when it isn't
/// block-compressed (by extension).
pub fn cook_texture(path: &str, bytes: &[u8], job: &CookJob, quality: Bc7Quality) -> (Vec<u8>, CookMeta) {
    let alpha = job.alpha;
    let normal = job.normal.is_some();
    // A normal companion is remapped to the canonical layout (+Y, linear: `sf_texture::rewrap::canonical`);
    // colour rewraps keep every shipped channel: the alpha use only matters to an encode.
    let (layout, role) = match job.normal {
        Some(n) => (n.layout, TexRole::Normal { source_y: n.source_y }),
        None => (ChannelMap::IDENTITY, alpha.role()),
    };
    match rewrap_mapped(bytes, &layout, role) {
        Rewrap::Rewrapped(r) => {
            let shipped_vram = read_source(bytes).ok().flatten().map_or(0, |s| s.levels.iter().map(|l| l.len() as u64).sum());
            let dds = dds_payload(bytes);
            let in_place = r.as_shipped || (dds.is_some() && !r.remapped && r.shipped == r.levels);
            let meta = CookMeta {
                path: if in_place {
                    CookPath::InPlace { dds_payload: dds }
                } else {
                    CookPath::Rewrapped { filled: r.levels - r.shipped, remapped: r.remapped }
                },
                alpha,
                format: format_name(r.format).to_owned(),
                width: r.width,
                height: r.height,
                level_bytes: level_table(r.format, r.width, r.height, r.levels),
                shipped_vram,
                normal,
            };
            (if in_place { Vec::new() } else { r.ktx2 }, meta)
        }
        Rewrap::Skip(reason) => {
            let skip = match reason {
                SkipReason::Unaligned { .. } => CookSkip::Unaligned,
                SkipReason::Unsupported(_) => CookSkip::Unsupported,
                SkipReason::Supercompressed => CookSkip::Supercompressed,
                SkipReason::Malformed(_) => CookSkip::Undecodable,
            };
            (Vec::new(), CookMeta::skipped(skip, alpha))
        }
        Rewrap::Cook(_) => {
            let Ok(mut image) = decode_rgba(path, bytes) else {
                return (Vec::new(), CookMeta::skipped(CookSkip::Undecodable, alpha));
            };
            let (w, h) = (image.width, image.height);
            if !is_block_aligned(w, h) {
                return (Vec::new(), CookMeta::skipped(CookSkip::Unaligned, alpha));
            }
            let alpha = match alpha {
                CookAlpha::Test(_) if job.candidate && !alpha_is_cutout(&image) => CookAlpha::Blend,
                other => other,
            };
            let role = if normal {
                // The decoded texels in the role's shipped layout; the Y convention is the cook's.
                layout.apply_rgba(&mut image.rgba);
                role
            } else {
                alpha.role()
            };
            let levels = cook_levels(role, &image.rgba, w, h, quality);
            let format = format_for(role);
            let meta = CookMeta {
                path: CookPath::Encoded,
                alpha,
                format: format_name(format).to_owned(),
                width: w,
                height: h,
                level_bytes: levels.iter().map(|l| l.len() as u64).collect(),
                shipped_vram: u64::from(w) * u64::from(h) * 4,
                normal,
            };
            (write_ktx2(format, w, h, &levels), meta)
        }
    }
}

/// The `texture.cook` object: the KTX2 bytes, then the meta and its length.
fn product(ktx2: Vec<u8>, meta: &CookMeta) -> Result<Vec<u8>, postcard::Error> {
    let mut out = ktx2;
    let tail = postcard::to_stdvec(meta)?;
    let len = u32::try_from(tail.len()).unwrap_or(u32::MAX);
    out.extend_from_slice(&tail);
    out.extend_from_slice(&len.to_le_bytes());
    Ok(out)
}

/// The KTX2 length and the meta of a `texture.cook` object.
pub fn split_product(bytes: &[u8]) -> Option<(usize, CookMeta)> {
    let len_at = bytes.len().checked_sub(4)?;
    let len = u32::from_le_bytes(bytes[len_at..].try_into().ok()?) as usize;
    let meta_at = len_at.checked_sub(len)?;
    Some((meta_at, postcard::from_bytes(&bytes[meta_at..len_at]).ok()?))
}

/// `texture.cook`: [`cook_texture`] one planned winner.
pub struct CookStep {
    pub jobs: Arc<BTreeMap<String, CookJob>>,
}

impl CookStep {
    fn job(&self, subject: &str) -> Result<&CookJob, BakeError> {
        self.jobs.get(subject).ok_or_else(|| BakeError::step(&ProductKey::new(COOK_STEP, subject), "not a planned texture"))
    }
}

impl Step for CookStep {
    fn id(&self) -> &'static str {
        COOK_STEP
    }

    fn version(&self) -> u32 {
        VERSION
    }

    fn plan(&self, subject: &str, cx: &mut PlanCx) -> Result<(), BakeError> {
        let job = self.job(subject)?;
        let role = postcard::to_stdvec(job).map_err(|e| BakeError::step(&ProductKey::new(COOK_STEP, subject), e))?;
        cx.file(subject).param("role", role).param("quality", [u8::from(QUALITY == Bc7Quality::Fast)]);
        Ok(())
    }

    fn run(&self, subject: &str, cx: &mut RunCx<'_>) -> Result<Vec<u8>, BakeError> {
        let job = *self.job(subject)?;
        let bytes = cx.read_file(subject)?;
        let (ktx2, meta) = cook_texture(subject, &bytes, &job, QUALITY);
        product(ktx2, &meta).map_err(|e| BakeError::step(&ProductKey::new(COOK_STEP, subject), e))
    }

    fn compress(&self) -> bool {
        false
    }
}

/// `texture.cook.meta`: the [`CookMeta`] of one `texture.cook` product, postcard.
pub struct MetaStep;

impl Step for MetaStep {
    fn id(&self) -> &'static str {
        META_STEP
    }

    fn version(&self) -> u32 {
        1
    }

    fn plan(&self, subject: &str, cx: &mut PlanCx) -> Result<(), BakeError> {
        cx.product(COOK_STEP, subject);
        Ok(())
    }

    fn run(&self, subject: &str, cx: &mut RunCx<'_>) -> Result<Vec<u8>, BakeError> {
        let key = || ProductKey::new(META_STEP, subject);
        let bytes = cx.product(COOK_STEP, subject)?;
        let (_, meta) = split_product(&bytes).ok_or_else(|| BakeError::step(&key(), "cook product has no meta"))?;
        postcard::to_stdvec(&meta).map_err(|e| BakeError::step(&key(), e))
    }
}

/// One texture in the cook index.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CookEntry {
    /// The winning source file.
    pub source: String,
    /// The source's bake fingerprint when it was cooked.
    pub fingerprint: Digest,
    /// The `texture.cook` object the layer serves; `None` when the source is served in place.
    pub object: Option<Digest>,
    pub meta: CookMeta,
}

/// `texture.cook.index`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct CookIndex {
    pub entries: Vec<CookEntry>,
    /// Textures skipped at cook time, by reason.
    pub skipped: BTreeMap<CookSkip, u32>,
    /// Base reference stem → its normal-map companion's winning file, for companions the cook didn't
    /// skip (T4; [`plan_normals`]). The runtime material builder reads it through the layer
    /// ([`COMPANIONS_PATH`]), never probing the VFS per material.
    pub normals: BTreeMap<String, String>,
}

/// `texture.cook.index`: plans every job's source, cook product and meta; reads only the metas.
pub struct IndexStep {
    pub jobs: Arc<BTreeMap<String, CookJob>>,
    /// [`CookPlan::normals`].
    pub normals: Arc<BTreeMap<String, String>>,
}

impl Step for IndexStep {
    fn id(&self) -> &'static str {
        INDEX_STEP
    }

    fn version(&self) -> u32 {
        2
    }

    fn plan(&self, subject: &str, cx: &mut PlanCx) -> Result<(), BakeError> {
        for source in self.jobs.keys() {
            cx.file(source.as_str()).product(COOK_STEP, source.as_str()).product(META_STEP, source.as_str());
        }
        let normals = postcard::to_stdvec(&*self.normals).map_err(|e| BakeError::step(&ProductKey::new(INDEX_STEP, subject), e))?;
        cx.param("normals", normals);
        Ok(())
    }

    fn run(&self, subject: &str, cx: &mut RunCx<'_>) -> Result<Vec<u8>, BakeError> {
        let key = || ProductKey::new(INDEX_STEP, subject);
        let mut index = CookIndex::default();
        for source in self.jobs.keys() {
            let fingerprint = cx.file_fingerprint(source)?;
            let object = cx.product_digest(COOK_STEP, source)?;
            let meta: CookMeta = postcard::from_bytes(&cx.product(META_STEP, source)?).map_err(|e| BakeError::step(&key(), e))?;
            let object = match meta.path {
                CookPath::Skipped(skip) => {
                    *index.skipped.entry(skip).or_default() += 1;
                    continue;
                }
                CookPath::InPlace { .. } => None,
                CookPath::Rewrapped { .. } | CookPath::Encoded => Some(object),
            };
            index.entries.push(CookEntry { source: source.clone(), fingerprint, object, meta });
        }
        let cooked: BTreeSet<&str> = index.entries.iter().filter(|e| e.meta.normal).map(|e| e.source.as_str()).collect();
        index.normals = self.normals.iter().filter(|(_, c)| cooked.contains(c.as_str())).map(|(b, c)| (b.clone(), c.clone())).collect();
        postcard::to_stdvec(&index).map_err(|e| BakeError::step(&key(), e))
    }

    fn compress(&self) -> bool {
        false
    }
}

/// The baked cook index of a profile, if its manifest has one.
pub fn load_index(store: &Store, manifest: &Manifest) -> Option<CookIndex> {
    let digest = manifest.get(INDEX_STEP, INDEX_SUBJECT)?;
    postcard::from_bytes(&store.get(&digest).ok()?).ok()
}

/// What [`bake_cook`] did: the bake stats line (textures.md §8, with the T0 census totals).
#[derive(Debug, Default)]
pub struct CookReport {
    /// Index entries by how they were cooked: in place, rewrapped, encoded.
    pub in_place: u32,
    pub rewrapped: u32,
    /// Of the rewraps: with levels filled, through a remap.
    pub filled: u32,
    pub remapped: u32,
    pub encoded: u32,
    /// Winners left to de-light.
    pub delit: u32,
    /// References that resolve to no file.
    pub missing: u32,
    /// Entries by alpha use ([`CookAlpha`] name) and by cooked format.
    pub by_alpha: BTreeMap<&'static str, u32>,
    pub by_format: BTreeMap<String, u32>,
    /// GPU bytes of every entry as shipped and as cooked (all levels resident).
    pub vram_shipped: u64,
    pub vram_cooked: u64,
    /// Bytes the layer serves from the store (rewrapped and encoded KTX2 files).
    pub served_bytes: u64,
    /// Normal-map companions published in the index (base textures that get a normal map).
    pub normals: u32,
    /// Skips by reason (plan and cook time).
    pub skipped: BTreeMap<CookSkip, u32>,
    /// `texture.cook` products built this run (the rest were cache hits), and their summed run time.
    pub built: u32,
    pub cpu: Duration,
    pub failures: Vec<(ProductKey, String)>,
    pub elapsed: Duration,
}

impl CookReport {
    /// Adds an index's entries and skips to the counts.
    pub fn count(&mut self, index: &CookIndex) {
        for entry in &index.entries {
            let meta = &entry.meta;
            match meta.path {
                CookPath::InPlace { .. } => self.in_place += 1,
                CookPath::Rewrapped { filled, remapped } => {
                    self.rewrapped += 1;
                    self.filled += u32::from(filled > 0);
                    self.remapped += u32::from(remapped);
                }
                CookPath::Encoded => self.encoded += 1,
                CookPath::Skipped(_) => {}
            }
            *self.by_alpha.entry(meta.alpha.name()).or_default() += 1;
            *self.by_format.entry(meta.format.clone()).or_default() += 1;
            self.vram_shipped += meta.shipped_vram;
            self.vram_cooked += meta.cooked_vram();
            if entry.object.is_some() {
                self.served_bytes += meta.cooked_vram();
            }
        }
        for (skip, n) in &index.skipped {
            *self.skipped.entry(*skip).or_default() += n;
        }
        self.normals += u32::try_from(index.normals.len()).unwrap_or(u32::MAX);
    }
}

fn mib(bytes: u64) -> f64 {
    #[allow(clippy::cast_precision_loss)] // a printed size
    let bytes = bytes as f64;
    bytes / (1024.0 * 1024.0)
}

fn counts<K: fmt::Display>(map: &BTreeMap<K, u32>) -> String {
    map.iter().map(|(k, n)| format!("{k} {n}")).collect::<Vec<_>>().join(", ")
}

impl fmt::Display for CookReport {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            "texture.cook: {} textures (in place {}, rewrapped {} [filled {}, remapped {}], encoded {}), {} left to de-light, \
             {} missing; {} normal companions; by alpha {{{}}}; by format {{{}}}; VRAM as shipped {:.1} MiB -> cooked {:.1} MiB, served from the \
             bake {:.1} MiB; skipped by reason {{{}}}; {} built, {:.1} CPU-s, {} failed, {:.1} s",
            self.in_place + self.rewrapped + self.encoded,
            self.in_place,
            self.rewrapped,
            self.filled,
            self.remapped,
            self.encoded,
            self.delit,
            self.missing,
            self.normals,
            counts(&self.by_alpha),
            counts(&self.by_format),
            mib(self.vram_shipped),
            mib(self.vram_cooked),
            mib(self.served_bytes),
            counts(&self.skipped),
            self.built,
            self.cpu.as_secs_f64(),
            self.failures.len(),
            self.elapsed.as_secs_f64()
        )
    }
}

/// Registers the cook steps and cooks every texture the referenced models use that de-light
/// doesn't serve (`delight`: its report, run first on the same baker); `vfs` is the bake's VFS.
pub fn bake_cook(baker: &mut Baker, vfs: &Vfs, delight: &DelightReport) -> CookReport {
    let started = Instant::now();
    let resolve = |r: &str| {
        let path = VfsPath::new(r).ok()?;
        vfs.resolve_preferring(&path, TEXTURE_ALTERNATES).map(|(p, _)| p.as_str().to_owned())
    };
    let exists = |p: &str| VfsPath::new(p).is_ok_and(|p| vfs.contains(&p));
    let mut plan = plan_cook(&delight.uses, resolve, exists, &delight.delit);
    plan_normals(&mut plan, &delight.uses, crate::texture_profiles::default_profile(), resolve, exists);
    let mut report = CookReport { delit: plan.delit, missing: plan.missing, skipped: plan.skipped, ..CookReport::default() };
    let jobs = Arc::new(plan.jobs);
    let normals = Arc::new(plan.normals);
    baker.register(CookStep { jobs: jobs.clone() }).register(MetaStep).register(IndexStep { jobs, normals });
    let result = baker.bake(&[ProductKey::new(INDEX_STEP, INDEX_SUBJECT)]);
    for (key, error) in result.failures() {
        report.failures.push((key.clone(), error.to_string()));
    }
    if let Some(stats) = result.steps.get(COOK_STEP) {
        report.built = stats.built;
        report.cpu = stats.run_time;
    }
    let store = baker.store();
    let index = result
        .products
        .iter()
        .find(|(k, _)| k.step == INDEX_STEP)
        .and_then(|(_, r)| r.as_ref().ok())
        .and_then(|d| store.get(d).ok())
        .and_then(|bytes| postcard::from_bytes::<CookIndex>(&bytes).ok())
        .unwrap_or_default();
    report.count(&index);
    report.elapsed = started.elapsed();
    report
}

/// One file the layer serves.
#[derive(Clone, Copy, Debug)]
struct Served {
    object: Digest,
    /// A `texture.cook` object: the KTX2 is followed by its meta, cut off when read.
    trailer: bool,
}

/// The runtime texture layer: serves `<stem>.ktx2` for the de-light and cook index entries that
/// are still current (module docs).
pub struct CookedTextureLayer {
    store: Store,
    files: BTreeMap<VfsPath, Served>,
    /// [`COMPANIONS_PATH`]'s bytes: the index's normal companions whose entry is current.
    companions: Option<Vec<u8>>,
}

/// What [`CookedTextureLayer::build`] found.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct LayerStats {
    /// De-lit textures served.
    pub delit: usize,
    /// Cooked (rewrapped or encoded) textures served.
    pub cooked: usize,
    /// Cook entries whose current source is itself the cooked texture (nothing to serve).
    pub in_place: usize,
    /// Entries whose source is no longer the sole winner with the baked fingerprint.
    pub stale: usize,
    /// Base textures with a current normal-map companion ([`COMPANIONS_PATH`]).
    pub normals: usize,
}

impl CookedTextureLayer {
    /// The layer over `vfs` (which must not contain it): every de-light entry, then every cook
    /// entry for a stem not served yet, whose source is still the sole variant of its stem in
    /// `vfs` ([`sole_variant`]) with the fingerprint the bake recorded (`inputs` over `vfs`).
    pub fn build(
        store: Store,
        delight: Option<&DelightIndex>,
        cook: Option<&CookIndex>,
        vfs: &Vfs,
        inputs: &dyn Inputs,
    ) -> (Self, LayerStats) {
        let exists = |p: &str| VfsPath::new(p).is_ok_and(|p| vfs.contains(&p));
        let current = |source: &str, fingerprint: Digest| {
            sole_variant(source, exists) && inputs.file_fingerprint(source).ok().flatten() == Some(fingerprint)
        };
        let served_path = |source: &str| VfsPath::new(&format!("{}.ktx2", stem(source))).ok();
        let mut files = BTreeMap::new();
        let mut stats = LayerStats::default();
        for entry in delight.map_or(&[][..], |d| &d.entries) {
            match served_path(&entry.source) {
                Some(path) if current(&entry.source, entry.fingerprint) => {
                    files.insert(path, Served { object: entry.ktx2, trailer: false });
                    stats.delit += 1;
                }
                _ => stats.stale += 1,
            }
        }
        let mut fresh: BTreeSet<&str> = BTreeSet::new();
        for entry in cook.map_or(&[][..], |c| &c.entries) {
            let path = served_path(&entry.source);
            if path.as_ref().is_some_and(|p| files.contains_key(p)) {
                continue; // De-light serves this stem (the bake plans no such entry; a stale pair).
            }
            match (path, entry.object) {
                (Some(_), None) if current(&entry.source, entry.fingerprint) => {
                    stats.in_place += 1;
                    fresh.insert(&entry.source);
                }
                (Some(path), Some(object)) if current(&entry.source, entry.fingerprint) => {
                    files.insert(path, Served { object, trailer: true });
                    stats.cooked += 1;
                    fresh.insert(&entry.source);
                }
                _ => stats.stale += 1,
            }
        }
        // A stale companion would load as shipped (its Y convention unconverted): no normal map
        // until the next bake rather than inverted relief.
        let empty = BTreeMap::new();
        let normals: BTreeMap<&String, &String> =
            cook.map_or(&empty, |c| &c.normals).iter().filter(|(_, c)| fresh.contains(c.as_str())).collect();
        stats.normals = normals.len();
        let companions = if normals.is_empty() { None } else { postcard::to_stdvec(&normals).ok() };
        (Self { store, files, companions }, stats)
    }
}

impl VfsSource for CookedTextureLayer {
    fn label(&self) -> &str {
        LAYER_LABEL
    }

    fn visit_files(&self, visit: &mut dyn FnMut(VfsPath)) {
        for path in self.files.keys() {
            visit(path.clone());
        }
        if self.companions.is_some()
            && let Ok(path) = VfsPath::new(COMPANIONS_PATH)
        {
            visit(path);
        }
    }

    fn read(&self, path: &VfsPath) -> std::io::Result<Vec<u8>> {
        if path.as_str() == COMPANIONS_PATH
            && let Some(bytes) = &self.companions
        {
            return Ok(bytes.clone());
        }
        let served = self.files.get(path).ok_or_else(|| std::io::Error::new(std::io::ErrorKind::NotFound, path.to_string()))?;
        let mut bytes = self.store.get(&served.object)?;
        if served.trailer {
            let (len, _) = split_product(&bytes)
                .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::InvalidData, format!("{path}: cook product has no meta")))?;
            bytes.truncate(len);
        }
        Ok(bytes)
    }
}

/// The normal-map companions the mounted layer serves ([`COMPANIONS_PATH`]): base reference stem
/// (`textures/foo` for `textures/foo.tga`) → companion file to load. Empty without a bake or a pack.
pub fn read_companions(vfs: &Vfs) -> BTreeMap<String, String> {
    VfsPath::new(COMPANIONS_PATH)
        .ok()
        .and_then(|p| vfs.read(&p).ok())
        .and_then(|bytes| postcard::from_bytes(&bytes).ok())
        .unwrap_or_default()
}

/// The normal-map companion [`read_companions`] lists for a base texture `reference` (any
/// spelling the VFS accepts; the extension is ignored, as the cook's alternates are).
pub fn companion_of<'a>(companions: &'a BTreeMap<String, String>, reference: &str) -> Option<&'a str> {
    let path = VfsPath::new(reference).ok()?;
    companions.get(stem(path.as_str())).map(String::as_str)
}

/// The source index of the baked texture layer in `vfs`, if mounted (always the top-most source).
pub fn layer_index(vfs: &Vfs) -> Option<sf_vfs::SourceIndex> {
    let top = vfs.source_count().checked_sub(1)?;
    let index = sf_vfs::SourceIndex(u16::try_from(top).ok()?);
    (vfs.source(index).label() == LAYER_LABEL).then_some(index)
}

#[cfg(test)]
mod tests;
