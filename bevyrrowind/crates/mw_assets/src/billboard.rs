//! `NiBillboardNode`: the node's subtree always faces the camera.

use bevy::prelude::*;

/// Marks a node that must face the camera (`NiBillboardNode`).
#[derive(Component, Clone, Copy, Debug, Default, Reflect)]
#[reflect(Component)]
pub struct Billboard;

/// Turns billboards to face the camera (screen-aligned, the Morrowind
/// behavior), keeping their position and scale. Runs before transform
/// propagation; parents' global transforms are last frame's, which is
/// exact for the static props billboards sit on.
pub(crate) fn face_camera(
    camera: Query<&Transform, (With<Camera3d>, Without<Billboard>)>,
    parents: Query<&GlobalTransform>,
    mut billboards: Query<(&mut Transform, &ChildOf), With<Billboard>>,
) {
    let Ok(camera) = camera.single() else { return };
    let world = billboard_rotation(camera.rotation);
    for (mut transform, child_of) in &mut billboards {
        let parent = parents.get(child_of.parent()).map_or(Quat::IDENTITY, |g| g.compute_transform().rotation);
        transform.rotation = parent.inverse() * world;
    }
}

/// World rotation of a billboard for a camera rotation.
///
/// The original engine aligns the node's *Morrowind-space* axes with view
/// space: X right, Y up, Z toward the viewer. In engine space (see
/// `mw_nif::coords`) those axes are engine X, -Z, and Y, so the camera's
/// orientation is preceded by a quarter turn about X.
pub fn billboard_rotation(camera: Quat) -> Quat {
    camera * Quat::from_rotation_x(std::f32::consts::FRAC_PI_2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn morrowind_axes_align_with_the_view() {
        let camera = Quat::from_rotation_y(0.7) * Quat::from_rotation_x(-0.3);
        let q = billboard_rotation(camera);
        // Engine images of Morrowind X, Y, Z.
        let (mw_x, mw_y, mw_z) = (Vec3::X, Vec3::NEG_Z, Vec3::Y);
        assert!((q * mw_x - camera * Vec3::X).length() < 1e-5, "X -> right");
        assert!((q * mw_y - camera * Vec3::Y).length() < 1e-5, "Y -> up");
        assert!((q * mw_z - camera * Vec3::Z).length() < 1e-5, "Z -> toward the viewer");
    }
}
