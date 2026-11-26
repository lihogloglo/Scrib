//! The `ai.*` console commands (ai.md "Debugging", console.md). They read the trace store and
//! the library; `ai.select` also watches the actor (tracing is the debugger's own state, never
//! gameplay's).

use bevy_app::App;
use bevy_ecs::prelude::*;
use sf_ai::{ActorKey, CpuTier, TRACE_LEN};
use sf_console_bevy::ConsoleAppExt;
use sf_console_bevy::sf_console::{ArgKind, CommandSpec, Param};

use crate::alloc::CPU_TIER_CHOICES;
use crate::{
    AiAllocSettings, AiAllocStats, AiDecideStats, AiLibrary, AiLogical, AiLogicalStats, AiSelected, AiTraces, DEFAULT_TRACE_LINES, actor_hex, directives_text,
    parse_actor, trace_text,
};

const NO_ACTOR: &str = "no actor selected: `ai.select <id>`, or click one with the console open and `ai.select`";

pub(crate) fn register(app: &mut App) {
    app.add_console_command(
        CommandSpec::new("ai.select")
            .help(
                "Watch an actor and show it in the AI overlay: its key (hex), nothing for the clicked \
                 reference, or `off`",
            )
            .param(Param::new("actor", ArgKind::Str).optional()),
        |inv| {
            let arg = inv.args.str("actor").map(str::trim).filter(|s| !s.is_empty()).map(str::to_owned);
            let world = &mut *inv.ctx;
            let text = match arg.as_deref() {
                Some(off) if off.eq_ignore_ascii_case("off") || off.eq_ignore_ascii_case("none") => {
                    match world.resource_mut::<AiSelected>().0.take() {
                        Some(key) => {
                            world.resource_mut::<AiTraces>().unwatch(key);
                            format!("deselected actor {} (unwatched, trace dropped)", actor_hex(key))
                        }
                        None => "no actor selected".to_owned(),
                    }
                }
                Some(id) => {
                    let key = parse_actor(id).ok_or_else(|| format!("bad actor key {id:?}: hex, up to 32 digits"))?;
                    select(world, key)
                }
                None => {
                    let key = clicked_actor(world)?;
                    select(world, key)
                }
            };
            inv.log.info(&text);
            Ok(())
        },
    )
    .add_console_command(
        CommandSpec::new("ai.trace")
            .help("The last decision steps of the selected (or given) actor, oldest first")
            .param(Param::new("n", ArgKind::Int { min: 1, max: i64::try_from(TRACE_LEN).unwrap_or(i64::MAX) }).optional())
            .param(Param::new("actor", ArgKind::Str).optional()),
        |inv| {
            let n = inv
                .args
                .int("n")
                .and_then(|n| usize::try_from(n).ok())
                .unwrap_or(DEFAULT_TRACE_LINES);
            let world = &*inv.ctx;
            let key = match inv.args.str("actor").map(str::trim).filter(|s| !s.is_empty()) {
                Some(id) => parse_actor(id).ok_or_else(|| format!("bad actor key {id:?}: hex, up to 32 digits"))?,
                None => world.resource::<AiSelected>().0.ok_or(NO_ACTOR)?,
            };
            let text = trace_text(world.resource::<AiTraces>(), &world.resource::<AiLibrary>().library, key, n)?;
            for line in text.lines() {
                inv.log.info(line);
            }
            Ok(())
        },
    )
    .add_console_command(
        CommandSpec::new("ai.directives")
            .help("The selected (or given) actor's directive stack, top first, and what it serves")
            .param(Param::new("actor", ArgKind::Str).optional()),
        |inv| {
            let world = &mut *inv.ctx;
            let key = match inv.args.str("actor").map(str::trim).filter(|s| !s.is_empty()) {
                Some(id) => parse_actor(id).ok_or_else(|| format!("bad actor key {id:?}: hex, up to 32 digits"))?,
                None => world.resource::<AiSelected>().0.ok_or(NO_ACTOR)?,
            };
            for line in directives_text(world, key)?.lines() {
                inv.log.info(line);
            }
            Ok(())
        },
    );
    register_summaries(app);
}

/// `ai.tiers` and `ai.stats`.
fn register_summaries(app: &mut App) {
    app.add_console_command(
        CommandSpec::new("ai.tiers").help(
            "Actors per AI tier from the last allocation, against the caps, with pins, over-cap \
             actors, in-view holds and the CPU tier (from the traces on hosts without the allocator)",
        ),
        |inv| {
            let world = &*inv.ctx;
            let text = match (world.get_resource::<AiAllocStats>(), world.get_resource::<AiAllocSettings>()) {
                (Some(stats), Some(settings)) => tiers_text(stats, settings, world.get_resource::<AiDecideStats>()),
                _ => traced_tiers_text(world.resource::<AiTraces>()),
            };
            inv.log.info(&text);
            Ok(())
        },
    )
    .add_console_command(
        CommandSpec::new("ai.stats")
            .help(
                "Trace store settings and counts, HTN library size and load errors, the allocator's tick and cost, \
                 tier-0 states stepped and the promotion queue",
            ),
        |inv| {
            let traces = inv.ctx.resource::<AiTraces>();
            let lib = inv.ctx.resource::<AiLibrary>();
            let admit = crate::snake_name(&traces.admit());
            let on = |b: bool| if b { "on" } else { "off" };
            inv.log.info(&format!(
                "traces {}, admit {admit}, timings {}: {} traced of cap {}, {} watched; \
                 library {} tasks, {} methods, {} load errors",
                on(traces.enabled()),
                on(traces.timings()),
                traces.len(),
                traces.cap(),
                traces.watched().len(),
                lib.library.task_count(),
                lib.library.method_count(),
                lib.report.as_ref().map_or(0, |r| r.errors.len()),
            ));
            if let Some(alloc) = inv.ctx.get_resource::<AiAllocStats>() {
                inv.log.info(&match alloc.allocate_us {
                    Some(us) => format!("alloc: tick {}, allocate {us:.1} µs", alloc.tick),
                    None => format!("alloc: tick {}, timings off", alloc.tick),
                });
            }
            if let (Some(_), Some(l)) = (inv.ctx.get_resource::<AiLogical>(), inv.ctx.get_resource::<AiLogicalStats>()) {
                let cost = l.promote_us.map_or_else(|| "timings off".to_owned(), |us| format!("{us:.1} µs"));
                inv.log.info(&format!(
                    "logical: {} states, {} stepped; promote: queue {}, waiting {}, tried {}, placed {}, {cost}",
                    l.states, l.stepped, l.queued, l.waiting, l.tried, l.placed,
                ));
            }
            Ok(())
        },
    );
}

/// The `ai.tiers` line from the allocator's last run, with the plans of the last second (`AI9g`).
fn tiers_text(alloc: &AiAllocStats, settings: &AiAllocSettings, decides: Option<&AiDecideStats>) -> String {
    let (s, caps) = (&alloc.stats, alloc.caps);
    let tier = CpuTier::ALL
        .iter()
        .position(|&t| t == alloc.cpu_tier)
        .and_then(|i| CPU_TIER_CHOICES.get(i + 1))
        .copied()
        .unwrap_or("?");
    let how = if settings.cpu_tier_choice.is_some() { "set" } else { "auto" };
    let plans = decides.map_or_else(String::new, |d| {
        let d = d.last_second();
        format!(
            "; plans last second: full {}, reduced {}, untiered {} ({} on events)",
            d.full, d.reduced, d.untiered, d.events
        )
    });
    format!(
        "tiers: full {}/{}, reduced {}/{}, logical {}; pinned {}, over cap full {} reduced {}, \
         held in view {}; cpu tier {tier} ({how}){plans}",
        s.full, caps.full, s.reduced, caps.reduced, s.logical, s.pinned, s.over_cap_full, s.over_cap_reduced,
        s.held_in_view,
    )
}

/// The `ai.tiers` line without the allocator: each traced actor's last step.
fn traced_tiers_text(store: &AiTraces) -> String {
    let mut counts = [0usize; 3];
    let mut traced = 0;
    for (_, trace) in store.iter() {
        if let Some(e) = trace.last() {
            traced += 1;
            if let Some(c) = counts.get_mut(usize::from(e.tier)) {
                *c += 1;
            }
        }
    }
    format!(
        "tiers (no allocator: each traced actor's last step): t2 {}, t1 {}, t0 {} ({traced} traced)",
        counts[2], counts[1], counts[0]
    )
}

fn select(world: &mut World, key: ActorKey) -> String {
    world.resource_mut::<AiTraces>().watch(key);
    world.resource_mut::<AiSelected>().0 = Some(key);
    let entries = world.resource::<AiTraces>().get(key).map_or(0, sf_ai::Trace::len);
    format!("selected actor {}: watched, trace {entries}/{TRACE_LEN}", actor_hex(key))
}

/// The console's click-selected entity's actor key.
#[cfg(feature = "debug")]
fn clicked_actor(world: &World) -> Result<ActorKey, String> {
    let entity = world
        .get_resource::<sf_console_bevy::overlay::ConsoleSelection>()
        .and_then(|s| s.0)
        .ok_or(NO_ACTOR)?;
    world
        .get::<crate::AiActor>(entity)
        .map(|a| a.0)
        .ok_or_else(|| format!("entity {entity} is not an AI actor (no AiActor component)"))
}

#[cfg(not(feature = "debug"))]
fn clicked_actor(_: &World) -> Result<ActorKey, String> {
    Err("give the actor's key (hex); the clicked reference needs sf_ai_bevy's `debug` feature".to_owned())
}
