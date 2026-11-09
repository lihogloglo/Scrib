use bevy_ecs::{
    component::Component,
    entity::Entity,
    system::{ResMut, SystemParam, SystemParamItem},
};
use gpu_preprocessing::UntypedPhaseIndirectParametersBuffers;
use nonmax::NonMaxU32;

use bevy_material::{descriptor::CachedRenderPipelineId, labels::DrawFunctionId};

use crate::{
    batching::gpu_preprocessing::BufferDataInput,
    render_phase::{
        BinnedPhaseItem, CachedRenderPipelinePhaseItem, PhaseItemExtraIndex, SortedPhaseItem,
        SortedRenderPhase, ViewBinnedRenderPhases,
    },
    render_resource::GpuArrayBufferable,
    sync_world::MainEntity,
};

pub mod gpu_preprocessing;
pub mod no_gpu_preprocessing;

/// Add this component to mesh entities to disable automatic batching
#[derive(Component, Default, Clone, Copy)]
pub struct NoAutomaticBatching;

/// Data necessary to be equal for two draw commands to be mergeable
///
/// This is based on the following assumptions:
/// - Only entities with prepared assets (pipelines, materials, meshes) are
///   queued to phases
/// - View bindings are constant across a phase for a given draw function as
///   phases are per-view
/// - `batch_and_prepare_render_phase` is the only system that performs this
///   batching and has sole responsibility for preparing the per-object data.
///   As such the mesh binding and dynamic offsets are assumed to only be
///   variable as a result of the `batch_and_prepare_render_phase` system, e.g.
///   due to having to split data across separate uniform bindings within the
///   same buffer due to the maximum uniform buffer binding size.
#[derive(PartialEq)]
struct BatchSetMeta<T: PartialEq> {
    /// The pipeline id encompasses all pipeline configuration including vertex
    /// buffers and layouts, shaders and their specializations, bind group
    /// layouts, etc.
    pipeline_id: CachedRenderPipelineId,
    /// The draw function id defines the `RenderCommands` that are called to
    /// set the pipeline and bindings, and make the draw command
    draw_function_id: DrawFunctionId,
    dynamic_offset: Option<NonMaxU32>,
    user_data: T,
}

impl<T: PartialEq> BatchSetMeta<T> {
    fn new(item: &impl CachedRenderPipelinePhaseItem, user_data: T) -> Self {
        BatchSetMeta {
            pipeline_id: item.cached_pipeline(),
            draw_function_id: item.draw_function(),
            dynamic_offset: match item.extra_index() {
                PhaseItemExtraIndex::DynamicOffset(dynamic_offset) => {
                    NonMaxU32::new(dynamic_offset)
                }
                PhaseItemExtraIndex::None | PhaseItemExtraIndex::IndirectParametersIndex { .. } => {
                    None
                }
            },
            user_data,
        }
    }
}

/// A trait to support getting data used for batching draw commands via phase
/// items.
///
/// This is a simple version that only allows for sorting, not binning, as well
/// as only CPU processing, not GPU preprocessing. For these fancier features,
/// see [`GetFullBatchData`].
pub trait GetBatchData {
    /// The system parameters [`GetBatchData::get_batch_data`] needs in
    /// order to compute the batch data.
    type Param: SystemParam + 'static;
    /// Data used for comparison between phase items to decide whether items can
    /// be batched.
    ///
    /// If this data, and the [`Self::BatchSetCompareData`], are identical to
    /// those of the previous phase item, the items can be batched together.
    type BatchCompareData: PartialEq;
    /// Data used for comparison between phase items to decide whether items can
    /// be grouped in the same batch set (i.e. multi-drawn).
    ///
    /// If this data is identical to that of the previous phase items, and the
    /// current platform supports multi-draw, the items can be multi-drawn
    /// together.
    type BatchSetCompareData: PartialEq;
    /// The per-instance data to be inserted into the
    /// [`crate::render_resource::GpuArrayBuffer`] containing these data for all
    /// instances.
    type BufferData: GpuArrayBufferable + Sync + Send + 'static;
    /// Get the per-instance data to be inserted into the
    /// [`crate::render_resource::GpuArrayBuffer`]. If the instance can be
    /// batched, also return the data used for comparison when deciding whether
    /// draws can be batched, else return None for the `CompareData`.
    ///
    /// This is only called when building instance data on CPU. In the GPU
    /// instance data building path, we use
    /// [`GetFullBatchData::get_index_and_compare_data`] instead.
    fn get_batch_data(
        param: &SystemParamItem<Self::Param>,
        query_item: (Entity, MainEntity),
    ) -> Option<(
        Self::BufferData,
        Option<(Self::BatchSetCompareData, Self::BatchCompareData)>,
    )>;
}

/// A trait to support getting data used for batching draw commands via phase
/// items.
///
/// This version allows for binning and GPU preprocessing.
pub trait GetFullBatchData: GetBatchData {
    /// The per-instance data that was inserted into the
    /// [`crate::render_resource::BufferVec`] during extraction.
    type BufferInputData: BufferDataInput;

    /// Get the per-instance data to be inserted into the
    /// [`crate::render_resource::GpuArrayBuffer`].
    ///
    /// This is only called when building uniforms on CPU. In the GPU instance
    /// buffer building path, we use
    /// [`GetFullBatchData::get_index_and_compare_data`] instead.
    fn get_binned_batch_data(
        param: &SystemParamItem<Self::Param>,
        query_item: MainEntity,
    ) -> Option<Self::BufferData>;

    /// Returns the index of the [`GetFullBatchData::BufferInputData`] that the
    /// GPU preprocessing phase will use.
    ///
    /// We already inserted the [`GetFullBatchData::BufferInputData`] during the
    /// extraction phase before we got here, so this function shouldn't need to
    /// look up any render data. If CPU instance buffer building is in use, this
    /// function will never be called.
    fn get_index_and_compare_data(
        param: &SystemParamItem<Self::Param>,
        query_item: MainEntity,
    ) -> Option<(
        NonMaxU32,
        Option<(Self::BatchSetCompareData, Self::BatchCompareData)>,
    )>;

    /// Returns the index of the [`GetFullBatchData::BufferInputData`] that the
    /// GPU preprocessing phase will use.
    ///
    /// We already inserted the [`GetFullBatchData::BufferInputData`] during the
    /// extraction phase before we got here, so this function shouldn't need to
    /// look up any render data.
    ///
    /// This function is currently only called for unbatchable entities when GPU
    /// instance buffer building is in use. For batchable entities, the uniform
    /// index is written during queuing (e.g. in `queue_material_meshes`). In
    /// the case of CPU instance buffer building, the CPU writes the uniforms,
    /// so there's no index to return.
    fn get_binned_index(
        param: &SystemParamItem<Self::Param>,
        query_item: MainEntity,
    ) -> Option<NonMaxU32>;

    /// Writes the [`gpu_preprocessing::IndirectParametersMetadata`] necessary
    /// to draw this batch into the given metadata buffer at the given index.
    ///
    /// This is only used if GPU culling is enabled (which requires GPU
    /// preprocessing), and only for phase items that don't support GPU uniform
    /// allocation yet.
    ///
    /// * `indexed` is true if the mesh is indexed or false if it's non-indexed.
    ///
    /// * `base_output_index` is the index of the first mesh instance in this
    ///   batch in the `MeshUniform` output buffer.
    ///
    /// * `batch_set_index` is the index of the batch set in the
    ///   [`gpu_preprocessing::IndirectBatchSet`] buffer, if this batch belongs to
    ///   a batch set.
    ///
    /// * `indirect_parameters_buffers` is the buffer in which to write the
    ///   metadata.
    ///
    /// * `indirect_parameters_offset` is the index in that buffer at which to
    ///   write the metadata.
    fn write_batch_indirect_parameters_metadata(
        indexed: bool,
        base_output_index: u32,
        batch_set_index: Option<NonMaxU32>,
        indirect_parameters_buffers: &mut UntypedPhaseIndirectParametersBuffers,
        indirect_parameters_offset: u32,
    );
    // superfargoth patch (bevyengine/bevy#25978)

    /// Returns the geometry spans of a GPU-authored instance batch, or `None`
    /// for entities handled by ordinary batching or a custom draw command.
    /// An empty slice identifies a GPU batch whose inputs are unavailable;
    /// it must not draw or allocate work items this frame.
    ///
    /// Sorted phases treat each batch as one item, and GPU compaction does not
    /// preserve instance order.
    // superfargoth patch (own, GS: ranges): one span per geometry range,
    // replacing the single input range of #25978 (an empty slice is the old
    // empty range).
    fn get_instance_batch<'a>(
        _param: &'a SystemParamItem<Self::Param>,
        _query_item: MainEntity,
    ) -> Option<&'a [InstanceBatchSpan]> {
        None
    }
}

// superfargoth patch (own, GS: ranges)
/// One indirect sub-draw of a GPU-authored instance batch: a contiguous span
/// of mesh-input indices, all drawing one geometry range. All spans of one
/// batch share one batch set, so the batch is still a single
/// `multi_draw_indirect_count`.
///
/// Spans are in slot order and their counts sum to the batch's capacity: the
/// `MeshUniform` space a batch takes is unchanged by the split.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct InstanceBatchSpan {
    /// First index in the `MeshInputUniform` buffer.
    pub base_input_index: u32,
    /// Number of instance slots.
    pub count: u32,
}

// superfargoth patch (own, GS: ranges)
/// One indirect draw command of a GPU-authored instance batch: the slot it
/// takes in the phase's indirect parameters buffers, the `MeshUniform` slots
/// its instances land in, and the work item that fills them.
///
/// Produced by [`instance_batch_sub_draws`], which is the single place a
/// batch's sub-draw layout is decided.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct InstanceBatchSubDraw {
    /// Index of this command in the phase's `IndirectParametersBuffers`.
    pub indirect_parameters_index: u32,
    /// First `MeshUniform` slot this command's instances occupy.
    pub base_output_index: u32,
    /// First `MeshInputUniform` index, from the span.
    pub base_input_index: u32,
    /// Number of instance slots.
    pub count: u32,
}

// superfargoth patch (own, GS: ranges)
/// Lays a batch's spans out over `spans.len()` *contiguous* indirect parameters
/// slots starting at `indirect_parameters_base`, and over the `MeshUniform`
/// slots starting at `output_base`.
///
/// Sub-draw *i* takes slot `indirect_parameters_base + i` and the output slots
/// the first *i* spans do not, so the commands are in span (slot) order and the
/// batch still occupies exactly `output_base..output_base + sum(counts)` - the
/// one contiguous allocation #25978 made for its single command.
///
/// All the sub-draws of one batch share one `IndirectBatchSet`, so a batch of
/// N ranges is one `multi_draw_indirect_count` of N commands. A batch with no
/// ranges has one span (`bevy_pbr`'s `batch_spans`) and so exactly one
/// sub-draw, byte for byte what #25978 built.
pub fn instance_batch_sub_draws(
    indirect_parameters_base: u32,
    output_base: u32,
    spans: &[InstanceBatchSpan],
) -> impl Iterator<Item = InstanceBatchSubDraw> + '_ {
    let mut base_output_index = output_base;
    spans.iter().enumerate().map(move |(index, span)| {
        let sub_draw = InstanceBatchSubDraw {
            indirect_parameters_index: indirect_parameters_base + index as u32,
            base_output_index,
            base_input_index: span.base_input_index,
            count: span.count,
        };
        base_output_index += span.count;
        sub_draw
    })
}

/// Sorts a render phase that uses bins.
pub fn sort_binned_render_phase<BPI>(mut phases: ResMut<ViewBinnedRenderPhases<BPI>>)
where
    BPI: BinnedPhaseItem,
{
    for phase in phases.values_mut() {
        phase.multidrawable_meshes.sort_unstable_keys();
        phase.batchable_meshes.sort_unstable_keys();
        phase.unbatchable_meshes.sort_unstable_keys();
        phase.non_mesh_items.sort_unstable_keys();
    }
}

/// Batches the items in a sorted render phase.
///
/// This means comparing metadata needed to draw each phase item and trying to
/// combine the draws into a batch.
///
/// This is common code factored out from
/// [`gpu_preprocessing::batch_and_prepare_sorted_render_phase`] and
/// [`no_gpu_preprocessing::batch_and_prepare_sorted_render_phase`].
fn batch_and_prepare_sorted_render_phase<I, GBD>(
    phase: &mut SortedRenderPhase<I>,
    mut process_item: impl FnMut(&mut I) -> Option<(GBD::BatchSetCompareData, GBD::BatchCompareData)>,
) where
    I: CachedRenderPipelinePhaseItem + SortedPhaseItem,
    GBD: GetBatchData,
{
    let items = phase.items.values_mut().map(|item| {
        let batch_data = match process_item(item) {
            Some(compare_data) if I::AUTOMATIC_BATCHING => {
                Some(BatchSetMeta::new(item, compare_data))
            }
            _ => None,
        };
        (item.batch_range_mut(), batch_data)
    });

    items.reduce(|(start_range, prev_batch_meta), (range, batch_meta)| {
        if batch_meta.is_some() && prev_batch_meta == batch_meta {
            start_range.end = range.end;
            // superfargoth patch (bevyengine/bevy#25978)
            *range = range.end..range.end;
            (start_range, prev_batch_meta)
        } else {
            (range, batch_meta)
        }
    });
}
