//! [`MwActors`]: the `CharacterSource` and `PartLookup` for `NPC_`/`CREA`.

use std::collections::HashMap;
use std::sync::{Arc, Mutex, OnceLock};

use glam::{Affine3A, Vec3};
use sf_character::{
    AnimGraphRef, BodyPart, CharacterError, CharacterRecipe, CharacterSource, CharacterState, FirstPersonRecipe, HeldItem, Layer, PartKind,
    PartLookup, PartRef, RigMap, RigRef, ShapeWeights, SlotId, SlotRegistry, TintParams, Wearable, WearableRef,
};
use sf_records::{RecordDb, RecordId};
use sf_vfs::{Vfs, VfsPath};
use tes3::esp::{
    Armor, Bodypart, BodypartFlags, BodypartType, Clothing, ClothingType, Creature, CreatureFlags, LeveledCreature, Npc, NpcFlags, Race, RaceFlags,
};

use crate::equip::{self, EquipRules, EquipSlot, Item};
use crate::held::{self, HeldContext, Loadout};
use crate::slots::{self, FIRST_PERSON, PART_SLOTS, PRT_COUNT, prt};

/// Record kinds [`MwActors::recipe`] takes.
pub const NPC_KIND: &str = "NPC_";
pub const CREATURE_KIND: &str = "CREA";
/// Leveled creature lists (placed like creatures; see [`leveled_pick`]).
pub const LEVELED_CREATURE_KIND: &str = "LEVC";

/// Skeletons (VFS paths) by body type.
pub mod skeleton {
    pub const MALE: &str = "meshes/base_anim.nif";
    pub const FEMALE: &str = "meshes/base_anim_female.nif";
    pub const BEAST: &str = "meshes/base_animkna.nif";
    pub const FIRST_PERSON: &str = "meshes/base_anim.1st.nif";
    pub const FIRST_PERSON_FEMALE: &str = "meshes/base_anim_female.1st.nif";
    pub const FIRST_PERSON_BEAST: &str = "meshes/base_animkna.1st.nif";
}

/// Part priorities of the original renderer (base, clothing, armor, skirt,
/// robe), as layers: a higher priority is a later layer.
pub mod layer {
    use sf_character::Layer;
    pub const CLOTHING: Layer = Layer::CLOTHING;
    pub const ARMOR: Layer = Layer::ARMOR;
    pub const SKIRT: Layer = Layer(175);
    pub const ROBE: Layer = Layer::OUTER;
}

/// What the adapter reads from a part model (`None` from the probe:
/// unreadable).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PartModel {
    /// Skinned (bound to the skeleton's bones) rather than rigid (bolted to
    /// its slot's bone).
    pub skinned: bool,
    /// The model root's authored transform (engine space). The original
    /// engine keeps it when it bolts a rigid part under its bone (placed
    /// objects drop it, `mw_nif::Model::root_transform`): pants ankle parts
    /// hang 37 cm up the shin from `Ankle` through it.
    pub root: Affine3A,
    /// Model-space position (root transform included) of a node named
    /// `BoneOffset`, which held models may carry ([`crate::held::mount_offset`]).
    pub bone_offset: Option<Vec3>,
}

impl PartModel {
    /// A part model with an identity root.
    pub fn new(skinned: bool) -> Self {
        Self { skinned, root: Affine3A::IDENTITY, bone_offset: None }
    }

    /// Reads a converted NIF.
    pub fn of(model: &mw_nif::Model) -> Self {
        let t = model.root_transform;
        let root = Affine3A::from_scale_rotation_translation(Vec3::splat(t.scale), glam::Quat::from_array(t.rotation), Vec3::from_array(t.translation));
        let bone_offset = model.nodes.iter().position(|n| n.name.eq_ignore_ascii_case("BoneOffset")).map(|i| {
            let world = mw_nif::world_transforms(model)[i];
            root.transform_point3(Vec3::from_array(world.translation.to_array()))
        });
        Self { skinned: model.had_skin, root, bone_offset }
    }
}

/// Reads a part model (`None`: unreadable). Rigid parts are bolted to their
/// slot's bone through their root transform.
pub type ModelProbe = dyn Fn(&VfsPath) -> Option<PartModel> + Send + Sync;

/// Animation graphs and the first-person field of view the recipes name.
#[derive(Clone, Debug)]
pub struct ActorConfig {
    pub humanoid_graph: AnimGraphRef,
    pub creature_graph: AnimGraphRef,
    pub first_person_graph: AnimGraphRef,
    pub first_person_fov_y_deg: f32,
}

impl Default for ActorConfig {
    /// The graph templates `mw_graphs` ships (`mw_graphs::Template::path`).
    fn default() -> Self {
        let p = |s: &str| VfsPath::new(s).expect("valid path");
        Self {
            humanoid_graph: p("animation/graphs/mw_humanoid.animgraph.ron"),
            creature_graph: p("animation/graphs/mw_creature.animgraph.ron"),
            first_person_graph: p("animation/graphs/mw_first_person.animgraph.ron"),
            first_person_fov_y_deg: 60.0,
        }
    }
}

enum PartSource {
    /// A `BODY` record in a part slot.
    Body { body: RecordId, prt: usize },
    /// A creature's whole model.
    Creature { model: VfsPath },
}

struct PartEntry {
    source: PartSource,
    part: OnceLock<Option<BodyPart>>,
}

/// Skin `BODY` ids per part slot for one (race, female, first person).
type SkinTable = [Option<RecordId>; PRT_COUNT];

/// The Morrowind character adapter. Part records are built on first use
/// (each needs its model read to tell skinned from rigid); wearables and
/// skin tables at construction.
pub struct MwActors {
    db: Arc<RecordDb>,
    vfs: Arc<Vfs>,
    probe: Box<ModelProbe>,
    slots: SlotRegistry,
    prt_slots: [SlotId; PRT_COUNT],
    body_slot: SlotId,
    rules: EquipRules,
    parts: HashMap<RecordId, PartEntry>,
    wearables: HashMap<RecordId, Wearable>,
    skins: HashMap<(String, bool, bool), SkinTable>,
    /// Held models probed so far: their mount offset (`None`: not in the VFS).
    held_models: Mutex<HashMap<VfsPath, Option<Affine3A>>>,
    pub config: ActorConfig,
}

/// Part record id of a `BODY` in a slot: `"<body id>#<slot name>"`.
pub fn part_id(body: &str, prt: usize) -> RecordId {
    RecordId::new(&format!("{body}#{}", PART_SLOTS[prt].slot))
}

/// Part record id of a creature model: `"<model path>#body"`.
pub fn creature_part_id(model: &VfsPath) -> RecordId {
    RecordId::new(&format!("{}#{}", model.as_str(), slots::slot::BODY))
}

/// Wearable record id of an item for one body: `"<item id>#m"` or `"#f"`.
pub fn wearable_id(item: &str, female: bool) -> RecordId {
    RecordId::new(&format!("{item}#{}", if female { 'f' } else { 'm' }))
}

fn vfs_model(mesh: &str) -> Option<VfsPath> {
    VfsPath::new(&mw_world::model_path(mesh)?).ok()
}

impl MwActors {
    /// The adapter over loaded content, probing part models through `vfs`.
    pub fn new(db: Arc<RecordDb>, vfs: Arc<Vfs>) -> Self {
        let reader = Arc::clone(&vfs);
        let probe = move |path: &VfsPath| reader.read(path).ok().and_then(|b| mw_nif::load(&b).ok()).map(|m| PartModel::of(&m));
        Self::with_probe(db, vfs, Box::new(probe))
    }

    /// As [`Self::new`] with another part model probe (tests, tools).
    pub fn with_probe(db: Arc<RecordDb>, vfs: Arc<Vfs>, probe: Box<ModelProbe>) -> Self {
        let mut slots = SlotRegistry::new();
        let (prt_slots, body_slot) = slots::register(&mut slots);
        let rules = EquipRules::from_db(&db);
        let mut parts = HashMap::new();
        let mut add = |id: RecordId, source: PartSource| {
            parts.entry(id).or_insert_with(|| PartEntry { source, part: OnceLock::new() });
        };
        for (id, body) in db.iter::<Bodypart>() {
            for &p in slots::part_slots(body.data.part) {
                add(part_id(id.as_str(), p), PartSource::Body { body: id.clone(), prt: p });
            }
        }
        let item_parts = db
            .iter::<Armor>()
            .flat_map(|(_, a)| &a.biped_objects)
            .chain(db.iter::<Clothing>().flat_map(|(_, c)| &c.biped_objects));
        for o in item_parts {
            let p = o.biped_object_type as usize;
            for body in [&o.male_bodypart, &o.female_bodypart] {
                for b in [body.clone(), format!("{body}.1st")] {
                    let b = RecordId::new(&b);
                    if !body.is_empty() && db.get::<Bodypart>(&b).is_some() {
                        add(part_id(b.as_str(), p), PartSource::Body { body: b, prt: p });
                    }
                }
            }
        }
        for (_, crea) in db.iter::<Creature>() {
            if let Some(model) = vfs_model(&crea.mesh) {
                add(creature_part_id(&model), PartSource::Creature { model });
            }
        }
        let mut this = Self {
            skins: skin_tables(&db),
            db,
            vfs,
            probe,
            slots,
            prt_slots,
            body_slot,
            rules,
            parts,
            wearables: HashMap::new(),
            held_models: Mutex::new(HashMap::new()),
            config: ActorConfig::default(),
        };
        let mut wearables = HashMap::new();
        for (id, item) in this.db.iter::<Armor>().map(|(i, a)| (i, Item::Armor(a))).chain(this.db.iter::<Clothing>().map(|(i, c)| (i, Item::Clothing(c)))) {
            for female in [false, true] {
                wearables.insert(wearable_id(id.as_str(), female), this.wearable(item, female));
            }
        }
        this.wearables = wearables;
        this
    }

    /// The slot registry recipes resolve against.
    pub fn slots(&self) -> &SlotRegistry {
        &self.slots
    }

    pub fn db(&self) -> &RecordDb {
        &self.db
    }

    /// Number of part records known (built or not).
    pub fn part_count(&self) -> usize {
        self.parts.len()
    }

    pub fn wearable_count(&self) -> usize {
        self.wearables.len()
    }

    /// The part a `BODY` gives in a slot, if that `BODY` exists.
    fn body_part(&self, body: &str, p: usize) -> Option<PartRef> {
        let id = part_id(&body.to_ascii_lowercase(), p);
        self.parts.contains_key(&id).then_some(id)
    }

    /// `BODY` lookup by id (`None` for an empty or unknown id).
    fn find_body(&self, id: &str) -> Option<&Bodypart> {
        (!id.is_empty()).then(|| self.db.get::<Bodypart>(&RecordId::new(id))).flatten()
    }

    /// An item's look for one body. Parts: the female `BODY` when the body is
    /// female and the item has one, else the male one. Slots without a `BODY`
    /// are reserved (hidden), as are a robe's and a skirt's cover slots, and
    /// the hair under anything on the head.
    fn wearable(&self, item: Item<'_>, female: bool) -> Wearable {
        let (layer, reserve): (Layer, &[usize]) = match item {
            Item::Armor(_) => (layer::ARMOR, &[]),
            Item::Clothing(c) => match c.data.clothing_type {
                ClothingType::Robe => (
                    layer::ROBE,
                    &[
                        prt::GROIN,
                        prt::SKIRT,
                        prt::R_LEG,
                        prt::L_LEG,
                        prt::R_UPPER_ARM,
                        prt::L_UPPER_ARM,
                        prt::R_KNEE,
                        prt::L_KNEE,
                        prt::R_FOREARM,
                        prt::L_FOREARM,
                        prt::CUIRASS,
                    ],
                ),
                ClothingType::Skirt => (layer::SKIRT, &[prt::GROIN, prt::R_LEG, prt::L_LEG]),
                _ => (layer::CLOTHING, &[]),
            },
        };
        let mut w = Wearable { layer, ..Wearable::default() };
        let mut hidden: Vec<usize> = reserve.to_vec();
        for o in item.parts() {
            let p = o.biped_object_type as usize;
            let pick = |suffix: &str, arm_fallback: bool| {
                let names = if female { vec![&o.female_bodypart, &o.male_bodypart] } else { vec![&o.male_bodypart] };
                names.into_iter().filter(|n| !n.is_empty()).find_map(|n| {
                    if let Some(b) = self.find_body(&format!("{n}{suffix}")) {
                        return Some((format!("{n}{suffix}"), b));
                    }
                    let b = self.find_body(n).filter(|b| arm_fallback && slots::is_arm(b.data.part))?;
                    Some((n.clone(), b))
                })
            };
            match pick("", false) {
                Some((body, _)) => w.parts.push((self.prt_slots[p], part_id(&body, p))),
                None => hidden.push(p),
            }
            if FIRST_PERSON.contains(&p)
                && let Some((body, _)) = pick(".1st", true)
            {
                w.first_person.push((self.prt_slots[p], part_id(&body, p)));
            }
        }
        if hidden.contains(&prt::HEAD) || item.parts().iter().any(|o| o.biped_object_type as usize == prt::HEAD) {
            hidden.push(prt::HAIR);
        }
        hidden.sort_unstable();
        hidden.dedup();
        w.hides = hidden.into_iter().map(|p| self.prt_slots[p]).collect();
        w
    }

    fn build_part(&self, source: &PartSource) -> Option<BodyPart> {
        match source {
            PartSource::Creature { model } => Some(BodyPart { model: model.clone(), slot: self.body_slot, kind: PartKind::Skinned, node_filter: None }),
            PartSource::Body { body, prt } => {
                let record = self.db.get::<Bodypart>(body)?;
                let model = vfs_model(&record.mesh)?;
                let slot = PART_SLOTS[*prt];
                // Unreadable models count as skinned: the cook reports them.
                let probed = (self.probe)(&model).unwrap_or(PartModel::new(true));
                let (kind, node_filter) = if probed.skinned {
                    (PartKind::Skinned, Some(slot.filter.to_owned()))
                } else {
                    // The original renderer mirrors rigid parts on left-side
                    // bones, above the part's own root transform.
                    let mirror = if slot.bone.contains("Left") { Affine3A::from_scale(Vec3::new(-1.0, 1.0, 1.0)) } else { Affine3A::IDENTITY };
                    (PartKind::Rigid { joint: slot.bone.to_owned(), offset: mirror * probed.root }, None)
                };
                Some(BodyPart { model, slot: self.prt_slots[*prt], kind, node_filter })
            }
        }
    }

    /// The items an NPC wears by default, in equip slot order. Leveled list
    /// entries (`random_armor_chitin`, guards' helmets) hold the scaffold
    /// [`held::leveled_item_pick`], as for held items: the original resolves
    /// them when the actor spawns.
    pub fn default_equipment<'a>(&'a self, npc: &'a Npc) -> Vec<(EquipSlot, Item<'a>)> {
        let beast = self.db.get::<Race>(&RecordId::new(&npc.race)).is_some_and(|r| r.data.flags.contains(RaceFlags::BEAST_RACE));
        let skills = equip::npc_skills(&self.db, npc);
        let items = npc.inventory.iter().filter(|(count, _)| *count != 0).filter_map(|(_, id)| held::leveled_item_pick(&self.db, id.as_str()));
        equip::auto_equip(&self.db, &self.rules, items, &skills, beast)
    }

    /// The weapon, shield or light an NPC holds ([`held`]).
    pub fn npc_loadout<'a>(&'a self, npc: &'a Npc, ctx: HeldContext) -> Loadout<'a> {
        let skills = equip::npc_skills(&self.db, npc);
        let items: Vec<&str> = npc.inventory.iter().filter(|(count, _)| *count != 0).filter_map(|(_, id)| held::leveled_item_pick(&self.db, id.as_str())).collect();
        held::auto_held(&self.db, &items, &skills, ctx)
    }

    /// The held items of a creature: only with the `Weapon and Shield` flag,
    /// every weapon skill counting as its combat skill.
    #[allow(clippy::cast_precision_loss)]
    pub fn creature_loadout<'a>(&'a self, crea: &'a Creature, ctx: HeldContext) -> Loadout<'a> {
        if !crea.creature_flags.contains(CreatureFlags::WEAPON_AND_SHIELD) {
            return Loadout::default();
        }
        let items: Vec<&str> = crea.inventory.iter().filter(|(count, _)| *count != 0).filter_map(|(_, id)| held::leveled_item_pick(&self.db, id.as_str())).collect();
        held::auto_held(&self.db, &items, &[crea.data.combat as f32; 27], ctx)
    }

    /// The held items an actor (`NPC_`, `CREA`, `LEVC`) carries in a
    /// context (dark, in combat), as `CharacterState::held` takes them.
    /// Recipes without a held override use the default context (daylight,
    /// no combat); the host re-evaluates at dusk, dawn and cell changes.
    pub fn held(&self, kind: &str, id: &RecordId, ctx: HeldContext) -> Result<Vec<HeldItem>, CharacterError> {
        let unknown = || CharacterError::UnknownBase { kind: kind.to_owned(), id: id.clone() };
        if kind.eq_ignore_ascii_case(NPC_KIND) {
            let npc = self.db.get::<Npc>(id).ok_or_else(unknown)?;
            Ok(self.held_items(&self.npc_loadout(npc, ctx), npc.npc_flags.contains(NpcFlags::FEMALE)))
        } else if kind.eq_ignore_ascii_case(CREATURE_KIND) {
            Ok(self.held_items(&self.creature_loadout(self.db.get::<Creature>(id).ok_or_else(unknown)?, ctx), false))
        } else if kind.eq_ignore_ascii_case(LEVELED_CREATURE_KIND) {
            let (kind, id) = leveled_pick(&self.db, id).ok_or_else(unknown)?;
            self.held(kind, &id, ctx)
        } else {
            Err(unknown())
        }
    }

    /// A loadout as framework held items: models from the VFS (missing
    /// ones skipped), offsets from each model's root and `BoneOffset`.
    pub fn held_items(&self, loadout: &Loadout<'_>, female: bool) -> Vec<HeldItem> {
        let shield = loadout.shield().map(|s| held::shield_mesh(&self.db, s, female));
        held::held_items(
            loadout,
            |mesh| {
                let path = vfs_model(mesh)?;
                let mut cache = self.held_models.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
                let offset = *cache.entry(path.clone()).or_insert_with(|| {
                    self.vfs.contains(&path).then(|| {
                        let probed = (self.probe)(&path).unwrap_or(PartModel::new(false));
                        held::mount_offset(probed.root, probed.bone_offset)
                    })
                });
                Some((path, offset?))
            },
            shield,
        )
    }

    fn npc_recipe(&self, npc: &Npc, state: &CharacterState) -> Result<CharacterRecipe, CharacterError> {
        let race = self
            .db
            .get::<Race>(&RecordId::new(&npc.race))
            .ok_or_else(|| CharacterError::MissingRecord { kind: "RACE", id: RecordId::new(&npc.race) })?;
        let female = npc.npc_flags.contains(NpcFlags::FEMALE);
        let beast = race.data.flags.contains(RaceFlags::BEAST_RACE);
        let default_skeleton = if beast {
            skeleton::BEAST
        } else if female {
            skeleton::FEMALE
        } else {
            skeleton::MALE
        };
        let skeleton = vfs_model(&npc.mesh).unwrap_or_else(|| VfsPath::new(default_skeleton).expect("valid"));
        let race_key = race.id.to_ascii_lowercase();
        let skin = |first_person: bool| self.skins.get(&(race_key.clone(), female, first_person));
        let base: Vec<PartRef> = skin(false)
            .into_iter()
            .flat_map(|t| t.iter().enumerate())
            .filter(|(p, _)| *p != prt::HEAD && *p != prt::HAIR)
            .filter_map(|(p, b)| b.as_ref().map(|b| part_id(b.as_str(), p)))
            .collect();
        let head = self.body_part(&npc.head, prt::HEAD).filter(|_| !npc.head.is_empty());
        let hair = self.body_part(&npc.hair, prt::HAIR).filter(|_| !npc.hair.is_empty());
        let worn = if let Some(ids) = &state.worn { self.order_worn(ids.iter().map(|id| id.as_str().split('#').next().unwrap_or_default()), female) } else {
            let equipped = self.default_equipment(npc);
            self.order_worn(equipped.iter().map(|(_, item)| item.id()), female)
        };
        let (w, h) = if female { (race.data.weight[1], race.data.height[1]) } else { (race.data.weight[0], race.data.height[0]) };
        let first_person = {
            let preferred = if beast {
                skeleton::FIRST_PERSON_BEAST
            } else if female {
                skeleton::FIRST_PERSON_FEMALE
            } else {
                skeleton::FIRST_PERSON
            };
            let fp = [preferred, skeleton::FIRST_PERSON].into_iter().filter_map(|p| VfsPath::new(p).ok()).find(|p| self.vfs.contains(p));
            let skeleton = fp.unwrap_or_else(|| VfsPath::new(skeleton::FIRST_PERSON).expect("valid"));
            let parts = skin(true)
                .into_iter()
                .flat_map(|t| t.iter().enumerate())
                .filter(|(p, _)| FIRST_PERSON.contains(p))
                .filter_map(|(p, b)| b.as_ref().map(|b| part_id(b.as_str(), p)))
                .collect();
            FirstPersonRecipe {
                rig: Some(skeleton.clone()),
                skeleton,
                parts,
                graph: self.config.first_person_graph.clone(),
                fov_y_deg: self.config.first_person_fov_y_deg,
            }
        };
        Ok(CharacterRecipe {
            rig: RigRef::Humanoid(skeleton.clone()),
            skeleton,
            base,
            head,
            hair,
            worn,
            held: state.held.clone().unwrap_or_else(|| self.held_items(&self.npc_loadout(npc, HeldContext::default()), female)),
            // Height on Y, build on X and Z (engine space).
            scale: Vec3::new(w, h, w),
            graph: self.config.humanoid_graph.clone(),
            tint: TintParams::default(),
            // Morrowind has no shape sliders: every NIF geometry morph animates (mw_bake marks them Expression).
            shape: ShapeWeights::default(),
            first_person: Some(first_person),
        })
    }

    /// Wearable ids for items, ordered so that `resolve` (later wins within a
    /// layer) reproduces the original priority ties (earlier slot wins).
    fn order_worn<'a>(&self, items: impl Iterator<Item = &'a str>, female: bool) -> Vec<WearableRef> {
        let mut found: Vec<(EquipSlot, RecordId)> = items
            .filter_map(|id| {
                let item = Item::find(&self.db, &RecordId::new(id))?;
                Some((item.slots()[0], wearable_id(item.id(), female)))
            })
            .filter(|(_, w)| self.wearables.contains_key(w))
            .collect();
        found.sort_by_key(|f| std::cmp::Reverse(f.0));
        found.into_iter().map(|(_, w)| w).collect()
    }

    fn creature_recipe(&self, crea: &Creature, state: &CharacterState) -> Result<CharacterRecipe, CharacterError> {
        let skeleton = vfs_model(&crea.mesh).ok_or_else(|| CharacterError::Adapter(format!("creature {} has no model", crea.id)))?;
        let body = creature_part_id(&skeleton);
        Ok(CharacterRecipe {
            rig: RigRef::Profile(skeleton.clone()),
            base: vec![body],
            skeleton,
            head: None,
            hair: None,
            worn: Vec::new(),
            held: state.held.clone().unwrap_or_else(|| self.held_items(&self.creature_loadout(crea, HeldContext::default()), false)),
            scale: Vec3::splat(crea.scale.unwrap_or(1.0)),
            // Biped creatures (dremora, golden saints, skeletons) play the
            // humanoid groups (`xbase_anim` layered under their own): the
            // humanoid graph (`mw_graphs::creature_template`).
            graph: if u32::from(crea.creature_flags.bits()) & mw_anim::rig::crea_flags::BIPED != 0 {
                self.config.humanoid_graph.clone()
            } else {
                self.config.creature_graph.clone()
            },
            tint: TintParams::default(),
            // Morrowind has no shape sliders: every NIF geometry morph animates (mw_bake marks them Expression).
            shape: ShapeWeights::default(),
            first_person: None,
        })
    }

    /// How a creature moves (CREA flags).
    pub fn creature_mover(&self, id: &str) -> Option<mw_anim::rig::CreatureMover> {
        let crea = self.db.get::<Creature>(&RecordId::new(id))?;
        Some(mw_anim::rig::CreatureMover::from_crea_flags(u32::from(crea.creature_flags.bits())))
    }
}

impl PartLookup for MwActors {
    fn part(&self, id: &RecordId) -> Option<&BodyPart> {
        let entry = self.parts.get(id)?;
        entry.part.get_or_init(|| self.build_part(&entry.source)).as_ref()
    }

    fn wearable(&self, id: &RecordId) -> Option<&Wearable> {
        self.wearables.get(id)
    }
}

impl CharacterSource for MwActors {
    fn recipe(&self, kind: &str, id: &RecordId, state: &CharacterState) -> Result<CharacterRecipe, CharacterError> {
        let unknown = || CharacterError::UnknownBase { kind: kind.to_owned(), id: id.clone() };
        if kind.eq_ignore_ascii_case(NPC_KIND) {
            self.npc_recipe(self.db.get::<Npc>(id).ok_or_else(unknown)?, state)
        } else if kind.eq_ignore_ascii_case(CREATURE_KIND) {
            self.creature_recipe(self.db.get::<Creature>(id).ok_or_else(unknown)?, state)
        } else if kind.eq_ignore_ascii_case(LEVELED_CREATURE_KIND) {
            let (kind, id) = leveled_pick(&self.db, id).ok_or_else(unknown)?;
            self.recipe(kind, &id, state)
        } else {
            Err(unknown())
        }
    }

    /// The humanoid skeletons found in the VFS (third and first person).
    /// Creatures need no humanoid map (their recipes name a rig profile).
    fn rig_maps(&self) -> Vec<(VfsPath, RigMap)> {
        use skeleton::{MALE, FEMALE, BEAST, FIRST_PERSON, FIRST_PERSON_FEMALE, FIRST_PERSON_BEAST};
        [MALE, FEMALE, BEAST, FIRST_PERSON, FIRST_PERSON_FEMALE, FIRST_PERSON_BEAST]
            .into_iter()
            .filter_map(|p| {
                let path = VfsPath::new(p).ok()?;
                let model = mw_nif::load(&self.vfs.read(&path).ok()?).ok()?;
                let rig = mw_anim::rig::humanoid(&mw_anim::rig::skeleton(&model).ok()?).ok()?;
                Some((path, rig.map))
            })
            .collect()
    }
}

/// Skin parts per (race, female, first person), from `BODY` records of type
/// skin that are playable and not vampire parts, in id order (the last
/// match wins). A female body falls back to male parts; first person uses
/// `.1st` parts, else third-person arm parts (same gender first).
fn skin_tables(db: &RecordDb) -> HashMap<(String, bool, bool), SkinTable> {
    let mut by_race: HashMap<String, Vec<(&RecordId, &Bodypart)>> = HashMap::new();
    for (id, b) in db.iter::<Bodypart>() {
        if b.data.bodypart_type == BodypartType::Skin && !b.data.flags.contains(BodypartFlags::NOT_PLAYABLE) && !b.data.vampire {
            by_race.entry(b.race.to_ascii_lowercase()).or_default().push((id, b));
        }
    }
    let mut out = HashMap::new();
    for (race, mut list) in by_race {
        list.sort_by(|a, b| a.0.cmp(b.0));
        for female in [false, true] {
            for first_person in [false, true] {
                out.insert((race.clone(), female, first_person), skin_table(&list, female, first_person));
            }
        }
    }
    out
}

fn skin_table(list: &[(&RecordId, &Bodypart)], female: bool, first_person: bool) -> SkinTable {
    let mut parts: [Option<(RecordId, bool, bool)>; PRT_COUNT] = Default::default(); // (id, female part, 1st-person part)
    for (id, b) in list {
        let part_female = b.data.flags.contains(BodypartFlags::FEMALE);
        // Ids are lowercase (`RecordId`).
        #[allow(clippy::case_sensitive_file_extension_comparisons)]
        let part_1st = id.as_str().ends_with(".1st");
        let same_gender = part_female == female;
        let arm = slots::is_arm(b.data.part);
        let entry = || Some(((*id).clone(), part_female, part_1st));
        for &p in slots::skin_slots(b.data.part) {
            let cur = &mut parts[p];
            if first_person && arm && !part_1st {
                // Third-person arms as a first-person fallback.
                let replace = match cur {
                    None => same_gender || female,
                    Some((_, f, _)) => same_gender && *f != female,
                };
                if replace {
                    *cur = entry();
                }
            } else if part_1st != first_person {
            } else if female && !part_female {
                // Male parts as a fallback for female bodies.
                if cur.is_none() || (arm && cur.as_ref().is_some_and(|c| !c.2)) {
                    *cur = entry();
                }
            } else if same_gender {
                *cur = entry();
            }
        }
    }
    parts.map(|p| p.map(|(id, _, _)| id))
}

/// **Scaffold** (`leveled pick`): the entry a placed leveled creature list
/// shows, following nested lists: the lowest-level entry (ties: the first),
/// as for a level-1 player, ignoring `chance_none`. Gameplay (player level,
/// a seeded roll per placement, respawns) owns the real choice; removal:
/// `sf_rpg` resolves lists before spawning and hands the adapter a creature.
pub fn leveled_pick(db: &RecordDb, id: &RecordId) -> Option<(&'static str, RecordId)> {
    let mut id = id.clone();
    for _ in 0..8 {
        let Some(list) = db.get::<LeveledCreature>(&id) else {
            return if db.get::<Creature>(&id).is_some() {
                Some((CREATURE_KIND, id))
            } else {
                db.get::<Npc>(&id).is_some().then_some((NPC_KIND, id))
            };
        };
        let (next, _) = list.creatures.iter().min_by_key(|(_, level)| *level)?;
        id = RecordId::new(next);
    }
    None
}
