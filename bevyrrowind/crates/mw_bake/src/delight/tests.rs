use std::sync::Arc;

use sf_bake::{BakeError, Baker, Digest, Input, Inputs, ProductKey, Store};
use sf_vfs::{MemSource, Vfs, VfsPath};

use super::*;
use crate::cook::{CookedTextureLayer, LayerStats};

fn albedo(texture: &str) -> TextureUse {
    TextureUse { texture: texture.to_owned(), role: Role::Albedo, clamp: [false; 2], cutoff: None, candidate: false }
}

fn with(texture: &str, role: Role) -> TextureUse {
    TextureUse { role, ..albedo(texture) }
}

/// `plan_jobs` over a VFS that holds exactly `files`; references resolve like the runtime.
type Case = (&'static str, Vec<TextureUse>, &'static [&'static str], Option<Skip>);

fn plan(uses: Vec<TextureUse>, files: &[&str]) -> (BTreeMap<String, TextureJob>, BTreeMap<Skip, u32>) {
    let mut source = MemSource::new("data");
    for f in files {
        source.insert(f, *f).unwrap();
    }
    let vfs = Vfs::builder().mount(source).build();
    let resolve =
        |r: &str| vfs.resolve_preferring(&VfsPath::new(r).ok()?, TEXTURE_ALTERNATES).map(|(p, _)| p.as_str().to_owned());
    let source_of = |p: &str| VfsPath::new(p).ok().and_then(|p| vfs.resolve(&p));
    plan_jobs(uses, resolve, source_of)
}

#[test]
fn exclusion_policy() {
    let cases: Vec<Case> = vec![
        ("plain albedo, tga ref resolves to dds", vec![albedo("textures/a.tga")], &["textures/a.dds"], None),
        ("missing", vec![albedo("textures/a.tga")], &[], Some(Skip::Missing)),
        ("normal companion", vec![albedo("textures/a.dds")], &["textures/a.dds", "textures/a_n.dds"], Some(Skip::Companion)),
        ("parallax companion", vec![albedo("textures/a.dds")], &["textures/a.dds", "textures/a_nh.png"], Some(Skip::Companion)),
        ("spec companion", vec![albedo("textures/a.dds")], &["textures/a.dds", "textures/a_spec.ktx2"], Some(Skip::Companion)),
        ("also a glow map", vec![albedo("textures/a.dds"), with("textures/a.dds", Role::Glow)], &["textures/a.dds"], Some(Skip::Role(Role::Glow))),
        ("also a detail map", vec![albedo("textures/a.dds"), with("textures/a.tga", Role::Detail)], &["textures/a.dds"], Some(Skip::Role(Role::Detail))),
        ("glows through emissive", vec![with("textures/a.dds", Role::Emissive)], &["textures/a.dds"], Some(Skip::Role(Role::Emissive))),
        ("particle / effect", vec![with("textures/a.dds", Role::Effect)], &["textures/a.dds"], Some(Skip::Role(Role::Effect))),
        ("translucent", vec![albedo("textures/a.dds"), with("textures/a.dds", Role::Translucent)], &["textures/a.dds"], Some(Skip::Role(Role::Translucent))),
        ("sky", vec![albedo("textures/tx_sky_clear.dds")], &["textures/tx_sky_clear.dds"], Some(Skip::SkyUi)),
        ("moon", vec![albedo("textures/tx_secunda_full.dds")], &["textures/tx_secunda_full.dds"], Some(Skip::SkyUi)),
        ("ui", vec![albedo("textures/menu_head_block.dds")], &["textures/menu_head_block.dds"], Some(Skip::SkyUi)),
        ("authored ktx2", vec![albedo("textures/a.tga")], &["textures/a.ktx2"], Some(Skip::AuthoredKtx2)),
        ("two variants", vec![albedo("textures/a.tga")], &["textures/a.tga", "textures/a.dds"], Some(Skip::Ambiguous)),
    ];
    for (name, uses, files, expected) in cases {
        let (jobs, skipped) = plan(uses, files);
        match expected {
            None => {
                assert_eq!(jobs.len(), 1, "{name}: {jobs:?} {skipped:?}");
                assert!(skipped.is_empty(), "{name}: {skipped:?}");
            }
            Some(skip) => {
                assert!(jobs.is_empty(), "{name}: {jobs:?}");
                assert_eq!(skipped, BTreeMap::from([(skip, 1)]), "{name}");
            }
        }
    }
}

#[test]
fn glow_only_textures_are_not_counted() {
    let (jobs, skipped) = plan(vec![with("textures/g.dds", Role::Glow)], &["textures/g.dds"]);
    assert!(jobs.is_empty() && skipped.is_empty());
}

/// Textures-adapter §3 (T4): a normals-only pack over another source's albedo still de-lights it;
/// a pack shipping the albedo and its companion together is left as authored.
#[test]
fn companion_skips_de_lighting_only_from_the_albedo_source() {
    let mount = |base: &[&str], pack: &[&str]| {
        let (mut data, mut mods) = (MemSource::new("data"), MemSource::new("pack"));
        for f in base {
            data.insert(f, *f).unwrap();
        }
        for f in pack {
            mods.insert(f, *f).unwrap();
        }
        Vfs::builder().mount(data).mount(mods).build()
    };
    let run = |vfs: &Vfs| {
        let resolve =
            |r: &str| vfs.resolve_preferring(&VfsPath::new(r).ok()?, TEXTURE_ALTERNATES).map(|(p, _)| p.as_str().to_owned());
        let source_of = |p: &str| VfsPath::new(p).ok().and_then(|p| vfs.resolve(&p));
        plan_jobs([albedo("textures/a.tga")], resolve, source_of)
    };
    // Normals-only pack over vanilla albedo: de-lit.
    let (jobs, skipped) = run(&mount(&["textures/a.dds"], &["textures/a_n.dds"]));
    assert_eq!(jobs.keys().collect::<Vec<_>>(), ["textures/a.dds"], "{skipped:?}");
    // Same for `_nh` and `_spec`.
    let (jobs, _) = run(&mount(&["textures/a.dds"], &["textures/a_nh.dds", "textures/a_spec.dds"]));
    assert_eq!(jobs.len(), 1);
    // The pack ships both: left as authored.
    let (jobs, skipped) = run(&mount(&["textures/a.dds"], &["textures/a.dds", "textures/a_n.dds"]));
    assert!(jobs.is_empty());
    assert_eq!(skipped, BTreeMap::from([(Skip::Companion, 1)]));
}

#[test]
fn job_merges_uses() {
    let clamped = TextureUse { clamp: [true, true], ..albedo("textures/a.tga") };
    let s_only = TextureUse { clamp: [false, true], cutoff: Some(0.5), candidate: true, ..albedo("textures/a.dds") };
    let masked = TextureUse { clamp: [true, true], cutoff: Some(0.25), ..albedo("textures/a.dds") };
    let masked2 = TextureUse { cutoff: Some(0.25), ..clamped.clone() };
    let (jobs, _) = plan(vec![clamped.clone(), s_only.clone()], &["textures/a.dds"]);
    assert_eq!(jobs["textures/a.dds"], TextureJob { wrap: [true, false], cutoff: Some(0.5), candidate: true });
    // Most common cutoff wins; a real alpha test makes it unconditional.
    let (jobs, _) = plan(vec![s_only, masked, masked2], &["textures/a.dds"]);
    assert_eq!(jobs["textures/a.dds"], TextureJob { wrap: [true, false], cutoff: Some(0.25), candidate: false });
    let (jobs, _) = plan(vec![clamped], &["textures/a.dds"]);
    assert_eq!(jobs["textures/a.dds"], TextureJob { wrap: [false, false], cutoff: None, candidate: false });
}

#[test]
fn decode_time_skips() {
    let job = TextureJob { wrap: [true; 2], cutoff: None, candidate: false };
    assert_eq!(delight_texture("textures/a.png", &png(32, 64, 0), &job, 0.5), Err(Skip::Small));
    assert_eq!(delight_texture("textures/a.png", &png(66, 64, 0), &job, 0.5), Err(Skip::Unaligned));
    assert_eq!(delight_texture("textures/a.dds", b"not a dds", &job, 0.5), Err(Skip::Undecodable));
    let ktx2 = delight_texture("textures/a.png", &png(64, 64, 0), &job, 0.5).unwrap();
    assert_eq!(&ktx2[..12], b"\xABKTX 20\xBB\r\n\x1A\n");
}

/// A `w` x `h` PNG with a light gradient painted in; `seed` changes the colours.
fn png(w: u32, h: u32, seed: u8) -> Vec<u8> {
    let image = image::RgbaImage::from_fn(w, h, |x, _| {
        let lit = u8::try_from(64 + x * 128 / w).unwrap();
        image::Rgba([lit, lit.wrapping_add(seed), 90, 255])
    });
    let mut bytes = std::io::Cursor::new(Vec::new());
    image.write_to(&mut bytes, image::ImageFormat::Png).unwrap();
    bytes.into_inner()
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

/// Bakes `texture.delight` + index for the jobs of `refs` over `vfs`; returns built count and index.
fn bake(store: &Store, vfs: &Arc<Vfs>, refs: &[&str], strength: f32) -> (u32, DelightIndex) {
    let resolve =
        |r: &str| vfs.resolve_preferring(&VfsPath::new(r).ok()?, TEXTURE_ALTERNATES).map(|(p, _)| p.as_str().to_owned());
    let source_of = |p: &str| VfsPath::new(p).ok().and_then(|p| vfs.resolve(&p));
    let (jobs, _) = plan_jobs(refs.iter().map(|r| albedo(r)), resolve, source_of);
    let jobs = Arc::new(jobs);
    let mut baker = Baker::new(store.clone(), Arc::new(TestInputs(vfs.clone())));
    baker.register(DelightStep { strength, jobs: jobs.clone() }).register(IndexStep { jobs });
    let report = baker.bake(&[ProductKey::new(INDEX_STEP, INDEX_SUBJECT)]);
    assert_eq!(report.failures().count(), 0);
    let built = report.steps.get(DELIGHT_STEP).map_or(0, |s| s.built);
    (built, load_index(store, &baker.manifest()).unwrap())
}

#[test]
fn memo_key_strength_and_source() {
    let dir = tempfile::tempdir().unwrap();
    let store = Store::open(dir.path()).unwrap();
    let base = |seed| {
        Arc::new(
            Vfs::builder()
                .mount(MemSource::new("data").with("textures/a.png", png(64, 64, seed)).with("textures/b.png", png(64, 64, 9)))
                .build(),
        )
    };
    let refs = ["textures/a.tga", "textures/b.tga"];
    let (built, index) = bake(&store, &base(0), &refs, 0.5);
    assert_eq!((built, index.entries.len()), (2, 2));
    // Same input: nothing re-runs, same index.
    let (built, again) = bake(&store, &base(0), &refs, 0.5);
    assert_eq!((built, &again), (0, &index));
    // One source changed: only it re-bakes.
    let (built, changed) = bake(&store, &base(1), &refs, 0.5);
    assert_eq!(built, 1);
    assert_ne!(changed.entries[0].fingerprint, index.entries[0].fingerprint);
    assert_eq!(changed.entries[1], index.entries[1]);
    // Strength change: every texture re-bakes.
    let (built, stronger) = bake(&store, &base(1), &refs, 0.6);
    assert_eq!(built, 2);
    assert_ne!(stronger.entries[1].ktx2, index.entries[1].ktx2);
}

#[test]
fn layer_precedence() {
    let dir = tempfile::tempdir().unwrap();
    let store = Store::open(dir.path()).unwrap();
    let data = || MemSource::new("data").with("textures/a.png", png(64, 64, 0)).with("textures/b.png", png(64, 64, 9));
    let base = Arc::new(Vfs::builder().mount(data()).build());
    let (_, index) = bake(&store, &base, &["textures/a.tga", "textures/b.tga"], 0.5);
    let mount = |vfs: &Arc<Vfs>| {
        let (layer, stats) = CookedTextureLayer::build(store.clone(), Some(&index), None, vfs, &TestInputs(vfs.clone()));
        (vfs.with_top_layer(Arc::new(layer)), stats)
    };
    let resolve = |vfs: &Vfs, r: &str| vfs.resolve_preferring(&VfsPath::new(r).unwrap(), TEXTURE_ALTERNATES).unwrap().0;

    // Current winners are served from the cache, as KTX2.
    let (vfs, stats) = mount(&base);
    assert_eq!(stats, LayerStats { delit: 2, ..LayerStats::default() });
    assert_eq!(resolve(&vfs, "textures/a.tga").as_str(), "textures/a.ktx2");
    assert_eq!(&vfs.read(&VfsPath::new("textures/a.ktx2").unwrap()).unwrap()[..6], b"\xABKTX 2");
    assert_eq!(layer_index(&vfs), Some(sf_vfs::SourceIndex(1)));
    assert_eq!(layer_index(&base), None);

    // A higher mod's own newer file of the same name: the entry is stale, the mod's file wins.
    let modded =
        Arc::new(Vfs::builder().mount(data()).mount(MemSource::new("mod").with("textures/a.png", png(64, 64, 5))).build());
    let (vfs, stats) = mount(&modded);
    assert_eq!(stats, LayerStats { delit: 1, stale: 1, ..LayerStats::default() });
    assert_eq!(resolve(&vfs, "textures/a.tga").as_str(), "textures/a.png");
    assert_eq!(vfs.read(&VfsPath::new("textures/a.png").unwrap()).unwrap(), png(64, 64, 5));
    assert_eq!(resolve(&vfs, "textures/b.tga").as_str(), "textures/b.ktx2");

    // A mod's .dds (another variant) or authored .ktx2: stale too, the mod wins.
    for file in ["textures/a.dds", "textures/a.ktx2"] {
        let modded = Arc::new(Vfs::builder().mount(data()).mount(MemSource::new("mod").with(file, "authored")).build());
        let (vfs, stats) = mount(&modded);
        assert_eq!(stats, LayerStats { delit: 1, stale: 1, ..LayerStats::default() }, "{file}");
        assert_eq!(resolve(&vfs, "textures/a.tga").as_str(), file);
        assert_eq!(vfs.read(&VfsPath::new(file).unwrap()).unwrap(), b"authored", "{file}");
    }
}

#[test]
fn roles_from_nif_materials() {
    let tex = |t: &str| Some(format!("textures/{t}.tga"));
    let m = mw_nif::Material::default;
    let nif = mw_nif::Model {
        materials: vec![
            mw_nif::Material { base_texture: tex("wall"), base_clamp: [true, false], ..m() },
            mw_nif::Material { base_texture: tex("lamp"), emissive: [0.5, 0.4, 0.2], ..m() },
            mw_nif::Material { base_texture: tex("rune"), glow_texture: tex("rune_glow"), detail_texture: tex("grime"), ..m() },
            mw_nif::Material { base_texture: tex("flame"), alpha_mode: AlphaMode::Blend { additive: true }, ..m() },
            mw_nif::Material { base_texture: tex("glass"), alpha_mode: AlphaMode::Blend { additive: false }, ..m() },
            mw_nif::Material {
                base_texture: tex("leaf"),
                alpha_mode: AlphaMode::Blend { additive: false },
                cutout: Cutout::Candidate,
                ..m()
            },
            mw_nif::Material { base_texture: tex("fence"), alpha_mode: AlphaMode::Mask { cutoff: 0.5 }, ..m() },
            mw_nif::Material { base_texture: tex("ghost"), unlit_lighting: true, ..m() },
        ],
        ..Default::default()
    };
    let roles: Vec<(String, Role, Option<f32>, bool)> =
        roles_of("meshes/x/a.nif", &nif).into_iter().map(|u| (u.texture, u.role, u.cutoff, u.candidate)).collect();
    let t = |s: &str| format!("textures/{s}.tga");
    assert_eq!(
        roles,
        vec![
            (t("wall"), Role::Albedo, None, false),
            (t("lamp"), Role::Emissive, None, false),
            (t("rune_glow"), Role::Glow, None, false),
            (t("grime"), Role::Detail, None, false),
            (t("rune"), Role::Albedo, None, false),
            (t("flame"), Role::Effect, None, false),
            (t("glass"), Role::Translucent, None, false),
            (t("leaf"), Role::Albedo, Some(mw_nif::CUTOUT_CUTOFF), true),
            (t("fence"), Role::Albedo, Some(0.5), false),
            (t("ghost"), Role::Emissive, None, false),
        ]
    );
    assert_eq!(roles_of("meshes/x/a.nif", &nif)[0].clamp, [true, false]);
    // Spell effect models: every base texture is an effect.
    assert!(roles_of("meshes/e/magic.nif", &nif).iter().filter(|u| u.texture == t("wall")).all(|u| u.role == Role::Effect));
}

/// A `w` x `h` PNG whose alpha is `alpha(x, y)`: the whole input of an [`AlphaClass`].
fn png_alpha(w: u32, h: u32, alpha: impl Fn(u32, u32) -> u8) -> Vec<u8> {
    let image = image::RgbaImage::from_fn(w, h, |x, y| image::Rgba([200, 180, 90, alpha(x, y)]));
    let mut bytes = std::io::Cursor::new(Vec::new());
    image.write_to(&mut bytes, image::ImageFormat::Png).unwrap();
    bytes.into_inner()
}

fn candidate(texture: &str) -> TextureUse {
    TextureUse { cutoff: Some(mw_nif::CUTOUT_CUTOFF), candidate: true, ..albedo(texture) }
}

/// `texture.alpha_class`: one class per *winning file* for every cutout candidate's reference,
/// collected into the index the static-scene bake reads, and memoized on the file.
#[test]
fn alpha_classes_per_texture_file() {
    let dir = tempfile::tempdir().unwrap();
    let store = Store::open(dir.path()).unwrap();
    // Holes and solid texels (a leaf card), flat half-alpha (gauze), and a file that won't decode.
    let leaf = |cutout: bool| png_alpha(16, 16, move |x, _| if !cutout { 128 } else if x % 2 == 0 { 0 } else { 255 });
    let world = |cutout| {
        Arc::new(
            Vfs::builder()
                .mount(
                    MemSource::new("data")
                        .with("textures/leaf.png", leaf(cutout))
                        .with("textures/glass.png", png_alpha(16, 16, |_, _| 128))
                        .with("textures/broken.dds", b"not a dds".to_vec()),
                )
                .build(),
        )
    };
    // References as a material names them (`.dds`), resolving to the files that exist.
    let uses: Vec<TextureUse> =
        ["textures/leaf.dds", "textures/glass.dds", "textures/broken.dds", "textures/absent.dds"].iter().map(|t| candidate(t)).collect();
    let bake = |vfs: &Arc<Vfs>| {
        let resolve =
            |r: &str| vfs.resolve_preferring(&VfsPath::new(r).ok()?, TEXTURE_ALTERNATES).map(|(p, _)| p.as_str().to_owned());
        let textures = Arc::new(alpha_class_jobs(&uses, resolve));
        // `absent.dds` resolves to nothing: no class, and the rule's answer is "not a cutout".
        assert_eq!(textures.len(), 3, "{textures:?}");
        assert_eq!(textures.get("textures/leaf.dds").map(String::as_str), Some("textures/leaf.png"));
        let mut baker = Baker::new(store.clone(), Arc::new(TestInputs(vfs.clone())));
        baker.register(AlphaClassStep).register(AlphaIndexStep { textures });
        let report = baker.bake(&[ProductKey::new(ALPHA_INDEX_STEP, ALPHA_INDEX_SUBJECT)]);
        assert_eq!(report.failures().count(), 0);
        let built = report.steps.get(ALPHA_CLASS_STEP).map_or(0, |s| s.built);
        (built, load_alpha_classes(&store, &baker.manifest()).unwrap())
    };
    let class = |t: &str, c| (t.to_owned(), c);
    let (built, index) = bake(&world(true));
    assert_eq!(built, 3);
    assert_eq!(
        index.classes,
        BTreeMap::from([
            class("textures/leaf.dds", AlphaClass::Cutout),
            class("textures/glass.dds", AlphaClass::Soft),
            class("textures/broken.dds", AlphaClass::Undecodable),
        ])
    );
    // Same files: nothing re-runs, same index.
    let (built, again) = bake(&world(true));
    assert_eq!((built, &again), (0, &index));
    // The leaf texture is replaced by a translucent one: only it re-runs, and it is no longer a
    // cutout, so the material rule stops promoting it (and the cells re-bake: the classes are
    // part of `MwSceneSource::fingerprint`).
    let (built, soft) = bake(&world(false));
    assert_eq!(built, 1);
    assert_eq!(soft.classes["textures/leaf.dds"], AlphaClass::Soft);
}

#[test]
fn only_cutout_candidates_are_classified() {
    let resolve = |r: &str| Some(r.to_owned());
    // Plain albedo, a glow map, a translucent blend: their alpha class is nobody's question.
    let other = [albedo("textures/a.dds"), with("textures/b.dds", Role::Glow), with("textures/c.dds", Role::Translucent)];
    assert!(alpha_class_jobs(&other, resolve).is_empty());
    // An authored alpha test is not a candidate either: it is already alpha-tested.
    let tested = TextureUse { cutoff: Some(0.5), ..albedo("textures/d.dds") };
    assert!(alpha_class_jobs(&[tested], resolve).is_empty());
    assert_eq!(alpha_class_jobs(&[candidate("textures/leaf.dds")], resolve).len(), 1);
}

/// textures-adapter.md §3 + §8: a companion the albedo's source profile names (here a renamed
/// `_normal`) marks a PBR replacer in that source only; elsewhere the same name is no companion.
#[test]
fn a_profile_companion_skips_de_light_in_its_own_source() {
    use crate::texture_profiles::TextureSources;
    let section = sf_mods::TextureSection {
        suffixes: BTreeMap::from([("_normal".into(), "_n".into())]),
        ..sf_mods::TextureSection::default()
    };
    let sources = TextureSources::from_sections([(1..2, "pack", &section)]);
    let data = MemSource::new("data").with("textures/a.dds", "a").with("textures/a_normal.dds", "n");
    let pack = MemSource::new("pack").with("textures/b.dds", "b").with("textures/b_normal.dds", "n");
    let vfs = Vfs::builder().mount(data).mount(pack).build();
    let resolve =
        |r: &str| vfs.resolve_preferring(&VfsPath::new(r).ok()?, TEXTURE_ALTERNATES).map(|(p, _)| p.as_str().to_owned());
    let source_of = |p: &str| VfsPath::new(p).ok().and_then(|p| vfs.resolve(&p));
    let companions = |s: &sf_vfs::SourceIndex| sources.companion_suffixes(*s).map(str::to_owned).collect();
    let (jobs, skipped) = plan_jobs_with([albedo("textures/a.tga"), albedo("textures/b.tga")], resolve, source_of, companions);
    assert_eq!(jobs.keys().collect::<Vec<_>>(), ["textures/a.dds"]);
    assert_eq!(skipped, BTreeMap::from([(Skip::Companion, 1)]));
}
