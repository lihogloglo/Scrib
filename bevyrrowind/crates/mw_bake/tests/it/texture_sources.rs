//! The texture source matrix (textures-adapter.md §9, TS-1b): the same texture in every container
//! a Morrowind or `OpenMW` pack ships (legacy DDS, DX10 DDS, KTX2, TGA, PNG, BMP; block-compressed and
//! uncompressed), served from a BSA archive, a loose `Data Files` folder and a user mod folder
//! layered above, plus normal-map companions from another source than their albedo. Every case
//! cooks, the winner is the highest source, and the one documented skip (one stem in two
//! extensions across sources: ambiguous) is counted.

use std::path::Path;
use std::sync::Arc;

use ddsfile::{AlphaMode, D3D10ResourceDimension, D3DFormat, Dds, DxgiFormat, NewD3dParams, NewDxgiParams};
use image::ExtendedColorType;
use mw_bake::cook::{
    CookAlpha, CookJob, CookPath, CookSkip, CookedTextureLayer, bake_cook, cook_texture, load_index,
};
use mw_bake::delight::{DelightReport, Role, TextureUse};
use mw_bake::texture_profiles::TextureSources;
use mw_bake::textures::{TEXTURE_ALTERNATES, decode_rgba};
use mw_vfs::BsaSource;
use sf_bake::{BakeError, Baker, Digest, Input, Inputs, Store};
use sf_texture::{Bc7Quality, BlockFormat, ChannelMap, SourceY, encode_level, write_ktx2};
use sf_vfs::{DirSource, Vfs, VfsPath};

const SIZE: u32 = 16;
/// The colour of the copy in the BSA, the loose folder and the mod; distinct in R too (L8 reads R).
const SEEDS: [[u8; 3]; 3] = [[220, 60, 60], [60, 220, 140], [140, 60, 220]];

fn solid(c: [u8; 3]) -> Vec<u8> {
    [c[0], c[1], c[2], 255].repeat((SIZE * SIZE) as usize)
}

fn blocks(format: BlockFormat, c: [u8; 3]) -> Vec<u8> {
    encode_level(format, &solid(c), SIZE, SIZE, Bc7Quality::Fast)
}

fn write_dds(dds: &mut Dds, data: &[u8]) -> Vec<u8> {
    dds.data[..data.len()].copy_from_slice(data);
    let mut out = Vec::new();
    dds.write(&mut out).unwrap();
    out
}

fn legacy(format: D3DFormat, levels: u32, data: &[u8]) -> Vec<u8> {
    let params = NewD3dParams { height: SIZE, width: SIZE, depth: None, format, mipmap_levels: Some(levels), caps2: None };
    write_dds(&mut Dds::new_d3d(params).unwrap(), data)
}

fn dx10(format: DxgiFormat, data: &[u8]) -> Vec<u8> {
    let params = NewDxgiParams {
        height: SIZE,
        width: SIZE,
        depth: None,
        format,
        mipmap_levels: None,
        array_layers: None,
        caps2: None,
        is_cubemap: false,
        resource_dimension: D3D10ResourceDimension::Texture2D,
        alpha_mode: AlphaMode::Unknown,
    };
    write_dds(&mut Dds::new_dxgi(params).unwrap(), data)
}

/// A one-level uncompressed KTX2 (`VK_FORMAT_R8G8B8A8_UNORM`, an empty 4-byte DFD).
fn ktx2_rgba(c: [u8; 3]) -> Vec<u8> {
    let data = solid(c);
    let mut out = b"\xABKTX 20\xBB\r\n\x1A\n".to_vec();
    for v in [37, 1, SIZE, SIZE, 0, 0, 1, 1, 0] {
        out.extend_from_slice(&u32::to_le_bytes(v));
    }
    for v in [104u32, 4, 0, 0] {
        out.extend_from_slice(&v.to_le_bytes());
    }
    out.extend_from_slice(&[0; 16]);
    let len = data.len() as u64;
    for v in [108, len, len] {
        out.extend_from_slice(&u64::to_le_bytes(v));
    }
    out.extend_from_slice(&4u32.to_le_bytes());
    out.extend_from_slice(&data);
    out
}

fn with_image(c: [u8; 3], format: image::ImageFormat) -> Vec<u8> {
    let mut out = Vec::new();
    let img = image::RgbImage::from_pixel(SIZE, SIZE, image::Rgb(c));
    img.write_to(&mut std::io::Cursor::new(&mut out), format).unwrap();
    out
}

/// A legacy `L8` DDS of R (flagged `DDPF_LUMINANCE` as D3DX and `DirectXTex` write it; `ddsfile`
/// writes `DDPF_RGB`).
fn l8(c: [u8; 3]) -> Vec<u8> {
    let mut out = legacy(D3DFormat::L8, 1, &[c[0]].repeat((SIZE * SIZE) as usize));
    out[80..84].copy_from_slice(&0x2_0000u32.to_le_bytes());
    out
}

/// A container: name, file extension, the file for a colour, whether it reads as grey (of R).
type Container = (&'static str, &'static str, fn([u8; 3]) -> Vec<u8>, bool);

const CONTAINERS: [Container; 11] = [
    // Full-chain DXT1: indexed in place. 16, 8, 4, 2, 1: 16 + 4 + 1 + 1 + 1 blocks of one solid block.
    ("dxt1-full", "dds", |c| legacy(D3DFormat::DXT1, 5, &blocks(BlockFormat::Bc1, c)[..8].repeat(23)), false),
    // One-level DXT1: rewrapped, levels filled.
    ("dxt1", "dds", |c| legacy(D3DFormat::DXT1, 1, &blocks(BlockFormat::Bc1, c)), false),
    ("a8r8g8b8", "dds", |c| legacy(D3DFormat::A8R8G8B8, 1, &solid([c[2], c[1], c[0]])), false),
    ("l8", "dds", l8, true),
    ("dx10-bc7", "dds", |c| dx10(DxgiFormat::BC7_UNorm_sRGB, &blocks(BlockFormat::Bc7Srgb, c)), false),
    ("dx10-rgba", "dds", |c| dx10(DxgiFormat::R8G8B8A8_UNorm_sRGB, &solid(c)), false),
    ("ktx2-bc7", "ktx2", |c| write_ktx2(BlockFormat::Bc7Srgb, SIZE, SIZE, &[blocks(BlockFormat::Bc7Srgb, c)]), false),
    ("ktx2-rgba", "ktx2", ktx2_rgba, false),
    ("tga", "tga", |c| {
        let mut out = Vec::new();
        image::codecs::tga::TgaEncoder::new(&mut out).encode(&solid(c), SIZE, SIZE, ExtendedColorType::Rgba8).unwrap();
        out
    }, false),
    ("png", "png", |c| with_image(c, image::ImageFormat::Png), false),
    ("bmp", "bmp", |c| with_image(c, image::ImageFormat::Bmp), false),
];

/// Where a texture is: in the BSA only, loose over the BSA, or in the mod over both.
const PLACEMENTS: [(&str, usize); 3] = [("bsa", 1), ("loose", 2), ("mod", 3)];

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

fn albedo(texture: &str) -> TextureUse {
    TextureUse { texture: texture.to_owned(), role: Role::Albedo, clamp: [false; 2], cutoff: None, candidate: false }
}

/// The three sources' files: (`textures/<name>`, bytes) for the BSA, the loose folder, the mod.
#[derive(Default)]
struct Sources([Vec<(String, Vec<u8>)>; 3]);

impl Sources {
    fn add(&mut self, source: usize, name: &str, bytes: Vec<u8>) {
        self.0[source].push((name.to_owned(), bytes));
    }

    fn mount(&self, root: &Path) -> Arc<Vfs> {
        let bsa = root.join("Morrowind.bsa");
        let mut builder = tes3::bsa::Builder::new();
        for (name, bytes) in &self.0[0] {
            builder.insert(format!("textures\\{name}").into_bytes(), bytes).unwrap();
        }
        builder.save_path(&bsa).unwrap();
        let mut vfs = Vfs::builder().mount(BsaSource::open(&bsa).unwrap());
        for (i, label) in [(1, "Data Files"), (2, "user mod")] {
            let dir = root.join(label).join("Textures");
            std::fs::create_dir_all(&dir).unwrap();
            for (name, bytes) in &self.0[i] {
                std::fs::write(dir.join(name), bytes).unwrap();
            }
            vfs = vfs.mount(DirSource::scan(label, root.join(label)).unwrap());
        }
        Arc::new(vfs.build())
    }
}

fn close(a: &[u8], b: [u8; 3]) -> bool {
    a.iter().zip(b).all(|(&x, y)| x.abs_diff(y) <= 12)
}

#[test]
fn every_container_from_every_source_cooks_and_the_highest_source_wins() {
    let mut sources = Sources::default();
    let mut uses = Vec::new();
    // (reference, winning file, winning bytes, expected colour).
    let mut cases = Vec::new();
    for (container, ext, make, grey) in CONTAINERS {
        for (placement, copies) in PLACEMENTS {
            let name = format!("{container}_{placement}.{ext}");
            for (source, seed) in SEEDS.iter().enumerate().take(copies) {
                sources.add(source, &name, make(*seed));
            }
            let seed = SEEDS[copies - 1];
            let colour = if grey { [seed[0]; 3] } else { seed };
            // NIFs name `.tga` (found as DDS, KTX2, PNG through `TEXTURE_ALTERNATES`) or `.bmp` (only as itself).
            let reference_ext = if ext == "bmp" { "bmp" } else { "tga" };
            uses.push(albedo(&format!("textures/{container}_{placement}.{reference_ext}")));
            cases.push((format!("textures/{container}_{placement}.{reference_ext}"), format!("textures/{name}"), make(seed), colour));
        }
    }
    // Normal companions from another source than their albedo: BC7, which the default profile's
    // DirectX Y can't flip on the blocks (decode + cook), and a legacy DXT5.
    sources.add(0, "wall.dds", legacy(D3DFormat::DXT1, 1, &blocks(BlockFormat::Bc1, SEEDS[0])));
    sources.add(2, "wall_n.dds", dx10(DxgiFormat::BC7_UNorm, &blocks(BlockFormat::Bc7, [128, 128, 255])));
    sources.add(1, "floor.tga", with_image(SEEDS[1], image::ImageFormat::Tga));
    sources.add(2, "floor_n.ktx2", write_ktx2(BlockFormat::Bc7, SIZE, SIZE, &[blocks(BlockFormat::Bc7, [128, 128, 255])]));
    sources.add(2, "roof.png", with_image(SEEDS[2], image::ImageFormat::Png));
    sources.add(0, "roof_n.dds", legacy(D3DFormat::DXT5, 1, &blocks(BlockFormat::Bc3, [128, 128, 255])));
    uses.extend(["textures/wall.tga", "textures/floor.tga", "textures/roof.tga"].map(albedo));
    // One stem in two extensions across sources: ambiguous (a `.dds` and a `.tga` reference would
    // resolve to different files), served as shipped.
    sources.add(0, "mixed.dds", legacy(D3DFormat::DXT1, 1, &blocks(BlockFormat::Bc1, SEEDS[0])));
    sources.add(2, "mixed.tga", with_image(SEEDS[2], image::ImageFormat::Tga));
    uses.push(albedo("textures/mixed.tga"));

    let dir = tempfile::tempdir().unwrap();
    let vfs = sources.mount(dir.path());
    let store_dir = tempfile::tempdir().unwrap();
    let store = Store::open(store_dir.path()).unwrap();
    let mut baker = Baker::new(store.clone(), Arc::new(TestInputs(vfs.clone())));
    let report = DelightReport { uses, ..DelightReport::default() };
    let cooked = bake_cook(&mut baker, &vfs, &report, &TextureSources::default());
    assert!(cooked.failures.is_empty(), "{:?}", cooked.failures);
    let index = load_index(&store, &baker.manifest()).unwrap();
    let (layer, _) = CookedTextureLayer::build(store, None, Some(&index), &vfs, &TestInputs(vfs.clone()));
    let served = vfs.with_top_layer(Arc::new(layer));

    for (reference, winner, bytes, colour) in &cases {
        let entry = index.entries.iter().find(|e| &e.source == winner).unwrap_or_else(|| panic!("{winner} not cooked"));
        assert_eq!(entry.fingerprint, Digest::of(bytes), "{winner}: the highest source's file");
        assert!(!matches!(entry.meta.path, CookPath::Skipped(_)), "{winner}: {:?}", entry.meta.path);
        let (file, _) = served.resolve_preferring(&VfsPath::new(reference).unwrap(), TEXTURE_ALTERNATES).unwrap();
        let img = decode_rgba(file.as_str(), &served.read(&file).unwrap()).unwrap();
        let centre = ((SIZE / 2 * SIZE + SIZE / 2) * 4) as usize;
        assert!(close(&img.rgba[centre..centre + 3], *colour), "{winner} served as {file:?}: {:?}", &img.rgba[centre..centre + 4]);
    }
    for (base, companion) in [("wall", "wall_n.dds"), ("floor", "floor_n.ktx2"), ("roof", "roof_n.dds")] {
        assert_eq!(index.normals.get(&format!("textures/{base}")).map(String::as_str), Some(&*format!("textures/{companion}")));
        let entry = index.entries.iter().find(|e| e.source == format!("textures/{companion}")).expect(companion);
        assert!(entry.meta.normal, "{companion}");
        assert!(!matches!(entry.meta.path, CookPath::Skipped(_)), "{companion}: {:?}", entry.meta.path);
    }
    assert!(index.entries.iter().all(|e| !e.source.starts_with("textures/mixed.")));
    assert_eq!(cooked.skipped.get(&CookSkip::Ambiguous), Some(&1), "{:?}", cooked.skipped);
    let (mixed, _) = served.resolve_preferring(&VfsPath::new("textures/mixed.tga").unwrap(), TEXTURE_ALTERNATES).unwrap();
    assert_eq!(mixed.as_str(), "textures/mixed.tga", "the mod's file loads as shipped");
}

/// The cook's decode path (`Rewrap::Cook`) reads what the rewrap reads: a BC7 normal whose Y the
/// blocks can't flip, BC4/BC5 used as colour. Before TS-1b these were `Undecodable`.
#[test]
fn block_compressed_sources_the_rewrap_cannot_keep_cook_through_the_decoder() {
    let normal = CookJob::normal(ChannelMap::IDENTITY, SourceY::Down);
    let colour = CookJob { alpha: CookAlpha::Opaque, candidate: false, normal: None };
    let bc7 = blocks(BlockFormat::Bc7, [128, 100, 255]);
    let cases = [
        ("n.dds", dx10(DxgiFormat::BC7_UNorm, &bc7), normal),
        ("n.ktx2", write_ktx2(BlockFormat::Bc7, SIZE, SIZE, &[&bc7]), normal),
        ("bc4.dds", dx10(DxgiFormat::BC4_UNorm, &blocks(BlockFormat::Bc4, [90, 0, 0])), colour),
        ("bc5.ktx2", write_ktx2(BlockFormat::Bc5, SIZE, SIZE, &[blocks(BlockFormat::Bc5, [90, 30, 0])]), colour),
    ];
    for (path, bytes, job) in cases {
        let (ktx2, meta) = cook_texture(path, &bytes, &job, Bc7Quality::Fast);
        assert_eq!(meta.path, CookPath::Encoded, "{path}");
        assert!(!ktx2.is_empty(), "{path}");
    }
}
