use glam::{IVec3, Vec3};

use super::*;

fn s(x: f32, y: f32, z: f32, r: f32) -> Sphere {
    Sphere { center: Vec3::new(x, y, z), radius: r }
}

#[test]
fn texture_ids_follow_the_canonical_stem() {
    let id = texture_id("textures/tx_a.dds");
    // The same stem whatever the container, separators, case and empty segments.
    for alias in ["textures/tx_a.ktx2", "Textures\\TX_A.DDS", "/textures//./tx_a.tga", "textures/tx_a"] {
        assert_eq!(texture_id(alias), id, "{alias}");
    }
    assert_ne!(texture_id("textures/tx_b.dds"), id);
    assert_ne!(texture_id("textures/tx_a_n.dds"), id, "a companion is its own texture");
    // Only the last segment's extension goes; a dotted folder stays, and a dotfile keeps its name.
    assert_ne!(texture_id("textures/a.b/c.dds"), texture_id("textures/a/c.dds"));
    assert_ne!(texture_id("textures/.x"), texture_id("textures/"));
    // The canonical form is `sf_vfs::VfsPath`'s.
    let canon = sf_vfs::VfsPath::new("Textures\\Sub\\TX_A.dds").unwrap();
    assert_eq!(texture_id(canon.as_str()), texture_id("Textures\\Sub\\TX_A.dds"));
    // Pinned: ids are stored in products and must not drift. FNV-1a 64 reference vectors.
    assert_eq!(texture_id("a"), 0xaf63_dc4c_8601_ec8c);
    assert_eq!(texture_id("FooBar.dds"), 0x8594_4171_f739_67e8);
}

#[test]
fn products_round_trip_and_malformed_ones_fail() {
    let demand = CellDemand {
        origin_cell: IVec3::new(-3, 0, 7),
        clusters: vec![
            DemandCluster { sphere: s(1.0, 2.0, 3.0, 4.0), textures: vec![DemandTexture { id: 5, rho: Some(1.5) }, DemandTexture { id: 9, rho: None }] },
            DemandCluster { sphere: s(-1.0, 0.0, 0.0, 30.0), textures: vec![] },
        ],
    };
    let bytes = demand.write().unwrap();
    assert_eq!(bytes.len(), 28 + 2 * 24 + 2 * 12);
    assert_eq!(CellDemand::read(&bytes).unwrap(), demand);
    assert_eq!(CellDemand::read(b"XXXX"), Err(DemandError::Magic));
    assert_eq!(CellDemand::read(&bytes[..bytes.len() - 1]), Err(DemandError::Length));
    let mut bad = bytes.clone();
    bad[4] = 9;
    assert_eq!(CellDemand::read(&bad), Err(DemandError::Version(9)));
    let mut bad = bytes;
    bad[28 + 20] = 3; // first cluster: 3 textures from 0, of 2
    assert_eq!(CellDemand::read(&bad), Err(DemandError::Range(0)));
}

/// A deterministic scatter: `n` spheres over a `side` m square, radii 0.5-4.5 m.
fn scatter(n: usize, side: f32, seed: u64) -> Vec<Sphere> {
    let mut x = seed;
    let mut next = || {
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        #[allow(clippy::cast_precision_loss)]
        let v = (x >> 11) as f32 / (1u64 << 53) as f32;
        v
    };
    (0..n).map(|_| s(next() * side, next() * 10.0, next() * side, 0.5 + next() * 4.0)).collect()
}

#[test]
fn clusters_stay_within_the_radius_and_cover_their_members() {
    let mut spheres = scatter(400, 128.0, 7);
    spheres.push(s(60.0, 0.0, 60.0, 40.0)); // one placement larger than any cluster
    let clusters = cluster(&spheres);
    let mut seen = vec![0; spheres.len()];
    for (bound, members) in &clusters {
        assert!(!members.is_empty());
        if members.len() > 1 {
            assert!(bound.radius <= MAX_CLUSTER_RADIUS, "{bound:?}");
        }
        for &m in members {
            seen[m] += 1;
            let p = spheres[m];
            assert!(p.center.distance(bound.center) + p.radius <= bound.radius * (1.0 + 1e-5), "member {m} outside");
        }
    }
    assert!(seen.iter().all(|&n| n == 1), "every placement in exactly one cluster");
    let big: Vec<_> = clusters.iter().filter(|(b, _)| b.radius > MAX_CLUSTER_RADIUS).collect();
    assert_eq!(big.len(), 1);
    assert_eq!(big[0].1, vec![400], "the oversized placement is alone");
    // Fewer clusters than placements: neighbours share one.
    assert!(clusters.len() < spheres.len() / 4, "{} clusters", clusters.len());
}

#[test]
fn clustering_is_deterministic_and_independent_of_input_order() {
    let spheres = scatter(300, 128.0, 3);
    assert_eq!(cluster(&spheres), cluster(&spheres));
    // Reversed input: the same clusters, members named by their new indices.
    let reversed: Vec<Sphere> = spheres.iter().rev().copied().collect();
    let n = spheres.len();
    let canon = |c: Vec<(Sphere, Vec<usize>)>, map: &dyn Fn(usize) -> usize| {
        let mut v: Vec<(Vec<usize>, [u32; 4])> = c
            .into_iter()
            .map(|(b, m)| {
                let mut m: Vec<usize> = m.into_iter().map(map).collect();
                m.sort_unstable();
                (m, [b.center.x.to_bits(), b.center.y.to_bits(), b.center.z.to_bits(), b.radius.to_bits()])
            })
            .collect();
        v.sort();
        v
    };
    assert_eq!(canon(cluster(&spheres), &|i| i), canon(cluster(&reversed), &|i| n - 1 - i));
}

#[test]
fn merged_textures_keep_the_densest_rho() {
    let t = |id, rho| DemandTexture { id, rho };
    let merged = merge_textures([t(2, Some(1.0)), t(1, None), t(2, Some(3.0)), t(1, Some(0.5)), t(3, None)]);
    assert_eq!(merged, vec![t(1, Some(0.5)), t(2, Some(3.0)), t(3, None)]);
}

#[cfg(feature = "bake")]
mod bake;
