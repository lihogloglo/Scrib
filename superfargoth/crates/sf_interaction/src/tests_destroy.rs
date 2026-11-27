//! `IN1a` destruction core (`IN39a1`: `force_stage` reaches the stage `apply_damage` misses by one ulp): stage math (one stage, several per hit, never twice, no-op damage,
//! overkill), the debris distance gate, the break queue's overflow to the next tick and the rest
//! state codec; `IN1b`: the break seed (splitmix64 reference value, fixed seeds, the u128 high word too).

use sf_records::RecordId;
use sf_world::PlacementId;
use smallvec::smallvec;

use crate::*;

fn stage(at: f32, model: Option<&str>, on_break: BreakAction) -> Stage {
    Stage { at, model: model.map(RecordId::new), debris: smallvec![], chunks: None, effect: None, on_break }
}

/// A 40-health crate: cracked at half, splintered at a quarter, gone at zero (spills).
fn crate_def() -> DestructibleDef {
    DestructibleDef {
        health: 40.0,
        stages: smallvec![
            stage(0.5, Some("crate_cracked"), BreakAction::None),
            stage(0.25, Some("crate_splintered"), BreakAction::None),
            stage(0.0, None, BreakAction::SpillContents),
        ],
        ignore_external: false,
        loot: None,
        salvage: smallvec![(RecordId::new("plank"), 2)],
    }
}

#[test]
fn a_hit_crosses_the_stage_whose_fraction_it_reaches() {
    let def = crate_def();
    let s0 = DestructState::intact(&def);
    assert_eq!(s0, DestructState { health: 40.0, stage: 0 });
    // 40 -> 30 (0.75): no stage.
    let h = apply_damage(&def, s0, 10.0);
    assert_eq!(h.state, DestructState { health: 30.0, stage: 0 });
    assert_eq!(h.crossed, 0..0);
    assert!(h.overkill.abs() < f32::EPSILON);
    // 30 -> 20 (0.5, exactly `at`): stage 0 crossed.
    let h = apply_damage(&def, h.state, 10.0);
    assert_eq!(h.state, DestructState { health: 20.0, stage: 1 });
    assert_eq!(h.crossed, 0..1);
    // 20 -> 15 (0.375): nothing new, the crossed stage is not crossed again.
    let h = apply_damage(&def, h.state, 5.0);
    assert_eq!(h.state, DestructState { health: 15.0, stage: 1 });
    assert_eq!(h.crossed, 1..1);
}

#[test]
fn one_hard_hit_crosses_several_stages_in_order_with_overkill() {
    let def = crate_def();
    // 40 -> 0 with 20 past zero: all three stages, overkill 20 / 40.
    let h = apply_damage(&def, DestructState::intact(&def), 60.0);
    assert_eq!(h.state, DestructState { health: 0.0, stage: 3 });
    assert_eq!(h.crossed, 0..3);
    assert!((h.overkill - 0.5).abs() < f32::EPSILON, "{}", h.overkill);
    // 40 -> 8 (0.2): cracked and splintered, not gone.
    let h = apply_damage(&def, DestructState::intact(&def), 32.0);
    assert_eq!(h.state, DestructState { health: 8.0, stage: 2 });
    assert_eq!(h.crossed, 0..2);
    assert!(h.overkill.abs() < f32::EPSILON);
    // Exactly to zero: gone, no overkill.
    let h = apply_damage(&def, h.state, 8.0);
    assert_eq!(h.state, DestructState { health: 0.0, stage: 3 });
    assert_eq!(h.crossed, 2..3);
    assert!(h.overkill.abs() < f32::EPSILON);
    // Beating the remains: no stage left, health stays at zero, all overkill.
    let h = apply_damage(&def, h.state, 4.0);
    assert_eq!(h.state, DestructState { health: 0.0, stage: 3 });
    assert_eq!(h.crossed, 3..3);
    assert!((h.overkill - 0.1).abs() < 1e-6, "{}", h.overkill);
}

#[test]
fn no_op_damage_and_healthless_defs() {
    let def = crate_def();
    let s = DestructState { health: 30.0, stage: 0 };
    for d in [0.0, -5.0, f32::NAN, f32::INFINITY] {
        let h = apply_damage(&def, s, d);
        assert_eq!(h, Hit { state: s, crossed: 0..0, overkill: 0.0 }, "{d}");
    }
    // A def with no health breaks through every stage at the first hit.
    let mut zero = crate_def();
    zero.health = 0.0;
    let h = apply_damage(&zero, DestructState::intact(&zero), 1.0);
    assert_eq!(h.state, DestructState { health: 0.0, stage: 3 });
    assert_eq!(h.crossed, 0..3);
    // A def with no stages only loses health.
    let mut bare = crate_def();
    bare.stages.clear();
    let h = apply_damage(&bare, DestructState::intact(&bare), 50.0);
    assert_eq!(h.state, DestructState { health: 0.0, stage: 0 });
    assert_eq!(h.crossed, 0..0);
}

#[test]
fn debris_lod_gates_bodies_by_distance() {
    let lod = InteractParams::default().debris_lod_m;
    assert!(debris_lod(0.0, lod));
    assert!(debris_lod(lod, lod), "inclusive at the edge");
    assert!(!debris_lod(lod + 0.01, lod));
    assert!(!debris_lod(f32::NAN, lod));
    assert!(!debris_lod(f32::INFINITY, lod));
}

#[test]
fn breaks_over_the_cap_wait_for_the_next_tick_in_order() {
    let mut q = BreakQueue::new();
    for i in 0..8u32 {
        q.push(i);
    }
    assert_eq!(q.take_tick(3).collect::<Vec<_>>(), [0, 1, 2]);
    assert_eq!(q.len(), 5);
    q.push(8);
    assert_eq!(q.take_tick(3).collect::<Vec<_>>(), [3, 4, 5]);
    assert_eq!(q.take_tick(3).collect::<Vec<_>>(), [6, 7, 8]);
    assert!(q.is_empty());
    assert_eq!(q.take_tick(3).count(), 0);
    let mut q = BreakQueue::new();
    q.push(1u32);
    assert_eq!(q.take_tick(0).count(), 0, "a zero cap applies nothing");
    assert_eq!(q.len(), 1);
}

#[test]
fn rest_bytes_round_trip_and_reject_malformed() {
    for s in [
        DestructState { health: 40.0, stage: 0 },
        DestructState { health: 12.5, stage: 2 },
        DestructState { health: 0.0, stage: 3 },
    ] {
        let b = s.rest_bytes();
        assert_eq!(b[0], DESTRUCT_REST_VERSION);
        assert_eq!(DestructState::from_rest_bytes(&b), Some(s));
    }
    let good = DestructState { health: 12.5, stage: 2 }.rest_bytes();
    assert_eq!(DestructState::from_rest_bytes(&good[..5]), None, "short");
    assert_eq!(DestructState::from_rest_bytes(&[good.as_slice(), &[0]].concat()), None, "long");
    let mut v2 = good;
    v2[0] = 2;
    assert_eq!(DestructState::from_rest_bytes(&v2), None, "version");
    for bad in [-1.0f32, f32::NAN, f32::INFINITY] {
        let mut b = good;
        b[2..6].copy_from_slice(&bad.to_le_bytes());
        assert_eq!(DestructState::from_rest_bytes(&b), None, "{bad}");
    }
    assert_eq!(DESTRUCT_STATE_OWNER, "destruct");
}

#[test]
fn break_seed_is_splitmix64_over_placement_tick_and_stage() {
    // The reference first output of SplitMix64 seeded 0 (Steele et al. 2014).
    assert_eq!(splitmix64(0), 0xE220_A839_7B1D_CDAF);
    let p = PlacementId(7);
    assert_eq!(break_seed(p, Tick(100), 0), 0x8890_7C86_2F28_3E61);
    // Each input moves it: the stage, the tick and the placement's high word.
    assert_eq!(break_seed(p, Tick(100), 1), 0xE62C_9292_0F82_8E90);
    assert_eq!(break_seed(p, Tick(101), 0), 0xA3CF_7A21_94EB_189C);
    assert_eq!(break_seed(PlacementId((1 << 64) | 7), Tick(100), 0), 0x279C_0320_C6CA_D5D9);
}

/// A 37-health post: cracked at half, splintered at a tenth, gone at zero.
fn post_def(ignore_external: bool) -> DestructibleDef {
    DestructibleDef {
        health: 37.0,
        stages: smallvec![
            stage(0.5, Some("post_cracked"), BreakAction::None),
            stage(0.1, Some("post_splintered"), BreakAction::None),
            stage(0.0, None, BreakAction::None),
        ],
        ignore_external,
        loot: None,
        salvage: smallvec![],
    }
}

#[test]
fn force_stage_reaches_the_stage_a_subtraction_misses_by_one_ulp() {
    let def = post_def(false);
    let s0 = DestructState::intact(&def);
    let target = def.health * def.stages[1].at;
    // `health - H x at` as damage: 37 - 33.3 leaves 3.7000008, a fraction one ulp above 0.1.
    let miss = apply_damage(&def, s0, s0.health - target);
    assert_eq!(miss.crossed, 0..1, "the subtraction misses stage 1");
    assert!(miss.state.health / def.health > def.stages[1].at);

    let kick = DAMAGE_IMPACT;
    let h = force_stage(&def, s0, 1, kick);
    assert_eq!(h.state, DestructState { health: target, stage: 2 });
    assert_eq!(h.crossed, 0..2);
    assert!(h.overkill.abs() < f32::EPSILON);
    // Health never rises: a lower health keeps its value.
    let low = DestructState { health: 1.0, stage: 1 };
    assert_eq!(force_stage(&def, low, 1, kick).state, DestructState { health: 1.0, stage: 2 });
    // The last stage: health 0, gone.
    assert_eq!(force_stage(&def, h.state, 2, kick), Hit { state: DestructState { health: 0.0, stage: 3 }, crossed: 2..3, overkill: 0.0 });

    // Nothing: already reached, past the def's stages, gone, `ignore_external` unless scripted.
    let none = |state: DestructState| Hit { state, crossed: state.stage..state.stage, overkill: 0.0 };
    assert_eq!(force_stage(&def, h.state, 0, kick), none(h.state));
    assert_eq!(force_stage(&def, h.state, 1, kick), none(h.state));
    assert_eq!(force_stage(&def, s0, 3, kick), none(s0));
    let gone_early = DestructibleDef {
        stages: smallvec![stage(0.5, None, BreakAction::None), stage(0.0, Some("rubble"), BreakAction::None)],
        ..post_def(false)
    };
    let removed = DestructState { health: 18.0, stage: 1 };
    assert_eq!(force_stage(&gone_early, removed, 1, kick), none(removed));
    let held = post_def(true);
    assert_eq!(force_stage(&held, s0, 1, kick), none(s0));
    assert_eq!(force_stage(&held, s0, 1, DAMAGE_SCRIPT).crossed, 0..2);
}
