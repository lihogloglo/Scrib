//! Runtime NIF effects on actor models, counted on the real game data
//! (nif-runtime.md "Actor parts"): which controller targets (material
//! alpha / color / UV, visibility, node keyframes), billboards and particle
//! systems sit in the models characters wear (`BODY` parts) and creatures
//! (`CREA`). Skips (passes) without a Morrowind install. `-- --nocapture`
//! prints the census.
#![allow(clippy::too_many_lines)]

use std::collections::{BTreeMap, BTreeSet};

use mw_install::MorrowindInstall;
use mw_nif::NodeKind;
use mw_nif::anim::Target;
use sf_model::{CookSettings, Cooked};
use sf_records::RecordDb;
use sf_vfs::{Vfs, VfsPath};
use tes3::esp::{Bodypart, Creature};

#[derive(Default)]
struct Census {
    models: usize,
    /// Controller target kind -> (auto-play controllers, models).
    targets: BTreeMap<&'static str, (usize, BTreeSet<String>)>,
    billboards: BTreeMap<String, usize>,
    particles: BTreeMap<String, usize>,
    /// One line per visibility controller (what it hides).
    details: Vec<String>,
}

fn kind(target: &Target) -> &'static str {
    match target {
        Target::Transform { .. } => "transform",
        Target::Visibility { .. } => "visibility",
        Target::Alpha { .. } => "alpha",
        Target::MaterialColor { .. } => "material color",
        Target::Uv { .. } => "uv",
    }
}

fn survey(vfs: &Vfs, models: &BTreeSet<String>) -> Census {
    let mut census = Census::default();
    for path in models {
        let Some(bytes) = VfsPath::new(path).ok().and_then(|p| vfs.read(&p).ok()) else { continue };
        let Ok(nif) = mw_nif::load(&bytes) else { continue };
        census.models += 1;
        for c in nif.controllers.iter().filter(|c| c.auto_play) {
            let entry = census.targets.entry(kind(&c.target)).or_default();
            entry.0 += 1;
            entry.1.insert(path.clone());
            if let Target::Visibility { node, keys } = &c.target {
                let n = &nif.nodes[*node];
                let skinned = n.mesh.is_some_and(|m| nif.meshes[m].skinned);
                let parent = n.parent.map_or("-", |p| nif.nodes[p].name.as_str());
                let range = keys.time_range().unwrap_or((0.0, 0.0));
                #[allow(clippy::cast_precision_loss)]
                let shown: String =
                    (0..16).map(|i| if keys.sample(range.0 + (range.1 - range.0) * i as f32 / 16.0).unwrap_or(true) { '1' } else { '0' }).collect();
                census.details.push(format!(
                    "{path}: vis {:?} {:?} mesh={} skinned={skinned} parent={parent} range={range:?} {shown}",
                    n.name,
                    n.kind,
                    n.mesh.is_some()
                ));
            }
        }
        let billboards = nif.nodes.iter().filter(|n| n.kind == NodeKind::Billboard).count();
        if billboards > 0 {
            census.billboards.insert(path.clone(), billboards);
        }
        if !nif.particles.is_empty() {
            census.particles.insert(path.clone(), nif.particles.len());
        }
    }
    census
}

fn print(label: &str, census: &Census) {
    eprintln!("{label}: {} models", census.models);
    for (kind, (count, models)) in &census.targets {
        let examples: Vec<&str> = models.iter().take(8).map(String::as_str).collect();
        eprintln!("  {kind}: {count} auto-play controllers in {} models, e.g. {examples:?}", models.len());
    }
    eprintln!("  billboards: {} models {:?}", census.billboards.len(), census.billboards);
    eprintln!("  particles: {} models {:?}", census.particles.len(), census.particles);
    for d in &census.details {
        eprintln!("    {d}");
    }
}

#[test]
fn actor_models_runtime_effects_census() {
    let Ok((install, _)) = MorrowindInstall::detect(None) else {
        eprintln!("no Morrowind install detected; skipping");
        return;
    };
    let vfs = mw_vfs::mount_install(&install, Vfs::builder()).unwrap().vfs;
    let db: RecordDb = mw_records::load(&vfs, &install.game_files()).unwrap().db;
    let parts: BTreeSet<String> = db.iter::<Bodypart>().filter_map(|(_, b)| mw_bake::model_path(&b.mesh)).collect();
    let creatures: BTreeSet<String> = db.iter::<Creature>().filter_map(|(_, c)| mw_bake::model_path(&c.mesh)).collect();
    let parts = survey(&vfs, &parts);
    let creatures = survey(&vfs, &creatures);
    print("BODY parts", &parts);
    print("CREA models", &creatures);
    assert!(parts.models > 500 && creatures.models > 80, "install read");
    // The Heart of Akulakhan hangs six parts under billboard nodes.
    assert!(creatures.billboards.keys().any(|m| m.contains("heart_akulakhan")), "heart billboards");

    // Effect curves the animation import gives each creature (nif-runtime.md
    // "Actor parts"), and models whose root carries a transform.
    let (mut curves, mut steps, mut skipped, mut rooted) = (0, 0, 0, Vec::new());
    for path in db.iter::<Creature>().filter_map(|(_, c)| mw_bake::model_path(&c.mesh)).collect::<BTreeSet<_>>() {
        let Some(nif) = VfsPath::new(&path).ok().and_then(|p| vfs.read(&p).ok()).and_then(|b| mw_nif::load(&b).ok()) else { continue };
        let effects = mw_anim::effects::Effects::of_model(&nif);
        let names: BTreeSet<&str> = effects.curves.iter().map(|(n, _)| n.as_str()).collect();
        if !names.is_empty() {
            eprintln!("  {path}: {names:?}");
        }
        curves += effects.curves.len();
        steps += effects.steps.len();
        skipped += effects.skipped;
        if nif.root_transform != mw_nif::coords::Trs::default() {
            rooted.push(path);
        }
    }
    eprintln!("effect curves: {curves} material channels, {steps} visibility, {skipped} skipped; root transforms: {rooted:?}");
    assert!(steps >= 101 && curves > 0, "every visibility controller becomes a curve");

    // Billboards join the rig: the heart's six rays bind to billboard joints
    // of its animation skeleton, not to the root.
    let heart = "meshes/r/heart_akulakhan.nif";
    let nif = mw_nif::load(&vfs.read(&VfsPath::new(heart).unwrap()).unwrap()).unwrap();
    let billboards = mw_anim::rig::billboard_joints(&nif);
    assert_eq!(billboards.len(), 6, "{billboards:?}");
    let (joints, _) = mw_anim::rig::skeleton_source(&nif);
    let names: BTreeSet<&str> = joints.iter().map(|j| j.name.as_str()).collect();
    let Ok(Cooked::Skinned(rig)) = sf_model::cook_rig(&mw_bake::to_model(&nif), &CookSettings::default()) else { panic!("heart rig") };
    let missing: Vec<&str> = rig.joints.iter().map(|j| j.name.as_str()).filter(|j| !names.contains(j)).collect();
    assert!(missing.is_empty(), "heart palette joints outside its skeleton: {missing:?}");
    for b in &billboards {
        assert!(rig.joints.iter().any(|j| &j.name == b), "no part binds to billboard {b}");
    }
    eprintln!("heart billboards (rig joints): {billboards:?}");

    // The heart's clips carry its effect curves, cut per group, and some
    // change within a clip (the controller advances with the animation).
    // Its clips: `xheart_akulakhan.kf` with the `.nif` companion (as the bake imports them).
    let read = |p: &str| vfs.read(&VfsPath::new(p).unwrap()).ok();
    let (kf, x) = ("meshes/r/xheart_akulakhan.kf", "meshes/r/xheart_akulakhan.nif");
    let source = match read(kf) {
        Some(bytes) => sf_anim::AnimImporter::import_with(&mw_anim::KfAnimImporter, kf, &bytes, read(x).as_deref()).unwrap(),
        None => sf_anim::AnimImporter::import(&mw_anim::NifAnimImporter, x, &read(x).unwrap()).unwrap(),
    };
    let skeleton = sf_anim::AnimImporter::import(&mw_anim::NifAnimImporter, heart, &read(heart).unwrap()).unwrap().skeleton.unwrap();
    assert!(billboards.iter().all(|b| skeleton.iter().any(|j| &j.name == b)), "billboards in the imported skeleton");
    let mut moving = BTreeSet::new();
    for clip in &source.clips {
        for c in clip.curves.iter().filter(|c| mw_anim::effects::parse(&c.name).is_some()) {
            if c.curve.values.windows(2).any(|w| (w[0] - w[1]).abs() > 1e-4) {
                moving.insert(format!("{}/{}", clip.name, c.name));
            }
        }
    }
    eprintln!("heart: {} clips, effect curves changing within a clip: {moving:?}", source.clips.len());
    assert!(moving.iter().any(|m| m.contains("hide:")) && moving.iter().any(|m| m.contains("mat:")), "heart effects advance");
}
