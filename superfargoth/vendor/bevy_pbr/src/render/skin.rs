use core::mem::{self, size_of};

use bevy_asset::{prelude::AssetChanged, AssetId, Assets};
use bevy_camera::visibility::ViewVisibility;
use bevy_ecs::prelude::*;
use bevy_math::{Affine3A, Mat4};
use bevy_mesh::skinning::{SkinnedMesh, SkinnedMeshInverseBindposes};
// superfargoth patch (own, P5: shared skin joint blocks): block lookup by binding.
use bevy_platform::collections::HashMap;
use bevy_render::render_resource::{Buffer, BufferDescriptor};
use bevy_render::settings::WgpuLimits;
use bevy_render::sync_world::{MainEntity, MainEntityHashMap};
use bevy_render::{
    batching::NoAutomaticBatching,
    render_resource::BufferUsages,
    renderer::{RenderDevice, RenderQueue},
    Extract,
};
use bevy_transform::prelude::GlobalTransform;
use offset_allocator::{Allocation, Allocator};
use tracing::error;

/// Maximum number of joints supported for skinned meshes.
///
/// It is used to allocate buffers.
/// The correctness of the value depends on the GPU/platform.
/// The current value is chosen because it is guaranteed to work everywhere.
/// To allow for bigger values, a check must be made for the limits
/// of the GPU at runtime, which would mean not using consts anymore.
pub const MAX_JOINTS: usize = 256;

/// The total number of joints we support.
///
/// This is 256 GiB worth of joint matrices, which we will never hit under any
/// reasonable circumstances.
const MAX_TOTAL_JOINTS: u32 = 1024 * 1024 * 1024;

/// The number of joints that we allocate at a time.
///
/// Some hardware requires that uniforms be allocated on 256-byte boundaries, so
/// we need to allocate 4 64-byte matrices at a time to satisfy alignment
/// requirements.
const JOINTS_PER_ALLOCATION_UNIT: u32 = (256 / size_of::<Mat4>()) as u32;

/// The location of the first joint matrix in the skin uniform buffer.
#[derive(Clone, Copy)]
pub struct SkinByteOffset {
    /// The byte offset of the first joint matrix.
    pub byte_offset: u32,
}

impl SkinByteOffset {
    /// Index to be in address space based on the size of a skin uniform.
    const fn from_index(index: usize) -> Self {
        SkinByteOffset {
            byte_offset: (index * size_of::<Mat4>()) as u32,
        }
    }

    /// Returns this skin index in elements (not bytes).
    ///
    /// Each element is a 4x4 matrix.
    pub fn index(&self) -> u32 {
        self.byte_offset / size_of::<Mat4>() as u32
    }
}

/// The GPU buffers containing joint matrices for all skinned meshes.
///
/// This is double-buffered: we store the joint matrices of each mesh for the
/// previous frame in addition to those of each mesh for the current frame. This
/// is for motion vector calculation. Every frame, we swap buffers and overwrite
/// the joint matrix buffer from two frames ago with the data for the current
/// frame.
///
/// Notes on implementation: see comment on top of the `extract_skins` system.
///
/// superfargoth patch (own, P5: shared skin joint blocks): the CPU side (staging
/// buffer, allocator, per-skin info) moved into [`SkinJointBlocks`], where
/// skinned meshes with the same skeleton binding share one joint block.
#[derive(Resource)]
pub struct SkinUniforms {
    /// The GPU-side buffer that stores the joint matrices for skinned meshes in
    /// the current frame.
    pub current_buffer: Buffer,
    /// The GPU-side buffer that stores the joint matrices for skinned meshes in
    /// the previous frame.
    pub prev_buffer: Buffer,
    /// superfargoth patch (own, P5: shared skin joint blocks): joint blocks,
    /// their placement in [`Self::current_buffer`] and the CPU staging copy.
    pub blocks: SkinJointBlocks,
}

pub fn skin_uniforms_from_world(device: Res<RenderDevice>, mut commands: Commands) {
    let buffer_usages = (if skins_use_uniform_buffers(&device.limits()) {
        BufferUsages::UNIFORM
    } else {
        BufferUsages::STORAGE
    }) | BufferUsages::COPY_DST;

    // Create the current and previous buffer with the minimum sizes.
    //
    // These will be swapped every frame.
    let current_buffer = device.create_buffer(&BufferDescriptor {
        label: Some("skin uniform buffer"),
        size: MAX_JOINTS as u64 * size_of::<Mat4>() as u64,
        usage: buffer_usages,
        mapped_at_creation: false,
    });
    let prev_buffer = device.create_buffer(&BufferDescriptor {
        label: Some("skin uniform buffer"),
        size: MAX_JOINTS as u64 * size_of::<Mat4>() as u64,
        usage: buffer_usages,
        mapped_at_creation: false,
    });

    let res = SkinUniforms {
        current_buffer,
        prev_buffer,
        blocks: SkinJointBlocks::default(),
    };

    commands.insert_resource(res);
}

impl SkinUniforms {
    /// Returns the current offset in joints of the skin in the buffer.
    pub fn skin_index(&self, skin: MainEntity) -> Option<u32> {
        self.blocks.skin_index(skin)
    }

    /// Returns the current offset in bytes of the skin in the buffer.
    pub fn skin_byte_offset(&self, skin: MainEntity) -> Option<SkinByteOffset> {
        self.blocks
            .skin_index(skin)
            .map(|index| SkinByteOffset::from_index(index as usize))
    }

    /// Returns an iterator over all skins in the scene.
    pub fn all_skins(&self) -> impl Iterator<Item = &MainEntity> {
        self.blocks.skins()
    }
}

// superfargoth patch (own, P5: shared skin joint blocks) begin.
//
// Stock Bevy 0.20 (and main as of bevyengine/bevy#23283) keeps one joint block
// per skinned-mesh entity. A character split into several meshes (one per
// material) on one skeleton then extracts, stores and uploads the same joint
// matrices once per mesh. Here a block is keyed by its skeleton binding: the
// inverse-bindposes asset and the joint entities in order. Equal keys give
// bit-identical matrices, so the meshes share one block (refcounted), the joint
// loop runs once per block and the staging buffer (uploaded whole every frame)
// shrinks accordingly. Previous-frame matrices (motion vectors) are read at the
// same offset in `prev_buffer`, so sharing keeps them correct.

/// What makes two skins' joint matrices identical: the same inverse-bindposes
/// asset and the same joint entities in the same order.
#[derive(Clone, PartialEq, Eq, Hash)]
struct SkinBindingKey {
    inverse_bindposes: AssetId<SkinnedMeshInverseBindposes>,
    joints: Box<[Entity]>,
}

/// One joint block in the buffers, shared by every skin with its binding.
struct SkinBlock {
    /// The allocation of the joints within the [`SkinUniforms::current_buffer`].
    allocation: Allocation,
    /// The binding, which also holds the joint entities.
    key: SkinBindingKey,
    /// Skins (visible skinned-mesh entities) using this block.
    users: u32,
    /// Filled from the full joint transforms this frame, so the changed-joint
    /// pass can skip it.
    fresh: bool,
}

impl SkinBlock {
    /// The offset in joints within the staging buffer.
    fn offset(&self) -> u32 {
        self.allocation.offset * JOINTS_PER_ALLOCATION_UNIT
    }
}

/// Allocation information about each skin.
#[derive(Clone, Copy)]
struct SkinUniformInfo {
    /// Index of the skin's block in [`SkinJointBlocks::blocks`].
    block: u32,
    /// The block's offset in joints (cached for the hot lookups).
    offset: u32,
}

/// The CPU side of [`SkinUniforms`]: joint blocks shared by skins with the same
/// skeleton binding, their allocation, and the staging buffer that
/// [`prepare_skins`] uploads.
pub struct SkinJointBlocks {
    /// The CPU-side buffer that stores the joint matrices for skinned meshes in
    /// the current frame.
    staging: Vec<Mat4>,
    /// The offset allocator that manages the placement of the joints within the
    /// [`SkinUniforms::current_buffer`].
    allocator: Allocator,
    /// The block of each visible skin.
    skin_uniform_info: MainEntityHashMap<SkinUniformInfo>,
    /// Block slots; `None` is free (listed in `free_blocks`).
    blocks: Vec<Option<SkinBlock>>,
    free_blocks: Vec<u32>,
    block_by_key: HashMap<SkinBindingKey, u32>,
    /// The total number of joints in all blocks.
    total_joints: usize,
    /// Joints visited (matrix lookups) since [`Self::begin_extract`].
    joints_visited: usize,
}

impl Default for SkinJointBlocks {
    fn default() -> Self {
        Self {
            staging: Vec::new(),
            allocator: Allocator::new(MAX_TOTAL_JOINTS),
            skin_uniform_info: MainEntityHashMap::default(),
            blocks: Vec::new(),
            free_blocks: Vec::new(),
            block_by_key: HashMap::default(),
            total_joints: 0,
            joints_visited: 0,
        }
    }
}

impl SkinJointBlocks {
    /// The current offset in joints of the skin's block in the buffer.
    pub fn skin_index(&self, skin: MainEntity) -> Option<u32> {
        self.skin_uniform_info.get(&skin).map(|info| info.offset)
    }

    /// All skins (visible skinned-mesh entities) with a block.
    pub fn skins(&self) -> impl Iterator<Item = &MainEntity> {
        self.skin_uniform_info.keys()
    }

    /// Number of skins with a block.
    pub fn skin_count(&self) -> usize {
        self.skin_uniform_info.len()
    }

    /// Number of live joint blocks (distinct skeleton bindings).
    pub fn block_count(&self) -> usize {
        self.block_by_key.len()
    }

    /// Joints in all live blocks.
    pub fn total_joints(&self) -> usize {
        self.total_joints
    }

    /// Joints visited since the last [`Self::begin_extract`] (one per joint per
    /// block, not per skin).
    pub fn joints_visited(&self) -> usize {
        self.joints_visited
    }

    /// The staging copy of the joint matrices, uploaded whole each frame.
    pub fn staging_buffer(&self) -> &[Mat4] {
        &self.staging
    }

    /// Starts a frame's extraction (resets [`Self::joints_visited`]).
    pub fn begin_extract(&mut self) {
        self.joints_visited = 0;
    }

    /// Gives `skin` the block of its binding, creating it if needed, and
    /// (re)fills that block from `joint_transform` (the joint's current global
    /// transform). Refilling on every add keeps a block current when its
    /// inverse-bindposes asset changed: its users are removed and re-added.
    pub fn add_skin(
        &mut self,
        skin: MainEntity,
        skinned_mesh: &SkinnedMesh,
        inverse_bindposes: &Assets<SkinnedMeshInverseBindposes>,
        joint_transform: impl Fn(Entity) -> Option<Affine3A>,
    ) {
        self.remove_skin(skin);
        let key = SkinBindingKey {
            inverse_bindposes: skinned_mesh.inverse_bindposes.id(),
            joints: skinned_mesh.joints.as_slice().into(),
        };
        let block_index = if let Some(&index) = self.block_by_key.get(&key) {
            index
        } else {
            // Allocate space for the joints.
            let Some(allocation) = self.allocator.allocate(
                key.joints
                    .len()
                    .div_ceil(JOINTS_PER_ALLOCATION_UNIT as usize) as u32,
            ) else {
                error!(
                    "Out of space for skin: {:?}. Tried to allocate space for {:?} joints.",
                    skin,
                    key.joints.len()
                );
                return;
            };
            self.total_joints += key.joints.len();
            let block = SkinBlock {
                allocation,
                key: key.clone(),
                users: 0,
                fresh: false,
            };
            let index = match self.free_blocks.pop() {
                Some(index) => {
                    self.blocks[index as usize] = Some(block);
                    index
                }
                None => {
                    self.blocks.push(Some(block));
                    (self.blocks.len() - 1) as u32
                }
            };
            self.block_by_key.insert(key, index);
            index
        };

        let Some(block) = self.blocks[block_index as usize].as_mut() else {
            return;
        };
        block.users += 1;
        block.fresh = true;
        let offset = block.offset();
        let inverse_bindposes = inverse_bindposes.get(block.key.inverse_bindposes);
        for (joint_index, &joint) in block.key.joints.iter().enumerate() {
            // Calculate the initial joint matrix.
            let inverse_bindpose = inverse_bindposes
                .and_then(|inverse_bindposes| inverse_bindposes.get(joint_index));
            let joint_matrix = match (inverse_bindpose, joint_transform(joint)) {
                (Some(inverse_bindpose), Some(transform)) => transform * *inverse_bindpose,
                _ => Mat4::IDENTITY,
            };

            // Write in the new joint matrix, growing the staging buffer if
            // necessary.
            let buffer_index = offset as usize + joint_index;
            if self.staging.len() < buffer_index + 1 {
                self.staging.resize(buffer_index + 1, Mat4::IDENTITY);
            }
            self.staging[buffer_index] = joint_matrix;
        }
        self.joints_visited += block.key.joints.len();

        self.skin_uniform_info.insert(
            skin,
            SkinUniformInfo {
                block: block_index,
                offset,
            },
        );
    }

    /// Takes `skin` off its block; the block is freed with its last user.
    pub fn remove_skin(&mut self, skin: MainEntity) {
        let Some(info) = self.skin_uniform_info.remove(&skin) else {
            return;
        };
        let slot = &mut self.blocks[info.block as usize];
        let Some(block) = slot.as_mut() else {
            return;
        };
        block.users -= 1;
        if block.users > 0 {
            return;
        }
        let Some(block) = slot.take() else {
            return;
        };
        // Free the allocation.
        self.allocator.free(block.allocation);
        // Update the total number of joints.
        self.total_joints -= block.key.joints.len();
        self.block_by_key.remove(&block.key);
        self.free_blocks.push(info.block);
    }

    /// Rewrites, once per block, the matrices of joints whose global transform
    /// changed (`changed_transform` returns `None` for unchanged joints).
    /// Blocks filled by [`Self::add_skin`] this frame are skipped.
    pub fn extract_changed_joints(
        &mut self,
        inverse_bindposes: &Assets<SkinnedMeshInverseBindposes>,
        changed_transform: impl Fn(Entity) -> Option<Affine3A>,
    ) {
        for block in self.blocks.iter_mut().flatten() {
            // If we initialized the block this frame, we already populated all
            // the joints, so there's no need to populate them again.
            if mem::take(&mut block.fresh) {
                continue;
            }
            let Some(inverse_bindposes) = inverse_bindposes.get(block.key.inverse_bindposes)
            else {
                continue;
            };
            let offset = block.offset() as usize;
            // Calculate and write in the new joint matrices, if they changed
            // this frame.
            for (joint_index, (&joint, inverse_bindpose)) in block
                .key
                .joints
                .iter()
                .zip(inverse_bindposes.iter())
                .enumerate()
            {
                // Skip if the global transform for this joint didn't change.
                let Some(transform) = changed_transform(joint) else {
                    continue;
                };
                self.staging[offset + joint_index] = transform * *inverse_bindpose;
            }
            self.joints_visited += block.key.joints.len().min(inverse_bindposes.len());
        }
    }
}
// superfargoth patch (own, P5: shared skin joint blocks) end.

/// Returns true if skinning must use uniforms (and dynamic offsets) because
/// storage buffers aren't supported on the current platform.
pub fn skins_use_uniform_buffers(limits: &WgpuLimits) -> bool {
    bevy_render::storage_buffers_are_unsupported(limits)
}

/// Uploads the buffers containing the joints to the GPU.
pub fn prepare_skins(
    render_device: Res<RenderDevice>,
    render_queue: Res<RenderQueue>,
    uniform: ResMut<SkinUniforms>,
) {
    let uniform = uniform.into_inner();
    // superfargoth patch (own, P5: shared skin joint blocks): the staging buffer
    // lives in `SkinJointBlocks`.

    if uniform.blocks.staging.is_empty() {
        return;
    }

    // Swap current and previous buffers.
    mem::swap(&mut uniform.current_buffer, &mut uniform.prev_buffer);

    // Resize the buffers if necessary. Include extra space equal to `MAX_JOINTS`
    // because we need to be able to bind a full uniform buffer's worth of data
    // if skins use uniform buffers on this platform.
    let needed_size = (uniform.blocks.staging.len() as u64 + MAX_JOINTS as u64)
        * size_of::<Mat4>() as u64;
    if uniform.current_buffer.size() < needed_size {
        let mut new_size = uniform.current_buffer.size();
        while new_size < needed_size {
            // 1.5× growth factor.
            new_size = (new_size + new_size / 2).next_multiple_of(4);
        }

        // Create the new buffers.
        let buffer_usages = if skins_use_uniform_buffers(&render_device.limits()) {
            BufferUsages::UNIFORM
        } else {
            BufferUsages::STORAGE
        } | BufferUsages::COPY_DST;
        uniform.current_buffer = render_device.create_buffer(&BufferDescriptor {
            label: Some("skin uniform buffer"),
            usage: buffer_usages,
            size: new_size,
            mapped_at_creation: false,
        });
        uniform.prev_buffer = render_device.create_buffer(&BufferDescriptor {
            label: Some("skin uniform buffer"),
            usage: buffer_usages,
            size: new_size,
            mapped_at_creation: false,
        });

        // We've created a new `prev_buffer` but we don't have the previous joint
        // data needed to fill it out correctly. Use the current joint data
        // instead.
        //
        // TODO: This is a bug - will cause motion blur to ignore joint movement
        // for one frame.
        render_queue.write_buffer(
            &uniform.prev_buffer,
            0,
            bytemuck::must_cast_slice(&uniform.blocks.staging[..]),
        );
    }

    // Write the data from `uniform.blocks.staging` into
    // `uniform.current_buffer`.
    render_queue.write_buffer(
        &uniform.current_buffer,
        0,
        bytemuck::must_cast_slice(&uniform.blocks.staging[..]),
    );

    // We don't need to write `uniform.prev_buffer` because we already wrote it
    // last frame, and the data should still be on the GPU.
}

// Notes on implementation:
// We define the uniform binding as an array<mat4x4<f32>, N> in the shader,
// where N is the maximum number of Mat4s we can fit in the uniform binding,
// which may be as little as 16kB or 64kB. But, we may not need all N.
// We may only need, for example, 10.
//
// If we used uniform buffers ‘normally’ then we would have to write a full
// binding of data for each dynamic offset binding, which is wasteful, makes
// the buffer much larger than it needs to be, and uses more memory bandwidth
// to transfer the data, which then costs frame time So @superdump came up
// with this design: just bind data at the specified offset and interpret
// the data at that offset as an array<T, N> regardless of what is there.
//
// So instead of writing N Mat4s when you only need 10, you write 10, and
// then pad up to the next dynamic offset alignment. Then write the next.
// And for the last dynamic offset binding, make sure there is a full binding
// of data after it so that the buffer is of size
// `last dynamic offset` + `array<mat4x4<f32>>`.
//
// Then when binding the first dynamic offset, the first 10 entries in the array
// are what you expect, but if you read the 11th you’re reading ‘invalid’ data
// which could be padding or could be from the next binding.
//
// In this way, we can pack ‘variable sized arrays’ into uniform buffer bindings
// which normally only support fixed size arrays. You just have to make sure
// in the shader that you only read the values that are valid for that binding.
//
// superfargoth patch (own, P5: shared skin joint blocks): joints are extracted
// per shared block (`SkinJointBlocks`), not per skinned-mesh entity, so the
// all-skinned-meshes query is gone.
pub fn extract_skins(
    skin_uniforms: ResMut<SkinUniforms>,
    changed_skinned_meshes: Extract<
        Query<
            (Entity, &ViewVisibility, &SkinnedMesh),
            Or<(
                Changed<ViewVisibility>,
                Changed<SkinnedMesh>,
                AssetChanged<SkinnedMesh>,
            )>,
        >,
    >,
    skinned_mesh_inverse_bindposes: Extract<Res<Assets<SkinnedMeshInverseBindposes>>>,
    changed_transforms: Extract<Query<(Entity, &GlobalTransform), Changed<GlobalTransform>>>,
    joints: Extract<Query<&GlobalTransform>>,
    mut removed_skinned_meshes_query: Extract<RemovedComponents<SkinnedMesh>>,
) {
    let blocks = &mut skin_uniforms.into_inner().blocks;
    blocks.begin_extract();

    // Find every skinned mesh that changed one of (1) visibility; (2) joint
    // entities (part of `SkinnedMesh`); (3) the associated
    // `SkinnedMeshInverseBindposes` asset. Allocate, reallocate, or free space
    // for them as necessary.
    for (skinned_mesh_entity, skinned_mesh_view_visibility, skinned_mesh) in &changed_skinned_meshes
    {
        // Remove the skin if it existed last frame.
        let skinned_mesh_entity = MainEntity::from(skinned_mesh_entity);
        blocks.remove_skin(skinned_mesh_entity);

        // If the skin is invisible, we're done.
        if !(*skinned_mesh_view_visibility).get() {
            continue;
        }

        // Initialize the skin (or join the block of its binding).
        blocks.add_skin(
            skinned_mesh_entity,
            skinned_mesh,
            &skinned_mesh_inverse_bindposes,
            |joint| joints.get(joint).ok().map(GlobalTransform::affine),
        );
    }

    // Extract the transforms of changed joints, once per block, and write them
    // into the staging buffer at the appropriate spot.
    blocks.extract_changed_joints(&skinned_mesh_inverse_bindposes, |joint| {
        changed_transforms
            .get(joint)
            .ok()
            .map(|(_, transform)| transform.affine())
    });

    // Delete skins that became invisible.
    for skinned_mesh_entity in removed_skinned_meshes_query.read() {
        // Only remove a skin if we didn't pick it up above. It's possible that
        // a necessary component was removed and re-added in the same frame.
        if !changed_skinned_meshes.contains(skinned_mesh_entity) {
            blocks.remove_skin(skinned_mesh_entity.into());
        }
    }
}

// NOTE: The skinned joints uniform buffer has to be bound at a dynamic offset per
// entity and so cannot currently be batched on WebGL 2.
pub fn no_automatic_skin_batching(
    mut commands: Commands,
    query: Query<Entity, (With<SkinnedMesh>, Without<NoAutomaticBatching>)>,
    render_device: Res<RenderDevice>,
) {
    if !skins_use_uniform_buffers(&render_device.limits()) {
        return;
    }

    for entity in &query {
        commands.entity(entity).try_insert(NoAutomaticBatching);
    }
}
