use std::collections::{BTreeSet, HashMap};

use semver::{Version, VersionReq};

use crate::{ModId, ModManifest};

#[derive(Debug, PartialEq, Eq, thiserror::Error)]
pub enum LoadOrderError {
    #[error("profile enables {0}, but no such mod is installed")]
    UnknownMod(ModId),
    #[error("mod {0} is installed more than once")]
    DuplicateMod(ModId),
    #[error("profile enables {0} more than once")]
    DuplicateInProfile(ModId),
    #[error("{dependent} requires {dependency}, which is not enabled")]
    MissingDependency { dependent: ModId, dependency: ModId },
    #[error("{dependent} requires {dependency} {required}, but {found} is installed")]
    VersionMismatch { dependent: ModId, dependency: ModId, required: VersionReq, found: Version },
    #[error("load order constraints form a cycle involving: {}", join(.0))]
    Cycle(Vec<ModId>),
}

fn join(ids: &[ModId]) -> String {
    ids.iter().map(ModId::as_str).collect::<Vec<_>>().join(", ")
}

/// Orders the enabled mods so that every dependency and ordering hint holds,
/// staying as close to the profile's order as those constraints allow.
///
/// `installed` is every known manifest; `profile` lists the enabled mods in
/// the user's preferred order.
pub fn resolve_load_order<'a>(
    installed: &'a [ModManifest],
    profile: &[ModId],
) -> Result<Vec<&'a ModManifest>, LoadOrderError> {
    let mut by_id: HashMap<&ModId, &ModManifest> = HashMap::new();
    for manifest in installed {
        if by_id.insert(&manifest.id, manifest).is_some() {
            return Err(LoadOrderError::DuplicateMod(manifest.id.clone()));
        }
    }

    // Position of each enabled mod in the profile; this is the tie-breaker.
    let mut rank: HashMap<&ModId, usize> = HashMap::new();
    for (i, id) in profile.iter().enumerate() {
        if !by_id.contains_key(id) {
            return Err(LoadOrderError::UnknownMod(id.clone()));
        }
        if rank.insert(id, i).is_some() {
            return Err(LoadOrderError::DuplicateInProfile(id.clone()));
        }
    }

    // Edges: before -> after, over profile indices.
    let mut successors: Vec<Vec<usize>> = vec![Vec::new(); profile.len()];
    let mut indegree = vec![0usize; profile.len()];
    let mut edge = |before: usize, after: usize| {
        successors[before].push(after);
        indegree[after] += 1;
    };

    for (i, id) in profile.iter().enumerate() {
        let manifest = by_id[id];
        for (dep, req) in &manifest.dependencies {
            let Some(&d) = rank.get(dep) else {
                return Err(LoadOrderError::MissingDependency { dependent: id.clone(), dependency: dep.clone() });
            };
            let found = &by_id[dep].version;
            if !req.matches(found) {
                return Err(LoadOrderError::VersionMismatch {
                    dependent: id.clone(),
                    dependency: dep.clone(),
                    required: req.clone(),
                    found: found.clone(),
                });
            }
            edge(d, i);
        }
        for other in &manifest.load_after {
            if let Some(&o) = rank.get(other) {
                edge(o, i);
            }
        }
        for other in &manifest.load_before {
            if let Some(&o) = rank.get(other) {
                edge(i, o);
            }
        }
    }

    // Kahn's algorithm, always taking the ready mod earliest in the profile.
    let mut ready: BTreeSet<usize> = (0..profile.len()).filter(|&i| indegree[i] == 0).collect();
    let mut order = Vec::with_capacity(profile.len());
    while let Some(i) = ready.pop_first() {
        order.push(by_id[&profile[i]]);
        for &next in &successors[i] {
            indegree[next] -= 1;
            if indegree[next] == 0 {
                ready.insert(next);
            }
        }
    }

    if order.len() < profile.len() {
        let stuck = (0..profile.len()).filter(|&i| indegree[i] > 0).map(|i| profile[i].clone()).collect();
        return Err(LoadOrderError::Cycle(stuck));
    }
    Ok(order)
}
