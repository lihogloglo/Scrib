//! The Morrowind material signature (catalog module docs).

use super::*;
use crate::delight::AlphaClass;

fn material(texture: &str) -> Material {
    Material { base_color_texture: Some(texture.to_owned()), ..Material::default() }
}

fn with_extra(mut m: Material, key: &str, value: &str) -> Material {
    m.extras.insert(key.to_owned(), value.to_owned());
    m
}

fn facts() -> TextureFacts {
    TextureFacts {
        alpha: AlphaClasses::new(BTreeMap::from([
            ("textures/tx_leaf.dds".to_owned(), AlphaClass::Cutout),
            ("textures/tx_glass.dds".to_owned(), AlphaClass::Soft),
        ])),
        normals: BTreeMap::from([("textures/tx_wall".to_owned(), "textures/tx_wall_n.dds".to_owned())]),
        depths: BTreeMap::from([("textures/tx_wall".to_owned(), "textures/tx_wall_d.ktx2".to_owned())]),
    }
}

#[test]
fn per_material_data_does_not_split_a_signature() {
    let facts = facts();
    let plain = signature(&material("textures/tx_rock.dds"), &facts);
    let tinted = Material { base_color: [0.3, 0.2, 0.1, 1.0], emissive: [0.5; 3], ..material("textures/tx_sand.dds") };
    assert_eq!(signature(&tinted, &facts), plain, "texture, tint and emissive are material data, not variants");
    assert_eq!(plain.fields["alpha"], "opaque");
}

#[test]
fn variant_fields_split() {
    let facts = facts();
    let plain = signature(&material("textures/tx_rock.dds"), &facts);
    let wall = signature(&material("textures/tx_wall.dds"), &facts);
    assert_ne!(wall, plain);
    assert_eq!((wall.fields["normal_map"].as_str(), wall.fields["depth_map"].as_str()), ("true", "true"));
    let unlit_wall = signature(&Material { unlit: true, ..material("textures/tx_wall.dds") }, &facts);
    assert_eq!(unlit_wall.fields["normal_map"], "false", "unlit surfaces bind no companions");

    let blend = Material { alpha: AlphaMode::Blend, ..material("textures/tx_glass.dds") };
    let additive = signature(&with_extra(blend.clone(), extras::BLEND, "additive"), &facts);
    assert_eq!(additive.fields["alpha"], "additive");
    let glass = signature(&with_extra(blend.clone(), extras::BLEND, "cutout_candidate"), &facts);
    assert_eq!(glass.fields["alpha"], "blend", "a soft candidate stays blended");
    let leaf = Material { base_color_texture: Some("textures/tx_leaf.dds".into()), ..blend };
    let leaf = signature(&with_extra(leaf, extras::BLEND, "cutout_candidate"), &facts);
    assert_eq!((leaf.fields["alpha"].as_str(), leaf.fields["foliage"].as_str()), ("mask", "true"));
    let two_sided = signature(&Material { double_sided: true, ..material("textures/tx_rock.dds") }, &facts);
    assert_ne!(two_sided, plain);
}

#[test]
fn without_texture_facts_nothing_is_a_companion_or_cutout() {
    let none = TextureFacts::default();
    let wall = signature(&material("textures/tx_wall.dds"), &none);
    assert_eq!(wall.fields["normal_map"], "false");
    let source = MwCatalogSource::new(Vec::new());
    assert_ne!(source.fingerprint(), MwCatalogSource::new(Vec::new()).without_textures().fingerprint());
    assert_eq!(source.builtins().len(), BUILTINS.len());
}
