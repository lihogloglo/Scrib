//! Pipeline warm-up W3 (`pipeline-warmup.md` §5.2): the precache builds the draw path's keys.
//!
//! GPU-free: (1) the shared key functions (`bevy_pbr::precache`, now called by
//! `specialize_material_meshes`, `specialize_prepass_material_meshes`, `specialize_shadows`) give
//! exactly the bits the draw path's former inline code gave (oracles below are that code, copied);
//! (2) a specimen's plan is exactly the draw-path keys for every view kind; (3) `precache_specimens`
//! in a bare world, with a stand-in specialiser keyed by `(layout, key)` like
//! `SpecializedMeshPipelines`, hands out the ids a later draw then hits; (4) the tag status.
//! The real `PipelineCache` needs a render device (not here).

use std::any::TypeId;
use std::sync::Arc;

use bevy_ecs::{prelude::*, system::SystemState};
use bevy_material::{
    AlphaMode, MaterialProperties, OpaqueRendererMethod, RenderPhaseType,
    descriptor::CachedRenderPipelineId,
    key::{ErasedMaterialKey, ErasedMaterialPipelineKey, ErasedMeshPipelineKey},
    labels::ShaderLabel,
    specialize::SpecializedMeshPipelineError,
};
use bevy_mesh::{
    BaseMeshPipelineKey, MeshAttributeCompressionFlags, MeshVertexBufferLayout, MeshVertexBufferLayoutRef,
    MeshVertexBufferLayouts, PrimitiveTopology, VertexBufferLayout,
};
use bevy_pbr::precache::{
    PrecachePass, PrecacheParam, PrecacheViewKeys, PrepassViewKey, main_pass_mesh_key, material_pipeline_key,
    plan_specimen, precache_specimens, precache_specimens_with, prepass_mesh_key, shadow_mesh_key, shadow_view_key,
};
use bevy_pbr::{MeshPipelineKey, PreparedMaterial, PrepassFragmentShader, alpha_mode_pipeline_key};
use bevy_platform::collections::HashMap;
use bevy_render::{
    erased_render_asset::ErasedRenderAssets,
    material_bind_groups::MaterialBindingId,
    mesh::{RenderMesh, RenderMeshBufferInfo},
    render_asset::RenderAssets,
    view::Msaa,
};

use crate::precache::{PipelinePrecache, Precache, PrecacheLimit, Progress, Specimen, SpecimenFlags, specimen_progress, tag_status};
use crate::{RevealGate, RevealInputs, RevealStep};

// ---- inputs ----

/// The entity components the draw path reads, as plain values.
#[derive(Clone, Copy, Debug)]
#[allow(clippy::struct_excessive_bools, reason = "test inputs")]
struct Ent {
    shadow_caster: bool,
    /// `Some(bicubic)` when lightmapped.
    lightmap: Option<bool>,
    dither: bool,
    prev_skin: bool,
    prev_morph: bool,
}

impl Ent {
    fn flags(self) -> SpecimenFlags {
        let mut f = SpecimenFlags::empty();
        f.set(SpecimenFlags::SHADOW_CASTER, self.shadow_caster);
        if let Some(bicubic) = self.lightmap {
            f |= SpecimenFlags::LIGHTMAPPED;
            f.set(SpecimenFlags::LIGHTMAP_BICUBIC_SAMPLING, bicubic);
        }
        f.set(SpecimenFlags::VISIBILITY_RANGE_DITHER, self.dither);
        f.set(SpecimenFlags::HAS_PREVIOUS_SKIN, self.prev_skin);
        f.set(SpecimenFlags::HAS_PREVIOUS_MORPH, self.prev_morph);
        f
    }
}

fn all_ents() -> Vec<Ent> {
    let mut out = Vec::new();
    for bits in 0..16u8 {
        for lightmap in [None, Some(false), Some(true)] {
            out.push(Ent {
                shadow_caster: bits & 1 != 0,
                lightmap,
                dither: bits & 2 != 0,
                prev_skin: bits & 4 != 0,
                prev_morph: bits & 8 != 0,
            });
        }
    }
    out
}

#[derive(Clone, Copy, Debug)]
#[allow(clippy::struct_excessive_bools, reason = "test inputs")]
struct Mat {
    alpha: AlphaMode,
    deferred: bool,
    prepass: bool,
    shadows: bool,
    transmission: bool,
    reads_material: bool,
}

#[allow(clippy::unnecessary_wraps, reason = "BaseSpecializeFn signature")]
fn fake_main(
    world: &mut World,
    key: ErasedMaterialPipelineKey,
    layout: &MeshVertexBufferLayoutRef,
    _: &Arc<MaterialProperties>,
) -> Result<CachedRenderPipelineId, SpecializedMeshPipelineError> {
    Ok(world.resource_mut::<FakeSpecializers>().specialize(0, key, layout))
}

#[allow(clippy::unnecessary_wraps, reason = "PrepassSpecializeFn signature")]
fn fake_prepass(
    world: &mut World,
    key: &ErasedMaterialPipelineKey,
    layout: &MeshVertexBufferLayoutRef,
    _: &Arc<MaterialProperties>,
) -> Result<CachedRenderPipelineId, SpecializedMeshPipelineError> {
    Ok(world.resource_mut::<FakeSpecializers>().specialize(1, key.clone(), layout))
}

/// Stand-in for `SpecializedMeshPipelines<MaterialPipelineSpecializer>` (0) and
/// `<PrepassPipelineSpecializer>` (1): one id per `(layout, key)`, ids from one counter like the
/// pipeline cache's. A new id is "compiling" (in `compiling`, W4b1-fix) until a test finishes it.
#[derive(Resource, Default)]
struct FakeSpecializers {
    maps: [HashMap<(MeshVertexBufferLayoutRef, ErasedMaterialPipelineKey), CachedRenderPipelineId>; 2],
    next: usize,
    compiling: Vec<CachedRenderPipelineId>,
}

impl FakeSpecializers {
    fn specialize(&mut self, which: usize, key: ErasedMaterialPipelineKey, layout: &MeshVertexBufferLayoutRef) -> CachedRenderPipelineId {
        let next = &mut self.next;
        let compiling = &mut self.compiling;
        *self.maps[which].entry((layout.clone(), key)).or_insert_with(|| {
            *next += 1;
            let id = CachedRenderPipelineId::new(*next);
            compiling.push(id);
            id
        })
    }
    fn len(&self) -> usize {
        self.maps[0].len() + self.maps[1].len()
    }
}

fn props(m: Mat) -> MaterialProperties {
    let mut p = MaterialProperties {
        alpha_mode: m.alpha,
        render_method: if m.deferred { OpaqueRendererMethod::Deferred } else { OpaqueRendererMethod::Forward },
        prepass_enabled: m.prepass,
        shadows_enabled: m.shadows,
        reads_view_transmission_texture: m.transmission,
        render_phase_type: match m.alpha {
            AlphaMode::Mask(_) => RenderPhaseType::AlphaMask,
            AlphaMode::Opaque => RenderPhaseType::Opaque,
            _ => RenderPhaseType::Transparent,
        },
        // StandardMaterial sets no material-specific mesh-key bits for these cases.
        mesh_pipeline_key_bits: ErasedMeshPipelineKey::new(MeshPipelineKey::NONE),
        material_key: ErasedMaterialKey::new(7u32),
        base_specialize: Some(fake_main),
        prepass_specialize: Some(fake_prepass),
        ..Default::default()
    };
    if m.reads_material {
        p.shaders.push((PrepassFragmentShader.intern(), bevy_asset::Handle::default()));
    }
    p
}

fn all_mats() -> Vec<Mat> {
    let mut out = Vec::new();
    for alpha in [AlphaMode::Opaque, AlphaMode::Mask(0.5), AlphaMode::Blend, AlphaMode::AlphaToCoverage, AlphaMode::Premultiplied] {
        for bits in 0..32u8 {
            out.push(Mat {
                alpha,
                deferred: bits & 1 != 0,
                prepass: bits & 2 != 0,
                shadows: bits & 4 != 0,
                transmission: bits & 8 != 0,
                reads_material: bits & 16 != 0,
            });
        }
    }
    out
}

fn meshes() -> [BaseMeshPipelineKey; 3] {
    let tri = BaseMeshPipelineKey::from_primitive_topology_and_strip_index(PrimitiveTopology::TriangleList, None);
    let strip = BaseMeshPipelineKey::from_primitive_topology_and_strip_index(PrimitiveTopology::TriangleStrip, None);
    [tri.clone(), tri | BaseMeshPipelineKey::MORPH_TARGETS, strip]
}

const MV: MeshPipelineKey = MeshPipelineKey::MOTION_VECTOR_PREPASS;

fn main_views() -> [MeshPipelineKey; 2] {
    [
        MeshPipelineKey::from_msaa_samples(4) | MeshPipelineKey::TONEMAP_IN_SHADER,
        MeshPipelineKey::from_msaa_samples(1) | MeshPipelineKey::TONEMAP_IN_SHADER | MV | MeshPipelineKey::DEPTH_PREPASS | MeshPipelineKey::NORMAL_PREPASS,
    ]
}

fn prepass_views() -> Vec<PrepassViewKey> {
    let mut out = Vec::new();
    for samples in [1, 4] {
        for motion_vector_prepass in [false, true] {
            for deferred_prepass in [false, true] {
                let mut key = MeshPipelineKey::from_msaa_samples(samples) | MeshPipelineKey::DEPTH_PREPASS;
                key.set(MV, motion_vector_prepass);
                out.push(PrepassViewKey { key, msaa: Msaa::from_samples(samples), deferred_prepass, motion_vector_prepass });
            }
        }
    }
    out
}

// ---- oracles: the draw path's inline code before W3 (Extracted draws) ----

fn oracle_main(view_key: MeshPipelineKey, mesh: &BaseMeshPipelineKey, p: &MaterialProperties, e: Ent) -> MeshPipelineKey {
    let mut bits: MeshPipelineKey = p.mesh_pipeline_key_bits.downcast();
    bits.insert(alpha_mode_pipeline_key(p.alpha_mode, &Msaa::from_samples(view_key.msaa_samples())));
    let mut k = view_key | MeshPipelineKey::from_bits_retain(mesh.bits()) | bits;
    if let Some(bicubic) = e.lightmap {
        k |= MeshPipelineKey::LIGHTMAPPED;
        if bicubic {
            k |= MeshPipelineKey::LIGHTMAP_BICUBIC_SAMPLING;
        }
    }
    if e.dither {
        k |= MeshPipelineKey::VISIBILITY_RANGE_DITHER;
    }
    if view_key.contains(MV) {
        if e.prev_skin {
            k |= MeshPipelineKey::HAS_PREVIOUS_SKIN;
        }
        if e.prev_morph {
            k |= MeshPipelineKey::HAS_PREVIOUS_MORPH;
        }
    }
    k
}

fn oracle_prepass(v: PrepassViewKey, mesh: &BaseMeshPipelineKey, p: &MaterialProperties, e: Ent) -> Option<MeshPipelineKey> {
    if !p.prepass_enabled {
        return None;
    }
    let mut k = v.key | MeshPipelineKey::from_bits_retain(mesh.bits());
    match p.alpha_mode {
        AlphaMode::Opaque | AlphaMode::AlphaToCoverage | AlphaMode::Mask(_) => k |= alpha_mode_pipeline_key(p.alpha_mode, &v.msaa),
        _ => return None,
    }
    if p.reads_view_transmission_texture {
        return None;
    }
    let forward = matches!(p.render_method, OpaqueRendererMethod::Forward);
    let deferred = v.deferred_prepass && !forward;
    if deferred {
        k |= MeshPipelineKey::DEFERRED_PREPASS;
    }
    if let Some(bicubic) = e.lightmap {
        k |= MeshPipelineKey::LIGHTMAPPED;
        if bicubic && deferred {
            k |= MeshPipelineKey::LIGHTMAP_BICUBIC_SAMPLING;
        }
    }
    if e.dither {
        k |= MeshPipelineKey::VISIBILITY_RANGE_DITHER;
    }
    if v.motion_vector_prepass {
        if e.prev_skin {
            k |= MeshPipelineKey::HAS_PREVIOUS_SKIN;
        }
        if e.prev_morph {
            k |= MeshPipelineKey::HAS_PREVIOUS_MORPH;
        }
    }
    if p.get_shader(PrepassFragmentShader).is_some() {
        k |= MeshPipelineKey::PREPASS_READS_MATERIAL;
    }
    Some(k)
}

fn oracle_light_key(directional: bool) -> MeshPipelineKey {
    let mut k = MeshPipelineKey::DEPTH_PREPASS;
    k.set(MeshPipelineKey::VIEW_PROJECTION_ORTHOGRAPHIC | MeshPipelineKey::UNCLIPPED_DEPTH_ORTHO, directional);
    k.set(MeshPipelineKey::VIEW_PROJECTION_PERSPECTIVE, !directional);
    k
}

fn oracle_shadow(light_key: MeshPipelineKey, mesh: &BaseMeshPipelineKey, p: &MaterialProperties, e: Ent) -> Option<MeshPipelineKey> {
    if !p.shadows_enabled || !e.shadow_caster {
        return None;
    }
    let mut k = light_key | MeshPipelineKey::from_bits_retain(mesh.bits());
    if e.lightmap.is_some() {
        k |= MeshPipelineKey::LIGHTMAPPED;
    }
    if matches!(
        p.alpha_mode,
        AlphaMode::Mask(_) | AlphaMode::Blend | AlphaMode::Premultiplied | AlphaMode::Add | AlphaMode::AlphaToCoverage
    ) {
        k |= MeshPipelineKey::MAY_DISCARD;
    }
    if p.get_shader(PrepassFragmentShader).is_some() {
        k |= MeshPipelineKey::PREPASS_READS_MATERIAL;
    }
    Some(k)
}

// ---- (1) shared key functions == the former draw-path code ----

#[test]
fn shared_keys_match_the_former_draw_path_bit_for_bit() {
    let (mut main, mut prepass, mut shadow) = (0, 0, 0);
    for m in all_mats() {
        let p = props(m);
        for mesh in &meshes() {
            for e in all_ents() {
                for v in main_views() {
                    assert_eq!(main_pass_mesh_key(v, mesh, &p, e.flags()), oracle_main(v, mesh, &p, e), "main {m:?} {e:?}");
                    main += 1;
                }
                for v in prepass_views() {
                    let got = prepass_mesh_key(v.key, mesh, &p, &v.msaa, v.deferred_prepass, v.motion_vector_prepass, e.flags());
                    assert_eq!(got, oracle_prepass(v, mesh, &p, e), "prepass {m:?} {e:?} {v:?}");
                    prepass += usize::from(got.is_some());
                }
                for directional in [true, false] {
                    let got = shadow_mesh_key(shadow_view_key(directional), mesh, &p, e.flags());
                    assert_eq!(got, oracle_shadow(oracle_light_key(directional), mesh, &p, e), "shadow {m:?} {e:?}");
                    shadow += usize::from(got.is_some());
                }
            }
        }
    }
    // Every branch was exercised.
    assert!(main > 0 && prepass > 0 && shadow > 0);
}

#[test]
fn cascade_point_and_spot_view_kinds() {
    // A directional cascade: orthographic + unclipped depth; point faces and spots: one perspective key.
    assert_eq!(shadow_view_key(true), oracle_light_key(true));
    assert_eq!(shadow_view_key(false), oracle_light_key(false));
    assert!(shadow_view_key(true).contains(MeshPipelineKey::UNCLIPPED_DEPTH_ORTHO));
    assert_ne!(shadow_view_key(true), shadow_view_key(false));
    // Both templates are there before any light view, and a live light's key adds nothing new.
    let mut views = PrecacheViewKeys::default();
    assert_eq!(views.shadow, vec![shadow_view_key(true), shadow_view_key(false)]);
    views.insert_shadow(oracle_light_key(false));
    assert_eq!(views.shadow.len(), 2);
}

// ---- (2) a specimen's plan is exactly the draw-path keys ----

fn draw_keys(views: &PrecacheViewKeys, mesh: &BaseMeshPipelineKey, p: &MaterialProperties, ty: TypeId, flags: SpecimenFlags) -> Vec<(PrecachePass, ErasedMaterialPipelineKey)> {
    let mut out = Vec::new();
    let mut push = |e| {
        if !out.contains(&e) {
            out.push(e);
        }
    };
    for &v in &views.main {
        push((PrecachePass::Main, material_pipeline_key(ty, main_pass_mesh_key(v, mesh, p, flags), p)));
    }
    if !matches!(p.render_phase_type, RenderPhaseType::Transmissive | RenderPhaseType::Transparent) {
        for v in &views.prepass {
            if let Some(k) = prepass_mesh_key(v.key, mesh, p, &v.msaa, v.deferred_prepass, v.motion_vector_prepass, flags) {
                push((PrecachePass::Prepass, material_pipeline_key(ty, k, p)));
            }
        }
    }
    for &l in &views.shadow {
        if let Some(k) = shadow_mesh_key(l, mesh, p, flags) {
            push((PrecachePass::Shadow, material_pipeline_key(ty, k, p)));
        }
    }
    out
}

fn known_views() -> PrecacheViewKeys {
    let mut views = PrecacheViewKeys::default();
    for v in main_views() {
        views.insert_main(v);
    }
    let pv = prepass_views();
    views.insert_prepass(pv[0]); // msaa 1, no motion vectors, forward
    views.insert_prepass(pv[3]); // msaa 1, motion vectors, deferred
    views
}

fn as_set(v: &[(PrecachePass, ErasedMaterialPipelineKey)]) -> Vec<String> {
    let mut s: Vec<String> = v.iter().map(|(pass, k)| format!("{pass:?} {:x}", k.mesh_key.downcast::<MeshPipelineKey>().bits())).collect();
    s.sort();
    s
}

#[test]
fn plan_is_the_draw_path_keys_per_view_kind() {
    let views = known_views();
    let ty = TypeId::of::<u8>();
    let [tri, morph, _] = meshes();
    let base = Mat { alpha: AlphaMode::Opaque, deferred: false, prepass: true, shadows: true, transmission: false, reads_material: false };
    let caster = Ent { shadow_caster: true, lightmap: None, dither: false, prev_skin: false, prev_morph: false };
    let cases = [
        ("opaque caster", base, caster, &tri),
        ("alpha mask", Mat { alpha: AlphaMode::Mask(0.5), ..base }, caster, &tri),
        ("not a caster", base, Ent { shadow_caster: false, ..caster }, &tri),
        ("blended", Mat { alpha: AlphaMode::Blend, ..base }, caster, &tri),
        ("deferred, lightmapped, dithered", Mat { deferred: true, ..base }, Ent { lightmap: Some(true), dither: true, ..caster }, &tri),
        ("custom prepass shader", Mat { reads_material: true, ..base }, caster, &tri),
    ];
    for (name, m, e, mesh) in cases {
        let p = props(m);
        let plan = plan_specimen(&views, mesh, &p, ty, e.flags());
        assert_eq!(plan.len(), as_set(&plan).len(), "{name}: no duplicates");
        assert_eq!(as_set(&plan), as_set(&draw_keys(&views, mesh, &p, ty, e.flags())), "{name}");
        let count = |pass| plan.iter().filter(|(p, _)| *p == pass).count();
        assert_eq!(count(PrecachePass::Main), 2, "{name}: one per camera key");
        match name {
            "not a caster" => assert_eq!(count(PrecachePass::Shadow), 0),
            "blended" => assert_eq!((count(PrecachePass::Prepass), count(PrecachePass::Shadow)), (0, 2)),
            _ => assert_eq!((count(PrecachePass::Prepass), count(PrecachePass::Shadow)), (2, 2), "{name}"),
        }
    }
    // Alpha mask: its shadow keys may discard; opaque ones don't.
    let mask = plan_specimen(&views, &tri, &props(Mat { alpha: AlphaMode::Mask(0.5), ..base }), ty, caster.flags());
    assert!(mask.iter().filter(|(p, _)| *p == PrecachePass::Shadow).all(|(_, k)| k.mesh_key.downcast::<MeshPipelineKey>().contains(MeshPipelineKey::MAY_DISCARD)));

    // A skinned + morphed specimen: the draw path's keys for every subset of the previous-frame bits
    // (both from the second frame on, none on the first, previous skin alone when the morph weights
    // did not change, previous morph alone); only the motion-vector views differ, so each of the 3
    // extra variants adds 1 main key and 1 prepass key.
    let p = props(base);
    let skinned = Ent { prev_skin: true, prev_morph: true, ..caster };
    let plan = plan_specimen(&views, &morph, &p, ty, skinned.flags());
    let mut expected = Vec::new();
    for e in [skinned, caster, Ent { prev_morph: false, ..skinned }, Ent { prev_skin: false, ..skinned }] {
        for k in draw_keys(&views, &morph, &p, ty, e.flags()) {
            if !expected.contains(&k) {
                expected.push(k);
            }
        }
    }
    assert_eq!(plan.len(), as_set(&plan).len(), "no duplicates");
    assert_eq!(as_set(&plan), as_set(&expected));
    assert_eq!(plan.iter().filter(|(p, _)| *p == PrecachePass::Main).count(), 5);
    assert_eq!(plan.iter().filter(|(p, _)| *p == PrecachePass::Prepass).count(), 5);
    // Previous skin alone (no morph bits) keeps its two variants: with and without.
    let skin_only = Ent { prev_skin: true, ..caster };
    let plan = plan_specimen(&views, &morph, &p, ty, skin_only.flags());
    assert_eq!(plan.iter().filter(|(p, _)| *p == PrecachePass::Main).count(), 3);
}

// ---- (3) precache_specimens in a bare world: the draw later hits ----

fn layout() -> MeshVertexBufferLayoutRef {
    MeshVertexBufferLayouts::default().insert(MeshVertexBufferLayout::new(
        Vec::new(),
        VertexBufferLayout { array_stride: 12, ..Default::default() },
        MeshAttributeCompressionFlags::empty(),
    ))
}

#[test]
fn precached_ids_are_what_the_draw_path_gets() {
    let mut world = World::new();
    world.init_resource::<FakeSpecializers>();
    world.init_resource::<RenderAssets<RenderMesh>>();
    world.init_resource::<ErasedRenderAssets<PreparedMaterial>>();
    let mut precache = PipelinePrecache::default();
    *precache.views_mut() = known_views();
    let mesh_id = bevy_asset::AssetId::<bevy_mesh::Mesh>::default();
    let material_id = bevy_asset::AssetId::<bevy_pbr::StandardMaterial>::default().untyped();
    let flags = SpecimenFlags::SHADOW_CASTER | SpecimenFlags::LIGHTMAPPED;
    precache.request(0, Specimen { mesh: mesh_id, material: material_id, flags });
    world.insert_resource(precache);
    let mut state = SystemState::<PrecacheParam<'static, 'static>>::new(&mut world);

    // No mesh or material yet: waits, counted as pending assets.
    precache_specimens(&mut world, &mut state);
    let p = world.resource::<PipelinePrecache>();
    assert_eq!((p.waiting().len(), p.resolved().len(), p.stats().pending_assets), (1, 0, 1));
    let status = tag_status(1, p.waiting(), p.resolved(), |_| Progress::Ready);
    assert_eq!(status[0].pending_assets, 1);

    // Assets arrive: resolved for every view kind.
    let properties = Arc::new(props(Mat { alpha: AlphaMode::Opaque, deferred: false, prepass: true, shadows: true, transmission: false, reads_material: false }));
    let [key_bits, ..] = meshes();
    let layout = layout();
    world.resource_mut::<RenderAssets<RenderMesh>>().insert(
        mesh_id,
        RenderMesh { vertex_count: 3, aabb_center: glam::Vec3::ZERO, buffer_info: RenderMeshBufferInfo::NonIndexed, key_bits: key_bits.clone(), layout: layout.clone() },
    );
    world
        .resource_mut::<ErasedRenderAssets<PreparedMaterial>>()
        .insert(material_id, PreparedMaterial { binding: MaterialBindingId::default(), properties: properties.clone() });
    precache_specimens(&mut world, &mut state);
    let p = world.resource::<PipelinePrecache>();
    assert_eq!((p.waiting().len(), p.resolved().len(), p.stats().pending_assets), (0, 1, 0));
    let ids = p.resolved()[0].pipelines.clone();
    let plan = plan_specimen(&known_views(), &key_bits, &properties, material_id.type_id(), flags);
    assert_eq!(ids.len(), plan.len());
    let compiled = world.resource::<FakeSpecializers>().len();
    assert_eq!(compiled, plan.len());

    // The draw path, for an entity with these components in the motion-vector camera view: its key
    // from the shared functions, through the same specialiser => the precached id, nothing new.
    let view_key = main_views()[1];
    let draw_key = material_pipeline_key(material_id.type_id(), main_pass_mesh_key(view_key, &key_bits, &properties, flags), &properties);
    let id = fake_main(&mut world, draw_key, &layout, &properties).unwrap();
    assert!(ids.contains(&id));
    assert_eq!(world.resource::<FakeSpecializers>().len(), compiled, "a hash hit, no second compile");
    let light_key = material_pipeline_key(material_id.type_id(), shadow_mesh_key(shadow_view_key(false), &key_bits, &properties, flags).unwrap(), &properties);
    let shadow_id = fake_prepass(&mut world, &light_key, &layout, &properties).unwrap();
    assert!(ids.contains(&shadow_id));
    assert_eq!(world.resource::<FakeSpecializers>().len(), compiled);

    // Hit/miss accounting: each distinct pipeline once; an unpredicted one (a dithered entity) misses.
    let dithered = material_pipeline_key(
        material_id.type_id(),
        main_pass_mesh_key(view_key, &key_bits, &properties, flags | SpecimenFlags::VISIBILITY_RANGE_DITHER),
        &properties,
    );
    let miss_id = fake_main(&mut world, dithered, &layout, &properties).unwrap();
    assert!(!ids.contains(&miss_id));
    let mut p = world.resource_mut::<PipelinePrecache>();
    for draw in [id, id, shadow_id, miss_id, miss_id] {
        p.note_draw(draw);
    }
    let totals = p.stats();
    assert_eq!((totals.requested, totals.hit, totals.missed), (1, 2, 1));

    // Status: by the pipelines' states.
    let p = world.resource::<PipelinePrecache>();
    let first = ids[0];
    let rate = |progress: Progress| tag_status(1, p.waiting(), p.resolved(), |pid| if pid == first { progress } else { Progress::Ready })[0];
    assert_eq!(rate(Progress::Queued).queued, 1);
    assert_eq!(rate(Progress::Compiling).compiling, 1);
    assert_eq!(rate(Progress::Ready).ready, 1);
    assert_eq!(rate(Progress::Failed).failed, 1);
}

// ---- (4) status accounting ----

#[test]
fn tag_status_counts_pending_queued_compiling_ready_failed() {
    let s = |n: usize| CachedRenderPipelineId::new(n);
    let specimen = Specimen {
        mesh: bevy_asset::AssetId::default(),
        material: bevy_asset::AssetId::<bevy_pbr::StandardMaterial>::default().untyped(),
        flags: SpecimenFlags::empty(),
    };
    let waiting = [
        bevy_pbr::precache::PrecacheWaiting { tag: 0, specimen, assets_ready: false },
        bevy_pbr::precache::PrecacheWaiting { tag: 0, specimen, assets_ready: true },
        bevy_pbr::precache::PrecacheWaiting { tag: 1, specimen, assets_ready: false },
    ];
    let resolved = |tag, pipelines: Vec<CachedRenderPipelineId>, errors| bevy_pbr::precache::PrecacheResolved { tag, specimen, pipelines, errors };
    let resolved = [
        resolved(0, vec![s(1), s(2)], 0), // ready + queued -> queued
        resolved(0, vec![s(1), s(3)], 0), // ready + compiling -> compiling
        resolved(0, vec![s(1)], 0),       // ready
        resolved(0, vec![], 0),           // nothing to compile: ready
        resolved(0, vec![s(2), s(4)], 0), // queued + failed -> failed
        resolved(0, vec![s(1)], 1),       // a specialisation error -> failed
        resolved(1, vec![s(3)], 0),
        resolved(9, vec![s(1)], 0), // a tag not (yet) known: ignored
    ];
    let progress = |id: CachedRenderPipelineId| match id.id() {
        1 => Progress::Ready,
        2 => Progress::Queued,
        3 => Progress::Compiling,
        _ => Progress::Failed,
    };
    let status = tag_status(2, &waiting, &resolved, progress);
    assert_eq!(status.len(), 2);
    let t0 = status[0];
    assert_eq!((t0.pending_assets, t0.queued, t0.compiling, t0.ready, t0.failed), (1, 2, 1, 2, 2));
    assert_eq!(t0.total(), 8);
    assert!(!t0.is_settled());
    let t1 = status[1];
    assert_eq!((t1.pending_assets, t1.compiling, t1.total()), (1, 1, 2));

    assert_eq!(specimen_progress([Progress::Ready, Progress::Ready], 0), Progress::Ready);
    assert_eq!(specimen_progress([Progress::Compiling, Progress::Queued], 0), Progress::Queued);
    let settled = crate::PrecacheStatus { ready: 3, failed: 1, ..Default::default() };
    assert!(settled.is_settled());
}

#[test]
fn pipeline_states_map_to_progress() {
    use bevy_render::render_resource::CachedPipelineState;
    use bevy_shader::ShaderCacheError;
    use crate::pipeline_progress;
    assert_eq!(pipeline_progress(&CachedPipelineState::Queued), Progress::Queued);
    // A shader still loading is retried by the cache: queued, not failed.
    let not_loaded = CachedPipelineState::Err(ShaderCacheError::ShaderNotLoaded(bevy_asset::AssetId::default()));
    assert_eq!(pipeline_progress(&not_loaded), Progress::Queued);
    assert_eq!(pipeline_progress(&CachedPipelineState::Err(ShaderCacheError::ShaderImportNotYetAvailable)), Progress::Queued);
    let broken = CachedPipelineState::Err(ShaderCacheError::CreateShaderModule("bad".into()));
    assert_eq!(pipeline_progress(&broken), Progress::Failed);
}

#[test]
fn specimen_flags_from_components() {
    let lightmap = bevy_pbr::Lightmap { bicubic_sampling: true, ..Default::default() };
    let f = crate::specimen_flags(true, None, Some(&lightmap), true, false);
    assert_eq!(
        f,
        SpecimenFlags::SHADOW_CASTER | SpecimenFlags::LIGHTMAPPED | SpecimenFlags::LIGHTMAP_BICUBIC_SAMPLING | SpecimenFlags::HAS_PREVIOUS_SKIN
    );
    let abrupt = bevy_camera::visibility::VisibilityRange::abrupt(0.0, 10.0);
    assert!(!crate::specimen_flags(false, Some(&abrupt), None, false, false).contains(SpecimenFlags::VISIBILITY_RANGE_DITHER));
    let fading = bevy_camera::visibility::VisibilityRange { start_margin: 0.0..1.0, end_margin: 9.0..10.0, use_aabb: false };
    assert!(crate::specimen_flags(false, Some(&fading), None, false, false).contains(SpecimenFlags::VISIBILITY_RANGE_DITHER));
}

// ---- (5) W4b1-fix: low priority, at most K precache pipelines queued or compiling ----

/// `n` prepared specimens, each with its own vertex layout (so every planned pipeline is new),
/// and how many pipelines one plans.
fn throttle_world(n: usize) -> (World, Vec<Specimen>, usize) {
    let mut world = World::new();
    world.init_resource::<FakeSpecializers>();
    world.init_resource::<RenderAssets<RenderMesh>>();
    world.init_resource::<ErasedRenderAssets<PreparedMaterial>>();
    let properties = Arc::new(props(Mat { alpha: AlphaMode::Opaque, deferred: false, prepass: true, shadows: true, transmission: false, reads_material: false }));
    let material = bevy_asset::AssetId::<bevy_pbr::StandardMaterial>::default().untyped();
    world
        .resource_mut::<ErasedRenderAssets<PreparedMaterial>>()
        .insert(material, PreparedMaterial { binding: MaterialBindingId::default(), properties: properties.clone() });
    let [key_bits, ..] = meshes();
    let mut layouts = MeshVertexBufferLayouts::default();
    let mut specimens = Vec::new();
    for i in 0..n {
        let mesh = bevy_asset::AssetId::<bevy_mesh::Mesh>::Uuid { uuid: bevy_asset::uuid::Uuid::from_u128(u128::try_from(i).unwrap() + 1) };
        let stride = 12 + 4 * u64::try_from(i).unwrap();
        let layout = layouts.insert(MeshVertexBufferLayout::new(
            Vec::new(),
            VertexBufferLayout { array_stride: stride, ..Default::default() },
            MeshAttributeCompressionFlags::empty(),
        ));
        world.resource_mut::<RenderAssets<RenderMesh>>().insert(
            mesh,
            RenderMesh { vertex_count: 3, aabb_center: glam::Vec3::ZERO, buffer_info: RenderMeshBufferInfo::NonIndexed, key_bits: key_bits.clone(), layout },
        );
        specimens.push(Specimen { mesh, material, flags: SpecimenFlags::SHADOW_CASTER });
    }
    let per = plan_specimen(&known_views(), &key_bits, &properties, material.type_id(), SpecimenFlags::SHADOW_CASTER).len();
    (world, specimens, per)
}

fn fake_in_progress(world: &World, id: CachedRenderPipelineId) -> bool {
    world.resource::<FakeSpecializers>().compiling.contains(&id)
}

/// With limit K, N specimens never have more than K precache pipelines outstanding (a plan longer
/// than K goes alone: at most its own length), held-back specimens count as queued (so the reveal
/// gate's catalog term holds on them), and all resolve as pipelines finish (one per frame here).
/// Without a limit every prepared specimen resolves at once (W3's behaviour).
#[test]
fn throttle_keeps_at_most_k_precache_pipelines_outstanding() {
    const N: usize = 12;
    let (_, _, per) = throttle_world(1);
    assert!(per >= 3, "main + prepass + shadow pipelines: {per}");

    let (mut world, specimens, _) = throttle_world(N);
    let mut precache = PipelinePrecache::default();
    *precache.views_mut() = known_views();
    for s in &specimens {
        precache.request(0, *s);
    }
    world.insert_resource(precache);
    let mut state = SystemState::<PrecacheParam<'static, 'static>>::new(&mut world);
    precache_specimens_with(&mut world, &mut state, fake_in_progress);
    let p = world.resource::<PipelinePrecache>();
    assert_eq!((p.resolved().len(), p.outstanding(), p.stats().throttled), (N, N * per, 0), "no limit: all at once");

    for k in [1, per, 2 * per + 1] {
        let (mut world, specimens, per) = throttle_world(N);
        let mut precache = PipelinePrecache::default();
        *precache.views_mut() = known_views();
        precache.set_limit(Some(k));
        for s in &specimens {
            precache.request(0, *s);
        }
        world.insert_resource(precache);
        let mut state = SystemState::<PrecacheParam<'static, 'static>>::new(&mut world);
        let main = Precache::default();
        main.request(std::iter::empty(), crate::CATALOG_TAG);
        let mut gate = RevealGate::default();
        let (mut throttled_seen, mut frames) = (false, 0);
        loop {
            precache_specimens_with(&mut world, &mut state, fake_in_progress);
            let compiling = world.resource::<FakeSpecializers>().compiling.clone();
            let p = world.resource::<PipelinePrecache>();
            assert!(p.outstanding() <= k.max(per), "limit {k}: {} outstanding", p.outstanding());
            assert_eq!(p.outstanding(), compiling.len(), "every compiling pipeline is counted");
            let totals = p.stats();
            throttled_seen |= totals.throttled > 0;
            let status = tag_status(1, p.waiting(), p.resolved(), |id| if compiling.contains(&id) { Progress::Compiling } else { Progress::Ready });
            assert_eq!((status[0].queued, status[0].total()), (totals.throttled, u32::try_from(N).unwrap()), "held back = queued");
            main.report(status);
            let s = main.status(crate::CATALOG_TAG);
            let catalog = (s.pending_assets + s.queued + s.compiling) as usize;
            let step = gate.step("loading", "x", 0.016, &RevealInputs { cells: true, textures: true, catalog, ..RevealInputs::default() });
            if catalog > 0 {
                assert_eq!(step, RevealStep::Holding, "limit {k}: the catalog term holds");
            } else if matches!(step, RevealStep::Open { warning: None, .. }) {
                break;
            }
            let mut fake = world.resource_mut::<FakeSpecializers>();
            if !fake.compiling.is_empty() {
                fake.compiling.remove(0);
            }
            frames += 1;
            assert!(frames < 1000, "limit {k}: never settled");
        }
        let p = world.resource::<PipelinePrecache>();
        assert_eq!((p.waiting().len(), p.resolved().len(), p.outstanding()), (0, N, 0), "limit {k}");
        assert_eq!(world.resource::<FakeSpecializers>().len(), N * per, "limit {k}: each pipeline specialised once");
        assert!(throttled_seen, "limit {k} held some back");
    }
}

// ---- (6) W4b1-throughput: the cap applies only while a draw-path pipeline waits ----

/// `n` specimens with distinct layouts requested under one limit, ready for `precache_specimens_with`.
fn limited_world(n: usize, limit: Option<PrecacheLimit>) -> (World, SystemState<PrecacheParam<'static, 'static>>, usize) {
    let (mut world, specimens, per) = throttle_world(n);
    let mut precache = PipelinePrecache::default();
    *precache.views_mut() = known_views();
    precache.set_limits(limit);
    for s in &specimens {
        precache.request(0, *s);
    }
    world.insert_resource(precache);
    let state = SystemState::<PrecacheParam<'static, 'static>>::new(&mut world);
    (world, state, per)
}

/// A draw-path pipeline compiling: in the fake cache's compiling set, noted by the draw path.
fn draw_compiling(world: &mut World, id: CachedRenderPipelineId) {
    world.resource_mut::<FakeSpecializers>().compiling.push(id);
    world.resource_mut::<PipelinePrecache>().note_draw_with(id, PrecachePass::Main, None, true);
}

fn finish(world: &mut World, id: CachedRenderPipelineId) {
    world.resource_mut::<FakeSpecializers>().compiling.retain(|&c| c != id);
}

fn draw_id() -> CachedRenderPipelineId {
    CachedRenderPipelineId::new(1_000_000)
}
const DRAWS: usize = 2_000_000;

/// (a) + throughput: no draw waiting => the precache fills to the full (idle) limit exactly, and
/// refills to it each frame as pipelines finish (one pipeline at a time, a specimen across frames),
/// even when K is no multiple of a specimen's plan (W4b1-fix: one specimen in flight at K=7, plan 5).
#[test]
fn no_draw_waiting_fills_to_the_full_limit() {
    let (mut world, mut state, per) = limited_world(12, Some(PrecacheLimit { idle: 8, draw_waiting: 4 }));
    assert!(per >= 3 && 8 % per != 0, "plan {per}");
    for frame in 0..20 {
        precache_specimens_with(&mut world, &mut state, fake_in_progress);
        let p = world.resource::<PipelinePrecache>();
        let left = 12 * per - world.resource::<FakeSpecializers>().len() + p.outstanding();
        assert_eq!(p.outstanding(), left.min(8), "frame {frame}: refilled to the limit");
        assert_eq!((p.stats().limit, p.stats().draw_waiting), (8, 0));
        let first = world.resource::<FakeSpecializers>().compiling.first().copied();
        if let Some(id) = first {
            finish(&mut world, id);
        }
    }
}

/// (b) a draw-path pipeline waiting => at most half; once it finishes, the full limit again. A draw
/// waiting on a precache pipeline does not count (it needs no extra thread).
#[test]
fn draw_waiting_caps_the_precache_at_half() {
    let (mut world, mut state, _) = limited_world(12, Some(PrecacheLimit { idle: 8, draw_waiting: 4 }));
    draw_compiling(&mut world, draw_id());
    for _ in 0..3 {
        precache_specimens_with(&mut world, &mut state, fake_in_progress);
        let p = world.resource::<PipelinePrecache>();
        assert_eq!((p.outstanding(), p.stats().limit, p.stats().draw_waiting), (4, 4, 1));
    }
    finish(&mut world, draw_id());
    precache_specimens_with(&mut world, &mut state, fake_in_progress);
    let p = world.resource::<PipelinePrecache>();
    assert_eq!((p.outstanding(), p.stats().limit, p.stats().draw_waiting), (8, 8, 0));

    // The draw path now waits on a precache pipeline: still the full limit.
    let ours = world.resource::<FakeSpecializers>().compiling[0];
    world.resource_mut::<PipelinePrecache>().note_draw_with(ours, PrecachePass::Main, None, true);
    precache_specimens_with(&mut world, &mut state, fake_in_progress);
    let p = world.resource::<PipelinePrecache>();
    assert_eq!((p.outstanding(), p.stats().draw_waiting), (8, 0));
}

/// (c) never more than the full limit, nothing added past half while a draw waits, whatever the
/// draws do, and every specimen resolves with each pipeline specialised once.
#[test]
fn never_more_than_the_full_limit() {
    let (mut world, mut state, per) = limited_world(12, Some(PrecacheLimit { idle: 8, draw_waiting: 4 }));
    let mut frames = 0;
    loop {
        // A draw arrives every third frame and compiles for two frames.
        if frames % 3 == 0 {
            draw_compiling(&mut world, CachedRenderPipelineId::new(DRAWS + frames));
        }
        let before = world.resource::<PipelinePrecache>().outstanding();
        precache_specimens_with(&mut world, &mut state, fake_in_progress);
        let p = world.resource::<PipelinePrecache>();
        assert!(p.outstanding() <= 8, "frame {frames}: {}", p.outstanding());
        if p.stats().draw_waiting > 0 {
            assert!(p.outstanding() <= before.max(4), "frame {frames}: nothing added past half while a draw waits");
        }
        if p.waiting().is_empty() && p.outstanding() == 0 {
            break;
        }
        let compiling = world.resource::<FakeSpecializers>().compiling.clone();
        for id in compiling.iter().filter(|id| id.id() >= DRAWS && id.id() + 2 <= DRAWS + frames) {
            finish(&mut world, *id);
        }
        if let Some(&id) = compiling.iter().find(|id| id.id() < DRAWS) {
            finish(&mut world, id);
        }
        frames += 1;
        assert!(frames < 1000, "never settled");
    }
    assert_eq!(world.resource::<PipelinePrecache>().resolved().len(), 12);
    assert_eq!(world.resource::<FakeSpecializers>().len(), 12 * per, "each pipeline specialised once");
}

/// (d) the limits from the compile threads: full pool, half with a draw waiting, 1 in play.
#[test]
fn precache_limit_full_pool_half_with_a_draw_one_in_play() {
    assert_eq!(crate::precache_limit(14, true), PrecacheLimit { idle: 14, draw_waiting: 7 });
    assert_eq!(crate::precache_limit(1, true), PrecacheLimit { idle: 1, draw_waiting: 1 });
    assert_eq!(crate::precache_limit(0, true), PrecacheLimit { idle: 1, draw_waiting: 1 });
    assert_eq!(crate::precache_limit(14, false), PrecacheLimit::uniform(1));
    assert_eq!(crate::precache_limit(4, false), PrecacheLimit::uniform(1));
    // In play the render world submits one pipeline at a time, draw or not.
    let (mut world, mut state, _) = limited_world(3, Some(crate::precache_limit(4, false)));
    precache_specimens_with(&mut world, &mut state, fake_in_progress);
    assert_eq!(world.resource::<PipelinePrecache>().outstanding(), 1);
    draw_compiling(&mut world, draw_id());
    precache_specimens_with(&mut world, &mut state, fake_in_progress);
    assert_eq!(world.resource::<PipelinePrecache>().outstanding(), 1);
}

/// Throughput: a hash hit (a specimen sharing another's pipelines) costs no slot; W4b1-fix held it
/// back until its whole plan fit beside the outstanding pipelines.
#[test]
fn hash_hits_cost_no_slot() {
    let (mut world, specimens, per) = throttle_world(1);
    let mut precache = PipelinePrecache::default();
    *precache.views_mut() = known_views();
    precache.set_limit(Some(per + 1));
    precache.request(0, specimens[0]);
    precache.request(1, specimens[0]);
    world.insert_resource(precache);
    let mut state = SystemState::<PrecacheParam<'static, 'static>>::new(&mut world);
    precache_specimens_with(&mut world, &mut state, fake_in_progress);
    let p = world.resource::<PipelinePrecache>();
    assert_eq!((p.resolved().len(), p.outstanding(), p.stats().throttled), (2, per, 0));
    assert_eq!(p.resolved()[0].pipelines, p.resolved()[1].pipelines);
}

/// Throughput: a compile task that has finished frees its slot before the cache's next
/// `process_queue` stores the pipeline; a running one, a queued one or one waiting for its shader
/// holds it.
#[test]
fn finished_compile_task_is_not_in_progress() {
    use bevy_pbr::precache::pipeline_in_progress;
    use bevy_render::render_resource::CachedPipelineState;
    use bevy_shader::ShaderCacheError;

    assert!(pipeline_in_progress(&CachedPipelineState::Queued));
    assert!(pipeline_in_progress(&CachedPipelineState::Err(ShaderCacheError::ShaderImportNotYetAvailable)));
    let pool = bevy_tasks::TaskPool::new();
    let done = CachedPipelineState::Creating(pool.spawn(async { Err(ShaderCacheError::ShaderImportNotYetAvailable) }));
    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(10);
    while pipeline_in_progress(&done) {
        assert!(std::time::Instant::now() < deadline, "the task never finished");
        std::thread::sleep(std::time::Duration::from_millis(1));
    }
    let running = CachedPipelineState::Creating(pool.spawn(async {
        bevy_tasks::futures_lite::future::pending::<()>().await;
        Err(ShaderCacheError::ShaderImportNotYetAvailable)
    }));
    assert!(pipeline_in_progress(&running), "a running task holds its slot");
}

/// Misses: a draw specialised in the frame its specimen is submitted settles as a hit; an
/// unpredicted one stays missed (and is logged once, "precache missed: ...").
#[test]
fn same_frame_draw_settles_as_hit_and_misses_are_logged_once() {
    let (mut world, mut state, _) = limited_world(1, None);
    let first = CachedRenderPipelineId::new(1); // the fake cache's first id
    let unpredicted = CachedRenderPipelineId::new(999);
    let material_key = ErasedMaterialKey::new(7u32);
    {
        let mut p = world.resource_mut::<PipelinePrecache>();
        p.note_draw_with(first, PrecachePass::Main, None, false);
        p.note_draw_with(unpredicted, PrecachePass::Main, Some((main_views()[0], TypeId::of::<u32>(), &material_key)), false);
        p.note_draw_with(unpredicted, PrecachePass::Main, None, false);
        assert_eq!((p.stats().hit, p.stats().missed), (0, 2));
    }
    precache_specimens_with(&mut world, &mut state, fake_in_progress);
    let p = world.resource::<PipelinePrecache>();
    assert_eq!((p.stats().hit, p.stats().missed), (1, 1));
}
