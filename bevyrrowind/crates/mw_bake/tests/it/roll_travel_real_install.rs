//! The roll's root travel against time (combat.md, the sword sheet's `roll`): UAL1's `Roll`
//! retargeted onto `base_anim` through the bake, its root track printed per clip frame with the
//! share of the whole travel reached, so the sheet's cancel points and `rate` are set from data.
//! Measurement only (`-- --ignored --nocapture`); skips without a Morrowind install or the
//! library file (`BEVYRROWIND_ANIM_SOURCES`, default `<repo>/animation packs`).

use std::collections::BTreeSet;
use std::sync::Arc;

use mw_bake::VfsInputs;
use mw_bake::anim::{self, ActorAnimSet, ActorSets};
use mw_bake::sources::AnimSources;
use mw_graphs::{HUMANOID_SETS, Template};
use mw_install::MorrowindInstall;
use sf_anim::Clip;
use sf_anim::steps::{ANIM_RETARGET, anim_subject, retarget_subject};
use sf_bake::{Baker, ProductKey, Store};
use sf_vfs::Vfs;

const ROLL: &str = "quaternius_ual1:Roll";

#[test]
#[ignore = "measurement: roll root travel against clip time on base_anim (sheet cancel points and rate)"]
#[allow(clippy::cast_precision_loss, clippy::cast_possible_truncation, clippy::cast_sign_loss, reason = "printed measurements")]
fn roll_root_travel_against_time() {
    let Ok((install, _)) = MorrowindInstall::detect(None) else {
        eprintln!("no Morrowind install detected; skipping");
        return;
    };
    let sources = Arc::new(AnimSources::local());
    let Some((file, _)) = sources.references().into_iter().find(|(_, r)| r.reference == ROLL) else {
        eprintln!("the manifest names no {ROLL}; skipping");
        return;
    };
    if sources.real_path(&file).is_none() {
        eprintln!("{file} not in {}; skipping", sources.dir.display());
        return;
    }
    let vfs = Arc::new(mw_vfs::mount_install(&install, Vfs::builder().mount(mw_graphs::builtin_source())).unwrap().vfs);
    let set = &HUMANOID_SETS[0];
    let mut sets = ActorSets::default();
    let s = ActorAnimSet {
        skeleton: set.skeleton.to_owned(),
        files: set.animations.iter().map(|f| (*f).to_owned()).collect(),
        humanoid: true,
        graphs: BTreeSet::from([Template::Humanoid.path().to_owned()]),
        actors: 1,
        ..ActorAnimSet::default()
    };
    sets.sets.insert(s.skeleton.clone(), s);
    let dir = tempfile::tempdir().unwrap();
    let store = Store::open(dir.path().join("store")).unwrap();
    let inputs = Arc::new(VfsInputs::new(vfs, &install, dir.path().join("filehash.memo")).with_anim_sources(sources.clone()));
    let mut baker = Baker::new(store, inputs);
    let report = anim::bake_animation(&mut baker, &sets, &sources);
    for (key, e) in &report.failures {
        eprintln!("failed: {key}: {e}");
    }
    let key = ProductKey::new(ANIM_RETARGET, retarget_subject(set.skeleton, &anim_subject(&file, &file, ROLL)));
    let digest = baker.product(&key).unwrap_or_else(|e| panic!("{key}: {e}"));
    let clip = Clip::from_bytes(&baker.store().get(&digest).unwrap()).unwrap();
    assert!(clip.root_motion.is_some(), "the roll has no root motion");

    let start = clip.root_at(0.0).0;
    let planar = |t: f32| {
        let p = clip.root_at(t).0 - start;
        p.x.hypot(p.z)
    };
    let total = planar(clip.duration);
    println!("{ROLL} on {}: {:.3} s, planar root travel {total:.3} m", set.skeleton, clip.duration);
    println!("clip_s  travel_m  share  speed_m_s");
    let frames = (clip.duration * 30.0).round() as usize;
    let mut last = 0.0;
    for i in 0..=frames {
        let t = clip.duration * i as f32 / frames as f32;
        let d = planar(t);
        let speed = (d - last) * frames as f32 / clip.duration;
        println!("{t:6.3}  {d:8.3}  {:5.3}  {speed:8.2}", d / total.max(1e-6));
        last = d;
    }
    for share in [0.5, 0.8, 0.9, 0.95, 0.98, 0.99] {
        let t = (0..=1000).map(|i| clip.duration * i as f32 / 1000.0).find(|&t| planar(t) >= share * total).unwrap_or(clip.duration);
        println!("{:>3.0}% of travel by clip {t:.3} s", share * 100.0);
    }
}
