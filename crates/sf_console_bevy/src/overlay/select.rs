//! Click-select (Bethesda console style), the selection box, and commands on the selection.
//!
//! A left click with the console open casts a ray from the camera through the cursor
//! ([`ConsolePick`]); the nearest visible mesh it hits selects its nearest [`PickRoot`]
//! ancestor (the app marks its placed objects), or the mesh itself; a miss deselects.
//! The ray cast is `bevy_picking`'s [`MeshRayCast`], run once per click: no picking backend
//! raycasting every frame.

use bevy_app::{App, PostUpdate, PreUpdate};
use bevy_asset::Assets;
use bevy_camera::primitives::Aabb;
use bevy_camera::visibility::Visibility;
use bevy_camera::{Camera, Camera3d};
use bevy_color::Color;
use bevy_ecs::prelude::*;
use bevy_ecs::reflect::ReflectComponent;
use bevy_gizmos::config::GizmoConfigStore;
use bevy_gizmos::gizmos::Gizmos;
use bevy_input::ButtonInput;
use bevy_input::mouse::MouseButton;
use bevy_math::Vec3;
use bevy_mesh::Mesh;
use bevy_picking::mesh_picking::ray_cast::{MeshRayCast, MeshRayCastSettings, RayCastVisibility};
use bevy_reflect::Reflect;
use bevy_reflect::prelude::ReflectDefault;
use bevy_shape::Ray3d;
use bevy_transform::TransformSystems;
use bevy_transform::components::{GlobalTransform, Transform};
use bevy_ui::{ComputedNode, Node, UiGlobalTransform};
use bevy_window::{PrimaryWindow, Window};
use sf_console::{ArgKind, CommandSpec, Param};

use super::{BlocksPick, ConsoleInputSystems, over_panel};
use crate::{ConsoleAppExt, ConsoleOpen, ConsoleSystems};

/// The selected entity (the console's "current reference").
#[derive(Resource, Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct ConsoleSelection(pub Option<Entity>);

/// Marks what a click selects: a placed object's root, above its meshes.
#[derive(Component, Reflect, Clone, Copy, Debug, Default)]
#[reflect(Component, Default)]
pub struct PickRoot;

/// The camera to pick through and move with `moveto`. Without one: the active 3D camera
/// with the highest order.
#[derive(Component, Reflect, Clone, Copy, Debug, Default)]
#[reflect(Component, Default)]
pub struct ConsoleCamera;

/// Select what this ray hits first (a miss deselects). Written by a click; tests, gamepad
/// or VR front ends write their own.
#[derive(Message, Clone, Copy, Debug, PartialEq)]
pub struct ConsolePick {
    pub ray: Ray3d,
}

/// `moveto` put this camera here. The console sets the camera's `Transform`; a camera rig
/// that drives the camera itself reads this to move along.
#[derive(Message, Clone, Copy, Debug, PartialEq)]
pub struct ConsoleMoveTo {
    pub camera: Entity,
    pub transform: Transform,
}

/// Most entities `find` lists.
const FIND_MAX: usize = 50;
const NOTHING_SELECTED: &str = "nothing selected: click an object, or `prid <name>`";
const BOX_COLOR: Color = Color::srgb(1.0, 0.85, 0.2);

pub(super) fn build(app: &mut App) {
    app.init_resource::<ConsoleSelection>()
        .register_type::<PickRoot>()
        .register_type::<ConsoleCamera>()
        .add_message::<ConsolePick>()
        .add_message::<ConsoleMoveTo>()
        .add_systems(
            PreUpdate,
            (
                apply_pick
                    .run_if(resource_exists::<Assets<Mesh>>)
                    .after(click_to_pick)
                    .in_set(ConsoleInputSystems),
                forget_despawned.before(ConsoleSystems),
            ),
        )
        .add_systems(
            PostUpdate,
            draw_selection_box
                .after(TransformSystems::Propagate)
                .run_if(resource_exists::<GizmoConfigStore>)
                .run_if(|open: Res<ConsoleOpen>, sel: Res<ConsoleSelection>| {
                    open.0 && sel.0.is_some()
                }),
        );
    register_commands(app);
}

/// The camera to pick through: [`ConsoleCamera`] first, then the highest-order active
/// 3D camera.
fn pick_camera<'a>(
    cameras: impl Iterator<Item = (Entity, &'a Camera, &'a GlobalTransform, bool, bool)>,
) -> Option<(Entity, &'a Camera, &'a GlobalTransform)> {
    cameras
        .filter(|(_, camera, _, _, is_3d)| camera.is_active && *is_3d)
        .max_by_key(|(_, camera, _, marked, _)| (*marked, camera.order))
        .map(|(e, c, t, ..)| (e, c, t))
}

type CameraItem<'a> = (
    Entity,
    &'a Camera,
    &'a GlobalTransform,
    Has<ConsoleCamera>,
    Has<Camera3d>,
);

/// Left click with the console open, not over a console panel: a ray through the cursor.
pub(super) fn click_to_pick(
    open: Res<ConsoleOpen>,
    buttons: Option<Res<ButtonInput<MouseButton>>>,
    windows: Query<&Window, With<PrimaryWindow>>,
    cameras: Query<CameraItem>,
    panels: Query<(&ComputedNode, &UiGlobalTransform, &Node), With<BlocksPick>>,
    mut picks: MessageWriter<ConsolePick>,
) {
    if !open.0 || !buttons.is_some_and(|b| b.just_pressed(MouseButton::Left)) {
        return;
    }
    let Ok(window) = windows.single() else {
        return;
    };
    let (Some(cursor), Some(physical)) =
        (window.cursor_position(), window.physical_cursor_position())
    else {
        return;
    };
    if over_panel(physical, &panels) {
        return;
    }
    let Some((_, camera, transform)) = pick_camera(cameras.iter()) else {
        return;
    };
    let origin = camera
        .logical_viewport_rect()
        .map_or(cursor, |r| cursor - r.min);
    if let Ok(ray) = camera.viewport_to_world(transform, origin) {
        picks.write(ConsolePick { ray });
    }
}

/// The nearest visible mesh along each pick ray selects its [`PickRoot`] (or itself).
fn apply_pick(
    mut picks: MessageReader<ConsolePick>,
    mut ray_cast: MeshRayCast,
    parents: Query<&ChildOf>,
    roots: Query<(), With<PickRoot>>,
    mut selection: ResMut<ConsoleSelection>,
) {
    for pick in picks.read() {
        let settings = MeshRayCastSettings::default().with_visibility(RayCastVisibility::Visible);
        let hit = ray_cast
            .cast_ray(pick.ray, &settings)
            .first()
            .map(|(e, _)| *e);
        selection.0 = hit.map(|e| pick_root(e, &parents, &roots));
    }
}

/// The nearest ancestor of `hit` (itself included) marked [`PickRoot`], else `hit`.
pub fn pick_root(
    hit: Entity,
    parents: &Query<&ChildOf>,
    roots: &Query<(), With<PickRoot>>,
) -> Entity {
    std::iter::once(hit)
        .chain(parents.iter_ancestors(hit))
        .find(|&e| roots.contains(e))
        .unwrap_or(hit)
}

fn forget_despawned(
    mut selection: ResMut<ConsoleSelection>,
    entities: &bevy_ecs::entity::Entities,
) {
    if selection.0.is_some_and(|e| !entities.contains(e)) {
        selection.0 = None;
    }
}

/// One box around the selection and its meshes, in the selection's own frame: the
/// Construction Set's selection box.
fn draw_selection_box(
    selection: Res<ConsoleSelection>,
    transforms: Query<&GlobalTransform>,
    children: Query<&Children>,
    bounds: Query<(&Aabb, &GlobalTransform)>,
    mut gizmos: Gizmos,
) {
    let Some(root) = selection.0 else {
        return;
    };
    let Ok(root_transform) = transforms.get(root) else {
        return;
    };
    let meshes = std::iter::once(root)
        .chain(children.iter_descendants(root))
        .filter_map(|e| bounds.get(e).ok());
    let (min, max) =
        local_box(root_transform, meshes).unwrap_or((Vec3::splat(-0.25), Vec3::splat(0.25)));
    let size = (max - min).max(Vec3::splat(1e-3));
    let local = Transform::from_translation((min + max) * 0.5).with_scale(size);
    gizmos.cube(root_transform.mul_transform(local), BOX_COLOR);
}

/// Bounds of `meshes` (each an `Aabb` in its own frame) in `root`'s frame.
fn local_box<'a>(
    root: &GlobalTransform,
    meshes: impl Iterator<Item = (&'a Aabb, &'a GlobalTransform)>,
) -> Option<(Vec3, Vec3)> {
    let to_root = root.affine().inverse();
    if !to_root.is_finite() {
        return None;
    }
    let mut out: Option<(Vec3, Vec3)> = None;
    for (aabb, transform) in meshes {
        let (center, half) = (Vec3::from(aabb.center), Vec3::from(aabb.half_extents));
        for i in 0..8u8 {
            let sign = Vec3::new(
                if i & 1 == 0 { -1.0 } else { 1.0 },
                if i & 2 == 0 { -1.0 } else { 1.0 },
                if i & 4 == 0 { -1.0 } else { 1.0 },
            );
            let p = to_root.transform_point3(transform.transform_point(center + sign * half));
            out = Some(out.map_or((p, p), |(lo, hi)| (lo.min(p), hi.max(p))));
        }
    }
    out
}

/// World transform from the local `Transform`s up the parent chain, so a command sees
/// the effect of the previous command on the same line (`GlobalTransform` updates later).
fn world_transform(world: &World, e: Entity) -> GlobalTransform {
    let mut out = GlobalTransform::from(world.get::<Transform>(e).copied().unwrap_or_default());
    let mut at = e;
    while let Some(parent) = world.get::<ChildOf>(at).map(ChildOf::parent) {
        let local = world.get::<Transform>(parent).copied().unwrap_or_default();
        out = GlobalTransform::from(local) * out;
        at = parent;
    }
    out
}

/// The selection, or why there is none.
fn selected(world: &World) -> Result<Entity, String> {
    world
        .get_resource::<ConsoleSelection>()
        .and_then(|s| s.0)
        .filter(|&e| world.entities().contains(e))
        .ok_or_else(|| NOTHING_SELECTED.to_owned())
}

/// `Name (12v0)`, or the id alone.
pub(crate) fn label(world: &World, e: Entity) -> String {
    match world.get::<Name>(e) {
        Some(name) => format!("{name} ({e})"),
        None => e.to_string(),
    }
}

fn fmt_vec(v: Vec3) -> String {
    format!("{:.3} {:.3} {:.3}", v.x, v.y, v.z)
}

/// Sets `e`'s world position, keeping its parent.
fn set_world_position(world: &mut World, e: Entity, position: Vec3) -> Result<(), String> {
    let local = match world.get::<ChildOf>(e).map(ChildOf::parent) {
        Some(parent) => world_transform(world, parent)
            .affine()
            .inverse()
            .transform_point3(position),
        None => position,
    };
    if world.get::<Transform>(e).is_none() {
        return Err(format!("{} has no Transform", label(world, e)));
    }
    world.get_mut::<Transform>(e).expect("checked").translation = local;
    Ok(())
}

/// World-space center and radius of `e` and its meshes (last frame's transforms).
fn world_bounds(world: &mut World, e: Entity) -> (Vec3, f32) {
    let mut meshes = world.query::<(&Aabb, &GlobalTransform)>();
    let mut children = world.query::<&Children>();
    let ids: Vec<Entity> = std::iter::once(e)
        .chain(children.query(world).iter_descendants(e))
        .collect();
    let identity = GlobalTransform::IDENTITY;
    let found = local_box(
        &identity,
        ids.iter().filter_map(|&id| meshes.get(world, id).ok()),
    );
    match found {
        Some((min, max)) => ((min + max) * 0.5, ((max - min) * 0.5).length().max(0.1)),
        None => (world_transform(world, e).translation(), 1.0),
    }
}

#[allow(clippy::too_many_lines)]
fn register_commands(app: &mut App) {
    app.add_console_command(
        CommandSpec::new("prid")
            .help("Select an entity by Name (case-insensitive; `find` lists names)")
            .param(Param::rest("name")),
        |inv| {
            let wanted = inv.args.str("name").unwrap_or_default().trim_matches('"');
            let world = &mut *inv.ctx;
            let mut named = world.query::<(Entity, &Name, Has<PickRoot>)>();
            let mut exact: Vec<(bool, Entity)> = named
                .iter(world)
                .filter(|(_, n, _)| n.as_str().eq_ignore_ascii_case(wanted))
                .map(|(e, _, root)| (!root, e))
                .collect();
            exact.sort();
            let Some(&(_, e)) = exact.first() else {
                return Err(format!("no entity named {wanted:?}; try `find {wanted}`"));
            };
            world.resource_mut::<ConsoleSelection>().0 = Some(e);
            let more = if exact.len() > 1 {
                format!(" (first of {} with this name)", exact.len())
            } else {
                String::new()
            };
            inv.log.info(&format!("selected {}{more}", label(world, e)));
            Ok(())
        },
    )
    .add_console_command(
        CommandSpec::new("find")
            .help("List entities whose Name contains text")
            .param(Param::rest("text")),
        |inv| {
            let text = inv
                .args
                .str("text")
                .unwrap_or_default()
                .trim_matches('"')
                .to_lowercase();
            let world = &mut *inv.ctx;
            let mut named = world.query::<(Entity, &Name)>();
            let mut hits: Vec<(String, Entity)> = named
                .iter(world)
                .filter(|(_, n)| n.as_str().to_lowercase().contains(&text))
                .map(|(e, n)| (n.as_str().to_owned(), e))
                .collect();
            hits.sort();
            inv.log
                .info(&format!("{} entities match {text:?}", hits.len()));
            for (name, e) in hits.iter().take(FIND_MAX) {
                inv.log.info(&format!("  {name} ({e})"));
            }
            if hits.len() > FIND_MAX {
                inv.log
                    .info(&format!("  ... {} more", hits.len() - FIND_MAX));
            }
            Ok(())
        },
    )
    .add_console_command(
        CommandSpec::new("getpos").help("Print the selection's world position"),
        |inv| {
            let e = selected(inv.ctx)?;
            let p = world_transform(inv.ctx, e).translation();
            inv.log
                .info(&format!("{}: {}", label(inv.ctx, e), fmt_vec(p)));
            Ok(())
        },
    )
    .add_console_command(
        CommandSpec::new("setpos")
            .help("Move the selection to a world position")
            .param(Param::new("x", ArgKind::float()))
            .param(Param::new("y", ArgKind::float()))
            .param(Param::new("z", ArgKind::float())),
        |inv| {
            let e = selected(inv.ctx)?;
            #[allow(clippy::cast_possible_truncation)]
            let p = ["x", "y", "z"].map(|a| inv.args.float(a).unwrap_or_default() as f32);
            set_world_position(inv.ctx, e, Vec3::from_array(p))?;
            inv.log.info(&format!(
                "{}: {}",
                label(inv.ctx, e),
                fmt_vec(world_transform(inv.ctx, e).translation())
            ));
            Ok(())
        },
    )
    .add_console_command(
        CommandSpec::new("moveto").help("Move the camera to look at the selection"),
        |inv| {
            let e = selected(inv.ctx)?;
            let world = &mut *inv.ctx;
            let mut cameras = world.query::<CameraItem>();
            let camera = pick_camera(cameras.iter(world)).map(|(c, ..)| c);
            let camera = camera.ok_or("no active 3D camera")?;
            let (center, radius) = world_bounds(world, e);
            let from = world_transform(world, camera);
            let forward = from.forward().as_vec3();
            let eye = center - forward * (radius * 2.5 + 1.0);
            let target = Transform::from_translation(eye).looking_at(center, Vec3::Y);
            let local = match world.get::<ChildOf>(camera).map(ChildOf::parent) {
                Some(parent) => {
                    let parent = world_transform(world, parent);
                    GlobalTransform::from(target).reparented_to(&parent)
                }
                None => target,
            };
            if let Some(mut t) = world.get_mut::<Transform>(camera) {
                *t = local;
            }
            world.write_message(ConsoleMoveTo {
                camera,
                transform: target,
            });
            inv.log.info(&format!("camera at {}", fmt_vec(eye)));
            Ok(())
        },
    )
    .add_console_command(
        CommandSpec::new("disable").help("Hide the selection (Visibility::Hidden)"),
        |inv| set_visibility(inv.ctx, inv.log, Visibility::Hidden),
    )
    .add_console_command(
        CommandSpec::new("enable").help("Show the selection again (Visibility::Inherited)"),
        |inv| set_visibility(inv.ctx, inv.log, Visibility::Inherited),
    )
    .add_console_command(
        CommandSpec::new("delete").help("Despawn the selection and its children"),
        |inv| {
            let e = selected(inv.ctx)?;
            let name = label(inv.ctx, e);
            inv.ctx.despawn(e);
            inv.ctx.resource_mut::<ConsoleSelection>().0 = None;
            inv.log.info(&format!("deleted {name}"));
            Ok(())
        },
    );
}

fn set_visibility(
    world: &mut World,
    log: &mut sf_console::Log,
    visibility: Visibility,
) -> Result<(), String> {
    let e = selected(world)?;
    world.entity_mut(e).insert(visibility);
    log.info(&format!("{}: {visibility:?}", label(world, e)));
    Ok(())
}
