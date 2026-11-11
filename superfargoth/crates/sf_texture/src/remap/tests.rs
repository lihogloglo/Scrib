//! Exactness of the block remaps: each output block, decoded with an exact reference decoder
//! (palette entries scaled to integers, no rounding: the formats' definitions), equals the
//! channel-mapped decode of its input; plus the in-tree decoders (libsquish port, BC7) within
//! their rounding, and the BC7 permutation bit-exact against the normative integer decoder.

use super::*;
use crate::cook::Channel::{A, B, G, R};
use crate::rewrap::decode_level;
use crate::rewrap::tests::noise;

const fn ch(channel: Channel) -> Src {
    Src::Channel(channel)
}
const fn inv(channel: Channel) -> Src {
    Src::Inverted(channel)
}
const U: Src = Src::Unused;

/// 5- or 6-bit endpoint field → 8 bits by bit replication.
fn expand(value: u16, bits: u32) -> u32 {
    let value = u32::from(value);
    (value << (8 - bits)) | (value >> (2 * bits - 8))
}

/// Reference BC1 / BC2-BC3 colour decode, every value ×6 (exact: denominators 2 and 3).
/// `four`: BC2/BC3 colour (always four-colour). RGBA per texel; A is 0 or 6·255.
fn colour_ref(block: [u8; 8], four: bool) -> [[u32; 4]; 16] {
    let c0 = u16::from_le_bytes([block[0], block[1]]);
    let c1 = u16::from_le_bytes([block[2], block[3]]);
    let rgb = |c: u16| {
        [
            expand(c >> 11, 5),
            expand((c >> 5) & 0x3F, 6),
            expand(c & 0x1F, 5),
        ]
    };
    let (e0, e1) = (rgb(c0), rgb(c1));
    let opaque = 6 * 255;
    let palette: [[u32; 4]; 4] = if four || c0 > c1 {
        [
            [6 * e0[0], 6 * e0[1], 6 * e0[2], opaque],
            [6 * e1[0], 6 * e1[1], 6 * e1[2], opaque],
            std::array::from_fn(|c| {
                if c == 3 {
                    opaque
                } else {
                    2 * (2 * e0[c] + e1[c])
                }
            }),
            std::array::from_fn(|c| {
                if c == 3 {
                    opaque
                } else {
                    2 * (e0[c] + 2 * e1[c])
                }
            }),
        ]
    } else {
        [
            [6 * e0[0], 6 * e0[1], 6 * e0[2], opaque],
            [6 * e1[0], 6 * e1[1], 6 * e1[2], opaque],
            std::array::from_fn(|c| if c == 3 { opaque } else { 3 * (e0[c] + e1[c]) }),
            [0; 4],
        ]
    };
    let indices = u32::from_le_bytes([block[4], block[5], block[6], block[7]]);
    std::array::from_fn(|texel| palette[usize::try_from((indices >> (2 * texel)) & 3).unwrap()])
}

fn idx(i: usize) -> u32 {
    u32::try_from(i).unwrap()
}

/// Reference BC4 decode (BC3 alpha, BC5 halves), every value ×35 (denominators 5 and 7).
fn bc4_ref(block: [u8; 8]) -> [u32; 16] {
    let (a0, a1) = (u32::from(block[0]), u32::from(block[1]));
    let palette: [u32; 8] = if a0 > a1 {
        std::array::from_fn(|i| match i {
            0 => 35 * a0,
            1 => 35 * a1,
            _ => 5 * ((8 - idx(i)) * a0 + (idx(i) - 1) * a1),
        })
    } else {
        std::array::from_fn(|i| match i {
            0 => 35 * a0,
            1 => 35 * a1,
            6 => 0,
            7 => 35 * 255,
            _ => 7 * ((6 - idx(i)) * a0 + (idx(i) - 1) * a1),
        })
    };
    let mut bits = [0u8; 8];
    bits[..6].copy_from_slice(&block[2..]);
    let indices = u64::from_le_bytes(bits);
    std::array::from_fn(|texel| palette[usize::try_from((indices >> (3 * texel)) & 7).unwrap()])
}

/// The value `src` asks for from a reference texel scaled by `scale` (`None` when unused).
fn mapped(src: Src, texel: [u32; 4], scale: u32) -> Option<u32> {
    match src {
        Src::Unused => None,
        Src::Channel(channel) => Some(texel[channel_index(channel)]),
        Src::Inverted(channel) => Some(scale * 255 - texel[channel_index(channel)]),
    }
}

fn block8(bytes: &[u8]) -> [u8; 8] {
    bytes.try_into().unwrap()
}

/// `count` noise blocks of `size` bytes.
fn blocks(count: usize, size: usize, seed: u32) -> Vec<u8> {
    noise(count * size, seed)
}

/// Maps a BC1 must satisfy exactly (BC1 → BC1).
const BC1_MAPS: [ChannelMap; 4] = [
    ChannelMap([ch(R), inv(G), ch(B), ch(A)]),
    ChannelMap([ch(B), ch(G), ch(R), ch(A)]),
    ChannelMap([ch(B), inv(G), ch(R), ch(A)]),
    ChannelMap([inv(R), ch(G), inv(B), ch(A)]),
];

#[test]
fn bc1_y_flip_and_rb_swap_decode_exactly() {
    let source = blocks(4096, 8, 11);
    let (mut flipped_order, mut punch_through, mut checked) = (0, 0, 0);
    for map in BC1_MAPS {
        let remap = plan(BlockFormat::Bc1, &map).unwrap();
        assert_eq!(remap.output, BlockFormat::Bc1);
        for block in source.as_chunks::<8>().0 {
            let block = block8(block);
            let Ok(out) = remap.level(&block) else {
                // Only a three-colour block with transparent texels, under a complement.
                let c0 = u16::from_le_bytes([block[0], block[1]]);
                let c1 = u16::from_le_bytes([block[2], block[3]]);
                assert!(c0 <= c1 && map.0.iter().any(|s| matches!(s, Src::Inverted(_))));
                assert!(colour_ref(block, false).iter().any(|t| t[3] == 0));
                punch_through += 1;
                continue;
            };
            let out = block8(&out);
            let Op::Bc1(op) = remap.op else {
                panic!("a BC1 op");
            };
            let (c0, c1) = (
                u16::from_le_bytes([block[0], block[1]]),
                u16::from_le_bytes([block[2], block[3]]),
            );
            flipped_order += usize::from((op.apply565(c0) > op.apply565(c1)) != (c0 > c1));
            let (src, dst) = (colour_ref(block, false), colour_ref(out, false));
            for (s, d) in src.iter().zip(&dst) {
                for (o, &src_channel) in map.0.iter().enumerate() {
                    // Transparent black stays black (only reachable without a complement).
                    let want = if s[3] == 0 {
                        Some(0)
                    } else {
                        mapped(src_channel, *s, 6)
                    };
                    assert_eq!(Some(d[o]), want, "{map:?} {block:?} -> {out:?}");
                }
            }
            checked += 1;
        }
    }
    assert!(
        checked > 10_000 && punch_through > 0,
        "{checked} / {punch_through}"
    );
    assert!(flipped_order > 0, "mode-order fix-ups exercised");
}

#[test]
fn bc1_remap_matches_the_in_tree_decoder_within_rounding() {
    let source = blocks(256, 8, 5);
    let map = ChannelMap([ch(B), inv(G), ch(R), ch(A)]);
    let remap = plan(BlockFormat::Bc1, &map).unwrap();
    for block in source.as_chunks::<8>().0 {
        let Ok(out) = remap.level(block) else {
            continue;
        };
        let mut want = decode_level(BlockFormat::Bc1, block, 4, 4);
        map.apply_rgba(&mut want);
        let got = decode_level(BlockFormat::Bc1, &out, 4, 4);
        for (w, g) in want.as_chunks::<4>().0.iter().zip(got.as_chunks::<4>().0) {
            if w[3] == 0 {
                continue;
            }
            assert!(
                w.iter().zip(g).all(|(a, b)| a.abs_diff(*b) <= 1),
                "{w:?} vs {g:?}"
            );
        }
    }
}

#[test]
fn bc3_colour_and_alpha_remaps_decode_exactly() {
    let source = blocks(2048, 16, 21);
    for map in [
        ChannelMap([ch(R), inv(G), ch(B), ch(A)]),
        ChannelMap([ch(B), ch(G), ch(R), inv(A)]),
    ] {
        for (format, output) in [
            (BlockFormat::Bc3, BlockFormat::Bc3),
            (BlockFormat::Bc2, BlockFormat::Bc2),
        ] {
            let remap = plan(format, &map).unwrap();
            assert_eq!(remap.output, output);
            let out = remap.level(&source).unwrap();
            for (s, d) in source
                .as_chunks::<16>()
                .0
                .iter()
                .zip(out.as_chunks::<16>().0.iter())
            {
                let (sc, dc) = (
                    colour_ref(block8(&s[8..]), true),
                    colour_ref(block8(&d[8..]), true),
                );
                for (st, dt) in sc.iter().zip(&dc) {
                    for (value, src) in dt.iter().zip(&map.0).take(3) {
                        assert_eq!(Some(*value), mapped(*src, *st, 6), "{format:?} {map:?}");
                    }
                }
                let invert = matches!(map.0[3], Src::Inverted(_));
                if format == BlockFormat::Bc3 {
                    let (sa, da) = (bc4_ref(block8(&s[..8])), bc4_ref(block8(&d[..8])));
                    for (a, b) in sa.iter().zip(&da) {
                        assert_eq!(*b, if invert { 35 * 255 - a } else { *a });
                    }
                } else {
                    for (a, b) in s[..8].iter().zip(&d[..8]) {
                        assert_eq!(*b, if invert { !a } else { *a });
                    }
                }
            }
        }
    }
}

#[test]
fn bc2_and_bc3_drop_their_alpha_to_bc1_exactly() {
    let mut source = blocks(2048, 16, 31);
    // Equal endpoints (every palette entry the same colour) and an explicit c0 < c1.
    source[8..12].copy_from_slice(&[0x34, 0x12, 0x34, 0x12]);
    source[24..28].copy_from_slice(&[0x00, 0x10, 0xFF, 0xFF]);
    for format in [BlockFormat::Bc2, BlockFormat::Bc3] {
        for map in [
            ChannelMap([ch(R), ch(G), ch(B), U]),
            ChannelMap([ch(B), ch(G), ch(R), U]),
            ChannelMap([ch(R), inv(G), ch(B), U]),
        ] {
            let remap = plan(format, &map).unwrap();
            assert_eq!(remap.output, BlockFormat::Bc1, "{format:?}");
            let out = remap.level(&source).unwrap();
            assert_eq!(out.len(), source.len() / 2);
            for (s, d) in source
                .as_chunks::<16>()
                .0
                .iter()
                .zip(out.as_chunks::<8>().0.iter())
            {
                let (sc, dc) = (
                    colour_ref(block8(&s[8..]), true),
                    colour_ref(block8(d), false),
                );
                for (st, dt) in sc.iter().zip(&dc) {
                    assert_eq!(dt[3], 6 * 255, "opaque BC1 texel");
                    for (value, src) in dt.iter().zip(&map.0).take(3) {
                        assert_eq!(Some(*value), mapped(*src, *st, 6));
                    }
                }
            }
            // The in-tree decoder reads every texel opaque too.
            let decoded = decode_level(BlockFormat::Bc1, &out, 128, 256);
            assert!(decoded.as_chunks::<4>().0.iter().all(|t| t[3] == 255));
        }
    }
}

#[test]
fn bc4_complement_decodes_exactly_in_both_modes() {
    let source = blocks(4096, 8, 41);
    let remap = plan(BlockFormat::Bc4, &ChannelMap([inv(R), U, U, U])).unwrap();
    let out = remap.level(&source).unwrap();
    let mut modes = [0; 2];
    for (s, d) in source
        .as_chunks::<8>()
        .0
        .iter()
        .zip(out.as_chunks::<8>().0.iter())
    {
        modes[usize::from(s[0] > s[1])] += 1;
        for (a, b) in bc4_ref(block8(s)).iter().zip(&bc4_ref(block8(d))) {
            assert_eq!(*b, 35 * 255 - a, "{s:?} -> {d:?}");
        }
        assert_eq!(d[0] > d[1], s[0] > s[1], "mode kept");
    }
    assert!(modes[0] > 0 && modes[1] > 0);
}

#[test]
fn bc5_flips_swaps_and_splits_exactly() {
    let source = blocks(1024, 16, 51);
    let halves = |block: &[u8]| (bc4_ref(block8(&block[..8])), bc4_ref(block8(&block[8..])));
    // Y flip: G complemented, R kept.
    let flip = plan(BlockFormat::Bc5, &ChannelMap([ch(R), inv(G), U, U])).unwrap();
    let out = flip.level(&source).unwrap();
    for (s, d) in source
        .as_chunks::<16>()
        .0
        .iter()
        .zip(out.as_chunks::<16>().0.iter())
    {
        let ((sr, sg), (dr, dg)) = (halves(s), halves(d));
        assert_eq!(sr, dr);
        assert!(sg.iter().zip(&dg).all(|(a, b)| *b == 35 * 255 - a));
    }
    // Swap: the halves trade places.
    let swap = plan(BlockFormat::Bc5, &ChannelMap([ch(G), ch(R), U, U])).unwrap();
    let out = swap.level(&source).unwrap();
    for (s, d) in source
        .as_chunks::<16>()
        .0
        .iter()
        .zip(out.as_chunks::<16>().0.iter())
    {
        assert_eq!((&s[..8], &s[8..]), (&d[8..], &d[..8]));
    }
    // One half as BC4.
    let split = plan(BlockFormat::Bc5, &ChannelMap([inv(G), U, U, U])).unwrap();
    assert_eq!(split.output, BlockFormat::Bc4);
    let out = split.level(&source).unwrap();
    for (s, d) in source
        .as_chunks::<16>()
        .0
        .iter()
        .zip(out.as_chunks::<8>().0.iter())
    {
        let (_, sg) = halves(s);
        assert!(
            sg.iter()
                .zip(&bc4_ref(block8(d)))
                .all(|(a, b)| *b == 35 * 255 - a)
        );
    }
}

#[test]
fn bc3_alpha_splits_out_as_a_bc4_block() {
    let source = blocks(512, 16, 61);
    for invert in [false, true] {
        let src = if invert { inv(A) } else { ch(A) };
        let remap = plan(BlockFormat::Bc3, &ChannelMap([src, U, U, U])).unwrap();
        assert_eq!(remap.output, BlockFormat::Bc4);
        let out = remap.level(&source).unwrap();
        for (s, d) in source
            .as_chunks::<16>()
            .0
            .iter()
            .zip(out.as_chunks::<8>().0.iter())
        {
            if invert {
                let (a, b) = (bc4_ref(block8(&s[..8])), bc4_ref(block8(d)));
                assert!(a.iter().zip(&b).all(|(a, b)| *b == 35 * 255 - a));
            } else {
                assert_eq!(&s[..8], d, "the alpha block's bytes unchanged");
            }
        }
    }
}

/// A random BC7 block in `mode` (any bits after the mode prefix form a valid block).
fn bc7_block_in(mode: u32, seed: u32) -> [u8; 16] {
    let mut block: [u8; 16] = noise(16, seed).try_into().unwrap();
    let prefix = 1u8 << mode;
    block[0] = if mode == 7 {
        prefix
    } else {
        (block[0] << (mode + 1)) | prefix
    };
    block
}

#[test]
fn bc7_rgb_permutations_are_bit_exact_in_every_mode() {
    let perms: [[Channel; 3]; 6] = [
        [R, G, B],
        [B, G, R],
        [G, R, B],
        [R, B, G],
        [G, B, R],
        [B, R, G],
    ];
    let mut rotated = 0;
    for mode in 0..8 {
        for seed in 0..64 {
            let block = bc7_block_in(mode, seed * 8 + mode + 1);
            assert_eq!(block[0].trailing_zeros(), mode);
            if (4..=5).contains(&mode) && (u128::from_le_bytes(block) >> (mode + 1)) & 3 != 0 {
                rotated += 1;
            }
            let src = decode_level(BlockFormat::Bc7, &block, 4, 4);
            for perm in perms {
                let map = ChannelMap([ch(perm[0]), ch(perm[1]), ch(perm[2]), ch(A)]);
                let remap = plan(BlockFormat::Bc7, &map).unwrap();
                let out = remap.level(&block).unwrap();
                let got = decode_level(BlockFormat::Bc7, &out, 4, 4);
                let mut want = src.clone();
                map.apply_rgba(&mut want);
                assert_eq!(got, want, "mode {mode} perm {perm:?}");
            }
        }
    }
    assert!(rotated > 0, "rotations exercised");
}

#[test]
fn what_has_no_exact_block_operation_says_why() {
    let flip = ChannelMap([ch(R), inv(G), U, U]);
    assert_eq!(plan(BlockFormat::Bc7, &flip), Err(RemapError::Bc7Invert));
    assert_eq!(
        plan(BlockFormat::Bc7Srgb, &flip),
        Err(RemapError::Bc7Invert)
    );
    // A three-colour BC1 block (c0 <= c1) whose texels all use transparent black.
    let punch = [0x00, 0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF];
    let remap = plan(BlockFormat::Bc1, &ChannelMap([ch(R), inv(G), ch(B), ch(A)])).unwrap();
    assert_eq!(remap.level(&punch), Err(RemapError::PunchThroughInvert));
    // ...while a swap keeps black black.
    let swap = plan(BlockFormat::Bc1, &ChannelMap([ch(B), ch(G), ch(R), ch(A)])).unwrap();
    assert!(swap.level(&punch).is_ok());
    for (format, map) in [
        (BlockFormat::Bc1, ChannelMap([ch(G), ch(R), ch(B), ch(A)])),
        (BlockFormat::Bc1, ChannelMap([ch(R), ch(G), ch(B), inv(A)])),
        (BlockFormat::Bc2, ChannelMap([ch(A), U, U, U])),
        (BlockFormat::Bc3, ChannelMap([ch(A), ch(G), U, U])),
        (BlockFormat::Bc4, ChannelMap([ch(R), ch(G), U, U])),
        (BlockFormat::Bc5, ChannelMap([ch(R), ch(G), ch(B), U])),
        (BlockFormat::Bc7, ChannelMap([ch(A), ch(G), ch(B), U])),
        (BlockFormat::Bc7, ChannelMap([ch(R), ch(R), ch(B), U])),
    ] {
        assert_eq!(
            plan(format, &map),
            Err(RemapError::Unrepresentable),
            "{format:?} {map:?}"
        );
    }
}

#[test]
fn identity_maps_plan_identity_and_compose() {
    for format in [BlockFormat::Bc1, BlockFormat::Bc3, BlockFormat::Bc7] {
        assert!(plan(format, &ChannelMap::IDENTITY).unwrap().is_identity());
    }
    // Unused colour channels keep theirs: a normal map's Z.
    assert!(
        plan(BlockFormat::Bc1, &ChannelMap([ch(R), ch(G), U, U]))
            .unwrap()
            .is_identity()
    );
    assert!(
        plan(BlockFormat::Bc5, &ChannelMap([ch(R), ch(G), U, U]))
            .unwrap()
            .is_identity()
    );
    // A layout then a canonical map: ORM swap then a Y flip-style complement of G.
    let layout = ChannelMap([ch(B), ch(G), ch(R), ch(A)]);
    let next = ChannelMap([ch(R), inv(G), U, inv(A)]);
    assert_eq!(layout.then(next), ChannelMap([ch(B), inv(G), U, inv(A)]));
    assert_eq!(
        ChannelMap([inv(A), U, U, U]).then(ChannelMap([inv(R), ch(G), U, U])),
        ChannelMap([ch(A), U, U, U])
    );
}
