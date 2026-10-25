//! Geometry-morph weight curves as clip curves, on the real game files.
//! Skips (passes) when no Morrowind install is detected. `--nocapture`
//! prints the curves found and the head's face text keys.

use mw_anim::{Animation, KfAnimImporter};
use mw_install::MorrowindInstall;
use sf_anim::AnimImporter;
use sf_vfs::{Vfs, VfsPath};

fn install() -> Option<Vfs> {
    let Ok((install, _)) = MorrowindInstall::detect(None) else {
        eprintln!("no Morrowind install detected; skipping");
        return None;
    };
    Some(mw_vfs::mount_install(&install, Vfs::builder()).unwrap().vfs)
}

fn read(vfs: &Vfs, path: &str) -> Vec<u8> {
    vfs.read(&VfsPath::new(path).unwrap()).unwrap_or_else(|e| panic!("{path}: {e}"))
}

#[test]
fn kwama_forager_clips_carry_its_body_morph_and_match_the_cooked_names() {
    let Some(vfs) = install() else { return };
    let path = "meshes/r/xkwama forager.kf";
    let importer = KfAnimImporter;
    let companion = importer.companion(path).unwrap();
    let source = importer.import_with(path, &read(&vfs, path), Some(&read(&vfs, &companion))).unwrap();
    let mut moving = 0;
    for clip in &source.clips {
        for c in &clip.curves {
            #[allow(clippy::cast_precision_loss)]
            let samples: Vec<f32> = (0..=20).map(|i| c.curve.sample(clip.duration * i as f32 / 20.0)).collect();
            let (lo, hi) = samples.iter().fold((f32::MAX, f32::MIN), |(lo, hi), &v| (lo.min(v), hi.max(v)));
            println!("{} {}: {lo:.3}..{hi:.3} over {:.2} s", clip.name, c.name, clip.duration);
            moving += usize::from(hi - lo > 0.05);
        }
    }
    assert!(moving > 0, "no clip of {path} moves a morph weight");
    // The cooked body (the model without `x`) names its targets the same way.
    let body = Animation::from_model(&mw_nif::load(&read(&vfs, "meshes/r/kwama forager.nif")).unwrap());
    let curves: Vec<&str> = source.clips.iter().flat_map(|c| c.curves.iter().map(|c| c.name.as_str())).collect();
    assert!(!body.morphs.is_empty() && body.morphs.iter().all(|(name, _)| curves.contains(&name.as_str())), "{:?} vs {curves:?}", body.morphs.iter().map(|m| &m.0).collect::<Vec<_>>());
}

#[test]
fn npc_heads_key_blink_and_talk() {
    let Some(vfs) = install() else { return };
    let path = "meshes/b/b_n_dark elf_m_head_01.nif";
    let head = mw_nif::load(&read(&vfs, path)).unwrap();
    println!("{path} text keys: {:?}", head.text_keys);
    let animation = Animation::from_model(&head);
    for (name, curve) in &animation.morphs {
        println!("  {name}: {} keys", curve.keys().len());
    }
    let lower: Vec<String> = head.text_keys.iter().flat_map(|(_, l)| l.lines().map(|l| l.trim().to_ascii_lowercase()).collect::<Vec<_>>()).collect();
    for key in ["talk: start", "talk: stop", "blink: start", "blink: stop"] {
        assert!(lower.iter().any(|l| l == key), "{path}: no {key:?} in {lower:?}");
    }
    assert!(!animation.morphs.is_empty());
}

