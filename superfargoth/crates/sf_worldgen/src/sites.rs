//! **Wilderness and settlement sites, `sites.json` formats 1 and 2** (spec `sites.md` "sites.json format 2", slice
//! WG-P1d): the generic Scrib reader of the site list an offline placer writes into a guide folder. Bevy-free and
//! game-agnostic: kinds, access classes and reach demands are the spec's generic vocabulary; ids, cultures,
//! templates and kits are opaque strings the adapter's data gives.
//!
//! **Frame.** Guide frame metres, x east, y north ([`crate::guide`]).
//!
//! **Format 2** (a superset of format 1, whose fields keep their meaning): header `format, seed, guide_blake3,
//! table_blake3, packs_blake3` (+ optional `producer`); per site the fields of [`Site`]. The placement-time fields come
//! from the placer, the status fields (`status, moved_m, travel_s, via, reason`) from reachability (WG-V1); both are
//! optional here so the same reader takes `sites.json` and `sites.valid.json`. `buildings` and `installations` are the
//! placer's serde, passed through untouched.
//!
//! **Format 1** (the town list the road router reads): header `format` (+ `producer`), per site `key, name,
//! population, culture, centre_m, radius_m, laid_out, anchor`. It reads into the same types: kind `town`, kept, no
//! clearing or pad, reach `any`, no access class.
//!
//! **Checks** (the settle convention): `format` first, then every object rejects an unknown field (a format-2 field
//! in a format-1 file is unknown there), and a key may appear once.

use std::collections::BTreeSet;
use std::fs::File;
use std::io::{BufReader, BufWriter, Read, Write};
use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::{Error, Result};

#[cfg(test)]
#[path = "tests_sites.rs"]
mod tests;

#[path = "site_clearings.rs"]
pub mod clearings;

#[path = "site_pads.rs"]
pub mod pads;

/// The `sites.json` format this module writes (and reads, beside [`FORMAT_V1`]).
// tunables: const (format version)
pub const FORMAT: u32 = 2;
/// The older town-list format, still read.
// tunables: const (format version)
pub const FORMAT_V1: u32 = 1;

/// What placed a site (spec "Site table" `source`, plus kept towns).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SiteKind {
    Town,
    Hamlet,
    Farm,
    Activity,
    Remote,
    Node,
}

/// How a site joins the road network (the placer's `access_class`; the table's `access`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AccessClass {
    Road,
    Track,
    Trail,
    /// No path leads there; never routed.
    None,
}

/// What reachability demands of a site (the table's `reach`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Reach {
    /// Reached on foot, no water step.
    Foot,
    /// Reached on foot or by swimming.
    Swim,
    /// No demand.
    #[default]
    Any,
}

/// Reachability verdict (WG-V1).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Status {
    Placed,
    Reseated,
    Dropped,
}

/// How the best path reaches a site (WG-V1).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Via {
    Foot,
    Swim,
}

/// One site, format 2. Required: `key`, `centre_m`, `kind`, `kept`; the rest default to empty, zero or absent.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Site {
    /// Unique key: kept towns their own, generated sites `site:<row>:<placer id>`.
    pub key: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub name: String,
    /// Name grammar key (WG-NM1).
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub name_key: String,
    #[serde(default, skip_serializing_if = "is_zero_u32")]
    pub population: u32,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub culture: String,
    pub centre_m: [f64; 2],
    /// Settlement extent (m); 0 = the placer's own.
    #[serde(default, skip_serializing_if = "is_zero_f64")]
    pub radius_m: f64,
    /// A town laid out after the roads (format 1 meaning).
    #[serde(default, skip_serializing_if = "is_false")]
    pub laid_out: bool,
    /// Lead anchor the site was taken from (format 1 meaning), if any.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub anchor: Option<String>,
    pub kind: SiteKind,
    /// A kept (hand-made) settlement, not generated.
    pub kept: bool,
    /// Site-table row id (empty for kept towns).
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub row: String,
    /// Placer template id.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub template: String,
    /// Placer mode (`attached_external`, `remote_seasonal`, `network_node`, ..).
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub mode: String,
    /// Parent settlement key.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub parent: String,
    /// Ground height at the site (m), when the placer gave it.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ground_m: Option<f64>,
    #[serde(default, skip_serializing_if = "is_zero_f64")]
    pub yaw_deg: f64,
    /// Footprint ring (guide m), empty when the placer gave none (then `radius_m` stands for it).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub footprint: Vec<[f64; 2]>,
    /// Point joining the road network.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub access: Option<[f64; 2]>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub access_class: Option<AccessClass>,
    /// Remote places: how one gets there (`cliff-stair`, `causeway`, ..); its line is an installation.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub approach: String,
    /// Remote places: the terrain situation (`summit`, `islet`, ..).
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub situation: String,
    /// Kit style id.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub kit: String,
    /// Flora-free margin beyond the footprint (m).
    #[serde(default, skip_serializing_if = "is_zero_f64")]
    pub clearing_m: f64,
    /// Flatten pad wanted (WG-P2).
    #[serde(default, skip_serializing_if = "is_false")]
    pub pad: bool,
    #[serde(default, skip_serializing_if = "is_any")]
    pub reach: Reach,
    /// Placer buildings, passed through.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub buildings: Vec<serde_json::Value>,
    /// Placer ground installations, passed through.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub installations: Vec<serde_json::Value>,
    /// WG-V1 verdict.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<Status>,
    /// WG-V1: distance re-seated (m).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub moved_m: Option<f64>,
    /// WG-V1: travel time from the nearest source (s).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub travel_s: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub via: Option<Via>,
    /// WG-V1: why a site was dropped.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub reason: String,
}

/// A site list in the guide folder, read from format 1 or 2, written as format 2.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SitesFile {
    pub seed: u64,
    /// blake3 (hex) of the guide, the site table and the placer packs the list came from.
    pub guide_blake3: String,
    pub table_blake3: String,
    pub packs_blake3: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub producer: String,
    pub sites: Vec<Site>,
}

/// Format 1 header.
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct FileV1 {
    #[serde(rename = "format")]
    _format: u32,
    #[serde(default)]
    producer: String,
    sites: Vec<SiteV1>,
}

/// Format 1 site.
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct SiteV1 {
    key: String,
    #[serde(default)]
    name: String,
    population: u32,
    culture: String,
    centre_m: [f64; 2],
    #[serde(default)]
    radius_m: f64,
    #[serde(default)]
    laid_out: bool,
    #[serde(default)]
    anchor: Option<String>,
}

/// Format 2 on disk: the header's `format` beside the file.
#[derive(Serialize)]
struct Out<'a> {
    format: u32,
    #[serde(flatten)]
    file: &'a SitesFile,
}

impl From<SiteV1> for Site {
    fn from(s: SiteV1) -> Self {
        Self {
            key: s.key,
            name: s.name,
            name_key: String::new(),
            population: s.population,
            culture: s.culture,
            centre_m: s.centre_m,
            radius_m: s.radius_m,
            laid_out: s.laid_out,
            anchor: s.anchor,
            kind: SiteKind::Town,
            kept: true,
            row: String::new(),
            template: String::new(),
            mode: String::new(),
            parent: String::new(),
            ground_m: None,
            yaw_deg: 0.0,
            footprint: Vec::new(),
            access: None,
            access_class: None,
            approach: String::new(),
            situation: String::new(),
            kit: String::new(),
            clearing_m: 0.0,
            pad: false,
            reach: Reach::Any,
            buildings: Vec::new(),
            installations: Vec::new(),
            status: None,
            moved_m: None,
            travel_s: None,
            via: None,
            reason: String::new(),
        }
    }
}

impl SitesFile {
    /// Reads `path` (format 1 or 2).
    ///
    /// # Errors
    /// An unreadable or malformed file, a missing or unknown `format`, an unknown field, a duplicate key.
    pub fn read(path: &Path) -> Result<Self> {
        Self::from_reader(BufReader::new(File::open(path)?))
            .map_err(|e| Error::Invalid(format!("{}: {e}", path.display())))
    }

    /// Reads a site list (format 1 or 2) from `reader`.
    ///
    /// # Errors
    /// As [`SitesFile::read`].
    pub fn from_reader(reader: impl Read) -> Result<Self> {
        let mut value: serde_json::Value = serde_json::from_reader(reader)?;
        let format = value.get("format").ok_or_else(|| Error::Invalid("sites.json: no `format`".into()))?;
        let (number, shown) = (format.as_u64(), format.to_string());
        let file = match number {
            Some(f) if f == u64::from(FORMAT) => {
                if let Some(o) = value.as_object_mut() {
                    o.remove("format");
                }
                serde_json::from_value::<Self>(value)?
            }
            Some(f) if f == u64::from(FORMAT_V1) => {
                let v1: FileV1 = serde_json::from_value(value)?;
                Self {
                    seed: 0,
                    guide_blake3: String::new(),
                    table_blake3: String::new(),
                    packs_blake3: String::new(),
                    producer: v1.producer,
                    sites: v1.sites.into_iter().map(Site::from).collect(),
                }
            }
            _ => {
                return Err(Error::Invalid(format!(
                    "sites.json format {shown} (reads {FORMAT_V1} and {FORMAT})"
                )));
            }
        };
        let mut keys = BTreeSet::new();
        if let Some(s) = file.sites.iter().find(|s| !keys.insert(s.key.as_str())) {
            return Err(Error::Invalid(format!("sites.json: site key `{}` twice", s.key)));
        }
        Ok(file)
    }

    /// Format 2 JSON (pretty).
    ///
    /// # Errors
    /// Serialisation failure (none for finite values).
    pub fn to_json(&self) -> Result<String> {
        Ok(serde_json::to_string_pretty(&Out { format: FORMAT, file: self })?)
    }

    /// Writes format 2 to `path`.
    ///
    /// # Errors
    /// An unwritable path.
    pub fn write(&self, path: &Path) -> Result<()> {
        let mut w = BufWriter::new(File::create(path)?);
        w.write_all(self.to_json()?.as_bytes())?;
        w.flush()?;
        Ok(())
    }

    /// The site with `key`.
    #[must_use]
    pub fn site(&self, key: &str) -> Option<&Site> {
        self.sites.iter().find(|s| s.key == key)
    }
}

// serde `skip_serializing_if` takes references.
#[allow(clippy::trivially_copy_pass_by_ref)]
fn is_false(b: &bool) -> bool {
    !*b
}

#[allow(clippy::trivially_copy_pass_by_ref)]
fn is_zero_u32(v: &u32) -> bool {
    *v == 0
}

#[allow(clippy::trivially_copy_pass_by_ref)]
fn is_zero_f64(v: &f64) -> bool {
    *v == 0.0
}

#[allow(clippy::trivially_copy_pass_by_ref)]
fn is_any(r: &Reach) -> bool {
    *r == Reach::Any
}
