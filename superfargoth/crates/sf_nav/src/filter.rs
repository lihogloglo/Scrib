//! Query filters (Detour `dtQueryFilter`, UE `FNavigationQueryFilter`): which polygons a query may
//! use and what each area costs.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::{AgentClass, AgentClassId, AreaId, AreaTable, MAX_AREAS, PolyFlags};

/// A polygon passes when its flags share a bit with `include` and none with `exclude` (Detour's
/// rule) and its area's cost is finite. Moving a meter across an area costs its multiplier.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct QueryFilter {
    pub include: PolyFlags,
    pub exclude: PolyFlags,
    #[serde(serialize_with = "ser_costs", deserialize_with = "de_costs")]
    area_cost: [f32; MAX_AREAS],
}

impl Default for QueryFilter {
    /// Detour's default: every flag included, none excluded, every area costs 1.
    fn default() -> Self {
        Self {
            include: PolyFlags::all(),
            exclude: PolyFlags::empty(),
            area_cost: [1.0; MAX_AREAS],
        }
    }
}

impl QueryFilter {
    /// Costs from the adapter's table for `class` (areas it doesn't list cost 1), every flag
    /// included, [`PolyFlags::DISABLED`] excluded.
    pub fn for_class(table: &AreaTable, class: AgentClassId) -> Self {
        let mut f = Self {
            exclude: PolyFlags::DISABLED,
            ..Self::default()
        };
        for a in table.iter() {
            f.area_cost[a.id.index()] = a.cost[class.index()];
        }
        f
    }

    /// [`QueryFilter::for_class`], also excluding [`PolyFlags::SWIM`] for a class that can't swim.
    pub fn for_agent(table: &AreaTable, class: &AgentClass) -> Self {
        let mut f = Self::for_class(table, class.id);
        if !class.can_swim {
            f.exclude |= PolyFlags::SWIM;
        }
        f
    }

    #[must_use]
    pub fn including(mut self, flags: PolyFlags) -> Self {
        self.include = flags;
        self
    }

    #[must_use]
    pub fn excluding(mut self, flags: PolyFlags) -> Self {
        self.exclude |= flags;
        self
    }

    pub fn area_cost(&self, area: AreaId) -> f32 {
        self.area_cost
            .get(area.index())
            .copied()
            .unwrap_or(f32::INFINITY)
    }

    /// Sets one area's cost multiplier; `f32::INFINITY` excludes the area (UE `SetExcludedArea`).
    /// Costs below 1 are allowed but make Detour's A* inexact (its heuristic assumes 1).
    pub fn set_area_cost(&mut self, area: AreaId, cost: f32) {
        assert!(
            area.index() < MAX_AREAS,
            "area id {} outside 0..{MAX_AREAS}",
            area.0
        );
        assert!(cost >= 0.0, "area cost {cost} is negative");
        self.area_cost[area.index()] = cost;
    }

    /// Whether a polygon with this area and these flags may be used.
    pub fn passes(&self, area: AreaId, flags: PolyFlags) -> bool {
        flags.intersects(self.include)
            && !flags.intersects(self.exclude)
            && self.area_cost(area).is_finite()
    }

    /// The cheapest finite area cost: the scale of an admissible A* heuristic.
    pub fn min_cost(&self) -> f32 {
        let m = self
            .area_cost
            .iter()
            .copied()
            .filter(|c| c.is_finite())
            .fold(f32::INFINITY, f32::min);
        if m.is_finite() { m } else { 0.0 }
    }
}

fn ser_costs<S: Serializer>(costs: &[f32; MAX_AREAS], s: S) -> Result<S::Ok, S::Error> {
    s.collect_seq(costs)
}

fn de_costs<'de, D: Deserializer<'de>>(d: D) -> Result<[f32; MAX_AREAS], D::Error> {
    let v = Vec::<f32>::deserialize(d)?;
    let n = v.len();
    v.try_into()
        .map_err(|_| serde::de::Error::invalid_length(n, &"64 area costs"))
}
