//! River flow (`flow.rs`): the CPU twin of the shader's two-phase advection, the uniform slots,
//! and the vertex layout that carries the flow attribute only on the mesh-body path.

use bevy_asset::RenderAssetUsages;
use bevy_material::descriptor::{FragmentState, RenderPipelineDescriptor, VertexState};
use bevy_mesh::{Mesh, MeshVertexBufferLayoutRef, MeshVertexBufferLayouts, PrimitiveTopology, VertexBufferLayout, VertexFormat};
use bevy_render::render_resource::VertexAttribute;
use bevy_shader::ShaderDefVal;

use crate::flow::{phases, slots, WaterFlowSettings};
use crate::material::{flow_attribute, specialize_water, FLOW_LOCATION, WATER_FLOW_DEF, WATER_MESH_DEF};
use crate::mesh::ATTRIBUTE_WATER_FLOW;

/// The two phases are half a cycle apart, their weights sum to 1 and vanish where a phase wraps.
#[test]
fn phases_are_half_a_cycle_apart_and_weights_sum_to_one() {
    for i in -400..=400 {
        #[allow(clippy::cast_precision_loss)]
        let t = i as f32 * 0.0137;
        let [(a, wa), (b, wb)] = phases(t);
        assert!((0.0..1.0).contains(&a) && (0.0..1.0).contains(&b), "{t}: {a} {b}");
        let apart = (b - a).rem_euclid(1.0);
        assert!((apart - 0.5).abs() < 1e-5, "{t}: {apart}");
        assert!((wa + wb - 1.0).abs() < 1e-5, "{t}: {wa} + {wb}");
        assert!((0.0..=1.0).contains(&wa) && (0.0..=1.0).contains(&wb));
    }
    // Each weight is 0 as its phase wraps and 1 half-way.
    assert!(phases(0.0)[0].1.abs() < 1e-6 && (phases(0.0)[1].1 - 1.0).abs() < 1e-6);
    assert!((phases(0.5)[0].1 - 1.0).abs() < 1e-6 && phases(0.5)[1].1.abs() < 1e-6);
}

/// The blend is continuous in time: a smooth field read at the two advected offsets and
/// cross-faded never jumps, not even where a phase wraps (its offset jumps back while its weight is 0).
#[test]
fn advected_blend_is_continuous_in_time() {
    // A ripple field along the flow, and a flow of 3 m per cycle.
    let field = |x: f32| (x * 2.1).sin() + 0.5 * (x * 5.3).cos();
    let travel = 3.0;
    let blend = |t: f32| phases(t).iter().map(|&(p, w)| w * field(-travel * p)).sum::<f32>();
    let dt = 1e-3;
    let mut worst = 0.0f32;
    for i in 0..4000 {
        #[allow(clippy::cast_precision_loss)]
        let t = i as f32 * dt;
        worst = worst.max((blend(t + dt) - blend(t)).abs());
    }
    // The field's slope bound × the offset rate (travel / cycle) × dt, plus the weights' rate × the
    // field's range: a jump would be of order 1.
    assert!(worst < 0.05, "the blend jumps by {worst} in {dt} of a cycle");
    // Zero flow: both taps read the same point, the blend is the field there.
    let still = phases(0.3).iter().map(|&(_, w)| w * field(0.0)).sum::<f32>();
    assert!((still - field(0.0)).abs() < 1e-6);
}

/// The uniform's clock wraps on whole cycles (the shader's phase never jumps), the slots carry the
/// tunables' defaults, and bad binds are clamped (no division by zero, foam full above its start).
#[test]
#[allow(clippy::float_cmp)] // exact: the defaults pass through
fn slots_wrap_on_whole_cycles_and_clamp() {
    let s = WaterFlowSettings::default();
    let [flow, flow2, flow3] = slots(0.0, &s);
    assert_eq!((flow.y, flow.z, flow.w), (s.cycle_s, s.advect_scale, s.phase_noise));
    assert_eq!((flow2.y, flow2.z, flow2.w, flow3.x), (s.normal_gain, s.foam_start_mps, s.foam_full_mps, s.foam));
    assert_eq!(flow2.x.fract(), 0.0, "noise cells not whole per noise period");
    let cycle = f64::from(s.cycle_s);
    for t in [0.3, 1.9, 12_345.678, 1.0e6 + 0.25] {
        let a = slots(t, &s)[0].x / s.cycle_s;
        let b = slots(t + cycle * 7.0, &s)[0].x / s.cycle_s;
        assert!((a - b).abs() < 1e-3 && (0.0..=1.0).contains(&a), "{t}: {a} vs {b}");
    }
    let bad = WaterFlowSettings { cycle_s: 0.0, foam_start_mps: 3.0, foam_full_mps: 1.0, phase_noise: 4.0, foam: -1.0, ..s };
    let [flow, flow2, flow3] = slots(5.0, &bad);
    assert!(flow.y > 0.0 && flow.x < flow.y);
    assert!(flow2.w > flow2.z);
    assert_eq!((flow.w, flow3.x), (1.0, 0.0));
    let nan = WaterFlowSettings { cycle_s: f32::NAN, ..s };
    assert!(slots(5.0, &nan)[0].y > 0.0);
}

/// A body mesh with `flow` (the attribute) or without, its layout as Bevy keys the pipeline.
fn layout(flow: bool) -> MeshVertexBufferLayoutRef {
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::RENDER_WORLD);
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vec![[0.0f32; 3]; 3]);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, vec![[0.0f32, 1.0, 0.0]; 3]);
    if flow {
        mesh.insert_attribute(ATTRIBUTE_WATER_FLOW, vec![[1.0f32, 0.0]; 3]);
    }
    mesh.get_mesh_vertex_buffer_layout(&mut MeshVertexBufferLayouts::default())
}

/// A descriptor as Bevy's mesh pipeline leaves it: the mesh buffer with position and normal.
fn descriptor(layout: &MeshVertexBufferLayoutRef) -> RenderPipelineDescriptor {
    let base = layout.0.get_layout(&[Mesh::ATTRIBUTE_POSITION.at_shader_location(0), Mesh::ATTRIBUTE_NORMAL.at_shader_location(1)]).expect("base layout");
    RenderPipelineDescriptor { vertex: VertexState { buffers: vec![base], ..Default::default() }, fragment: Some(FragmentState::default()), ..Default::default() }
}

fn defs(d: &RenderPipelineDescriptor) -> (Vec<ShaderDefVal>, Vec<ShaderDefVal>) {
    (d.vertex.shader_defs.clone(), d.fragment.as_ref().map(|f| f.shader_defs.clone()).unwrap_or_default())
}

fn has(defs: &[ShaderDefVal], name: &'static str) -> bool {
    defs.contains(&ShaderDefVal::from(name))
}

fn flow_in(buffer: &VertexBufferLayout) -> Option<&VertexAttribute> {
    buffer.attributes.iter().find(|a| a.shader_location == FLOW_LOCATION)
}

/// The vertex layout carries the flow attribute (`Float32x2` at `FLOW_LOCATION`, in the mesh's own
/// interleaved buffer) and both stages get `WATER_FLOW` only on the mesh path of a mesh that has
/// it; a lake mesh without it and the clipmap keep today's layout and defs.
#[test]
fn flow_attribute_only_on_the_mesh_path() {
    let with = layout(true);
    let without = layout(false);
    let a = flow_attribute(true, &with).expect("the flow attribute on the mesh path");
    assert_eq!((a.format, a.shader_location), (VertexFormat::Float32x2, FLOW_LOCATION));
    assert!(flow_attribute(false, &with).is_none(), "the clipmap path reads no flow");
    assert!(flow_attribute(true, &without).is_none(), "a mesh without flow has none");

    let mut river = descriptor(&with);
    specialize_water(&mut river, &with, true);
    let buffer = &river.vertex.buffers[0];
    let flow = flow_in(buffer).expect("flow in the vertex buffer");
    assert_eq!(flow.format, VertexFormat::Float32x2);
    assert!(flow.offset + 8 <= buffer.array_stride, "the flow lies outside the mesh's vertex");
    assert!(buffer.attributes.iter().filter(|x| x.shader_location != FLOW_LOCATION).all(|x| x.offset != flow.offset), "the flow aliases another attribute");
    let (v, f) = defs(&river);
    assert!(has(&v, WATER_MESH_DEF) && has(&f, WATER_MESH_DEF));
    assert!(has(&v, WATER_FLOW_DEF) && has(&f, WATER_FLOW_DEF), "{v:?} {f:?}");

    let mut lake = descriptor(&without);
    let before = lake.vertex.buffers.clone();
    specialize_water(&mut lake, &without, true);
    assert_eq!(lake.vertex.buffers, before, "a still mesh's layout changed");
    let (v, f) = defs(&lake);
    assert!(has(&v, WATER_MESH_DEF) && !has(&v, WATER_FLOW_DEF) && !has(&f, WATER_FLOW_DEF));

    let mut clipmap = descriptor(&with);
    let before = clipmap.vertex.buffers.clone();
    specialize_water(&mut clipmap, &with, false);
    assert_eq!(clipmap.vertex.buffers, before, "the clipmap's layout changed");
    let (v, f) = defs(&clipmap);
    assert!(v.is_empty() && f.is_empty(), "{v:?} {f:?}");
    // Both paths keep the water's double-sided, depth-writing draw.
    assert!(river.primitive.cull_mode.is_none() && clipmap.primitive.cull_mode.is_none());
}
