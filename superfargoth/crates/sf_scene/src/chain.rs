//! The mesh behind a [`crate::LodChain`] (static-scene.md §2): how one model part's LOD levels
//! become one vertex buffer, one index buffer and an index range per level.
//!
//! **A chain is one (model, material part).** A cooked model's LOD levels are each complete,
//! with their own vertices and indices per submesh. The chain's mesh concatenates, finest level
//! first, every submesh of that material in each level: vertices back to back, and each
//! submesh's indices rebased by the vertex count before it (`index + vertex_base`), so one index
//! buffer addresses one vertex buffer. A level's [`LodLevel`] is its range in that index buffer
//! and its error. A level with **no geometry** of the material — no submesh of it, or only
//! zero-index ones, which draw nothing — repeats the next finer level (same range, same error:
//! it draws exactly that geometry), as SPUD does; the chain is an error only when its finest
//! level has none (every level would repeat nothing).
//!
//! The bake (`scene.cells`) and the runtime (`sf_scene_bevy`, which builds the buffer) both use
//! [`chain_layout`], so the ranges in the table always match the uploaded mesh.

use crate::table::{LodLevel, MAX_LOD_LEVELS};

/// One source LOD level of a chain: its error and the (vertex count, index count) of each of
/// its submeshes of the chain's material, in submesh order (usually zero or one).
#[derive(Clone, Debug, Default, PartialEq)]
pub struct LevelSource {
    /// Worst deviation from level 0, metres.
    pub error: f32,
    pub submeshes: Vec<(u32, u32)>,
}

/// Where one source submesh lands in the chain's buffers.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Segment {
    /// Source level index.
    pub level: usize,
    /// Submesh index within [`LevelSource::submeshes`].
    pub submesh: usize,
    /// First vertex in the chain's vertex buffer; added to every index of this submesh.
    pub vertex_base: u32,
    /// First index in the chain's index buffer.
    pub index_base: u32,
    pub vertex_count: u32,
    pub index_count: u32,
}

/// The chain's buffers and levels.
#[derive(Clone, Debug, PartialEq)]
pub struct ChainLayout {
    /// One per source level, finest first.
    pub levels: Vec<LodLevel>,
    /// Every source submesh in buffer order (level, then submesh).
    pub segments: Vec<Segment>,
    pub vertex_count: u32,
    pub index_count: u32,
}

#[derive(Clone, Debug, PartialEq, thiserror::Error)]
pub enum LayoutError {
    #[error("a chain needs 1..={MAX_LOD_LEVELS} levels, got {0}")]
    LevelCount(usize),
    #[error("the finest level has no geometry for this part")]
    EmptyFinest,
    #[error("level {0} has a negative or non-finite error")]
    Error(usize),
    #[error("chain too large for 32-bit counts")]
    TooLarge,
}

/// The layout rule (module docs).
pub fn chain_layout(levels: &[LevelSource]) -> Result<ChainLayout, LayoutError> {
    if levels.is_empty() || levels.len() > MAX_LOD_LEVELS {
        return Err(LayoutError::LevelCount(levels.len()));
    }
    if levels[0].submeshes.iter().all(|&(_, indices)| indices == 0) {
        return Err(LayoutError::EmptyFinest);
    }
    let (mut vertex_base, mut index_base) = (0u32, 0u32);
    let mut out = ChainLayout { levels: Vec::with_capacity(levels.len()), segments: Vec::new(), vertex_count: 0, index_count: 0 };
    for (level, source) in levels.iter().enumerate() {
        if !(source.error.is_finite() && source.error >= 0.0) {
            return Err(LayoutError::Error(level));
        }
        let first_index = index_base;
        for (submesh, &(vertex_count, index_count)) in source.submeshes.iter().enumerate() {
            // A submesh with no index draws nothing, and its vertices would be unreachable:
            // "no geometry for this part", the same test as the finest-level guard above. Taking
            // it would emit a zero-length draw range (the object vanishes at that distance) and
            // count its vertices into the chain's buffer.
            if index_count == 0 {
                continue;
            }
            out.segments.push(Segment { level, submesh, vertex_base, index_base, vertex_count, index_count });
            vertex_base = vertex_base.checked_add(vertex_count).ok_or(LayoutError::TooLarge)?;
            index_base = index_base.checked_add(index_count).ok_or(LayoutError::TooLarge)?;
        }
        if index_base == first_index {
            // Nothing of this part at this level: repeat the next finer one (same range, same
            // error, since it draws exactly that geometry), as SPUD does.
            let finer = *out.levels.last().expect("level 0 has geometry");
            out.levels.push(finer);
        } else {
            out.levels.push(LodLevel { first_index, index_count: index_base - first_index, error: source.error });
        }
    }
    out.vertex_count = vertex_base;
    out.index_count = index_base;
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn level(error: f32, submeshes: &[(u32, u32)]) -> LevelSource {
        LevelSource { error, submeshes: submeshes.to_vec() }
    }

    fn lod(first_index: u32, index_count: u32, error: f32) -> LodLevel {
        LodLevel { first_index, index_count, error }
    }

    #[test]
    fn levels_concatenate_finest_first() {
        let layout = chain_layout(&[level(0.0, &[(100, 300)]), level(0.05, &[(40, 90)]), level(0.2, &[(10, 18)])]).unwrap();
        assert_eq!(layout.levels, [lod(0, 300, 0.0), lod(300, 90, 0.05), lod(390, 18, 0.2)]);
        let bases: Vec<(usize, u32, u32)> = layout.segments.iter().map(|s| (s.level, s.vertex_base, s.index_base)).collect();
        assert_eq!(bases, [(0, 0, 0), (1, 100, 300), (2, 140, 390)]);
        assert_eq!((layout.vertex_count, layout.index_count), (150, 408));
    }

    #[test]
    fn a_level_without_the_part_repeats_the_finer_one() {
        // Level 1 dropped this part (a small detail), level 2 has it again (unusual, but legal).
        let layout =
            chain_layout(&[level(0.0, &[(8, 12)]), level(0.1, &[]), level(0.3, &[(4, 6)]), level(0.9, &[])]).unwrap();
        assert_eq!(layout.levels, [lod(0, 12, 0.0), lod(0, 12, 0.0), lod(12, 6, 0.3), lod(12, 6, 0.3)]);
        assert_eq!(layout.segments.len(), 2);
        assert_eq!(layout.segments[1], Segment { level: 2, submesh: 0, vertex_base: 8, index_base: 12, vertex_count: 4, index_count: 6 });
        assert_eq!((layout.vertex_count, layout.index_count), (12, 18));
    }

    #[test]
    fn a_level_whose_submeshes_draw_nothing_repeats_the_finer_one() {
        // A submesh the cooker kept with zero indices draws nothing: the level has no geometry
        // for this part, exactly as if the submesh were absent. Taking it would have emitted a
        // zero-length range (the part vanishes at that distance) and counted its vertices.
        let layout =
            chain_layout(&[level(0.0, &[(8, 12)]), level(0.1, &[(5, 0)]), level(0.3, &[(4, 6), (7, 0)])]).unwrap();
        assert_eq!(layout.levels, [lod(0, 12, 0.0), lod(0, 12, 0.0), lod(12, 6, 0.3)]);
        // Only the two drawing submeshes are in the buffers; the empty ones add no vertices.
        assert_eq!(layout.segments.iter().map(|s| s.level).collect::<Vec<_>>(), [0, 2]);
        assert_eq!((layout.vertex_count, layout.index_count), (12, 18));
    }

    #[test]
    fn several_submeshes_of_one_material_share_a_level() {
        let layout = chain_layout(&[level(0.0, &[(3, 3), (6, 12)]), level(0.5, &[(3, 3)])]).unwrap();
        assert_eq!(layout.levels, [lod(0, 15, 0.0), lod(15, 3, 0.5)]);
        let segs: Vec<(usize, usize, u32, u32)> =
            layout.segments.iter().map(|s| (s.level, s.submesh, s.vertex_base, s.index_base)).collect();
        assert_eq!(segs, [(0, 0, 0, 0), (0, 1, 3, 3), (1, 0, 9, 15)]);
    }

    #[test]
    fn bad_inputs_are_errors() {
        assert_eq!(chain_layout(&[]), Err(LayoutError::LevelCount(0)));
        let many = vec![level(0.0, &[(3, 3)]); MAX_LOD_LEVELS + 1];
        assert_eq!(chain_layout(&many), Err(LayoutError::LevelCount(MAX_LOD_LEVELS + 1)));
        assert_eq!(chain_layout(&[level(0.0, &[])]), Err(LayoutError::EmptyFinest));
        assert_eq!(chain_layout(&[level(0.0, &[(3, 0)]), level(0.1, &[(3, 3)])]), Err(LayoutError::EmptyFinest));
        assert_eq!(chain_layout(&[level(0.0, &[(3, 3)]), level(f32::NAN, &[])]), Err(LayoutError::Error(1)));
        assert_eq!(chain_layout(&[level(-1.0, &[(3, 3)])]), Err(LayoutError::Error(0)));
        assert_eq!(chain_layout(&[level(0.0, &[(u32::MAX, 3), (1, 3)])]), Err(LayoutError::TooLarge));
    }
}
