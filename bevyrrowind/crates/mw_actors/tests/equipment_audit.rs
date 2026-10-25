//! Expected-vs-actual equipment audit over every `NPC_` of the install
//! (missing boots and armor pieces, 2026-10-12; `.sessions/missingparts.md`).
//!
//! Per NPC: the items auto-equip picks, the part each item's part
//! references should show (BODY lookup, female → male fallback), the
//! original renderer's slot rules re-run independently here (`OpenMW`
//! `NpcAnimation::updateParts`: slot list order, priorities clothing 2 /
//! armor 3 / skirt 8 / robe 24, robe and skirt reservations, parts without a
//! BODY reserve their slot, skin at priority 1, hair under a head part),
//! versus the outfit `resolve_recipe` gives. Then every distinct visible
//! part (all NPC outfits and every ARMO/CLOT wearable, both sexes) is built,
//! read, cooked as the bake does and decoded through its node filter: it
//! must keep geometry, and rigid parts must sit near their bone.
//!
//! Drop classes are printed with counts and examples. Legit (vanilla) ones:
//! beast races refuse boots and full helmets, item parts without a BODY,
//! parts outranked or reserved by a robe/skirt/higher priority, items in
//! slots the original never draws. Bug classes must stay at zero. Skips
//! (passes) when no Morrowind install is detected.

use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::sync::Arc;

use mw_actors::equip::{EquipSlot, Item};
use mw_actors::slots::{PART_SLOTS, PRT_COUNT, prt};
use mw_actors::{MwActors, wearable_id};
use mw_install::MorrowindInstall;
use sf_character::{BodyPart, CharacterSource, CharacterState, DecodedPart, PartKind, PartLookup, View, resolve_recipe};
use sf_model::{CookSettings, Cooked, CookedSkinned, ModelImporter};
use sf_records::RecordId;
use sf_vfs::{Vfs, VfsPath};
use tes3::esp::{Armor, Bodypart, Clothing, Npc, NpcFlags, Race, RaceFlags};

fn setup() -> Option<(MwActors, Arc<Vfs>)> {
    let Ok((install, _)) = MorrowindInstall::detect(None) else {
        eprintln!("no Morrowind install detected; skipping");
        return None;
    };
    let vfs = Arc::new(mw_vfs::mount_install(&install, Vfs::builder()).unwrap().vfs);
    let content = mw_records::load(&vfs, &install.game_files()).unwrap();
    Some((MwActors::new(Arc::new(content.db), Arc::clone(&vfs)), vfs))
}

/// Counts and a few examples per class.
#[derive(Default)]
struct Classes(BTreeMap<&'static str, (usize, Vec<String>)>);

impl Classes {
    fn add(&mut self, class: &'static str, example: impl FnOnce() -> String) {
        let e = self.0.entry(class).or_default();
        e.0 += 1;
        if e.1.len() < 6 {
            e.1.push(example());
        }
    }

    fn count(&self, class: &str) -> usize {
        self.0.get(class).map_or(0, |e| e.0)
    }

    fn print(&self, title: &str) {
        eprintln!("== {title}");
        for (class, (n, ex)) in &self.0 {
            eprintln!("  {class:<34} {n:>6}  e.g. {ex:?}");
        }
    }
}

/// The original engine's base priority per equipment slot (robe 11, skirt
/// 3, else 0); `None`: a slot it never draws (belt, amulet, rings, carried).
fn base_priority(slot: EquipSlot) -> Option<i32> {
    use EquipSlot as S;
    match slot {
        S::Robe => Some(11),
        S::Skirt => Some(3),
        S::Helmet
        | S::Cuirass
        | S::Greaves
        | S::LeftPauldron
        | S::RightPauldron
        | S::Boots
        | S::LeftGauntlet
        | S::RightGauntlet
        | S::Shirt
        | S::Pants => Some(0),
        S::CarriedLeft | S::Belt | S::Amulet | S::LeftRing | S::RightRing => None,
    }
}

const ROBE_RESERVES: [usize; 11] =
    [prt::GROIN, prt::SKIRT, prt::R_LEG, prt::L_LEG, prt::R_UPPER_ARM, prt::L_UPPER_ARM, prt::R_KNEE, prt::L_KNEE, prt::R_FOREARM, prt::L_FOREARM, prt::CUIRASS];
const SKIRT_RESERVES: [usize; 3] = [prt::GROIN, prt::R_LEG, prt::L_LEG];

/// The BODY an item part reference shows for a body (female: the female
/// part, else the male one), lowercase.
fn item_body(a: &MwActors, male: &str, female_part: &str, female: bool) -> Option<String> {
    let find = |n: &str| (!n.is_empty() && a.db().get::<Bodypart>(&RecordId::new(n)).is_some()).then(|| n.to_ascii_lowercase());
    if female && let Some(b) = find(female_part) {
        return Some(b);
    }
    find(male)
}

/// Who fills a part slot in the original renderer's rules.
#[derive(Clone, Debug, PartialEq)]
enum Fill {
    Empty,
    /// Priority, equipment slot, BODY id (`None`: reserved).
    Item(i32, EquipSlot, Option<String>),
}

/// `NpcAnimation::updateParts` for third person, over equipped items.
fn original_slots(a: &MwActors, equipped: &[(EquipSlot, Item<'_>)], female: bool) -> [Fill; PRT_COUNT] {
    let mut fill: [Fill; PRT_COUNT] = std::array::from_fn(|_| Fill::Empty);
    let prio = |f: &Fill| match f {
        Fill::Empty => 0,
        Fill::Item(p, ..) => *p,
    };
    // Slot list order is `EquipSlot` order (robe, skirt, helmet, ...).
    let mut sorted: Vec<&(EquipSlot, Item<'_>)> = equipped.iter().collect();
    sorted.sort_by_key(|(s, _)| *s);
    for (slot, item) in sorted {
        let Some(base) = base_priority(*slot) else { continue };
        let p = ((base + 1) << 1) + i32::from(matches!(item, Item::Armor(_)));
        for o in item.parts() {
            let k = o.biped_object_type as usize;
            let body = item_body(a, &o.male_bodypart, &o.female_bodypart, female);
            if p > prio(&fill[k]) {
                fill[k] = Fill::Item(p, *slot, body);
            }
        }
        let reserves: &[usize] = match slot {
            EquipSlot::Robe => &ROBE_RESERVES,
            EquipSlot::Skirt => &SKIRT_RESERVES,
            _ => &[],
        };
        for &k in reserves {
            if p > prio(&fill[k]) {
                fill[k] = Fill::Item(p, *slot, None);
            }
        }
    }
    fill
}

/// The BODY id of a part record id (`"<body>#<slot>"`).
fn body_of(part: &str) -> &str {
    part.split('#').next().unwrap_or_default()
}

/// Model-space bounds of a rigid part on a rest skeleton (as `CookedSkinned::rigid` places it).
fn rigid_bounds(model: &mw_nif::Model, skeleton: &sf_anim::Skeleton, joint: &str, offset: glam::Affine3A) -> Option<(glam::Vec3, glam::Vec3)> {
    let rest = skeleton.rest_model()[skeleton.find(joint)?];
    let world = mw_nif::world_transforms(model);
    let (mut lo, mut hi) = (glam::Vec3::splat(f32::INFINITY), glam::Vec3::splat(f32::NEG_INFINITY));
    for (i, node) in model.nodes.iter().enumerate() {
        let Some(mesh) = node.mesh else { continue };
        let w = glam::Affine3A::from_cols_array(&world[i].to_cols_array());
        for p in &model.meshes[mesh].positions {
            let q = (rest * offset).transform_point3(w.transform_point3(glam::Vec3::from_array(*p)));
            (lo, hi) = (lo.min(q), hi.max(q));
        }
    }
    lo.x.is_finite().then_some((lo, hi))
}

#[test]
#[allow(clippy::too_many_lines)]
#[allow(clippy::many_single_char_names)]
fn every_npc_shows_its_equipment() {
    let Some((a, vfs)) = setup() else { return };
    let db = a.db();
    let started = std::time::Instant::now();
    let mut npcs: Vec<(&RecordId, &Npc)> = db.iter::<Npc>().collect();
    npcs.sort_by(|x, y| x.0.cmp(y.0));
    let mut c = Classes::default();
    let rules = mw_actors::equip::EquipRules::from_db(db);
    // Visible parts to check, with the skeleton they hang on.
    let mut visible: BTreeMap<RecordId, &'static str> = BTreeMap::new();
    let mut outfits: Vec<(RecordId, Vec<RecordId>)> = Vec::new();
    let skeleton_of = |female: bool, beast: bool| match (beast, female) {
        (true, _) => mw_actors::skeleton::BEAST,
        (false, true) => mw_actors::skeleton::FEMALE,
        (false, false) => mw_actors::skeleton::MALE,
    };
    for (id, npc) in &npcs {
        let female = npc.npc_flags.contains(NpcFlags::FEMALE);
        let beast = db.get::<Race>(&RecordId::new(&npc.race)).is_some_and(|r| r.data.flags.contains(RaceFlags::BEAST_RACE));
        let equipped = a.default_equipment(npc);
        // Inventory armor/clothing auto-equip leaves out.
        let inventory: Vec<&str> = npc.inventory.iter().filter(|(n, _)| *n != 0).filter_map(|(_, i)| mw_actors::held::leveled_item_pick(db, i.as_str())).collect();
        if beast {
            let human = mw_actors::equip::auto_equip(db, &mw_actors::equip::EquipRules::from_db(db), inventory.iter().copied(), &mw_actors::equip::npc_skills(db, npc), false);
            for (slot, item) in &human {
                if !equipped.iter().any(|(s, i)| s == slot && i.id() == item.id()) && !item.fits_beast() {
                    c.add("legit: beast race refuses item", || format!("{id}: {}", item.id()));
                }
            }
        }
        // `InventoryStore::autoEquipArmor` truncates the unarmored rating to an integer.
        let skills = mw_actors::equip::npc_skills(db, npc);
        let unarmored = rules.unarmored_rating(&skills).max(0.0);
        for entry in &inventory {
            if let Some(armor) = db.get::<Armor>(&RecordId::new(entry))
                && armor.data.armor_type != tes3::esp::ArmorType::Shield
                && (!beast || Item::Armor(armor).fits_beast())
            {
                let rating = rules.effective_rating(armor, &skills);
                if rating > unarmored.trunc() && rating <= unarmored {
                    let worn = equipped.iter().any(|(_, i)| i.id() == armor.id);
                    let outranked = equipped.iter().any(|(_, i)| matches!(i, Item::Armor(o) if o.data.armor_type == armor.data.armor_type && o.id != armor.id));
                    if worn || outranked {
                        c.add("fixed: armor kept by the integer unarmored rating", || format!("{id}: {entry} ({rating:.2} vs {unarmored:.2})"));
                    } else {
                        c.add("BUG: armor skipped by a fractional unarmored rating", || format!("{id}: {entry} ({rating:.2} vs {unarmored:.2})"));
                    }
                }
            }
        }
        for entry in npc.inventory.iter().filter(|(n, _)| *n != 0).map(|(_, i)| i.as_str()) {
            if db.get::<tes3::esp::LeveledItem>(&RecordId::new(entry)).is_none() {
                continue;
            }
            let Some(pick) = mw_actors::held::leveled_item_pick(db, entry).and_then(|p| Item::find(db, &RecordId::new(p))) else { continue };
            if equipped.iter().any(|(_, i)| i.id() == pick.id()) {
                c.add("fixed: leveled-list item now worn (scaffold pick)", || format!("{id}: {entry} -> {}", pick.id()));
            } else {
                c.add("legit: leveled-list item picked, not worn (outranked/undrawn)", || format!("{id}: {entry} -> {}", pick.id()));
            }
        }
        let recipe = a.recipe("NPC_", id, &CharacterState::default()).unwrap();
        let outfit = resolve_recipe(&a, a.slots(), &recipe, View::ThirdPerson).unwrap();
        let actual: HashMap<&str, &str> = outfit.parts.iter().map(|(s, p)| (a.slots().name(*s).unwrap(), p.as_str())).collect();
        for (_, p) in &outfit.parts {
            visible.insert(p.clone(), skeleton_of(female, beast));
        }
        outfits.push(((*id).clone(), outfit.parts.iter().map(|(_, p)| p.clone()).collect::<Vec<_>>()));
        let expected = original_slots(&a, &equipped, female);
        // Per item part reference: shown, or why not.
        for (slot, item) in &equipped {
            for o in item.parts() {
                let k = o.biped_object_type as usize;
                let name = PART_SLOTS[k].slot;
                let body = item_body(&a, &o.male_bodypart, &o.female_bodypart, female);
                let ex = || format!("{id}: {} {name}", item.id());
                let shown = body.as_deref().is_some_and(|b| actual.get(name).is_some_and(|p| body_of(p) == b));
                if base_priority(*slot).is_none() {
                    if shown {
                        c.add("BUG: drawn in a slot the original never draws", ex);
                    } else {
                        c.add("legit: item slot never drawn (belt/amulet/ring)", ex);
                    }
                } else if body.is_none() {
                    c.add("legit: part reference without BODY (reserved)", ex);
                } else if shown {
                    c.add("shown", ex);
                } else {
                    match &expected[k] {
                        Fill::Item(_, s, b) if s == slot && b == &body => c.add("BUG: expected part not visible", || format!("{} (got {:?})", ex(), actual.get(name))),
                        Fill::Item(_, s, _) if matches!(s, EquipSlot::Robe | EquipSlot::Skirt) && s != slot => c.add("legit: covered by robe/skirt", ex),
                        Fill::Item(..) => c.add("legit: outranked by another item part", ex),
                        Fill::Empty => c.add("BUG: expected part not visible", ex),
                    }
                }
            }
        }
        // Slot by slot against the original rules (skin, head, hair included).
        for (k, fill) in expected.iter().enumerate() {
            let name = PART_SLOTS[k].slot;
            let want: Option<String> = match fill {
                Fill::Item(_, _, b) => b.clone(),
                Fill::Empty if k == prt::HEAD => recipe.head.as_ref().map(|p| body_of(p.as_str()).to_owned()),
                Fill::Empty if k == prt::HAIR => {
                    let head_prio = match &expected[prt::HEAD] {
                        Fill::Item(p, ..) => *p,
                        Fill::Empty => 1,
                    };
                    recipe.hair.as_ref().filter(|_| head_prio <= 1).map(|p| body_of(p.as_str()).to_owned())
                }
                Fill::Empty => recipe.base.iter().find(|p| p.as_str().ends_with(&format!("#{name}"))).map(|p| body_of(p.as_str()).to_owned()),
            };
            let got = actual.get(name).map(|p| body_of(p).to_owned());
            if want != got {
                c.add("BUG: slot differs from the original rules", || format!("{id}: {name} want {want:?} got {got:?}"));
            }
        }
    }
    // Every wearable's parts too (forced outfits: `character id+item`).
    for (item_id, item) in db.iter::<Armor>().map(|(i, a)| (i, Item::Armor(a))).chain(db.iter::<Clothing>().map(|(i, c)| (i, Item::Clothing(c)))) {
        for female in [false, true] {
            let w = a.wearable(&wearable_id(item_id.as_str(), female)).unwrap();
            for (_, p) in &w.parts {
                // Beast-wearable items show on beasts too.
                let skeleton = if item.fits_beast() && !female { mw_actors::skeleton::BEAST } else { skeleton_of(female, false) };
                visible.entry(p.clone()).or_insert(skeleton);
            }
        }
    }
    eprintln!("{} NPCs audited, {} distinct visible parts ({:.1} s)", npcs.len(), visible.len(), started.elapsed().as_secs_f32());

    // Part by part: buildable, readable, keeps geometry after filter + cook, placed near its bone.
    let mut parts = Classes::default();
    let mut skeletons: HashMap<&str, sf_anim::Skeleton> = HashMap::new();
    for path in [mw_actors::skeleton::MALE, mw_actors::skeleton::FEMALE, mw_actors::skeleton::BEAST] {
        let model = mw_nif::load(&vfs.read(&VfsPath::new(path).unwrap()).unwrap()).unwrap();
        skeletons.insert(path, mw_anim::rig::skeleton(&model).unwrap());
    }
    let mut cooked: HashMap<VfsPath, Option<Cooked>> = HashMap::new();
    let mut rig_cooked: std::collections::HashSet<VfsPath> = std::collections::HashSet::new();
    for (id, skeleton) in &visible {
        let ex = || id.as_str().to_owned();
        let Some(part) = a.part(id) else {
            parts.add("BUG: part record not buildable", ex);
            continue;
        };
        let part: &BodyPart = part;
        let Ok(bytes) = vfs.read(&part.model) else {
            parts.add("BUG: model missing from the VFS", || format!("{id}: {}", part.model));
            continue;
        };
        let Ok(nif) = mw_nif::load(&bytes) else {
            parts.add("BUG: model unreadable", || format!("{id}: {}", part.model));
            continue;
        };
        // As the bake and `MwCharacters::cooked` do: `mesh.rig` for the
        // rigid parts the plain cook skips (`ActorSets::rigid_parts`), else `mesh.cook`.
        let cook = cooked.entry(part.model.clone()).or_insert_with(|| {
            let model = mw_bake::NifImporter.import(part.model.as_str(), &bytes).ok()?;
            if !nif.had_skin && mw_bake::anim::rigid_part_needs_rig(&model) {
                rig_cooked.insert(part.model.clone());
                sf_model::cook_rig(&model, &CookSettings::default()).ok()
            } else {
                sf_model::cook(&model, &CookSettings::default()).ok()
            }
        });
        if rig_cooked.contains(&part.model) {
            parts.add("fixed: rigid part cooked through mesh.rig (was skipped)", ex);
        }
        let Some(cook) = cook else {
            parts.add("BUG: model does not cook", || format!("{id}: {}", part.model));
            continue;
        };
        let skinned = match (&part.kind, &*cook) {
            (PartKind::Rigid { joint, offset }, c) => CookedSkinned::rigid(c, joint, *offset).map_err(|e| e.to_string()),
            (PartKind::Skinned, Cooked::Skinned(s)) => Ok(Some(s.clone())),
            (PartKind::Skinned, _) => Ok(None),
        };
        let Ok(Some(skinned)) = skinned else {
            let cooked_as = match &*cook {
                Cooked::Skipped { reason } => format!("skipped: {reason}"),
                Cooked::Static(_) => "static".to_owned(),
                Cooked::Skinned(_) => "skinned".to_owned(),
            };
            parts.add("BUG: cook kind differs from the part kind", || format!("{id}: {} rigid={} cooked {cooked_as} {skinned:?}", part.model, matches!(part.kind, PartKind::Rigid { .. })));
            continue;
        };
        let decoded = DecodedPart::decode(&skinned, 0, Some(part)).unwrap();
        let vertices: usize = decoded.submeshes.iter().map(|s| s.vertices.len()).sum();
        // Vanilla data: Ten Pace Boots' one skinned shape `Tri Foot` matches
        // neither foot filter (UESP: "makes the wearer's feet vanish", fixed
        // by the Morrowind Patch Project), and the werewolf hair model has no
        // mesh. The original renderers show nothing either.
        if vertices == 0 && (id.as_str().starts_with("a_tenpaceboot#") || nif.nodes.iter().all(|n| n.mesh.is_none())) {
            parts.add("legit: vanilla model shows nothing", ex);
            continue;
        }
        if vertices == 0 {
            parts.add("BUG: no geometry after node filter + cook", || format!("{id}: {} filter {:?}; mesh nodes {:?}", part.model, part.node_filter, nif.nodes.iter().filter(|n| n.mesh.is_some()).map(|n| (n.name.as_str(), nif.meshes[n.mesh.unwrap()].positions.len(), nif.meshes[n.mesh.unwrap()].skinned)).collect::<Vec<_>>()));
            continue;
        }
        let skeleton = &skeletons[skeleton];
        match &part.kind {
            PartKind::Skinned => {
                let unknown: BTreeSet<&str> = skinned.joints.iter().map(|j| j.name.as_str()).filter(|n| skeleton.find(n).is_none()).collect();
                if !unknown.is_empty() {
                    parts.add("note: skin joints not in the skeleton", || format!("{id}: {unknown:?}"));
                }
                parts.add("ok: skinned", ex);
            }
            PartKind::Rigid { joint, offset } => {
                let Some((lo, hi)) = rigid_bounds(&nif, skeleton, joint, *offset) else {
                    parts.add("BUG: rigid joint not in the skeleton", || format!("{id}: {joint}"));
                    continue;
                };
                let rest = skeleton.rest_model()[skeleton.find(joint).unwrap()];
                let center = (lo + hi) * 0.5;
                let far = (center - glam::Vec3::from(rest.translation)).length();
                // The cooked part where assembly puts it (joint · inverse bind · v).
                let bind = glam::Affine3A::from_mat4(glam::Mat4::from_cols_array(&skinned.joints[0].inverse_bind));
                let (mut clo, mut chi) = (glam::Vec3::splat(f32::INFINITY), glam::Vec3::splat(f32::NEG_INFINITY));
                for v in decoded.submeshes.iter().flat_map(|s| &s.vertices) {
                    let q = (rest * bind).transform_point3(glam::Vec3::from_array(v.position));
                    (clo, chi) = (clo.min(q), chi.max(q));
                }
                if (clo - lo).abs().max_element() > 0.02 || (chi - hi).abs().max_element() > 0.02 {
                    parts.add("BUG: cooked rigid part off its NIF placement", || format!("{id}: cooked {clo:.2}..{chi:.2} vs NIF {lo:.2}..{hi:.2}"));
                } else if lo.y < -0.06 || hi.y > 2.3 || far > 0.6 {
                    parts.add("BUG?: rigid part far from its bone", || format!("{id}: y {:.2}..{:.2}, center {far:.2} m from {joint}", lo.y, hi.y));
                } else {
                    parts.add("ok: rigid", ex);
                }
            }
        }
    }
    let rigged = |p: &RecordId| a.part(p).is_some_and(|part| rig_cooked.contains(&part.model));
    let npcs_fixed = outfits.iter().filter(|(_, ps)| ps.iter().any(rigged)).count();
    eprintln!("NPCs whose default outfit shows a part only the mesh.rig cook renders: {npcs_fixed}");
    assert!(npcs_fixed > 0);
    c.print("item part references of auto-equipped NPCs");
    parts.print("visible parts (NPC outfits + every wearable)");
    eprintln!("total {:.1} s", started.elapsed().as_secs_f32());
    for class in c.0.keys().chain(parts.0.keys()).filter(|k| k.starts_with("BUG:")) {
        let n = c.count(class) + parts.count(class);
        assert_eq!(n, 0, "{class}: {n}");
    }
}

/// The `character` lineup's 6th actor (`@nord,+iron_cuirass+...+iron_boots+...`,
/// `shots/smoothlive/lineup_smooth.png`): Abbard the Wild in iron armor.
#[test]
fn abbard_the_wild_in_iron_armor_shows_every_piece() {
    let Some((a, vfs)) = setup() else { return };
    let items = ["iron_cuirass", "iron_greaves", "iron_pauldron_left", "iron_pauldron_right", "iron boots", "iron_gauntlet_left", "iron_gauntlet_right"];
    for item in items {
        let found = Item::find(a.db(), &RecordId::new(item)).unwrap_or_else(|| panic!("no item {item}"));
        let refs: Vec<(usize, &str, &str)> = found.parts().iter().map(|o| (o.biped_object_type as usize, o.male_bodypart.as_str(), o.female_bodypart.as_str())).collect();
        eprintln!("{item}: {refs:?}");
    }
    let state = CharacterState { worn: Some(items.iter().map(|i| RecordId::new(i)).collect()), held: None };
    let recipe = a.recipe("NPC_", &RecordId::new("abbard the wild"), &state).unwrap();
    let outfit = resolve_recipe(&a, a.slots(), &recipe, View::ThirdPerson).unwrap();
    eprintln!("skeleton {} worn {:?}", recipe.skeleton, recipe.worn);
    for (s, p) in &outfit.parts {
        let part = a.part(p).unwrap();
        eprintln!("  {:<16} {p:<40} {} {:?} filter {:?}", a.slots().name(*s).unwrap(), part.model, part.kind, part.node_filter);
        assert!(vfs.contains(&part.model));
    }
    let names: Vec<&str> = outfit.parts.iter().map(|(s, _)| a.slots().name(*s).unwrap()).collect();
    for (slot, body) in [("left_foot", "a_iron_boot_f"), ("right_foot", "a_iron_boot_f"), ("left_ankle", "a_iron_boot_a"), ("right_ankle", "a_iron_boot_a")] {
        let p = outfit.parts.iter().find(|(s, _)| a.slots().name(*s) == Some(slot)).map(|(_, p)| p.as_str());
        eprintln!("{slot}: {p:?} (expected a {body}* part) of {names:?}");
    }
}

/// Item ids that tools and visual checks dress actors in (`smooth_survey`
/// outfits, docs/visual-checks.md CH/SB scenes). `CharacterState::worn`
/// skips unknown ids, so a typo silently undresses a slot: the SB-1 lineup's
/// `iron_boots` (the record is `iron boots`) left the Nord's shins and feet
/// bare.
#[test]
fn named_outfit_items_exist() {
    let Some((a, _)) = setup() else { return };
    let named = [
        "netch_leather_cuirass", "netch_leather_greaves", "netch_leather_pauldron_left", "netch_leather_pauldron_right", "netch_leather_boots",
        "netch_leather_gauntlet_left", "netch_leather_gauntlet_right", "glass_cuirass", "glass_greaves", "glass_pauldron_left", "glass_pauldron_right",
        "glass_boots", "glass_bracer_left", "glass_bracer_right", "bonemold_cuirass", "bonemold_greaves", "bonemold_pauldron_l", "bonemold_pauldron_r",
        "bonemold_boots", "bonemold_bracer_left", "bonemold_bracer_right", "iron_cuirass", "iron_greaves", "iron_pauldron_left", "iron_pauldron_right",
        "iron boots", "iron_gauntlet_left", "iron_gauntlet_right", "daedric_cuirass", "daedric_greaves", "daedric_pauldron_left", "daedric_pauldron_right",
        "daedric_boots", "daedric_gauntlet_left", "daedric_gauntlet_right", "common_robe_01", "common_shoes_01", "common_skirt_01", "common_shirt_01",
        "common_pants_01",
    ];
    let mut missing = Vec::new();
    for id in named {
        if Item::find(a.db(), &RecordId::new(id)).is_none() {
            let loose = |s: &str| s.replace(['_', ' '], "");
            let near: Vec<String> = a
                .db()
                .iter::<Armor>()
                .map(|(i, _)| i.as_str().to_owned())
                .chain(a.db().iter::<Clothing>().map(|(i, _)| i.as_str().to_owned()))
                .filter(|i| loose(i) == loose(id))
                .collect();
            missing.push(format!("{id} (did you mean {near:?})"));
        }
    }
    assert!(missing.is_empty(), "unknown item ids: {missing:?}");
}

/// The bake cooks the rigid part models the plain cook skips (a one-key
/// transform controller on the root) as `mesh.rig`, which character assembly
/// prefers: before, iron boots' feet, common shoes 1 and 2 and 15 more models
/// (36 parts) never showed (2026-10-12).
#[test]
fn bake_cooks_animated_rigid_parts_as_rigs() {
    let Ok((install, _)) = MorrowindInstall::detect(None) else { return };
    let vfs = Arc::new(mw_vfs::mount_install(&install, Vfs::builder()).unwrap().vfs);
    let db = Arc::new(mw_records::load(&vfs, &install.game_files()).unwrap().db);
    let sets = mw_bake::anim::actor_sets(&db, &vfs);
    eprintln!("{} rigid part rigs: {:?}", sets.rigid_parts.len(), sets.rigid_parts);
    for model in ["meshes/a/a_iron_boot_f.nif", "meshes/c/c_shoes_common1.nif", "meshes/c/c_shoes_common2.nif", "meshes/a/a_glass_boots_a.nif"] {
        assert!(sets.rigid_parts.contains(model), "{model} not cooked as mesh.rig");
    }
    let keys = sets.body_rig_keys();
    assert!(keys.iter().any(|k| k.subject == "meshes/a/a_iron_boot_f.nif"), "rig keys miss the iron boot");
    // Skinned and plain rigid parts keep their `mesh.cook` product.
    assert!(!sets.rigid_parts.contains("meshes/a/a_iron_skinned.nif") && !sets.rigid_parts.contains("meshes/a/a_iron_boot_a.nif"));
}
