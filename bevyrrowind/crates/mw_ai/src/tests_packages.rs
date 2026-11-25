//! `MA2a`: record AI packages -> directives (`packages.rs`).

use std::collections::HashMap;

use glam::DVec3;
use mw_world::convert::{UNIT_METERS_F64, meters, world_pos};
use mw_world::ids::{exterior, interior};
use sf_ai::{ActorKey, DirectiveKind, DirectiveSource, Limit};
use sf_records::RecordId;
use sf_world::{Location, PlacementId};
use tes3::esp::{
    AiActivatePackage, AiEscortPackage, AiFollowPackage, AiPackage, AiTravelPackage,
    AiWanderPackage, FixedString,
};

use crate::packages::{
    NOWHERE, PackageCtx, PackageExtra, PackageResolver, SkipReason, Skipped, directives,
};
use crate::wander::Wander;

struct Lookup {
    actors: HashMap<&'static str, ActorKey>,
    objects: HashMap<&'static str, PlacementId>,
}

impl PackageResolver for Lookup {
    fn actor(&self, id: &str) -> Option<ActorKey> {
        self.actors.get(id).copied()
    }
    fn placement(&self, id: &str) -> Option<PlacementId> {
        self.objects.get(id).copied()
    }
}

const PLAYER: ActorKey = ActorKey(7);
const LEVER: PlacementId = PlacementId(42);

fn lookup() -> Lookup {
    Lookup {
        actors: HashMap::from([("player", PLAYER)]),
        objects: HashMap::from([("lever_01", LEVER)]),
    }
}

fn home() -> Location {
    Location::new(exterior(), world_pos([100.0, 200.0, 300.0]))
}

fn ctx(resolver: &Lookup) -> PackageCtx<'_> {
    PackageCtx { actor: RecordId::new("fargoth"), home: home(), resolver }
}

fn name(s: &str) -> FixedString<32> {
    FixedString(s.to_owned())
}

fn wander(duration: u16, reset: i8) -> AiPackage {
    AiPackage::Wander(AiWanderPackage {
        distance: 512,
        duration,
        game_hour: 6,
        idle2: 60,
        idle3: 20,
        reset,
        ..AiWanderPackage::default()
    })
}

fn travel(location: [f32; 3]) -> AiPackage {
    AiPackage::Travel(AiTravelPackage { location, reset: 0 })
}

fn escort(target: &str, cell: &str, location: [f32; 3]) -> AiPackage {
    AiPackage::Escort(AiEscortPackage {
        location,
        duration: 5,
        target: name(target),
        reset: 0,
        cell: cell.to_owned(),
    })
}

fn follow(target: &str, cell: &str, location: [f32; 3]) -> AiPackage {
    AiPackage::Follow(AiFollowPackage {
        location,
        duration: 0,
        target: name(target),
        reset: 1,
        cell: cell.to_owned(),
    })
}

fn activate(target: &str) -> AiPackage {
    AiPackage::Activate(AiActivatePackage { target: name(target), reset: 0 })
}

fn engine(p: [f64; 3]) -> DVec3 {
    DVec3::new(p[0], p[2], -p[1]) * UNIT_METERS_F64
}

/// Within f32 precision (`WorldPos` keeps an f32 offset in its partition).
fn assert_at(loc: &Location, worldspace: &sf_world::WorldspaceId, mw: [f64; 3]) {
    assert_eq!(&loc.worldspace, worldspace);
    let got = loc.pos.to_f64();
    assert!(got.abs_diff_eq(engine(mw), 1e-6), "{got} vs {}", engine(mw));
}

#[test]
fn wander_stays_near_home_with_duration_and_repeat() {
    let r = lookup();
    let out = directives(&[wander(3, 1)], &ctx(&r));
    assert!(out.skipped.is_empty());
    let d = &out.directives[0];
    assert_eq!(d.index, 0);
    assert_eq!(d.spec.kind, DirectiveKind::StayNear { home: home(), radius: meters(512.0) });
    assert_eq!(d.spec.source, DirectiveSource::Record);
    assert_eq!(d.spec.priority, 0);
    assert!(d.spec.repeat);
    assert_eq!(d.spec.limit, Some(Limit::For { minutes: 180 }));
    assert_eq!(d.spec.origin, Some(RecordId::new("fargoth")));
    let AiPackage::Wander(p) = wander(3, 1) else { unreachable!() };
    assert_eq!(d.extra, PackageExtra::Wander(Wander::from_package(&p)));
}

#[test]
fn wander_zero_or_negative_duration_has_no_limit() {
    let r = lookup();
    // 0 never ends; 0xFFFF is -1 as the format's int16 (OpenMW clamps it to 0).
    let out = directives(&[wander(0, 0), wander(0xFFFF, 0)], &ctx(&r));
    for d in &out.directives {
        assert_eq!(d.spec.limit, None);
        assert!(!d.spec.repeat);
    }
}

#[test]
fn travel_converts_units_and_axes_in_the_actors_worldspace() {
    let r = lookup();
    let interior_home = Location::new(interior("Seyda Neen, Census and Excise Office"), home().pos);
    let c = PackageCtx { home: interior_home.clone(), ..ctx(&r) };
    let out = directives(&[travel([640.0, 128.0, 64.0])], &c);
    let DirectiveKind::GoTo(to) = &out.directives[0].spec.kind else { panic!("not go_to") };
    assert_at(to, &interior_home.worldspace, [640.0, 128.0, 64.0]);
    // 640 units = 9.144 m east, 64 up, 128 north (-Z).
    assert!(to.pos.to_f64().abs_diff_eq(DVec3::new(9.144, 0.9144, -1.8288), 1e-6));
    assert_eq!(out.directives[0].spec.limit, None);
    assert!(!out.directives[0].spec.repeat);
}

#[test]
fn escort_with_and_without_cell() {
    let r = lookup();
    let out = directives(
        &[escort("player", "", [10.0, 20.0, 30.0]), escort("player", "Balmora, Guild of Mages", [1.0, 2.0, 3.0])],
        &ctx(&r),
    );
    assert!(out.skipped.is_empty());
    let DirectiveKind::Escort { who, to } = &out.directives[0].spec.kind else { panic!("not escort") };
    assert_eq!(*who, PLAYER);
    assert_at(to, &exterior(), [10.0, 20.0, 30.0]);
    assert_eq!(out.directives[0].spec.limit, Some(Limit::For { minutes: 300 }));
    let DirectiveKind::Escort { to, .. } = &out.directives[1].spec.kind else { panic!("not escort") };
    assert_at(to, &interior("Balmora, Guild of Mages"), [1.0, 2.0, 3.0]);
}

#[test]
fn escort_without_destination_is_skipped() {
    let r = lookup();
    let out = directives(&[escort("player", "", [NOWHERE, NOWHERE, NOWHERE])], &ctx(&r));
    assert!(out.directives.is_empty());
    assert_eq!(out.skipped, vec![Skipped { index: 0, tag: "AI_E", reason: SkipReason::NoDestination }]);
}

#[test]
fn follow_with_and_without_cell() {
    let r = lookup();
    let out = directives(
        &[
            follow("player", "", [10.0, 20.0, 30.0]),
            follow("player", "Arrille's Tradehouse", [1.0, 2.0, 3.0]),
            follow("player", "", [NOWHERE, NOWHERE, NOWHERE]),
        ],
        &ctx(&r),
    );
    assert!(out.skipped.is_empty());
    for d in &out.directives {
        assert_eq!(d.spec.kind, DirectiveKind::Follow(PLAYER));
        assert!(d.spec.repeat);
        assert_eq!(d.spec.limit, None);
    }
    let PackageExtra::FollowUntil(end) = &out.directives[0].extra else { panic!("no end") };
    assert_at(end, &exterior(), [10.0, 20.0, 30.0]);
    let PackageExtra::FollowUntil(end) = &out.directives[1].extra else { panic!("no end") };
    assert_at(end, &interior("Arrille's Tradehouse"), [1.0, 2.0, 3.0]);
    assert_eq!(out.directives[2].extra, PackageExtra::None);
}

#[test]
fn activate_uses_the_resolved_placement() {
    let r = lookup();
    let out = directives(&[activate("lever_01")], &ctx(&r));
    let d = &out.directives[0];
    assert_eq!(d.spec.kind, DirectiveKind::Use(LEVER));
    assert_eq!(d.spec.source, DirectiveSource::Record);
    assert_eq!(d.spec.priority, 0);
    assert_eq!(d.spec.limit, None);
    assert_eq!(d.extra, PackageExtra::None);
}

#[test]
fn list_keeps_record_order() {
    let r = lookup();
    let list = [activate("lever_01"), travel([0.0, 0.0, 0.0]), wander(0, 0), follow("player", "", [0.0; 3])];
    let out = directives(&list, &ctx(&r));
    let names: Vec<_> = out.specs().map(|s| s.kind.name()).collect();
    assert_eq!(names, ["use", "go_to", "stay_near", "follow"]);
    let indices: Vec<_> = out.directives.iter().map(|d| d.index).collect();
    assert_eq!(indices, [0, 1, 2, 3]);
}

#[test]
fn unresolvable_targets_are_skipped_and_reported() {
    let r = lookup();
    let list = [
        follow("nobody", "", [0.0; 3]),
        wander(0, 0),
        escort("ghost", "", [1.0; 3]),
        activate("missing_door"),
    ];
    let out = directives(&list, &ctx(&r));
    assert_eq!(out.directives.len(), 1);
    assert_eq!(out.directives[0].index, 1);
    assert_eq!(
        out.skipped,
        vec![
            Skipped { index: 0, tag: "AI_F", reason: SkipReason::UnknownActor("nobody".into()) },
            Skipped { index: 2, tag: "AI_E", reason: SkipReason::UnknownActor("ghost".into()) },
            Skipped { index: 3, tag: "AI_A", reason: SkipReason::UnknownObject("missing_door".into()) },
        ]
    );
}
