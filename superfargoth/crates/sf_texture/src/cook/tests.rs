#![allow(
    clippy::cast_possible_truncation,
    clippy::cast_precision_loss,
    clippy::cast_sign_loss,
    reason = "test fixtures: small sizes"
)]

use super::*;
use crate::ktx2_out::{bc7_ktx2, bc7_ktx2_cutout, write_ktx2_bc7};
use ktx2::{ColorModel, Format, Header, LevelIndex, TransferFunction};

const ALL_FORMATS: [BlockFormat; 10] = [
    BlockFormat::Bc1,
    BlockFormat::Bc1Srgb,
    BlockFormat::Bc2,
    BlockFormat::Bc2Srgb,
    BlockFormat::Bc3,
    BlockFormat::Bc3Srgb,
    BlockFormat::Bc4,
    BlockFormat::Bc5,
    BlockFormat::Bc7,
    BlockFormat::Bc7Srgb,
];

/// A tiny deterministic generator (LCG) for fixtures.
struct Lcg(u32);

impl Lcg {
    fn next(&mut self) -> u32 {
        self.0 = self.0.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
        self.0 >> 8
    }

    /// Uniform in `-1.0..1.0`.
    fn signed(&mut self) -> f32 {
        self.next() as f32 / (1 << 23) as f32 - 1.0
    }
}

fn level_size(width: u32, height: u32, level: u32) -> (u32, u32) {
    ((width >> level).max(1), (height >> level).max(1))
}

/// Byte-encoded unit normals with Z > 0.3: `smooth` a slow wave, else noise.
fn normal_field(width: u32, height: u32, smooth: bool) -> Vec<u8> {
    let mut rng = Lcg(7);
    let encode = |value: f32| ((value * 0.5 + 0.5) * 255.0).round() as u8;
    (0..width * height)
        .flat_map(|i| {
            let (x, y) = ((i % width) as f32, (i / width) as f32);
            let (nx, ny) = if smooth {
                ((x * 0.11).sin() * 0.6, (y * 0.07).cos() * 0.6)
            } else {
                (rng.signed() * 0.7, rng.signed() * 0.7)
            };
            let z = (1.0 - nx * nx - ny * ny).sqrt();
            [encode(nx), encode(ny), encode(z), 255]
        })
        .collect()
}

fn decode_unit(byte: u8) -> f32 {
    f32::from(byte) / 255.0 * 2.0 - 1.0
}

/// BC4 block decode (Khronos Data Format spec §20.4): two endpoints, 3-bit indices, row-major.
fn decode_bc4_block(block: &[u8]) -> [u8; 16] {
    let (r0, r1) = (u32::from(block[0]), u32::from(block[1]));
    let palette: [u32; 8] = std::array::from_fn(|i| {
        let i = i as u32;
        match i {
            0 => r0,
            1 => r1,
            _ if r0 > r1 => ((8 - i) * r0 + (i - 1) * r1 + 3) / 7,
            2..=5 => ((6 - i) * r0 + (i - 1) * r1 + 2) / 5,
            6 => 0,
            _ => 255,
        }
    });
    let bits = block[2..8]
        .iter()
        .rev()
        .fold(0u64, |bits, &byte| bits << 8 | u64::from(byte));
    std::array::from_fn(|texel| palette[(bits >> (3 * texel) & 7) as usize] as u8)
}

/// Decoded channels (one plane per channel, `channels` of 8-byte BC4 halves per block) of a
/// BC4/BC5 level whose size is a multiple of 4.
fn decode_planes(blocks: &[u8], width: u32, height: u32, channels: usize) -> Vec<Vec<u8>> {
    let (w, h) = (width as usize, height as usize);
    let mut planes = vec![vec![0; w * h]; channels];
    for (index, block) in blocks.chunks_exact(8 * channels).enumerate() {
        let (bx, by) = (index % (w / 4), index / (w / 4));
        for (channel, plane) in planes.iter_mut().enumerate() {
            let texels = decode_bc4_block(&block[channel * 8..][..8]);
            for (texel, value) in texels.into_iter().enumerate() {
                plane[(by * 4 + texel / 4) * w + bx * 4 + texel % 4] = value;
            }
        }
    }
    planes
}

#[test]
fn vk_format_codes_and_block_bytes() {
    let table: Vec<(u32, usize, bool)> = ALL_FORMATS
        .iter()
        .map(|format| {
            (
                format.vk_format().value(),
                format.block_bytes(),
                format.is_srgb(),
            )
        })
        .collect();
    assert_eq!(
        table,
        [
            (133, 8, false),
            (134, 8, true),
            (135, 16, false),
            (136, 16, true),
            (137, 16, false),
            (138, 16, true),
            (139, 8, false),
            (141, 16, false),
            (145, 16, false),
            (146, 16, true),
        ]
    );
}

#[test]
fn level_lengths_round_partial_blocks_up() {
    for (width, height, blocks) in [(4, 4, 1), (8, 8, 4), (5, 3, 2), (1, 1, 1), (75, 37, 190)] {
        assert_eq!(level_len(BlockFormat::Bc4, width, height), blocks * 8);
        assert_eq!(level_len(BlockFormat::Bc1Srgb, width, height), blocks * 8);
        assert_eq!(level_len(BlockFormat::Bc5, width, height), blocks * 16);
        assert_eq!(level_len(BlockFormat::Bc7, width, height), blocks * 16);
    }
}

#[test]
fn role_formats() {
    let table = [
        (
            Role::Color {
                alpha: AlphaUse::None,
            },
            BlockFormat::Bc7Srgb,
        ),
        (
            Role::Color {
                alpha: AlphaUse::Test(0.5),
            },
            BlockFormat::Bc7Srgb,
        ),
        (
            Role::Color {
                alpha: AlphaUse::Blend,
            },
            BlockFormat::Bc7Srgb,
        ),
        (
            Role::Normal {
                source_y: SourceY::Down,
            },
            BlockFormat::Bc5,
        ),
        (
            Role::Mask {
                channel: Channel::R,
                invert: false,
            },
            BlockFormat::Bc4,
        ),
        (Role::height(Channel::A), BlockFormat::Bc4),
        (Role::Packed, BlockFormat::Bc7),
    ];
    for (role, format) in table {
        assert_eq!(format_for(role), format, "{role:?}");
    }
}

/// Every format through the container: header, level count, level lengths, offsets aligned to
/// the block size, smallest level first, DFD colour model, transfer and block bytes.
#[test]
fn write_ktx2_every_format_parses_back() {
    for format in ALL_FORMATS {
        for (width, height) in [(64, 32), (75, 37), (1, 1)] {
            let count = mip_count(width, height);
            let levels: Vec<Vec<u8>> = (0..count)
                .map(|level| {
                    let (w, h) = level_size(width, height, level);
                    vec![level as u8; level_len(format, w, h)]
                })
                .collect();
            let file = write_ktx2(format, width, height, &levels);
            let reader = ktx2::Reader::new(&file).expect("parses");
            let header = reader.header();
            assert_eq!(header.format, Some(format.vk_format()));
            assert_eq!((header.pixel_width, header.pixel_height), (width, height));
            assert_eq!(header.level_count, count);
            let lengths: Vec<usize> = reader.levels().map(|level| level.data.len()).collect();
            let expected: Vec<usize> = (0..count)
                .map(|level| {
                    let (w, h) = level_size(width, height, level);
                    level_len(format, w, h)
                })
                .collect();
            assert_eq!(lengths, expected, "{format:?} {width}x{height}");
            for (level, read) in reader.levels().enumerate() {
                assert!(read.data.iter().all(|&byte| byte == level as u8));
            }
            let offsets: Vec<u64> = file[Header::LENGTH..]
                .as_chunks::<{ LevelIndex::LENGTH }>()
                .0
                .iter()
                .take(count as usize)
                .map(|entry| LevelIndex::from_bytes(entry).byte_offset)
                .collect();
            assert!(
                offsets
                    .iter()
                    .all(|offset| offset % format.block_bytes() as u64 == 0)
            );
            assert!(offsets.windows(2).all(|pair| pair[0] > pair[1]));
            assert_eq!(offsets[0] as usize + expected[0], file.len());

            let basic = reader.basic_dfd().expect("basic DFD");
            let model = match format {
                BlockFormat::Bc1 | BlockFormat::Bc1Srgb => ColorModel::BC1A,
                BlockFormat::Bc2 | BlockFormat::Bc2Srgb => ColorModel::BC2,
                BlockFormat::Bc3 | BlockFormat::Bc3Srgb => ColorModel::BC3,
                BlockFormat::Bc4 => ColorModel::BC4,
                BlockFormat::Bc5 => ColorModel::BC5,
                BlockFormat::Bc7 | BlockFormat::Bc7Srgb => ColorModel::BC7,
            };
            assert_eq!(basic.color_model, Some(model));
            let transfer = if format.is_srgb() {
                TransferFunction::SRGB
            } else {
                TransferFunction::Linear
            };
            assert_eq!(basic.transfer_function, Some(transfer), "{format:?}");
            assert_eq!(basic.texel_block_dimensions.map(u8::from), [4, 4, 1, 1]);
            assert_eq!(usize::from(basic.bytes_planes[0]), format.block_bytes());
        }
    }
}

#[test]
fn bc7_wrapper_is_write_ktx2() {
    let levels: Vec<Vec<u8>> = (0..mip_count(64, 32))
        .map(|level| {
            let (w, h) = level_size(64, 32, level);
            vec![level as u8 * 3; level_len(BlockFormat::Bc7, w, h)]
        })
        .collect();
    for srgb in [true, false] {
        assert_eq!(
            write_ktx2_bc7(64, 32, srgb, &levels),
            write_ktx2(BlockFormat::bc7(srgb), 64, 32, &levels)
        );
    }
}

/// Every role through `cook_rgba`: its format, a full chain, and level lengths, at sizes that
/// are and aren't whole blocks.
#[test]
fn cook_every_role_parses_back() {
    let roles = [
        Role::Color {
            alpha: AlphaUse::None,
        },
        Role::Color {
            alpha: AlphaUse::Test(0.5),
        },
        Role::Normal {
            source_y: SourceY::Up,
        },
        Role::Mask {
            channel: Channel::G,
            invert: false,
        },
        Role::Packed,
    ];
    for role in roles {
        for (width, height) in [(64, 32), (30, 18)] {
            let rgba = normal_field(width, height, false);
            let file = cook_rgba(role, &rgba, width, height, Bc7Quality::Fast);
            let reader = ktx2::Reader::new(&file).expect("parses");
            let format = format_for(role);
            assert_eq!(reader.header().format, Some(format.vk_format()));
            assert_eq!(reader.header().level_count, mip_count(width, height));
            for (level, read) in reader.levels().enumerate() {
                let (w, h) = level_size(width, height, level as u32);
                assert_eq!(read.data.len(), level_len(format, w, h), "{role:?} {level}");
            }
        }
    }
    let file = cook_rgba(
        Role::Packed,
        &normal_field(8, 8, true),
        8,
        8,
        Bc7Quality::Fast,
    );
    assert_eq!(
        ktx2::Reader::new(&file).expect("parses").header().format,
        Some(Format::BC7_UNORM_BLOCK)
    );
}

/// Colour roles are the de-light path's encode: `Test` is `bc7_ktx2_cutout`, `Blend` is
/// `bc7_ktx2`, `None` is `bc7_ktx2` of the texture made opaque.
#[test]
fn color_roles_reuse_the_bc7_path() {
    let rgba: Vec<u8> = (0..32u32 * 16)
        .flat_map(|i| [(i % 251) as u8, (i % 13 * 19) as u8, 77, (i % 7 * 40) as u8])
        .collect();
    let cook = |alpha| cook_rgba(Role::Color { alpha }, &rgba, 32, 16, Bc7Quality::Fast);
    assert_eq!(
        cook(AlphaUse::Test(0.4)),
        bc7_ktx2_cutout(&rgba, 32, 16, true, Bc7Quality::Fast, Some(0.4))
    );
    assert_eq!(
        cook(AlphaUse::Blend),
        bc7_ktx2(&rgba, 32, 16, true, Bc7Quality::Fast)
    );
    let mut opaque = rgba.clone();
    for texel in opaque.as_chunks_mut::<4>().0 {
        texel[3] = 255;
    }
    assert_eq!(
        cook(AlphaUse::None),
        bc7_ktx2(&opaque, 32, 16, true, Bc7Quality::Fast)
    );
}

#[test]
fn flat_normal_stays_flat_at_every_level() {
    for (width, height) in [(64, 64), (75, 37)] {
        let rgba = [128, 128, 255, 255].repeat((width * height) as usize);
        for source_y in [SourceY::Up, SourceY::Down] {
            let chain = normal_mip_chain(&rgba, width, height, source_y);
            assert_eq!(chain.len() as u32, mip_count(width, height));
            let g = if source_y == SourceY::Up { 128 } else { 127 };
            for level in &chain {
                assert!(
                    level
                        .rgba
                        .as_chunks::<4>()
                        .0
                        .iter()
                        .all(|texel| (texel[0], texel[1]) == (128, g)),
                    "{source_y:?} {}x{}",
                    level.width,
                    level.height
                );
            }
        }
    }
}

#[test]
fn normals_are_unit_length_at_every_level() {
    let chain = normal_mip_chain(&normal_field(128, 96, false), 128, 96, SourceY::Up);
    for level in &chain {
        for texel in level.rgba.as_chunks::<4>().0 {
            let [x, y, z] = [texel[0], texel[1], texel[2]].map(decode_unit);
            let length = (x * x + y * y + z * z).sqrt();
            assert!(
                (length - 1.0).abs() <= 2.0 / 255.0,
                "length {length} at {}x{}",
                level.width,
                level.height
            );
        }
    }
}

#[test]
fn directx_sources_invert_green_exactly() {
    let rgba = normal_field(32, 32, false);
    let up = normal_mip_chain(&rgba, 32, 32, SourceY::Up);
    let down = normal_mip_chain(&rgba, 32, 32, SourceY::Down);
    for ((source, up), down) in rgba
        .as_chunks::<4>()
        .0
        .iter()
        .zip(up[0].rgba.as_chunks::<4>().0)
        .zip(down[0].rgba.as_chunks::<4>().0)
    {
        assert_eq!((up[0], up[1]), (source[0], source[1]));
        assert_eq!((down[0], down[1]), (source[0], 255 - source[1]));
        assert_eq!(down[2], up[2]);
    }
}

#[test]
fn opposing_normals_average_to_up() {
    // Left texel tilts fully +X, right fully -X: the 1x1 mip renormalises to +Z.
    let rgba = [[255, 128, 128, 255], [0, 127, 128, 255]].concat();
    let chain = normal_mip_chain(&rgba, 2, 1, SourceY::Up);
    let top = &chain[1].rgba;
    assert_eq!((top[0], top[2]), (128, 255));
}

#[test]
fn mask_keeps_one_channel_and_inverts_height() {
    let rgba: Vec<u8> = (0..16u32 * 8)
        .flat_map(|i| [9, (i * 3) as u8, 200, (i % 5 * 50) as u8])
        .collect();
    let green = mask_mip_chain(&rgba, 16, 8, Channel::G, false);
    let depth = mask_mip_chain(&rgba, 16, 8, Channel::A, true);
    assert_eq!(green.len() as u32, mip_count(16, 8));
    for (i, texel) in rgba.as_chunks::<4>().0.iter().enumerate() {
        assert_eq!(green[0].values[i], texel[1]);
        assert_eq!(depth[0].values[i], 255 - texel[3]);
    }
    let blue = mask_mip_chain(&rgba, 16, 8, Channel::B, false);
    assert!(
        blue.iter()
            .all(|level| level.values.iter().all(|&value| value == 200))
    );
}

#[test]
fn packed_maps_filter_alpha_as_data() {
    // Alpha 0 next to alpha 255 must not weight the colour (the colour path would).
    let rgba = [[0, 40, 0, 0], [255, 200, 100, 255]].concat();
    let chain = linear_mip_chain(&rgba, 2, 1);
    assert_eq!(chain[1].rgba, [128, 120, 50, 128]);
}

#[test]
fn bc5_round_trips_within_sanity() {
    let (width, height) = (64, 64);
    let rgba = normal_field(width, height, true);
    let rg: Vec<u8> = rgba
        .as_chunks::<4>()
        .0
        .iter()
        .flat_map(|texel| [texel[0], texel[1]])
        .collect();
    let planes = decode_planes(&encode_bc5(&rg, width, height), width, height, 2);
    let mut worst = 0;
    for (i, texel) in rgba.as_chunks::<4>().0.iter().enumerate() {
        worst = worst
            .max(planes[0][i].abs_diff(texel[0]))
            .max(planes[1][i].abs_diff(texel[1]));
    }
    assert!(worst <= 8, "BC5 worst channel error {worst}");
}

#[test]
fn bc4_round_trips_and_pads() {
    let values: Vec<u8> = (0..30u32 * 18).map(|i| (i % 30 * 8) as u8).collect();
    let blocks = encode_bc4(&values, 30, 18);
    assert_eq!(blocks.len(), level_len(BlockFormat::Bc4, 30, 18));
    // Decode the padded 32x20 grid; the real texels must match within sanity.
    let plane = &decode_planes(&blocks, 32, 20, 1)[0];
    for y in 0..18 {
        for x in 0..30 {
            let error = plane[y * 32 + x].abs_diff(values[y * 30 + x]);
            assert!(error <= 8, "BC4 error {error} at {x},{y}");
        }
    }
}

#[test]
#[ignore = "measurement: ms per MPix per format and per role cook, 1024x1024 (run with --release)"]
fn measure_encode_per_format() {
    let (width, height) = (1024, 1024);
    let mpix = f64::from(width * height) / 1e6;
    let rgba = normal_field(width, height, true);
    let rg: Vec<u8> = rgba
        .as_chunks::<4>()
        .0
        .iter()
        .flat_map(|texel| [texel[0], texel[1]])
        .collect();
    let r: Vec<u8> = rgba
        .as_chunks::<4>()
        .0
        .iter()
        .map(|texel| texel[0])
        .collect();
    let time = |name: &str, run: &mut dyn FnMut() -> usize| {
        let start = std::time::Instant::now();
        let bytes = run();
        let ms = start.elapsed().as_secs_f64() * 1e3;
        println!("{name:>24}: {:8.1} ms/MPix ({bytes} bytes)", ms / mpix);
    };
    time("encode BC7 basic", &mut || {
        encode_bc7(&rgba, width, height, Bc7Quality::Basic).len()
    });
    time("encode BC7 fast", &mut || {
        encode_bc7(&rgba, width, height, Bc7Quality::Fast).len()
    });
    time("encode BC5", &mut || encode_bc5(&rg, width, height).len());
    time("encode BC4", &mut || encode_bc4(&r, width, height).len());
    for (name, role) in [
        (
            "cook Color (basic)",
            Role::Color {
                alpha: AlphaUse::None,
            },
        ),
        (
            "cook Normal",
            Role::Normal {
                source_y: SourceY::Down,
            },
        ),
        (
            "cook Mask",
            Role::Mask {
                channel: Channel::R,
                invert: false,
            },
        ),
        ("cook Packed (basic)", Role::Packed),
    ] {
        time(name, &mut || {
            cook_rgba(role, &rgba, width, height, Bc7Quality::Basic).len()
        });
    }
}
