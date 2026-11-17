//! Which meshes may cast ground shelter (surface weather; shelter audit 2026-11-17).
//!
//! The shelter maps (`sf_surface_bevy::shelter`) are captures of the *static* world: a top-down
//! depth map redrawn when the camera moved or casters streamed in, never per frame. The framework
//! keeps skinned meshes out, but a body is more than its skin: heads, hair, helmets, pauldrons,
//! weapons and shields are plain rigid meshes parented to the skeleton, and each one stamped a
//! sheltered (snow-free, sunken) spot where it stood on the frame of a redraw, left behind as the
//! character walked on. The standard is the one cached shadow maps and baked occlusion follow
//! (UE's static/movable split, RDR2's static-dynamic shadow split): a capture that is not redrawn
//! per frame holds static geometry only.
//!
//! So every mesh under an animated body (an entity with an [`Animator`]: player, NPCs, creatures)
//! gets `NoSurfaceShelter`, whichever appears first, the mesh or the animator.

use bevy::prelude::*;
use sf_surface_bevy::NoSurfaceShelter;

/// A root whose meshes move with an animation: its descendants never cast ground shelter. A
/// trait only so the headless test can stand in for an `Animator`.
pub(crate) trait Animated: Component {}

impl Animated for sf_anim_bevy::Animator {}

/// A mesh that could still cast shelter.
type Untagged = (With<Mesh3d>, Without<NoSurfaceShelter>);

/// Tags the meshes of animated bodies [`NoSurfaceShelter`]: a new mesh with an animated ancestor
/// (or animated itself), and every mesh below a new animated root.
#[allow(clippy::type_complexity)]
pub(crate) fn no_shelter_for_animated<A: Animated>(
    mut commands: Commands,
    new_meshes: Query<Entity, (Added<Mesh3d>, Without<NoSurfaceShelter>)>,
    new_roots: Query<Entity, Added<A>>,
    roots: Query<(), With<A>>,
    parents: Query<&ChildOf>,
    children: Query<&Children>,
    meshes: Query<(), Untagged>,
) {
    for mesh in &new_meshes {
        if roots.contains(mesh) || parents.iter_ancestors(mesh).any(|a| roots.contains(a)) {
            commands.entity(mesh).try_insert(NoSurfaceShelter);
        }
    }
    for root in &new_roots {
        for e in std::iter::once(root).chain(children.iter_descendants(root)) {
            if meshes.contains(e) {
                commands.entity(e).try_insert(NoSurfaceShelter);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Component)]
    struct Body;
    impl Animated for Body {}

    fn app() -> App {
        let mut app = App::new();
        app.add_systems(Update, no_shelter_for_animated::<Body>);
        app
    }

    fn mesh() -> Mesh3d {
        Mesh3d(Handle::default())
    }

    #[test]
    fn rigid_parts_of_a_body_never_cast_shelter_statics_still_do() {
        let mut app = app();
        let w = app.world_mut();
        // A body whose parts exist before the frame runs: a joint, a held prop under it, and the
        // body's own mesh.
        let body = w.spawn((Body, mesh())).id();
        let joint = w.spawn(ChildOf(body)).id();
        let prop = w.spawn((mesh(), ChildOf(joint))).id();
        // A static (a roof) and a mesh under a plain parent (a cell root).
        let roof = w.spawn(mesh()).id();
        let cell = w.spawn_empty().id();
        let tree = w.spawn((mesh(), ChildOf(cell))).id();
        app.update();
        let tagged = |app: &App, e: Entity| app.world().get::<NoSurfaceShelter>(e).is_some();
        assert!(tagged(&app, body) && tagged(&app, prop), "the body and its held prop are out");
        assert!(!tagged(&app, roof) && !tagged(&app, tree), "statics still cast");

        // A helmet equipped later: a new mesh under the old body.
        let helmet = app.world_mut().spawn((mesh(), ChildOf(joint))).id();
        // A scene spawned first and animated later: the meshes come before the root's marker.
        let late = app.world_mut().spawn_empty().id();
        let late_part = app.world_mut().spawn((mesh(), ChildOf(late))).id();
        app.update();
        assert!(tagged(&app, helmet), "a part added to a live body is out");
        assert!(!tagged(&app, late_part), "not animated yet: still a caster");
        app.world_mut().entity_mut(late).insert(Body);
        app.update();
        assert!(tagged(&app, late_part), "meshes under a root that becomes animated are out");
        assert!(!tagged(&app, roof) && !tagged(&app, tree));
    }
}
