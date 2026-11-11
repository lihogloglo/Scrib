#![allow(
    clippy::cast_possible_truncation,
    reason = "test fixtures: small sizes"
)]

use super::*;
use block_compression::{BC7Settings, CompressionVariant, decode::decompress_blocks_as_rgba8};

fn flat(width: u32, height: u32, texel: [u8; 4]) -> Vec<u8> {
    texel.repeat(width as usize * height as usize)
}

/// Decoded RGBA8 of a BC7 level (`width`, `height` multiples of 4).
fn decode_bc7(blocks: &[u8], width: u32, height: u32) -> Vec<u8> {
    let mut rgba = vec![0; width as usize * height as usize * 4];
    decompress_blocks_as_rgba8(
        CompressionVariant::BC7(BC7Settings::opaque_basic()),
        width,
        height,
        blocks,
        &mut rgba,
    );
    rgba
}

fn sizes(levels: &[MipLevel]) -> Vec<(u32, u32)> {
    levels
        .iter()
        .map(|level| (level.width, level.height))
        .collect()
}

#[test]
fn mip_sizes_power_of_two() {
    let levels = mip_chain(&flat(512, 256, [10, 20, 30, 255]), 512, 256, true);
    assert_eq!(mip_count(512, 256), 10);
    assert_eq!(
        sizes(&levels),
        [
            (512, 256),
            (256, 128),
            (128, 64),
            (64, 32),
            (32, 16),
            (16, 8),
            (8, 4),
            (4, 2),
            (2, 1),
            (1, 1)
        ]
    );
    for level in &levels {
        assert_eq!(
            level.rgba.len(),
            level.width as usize * level.height as usize * 4
        );
    }
}

#[test]
fn mip_sizes_npot() {
    let levels = mip_chain(&flat(300, 200, [10, 20, 30, 255]), 300, 200, true);
    assert_eq!(mip_count(300, 200), 9);
    assert_eq!(
        sizes(&levels),
        [
            (300, 200),
            (150, 100),
            (75, 50),
            (37, 25),
            (18, 12),
            (9, 6),
            (4, 3),
            (2, 1),
            (1, 1)
        ]
    );
}

#[test]
fn uniform_colour_stays_uniform() {
    for srgb in [true, false] {
        for (width, height) in [(300, 200), (64, 64), (7, 3)] {
            let texel = [200, 117, 3, 140];
            for level in mip_chain(&flat(width, height, texel), width, height, srgb) {
                assert!(
                    level.rgba.as_chunks::<4>().0.iter().all(|t| *t == texel),
                    "{width}x{height} srgb {srgb}: {:?}",
                    &level.rgba[..4]
                );
            }
        }
    }
}

#[test]
fn averages_in_linear_light() {
    // Black and white average to linear 0.5, sRGB 188, not 128.
    let levels = mip_chain(&[0, 0, 0, 255, 255, 255, 255, 255], 2, 1, true);
    assert_eq!(levels[1].rgba, [188, 188, 188, 255]);
    let levels = mip_chain(&[0, 0, 0, 255, 255, 255, 255, 255], 2, 1, false);
    assert_eq!(levels[1].rgba, [128, 128, 128, 255]);
}

#[test]
fn transparent_texels_do_not_bleed() {
    // One opaque red texel among three transparent black ones: colour stays red, alpha averages.
    let rgba = [[255, 0, 0, 255], [0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0]].concat();
    let levels = mip_chain(&rgba, 2, 2, true);
    assert_eq!(levels[1].rgba, [255, 0, 0, 64]);
    // All transparent: the colours still average.
    let rgba = [[255, 255, 255, 0], [0, 0, 0, 0]].concat();
    assert_eq!(mip_chain(&rgba, 2, 1, false)[1].rgba, [128, 128, 128, 0]);
}

#[test]
fn odd_axis_blends_partial_texels() {
    // 3 -> 1: every texel weighs a third (a 2-tap filter would drop the last).
    let levels = mip_chain(
        &[0, 0, 0, 255, 0, 0, 0, 255, 255, 255, 255, 255],
        3,
        1,
        false,
    );
    assert_eq!(levels[1].rgba, [85, 85, 85, 255]);
    // 5 -> 2: weights 1, 1, 0.5 | 0.5, 1, 1 (of 2.5).
    let taps = taps(5, 2);
    assert_eq!(taps[0], [(0, 0.4), (1, 0.4), (2, 0.2)]);
    assert_eq!(taps[1], [(2, 0.2), (3, 0.4), (4, 0.4)]);
}

#[test]
fn bc7_flat_colour_round_trips() {
    for (texel, quality) in [
        ([200, 117, 3, 255], Bc7Quality::Basic),
        ([30, 90, 250, 255], Bc7Quality::Fast),
    ] {
        let blocks = encode_bc7(&flat(16, 8, texel), 16, 8, quality);
        assert_eq!(blocks.len(), bc7_level_len(16, 8));
        for decoded in decode_bc7(&blocks, 16, 8).as_chunks::<4>().0 {
            for (got, want) in decoded.iter().zip(texel) {
                assert!(got.abs_diff(want) <= 2, "{decoded:?} vs {texel:?}");
            }
        }
    }
}

#[test]
fn bc7_keeps_alpha_only_when_present() {
    // A horizontal alpha ramp survives through the alpha profile.
    let rgba: Vec<u8> = (0..8u8)
        .flat_map(|_| (0..8u8).flat_map(|x| [90, 60, 30, x * 36]))
        .collect();
    let decoded = decode_bc7(&encode_bc7(&rgba, 8, 8, Bc7Quality::Basic), 8, 8);
    for (got, want) in decoded
        .as_chunks::<4>()
        .0
        .iter()
        .zip(rgba.as_chunks::<4>().0.iter())
    {
        assert!(
            got[3].abs_diff(want[3]) <= 4,
            "alpha {} vs {}",
            got[3],
            want[3]
        );
    }
    let decoded = decode_bc7(
        &encode_bc7(&flat(8, 8, [90, 60, 30, 255]), 8, 8, Bc7Quality::Basic),
        8,
        8,
    );
    assert!(decoded.as_chunks::<4>().0.iter().all(|t| t[3] == 255));
}

#[test]
fn bc7_pads_partial_blocks() {
    // 6x3 needs 2x1 blocks; the decoded texels inside the texture keep the colour.
    let texel = [10, 200, 100, 255];
    let blocks = encode_bc7(&flat(6, 3, texel), 6, 3, Bc7Quality::Fast);
    assert_eq!(blocks.len(), 2 * BC7_BLOCK_BYTES);
    let decoded = decode_bc7(&blocks, 8, 4);
    for y in 0..3 {
        for x in 0..6 {
            let got = &decoded[(y * 8 + x) * 4..][..4];
            assert!(
                got.iter().zip(texel).all(|(g, w)| g.abs_diff(w) <= 2),
                "({x}, {y}): {got:?}"
            );
        }
    }
}

#[test]
fn ktx2_parses_back() {
    for (width, height, srgb) in [(512, 256, true), (300, 200, false)] {
        let file = bc7_ktx2(
            &flat(width, height, [120, 80, 40, 255]),
            width,
            height,
            srgb,
            Bc7Quality::Fast,
        );
        let reader = ktx2::Reader::new(&file).expect("parses");
        let header = reader.header();
        let format = if srgb {
            Format::BC7_SRGB_BLOCK
        } else {
            Format::BC7_UNORM_BLOCK
        };
        assert_eq!(header.format, Some(format));
        assert_eq!(
            (header.pixel_width, header.pixel_height, header.pixel_depth),
            (width, height, 0)
        );
        assert_eq!(
            (header.layer_count, header.face_count, header.type_size),
            (0, 1, 1)
        );
        assert_eq!(header.level_count, mip_count(width, height));
        assert_eq!(header.supercompression_scheme, None);

        let expected: Vec<usize> = (0..header.level_count)
            .map(|level| bc7_level_len((width >> level).max(1), (height >> level).max(1)))
            .collect();
        let lengths: Vec<usize> = reader.levels().map(|level| level.data.len()).collect();
        assert_eq!(lengths, expected);
        assert!(
            reader
                .levels()
                .all(|level| level.uncompressed_byte_length == level.data.len() as u64)
        );

        // Level offsets: multiples of 16, smallest level first, nothing after level 0.
        let offsets: Vec<u64> = file[Header::LENGTH..]
            .as_chunks::<{ LevelIndex::LENGTH }>()
            .0
            .iter()
            .take(header.level_count as usize)
            .map(|entry| LevelIndex::from_bytes(entry).byte_offset)
            .collect();
        assert!(offsets.iter().all(|offset| offset % 16 == 0));
        assert!(offsets.windows(2).all(|pair| pair[0] > pair[1]));
        assert_eq!(offsets[0] as usize + expected[0], file.len());

        let basic = reader.basic_dfd().expect("basic DFD");
        assert_eq!(basic.color_model, Some(ktx2::ColorModel::BC7));
        let transfer = if srgb {
            ktx2::TransferFunction::SRGB
        } else {
            ktx2::TransferFunction::Linear
        };
        assert_eq!(basic.transfer_function, Some(transfer));
        assert_eq!(basic.texel_block_dimensions.map(u8::from), [4, 4, 1, 1]);
        assert_eq!(basic.bytes_planes[0], 16);

        let kvd: Vec<(&str, &[u8])> = reader.key_value_data().collect();
        assert_eq!(kvd, [("KTXwriter", format!("{KTX_WRITER}\0").as_bytes())]);
    }
}

#[test]
fn ktx2_level_data_is_the_encoded_levels() {
    let rgba: Vec<u8> = (0..64u32 * 32)
        .flat_map(|i| [(i % 251) as u8, (i % 13 * 19) as u8, 77, 255])
        .collect();
    let chain = mip_chain(&rgba, 64, 32, true);
    let levels: Vec<Vec<u8>> = chain
        .iter()
        .map(|level| encode_bc7(&level.rgba, level.width, level.height, Bc7Quality::Fast))
        .collect();
    let file = write_ktx2_bc7(64, 32, true, &levels);
    assert_eq!(file, bc7_ktx2(&rgba, 64, 32, true, Bc7Quality::Fast));
    let reader = ktx2::Reader::new(&file).expect("parses");
    for (read, written) in reader.levels().zip(&levels) {
        assert_eq!(read.data, written.as_slice());
    }
}

/// A cutout leaf, `width` x `height`: a rotated, off-centre ellipse with a midrib slit and a few
/// round holes, green with a little colour noise. `soft`: an anti-aliased edge (a 2-texel alpha
/// ramp, jittered so edge texels rarely share a value); else hard-edged (alpha 0 or 255).
#[allow(
    clippy::cast_precision_loss,
    clippy::cast_sign_loss,
    clippy::cast_possible_wrap,
    reason = "test fixture: small sizes"
)]
fn leaf(width: u32, height: u32, soft: bool) -> Vec<u8> {
    let (w, h) = (width as f32, height as f32);
    let (sin, cos) = 0.6_f32.sin_cos();
    let holes = [(0.31, 0.42, 0.05), (0.62, 0.55, 0.04), (0.48, 0.30, 0.03)];
    let mut rgba = Vec::with_capacity(width as usize * height as usize * 4);
    for y in 0..height {
        for x in 0..width {
            let (px, py) = ((x as f32 + 0.5) / w, (y as f32 + 0.5) / h);
            let (dx, dy) = (px - 0.471, py - 0.483);
            let (u, v) = (dx * cos + dy * sin, -dx * sin + dy * cos);
            // Approximate signed distance in texels, positive inside.
            let ellipse = (1.0 - ((u / 0.43).powi(2) + (v / 0.19).powi(2)).sqrt()) * 0.19 * h;
            let rib = (v.abs() - 0.012) * h;
            let hole = holes
                .iter()
                .map(|&(hx, hy, r)| ((px - hx).hypot(py - hy) - r) * w)
                .fold(f32::INFINITY, f32::min);
            let distance = ellipse.min(rib).min(hole);
            let noise = (x * 7 + y * 13) % 23;
            let alpha = if soft {
                let ramp = unit_to_byte(0.5 + 0.5 * distance);
                if ramp == 0 || ramp == 255 {
                    ramp
                } else {
                    (i32::from(ramp) + noise as i32 - 11).clamp(1, 254) as u8
                }
            } else if distance >= 0.0 {
                255
            } else {
                0
            };
            rgba.extend_from_slice(&[40 + noise as u8, 120 + noise as u8, 30, alpha]);
        }
    }
    rgba
}

/// Texels of a level with `alpha / 255 >= cutoff`.
fn passing(level: &MipLevel, cutoff: f32) -> usize {
    coverage(&level.rgba, cutoff, 1.0)
}

fn texels(level: &MipLevel) -> usize {
    level.width as usize * level.height as usize
}

/// The pass count of a `texels`-texel level closest to the top level's fraction.
fn wanted(top: &MipLevel, cutoff: f32, texels_here: usize) -> usize {
    (2 * passing(top, cutoff) * texels_here + texels(top)) / (2 * texels(top))
}

const CUTOFFS: [f32; 4] = [0.3, 0.5, 133.0 / 255.0, 0.75];

#[test]
#[allow(clippy::cast_precision_loss, reason = "test fixture: small sizes")]
fn coverage_is_preserved_on_every_level() {
    let (width, height) = (96, 80);
    let rgba = leaf(width, height, true);
    for cutoff in CUTOFFS {
        let plain = mip_chain(&rgba, width, height, true);
        let mut kept = plain.clone();
        preserve_coverage(&mut kept, cutoff);
        assert_eq!(kept[0], plain[0], "the top level is the source");
        let top = &kept[0];
        let fraction = |count: usize, n: usize| count as f64 / n as f64;
        let top_fraction = fraction(passing(top, cutoff), texels(top));
        assert!(
            (0.15..0.6).contains(&top_fraction),
            "fixture coverage {top_fraction}"
        );
        let mut plain_worst = 0.0_f64;
        for (level, before) in kept.iter().zip(&plain).skip(1) {
            let n = texels(level);
            // Within one texel-fraction step of the top level's coverage.
            let error = (fraction(passing(level, cutoff), n) - top_fraction).abs();
            assert!(
                error * n as f64 <= 1.0,
                "cutoff {cutoff}, {}x{}: coverage off by {error} ({} texels)",
                level.width,
                level.height,
                error * n as f64
            );
            plain_worst =
                plain_worst.max((fraction(passing(before, cutoff), n) - top_fraction).abs());
            // Colour is untouched, only alpha moves.
            let colour = |level: &MipLevel| -> Vec<[u8; 3]> {
                level
                    .rgba
                    .as_chunks::<4>()
                    .0
                    .iter()
                    .map(|t| [t[0], t[1], t[2]])
                    .collect()
            };
            assert_eq!(colour(level), colour(before));
        }
        // The fixture is meaningful: plain averaging does lose (or gain) coverage.
        assert!(
            plain_worst > 0.05,
            "cutoff {cutoff}: plain mips off by only {plain_worst}"
        );
    }
}

#[test]
fn hard_edged_coverage_is_the_closest_reachable() {
    // A hard-edged cutout's mips hold few distinct alphas, so ties can make the exact count
    // unreachable; the scale still lands on the reachable count nearest the top level's.
    let (width, height) = (96, 80);
    let rgba = leaf(width, height, false);
    for cutoff in CUTOFFS {
        let plain = mip_chain(&rgba, width, height, true);
        let mut kept = plain.clone();
        preserve_coverage(&mut kept, cutoff);
        for (level, before) in kept.iter().zip(&plain).skip(1) {
            let want = wanted(&kept[0], cutoff, texels(level));
            // Scaling keeps alpha order, so a reachable count is "every texel at or above a
            // source alpha" (or none).
            let reachable = (0..=256_u32).map(|threshold| {
                before
                    .rgba
                    .as_chunks::<4>()
                    .0
                    .iter()
                    .filter(|t| u32::from(t[3]) >= threshold)
                    .count()
            });
            let best = reachable
                .map(|count| count.abs_diff(want))
                .min()
                .unwrap_or(0);
            assert_eq!(
                passing(level, cutoff).abs_diff(want),
                best,
                "cutoff {cutoff}, {}x{}",
                level.width,
                level.height
            );
        }
    }
}

#[test]
fn opaque_texture_is_unchanged_by_coverage() {
    let mut rgba = leaf(64, 48, true);
    for texel in rgba.as_chunks_mut::<4>().0 {
        texel[3] = 255;
    }
    let plain = mip_chain(&rgba, 64, 48, true);
    let mut kept = plain.clone();
    preserve_coverage(&mut kept, 0.5);
    assert_eq!(kept, plain);
    assert_eq!(
        bc7_ktx2_cutout(&rgba, 64, 48, true, Bc7Quality::Fast, Some(0.5)),
        bc7_ktx2(&rgba, 64, 48, true, Bc7Quality::Fast)
    );
}

#[test]
fn no_cutoff_is_the_plain_encode() {
    let rgba = leaf(64, 32, true);
    // The pipeline `bc7_ktx2` ran before coverage existed, spelled out.
    let levels: Vec<Vec<u8>> = mip_chain(&rgba, 64, 32, true)
        .iter()
        .map(|level| encode_bc7(&level.rgba, level.width, level.height, Bc7Quality::Fast))
        .collect();
    let before = write_ktx2_bc7(64, 32, true, &levels);
    assert_eq!(
        bc7_ktx2_cutout(&rgba, 64, 32, true, Bc7Quality::Fast, None),
        before
    );
    assert_eq!(bc7_ktx2(&rgba, 64, 32, true, Bc7Quality::Fast), before);
    // And a cutoff does change a cutout's file.
    assert_ne!(
        bc7_ktx2_cutout(&rgba, 64, 32, true, Bc7Quality::Fast, Some(0.5)),
        before
    );
}

#[test]
fn block_alignment() {
    assert!(is_block_aligned(300, 200));
    assert!(!is_block_aligned(30, 30));
    assert!(!is_block_aligned(0, 4));
}

#[test]
#[ignore = "measurement: ms to mip and BC7-encode one 512x512 RGBA (run with --release)"]
fn measure_encode_512() {
    let mut seed = 12_345u32;
    let rgba: Vec<u8> = (0..512u32 * 512)
        .flat_map(|i| {
            seed = seed.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
            let noise = (seed >> 27) as u8;
            [
                ((i % 512) / 2) as u8 + noise,
                ((i / 512) / 2) as u8 + noise,
                90 + noise,
                255,
            ]
        })
        .collect();
    let mut alpha = rgba.clone();
    for texel in alpha.as_chunks_mut::<4>().0 {
        texel[3] = texel[0];
    }
    for (name, data) in [("opaque", &rgba), ("alpha", &alpha)] {
        for quality in [Bc7Quality::Fast, Bc7Quality::Basic] {
            let start = std::time::Instant::now();
            let chain = mip_chain(data, 512, 512, true);
            let mips = start.elapsed();
            let levels: Vec<Vec<u8>> = chain
                .iter()
                .map(|level| encode_bc7(&level.rgba, level.width, level.height, quality))
                .collect();
            let encoded = start.elapsed();
            let file = write_ktx2_bc7(512, 512, true, &levels);
            let total = start.elapsed();
            println!(
                "{name} {quality:?}: mips {:.1} ms, BC7 {:.1} ms, total {:.1} ms, {} bytes",
                mips.as_secs_f64() * 1e3,
                encoded.saturating_sub(mips).as_secs_f64() * 1e3,
                total.as_secs_f64() * 1e3,
                file.len()
            );
        }
    }
}

/// FNV-1a 64 of a byte string: a compact golden for whole files.
fn fnv1a(bytes: &[u8]) -> u64 {
    bytes.iter().fold(0xcbf2_9ce4_8422_2325, |hash, &byte| {
        (hash ^ u64::from(byte)).wrapping_mul(0x0100_0000_01b3)
    })
}

/// Synthetic BC7 levels (not real blocks: the container only copies them) for `width` x `height`.
fn synthetic_bc7_levels(width: u32, height: u32, count: u32) -> Vec<Vec<u8>> {
    (0..count)
        .map(|level| {
            let len = bc7_level_len((width >> level).max(1), (height >> level).max(1));
            (0..len)
                .map(|i| (i * 7 + level as usize * 31) as u8)
                .collect()
        })
        .collect()
}

/// Goldens taken from `write_ktx2_bc7` and `bc7_ktx2` before the container was generalised to every
/// block format (textures.md T1): the de-light products must not change.
#[test]
fn bc7_container_is_byte_identical_to_the_golden() {
    let cases = [
        (64, 32, true, 7),
        (300, 200, false, 9),
        (300, 200, true, 1),
        (1, 1, true, 1),
    ];
    let hashes: Vec<(usize, u64)> = cases
        .iter()
        .map(|&(w, h, srgb, count)| {
            let file = write_ktx2_bc7(w, h, srgb, &synthetic_bc7_levels(w, h, count));
            (file.len(), fnv1a(&file))
        })
        .collect();
    let rgba: Vec<u8> = (0..64u32 * 32)
        .flat_map(|i| [(i % 251) as u8, (i % 13 * 19) as u8, 77, (i % 7 * 40) as u8])
        .collect();
    let encoded = bc7_ktx2_cutout(&rgba, 64, 32, true, Bc7Quality::Fast, Some(0.5));
    assert_eq!(
        hashes,
        [
            (3104, 0x2097_a90a_ca49_a28b),
            (81040, 0x28dd_807b_17e0_f132),
            (60192, 0x021b_d06b_ba18_24f4),
            (208, 0xa571_85de_e7b3_bc3f),
        ]
    );
    assert_eq!(
        (encoded.len(), fnv1a(&encoded)),
        (3104, 0xb4d0_4dd4_8a25_6c9f)
    );
}
