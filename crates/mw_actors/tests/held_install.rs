//! Held items on the real game data: every NPC_ by drawn stance, guards'
//! weapons, Redoran shields, torches at night, held models in the VFS,
//! weapon-carrying creatures, darkness of interiors. Skips (passes) when no
//! Morrowind install is detected.

use std::collections::BTreeMap;
use std::sync::Arc;

use mw_actors::MwActors;
use mw_actors::held::{self, DayCycle, HeldContext, bone};
use mw_graphs::Stance;
use mw_install::MorrowindInstall;
use sf_character::{CharacterSource, CharacterState, Readiness};
use sf_records::RecordId;
use sf_vfs::Vfs;
use tes3::esp::{Cell, CellFlags, Creature, CreatureFlags, Npc};

fn setup() -> Option<(MwActors, Arc<Vfs>)> {
    let Ok((install, _)) = MorrowindInstall::detect(None) else {
        eprintln!("no Morrowind install detected; skipping");
        return None;
    };
    let vfs = Arc::new(mw_vfs::mount_install(&install, Vfs::builder()).unwrap().vfs);
    let content = mw_records::load(&vfs, &install.game_files()).unwrap();
    Some((MwActors::new(Arc::new(content.db), Arc::clone(&vfs)), vfs))
}

#[test]
#[allow(clippy::too_many_lines)]
fn npcs_hold_weapons_shields_and_torches() {
    let Some((a, vfs)) = setup() else { return };
    let db = a.db();
    let day = HeldContext::default();
    let night = HeldContext { dark: true, in_combat: false };
    let mut stances: BTreeMap<String, usize> = BTreeMap::new();
    let (mut npcs, mut shields, mut torches_night, mut torches_day, mut missing) = (0, 0, 0, 0, 0);
    let mut guards: BTreeMap<String, usize> = BTreeMap::new();
    let mut guard_count = 0;
    let mut redoran_shield = None;
    let mut torch_example = None;
    for (id, npc) in db.iter::<Npc>() {
        npcs += 1;
        let l = a.npc_loadout(npc, day);
        let stance = Stance::of_weapon(Readiness::Drawn, l.weapon_type());
        *stances.entry(stance.name().to_owned()).or_default() += 1;
        if npc.class.eq_ignore_ascii_case("guard") {
            guard_count += 1;
            *guards.entry(l.weapon_type().map_or("none".to_owned(), |t| format!("{t:?}"))).or_default() += 1;
            if l.weapon.is_none() {
                eprintln!("unarmed guard {id}: {:?}", npc.inventory.iter().map(|(n, i)| format!("{n} {}", i.as_str())).collect::<Vec<_>>());
            }
        }
        if l.shield().is_some() {
            shields += 1;
            if redoran_shield.is_none() && npc.faction.eq_ignore_ascii_case("Redoran") {
                redoran_shield = Some((id.clone(), l.shield().unwrap().id.clone()));
            }
        }
        torches_day += usize::from(l.light().is_some());
        let n = a.npc_loadout(npc, night);
        if let Some(light) = n.light() {
            torches_night += 1;
            torch_example.get_or_insert_with(|| (id.clone(), light.id.clone()));
        }
        // Every chosen item's model is in the VFS (`held_items` skips missing ones).
        let expected = usize::from(n.weapon.is_some()) + usize::from(n.carried_left.is_some());
        let items = a.held("NPC_", id, night).unwrap();
        if items.len() != expected {
            missing += 1;
            eprintln!("{id}: {} of {expected} held models found", items.len());
        }
    }
    eprintln!("{npcs} NPC_; drawn stances {stances:?}");
    eprintln!("{guard_count} guards (class Guard): {guards:?}");
    eprintln!(
        "shields by day {shields}; lights by day {torches_day}, at night {torches_night} (e.g. {torch_example:?}); Redoran shield {redoran_shield:?}"
    );
    assert_eq!(torches_day, 0, "lights only in the dark");
    assert!(missing == 0, "{missing} NPCs miss held models");
    assert!(guards.get("none").copied().unwrap_or(0) * 10 < guard_count, "most guards hold a weapon");
    assert!(redoran_shield.is_some(), "some Redoran carries a shield");
    assert!(torches_night > 0, "some NPC carries a torch at night");
    assert!(stances.len() >= 5, "stances {stances:?}");

    // A guard's recipe holds its weapon (default context: daylight).
    let guard = db.iter::<Npc>().find(|(_, n)| n.class.eq_ignore_ascii_case("guard") && a.npc_loadout(n, day).weapon.is_some()).unwrap().0.clone();
    let recipe = a.recipe("NPC_", &guard, &CharacterState::default()).unwrap();
    let weapon = recipe.held.iter().find(|h| h.drawn.as_ref().is_some_and(|m| m.socket == bone::WEAPON)).expect("guard weapon");
    assert!(weapon.sheathed.is_none() && vfs.contains(&weapon.model));
    eprintln!("guard {guard}: {:?}", recipe.held.iter().map(|h| (h.item.as_str(), h.model.as_str())).collect::<Vec<_>>());

    // The torch item carries its light at the flame node.
    if let Some((id, _)) = &torch_example {
        let items = a.held("NPC_", id, night).unwrap();
        let record = a.db().get::<tes3::esp::Light>(&RecordId::new(&torch_example.as_ref().unwrap().1)).unwrap();
        eprintln!("torch record {:?} mesh {}", record.data, record.mesh);
        let torch = items.iter().find(|h| h.light.is_some()).expect("torch light");
        eprintln!("{id} torch {}: {:?}", torch.model, torch.light);
    }

    // Held models' offsets: root transforms and `BoneOffset` nodes.
    let (mut boneoffset, mut rooted, mut models) = (0, 0, 0);
    let mut seen = std::collections::HashSet::new();
    for (_, npc) in db.iter::<Npc>() {
        for l in [a.npc_loadout(npc, night), a.npc_loadout(npc, day)] {
            let meshes = l.weapon.map(|w| w.mesh.clone()).into_iter().chain(l.light().map(|x| x.mesh.clone()));
            for mesh in meshes.chain(l.shield().map(|s| held::shield_mesh(db, s, false).to_owned())) {
                let Some(path) = mw_world::model_path(&mesh).and_then(|p| sf_vfs::VfsPath::new(&p).ok()) else { continue };
                if !seen.insert(path.clone()) {
                    continue;
                }
                let Ok(bytes) = vfs.read(&path) else { continue };
                let model = mw_nif::load(&bytes).unwrap();
                let p = mw_actors::PartModel::of(&model);
                models += 1;
                boneoffset += usize::from(p.bone_offset.is_some());
                rooted += usize::from(!p.root.abs_diff_eq(glam::Affine3A::IDENTITY, 1e-4));
            }
        }
    }
    eprintln!("{models} held models: {rooted} with a root transform, {boneoffset} with a BoneOffset node");

    // Skeleton bones.
    let skeleton = mw_nif::load(&vfs.read(&sf_vfs::VfsPath::new(mw_actors::skeleton::MALE).unwrap()).unwrap()).unwrap();
    let has = |name: &str| skeleton.nodes.iter().any(|n| n.name.eq_ignore_ascii_case(name));
    eprintln!("base_anim: Weapon Bone {}, Shield Bone {}, Weapon Bone Left {}", has(bone::WEAPON), has(bone::SHIELD), has("Weapon Bone Left"));
    assert!(has(bone::WEAPON) && has(bone::SHIELD));

    // Weapon-carrying creatures.
    let armed: Vec<String> = db
        .iter::<Creature>()
        .filter(|(_, c)| c.creature_flags.contains(CreatureFlags::WEAPON_AND_SHIELD))
        .filter(|(_, c)| a.creature_loadout(c, day).weapon.is_some())
        .map(|(id, _)| id.as_str().to_owned())
        .collect();
    let flagged = db.iter::<Creature>().filter(|(_, c)| c.creature_flags.contains(CreatureFlags::WEAPON_AND_SHIELD)).count();
    eprintln!("creatures: {flagged} with the weapon flag, {} hold a weapon (e.g. {:?})", armed.len(), armed.iter().take(5).collect::<Vec<_>>());

    // Interiors where torches come out.
    let c = DayCycle::default();
    let (mut interiors, mut dark) = (0, 0);
    for (_, cell) in db.iter::<Cell>() {
        if cell.data.flags.contains(CellFlags::IS_INTERIOR) && !cell.data.flags.contains(CellFlags::BEHAVES_LIKE_EXTERIOR) {
            interiors += 1;
            dark += usize::from(held::is_dark(cell, 12.0, c));
        }
    }
    eprintln!("{dark} of {interiors} interiors are dark (torches out)");
    let census = db.get::<Cell>(&RecordId::new("Seyda Neen, Census and Excise Office")).map(|cell| held::is_dark(cell, 12.0, c));
    eprintln!("Census and Excise Office dark: {census:?}");
}
