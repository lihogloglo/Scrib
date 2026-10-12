//! Which Morrowind models blend into the terrain near their base, and over
//! what band (meters above the ground): the content pack's rule behind
//! `sf_world_bevy::GroundBlendModels` (superfargoth terrain.md
//! "Object-terrain blending").
//!
//! By file name, from a census of the vanilla + expansions meshes
//! (2026-10-12): exterior rocks and cliffs (`terrain_rock_*`,
//! `terrain_rocks_*`, `terrain_ashland_rock*`, `ex_cave_coastrock`, ...),
//! tree trunks and roots (`flora_tree*`, `flora_bc_tree*`, `flora_root*`,
//! Telvanni roots), and exterior architecture that stands on the ground
//! (Hlaalu, Redoran, Imperial, Telvanni, Velothi, Dwemer, Nord, stronghold
//! sets). Pieces that hang above the ground (bridges, docks, awnings,
//! windows, doors, balconies, chimneys, poles) and small props never blend:
//! the blend keys off height above the terrain, so they'd only waste the
//! lookup.

/// Rocks and cliffs: the widest band (they sink deepest into soil).
pub const ROCK_BAND: f32 = 0.6;
/// Building bases and walls.
pub const BUILDING_BAND: f32 = 0.4;
/// Tree trunks, roots, logs.
pub const FLORA_BAND: f32 = 0.3;

/// Exterior architecture sets.
const BUILDING_SETS: [&str; 11] = [
    "ex_hlaalu_",
    "ex_common_",
    "ex_redoran_",
    "ex_velothi_",
    "ex_dwrv_",
    "ex_imp_",
    "ex_t_",
    "ex_nord_",
    "ex_stronghold",
    "ex_de_",
    "ex_ashl_",
];

/// Parts of those sets that hang above the ground or are too thin to matter.
const ABOVE_GROUND: [&str; 21] = [
    "bridge", "dock", "awning", "win", "door", "balcon", "chimney", "dormer", "pole", "sign", "banner", "skywalk",
    "trellis", "mooring", "hook", "arrowslit", "header", "pipe", "roof", "ship", "lamp",
];

/// The band in meters for a model path (any case, any separator), or
/// `None` when the model doesn't blend.
pub fn ground_blend_band(model: &str) -> Option<f32> {
    let lower = model.to_ascii_lowercase();
    let name = lower.rsplit(['/', '\\']).next().unwrap_or(&lower);
    let stem = name.strip_suffix(".nif").unwrap_or(name);
    if stem.contains("rock") && (stem.starts_with("terrain_") || stem.starts_with("ex_") || stem.starts_with("bm_ex_")) {
        return Some(ROCK_BAND);
    }
    if stem.starts_with("terrain_") && stem.contains("cliff") {
        return Some(ROCK_BAND);
    }
    let flora = ["flora_tree", "flora_bc_tree", "flora_root", "flora_bc_knee", "flora_bc_log", "flora_treestump"];
    if flora.iter().any(|p| stem.starts_with(p)) || (stem.starts_with("ex_t_") && stem.contains("root")) {
        return Some(FLORA_BAND);
    }
    if BUILDING_SETS.iter().any(|p| stem.starts_with(p)) && !ABOVE_GROUND.iter().any(|w| stem.contains(w)) {
        return Some(BUILDING_BAND);
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn census_families() {
        for (path, band) in [
            ("meshes/x/Terrain_rock_AI_01.nif", Some(ROCK_BAND)),
            ("meshes\\x\\terrain_rock_bc_12.nif", Some(ROCK_BAND)),
            ("meshes/x/terrain_rocks_ai_03.nif", Some(ROCK_BAND)),
            ("meshes/x/terrain_ashland_rock_06.nif", Some(ROCK_BAND)),
            ("meshes/x/ex_cave_coastrock01.nif", Some(ROCK_BAND)),
            ("meshes/x/ex_hlaalu_b_01.nif", Some(BUILDING_BAND)),
            ("meshes/x/ex_hlaalu_wall_03.nif", Some(BUILDING_BAND)),
            ("meshes/x/ex_hlaalu_canal_02.nif", Some(BUILDING_BAND)),
            ("meshes/x/ex_imp_towerb_01.nif", Some(BUILDING_BAND)),
            ("meshes/f/flora_tree_ai_01.nif", Some(FLORA_BAND)),
            ("meshes/f/flora_bc_tree_03.nif", Some(FLORA_BAND)),
            ("meshes/f/flora_root_wg_02.nif", Some(FLORA_BAND)),
            ("meshes/x/ex_t_root_05.nif", Some(FLORA_BAND)),
            ("meshes/x/ex_hlaalu_bridge_01.nif", None),
            ("meshes/x/ex_hlaalu_win_01.nif", None),
            ("meshes/x/ex_imp_dock_01.nif", None),
            ("meshes/x/ex_common_awning_01.nif", None),
            ("meshes/x/ex_redoran_window_01.nif", None),
            ("meshes/o/contain_rock_ebony_01.nif", None),
            ("meshes/x/in_lava_rock_01.nif", None),
            ("meshes/f/flora_bc_fern_01.nif", None),
            ("meshes/f/flora_bc_mushroom_02.nif", None),
            ("meshes/m/misc_com_bottle_01.nif", None),
        ] {
            assert_eq!(ground_blend_band(path), band, "{path}");
        }
    }
}
