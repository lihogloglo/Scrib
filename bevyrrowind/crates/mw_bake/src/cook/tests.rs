use std::sync::Arc;

use ddsfile::{D3DFormat, Dds, NewD3dParams};
use sf_bake::{BakeError, Baker, Digest, Input, Inputs, Store};
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
    let uses: Vec<TextureUse> = refs.iter().map(|r| use_of(r, Role::Albedo, None, false)).collect();
    let report = DelightReport { uses, ..DelightReport::default() };
    let mut baker = Baker::new(store.clone(), Arc::new(TestInputs(vfs.clone())));
    let cooked = bake_cook(&mut baker, vfs, &report);
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
    assert_eq!(stats, LayerStats { delit: 1, cooked: 1, in_place: 1, stale: 0, normals: 0 }, "a's cook entry is shadowed by de-light");
    assert_eq!(read(&vfs, "textures/a.ktx2"), b"delit ktx2");
    // The cooked KTX2 is served without its meta trailer.
    let b = read(&vfs, "textures/b.ktx2");
    let product = store.get(&index.entries[1].object.unwrap()).unwrap();
    assert_eq!(&b[..6], b"\xABKTX 2");
    assert_eq!(b.len(), split_product(&product).unwrap().0);
    assert_eq!(resolve(&vfs, "textures/c.tga").as_str(), "textures/c.dds", "in place: the shipped DDS loads");
    assert_eq!(layer_index(&vfs), Some(sf_vfs::SourceIndex(1)));

    // Without de-light, the cook serves `a` too.
    let (vfs, stats) = mount(&base, None);
    assert_eq!(stats, LayerStats { delit: 0, cooked: 2, in_place: 1, stale: 0, normals: 0 });
    assert_eq!(&read(&vfs, "textures/a.ktx2")[..6], b"\xABKTX 2");

    // A mod's newer `b` or another variant of it: the entry is stale and the mod's file loads.
    for (file, bytes) in [("textures/b.png", png(16, 16, 3)), ("textures/b.dds", dds(D3DFormat::DXT1, 16, 16, 5))] {
        let modded = Arc::new(Vfs::builder().mount(data()).mount(MemSource::new("mod").with(file, bytes.clone())).build());
        let (vfs, stats) = mount(&modded, None);
        assert_eq!(stats, LayerStats { delit: 0, cooked: 1, in_place: 1, stale: 1, normals: 0 }, "{file}");
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
    };
    let mut report = CookReport { delit: 5, missing: 1, skipped: BTreeMap::from([(CookSkip::SkyUi, 3)]), built: 2, ..CookReport::default() };
    report.count(&index);
    assert_eq!(
        report.to_string(),
        "texture.cook: 3 textures (in place 1, rewrapped 1 [filled 1, remapped 0], encoded 1), 5 left to de-light, 1 missing; 0 normal companions; \
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
        ["ktx2", "dds", "png", "tga"].iter().map(|e| format!("{s}.{e}")).find(|p| exists(p))
    };
    let mut uses: Vec<TextureUse> =
        ["a", "b", "c", "plain"].iter().map(|n| use_of(&format!("textures/{n}.tga"), Role::Albedo, None, false)).collect();
    uses.push(use_of("textures/g.dds", Role::Glow, None, false));
    let mut plan = CookPlan::default();
    plan_normals(&mut plan, &uses, crate::texture_profiles::default_profile(), resolve, exists);
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
