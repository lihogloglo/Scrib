//! The world index as `sf_bake` steps (feature `bake`).
//!
//! Four steps, keyed so that only pages touching a changed source region
//! rebuild (streaming.md, "Index building"):
//!
//! | Step | Subject | Reads | Output |
//! |---|---|---|---|
//! | `world.region` | `<ws>/<region hex>` | the region's fingerprint (param) | the region's placements, bucketed by page |
//! | `world.layout` | `<ws>` | every `world.region` of the worldspace | page → regions map |
//! | `world.page` | `<ws>/<x>,<y>,<z>` | the layout; *dynamic*: the regions touching the page, `model.info` of each model | [`IndexPage`] bytes |
//! | `world.dir` | `<ws>` | the layout; *dynamic*: every page | [`CellDirectory`](crate::CellDirectory) bytes |
//!
//! An unchanged region fingerprint is a trace hit for `world.region`, and
//! unchanged product bytes stop propagation (`sf_bake`'s early cutoff), so a
//! changed region rebuilds its bucket, the layout (cheap), and only the
//! pages it touches; `world.dir` then re-reads every page (decoding is
//! fast) but most come from the cache.
//!
//! [`BakedIndex`] is the runtime [`IndexReader`] over a store and manifest.

use std::collections::{BTreeMap, BTreeSet};
use std::sync::{Arc, Mutex, PoisonError};

use glam::{IVec3, Vec3};
use serde::{Deserialize, Serialize};
use sf_bake::{BakeError, Manifest, PlanCx, ProductKey, RunCx, Step, Store};
use sf_model::steps::ModelInfoStep;
use sf_vfs::VfsPath;

use crate::codec::{CodecError, Header, PlacementRepr, Strings};
use crate::{
    Fingerprint, IndexPage, IndexParams, IndexReader, ModelInfo, Partition, Placement, RegionId,
    WorldSource, WorldspaceId, build_directory, build_pages,
};

const REGION_MAGIC: [u8; 4] = *b"SFWR";
const LAYOUT_MAGIC: [u8; 4] = *b"SFWL";

/// Step ids.
pub const REGION_STEP: &str = "world.region";
pub const LAYOUT_STEP: &str = "world.layout";
pub const PAGE_STEP: &str = "world.page";
pub const DIR_STEP: &str = "world.dir";

/// Subject of a worldspace-wide product (`world.layout`, `world.dir`).
pub fn worldspace_subject(ws: &WorldspaceId) -> String {
    ws.as_str().to_owned()
}

/// Subject of a `world.region` product.
pub fn region_subject(ws: &WorldspaceId, region: RegionId) -> String {
    format!("{}/{:016x}", ws.as_str(), region.0)
}

/// Subject of a `world.page` product.
pub fn page_subject(ws: &WorldspaceId, page: IVec3) -> String {
    format!("{}/{},{},{}", ws.as_str(), page.x, page.y, page.z)
}

/// Splits `<ws>/<tail>` at the last `/` (worldspace ids may contain `/`;
/// tails never do).
fn split_subject(subject: &str) -> Option<(WorldspaceId, &str)> {
    let (ws, tail) = subject.rsplit_once('/')?;
    Some((WorldspaceId::new(ws), tail))
}

pub(crate) fn parse_region(subject: &str) -> Option<(WorldspaceId, RegionId)> {
    let (ws, tail) = split_subject(subject)?;
    Some((ws, RegionId(u64::from_str_radix(tail, 16).ok()?)))
}

pub(crate) fn parse_page(subject: &str) -> Option<(WorldspaceId, IVec3)> {
    let (ws, tail) = split_subject(subject)?;
    let mut it = tail.split(',').map(str::parse::<i32>);
    let page = IVec3::new(it.next()?.ok()?, it.next()?.ok()?, it.next()?.ok()?);
    it.next().is_none().then_some((ws, page))
}

/// The resident-size estimate of a model from its `model.info`: decoded
/// vertex and index data, plus half again for its LOD chain.
pub fn model_bytes(info: &sf_model::ModelInfo) -> u64 {
    (u64::from(info.vertices) * 36 + u64::from(info.triangles) * 12) * 3 / 2
}

/// `model.info` as the index sees it (`None` for a model without geometry).
pub fn index_model_info(info: &sf_model::ModelInfo) -> Option<ModelInfo> {
    if info.bounds.is_empty() {
        return None;
    }
    Some(ModelInfo {
        min: Vec3::from_array(info.bounds.min),
        max: Vec3::from_array(info.bounds.max),
        bytes: model_bytes(info),
        lod_distance: 0.0,
    })
}

/// Shared inputs of the four steps: the adapter's source, the index
/// settings, and a per-worldspace snapshot of region fingerprints (taken
/// once per bake session).
pub struct WorldBake {
    source: Arc<dyn WorldSource>,
    params: IndexParams,
    /// Worldspaces indexed with other settings (e.g. interiors with
    /// `Partition::Single`).
    overrides: BTreeMap<WorldspaceId, IndexParams>,
    regions: Mutex<BTreeMap<WorldspaceId, Arc<BTreeMap<RegionId, Fingerprint>>>>,
}

impl WorldBake {
    /// One set of index settings for every worldspace.
    pub fn new(source: Arc<dyn WorldSource>, params: IndexParams) -> Arc<Self> {
        Self::with_overrides(source, params, BTreeMap::new())
    }

    /// `params` by default, `overrides` for the worldspaces they name. The
    /// usual source of overrides is each `Worldspace` record's `partition`
    /// (interiors load whole: `Partition::Single`).
    pub fn with_overrides(
        source: Arc<dyn WorldSource>,
        params: IndexParams,
        overrides: BTreeMap<WorldspaceId, IndexParams>,
    ) -> Arc<Self> {
        Arc::new(Self { source, params, overrides, regions: Mutex::new(BTreeMap::new()) })
    }

    /// The index settings of a worldspace.
    pub fn params(&self, ws: &WorldspaceId) -> IndexParams {
        self.overrides.get(ws).copied().unwrap_or(self.params)
    }

    /// Registers the four `world.*` steps on a baker (which also needs
    /// `model.info`).
    pub fn register(self: &Arc<Self>, baker: &mut sf_bake::Baker) {
        baker
            .register(RegionStep(self.clone()))
            .register(LayoutStep(self.clone()))
            .register(PageStep(self.clone()))
            .register(DirStep(self.clone()));
    }

    /// The product that pulls in a worldspace's whole index.
    pub fn directory_key(ws: &WorldspaceId) -> ProductKey {
        ProductKey::new(DIR_STEP, worldspace_subject(ws))
    }

    /// Bakes whole worldspaces with their pages in parallel: resolves each
    /// layout, then every page and directory in one `Baker::bake` call.
    /// (Baking only the directory works too, but its page reads are
    /// dynamic, so they would run one after another.)
    pub fn bake_worldspaces(baker: &sf_bake::Baker, worldspaces: &[WorldspaceId]) -> sf_bake::BakeReport {
        // Every layout first, in one parallel call: a world with many small
        // worldspaces (interiors) would otherwise resolve them one by one.
        let layouts: Vec<ProductKey> =
            worldspaces.iter().map(|ws| ProductKey::new(LAYOUT_STEP, worldspace_subject(ws))).collect();
        if layouts.len() > 1 {
            baker.bake(&layouts);
        }
        let mut keys = Vec::new();
        for ws in worldspaces {
            let layout = baker
                .product(&ProductKey::new(LAYOUT_STEP, worldspace_subject(ws)))
                .and_then(|d| Ok(baker.store().get(&d)?))
                .ok()
                .and_then(|bytes| Layout::from_bytes(&bytes).ok());
            if let Some(layout) = layout {
                keys.extend(layout.page_keys().map(|p| ProductKey::new(PAGE_STEP, page_subject(ws, p))));
            }
            keys.push(Self::directory_key(ws));
        }
        baker.bake(&keys)
    }

    fn regions(&self, ws: &WorldspaceId) -> Arc<BTreeMap<RegionId, Fingerprint>> {
        let mut cache = self.regions.lock().unwrap_or_else(PoisonError::into_inner);
        cache
            .entry(ws.clone())
            .or_insert_with(|| {
                let mut map = BTreeMap::new();
                self.source.regions(ws, &mut |r, fp| {
                    map.insert(r, fp);
                });
                Arc::new(map)
            })
            .clone()
    }

    fn params_bytes(&self, ws: &WorldspaceId) -> Vec<u8> {
        let p = self.params(ws);
        postcard::to_stdvec(&(p.partition, p.far_min_radius)).expect("params serialize")
    }
}

fn bad_subject(step: &str, subject: &str) -> BakeError {
    BakeError::step(&ProductKey::new(step, subject), "malformed subject")
}

fn codec_error(step: &str, subject: &str, e: &CodecError) -> BakeError {
    BakeError::step(&ProductKey::new(step, subject), e)
}

// -- region buckets -----------------------------------------------------------

/// The leading part of a region blob: enough for the layout step to read
/// the page list without decoding placements.
#[derive(Serialize, Deserialize)]
struct RegionHead {
    header: Header,
    partition: Partition,
    pages: Vec<[i32; 3]>,
}

#[derive(Serialize, Deserialize)]
struct RegionRepr {
    head: RegionHead,
    strings: Vec<String>,
    buckets: Vec<Vec<PlacementRepr>>,
}

/// One source region's placements, split by index page (`world.region`).
#[derive(Clone, Debug, PartialEq)]
pub struct RegionBuckets {
    pub partition: Partition,
    /// Pages in key order, each with its placements in source order.
    pub pages: Vec<(IVec3, Vec<Placement>)>,
}

impl RegionBuckets {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut strings = Strings::default();
        let buckets =
            self.pages.iter().map(|(_, ps)| ps.iter().map(|p| PlacementRepr::encode(p, &mut strings)).collect()).collect();
        let repr = RegionRepr {
            head: RegionHead {
                header: Header::new(REGION_MAGIC),
                partition: self.partition,
                pages: self.pages.iter().map(|(p, _)| p.to_array()).collect(),
            },
            strings: strings.list,
            buckets,
        };
        postcard::to_stdvec(&repr).expect("region buckets always serialize")
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self, CodecError> {
        Header::expect(bytes, REGION_MAGIC, "region buckets")?;
        let repr: RegionRepr = postcard::from_bytes(bytes)?;
        let pages = repr
            .head
            .pages
            .iter()
            .zip(&repr.buckets)
            .map(|(page, bucket)| {
                let placements = bucket.iter().map(|p| p.decode(&repr.strings)).collect::<Result<_, _>>()?;
                Ok((IVec3::from_array(*page), placements))
            })
            .collect::<Result<_, CodecError>>()?;
        Ok(Self { partition: repr.head.partition, pages })
    }

    /// Only the page list, without decoding placements.
    pub fn page_keys(bytes: &[u8]) -> Result<Vec<IVec3>, CodecError> {
        Header::expect(bytes, REGION_MAGIC, "region buckets")?;
        let (head, _) = postcard::take_from_bytes::<RegionHead>(bytes)?;
        Ok(head.pages.into_iter().map(IVec3::from_array).collect())
    }
}

// -- layout ---------------------------------------------------------------------

#[derive(Serialize, Deserialize)]
struct LayoutRepr {
    header: Header,
    pages: Vec<([i32; 3], Vec<u64>)>,
}

/// A worldspace's page → source regions map (`world.layout`).
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Layout {
    pub pages: BTreeMap<[i32; 3], Vec<RegionId>>,
}

impl Layout {
    pub fn to_bytes(&self) -> Vec<u8> {
        let repr = LayoutRepr {
            header: Header::new(LAYOUT_MAGIC),
            pages: self.pages.iter().map(|(k, v)| (*k, v.iter().map(|r| r.0).collect())).collect(),
        };
        postcard::to_stdvec(&repr).expect("layouts always serialize")
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self, CodecError> {
        Header::expect(bytes, LAYOUT_MAGIC, "world layout")?;
        let repr: LayoutRepr = postcard::from_bytes(bytes)?;
        Ok(Self { pages: repr.pages.into_iter().map(|(k, v)| (k, v.into_iter().map(RegionId).collect())).collect() })
    }

    pub fn page_keys(&self) -> impl Iterator<Item = IVec3> + '_ {
        self.pages.keys().map(|k| IVec3::from_array(*k))
    }
}

// -- steps ----------------------------------------------------------------------

/// `world.region`: one source region's placements bucketed by page.
pub struct RegionStep(pub Arc<WorldBake>);

impl Step for RegionStep {
    fn id(&self) -> &'static str {
        REGION_STEP
    }
    fn version(&self) -> u32 {
        1
    }
    fn plan(&self, subject: &str, cx: &mut PlanCx) -> Result<(), BakeError> {
        let (ws, region) = parse_region(subject).ok_or_else(|| bad_subject(REGION_STEP, subject))?;
        let fp = self.0.regions(&ws).get(&region).copied().ok_or_else(|| {
            BakeError::step(&ProductKey::new(REGION_STEP, subject), "region not in the world source")
        })?;
        cx.param("fingerprint", fp.0.to_vec()).param("index", self.0.params_bytes(&ws));
        Ok(())
    }
    fn run(&self, subject: &str, _cx: &mut RunCx<'_>) -> Result<Vec<u8>, BakeError> {
        let (ws, region) = parse_region(subject).ok_or_else(|| bad_subject(REGION_STEP, subject))?;
        let mut placements = Vec::new();
        self.0.source.placements(&ws, region, &mut |p| placements.push(p));
        let partition = self.0.params(&ws).partition;
        let pages = crate::bucket_by_page(partition, placements);
        Ok(RegionBuckets { partition, pages }.to_bytes())
    }
}

/// `world.layout`: which regions touch which page.
pub struct LayoutStep(pub Arc<WorldBake>);

impl Step for LayoutStep {
    fn id(&self) -> &'static str {
        LAYOUT_STEP
    }
    fn version(&self) -> u32 {
        1
    }
    fn plan(&self, subject: &str, cx: &mut PlanCx) -> Result<(), BakeError> {
        let ws = WorldspaceId::new(subject);
        for region in self.0.regions(&ws).keys() {
            cx.product(REGION_STEP, region_subject(&ws, *region));
        }
        cx.param("index", self.0.params_bytes(&ws));
        Ok(())
    }
    fn run(&self, subject: &str, cx: &mut RunCx<'_>) -> Result<Vec<u8>, BakeError> {
        let ws = WorldspaceId::new(subject);
        let mut layout = Layout::default();
        for region in self.0.regions(&ws).keys() {
            let bytes = cx.product(REGION_STEP, &region_subject(&ws, *region))?;
            let pages = RegionBuckets::page_keys(&bytes).map_err(|e| codec_error(LAYOUT_STEP, subject, &e))?;
            for page in pages {
                layout.pages.entry(page.to_array()).or_default().push(*region);
            }
        }
        Ok(layout.to_bytes())
    }
}

/// `world.page`: one index page from the regions touching it.
pub struct PageStep(pub Arc<WorldBake>);

impl Step for PageStep {
    fn id(&self) -> &'static str {
        PAGE_STEP
    }
    fn version(&self) -> u32 {
        1
    }
    fn plan(&self, subject: &str, cx: &mut PlanCx) -> Result<(), BakeError> {
        let (ws, _) = parse_page(subject).ok_or_else(|| bad_subject(PAGE_STEP, subject))?;
        cx.product(LAYOUT_STEP, worldspace_subject(&ws)).param("index", self.0.params_bytes(&ws));
        Ok(())
    }
    fn run(&self, subject: &str, cx: &mut RunCx<'_>) -> Result<Vec<u8>, BakeError> {
        let (ws, page) = parse_page(subject).ok_or_else(|| bad_subject(PAGE_STEP, subject))?;
        let layout = Layout::from_bytes(&cx.product(LAYOUT_STEP, &worldspace_subject(&ws))?)
            .map_err(|e| codec_error(PAGE_STEP, subject, &e))?;
        let regions = layout.pages.get(&page.to_array()).cloned().unwrap_or_default();

        // Placements of this page, region by region (regions in key order,
        // placements in source order: deterministic).
        let mut placements = Vec::new();
        for region in regions {
            let bytes = cx.product(REGION_STEP, &region_subject(&ws, region))?;
            let mut buckets = RegionBuckets::from_bytes(&bytes).map_err(|e| codec_error(PAGE_STEP, subject, &e))?;
            if let Some(i) = buckets.pages.iter().position(|(p, _)| *p == page) {
                placements.append(&mut buckets.pages[i].1);
            }
        }

        // model.info of every model, as dynamic inputs. A model whose info
        // fails (no importer, broken file) is indexed as a point; its file
        // is still recorded, so fixing it rebuilds this page.
        let models: BTreeSet<VfsPath> = placements.iter().filter_map(|p| p.model.clone()).collect();
        let mut infos = BTreeMap::new();
        for model in models {
            match cx.product(ModelInfoStep::ID, model.as_str()) {
                Ok(bytes) => {
                    let info = ModelInfoStep::decode(&bytes)
                        .map_err(|e| BakeError::step(&ProductKey::new(PAGE_STEP, subject), e))?;
                    if let Some(info) = index_model_info(&info) {
                        infos.insert(model, info);
                    }
                }
                Err(_) => {
                    cx.file_exists(model.as_str())?;
                }
            }
        }
        let (pages, _stats) = build_pages(&self.0.params(&ws), placements, &|m: &VfsPath| infos.get(m).copied());
        let page = pages.into_iter().find(|p| p.page == page).unwrap_or(IndexPage {
            partition: self.0.params(&ws).partition,
            page,
            cells: Vec::new(),
        });
        Ok(page.to_bytes())
    }
}

/// `world.dir`: the always-resident directory, from every page.
pub struct DirStep(pub Arc<WorldBake>);

impl Step for DirStep {
    fn id(&self) -> &'static str {
        DIR_STEP
    }
    fn version(&self) -> u32 {
        1
    }
    fn plan(&self, subject: &str, cx: &mut PlanCx) -> Result<(), BakeError> {
        cx.product(LAYOUT_STEP, subject).param("index", self.0.params_bytes(&WorldspaceId::new(subject)));
        Ok(())
    }
    fn run(&self, subject: &str, cx: &mut RunCx<'_>) -> Result<Vec<u8>, BakeError> {
        let ws = WorldspaceId::new(subject);
        let layout =
            Layout::from_bytes(&cx.product(LAYOUT_STEP, subject)?).map_err(|e| codec_error(DIR_STEP, subject, &e))?;
        let mut pages = Vec::with_capacity(layout.pages.len());
        for page in layout.page_keys() {
            let bytes = cx.product(PAGE_STEP, &page_subject(&ws, page))?;
            let page = IndexPage::from_bytes(&bytes).map_err(|e| codec_error(DIR_STEP, subject, &e))?;
            if !page.cells.is_empty() {
                pages.push(page);
            }
        }
        let params = self.0.params(&ws);
        Ok(build_directory(ws, &params, &pages).to_bytes())
    }
}

/// The runtime [`IndexReader`] over bake products: `world.dir` and
/// `world.page` objects found through a profile manifest.
#[derive(Clone)]
pub struct BakedIndex {
    store: Store,
    manifest: Arc<Manifest>,
}

impl BakedIndex {
    pub fn new(store: Store, manifest: Arc<Manifest>) -> Self {
        Self { store, manifest }
    }

    fn read(&self, step: &str, subject: &str) -> Result<Option<Vec<u8>>, String> {
        let Some(digest) = self.manifest.get(step, subject) else { return Ok(None) };
        self.store.get(&digest).map(Some).map_err(|e| format!("{step}({subject}): {e}"))
    }
}

impl IndexReader for BakedIndex {
    fn directory(&self, ws: &WorldspaceId) -> Result<Option<Vec<u8>>, String> {
        self.read(DIR_STEP, &worldspace_subject(ws))
    }

    fn page(&self, ws: &WorldspaceId, page: IVec3) -> Result<Option<Vec<u8>>, String> {
        self.read(PAGE_STEP, &page_subject(ws, page))
    }
}
