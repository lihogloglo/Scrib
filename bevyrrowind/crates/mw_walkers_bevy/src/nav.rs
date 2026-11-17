//! The game's navmesh (nav.md `NV4c`, ai-adapter.md "Navigation"): `sf_nav_bevy`'s [`NavPlugin`]
//! with a Recast backend over Morrowind's two agent classes, and its baked ring
//! ([`NavRingPlugin`]) fed from this session's bake store, anchored on the player (the
//! `Gameplay` streaming anchor). Region and classes are `mw_bake::nav`'s, so the ring reads the
//! subjects the bake wrote. Only the humanoid class is kept resident: no actor paths on the large
//! mesh yet, and NV5 makes residency per class on demand.
//!
//! Scaffold `NavRingScaffold` (see `sf_nav_bevy::residency`): removal path NV5, per-anchor,
//! per-portal, budgeted residency on the streaming scheduler.
//!
//! Debug: `nav.draw tiles` draws the resident tiles in the floating origin's frame
//! ([`NavDrawFrame`], from [`WorldOrigin`]); `nav.stats` prints the counts.

use std::collections::BTreeMap;
use std::sync::Arc;

use bevy_app::{App, PostUpdate, Update};
use bevy_ecs::prelude::*;
use bevy_log::{info, warn};
use bevy_math::Vec3;
use mw_bake::nav::{HUMANOID, REGION, REGION_ORIGIN, classes};
use sf_bake::{Manifest, ProductKey, Store};
use sf_coords::WorldPos;
use sf_nav::{ClassTiling, RegionFrame, RingLayout};
use sf_nav_bevy::{NavAnchor, NavDrawFrame, NavMeshes, NavPlugin, NavRingPlugin, NavSystems};
use sf_nav_recast::{RecastBackend, RecastConfig};
use sf_world::AnchorRole;
use sf_world_bevy::{StreamingAnchor, WorldOrigin, WorldOriginSystems, WorldPosOf};

/// Whether the manifest holds any `nav.tiles` product.
fn has_tiles(manifest: &Manifest) -> bool {
    manifest
        .products
        .range(ProductKey::new(sf_nav_bake::TILES, "")..)
        .next()
        .is_some_and(|(k, _)| k.step == sf_nav_bake::TILES)
}

/// Adds the navmesh and its baked ring. Without baked tiles (a bake run with
/// `BEVYRROWIND_BAKE_SKIP_NAV`) it logs one line and adds nothing.
pub fn add(app: &mut App, store: Store, manifest: Arc<Manifest>) {
    if !has_tiles(&manifest) {
        info!("nav: the bake has no navmesh tiles (BEVYRROWIND_BAKE_SKIP_NAV?): no navmesh this run");
        return;
    }
    let classes = classes();
    let backend = match RecastBackend::new(RecastConfig::new(classes.clone())) {
        Ok(b) => b,
        Err(e) => {
            warn!("nav: no navmesh backend: {e}");
            return;
        }
    };
    let layout = RingLayout {
        classes: classes.iter().filter(|c| c.class.id == HUMANOID).map(ClassTiling::of).collect(),
        regions: BTreeMap::new(),
        default_regions: vec![RegionFrame { region: REGION, origin: REGION_ORIGIN }],
    };
    app.add_plugins((NavPlugin, NavRingPlugin { store, manifest, layout }))
        .insert_resource(NavMeshes(Box::new(backend)))
        .add_systems(Update, follow_player.before(NavSystems::Residency))
        .add_systems(PostUpdate, draw_frame.after(WorldOriginSystems).before(NavSystems::Draw));
}

/// The ring's anchor: the first `Gameplay` streaming anchor (the player).
fn follow_player(anchors: Query<(Entity, &StreamingAnchor)>, pos_of: WorldPosOf, mut anchor: ResMut<NavAnchor>) {
    let at = anchors.iter().find(|(_, a)| a.role == AnchorRole::Gameplay).and_then(|(e, _)| pos_of.location(e));
    if at.is_some() {
        anchor.set_if_neq(NavAnchor(at));
    }
}

/// Debug draw in the floating origin's frame, active worldspace only.
fn draw_frame(origin: Option<Res<WorldOrigin>>, mut frame: ResMut<NavDrawFrame>) {
    let Some(o) = origin else {
        return;
    };
    frame.set_if_neq(NavDrawFrame { worldspace: o.worldspace.clone(), origin: WorldPos::new(o.cell, Vec3::ZERO) });
}

#[cfg(test)]
mod tests {
    use sf_bake::Digest;

    use super::*;

    #[test]
    fn tiles_are_found_among_other_steps() {
        let mut m = Manifest::default();
        assert!(!has_tiles(&m));
        let d = Digest::of(b"x");
        m.products.insert(ProductKey::new("nav.links", "ws"), d);
        m.products.insert(ProductKey::new("terrain.tile", "a"), d);
        assert!(!has_tiles(&m));
        m.products.insert(ProductKey::new(sf_nav_bake::TILES, "0/0/1/2/ws"), d);
        assert!(has_tiles(&m));
    }
}
