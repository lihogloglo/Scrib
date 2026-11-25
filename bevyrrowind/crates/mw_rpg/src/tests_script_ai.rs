//! AI package functions in result scripts (`script_ai.rs`, slice `MA2c1`): each push becomes a
//! [`GameOp::Ai`] on its target with `OpenMW`'s argument reading, pushes on the player do
//! nothing, reads go through the world's [`MwAi`], and [`AiPush::package`] gives the record
//! struct `mw_ai` maps. Synthetic scripts.

use std::collections::HashMap;

use sf_records::RecordId;
use sf_rpg::{ActorKey, CharacterRpg, ConversationCtx, PlayerId, WorldRpg};
use tes3::esp::{
    AiEscortPackage, AiFollowPackage, AiPackage, AiTravelPackage, AiWanderPackage, FixedString,
};

use crate::{
    AiPush, GameOp, GlobalNumber, MwAi, MwResults, MwWorld, ResultCall, ResultRunner, ResultState,
    ScriptVar, Target, VarType,
};

const SPEAKER: ActorKey = ActorKey(1);

fn id(s: &str) -> RecordId {
    RecordId::new(s)
}

/// The speaker finished its package; it last ran a travel; `arrille` runs nothing.
struct Ai;

impl MwAi for Ai {
    fn package_done(&self, _: &ConversationCtx, target: &Target) -> bool {
        *target == Target::Speaker
    }
    fn current_package(&self, _: &ConversationCtx, target: &Target) -> i32 {
        if *target == Target::Speaker { mw_script::aifn::package_type::TRAVEL } else { -1 }
    }
}

/// Float speaker locals `a`-`d` capture reads; `with_ai` answers the AI reads with [`Ai`].
struct World {
    with_ai: bool,
}

impl MwWorld for World {
    fn global(&self, _: &RecordId) -> Option<GlobalNumber> {
        None
    }
    fn var(&self, _: &ConversationCtx, target: &Target, name: &RecordId) -> Option<ScriptVar> {
        let local = *target == Target::Speaker && ["a", "b", "c", "d"].contains(&name.as_str());
        local.then_some(ScriptVar { ty: VarType::Float, value: 0.0 })
    }
    fn item_count(&self, _: &ConversationCtx, _: &Target, _: &RecordId) -> u64 {
        0
    }
    fn ai(&self) -> &dyn MwAi {
        if self.with_ai { &Ai } else { &crate::NoAi }
    }
}

/// The run's AI ops and captured locals.
fn run(world: &World, script: &str) -> (Vec<(Target, AiPush)>, HashMap<String, f32>) {
    let (rpg, pc) = (WorldRpg::default(), CharacterRpg::default());
    let mut results = MwResults::new();
    let ctx = ConversationCtx {
        player: PlayerId::HOST,
        speaker: SPEAKER,
        speaker_base: id("guard"),
        choice: None,
    };
    let (topic, response) = (id("topic"), id("r"));
    let effects = results
        .runner(ResultState { world: &rpg, player: &pc, env: world })
        .run(ResultCall { ctx: &ctx, topic: &topic, response: &response, script })
        .unwrap();
    assert!(results.drain_reports().is_empty(), "AI functions are never unhandled");
    let (mut pushes, mut locals) = (Vec::new(), HashMap::new());
    for e in effects {
        match e {
            GameOp::Ai { target, push } => pushes.push((target, push)),
            GameOp::SetLocal { name, value, .. } => {
                locals.insert(name.as_str().to_owned(), value);
            }
            other => panic!("unexpected effect {other:?}"),
        }
    }
    (pushes, locals)
}

#[test]
fn every_push_function_becomes_an_ai_op() {
    let (pushes, _) = run(
        &World { with_ai: false },
        "AITravel 100 -200.5 30\n\
         AIEscort \"Fargoth\" 5.9 10 20 30\n\
         AIEscortCell fargoth \"Seyda Neen, Census and Excise Office\" 2 1 2 3 0\n\
         AIFollow player 1.5 0 0 0\n\
         AIFollowCell player \"Balmora\" 0 4 5 6\n\
         AIWander 512 5 0\n\
         AIWander 128 0 6 99 40 300 -3\n\
         AIWander 64 1 2 0 10 20 30 40 50 60 70 80 0 77",
    );
    let s = Target::Speaker;
    assert_eq!(
        pushes,
        [
            (s.clone(), AiPush::Travel { to: [100.0, -200.5, 30.0], repeat: false }),
            (
                s.clone(),
                AiPush::Escort { actor: id("fargoth"), cell: None, hours: 5, to: [10.0, 20.0, 30.0], repeat: false }
            ),
            (
                s.clone(),
                AiPush::Escort {
                    actor: id("fargoth"),
                    cell: Some(id("seyda neen, census and excise office")),
                    hours: 2,
                    to: [1.0, 2.0, 3.0],
                    repeat: true,
                }
            ),
            (
                s.clone(),
                AiPush::Follow { actor: id("player"), cell: None, hours: 1.5, to: [0.0; 3], repeat: false }
            ),
            (
                s.clone(),
                AiPush::Follow {
                    actor: id("player"),
                    cell: Some(id("balmora")),
                    hours: 0.0,
                    to: [4.0, 5.0, 6.0],
                    repeat: false,
                }
            ),
            (s.clone(), AiPush::Wander { range: 512, hours: 5, hour: 0, idles: vec![], repeat: false }),
            (
                s.clone(),
                AiPush::Wander { range: 128, hours: 0, hour: 6, idles: vec![40, 255, 0], repeat: true }
            ),
            (
                s,
                AiPush::Wander {
                    range: 64,
                    hours: 1,
                    hour: 2,
                    idles: vec![10, 20, 30, 40, 50, 60, 70, 80],
                    repeat: false,
                }
            ),
        ]
    );
}

#[test]
fn ref_pushes_target_the_ref_and_never_the_player() {
    let (pushes, _) = run(
        &World { with_ai: false },
        "\"Arrille\"->AITravel 1 2 3 1\nplayer->AITravel 4 5 6\nplayer->AIWander 0 0 0",
    );
    assert_eq!(pushes, [(Target::Actor(id("arrille")), AiPush::Travel { to: [1.0, 2.0, 3.0], repeat: true })]);
}

#[test]
#[allow(clippy::float_cmp, reason = "small integers, exact in f32")]
fn reads_go_through_the_worlds_ai() {
    let script = "set a to GetAIPackageDone\nset b to GetCurrentAIPackage\n\
                  set c to \"arrille\"->GetAIPackageDone\nset d to \"arrille\"->GetCurrentAIPackage";
    let (pushes, l) = run(&World { with_ai: true }, script);
    assert!(pushes.is_empty());
    assert_eq!([l["a"], l["b"], l["c"], l["d"]], [1.0, 1.0, 0.0, -1.0], "done; travel = 1; none = -1");
    let (_, l) = run(&World { with_ai: false }, script);
    assert_eq!([l["a"], l["b"], l["c"], l["d"]], [0.0, -1.0, 0.0, -1.0], "no AI: not done, none");
}

/// The record structs `mw_ai::packages::directive` maps, so a script push yields the spec the
/// same `AI_*` record would.
#[test]
fn pushes_map_to_the_record_package() {
    let fargoth = FixedString("fargoth".to_owned());
    assert_eq!(
        AiPush::Travel { to: [1.0, 2.0, 3.0], repeat: true }.package(),
        AiPackage::Travel(AiTravelPackage { location: [1.0, 2.0, 3.0], reset: 1 })
    );
    assert_eq!(
        AiPush::Escort { actor: id("fargoth"), cell: Some(id("balmora")), hours: 2, to: [1.0; 3], repeat: false }
            .package(),
        AiPackage::Escort(AiEscortPackage {
            location: [1.0; 3],
            duration: 2,
            target: fargoth.clone(),
            reset: 0,
            cell: "balmora".to_owned(),
        })
    );
    assert_eq!(
        AiPush::Follow { actor: id("fargoth"), cell: None, hours: 1.5, to: [0.0; 3], repeat: true }.package(),
        AiPackage::Follow(AiFollowPackage {
            location: [0.0; 3],
            duration: 1,
            target: fargoth,
            reset: 1,
            cell: String::new(),
        })
    );
    assert_eq!(
        AiPush::Wander { range: 70_000, hours: -1, hour: 300, idles: vec![10, 20], repeat: true }.package(),
        AiPackage::Wander(AiWanderPackage {
            distance: u16::MAX,
            duration: u16::MAX,
            game_hour: u8::MAX,
            idle2: 10,
            idle3: 20,
            reset: 1,
            ..AiWanderPackage::default()
        }),
        "range and hour clamp; -1 hours is the format's int16 -1 (no limit)"
    );
    let ids: Vec<i32> = [
        AiPush::Wander { range: 0, hours: 0, hour: 0, idles: vec![], repeat: false },
        AiPush::Travel { to: [0.0; 3], repeat: false },
        AiPush::Escort { actor: id("a"), cell: None, hours: 0, to: [0.0; 3], repeat: false },
        AiPush::Follow { actor: id("a"), cell: None, hours: 0.0, to: [0.0; 3], repeat: false },
    ]
    .iter()
    .map(AiPush::type_id)
    .collect();
    assert_eq!(ids, [0, 1, 2, 3], "OpenMW AiPackageTypeId");
}
