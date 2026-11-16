//! CW-7 (ROADMAP 1.25): vendored `bevy_pbr` skin.rs "skip unchanged skins". Joint blocks whose matrices did
//! not change are neither walked nor uploaded, and the two GPU joint buffers (current and previous frame, for
//! motion vectors) stay consistent. The GPU side is modelled by [`Gpu`]: two buffers swapped every frame and
//! written exactly as `prepare_skins` writes them from `SkinJointBlocks::plan_upload`.

use bevy::math::{Affine3A, Mat4};
use bevy::mesh::skinning::{SkinnedMesh, SkinnedMeshInverseBindposes};
use bevy::pbr::{QuerySkinTree, SkinJointBlocks, SkinTree};
use bevy::platform::collections::HashMap;
use bevy::prelude::*;
use bevy::render::sync_world::MainEntity;
use bevy::transform::components::TransformTreeChanged;

use super::{app, npc, run_until};
use crate::{CharacterBody, CharacterState};

/// The two joint buffers as `prepare_skins` swaps and writes them.
#[derive(Default)]
struct Gpu {
    buffers: [Vec<Mat4>; 2],
    current: usize,
    /// Joints written by the last frame.
    written: usize,
}

impl Gpu {
    /// One `prepare_skins` (no buffer growth unless `resized`).
    fn frame(&mut self, blocks: &mut SkinJointBlocks, resized: bool) {
        let staging = blocks.staging_buffer().to_vec();
        if staging.is_empty() {
            return;
        }
        self.current ^= 1;
        let plan = blocks.plan_upload(resized);
        self.written = 0;
        for (buffer, ranges) in [(self.current, &plan.current), (self.current ^ 1, &plan.prev)] {
            let buffer = &mut self.buffers[buffer];
            if buffer.len() < staging.len() {
                buffer.resize(staging.len(), Mat4::from_cols_array(&[f32::NAN; 16]));
            }
            for range in ranges {
                buffer[range.clone()].copy_from_slice(&staging[range.clone()]);
                self.written += range.len();
            }
        }
        assert_eq!(self.written, blocks.last_upload().joints, "the stats count what was written");
    }

    fn current(&self, at: usize, n: usize) -> &[Mat4] {
        &self.buffers[self.current][at..at + n]
    }

    fn prev(&self, at: usize, n: usize) -> &[Mat4] {
        &self.buffers[self.current ^ 1][at..at + n]
    }
}

/// A flat test rig: joints with poses the test sets, one skin per character.
struct Rig {
    world: World,
    assets: Assets<SkinnedMeshInverseBindposes>,
    poses: HashMap<Entity, Affine3A>,
    /// Joints whose pose was set since the last extraction (`Changed<GlobalTransform>`).
    changed: Vec<Entity>,
}

impl Rig {
    fn new() -> Self {
        Self { world: World::new(), assets: Assets::default(), poses: HashMap::default(), changed: Vec::new() }
    }

    fn character(&mut self, joints: usize) -> (Entity, SkinnedMesh) {
        let joints: Vec<Entity> = (0..joints).map(|_| self.world.spawn_empty().id()).collect();
        for (k, &j) in joints.iter().enumerate() {
            #[allow(clippy::cast_precision_loss)]
            self.poses.insert(j, Affine3A::from_translation(Vec3::new(k as f32, 0.0, 0.0)));
        }
        let inverse = self.assets.add(SkinnedMeshInverseBindposes::from(vec![Mat4::IDENTITY; joints.len()]));
        (self.world.spawn_empty().id(), SkinnedMesh { inverse_bindposes: inverse, joints })
    }

    fn add(&mut self, blocks: &mut SkinJointBlocks, mesh: Entity, skin: &SkinnedMesh) {
        let poses = &self.poses;
        blocks.add_skin(mesh.into(), skin, &self.assets, |j| poses.get(&j).copied());
    }

    fn pose(&mut self, joint: Entity, at: Vec3) {
        self.poses.insert(joint, Affine3A::from_translation(at));
        self.changed.push(joint);
    }

    /// `extract_skins`' joint pass (ungated: these tests check the upload side).
    fn extract(&mut self, blocks: &mut SkinJointBlocks) {
        let changed = std::mem::take(&mut self.changed);
        let poses = &self.poses;
        blocks.begin_extract();
        blocks.extract_changed_joints(&self.assets, |j| changed.contains(&j).then(|| poses[&j]));
    }

    fn matrices(&self, skin: &SkinnedMesh) -> Vec<Mat4> {
        skin.joints.iter().map(|j| Mat4::from(self.poses[j])).collect()
    }
}

fn at(blocks: &SkinJointBlocks, mesh: Entity) -> usize {
    blocks.skin_index(MainEntity::from(mesh)).unwrap() as usize
}

/// Frozen (A3) characters are uploaded on the frames that fill both buffers, then never again; a pose change
/// is written into each buffer once; motion vectors read the frozen pose as "previous" when a character
/// resumes, so there is no spike.
#[test]
fn unchanged_skins_are_not_uploaded_and_previous_joints_stay_correct() {
    let mut rig = Rig::new();
    let mut blocks = SkinJointBlocks::default();
    blocks.set_skip_unchanged(true);
    let mut gpu = Gpu::default();
    let (ma, a) = rig.character(30);
    let (mb, b) = rig.character(30);
    blocks.begin_extract();
    rig.add(&mut blocks, ma, &a);
    rig.add(&mut blocks, mb, &b);
    rig.extract(&mut blocks);
    gpu.frame(&mut blocks, false);
    let (pa, pb, n) = (at(&blocks, ma), at(&blocks, mb), 30);
    // First frame: both buffers hold the new blocks (zero motion, not stale data).
    assert_eq!(gpu.current(pa, n), rig.matrices(&a));
    assert_eq!(gpu.prev(pa, n), rig.matrices(&a), "new block: previous = current on its first frame");
    assert_eq!(gpu.prev(pb, n), rig.matrices(&b));

    // Frozen: nothing written, zero motion.
    for _ in 0..3 {
        rig.extract(&mut blocks);
        gpu.frame(&mut blocks, false);
        assert_eq!(gpu.written, 0, "no joint write for unchanged poses");
        assert_eq!(gpu.current(pa, n), gpu.prev(pa, n));
        assert_eq!(gpu.current(pa, n), rig.matrices(&a));
    }

    // A resumes: frame 1 writes its block into current; previous is the frozen pose (no MV spike).
    let frozen = rig.matrices(&a);
    rig.pose(a.joints[3], Vec3::new(9.0, 1.0, 0.0));
    rig.extract(&mut blocks);
    gpu.frame(&mut blocks, false);
    assert_eq!(blocks.blocks_changed(), 1);
    assert_eq!(gpu.written, 32, "one block (30 joints, padded to 32)");
    assert_eq!(gpu.current(pa, n), rig.matrices(&a));
    assert_eq!(gpu.prev(pa, n), frozen, "previous frame = the frozen pose");
    assert_eq!(gpu.current(pb, n), rig.matrices(&b), "B untouched and still right");
    // Frame 2 (pose held): the other buffer catches up; motion is zero again.
    rig.extract(&mut blocks);
    gpu.frame(&mut blocks, false);
    assert_eq!(gpu.written, 32);
    assert_eq!(gpu.current(pa, n), rig.matrices(&a));
    assert_eq!(gpu.prev(pa, n), rig.matrices(&a));
    // Frame 3: nothing left to write.
    rig.extract(&mut blocks);
    gpu.frame(&mut blocks, false);
    assert_eq!(gpu.written, 0);

    // Moving every frame: written every frame, previous = last frame's pose.
    for step in 0..4 {
        let last = rig.matrices(&a);
        #[allow(clippy::cast_precision_loss)]
        rig.pose(a.joints[0], Vec3::splat(step as f32));
        rig.extract(&mut blocks);
        gpu.frame(&mut blocks, false);
        assert_eq!(gpu.current(pa, n), rig.matrices(&a));
        assert_eq!(gpu.prev(pa, n), last);
    }

    // A rewrite with an equal value (propagation marks whole subtrees) is no change.
    rig.extract(&mut blocks);
    gpu.frame(&mut blocks, false);
    rig.extract(&mut blocks);
    gpu.frame(&mut blocks, false);
    let same = rig.poses[&b.joints[5]].translation;
    rig.pose(b.joints[5], same.into());
    rig.extract(&mut blocks);
    gpu.frame(&mut blocks, false);
    assert_eq!((blocks.blocks_changed(), gpu.written), (0, 0));

    // Buffer growth: everything into both buffers (stock), then quiet again.
    gpu.frame(&mut blocks, true);
    assert_eq!(gpu.written, 2 * blocks.staging_buffer().len());
    rig.extract(&mut blocks);
    gpu.frame(&mut blocks, false);
    assert_eq!(gpu.written, 0);
}

/// Despawn and respawn: a freed block's offset is reused by another character, whose first frame writes both
/// buffers (no motion vector from the departed character's joints).
#[test]
fn respawned_skins_get_both_buffers_on_their_first_frame() {
    let mut rig = Rig::new();
    let mut blocks = SkinJointBlocks::default();
    blocks.set_skip_unchanged(true);
    let mut gpu = Gpu::default();
    let (ma, a) = rig.character(8);
    blocks.begin_extract();
    rig.add(&mut blocks, ma, &a);
    gpu.frame(&mut blocks, false);
    let offset = at(&blocks, ma);
    blocks.remove_skin(ma.into());
    let (mc, mut c) = rig.character(8);
    for j in &c.joints {
        rig.poses.insert(*j, Affine3A::from_translation(Vec3::new(50.0, 2.0, 1.0)));
    }
    c.joints.rotate_left(1);
    blocks.begin_extract();
    rig.add(&mut blocks, mc, &c);
    assert_eq!(at(&blocks, mc), offset, "the freed offset is reused");
    rig.extract(&mut blocks);
    gpu.frame(&mut blocks, false);
    assert_eq!(gpu.current(offset, 8), rig.matrices(&c));
    assert_eq!(gpu.prev(offset, 8), rig.matrices(&c), "previous is not the old character's pose");
    rig.extract(&mut blocks);
    gpu.frame(&mut blocks, false);
    assert_eq!(gpu.written, 0);
    // Same character back (visibility toggled): refilled, so uploaded again into both buffers over 2 frames.
    blocks.remove_skin(mc.into());
    blocks.begin_extract();
    rig.add(&mut blocks, mc, &c);
    rig.extract(&mut blocks);
    gpu.frame(&mut blocks, false);
    assert_eq!(gpu.current(offset, 8), rig.matrices(&c));
    assert_eq!(gpu.prev(offset, 8), rig.matrices(&c));
}

/// Switch off: stock behaviour (whole staging buffer into current every frame). Turning it on mid-run keeps
/// the buffers right (pending writes are tracked either way).
#[test]
fn skip_off_uploads_everything_and_switching_on_mid_run_stays_correct() {
    let mut rig = Rig::new();
    let mut blocks = SkinJointBlocks::default();
    let mut gpu = Gpu::default();
    let (ma, a) = rig.character(12);
    blocks.begin_extract();
    rig.add(&mut blocks, ma, &a);
    gpu.frame(&mut blocks, false);
    let (pa, n) = (at(&blocks, ma), 12);
    for _ in 0..2 {
        rig.extract(&mut blocks);
        gpu.frame(&mut blocks, false);
        assert_eq!(gpu.written, blocks.staging_buffer().len(), "off: the whole staging buffer");
    }
    // Change while off, then on the next frame.
    let last = rig.matrices(&a);
    rig.pose(a.joints[1], Vec3::new(3.0, 3.0, 3.0));
    rig.extract(&mut blocks);
    gpu.frame(&mut blocks, false);
    assert_eq!(gpu.prev(pa, n), last);
    blocks.set_skip_unchanged(true);
    rig.extract(&mut blocks);
    gpu.frame(&mut blocks, false);
    assert_eq!(gpu.current(pa, n), rig.matrices(&a), "the stale buffer is caught up after the switch");
    assert_eq!(gpu.prev(pa, n), rig.matrices(&a));
    rig.extract(&mut blocks);
    gpu.frame(&mut blocks, false);
    assert_eq!(gpu.written, 0);
}

/// Counters from the gate probe, per frame.
#[derive(Resource, Default)]
struct Probe {
    blocks: SkinJointBlocks,
    started: bool,
    /// (walked, gated, changed) per frame.
    frames: Vec<(usize, usize, usize)>,
}

/// `extract_skins`' joint pass inside the main world, after transform propagation, with the real gate
/// ([`QuerySkinTree`]) and the same change-detection window (one run per frame).
#[allow(clippy::needless_pass_by_value, clippy::type_complexity)]
fn probe(
    mut probe: ResMut<Probe>,
    skins: Query<(Entity, &SkinnedMesh)>,
    assets: Res<Assets<SkinnedMeshInverseBindposes>>,
    all: Query<&GlobalTransform>,
    changed: Query<&GlobalTransform, Changed<GlobalTransform>>,
    trees: Query<(Ref<'static, GlobalTransform>, Option<Ref<'static, TransformTreeChanged>>)>,
    parents: Query<&'static ChildOf>,
) {
    let probe = &mut *probe;
    probe.blocks.begin_extract();
    if !probe.started {
        probe.started = true;
        probe.blocks.set_skip_unchanged(true);
        for (mesh, skin) in &skins {
            probe.blocks.add_skin(mesh.into(), skin, &assets, |j| all.get(j).ok().map(GlobalTransform::affine));
        }
    }
    let tree = QuerySkinTree { trees: &trees, parents: &parents };
    probe.blocks.extract_changed_joints_gated(&assets, |j| changed.get(j).ok().map(GlobalTransform::affine), &tree);
    let b = &probe.blocks;
    let frame = (b.blocks_walked(), b.blocks_gated(), b.blocks_changed());
    probe.frames.push(frame);
}

fn last(app: &App) -> (usize, usize, usize) {
    *app.world().resource::<Probe>().frames.last().unwrap()
}

/// The changed-tree gate on a real character (Bevy's transform propagation and `TransformTreeChanged`): a
/// character frozen at A3 is not walked; moving its root or writing one joint is seen.
#[test]
fn frozen_character_is_not_walked_and_any_transform_change_is_seen() {
    let mut app = app();
    let root = app.world_mut().spawn(npc(CharacterState::default())).id();
    run_until(&mut app, |w| w.get::<CharacterBody>(root).is_some());
    let body = app.world().get::<CharacterBody>(root).unwrap().body;
    app.world_mut().get_mut::<sf_anim_bevy::AnimLod>(body).unwrap().forced = Some(sf_anim_bevy::AnimTier::A3);
    app.init_resource::<Probe>().add_systems(Last, probe);
    for _ in 0..6 {
        app.update();
    }
    let blocks = app.world().resource::<Probe>().blocks.block_count();
    assert_eq!(blocks, 1, "one palette (P5)");
    assert_eq!(last(&app), (0, 1, 0), "frozen: gated, not walked");
    assert!(app.world().get::<SkinnedMesh>(app.world().get::<CharacterBody>(root).unwrap().meshes[0]).is_some());

    // Root moves: every joint's global transform changes.
    app.world_mut().get_mut::<Transform>(root).unwrap().translation.x += 2.0;
    app.update();
    assert_eq!(last(&app), (1, 0, 1), "a moved root is walked and changes the block");
    app.update();
    assert_eq!(last(&app), (0, 1, 0));

    // One joint (deep in the skeleton) moves.
    let mesh = app.world().get::<CharacterBody>(root).unwrap().meshes[0];
    let joint = *app.world().get::<SkinnedMesh>(mesh).unwrap().joints.last().unwrap();
    app.world_mut().get_mut::<Transform>(joint).unwrap().rotate_x(0.3);
    app.update();
    assert_eq!(last(&app), (1, 0, 1), "a joint write is seen through TransformTreeChanged");
    app.update();
    assert_eq!(last(&app), (0, 1, 0));

    // The gate's tree finds the skeleton: the joints' common ancestor is under the character root.
    let world = app.world_mut();
    let mut state = bevy::ecs::system::SystemState::<(
        Query<(Ref<GlobalTransform>, Option<Ref<TransformTreeChanged>>)>,
        Query<&ChildOf>,
    )>::new(world);
    let (trees, parents) = state.get(world).unwrap();
    let tree = QuerySkinTree { trees: &trees, parents: &parents };
    let mut up = joint;
    let mut depth = 0;
    while let Some(p) = tree.parent(up) {
        up = p;
        depth += 1;
    }
    assert!(depth > 1, "joints hang under the character root");
}
