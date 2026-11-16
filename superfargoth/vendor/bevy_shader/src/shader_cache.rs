use crate::shader::*;
use alloc::borrow::Cow;
use alloc::sync::Arc;
use bevy_asset::AssetId;
use bevy_platform::collections::{HashMap, HashSet}; // superfargoth patch (own, W2a): `EntryRef` unused
use core::hash::Hash;
use thiserror::Error;
use tracing::debug;
use tracing::warn;

pub(crate) fn wesl_module_path(import_path: &ShaderImport) -> Option<wesl::syntax::ModulePath> {
    match import_path {
        ShaderImport::Custom(name) => {
            let mut segments = name.split("::");
            let package = segments.next().filter(|s| !s.is_empty())?;
            let components = segments
                .map(|s| (!s.is_empty()).then(|| s.to_string()))
                .collect::<Option<Vec<_>>>()?;
            Some(wesl::syntax::ModulePath {
                origin: wesl::syntax::PathOrigin::Package(package.to_string()),
                components,
            })
        }
        // `ModulePath::from_path` would strip anything after a `.` as an extension.
        ShaderImport::AssetPath(path) => {
            let components: Vec<String> = path
                .split('/')
                .filter(|component| !component.is_empty())
                .map(str::to_string)
                .collect();
            (!components.is_empty()).then_some(wesl::syntax::ModulePath {
                origin: wesl::syntax::PathOrigin::Absolute,
                components,
            })
        }
    }
}

fn is_module_not_found(error: &wesl::Error) -> bool {
    match error {
        wesl::Error::ResolveError(wesl::error::ResolveError::ModuleNotFound(..))
        | wesl::Error::ImportError(wesl::error::ImportError::ResolveError(
            wesl::error::ResolveError::ModuleNotFound(..),
        )) => true,
        wesl::Error::Error(diagnostic) => is_module_not_found(&diagnostic.error),
        _ => false,
    }
}

/// Fully composed source code of a shader module, with all shader defs applied.
///
/// This is roughly equivalent to [`wgpu::ShaderSource`](https://docs.rs/wgpu/latest/wgpu/enum.ShaderSource.html),
/// but with less variants and more concrete types instead of [`Cow`].
///
/// This source will be parsed and validated by the renderer.
///
/// Any necessary shader translation (e.g. from WGSL to SPIR-V or vice versa)
/// must be done internally by the renderer.
#[derive(Clone, Debug)]
pub enum ShaderCacheSource<'a> {
    /// SPIR-V module represented as a slice of words.
    SpirV(&'a [u8]),
    /// WGSL module as a string slice.
    Wgsl(String),
}

/// An id of a pipeline, typically in the [`PipelineCache`](https://docs.rs/bevy/latest/bevy/render/render_resource/struct.PipelineCache.html)
/// Typically corresponds to a unique combination of [`Shader`] and [`ShaderDefVal`]s.
pub type CachedPipelineId = usize;

struct ShaderData<ShaderModule> {
    pipelines: HashSet<CachedPipelineId>,
    processed_shaders: HashMap<Box<[ShaderDefVal]>, Arc<ShaderModule>>,
    resolved_imports: HashMap<ShaderImport, AssetId<Shader>>,
    dependents: HashSet<AssetId<Shader>>,
}

impl<T> Default for ShaderData<T> {
    fn default() -> Self {
        Self {
            pipelines: Default::default(),
            processed_shaders: Default::default(),
            resolved_imports: Default::default(),
            dependents: Default::default(),
        }
    }
}

/// A cache for shaders and shader imports, with asset state-tracking for
/// waiting to load shaders until all imports are resolved.
///
/// Note that the `RenderDevice` generic parameter is a means by which
/// to avoid a cyclic dependency with `bevy_render`, while also permitting
/// alternative rendering implementations. The actual processing of the
/// shader source into a usable compiled module is left to the renderer.
pub struct ShaderCache<ShaderModule, RenderDevice> {
    device: RenderDevice,
    data: HashMap<AssetId<Shader>, ShaderData<ShaderModule>>,
    load_module: fn(
        &RenderDevice,
        ShaderCacheSource,
        &ValidateShader,
    ) -> Result<ShaderModule, ShaderCacheError>,
    // superfargoth patch (own, W2a: front end outside the lock): `Arc` snapshots for `ShaderJob`s
    // (copy-on-write), the change epoch of each shader, and the modules being composed.
    module_path_to_asset_id: Arc<HashMap<wesl::syntax::ModulePath, AssetId<Shader>>>,
    shaders: Arc<HashMap<AssetId<Shader>, Shader>>,
    epoch: u64,
    changed_at: HashMap<AssetId<Shader>, u64>,
    in_flight: HashSet<(AssetId<Shader>, Box<[ShaderDefVal]>)>,
    import_path_shaders: HashMap<ShaderImport, AssetId<Shader>>,
    waiting_on_import: HashMap<ShaderImport, Vec<AssetId<Shader>>>,
    missing_import_logged: HashSet<AssetId<Shader>>,
}

/// A compile time shader value definition to be inlined into the shader source.
/// Variant tuples contain the name of the definition, and the value.
#[expect(missing_docs, reason = "Enum variants are self-explanatory")]
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Debug, Hash)]
pub enum ShaderDefVal {
    Bool(Cow<'static, str>, bool),
    Int(Cow<'static, str>, i32),
    UInt(Cow<'static, str>, u32),
}

impl From<&'static str> for ShaderDefVal {
    fn from(key: &'static str) -> Self {
        ShaderDefVal::Bool(key.into(), true)
    }
}

impl From<String> for ShaderDefVal {
    fn from(key: String) -> Self {
        ShaderDefVal::Bool(key.into(), true)
    }
}

impl ShaderDefVal {
    /// Returns the value of the define as a string.
    pub fn value_as_string(&self) -> String {
        match self {
            ShaderDefVal::Bool(_, def) => def.to_string(),
            ShaderDefVal::Int(_, def) => def.to_string(),
            ShaderDefVal::UInt(_, def) => def.to_string(),
        }
    }
}

impl<ShaderModule, RenderDevice> ShaderCache<ShaderModule, RenderDevice> {
    /// Creates a new [`ShaderCache`] with the given shader module loading
    /// function. `load_module` is responsible for actually compiling shader
    /// source into a module usable by the render device.
    pub fn new(
        device: RenderDevice,
        load_module: fn(
            &RenderDevice,
            ShaderCacheSource,
            &ValidateShader,
        ) -> Result<ShaderModule, ShaderCacheError>,
    ) -> Self {
        Self {
            device,
            load_module,
            data: Default::default(),
            module_path_to_asset_id: Default::default(),
            shaders: Default::default(),
            import_path_shaders: Default::default(),
            waiting_on_import: Default::default(),
            missing_import_logged: Default::default(),
            epoch: 0,                    // superfargoth patch (own, W2a)
            changed_at: Default::default(), // superfargoth patch (own, W2a)
            in_flight: Default::default(),  // superfargoth patch (own, W2a)
        }
    }

    /// Attempts to retrieve or create a compiled shader module for the given
    /// shader id and shader definitions.
    ///
    /// The provided `pipeline` is tracked so it may later be reported "dirty"
    /// when a shader is removed or replaced.
    ///
    /// Note that the cache is keyed by `id` and `shader_defs`, meaning providing
    /// the same `shader_defs` in a different order, or with redundancies, will
    /// not result in cache hits, and thus require re-composing the module and
    /// calling `load_module` again.
    // superfargoth patch (own, W2a: front end outside the lock), pipeline-warmup.md W2: `get` is
    // `prepare` + `ShaderJob::compile` + `finish`; callers holding the cache behind a lock call the
    // three themselves and compose outside it (bevyengine/bevy#26152).
    pub fn get(
        &mut self,
        pipeline: CachedPipelineId,
        id: AssetId<Shader>,
        shader_defs: &[ShaderDefVal],
    ) -> Result<Arc<ShaderModule>, ShaderCacheError> {
        match self.prepare(pipeline, id, shader_defs)? {
            ShaderLookup::Ready(module) => Ok(module),
            ShaderLookup::Compile(job) => {
                let output = job.compile(&self.device);
                self.finish(output)
            }
        }
    }

    // superfargoth patch (own, W2a: front end outside the lock)
    /// First step of [`Self::get`]: the cached module, or a [`ShaderJob`] that composes and
    /// creates it without borrowing the cache (run it outside any lock, then hand its output to
    /// [`Self::finish`], or the job to [`Self::cancel`]). A module another job is already
    /// composing returns [`ShaderCacheError::ShaderImportNotYetAvailable`] (retry later: it is a
    /// cache hit once that job finishes).
    pub fn prepare(
        &mut self,
        pipeline: CachedPipelineId,
        id: AssetId<Shader>,
        shader_defs: &[ShaderDefVal],
    ) -> Result<ShaderLookup<ShaderModule, RenderDevice>, ShaderCacheError> {
        let shader = self
            .shaders
            .get(&id)
            .ok_or(ShaderCacheError::ShaderNotLoaded(id))?;

        let data = self.data.entry(id).or_default();

        // Wesl imports are scanned as both module and item candidates.
        if !matches!(shader.source, Source::Wesl(_)) {
            let n_asset_imports = shader
                .imports
                .iter()
                .filter(|import| matches!(import, ShaderImport::AssetPath(_)))
                .count();
            let n_resolved_asset_imports = data
                .resolved_imports
                .keys()
                .filter(|import| matches!(import, ShaderImport::AssetPath(_)))
                .count();
            if n_asset_imports != n_resolved_asset_imports {
                return Err(ShaderCacheError::ShaderImportNotYetAvailable);
            }
        }

        data.pipelines.insert(pipeline);

        if let Some(module) = data.processed_shaders.get(shader_defs) {
            return Ok(ShaderLookup::Ready(module.clone()));
        }

        let wesl = match &shader.source {
            Source::Wesl(_) => {
                let Some(module_path) = wesl_module_path(&shader.import_path) else {
                    return Err(ShaderCacheError::ProcessShaderError(format!(
                        "Wesl shader `{}` has a malformed import path `{:?}`",
                        shader.path, shader.import_path
                    )));
                };
                let mut compiler_options = wesl::CompileOptions {
                    imports: true,
                    condcomp: true,
                    visibility: false,
                    ..Default::default()
                };

                let mut closure_defs = Vec::new();
                let mut visited = HashSet::new();
                let mut to_visit = vec![id];
                while let Some(dep_id) = to_visit.pop() {
                    if !visited.insert(dep_id) {
                        continue;
                    }
                    let Some(dep) = self.shaders.get(&dep_id) else {
                        continue;
                    };
                    if dep_id != id {
                        closure_defs.extend(dep.shader_defs.iter());
                    }
                    for import in &dep.imports {
                        if let Some(import_id) = self.import_path_shaders.get(import) {
                            to_visit.push(*import_id);
                        }
                    }
                }
                let mut constants = alloc::collections::BTreeMap::new();
                for shader_def in closure_defs
                    .into_iter()
                    .chain(shader_defs.iter())
                    .chain(shader.shader_defs.iter())
                {
                    match shader_def {
                        ShaderDefVal::Bool(key, value) => {
                            compiler_options
                                .features
                                .flags
                                .insert(key.to_string(), (*value).into());
                        }
                        ShaderDefVal::Int(key, value) => {
                            compiler_options
                                .features
                                .flags
                                .insert(key.to_string(), true.into());
                            constants.insert(key.as_ref(), value.to_string());
                        }
                        ShaderDefVal::UInt(key, value) => {
                            compiler_options
                                .features
                                .flags
                                .insert(key.to_string(), true.into());
                            constants.insert(key.as_ref(), value.to_string());
                        }
                    }
                }
                let constants_source: String = constants
                    .iter()
                    .map(|(name, value)| format!("const {name} = {value};\n"))
                    .collect();
                Some(WeslJob {
                    module_path,
                    compiler_options,
                    constants_source,
                })
            }
            Source::Wgsl(_) | Source::SpirV(_) => None,
        };

        let shader_defs: Box<[ShaderDefVal]> = shader_defs.into();
        if !self.in_flight.insert((id, shader_defs.clone())) {
            return Err(ShaderCacheError::ShaderImportNotYetAvailable);
        }
        debug!("processing shader {}, with shader defs {:?}", id, shader_defs);
        Ok(ShaderLookup::Compile(ShaderJob {
            id,
            shader_defs,
            epoch: self.epoch,
            load_module: self.load_module,
            shaders: self.shaders.clone(),
            module_path_to_asset_id: self.module_path_to_asset_id.clone(),
            wesl,
        }))
    }

    // superfargoth patch (own, W2a: front end outside the lock)
    /// Last step of [`Self::get`]: stores a finished job's module (so later lookups hit) and
    /// returns it. A module whose shader or one of its imports changed while it was composed is
    /// dropped as stale and returns [`ShaderCacheError::ShaderImportNotYetAvailable`] (retry).
    pub fn finish(
        &mut self,
        output: ShaderJobOutput<ShaderModule>,
    ) -> Result<Arc<ShaderModule>, ShaderCacheError> {
        let ShaderJobOutput {
            id,
            shader_defs,
            epoch,
            module,
            dependencies,
            missing_import,
        } = output;
        let key = (id, shader_defs);
        self.in_flight.remove(&key);
        if let Some(message) = missing_import
            && self.missing_import_logged.insert(id)
        {
            warn!("{message}");
        }
        let module = module?;
        let stale = core::iter::once(&id)
            .chain(&dependencies)
            .any(|shader| self.changed_at.get(shader).is_some_and(|&at| at > epoch));
        if stale {
            return Err(ShaderCacheError::ShaderImportNotYetAvailable);
        }
        let module = self
            .data
            .entry(id)
            .or_default()
            .processed_shaders
            .entry(key.1)
            .or_insert_with(|| Arc::new(module))
            .clone();
        for dep_id in dependencies {
            self.data.entry(dep_id).or_default().dependents.insert(id);
        }
        Ok(module)
    }

    // superfargoth patch (own, W2a: front end outside the lock)
    /// Drops a job from [`Self::prepare`] that will not run (another stage of the same pipeline
    /// failed), so the module is not left marked as being composed.
    pub fn cancel(&mut self, job: ShaderJob<ShaderModule, RenderDevice>) {
        self.in_flight.remove(&(job.id, job.shader_defs));
    }

    // superfargoth patch (own, W2a: front end outside the lock)
    /// Modules being composed by a [`ShaderJob`] right now.
    pub fn in_flight(&self) -> usize {
        self.in_flight.len()
    }

    fn clear(&mut self, id: AssetId<Shader>) -> Vec<CachedPipelineId> {
        let mut shaders_to_clear = vec![id];
        let mut visited = HashSet::new();
        let mut pipelines_to_queue = Vec::new();
        while let Some(handle) = shaders_to_clear.pop() {
            if !visited.insert(handle) {
                continue;
            }
            self.changed_at.insert(handle, self.epoch); // superfargoth patch (own, W2a): jobs older are stale
            if let Some(data) = self.data.get_mut(&handle) {
                data.processed_shaders.clear();
                pipelines_to_queue.extend(data.pipelines.iter().copied());
                shaders_to_clear.extend(data.dependents.iter().copied());
            }
        }

        pipelines_to_queue
    }

    /// Inserts and possibly replaces a shader at the given asset id.
    ///
    /// Returns a vec of which cached pipelines depended on it
    /// (directly or indirectly via a shader import) and thus must be recompiled.
    pub fn set_shader(&mut self, id: AssetId<Shader>, shader: Shader) -> Vec<CachedPipelineId> {
        self.epoch += 1; // superfargoth patch (own, W2a)
        self.missing_import_logged.clear();
        let mut pipelines_to_queue = self.clear(id);
        let path = &shader.import_path;
        self.import_path_shaders.insert(path.clone(), id);
        if let Some(waiting_shaders) = self.waiting_on_import.get_mut(path) {
            for waiting_shader in waiting_shaders.drain(..) {
                // resolve waiting shader import
                let data = self.data.entry(waiting_shader).or_default();
                data.resolved_imports.insert(path.clone(), id);
                // add waiting shader as dependent of this shader
                let data = self.data.entry(id).or_default();
                data.dependents.insert(waiting_shader);
            }
        }

        for import in shader.imports.iter() {
            if let Some(import_id) = self.import_path_shaders.get(import).copied() {
                // resolve import because it is currently available
                let data = self.data.entry(id).or_default();
                data.resolved_imports.insert(import.clone(), import_id);
                // add this shader as a dependent of the import
                let data = self.data.entry(import_id).or_default();
                data.dependents.insert(id);
            } else {
                let waiting = self.waiting_on_import.entry(import.clone()).or_default();
                waiting.push(id);
            }
        }

        if let Source::Wesl(_) = shader.source {
            match wesl_module_path(&shader.import_path) {
                Some(module_path) => {
                    if let Some(existing) = self.module_path_to_asset_id.get(&module_path).copied()
                        && existing != id
                    {
                        warn!(
                            "Shader module path `{module_path}` was registered to a different \
                            shader, replacing with `{}`",
                            shader.path
                        );
                        pipelines_to_queue.extend(self.clear(existing));
                    }
                    Arc::make_mut(&mut self.module_path_to_asset_id).insert(module_path, id); // superfargoth patch (own, W2a)
                }
                None => warn!(
                    "Shader `{}` has a malformed import path `{:?}`",
                    shader.path, shader.import_path
                ),
            }
        }
        Arc::make_mut(&mut self.shaders).insert(id, shader); // superfargoth patch (own, W2a)
        pipelines_to_queue
    }

    /// Removes the shader with the given asset id.
    ///
    /// Returns a vec of which cached pipelines depended on it
    /// (directly or indirectly via a shader import) and thus must be recompiled.
    pub fn remove(&mut self, id: AssetId<Shader>) -> Vec<CachedPipelineId> {
        self.epoch += 1; // superfargoth patch (own, W2a)
        let pipelines_to_queue = self.clear(id);
        if let Some(shader) = Arc::make_mut(&mut self.shaders).remove(&id) { // superfargoth patch (own, W2a)
            if self.import_path_shaders.get(&shader.import_path) == Some(&id) {
                self.import_path_shaders.remove(&shader.import_path);
            }
            if let Source::Wesl(_) = shader.source
                && let Some(module_path) = wesl_module_path(&shader.import_path)
                && self.module_path_to_asset_id.get(&module_path) == Some(&id)
            {
                Arc::make_mut(&mut self.module_path_to_asset_id).remove(&module_path); // superfargoth patch (own, W2a)
            }
        }

        pipelines_to_queue
    }
}

// superfargoth patch (own, W2a: front end outside the lock), pipeline-warmup.md W2
/// What [`ShaderCache::prepare`] found: the cached module, or the job that makes it.
pub enum ShaderLookup<ShaderModule, RenderDevice> {
    /// Cache hit.
    Ready(Arc<ShaderModule>),
    /// Cache miss: run [`ShaderJob::compile`], then [`ShaderCache::finish`].
    Compile(ShaderJob<ShaderModule, RenderDevice>),
}

/// WESL composition inputs gathered under the cache borrow.
struct WeslJob {
    module_path: wesl::syntax::ModulePath,
    compiler_options: wesl::CompileOptions,
    constants_source: String,
}

/// One module to compose (WESL) and create (`load_module`), owning everything it reads: the
/// shader sources are a snapshot (`Arc`, copy-on-write in the cache), so it runs without the
/// cache and many run at once.
pub struct ShaderJob<ShaderModule, RenderDevice> {
    id: AssetId<Shader>,
    shader_defs: Box<[ShaderDefVal]>,
    epoch: u64,
    load_module:
        fn(&RenderDevice, ShaderCacheSource, &ValidateShader) -> Result<ShaderModule, ShaderCacheError>,
    shaders: Arc<HashMap<AssetId<Shader>, Shader>>,
    module_path_to_asset_id: Arc<HashMap<wesl::syntax::ModulePath, AssetId<Shader>>>,
    wesl: Option<WeslJob>,
}

/// A finished [`ShaderJob`], to hand back to [`ShaderCache::finish`].
pub struct ShaderJobOutput<ShaderModule> {
    id: AssetId<Shader>,
    shader_defs: Box<[ShaderDefVal]>,
    epoch: u64,
    module: Result<ShaderModule, ShaderCacheError>,
    dependencies: Vec<AssetId<Shader>>,
    missing_import: Option<String>,
}

impl<ShaderModule, RenderDevice> ShaderJob<ShaderModule, RenderDevice> {
    /// Composes the shader and creates its module (the cache is not needed: call it outside the
    /// lock guarding the cache).
    pub fn compile(self, device: &RenderDevice) -> ShaderJobOutput<ShaderModule> {
        let mut dependencies = Vec::new();
        let mut missing_import = None;
        let module = match self.shaders.get(&self.id) {
            None => Err(ShaderCacheError::ShaderNotLoaded(self.id)),
            Some(shader) => {
                let source = match (&shader.source, &self.wesl) {
                    (Source::SpirV(data), _) => Ok(ShaderCacheSource::SpirV(data.as_ref())),
                    (Source::Wgsl(source), _) => Ok(ShaderCacheSource::Wgsl(source.to_string())),
                    (Source::Wesl(_), None) => Err(ShaderCacheError::ProcessShaderError(format!(
                        "Wesl shader `{}` has a malformed import path `{:?}`",
                        shader.path, shader.import_path
                    ))),
                    (Source::Wesl(_), Some(wesl_job)) => {
                        let shader_resolver = ShaderResolver::new(
                            &self.module_path_to_asset_id,
                            &self.shaders,
                            &wesl_job.constants_source,
                        );
                        match wesl::compile(
                            &wesl_job.module_path,
                            &wesl_job.compiler_options,
                            &shader_resolver,
                        ) {
                            Err(error) => {
                                // We use render_plain to avoid rendering ANSI codes
                                // Workaround for: https://github.com/tokio-rs/tracing/issues/3378
                                if is_module_not_found(&error) {
                                    missing_import = Some(format!(
                                        "Shader `{}` has an unresolved import:\n{}",
                                        shader.path,
                                        error.diagnostic().render_plain()
                                    ));
                                    Err(ShaderCacheError::ShaderImportNotYetAvailable)
                                } else {
                                    Err(ShaderCacheError::ProcessShaderError(
                                        error.diagnostic().render_plain(),
                                    ))
                                }
                            }
                            Ok(compiled) => {
                                for used in &compiled.modules {
                                    let used_path = match &used.path.origin {
                                        wesl::syntax::PathOrigin::Package(pkg)
                                            if pkg.contains('/') =>
                                        {
                                            Cow::Owned(wesl::syntax::ModulePath {
                                                origin: wesl::syntax::PathOrigin::Package(
                                                    pkg.rsplit('/').next().unwrap().to_string(),
                                                ),
                                                components: used.path.components.clone(),
                                            })
                                        }
                                        _ => Cow::Borrowed(&used.path),
                                    };
                                    if let Some(dep_id) =
                                        self.module_path_to_asset_id.get(used_path.as_ref())
                                        && *dep_id != self.id
                                    {
                                        dependencies.push(*dep_id);
                                    }
                                }
                                Ok(ShaderCacheSource::Wgsl(compiled.to_string()))
                            }
                        }
                    }
                };
                source.and_then(|source| (self.load_module)(device, source, &shader.validate_shader))
            }
        };
        ShaderJobOutput {
            id: self.id,
            shader_defs: self.shader_defs,
            epoch: self.epoch,
            module,
            dependencies,
            missing_import,
        }
    }
}
// end superfargoth patch (own, W2a: front end outside the lock)

/// A Wesl import resolver. Maps module paths to actual Wesl shader source.
pub struct ShaderResolver<'a> {
    module_path_to_asset_id: &'a HashMap<wesl::syntax::ModulePath, AssetId<Shader>>,
    shaders: &'a HashMap<AssetId<Shader>, Shader>,
    constants_source: &'a str,
}

impl<'a> ShaderResolver<'a> {
    /// Creates a shader resolver with the given map of module paths to shader asset ids,
    /// map of shader asset ids to shader source, and the source of the virtual
    /// `constants` module. This resolver is not meant to be long living.
    pub fn new(
        module_path_to_asset_id: &'a HashMap<wesl::syntax::ModulePath, AssetId<Shader>>,
        shaders: &'a HashMap<AssetId<Shader>, Shader>,
        constants_source: &'a str,
    ) -> Self {
        Self {
            module_path_to_asset_id,
            shaders,
            constants_source,
        }
    }
}

impl<'a> wesl::Resolver for ShaderResolver<'a> {
    fn resolve_source(
        &self,
        module_path: &wesl::syntax::ModulePath,
    ) -> Result<Cow<'_, str>, wesl::error::ResolveError> {
        let module_path = self.canonical_path(module_path);
        if module_path.origin == wesl::syntax::PathOrigin::Package("constants".to_string())
            && module_path.components.is_empty()
        {
            return Ok(Cow::Borrowed(self.constants_source));
        }
        let asset_id = self
            .module_path_to_asset_id
            .get(&module_path)
            .ok_or_else(|| {
                wesl::error::ResolveError::ModuleNotFound(
                    module_path.clone(),
                    "no shader registered for this module path".to_string(),
                )
            })?;

        let shader = self.shaders.get(asset_id).ok_or_else(|| {
            wesl::error::ResolveError::ModuleNotFound(
                module_path.clone(),
                "shader asset not loaded".to_string(),
            )
        })?;
        Ok(Cow::Borrowed(shader.source.as_str()))
    }

    fn canonical_path(&self, module_path: &wesl::syntax::ModulePath) -> wesl::syntax::ModulePath {
        match &module_path.origin {
            wesl::syntax::PathOrigin::Package(pkg) if pkg.contains('/') => {
                wesl::syntax::ModulePath {
                    origin: wesl::syntax::PathOrigin::Package(
                        pkg.rsplit('/').next().unwrap().to_string(),
                    ),
                    components: module_path.components.clone(),
                }
            }
            _ => module_path.clone(),
        }
    }

    fn display_name(&self, module_path: &wesl::syntax::ModulePath) -> Option<String> {
        let module_path = self.canonical_path(module_path);
        let asset_id = self.module_path_to_asset_id.get(&module_path)?;
        let shader = self.shaders.get(asset_id)?;
        Some(shader.path.clone())
    }
}

/// Type of error returned by a `PipelineCache` when the creation of a GPU pipeline object failed.
#[expect(missing_docs, reason = "Enum variants are self-explanatory")]
#[derive(Error, Debug)]
pub enum ShaderCacheError {
    #[error(
        "Pipeline could not be compiled because the following shader could not be loaded: {0:?}"
    )]
    ShaderNotLoaded(AssetId<Shader>),
    #[error("Failed to process shader:\n{0}")]
    ProcessShaderError(String),
    #[error("Shader import not yet available.")]
    ShaderImportNotYetAvailable,
    #[error("Could not create shader module: {0}")]
    CreateShaderModule(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_cache() -> ShaderCache<String, ()> {
        ShaderCache::new((), |_, source, _| match source {
            ShaderCacheSource::Wgsl(wgsl) => Ok(wgsl),
            _ => panic!("expected wgsl output"),
        })
    }

    #[test]
    fn import_resolution() {
        let mut cache = test_cache();

        let (maths_id, lighting_id, root_id) = set_test_shaders(&mut cache);

        let compiled = cache
            .get(0, root_id, &[ShaderDefVal::Bool("BRIGHT".into(), true)])
            .unwrap();
        assert!(compiled.contains("fn fragment"));
        assert!(compiled.contains("* 2.0"));
        assert!(compiled.contains("+ 0.1"));

        let (maths, lighting, _) = test_shaders();
        assert!(cache.set_shader(lighting_id, lighting).contains(&0));
        let _ = cache.get(0, root_id, &[ShaderDefVal::Bool("BRIGHT".into(), true)]);
        assert!(cache.set_shader(maths_id, maths).contains(&0));
    }

    #[test]
    fn constants_module() {
        let mut cache = test_cache();
        let mut shader = Shader::from_wesl(
            r#"
@group(constants::MATERIAL_BIND_GROUP) @binding(0) var<uniform> scale: f32;
var<uniform> batch: array<vec4<f32>, constants::BATCH_SIZE>;

@fragment
fn fragment() -> @location(0) vec4<f32> {
    return vec4<f32>(scale) + batch[0];
}
"#,
            "shaders/constants_user.wesl",
        );
        shader.shader_defs = vec![ShaderDefVal::UInt("BATCH_SIZE".into(), 4)];
        let id = AssetId::Uuid {
            uuid: bevy_asset::uuid::Uuid::from_u128(7),
        };
        cache.set_shader(id, shader);

        let compiled = cache
            .get(
                0,
                id,
                &[ShaderDefVal::UInt("MATERIAL_BIND_GROUP".into(), 2)],
            )
            .unwrap();
        assert!(compiled.contains("= 2;"));
        assert!(compiled.contains("= 4;"));
    }

    #[test]
    fn compile_error_display_names() {
        let mut cache = test_cache();
        let (maths, _, root) = test_shaders();
        let (maths_id, lighting_id, root_id) = test_ids();
        let broken_lighting = Shader::from_wesl(
            "fn brighten(x: f32) -> f32 { return x + ; }",
            "embedded://bevy_pbr/render/lighting.wesl",
        );
        cache.set_shader(maths_id, maths);
        cache.set_shader(lighting_id, broken_lighting);
        cache.set_shader(root_id, root);

        let error = cache
            .get(0, root_id, &[ShaderDefVal::Bool("BRIGHT".into(), true)])
            .expect_err("syntax error");
        let ShaderCacheError::ProcessShaderError(message) = error else {
            panic!("expected ProcessShaderError, got: {error:?}");
        };
        assert!(message.contains("embedded://bevy_pbr/render/lighting.wesl"));
    }

    #[test]
    fn import_retry() {
        let mut cache = test_cache();
        let (maths, lighting, root) = test_shaders();
        let (maths_id, lighting_id, root_id) = test_ids();

        cache.set_shader(root_id, root);
        assert!(matches!(
            cache.get(0, root_id, &[]),
            Err(ShaderCacheError::ShaderImportNotYetAvailable)
        ));

        cache.set_shader(lighting_id, lighting);
        cache.set_shader(maths_id, maths);
        cache.get(0, root_id, &[]).unwrap();
    }

    fn test_shaders() -> (Shader, Shader, Shader) {
        let maths = Shader::from_wesl(
            "fn double(x: f32) -> f32 { return x * 2.0; }",
            "embedded://bevy_render/maths.wesl",
        );
        let lighting = Shader::from_wesl(
            "fn brighten(x: f32) -> f32 { return x + 0.1; }",
            "embedded://bevy_pbr/render/lighting.wesl",
        );
        let root = Shader::from_wesl(
            r#"
import bevy_pbr::render::lighting::brighten;

@fragment
fn fragment() -> @location(0) vec4<f32> {
    var value = bevy_render::maths::double(1.0);
    @if(BRIGHT) { value = brighten(value); }
    return vec4<f32>(value);
}
"#,
            "shaders/root.wesl",
        );
        (maths, lighting, root)
    }

    fn test_ids() -> (AssetId<Shader>, AssetId<Shader>, AssetId<Shader>) {
        let id = |n| AssetId::Uuid {
            uuid: bevy_asset::uuid::Uuid::from_u128(n),
        };
        (id(1), id(2), id(3))
    }

    #[test]
    fn library_def_scoping() {
        let mut cache = test_cache();
        let id = |n| AssetId::Uuid {
            uuid: bevy_asset::uuid::Uuid::from_u128(n),
        };

        let mut lib_a = Shader::from_wesl(
            "var<uniform> batch_a: array<vec4<f32>, constants::BATCH_SIZE>;",
            "embedded://bevy_a/bindings.wesl",
        );
        lib_a.shader_defs = vec![ShaderDefVal::UInt("BATCH_SIZE".into(), 3)];
        let mut lib_b = Shader::from_wesl(
            "var<uniform> batch_b: array<vec4<f32>, constants::BATCH_SIZE>;",
            "embedded://bevy_b/bindings.wesl",
        );
        lib_b.shader_defs = vec![ShaderDefVal::UInt("BATCH_SIZE".into(), 7)];

        let root_a = Shader::from_wesl(
            r#"
import bevy_a::bindings::batch_a;
@fragment
fn fragment() -> @location(0) vec4<f32> { return batch_a[0]; }
"#,
            "shaders/root_a.wesl",
        );
        let root_b = Shader::from_wesl(
            r#"
import bevy_b::bindings::batch_b;
@fragment
fn fragment() -> @location(0) vec4<f32> { return batch_b[0]; }
"#,
            "shaders/root_b.wesl",
        );

        cache.set_shader(id(1), lib_a);
        cache.set_shader(id(2), lib_b);
        cache.set_shader(id(3), root_a);
        cache.set_shader(id(4), root_b);

        let compiled_a = cache.get(0, id(3), &[]).unwrap();
        let compiled_b = cache.get(1, id(4), &[]).unwrap();
        assert!(compiled_a.contains("= 3;") && !compiled_a.contains("= 7;"));
        assert!(compiled_b.contains("= 7;") && !compiled_b.contains("= 3;"));
    }

    #[test]
    fn cyclic_import_invalidation() {
        let mut cache = test_cache();
        let id = |n| AssetId::Uuid {
            uuid: bevy_asset::uuid::Uuid::from_u128(n),
        };
        let module_a = Shader::from_wesl(
            "import bevy_cycle::b::from_b;\nfn from_a() -> f32 { return 1.0; }",
            "embedded://bevy_cycle/a.wesl",
        );
        let module_b = Shader::from_wesl(
            "import bevy_cycle::a::from_a;\nfn from_b() -> f32 { return 2.0; }",
            "embedded://bevy_cycle/b.wesl",
        );
        cache.set_shader(id(1), module_a);
        cache.set_shader(id(2), module_b);
        let module_a = Shader::from_wesl(
            "import bevy_cycle::b::from_b;\nfn from_a() -> f32 { return 3.0; }",
            "embedded://bevy_cycle/a.wesl",
        );
        cache.set_shader(id(1), module_a);
    }

    fn set_test_shaders(
        cache: &mut ShaderCache<String, ()>,
    ) -> (AssetId<Shader>, AssetId<Shader>, AssetId<Shader>) {
        let (maths, lighting, root) = test_shaders();
        let (maths_id, lighting_id, root_id) = test_ids();
        cache.set_shader(maths_id, maths);
        cache.set_shader(lighting_id, lighting);
        cache.set_shader(root_id, root);
        (maths_id, lighting_id, root_id)
    }
}
