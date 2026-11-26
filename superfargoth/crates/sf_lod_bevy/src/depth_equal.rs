//! The impostor's alpha test runs once, in the camera prepass (big-world-rendering.md "Fill
//! rules", rule 5; UE `r.EarlyZPassOnlyMaterialMasking`): in a view with a depth prepass the main
//! pass tests depth-equal, writes no depth and never discards, so only the pixels the prepass
//! kept are shaded. [`depth_equal_main_pass`] is the pipeline half, called from
//! `ImpostorMaterial::specialize`; `impostor.wesl` drops its discards under
//! [`DEPTH_EQUAL_MAIN_DEF`].
//!
//! With `Equal`, a part the main pass draws and no prepass drew is missing instead of drawn.
//! Bevy culls each render phase on the GPU with the same shader on the same inputs (two-phase
//! occlusion culling included: `bevy_pbr` `mesh_preprocess.wesl`, `build_indirect_params.wesl`),
//! so the two phases can only differ on the CPU: a part whose main-pass pipeline is ready while
//! its prepass pipeline is missing or still compiling. [`count_unprepassed`] counts those
//! (`far/impostors/no_prepass`, expected 0) from Bevy's public specialization caches.

use bevy_app::App;
use bevy_ecs::prelude::*;
use bevy_ecs::system::SystemParam;
use bevy_pbr::{MeshMaterial3d, MeshPipelineKey, SpecializedMaterialPipelineCache, SpecializedPrepassMaterialPipelineCache, ViewKeyCache};
use bevy_render::camera::ExtractedCamera;
use bevy_render::render_resource::{CompareFunction, PipelineCache, RenderPipelineDescriptor};
use bevy_render::sync_world::MainEntity;
use bevy_render::view::ExtractedView;
use bevy_render::{Extract, ExtractSchedule, Render, RenderApp, RenderSystems};
use bevy_shader::ShaderDefVal;
use sf_diag_bevy::DiagCounters;
use sf_diag_bevy::sf_diag::unit;

use crate::ImpostorMaterial;

/// Set on the main-pass fragment shader with the depth-equal state.
pub const DEPTH_EQUAL_MAIN_DEF: &str = "DEPTH_EQUAL_MAIN";

/// How `bevy_pbr` marks the descriptors of the prepass pipeline, which shadow views share
/// (`prepass/mod.rs`: the shader def and the label). The mesh key cannot tell them from the main
/// pass: all three carry `DEPTH_PREPASS`.
const PREPASS_PIPELINE_DEF: &str = "PREPASS_PIPELINE";
const PREPASS_PIPELINE_LABEL: &str = "prepass_pipeline";

/// Main-pass draws that no prepass can draw, per frame, summed over the depth-prepass views.
pub const NO_PREPASS_COUNTER: &str = "far/impostors/no_prepass";

/// Switches a main-pass descriptor of a view with a depth prepass to depth-equal with no depth
/// write, and tells its fragment shader ([`DEPTH_EQUAL_MAIN_DEF`]). Everything else is left as
/// Bevy built it: prepass and shadow descriptors, a view without a depth prepass, and a pass that
/// writes no depth (blended). Returns whether it switched.
pub fn depth_equal_main_pass(descriptor: &mut RenderPipelineDescriptor, mesh_key: MeshPipelineKey) -> bool {
    let prepass = descriptor.vertex.shader_defs.contains(&ShaderDefVal::from(PREPASS_PIPELINE_DEF))
        || descriptor.label.as_deref() == Some(PREPASS_PIPELINE_LABEL);
    if prepass || !mesh_key.contains(MeshPipelineKey::DEPTH_PREPASS) {
        return false;
    }
    let (Some(depth), Some(fragment)) = (descriptor.depth_stencil.as_mut(), descriptor.fragment.as_mut()) else {
        return false;
    };
    if depth.depth_write_enabled != Some(true) {
        return false;
    }
    depth.depth_compare = Some(CompareFunction::Equal);
    depth.depth_write_enabled = Some(false);
    fragment.shader_defs.push(DEPTH_EQUAL_MAIN_DEF.into());
    true
}

/// The far field's impostor parts this frame (render world).
#[derive(Resource, Default)]
struct ImpostorParts(Vec<MainEntity>);

pub(crate) fn add_watch(app: &mut App) {
    let counters = app.world().get_resource::<DiagCounters>().cloned();
    let Some(render) = app.get_sub_app_mut(RenderApp) else { return };
    if let Some(counters) = counters {
        render.insert_resource(counters);
    }
    render
        .init_resource::<ImpostorParts>()
        .add_systems(ExtractSchedule, extract_impostor_parts)
        .add_systems(Render, count_unprepassed.in_set(RenderSystems::Cleanup));
}

#[allow(clippy::needless_pass_by_value)] // system parameters
fn extract_impostor_parts(mut parts: ResMut<ImpostorParts>, impostors: Extract<Query<Entity, With<MeshMaterial3d<ImpostorMaterial>>>>) {
    parts.0.clear();
    parts.0.extend(impostors.iter().map(MainEntity::from));
}

/// How many of `parts` have a ready main-pass pipeline and no ready prepass pipeline.
fn unprepassed<K: Copy>(
    parts: &[MainEntity],
    main: impl Fn(&MainEntity) -> Option<K>,
    prepass: impl Fn(&MainEntity) -> Option<K>,
    ready: impl Fn(K) -> bool,
) -> usize {
    parts.iter().filter(|&part| main(part).is_some_and(&ready) && !prepass(part).is_some_and(&ready)).count()
}

/// One episode of missed prepasses, for the log.
#[derive(Default)]
struct Missed {
    frames: u32,
    peak: usize,
}

/// Bevy's pipelines per view and mesh (public render-world resources).
#[derive(SystemParam)]
struct Specialized<'w> {
    cache: Res<'w, PipelineCache>,
    main: Option<Res<'w, SpecializedMaterialPipelineCache>>,
    prepass: Option<Res<'w, SpecializedPrepassMaterialPipelineCache>>,
    view_keys: Option<Res<'w, ViewKeyCache>>,
}

/// See the module docs. Reads the specialization caches after `Queue`, like `sf_diag_bevy`'s
/// pipeline watch; a part counts whether or not it is on screen this frame.
#[allow(clippy::needless_pass_by_value)] // system parameters
fn count_unprepassed(
    parts: Res<ImpostorParts>,
    specialized: Specialized,
    views: Query<&ExtractedView, With<ExtractedCamera>>,
    counters: Option<Res<DiagCounters>>,
    mut missed: Local<Missed>,
) {
    let Specialized { cache, main, prepass, view_keys } = specialized;
    let mut n = 0;
    if let (Some(main), Some(view_keys)) = (main, view_keys) {
        let ready = |id| cache.get_render_pipeline(id).is_some();
        for view in &views {
            let id = &view.retained_view_entity;
            if !view_keys.get(id).is_some_and(|key| key.contains(MeshPipelineKey::DEPTH_PREPASS)) {
                continue;
            }
            let Some(main) = main.get(id) else { continue };
            let prepass = prepass.as_ref().and_then(|p| p.get(id));
            n += unprepassed(&parts.0, |part| main.get(part).copied(), |part| prepass.and_then(|p| p.get(part)).map(|entry| entry.1), ready);
        }
    }
    if let Some(counters) = counters {
        #[allow(clippy::cast_precision_loss)] // a count of entities
        counters.set_named(NO_PREPASS_COUNTER, unit::COUNT, n as f64);
    }
    if n > 0 {
        if missed.frames == 0 {
            log::warn!("sf_lod_bevy: {n} impostor parts have a main-pass pipeline and no prepass pipeline (not drawn while the main pass tests depth-equal)");
        }
        missed.frames += 1;
        missed.peak = missed.peak.max(n);
    } else if missed.frames > 0 {
        log::warn!("sf_lod_bevy: impostor parts without a prepass pipeline: up to {} for {} frames", missed.peak, missed.frames);
        *missed = Missed::default();
    }
}

#[cfg(test)]
mod tests {
    use bevy_ecs::entity::Entity;
    use bevy_render::render_resource::{DepthBiasState, DepthStencilState, FragmentState, StencilState, TextureFormat, VertexState};

    use super::*;

    /// A descriptor as `bevy_pbr` hands it to `Material::specialize`: the label, the shader defs
    /// (on both stages) and the depth state of `MeshPipeline::specialize` (`render/mesh.rs`) or
    /// of the prepass pipeline (`prepass/mod.rs`).
    fn stock(label: &'static str, defs: &[&'static str], depth_write: bool) -> RenderPipelineDescriptor {
        let shader_defs: Vec<ShaderDefVal> = defs.iter().map(|&d| d.into()).collect();
        RenderPipelineDescriptor {
            label: Some(label.into()),
            vertex: VertexState { shader_defs: shader_defs.clone(), ..VertexState::default() },
            fragment: Some(FragmentState { shader_defs, ..FragmentState::default() }),
            depth_stencil: Some(DepthStencilState {
                format: TextureFormat::Depth32Float,
                depth_write_enabled: Some(depth_write),
                depth_compare: Some(CompareFunction::GreaterEqual),
                stencil: StencilState::default(),
                bias: DepthBiasState::default(),
            }),
            ..RenderPipelineDescriptor::default()
        }
    }

    const MASK: MeshPipelineKey = MeshPipelineKey::MAY_DISCARD;

    #[test]
    fn main_pass_with_a_depth_prepass_tests_equal_and_writes_no_depth() {
        let key = MASK | MeshPipelineKey::DEPTH_PREPASS | MeshPipelineKey::MOTION_VECTOR_PREPASS;
        let before = stock("opaque_mesh_pipeline", &["MAY_DISCARD", "DEPTH_PREPASS", "MOTION_VECTOR_PREPASS"], true);
        let mut d = before.clone();
        assert!(depth_equal_main_pass(&mut d, key));

        let depth = d.depth_stencil.as_ref().unwrap();
        assert_eq!(depth.depth_compare, Some(CompareFunction::Equal));
        assert_eq!(depth.depth_write_enabled, Some(false));
        assert_eq!(d.fragment.as_ref().unwrap().shader_defs.last(), Some(&ShaderDefVal::from(DEPTH_EQUAL_MAIN_DEF)));

        // Nothing else moved.
        let mut expected = before;
        let depth = expected.depth_stencil.as_mut().unwrap();
        depth.depth_compare = Some(CompareFunction::Equal);
        depth.depth_write_enabled = Some(false);
        expected.fragment.as_mut().unwrap().shader_defs.push(DEPTH_EQUAL_MAIN_DEF.into());
        assert_eq!(d, expected);
    }

    #[test]
    fn main_pass_without_a_depth_prepass_is_stock() {
        // Normal or motion-vector prepass alone lays no depth the main pass could equal.
        for key in [MASK, MASK | MeshPipelineKey::NORMAL_PREPASS | MeshPipelineKey::MOTION_VECTOR_PREPASS] {
            let before = stock("opaque_mesh_pipeline", &["MAY_DISCARD"], true);
            let mut d = before.clone();
            assert!(!depth_equal_main_pass(&mut d, key));
            assert_eq!(d, before);
        }
    }

    #[test]
    fn prepass_and_shadow_pipelines_are_unchanged() {
        // Camera prepass and shadow views share one pipeline type; both keys carry
        // `DEPTH_PREPASS`, as the main pass key of a view with a depth prepass does.
        let key = MASK | MeshPipelineKey::DEPTH_PREPASS;
        for defs in [
            &["PREPASS_PIPELINE", "DEPTH_PREPASS", "MAY_DISCARD", "MOTION_VECTOR_PREPASS", "PREPASS_FRAGMENT"][..],
            &["PREPASS_PIPELINE", "DEPTH_PREPASS", "MAY_DISCARD", "UNCLIPPED_DEPTH_ORTHO_EMULATION"][..],
        ] {
            let before = stock("prepass_pipeline", defs, true);
            let mut d = before.clone();
            assert!(!depth_equal_main_pass(&mut d, key));
            assert_eq!(d, before);
        }
        // Either mark alone is enough.
        for before in [stock("prepass_pipeline", &["DEPTH_PREPASS"], true), stock("some_label", &["PREPASS_PIPELINE"], true)] {
            let mut d = before.clone();
            assert!(!depth_equal_main_pass(&mut d, key));
            assert_eq!(d, before);
        }
    }

    #[test]
    fn a_pass_that_writes_no_depth_is_unchanged() {
        // Blended passes have no prepass twin: nothing to equal.
        let before = stock("alpha_blend_mesh_pipeline", &["DEPTH_PREPASS"], false);
        let mut d = before.clone();
        assert!(!depth_equal_main_pass(&mut d, MeshPipelineKey::BLEND_ALPHA | MeshPipelineKey::DEPTH_PREPASS));
        assert_eq!(d, before);
    }

    /// The marks [`depth_equal_main_pass`] reads are written by the vendored `bevy_pbr`, and the
    /// main pass still starts from the state the tests above assume. A Bevy update that renames
    /// one fails here, not on screen.
    #[test]
    fn vendored_bevy_still_marks_its_pipelines() {
        let prepass = include_str!("../../../vendor/bevy_pbr/src/prepass/mod.rs");
        assert!(prepass.contains(&format!("shader_defs.push(\"{PREPASS_PIPELINE_DEF}\".into());")));
        assert!(prepass.contains(&format!("label: Some(\"{PREPASS_PIPELINE_LABEL}\".into()),")));
        let mesh = include_str!("../../../vendor/bevy_pbr/src/render/mesh.rs");
        assert!(mesh.contains("label = \"opaque_mesh_pipeline\".into();"));
        assert!(mesh.contains("depth_compare: Some(CompareFunction::GreaterEqual),"));
        assert!(mesh.contains("shader_defs.push(\"DEPTH_PREPASS\".into());"));
    }

    #[test]
    fn counts_parts_the_main_pass_can_draw_and_the_prepass_cannot() {
        let parts: Vec<MainEntity> = (1..=5).map(|i| MainEntity::from(Entity::from_raw_u32(i).unwrap())).collect();
        let index = |part: &MainEntity| parts.iter().position(|p| p == part).unwrap();
        // Pipeline ids: 1 and 2 are compiled, 9 is still compiling.
        let ready = |id: u32| id != 9;
        // Part 0: both ready. 1: no prepass pipeline. 2: prepass compiling. 3: main compiling
        // (not drawn at all, as in stock). 4: no main pipeline.
        let main = [Some(1), Some(1), Some(1), Some(9), None];
        let prepass = [Some(2), None, Some(9), None, Some(2)];
        assert_eq!(unprepassed(&parts, |p| main[index(p)], |p| prepass[index(p)], ready), 2);
        assert_eq!(unprepassed(&parts[..1], |p| main[index(p)], |p| prepass[index(p)], ready), 0);
    }
}
