//! `Worldspace` and `DoorLink` records from cells and door references.
//!
//! Spec: `docs/specs/world-adapter.md` ("Worldspaces" and "Door links").

use std::collections::{BTreeMap, HashMap};

use glam::{Quat, Vec2};
use sf_records::{RecordDb, RecordError, RecordId, SourceId};
use sf_world::{DoorLink, LinkEnd, Partition, PlacementId, RecordRef, Worldspace, WorldspaceId};
use sf_coords::WorldTransform;
use tes3::esp::{Cell, CellFlags, Door};

use crate::convert::{meters, world_transform};
use crate::ids;

/// An interior's environment: what `Worldspace::environment` points at.
#[derive(Clone, Debug, PartialEq)]
pub struct CellEnvironment {
    /// `AMBI` colors as sRGB 0..=1, and fog density. `None` without `AMBI`.
    pub atmosphere: Option<Atmosphere>,
    /// "Behave like exterior": sky and weather (Mournhold's plazas).
    pub sky: bool,
    /// Water surface height in meters (engine Y), when the cell has water.
    pub water_level: Option<f32>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Atmosphere {
    pub ambient: [f32; 3],
    pub sunlight: [f32; 3],
    pub fog_color: [f32; 3],
    pub fog_density: f32,
}

impl CellEnvironment {
    pub const KIND: &'static str = "mw.cell_environment";

    pub fn of(cell: &Cell) -> Self {
        let rgb = |c: [u8; 4]| [c[0], c[1], c[2]].map(|v| f32::from(v) / 255.0);
        let water = cell.data.flags.contains(CellFlags::HAS_WATER);
        Self {
            atmosphere: cell.atmosphere_data.as_ref().map(|a| Atmosphere {
                ambient: rgb(a.ambient_color),
                sunlight: rgb(a.sunlight_color),
                fog_color: rgb(a.fog_color),
                fog_density: a.fog_density,
            }),
            sky: cell.data.flags.contains(CellFlags::BEHAVES_LIKE_EXTERIOR),
            water_level: water.then(|| meters(cell.water_height.unwrap_or(0.0))),
        }
    }
}

/// Door pairing threshold: the far door must stand within this distance of
/// the near door's arrival point, and its own arrival point within this
/// distance of the near door. Chosen from the vanilla distribution
/// (spec: "Door links").
pub const PAIR_DISTANCE_M: f32 = 8.0;

/// A door reference with a destination.
#[derive(Clone, Debug, PartialEq)]
pub struct DoorEnd {
    pub id: PlacementId,
    /// The door reference's base id and cell (for reports).
    pub base: String,
    pub cell: RecordId,
    pub worldspace: WorldspaceId,
    pub pose: WorldTransform,
    pub destination: WorldspaceId,
    pub arrival: WorldTransform,
}

/// Counters from building door links.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct DoorStats {
    /// Door references with a destination.
    pub doors: usize,
    /// Two-way links (two doors each).
    pub paired: usize,
    /// One-way links.
    pub one_way: usize,
    /// Destinations naming no existing interior: (door cell, destination name).
    pub dangling: Vec<(RecordId, String)>,
    /// For each door, the score (sum of both distances, meters) of its best
    /// back-pointing candidate, if any: the data behind [`PAIR_DISTANCE_M`].
    pub best_scores: Vec<f32>,
}

/// Every worldspace and door link of the loaded content.
#[derive(Clone, Debug, Default)]
pub struct WorldRecords {
    pub worldspaces: Vec<(WorldspaceId, Worldspace)>,
    pub environments: Vec<(RecordId, CellEnvironment)>,
    pub door_links: Vec<(RecordId, DoorLink)>,
    pub doors: DoorStats,
}

impl WorldRecords {
    /// Builds every record from the loaded cells.
    pub fn build(db: &RecordDb) -> Self {
        let mut out = Self::default();
        let mut exterior = Worldspace::new("Morrowind");
        exterior.partition = Partition::STREAMING;
        exterior.has_terrain = true;
        exterior.sea_level = Some(0.0);
        out.worldspaces.push((ids::exterior(), exterior));
        for (id, cell) in db.iter::<Cell>().filter(|(_, c)| c.is_interior()) {
            let env = CellEnvironment::of(cell);
            let mut ws = Worldspace::new(cell.name.clone());
            ws.partition = Partition::Single;
            ws.sea_level = env.water_level;
            ws.environment = Some(RecordRef::new(CellEnvironment::KIND, id.clone()));
            out.worldspaces.push((ids::interior(&cell.name), ws));
            out.environments.push((id.clone(), env));
        }
        let (ends, dangling) = door_ends(db);
        let (links, stats) = link_doors(&ends);
        out.door_links = links;
        out.doors = DoorStats { dangling, ..stats };
        out
    }

    /// Registers the kinds and defines every record under `source` (the
    /// adapter's synthesized source, applied after the content files).
    pub fn define_into(&self, db: &mut RecordDb, source: SourceId) -> Result<(), RecordError> {
        sf_world::register_kinds(db)?;
        db.register::<CellEnvironment>(CellEnvironment::KIND)?;
        for (id, ws) in &self.worldspaces {
            db.define(source, id.0.clone(), ws.clone())?;
        }
        for (id, env) in &self.environments {
            db.define(source, id.clone(), env.clone())?;
        }
        for (id, link) in &self.door_links {
            db.define(source, id.clone(), link.clone())?;
        }
        Ok(())
    }
}

/// Every door reference with a destination, sorted by id. Destinations
/// naming no interior are returned separately.
pub fn door_ends(db: &RecordDb) -> (Vec<DoorEnd>, Vec<(RecordId, String)>) {
    let mut ends = Vec::new();
    let mut dangling = Vec::new();
    for (cell_id, cell) in db.iter::<Cell>() {
        for (&(source, refr_index), reference) in &cell.references {
            let Some(dest) = &reference.destination else { continue };
            if db.get::<Door>(&RecordId::new(&reference.id)).is_none() {
                continue;
            }
            let destination = if dest.cell.trim().is_empty() {
                ids::exterior()
            } else {
                match db.get::<Cell>(&RecordId::new(dest.cell.trim())) {
                    Some(c) if c.is_interior() => ids::interior(&c.name),
                    _ => {
                        dangling.push((cell_id.clone(), dest.cell.clone()));
                        continue;
                    }
                }
            };
            let file = &db.source(SourceId(source)).name;
            ends.push(DoorEnd {
                id: ids::placement_id(file, refr_index),
                base: reference.id.clone(),
                cell: cell_id.clone(),
                worldspace: ids::worldspace_of(cell),
                pose: world_transform(reference.translation, reference.rotation, 1.0),
                destination,
                arrival: world_transform(dest.translation, dest.rotation, 1.0),
            });
        }
    }
    ends.sort_by_key(|e| e.id);
    dangling.sort();
    (ends, dangling)
}

/// Pairs doors whose destinations point at each other into two-way links;
/// the rest become one-way links. Returns the links (sorted by id) and their
/// statistics (everything but `dangling`).
///
/// A candidate pair (i, j): j is in i's destination worldspace, i in j's,
/// j stands within [`PAIR_DISTANCE_M`] of i's arrival and i within it of
/// j's arrival. Pairs are accepted greedily by score (sum of the two
/// distances), then by id, so the result doesn't depend on input order.
pub fn link_doors(ends: &[DoorEnd]) -> (Vec<(RecordId, DoorLink)>, DoorStats) {
    let mut by_ws: HashMap<&WorldspaceId, Vec<usize>> = HashMap::new();
    for (i, e) in ends.iter().enumerate() {
        by_ws.entry(&e.worldspace).or_default().push(i);
    }
    let dist = |a: &WorldTransform, b: &WorldTransform| a.pos.delta(b.pos).length();
    let mut candidates: Vec<(f64, usize, usize)> = Vec::new();
    let mut best_scores = Vec::new();
    for (i, a) in ends.iter().enumerate() {
        let mut best = f64::INFINITY;
        for &j in by_ws.get(&a.destination).into_iter().flatten() {
            let b = &ends[j];
            if i == j || b.destination != a.worldspace {
                continue;
            }
            let (d1, d2) = (dist(&b.pose, &a.arrival), dist(&a.pose, &b.arrival));
            best = best.min(d1 + d2);
            let limit = f64::from(PAIR_DISTANCE_M);
            if i < j && d1 <= limit && d2 <= limit {
                candidates.push((d1 + d2, i, j));
            }
        }
        if best.is_finite() {
            #[allow(clippy::cast_possible_truncation)]
            best_scores.push(best as f32);
        }
    }
    candidates.sort_by(|a, b| a.0.total_cmp(&b.0).then(ends[a.1].id.cmp(&ends[b.1].id)).then(ends[a.2].id.cmp(&ends[b.2].id)));
    let mut partner: Vec<Option<usize>> = vec![None; ends.len()];
    for (_, i, j) in candidates {
        if partner[i].is_none() && partner[j].is_none() {
            partner[i] = Some(j);
            partner[j] = Some(i);
        }
    }

    let mut links = BTreeMap::new();
    let (mut paired, mut one_way) = (0, 0);
    for (i, a) in ends.iter().enumerate() {
        match partner[i] {
            Some(j) if j < i => {}
            Some(j) => {
                let b = &ends[j];
                paired += 1;
                links.insert(link_id(a.id), DoorLink { a: door_end(a, b), b: door_end(b, a), two_way: true });
            }
            None => {
                one_way += 1;
                let far = LinkEnd {
                    worldspace: a.destination.clone(),
                    frame: arrival_frame(&a.arrival),
                    arrival: a.arrival,
                    door: None,
                    extent: Vec2::ZERO,
                };
                let near = LinkEnd {
                    worldspace: a.worldspace.clone(),
                    frame: a.pose,
                    arrival: a.pose,
                    door: Some(a.id),
                    extent: Vec2::ZERO,
                };
                links.insert(link_id(a.id), DoorLink { a: near, b: far, two_way: false });
            }
        }
    }
    let stats = DoorStats { doors: ends.len(), paired, one_way, dangling: Vec::new(), best_scores };
    (links.into_iter().collect(), stats)
}

/// The record id of the link leaving through door `id`.
pub fn link_id(id: PlacementId) -> RecordId {
    RecordId::from(format!("mw.door/{id}"))
}

/// The end at door `here`, whose arrivals come from `there`'s destination.
fn door_end(here: &DoorEnd, there: &DoorEnd) -> LinkEnd {
    let mut frame = arrival_frame(&there.arrival);
    frame.pos = here.pose.pos;
    LinkEnd { worldspace: here.worldspace.clone(), frame, arrival: there.arrival, door: Some(here.id), extent: Vec2::ZERO }
}

/// A portal frame from an arrival pose: +Z along the arriving traveler's
/// facing (engine forward is −Z), i.e. pointing into the worldspace.
fn arrival_frame(arrival: &WorldTransform) -> WorldTransform {
    WorldTransform {
        pos: arrival.pos,
        rotation: (arrival.rotation * Quat::from_rotation_y(std::f32::consts::PI)).normalize(),
        scale: glam::Vec3::ONE,
    }
}
