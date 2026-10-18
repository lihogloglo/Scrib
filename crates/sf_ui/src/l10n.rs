//! Fluent localisation (ui.md "Stack decision" 3: `fluent-bundle`, Bevy-free).
//!
//! One bundle per locale of the fallback chain (most preferred first). Mods
//! add Fluent resources in load order; a later mod's message replaces an
//! earlier one with the same id (`add_resource_overriding`), which is how a
//! mod retranslates or rewords another mod's strings. VFS convention:
//! `l10n/<locale>/<mod id>.ftl`, so mods never shadow each other's files.
//!
//! Lookups fall back through the chain, then to the key itself, so a missing
//! string shows its key instead of nothing.

use std::borrow::Cow;

use fluent_bundle::FluentResource;
use fluent_bundle::concurrent::FluentBundle;
pub use fluent_bundle::{FluentArgs, FluentValue};
pub use unic_langid::LanguageIdentifier;

/// One Fluent file of one mod.
#[derive(Clone, Debug)]
pub struct L10nSource {
    pub mod_id: String,
    pub locale: LanguageIdentifier,
    /// File path or other label, for reports.
    pub path: String,
    pub text: String,
}

/// A problem with a Fluent file, attributed to its mod.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct L10nIssue {
    pub mod_id: String,
    pub path: String,
    pub message: String,
}

/// The fallback chain for `requested` locales (player preference first) over
/// the `available` ones, ending with `default`. For each requested locale an
/// exact match comes first, then available locales of the same language
/// (`fr-CA` finds `fr`), in `available` order. Unparseable tags are skipped.
///
/// A small subset of `fluent-langneg`'s filtering strategy: `fluent-langneg`
/// 0.14 moved to `icu_locid` while `fluent-bundle` 0.16 still speaks
/// `unic-langid`, and the subset is all a locale setting needs.
pub fn negotiate(
    requested: &[&str],
    available: &[LanguageIdentifier],
    default: &LanguageIdentifier,
) -> Vec<LanguageIdentifier> {
    let mut chain: Vec<LanguageIdentifier> = Vec::new();
    let mut add = |l: &LanguageIdentifier| {
        if !chain.contains(l) {
            chain.push(l.clone());
        }
    };
    for want in requested
        .iter()
        .filter_map(|s| s.parse::<LanguageIdentifier>().ok())
    {
        available.iter().filter(|a| **a == want).for_each(&mut add);
        available
            .iter()
            .filter(|a| a.language == want.language)
            .for_each(&mut add);
    }
    add(default);
    chain
}

/// Formatted UI strings for a locale chain.
pub struct Localizer {
    bundles: Vec<FluentBundle<FluentResource>>,
}

impl Localizer {
    /// Builds bundles for `chain` (most preferred first) from mod sources in
    /// load order. Sources for locales outside the chain are ignored. A file
    /// with syntax errors still contributes its valid messages.
    pub fn build(
        chain: &[LanguageIdentifier],
        sources: &[L10nSource],
        use_isolating: bool,
    ) -> (Self, Vec<L10nIssue>) {
        let mut issues = Vec::new();
        let bundles = chain
            .iter()
            .map(|locale| {
                let mut bundle = FluentBundle::new_concurrent(vec![locale.clone()]);
                bundle.set_use_isolating(use_isolating);
                for source in sources.iter().filter(|s| &s.locale == locale) {
                    let resource = match FluentResource::try_new(source.text.clone()) {
                        Ok(r) => r,
                        Err((r, errors)) => {
                            issues.extend(errors.iter().map(|e| L10nIssue {
                                mod_id: source.mod_id.clone(),
                                path: source.path.clone(),
                                message: format!("{e}"),
                            }));
                            r
                        }
                    };
                    bundle.add_resource_overriding(resource);
                }
                bundle
            })
            .collect();
        (Self { bundles }, issues)
    }

    /// Locales of the chain, most preferred first.
    pub fn locales(&self) -> impl Iterator<Item = &LanguageIdentifier> {
        self.bundles.iter().filter_map(|b| b.locales.first())
    }

    /// Whether any bundle has the message (`id` or `id.attribute`).
    pub fn has(&self, key: &str) -> bool {
        let (id, attr) = split_key(key);
        self.bundles.iter().any(|b| {
            b.get_message(id)
                .is_some_and(|m| attr.map_or(m.value().is_some(), |a| m.get_attribute(a).is_some()))
        })
    }

    /// Formats `id` or `id.attribute` with the first bundle that has it.
    /// Returns the key itself when no bundle does. Formatting errors
    /// (a missing variable) still produce text, with the variable named in braces.
    pub fn format<'a>(&'a self, key: &'a str, args: Option<&FluentArgs<'_>>) -> Cow<'a, str> {
        let (id, attr) = split_key(key);
        for bundle in &self.bundles {
            let Some(message) = bundle.get_message(id) else {
                continue;
            };
            let pattern = match attr {
                Some(a) => message.get_attribute(a).map(|a| a.value()),
                None => message.value(),
            };
            if let Some(pattern) = pattern {
                let mut errors = Vec::new();
                return bundle.format_pattern(pattern, args, &mut errors);
            }
        }
        Cow::Borrowed(key)
    }
}

fn split_key(key: &str) -> (&str, Option<&str>) {
    match key.split_once('.') {
        Some((id, attr)) => (id, Some(attr)),
        None => (key, None),
    }
}
