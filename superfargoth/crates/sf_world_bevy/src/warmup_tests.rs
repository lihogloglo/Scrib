//! Pipeline warm-up W2 (pipeline-warmup.md §5.7): the vendored `bevy_shader` front-end split
//! (`superfargoth patch (own, W2a)`: `ShaderCache::{prepare, finish, cancel}` + `ShaderJob::compile`)
//! and the compile pool size. Exact, GPU-free: modules are the composed WGSL strings.

use std::sync::{Arc, Mutex};

use bevy_asset::AssetId;
use bevy_shader::{Shader, ShaderCache, ShaderCacheError, ShaderCacheSource, ShaderDefVal, ShaderLookup};

use crate::compile_pool_threads;

type Cache = ShaderCache<String, ()>;

fn cache() -> Cache {
    ShaderCache::new((), |(), source, _| match source {
        ShaderCacheSource::Wgsl(wgsl) => Ok(wgsl),
        ShaderCacheSource::SpirV(_) => Err(ShaderCacheError::CreateShaderModule("spir-v".into())),
    })
}

const MATHS: AssetId<Shader> = AssetId::Uuid { uuid: bevy_asset::uuid::Uuid::from_u128(1) };
const ROOT: AssetId<Shader> = AssetId::Uuid { uuid: bevy_asset::uuid::Uuid::from_u128(3) };

fn maths(factor: &str) -> Shader {
    Shader::from_wesl(format!("fn double(x: f32) -> f32 {{ return x * {factor}; }}"), "embedded://bevy_render/maths.wesl")
}

fn root() -> Shader {
    Shader::from_wesl(
        "@fragment\nfn fragment() -> @location(0) vec4<f32> {\n    var value = bevy_render::maths::double(1.0);\n    @if(BRIGHT) { value = value + 0.5; }\n    return vec4<f32>(value);\n}\n",
        "shaders/root.wesl",
    )
}

fn loaded() -> Cache {
    let mut cache = cache();
    cache.set_shader(MATHS, maths("2.0"));
    cache.set_shader(ROOT, root());
    cache
}

fn defs(bright: bool) -> Vec<ShaderDefVal> {
    vec![ShaderDefVal::Bool("BRIGHT".into(), bright)]
}

fn job(cache: &mut Cache, pipeline: usize, shader_defs: &[ShaderDefVal]) -> bevy_shader::ShaderJob<String, ()> {
    match cache.prepare(pipeline, ROOT, shader_defs) {
        Ok(ShaderLookup::Compile(job)) => job,
        Ok(ShaderLookup::Ready(_)) => panic!("expected a miss"),
        Err(err) => panic!("prepare failed: {err}"),
    }
}

fn is_retry<T>(result: &Result<T, ShaderCacheError>) -> bool {
    matches!(result, Err(ShaderCacheError::ShaderImportNotYetAvailable))
}

/// `get` (prepare + compile + finish in one borrow) composes as stock did, then hits.
#[test]
fn get_composes_then_hits() {
    let mut cache = loaded();
    let first = cache.get(0, ROOT, &defs(true)).unwrap();
    assert!(first.contains("* 2.0") && first.contains("+ 0.5"), "{first}");
    let again = cache.get(1, ROOT, &defs(true)).unwrap();
    assert!(Arc::ptr_eq(&first, &again), "second lookup is a cache hit");
    let dim = cache.get(2, ROOT, &defs(false)).unwrap();
    assert!(!dim.contains("+ 0.5"), "other defs, other module");
}

/// A job composes without the cache: many run at once on other threads while the cache stays
/// usable (the lock is free), and the finished modules hit afterwards.
#[test]
fn jobs_compile_outside_the_cache_in_parallel() {
    let cache = Mutex::new(loaded());
    let jobs: Vec<_> = (0_u8..8).map(|n| {
        let shader_defs = vec![ShaderDefVal::Int("VARIANT".into(), i32::from(n))];
        (job(&mut cache.lock().unwrap(), usize::from(n), &shader_defs), shader_defs)
    }).collect();
    assert_eq!(cache.lock().unwrap().in_flight(), 8);
    let outputs: Vec<_> = std::thread::scope(|s| {
        let handles: Vec<_> = jobs.into_iter().map(|(job, shader_defs)| s.spawn(move || (job.compile(&()), shader_defs))).collect();
        // The cache is not locked while the jobs run.
        assert!(cache.try_lock().is_ok());
        handles.into_iter().map(|h| h.join().unwrap()).collect()
    });
    let mut cache = cache.into_inner().unwrap();
    for (output, shader_defs) in outputs {
        let module = cache.finish(output).unwrap();
        assert!(module.contains("* 2.0"));
        assert!(matches!(cache.prepare(99, ROOT, &shader_defs), Ok(ShaderLookup::Ready(m)) if Arc::ptr_eq(&m, &module)));
    }
    assert_eq!(cache.in_flight(), 0);
}

/// A module already being composed is not composed twice: the second lookup says "retry" (the
/// pipeline cache re-queues it) and hits once the first job finishes.
#[test]
fn module_in_flight_is_retried_not_duplicated() {
    let mut cache = loaded();
    let first = job(&mut cache, 0, &defs(true));
    assert!(is_retry(&cache.prepare(1, ROOT, &defs(true)).map(|_| ())));
    assert!(matches!(cache.prepare(2, ROOT, &defs(false)), Ok(ShaderLookup::Compile(_))), "other defs are another module");
    let module = cache.finish(first.compile(&())).unwrap();
    assert!(matches!(cache.prepare(1, ROOT, &defs(true)), Ok(ShaderLookup::Ready(m)) if Arc::ptr_eq(&m, &module)));
}

/// A cancelled job (another stage of its pipeline failed) frees the module for the next lookup.
#[test]
fn cancel_frees_the_module() {
    let mut cache = loaded();
    let first = job(&mut cache, 0, &defs(true));
    cache.cancel(first);
    assert_eq!(cache.in_flight(), 0);
    assert!(matches!(cache.prepare(1, ROOT, &defs(true)), Ok(ShaderLookup::Compile(_))));
}

/// A module composed from sources that changed meanwhile (hot reload of the shader or an import)
/// is dropped as stale ("retry"); the retry composes the new source.
#[test]
fn stale_job_is_dropped_after_a_shader_change() {
    for changed in [MATHS, ROOT] {
        let mut cache = loaded();
        let old = job(&mut cache, 0, &defs(true));
        let requeued = if changed == MATHS { cache.set_shader(MATHS, maths("3.0")) } else { cache.set_shader(ROOT, root()) };
        let output = old.compile(&());
        assert!(is_retry(&cache.finish(output)), "{changed:?}: stale module dropped");
        if changed == ROOT {
            assert!(requeued.contains(&0), "the pipeline is re-queued by the change");
        }
        let fresh = cache.get(0, ROOT, &defs(true)).unwrap();
        let factor = if changed == MATHS { "* 3.0" } else { "* 2.0" };
        assert!(fresh.contains(factor), "{changed:?}: {fresh}");
    }
}

/// A module whose import is not loaded yet says "retry" and is not left in flight.
#[test]
fn missing_import_retries_and_clears() {
    let mut cache = cache();
    cache.set_shader(ROOT, root());
    let pending = job(&mut cache, 0, &defs(true));
    assert!(is_retry(&cache.finish(pending.compile(&()))));
    assert_eq!(cache.in_flight(), 0);
    cache.set_shader(MATHS, maths("2.0"));
    assert!(cache.get(0, ROOT, &defs(true)).unwrap().contains("* 2.0"));
}

/// Pool size: every core but the main and render threads, never under Bevy's 4 async threads.
#[test]
fn compile_pool_keeps_two_cores_and_at_least_four_threads() {
    assert_eq!(compile_pool_threads(16), 14);
    assert_eq!(compile_pool_threads(8), 6);
    assert_eq!(compile_pool_threads(4), 4);
    assert_eq!(compile_pool_threads(1), 4);
}
