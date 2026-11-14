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
//! - `texture.cook.depth` / `.depth.meta` (per height-normal companion, tx-pom): its height as a
//!   BC4 depth map ([`cook_depth`]), served at [`depth_path`] and listed in [`DEPTHS_PATH`].
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
    AlphaUse, Bc7Quality, BlockFormat, Channel, ChannelMap, PackProfile, PackRole, Rewrap, Role as TexRole, SkipReason, SourceY, SuffixRule,
    cook_levels, encode_level, fill_empty_level0, linear_mip_chain, specular_to_orm,
    format_for, level_len, read_source, rewrap_mapped, write_ktx2,
};
use sf_vfs::{SourceIndex, Vfs, VfsPath, VfsSource};

use crate::delight::{DelightIndex, DelightReport, Role, TextureUse, is_sky_or_ui, sole_variant, stem};
use crate::texture_profiles::TextureSources;
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
/// The file the layer serves with the current ORM-slot companions (postcard
/// `BTreeMap<base stem, OrmCompanion>`, [`read_orms`]).
pub const ORMS_PATH: &str = "cooked/texture_orms.postcard";
/// `texture.cook.depth`: the parallax depth map of one height-normal companion (tx-pom,
/// [`cook_depth`]); subject: the companion's winning file, which `texture.cook` cooks as the normal.
pub const DEPTH_STEP: &str = "texture.cook.depth";
/// `texture.cook.depth.meta`: one depth map's [`CookMeta`].
pub const DEPTH_META_STEP: &str = "texture.cook.depth.meta";
/// Bump when [`cook_depth`]'s output changes.
pub const DEPTH_VERSION: u32 = 1;
/// The file the layer serves with the current depth companions (postcard
/// `BTreeMap<base stem, served depth path>`, [`read_depths`]).
pub const DEPTHS_PATH: &str = "cooked/texture_depths.postcard";
/// Largest side of a depth map's level 0 (top levels dropped above it). Bevy's parallax samples
/// the depth map at level 0 whatever the distance (`parallax_mapping.wesl` `sample_depth_map`:
/// `textureSampleLevel(.., 0.0)`, so the march's 16-32 fetches stay out of a loop with
/// gradients) and its docs warn that larger depth maps cost disproportionately; height is
/// low-frequency, and 1024² over a 2-4 m tile is 2-4 mm per texel.
pub const DEPTH_MAX_SIZE: u32 = 1024;
/// What a depth map's served path adds to its companion's stem ([`depth_path`]).
pub const DEPTH_SUFFIX: &str = "_depth";
/// Bump when the cook's decisions or `sf_texture`'s rewrap, mips or encoders change.
pub const VERSION: u32 = 3;
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
    /// An ORM-slot companion, cooked linear glTF ORM ([`OrmJob`], [`cook_orm`]).
    pub orm: bool,
    /// Level 0's empty channel was rebuilt from level 1 (the profile's
    /// [`sf_texture::Quirk::FillEmptyLevel0`]).
    pub level0_rebuilt: bool,
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
            orm: false,
            level0_rebuilt: false,
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
    /// ORM-slot companions ([`plan_orms`]): base reference stem → the companion.
    pub orms: BTreeMap<String, OrmCompanion>,
    /// The ORM-slot companions to cook by winning file (not in [`Self::jobs`]: [`cook_orm`]).
    pub orm_jobs: BTreeMap<String, OrmJob>,
    /// Height-normal companions whose height becomes a depth map ([`plan_normals`]), by winning
    /// file (also in [`Self::jobs`] as the normal).
    pub depth_jobs: BTreeMap<String, DepthJob>,
    /// Depth companions ([`plan_normals`]): base reference stem → the depth map's served path
    /// ([`depth_path`]).
    pub depths: BTreeMap<String, String>,
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

/// How a height-normal companion's height is laid out (its pack profile's rule, textures-adapter.md
/// §7): canonical A ← the shipped height channel; the cook stores `1 − height` (Bevy's depth).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct DepthJob {
    pub layout: ChannelMap,
}

/// Where the layer serves the depth map cooked from companion `source`: `<stem>_depth.ktx2`.
pub fn depth_path(source: &str) -> String {
    format!("{}{DEPTH_SUFFIX}.ktx2", stem(source))
}

impl CookJob {
    /// The job of a normal-map companion.
    pub const fn normal(layout: ChannelMap, source_y: SourceY) -> Self {
        Self { alpha: CookAlpha::Opaque, candidate: false, normal: Some(NormalJob { layout, source_y }) }
    }
}

/// How an ORM-slot companion is laid out and converted (its pack profile's rule, textures-adapter.md
/// §7, §10).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct OrmJob {
    /// Canonical occlusion, roughness, metallic ← shipped channel (the rule's layout).
    pub layout: ChannelMap,
    /// A legacy specular map ([`PackRole::SpecularShininess`]): decoded and converted
    /// ([`specular_to_orm`]), never rewrapped.
    pub specular: bool,
    /// The canonical channel the profile's [`sf_texture::Quirk::FillEmptyLevel0`] rebuilds at
    /// level 0 when it is empty there and not at level 1; `None`: as shipped.
    pub fill_level0: Option<Channel>,
}

/// An ORM-slot companion the index publishes ([`read_orms`]).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct OrmCompanion {
    /// The companion's winning file.
    pub file: String,
    /// R holds occlusion (a packed ORM, bound as `occlusion_texture` too); false for a converted
    /// specular map, whose R is 1 everywhere (no occlusion fetch).
    pub occlusion: bool,
}

/// The ORM-slot companion of base texture `reference` ([`normal_companion`]'s rule over
/// [`TextureSources::orm_suffixes`]): the first `<stem><suffix>` that resolves and whose winning
/// file's own source profile has that suffix as an ORM rule ([`TextureSources::orm_rule`]). A
/// vanilla source's `_spec` is a specular map (`openmw` profile), a PBR pack's an ORM.
pub fn orm_companion<'s>(
    sources: &'s TextureSources,
    reference: &str,
    mut resolve: impl FnMut(&str) -> Option<(String, SourceIndex)>,
) -> Option<(String, &'s PackProfile, &'s SuffixRule)> {
    let base = stem(reference);
    sources.orm_suffixes().into_iter().find_map(|suffix| {
        let (winner, source) = resolve(&format!("{base}{suffix}.tga"))?;
        sources.orm_rule(source, suffix).map(|(profile, rule)| (winner, profile, rule))
    })
}

/// Adds the ORM-slot companions of every base-colour reference in `uses` to `plan`, as
/// [`plan_normals`] does normals: an [`OrmJob`] per companion ([`CookPlan::orm_jobs`], taken out of [`CookPlan::jobs`]) that is the sole variant of its
/// stem (else [`CookSkip::Ambiguous`]), laid out by its own source's profile, and the base stem →
/// companion map the index publishes ([`CookPlan::orms`]). No tangents needed.
pub fn plan_orms<'a>(
    plan: &mut CookPlan,
    uses: impl IntoIterator<Item = &'a TextureUse>,
    sources: &TextureSources,
    resolve: impl Fn(&str) -> Option<(String, SourceIndex)>,
    exists: impl Fn(&str) -> bool,
) {
    let bases: BTreeSet<&str> = uses.into_iter().filter(|u| u.role == Role::Albedo).map(|u| u.texture.as_str()).collect();
    let mut companions: BTreeMap<String, OrmJob> = BTreeMap::new();
    for reference in bases {
        let Some((winner, profile, rule)) = orm_companion(sources, reference, &resolve) else { continue };
        let specular = rule.role == PackRole::SpecularShininess;
        plan.orms.insert(stem(reference).to_owned(), OrmCompanion { file: winner.clone(), occlusion: !specular });
        let fill_level0 = if specular { None } else { profile.fill_empty_level0(rule.role) };
        companions.insert(winner, OrmJob { layout: rule.layout, specular, fill_level0 });
    }
    for (winner, orm) in companions {
        if sole_variant(&winner, &exists) {
            plan.jobs.remove(&winner);
            plan.orm_jobs.insert(winner, orm);
        } else {
            *plan.skipped.entry(CookSkip::Ambiguous).or_default() += 1;
        }
    }
}

/// The normal-map companion of base texture `reference` (textures-adapter.md §2, §8): the first
/// suffix in [`TextureSources::normal_suffixes`] (`_nh` before `_n`, then `_nm`, then the
/// suffixes only some sources' profiles have) whose `<stem><suffix>` resolves **and** whose
/// winning file's own source profile has that suffix as a normal rule; that profile and rule read
/// it (its layout, its `normal_y`). The companion's source decides, not the albedo's: a
/// normals-only pack over vanilla diffuse is read by the pack's convention. `resolve`: the
/// runtime's rule with [`TEXTURE_ALTERNATES`] and the winner's VFS source, given a `.tga`
/// reference so TGA, KTX2, DDS and PNG files all count. The bake's one companion lookup: the
/// texture cook asks it, `mesh.cook` ([`tangent_rule`]) a superset of it.
pub fn normal_companion<'s>(
    sources: &'s TextureSources,
    reference: &str,
    mut resolve: impl FnMut(&str) -> Option<(String, SourceIndex)>,
) -> Option<(String, &'s PackProfile, &'s SuffixRule)> {
    let base = stem(reference);
    sources.normal_suffixes().into_iter().find_map(|suffix| {
        let (winner, source) = resolve(&format!("{base}{suffix}.tga"))?;
        sources.normal_rule(source, suffix).map(|(profile, rule)| (winner, profile, rule))
    })
}

/// `mesh.cook`'s tangent rule ([`sf_model::steps::TangentRule`], textures-adapter.md §2): a lit
/// material gets `MikkTSpace` tangents when its base texture has a file under any suffix of
/// [`TextureSources::normal_suffixes`] (the union over every source's profile). Every candidate
/// file it looks at goes through [`RunCx::file_exists`], so it becomes a dependency: a pack that
/// adds or removes a companion re-cooks only the models whose materials use that base texture.
/// Existence only (any extension in [`TEXTURE_ALTERNATES`] counts), which is what
/// [`Vfs::resolve_preferring`] needs to answer `Some`. A superset of [`normal_companion`] (it
/// doesn't ask which source's profile owns the suffix) and so of what the cook binds: a companion
/// the texture cook skips (ambiguous, or a suffix its source's profile lacks) still gets tangents,
/// harmless, the runtime binds no normal map for it; never the reverse, which would shade wrong.
/// The rule's version is 1 while the union is the default profile's (unchanged keys for every
/// install that names no profile) and follows the union otherwise, so a new suffix re-cooks.
pub fn tangent_rule(sources: &TextureSources) -> sf_model::steps::TangentRule {
    let suffixes: Vec<String> = sources.normal_suffixes().into_iter().map(str::to_owned).collect();
    let defaults = TextureSources::default();
    let default = defaults.normal_suffixes();
    let version = if suffixes.iter().map(String::as_str).eq(default.iter().copied()) {
        1
    } else {
        let digest = Digest::of(suffixes.join("\n").to_ascii_lowercase().as_bytes()).0;
        u32::from_le_bytes([digest[0], digest[1], digest[2], digest[3]]).max(2)
    };
    sf_model::steps::TangentRule {
        version,
        wants: Arc::new(move |material: &sf_model::Material, cx: &mut RunCx<'_>| {
            let Some(reference) = material.base_color_texture.as_deref().filter(|_| !material.unlit) else { return Ok(false) };
            let base = stem(reference);
            for suffix in &suffixes {
                let Ok(path) = VfsPath::new(&format!("{base}{suffix}.tga")) else { continue };
                let tries = std::iter::once(path.clone()).chain(TEXTURE_ALTERNATES.iter().map(|ext| path.with_extension(ext)));
                for p in tries {
                    if cx.file_exists(p.as_str())? {
                        return Ok(true);
                    }
                }
            }
            Ok(false)
        }),
    }
}

/// Adds the normal-map companions of every base-colour reference in `uses` to `plan`: a
/// [`CookJob::normal`] per companion that is the sole variant of its stem (else counted
/// [`CookSkip::Ambiguous`]), laid out and Y-flipped by its own source's profile
/// ([`normal_companion`]), and the base stem → companion map the index publishes
/// ([`CookPlan::normals`]). Glow, detail and effect slots get no normal map. A changed profile
/// changes the job (the cook's `role` param), so only that source's companions re-cook.
///
/// A companion its profile reads as a height-normal ([`PackRole::NormalHeight`], `_nh`) also gets
/// a [`DepthJob`] ([`CookPlan::depth_jobs`], [`cook_depth`]) and its bases a depth companion
/// ([`CookPlan::depths`]): the winning normal companion decides, so a base whose `_nh` loses to
/// nothing but whose source reads it as a plain normal gets no parallax.
pub fn plan_normals<'a>(
    plan: &mut CookPlan,
    uses: impl IntoIterator<Item = &'a TextureUse>,
    sources: &TextureSources,
    resolve: impl Fn(&str) -> Option<(String, SourceIndex)>,
    exists: impl Fn(&str) -> bool,
) {
    let bases: BTreeSet<&str> = uses.into_iter().filter(|u| u.role == Role::Albedo).map(|u| u.texture.as_str()).collect();
    let mut companions: BTreeMap<String, (NormalJob, Option<DepthJob>)> = BTreeMap::new();
    let mut heights: Vec<(String, String)> = Vec::new();
    for reference in bases {
        let Some((winner, profile, rule)) = normal_companion(sources, reference, &resolve) else { continue };
        plan.normals.insert(stem(reference).to_owned(), winner.clone());
        let depth = (rule.role == PackRole::NormalHeight).then_some(DepthJob { layout: rule.layout });
        if depth.is_some() {
            heights.push((stem(reference).to_owned(), winner.clone()));
        }
        companions.insert(winner, (NormalJob { layout: rule.layout, source_y: profile.normal_y }, depth));
    }
    for (winner, (normal, depth)) in companions {
        if sole_variant(&winner, &exists) {
            plan.jobs.insert(winner.clone(), CookJob::normal(normal.layout, normal.source_y));
            if let Some(depth) = depth {
                plan.depth_jobs.insert(winner, depth);
            }
        } else {
            *plan.skipped.entry(CookSkip::Ambiguous).or_default() += 1;
        }
    }
    for (base, winner) in heights {
        if plan.depth_jobs.contains_key(&winner) {
            plan.depths.insert(base, depth_path(&winner));
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
    cook_with(path, bytes, job, None, quality)
}

/// One ORM-slot companion's `texture.cook` output (TS-2, textures-adapter.md §10): a packed ORM
/// rewrapped through its layout's block-exact remap (decoded and cooked to BC7 linear where none
/// exists), its empty level-0 channel rebuilt if the profile asks ([`OrmJob::fill_level0`]); a
/// legacy specular map decoded and converted. Linear either way.
pub fn cook_orm(path: &str, bytes: &[u8], orm: &OrmJob, quality: Bc7Quality) -> (Vec<u8>, CookMeta) {
    if orm.specular {
        return cook_specular(path, bytes, orm.layout, quality);
    }
    cook_with(path, bytes, &CookJob { alpha: CookAlpha::Opaque, candidate: false, normal: None }, Some(orm), quality)
}

/// One height-normal companion's `texture.cook.depth` output (tx-pom, textures-adapter.md §7): its
/// height channel as Bevy's linear depth map (`1 − height`, BC4). A DXT5 `_nh` splits block-exactly
/// (its alpha block *is* a BC4 block, complemented: `sf_texture::remap`); an uncompressed or BC7
/// one is decoded and its height cooked (`sf_texture::cook`, BC4). Then capped to
/// [`DEPTH_MAX_SIZE`] by dropping top levels. Always a served file (never in place: the shipped
/// file is the normal). `shipped_vram` is 0: the height ships inside the normal companion.
pub fn cook_depth(path: &str, bytes: &[u8], depth: &DepthJob, quality: Bc7Quality) -> (Vec<u8>, CookMeta) {
    let alpha = CookAlpha::Opaque;
    let role = TexRole::height(Channel::A);
    let (ktx2, path_kind) = match rewrap_mapped(bytes, &depth.layout, role) {
        Rewrap::Rewrapped(r) => (r.ktx2, CookPath::Rewrapped { filled: r.levels - r.shipped, remapped: r.remapped }),
        Rewrap::Skip(reason) => {
            let skip = match reason {
                SkipReason::Unaligned { .. } => CookSkip::Unaligned,
                SkipReason::Unsupported(_) => CookSkip::Unsupported,
                SkipReason::Supercompressed => CookSkip::Supercompressed,
                SkipReason::Malformed(_) => CookSkip::Undecodable,
            };
            return (Vec::new(), CookMeta::skipped(skip, alpha));
        }
        Rewrap::Cook(_) => {
            let Ok(mut image) = decode_rgba(path, bytes) else {
                return (Vec::new(), CookMeta::skipped(CookSkip::Undecodable, alpha));
            };
            let (w, h) = (image.width, image.height);
            if !is_block_aligned(w, h) {
                return (Vec::new(), CookMeta::skipped(CookSkip::Unaligned, alpha));
            }
            depth.layout.apply_rgba(&mut image.rgba);
            let levels = cook_levels(role, &image.rgba, w, h, quality);
            (write_ktx2(format_for(role), w, h, &levels), CookPath::Encoded)
        }
    };
    let ktx2 = match sf_texture::level_cap::cap_ktx2(&ktx2, Some(DEPTH_MAX_SIZE)) {
        Ok((capped, plan)) if plan.skip > 0 => capped.into_owned(),
        _ => ktx2,
    };
    let Ok(Some(cooked)) = read_source(&ktx2) else {
        return (Vec::new(), CookMeta::skipped(CookSkip::Undecodable, alpha));
    };
    let meta = CookMeta {
        path: path_kind,
        alpha,
        format: format_name(cooked.format).to_owned(),
        width: cooked.width,
        height: cooked.height,
        level_bytes: cooked.levels.iter().map(|l| l.len() as u64).collect(),
        shipped_vram: 0,
        normal: false,
        orm: false,
        level0_rebuilt: false,
    };
    (ktx2, meta)
}

fn cook_with(path: &str, bytes: &[u8], job: &CookJob, orm_job: Option<&OrmJob>, quality: Bc7Quality) -> (Vec<u8>, CookMeta) {
    let alpha = job.alpha;
    let normal = job.normal.is_some();
    let orm = orm_job.is_some();
    // A normal or ORM companion is remapped to the canonical layout (+Y, linear:
    // `sf_texture::rewrap::canonical`); colour rewraps keep every shipped channel: the alpha use
    // only matters to an encode.
    let (layout, role) = match (job.normal, orm_job) {
        (Some(n), _) => (n.layout, TexRole::Normal { source_y: n.source_y }),
        (None, Some(o)) => (o.layout, TexRole::Packed),
        (None, None) => (ChannelMap::IDENTITY, alpha.role()),
    };
    match rewrap_mapped(bytes, &layout, role) {
        Rewrap::Rewrapped(mut r) => {
            let shipped_vram = read_source(bytes).ok().flatten().map_or(0, |s| s.levels.iter().map(|l| l.len() as u64).sum());
            let level0_rebuilt = match orm_job.and_then(|o| o.fill_level0) {
                Some(channel) => rebuild_level0(&mut r, channel),
                None => false,
            };
            let dds = dds_payload(bytes);
            let in_place = !level0_rebuilt && (r.as_shipped || (dds.is_some() && !r.remapped && r.shipped == r.levels));
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
                orm,
                level0_rebuilt,
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
            let role = if normal || orm {
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
                orm,
                level0_rebuilt: false,
            };
            (write_ktx2(format, w, h, &levels), meta)
        }
    }
}

/// Rebuilds a rewrapped ORM's empty level-0 `channel` from level 1 ([`fill_empty_level0`], the
/// profile's quirk), rewriting its KTX2. False (unchanged) when the rule doesn't apply.
fn rebuild_level0(r: &mut sf_texture::Rewrapped, channel: Channel) -> bool {
    let Ok(Some(source)) = read_source(&r.ktx2) else { return false };
    let mut levels: Vec<Vec<u8>> = source.levels.iter().map(|l| l.to_vec()).collect();
    if !fill_empty_level0(r.format, &mut levels, r.width, r.height, channel) {
        return false;
    }
    r.ktx2 = write_ktx2(r.format, r.width, r.height, &levels);
    true
}

/// A legacy specular map's ORM (textures-adapter.md §10): decoded, laid out by its rule,
/// converted ([`specular_to_orm`]: roughness from the exponent, occlusion 1, dielectric),
/// box-filtered from level 0 and encoded BC1 linear (one channel varies: BC1's 4 bpp keep it at
/// 6 bits, half BC7's size and far cheaper to encode).
fn cook_specular(path: &str, bytes: &[u8], layout: ChannelMap, quality: Bc7Quality) -> (Vec<u8>, CookMeta) {
    let alpha = CookAlpha::Opaque;
    let Ok(mut image) = decode_rgba(path, bytes) else {
        return (Vec::new(), CookMeta::skipped(CookSkip::Undecodable, alpha));
    };
    let (w, h) = (image.width, image.height);
    if !is_block_aligned(w, h) {
        return (Vec::new(), CookMeta::skipped(CookSkip::Unaligned, alpha));
    }
    layout.apply_rgba(&mut image.rgba);
    specular_to_orm(&mut image.rgba);
    let format = BlockFormat::Bc1;
    let levels: Vec<Vec<u8>> =
        linear_mip_chain(&image.rgba, w, h).iter().map(|l| encode_level(format, &l.rgba, l.width, l.height, quality)).collect();
    let shipped_vram = read_source(bytes)
        .ok()
        .flatten()
        .map_or(u64::from(w) * u64::from(h) * 4, |s| s.levels.iter().map(|l| l.len() as u64).sum());
    let meta = CookMeta {
        path: CookPath::Encoded,
        alpha,
        format: format_name(format).to_owned(),
        width: w,
        height: h,
        level_bytes: levels.iter().map(|l| l.len() as u64).collect(),
        shipped_vram,
        normal: false,
        orm: true,
        level0_rebuilt: false,
    };
    (write_ktx2(format, w, h, &levels), meta)
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
    /// [`CookPlan::orm_jobs`]: cooked by [`cook_orm`], keyed by an `orm` param instead of `role`.
    pub orms: Arc<BTreeMap<String, OrmJob>>,
}

/// One planned `texture.cook` subject.
#[derive(Clone, Copy)]
enum Job {
    Texture(CookJob),
    Orm(OrmJob),
}

impl CookStep {
    fn job(&self, subject: &str) -> Result<Job, BakeError> {
        match (self.jobs.get(subject), self.orms.get(subject)) {
            (_, Some(orm)) => Ok(Job::Orm(*orm)),
            (Some(job), None) => Ok(Job::Texture(*job)),
            (None, None) => Err(BakeError::step(&ProductKey::new(COOK_STEP, subject), "not a planned texture")),
        }
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
        let key = || ProductKey::new(COOK_STEP, subject);
        let (name, job) = match self.job(subject)? {
            Job::Texture(job) => ("role", postcard::to_stdvec(&job).map_err(|e| BakeError::step(&key(), e))?),
            Job::Orm(orm) => ("orm", postcard::to_stdvec(&orm).map_err(|e| BakeError::step(&key(), e))?),
        };
        cx.file(subject).param(name, job).param("quality", [u8::from(QUALITY == Bc7Quality::Fast)]);
        Ok(())
    }

    fn run(&self, subject: &str, cx: &mut RunCx<'_>) -> Result<Vec<u8>, BakeError> {
        let job = self.job(subject)?;
        let bytes = cx.read_file(subject)?;
        let (ktx2, meta) = match job {
            Job::Texture(job) => cook_texture(subject, &bytes, &job, QUALITY),
            Job::Orm(orm) => cook_orm(subject, &bytes, &orm, QUALITY),
        };
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

/// `texture.cook.depth`: [`cook_depth`] per height-normal companion, stored like `texture.cook`
/// (KTX2, then the meta trailer).
pub struct DepthStep {
    pub jobs: Arc<BTreeMap<String, DepthJob>>,
}

impl Step for DepthStep {
    fn id(&self) -> &'static str {
        DEPTH_STEP
    }

    fn version(&self) -> u32 {
        DEPTH_VERSION
    }

    fn plan(&self, subject: &str, cx: &mut PlanCx) -> Result<(), BakeError> {
        let key = || ProductKey::new(DEPTH_STEP, subject);
        let job = self.jobs.get(subject).ok_or_else(|| BakeError::step(&key(), "not a planned depth map"))?;
        let job = postcard::to_stdvec(job).map_err(|e| BakeError::step(&key(), e))?;
        cx.file(subject).param("depth", job).param("max", DEPTH_MAX_SIZE.to_le_bytes());
        Ok(())
    }

    fn run(&self, subject: &str, cx: &mut RunCx<'_>) -> Result<Vec<u8>, BakeError> {
        let key = || ProductKey::new(DEPTH_STEP, subject);
        let job = self.jobs.get(subject).ok_or_else(|| BakeError::step(&key(), "not a planned depth map"))?;
        let bytes = cx.read_file(subject)?;
        let (ktx2, meta) = cook_depth(subject, &bytes, job, QUALITY);
        product(ktx2, &meta).map_err(|e| BakeError::step(&key(), e))
    }

    fn compress(&self) -> bool {
        false
    }
}

/// `texture.cook.depth.meta`: a depth map's [`CookMeta`] alone, as [`MetaStep`] for `texture.cook`.
pub struct DepthMetaStep;

impl Step for DepthMetaStep {
    fn id(&self) -> &'static str {
        DEPTH_META_STEP
    }

    fn version(&self) -> u32 {
        1
    }

    fn plan(&self, subject: &str, cx: &mut PlanCx) -> Result<(), BakeError> {
        cx.product(DEPTH_STEP, subject);
        Ok(())
    }

    fn run(&self, subject: &str, cx: &mut RunCx<'_>) -> Result<Vec<u8>, BakeError> {
        let key = || ProductKey::new(DEPTH_META_STEP, subject);
        let bytes = cx.product(DEPTH_STEP, subject)?;
        let (_, meta) = split_product(&bytes).ok_or_else(|| BakeError::step(&key(), "depth product has no meta"))?;
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
    /// Base reference stem → its ORM-slot companion, for companions the cook didn't skip (TS-2;
    /// [`plan_orms`]), read by the material builder through the layer ([`ORMS_PATH`]).
    pub orms: BTreeMap<String, OrmCompanion>,
    /// One entry per depth map the cook made (tx-pom; [`cook_depth`]): `source` is the height-normal
    /// companion, `object` the `texture.cook.depth` product, served at [`depth_path`].
    pub depth_entries: Vec<CookEntry>,
    /// Base reference stem → its depth map's served path, for depth maps the cook made (tx-pom;
    /// [`plan_normals`]), read by the material builder through the layer ([`DEPTHS_PATH`]).
    pub depths: BTreeMap<String, String>,
}

/// `texture.cook.index`: plans every job's source, cook product and meta; reads only the metas.
pub struct IndexStep {
    pub jobs: Arc<BTreeMap<String, CookJob>>,
    /// [`CookPlan::normals`].
    pub normals: Arc<BTreeMap<String, String>>,
    /// [`CookPlan::orm_jobs`] (their sources are indexed too).
    pub orm_jobs: Arc<BTreeMap<String, OrmJob>>,
    /// [`CookPlan::orms`].
    pub orms: Arc<BTreeMap<String, OrmCompanion>>,
    /// [`CookPlan::depth_jobs`] (their depth products are indexed too).
    pub depth_jobs: Arc<BTreeMap<String, DepthJob>>,
    /// [`CookPlan::depths`].
    pub depths: Arc<BTreeMap<String, String>>,
}

impl Step for IndexStep {
    fn id(&self) -> &'static str {
        INDEX_STEP
    }

    fn version(&self) -> u32 {
        4
    }

    fn plan(&self, subject: &str, cx: &mut PlanCx) -> Result<(), BakeError> {
        for source in self.jobs.keys().chain(self.orm_jobs.keys()) {
            cx.file(source.as_str()).product(COOK_STEP, source.as_str()).product(META_STEP, source.as_str());
        }
        for source in self.depth_jobs.keys() {
            cx.file(source.as_str()).product(DEPTH_STEP, source.as_str()).product(DEPTH_META_STEP, source.as_str());
        }
        let key = || ProductKey::new(INDEX_STEP, subject);
        let normals = postcard::to_stdvec(&*self.normals).map_err(|e| BakeError::step(&key(), e))?;
        let orms = postcard::to_stdvec(&*self.orms).map_err(|e| BakeError::step(&key(), e))?;
        let depths = postcard::to_stdvec(&*self.depths).map_err(|e| BakeError::step(&key(), e))?;
        cx.param("normals", normals).param("orms", orms).param("depths", depths);
        Ok(())
    }

    fn run(&self, subject: &str, cx: &mut RunCx<'_>) -> Result<Vec<u8>, BakeError> {
        let key = || ProductKey::new(INDEX_STEP, subject);
        let mut index = CookIndex::default();
        for source in self.jobs.keys().chain(self.orm_jobs.keys()) {
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
        let cooked: BTreeSet<&str> = index.entries.iter().filter(|e| e.meta.orm).map(|e| e.source.as_str()).collect();
        index.orms = self.orms.iter().filter(|(_, c)| cooked.contains(c.file.as_str())).map(|(b, c)| (b.clone(), c.clone())).collect();
        for source in self.depth_jobs.keys() {
            let fingerprint = cx.file_fingerprint(source)?;
            let object = cx.product_digest(DEPTH_STEP, source)?;
            let meta: CookMeta =
                postcard::from_bytes(&cx.product(DEPTH_META_STEP, source)?).map_err(|e| BakeError::step(&key(), e))?;
            if let CookPath::Skipped(skip) = meta.path {
                *index.skipped.entry(skip).or_default() += 1;
                continue;
            }
            index.depth_entries.push(CookEntry { source: source.clone(), fingerprint, object: Some(object), meta });
        }
        let cooked: BTreeSet<String> = index.depth_entries.iter().map(|e| depth_path(&e.source)).collect();
        index.depths = self.depths.iter().filter(|(_, d)| cooked.contains(d.as_str())).map(|(b, d)| (b.clone(), d.clone())).collect();
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
    /// ORM-slot companions published in the index (base textures that get an ORM), of them from
    /// converted specular maps, and ORMs whose empty level-0 channel was rebuilt (profile quirk).
    pub orms: u32,
    pub orms_specular: u32,
    pub level0_rebuilt: u32,
    /// Normal-map companions published in the index (base textures that get a normal map).
    pub normals: u32,
    /// Depth companions published in the index (base textures that get parallax, tx-pom); their
    /// depth maps count among the textures, formats and VRAM above.
    pub depths: u32,
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
        for entry in index.entries.iter().chain(&index.depth_entries) {
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
        self.depths += u32::try_from(index.depths.len()).unwrap_or(u32::MAX);
        self.orms += u32::try_from(index.orms.len()).unwrap_or(u32::MAX);
        self.orms_specular += u32::try_from(index.orms.values().filter(|c| !c.occlusion).count()).unwrap_or(u32::MAX);
        self.level0_rebuilt += u32::try_from(index.entries.iter().filter(|e| e.meta.level0_rebuilt).count()).unwrap_or(u32::MAX);
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
             {} missing; {} normal companions ({} with depth); {} ORM companions ({} from specular maps, {} level 0 rebuilt); by alpha {{{}}}; by format {{{}}}; VRAM as shipped {:.1} MiB -> cooked {:.1} MiB, served from the \
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
            self.depths,
            self.orms,
            self.orms_specular,
            self.level0_rebuilt,
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
/// doesn't serve (`delight`: its report, run first on the same baker); `vfs` is the bake's VFS,
/// `sources` its sources' pack profiles (textures-adapter.md §8).
pub fn bake_cook(baker: &mut Baker, vfs: &Vfs, delight: &DelightReport, sources: &TextureSources) -> CookReport {
    let started = Instant::now();
    let resolve_in = |r: &str| {
        let path = VfsPath::new(r).ok()?;
        vfs.resolve_preferring(&path, TEXTURE_ALTERNATES).map(|(p, s)| (p.as_str().to_owned(), s))
    };
    let resolve = |r: &str| resolve_in(r).map(|(p, _)| p);
    let exists = |p: &str| VfsPath::new(p).is_ok_and(|p| vfs.contains(&p));
    let mut plan = plan_cook(&delight.uses, resolve, exists, &delight.delit);
    plan_normals(&mut plan, &delight.uses, sources, resolve_in, exists);
    plan_orms(&mut plan, &delight.uses, sources, resolve_in, exists);
    let mut report = CookReport { delit: plan.delit, missing: plan.missing, skipped: plan.skipped, ..CookReport::default() };
    let jobs = Arc::new(plan.jobs);
    let normals = Arc::new(plan.normals);
    let orms = Arc::new(plan.orms);
    let orm_jobs = Arc::new(plan.orm_jobs);
    let depth_jobs = Arc::new(plan.depth_jobs);
    let depths = Arc::new(plan.depths);
    baker
        .register(CookStep { jobs: jobs.clone(), orms: orm_jobs.clone() })
        .register(MetaStep)
        .register(DepthStep { jobs: depth_jobs.clone() })
        .register(DepthMetaStep)
        .register(IndexStep { jobs, normals, orm_jobs, orms, depth_jobs, depths });
    let result = baker.bake(&[ProductKey::new(INDEX_STEP, INDEX_SUBJECT)]);
    for (key, error) in result.failures() {
        report.failures.push((key.clone(), error.to_string()));
    }
    for step in [COOK_STEP, DEPTH_STEP] {
        if let Some(stats) = result.steps.get(step) {
            report.built += stats.built;
            report.cpu += stats.run_time;
        }
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
    /// [`ORMS_PATH`]'s bytes: the index's ORM-slot companions whose entry is current.
    orms: Option<Vec<u8>>,
    /// [`DEPTHS_PATH`]'s bytes: the index's depth companions whose depth entry is current.
    depths: Option<Vec<u8>>,
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
    /// Base textures with a current ORM-slot companion ([`ORMS_PATH`]).
    pub orms: usize,
    /// Base textures with a current depth companion ([`DEPTHS_PATH`], tx-pom); the depth maps
    /// themselves count in `cooked` / `stale`.
    pub depths: usize,
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
        // A stale ORM would load as shipped (metal in the wrong channel): none until the next bake.
        let empty = BTreeMap::new();
        let orms: BTreeMap<&String, &OrmCompanion> =
            cook.map_or(&empty, |c| &c.orms).iter().filter(|(_, c)| fresh.contains(c.file.as_str())).collect();
        stats.orms = orms.len();
        let orms = if orms.is_empty() { None } else { postcard::to_stdvec(&orms).ok() };
        // Depth maps (tx-pom): served at their own path while their companion is current; a stale
        // one is withheld with its companion (no parallax until the next bake).
        let mut fresh_depths: BTreeSet<String> = BTreeSet::new();
        for entry in cook.map_or(&[][..], |c| &c.depth_entries) {
            let served = depth_path(&entry.source);
            match (VfsPath::new(&served).ok(), entry.object) {
                (Some(path), Some(object)) if current(&entry.source, entry.fingerprint) => {
                    files.insert(path, Served { object, trailer: true });
                    stats.cooked += 1;
                    fresh_depths.insert(served);
                }
                _ => stats.stale += 1,
            }
        }
        let empty = BTreeMap::new();
        let depths: BTreeMap<&String, &String> =
            cook.map_or(&empty, |c| &c.depths).iter().filter(|(_, d)| fresh_depths.contains(d.as_str())).collect();
        stats.depths = depths.len();
        let depths = if depths.is_empty() { None } else { postcard::to_stdvec(&depths).ok() };
        (Self { store, files, companions, orms, depths }, stats)
    }

    /// The layer's index files and their bytes, if served.
    fn index_files(&self) -> [(&Option<Vec<u8>>, &'static str); 3] {
        [(&self.companions, COMPANIONS_PATH), (&self.orms, ORMS_PATH), (&self.depths, DEPTHS_PATH)]
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
        for (bytes, file) in self.index_files() {
            if bytes.is_some()
                && let Ok(path) = VfsPath::new(file)
            {
                visit(path);
            }
        }
    }

    fn read(&self, path: &VfsPath) -> std::io::Result<Vec<u8>> {
        for (bytes, file) in self.index_files() {
            if path.as_str() == file
                && let Some(bytes) = bytes
            {
                return Ok(bytes.clone());
            }
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

/// The ORM-slot companions the mounted layer serves ([`ORMS_PATH`]): base reference stem →
/// companion. Empty without a bake or without `_spec`-style maps.
pub fn read_orms(vfs: &Vfs) -> BTreeMap<String, OrmCompanion> {
    VfsPath::new(ORMS_PATH)
        .ok()
        .and_then(|p| vfs.read(&p).ok())
        .and_then(|bytes| postcard::from_bytes(&bytes).ok())
        .unwrap_or_default()
}

/// The ORM-slot companion [`read_orms`] lists for a base texture `reference` (as [`companion_of`]).
pub fn orm_of<'a>(orms: &'a BTreeMap<String, OrmCompanion>, reference: &str) -> Option<&'a OrmCompanion> {
    let path = VfsPath::new(reference).ok()?;
    orms.get(stem(path.as_str()))
}

/// The depth companions the mounted layer serves ([`DEPTHS_PATH`], tx-pom): base reference stem →
/// the depth map's served path. Empty without a bake or without `_nh` height-normal maps.
pub fn read_depths(vfs: &Vfs) -> BTreeMap<String, String> {
    VfsPath::new(DEPTHS_PATH)
        .ok()
        .and_then(|p| vfs.read(&p).ok())
        .and_then(|bytes| postcard::from_bytes(&bytes).ok())
        .unwrap_or_default()
}

/// The depth map [`read_depths`] lists for a base texture `reference` (as [`companion_of`]).
pub fn depth_of<'a>(depths: &'a BTreeMap<String, String>, reference: &str) -> Option<&'a str> {
    let path = VfsPath::new(reference).ok()?;
    depths.get(stem(path.as_str())).map(String::as_str)
}

/// The normal-map companion [`read_companions`] lists for a base texture `reference` (any
/// spelling the VFS accepts; the extension is ignored, as the cook's alternates are).
pub fn companion_of<'a>(companions: &'a BTreeMap<String, String>, reference: &str) -> Option<&'a str> {
    let path = VfsPath::new(reference).ok()?;
    companions.get(stem(path.as_str())).map(String::as_str)
}

/// The source index of the baked texture layer in `vfs`, if mounted (always the top-most source).
pub fn layer_index(vfs: &Vfs) -> Option<SourceIndex> {
    let top = vfs.source_count().checked_sub(1)?;
    let index = SourceIndex(u16::try_from(top).ok()?);
    (vfs.source(index).label() == LAYER_LABEL).then_some(index)
}

#[cfg(test)]
mod tests;
