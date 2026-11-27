//! RG-P1: model ids round trip, `resolve` on a fixture library, `read_libraries` over a store baked by
//! `LibraryStep` (feature `bake`).

use super::{Libraries, Resolved, model_id, parse_model_id};
use crate::{Aabb, CLASSES, Library, RockLods, Variant};

#[test]
fn model_ids_round_trip_for_every_class_and_refuse_others() {
    for class in CLASSES {
        for index in [0, 1, 7, 31, u32::MAX] {
            let id = model_id(class.name, index);
            assert_eq!(id, format!("rockgen/{}/{index}", class.name));
            assert_eq!(parse_model_id(&id), Some((class.name, index)), "{id}");
            // Class names survive a lowercasing path type unchanged.
            assert_eq!(id.to_lowercase(), id);
        }
    }
    for bad in [
        "meshes/x/rock.nif",
        "rockgen",
        "rockgen/",
        "rockgen/talus",
        "rockgen/talus/",
        "rockgen//3",
        "rockgen/talus/x",
        "rockgen/talus/3a",
        "rockgen/talus/-3",
        "rockgen/talus/+3",
        "rockgen/talus/03",
        "rockgen/talus/3/4",
        "rockgen/talus/4294967296",
        "rockgenx/talus/3",
        "other/talus/3",
    ] {
        assert_eq!(parse_model_id(bad), None, "{bad}");
    }
}

/// A variant whose LOD0 bounds are `extent` centred on the origin (no meshes: `resolve` reads only the bounds).
fn variant(seed: u64, extent: [f32; 3]) -> Variant {
    let half = extent.map(|e| e * 0.5);
    Variant {
        seed,
        lods: RockLods { lods: Vec::new(), bounds: Aabb { min: half.map(|h| -h), max: half }, hash: seed },
        hull: None,
    }
}

fn fixture() -> Libraries {
    let three = Library {
        key: 11,
        size_m: 2.0,
        variants: vec![variant(1, [2.0, 1.5, 0.8]), variant(2, [2.0, 1.2, 1.0]), variant(3, [1.6, 2.0, 0.5])],
    };
    let empty = Library { key: 12, size_m: 1.0, variants: Vec::new() };
    Libraries::new([("talus".to_owned(), three), ("slab".to_owned(), empty)])
}

#[test]
#[allow(clippy::float_cmp)] // exact: the spec's formula, evaluated the same way
fn resolve_takes_hash_modulo_count_scale_and_lift_exactly() {
    let libs = fixture();
    let lib = libs.get("talus").expect("talus");
    for (hash, mark, sink) in [(0_u64, 2.0_f32, 0.0_f32), (1, 1.0, 0.25), (5, 3.5, 0.5), (u64::MAX, 0.4, 0.8)] {
        let r = libs.resolve("talus", hash, mark, sink).expect("resolves");
        let index = u32::try_from(hash % 3).expect("small");
        let scale = mark / 2.0;
        let extent_z = lib.variants[index as usize].lods.bounds.extent()[2];
        assert_eq!(r, Resolved { index, scale, lift_m: (0.5 - sink) * extent_z * scale }, "hash {hash}");
        assert_eq!(libs.variant("talus", r.index), Some(&lib.variants[index as usize]));
    }
    // Half sunk: no lift; fully sunk: the centre goes down half the short axis.
    assert_eq!(libs.resolve("talus", 0, 2.0, 0.5).expect("resolves").lift_m, 0.0);
    assert_eq!(libs.resolve("talus", 0, 2.0, 1.0).expect("resolves").lift_m, -0.4);

    assert_eq!(libs.resolve("cobble", 0, 1.0, 0.0), None, "unknown class");
    assert_eq!(libs.resolve("slab", 0, 1.0, 0.0), None, "empty library");
    assert_eq!(libs.variant("talus", 3), None);
    assert_eq!(libs.variant("cobble", 0), None);
    assert_eq!(libs.classes().collect::<Vec<_>>(), ["slab", "talus"]);
    assert_eq!(libs.len(), 2);
}

#[cfg(feature = "bake")]
mod read {
    use std::sync::Arc;

    use sf_bake::{BakeError, Baker, Digest, Input, Inputs, ProductKey, Store};

    use super::super::{ReadError, read_libraries};
    use crate::bake::{LIBRARY_SEED, LIBRARY_STEP, LibrarySpec, LibraryStep, class_seed};
    use crate::{BoulderParams, CLASSES, LibrarySettings, build_library};

    struct NoInputs;

    impl Inputs for NoInputs {
        fn file_fingerprint(&self, _: &str) -> Result<Option<Digest>, BakeError> {
            Ok(None)
        }
        fn read_file(&self, path: &str) -> Result<Vec<u8>, BakeError> {
            Err(BakeError::MissingInput(Input::File(path.into())))
        }
        fn record_fingerprint(&self, _: &str, _: &str) -> Result<Option<Digest>, BakeError> {
            Ok(None)
        }
    }

    fn spec(name: &str) -> LibrarySpec {
        let class = CLASSES.iter().find(|c| c.name == name).expect("class exists");
        LibrarySpec {
            params: class.params(BoulderParams::default()),
            settings: LibrarySettings { variants: 8, ..LibrarySettings::default() },
            seed: class_seed(LIBRARY_SEED, name),
        }
    }

    #[test]
    fn read_libraries_matches_the_baked_products_and_skips_absent_classes() {
        let dir = tempfile::tempdir().expect("tempdir");
        let names = ["block", "talus"];
        let step = LibraryStep::new(names.map(|n| (n.to_owned(), spec(n))));
        let keys = step.keys();
        let mut baker = Baker::new(Store::open(dir.path()).expect("store"), Arc::new(NoInputs));
        baker.register(step);
        assert_eq!(baker.bake(&keys).failures().count(), 0);
        let manifest = baker.manifest();

        let libs = read_libraries(baker.store(), &manifest, CLASSES.iter().map(|c| c.name)).expect("reads");
        assert_eq!(libs.classes().collect::<Vec<_>>(), names, "only the baked classes");
        for name in names {
            let s = spec(name);
            let direct = build_library(&s.params, &s.settings, s.seed);
            assert_eq!(libs.get(name).expect("present").content_hash(), direct.content_hash(), "{name}");
        }

        // A product that does not decode is an error naming its class.
        let mut broken = manifest.clone();
        let junk = baker.store().put(&[0xff; 3], false).expect("put");
        broken.products.insert(ProductKey::new(LIBRARY_STEP, "talus"), junk);
        let err = read_libraries(baker.store(), &broken, ["talus"]).expect_err("junk");
        assert!(matches!(&err, ReadError::Decode { class, .. } if class == "talus"), "{err}");
    }
}
