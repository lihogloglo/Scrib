//! `rockgen.library` through a real `Baker` on a temporary store: round trip, reuse, one-class invalidation.

use std::sync::Arc;

use sf_bake::{BakeError, Baker, Digest, Input, Inputs, ProductKey, Store};

use super::{LIBRARY_SEED, LIBRARY_STEP, LibrarySpec, LibraryStep, class_seed, decode};
use crate::{BoulderParams, CLASSES, LibrarySettings, build_library};

/// The step reads nothing: every input is a planned parameter.
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

fn small() -> LibrarySettings {
    LibrarySettings { variants: 8, ..LibrarySettings::default() }
}

/// Two classes (one eroded, one sharp) at the smallest library size: enough to see one re-bake beside one reuse.
fn specs(chips_talus: u32) -> Vec<(String, LibrarySpec)> {
    let base = BoulderParams::default();
    ["block", "talus"]
        .into_iter()
        .map(|name| {
            let class = CLASSES.iter().find(|c| c.name == name).expect("class exists");
            let mut params = class.params(base);
            if name == "talus" {
                params.chips = chips_talus;
            }
            (name.to_owned(), LibrarySpec { params, settings: small(), seed: class_seed(LIBRARY_SEED, name) })
        })
        .collect()
}

fn baker(root: &std::path::Path, specs: Vec<(String, LibrarySpec)>) -> (Baker, Vec<ProductKey>) {
    let step = LibraryStep::new(specs);
    let keys = step.keys();
    let mut baker = Baker::new(Store::open(root).expect("store"), Arc::new(NoInputs));
    baker.register(step);
    (baker, keys)
}

fn stats(baker: &Baker) -> (u32, u32) {
    let s = baker.step_stats().get(LIBRARY_STEP).copied().unwrap_or_default();
    (s.built, s.hits)
}

#[test]
fn round_trip_reuse_and_one_class_rebake() {
    let dir = tempfile::tempdir().expect("tempdir");
    let talus_chips = CLASSES.iter().find(|c| c.name == "talus").expect("talus").params(BoulderParams::default()).chips;

    // First bake: both built; each product decodes to exactly the library built directly.
    let (first, keys) = baker(dir.path(), specs(talus_chips));
    let report = first.bake(&keys);
    assert_eq!(report.failures().count(), 0);
    assert_eq!(stats(&first), (2, 0));
    for ((key, digest), (name, spec)) in report.products.iter().zip(specs(talus_chips)) {
        assert_eq!(key.subject, name);
        let bytes = first.store().get(digest.as_ref().expect("baked")).expect("stored");
        let baked = decode(&bytes).expect("decodes");
        let direct = build_library(&spec.params, &spec.settings, spec.seed);
        assert_eq!(baked.key, spec.key());
        assert_eq!(baked.content_hash(), direct.content_hash(), "{name}");
        assert_eq!(baked, direct, "{name}");
        assert_eq!(baked.variants.len(), 8);
        assert_eq!(baked.variants.iter().all(|v| v.hull.is_some()), cfg!(feature = "collision"), "{name} hulls");
    }

    // Second session, same specs: nothing runs.
    let (second, keys) = baker(dir.path(), specs(talus_chips));
    let again = second.bake(&keys);
    assert_eq!(stats(&second), (0, 2));
    let digests = |r: &sf_bake::BakeReport| r.products.iter().map(|(_, d)| *d.as_ref().expect("baked")).collect::<Vec<_>>();
    assert_eq!(digests(&again), digests(&report));

    // One class parameter changes: only that class re-bakes.
    let (third, keys) = baker(dir.path(), specs(talus_chips + 1));
    let changed = third.bake(&keys);
    assert_eq!(stats(&third), (1, 1));
    let (old, new) = (digests(&report), digests(&changed));
    assert_eq!(old[0], new[0], "block reused");
    assert_ne!(old[1], new[1], "talus re-baked");
}

#[test]
fn every_class_has_a_key_and_unknown_subjects_fail() {
    let step = LibraryStep::from_classes(CLASSES, BoulderParams::default(), LibrarySettings::default(), LIBRARY_SEED);
    let keys = step.keys();
    assert_eq!(keys.len(), CLASSES.len());
    let mut seeds: Vec<u64> = CLASSES.iter().map(|c| step.spec(c.name).expect("spec").seed).collect();
    seeds.sort_unstable();
    seeds.dedup();
    assert_eq!(seeds.len(), CLASSES.len(), "classes never share a seed");
    let dir = tempfile::tempdir().expect("tempdir");
    let mut baker = Baker::new(Store::open(dir.path()).expect("store"), Arc::new(NoInputs));
    baker.register(step);
    assert!(baker.product(&ProductKey::new(LIBRARY_STEP, "no-such-class")).is_err());
}

#[test]
#[ignore = "measurement: one full bake of every class's library (default settings) through a Baker, cold and warm"]
fn measure_library_bake() {
    let dir = tempfile::tempdir().expect("tempdir");
    for pass in ["cold", "warm"] {
        let step = LibraryStep::from_classes(CLASSES, BoulderParams::default(), LibrarySettings::default(), LIBRARY_SEED);
        let keys = step.keys();
        let mut baker = Baker::new(Store::open(dir.path()).expect("store"), Arc::new(NoInputs));
        baker.register(step);
        let report = baker.bake(&keys);
        let s = baker.step_stats().get(LIBRARY_STEP).copied().unwrap_or_default();
        println!(
            "rockgen.library {pass}: {} classes in {:.2?} wall (built {}, hit {}, run {:.2} s summed, {} KiB written)",
            keys.len(),
            report.elapsed,
            s.built,
            s.hits,
            s.run_time.as_secs_f64(),
            s.bytes_written / 1024
        );
    }
}
