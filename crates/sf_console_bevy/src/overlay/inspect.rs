//! The inspector: a read-only panel about the selection (Name, id, transform, parent chain,
//! game rows from [`InspectorRows`] hooks, reflected components).

use std::fmt::Write as _;

use bevy_app::{App, PostUpdate};
use bevy_camera::visibility::Visibility;
use bevy_ecs::prelude::*;
use bevy_ecs::reflect::{AppTypeRegistry, ReflectComponent};
use bevy_math::EulerRot;
use bevy_reflect::enums::VariantType;
use bevy_reflect::{PartialReflect, ReflectRef};
use bevy_text::{TextColor, TextSpan};
use bevy_transform::components::Transform;
use bevy_ui::widget::Text;
use bevy_ui::{BackgroundColor, Display, Node, Overflow, PositionType, UiRect, UiSystems, Val};

use super::select::{ConsoleSelection, label};
use super::{BlocksPick, ConsoleStyle};
use crate::ConsoleOpen;

/// Adds game rows to the inspector: `(label, value)` pairs for the selected entity (record
/// id, cell, owner...). Called on the inspector's refresh, with the world read-only.
pub type InspectorRowsFn = Box<dyn Fn(&World, Entity, &mut Vec<(String, String)>) + Send + Sync>;

/// The app's inspector hooks, in registration order.
#[derive(Resource, Default)]
pub struct InspectorRows(pub Vec<InspectorRowsFn>);

/// Registration of inspector hooks.
pub trait ConsoleOverlayAppExt {
    /// `app.add_inspector_rows(|world, e, rows| rows.push(("record".into(), id_of(world, e))))`.
    fn add_inspector_rows(
        &mut self,
        rows: impl Fn(&World, Entity, &mut Vec<(String, String)>) + Send + Sync + 'static,
    ) -> &mut Self;
}

impl ConsoleOverlayAppExt for App {
    fn add_inspector_rows(
        &mut self,
        rows: impl Fn(&World, Entity, &mut Vec<(String, String)>) + Send + Sync + 'static,
    ) -> &mut Self {
        self.world_mut()
            .get_resource_or_init::<InspectorRows>()
            .0
            .push(Box::new(rows));
        self
    }
}

/// Frames between refreshes of an unchanged selection (transforms move under it).
const REFRESH_FRAMES: u32 = 10;
/// Longest component value shown.
const MAX_VALUE: usize = 160;

#[derive(Component)]
pub(super) struct InspectorPanel;

pub(super) fn build(app: &mut App) {
    app.init_resource::<InspectorRows>()
        .add_systems(PostUpdate, refresh_panel.before(UiSystems::Prepare));
}

pub(super) fn spawn_panel(root: &mut ChildSpawnerCommands, style: &ConsoleStyle, top: Val) {
    root.spawn((
        InspectorPanel,
        BlocksPick,
        Node {
            display: Display::None,
            position_type: PositionType::Absolute,
            top,
            right: Val::Px(0.0),
            width: Val::Percent(36.0),
            max_height: Val::Percent(100.0 - style.height_percent),
            padding: UiRect::all(Val::Px(6.0)),
            overflow: Overflow::clip(),
            ..Node::default()
        },
        BackgroundColor(style.background),
        Text::default(),
        style.font.clone(),
        TextColor(style.level_colors[1]),
    ))
    .with_child((
        TextSpan::default(),
        style.font.clone(),
        TextColor(style.level_colors[0]),
    ));
}

/// Rows for `e`: what the panel shows, also usable by other front ends and tests.
pub fn inspect(world: &World, e: Entity) -> Vec<(String, String)> {
    let mut rows: Vec<(String, String)> = Vec::new();
    let mut row = |k: &str, v: String| rows.push((k.to_owned(), v));
    let Ok(entity) = world.get_entity(e) else {
        row("entity", format!("{e} (despawned)"));
        return rows;
    };
    if let Some(name) = entity.get::<Name>() {
        row("name", name.as_str().to_owned());
    }
    row("entity", e.to_string());
    if let Some(t) = entity.get::<Transform>() {
        let (y, x, z) = t.rotation.to_euler(EulerRot::YXZ);
        row("translation", fmt3(t.translation.to_array()));
        row(
            "rotation",
            format!(
                "yaw {:.1} pitch {:.1} roll {:.1}",
                y.to_degrees(),
                x.to_degrees(),
                z.to_degrees()
            ),
        );
        row("scale", fmt3(t.scale.to_array()));
    }
    if let Some(v) = entity.get::<Visibility>() {
        row("visibility", format!("{v:?}"));
    }
    let mut chain = Vec::new();
    let mut at = e;
    while let Some(parent) = world.get::<ChildOf>(at).map(ChildOf::parent) {
        chain.push(label(world, parent));
        at = parent;
    }
    if !chain.is_empty() {
        chain.reverse();
        row("parents", chain.join(" > "));
    }
    if let Some(children) = entity.get::<Children>() {
        row("children", children.len().to_string());
    }
    if let Some(hooks) = world.get_resource::<InspectorRows>() {
        for hook in &hooks.0 {
            hook(world, e, &mut rows);
        }
    }
    let registry = world.get_resource::<AppTypeRegistry>().map(|r| r.read());
    let mut components: Vec<(String, String)> = world
        .inspect_entity(e)
        .map(|infos| {
            infos
                .map(|(_, info)| {
                    let registered = info
                        .type_id()
                        .zip(registry.as_ref())
                        .and_then(|(id, reg)| reg.get(id));
                    // The registry knows the name even without Bevy's `debug` feature.
                    let name = registered.map_or_else(
                        || info.name().shortname().to_string(),
                        |r| r.type_info().type_path_table().short_path().to_owned(),
                    );
                    let value = registered
                        .and_then(|r| r.data::<ReflectComponent>())
                        .and_then(|rc| rc.reflect(entity))
                        .map_or_else(
                            || "(not reflected)".to_owned(),
                            |r| {
                                let mut out = String::new();
                                compact(r.as_partial_reflect(), 0, &mut out);
                                shorten(&out)
                            },
                        );
                    (name, value)
                })
                .collect()
        })
        .unwrap_or_default();
    components.sort();
    rows.push(("components".into(), components.len().to_string()));
    rows.extend(components.into_iter().map(|(k, v)| (format!("  {k}"), v)));
    rows
}

fn fmt3(v: [f32; 3]) -> String {
    format!("{:.3} {:.3} {:.3}", v[0], v[1], v[2])
}

/// A reflected value on one line: fields by name, numbers and opaque values by `Debug`,
/// unit structs as their name, collections with their first items.
// The wildcard arm also covers `ReflectRef::Function`, which exists only with bevy_reflect's
// `functions` feature (any crate in the app may turn it on).
#[allow(clippy::too_many_lines, clippy::match_wildcard_for_single_variants)]
fn compact(value: &dyn PartialReflect, depth: usize, out: &mut String) {
    const MAX_ITEMS: usize = 6;
    if depth > 4 {
        out.push_str("..");
        return;
    }
    if let Some(name) = value.try_downcast_ref::<Name>() {
        let _ = write!(out, "{:?}", name.as_str());
        return;
    }
    let short = || {
        value
            .get_represented_type_info()
            .map_or("?", |t| t.type_path_table().short_path())
    };
    let list = |items: &mut dyn Iterator<Item = (Option<&str>, &dyn PartialReflect)>,
                out: &mut String| {
        for (i, (name, v)) in items.enumerate() {
            if i == MAX_ITEMS {
                out.push_str(", ..");
                break;
            }
            if i > 0 {
                out.push_str(", ");
            }
            if let Some(name) = name {
                let _ = write!(out, "{name}: ");
            }
            compact(v, depth + 1, out);
        }
    };
    match value.reflect_ref() {
        ReflectRef::Struct(s) if s.field_len() == 0 => out.push_str(short()),
        ReflectRef::Struct(s) => {
            out.push_str("{ ");
            list(
                &mut (0..s.field_len()).filter_map(|i| Some((s.name_at(i), s.field_at(i)?))),
                out,
            );
            out.push_str(" }");
        }
        ReflectRef::TupleStruct(t) if t.field_len() == 1 => {
            compact(t.field(0).expect("one field"), depth + 1, out);
        }
        ReflectRef::TupleStruct(t) => {
            out.push('(');
            list(
                &mut (0..t.field_len()).filter_map(|i| Some((None, t.field(i)?))),
                out,
            );
            out.push(')');
        }
        ReflectRef::Tuple(t) => {
            out.push('(');
            list(
                &mut (0..t.field_len()).filter_map(|i| Some((None, t.field(i)?))),
                out,
            );
            out.push(')');
        }
        ReflectRef::List(l) => {
            let _ = write!(out, "[{}: ", l.len());
            list(&mut l.iter().map(|v| (None, v)), out);
            out.push(']');
        }
        ReflectRef::Array(a) => {
            out.push('[');
            list(&mut a.iter().map(|v| (None, v)), out);
            out.push(']');
        }
        ReflectRef::Set(set) => {
            let _ = write!(out, "{{{}: ", set.len());
            list(&mut set.iter().map(|v| (None, v)), out);
            out.push('}');
        }
        ReflectRef::Map(m) => {
            let _ = write!(out, "{{{} entries}}", m.len());
        }
        ReflectRef::Enum(e) => {
            out.push_str(e.variant_name());
            match e.variant_type() {
                VariantType::Unit => {}
                VariantType::Tuple => {
                    out.push('(');
                    list(
                        &mut (0..e.field_len()).filter_map(|i| Some((None, e.field_at(i)?))),
                        out,
                    );
                    out.push(')');
                }
                VariantType::Struct => {
                    out.push_str(" { ");
                    list(
                        &mut (0..e.field_len())
                            .filter_map(|i| Some((e.name_at(i), e.field_at(i)?))),
                        out,
                    );
                    out.push_str(" }");
                }
            }
        }
        // Opaque values (and functions).
        _ => {
            let _ = write!(out, "{value:?}");
        }
    }
}

/// One line, at most [`MAX_VALUE`] characters.
fn shorten(text: &str) -> String {
    let flat: String = text.split_whitespace().collect::<Vec<_>>().join(" ");
    match flat.char_indices().nth(MAX_VALUE) {
        Some((i, _)) => format!("{}...", &flat[..i]),
        None => flat,
    }
}

/// Redraws the panel when the selection changes, and every few frames while it stays.
fn refresh_panel(world: &mut World, mut frames: Local<u32>, mut shown: Local<Option<Entity>>) {
    let open = world.resource::<ConsoleOpen>().0;
    let selection = world.resource::<ConsoleSelection>().0.filter(|_| open);
    *frames += 1;
    if selection == *shown && (selection.is_none() || *frames < REFRESH_FRAMES) {
        return;
    }
    *frames = 0;
    *shown = selection;
    let (title, body) = match selection {
        Some(e) => {
            let rows = inspect(world, e);
            let body: Vec<String> = rows.iter().map(|(k, v)| format!("{k}: {v}")).collect();
            (format!("{}\n", label(world, e)), body.join("\n"))
        }
        None => (String::new(), String::new()),
    };
    let mut panels =
        world.query_filtered::<(&mut Node, &mut Text, &Children), With<InspectorPanel>>();
    let mut spans = Vec::new();
    for (mut node, mut text, children) in panels.iter_mut(world) {
        node.display = if selection.is_some() {
            Display::Flex
        } else {
            Display::None
        };
        text.0.clone_from(&title);
        spans.extend(children.iter());
    }
    for span in spans {
        if let Some(mut s) = world.get_mut::<TextSpan>(span) {
            s.0.clone_from(&body);
        }
    }
}
