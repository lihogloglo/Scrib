//! Bake product names and subjects (always compiled).
//!
//! The runtime reads what the bake wrote, so both halves need the step ids, the file extensions
//! and the subject grammar — and the runtime must **not** pull `sf_bake`, `sf_model`, `sf_terrain`
//! or `sf_world` (`feature = "bake"`, which also turns on `sf_terrain/bake` and `sf_world/bake`:
//! bake code in the shipped binary and a much longer cold build). So the names live here and
//! [`crate::bake`] re-exports them with its own typed wrappers.
//!
//! | Step | Subject | Extension | Product |
//! |---|---|---|---|
//! | [`CELLS_STEP`] | `<worldspace>/<x>,<y>,<z>` ([`cell_subject`]) | [`CELLS_EXT`] | one cell's [`crate::StaticCellTable`] |
//! | [`CHAINS_STEP`] | `<model>/<part>` ([`chain_subject`]) | [`CHAINS_EXT`] | one chain's geometry, an `sf_model` `.sfmesh` with one LOD level per chain level |
//!
//! A worldspace is named by a `&str` here, not an `sf_world::WorldspaceId`: that type lives in
//! `sf_world`. A chain's **part** is the material index inside its model
//! ([`crate::LodChain::part`]), so `<model>/<part>` names one (model, material) pair — exactly
//! what a chain is.

use glam::IVec3;

/// The step baking one [`crate::StaticCellTable`] per streaming cell.
pub const CELLS_STEP: &str = "scene.cells";
/// File extension of [`CELLS_STEP`] products in `baked://` asset paths.
pub const CELLS_EXT: &str = "sfscene";
/// The step baking one chain's geometry (one `.sfmesh` LOD level per chain level).
pub const CHAINS_STEP: &str = "scene.chains";
/// File extension of [`CHAINS_STEP`] products: the `sf_model` container, read by the existing
/// `.sfmesh` loader (no new asset type, no new loader).
pub const CHAINS_EXT: &str = "sfmesh";

/// Subject of a cell's table: `<worldspace>/<x>,<y>,<z>`.
pub fn cell_subject(worldspace: &str, cell: IVec3) -> String {
    format!("{worldspace}/{},{},{}", cell.x, cell.y, cell.z)
}

/// Inverse of [`cell_subject`] (the worldspace borrowed from `subject`).
pub fn parse_cell_subject(subject: &str) -> Option<(&str, IVec3)> {
    let (worldspace, tail) = subject.rsplit_once('/')?;
    let mut it = tail.split(',').map(str::parse::<i32>);
    let cell = IVec3::new(it.next()?.ok()?, it.next()?.ok()?, it.next()?.ok()?);
    it.next().is_none().then_some((worldspace, cell))
}

/// Subject of a chain's geometry: `<model>/<part>`, the model's `mesh.cook` subject and the
/// chain's material index in it.
pub fn chain_subject(model: &str, part: u32) -> String {
    format!("{model}/{part}")
}

/// Inverse of [`chain_subject`] (the model borrowed from `subject`).
///
/// The split is at the **last** `/`, so a model path's own separators are kept: a model is a
/// file path, so its last segment is a file name, never a bare number.
pub fn parse_chain_subject(subject: &str) -> Option<(&str, u32)> {
    let (model, part) = subject.rsplit_once('/')?;
    (!model.is_empty()).then_some((model, part.parse().ok()?))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cell_subjects_round_trip() {
        let s = cell_subject("ws", IVec3::new(-3, 0, 12));
        assert_eq!(s, "ws/-3,0,12");
        assert_eq!(parse_cell_subject(&s), Some(("ws", IVec3::new(-3, 0, 12))));
        // A worldspace name is not parsed, so one with a separator still round trips.
        assert_eq!(parse_cell_subject("a/b/1,2,3"), Some(("a/b", IVec3::new(1, 2, 3))));
        for bad in ["ws/1,2", "ws/1,2,3,4", "nope", "ws/1,2,x", "ws/"] {
            assert_eq!(parse_cell_subject(bad), None, "{bad}");
        }
    }

    #[test]
    fn chain_subjects_round_trip() {
        let s = chain_subject("meshes/x/wall.nif", 3);
        assert_eq!(s, "meshes/x/wall.nif/3");
        assert_eq!(parse_chain_subject(&s), Some(("meshes/x/wall.nif", 3)));
        assert_eq!(parse_chain_subject(&chain_subject("a.nif", 0)), Some(("a.nif", 0)));
        assert_eq!(parse_chain_subject(&chain_subject("a.nif", u32::MAX)), Some(("a.nif", u32::MAX)));
        for bad in ["meshes/wall.nif", "meshes/wall.nif/-1", "meshes/wall.nif/", "/0", "3"] {
            assert_eq!(parse_chain_subject(bad), None, "{bad}");
        }
    }

    #[test]
    fn steps_and_extensions_are_stable() {
        // Baked product names: changing one invalidates every cache and every `baked://` path.
        assert_eq!((CELLS_STEP, CELLS_EXT), ("scene.cells", "sfscene"));
        assert_eq!((CHAINS_STEP, CHAINS_EXT), ("scene.chains", "sfmesh"));
    }
}
