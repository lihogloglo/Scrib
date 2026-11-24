//! Exact tests of the vendored `bevy_grass` raw image reads (slice GC-3b4c, own patch "raw image reads"): the
//! byte-level fast paths give bit for bit what the crate's per-texel `get_color_at` reads give. They live here
//! because cargo cannot test the vendored crate, which is not a workspace member.

use bevy::asset::RenderAssetUsages;
use bevy::image::Image;
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};
use bevy_grass::superfargoth_test::{mask_words, pack_mask, pack_mask_per_texel, read_heights, read_heights_per_texel};

fn raw_image(width: u32, height: u32, format: TextureFormat, data: Vec<u8>) -> Image {
    Image::new(
        Extent3d {
            width,
            height,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        data,
        format,
        RenderAssetUsages::default(),
    )
}

#[test]
fn raw_mask_reads_match_the_per_texel_reads() {
    // Every byte value, over an odd texel count (17 by 17) so the last word is
    // a padded half.
    let (w, h) = (17u32, 17u32);
    let byte = |i: usize, salt: usize| u8::try_from((i * 7 + salt * 91) % 256).expect("below 256");
    let n = (w * h) as usize;
    for (format, per) in [
        (TextureFormat::Rg8Unorm, 2),
        (TextureFormat::R8Unorm, 1),
        (TextureFormat::Rgba8Unorm, 4),
    ] {
        let data: Vec<u8> = (0..n * per).map(|i| byte(i, per)).collect();
        let mut seen = [false; 256];
        for &b in &data {
            seen[b as usize] = true;
        }
        assert!(seen.iter().all(|&s| s), "{format:?}: every byte value is tested");
        let image = raw_image(w, h, format, data);
        let fast = pack_mask(&image);
        assert_eq!(fast, pack_mask_per_texel(&image), "{format:?}");
        assert_eq!(u32::try_from(fast.len()).expect("small"), mask_words(w * h), "{format:?}");
    }

    // No data: every texel is missing (0xffff), down the per-texel path.
    let mut empty = raw_image(3, 1, TextureFormat::Rg8Unorm, vec![0; 6]);
    empty.data = None;
    assert_eq!(pack_mask(&empty), vec![0xffff_ffff, 0xffff]);
    assert_eq!(pack_mask(&empty), pack_mask_per_texel(&empty));
}

#[test]
fn raw_height_reads_match_the_per_texel_reads() {
    let values = [
        0.0f32,
        -0.0,
        1.0,
        0.5,
        -1.0,
        f32::MIN_POSITIVE,
        f32::MAX,
        f32::MIN,
        f32::INFINITY,
        f32::NEG_INFINITY,
        f32::NAN,
        1.0e-40, // subnormal
        0.333_333_34,
        123_456.79,
        -0.001,
    ];
    let (w, h) = (5u32, 3u32);
    let data = values.iter().flat_map(|v| v.to_le_bytes()).collect();
    let image = raw_image(w, h, TextureFormat::R32Float, data);
    let bits = |heights: Vec<f32>| heights.into_iter().map(f32::to_bits).collect::<Vec<_>>();
    let fast = bits(read_heights(&image));
    assert_eq!(fast, bits(read_heights_per_texel(&image)));
    assert_eq!(fast, values.iter().map(|v| v.to_bits()).collect::<Vec<_>>());

    // No data: every height is 0, down the per-texel path.
    let mut empty = raw_image(2, 1, TextureFormat::R32Float, vec![0; 8]);
    empty.data = None;
    assert_eq!(read_heights(&empty), vec![0.0, 0.0]);
}
