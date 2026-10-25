use std::sync::Arc;

use glam::{DVec3, IVec3};

use super::{models, placement, ws};
use crate::*;

fn page_of(placements: Vec<Placement>) -> IndexPage {
    let (mut pages, _) = build_pages(&IndexParams::default(), placements, &models);
    pages.remove(0)
}

#[test]
fn level_steps_split_a_cell_into_far_near_and_activation() {
    let mut editor = placement("editor", DVec3::new(5.0, 0.0, 5.0), Some("big.mesh"), PlacementClass::Marker);
    editor.flags = PlacementFlags::EDITOR_ONLY;
    let page = page_of(vec![
        placement("big", DVec3::new(10.0, 0.0, 10.0), Some("big.mesh"), PlacementClass::Static),
        placement("small", DVec3::new(20.0, 0.0, 10.0), Some("small.mesh"), PlacementClass::Static),
        placement("light", DVec3::new(30.0, 0.0, 10.0), None, PlacementClass::Light),
        editor,
    ]);
    let cell = &page.cells[0].placements;
    let content = LevelContent { far_min_radius: 4.0, proxies: false };
    let keys = |content: LevelContent, from, to| -> Vec<String> {
        content.step(cell, from, to).map(|p| p.placement.base.id.as_str().to_owned()).collect()
    };
    assert_eq!(keys(content, Level::Unloaded, Level::Far), ["big"]);
    assert_eq!(keys(content, Level::Far, Level::Near), ["small", "light"]);
    assert_eq!(keys(content, Level::Near, Level::Far), ["small", "light"]);
    assert_eq!(keys(content, Level::Unloaded, Level::Near), ["big", "small", "light"]);
    // Activation covers everything spawned, never editor-only helpers.
    assert_eq!(keys(content, Level::Near, Level::Active).len(), 3);
    assert_eq!(keys(content, Level::Active, Level::Near).len(), 3);

    // With far-field proxies the far ring holds the proxy, not placements.
    let proxies = content.with_proxies(true);
    assert!(keys(proxies, Level::Unloaded, Level::Far).is_empty());
    assert_eq!(keys(proxies, Level::Far, Level::Near), ["big", "small", "light"]);
    assert_eq!(keys(proxies, Level::Near, Level::Active).len(), 3);
}

#[test]
fn far_eligibility_by_class_and_flags() {
    let mut p = placement("p", DVec3::ZERO, Some("big.mesh"), PlacementClass::Static);
    assert!(p.far_eligible());
    for class in [PlacementClass::Door, PlacementClass::Light] {
        p.class = class;
        assert!(p.far_eligible());
    }
    for class in [PlacementClass::Actor, PlacementClass::Dynamic, PlacementClass::Marker, PlacementClass::Custom(3)] {
        p.class = class;
        assert!(!p.far_eligible(), "{class:?}");
    }
    p.class = PlacementClass::Static;
    for flag in [PlacementFlags::NO_LOD, PlacementFlags::NEAR_ONLY, PlacementFlags::EDITOR_ONLY, PlacementFlags::DISABLED] {
        p.flags = flag;
        assert!(!p.far_eligible(), "{flag:?}");
    }
    p.flags = PlacementFlags::GROUNDED;
    assert!(p.far_eligible());
    p.model = None;
    assert!(!p.far_eligible());
}

#[test]
fn page_cache_evicts_least_recently_used() {
    let a = Arc::new(page_of(vec![placement("a", DVec3::new(10.0, 0.0, 10.0), None, PlacementClass::Static)]));
    let b = Arc::new(page_of(vec![placement("b", DVec3::new(2000.0, 0.0, 10.0), None, PlacementClass::Static)]));
    let c = Arc::new(page_of(vec![placement("c", DVec3::new(4000.0, 0.0, 10.0), None, PlacementClass::Static)]));
    // Room for two pages of 100 encoded bytes (300 + 256 each).
    let mut cache = PageCache::new(1200);
    cache.insert(&ws("w"), a.clone(), 100);
    cache.insert(&ws("w"), b.clone(), 100);
    assert!(cache.get(&ws("w"), a.page).is_some()); // a is now the most recent
    cache.insert(&ws("w"), c.clone(), 100);
    assert_eq!(cache.len(), 2);
    assert!(cache.get(&ws("w"), b.page).is_none(), "b was least recently used");
    assert!(cache.get(&ws("w"), a.page).is_some());
    assert!(cache.get(&ws("w"), c.page).is_some());
    assert!(cache.get(&ws("other"), c.page).is_none());
    assert_eq!(cache.used_bytes(), 2 * (300 + 256));
}

#[test]
fn memory_index_serves_what_it_was_given() {
    let source_pages = vec![placement("a", DVec3::new(10.0, 0.0, 10.0), Some("small.mesh"), PlacementClass::Static)];
    let (pages, _) = build_pages(&IndexParams::default(), source_pages, &models);
    let directory = build_directory(ws("w"), &IndexParams::default(), &pages);
    let mut index = MemoryIndex::new();
    index.insert(&WorldIndex { directory: directory.clone(), pages: pages.clone(), stats: IndexStats::default() });
    let dir = CellDirectory::from_bytes(&index.directory(&ws("w")).unwrap().unwrap()).unwrap();
    assert_eq!(dir, directory);
    assert_eq!(IndexPage::from_bytes(&index.page(&ws("w"), pages[0].page).unwrap().unwrap()).unwrap(), pages[0]);
    assert!(index.page(&ws("w"), IVec3::splat(7)).unwrap().is_none());
}
