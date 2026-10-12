//! Walkers turning through every direction (owner bug "some guards have a
//! wonky walk cycle at night", walkers5): the walk blend space syncs its
//! lanes by foot markers, and Morrowind's walk clips loop inside an intro
//! and an outro that carry markers too. A lane synced past its loop's end
//! played out to the clip's end and froze there once it led (torch-bearers
//! at 1.333 s while walking at 2.1 m/s). Every lane must stay inside its
//! loop and the leading lane must keep moving. Skips without an install.
#![allow(clippy::cast_precision_loss, clippy::cast_possible_truncation, clippy::cast_sign_loss)]

use glam::{Vec2, Vec3};
use mw_anim::Animation;
use mw_anim::rig;
use mw_graphs::build::params as p;
use mw_graphs::sim::Sim;
use mw_graphs::{AnimSet, HUMANOID_SETS, Template};
use mw_install::MorrowindInstall;
use sf_anim_graph::NodeKind;
use sf_vfs::{Vfs, VfsPath};

fn humanoid_sim() -> Option<Sim> {
    let Ok((install, _)) = MorrowindInstall::detect(None) else {
        eprintln!("no Morrowind install detected; skipping");
        return None;
    };
    let vfs = mw_vfs::mount_install(&install, Vfs::builder()).unwrap().vfs;
    let read = |path: &str| vfs.read(&VfsPath::new(path).ok()?).ok();
    let set = &HUMANOID_SETS[0];
    let model = mw_nif::load(&read(set.skeleton).unwrap()).unwrap();
    let skeleton = rig::skeleton(&model).unwrap();
    let mut anims = Vec::new();
    for a in set.animations {
        let Some(bytes) = read(a) else { continue };
        let anim = Animation::from_file(a, &bytes).unwrap();
        if !anim.text_keys.is_empty() {
            anims.push(((*a).to_owned(), anim));
        }
    }
    let mut set = AnimSet::new(set.name, skeleton, anims).with_humanoid_rig();
    let v = mw_graphs::validate(&mut set, Template::Humanoid).unwrap();
    let clips = set.binding_clips(&v.graph).unwrap();
    Some(Sim::new(v.graph, set.skeleton.clone(), clips).unwrap())
}

fn dump(sim: &Sim, path: &str) {
    let n = sim.graph.node(path).unwrap();
    let NodeKind::Player(def) = &sim.graph.nodes[n as usize].kind else { panic!() };
    for &c in &def.lanes {
        let clip = &sim.clips[usize::from(c)];
        eprintln!(
            "{path}: {} dur {:.3} loop {:?} looping {} markers {:?}",
            sim.graph.clips[usize::from(c)].reference,
            clip.duration,
            clip.loop_range,
            def.looping,
            clip.markers.iter().map(|m| (m.time, m.name.clone())).collect::<Vec<_>>()
        );
    }
}

#[test]
fn turning_walkers_keep_every_walk_lane_cycling() {
    let Some(mut sim) = humanoid_sim() else { return };
    let walk = sim.graph.nodes.iter().position(|n| n.name.ends_with("walk.none")).unwrap();
    let path = sim.graph.nodes[walk].name.clone();
    if std::env::var_os("TORCH_WALK_TRACE").is_some() {
        dump(&sim, &path);
    }
    let NodeKind::Player(def) = &sim.graph.nodes[walk].kind else { panic!("{path}: not a player") };
    let ends: Vec<f32> = def.lanes.iter().map(|&c| sim.clips[usize::from(c)].loop_range.map_or(f32::MAX, |r| r.1)).collect();
    for torch in [1.0, 0.0] {
        sim.set(p::TORCH, torch);
        sim.set(p::MOVING, 0.0);
        sim.velocity = Vec3::ZERO;
        sim.run(1.0, 1.0 / 60.0);
        let mut prev: Option<(usize, f32)> = None;
        let mut frozen = 0;
        for turn in 0..48 {
            let a = turn as f32 * std::f32::consts::TAU * 3.0 / 16.0;
            let d = Vec2::new(a.sin(), a.cos());
            sim.set(p::MOVING, 1.0);
            sim.set_vec2(p::MOVE, d);
            sim.velocity = Vec3::new(-d.x, 0.0, d.y) * 2.1;
            for _ in 0..20 {
                sim.run(1.0 / 60.0, 1.0 / 60.0);
                let (lanes, _) = sim.instance.player(walk as u32).unwrap();
                for (i, &(t, _)) in lanes.iter().enumerate() {
                    assert!(t <= ends[i] + 1e-4, "torch {torch} turn {turn}: lane {i} at {t}, past its loop end {}", ends[i]);
                }
                let lead = (0..lanes.len()).max_by(|&i, &j| lanes[i].1.total_cmp(&lanes[j].1)).unwrap();
                if prev.is_some_and(|(l, t)| l == lead && (lanes[lead].0 - t).abs() < 1e-6) {
                    frozen += 1;
                }
                prev = Some((lead, lanes[lead].0));
            }
        }
        assert_eq!(frozen, 0, "torch {torch}: the leading walk lane stood still on {frozen} frames");
    }
}
