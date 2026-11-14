use std::sync::Arc;

use ddsfile::{D3DFormat, Dds, NewD3dParams};
use sf_bake::{BakeError, Baker, Digest, Input, Inputs, Store};
use sf_mods::{NormalY, TextureSection};
use sf_vfs::{MemSource, Vfs, VfsPath};

use super::*;
use crate::delight::{DelightIndex, IndexEntry};

fn use_of(texture: &str, role: Role, cutoff: Option<f32>, candidate: bool) -> TextureUse {
    TextureUse { texture: texture.to_owned(), role, clamp: [false; 2], cutoff, candidate }
}

/// A `w` x `h` PNG; `alpha(x, y)` per texel, `seed` changes the colours.
fn png_with(w: u32, h: u32, seed: u8, alpha: impl Fn(u32, u32) -> u8) -> Vec<u8> {
    let image = image::RgbaImage::from_fn(w, h, |x, y| image::Rgba([u8::try_from(x % 256).unwrap(), seed, 90, alpha(x, y)]));
    let mut bytes = std::io::Cursor::new(Vec::new());
    image.write_to(&mut bytes, image::ImageFormat::Png).unwrap();
    bytes.into_inner()
}

fn png(w: u32, h: u32, seed: u8) -> Vec<u8> {
    png_with(w, h, seed, |_, _| 255)
}

/// A legacy (`FourCC`) DDS of zeroed blocks with `levels` mip levels.
fn dds(format: D3DFormat, w: u32, h: u32, levels: u32) -> Vec<u8> {
    let dds = Dds::new_d3d(NewD3dParams { height: h, width: w, depth: None, format, mipmap_levels: Some(levels), caps2: None }).unwrap();
    let mut out = Vec::new();
    dds.write(&mut out).unwrap();
    out
}

const OPAQUE: CookJob = CookJob { alpha: CookAlpha::Opaque, candidate: false, normal: None };

#[test]
fn role_table() {
    let t = "textures/a.dds";
    let job = |uses: &[TextureUse]| job_for(uses);
    // Opaque albedo and emissive base colour drop alpha.
    assert_eq!(job(&[use_of(t, Role::Albedo, None, false), use_of(t, Role::Emissive, None, false)]), OPAQUE);
    // Any blend, effect, glow or detail use keeps alpha.
    for role in [Role::Translucent, Role::Effect, Role::Glow, Role::Detail] {
        let uses = [use_of(t, Role::Albedo, Some(0.5), false), use_of(t, role, None, false)];
        assert_eq!(job(&uses), CookJob { alpha: CookAlpha::Blend, candidate: false, normal: None }, "{role:?}");
    }
    // Alpha test: the most common cutoff (ties: the lowest); candidate only if every tested use is.
    let uses = [
        use_of(t, Role::Albedo, Some(0.5), true),
        use_of(t, Role::Albedo, Some(0.25), true),
        use_of(t, Role::Albedo, Some(0.5), false),
        use_of(t, Role::Albedo, None, false),
    ];
    assert_eq!(job(&uses), CookJob { alpha: CookAlpha::Test(0.5), candidate: false, normal: None });
    assert_eq!(job(&uses[..2]), CookJob { alpha: CookAlpha::Test(0.25), candidate: true, normal: None });
    // Every colour alpha use cooks to BC7 sRGB when encoded.
    for alpha in [CookAlpha::Opaque, CookAlpha::Test(0.5), CookAlpha::Blend] {
        assert_eq!(format_for(alpha.role()), BlockFormat::Bc7Srgb);
    }
}

#[test]
fn source_to_format_table() {
    let cook = |path: &str, bytes: &[u8]| cook_texture(path, bytes, &OPAQUE, Bc7Quality::Fast);
    // Block-compressed DDS with a full chain: in place, no bytes, its colour format in sRGB.
    for (format, name, block) in [(D3DFormat::DXT1, "bc1-srgb", 8), (D3DFormat::DXT3, "bc2-srgb", 16), (D3DFormat::DXT5, "bc3-srgb", 16)] {
        let (ktx2, meta) = cook("textures/a.dds", &dds(format, 64, 32, 7));
        assert!(ktx2.is_empty(), "{name}");
        assert_eq!(meta.path, CookPath::InPlace { dds_payload: Some(128) }, "{name}");
        assert_eq!((meta.format.as_str(), meta.width, meta.height), (name, 64, 32));
        let expected: Vec<u64> = [(16, 8), (8, 4), (4, 2), (2, 1), (1, 1), (1, 1), (1, 1)].iter().map(|(w, h)| w * h * block).collect();
        assert_eq!(meta.level_bytes, expected, "{name}");
        assert_eq!(meta.shipped_vram, meta.cooked_vram(), "{name}");
    }
    // Missing levels: rewrapped, the 6 missing levels filled in the source format.
    let (ktx2, meta) = cook("textures/a.dds", &dds(D3DFormat::DXT1, 64, 32, 1));
    assert_eq!(&ktx2[..6], b"\xABKTX 2");
    assert_eq!(meta.path, CookPath::Rewrapped { filled: 6, remapped: false });
    assert_eq!((meta.format.as_str(), meta.level_bytes.len(), meta.shipped_vram), ("bc1-srgb", 7, 16 * 8 * 8));
    // Not block-compressed: decoded and encoded to BC7 sRGB with the full chain.
    let (ktx2, meta) = cook("textures/a.png", &png(64, 32, 0));
    assert_eq!(&ktx2[..6], b"\xABKTX 2");
    assert_eq!(meta.path, CookPath::Encoded);
    assert_eq!((meta.format.as_str(), meta.level_bytes.len(), meta.shipped_vram), ("bc7-srgb", 7, 64 * 32 * 4));
    assert_eq!(meta.level_bytes[0], 16 * 8 * 16);
}

#[test]
fn skip_reasons() {
    let cook = |path: &str, bytes: &[u8]| cook_texture(path, bytes, &OPAQUE, Bc7Quality::Fast);
    let skip = |path: &str, bytes: &[u8]| {
        let (ktx2, meta) = cook(path, bytes);
        assert!(ktx2.is_empty());
        meta.path
    };
    assert_eq!(skip("textures/a.png", &png(6, 8, 0)), CookPath::Skipped(CookSkip::Unaligned));
    assert_eq!(skip("textures/a.dds", &dds(D3DFormat::DXT1, 6, 8, 1)), CookPath::Skipped(CookSkip::Unaligned));
    assert_eq!(skip("textures/a.tga", b"not a texture"), CookPath::Skipped(CookSkip::Undecodable));
    assert_eq!(skip("textures/a.dds", b"DDS truncated"), CookPath::Skipped(CookSkip::Undecodable));

    // Plan time: de-lit winners left to de-light, missing references, ambiguous stems, sky/UI.
    let files = ["textures/a.dds", "textures/b.dds", "textures/c.tga", "textures/c.dds", "textures/tx_sky_clear.dds", "textures/menu_x.dds"];
    let exists = |p: &str| files.contains(&p);
    let resolve = |r: &str| {
        let s = stem(r);
        ["dds", "tga"].iter().map(|e| format!("{s}.{e}")).find(|p| exists(p))
    };
    let uses: Vec<TextureUse> = ["a", "b", "c", "tx_sky_clear", "menu_x", "gone"]
        .iter()
        .map(|n| use_of(&format!("textures/{n}.tga"), Role::Albedo, None, false))
        .collect();
    let delit = BTreeSet::from(["textures/a.dds".to_owned()]);
    let plan = plan_cook(&uses, resolve, exists, &delit);
    assert_eq!(plan.jobs.keys().collect::<Vec<_>>(), ["textures/b.dds"]);
    assert_eq!((plan.delit, plan.missing), (1, 1));
    assert_eq!(plan.skipped, BTreeMap::from([(CookSkip::Ambiguous, 1), (CookSkip::SkyUi, 2)]));
}

#[test]
fn candidate_cutoff_follows_the_alpha() {
    let job = CookJob { alpha: CookAlpha::Test(0.5), candidate: true, normal: None };
    let holes = png_with(16, 16, 0, |x, _| if x < 4 { 0 } else { 255 });
    let soft = png_with(16, 16, 0, |_, _| 128);
    assert_eq!(cook_texture("t/a.png", &holes, &job, Bc7Quality::Fast).1.alpha, CookAlpha::Test(0.5));
    assert_eq!(cook_texture("t/a.png", &soft, &job, Bc7Quality::Fast).1.alpha, CookAlpha::Blend);
    let fixed = CookJob { candidate: false, ..job };
    assert_eq!(cook_texture("t/a.png", &soft, &fixed, Bc7Quality::Fast).1.alpha, CookAlpha::Test(0.5));
}

/// Content-hashed inputs over a VFS (the bake's `VfsInputs` hashes the winning file too).
struct TestInputs(Arc<Vfs>);

impl Inputs for TestInputs {
    fn file_fingerprint(&self, path: &str) -> Result<Option<Digest>, BakeError> {
        let Ok(path) = VfsPath::new(path) else { return Ok(None) };
        Ok(self.0.read(&path).ok().map(|b| Digest::of(&b)))
    }
    fn read_file(&self, path: &str) -> Result<Vec<u8>, BakeError> {
        let vpath = VfsPath::new(path).map_err(|e| BakeError::Other(e.to_string()))?;
        self.0.read(&vpath).map_err(|_| BakeError::MissingInput(Input::File(path.to_owned())))
    }
    fn record_fingerprint(&self, _: &str, _: &str) -> Result<Option<Digest>, BakeError> {
        Ok(None)
    }
}

/// Cooks the opaque albedo references `refs` over `vfs`; returns the `texture.cook` builds and the index.
fn bake(store: &Store, vfs: &Arc<Vfs>, refs: &[&str]) -> (u32, CookIndex) {
    bake_with(store, vfs, refs, &TextureSources::default())
}

/// [`bake`] with `sources`' pack profiles.
fn bake_with(store: &Store, vfs: &Arc<Vfs>, refs: &[&str], sources: &TextureSources) -> (u32, CookIndex) {
    let uses: Vec<TextureUse> = refs.iter().map(|r| use_of(r, Role::Albedo, None, false)).collect();
    let report = DelightReport { uses, ..DelightReport::default() };
    let mut baker = Baker::new(store.clone(), Arc::new(TestInputs(vfs.clone())));
    let cooked = bake_cook(&mut baker, vfs, &report, sources);
    assert!(cooked.failures.is_empty(), "{:?}", cooked.failures);
    (cooked.built, load_index(store, &baker.manifest()).unwrap())
}

fn data() -> MemSource {
    MemSource::new("data")
        .with("textures/a.png", png(16, 16, 0))
        .with("textures/b.png", png(16, 16, 9))
        .with("textures/c.dds", dds(D3DFormat::DXT1, 16, 16, 5))
}

const REFS: [&str; 3] = ["textures/a.tga", "textures/b.tga", "textures/c.tga"];

#[test]
fn changed_winner_recooks_only_that_texture() {
    let dir = tempfile::tempdir().unwrap();
    let store = Store::open(dir.path()).unwrap();
    let base = Arc::new(Vfs::builder().mount(data()).build());
    let (built, index) = bake(&store, &base, &REFS);
    assert_eq!((built, index.entries.len()), (3, 3));
    assert_eq!(index.entries[2].object, None, "a full-chain DXT1 is indexed in place");
    // Same input: nothing re-cooks, same index.
    let (built, again) = bake(&store, &base, &REFS);
    assert_eq!((built, &again), (0, &index));
    // A mod replaces one winner: only it re-cooks; the other entries are unchanged.
    let modded = Arc::new(Vfs::builder().mount(data()).mount(MemSource::new("mod").with("textures/b.png", png(16, 16, 3))).build());
    let (built, changed) = bake(&store, &modded, &REFS);
    assert_eq!(built, 1);
    assert_eq!((&changed.entries[0], &changed.entries[2]), (&index.entries[0], &index.entries[2]));
    assert_ne!(changed.entries[1].fingerprint, index.entries[1].fingerprint);
    assert_ne!(changed.entries[1].object, index.entries[1].object);
}

#[test]
fn layer_serves_current_entries_only() {
    let dir = tempfile::tempdir().unwrap();
    let store = Store::open(dir.path()).unwrap();
    let base = Arc::new(Vfs::builder().mount(data()).build());
    let (_, index) = bake(&store, &base, &REFS);
    // A de-light entry for `a` (any stored object): de-light wins the stem.
    let delit_object = store.put(b"delit ktx2", false).unwrap();
    let a_fingerprint = Digest::of(&png(16, 16, 0));
    let delight = DelightIndex {
        entries: vec![IndexEntry { source: "textures/a.png".into(), fingerprint: a_fingerprint, ktx2: delit_object }],
        skipped: BTreeMap::new(),
    };
    let mount = |vfs: &Arc<Vfs>, delight: Option<&DelightIndex>| {
        let (layer, stats) = CookedTextureLayer::build(store.clone(), delight, Some(&index), vfs, &TestInputs(vfs.clone()));
        (vfs.with_top_layer(Arc::new(layer)), stats)
    };
    let read = |vfs: &Vfs, p: &str| vfs.read(&VfsPath::new(p).unwrap()).unwrap();
    let resolve = |vfs: &Vfs, r: &str| vfs.resolve_preferring(&VfsPath::new(r).unwrap(), TEXTURE_ALTERNATES).unwrap().0;

    let (vfs, stats) = mount(&base, Some(&delight));
    assert_eq!(stats, LayerStats { delit: 1, cooked: 1, in_place: 1, stale: 0, normals: 0, orms: 0, depths: 0 }, "a's cook entry is shadowed by de-light");
    assert_eq!(read(&vfs, "textures/a.ktx2"), b"delit ktx2");
    // The cooked KTX2 is served without its meta trailer.
    let b = read(&vfs, "textures/b.ktx2");
    let product = store.get(&index.entries[1].object.unwrap()).unwrap();
    assert_eq!(&b[..6], b"\xABKTX 2");
    assert_eq!(b.len(), split_product(&product).unwrap().0);
    assert_eq!(resolve(&vfs, "textures/c.tga").as_str(), "textures/c.dds", "in place: the shipped DDS loads");
    assert_eq!(layer_index(&vfs), Some(SourceIndex(1)));

    // Without de-light, the cook serves `a` too.
    let (vfs, stats) = mount(&base, None);
    assert_eq!(stats, LayerStats { delit: 0, cooked: 2, in_place: 1, stale: 0, normals: 0, orms: 0, depths: 0 });
    assert_eq!(&read(&vfs, "textures/a.ktx2")[..6], b"\xABKTX 2");

    // A mod's newer `b` or another variant of it: the entry is stale and the mod's file loads.
    for (file, bytes) in [("textures/b.png", png(16, 16, 3)), ("textures/b.dds", dds(D3DFormat::DXT1, 16, 16, 5))] {
        let modded = Arc::new(Vfs::builder().mount(data()).mount(MemSource::new("mod").with(file, bytes.clone())).build());
        let (vfs, stats) = mount(&modded, None);
        assert_eq!(stats, LayerStats { delit: 0, cooked: 1, in_place: 1, stale: 1, normals: 0, orms: 0, depths: 0 }, "{file}");
        assert_eq!(resolve(&vfs, "textures/b.tga").as_str(), file);
        assert_eq!(read(&vfs, file), bytes, "{file}");
    }
}

#[test]
fn stats_line() {
    let meta = |path, alpha, format: &str, level_bytes: Vec<u64>, shipped_vram| CookMeta {
        path,
        alpha,
        format: format.into(),
        width: 4,
        height: 4,
        level_bytes,
        shipped_vram,
        normal: false,
        orm: false,
        level0_rebuilt: false,
    };
    let entry = |source: &str, object, meta| CookEntry { source: source.into(), fingerprint: Digest::of(source.as_bytes()), object, meta };
    let mib = 1024 * 1024;
    let index = CookIndex {
        entries: vec![
            entry("a.dds", None, meta(CookPath::InPlace { dds_payload: Some(128) }, CookAlpha::Opaque, "bc1-srgb", vec![mib], mib)),
            entry(
                "b.dds",
                Some(Digest::of(b"b")),
                meta(CookPath::Rewrapped { filled: 2, remapped: false }, CookAlpha::Test(0.5), "bc3-srgb", vec![mib, mib], mib),
            ),
            entry("c.tga", Some(Digest::of(b"c")), meta(CookPath::Encoded, CookAlpha::Blend, "bc7-srgb", vec![mib], 4 * mib)),
        ],
        skipped: BTreeMap::from([(CookSkip::Unaligned, 2)]),
        normals: BTreeMap::new(),
        orms: BTreeMap::new(),
        ..CookIndex::default()
    };
    let mut report = CookReport { delit: 5, missing: 1, skipped: BTreeMap::from([(CookSkip::SkyUi, 3)]), built: 2, ..CookReport::default() };
    report.count(&index);
    assert_eq!(
        report.to_string(),
        "texture.cook: 3 textures (in place 1, rewrapped 1 [filled 1, remapped 0], encoded 1), 5 left to de-light, 1 missing; 0 normal companions (0 with depth); \
         0 ORM companions (0 from specular maps, 0 level 0 rebuilt); \
         by alpha {blend 1, cutout 1, opaque 1}; by format {bc1-srgb 1, bc3-srgb 1, bc7-srgb 1}; VRAM as shipped 6.0 MiB -> \
         cooked 4.0 MiB, served from the bake 3.0 MiB; skipped by reason {sky/ui 3, unaligned 2}; 2 built, 0.0 CPU-s, \
         0 failed, 0.0 s\n"
    );
}

#[test]
fn product_trailer_round_trips() {
    let meta = CookMeta::skipped(CookSkip::Unsupported, CookAlpha::Blend);
    let bytes = product(b"ktx2 bytes".to_vec(), &meta).unwrap();
    assert_eq!(split_product(&bytes), Some((10, meta)));
    assert_eq!(split_product(b"xy"), None);
}

/// A `w` x `h` PNG of one tangent-space normal `n` (unit vector), encoded `(n + 1) / 2`.
fn normal_png(w: u32, h: u32, n: [f32; 3]) -> Vec<u8> {
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    let texel = n.map(|c| (f32::midpoint(c, 1.0) * 255.0).round() as u8);
    let image = image::RgbaImage::from_fn(w, h, |_, _| image::Rgba([texel[0], texel[1], texel[2], 255]));
    let mut bytes = std::io::Cursor::new(Vec::new());
    image.write_to(&mut bytes, image::ImageFormat::Png).unwrap();
    bytes.into_inner()
}

/// The KTX2 `vkFormat` field (header bytes 12..16).
fn vk_format(ktx2: &[u8]) -> u32 {
    u32::from_le_bytes(ktx2[12..16].try_into().unwrap())
}

/// T4: an uncompressed normal companion cooks to BC5 UNORM (linear, never sRGB) with green
/// complemented (the default profile's DirectX `source_y: Down` → Bevy's +Y); a BC1 DDS one is
/// rewrapped block-exactly through the same Y flip and stays BC1 UNORM.
#[test]
fn normal_companions_cook_linear_with_the_profiles_y_flip() {
    let profile = crate::texture_profiles::default_profile();
    let rule = &profile.suffixes.iter().find(|r| r.suffix == "_n").unwrap();
    let job = CookJob::normal(rule.layout, profile.normal_y);
    assert_eq!(profile.normal_y, SourceY::Down);

    // Y points down the image in the source (-0.6): +0.6 after the cook.
    let (ktx2, meta) = cook_texture("textures/a_n.png", &normal_png(16, 16, [0.0, -0.6, 0.8]), &job, Bc7Quality::Fast);
    assert_eq!((meta.path, meta.format.as_str(), meta.normal), (CookPath::Encoded, "bc5", true));
    assert_eq!(vk_format(&ktx2), 141, "VK_FORMAT_BC5_UNORM_BLOCK");
    // The levels the encode path writes, level 0 decoded (the cook's own call, so no KTX2 parsing).
    let rgba = decode_rgba("a.png", &normal_png(16, 16, [0.0, -0.6, 0.8])).unwrap().rgba;
    let levels = cook_levels(TexRole::Normal { source_y: SourceY::Down }, &rgba, 16, 16, Bc7Quality::Fast);
    let texels = sf_texture::decode_level(BlockFormat::Bc5, &levels[0], 16, 16);
    let expected = (f32::midpoint(0.6f32, 1.0) * 255.0).round();
    for t in texels.as_chunks::<4>().0 {
        assert!((f32::from(t[1]) - expected).abs() <= 4.0, "G {} (expected about {expected})", t[1]);
        assert!((f32::from(t[0]) - 128.0).abs() <= 4.0, "R {}", t[0]);
    }

    // A full-chain BC1 DDS: rewrapped through the exact green complement, linear BC1.
    let (ktx2, meta) = cook_texture("textures/b_n.dds", &dds(D3DFormat::DXT1, 16, 16, 5), &job, Bc7Quality::Fast);
    assert_eq!((meta.path, meta.format.as_str(), meta.normal), (CookPath::Rewrapped { filled: 0, remapped: true }, "bc1", true));
    assert_eq!(vk_format(&ktx2), 133, "VK_FORMAT_BC1_RGB_UNORM_BLOCK, not the sRGB 132+ variants");

    // A colour job on the same file is untouched by any of this.
    let (_, meta) = cook_texture("textures/b.dds", &dds(D3DFormat::DXT1, 16, 16, 5), &OPAQUE, Bc7Quality::Fast);
    assert!(!meta.normal);
    assert_eq!(meta.format, "bc1-srgb");
}

/// T4 companions: `_nh` before `_n` before `_nm` (the profile's order), any extension through the
/// runtime's alternates, base-colour uses only, an ambiguous companion skipped.
#[test]
fn plan_normals_finds_companions_in_profile_order() {
    let files = [
        "textures/a.dds",
        "textures/a_n.dds",
        "textures/a_nh.png",
        "textures/b.dds",
        "textures/b_nm.tga",
        "textures/c.dds",
        "textures/c_n.dds",
        "textures/c_n.tga",
        "textures/g.dds",
        "textures/g_n.dds",
        "textures/plain.dds",
    ];
    let exists = |p: &str| files.contains(&p);
    let resolve = |r: &str| {
        let s = stem(r);
        ["ktx2", "dds", "png", "tga"].iter().map(|e| format!("{s}.{e}")).find(|p| exists(p)).map(|p| (p, SourceIndex(0)))
    };
    let mut uses: Vec<TextureUse> =
        ["a", "b", "c", "plain"].iter().map(|n| use_of(&format!("textures/{n}.tga"), Role::Albedo, None, false)).collect();
    uses.push(use_of("textures/g.dds", Role::Glow, None, false));
    let mut plan = CookPlan::default();
    plan_normals(&mut plan, &uses, &TextureSources::default(), resolve, exists);
    let normals: Vec<(&str, &str)> = plan.normals.iter().map(|(b, c)| (b.as_str(), c.as_str())).collect();
    assert_eq!(normals, [("textures/a", "textures/a_nh.png"), ("textures/b", "textures/b_nm.tga"), ("textures/c", "textures/c_n.dds")]);
    assert_eq!(plan.jobs.keys().collect::<Vec<_>>(), ["textures/a_nh.png", "textures/b_nm.tga"]);
    assert!(plan.jobs.values().all(|j| j.normal.is_some() && j.alpha == CookAlpha::Opaque));
    assert_eq!(plan.skipped, BTreeMap::from([(CookSkip::Ambiguous, 1)]));
}

/// T4 end to end: the index publishes the cooked companions and the layer serves the current ones
/// ([`read_companions`]); a stale companion is withheld (it would load with the wrong Y).
#[test]
fn layer_publishes_current_normal_companions() {
    let dir = tempfile::tempdir().unwrap();
    let store = Store::open(dir.path()).unwrap();
    let pack = || MemSource::new("pack").with("textures/a_n.png", normal_png(16, 16, [0.0, 0.0, 1.0]));
    let base = Arc::new(Vfs::builder().mount(data()).mount(pack()).build());
    let (_, index) = bake(&store, &base, &REFS);
    assert_eq!(index.normals, BTreeMap::from([("textures/a".to_owned(), "textures/a_n.png".to_owned())]));
    let entry = index.entries.iter().find(|e| e.source == "textures/a_n.png").unwrap();
    assert!(entry.meta.normal && entry.meta.format == "bc5");

    let mount = |vfs: &Arc<Vfs>| {
        let (layer, stats) = CookedTextureLayer::build(store.clone(), None, Some(&index), vfs, &TestInputs(vfs.clone()));
        (vfs.with_top_layer(Arc::new(layer)), stats)
    };
    let (vfs, stats) = mount(&base);
    assert_eq!(stats.normals, 1);
    assert_eq!(read_companions(&vfs), index.normals);
    let served = vfs.resolve_preferring(&VfsPath::new("textures/a_n.png").unwrap(), TEXTURE_ALTERNATES).unwrap().0;
    assert_eq!(served.as_str(), "textures/a_n.ktx2", "the cooked companion loads");
    // Without a layer (no bake) there is nothing to read.
    assert!(read_companions(&base).is_empty());

    // The pack's companion changed since the bake: withheld until the next bake.
    let changed = MemSource::new("pack").with("textures/a_n.png", normal_png(16, 16, [0.6, 0.0, 0.8]));
    let modded = Arc::new(Vfs::builder().mount(data()).mount(changed).build());
    let (vfs, stats) = mount(&modded);
    assert_eq!(stats.normals, 0);
    assert!(read_companions(&vfs).is_empty());
}

/// `pack` (VFS source 1) with `section`; the base data is source 0.
fn pack_sources(section: &TextureSection) -> TextureSources {
    TextureSources::from_sections([(1..2, "pack", section)])
}

/// [`plan_normals`] over `vfs` for albedo references `refs`.
fn plan_over(vfs: &Vfs, refs: &[&str], sources: &TextureSources) -> CookPlan {
    let resolve = |r: &str| {
        vfs.resolve_preferring(&VfsPath::new(r).ok()?, TEXTURE_ALTERNATES).map(|(p, s)| (p.as_str().to_owned(), s))
    };
    let exists = |p: &str| VfsPath::new(p).is_ok_and(|p| vfs.contains(&p));
    let uses: Vec<TextureUse> = refs.iter().map(|r| use_of(r, Role::Albedo, None, false)).collect();
    let mut plan = CookPlan::default();
    plan_normals(&mut plan, &uses, sources, resolve, exists);
    plan
}

/// textures-adapter.md §8: `normal_y = "up"` in one mod's section cooks that mod's companions
/// without the green flip; the base data's keep the default profile's.
#[test]
fn a_normal_y_override_applies_to_its_mod_only() {
    let data = MemSource::new("data").with("textures/a.dds", "a").with("textures/a_n.dds", "n").with("textures/b.dds", "b");
    let pack = MemSource::new("pack").with("textures/b_n.dds", "n");
    let vfs = Vfs::builder().mount(data).mount(pack).build();
    let up = TextureSection { normal_y: Some(NormalY::Up), ..TextureSection::default() };
    let plan = plan_over(&vfs, &["textures/a.tga", "textures/b.tga"], &pack_sources(&up));
    let y = |file: &str| plan.jobs[file].normal.map(|n| n.source_y);
    assert_eq!((y("textures/a_n.dds"), y("textures/b_n.dds")), (Some(SourceY::Down), Some(SourceY::Up)));
    // The companion's source decides, not the albedo's: b's albedo is the base data's.
    assert_eq!(vfs.resolve(&VfsPath::new("textures/b.dds").unwrap()), Some(SourceIndex(0)));
}

/// A renamed suffix (`_normal` = `_n`) is a companion only in the mod that declares it.
#[test]
fn a_renamed_suffix_is_found_only_in_its_mod() {
    let data = MemSource::new("data").with("textures/a.dds", "a").with("textures/a_normal.dds", "n").with("textures/b.dds", "b");
    let pack = MemSource::new("pack").with("textures/b_normal.dds", "n");
    let vfs = Vfs::builder().mount(data).mount(pack).build();
    let refs = ["textures/a.tga", "textures/b.tga"];
    let renamed = TextureSection { suffixes: BTreeMap::from([("_normal".into(), "_n".into())]), ..TextureSection::default() };
    let plan = plan_over(&vfs, &refs, &pack_sources(&renamed));
    assert_eq!(plan.normals, BTreeMap::from([("textures/b".to_owned(), "textures/b_normal.dds".to_owned())]));
    assert_eq!(plan.jobs["textures/b_normal.dds"].normal.map(|n| n.source_y), Some(SourceY::Down));
    // Without the section neither is a companion.
    assert!(plan_over(&vfs, &refs, &TextureSources::default()).normals.is_empty());
    // `mesh.cook` asks the union of suffixes: a new suffix changes the tangent rule's key, the
    // default union keeps version 1 (no re-cook for installs that name no profile).
    assert_eq!(tangent_rule(&TextureSources::default()).version, 1);
    assert_eq!(tangent_rule(&pack_sources(&TextureSection { normal_y: Some(NormalY::Up), ..TextureSection::default() })).version, 1);
    assert!(tangent_rule(&pack_sources(&renamed)).version > 1);
}

/// Changing one mod's profile re-cooks only that mod's companions (the job, in the cook's `role`
/// param, carries the profile's layout and Y); the base data's stay cached.
#[test]
fn changing_a_mods_profile_recooks_only_its_textures() {
    let dir = tempfile::tempdir().unwrap();
    let store = Store::open(dir.path()).unwrap();
    let base = data().with("textures/a_n.png", normal_png(16, 16, [0.0, -0.6, 0.8]));
    let pack = MemSource::new("pack").with("textures/b_n.png", normal_png(16, 16, [0.0, 0.6, 0.8]));
    let vfs = Arc::new(Vfs::builder().mount(base).mount(pack).build());
    let (_, before) = bake(&store, &vfs, &REFS);
    let (built, again) = bake(&store, &vfs, &REFS);
    assert_eq!((built, &again), (0, &before));

    let up = TextureSection { normal_y: Some(NormalY::Up), ..TextureSection::default() };
    let (built, after) = bake_with(&store, &vfs, &REFS, &pack_sources(&up));
    assert_eq!(built, 1, "only the pack's companion re-cooks");
    let entry = |index: &CookIndex, source: &str| index.entries.iter().find(|e| e.source == source).cloned().unwrap();
    assert_eq!(entry(&after, "textures/a_n.png"), entry(&before, "textures/a_n.png"));
    assert_ne!(entry(&after, "textures/b_n.png").object, entry(&before, "textures/b_n.png").object);
    for albedo in ["textures/a.png", "textures/b.png", "textures/c.dds"] {
        assert_eq!(entry(&after, albedo), entry(&before, albedo));
    }
    // Back to the default: again only the pack's companion re-cooks (one trace per key), to the
    // same product as the first time.
    let (built, back) = bake(&store, &vfs, &REFS);
    assert_eq!((built, &back), (1, &before));
}

/// The wareya-pbr profile for VFS source 1 (`pack_sources`).
fn pbr_pack() -> TextureSources {
    pack_sources(&TextureSection { profile: Some("wareya-pbr".into()), ..TextureSection::default() })
}

/// Shipped metal R ↔ canonical metal B (the wareya-pbr `_spec` layout).
fn rb_swap() -> ChannelMap {
    use sf_texture::Src;
    ChannelMap([Src::Channel(Channel::B), Src::Channel(Channel::G), Src::Channel(Channel::R), Src::Unused])
}

/// A 16x16 DXT1 DDS whose level-0 blocks have no red (shipped metal) and whose levels 1+ have
/// full red, the rest of the colour arbitrary: the pack defect of textures-adapter.md §7.
fn metal_in_mips_only_dds() -> Vec<u8> {
    let block = |c0: u16, c1: u16| [c0.to_le_bytes(), c1.to_le_bytes(), [0xE4, 0xE4], [0xE4, 0xE4]].concat();
    let rgb = |r: u16, g: u16, b: u16| (r << 11) | (g << 5) | b;
    let level0 = block(rgb(0, 40, 10), rgb(0, 5, 3)).repeat(16);
    let mips = block(rgb(31, 40, 10), rgb(31, 5, 3)).repeat(4 + 1 + 1 + 1);
    let params = NewD3dParams { height: 16, width: 16, depth: None, format: D3DFormat::DXT1, mipmap_levels: Some(5), caps2: None };
    let mut dds = Dds::new_d3d(params).unwrap();
    dds.data = [level0, mips].concat();
    let mut out = Vec::new();
    dds.write(&mut out).unwrap();
    out
}

/// TS-2: `_spec` follows its own source's profile. A wareya-pbr source's is a packed ORM (R↔B
/// layout, occlusion bound, no repair unless the profile lists the quirk); the base data's
/// (`openmw`) is a legacy specular map (converted, no occlusion). ORM jobs leave the colour jobs.
#[test]
fn plan_orms_reads_each_spec_with_its_own_sources_profile() {
    let data = MemSource::new("data").with("textures/a.dds", "a").with("textures/a_spec.dds", "s").with("textures/b.dds", "b");
    let pack = MemSource::new("pack").with("textures/b_spec.dds", "s");
    let vfs = Vfs::builder().mount(data).mount(pack).build();
    let refs = ["textures/a.tga", "textures/b.tga"];
    let mut plan = plan_over(&vfs, &refs, &pbr_pack());
    plan.jobs.insert("textures/a_spec.dds".into(), OPAQUE); // also referenced as a colour texture
    let resolve = |r: &str| vfs.resolve_preferring(&VfsPath::new(r).ok()?, TEXTURE_ALTERNATES).map(|(p, s)| (p.as_str().to_owned(), s));
    let exists = |p: &str| VfsPath::new(p).is_ok_and(|p| vfs.contains(&p));
    let uses: Vec<TextureUse> = refs.iter().map(|r| use_of(r, Role::Albedo, None, false)).collect();
    plan_orms(&mut plan, &uses, &pbr_pack(), resolve, exists);
    assert_eq!(
        plan.orm_jobs,
        BTreeMap::from([
            ("textures/a_spec.dds".to_owned(), OrmJob { layout: ChannelMap::IDENTITY, specular: true, fill_level0: None }),
            ("textures/b_spec.dds".to_owned(), OrmJob { layout: rb_swap(), specular: false, fill_level0: None }),
        ])
    );
    assert_eq!(
        plan.orms,
        BTreeMap::from([
            ("textures/a".to_owned(), OrmCompanion { file: "textures/a_spec.dds".into(), occlusion: false }),
            ("textures/b".to_owned(), OrmCompanion { file: "textures/b_spec.dds".into(), occlusion: true }),
        ])
    );
    assert!(!plan.jobs.contains_key("textures/a_spec.dds") && !plan.jobs.contains_key("textures/b_spec.dds"));
    assert!(plan.normals.is_empty());
    // The repair is data: off in the shipped profile, on when the profile lists the quirk.
    let quirks: Vec<sf_texture::Quirk> = ron::from_str("[FillEmptyLevel0(role: Orm, channel: B)]").unwrap();
    assert_eq!(quirks, [sf_texture::Quirk::FillEmptyLevel0 { role: PackRole::Orm, channel: Channel::B }]);
}

/// TS-2 end to end: a wareya-pbr `_spec` (DXT1) cooks to linear BC1 through the block-exact R↔B
/// swap, a vanilla one (PNG) is converted and encoded linear BC1; the index publishes both and the
/// layer serves them ([`read_orms`]), a stale one withheld.
#[test]
fn orm_companions_cook_linear_and_the_layer_publishes_them() {
    let dir = tempfile::tempdir().unwrap();
    let store = Store::open(dir.path()).unwrap();
    let spec = png_with(16, 16, 40, |x, _| u8::try_from(x * 16).unwrap());
    let base = data().with("textures/a_spec.png", spec);
    let pack = MemSource::new("pack").with("textures/b_spec.dds", metal_in_mips_only_dds());
    let vfs = Arc::new(Vfs::builder().mount(base).mount(pack).build());
    let (_, index) = bake_with(&store, &vfs, &REFS, &pbr_pack());
    assert_eq!(
        index.orms,
        BTreeMap::from([
            ("textures/a".to_owned(), OrmCompanion { file: "textures/a_spec.png".into(), occlusion: false }),
            ("textures/b".to_owned(), OrmCompanion { file: "textures/b_spec.dds".into(), occlusion: true }),
        ])
    );
    let meta = |source: &str| index.entries.iter().find(|e| e.source == source).unwrap().meta.clone();
    let (a, b) = (meta("textures/a_spec.png"), meta("textures/b_spec.dds"));
    assert_eq!((a.path, a.format.as_str(), a.orm, a.normal), (CookPath::Encoded, "bc1", true, false));
    assert_eq!(
        (b.path, b.format.as_str(), b.orm, b.level0_rebuilt),
        (CookPath::Rewrapped { filled: 0, remapped: true }, "bc1", true, false)
    );

    let (layer, stats) = CookedTextureLayer::build(store.clone(), None, Some(&index), &vfs, &TestInputs(vfs.clone()));
    let mounted = vfs.with_top_layer(Arc::new(layer));
    assert_eq!(stats.orms, 2);
    assert_eq!(read_orms(&mounted), index.orms);
    assert_eq!(orm_of(&index.orms, "Textures\\B.dds").map(|c| c.file.as_str()), Some("textures/b_spec.dds"));
    let served = mounted.read(&VfsPath::new("textures/b_spec.ktx2").unwrap()).unwrap();
    assert_eq!(vk_format(&served), 133, "VK_FORMAT_BC1_RGB_UNORM_BLOCK: linear");
    assert!(read_orms(&vfs).is_empty(), "no layer, no ORMs");

    // The pack's `_spec` changed since the bake: withheld until the next bake.
    let changed = MemSource::new("pack").with("textures/b_spec.dds", dds(D3DFormat::DXT1, 16, 16, 5));
    let modded = Arc::new(Vfs::builder().mount(data().with("textures/a_spec.png", png(16, 16, 1))).mount(changed).build());
    let (_, stats) = CookedTextureLayer::build(store.clone(), None, Some(&index), &modded, &TestInputs(modded.clone()));
    assert_eq!(stats.orms, 0);
}

/// Owner question 7's repair: only with the profile's quirk, only when level 0 has no metal and
/// level 1 has some; level 0's metal (canonical B) then decodes as level 1's.
#[test]
fn level0_metal_is_rebuilt_only_under_the_quirk() {
    let bytes = metal_in_mips_only_dds();
    let level0_metal = |ktx2: &[u8]| {
        let source = read_source(ktx2).unwrap().unwrap();
        let texels = sf_texture::decode_level(source.format, source.levels[0], 16, 16);
        texels.as_chunks::<4>().0.iter().map(|t| t[2]).collect::<BTreeSet<u8>>()
    };
    let off = OrmJob { layout: rb_swap(), specular: false, fill_level0: None };
    let (ktx2, meta) = cook_orm("textures/b_spec.dds", &bytes, &off, Bc7Quality::Fast);
    assert!(!meta.level0_rebuilt);
    assert_eq!(level0_metal(&ktx2), BTreeSet::from([0]), "as shipped: no metal up close");

    let on = OrmJob { fill_level0: Some(Channel::B), ..off };
    let (ktx2, meta) = cook_orm("textures/b_spec.dds", &bytes, &on, Bc7Quality::Fast);
    assert!(meta.level0_rebuilt);
    assert_eq!(level0_metal(&ktx2), BTreeSet::from([255]));
    // Level 1 without metal either: left alone even with the quirk on.
    let (_, meta) = cook_orm("textures/c_spec.dds", &dds(D3DFormat::DXT1, 16, 16, 5), &on, Bc7Quality::Fast);
    assert!(!meta.level0_rebuilt);
}

/// A DXT5 DDS whose blocks are pseudo-random bytes (every alpha mode and index pattern).
fn noise_dxt5(w: u32, h: u32, levels: u32, seed: u32) -> Vec<u8> {
    let params = NewD3dParams { height: h, width: w, depth: None, format: D3DFormat::DXT5, mipmap_levels: Some(levels), caps2: None };
    let mut dds = Dds::new_d3d(params).unwrap();
    let mut x = seed.wrapping_mul(2_654_435_761) | 1;
    for b in &mut dds.data {
        x ^= x << 13;
        x ^= x >> 17;
        x ^= x << 5;
        *b = x.to_le_bytes()[3];
    }
    let mut out = Vec::new();
    dds.write(&mut out).unwrap();
    out
}

fn depth_job() -> DepthJob {
    let profile = crate::texture_profiles::default_profile();
    let rule = profile.suffixes.iter().find(|r| r.suffix == "_nh").unwrap();
    assert_eq!(rule.role, PackRole::NormalHeight);
    DepthJob { layout: rule.layout }
}

/// tx-pom: a DXT5 `_nh` splits block-exactly: every level of the depth map is BC4 UNORM (linear)
/// and decodes as `255 - height`, the source's decoded alpha (within the in-tree decoder's 1/255
/// interpolant rounding, as `sf_texture`'s split test). An uncompressed one is decoded and cooked.
#[test]
fn dxt5_height_splits_into_a_complemented_bc4_depth_map() {
    let bytes = noise_dxt5(16, 16, 5, 7);
    let (ktx2, meta) = cook_depth("textures/a_nh.dds", &bytes, &depth_job(), Bc7Quality::Fast);
    assert_eq!(
        (meta.path, meta.format.as_str(), meta.normal, meta.orm),
        (CookPath::Rewrapped { filled: 0, remapped: true }, "bc4", false, false)
    );
    assert_eq!((meta.width, meta.height, meta.shipped_vram), (16, 16, 0));
    assert_eq!(vk_format(&ktx2), 139, "VK_FORMAT_BC4_UNORM_BLOCK: linear");
    let source = read_source(&bytes).unwrap().unwrap();
    let depth = read_source(&ktx2).unwrap().unwrap();
    assert_eq!(depth.levels.len(), 5);
    for (level, (s, d)) in source.levels.iter().zip(&depth.levels).enumerate() {
        let size = 16u32 >> level;
        let height = sf_texture::decode_level(BlockFormat::Bc3, s, size, size);
        let got = sf_texture::decode_level(BlockFormat::Bc4, d, size, size);
        for (h, g) in height.as_chunks::<4>().0.iter().zip(got.as_chunks::<4>().0) {
            assert!((255 - h[3]).abs_diff(g[0]) <= 1, "level {level}: height {} -> depth {}", h[3], g[0]);
        }
    }
    // An uncompressed `_nh` is decoded and its height cooked to BC4: flat height 200 -> depth 55.
    let png = png_with(16, 16, 3, |_, _| 200);
    let (ktx2, meta) = cook_depth("textures/b_nh.png", &png, &depth_job(), Bc7Quality::Fast);
    assert_eq!((meta.path, meta.format.as_str()), (CookPath::Encoded, "bc4"));
    let level0 = read_source(&ktx2).unwrap().unwrap().levels[0].to_vec();
    assert!(sf_texture::decode_level(BlockFormat::Bc4, &level0, 16, 16).as_chunks::<4>().0.iter().all(|t| t[0] == 55));
}

/// tx-pom: a depth map larger than [`DEPTH_MAX_SIZE`] drops its top levels: a 2048² `_nh` gives the
/// depth map a 1024² source of its levels 1.. would.
#[test]
fn depth_maps_are_capped_by_dropping_top_levels() {
    let big = noise_dxt5(2048, 2048, 12, 3);
    let (ktx2, meta) = cook_depth("textures/a_nh.dds", &big, &depth_job(), Bc7Quality::Fast);
    assert_eq!((meta.width, meta.height, meta.level_bytes.len()), (DEPTH_MAX_SIZE, DEPTH_MAX_SIZE, 11));
    let source = read_source(&big).unwrap().unwrap();
    let params = NewD3dParams { height: 1024, width: 1024, depth: None, format: D3DFormat::DXT5, mipmap_levels: Some(11), caps2: None };
    let mut half = Dds::new_d3d(params).unwrap();
    half.data = source.levels[1..].concat();
    let mut half_bytes = Vec::new();
    half.write(&mut half_bytes).unwrap();
    let (want, _) = cook_depth("textures/a_nh.dds", &half_bytes, &depth_job(), Bc7Quality::Fast);
    assert_eq!(read_source(&ktx2).unwrap().unwrap().levels, read_source(&want).unwrap().unwrap().levels);
}

/// tx-pom: a depth job only for a winning companion its own source's profile reads as a
/// height-normal: the default `_nh` yes, `_n` no, and a pack's renamed `_bump` (= `_nh`) in that
/// pack only.
#[test]
fn depth_jobs_follow_the_companions_profile() {
    let data = MemSource::new("data")
        .with("textures/a.dds", "a")
        .with("textures/a_nh.dds", "nh")
        .with("textures/b.dds", "b")
        .with("textures/b_n.dds", "n")
        .with("textures/c.dds", "c")
        .with("textures/c_bump.dds", "bump");
    let pack = MemSource::new("pack").with("textures/d.dds", "d").with("textures/d_bump.dds", "bump");
    let vfs = Vfs::builder().mount(data).mount(pack).build();
    let refs = ["textures/a.tga", "textures/b.tga", "textures/c.tga", "textures/d.tga"];
    let renamed = TextureSection { suffixes: BTreeMap::from([("_bump".into(), "_nh".into())]), ..TextureSection::default() };
    let plan = plan_over(&vfs, &refs, &pack_sources(&renamed));
    assert_eq!(plan.depth_jobs.keys().collect::<Vec<_>>(), ["textures/a_nh.dds", "textures/d_bump.dds"]);
    assert_eq!(
        plan.depths,
        BTreeMap::from([
            ("textures/a".to_owned(), "textures/a_nh_depth.ktx2".to_owned()),
            ("textures/d".to_owned(), "textures/d_bump_depth.ktx2".to_owned()),
        ])
    );
    assert!(plan.jobs.contains_key("textures/a_nh.dds") && plan.jobs.contains_key("textures/b_n.dds"), "normals as before");
    assert!(plan_over(&vfs, &refs, &TextureSources::default()).depths.keys().eq(["textures/a"]));
}

/// tx-pom end to end: the index publishes the depth maps the cook made, the layer serves the
/// current ones at their own path beside the normal ([`read_depths`], [`depth_of`]) and withholds a
/// stale one; the stats line counts them.
#[test]
fn layer_publishes_current_depth_companions() {
    let dir = tempfile::tempdir().unwrap();
    let store = Store::open(dir.path()).unwrap();
    let pack = |seed| MemSource::new("pack").with("textures/a_nh.dds", noise_dxt5(16, 16, 5, seed));
    let base = Arc::new(Vfs::builder().mount(data()).mount(pack(1)).build());
    let uses: Vec<TextureUse> = REFS.iter().map(|r| use_of(r, Role::Albedo, None, false)).collect();
    let report = DelightReport { uses, ..DelightReport::default() };
    let mut baker = Baker::new(store.clone(), Arc::new(TestInputs(base.clone())));
    let cooked = bake_cook(&mut baker, &base, &report, &TextureSources::default());
    assert!(cooked.failures.is_empty(), "{:?}", cooked.failures);
    assert_eq!(cooked.depths, 1);
    assert!(cooked.to_string().contains("1 normal companions (1 with depth)"), "{cooked}");
    let index = load_index(&store, &baker.manifest()).unwrap();
    assert_eq!(index.depths, BTreeMap::from([("textures/a".to_owned(), "textures/a_nh_depth.ktx2".to_owned())]));
    assert_eq!(index.depth_entries.len(), 1);
    assert_eq!(index.normals.get("textures/a").map(String::as_str), Some("textures/a_nh.dds"));

    let mount = |vfs: &Arc<Vfs>| {
        let (layer, stats) = CookedTextureLayer::build(store.clone(), None, Some(&index), vfs, &TestInputs(vfs.clone()));
        (vfs.with_top_layer(Arc::new(layer)), stats)
    };
    let (vfs, stats) = mount(&base);
    assert_eq!((stats.depths, stats.normals), (1, 1));
    let depths = read_depths(&vfs);
    assert_eq!(depths, index.depths);
    let served = depth_of(&depths, "Textures\\A.dds").unwrap();
    assert_eq!(vk_format(&vfs.read(&VfsPath::new(served).unwrap()).unwrap()), 139);
    let normal = vfs.resolve_preferring(&VfsPath::new("textures/a_nh.dds").unwrap(), TEXTURE_ALTERNATES).unwrap().0;
    assert_eq!(normal.as_str(), "textures/a_nh.ktx2", "the normal still loads from its own file");
    assert!(read_depths(&base).is_empty(), "no layer, no depth maps");

    // The pack's `_nh` changed since the bake: withheld until the next bake.
    let modded = Arc::new(Vfs::builder().mount(data()).mount(pack(2)).build());
    let (vfs, stats) = mount(&modded);
    assert_eq!(stats.depths, 0);
    assert!(read_depths(&vfs).is_empty());
    assert!(vfs.read(&VfsPath::new("textures/a_nh_depth.ktx2").unwrap()).is_err());
}
