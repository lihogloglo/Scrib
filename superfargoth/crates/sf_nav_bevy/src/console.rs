//! The `nav.*` console commands (nav.md "Debug draw", console.md): `nav.stats` and `nav.draw`.

use bevy_app::App;
use sf_console_bevy::ConsoleAppExt;
use sf_console_bevy::sf_console::{ArgKind, CommandSpec, Param};

use crate::{NavDebugPaths, NavDraw, NavMeshes, NavStats, PathLayer};

/// Every `nav.draw` layer, then `off`.
const LAYERS: [&str; 9] = [
    "tiles",
    "links",
    "doors",
    "obstacles",
    "paths",
    "boundary",
    "coarse",
    "graph",
    "off",
];

/// The layers whose data a later slice builds, and that slice.
fn not_built(layer: &str) -> Option<&'static str> {
    Some(match layer {
        "links" => "NV9",
        "doors" => "NV4",
        "obstacles" => "NV10",
        "boundary" => "NV7b",
        "coarse" => "NV12",
        "graph" => "NV11",
        _ => return None,
    })
}

/// Appended to state lines when the gizmos aren't compiled in.
const NO_GIZMOS: &str = if cfg!(feature = "debug") {
    ""
} else {
    " (no gizmos: sf_nav_bevy is built without feature `debug`)"
};

pub(crate) fn register(app: &mut App) {
    app.add_console_command(
        CommandSpec::new("nav.stats").help(
            "Navmesh tiles resident per agent class, polygons, MiB, query queue depth, ms per tick, rebuilds",
        ),
        |inv| {
            let line = NavStats::of(inv.ctx).line();
            inv.log.info(&line);
            Ok(())
        },
    )
    .add_console_command(
        CommandSpec::new("nav.draw")
            .help(
                "Toggle a navmesh debug layer: tiles, paths [actor] (all recorded paths without one), \
                 links, doors, obstacles, boundary <actor>, coarse, graph; `off` clears every layer",
            )
            .param(Param::new("layer", ArgKind::choice(&LAYERS)))
            .param(Param::new("actor", ArgKind::Str).optional()),
        |inv| {
            let layer = inv.args.str("layer").unwrap_or_default().to_owned();
            let actor = inv
                .args
                .str("actor")
                .map(str::trim)
                .filter(|s| !s.is_empty())
                .map(str::to_owned);
            if let Some(slice) = not_built(&layer) {
                inv.log.warn(&format!("nav.draw {layer}: not built yet ({slice})"));
                return Ok(());
            }
            let world = &mut *inv.ctx;
            let line = match layer.as_str() {
                "off" => {
                    *world.resource_mut::<NavDraw>() = NavDraw::default();
                    "nav.draw: every layer off".to_owned()
                }
                "tiles" => {
                    let tiles = world
                        .get_resource::<NavMeshes>()
                        .map_or(0, |b| b.0.tile_summaries().len());
                    let mut draw = world.resource_mut::<NavDraw>();
                    draw.tiles = !draw.tiles;
                    if draw.tiles {
                        format!("nav.draw tiles on ({tiles} resident){NO_GIZMOS}")
                    } else {
                        "nav.draw tiles off".to_owned()
                    }
                }
                "paths" => {
                    let want = actor.map_or(PathLayer::All, PathLayer::Actor);
                    let recorded = world.resource::<NavDebugPaths>().0.len();
                    let mut draw = world.resource_mut::<NavDraw>();
                    if draw.paths.as_ref() == Some(&want) {
                        draw.paths = None;
                        "nav.draw paths off".to_owned()
                    } else {
                        let whose = match &want {
                            PathLayer::All => format!("all {recorded} recorded"),
                            PathLayer::Actor(a) => format!("actor {a}"),
                        };
                        draw.paths = Some(want);
                        format!("nav.draw paths on ({whose}){NO_GIZMOS}")
                    }
                }
                other => return Err(format!("nav.draw: unknown layer {other:?}")),
            };
            inv.log.info(&line);
            Ok(())
        },
    );
}
