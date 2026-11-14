//! Texture pack profiles for Morrowind content (textures-adapter.md §7, §8): the conventions
//! packs follow, as data in `data/texture_profiles.ron`, read into `sf_texture::PackProfile`s. The
//! framework converts by profile and names no pack; which source follows which profile is data:
//! a mod's `mod.toml` `[texture]` section (`profile`, `normal_y`, renamed `suffixes`), or for the
//! synthesized mods (loose Data Files, the archives) the user's `textures.toml` (`mw_mods`).
//! [`TextureSources`] resolves them per VFS source; a source that names none gets
//! [`DEFAULT_PROFILE`], `OpenMW`'s vanilla conventions.

use std::collections::BTreeMap;
use std::ops::Range;
use std::sync::LazyLock;

use sf_mods::{LoadedMod, NormalY, TextureSection};
use sf_texture::{PackProfile, PackRole, SourceY, SuffixRule};
use sf_vfs::SourceIndex;

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

/// Whether `rule` is a normal-map companion.
pub fn is_normal(rule: &SuffixRule) -> bool {
    matches!(rule.role, PackRole::Normal | PackRole::NormalHeight)
}

/// Whether `rule` fills the ORM slot: a packed ORM map, or a legacy specular map converted to one.
pub fn is_orm(rule: &SuffixRule) -> bool {
    matches!(rule.role, PackRole::Orm | PackRole::SpecularShininess)
}

/// Whether `rule` is a terrain layer's albedo + roughness file (`OpenMW`'s `_diffusespec`).
pub fn is_terrain_roughness(rule: &SuffixRule) -> bool {
    rule.role == PackRole::TerrainAlbedoRoughness
}

/// The pack profile of every VFS source (textures-adapter.md §8), resolved once per bake from
/// the loaded mods' `[texture]` sections: a mod's layers all read with its section's profile; a
/// source no section names reads with [`default_profile`].
#[derive(Clone, Debug, PartialEq)]
pub struct TextureSources {
    /// The resolved profiles, `[0]` the default one as shipped; equal resolutions share one.
    profiles: Vec<PackProfile>,
    /// VFS source → its index in `profiles`; absent: `[0]`.
    by_source: BTreeMap<SourceIndex, usize>,
    /// What didn't resolve (an unknown profile name, a renamed suffix with no target), for the
    /// bake log. The source falls back to the part that did.
    pub warnings: Vec<String>,
}

impl Default for TextureSources {
    /// Every source on the default profile.
    fn default() -> Self {
        Self { profiles: vec![default_profile().clone()], by_source: BTreeMap::new(), warnings: Vec::new() }
    }
}

impl TextureSources {
    /// The profiles of a load's mods (`LoadedProfile::mods`).
    pub fn from_mods(mods: &[LoadedMod]) -> Self {
        Self::from_sections(mods.iter().map(|m| (m.layers.clone(), m.manifest.id.as_str(), &m.manifest.texture)))
    }

    /// The profiles of `(VFS layers, mod id, its [texture] section)` triples.
    pub fn from_sections<'a>(sections: impl IntoIterator<Item = (Range<u16>, &'a str, &'a TextureSection)>) -> Self {
        let mut sources = Self::default();
        for (layers, id, section) in sections {
            if section.is_empty() {
                continue;
            }
            let profile = resolve(id, section, &mut sources.warnings);
            let at = sources.profiles.iter().position(|p| same_conventions(p, &profile)).unwrap_or_else(|| {
                sources.profiles.push(profile);
                sources.profiles.len() - 1
            });
            if at != 0 {
                sources.by_source.extend(layers.map(|s| (SourceIndex(s), at)));
            }
        }
        sources
    }

    /// Whether every source reads with the default profile.
    pub fn is_default(&self) -> bool {
        self.by_source.is_empty()
    }

    /// The sources that read with another profile than the default, in source order.
    pub fn assigned(&self) -> impl Iterator<Item = (SourceIndex, &PackProfile)> {
        self.by_source.iter().map(|(&source, &at)| (source, &self.profiles[at]))
    }

    /// The profile `source`'s files read with.
    pub fn profile_of(&self, source: SourceIndex) -> &PackProfile {
        self.by_source.get(&source).map_or(&self.profiles[0], |&at| &self.profiles[at])
    }

    /// The rule a texture stem of `source` falls under, and its base stem
    /// ([`PackProfile::rule_for`] of [`Self::profile_of`]).
    pub fn rule_for<'s>(&self, stem: &'s str, source: SourceIndex) -> Option<(&'s str, &SuffixRule)> {
        self.profile_of(source).rule_for(stem)
    }

    /// The normal-map rule `suffix` names in `source`'s profile (ignoring ASCII case), if any.
    pub fn normal_rule(&self, source: SourceIndex, suffix: &str) -> Option<(&PackProfile, &SuffixRule)> {
        self.rule_where(source, suffix, is_normal)
    }

    /// Every normal-map suffix some source's profile has, in lookup order: the default profile's
    /// first (`_nh`, `_n`, `_nm`: `OpenMW`'s global order), then the new ones of the others in
    /// the order they resolved; deduplicated ignoring ASCII case.
    pub fn normal_suffixes(&self) -> Vec<&str> {
        self.suffixes_where(is_normal)
    }

    /// The ORM-slot rule (a packed ORM or a legacy specular map, [`is_orm`]) `suffix` names in
    /// `source`'s profile, if any.
    pub fn orm_rule(&self, source: SourceIndex, suffix: &str) -> Option<(&PackProfile, &SuffixRule)> {
        self.rule_where(source, suffix, is_orm)
    }

    /// Every ORM-slot suffix some source's profile has, in [`Self::normal_suffixes`]'s order.
    pub fn orm_suffixes(&self) -> Vec<&str> {
        self.suffixes_where(is_orm)
    }

    /// The terrain albedo + roughness rule ([`is_terrain_roughness`]) `suffix` names in
    /// `source`'s profile, if any.
    pub fn terrain_roughness_rule(&self, source: SourceIndex, suffix: &str) -> Option<(&PackProfile, &SuffixRule)> {
        self.rule_where(source, suffix, is_terrain_roughness)
    }

    /// Every terrain albedo + roughness suffix some source's profile has, in
    /// [`Self::normal_suffixes`]'s order.
    pub fn terrain_roughness_suffixes(&self) -> Vec<&str> {
        self.suffixes_where(is_terrain_roughness)
    }

    fn rule_where(&self, source: SourceIndex, suffix: &str, keep: fn(&SuffixRule) -> bool) -> Option<(&PackProfile, &SuffixRule)> {
        let profile = self.profile_of(source);
        profile.suffixes.iter().find(|r| keep(r) && r.suffix.eq_ignore_ascii_case(suffix)).map(|r| (profile, r))
    }

    fn suffixes_where(&self, keep: fn(&SuffixRule) -> bool) -> Vec<&str> {
        let mut out: Vec<&str> = Vec::new();
        for rule in self.profiles.iter().flat_map(|p| &p.suffixes).filter(|r| keep(r)) {
            if !out.iter().any(|s| s.eq_ignore_ascii_case(&rule.suffix)) {
                out.push(&rule.suffix);
            }
        }
        out
    }

    /// Every companion suffix (any non-base rule) of `source`'s profile.
    pub fn companion_suffixes(&self, source: SourceIndex) -> impl Iterator<Item = &str> {
        self.profile_of(source).suffixes.iter().map(|r| r.suffix.as_str()).filter(|s| !s.is_empty())
    }
}

/// Whether two profiles read files alike (their names aside).
fn same_conventions(a: &PackProfile, b: &PackProfile) -> bool {
    a.suffixes == b.suffixes && a.normal_y == b.normal_y && a.height_neutral.to_bits() == b.height_neutral.to_bits() && a.quirks == b.quirks
}

/// One mod's profile: the named one (else the default) with its overrides applied.
fn resolve(id: &str, section: &TextureSection, warnings: &mut Vec<String>) -> PackProfile {
    let base = match section.profile.as_deref() {
        None => default_profile(),
        Some(name) => profile(name).unwrap_or_else(|| {
            let known: Vec<&str> = profiles().iter().map(|p| p.name.as_str()).collect();
            warnings.push(format!("mod {id}: unknown texture profile {name:?} (known: {}); using {DEFAULT_PROFILE}", known.join(", ")));
            default_profile()
        }),
    };
    let mut resolved = base.clone();
    if let Some(y) = section.normal_y {
        resolved.normal_y = match y {
            NormalY::Up => SourceY::Up,
            NormalY::Down => SourceY::Down,
        };
    }
    for (suffix, target) in &section.suffixes {
        if !resolved.alias_suffix(suffix, target) {
            warnings.push(format!(
                "mod {id}: texture suffix {suffix:?} = {target:?} ignored (profile {} has no companion {target:?}, or {suffix:?} is taken)",
                base.name
            ));
        }
    }
    if resolved != *base {
        resolved.name = format!("{} ({id})", base.name);
    }
    resolved
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
        // OpenMW's vanilla `_spec` is RGB colour + shininess, not ORM: converted, not remapped.
        let (base, spec) = vanilla.rule_for("tx_wall_spec").unwrap();
        assert_eq!((base, spec.role, spec.layout), ("tx_wall", PackRole::SpecularShininess, ChannelMap::IDENTITY));
        assert_eq!(vanilla.fill_empty_level0(PackRole::Orm), None);
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

    fn named(profile: &str) -> TextureSection {
        TextureSection { profile: Some(profile.into()), ..TextureSection::default() }
    }

    /// textures-adapter.md §8: a mod naming `wareya-pbr` reads its `_spec` as ORM; the base game's
    /// sources stay on `openmw`, where `_spec` is a legacy specular map (converted, not ORM).
    #[test]
    fn a_mod_naming_a_profile_reads_only_its_own_files_with_it() {
        let pbr = named("wareya-pbr");
        let sources = TextureSources::from_sections([(2..4, "pbr-pack", &pbr)]);
        assert!(sources.warnings.is_empty(), "{:?}", sources.warnings);
        for layer in [2, 3] {
            let (base, rule) = sources.rule_for("textures/tx_wall_spec", SourceIndex(layer)).unwrap();
            assert_eq!((base, rule.role), ("textures/tx_wall", PackRole::Orm));
            assert_eq!(sources.profile_of(SourceIndex(layer)).name, "wareya-pbr");
        }
        for layer in [0, 1, 4] {
            let (base, rule) = sources.rule_for("textures/tx_wall_spec", SourceIndex(layer)).unwrap();
            assert_eq!((base, rule.role), ("textures/tx_wall", PackRole::SpecularShininess));
            assert_eq!(sources.profile_of(SourceIndex(layer)).name, DEFAULT_PROFILE);
        }
        let assigned: Vec<u16> = sources.assigned().map(|(s, _)| s.0).collect();
        assert_eq!(assigned, [2, 3]);
        // Its companions count for de-light's "same source" rule there, not elsewhere.
        // (`_spec` is a companion in both profiles since TS-2: OpenMW's specular map.)
        assert!(sources.companion_suffixes(SourceIndex(2)).any(|s| s == "_diffusespec"));
        assert!(!sources.companion_suffixes(SourceIndex(0)).any(|s| s == "_diffusespec"));
        assert!(sources.companion_suffixes(SourceIndex(0)).any(|s| s == "_spec"));
    }

    #[test]
    fn sections_that_change_nothing_assign_nothing_and_bad_ones_warn() {
        let empty = TextureSection::default();
        let openmw = named(DEFAULT_PROFILE);
        let sources = TextureSources::from_sections([(0..2, "a", &openmw), (2..3, "b", &empty)]);
        assert!(sources.is_default() && sources.warnings.is_empty());
        assert_eq!(sources, TextureSources::default());

        let unknown = named("no-such-profile");
        let bad_alias = TextureSection { suffixes: BTreeMap::from([("_x".into(), "_missing".into())]), ..TextureSection::default() };
        let sources = TextureSources::from_sections([(0..1, "typo", &unknown), (1..2, "alias", &bad_alias)]);
        assert!(sources.is_default(), "both fall back to the default profile unchanged");
        assert_eq!(sources.warnings.len(), 2);
        assert!(sources.warnings[0].contains("no-such-profile") && sources.warnings[0].contains("wareya-pbr"));
        assert!(sources.warnings[1].contains("\"_x\""));
    }

    /// Overrides apply to that mod only; equal resolutions share one profile; the normal suffixes
    /// are the default profile's then the new ones.
    #[test]
    fn overrides_resolve_per_mod() {
        let up = TextureSection { normal_y: Some(NormalY::Up), ..TextureSection::default() };
        let renamed = TextureSection { suffixes: BTreeMap::from([("_normal".into(), "_n".into())]), ..named("wareya-pbr") };
        let sources = TextureSources::from_sections([(1..2, "gl-a", &up), (2..3, "gl-b", &up), (3..4, "renamed", &renamed)]);
        assert_eq!(sources.profile_of(SourceIndex(0)).normal_y, SourceY::Down);
        assert_eq!(sources.profile_of(SourceIndex(1)).normal_y, SourceY::Up);
        assert_eq!(sources.profile_of(SourceIndex(1)).name, "openmw (gl-a)");
        assert_eq!(sources.profiles.len(), 3, "gl-a and gl-b share one");
        assert_eq!(sources.normal_suffixes(), ["_nh", "_n", "_nm", "_normal"]);
        assert!(sources.normal_rule(SourceIndex(3), "_NORMAL").is_some());
        assert!(sources.normal_rule(SourceIndex(0), "_normal").is_none());
        assert!(sources.normal_rule(SourceIndex(3), "_nm").is_none(), "wareya-pbr has no `_nm`");
    }
}
