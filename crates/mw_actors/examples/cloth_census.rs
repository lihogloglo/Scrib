//! Cloth census (physics-adapter.md "Cloth", M4 P10): Morrowind's robe and
//! skirt items, the slots they fill, and every cloth `BODY` part
//! (`mw_physics::MwClothRules`) cooked through `cloth.cook` on the base
//! skeleton of its sex: skinned or rigid, vertex count, lattice size, hem
//! perimeter vs what the legs need at a run stride, cook time.
//!
//! `cargo run --release -p mw_actors --example cloth_census`

#![allow(clippy::too_many_lines, clippy::cast_precision_loss, clippy::many_single_char_names)] // a linear diagnostic script, small counts

use std::collections::{BTreeMap, HashMap};
use std::sync::Arc;
use std::time::Instant;

use glam::Vec3;
use mw_actors::{MwActors, part_id};
use mw_install::MorrowindInstall;
use mw_physics::{ClothKind, MwClothRules};
use sf_anim::cloth::EdgeKind;
use sf_character::{ClothRules, PartKind, PartLookup, SkinSurface, cook_cloth};
use sf_model::{CookSettings, Cooked, CookedSkinned, ModelImporter};
use sf_records::RecordId;
use sf_vfs::{Vfs, VfsPath};
use tes3::esp::{Bodypart, BodypartFlags, Clothing, ClothingType};

/// Half the leg swing at a run (thigh angle each way), radians: a 50° stride.
const HALF_STRIDE: f32 = 25.0 * std::f32::consts::PI / 180.0;
/// Leg radius at the hem plus the cloth thickness, m.
const LEG_RADIUS: f32 = 0.065 + 0.015;

struct Rig {
    skeleton: sf_anim::Skeleton,
    rest: Vec<glam::Affine3A>,
    ragdoll: sf_ragdoll::RagdollDef,
    profile: sf_anim::RigProfile,
}

fn main() {
    let (install, _) = MorrowindInstall::detect(None).expect("Morrowind install");
    let vfs = Arc::new(mw_vfs::mount_install(&install, Vfs::builder()).unwrap().vfs);
    let content = mw_records::load(&vfs, &install.game_files()).unwrap();
    let db = Arc::new(content.db);
    let a = MwActors::new(Arc::clone(&db), Arc::clone(&vfs));
    let rules = MwClothRules::new(&db);

    // Items and the slots they fill.
    for (kind, ty) in [("robes", ClothingType::Robe), ("skirts", ClothingType::Skirt)] {
        let items: Vec<&Clothing> = db.iter::<Clothing>().map(|(_, c)| c).filter(|c| c.data.clothing_type == ty).collect();
        let mut slots: BTreeMap<String, usize> = BTreeMap::new();
        for c in &items {
            for o in &c.biped_objects {
                *slots.entry(format!("{:?}", o.biped_object_type)).or_default() += 1;
            }
        }
        let no_skirt = items.iter().filter(|c| !c.biped_objects.iter().any(|o| o.biped_object_type == tes3::esp::BipedObjectType::Skirt)).count();
        println!("{kind}: {} items, {no_skirt} without a skirt part; parts per slot {slots:?}", items.len());
    }
    println!("cloth BODY records: {}", rules.len());

    let mut rigs: HashMap<&str, Rig> = HashMap::new();
    for path in ["meshes/base_anim.nif", "meshes/base_anim_female.nif"] {
        let skeleton = mw_anim::rig::skeleton(&mw_nif::load(&vfs.read(&VfsPath::new(path).unwrap()).unwrap()).unwrap()).unwrap();
        // As the bake does for humanoid sets (`mw_bake::anim`): the rig map's profile.
        let profile = mw_anim::rig::humanoid(&skeleton).unwrap().map.profile(&skeleton).unwrap();
        let ragdoll = sf_ragdoll::cook(&skeleton, &profile, &mw_physics::ragdoll_params(path, true)).unwrap();
        let rest = skeleton.rest_model();
        rigs.insert(path, Rig { skeleton, rest, ragdoll, profile });
    }

    for (path, rig) in &rigs {
        let bodies: Vec<String> = rig
            .ragdoll
            .bodies
            .iter()
            .map(|b| format!("{} [{}]", rig.skeleton.name(usize::from(b.joint)), rig.ragdoll.regions[usize::from(b.region)]))
            .collect();
        println!("{path}: ragdoll bodies {bodies:?}; limbs {:?}", rig.profile.limbs.iter().map(|l| (&l.name, l.kind)).collect::<Vec<_>>());
    }
    let mut cooked: HashMap<String, Cooked> = HashMap::new();
    let (mut stretched, mut rigid, mut skinned, mut failed) = (Vec::new(), 0, 0, Vec::new());
    let mut per_kind: BTreeMap<String, Vec<f32>> = BTreeMap::new();
    let mut times = Vec::new();
    println!("\nbody | kind | sex | model | skin | verts | top..hem m | hem w x d m | hem perim m | needs m | stretch | filled | colliders | lattice/band verts | cook ms");
    for (body_id, _, kind) in rules.parts() {
        let id = RecordId::new(body_id);
        let Some(body) = db.get::<Bodypart>(&id) else {
            failed.push(format!("{body_id}: no BODY record"));
            continue;
        };
        let female = body.data.flags.contains(BodypartFlags::FEMALE);
        let rig = &rigs[if female { "meshes/base_anim_female.nif" } else { "meshes/base_anim.nif" }];
        let pid = part_id(body_id, if kind == ClothKind::Robe { 3 } else { 5 });
        let Some(part) = a.part(&pid) else {
            failed.push(format!("{body_id}: no part {}", pid.as_str()));
            continue;
        };
        let spec = rules.cloth(&pid, part).expect("cloth rules agree");
        let started = Instant::now();
        let path = part.model.as_str().to_owned();
        if !cooked.contains_key(&path) {
            let model = mw_bake::NifImporter.import(&path, &vfs.read(&part.model).unwrap()).unwrap();
            cooked.insert(path.clone(), sf_model::cook(&model, &CookSettings::default()).unwrap());
        }
        let s = match (&part.kind, &cooked[&path]) {
            (PartKind::Rigid { joint, offset }, c) => {
                rigid += 1;
                CookedSkinned::rigid(c, joint, *offset).unwrap()
            }
            (PartKind::Skinned, Cooked::Skinned(s)) => {
                skinned += 1;
                Some(s.clone())
            }
            _ => None,
        };
        let Some(s) = s else {
            failed.push(format!("{}: not skinnable", id.as_str()));
            continue;
        };
        let joint = |n: &str| rig.skeleton.find(n).and_then(|j| u16::try_from(j).ok());
        let surface = match SkinSurface::from_cooked(&s, 0, Some(part), joint, &rig.rest) {
            Ok(s) => s,
            Err(e) => {
                failed.push(format!("{}: {e}", id.as_str()));
                continue;
            }
        };
        let attach = joint(&spec.attach).unwrap();
        let top = rig.rest[usize::from(attach)].translation.y + spec.attach_offset;
        let colliders = rig.ragdoll.cloth_colliders(&rig.profile, &rig.rest, top);
        let ncol = colliders.len();
        let cook = match cook_cloth(&surface, &rig.rest, attach, &spec, colliders) {
            Ok(c) => c,
            Err(e) => {
                failed.push(format!("{} ({path}): {e}", id.as_str()));
                continue;
            }
        };
        let ms = started.elapsed().as_secs_f64() * 1e3;
        times.push(ms);
        let l = &cook.lattice;
        let hem0 = (usize::from(l.rows) - 1) * usize::from(l.cols);
        let hem: Vec<Vec3> = l.particles[hem0..].iter().map(|p| p.rest).collect();
        let perim: f32 =
            l.edges.iter().filter(|e| e.kind == EdgeKind::Structural && usize::from(e.a) >= hem0 && usize::from(e.b) >= hem0).map(|e| e.rest).sum();
        let (lo, hi) = hem.iter().fold((Vec3::MAX, Vec3::MIN), |(lo, hi), &p| (lo.min(p), hi.max(p)));
        // The legs at the hem row, thighs swung ±25°: two circles `w` apart side to side and `d` front to back.
        let hip = |n: &str| Vec3::from(rig.rest[rig.skeleton.find(n).unwrap()].translation);
        let (lh, rh) = (hip("Bip01 L Thigh"), hip("Bip01 R Thigh"));
        let drop = lh.y.midpoint(rh.y) - hem[0].y;
        let (w, d) = (lh.distance(rh), 2.0 * drop.max(0.0) * HALF_STRIDE.sin());
        let needs = 2.0 * w.hypot(d) + std::f32::consts::TAU * LEG_RADIUS;
        let stretch = needs / perim - 1.0;
        if stretch > 0.0 {
            stretched.push(format!("{} {:.0} %", id.as_str(), stretch * 100.0));
        }
        per_kind.entry(format!("{kind:?} {}", if female { "f" } else { "m" })).or_default().push(stretch);
        println!(
            "{} | {kind:?} | {} | {path} | {} | {} | {:.2}..{:.2} | {:.2} x {:.2} | {perim:.2} | {needs:.2} | {:+.0} % | {} | {ncol} | {}/{} | {ms:.1}",
            id.as_str(),
            if female { "f" } else { "m" },
            if matches!(part.kind, PartKind::Rigid { .. }) { "rigid" } else { "skinned" },
            surface.positions.len(),
            cook.top,
            cook.hem,
            hi.x - lo.x,
            hi.z - lo.z,
            stretch * 100.0,
            cook.filled,
            cook.rebind.on_lattice,
            cook.rebind.band,
        );
    }
    println!("\nparts: {} cooked ({skinned} skinned, {rigid} rigid), {} failed", times.len(), failed.len());
    for f in &failed {
        println!("  failed: {f}");
    }
    times.sort_by(f64::total_cmp);
    if let (Some(max), true) = (times.last(), !times.is_empty()) {
        println!("cook time: median {:.1} ms, max {max:.1} ms (incl. NIF import on first use)", times[times.len() / 2]);
    }
    for (k, v) in &per_kind {
        let fit = v.iter().filter(|&&s| s <= 0.0).count();
        println!("{k}: {} parts, {fit} hold the stride, stretch needed max {:.0} %", v.len(), v.iter().copied().fold(f32::MIN, f32::max) * 100.0);
    }
    println!("hems narrower than the stride: {}", stretched.len());
    let male = &rigs["meshes/base_anim.nif"];
    let waist = male.rest[male.skeleton.find(mw_physics::CLOTH_ATTACH).unwrap()].translation.y;
    let colliders = male.ragdoll.cloth_colliders(&male.profile, &male.rest, waist);
    println!(
        "colliders (male, below the waist {waist:.2} m): {:?}",
        colliders.iter().map(|c| rigs["meshes/base_anim.nif"].skeleton.name(usize::from(c.joint))).collect::<Vec<_>>()
    );
}
