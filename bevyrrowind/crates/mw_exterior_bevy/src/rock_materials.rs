//! Rock materials from adapter data (rockgen.md "(f) Placement in the game" step 7, slice RG-P4): the texture of
//! each rock class (`bevyrrowind/worldgen/rock_textures.toml`, [`RockTextureTable`]) built into one
//! [`RockMaterial`] per class and LOD ([`material_set`], the [`RockMaterialSet`] `sf_rockgen_bevy::RockSpawner`
//! draws library rocks with). One builder for the game and the rock lab (`level/rock_lab.rs`): a texture goes
//! through the statics' material path ([`rock_textures`]: `cooked_material_with` over the bake's companions) at the
//! terrain's land-texture tiling ([`terrain_tiling_m`]), and a class's tint darkens it ([`lod_materials`]).

use std::collections::{BTreeMap, HashMap};

use bevy::prelude::*;
use mw_assets::cooked::{TextureCompanions, cooked_material_with};
use serde::Deserialize;
use sf_rockgen_bevy::{CLASSES, RockMaterial, RockMaterialSet, RockMaterialSettings, RockTextures, rock_material};
use sf_vfs::Vfs;

/// The shipped table.
pub const SHIPPED_ROCK_TEXTURES: &str = include_str!("../../../worldgen/rock_textures.toml");

/// Which texture each rock class is drawn with (VFS paths, normalized as `--rock-textures`).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RockTextureTable {
    /// Every class without its own row.
    pub default: String,
    /// Built-in class name -> texture.
    pub class: BTreeMap<String, String>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct RawTable {
    default: String,
    #[serde(default)]
    class: BTreeMap<String, String>,
}

impl RockTextureTable {
    /// Parses and checks a table: unknown keys, a class that is not built in (`sf_rockgen::CLASSES`) or an empty
    /// texture are errors.
    pub fn parse(text: &str) -> Result<Self, String> {
        let raw: RawTable = toml::from_str(text).map_err(|e| format!("rock_textures: {e}"))?;
        let path = |what: &str, t: &str| crate::level::texture_lab::texture_path(t).ok_or_else(|| format!("rock_textures: {what} names no texture"));
        let default = path("default", &raw.default)?;
        let mut class = BTreeMap::new();
        for (name, texture) in &raw.class {
            if !CLASSES.iter().any(|c| c.name == name) {
                return Err(format!("rock_textures: [class] {name} is not a rock class"));
            }
            class.insert(name.clone(), path(&format!("[class] {name}"), texture)?);
        }
        Ok(Self { default, class })
    }

    /// The shipped table ([`SHIPPED_ROCK_TEXTURES`]).
    ///
    /// # Panics
    /// If the shipped file does not parse (a test checks it does).
    #[must_use]
    pub fn shipped() -> Self {
        Self::parse(SHIPPED_ROCK_TEXTURES).expect("the shipped rock_textures.toml parses")
    }

    /// The texture of `class`: its row, else `default`.
    #[must_use]
    pub fn texture_for(&self, class: &str) -> &str {
        self.class.get(class).unwrap_or(&self.default)
    }
}

/// Meters per texture repeat: the terrain's land-texture repeat, so rocks and ground share texel density.
#[must_use]
pub fn terrain_tiling_m() -> f32 {
    #[allow(clippy::cast_possible_truncation)]
    let m = (f64::from(mw_world::terrain::TEXEL_UNITS) * mw_world::convert::UNIT_METERS_F64) as f32;
    m
}

/// One texture's maps and factors, as the statics' material path builds them, at the terrain tiling.
#[must_use]
pub fn rock_textures(texture: &str, vfs: &Vfs, companions: &TextureCompanions, server: &AssetServer) -> RockTextures {
    let standard = cooked_material_with(&crate::level::texture_lab::swatch_source(texture), vfs, companions, server);
    RockTextures::from_standard(&standard, terrain_tiling_m())
}

/// One material per LOD level of `textures` darkened by a class `tint` (1 = as is).
pub fn lod_materials(
    textures: &RockTextures,
    tint: f32,
    settings: &RockMaterialSettings,
    lods: u32,
    lod_tint: bool,
    materials: &mut Assets<RockMaterial>,
) -> Vec<Handle<RockMaterial>> {
    let mut tinted = textures.clone();
    let c = tinted.base_color.to_linear();
    tinted.base_color = LinearRgba::new(c.red * tint, c.green * tint, c.blue * tint, c.alpha).into();
    (0..lods.max(1)).map(|k| materials.add(rock_material(&tinted, settings, k, lod_tint))).collect()
}

/// The library rocks' materials: per built-in class its table texture with its tint, `lods` levels each. Classes
/// that resolve to the same texture and tint share handles (one draw batch); `default` is the default texture
/// untinted, for a library class that is not built in.
#[allow(clippy::too_many_arguments)] // the builder's inputs: table, the material path's three, the material's three
pub fn material_set(
    table: &RockTextureTable,
    vfs: &Vfs,
    companions: &TextureCompanions,
    server: &AssetServer,
    settings: &RockMaterialSettings,
    lods: u32,
    lod_tint: bool,
    materials: &mut Assets<RockMaterial>,
) -> RockMaterialSet {
    let mut textures: HashMap<String, RockTextures> = HashMap::new();
    let mut built: HashMap<(String, u32), Vec<Handle<RockMaterial>>> = HashMap::new();
    let mut get = |texture: &str, tint: f32, materials: &mut Assets<RockMaterial>| {
        built
            .entry((texture.to_owned(), tint.to_bits()))
            .or_insert_with(|| {
                let t = textures.entry(texture.to_owned()).or_insert_with(|| rock_textures(texture, vfs, companions, server));
                lod_materials(t, tint, settings, lods, lod_tint, materials)
            })
            .clone()
    };
    let default = get(&table.default, 1.0, materials);
    let by_class = CLASSES.iter().map(|c| (c.name.to_owned(), get(table.texture_for(c.name), c.tint, materials))).collect();
    RockMaterialSet { default, by_class }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shipped_table_parses_and_names_built_in_classes() {
        let table = RockTextureTable::shipped();
        assert_eq!(table.default, "textures/tx_rock_wg_01.dds");
        for name in table.class.keys() {
            assert!(CLASSES.iter().any(|c| c.name == name), "{name}");
        }
        for c in CLASSES {
            assert_eq!(table.texture_for(c.name), table.class.get(c.name).unwrap_or(&table.default));
        }
    }

    #[test]
    fn a_class_without_a_row_takes_default() {
        let table = RockTextureTable::parse("default = \"tx_rock_wg_01\"\n[class]\nscoria = \"Textures\\\\TX_Ashland_Rock_03.dds\"\n").unwrap();
        assert_eq!(table.default, "textures/tx_rock_wg_01.dds");
        assert_eq!(table.texture_for("scoria"), "textures/tx_ashland_rock_03.dds");
        assert_eq!(table.texture_for("block"), "textures/tx_rock_wg_01.dds");
        assert_eq!(table.texture_for("not-a-class"), "textures/tx_rock_wg_01.dds");
        let bare = RockTextureTable::parse("default = \"textures/tx_a.dds\"\n").unwrap();
        assert!(bare.class.is_empty());
    }

    #[test]
    fn unknown_keys_classes_and_empty_textures_are_rejected() {
        let bad = [
            "default = \"a\"\ntint = 1.0\n",
            "default = \"a\"\n[class]\nbasalt = \"b\"\n",
            "default = \"a\"\n[klass]\nblock = \"b\"\n",
            "default = \" \"\n",
            "default = \"a\"\n[class]\nblock = \"\"\n",
            "[class]\nblock = \"b\"\n",
        ];
        for text in bad {
            assert!(RockTextureTable::parse(text).is_err(), "{text}");
        }
    }
}
