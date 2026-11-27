//! `IN19c` exact tests: which `LIGH` records get a light def, its slot and verbs, the carryable's
//! take on top, and the switch off.

use glam::Vec3;
use sf_interaction::{LIT, SlotKind, TagId, VERB_RELIGHT, VERB_SNUFF};
use sf_records::RecordId;

use crate::activation::verb;
use crate::defs::CLASS_VERB_PRIORITY;
use crate::light::{LIGHT_INTENSITY, LIGHT_SLOT, LightFacts, TAKE_PRIORITY, light_def, light_interactable};

const FLAME: Vec3 = Vec3::new(0.0, 0.4, 0.1);

fn candle() -> LightFacts {
    LightFacts { radius_m: 2.5, fire: true, carryable: false, negative: false, off_by_default: false }
}

fn id() -> RecordId {
    RecordId::new("light_com_candle_01")
}

#[test]
fn only_lit_fire_lights_get_a_def() {
    let d = light_def(&candle()).expect("a candle");
    assert_eq!((d.radius_m, d.intensity, d.lit_default), (2.5, LIGHT_INTENSITY, true));
    for f in [
        LightFacts { fire: false, ..candle() },
        LightFacts { negative: true, ..candle() },
        LightFacts { off_by_default: true, ..candle() },
        LightFacts { radius_m: 0.0, ..candle() },
    ] {
        assert_eq!(light_def(&f), None, "{f:?}");
        assert!(light_interactable(&id(), &f, FLAME, true).is_none(), "{f:?}");
    }
}

#[test]
fn fixed_flame_snuffs_on_its_slot_and_relights_slotless() {
    let def = light_interactable(&id(), &candle(), FLAME, true).expect("a def");
    assert_eq!(def.id, id());
    assert_eq!(def.slots.len(), 1);
    let slot = &def.slots[usize::from(LIGHT_SLOT)];
    assert!(matches!(slot.kind, SlotKind::Light(l) if l == light_def(&candle()).unwrap()));
    assert_eq!(slot.local.translation, FLAME);
    assert_eq!((slot.verb.verb, slot.verb.priority), (VERB_SNUFF, CLASS_VERB_PRIORITY));
    assert_eq!((slot.verb.requires.all.as_slice(), slot.verb.requires.none.as_slice()), (&[LIT][..], &[][..]));
    let verbs: Vec<_> = def.verbs.iter().map(|v| (v.verb, v.priority, v.requires.none.to_vec())).collect();
    assert_eq!(verbs, [(VERB_RELIGHT, CLASS_VERB_PRIORITY, vec![LIT])]);
    // The verb tags are the activation table's.
    assert_eq!((VERB_SNUFF, VERB_RELIGHT), (TagId::of(verb::SNUFF), TagId::of(verb::RELIGHT)));
}

#[test]
fn carryable_keeps_take_on_top_and_switch_off_drops_the_light_verbs() {
    let lantern = LightFacts { carryable: true, ..candle() };
    let def = light_interactable(&id(), &lantern, FLAME, true).expect("a def");
    let verbs: Vec<_> = def.verbs.iter().map(|v| (v.verb, v.priority)).collect();
    assert_eq!(verbs, [(TagId::of(verb::TAKE), TAKE_PRIORITY), (VERB_RELIGHT, CLASS_VERB_PRIORITY)]);
    const { assert!(TAKE_PRIORITY > CLASS_VERB_PRIORITY) };
    // Classic: still a light slot (registered for the AI), no relight; a carryable keeps take.
    let off = light_interactable(&id(), &lantern, FLAME, false).expect("a def");
    assert!(matches!(off.slots[0].kind, SlotKind::Light(_)));
    let verbs: Vec<_> = off.verbs.iter().map(|v| v.verb).collect();
    assert_eq!(verbs, [TagId::of(verb::TAKE)]);
    assert!(light_interactable(&id(), &candle(), FLAME, false).expect("a def").verbs.is_empty());
}
