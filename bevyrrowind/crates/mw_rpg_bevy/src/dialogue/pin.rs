//! The dialogue partner's AI pin (slice `MA20b`; `ai-adapter.md` "As built (`MA20b`)"): the NPC
//! the player talks to carries `AiPin(Pin::Dialogue)` while the talk is open, so the tier
//! allocator keeps it Full whatever the caps (pins exceed them, `ai.md` "Tiers and budget").
//!
//! One system, `pin_partner`, in `PostUpdate`, run only when [`ActiveTalk`] appears, changes or
//! goes (never a per-tick scan of the actors): every `Update` command that opens or ends a talk
//! has landed by then, and the next frame's `FixedUpdate` (`AiAllocSystems::Allocate`) reads the
//! pin. It finds the speaker's [`AiActor`] by key, inserts the pin there and removes it from the
//! previous partner. It only removes a pin it set: an actor that already carries a pin (combat, a
//! scripted scene) keeps it and is left alone, before, during and after the talk.

use bevy_app::{App, PostUpdate};
use bevy_ecs::prelude::*;
use sf_ai::Pin;
use sf_ai_bevy::{AiActor, AiPin};
use sf_rpg::ActorKey;

use super::ActiveTalk;

pub(super) fn add(app: &mut App) {
    app.add_systems(PostUpdate, pin_partner.run_if(resource_changed_or_removed::<ActiveTalk>));
}

/// What `pin_partner` last did: the speaker it handled (`None`: no talk) and the entity it pinned
/// (`None`: no `AiActor` for that speaker, or one already pinned for another reason).
#[derive(Default)]
struct Pinned {
    speaker: Option<ActorKey>,
    entity: Option<Entity>,
}

fn pin_partner(
    active: Option<Res<ActiveTalk>>,
    actors: Query<(Entity, &AiActor, Option<&AiPin>)>,
    mut pinned: Local<Pinned>,
    mut commands: Commands,
) {
    let speaker = active.map(|a| a.talk.speaker);
    // The same talk, mutated (a topic, a service): nothing to move.
    if speaker == pinned.speaker {
        return;
    }
    if let Some(e) = pinned.entity.take()
        && let Ok((_, _, Some(AiPin(Pin::Dialogue)))) = actors.get(e)
    {
        commands.entity(e).try_remove::<AiPin>();
    }
    pinned.speaker = speaker;
    let Some(speaker) = speaker else { return };
    let Some((e, _, pin)) = actors.iter().find(|(_, a, _)| a.0 == speaker) else { return };
    if pin.is_none() {
        commands.entity(e).try_insert(AiPin(Pin::Dialogue));
        pinned.entity = Some(e);
    }
}

#[cfg(test)]
mod tests {
    //! Headless: the allocator over actors with a `Mover`, caps of 0 Full slots, talks opened and
    //! closed by inserting and removing [`ActiveTalk`] as the dialogue host does.
    use std::sync::Arc;

    use bevy_app::FixedUpdate;
    use bevy_time::{Fixed, Time};
    use mw_rpg::NoSheets;
    use sf_ai::{Tier, TierCaps};
    use sf_ai_bevy::{AiAllocPlugin, AiAllocSettings, AiTier};
    use sf_controller_bevy::{Mover, MoverParams};
    use sf_coords::WorldPos;
    use sf_records::RecordDb;
    use tes3::esp::Npc;

    use super::super::{Rpg, id};
    use super::*;

    const PARTNER: ActorKey = ActorKey(7);
    const OTHER: ActorKey = ActorKey(8);

    /// Two NPCs the player may talk to, `partner` and `other`.
    fn rpg() -> Rpg {
        let mut db = RecordDb::new();
        mw_records::register_kinds(&mut db).unwrap();
        let src = db.add_source("test", &[]);
        for name in ["partner", "other"] {
            let npc = Npc { id: name.into(), name: name.into(), ..Default::default() };
            db.define(src, id(name), npc).unwrap();
        }
        let mut rpg = Rpg::new(Arc::new(db)).unwrap();
        assert!(rpg.add_actor(PARTNER, &id("partner"), "Balmora"));
        assert!(rpg.add_actor(OTHER, &id("other"), "Balmora"));
        rpg
    }

    /// The allocator with no Full slot and no Reduced slot, plus the pin.
    fn app() -> App {
        let mut app = App::new();
        app.insert_resource(Time::<Fixed>::from_hz(60.0)).add_plugins(AiAllocPlugin { timings: false });
        // The first frame binds the `ai.alloc.*` tunables' defaults; the override goes after it.
        app.update();
        app.world_mut().resource_mut::<AiAllocSettings>().set_caps(TierCaps { full: 0, reduced: 0 });
        app.insert_resource(rpg());
        add(&mut app);
        app
    }

    fn actor(app: &mut App, key: ActorKey, extra: Option<AiPin>) -> Entity {
        let mover = Mover::new(WorldPos::ORIGIN, 0.0, MoverParams::default(), WorldPos::ORIGIN);
        let mut e = app.world_mut().spawn((AiActor(key), mover));
        if let Some(pin) = extra {
            e.insert(pin);
        }
        e.id()
    }

    /// One frame, then the next fixed tick (the allocation that reads the pin).
    fn frame(app: &mut App) {
        app.update();
        app.world_mut().run_schedule(FixedUpdate);
    }

    fn open(app: &mut App, speaker: ActorKey) {
        let (talk, _) = app.world_mut().resource_mut::<Rpg>().open(speaker, &NoSheets).unwrap();
        app.insert_resource(ActiveTalk { talk, screen: Entity::PLACEHOLDER });
        frame(app);
    }

    fn close(app: &mut App) {
        app.world_mut().remove_resource::<ActiveTalk>();
        frame(app);
    }

    fn pin(app: &App, e: Entity) -> Option<Pin> {
        app.world().get::<AiPin>(e).map(|p| p.0)
    }

    fn tier(app: &App, e: Entity) -> Tier {
        app.world().get::<AiTier>(e).expect("allocated").tier
    }

    #[test]
    fn the_partner_is_pinned_full_while_the_talk_is_open() {
        let mut app = app();
        let partner = actor(&mut app, PARTNER, None);
        frame(&mut app);
        assert_eq!(pin(&app, partner), None);
        assert_ne!(tier(&app, partner), Tier::Full, "no Full slot without a pin");

        open(&mut app, PARTNER);
        assert_eq!(pin(&app, partner), Some(Pin::Dialogue));
        assert_eq!(tier(&app, partner), Tier::Full, "the pin exceeds a Full cap of 0");
        // A frame with the talk still open (and touched) keeps the pin.
        app.world_mut().resource_mut::<ActiveTalk>().set_changed();
        frame(&mut app);
        assert_eq!(pin(&app, partner), Some(Pin::Dialogue));

        close(&mut app);
        assert_eq!(pin(&app, partner), None);
        assert_ne!(tier(&app, partner), Tier::Full);
    }

    #[test]
    fn a_new_partner_takes_the_pin_from_the_previous_one() {
        let mut app = app();
        let partner = actor(&mut app, PARTNER, None);
        let other = actor(&mut app, OTHER, None);
        open(&mut app, PARTNER);
        open(&mut app, OTHER);
        assert_eq!(pin(&app, partner), None);
        assert_eq!(pin(&app, other), Some(Pin::Dialogue));
        assert_eq!(tier(&app, other), Tier::Full);
        close(&mut app);
        assert_eq!(pin(&app, other), None);
    }

    #[test]
    fn a_combat_pin_is_left_alone() {
        let mut app = app();
        let partner = actor(&mut app, PARTNER, Some(AiPin(Pin::Combat)));
        open(&mut app, PARTNER);
        assert_eq!(pin(&app, partner), Some(Pin::Combat));
        assert_eq!(tier(&app, partner), Tier::Full);
        close(&mut app);
        assert_eq!(pin(&app, partner), Some(Pin::Combat));
    }
}
