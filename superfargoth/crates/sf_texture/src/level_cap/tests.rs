use super::*;
use crate::ktx2_out::{BlockFormat, level_len, mip_count, write_ktx2};

/// A KTX2 file whose levels are filled with their level number (byte-identity checks).
fn file(format: BlockFormat, width: u32, height: u32) -> (Vec<u8>, Vec<Vec<u8>>) {
    let levels: Vec<Vec<u8>> = (0..mip_count(width, height))
        .map(|l| {
            let len = level_len(format, level_size(width, l), level_size(height, l));
            (0..len).map(|i| u8::try_from((l as usize * 31 + i) % 251).expect("< 251")).collect()
        })
        .collect();
    (write_ktx2(format, width, height, &levels), levels)
}

fn read(bytes: &[u8]) -> (Header, Vec<Vec<u8>>) {
    let reader = ktx2::Reader::new(bytes).expect("valid ktx2");
    (reader.header(), reader.levels().map(|l| l.data.to_vec()).collect())
}

#[test]
fn bc7_2048_under_1024_drops_the_top_level_byte_identical() {
    let (bytes, levels) = file(BlockFormat::Bc7Srgb, 2048, 2048);
    let (out, plan) = cap_ktx2(&bytes, Some(1024)).expect("caps");
    assert_eq!((plan.skip, plan.width, plan.height, plan.levels), (1, 1024, 1024, 11));
    let (header, kept) = read(&out);
    assert_eq!((header.pixel_width, header.pixel_height, header.level_count), (1024, 1024, 11));
    assert_eq!(header.format, Some(ktx2::Format::BC7_SRGB_BLOCK));
    assert_eq!(kept, levels[1..]);
    let top = level_len(BlockFormat::Bc7Srgb, 2048, 2048) as u64;
    let total: u64 = levels.iter().map(|l| l.len() as u64).sum();
    assert_eq!((plan.bytes_skipped, plan.bytes_kept), (top, total - top));
}

#[test]
fn no_cap_and_small_textures_are_untouched() {
    let (bytes, _) = file(BlockFormat::Bc7Srgb, 2048, 2048);
    let (out, plan) = cap_ktx2(&bytes, None).expect("reads");
    assert!(matches!(out, Cow::Borrowed(_)));
    assert_eq!((plan.skip, plan.levels, plan.bytes_skipped, plan.read_len), (0, 12, 0, None));

    let (small, _) = file(BlockFormat::Bc1, 512, 256);
    let (out, plan) = cap_ktx2(&small, Some(512)).expect("reads");
    assert_eq!(&*out, &small[..]);
    assert_eq!((plan.skip, plan.width, plan.height), (0, 512, 256));
}

#[test]
fn bc1_never_starts_below_a_whole_block() {
    // 64² BC1 under a 1 px cap: 4x4 is the smallest block-aligned level (then 2x2, 1x1 stay).
    let (bytes, levels) = file(BlockFormat::Bc1, 64, 64);
    let (out, plan) = cap_ktx2(&bytes, Some(1)).expect("caps");
    assert_eq!((plan.skip, plan.width, plan.height, plan.levels), (4, 4, 4, 3));
    assert_eq!(read(&out).1, levels[4..]);
    // 2048x8 under 512: level 2 would be 512x2, so it backs off to 1024x4.
    assert_eq!(first_level(2048, 8, mip_count(2048, 8), (4, 4), 512), 1);
    assert_eq!(first_level(2048, 8, mip_count(2048, 8), (1, 1), 512), 2);
}

#[test]
fn colour_space_follows_the_file() {
    for (format, vk) in [(BlockFormat::Bc7, ktx2::Format::BC7_UNORM_BLOCK), (BlockFormat::Bc1Srgb, ktx2::Format::BC1_RGBA_SRGB_BLOCK)] {
        let (bytes, _) = file(format, 1024, 1024);
        let (out, plan) = cap_ktx2(&bytes, Some(256)).expect("caps");
        assert_eq!(plan.skip, 2);
        assert_eq!(read(&out).0.format, Some(vk));
    }
}

#[test]
fn the_trim_needs_only_the_files_prefix() {
    let (bytes, levels) = file(BlockFormat::Bc5, 2048, 1024);
    let layout = Layout::parse(&bytes).expect("parses");
    let plan = layout.plan(Some(512));
    let len = usize::try_from(plan.read_len.expect("trimmed")).expect("fits");
    assert!(len < bytes.len() - levels[0].len());
    let out = trim(&bytes[..len], &layout, &plan).expect("prefix suffices");
    assert_eq!(read(&out).1, levels[2..]);
    assert_eq!(trim(&bytes[..len - 1], &layout, &plan), Err(CapError::Truncated));
}

#[test]
fn volumes_are_served_whole() {
    let (mut bytes, _) = file(BlockFormat::Bc7, 256, 256);
    let mut header = Header::from_bytes(bytes[..HEADER_LEN].try_into().expect("80 bytes")).expect("header");
    header.pixel_depth = 2;
    bytes[..HEADER_LEN].copy_from_slice(&header.as_bytes());
    let plan = Layout::parse(&bytes).expect("parses").plan(Some(64));
    assert_eq!((plan.skip, plan.uncappable), (0, Some(Uncappable::Volume)));
}
