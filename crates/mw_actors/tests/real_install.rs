//! Recipes on the real game data: every NPC_ and CREA, then named actors
//! (an Imperial guard, a Khajiit, an Argonian, a woman, a naked Fargoth, a
//! guar, a cliff racer, a centurion spider). Skips (passes) when no
//! Morrowind install is detected.

use std::collections::BTreeMap;
use std::sync::Arc;

use mw_actors::MwActors;
use mw_actors::equip::EquipSlot;
use mw_anim::rig::CreatureMover;
use mw_install::MorrowindInstall;
use sf_character::{BodyPart, CharacterRecipe, CharacterSource, CharacterState, Outfit, PartKind, PartLookup, View, resolve_recipe};
use sf_records::RecordId;
use sf_vfs::Vfs;
use tes3::esp::{Creature, Npc};

fn setup() -> Option<(MwActors, Arc<Vfs>)> {
    let Ok((install, _)) = MorrowindInstall::detect(None) else {
        eprintln!("no Morrowind install detected; skipping");
        return None;
    };
    let vfs = Arc::new(mw_vfs::mount_install(&install, Vfs::builder()).unwrap().vfs);
    let content = mw_records::load(&vfs, &install.game_files()).unwrap();
    Some((MwActors::new(Arc::new(content.db), Arc::clone(&vfs)), vfs))
}

fn names(a: &MwActors, outfit: &Outfit) -> Vec<(String, String)> {
    outfit.parts.iter().map(|(s, p)| (a.slots().name(*s).unwrap().to_owned(), p.as_str().to_owned())).collect()
}

fn slot<'a>(v: &'a [(String, String)], name: &str) -> Option<&'a str> {
    v.iter().find(|(s, _)| s == name).map(|(_, p)| p.as_str())
}

/// Meshes a part keeps of its model (its node filter applied to each mesh's node path).
fn kept_meshes(vfs: &Vfs, part: &BodyPart) -> (usize, usize) {
    let model = mw_nif::load(&vfs.read(&part.model).unwrap()).unwrap();
    let mut kept = 0;
    let mut total = 0;
    for (i, node) in model.nodes.iter().enumerate() {
        if node.mesh.is_none() {
            continue;
        }
        total += 1;
        let mut path = vec![];
        let mut n = Some(i);
        while let Some(j) = n {
            path.push(model.nodes[j].name.as_str());
            n = model.nodes[j].parent;
        }
        kept += usize::from(part.keeps_node(path));
    }
    (kept, total)
}

/// Resolves an NPC (base equipment unless `worn` is given), checks every
/// visible part exists, and that skinned parts keep some geometry.
fn dress(a: &MwActors, vfs: &Vfs, id: &str, worn: Option<Vec<RecordId>>) -> (CharacterRecipe, Vec<(String, String)>) {
    let recipe = a.recipe("NPC_", &RecordId::new(id), &CharacterState { worn, held: None }).unwrap();
    let outfit = resolve_recipe(a, a.slots(), &recipe, View::ThirdPerson).unwrap();
    for (_, p) in &outfit.parts {
        let part = a.part(p).unwrap();
        assert!(vfs.contains(&part.model), "{id}: {p} model {} missing", part.model);
        if part.node_filter.is_some() {
            let (kept, total) = kept_meshes(vfs, part);
            assert!(kept > 0, "{id}: {p} filter {:?} keeps none of {total} meshes", part.node_filter);
        }
    }
    let v = names(a, &outfit);
    eprintln!("{id}: {} [{}]\n  worn {:?}\n  {v:?}", recipe.skeleton, recipe.scale, recipe.worn.iter().map(RecordId::as_str).collect::<Vec<_>>());
    (recipe, v)
}

#[test]
#[allow(clippy::too_many_lines)]
fn npcs_and_creatures_dress_from_the_records() {
    let Some((a, vfs)) = setup() else { return };
    let db = a.db();
    eprintln!("{} part records, {} wearables", a.part_count(), a.wearable_count());

    // Every NPC_ and CREA gives a recipe; skeleton choice histogram.
    let mut skeletons: BTreeMap<String, usize> = BTreeMap::new();
    let (mut npcs, mut errors) = (0, Vec::new());
    for (id, _) in db.iter::<Npc>() {
        npcs += 1;
        match a.recipe("NPC_", id, &CharacterState::default()) {
            Ok(r) => *skeletons.entry(r.skeleton.as_str().to_owned()).or_default() += 1,
            Err(e) => errors.push(format!("{id}: {e}")),
        }
    }
    eprintln!("{npcs} NPCs, {} errors {:?}; skeletons {skeletons:?}", errors.len(), errors.iter().take(5).collect::<Vec<_>>());
    assert!(npcs > 2000 && errors.len() * 100 < npcs, "{errors:?}");
    let creatures = db.iter::<Creature>().filter(|(id, _)| a.recipe("CREA", id, &CharacterState::default()).is_ok()).count();
    eprintln!("{creatures} of {} creatures", db.len::<Creature>());
    assert_eq!(creatures, db.len::<Creature>());
    let maps = a.rig_maps();
    eprintln!("rig maps: {:?}", maps.iter().map(|(p, _)| p.as_str()).collect::<Vec<_>>());
    assert!(maps.len() >= 4);

    // An Imperial guard: helmet on the head (no hair), a cuirass on the chest.
    let mut guards: Vec<&RecordId> = db
        .iter::<Npc>()
        .filter(|(_, n)| n.class.eq_ignore_ascii_case("guard") && n.race.eq_ignore_ascii_case("imperial"))
        .filter(|(_, n)| a.default_equipment(n).iter().any(|(s, _)| *s == EquipSlot::Helmet))
        .map(|(id, _)| id)
        .collect();
    guards.sort();
    let (_, guard) = dress(&a, &vfs, guards[0].as_str(), None);
    // The helmet covers the hair: no hair BODY shows (open helmets fill the hair slot themselves).
    assert!(slot(&guard, "hair").is_none_or(|p| !p.starts_with("b_n_")), "{guard:?}");
    assert!(slot(&guard, "chest").is_some_and(|p| !p.starts_with("b_n_")), "{guard:?}");

    // Beast races: tail, no helmet or boots, the beast skeleton.
    for (id, race) in [("ajira", "khajiit"), ("huleeya", "argonian")] {
        let (r, v) = dress(&a, &vfs, id, None);
        assert_eq!(r.skeleton.as_str(), "meshes/base_animkna.nif");
        assert!(slot(&v, "tail").is_some(), "{id} ({race}) has no tail: {v:?}");
        assert!(!r.worn.iter().any(|w| matches!(a.wearable(w).map(|w| &w.parts), Some(p) if p.iter().any(|(s, _)| a.slots().name(*s) == Some("head")))));
    }

    // A woman: female skin where it exists, male parts as the fallback.
    let (r, v) = dress(&a, &vfs, "ajira", Some(Vec::new()));
    assert!(r.skeleton.as_str().contains("kna"));
    let female_parts = v.iter().filter(|(_, p)| p.contains("_f_")).count();
    let male_parts = v.iter().filter(|(_, p)| p.contains("_m_")).count();
    eprintln!("ajira naked: {female_parts} female, {male_parts} male fallback parts");
    assert!(female_parts > 0, "{v:?}");

    // Naked Fargoth: skin in every limb slot, head and hair.
    let (_, v) = dress(&a, &vfs, "fargoth", Some(Vec::new()));
    for s in ["head", "hair", "chest", "pelvis", "left_forearm", "right_forearm", "left_upper_leg", "right_foot", "neck"] {
        assert!(slot(&v, s).is_some(), "fargoth has no {s}: {v:?}");
    }

    // Rigid parts: count and mirror on the left in the guard's outfit.
    let rigid: Vec<_> = guard.iter().filter_map(|(_, p)| match &a.part(&RecordId::new(p)).unwrap().kind {
        PartKind::Rigid { joint, offset } => Some((joint.clone(), offset.matrix3.determinant() < 0.0)),
        PartKind::Skinned => None,
    }).collect();
    eprintln!("guard rigid parts (joint, mirrored): {rigid:?}");
    assert!(rigid.iter().all(|(j, m)| j.contains("Left") == *m));

    // Creatures: their model is skeleton and body; movers from the flags.
    for (id, mover) in [("guar", CreatureMover::Ground), ("cliff racer", CreatureMover::Fly), ("centurion_spider", CreatureMover::Ground)] {
        let r = a.recipe("CREA", &RecordId::new(id), &CharacterState::default()).unwrap();
        let body = a.part(&r.base[0]).unwrap();
        assert!(vfs.contains(&body.model) && body.model == r.skeleton, "{id}: {}", body.model);
        assert_eq!(a.creature_mover(id), Some(mover), "{id}");
        eprintln!("{id}: {} scale {}", r.skeleton, r.scale);
    }
}

/// Model-space bounds (min, max) of a rigid part on the rest skeleton: the
/// joint's rest transform, the part's offset (mirror and authored root),
/// then the part's own nodes, as `CookedSkinned::rigid` places it.
fn rigid_bounds(vfs: &Vfs, skeleton: &sf_anim::Skeleton, part: &BodyPart) -> Option<(glam::Vec3, glam::Vec3)> {
    let PartKind::Rigid { joint, offset } = &part.kind else { return None };
    let rest = skeleton.rest_model()[skeleton.find(joint.as_str()).unwrap_or_else(|| panic!("no joint {joint}"))];
    let model = mw_nif::load(&vfs.read(&part.model).unwrap()).unwrap();
    let world = mw_nif::world_transforms(&model);
    let (mut lo, mut hi) = (glam::Vec3::splat(f32::INFINITY), glam::Vec3::splat(f32::NEG_INFINITY));
    for (i, node) in model.nodes.iter().enumerate() {
        let Some(mesh) = node.mesh else { continue };
        let w = glam::Affine3A::from_cols_array(&world[i].to_cols_array());
        for p in &model.meshes[mesh].positions {
            let q = (rest * *offset).transform_point3(w.transform_point3(glam::Vec3::from_array(*p)));
            (lo, hi) = (lo.min(q), hi.max(q));
        }
    }
    Some((lo, hi))
}

/// Rigid parts sit where the original engine puts them: under their bone
/// through their model's authored root transform. Dressed Fargoth's pants
/// ankle parts hang from `Ankle` 37 cm up the shin by their root transform;
/// dropping it (placed-object semantics) put them underground and opened a
/// gap between the pants and the shoes (`character` lineup, 2026-10-12).
#[test]
fn rigid_parts_keep_their_root_transform_and_close_the_leg() {
    let Some((a, vfs)) = setup() else { return };
    let skeleton = mw_anim::rig::skeleton(&mw_nif::load(&vfs.read(&sf_vfs::VfsPath::new("meshes/base_anim.nif").unwrap()).unwrap()).unwrap()).unwrap();
    let (_, v) = dress(&a, &vfs, "fargoth", None);
    let bounds = |s: &str| {
        let p = slot(&v, s).unwrap_or_else(|| panic!("fargoth has nothing on {s}: {v:?}"));
        rigid_bounds(&vfs, &skeleton, a.part(&RecordId::new(p)).unwrap()).unwrap_or_else(|| panic!("{s}: {p} is not rigid"))
    };
    for side in ["left", "right"] {
        let chain = ["upper_leg", "lower_leg", "ankle", "foot"].map(|s| (s, bounds(&format!("{side}_{s}"))));
        eprintln!("fargoth {side} leg (min y, max y): {:?}", chain.map(|(s, (lo, hi))| (s, lo.y, hi.y)));
        // Each segment reaches down to the one below it (no gap over 1 cm).
        for w in chain.windows(2) {
            let ((upper, (lo, _)), (lower, (_, hi))) = (w[0], w[1]);
            assert!(lo.y <= hi.y + 0.01, "{side}: gap between {upper} (bottom {}) and {lower} (top {})", lo.y, hi.y);
        }
        let (_, (ankle_lo, _)) = chain[2];
        assert!(ankle_lo.y > -0.02, "{side} ankle part underground: {}", ankle_lo.y);
    }
    // Every rigid part of the outfit stays between the ground and head height
    // in the rest pose (the unscaled skeleton is 1.9 m tall).
    for (s, p) in &v {
        if let Some((lo, hi)) = rigid_bounds(&vfs, &skeleton, a.part(&RecordId::new(p)).unwrap()) {
            assert!(lo.y > -0.03 && hi.y < 2.2, "{s}: {p} spans {}..{}", lo.y, hi.y);
        }
    }
}
