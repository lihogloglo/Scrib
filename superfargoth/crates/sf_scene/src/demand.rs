//! Texture demand of the static scene (textures.md §6 "Demand", slice S3b): the baked
//! `texture.demand` product, one per streaming cell, which the texture streamer (S4) reads for
//! every resident cell.
//!
//! **Product.** The cell's placements grouped into clusters (module [`cluster`]): each cluster is
//! a bounding sphere (cell-relative, metres) and the textures drawn inside it, each with its UV
//! density ρ (UV units per metre, the densest use in the cluster; `None` = no data, the streamer's
//! 1 UV/m fallback). The streamer turns each (cluster, texture) into one
//! `sf_texture::residency::Demand` with the distance from the eye to the sphere.
//!
//! **Texture ids** ([`texture_id`]): a stable 64-bit hash (FNV-1a) of the texture's canonical VFS
//! path **without its extension**: lower case, `/` separators, no empty or `.` segments (the
//! `sf_vfs::VfsPath` form), then everything from the last segment's last `.` cut. The bake hashes
//! the reference a material names (`textures/tx_a.dds` → `textures/tx_a`), the runtime the path
//! it loaded the image from (`textures/tx_a.ktx2`, the same id): a texture cook may change the
//! served container but never the stem. The streamer wraps it as
//! `sf_texture::residency::TextureId(texture_id(path))`; give it the VFS path, not an asset-source
//! URL. Collisions: 64 bits over ~10⁴ textures, ~3·10⁻¹² for a whole install.
//!
//! **Format** (little endian, version [`DEMAND_FORMAT`]): magic `SFTD`, version u32, the cell
//! the positions are relative to (3 × i32), cluster count u32, texture count u32; per cluster
//! `center` (3 × f32), `radius` f32, first texture u32, texture count u32; per texture `id` u64,
//! `rho` f32 (0 = none). Clusters in [`cluster`] order, textures by id inside a cluster.

use glam::{IVec3, Vec3};

use crate::table::Sphere;

/// The bake step that writes [`CellDemand`] (subject: the cell, as `scene.cells`).
pub const DEMAND_STEP: &str = "texture.demand";
/// Magic of the product.
pub const DEMAND_MAGIC: [u8; 4] = *b"SFTD";
/// Format version of the product.
pub const DEMAND_FORMAT: u32 = 1;
/// Largest cluster radius, metres (textures.md §6). A single placement larger than this is a
/// cluster of its own, as large as it is.
pub const MAX_CLUSTER_RADIUS: f32 = 16.0;
/// Version of the clustering rule ([`cluster`]): part of the step's key.
pub const CLUSTER_RULE: u32 = 1;

/// The stable id of a texture (module docs).
pub fn texture_id(path: &str) -> u64 {
    // FNV-1a 64 over the canonical bytes; `/` between kept segments.
    const OFFSET: u64 = 0xcbf2_9ce4_8422_2325;
    const PRIME: u64 = 0x0100_0000_01b3;
    let segments: Vec<&str> = path.split(['/', '\\']).filter(|s| !s.is_empty() && *s != ".").collect();
    let mut h = OFFSET;
    let mut put = |b: u8| {
        h ^= u64::from(b.to_ascii_lowercase());
        h = h.wrapping_mul(PRIME);
    };
    for (k, segment) in segments.iter().enumerate() {
        if k > 0 {
            put(b'/');
        }
        let last = k + 1 == segments.len();
        let segment = match segment.rfind('.') {
            Some(dot) if last && dot > 0 => &segment[..dot],
            _ => segment,
        };
        segment.bytes().for_each(&mut put);
    }
    h
}

/// One texture of a cluster.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DemandTexture {
    /// [`texture_id`] of its path.
    pub id: u64,
    /// UV units per metre (world space, the densest use in the cluster); `None`: no data.
    pub rho: Option<f32>,
}

/// One cluster: a bounding sphere and the textures drawn inside it.
#[derive(Clone, Debug, PartialEq)]
pub struct DemandCluster {
    /// Relative to [`CellDemand::origin_cell`], metres.
    pub sphere: Sphere,
    /// By id, each once.
    pub textures: Vec<DemandTexture>,
}

/// A cell's `texture.demand` product.
#[derive(Clone, Debug, PartialEq)]
pub struct CellDemand {
    /// The grid cell the cluster centres are relative to (`sf_coords::WorldPos::cell`).
    pub origin_cell: IVec3,
    pub clusters: Vec<DemandCluster>,
}

/// A malformed product.
#[derive(Clone, Debug, PartialEq, Eq, thiserror::Error)]
pub enum DemandError {
    #[error("not a texture demand product (magic)")]
    Magic,
    #[error("texture demand format {0}, expected {DEMAND_FORMAT}")]
    Version(u32),
    #[error("texture demand product truncated or with trailing bytes")]
    Length,
    #[error("cluster {0} names textures outside the texture section")]
    Range(usize),
    #[error("more than u32::MAX clusters or textures")]
    TooLarge,
}

const HEADER: usize = 4 + 4 + 12 + 4 + 4;
const CLUSTER: usize = 16 + 8;
const TEXTURE: usize = 12;

impl CellDemand {
    /// Totals: clusters, textures summed over clusters.
    pub fn texture_count(&self) -> usize {
        self.clusters.iter().map(|c| c.textures.len()).sum()
    }

    pub fn write(&self) -> Result<Vec<u8>, DemandError> {
        let count = |n: usize| u32::try_from(n).map_err(|_| DemandError::TooLarge);
        let textures = self.texture_count();
        let mut out = Vec::with_capacity(HEADER + self.clusters.len() * CLUSTER + textures * TEXTURE);
        out.extend_from_slice(&DEMAND_MAGIC);
        out.extend_from_slice(&DEMAND_FORMAT.to_le_bytes());
        self.origin_cell.to_array().iter().for_each(|v| out.extend_from_slice(&v.to_le_bytes()));
        out.extend_from_slice(&count(self.clusters.len())?.to_le_bytes());
        out.extend_from_slice(&count(textures)?.to_le_bytes());
        let mut first = 0usize;
        for c in &self.clusters {
            c.sphere.center.to_array().iter().for_each(|v| out.extend_from_slice(&v.to_le_bytes()));
            out.extend_from_slice(&c.sphere.radius.to_le_bytes());
            out.extend_from_slice(&count(first)?.to_le_bytes());
            out.extend_from_slice(&count(c.textures.len())?.to_le_bytes());
            first += c.textures.len();
        }
        for t in self.clusters.iter().flat_map(|c| &c.textures) {
            out.extend_from_slice(&t.id.to_le_bytes());
            out.extend_from_slice(&t.rho.unwrap_or(0.0).to_le_bytes());
        }
        Ok(out)
    }

    /// Reads a product (the S4 streamer's entry point), validating every range.
    pub fn read(bytes: &[u8]) -> Result<Self, DemandError> {
        let u32_at = |at: usize| bytes.get(at..at + 4).map(|b| u32::from_le_bytes(b.try_into().expect("4 bytes")));
        let f32_at = |at: usize| u32_at(at).map(f32::from_bits);
        if bytes.get(..4) != Some(&DEMAND_MAGIC[..]) {
            return Err(DemandError::Magic);
        }
        let version = u32_at(4).ok_or(DemandError::Length)?;
        if version != DEMAND_FORMAT {
            return Err(DemandError::Version(version));
        }
        let header: Vec<u32> = (0..5).map(|k| u32_at(8 + 4 * k)).collect::<Option<_>>().ok_or(DemandError::Length)?;
        #[allow(clippy::cast_possible_wrap)] // stored from i32
        let origin_cell = IVec3::new(header[0] as i32, header[1] as i32, header[2] as i32);
        let (clusters, textures) = (header[3] as usize, header[4] as usize);
        let texture_base = HEADER + clusters * CLUSTER;
        if bytes.len() != texture_base + textures * TEXTURE {
            return Err(DemandError::Length);
        }
        let mut out = Vec::with_capacity(clusters);
        for k in 0..clusters {
            let at = HEADER + k * CLUSTER;
            let f = |i: usize| f32_at(at + 4 * i).expect("length checked");
            let (first, count) = (u32_at(at + 16).expect("length checked") as usize, u32_at(at + 20).expect("length checked") as usize);
            if first.checked_add(count).is_none_or(|end| end > textures) {
                return Err(DemandError::Range(k));
            }
            let list = (first..first + count)
                .map(|t| {
                    let at = texture_base + t * TEXTURE;
                    let id = u64::from_le_bytes(bytes[at..at + 8].try_into().expect("8 bytes"));
                    let rho = f32_at(at + 8).expect("length checked");
                    DemandTexture { id, rho: (rho > 0.0).then_some(rho) }
                })
                .collect();
            out.push(DemandCluster { sphere: Sphere { center: Vec3::new(f(0), f(1), f(2)), radius: f(3) }, textures: list });
        }
        Ok(Self { origin_cell, clusters: out })
    }
}

/// The smallest sphere this rule gives around `members` (the centre of their bounding box, the
/// radius reaching every member sphere): deterministic and cheap, at most √3/2 above optimal.
pub fn enclosing(members: impl IntoIterator<Item = Sphere> + Clone) -> Option<Sphere> {
    let (lo, hi) = members
        .clone()
        .into_iter()
        .map(|s| (s.center - s.radius, s.center + s.radius))
        .reduce(|(a, b), (c, d)| (a.min(c), b.max(d)))?;
    let center = (lo + hi) * 0.5;
    let radius = members.into_iter().map(|s| s.center.distance(center) + s.radius).fold(0.0, f32::max);
    Some(Sphere { center, radius })
}

/// Groups `spheres` (one per placement) into clusters of radius ≤ [`MAX_CLUSTER_RADIUS`];
/// returns each cluster's sphere and its members (indices into `spheres`, ascending).
///
/// Rule ([`CLUSTER_RULE`] 1): placements in order of (x, z, y, radius, index) of their spheres
/// (`f32::total_cmp`), so the result depends only on the spheres, not on the input order when
/// they are distinct; each joins the existing cluster whose enclosing sphere ([`enclosing`]) grows
/// the least while staying within the limit (ties: the earlier cluster), or starts a new one. A
/// sphere already over the limit is a cluster of its own.
pub fn cluster(spheres: &[Sphere]) -> Vec<(Sphere, Vec<usize>)> {
    let mut order: Vec<usize> = (0..spheres.len()).collect();
    order.sort_by(|&a, &b| {
        let (p, q) = (&spheres[a], &spheres[b]);
        p.center
            .x
            .total_cmp(&q.center.x)
            .then(p.center.z.total_cmp(&q.center.z))
            .then(p.center.y.total_cmp(&q.center.y))
            .then(p.radius.total_cmp(&q.radius))
            .then(a.cmp(&b))
    });
    let mut out: Vec<(Sphere, Vec<usize>)> = Vec::new();
    for i in order {
        let s = spheres[i];
        if s.radius.is_nan() || s.radius > MAX_CLUSTER_RADIUS {
            out.push((s, vec![i]));
            continue;
        }
        let mut best: Option<(usize, Sphere)> = None;
        for (k, (bound, members)) in out.iter().enumerate() {
            if bound.radius > MAX_CLUSTER_RADIUS {
                continue;
            }
            // Cheap reject: every member lies within the bound, and two spheres inside one of
            // radius R have centres at most 2R apart, so past 3R no grown cluster fits.
            if bound.center.distance(s.center) > 3.0 * MAX_CLUSTER_RADIUS {
                continue;
            }
            let Some(grown) = enclosing(members.iter().map(|&m| spheres[m]).chain([s])) else { continue };
            if grown.radius <= MAX_CLUSTER_RADIUS && best.is_none_or(|(_, b)| grown.radius < b.radius) {
                best = Some((k, grown));
            }
        }
        match best {
            Some((k, grown)) => {
                out[k].0 = grown;
                out[k].1.push(i);
            }
            None => out.push((s, vec![i])),
        }
    }
    for (_, members) in &mut out {
        members.sort_unstable();
    }
    out
}

/// Merges `(id, rho)` uses into one entry per id, the densest ρ winning (`None` loses to any
/// value), sorted by id.
pub fn merge_textures(uses: impl IntoIterator<Item = DemandTexture>) -> Vec<DemandTexture> {
    let mut v: Vec<DemandTexture> = uses.into_iter().collect();
    v.sort_by(|a, b| a.id.cmp(&b.id).then(b.rho.unwrap_or(0.0).total_cmp(&a.rho.unwrap_or(0.0))));
    v.dedup_by_key(|t| t.id);
    v
}

#[cfg(feature = "bake")]
pub mod bake;

#[cfg(test)]
mod tests;
