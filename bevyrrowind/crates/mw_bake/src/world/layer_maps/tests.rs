use sf_mods::{NormalY, TextureSection};
use sf_records::RecordDb;
use sf_terrain::bake::{LayerFile, LayerSource, TerrainLayer, normal_to_array_frame};
use sf_vfs::{MemSource, Vfs};
use tes3::esp::{FileType, Header, LandscapeTexture, Plugin, TES3Object};

use crate::texture_profiles::TextureSources;
use crate::world::MwLayers;

/// A PNG of one repeated texel.
fn png(texel: [u8; 4]) -> Vec<u8> {
    let image = image::RgbaImage::from_fn(4, 4, |_, _| image::Rgba(texel));
    let mut bytes = std::io::Cursor::new(Vec::new());
    image.write_to(&mut bytes, image::ImageFormat::Png).unwrap();
    bytes.into_inner()
}

/// `Test.esm` with one `LTEX` per name, texture `tx_<name>.tga`.
fn esm(names: &[&str]) -> Vec<u8> {
    let mut objects: Vec<TES3Object> = vec![Header { file_type: FileType::Esm, ..Header::default() }.into()];
    for (i, name) in names.iter().enumerate() {
        let index = u32::try_from(i).unwrap();
        objects.push(LandscapeTexture { id: (*name).into(), index, file_name: format!("tx_{name}.tga"), ..LandscapeTexture::default() }.into());
    }
    Plugin { objects }.save_bytes().unwrap()
}

/// The base data (VFS source 0): `Test.esm` and the albedo of every layer.
fn data(names: &[&str]) -> MemSource {
    names.iter().fold(MemSource::new("data").with("Test.esm", esm(names)), |m, n| m.with(&format!("textures/tx_{n}.tga"), "albedo"))
}

fn db(vfs: &Vfs) -> RecordDb {
    mw_records::load(vfs, &["Test.esm".to_owned()]).unwrap().db
}

/// The layer whose albedo is `tx_<name>`.
fn layer(layers: &MwLayers, name: &str) -> TerrainLayer {
    let albedo = format!("textures/tx_{name}.tga");
    layers.layers().iter().filter_map(|id| layers.layer(id)).find(|l| l.albedo_height == albedo).unwrap()
}

/// The first texel of `file` decoded as `role` of a layer in `layers`.
fn texel(layers: &MwLayers, vfs: &Vfs, file: &str, role: LayerFile) -> [u8; 4] {
    let bytes = vfs.read(&sf_vfs::VfsPath::new(file).unwrap()).unwrap();
    let img = layers.decode(file, &bytes, role).unwrap();
    img.rgba[..4].try_into().unwrap()
}

/// A default-profile (`openmw`, DirectX Y) `_n` becomes the layer's normal: canonical +Y up
/// complements its green, and the arrays' +V frame complements it back, so the array slice holds
/// the DirectX green as shipped (`sf_terrain` `layer_normals_shade_the_relief_the_albedo_shows`
/// pins that frame against the shader). A layer without companions keeps the defaults.
#[test]
fn a_directx_normal_companion_becomes_the_layer_normal() {
    let names = ["a", "b"];
    let vfs = Vfs::builder().mount(data(&names).with("textures/tx_a_n.png", png([128, 51, 230, 255]))).build();
    let layers = MwLayers::new(&db(&vfs), &vfs);
    let a = layer(&layers, "a");
    assert_eq!((a.normal.as_deref(), a.arm.as_deref()), (Some("textures/tx_a_n.png"), None));
    let canonical = texel(&layers, &vfs, "textures/tx_a_n.png", LayerFile::Normal);
    assert_eq!(canonical[..3], [128, 204, 230]);
    let mut array = canonical;
    normal_to_array_frame(&mut array);
    assert_eq!(array[..2], [128, 51]);
    let b = layer(&layers, "b");
    assert_eq!((b.normal, b.arm), (None, None));
    assert_eq!(layers.companion_counts(), (1, 0));
}

/// Each companion is read with its own source's profile: a mod with `normal_y = "up"` keeps its
/// `_n` green, the base data's `_n` is flipped; changing the mod's profile changes only its
/// layer's fingerprint (the arrays re-bake).
#[test]
fn a_companion_reads_with_its_own_sources_profile() {
    let names = ["a", "b"];
    let n = png([128, 51, 230, 255]);
    let base = data(&names).with("textures/tx_a_n.png", n.clone());
    let pack = MemSource::new("pack").with("textures/tx_b_n.png", n);
    let vfs = Vfs::builder().mount(base).mount(pack).build();
    let db = db(&vfs);
    let up = TextureSection { normal_y: Some(NormalY::Up), ..TextureSection::default() };
    let sources = TextureSources::from_sections([(1..2, "pack", &up)]);
    let layers = MwLayers::with_sources(&db, &vfs, &sources);
    assert_eq!(texel(&layers, &vfs, "textures/tx_a_n.png", LayerFile::Normal)[1], 204);
    assert_eq!(texel(&layers, &vfs, "textures/tx_b_n.png", LayerFile::Normal)[1], 51);

    let default = MwLayers::new(&db, &vfs);
    let prints = |l: &MwLayers| l.fingerprints().collect::<Vec<_>>();
    let (before, after) = (prints(&default), prints(&layers));
    let changed: Vec<&str> = before.iter().zip(&after).filter(|(x, y)| x.1 != y.1).map(|(x, _)| x.0.1.as_str()).collect();
    assert_eq!(changed, ["b"]);
}

/// ARM from the ORM-slot companions: a PBR pack's packed `_spec` (R metallic, G roughness, B AO)
/// becomes R = AO, G = roughness; the default profile's `_spec` is a Blinn-Phong specular map
/// converted to roughness (no occlusion); a PBR pack's terrain `_diffusespec` gives roughness from
/// its alpha when no `_spec` exists.
#[test]
fn orm_specular_and_diffusespec_companions_become_ao_roughness() {
    let names = ["a", "b", "c"];
    let base = data(&names).with("textures/tx_b_spec.png", png([200, 200, 200, 40]));
    let pack = MemSource::new("pack")
        .with("textures/tx_a_spec.png", png([10, 200, 90, 255]))
        .with("textures/tx_c_diffusespec.png", png([60, 70, 80, 77]));
    let vfs = Vfs::builder().mount(base).mount(pack).build();
    let pbr = TextureSection { profile: Some("wareya-pbr".into()), ..TextureSection::default() };
    let layers = MwLayers::with_sources(&db(&vfs), &vfs, &TextureSources::from_sections([(1..2, "pack", &pbr)]));
    let arm = |name: &str| {
        let file = layer(&layers, name).arm.unwrap();
        texel(&layers, &vfs, &file, LayerFile::Arm)
    };
    assert_eq!(arm("a")[..2], [90, 200]);
    let mut specular = [200, 200, 200, 40];
    sf_texture::specular_to_orm(&mut specular);
    assert_eq!(arm("b"), specular);
    assert_eq!(arm("b")[0], 255);
    assert_eq!(arm("c"), [255, 77, 0, 255]);
    assert_eq!(layer(&layers, "c").arm.as_deref(), Some("textures/tx_c_diffusespec.png"));
    assert_eq!(layers.companion_counts(), (0, 3));
}
