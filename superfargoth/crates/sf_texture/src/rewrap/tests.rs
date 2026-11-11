use ktx2::{Format, Reader};

use super::*;
use crate::cook::{AlphaUse, Channel, Role, SourceY};
use crate::ktx2_out::{Bc7Quality, BlockFormat, level_len, mip_count, write_ktx2};
use crate::remap::{ChannelMap, RemapError, Src, plan};

const COLOR: Role = Role::Color {
    alpha: AlphaUse::None,
};
const NORMAL_UP: Role = Role::Normal {
    source_y: SourceY::Up,
};
const NORMAL_DOWN: Role = Role::Normal {
    source_y: SourceY::Down,
};
const MASK_R: Role = Role::Mask {
    channel: Channel::R,
    invert: false,
};

/// The format field of a synthetic DDS: a legacy `FourCC`, a DX10 header with this DXGI value, or
/// uncompressed 32-bit BGRA.
#[derive(Clone, Copy)]
pub(crate) enum Pf {
    FourCc(&'static [u8; 4]),
    Dxgi(u32),
    Bgra,
}

/// A DDS file per the `DDS_HEADER` reference. `mips`: the `dwMipMapCount` value; the
/// `DDSD_MIPMAPCOUNT` flag is set only when `mip_flag`. `caps2`: e.g. 0xFE00 for a cubemap.
pub(crate) fn dds(
    pf: Pf,
    width: u32,
    height: u32,
    mips: u32,
    mip_flag: bool,
    caps2: u32,
    data: &[u8],
) -> Vec<u8> {
    let mut out = Vec::new();
    let mut put = |value: u32| out.extend_from_slice(&value.to_le_bytes());
    put(u32::from_le_bytes(*b"DDS "));
    put(124);
    put(0x1 | 0x2 | 0x4 | 0x1000 | if mip_flag { 0x2_0000 } else { 0 });
    put(height);
    put(width);
    put(0); // pitch / linear size: deliberately left 0 (rewrap must not trust it)
    put(0); // depth
    put(mips);
    for _ in 0..11 {
        put(0);
    }
    // DDS_PIXELFORMAT
    put(32);
    match pf {
        Pf::FourCc(code) => {
            put(0x4);
            put(u32::from_le_bytes(*code));
            for _ in 0..5 {
                put(0);
            }
        }
        Pf::Dxgi(_) => {
            put(0x4);
            put(u32::from_le_bytes(*b"DX10"));
            for _ in 0..5 {
                put(0);
            }
        }
        Pf::Bgra => {
            put(0x41); // RGB | ALPHAPIXELS
            put(0);
            put(32);
            put(0x00FF_0000);
            put(0x0000_FF00);
            put(0x0000_00FF);
            put(0xFF00_0000);
        }
    }
    put(0x1000); // caps: TEXTURE
    put(caps2);
    put(0);
    put(0);
    put(0);
    if let Pf::Dxgi(format) = pf {
        put(format);
        put(3); // TEXTURE2D
        put(0);
        put(1);
        put(0);
    }
    out.extend_from_slice(data);
    out
}

/// Deterministic bytes standing in for blocks (any bytes are valid BC1-BC5 blocks).
pub(crate) fn noise(len: usize, seed: u32) -> Vec<u8> {
    let mut state = seed.wrapping_mul(2_654_435_761).wrapping_add(1);
    (0..len)
        .map(|_| {
            state ^= state << 13;
            state ^= state >> 17;
            state ^= state << 5;
            state.to_le_bytes()[0]
        })
        .collect()
}

/// The full chain of `format` levels for `width` x `height`, as noise, level 0 first.
fn chain(format: BlockFormat, width: u32, height: u32, levels: usize) -> Vec<Vec<u8>> {
    (0..levels)
        .map(|level| {
            let (w, h) = level_size(width, height, level);
            noise(level_len(format, w, h), u32::try_from(level).unwrap() + 7)
        })
        .collect()
}

fn rewrapped(outcome: Rewrap) -> Rewrapped {
    match outcome {
        Rewrap::Rewrapped(rewrapped) => rewrapped,
        other => panic!("expected a rewrap, got {other:?}"),
    }
}

fn ktx2_format(ktx2: &[u8]) -> Format {
    Reader::new(ktx2).unwrap().header().format.unwrap()
}

fn ktx2_levels(ktx2: &[u8]) -> Vec<Vec<u8>> {
    Reader::new(ktx2)
        .unwrap()
        .levels()
        .map(|level| level.data.to_vec())
        .collect()
}

/// A smooth RGBA pattern (gradients; alpha from `alpha`).
fn pattern(width: u32, height: u32, alpha: impl Fn(u32, u32) -> u8) -> Vec<u8> {
    let mut rgba = Vec::new();
    for y in 0..height {
        for x in 0..width {
            let r = u8::try_from(x * 255 / width.max(2).saturating_sub(1).max(1)).unwrap_or(255);
            let g = u8::try_from(y * 255 / height.max(2).saturating_sub(1).max(1)).unwrap_or(255);
            rgba.extend_from_slice(&[r, g, 128, alpha(x, y)]);
        }
    }
    rgba
}

#[test]
fn legacy_fourcc_maps_to_the_exact_vk_format() {
    let cases: [(&[u8; 4], Role, BlockFormat, Format); 9] = [
        (
            b"DXT1",
            COLOR,
            BlockFormat::Bc1,
            Format::BC1_RGBA_SRGB_BLOCK,
        ),
        (b"DXT2", COLOR, BlockFormat::Bc2, Format::BC2_SRGB_BLOCK),
        (b"DXT3", COLOR, BlockFormat::Bc2, Format::BC2_SRGB_BLOCK),
        (b"DXT4", COLOR, BlockFormat::Bc3, Format::BC3_SRGB_BLOCK),
        (b"DXT5", COLOR, BlockFormat::Bc3, Format::BC3_SRGB_BLOCK),
        (b"ATI1", MASK_R, BlockFormat::Bc4, Format::BC4_UNORM_BLOCK),
        (b"BC4U", MASK_R, BlockFormat::Bc4, Format::BC4_UNORM_BLOCK),
        (
            b"ATI2",
            NORMAL_UP,
            BlockFormat::Bc5,
            Format::BC5_UNORM_BLOCK,
        ),
        (
            b"BC5U",
            NORMAL_UP,
            BlockFormat::Bc5,
            Format::BC5_UNORM_BLOCK,
        ),
    ];
    for (code, role, linear, vk) in cases {
        let data: Vec<u8> = chain(linear, 16, 16, 5).concat();
        let bytes = dds(Pf::FourCc(code), 16, 16, 5, true, 0, &data);
        let source = read_source(&bytes).unwrap().unwrap();
        assert_eq!(
            (source.format, source.declared_srgb),
            (linear, None),
            "{code:?}"
        );
        let out = rewrapped(rewrap(&bytes, role));
        assert_eq!(ktx2_format(&out.ktx2), vk, "{code:?}");
        assert_eq!(out.format.vk_format(), vk, "{code:?}");
    }
}

#[test]
fn dxgi_maps_to_the_exact_vk_format_and_colour_space_follows_the_role() {
    // (DXGI value, role, declared sRGB, written vkFormat)
    let cases: [(u32, Role, Option<bool>, Format); 13] = [
        (70, COLOR, None, Format::BC1_RGBA_SRGB_BLOCK),
        (71, COLOR, Some(false), Format::BC1_RGBA_SRGB_BLOCK),
        (72, COLOR, Some(true), Format::BC1_RGBA_SRGB_BLOCK),
        (74, COLOR, Some(false), Format::BC2_SRGB_BLOCK),
        (75, COLOR, Some(true), Format::BC2_SRGB_BLOCK),
        (77, COLOR, Some(false), Format::BC3_SRGB_BLOCK),
        (78, COLOR, Some(true), Format::BC3_SRGB_BLOCK),
        (80, MASK_R, Some(false), Format::BC4_UNORM_BLOCK),
        (83, NORMAL_UP, Some(false), Format::BC5_UNORM_BLOCK),
        (98, COLOR, Some(false), Format::BC7_SRGB_BLOCK),
        (99, COLOR, Some(true), Format::BC7_SRGB_BLOCK),
        (98, Role::Packed, Some(false), Format::BC7_UNORM_BLOCK),
        // An sRGB-labelled normal is still sampled linear.
        (99, NORMAL_UP, Some(true), Format::BC7_UNORM_BLOCK),
    ];
    for (dxgi, role, declared, vk) in cases {
        let probe = dds(Pf::Dxgi(dxgi), 8, 8, 1, false, 0, &[]);
        // Size the payload from the parsed block format.
        let format = match read_source(&[probe.as_slice(), &[0; 64]].concat()) {
            Ok(Some(source)) => source.format,
            other => panic!("DXGI {dxgi}: {other:?}"),
        };
        let bytes = dds(
            Pf::Dxgi(dxgi),
            8,
            8,
            4,
            true,
            0,
            &chain(format, 8, 8, 4).concat(),
        );
        let source = read_source(&bytes).unwrap().unwrap();
        assert_eq!(source.declared_srgb, declared, "DXGI {dxgi}");
        let out = rewrapped(rewrap(&bytes, role));
        assert_eq!(ktx2_format(&out.ktx2), vk, "DXGI {dxgi}");
    }
}

#[test]
fn full_chain_levels_are_the_dds_payload_byte_for_byte() {
    for (code, format, role, width, height) in [
        (b"DXT5", BlockFormat::Bc3, COLOR, 64, 32),
        (b"DXT1", BlockFormat::Bc1, COLOR, 12, 8),
        (b"ATI2", BlockFormat::Bc5, NORMAL_UP, 32, 32),
        (b"ATI1", BlockFormat::Bc4, MASK_R, 4, 4),
    ] {
        let full = mip_count(width, height) as usize;
        let levels = chain(format, width, height, full);
        let bytes = dds(
            Pf::FourCc(code),
            width,
            height,
            u32::try_from(full).unwrap(),
            true,
            0,
            &levels.concat(),
        );
        let out = rewrapped(rewrap(&bytes, role));
        assert_eq!(
            (
                out.width,
                out.height,
                out.levels,
                out.shipped,
                out.as_shipped
            ),
            (
                width,
                height,
                u32::try_from(full).unwrap(),
                u32::try_from(full).unwrap(),
                false
            ),
            "{code:?} {width}x{height}"
        );
        assert_eq!(ktx2_levels(&out.ktx2), levels, "{code:?} {width}x{height}");
    }
}

#[test]
fn npot_level_sizes_round_up_to_whole_blocks() {
    // 12x8: 12x8, 6x4, 3x2, 1x1 -> 3x2, 2x1, 1x1, 1x1 blocks of 8 bytes.
    let out = rewrapped(rewrap(
        &dds(
            Pf::FourCc(b"DXT1"),
            12,
            8,
            4,
            true,
            0,
            &chain(BlockFormat::Bc1, 12, 8, 4).concat(),
        ),
        COLOR,
    ));
    let lens: Vec<usize> = ktx2_levels(&out.ktx2).iter().map(Vec::len).collect();
    assert_eq!(lens, [48, 16, 8, 8]);
}

#[test]
fn mip_count_is_read_without_the_flag() {
    let levels = chain(BlockFormat::Bc1, 16, 16, 5);
    let out = rewrapped(rewrap(
        &dds(Pf::FourCc(b"DXT1"), 16, 16, 5, false, 0, &levels.concat()),
        COLOR,
    ));
    assert_eq!((out.shipped, out.levels), (5, 5));
    assert_eq!(ktx2_levels(&out.ktx2), levels);
}

#[test]
fn a_single_level_source_is_filled_down_to_1x1_in_its_format() {
    let (width, height) = (64, 32);
    let cases: [(BlockFormat, Pf, Role, bool); 8] = [
        (BlockFormat::Bc1, Pf::FourCc(b"DXT1"), COLOR, false),
        // Punch-through alpha: the texpresso path.
        (
            BlockFormat::Bc1,
            Pf::FourCc(b"DXT1"),
            Role::Color {
                alpha: AlphaUse::Test(0.5),
            },
            true,
        ),
        (BlockFormat::Bc2, Pf::FourCc(b"DXT3"), COLOR, true),
        (BlockFormat::Bc3, Pf::FourCc(b"DXT5"), COLOR, true),
        (BlockFormat::Bc4, Pf::FourCc(b"ATI1"), MASK_R, false),
        (BlockFormat::Bc5, Pf::FourCc(b"ATI2"), NORMAL_UP, false),
        (BlockFormat::Bc7, Pf::Dxgi(99), COLOR, true),
        (BlockFormat::Bc7, Pf::Dxgi(98), Role::Packed, false),
    ];
    for (format, pf, role, alpha) in cases {
        let rgba = pattern(width, height, |x, y| {
            if alpha && (x / 8 + y / 8) % 2 == 0 {
                0
            } else {
                255
            }
        });
        let level0 = encode_level(format, &rgba, width, height, Bc7Quality::Fast);
        let out = rewrapped(rewrap(&dds(pf, width, height, 1, false, 0, &level0), role));
        let written = ktx2_levels(&out.ktx2);
        assert_eq!((out.shipped, out.levels), (1, 7), "{format:?}");
        assert_eq!(written[0], level0, "{format:?}: level 0 untouched");
        let lens: Vec<usize> = written.iter().map(Vec::len).collect();
        let expected: Vec<usize> = (0..7)
            .map(|level| {
                let (w, h) = level_size(width, height, level);
                level_len(format, w, h)
            })
            .collect();
        assert_eq!(lens, expected, "{format:?}");
        // Sanity: the 1x1 level decodes to finite, plausible colour (the pattern's mean red ~128).
        let last = decode_level(out.format, &written[6], 1, 1);
        // (BC5 has no red mean to speak of; a punch-through 1x1 may be the transparent texel.)
        if !matches!(format, BlockFormat::Bc5) && last[3] > 0 {
            assert!(
                (40..=215).contains(&last[0]),
                "{format:?}: 1x1 red {}",
                last[0]
            );
        }
    }
}

#[test]
fn a_truncated_chain_keeps_its_complete_levels_and_fills_the_rest() {
    let levels = chain(BlockFormat::Bc1, 32, 32, 6);
    // Header claims 6 levels; the file holds 3 and half of the 4th.
    let payload: Vec<u8> = levels[..3].concat().into_iter().chain([0; 4]).collect();
    let out = rewrapped(rewrap(
        &dds(Pf::FourCc(b"DXT1"), 32, 32, 6, true, 0, &payload),
        COLOR,
    ));
    assert_eq!((out.shipped, out.levels), (3, 6));
    assert_eq!(ktx2_levels(&out.ktx2)[..3], levels[..3]);
}

#[test]
fn uncompressed_and_other_containers_go_to_the_cook_path() {
    let bgra = dds(Pf::Bgra, 4, 4, 1, false, 0, &[0; 64]);
    assert_eq!(
        rewrap(&bgra, COLOR),
        Rewrap::Cook(CookReason::NotBlockCompressed)
    );
    let png_magic = [0x89, b'P', b'N', b'G', b'\r', b'\n', 0x1A, b'\n'];
    assert_eq!(
        rewrap(&png_magic, COLOR),
        Rewrap::Cook(CookReason::NotBlockCompressed)
    );
    // DX10 R8G8B8A8_UNORM (28).
    let rgba8 = dds(Pf::Dxgi(28), 4, 4, 1, false, 0, &[0; 64]);
    assert_eq!(
        rewrap(&rgba8, COLOR),
        Rewrap::Cook(CookReason::NotBlockCompressed)
    );
}

#[test]
fn a_format_that_doesnt_suit_the_role_is_cooked() {
    let bc4 = dds(Pf::FourCc(b"ATI1"), 4, 4, 1, false, 0, &[0; 8]);
    assert_eq!(
        rewrap(&bc4, COLOR),
        Rewrap::Cook(CookReason::FormatForRole(BlockFormat::Bc4))
    );
    let bc7 = dds(Pf::Dxgi(98), 4, 4, 1, false, 0, &[0x40; 16]);
    assert_eq!(
        rewrap(&bc7, NORMAL_DOWN),
        Rewrap::Cook(CookReason::NotBlockExact(RemapError::Bc7Invert))
    );
    let green = Role::Mask {
        channel: Channel::G,
        invert: false,
    };
    assert_eq!(
        rewrap(&bc4, green),
        Rewrap::Cook(CookReason::FormatForRole(BlockFormat::Bc4))
    );
    let dxt3 = dds(Pf::FourCc(b"DXT3"), 4, 4, 1, false, 0, &[0; 16]);
    assert_eq!(
        rewrap(&dxt3, green),
        Rewrap::Cook(CookReason::FormatForRole(BlockFormat::Bc2))
    );
    // A punch-through BC1 normal whose transparent texels can't take the Y flip.
    let punch = dds(
        Pf::FourCc(b"DXT1"),
        4,
        4,
        1,
        false,
        0,
        &[0, 0, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF],
    );
    assert_eq!(
        rewrap(&punch, NORMAL_DOWN),
        Rewrap::Cook(CookReason::NotBlockExact(RemapError::PunchThroughInvert))
    );
}

#[test]
fn layout_changes_rewrap_with_remapped_blocks() {
    // (FourCC, role, source format, written format, the map the blocks went through)
    let flip = ChannelMap([
        Src::Channel(Channel::R),
        Src::Inverted(Channel::G),
        Src::Unused,
        Src::Unused,
    ]);
    let no_alpha = ChannelMap([
        Src::Channel(Channel::R),
        Src::Channel(Channel::G),
        Src::Unused,
        Src::Unused,
    ]);
    let depth = ChannelMap([
        Src::Inverted(Channel::A),
        Src::Unused,
        Src::Unused,
        Src::Unused,
    ]);
    let cases = [
        (
            b"DXT1",
            NORMAL_DOWN,
            BlockFormat::Bc1,
            BlockFormat::Bc1,
            flip,
        ),
        (
            b"ATI2",
            NORMAL_DOWN,
            BlockFormat::Bc5,
            BlockFormat::Bc5,
            flip,
        ),
        (
            b"DXT5",
            NORMAL_DOWN,
            BlockFormat::Bc3,
            BlockFormat::Bc1,
            flip,
        ),
        (
            b"DXT3",
            NORMAL_UP,
            BlockFormat::Bc2,
            BlockFormat::Bc1,
            no_alpha,
        ),
        (
            b"DXT5",
            Role::height(Channel::A),
            BlockFormat::Bc3,
            BlockFormat::Bc4,
            depth,
        ),
    ];
    for (code, role, source, written, map) in cases {
        let levels = chain(source, 32, 32, 6);
        // Opaque-only BC1 blocks (c0 > c1) so the flip never meets punch-through texels.
        let levels: Vec<Vec<u8>> = if source == BlockFormat::Bc1 {
            levels
                .into_iter()
                .map(|mut level| {
                    for block in level.as_chunks_mut::<8>().0 {
                        block[1] |= 0x80;
                        block[3] &= 0x7F;
                    }
                    level
                })
                .collect()
        } else {
            levels
        };
        let bytes = dds(Pf::FourCc(code), 32, 32, 6, true, 0, &levels.concat());
        let out = rewrapped(rewrap(&bytes, role));
        assert_eq!(out.format, written, "{code:?}");
        assert!(out.remapped && !out.as_shipped, "{code:?}");
        assert_eq!((out.shipped, out.levels), (6, 6), "{code:?}");
        let remap = plan(source, &map).unwrap();
        let expected: Vec<Vec<u8>> = levels.iter().map(|l| remap.level(l).unwrap()).collect();
        assert_eq!(ktx2_levels(&out.ktx2), expected, "{code:?}");
    }
}

#[test]
fn unfit_sources_skip_with_their_reason() {
    let skip = |bytes: &[u8]| match rewrap(bytes, COLOR) {
        Rewrap::Skip(reason) => reason,
        other => panic!("expected a skip, got {other:?}"),
    };
    assert_eq!(
        skip(&dds(Pf::FourCc(b"DXT1"), 6, 6, 1, false, 0, &[0; 32])),
        SkipReason::Unaligned {
            width: 6,
            height: 6
        }
    );
    assert_eq!(
        skip(&dds(Pf::FourCc(b"DXT1"), 4, 4, 1, false, 0xFE00, &[0; 48])),
        SkipReason::Unsupported("cubemap")
    );
    assert_eq!(
        skip(&dds(Pf::Dxgi(95), 4, 4, 1, false, 0, &[0; 16])),
        SkipReason::Unsupported("BC6H (HDR)")
    );
    assert_eq!(
        skip(&dds(Pf::FourCc(b"BC5S"), 4, 4, 1, false, 0, &[0; 16])),
        SkipReason::Unsupported("signed BC4/BC5")
    );
    assert!(matches!(
        skip(&dds(Pf::FourCc(b"DXT1"), 8, 8, 1, false, 0, &[0; 31])),
        SkipReason::Malformed(_)
    ));
    assert!(matches!(skip(b"DDS short"), SkipReason::Malformed(_)));
}

#[test]
fn ktx2_sources_rewrap_and_flag_as_shipped() {
    let levels = chain(BlockFormat::Bc7Srgb, 16, 16, 5);
    let full = write_ktx2(BlockFormat::Bc7Srgb, 16, 16, &levels);
    let out = rewrapped(rewrap(&full, COLOR));
    assert!(out.as_shipped);
    assert_eq!(ktx2_levels(&out.ktx2), levels);
    // Declared UNORM, sampled as colour: rewritten as sRGB, so not as shipped.
    let unorm = write_ktx2(BlockFormat::Bc7, 16, 16, &levels);
    let out = rewrapped(rewrap(&unorm, COLOR));
    assert!(!out.as_shipped);
    assert_eq!(ktx2_format(&out.ktx2), Format::BC7_SRGB_BLOCK);
    // One level: filled.
    let rgba = pattern(16, 16, |_, _| 255);
    let one = write_ktx2(
        BlockFormat::Bc5,
        16,
        16,
        &[encode_level(
            BlockFormat::Bc5,
            &rgba,
            16,
            16,
            Bc7Quality::Basic,
        )],
    );
    let out = rewrapped(rewrap(&one, NORMAL_UP));
    assert_eq!((out.shipped, out.levels, out.as_shipped), (1, 5, false));
}

#[test]
fn encode_then_decode_round_trips_within_a_sanity_bound() {
    let (width, height) = (16, 12);
    let rgba = pattern(width, height, |x, _| if x < 8 { 255 } else { 96 });
    for format in [
        BlockFormat::Bc1,
        BlockFormat::Bc2,
        BlockFormat::Bc3,
        BlockFormat::Bc4,
        BlockFormat::Bc5,
        BlockFormat::Bc7,
    ] {
        let blocks = encode_level(format, &rgba, width, height, Bc7Quality::Basic);
        assert_eq!(blocks.len(), level_len(format, width, height), "{format:?}");
        let decoded = decode_level(format, &blocks, width, height);
        assert_eq!(decoded.len(), rgba.len(), "{format:?}");
        let channels = match format {
            BlockFormat::Bc4 => 1,
            BlockFormat::Bc5 => 2,
            _ => 3,
        };
        let worst = rgba
            .as_chunks::<4>()
            .0
            .iter()
            .zip(decoded.as_chunks::<4>().0)
            // BC1 punch-through: texels below alpha 128 decode transparent black by design.
            .filter(|(a, _)| format != BlockFormat::Bc1 || a[3] >= 128)
            .flat_map(|(a, b)| (0..channels).map(move |c| a[c].abs_diff(b[c])))
            .max()
            .unwrap();
        assert!(worst <= 48, "{format:?}: worst channel error {worst}");
    }
}
