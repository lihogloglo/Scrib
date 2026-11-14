use super::*;
use bevy_asset::RenderAssetUsages;
use bevy_image::{ImageSampler, ImageType};
use sf_texture::ktx2_out::{BlockFormat, level_len, mip_count, write_ktx2};
use sf_texture::level_cap::{cap_ktx2, level_size};

fn file(format: BlockFormat, width: u32, height: u32) -> Vec<u8> {
    let levels: Vec<Vec<u8>> = (0..mip_count(width, height))
        .map(|l| vec![u8::try_from(l).expect("< 256"); level_len(format, level_size(width, l), level_size(height, l))])
        .collect();
    write_ktx2(format, width, height, &levels)
}

fn capped(bytes: &[u8], cap: Option<u32>) -> (Vec<u8>, CapPlan) {
    let mut reader = VecReader::new(bytes.to_vec());
    futures_lite::future::block_on(read_capped(&mut reader, cap)).expect("reads")
}

fn image(bytes: &[u8], srgb: bool) -> Image {
    Image::from_buffer(
        bytes,
        ImageType::Extension("ktx2"),
        CompressedImageFormats::BC,
        srgb,
        ImageSampler::Default,
        RenderAssetUsages::RENDER_WORLD,
    )
    .expect("bevy reads the trimmed file")
}

#[test]
fn reader_path_matches_the_in_memory_cap_and_bevy_builds_the_image() {
    let bytes = file(BlockFormat::Bc7Srgb, 2048, 2048);
    let (read, plan) = capped(&bytes, Some(1024));
    let (expected, _) = cap_ktx2(&bytes, Some(1024)).expect("caps");
    assert_eq!(read, &*expected);
    assert_eq!(plan.skip, 1);
    let img = image(&read, true);
    assert_eq!((img.width(), img.height(), img.texture_descriptor.mip_level_count), (1024, 1024, 11));
    assert_eq!(img.texture_descriptor.format, wgpu_types::TextureFormat::Bc7RgbaUnormSrgb);
    // Level 0 of the image is the file's level 1 (filled with 1s).
    let data = img.data.as_ref().expect("data");
    let top = level_len(BlockFormat::Bc7Srgb, 1024, 1024);
    assert!(data[..top].iter().all(|&b| b == 1));
    assert_eq!(data.len() as u64, plan.bytes_kept);
}

#[test]
fn settings_choose_the_colour_space_as_bevys_loader_does() {
    let bytes = file(BlockFormat::Bc7, 1024, 1024);
    let (read, _) = capped(&bytes, Some(512));
    assert_eq!(image(&read, false).texture_descriptor.format, wgpu_types::TextureFormat::Bc7RgbaUnorm);
    assert_eq!(image(&read, true).texture_descriptor.format, wgpu_types::TextureFormat::Bc7RgbaUnormSrgb);
}

#[test]
fn uncapped_files_pass_through_unchanged() {
    let bytes = file(BlockFormat::Bc1, 512, 512);
    assert_eq!(capped(&bytes, Some(512)).0, bytes);
    let big = file(BlockFormat::Bc1, 2048, 2048);
    assert_eq!(capped(&big, None).0, big);
}

#[test]
fn counters_sum_loaded_capped_and_bytes() {
    let counters = CapCounters::default();
    let big = file(BlockFormat::Bc7Srgb, 2048, 2048);
    let small = file(BlockFormat::Bc1, 256, 256);
    counters.record(&capped(&big, Some(1024)).1);
    counters.record(&capped(&small, Some(1024)).1);
    let top = level_len(BlockFormat::Bc7Srgb, 2048, 2048) as u64;
    let total = |f: BlockFormat, s: u32| -> u64 { (0..mip_count(s, s)).map(|l| level_len(f, level_size(s, l), level_size(s, l)) as u64).sum() };
    let c = counters.snapshot();
    assert_eq!(
        c,
        CapCounts {
            loaded: 2,
            capped: 1,
            uncappable: 0,
            bytes_loaded: total(BlockFormat::Bc7Srgb, 2048) - top + total(BlockFormat::Bc1, 256),
            bytes_skipped: top,
        }
    );
    assert_eq!(
        cap_line(Some(1024), CapCounts { loaded: 3, capped: 1, uncappable: 0, bytes_loaded: 3 << 20, bytes_skipped: 4 << 20 }),
        "texture.cap: 1024 px, 3 ktx2 loaded, 1 capped, 0 uncappable, 3.0 MiB loaded, 4.0 MiB skipped"
    );
}
