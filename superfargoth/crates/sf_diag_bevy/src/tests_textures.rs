//! Live texture/material gauge (feature `textures`): exact counts and GPU bytes, no GPU.

use bevy_app::App;
use bevy_asset::{Assets, RenderAssetUsages};
use bevy_camera::visibility::InheritedVisibility;
use bevy_ecs::system::RunSystemOnce;
use bevy_image::Image;
use bevy_mesh::{Mesh, Mesh3d, PrimitiveTopology};
use bevy_pbr::{MeshMaterial3d, StandardMaterial};
use bevy_render::render_resource::{Extent3d, TextureFormat};

use crate::textures::{GaugeInputs, MeshAttributeProbe, TextureStats, family, gpu_bytes};

fn image(format: TextureFormat, w: u32, h: u32, layers: u32, mips: u32) -> Image {
    let mut i = Image { data: None, ..Image::default() };
    i.texture_descriptor.format = format;
    i.texture_descriptor.size = Extent3d { width: w, height: h, depth_or_array_layers: layers };
    i.texture_descriptor.mip_level_count = mips;
    i
}

fn mesh(tangent: bool, uv0: bool) -> Mesh {
    let mut m = Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::all())
        .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, vec![[0.0f32; 3]; 3])
        .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, vec![[0.0f32, 1.0, 0.0]; 3]);
    if uv0 {
        m.insert_attribute(Mesh::ATTRIBUTE_UV_0, vec![[0.0f32; 2]; 3]);
    }
    if tangent {
        m.insert_attribute(Mesh::ATTRIBUTE_TANGENT, vec![[1.0f32, 0.0, 0.0, 1.0]; 3]);
    }
    m
}

#[test]
fn gpu_bytes_follow_blocks_mips_and_layers() {
    // BC5 256², full chain: 16-byte 4x4 blocks, the last three mips one block each.
    let bc5 = image(TextureFormat::Bc5RgUnorm, 256, 256, 1, 9);
    assert_eq!(gpu_bytes(&bc5), 65536 + 16384 + 4096 + 1024 + 256 + 64 + 16 + 16 + 16);
    assert_eq!(gpu_bytes(&image(TextureFormat::Rgba8UnormSrgb, 64, 64, 1, 1)), 64 * 64 * 4);
    assert_eq!(gpu_bytes(&image(TextureFormat::Rgba8Unorm, 4, 4, 6, 1)), 4 * 4 * 4 * 6, "array layers multiply");
    assert_eq!(gpu_bytes(&image(TextureFormat::Bc1RgbaUnorm, 6, 6, 1, 1)), 4 * 8, "partial blocks round up");
    assert_eq!(family(TextureFormat::Bc7RgbaUnormSrgb), 4);
    assert_eq!(family(TextureFormat::Bc2RgbaUnorm), 5);
    assert_eq!(family(TextureFormat::R8Unorm), 6);
}

#[test]
#[allow(clippy::similar_names)]
fn counts_materials_images_and_meshes_without_tangents() {
    let mut app = App::new();
    let mut images = Assets::<Image>::default();
    let mut materials = Assets::<StandardMaterial>::default();
    let mut meshes = Assets::<Mesh>::default();

    let bc5 = images.add(image(TextureFormat::Bc5RgUnorm, 256, 256, 1, 9));
    let srgb = images.add(image(TextureFormat::Rgba8UnormSrgb, 64, 64, 1, 1));
    let orm = images.add(image(TextureFormat::Bc7RgbaUnorm, 64, 64, 1, 1));

    let plain = materials.add(StandardMaterial::default());
    let pbr = materials.add(StandardMaterial {
        normal_map_texture: Some(bc5.clone()),
        metallic_roughness_texture: Some(orm.clone()),
        occlusion_texture: Some(orm.clone()),
        ..StandardMaterial::default()
    });
    let wrong = materials.add(StandardMaterial { normal_map_texture: Some(srgb.clone()), ..StandardMaterial::default() });

    let full = meshes.add(mesh(true, true));
    let no_tangent = meshes.add(mesh(false, true));
    let bare = meshes.add(mesh(false, false));
    // A RENDER_WORLD mesh after extraction: the main world can no longer read its attributes.
    let mut gone = mesh(false, true);
    gone.asset_usage = RenderAssetUsages::RENDER_WORLD;
    let _ = gone.take_gpu_data().expect("first extraction");
    let extracted = meshes.add(gone);
    let mut unread = mesh(true, true);
    unread.asset_usage = RenderAssetUsages::RENDER_WORLD;
    let _ = unread.take_gpu_data().expect("first extraction");
    let unread = meshes.add(unread);

    app.insert_resource(images).insert_resource(materials).insert_resource(meshes).init_resource::<MeshAttributeProbe>();
    let shown = InheritedVisibility::VISIBLE;
    let world = app.world_mut();
    world.spawn((Mesh3d(full.clone()), MeshMaterial3d(pbr.clone()), shown)); // good
    world.spawn((Mesh3d(no_tangent.clone()), MeshMaterial3d(pbr.clone()), shown)); // normal map ignored
    world.spawn((Mesh3d(bare.clone()), MeshMaterial3d(wrong.clone()), shown)); // no tangent, no UV0
    world.spawn((Mesh3d(no_tangent.clone()), MeshMaterial3d(plain.clone()), shown)); // no normal map: fine
    world.spawn((Mesh3d(no_tangent.clone()), MeshMaterial3d(pbr.clone()), InheritedVisibility::HIDDEN)); // not shown
    world.spawn((Mesh3d(extracted.clone()), MeshMaterial3d(pbr.clone()), shown)); // render world: no tangent
    world.spawn((Mesh3d(unread.clone()), MeshMaterial3d(pbr.clone()), shown)); // not prepared yet

    {
        let probe = world.resource::<MeshAttributeProbe>().clone();
        let mut l = probe.lock();
        l.prepared.insert(extracted.id());
        l.no_tangent.insert(extracted.id());
    }
    let stats = world
        .run_system_once(|inputs: GaugeInputs, probe: bevy_ecs::prelude::Res<MeshAttributeProbe>| inputs.collect(&probe.lock()))
        .expect("scan");

    let bc5_bytes = 87408;
    let srgb_bytes = 64 * 64 * 4;
    let bc7_bytes = 64 * 64;
    let mut families = [(0, 0); 7];
    families[3] = (1, bc5_bytes);
    families[4] = (1, bc7_bytes);
    families[6] = (1, srgb_bytes);
    assert_eq!(
        stats,
        TextureStats {
            images: 3,
            gpu_bytes: bc5_bytes + srgb_bytes + bc7_bytes,
            families,
            normal_srgb: 1,
            materials: 3,
            with_normal: 2,
            with_metallic_roughness: 1,
            with_occlusion: 1,
            with_depth: 0,
            shown: 6,
            normal_mapped: 5,
            normal_no_tangent: 3,
            normal_no_uv0: 1,
            unread: 1,
        }
    );
    assert_eq!(
        stats.material_line(),
        "materials.live: 3 StandardMaterial (normal 2, metallic_roughness 1, occlusion 1, depth 0); 6 shown entities, 5 normal-mapped, 3 normal map without tangents, 1 normal map without UV0, 1 unread"
    );
    assert!(stats.texture_line().starts_with("textures.live: 3 images, 0.1 MiB GPU (bc5 1/0.1 MiB, bc7 1/0.0 MiB, uncompressed 1/0.0 MiB), 1 normal maps sampled as sRGB"));

    // Change detection: >5 % on any count, any change from 0.
    let mut moved = stats.clone();
    assert!(!moved.changed_from(&stats));
    moved.materials = 3; // same
    moved.gpu_bytes += moved.gpu_bytes / 50; // +2 %
    assert!(!moved.changed_from(&stats));
    moved.with_depth = 1; // 0 -> 1
    assert!(moved.changed_from(&stats));
}
