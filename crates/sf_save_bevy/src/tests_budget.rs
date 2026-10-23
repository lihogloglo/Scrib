//! Size and time budgets (save.md "Size and time budgets") on a synthetic 100-hour world:
//! ~30k overlay changes, 2k journal entries, ~2 MiB of script-like state, a full inventory, a
//! runtime record overlay. Sizes are asserted in every build; times only in release builds
//! (`cargo test -p sf_save_bevy --release budgets -- --nocapture` prints them).
#![allow(clippy::cast_precision_loss)] // sizes printed in KiB and MiB

use std::collections::BTreeMap;
use std::time::{Duration, Instant};

use bevy_app::App;
use glam::{IVec3, Quat, Vec3};
use serde::{Deserialize, Serialize};
use sf_coords::{WorldPos, WorldTransform};
use sf_records::{PatchOp, RecordId, Value};
use sf_rpg::{AttrValue, Attrs, EntryDraft, GameTime, HeardLine, RpgCommand, StageFlags};
use sf_save::encode;
use sf_world::{Location, Placement, PlacementClass, PlacementFlags, PlacementId, RecordRef, WorldspaceId};

use crate::sections::order;
use crate::tests::{Globals, Overlay, Party, Records, Rpg, WS, load, new_game, save, state};
use crate::{LoadRequest, SaveReason, SaveRequest, SaveSection, SaveAppExt, read, write};

const KIB: u64 = 1024;
const MIB: u64 = 1024 * 1024;

/// Script-like state: what Lua `storage` tables hold (nested tables of short strings and numbers).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
struct ScriptLike(Vec<(String, f64, String)>);

#[derive(bevy_ecs::prelude::Resource, Clone, Debug, Default, PartialEq)]
struct Scripted(ScriptLike);

/// A spell created in play (`sf/records`).
#[derive(Serialize)]
struct Spell {
    name: String,
    effects: Vec<(String, f32, f32)>,
}

/// xorshift64*.
struct Rng(u64);
impl Rng {
    fn next(&mut self) -> u64 {
        self.0 ^= self.0 >> 12;
        self.0 ^= self.0 << 25;
        self.0 ^= self.0 >> 27;
        self.0.wrapping_mul(0x2545_f491_4f6c_dd1d)
    }
    fn below(&mut self, n: u64) -> u64 {
        self.next() % n
    }
    #[expect(clippy::cast_precision_loss, reason = "test data")]
    fn unit(&mut self) -> f32 {
        (self.below(1 << 20) as f32) / (1 << 20) as f32
    }
    fn word(&mut self) -> &'static str {
        const WORDS: [&str; 24] = [
            "ash", "guar", "netch", "silt", "strider", "mage", "guild", "fighter", "temple", "house", "redoran", "hlaalu",
            "telvanni", "ebony", "glass", "daedric", "shrine", "tomb", "cave", "mine", "egg", "kwama", "scrib", "cliff",
        ];
        WORDS[usize::try_from(self.below(24)).unwrap()]
    }
    fn sentence(&mut self, words: usize) -> String {
        (0..words).map(|_| self.word()).collect::<Vec<_>>().join(" ")
    }
}

fn placement(rng: &mut Rng, i: u64) -> Placement {
    Placement {
        id: PlacementId::from_key("budget", &i.to_le_bytes()),
        base: RecordRef::new("item", format!("{}_{}", rng.word(), rng.below(400))),
        model: None,
        transform: WorldTransform {
            pos: WorldPos::new(
                IVec3::new(i32::try_from(rng.below(64)).unwrap() - 32, 0, i32::try_from(rng.below(64)).unwrap() - 32),
                Vec3::new(rng.unit() * 256.0, rng.unit() * 20.0, rng.unit() * 256.0),
            ),
            rotation: Quat::from_rotation_y(rng.unit() * std::f32::consts::TAU),
            scale: Vec3::ONE,
        },
        class: PlacementClass::Dynamic,
        flags: PlacementFlags::empty(),
    }
}

/// Fills `app` with a 100-hour world.
fn hundred_hours(app: &mut App) {
    let mut rng = Rng(0x5eed);
    let w = app.world_mut();
    {
        let mut o = w.resource_mut::<Overlay>();
        let ws = WorldspaceId::new(WS);
        for i in 0..10_000 {
            o.0.remove(PlacementId::from_key("budget.removed", &u64::to_le_bytes(i)));
        }
        for i in 0..10_000 {
            let p = placement(&mut rng, i);
            let to = Location::new(ws.clone(), p.transform.pos);
            o.0.move_to(&p, &to, p.transform.rotation);
        }
        for i in 10_000..15_000 {
            let p = placement(&mut rng, i);
            o.0.spawn(ws.clone(), p);
        }
        for i in 0..5_000 {
            let blob = encode(&(rng.unit(), rng.below(100), [rng.unit(); 6]));
            o.0.set_state(PlacementId::from_key("budget.state", &u64::to_le_bytes(i)), "physics", blob);
        }
    }
    {
        let mut rpg = w.resource_mut::<Rpg>();
        for i in 0..2_000u32 {
            let quest = RecordId::new(&format!("quest_{}", i % 300));
            let draft = EntryDraft {
                source: Some(RecordId::new(&format!("info.{i}"))),
                text: rng.sentence(20),
                time: GameTime { day: i / 20, minute: u16::try_from(i % 1440).unwrap() },
            };
            let stage = i32::try_from(i % 100).unwrap() * 10;
            rpg.0.apply(RpgCommand::SetStage { quest, stage, flags: StageFlags::default(), entry: Some(draft) }).unwrap();
        }
        for i in 0..500 {
            let topic = RecordId::new(&format!("topic {i}"));
            for k in 0..2 {
                let line = HeardLine { source: RecordId::new(&format!("info.t{i}.{k}")), speaker: rng.sentence(2), text: rng.sentence(30) };
                rpg.0.apply(RpgCommand::HearLine { topic: topic.clone(), line }).unwrap();
            }
        }
    }
    {
        let mut party = w.resource_mut::<Party>();
        let hero = &mut party.0[0];
        for i in 0..300 {
            let attrs = Attrs::new().with("condition", AttrValue::Int(i64::try_from(rng.below(100)).unwrap()));
            hero.rpg.loadout.add(RecordId::new(&format!("{}_{i}", rng.word())), 1 + u32::try_from(rng.below(5)).unwrap(), attrs).unwrap();
        }
    }
    {
        let mut records = w.resource_mut::<Records>();
        for _ in 0..200 {
            let spell = Spell { name: rng.sentence(3), effects: (0..3).map(|_| (rng.word().to_owned(), rng.unit() * 50.0, rng.unit() * 30.0)).collect() };
            records.0.create("spell", "spell", &spell).unwrap();
        }
        for i in 0..500 {
            records.0.patch("npc", &RecordId::new(&format!("npc_{i}")), &[PatchOp::Set { path: "disposition".into(), value: Value::I64(50) }]);
        }
    }
    // ~2 MiB of script storage.
    let mut script = ScriptLike::default();
    let mut bytes = 0;
    while bytes < 2 * 1024 * 1024 {
        let entry = (format!("{}.{}", rng.word(), rng.below(100_000)), f64::from(rng.unit()), rng.sentence(4));
        bytes += encode(&entry).len();
        script.0.push(entry);
    }
    w.insert_resource(Scripted(script));
    let mut g = w.resource_mut::<Globals>();
    for i in 0..1_500 {
        g.0.insert(format!("global_{i}"), f64::from(rng.unit()));
    }
}

fn ms(d: Duration) -> f64 {
    d.as_secs_f64() * 1000.0
}

#[test]
fn budgets() {
    let dir = tempfile::tempdir().unwrap();
    let (mut a, _) = new_game(dir.path());
    let fresh = save(&mut a, SaveRequest::slot(SaveReason::Manual));
    println!("new game: {} bytes ({} sections)", fresh.bytes, fresh.sections);
    assert!(fresh.bytes <= 256 * KIB, "new game save {} B over 256 KiB", fresh.bytes);

    a.init_resource::<Scripted>()
        .add_save_section(SaveSection::value(sf_save::SectionKey::new("budget", "script").unwrap(), 1, read(|s: &Scripted| s.0.clone()), write(|s: &mut Scripted, v| s.0 = v)).order(order::SCRIPTS));
    hundred_hours(&mut a);
    let t = Instant::now();
    let big = save(&mut a, SaveRequest::slot(SaveReason::Manual));
    let frame = t.elapsed();
    let played = state(&a);
    let scripted = a.world().resource::<Scripted>().clone();

    let (mut b, _) = new_game(dir.path());
    b.init_resource::<Scripted>()
        .add_save_section(SaveSection::value(sf_save::SectionKey::new("budget", "script").unwrap(), 1, read(|s: &Scripted| s.0.clone()), write(|s: &mut Scripted, v| s.0 = v)).order(order::SCRIPTS));
    let t = Instant::now();
    let loaded = load(&mut b, LoadRequest::path(&big.path)).unwrap();
    let load_frame = t.elapsed();
    assert_eq!(state(&b), played);
    assert_eq!(*b.world().resource::<Scripted>(), scripted);

    let per_owner: BTreeMap<&str, String> = big.per_owner.iter().map(|(k, v)| (k.as_str(), format!("{:.1} KiB", *v as f64 / 1024.0))).collect();
    println!(
        "100-hour world: {:.2} MiB on disk; per owner {per_owner:?}\n  save: snapshot {:.2} ms (main), encode {:.2} ms, write {:.2} ms (frame {:.2} ms)\n  load: read+verify+migrate+decode {:.2} ms, apply {:.2} ms (frame {:.2} ms)",
        big.bytes as f64 / MIB as f64,
        ms(big.snapshot),
        ms(big.encode),
        ms(big.write),
        ms(frame),
        ms(loaded.prepare),
        ms(loaded.apply),
        ms(load_frame),
    );
    assert!(big.bytes <= 8 * MIB, "100-hour save {} B over 8 MiB", big.bytes);
    if !cfg!(debug_assertions) {
        assert!(big.snapshot <= Duration::from_millis(5), "snapshot {:?} over 5 ms", big.snapshot);
        assert!(big.encode + big.write <= Duration::from_millis(500), "encode + write {:?} over 500 ms", big.encode + big.write);
        assert!(loaded.prepare + loaded.apply <= Duration::from_millis(300), "load {:?} over 300 ms", loaded.prepare + loaded.apply);
    }
}
