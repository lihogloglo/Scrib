use super::*;
use crate::cook::Channel::{A, B, G, R};
use crate::ktx2_out::{BlockFormat, level_len, mip_count};
use crate::remap::{RemapError, Src};
use crate::rewrap::tests::{Pf, dds, noise};
use crate::rewrap::{CookReason, Rewrapped, decode_level, read_source};

const fn ch(channel: Channel) -> Src {
    Src::Channel(channel)
}

/// A profile with every role, in a made-up convention (DirectX Y, metal in R).
fn test_profile() -> PackProfile {
    let identity = ChannelMap::IDENTITY;
    let rule = |suffix: &str, role, layout| SuffixRule {
        suffix: suffix.into(),
        role,
        layout,
    };
    PackProfile {
        name: "test-convention".into(),
        suffixes: vec![
            rule("", PackRole::Albedo, identity),
            rule("_nh", PackRole::NormalHeight, identity),
            rule(
                "_n",
                PackRole::Normal,
                ChannelMap([ch(R), ch(G), ch(B), Src::Unused]),
            ),
            rule(
                "_spec",
                PackRole::Orm,
                ChannelMap([ch(B), ch(G), ch(R), Src::Unused]),
            ),
            rule("_diffusespec", PackRole::TerrainAlbedoRoughness, identity),
        ],
        normal_y: SourceY::Down,
        height_neutral: 0.5,
        quirks: Vec::new(),
    }
}

#[test]
fn a_profile_round_trips_through_ron() {
    let profile = test_profile();
    let text = ron::ser::to_string_pretty(&profile, ron::ser::PrettyConfig::default()).unwrap();
    let back: PackProfile = ron::from_str(&text).unwrap();
    assert_eq!(back, profile);
    // The hand-written form adapters ship (quirks may be left out).
    let written = r#"(
        name: "test-convention",
        suffixes: [
            (suffix: "", role: Albedo, layout: (Channel(R), Channel(G), Channel(B), Channel(A))),
            (suffix: "_nh", role: NormalHeight, layout: (Channel(R), Channel(G), Channel(B), Channel(A))),
            (suffix: "_n", role: Normal, layout: (Channel(R), Channel(G), Channel(B), Unused)),
            (suffix: "_spec", role: Orm, layout: (Channel(B), Channel(G), Channel(R), Unused)),
            (suffix: "_diffusespec", role: TerrainAlbedoRoughness, layout: (Channel(R), Channel(G), Channel(B), Channel(A))),
        ],
        normal_y: Down,
        height_neutral: 0.5,
    )"#;
    assert_eq!(ron::from_str::<PackProfile>(written).unwrap(), profile);
}

#[test]
fn stems_route_to_their_rule_and_base() {
    let profile = test_profile();
    let role = |stem: &'static str| profile.rule_for(stem).map(|(base, rule)| (base, rule.role));
    assert_eq!(role("tx_wall"), Some(("tx_wall", PackRole::Albedo)));
    assert_eq!(role("tx_wall_n"), Some(("tx_wall", PackRole::Normal)));
    assert_eq!(
        role("TX_Wall_NH"),
        Some(("TX_Wall", PackRole::NormalHeight))
    );
    assert_eq!(role("tx_wall_spec"), Some(("tx_wall", PackRole::Orm)));
    // The longest suffix wins: `_diffusespec` over `_spec`.
    assert_eq!(
        role("tx_ai_grass_diffusespec"),
        Some(("tx_ai_grass", PackRole::TerrainAlbedoRoughness))
    );
    let companions: Vec<String> = profile
        .companions("tx_wall")
        .map(|(stem, _)| stem)
        .collect();
    assert_eq!(
        companions,
        [
            "tx_wall_nh",
            "tx_wall_n",
            "tx_wall_spec",
            "tx_wall_diffusespec"
        ]
    );
    let no_base = PackProfile {
        suffixes: profile.suffixes[1..].to_vec(),
        ..profile.clone()
    };
    assert_eq!(no_base.rule_for("tx_wall"), None);
}

fn rule(profile: &PackProfile, role: PackRole) -> &SuffixRule {
    profile
        .suffixes
        .iter()
        .find(|rule| rule.role == role)
        .unwrap()
}

/// A product's slot and its written format + whether it was remapped, or why it cooks.
type Outcome = (Slot, Result<(BlockFormat, bool), CookReason>);

fn rewrapped(result: &Rewrap) -> &Rewrapped {
    match result {
        Rewrap::Rewrapped(rewrapped) => rewrapped,
        other => panic!("expected a rewrap, got {other:?}"),
    }
}

/// A 16x16 DDS with its full chain of noise blocks (BC1: opaque four-colour blocks, as a
/// normal or packed map is encoded).
fn noise_dds(pf: Pf, format: BlockFormat) -> (Vec<u8>, Vec<Vec<u8>>) {
    let levels: Vec<Vec<u8>> = (0..5)
        .map(|level| {
            let size = (16u32 >> level).max(1);
            let mut blocks = noise(level_len(format, size, size), 90 + level);
            if format == BlockFormat::Bc1 {
                for block in blocks.as_chunks_mut::<8>().0 {
                    block[1] |= 0x80;
                    block[3] &= 0x7F;
                }
            }
            blocks
        })
        .collect();
    (dds(pf, 16, 16, 5, true, 0, &levels.concat()), levels)
}

#[test]
fn apply_routes_each_suffix_to_its_block_operation() {
    use BlockFormat as F;
    let profile = test_profile();
    let alpha = AlphaUse::None;
    let summary = |rule: &SuffixRule, bytes: &[u8]| -> Vec<Outcome> {
        apply(&profile, rule, bytes, alpha)
            .into_iter()
            .map(|product| {
                let outcome = match &product.result {
                    Rewrap::Rewrapped(out) => Ok((out.format, out.remapped)),
                    Rewrap::Cook(reason) => Err(*reason),
                    Rewrap::Skip(reason) => panic!("{reason:?}"),
                };
                (product.slot, outcome)
            })
            .collect()
    };
    let (dxt1, _) = noise_dds(Pf::FourCc(b"DXT1"), BlockFormat::Bc1);
    let (dxt5, _) = noise_dds(Pf::FourCc(b"DXT5"), BlockFormat::Bc3);
    let (bc7, _) = noise_dds(Pf::Dxgi(98), BlockFormat::Bc7);
    let (ati2, _) = noise_dds(Pf::FourCc(b"ATI2"), BlockFormat::Bc5);
    let albedo = rule(&profile, PackRole::Albedo);
    let normal = rule(&profile, PackRole::Normal);
    let height = rule(&profile, PackRole::NormalHeight);
    let orm = rule(&profile, PackRole::Orm);
    let terrain = rule(&profile, PackRole::TerrainAlbedoRoughness);
    assert_eq!(
        summary(albedo, &dxt5),
        [(Slot::BaseColor, Ok((F::Bc3Srgb, false)))]
    );
    assert_eq!(
        summary(albedo, &bc7),
        [(Slot::BaseColor, Ok((F::Bc7Srgb, false)))]
    );
    // DirectX normals: G complemented on the blocks.
    assert_eq!(summary(normal, &dxt1), [(Slot::Normal, Ok((F::Bc1, true)))]);
    assert_eq!(summary(normal, &ati2), [(Slot::Normal, Ok((F::Bc5, true)))]);
    assert_eq!(
        summary(normal, &bc7),
        [(
            Slot::Normal,
            Err(CookReason::NotBlockExact(RemapError::Bc7Invert))
        )]
    );
    // Normal + height: split into a BC1 normal and a BC4 depth.
    assert_eq!(
        summary(height, &dxt5),
        [
            (Slot::Normal, Ok((F::Bc1, true))),
            (Slot::Depth, Ok((F::Bc4, true)))
        ]
    );
    // Metal-in-R packed map: R/B swapped to glTF ORM; DXT5's unused alpha dropped.
    assert_eq!(summary(orm, &dxt1), [(Slot::Orm, Ok((F::Bc1, true)))]);
    assert_eq!(summary(orm, &dxt5), [(Slot::Orm, Ok((F::Bc1, true)))]);
    assert_eq!(summary(orm, &bc7), [(Slot::Orm, Ok((F::Bc7, true)))]);
    assert_eq!(
        summary(terrain, &dxt5),
        [(Slot::TerrainLayer, Ok((F::Bc3Srgb, false)))]
    );
    // The products' maps: what the cook path applies to decoded texels.
    let maps: Vec<ChannelMap> = apply(&profile, height, &dxt5, alpha)
        .iter()
        .map(|p| p.map)
        .collect();
    assert_eq!(
        maps,
        [
            ChannelMap([ch(R), Src::Inverted(G), Src::Unused, Src::Unused]),
            ChannelMap([Src::Inverted(A), Src::Unused, Src::Unused, Src::Unused]),
        ]
    );
}

#[test]
fn split_products_keep_every_level_and_decode_as_the_mapped_source() {
    let profile = test_profile();
    let (dxt5, levels) = noise_dds(Pf::FourCc(b"DXT5"), BlockFormat::Bc3);
    let products = apply(
        &profile,
        rule(&profile, PackRole::NormalHeight),
        &dxt5,
        AlphaUse::None,
    );
    for product in &products {
        let out = rewrapped(&product.result);
        assert_eq!((out.levels, out.shipped), (mip_count(16, 16), 5));
        let written = read_source(&out.ktx2).unwrap().unwrap();
        for (level, (bytes, shipped)) in written.levels.iter().zip(&levels).enumerate() {
            let size = (16u32 >> level).max(1);
            assert_eq!(
                bytes.len(),
                level_len(out.format, size, size),
                "{:?} level {level}",
                product.slot
            );
            // Decoded output = the product's map on the decoded source (in-tree decoder rounding).
            let mut want = decode_level(BlockFormat::Bc3, shipped, size, size);
            product.map.apply_rgba(&mut want);
            let got = decode_level(out.format, bytes, size, size);
            let channels = if product.slot == Slot::Depth { 1 } else { 3 };
            for (w, g) in want.as_chunks::<4>().0.iter().zip(got.as_chunks::<4>().0) {
                for (a, b) in w.iter().zip(g).take(channels) {
                    assert!(a.abs_diff(*b) <= 1, "{:?}: {w:?} vs {g:?}", product.slot);
                }
            }
        }
    }
}

#[test]
fn a_single_level_split_fills_both_chains_in_their_formats() {
    let profile = test_profile();
    let level0 = noise(level_len(BlockFormat::Bc3, 32, 32), 7);
    let bytes = dds(Pf::FourCc(b"DXT5"), 32, 32, 1, false, 0, &level0);
    let products = apply(
        &profile,
        rule(&profile, PackRole::NormalHeight),
        &bytes,
        AlphaUse::None,
    );
    let shapes: Vec<(Slot, BlockFormat, u32, u32)> = products
        .iter()
        .map(|p| {
            let out = rewrapped(&p.result);
            (p.slot, out.format, out.shipped, out.levels)
        })
        .collect();
    assert_eq!(
        shapes,
        [
            (Slot::Normal, BlockFormat::Bc1, 1, 6),
            (Slot::Depth, BlockFormat::Bc4, 1, 6)
        ]
    );
}
