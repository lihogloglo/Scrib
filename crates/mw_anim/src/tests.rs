use mw_nif::anim::{Curve as NifCurve, RotationTrack, TransformTrack};
use mw_nif::glam as nif_glam;
use sf_anim::{Event, Looping};

use crate::{Animation, events, groups};

fn keys(lines: &[(f32, &str)]) -> Vec<(f32, String)> {
    lines.iter().map(|(t, l)| (*t, (*l).to_owned())).collect()
}

fn find<'a>(events: &'a [Event], name: &str, payload: &str) -> Option<&'a Event> {
    events.iter().find(|e| e.name == name && e.payload == payload)
}

#[test]
fn groups_split_the_timeline_and_map_events() {
    let keys = keys(&[
        (0.0, "Idle: Start"),
        (2.0, "Idle: Stop"),
        (2.0, "WalkForward: Start"),
        (2.0, "SoundGen: Left"),
        (2.5, "WalkForward: Loop Start"),
        (3.0, "SoundGen: Right"),
        (3.2, "Sound: FootBareLeft"),
        (3.5, "WalkForward: Loop Stop"),
        (3.5, "WalkForward: Stop"),
        (5.0, "WeaponOneHand: Equip Start"),
        (5.2, "WeaponOneHand: Equip Attach"),
        (5.4, "WeaponOneHand: Equip Stop"),
        (6.0, "WeaponOneHand: Chop Start"),
        (6.2, "WeaponOneHand: Chop Min Attack"),
        (6.5, "WeaponOneHand: Chop Max Attack"),
        (6.6, "WeaponOneHand: Chop Min Hit"),
        (6.7, "WeaponOneHand: Chop Hit"),
        (6.8, "WeaponOneHand: Chop Small Follow Start"),
        (6.9, "WeaponOneHand: Chop Small Follow Stop"),
        (6.8, "WeaponOneHand: Chop Large Follow Start"),
        (7.2, "WeaponOneHand: Chop Large Follow Stop"),
        (8.0, "SpellCast: Self Start"),
        (8.3, "SpellCast: Self Release"),
        (8.6, "SpellCast: Self Stop"),
        (9.0, "SpellCast: Mystery Key"),
        (10.0, "Lonely: Start"),
    ]);
    let groups = groups(&keys);
    let names: Vec<&str> = groups.iter().map(|g| g.name.as_str()).collect();
    assert_eq!(names, ["Idle", "WalkForward", "WeaponOneHand", "SpellCast"], "zero-length groups dropped");

    let idle = &groups[0];
    assert_eq!((idle.start, idle.stop, idle.looping()), (0.0, 2.0, Looping::Whole));
    // The SoundGen key on the shared boundary belongs to both groups.
    assert!(find(&idle.events, events::FOOTSTEP, "left").is_some_and(|e| (e.time - 2.0).abs() < 1e-6));

    let walk = &groups[1];
    assert_eq!(walk.loop_range, Some((0.5, 1.5)));
    assert_eq!(walk.looping(), Looping::Range(0.5, 1.5));
    assert!(find(&walk.events, events::FOOTSTEP, "left").is_some_and(|e| e.time == 0.0));
    assert!(find(&walk.events, events::FOOTSTEP, "right").is_some_and(|e| (e.time - 1.0).abs() < 1e-6));
    assert!(find(&walk.events, events::SOUND, "FootBareLeft").is_some());
    assert!(walk.events.windows(2).all(|w| w[0].time <= w[1].time), "sorted");

    let weapon = &groups[2];
    assert_eq!((weapon.start, weapon.stop, weapon.looping()), (5.0, 7.2, Looping::Once), "spans its own keys");
    let near = |a: f32, b: f32| (a - b).abs() < 1e-5;
    let equip = weapon.section("equip").unwrap();
    assert!(near(equip.start, 0.0) && near(equip.stop, 0.4));
    let chop = weapon.section("chop").unwrap();
    assert!(near(chop.start, 1.0) && near(chop.stop, 2.2), "follow-through ends the section: {chop:?}");
    assert!(find(&weapon.events, events::EQUIP_ATTACH, "").is_some_and(|e| near(e.time, 0.2)));
    let charge = find(&weapon.events, events::ATTACK_CHARGE, "chop").unwrap();
    assert!(near(charge.time, 1.2) && charge.end.is_some_and(|e| near(e, 1.5)));
    assert!(find(&weapon.events, events::ATTACK_MIN_HIT, "chop").is_some_and(|e| near(e.time, 1.6)));
    assert!(find(&weapon.events, events::ATTACK_HIT, "chop").is_some_and(|e| near(e.time, 1.7)));
    assert!(find(&weapon.events, events::ATTACK_FOLLOW, "chop small").is_some_and(|e| e.end.is_some_and(|e| near(e, 1.9))));
    assert!(find(&weapon.events, events::ATTACK_FOLLOW, "chop large").is_some_and(|e| e.end.is_some_and(|e| near(e, 2.2))));

    let cast = &groups[3];
    assert!(cast.section("self").is_some());
    assert!(find(&cast.events, events::CAST_RELEASE, "self").is_some());
    assert!(find(&cast.events, events::TEXT_KEY, "SpellCast: Mystery Key").is_some(), "unknown keys stay generic");
}

#[test]
fn group_names_and_keys_ignore_case() {
    let groups = groups(&keys(&[(1.0, "idle: start"), (1.5, "soundgen: LAND"), (2.0, "IDLE: STOP")]));
    assert_eq!(groups.len(), 1);
    assert_eq!(groups[0].name, "idle");
    assert_eq!(groups[0].events[0].name, events::LAND);
}

/// A cut Hermite curve is the same cubic as the uncut one.
#[test]
fn cut_curves_match_the_source() {
    let v = |x: f32, y: f32, z: f32| nif_glam::Vec3::new(x, y, z);
    let translation = NifCurve::hermite([
        (0.0, v(0.0, 0.0, 0.0), v(0.0, 0.0, 0.0), v(1.0, 2.0, 0.0)),
        (0.5, v(1.0, 0.5, 0.0), v(0.5, -1.0, 0.0), v(0.25, -0.5, 0.0)),
        (2.0, v(2.0, 0.0, 1.0), v(1.0, 0.0, 0.0), v(0.0, 0.0, 0.0)),
        (3.0, v(2.0, 1.0, 1.0), v(0.0, 1.0, 0.0), v(0.0, 0.0, 0.0)),
    ]);
    let yaw = |a: f32| nif_glam::Quat::from_rotation_y(a);
    let track = TransformTrack {
        translation: Some(translation.clone()),
        rotation: Some(RotationTrack::Quats(vec![(0.0, yaw(0.0)), (1.0, yaw(1.0)), (2.5, yaw(-0.5))])),
        scale: Some(NifCurve::linear([(0.0, 1.0), (3.0, 2.0)])),
    };
    let rotation = track.rotation.clone().unwrap();
    let animation = Animation { text_keys: keys(&[(0.3, "Cut: Start"), (2.4, "Cut: Stop")]), tracks: vec![("Bip01".into(), track)], ..Animation::default() };
    let group = &animation.groups()[0];
    let clip = animation.source_clip(group);
    assert_eq!(clip.name, "Cut");
    assert!((clip.duration - 2.1).abs() < 1e-6);
    let t = &clip.tracks[0];
    assert!(t.translation.as_ref().unwrap().is_valid() && t.rotation.as_ref().unwrap().is_valid());
    for i in 0..=42 {
        #[allow(clippy::cast_precision_loss)]
        let local = i as f32 * 0.05;
        let source = translation.sample(0.3 + local).unwrap();
        let cut = t.translation.as_ref().unwrap().sample(local);
        assert!((cut - glam::Vec3::from_array(source.to_array())).length() < 1e-4, "t={local}: {cut} vs {source}");
        let q = rotation.sample(0.3 + local).unwrap();
        let cut_q = t.rotation.as_ref().unwrap().sample(local);
        assert!(cut_q.dot(glam::Quat::from_array(q.to_array())).abs() > 0.99999, "t={local}");
        let s = t.scale.as_ref().unwrap().sample(local);
        assert!((s.x - (1.0 + (0.3 + local) / 3.0)).abs() < 1e-4 && (s.x - s.z).abs() < 1e-6);
    }
}

/// A root-motion wrap (the curve jumps back 3 m in one frame) is a curve
/// break: the snap segment is linear, the keys around it get one-sided
/// tangents, and the smooth motion before it stays inside its keys even
/// though the TCB tangents saw the snap.
#[test]
fn snaps_break_the_curve() {
    let v = |z: f32| nif_glam::Vec3::new(0.0, 0.0, z);
    let raw = [(0.0, 0.0), (0.4, -1.0), (0.8, -2.0), (1.2, -3.0), (1.2667, 0.0), (1.6667, -1.0)];
    let tcb = NifCurve::tcb(raw.iter().map(|&(t, z)| (t, v(z), 0.0, 0.0, 0.0)));
    let past = |c: &NifCurve<nif_glam::Vec3>, from: f32, to: f32| {
        (0..=40).map(|i| from + (to - from) * f32::from(u8::try_from(i).unwrap()) / 40.0).map(|t| (c.sample(t).unwrap().z + 3.0).min(0.0).abs()).fold(0.0, f32::max)
    };
    // Unbroken, the walk overshoots its last key on the way into the snap.
    assert!(past(&tcb, 0.8, 1.2) > 0.1);
    let broken = crate::convert::snap_breaks(&tcb);
    assert!(past(&broken, 0.0, 1.2) < 1e-4, "{}", past(&broken, 0.0, 1.2));
    let keys: Vec<_> = broken.keys().collect();
    // Snap segment linear; keys on both sides one-sided.
    assert_eq!(keys[3].3, v(3.0));
    assert_eq!(keys[4].2, v(3.0));
    assert_eq!(keys[3].2, v(-1.0));
    assert_eq!(keys[4].3, v(-1.0));
    // Authored handles that didn't see the snap keep their shape; the snap
    // segment itself still turns linear.
    let authored = NifCurve::hermite([
        (0.0, v(0.0), v(0.0), v(-0.5)),
        (0.4, v(-1.0), v(-1.5), v(-0.2)),
        (0.4667, v(2.0), v(0.0), v(0.3)),
        (0.8667, v(2.5), v(0.4), v(0.0)),
    ]);
    let keys: Vec<_> = crate::convert::snap_breaks(&authored).keys().collect();
    assert_eq!((keys[1].2, keys[1].3, keys[2].2, keys[2].3), (v(-1.5), v(3.0), v(3.0), v(0.3)));
    // Smooth curves (no snap) come back unchanged.
    let smooth = NifCurve::tcb(raw[..4].iter().map(|&(t, z)| (t, v(z), 0.0, 0.0, 0.0)));
    assert_eq!(crate::convert::snap_breaks(&smooth), smooth);
}

#[test]
fn rig_helpers_without_an_install() {
    use crate::rig::{CreatureMover, crea_flags, is_bone, model_key};
    assert_eq!(model_key(r"r\NixHound.NIF"), "nixhound");
    assert_eq!(model_key("meshes/r/cliffracer.nif"), "cliffracer");
    assert!(is_bone("Bip01 L Thigh") && is_bone("bip01") && !is_bone("Left Hand") && !is_bone(""));
    // Vanilla values: guar 0x48, netch 0x28, slaughterfish 0x18, dremora 0x0d.
    assert_eq!(CreatureMover::from_crea_flags(0x48), CreatureMover::Ground);
    assert_eq!(CreatureMover::from_crea_flags(0x28), CreatureMover::Fly);
    assert_eq!(CreatureMover::from_crea_flags(0x18), CreatureMover::Swim);
    assert_eq!(CreatureMover::from_crea_flags(0x0d), CreatureMover::Ground);
    assert_eq!(CreatureMover::from_crea_flags(crea_flags::WALKS | crea_flags::SWIMS), CreatureMover::Ground);
}
