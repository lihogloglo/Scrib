//! The content's exterior references at their places in a scaled world (`view world:<guide dir>` with
//! `anchors.json`, spec `worldgen-adapter.md` "Same map at 10× (WG-X)", step 6, slice WG-X4).
//!
//! Each reference is matched by its key (`Placement.id`, 32 hex digits) against the anchors' member keys
//! (`sf_worldgen::anchors`) and moved through its anchor, `x, y: S·c + (p − c)`:
//!
//! - a member of a **stamped** anchor: height + L (the stamp is the source terrain + L, so it sits as authored);
//! - a member of a **point** anchor (no stamp): the final ground at its new place (`ground`, the runtime CPU twin)
//!   plus its own offset above the source ground, so a lamp or signpost stands on the scaled land; over the sea
//!   (source ground below sea level, or no source land) it keeps its height + L, so buoys and wrecks keep the sea;
//! - any other reference whose source position lies inside a **stamped** anchor's footprint (flora, rocks, loose
//!   items, creatures in a town): moved with that anchor like a member, so towns keep their own plants at 1:1;
//! - everything else is dropped (nature outside footprints comes back by scatter, WG-X5).
//!
//! Rotation, scale, class and flags are kept. Frames: source and guide metres x east, y north; engine
//! `(X, Y, Z) = (x, height, −y)` (`sf_worldgen::tiles` "Frames").

use std::collections::{BTreeMap, HashMap};

use glam::DVec3;
use mw_worldgen::classify::Class;
use sf_world::Placement;
use sf_worldgen::anchors::AnchorTransform;

/// Bucket edge of the footprint lookup grid, metres.
// tunables: const (a lookup structure's bucket size: no look, feel or cost beyond the load-time search)
const BUCKET_M: f64 = 128.0;

/// Sea level of the source, metres (Morrowind's exterior water plane).
// tunables: const (format fact: Morrowind's exterior water plane is at z = 0)
const SEA_LEVEL_M: f64 = 0.0;

/// What the remap did.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub(crate) struct Census {
    /// References seen.
    pub seen: usize,
    /// Members of stamped anchors kept.
    pub members_stamped: usize,
    /// Members of point anchors kept (on the final ground).
    pub members_point: usize,
    /// ... of which kept their height + L (over the sea or without source land).
    pub members_point_sea: usize,
    /// Non-members inside a stamped footprint kept, by class.
    pub inside: BTreeMap<Class, usize>,
    /// Dropped, by class.
    pub dropped: BTreeMap<Class, usize>,
    /// Member keys that matched no reference (or don't parse).
    pub unmatched_keys: usize,
}

impl Census {
    /// One log line.
    pub fn line(&self) -> String {
        let by = |m: &BTreeMap<Class, usize>| {
            let parts: Vec<String> = m.iter().map(|(c, n)| format!("{c} {n}")).collect();
            if parts.is_empty() { "none".to_owned() } else { parts.join(", ") }
        };
        let inside: usize = self.inside.values().sum();
        let dropped: usize = self.dropped.values().sum();
        format!(
            "{} references: kept {} members of stamped anchors, {} of point anchors ({} at height + L: sea or no \
             source land), {inside} inside stamped footprints ({}); dropped {dropped} ({}); {} member keys unmatched",
            self.seen,
            self.members_stamped,
            self.members_point,
            self.members_point_sea,
            by(&self.inside),
            by(&self.dropped),
            self.unmatched_keys
        )
    }
}

/// Stamped anchors by the footprint buckets their bounds cover.
struct StampedGrid(HashMap<(i64, i64), Vec<usize>>);

impl StampedGrid {
    #[allow(clippy::cast_possible_truncation)] // bucket indices of metre coordinates stay far inside i64
    fn key(v: f64) -> i64 {
        (v / BUCKET_M).floor() as i64
    }

    fn new(t: &AnchorTransform) -> Self {
        let mut grid: HashMap<(i64, i64), Vec<usize>> = HashMap::new();
        for (i, a) in t.anchors.iter().enumerate() {
            let Some(b) = a.footprint.bounds().filter(|_| a.stamp.is_some()) else { continue };
            for bx in Self::key(b[0])..=Self::key(b[2]) {
                for by in Self::key(b[1])..=Self::key(b[3]) {
                    grid.entry((bx, by)).or_default().push(i);
                }
            }
        }
        Self(grid)
    }

    /// The first stamped anchor (by index) whose footprint holds `p`.
    fn find(&self, t: &AnchorTransform, p: [f64; 2]) -> Option<usize> {
        let ids = self.0.get(&(Self::key(p[0]), Self::key(p[1])))?;
        ids.iter().copied().find(|&i| t.anchors[i].footprint.contains(p))
    }
}

/// Moves `refs` (each with its class) through `anchors`; returns the kept placements and the census.
///
/// `ground(x, y)`: the final scaled world's height at guide point (x, y) (metres). `source_ground(x, y)`: the source
/// terrain's height at source point (x, y), `None` without land.
pub(crate) fn remap<'a>(
    refs: impl IntoIterator<Item = (&'a Placement, Class)>,
    anchors: &AnchorTransform,
    ground: &dyn Fn(f64, f64) -> f64,
    source_ground: &mut dyn FnMut(f64, f64) -> Option<f64>,
) -> (Vec<Placement>, Census) {
    let mut census = Census::default();
    let mut member_of: HashMap<u128, usize> = HashMap::new();
    for (i, a) in anchors.anchors.iter().enumerate() {
        for key in &a.members {
            match u128::from_str_radix(key, 16) {
                Ok(id) => {
                    member_of.insert(id, i);
                }
                Err(_) => census.unmatched_keys += 1,
            }
        }
    }
    let stamped = StampedGrid::new(anchors);
    let mut matched = 0usize;
    let mut out = Vec::new();
    for (p, class) in refs {
        census.seen += 1;
        let e = p.transform.pos.to_f64();
        let (src, h) = ([e.x, -e.z], e.y);
        let (xy, height) = if let Some(&i) = member_of.get(&p.id.0) {
            matched += 1;
            let a = &anchors.anchors[i];
            let (xy, lifted) = a.rigid(src, h);
            if a.stamp.is_some() {
                census.members_stamped += 1;
                (xy, lifted)
            } else {
                census.members_point += 1;
                if let Some(g) = source_ground(src[0], src[1]).filter(|&g| g >= SEA_LEVEL_M) {
                    (xy, ground(xy[0], xy[1]) + (h - g))
                } else {
                    census.members_point_sea += 1;
                    (xy, lifted)
                }
            }
        } else if let Some(i) = stamped.find(anchors, src) {
            *census.inside.entry(class).or_default() += 1;
            anchors.anchors[i].rigid(src, h)
        } else {
            *census.dropped.entry(class).or_default() += 1;
            continue;
        };
        out.push(mw_exterior_bevy::worldgen::placed_at(p.clone(), DVec3::new(xy[0], height, -xy[1])));
    }
    census.unmatched_keys += member_of.len().saturating_sub(matched);
    (out, census)
}

#[cfg(test)]
mod tests {
    use super::*;
    use sf_world::{PlacementClass, PlacementFlags, PlacementId, RecordRef};
    use sf_worldgen::anchors::{AnchorPlace, Footprint};

    const S: f64 = 10.0;

    /// A static at source point (x east, y north, height).
    fn placement(id: u128, source: [f64; 3]) -> Placement {
        let p = Placement {
            id: PlacementId(id),
            base: RecordRef::new("STAT", "x"),
            model: None,
            transform: mw_world::convert::world_transform([0.0; 3], [0.0; 3], 1.0),
            class: PlacementClass::Static,
            flags: PlacementFlags::empty(),
        };
        mw_exterior_bevy::worldgen::placed_at(p, DVec3::new(source[0], source[2], -source[1]))
    }

    fn anchor(stamp: Option<usize>, c: [f64; 2], lift: f64, members: &[u128], r: f64) -> AnchorPlace {
        AnchorPlace {
            name: "a".into(),
            stamp,
            source_centre_m: c,
            world_centre_m: [S * c[0], S * c[1]],
            lift_m: lift,
            members: members.iter().map(|m| format!("{m:032x}")).collect(),
            footprint: Footprint { discs: vec![[c[0], c[1], r]] },
        }
    }

    /// The engine position as (x east, y north, height).
    fn at(p: &Placement) -> [f64; 3] {
        let e = p.transform.pos.to_f64();
        [e.x, -e.z, e.y]
    }

    fn close(a: [f64; 3], b: [f64; 3]) -> bool {
        a.iter().zip(&b).all(|(x, y)| (x - y).abs() < 1e-3)
    }

    /// Stamped town at c = (100, -50), L = 42.5; point anchor (a lamp) at (2000, 300) with L = 7.
    fn world() -> AnchorTransform {
        AnchorTransform::new(
            S,
            vec![anchor(Some(0), [100.0, -50.0], 42.5, &[1, 2], 30.0), anchor(None, [2000.0, 300.0], 7.0, &[3, 99], 2.0)],
        )
    }

    #[test]
    fn members_move_with_their_anchor() {
        let t = world();
        let refs = [placement(1, [95.0, -48.0, 12.0]), placement(3, [2000.5, 300.0, 21.0])];
        // Final ground 500 m everywhere; source ground 20 m under the lamp (it stands 1 m above it).
        let (out, census) =
            remap(refs.iter().map(|p| (p, Class::HumanMade)), &t, &|_, _| 500.0, &mut |_, _| Some(20.0));
        assert_eq!(out.len(), 2);
        // S·c + (p − c), height + L.
        assert!(close(at(&out[0]), [1000.0 - 5.0, -500.0 + 2.0, 12.0 + 42.5]), "{:?}", at(&out[0]));
        // Point anchor: the final ground + its offset above the source ground.
        assert!(close(at(&out[1]), [20000.0 + 0.5, 3000.0, 500.0 + 1.0]), "{:?}", at(&out[1]));
        assert_eq!(out[0].transform.rotation, refs[0].transform.rotation);
        assert_eq!((census.members_stamped, census.members_point, census.members_point_sea), (1, 1, 0));
        // Key 2 and 99 name no reference here.
        assert_eq!(census.unmatched_keys, 2);
    }

    #[test]
    fn point_member_over_the_sea_keeps_its_height() {
        let t = world();
        let refs = [placement(3, [2000.0, 300.0, 0.5])];
        let (out, census) = remap(refs.iter().map(|p| (p, Class::HumanMade)), &t, &|_, _| -80.0, &mut |_, _| Some(-8.0));
        assert!(close(at(&out[0]), [20000.0, 3000.0, 0.5 + 7.0]), "{:?}", at(&out[0]));
        assert_eq!(census.members_point_sea, 1);
    }

    #[test]
    fn nature_rides_inside_stamped_footprints_only() {
        let t = world();
        let refs = [
            placement(10, [110.0, -40.0, 5.0]), // inside the town: kept, rigid
            placement(11, [2000.5, 300.5, 5.0]), // inside the point anchor's disc: dropped (no stamp)
            placement(12, [900.0, 900.0, 5.0]),  // the wilds: dropped
        ];
        let (out, census) = remap(refs.iter().map(|p| (p, Class::Nature)), &t, &|_, _| 0.0, &mut |_, _| Some(0.0));
        assert_eq!(out.len(), 1);
        assert_eq!(out[0].id, PlacementId(10));
        assert!(close(at(&out[0]), [1000.0 + 10.0, -500.0 + 10.0, 5.0 + 42.5]));
        assert_eq!(census.inside.get(&Class::Nature), Some(&1));
        assert_eq!(census.dropped.get(&Class::Nature), Some(&2));
    }

    #[test]
    fn index_of_remapped_placements() {
        let t = world();
        let refs = [placement(1, [95.0, -48.0, 12.0]), placement(2, [105.0, -52.0, 13.0]), placement(3, [2000.0, 300.0, 21.0])];
        let (out, _) = remap(refs.iter().map(|p| (p, Class::HumanMade)), &t, &|_, _| 500.0, &mut |_, _| Some(20.0));
        let pages: std::collections::BTreeSet<[i32; 3]> = out
            .iter()
            .map(|p| sf_world::Partition::page_of(sf_world::Partition::STREAMING.cell_of(p.transform.pos)).to_array())
            .collect();
        let content = mw_exterior_bevy::worldgen::WorldgenContent { placements: out, extra: Vec::new() };
        let index = mw_exterior_bevy::worldgen::content_index(&mw_world::ids::exterior(), Some(&content), &|_| None);
        // The town and the lamp are 19 km apart at 10×: two pages.
        assert_eq!(pages.len(), 2);
        assert_eq!(index.stats.pages, 2);
        assert_eq!(index.stats.placements, 3);
        for cell in index.pages.iter().flat_map(|p| &p.cells) {
            for ip in &cell.placements {
                let source = refs.iter().find(|r| r.id == ip.placement.id).expect("indexed id is a source reference");
                let moved = ip.placement.transform.pos.to_f64().distance(source.transform.pos.to_f64());
                assert!(moved > 100.0, "{:?} still at its source position", ip.placement.id);
            }
        }
    }
}
