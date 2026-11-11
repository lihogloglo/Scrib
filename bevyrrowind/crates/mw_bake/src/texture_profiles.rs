//! Texture pack profiles for Morrowind content (textures-adapter.md §7): the conventions packs
//! follow, as data in `data/texture_profiles.ron`, read into `sf_texture::PackProfile`s. The
//! framework converts by profile and names no pack; which pack follows which profile is this
//! file's data (and later a mod manifest's `texture.profile`). A pack that names none gets
//! [`DEFAULT_PROFILE`], `OpenMW`'s vanilla conventions.

use std::sync::LazyLock;

use sf_texture::PackProfile;

/// The profile of a pack that names none: `OpenMW`'s vanilla conventions.
pub const DEFAULT_PROFILE: &str = "openmw";

static PROFILES: LazyLock<Vec<PackProfile>> = LazyLock::new(|| {
    ron::from_str(include_str!("../data/texture_profiles.ron"))
        .unwrap_or_else(|error| panic!("data/texture_profiles.ron: {error}"))
});

/// Every known profile, in file order.
///
/// # Panics
/// If the embedded `data/texture_profiles.ron` doesn't parse (pinned by a test).
pub fn profiles() -> &'static [PackProfile] {
    &PROFILES
}

/// The profile called `name`.
pub fn profile(name: &str) -> Option<&'static PackProfile> {
    profiles().iter().find(|profile| profile.name == name)
}

/// [`DEFAULT_PROFILE`].
///
/// # Panics
/// If the embedded file lacks it (pinned by a test).
pub fn default_profile() -> &'static PackProfile {
    profile(DEFAULT_PROFILE).expect("data/texture_profiles.ron has the default profile")
}

#[cfg(test)]
mod tests {
    use sf_texture::Channel::{B, G, R};
    use sf_texture::{ChannelMap, PackRole, SourceY, Src};

    use super::*;

    #[test]
    fn the_shipped_profiles_parse_with_the_census_conventions() {
        let names: Vec<&str> = profiles().iter().map(|p| p.name.as_str()).collect();
        assert_eq!(names, ["openmw", "wareya-pbr"]);
        let vanilla = default_profile();
        assert_eq!(vanilla.name, "openmw");
        for profile in profiles() {
            assert_eq!(profile.normal_y, SourceY::Down, "{}", profile.name);
            assert!((profile.height_neutral - 0.5).abs() < f32::EPSILON);
            assert!(profile.quirks.is_empty());
            // `_nh` is looked up before `_n`.
            let order: Vec<&str> = profile.suffixes.iter().map(|r| r.suffix.as_str()).collect();
            let (nh, n) = (
                order.iter().position(|s| *s == "_nh"),
                order.iter().position(|s| *s == "_n"),
            );
            assert!(nh < n, "{}", profile.name);
        }
        // OpenMW's vanilla `_spec` is RGB colour + shininess, not ORM: not routed.
        assert!(
            vanilla
                .rule_for("tx_wall_spec")
                .is_some_and(|(base, _)| base == "tx_wall_spec")
        );
        let pbr = profile("wareya-pbr").unwrap();
        let (base, spec) = pbr.rule_for("tx_wall_spec").unwrap();
        assert_eq!((base, spec.role), ("tx_wall", PackRole::Orm));
        // Metal R, roughness G, AO B → glTF ORM (R AO, G roughness, B metal): R/B swapped.
        assert_eq!(
            spec.layout,
            ChannelMap([
                Src::Channel(B),
                Src::Channel(G),
                Src::Channel(R),
                Src::Unused
            ])
        );
        let (base, terrain) = pbr.rule_for("tx_ai_grass_diffusespec").unwrap();
        assert_eq!(
            (base, terrain.role),
            ("tx_ai_grass", PackRole::TerrainAlbedoRoughness)
        );
        assert_eq!(
            pbr.rule_for("tx_wall_nh").unwrap().1.role,
            PackRole::NormalHeight
        );
    }
}
