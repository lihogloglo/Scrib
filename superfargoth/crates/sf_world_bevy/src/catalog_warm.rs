//! Pipeline warm-up W4b1 (`docs/specs/pipeline-warmup.md` §5.3 row 2, §5.4, §7): the baked
//! content [`Catalog`] precached at the first reveal.
//!
//! The game hands [`CatalogWarmup`] the profile's catalog ([`CatalogWarmup::set_catalog`]) and
//! starts it ([`CatalogWarmup::start`], e.g. on entering its loading phase); any reveal hold
//! starts it too. Each entry's representative loads as a cooked model (the streaming path:
//! `baked://<step>/<model>.<mesh_ext>`, or [`CatalogWarmup::loader`]); its submesh and material
//! become a [`WarmSpecimen`] through the same material path streaming draws with (statics: the
//! streaming cache with its matclass fold; skinned: [`CatalogWarmup::skinned_materials`], the
//! character path's mapping), with the entry's flags (shadow caster, a crossfading range,
//! previous skin/morph when skinned/morphed). Once its textures are loaded the specimen goes to
//! [`Precache`] under [`CATALOG_TAG`]; the handles stay alive for the session. Builtins (terrain,
//! water, ...) are named in the catalog and built by [`BuiltinSpecimens`], which the game fills;
//! one not available yet ([`BuiltinOutcome::NotYet`]) is retried while a reveal holds.
//!
//! W4b3: the catalog never holds a reveal. [`CATALOG_TAG`] is a background precache tag
//! ([`Precache::set_background`]): admitted after the reveal's shown set
//! ([`crate::ResidentWarmup`]), paused while a reveal drains its last compiles, and at the play
//! limit after the reveal. [`CatalogWarmup::pending`] (representatives loading, then the tag's
//! specimens waiting for assets, queued or compiling) is shown in the gate's cap warning and the
//! `warmup: reveal` line. A representative that fails (no product, uncooked, a texture that
//! failed) is counted and logged once, never waited for; a specimen whose pipeline fails counts as
//! done (logged). One line when the tag resolves: `warmup: catalog N specimens (B builtins), M
//! pipelines, T s (before the reveal | +R s after the reveal)` (M: specimens resolved, ready or
//! failed; R: from the first reveal the warm-up saw).
//! Industry: UE5 PSO precaching at load (bundled PSO cache), Unity `ShaderVariantCollection.WarmUp`.

use std::collections::HashMap;
use std::sync::Arc;

use bevy_asset::{Asset, AssetServer, Assets, Handle, LoadState, RecursiveDependencyLoadState, UntypedHandle};
use bevy_camera::visibility::VisibilityRange;
use bevy_ecs::prelude::*;
use bevy_image::Image;
use bevy_mesh::Mesh;
use bevy_pbr::StandardMaterial;
use bevy_platform::time::Instant;
use sf_model::catalog::{Catalog, CatalogEntry, Representative};
use sf_model_bevy::{CookedModelAsset, baked_path};

use crate::precache::{Precache, Specimen, SpecimenFlags, specimen_flags};
use crate::reveal::RevealGate;
use crate::spawn::ModelMaterials;
use crate::stream::{Streaming, cached_material};

/// The precache tag of the catalog's specimens.
pub const CATALOG_TAG: &str = "catalog";

/// Starts loading a representative's cooked model; `None`: it has no asset path (counted failed).
pub type RepresentativeLoader = Arc<dyn Fn(&Representative, &AssetServer) -> Option<Handle<CookedModelAsset>> + Send + Sync>;

/// A specimen with the handles that keep its mesh and material alive.
#[derive(Clone, Debug)]
pub struct WarmSpecimen {
    pub mesh: Handle<Mesh>,
    pub material: UntypedHandle,
    pub flags: SpecimenFlags,
}

impl WarmSpecimen {
    pub fn new<M: Asset>(mesh: Handle<Mesh>, material: Handle<M>, flags: SpecimenFlags) -> Self {
        Self { mesh, material: material.untyped(), flags }
    }

    #[must_use]
    pub fn specimen(&self) -> Specimen {
        Specimen { mesh: self.mesh.id(), material: self.material.id(), flags: self.flags }
    }
}

/// What a builtin's builder gave.
#[derive(Clone, Debug)]
pub enum BuiltinOutcome {
    /// Its specimens (the system's own mesh and material).
    Ready(Vec<WarmSpecimen>),
    /// Its system has no live state yet (e.g. the terrain in an interior): retried while a reveal
    /// holds.
    NotYet(String),
    /// Never: not a mesh material, or not built in this app (logged once).
    Skip(String),
}

/// A builtin's builder.
pub type BuiltinFn = Arc<dyn Fn(&mut World) -> BuiltinOutcome + Send + Sync>;

/// The builders of the catalog's builtins, by name (the game registers them).
#[derive(Resource, Clone, Default)]
pub struct BuiltinSpecimens(HashMap<String, BuiltinFn>);

impl BuiltinSpecimens {
    pub fn register(&mut self, name: impl Into<String>, build: impl Fn(&mut World) -> BuiltinOutcome + Send + Sync + 'static) -> &mut Self {
        self.0.insert(name.into(), Arc::new(build));
        self
    }

    #[must_use]
    pub fn contains(&self, name: &str) -> bool {
        self.0.contains_key(name)
    }

    /// Runs `name`'s builder; an unknown name is a [`BuiltinOutcome::Skip`] (not fatal).
    pub fn build(&self, name: &str, world: &mut World) -> BuiltinOutcome {
        match self.0.get(name) {
            Some(build) => build(world),
            None => BuiltinOutcome::Skip(format!("no builder registered for builtin \"{name}\"")),
        }
    }
}

/// The catalog warm-up (see the module docs). `WorldStreamingPlugin` adds it, idle until a
/// catalog is set.
#[derive(Resource, Default)]
pub struct CatalogWarmup {
    /// Materials of skinned representatives (the character path's mapping, never folded);
    /// `None`: [`ModelMaterials`], unfolded.
    pub skinned_materials: Option<ModelMaterials>,
    /// Loads a representative; `None`: `baked://<step>/<model>.<mesh_ext>` (`StreamingConfig`).
    pub loader: Option<RepresentativeLoader>,
    catalog: Option<Catalog>,
    start: bool,
    run: Option<Run>,
}

enum Slot {
    Loading(Handle<CookedModelAsset>),
    /// Built; waiting for its material's textures.
    Textures(WarmSpecimen, Handle<StandardMaterial>, String),
    Done,
}

struct Run {
    started: Instant,
    slots: Vec<Slot>,
    /// Builtins not built yet (`NotYet`), by name.
    builtins: Vec<String>,
    tried_builtins: bool,
    /// Builtins whose `NotYet` was logged.
    noted: Vec<String>,
    models: Vec<Handle<CookedModelAsset>>,
    kept: Vec<WarmSpecimen>,
    skinned_cache: HashMap<String, Handle<StandardMaterial>>,
    failures: Vec<String>,
    builtin_specimens: usize,
    reported: bool,
    /// A reveal held while the warm-up ran.
    held: bool,
    /// When the first hold the warm-up saw ended (the catalog then runs in the background).
    revealed: Option<Instant>,
}

impl Run {
    fn loading(&self) -> usize {
        self.slots.iter().filter(|s| !matches!(s, Slot::Done)).count()
    }
}

impl CatalogWarmup {
    /// Sets the catalog to warm (once per session: a later one is ignored); `false` if one was
    /// already set.
    pub fn set_catalog(&mut self, catalog: Catalog) -> bool {
        if self.catalog.is_some() {
            return false;
        }
        self.catalog = Some(catalog);
        true
    }

    #[must_use]
    pub fn has_catalog(&self) -> bool {
        self.catalog.is_some()
    }

    /// Starts the warm-up on the next frame (a reveal hold starts it too).
    pub fn start(&mut self) {
        self.start = true;
    }

    #[must_use]
    pub fn is_started(&self) -> bool {
        self.run.is_some()
    }

    /// The representatives are in and the tag settled (the `warmup: catalog` line was written).
    #[must_use]
    pub fn is_resolved(&self) -> bool {
        self.run.as_ref().is_some_and(|r| r.reported)
    }

    /// Every specimen handed to the precache so far.
    #[must_use]
    pub fn specimens(&self) -> &[WarmSpecimen] {
        self.run.as_ref().map_or(&[], |r| &r.kept)
    }

    /// Representatives that failed to load or build, with the reason.
    #[must_use]
    pub fn failures(&self) -> &[String] {
        self.run.as_ref().map_or(&[], |r| &r.failures)
    }

    /// Catalog specimens still to come (shown by the reveal gate, never waited for since W4b3):
    /// every entry before the start, then representatives
    /// loading plus the tag's specimens waiting for assets, queued or compiling (failures never
    /// wait). `0` without a catalog.
    #[must_use]
    pub fn pending(&self, precache: Option<&Precache>) -> usize {
        let Some(catalog) = &self.catalog else { return 0 };
        let Some(run) = &self.run else { return catalog.entries.len() };
        let tag = precache.map_or(0, |p| {
            let s = p.status(CATALOG_TAG);
            (s.pending_assets + s.queued + s.compiling) as usize
        });
        run.loading() + tag
    }

    /// Work left this frame (the system's run condition).
    fn busy(&self, holding: bool) -> bool {
        match (&self.catalog, &self.run) {
            (None, _) => false,
            (Some(_), None) => self.start || holding,
            (Some(_), Some(run)) => !run.reported || run.loading() > 0 || (holding && !run.builtins.is_empty()),
        }
    }
}

/// The entry's entity key bits: shadow caster, a crossfading (non-abrupt) range, previous
/// skin/morph when skinned/morphed (`pipeline-warmup.md` §5.2).
#[must_use]
pub fn entry_flags(entry: &CatalogEntry) -> SpecimenFlags {
    let range = entry.flags.crossfade.then_some(VisibilityRange { start_margin: 0.0..0.0, end_margin: 0.0..1.0, use_aabb: false });
    specimen_flags(entry.flags.shadow_caster, range.as_ref(), None, entry.layout.skinned, entry.layout.morphs)
}

pub(crate) fn warmup_busy(warmup: Option<Res<CatalogWarmup>>, gate: Option<Res<RevealGate>>) -> bool {
    warmup.is_some_and(|w| w.busy(gate.is_some_and(|g| g.is_holding())))
}

/// One frame of the warm-up (exclusive: loads, materials, builtins).
pub(crate) fn drive_catalog_warmup(world: &mut World) {
    let holding = world.get_resource::<RevealGate>().is_some_and(RevealGate::is_holding);
    world.resource_scope(|world, mut warmup: Mut<CatalogWarmup>| {
        let warmup = &mut *warmup;
        let Some(catalog) = &warmup.catalog else { return };
        if warmup.run.is_none() {
            if !(warmup.start || holding) {
                return;
            }
            warmup.run = Some(begin(world, catalog, warmup.loader.as_ref()));
        }
        let run = warmup.run.as_mut().expect("begun");
        if holding {
            run.held = true;
        } else if run.held && run.revealed.is_none() {
            run.revealed = Some(Instant::now());
        }
        poll_models(world, catalog, run, warmup.skinned_materials.as_ref());
        let mut ready = poll_textures(world, run);
        if !run.tried_builtins || holding {
            run.tried_builtins = true;
            ready.extend(poll_builtins(world, run));
        }
        let precache = world.get_resource::<Precache>().cloned();
        if !ready.is_empty() {
            if let Some(precache) = &precache {
                // Background (W4b3): after the reveal's shown set; never holds a reveal.
                precache.set_background(CATALOG_TAG);
                precache.request(ready.iter().map(WarmSpecimen::specimen), CATALOG_TAG);
            }
            run.kept.extend(ready);
        }
        report(run, catalog, precache.as_ref());
    });
}

fn begin(world: &World, catalog: &Catalog, loader: Option<&RepresentativeLoader>) -> Run {
    let server = world.resource::<AssetServer>();
    let mesh_ext = world.get_resource::<Streaming>().map(|s| s.config().mesh_ext.clone());
    let mut failures = Vec::new();
    let slots = catalog
        .entries
        .iter()
        .map(|entry| {
            let rep = &entry.representative;
            let handle = match (loader, &mesh_ext) {
                (Some(load), _) => load(rep, server),
                (None, Some(ext)) => Some(server.load(baked_path(&rep.step, &rep.model, Some(ext)))),
                (None, None) => None,
            };
            handle.map_or_else(
                || {
                    failures.push(format!("{}: no asset path", rep.model));
                    Slot::Done
                },
                Slot::Loading,
            )
        })
        .collect();
    Run {
        started: Instant::now(),
        slots,
        builtins: catalog.builtins.clone(),
        tried_builtins: false,
        noted: Vec::new(),
        models: Vec::new(),
        kept: Vec::new(),
        skinned_cache: HashMap::new(),
        failures,
        builtin_specimens: 0,
        reported: false,
        held: false,
        revealed: None,
    }
}

/// What a loaded representative gives: its submesh's mesh, the material description, skinned.
fn pick(asset: &CookedModelAsset, rep: &Representative) -> Result<(Handle<Mesh>, sf_model::Material, bool), String> {
    if let Some(reason) = &asset.skipped {
        return Err(format!("not cooked ({reason})"));
    }
    let part = asset.lods.get(rep.lod as usize).and_then(|l| l.parts.get(rep.submesh as usize));
    let Some(&(mesh, material)) = part else { return Err(format!("no lod {} submesh {}", rep.lod, rep.submesh)) };
    let mesh = asset.meshes.get(mesh).cloned().ok_or_else(|| format!("mesh {mesh} missing"))?;
    let desc = asset.materials.get(material as usize).cloned().unwrap_or_default();
    Ok((mesh, desc, asset.skin.is_some()))
}

/// Loaded representatives become specimens (their textures still loading, so they wait).
fn poll_models(world: &mut World, catalog: &Catalog, run: &mut Run, skinned: Option<&ModelMaterials>) {
    let server = world.resource::<AssetServer>().clone();
    let mut picked = Vec::new();
    {
        let cooked = world.resource::<Assets<CookedModelAsset>>();
        for (i, slot) in run.slots.iter_mut().enumerate() {
            let Slot::Loading(handle) = slot else { continue };
            let rep = &catalog.entries[i].representative;
            let result = match server.get_recursive_dependency_load_state(handle.id()) {
                Some(RecursiveDependencyLoadState::Failed(e)) => Err(e.to_string()),
                Some(RecursiveDependencyLoadState::Loaded) | None => match cooked.get(handle.id()) {
                    Some(asset) => pick(asset, rep),
                    None if server.get_load_state(handle.id()).is_none() => Err("not an asset".to_owned()),
                    None => continue,
                },
                _ => continue,
            };
            run.models.push(handle.clone());
            *slot = Slot::Done;
            match result {
                Ok(p) => picked.push((i, p)),
                Err(e) => run.failures.push(format!("{}: {e}", rep.model)),
            }
        }
    }
    if picked.is_empty() {
        return;
    }
    let mapper = world.get_resource::<ModelMaterials>().cloned().unwrap_or_default();
    world.resource_scope(|world, mut materials: Mut<Assets<StandardMaterial>>| {
        let mut streaming = world.get_resource_mut::<Streaming>();
        for (i, (mesh, desc, is_skinned)) in picked {
            let material = if is_skinned {
                let mapper = skinned.unwrap_or(&mapper);
                cached_material(&desc, &mut materials, &mut run.skinned_cache, (mapper, None), &server)
            } else if let Some(streaming) = streaming.as_mut() {
                streaming.template_material(&desc, &mut materials, &mapper, &server)
            } else {
                cached_material(&desc, &mut materials, &mut run.skinned_cache, (&mapper, None), &server)
            };
            let specimen = WarmSpecimen::new(mesh, material.clone(), entry_flags(&catalog.entries[i]));
            if let Some(slot) = run.slots.get_mut(i) {
                *slot = Slot::Textures(specimen, material, catalog.entries[i].representative.model.clone());
            }
        }
    });
}

/// Where a material's textures stand: `Some(true)` loaded, `Some(false)` one failed, `None` loading.
pub(crate) fn textures(material: &StandardMaterial, images: &Assets<Image>, server: &AssetServer) -> Option<bool> {
    let all = [
        &material.base_color_texture,
        &material.normal_map_texture,
        &material.emissive_texture,
        &material.metallic_roughness_texture,
        &material.occlusion_texture,
        &material.depth_map,
    ];
    let mut loaded = true;
    for handle in all.into_iter().flatten() {
        if images.contains(handle.id()) {
            continue;
        }
        // Not the server's (inserted, e.g. matclass's flat normal map, render-world only): ready.
        match server.get_load_state(handle.id()) {
            None | Some(LoadState::Loaded) => {}
            Some(LoadState::Failed(_)) => return Some(false),
            Some(_) => loaded = false,
        }
    }
    loaded.then_some(true)
}

/// Specimens whose textures are in, for the precache (a failed texture fails the specimen: its
/// material would never prepare).
fn poll_textures(world: &World, run: &mut Run) -> Vec<WarmSpecimen> {
    let (Some(materials), Some(images)) = (world.get_resource::<Assets<StandardMaterial>>(), world.get_resource::<Assets<Image>>()) else {
        return Vec::new();
    };
    let server = world.resource::<AssetServer>();
    let mut ready = Vec::new();
    for slot in &mut run.slots {
        let Slot::Textures(specimen, material, model) = slot else { continue };
        let state = materials.get(material.id()).map_or(Some(true), |m| textures(m, images, server));
        match state {
            None => continue,
            Some(true) => ready.push(specimen.clone()),
            Some(false) => run.failures.push(format!("{model}: a texture failed to load")),
        }
        *slot = Slot::Done;
    }
    ready
}

/// Builds the builtins not built yet; `NotYet` ones stay for the next hold.
fn poll_builtins(world: &mut World, run: &mut Run) -> Vec<WarmSpecimen> {
    if run.builtins.is_empty() {
        return Vec::new();
    }
    let registry = world.get_resource::<BuiltinSpecimens>().cloned().unwrap_or_default();
    let mut ready = Vec::new();
    let mut builtins = std::mem::take(&mut run.builtins);
    builtins.retain(|name| {
        let known = registry.contains(name);
        match registry.build(name, world) {
            BuiltinOutcome::Ready(specimens) => {
                log::info!("warmup: catalog builtin \"{name}\": {} specimens", specimens.len());
                run.builtin_specimens += specimens.len();
                ready.extend(specimens);
                false
            }
            BuiltinOutcome::NotYet(reason) => {
                if !run.noted.contains(name) {
                    run.noted.push(name.clone());
                    log::info!("warmup: catalog builtin \"{name}\" not yet ({reason}); retried on reveals");
                }
                true
            }
            BuiltinOutcome::Skip(reason) if known => {
                log::info!("warmup: catalog builtin \"{name}\" skipped: {reason}");
                false
            }
            BuiltinOutcome::Skip(reason) => {
                log::warn!("warmup: catalog builtin \"{name}\" skipped: {reason}");
                false
            }
        }
    });
    run.builtins = builtins;
    ready
}

/// The `warmup: catalog` line, once the representatives are in and the tag settled.
fn report(run: &mut Run, catalog: &Catalog, precache: Option<&Precache>) {
    if run.reported || run.loading() > 0 {
        return;
    }
    let status = precache.map(|p| p.status(CATALOG_TAG)).unwrap_or_default();
    if !status.is_settled() {
        return;
    }
    run.reported = true;
    let when = match (run.held, run.revealed) {
        (_, Some(at)) => format!(" (+{:.2} s after the reveal)", at.elapsed().as_secs_f32()),
        (true, None) => " (before the reveal)".to_owned(),
        (false, None) => String::new(),
    };
    log::info!(
        "warmup: catalog {} specimens ({} builtins), {} pipelines, {:.2} s{when}",
        run.kept.len(),
        run.builtin_specimens,
        status.ready + status.failed,
        run.started.elapsed().as_secs_f32()
    );
    if status.failed > 0 {
        log::warn!("warmup: catalog: {} specimens failed to specialise or compile (counted as done)", status.failed);
    }
    if let Some(first) = run.failures.first() {
        log::warn!(
            "warmup: catalog: {} of {} representatives failed to load (not waited for; first: {first})",
            run.failures.len(),
            catalog.entries.len()
        );
    }
}
