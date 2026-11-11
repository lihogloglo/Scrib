//! Opt-in bake telemetry; collection never changes generated tile bytes.

use super::TileKey;
use crate::connectivity::{Connectivity, IslandSummary, Reachability, SCHEMA_VERSION};
use crate::detour::NavMesh;
use crate::recast::TriangulationReport;
use crate::{Budget, Budgeted, Error, Workspace};
use alloc::vec::Vec;

/// Span transitions performed by the three raster filters (not erosion).
#[derive(Clone, Debug, Default, PartialEq, Eq)]
#[non_exhaustive]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct FilterCounts {
    /// Spans promoted from null to walkable by the low-hanging filter (it removes none).
    pub low_hanging: u32,
    /// Walkable spans cleared by the ledge filter.
    pub ledge: u32,
    /// Walkable spans cleared by the low-height filter.
    pub low_height: u32,
}

/// Telemetry for one tile column, including empty and failed columns.
/// Counts exclude the raster halo. Filter counts and removed-region counts describe
/// the full bordered working field, because filters and regions can cross the halo.
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct TileReport {
    /// Field contract version.
    pub schema_version: u32,
    /// Bake tile column (`x`, `z`), matching the job output key.
    pub key: TileKey,
    /// Output layers: zero for an empty tile, one for a polygon tile.
    pub layers: u8,
    /// Ground and off-mesh polygon count in the output DNAV tile.
    pub polys: u32,
    /// Vertex count in the output DNAV tile.
    pub verts: u32,
    /// Interior columns with at least one non-null span assigned a surviving region.
    pub walkable_columns: u32,
    /// Interior raster columns, including empty columns.
    pub total_columns: u32,
    /// Span transitions in filter execution order, over the bordered field.
    pub spans_removed: FilterCounts,
    /// Pre-merge regions discarded by the small-region test, over the bordered field.
    pub regions_removed_small: u32,
    /// Output polygons by area id, including off-mesh polygons.
    #[cfg_attr(feature = "serde", serde(serialize_with = "serialize_histogram"))]
    pub area_histogram: [u32; 64],
    /// Partial ear-clipping counts.
    pub triangulation: TriangulationReport,
    /// Tile-local weak components, counting connections resolved inside this tile only.
    /// IDs are local DNAV refs and must not be compared with refs in a world mesh.
    pub islands: Vec<IslandSummary>,
    /// The tile's failure, if any; all measurements then describe progress before failure.
    pub limits_hit: Option<Error>,
}

#[cfg(feature = "serde")]
fn serialize_histogram<S: serde::Serializer>(
    histogram: &[u32; 64],
    serializer: S,
) -> Result<S::Ok, S::Error> {
    serde::Serialize::serialize(histogram.as_slice(), serializer)
}

impl TileReport {
    pub(super) fn new(x: i32, z: i32, width: i32, height: i32, border: i32) -> Self {
        Self {
            schema_version: SCHEMA_VERSION,
            key: TileKey { x, z },
            layers: 0,
            polys: 0,
            verts: 0,
            walkable_columns: 0,
            total_columns: ((width - 2 * border).max(0) as u32)
                .saturating_mul((height - 2 * border).max(0) as u32),
            spans_removed: FilterCounts::default(),
            regions_removed_small: 0,
            area_histogram: [0; 64],
            triangulation: TriangulationReport::default(),
            islands: Vec::new(),
            limits_hit: None,
        }
    }

    pub(super) fn fail(&mut self, error: &Error) {
        if let Error::PartialTriangulation {
            failed_contours,
            missing_triangles,
        } = error
        {
            self.triangulation = TriangulationReport {
                failed_contours: *failed_contours,
                missing_triangles: *missing_triangles,
            };
        }
        self.limits_hit = Some(error.clone());
    }

    pub(super) fn finish(
        &mut self,
        data: Option<&[u8]>,
        triangulation: TriangulationReport,
        ws: &mut Workspace,
    ) -> Result<(), Error> {
        self.triangulation = triangulation;
        if let Some(data) = data {
            // Conservative allowance for the temporary validated mesh, graph and
            // component summaries. Reporting is opt-in and spends the tile's limits.
            ws.charge_bytes::<u8>(data.len().checked_mul(16).ok_or(Error::SizeOverflow)?)?;
            ws.work((data.len() as u64).div_ceil(4))?;
            let (nav, _) = NavMesh::new_single(data.to_vec())?;
            if let Some(tile) = nav.tile(0) {
                self.layers = 1;
                self.polys = tile.header.poly_count as u32;
                self.verts = tile.header.vert_count as u32;
                for p in &tile.polys {
                    self.area_histogram[usize::from(p.area())] += 1;
                }
            }
            let mut budget = Budget::new(ws.limits().max_work.saturating_sub(ws.usage().work));
            let measured = (|| {
                let Budgeted::Complete(Some(conn)) = Connectivity::new(&nav, &mut budget)? else {
                    return Err(Error::LimitExceeded);
                };
                let Budgeted::Complete(Some(world)) = conn.reachability(0, &mut budget)? else {
                    return Err(Error::LimitExceeded);
                };
                self.islands = world.islands;
                Ok(())
            })();
            ws.work(budget.used())?;
            measured?;
        }
        Ok(())
    }
}

/// A job's completed tiles and its first failing tile, in row-major order.
/// `world` is supplied separately after the caller installs tiles: bake output
/// alone cannot establish cross-tile runtime connectivity or residency.
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct BakeReport {
    /// Field contract version.
    pub schema_version: u32,
    /// Finished tiles, plus a failing tile if a tile stage failed.
    pub tiles: Vec<TileReport>,
    /// Optional separately computed resident-world statistics.
    pub world: Option<Reachability>,
}

impl Default for BakeReport {
    fn default() -> Self {
        Self {
            schema_version: SCHEMA_VERSION,
            tiles: Vec::new(),
            world: None,
        }
    }
}
