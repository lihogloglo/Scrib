//! Where the sea can be drawn (CW-10a, water.md "Water pass" visibility gate): a quadtree of wet
//! rectangles over the shore field, so a camera can tell whether any water lies in its frustum and
//! the GPU can test a handful of boxes against the depth buffer instead of the whole clipmap.
//!
//! A leaf is [`LEAF_TEXELS`]² field texels; it is wet when any of its texels has a signed shore
//! distance at or above the threshold (the water draw discards below its land margin, so a
//! threshold at or below `-land margin` keeps every drawn texel inside a wet leaf). Nodes whose
//! leaves are all wet collapse into one rectangle (the open sea is a few big boxes, the coast a
//! band of leaves). Outside the field the sea is open water everywhere ([`ShoreField::sample`]):
//! [`WetTiles::exterior`] gives the four strips around it. Rectangles are field-relative metres
//! (`0, 0` = the field's corner, `origin - texel / 2`), so the host adds its own render offset.

use glam::DVec2;

use crate::ShoreField;

/// Leaf side in field texels (a layout constant: tighter boxes against more of them).
pub const LEAF_TEXELS: usize = 8; // tunables: const (layout: the tree's leaf; exactness never depends on it)

/// A rectangle (min x, min z, max x, max z), field-relative metres.
pub type Rect = [f32; 4];

#[derive(Clone, Debug, PartialEq)]
enum Node {
    Full(Rect),
    /// Rectangle and up to four children (indices into `nodes`).
    Partial(Rect, Vec<u32>),
}

/// The wet-leaf quadtree of one shore field.
#[derive(Clone, Debug, PartialEq)]
pub struct WetTiles {
    /// World XZ of the field's corner (`origin - texel / 2`).
    pub corner: DVec2,
    /// Field extent, metres.
    pub extent: [f32; 2],
    nodes: Vec<Node>,
    root: Option<u32>,
}

impl WetTiles {
    /// From a shore field: wet where the signed distance is `>= threshold` metres.
    #[must_use]
    pub fn from_field(field: &ShoreField, threshold: f32) -> Self {
        let corner = field.origin - DVec2::splat(field.texel * 0.5);
        Self::build(field.width, field.height, field.texel, corner, &field.distance, threshold)
    }

    /// From a `width × height` grid of signed distances (`[row * width + column]`, rows +Z).
    #[must_use]
    #[allow(clippy::cast_possible_truncation, clippy::cast_precision_loss)]
    pub fn build(width: usize, height: usize, texel: f64, corner: DVec2, distance: &[f32], threshold: f32) -> Self {
        let (lw, lh) = (width.div_ceil(LEAF_TEXELS), height.div_ceil(LEAF_TEXELS));
        let mut wet = vec![false; lw * lh];
        for z in 0..height {
            for x in 0..width {
                if distance.get(z * width + x).is_some_and(|d| *d >= threshold) {
                    wet[(z / LEAF_TEXELS) * lw + x / LEAF_TEXELS] = true;
                }
            }
        }
        let extent = [(width as f64 * texel) as f32, (height as f64 * texel) as f32];
        let leaf = (LEAF_TEXELS as f64 * texel) as f32;
        let side = lw.max(lh).max(1).next_power_of_two();
        let mut tiles = Self { corner, extent, nodes: Vec::new(), root: None };
        let grid = Grid { wet: &wet, lw, lh, leaf, extent };
        tiles.root = tiles.node(&grid, 0, 0, side);
        tiles
    }

    /// The node over leaves `[x0, x0 + side) × [z0, z0 + side)`: `None` when none is wet.
    fn node(&mut self, g: &Grid, x0: usize, z0: usize, side: usize) -> Option<u32> {
        if x0 >= g.lw || z0 >= g.lh {
            return None;
        }
        let rect = g.rect(x0, z0, side);
        if side == 1 {
            return g.wet[z0 * g.lw + x0].then(|| self.push(Node::Full(rect)));
        }
        let half = side / 2;
        let children: Vec<u32> = [(0, 0), (half, 0), (0, half), (half, half)].iter().filter_map(|(dx, dz)| self.node(g, x0 + dx, z0 + dz, half)).collect();
        if children.is_empty() {
            return None;
        }
        // Full when every in-grid child is full (out-of-grid quadrants are clipped away by `rect`).
        let in_grid = [(0, 0), (half, 0), (0, half), (half, half)].iter().filter(|(dx, dz)| x0 + dx < g.lw && z0 + dz < g.lh).count();
        let full = children.len() == in_grid && children.iter().all(|c| matches!(self.nodes[*c as usize], Node::Full(_)));
        if full {
            // The children were pushed last: drop them, keep one rectangle.
            self.nodes.truncate(self.nodes.len() - children.len());
            return Some(self.push(Node::Full(rect)));
        }
        Some(self.push(Node::Partial(rect, children)))
    }

    fn push(&mut self, node: Node) -> u32 {
        self.nodes.push(node);
        u32::try_from(self.nodes.len() - 1).unwrap_or(u32::MAX)
    }

    /// The wet rectangles, each grown by `margin` metres, that `visible` accepts. `visible` gets the
    /// grown rectangle of a node before its children: a rejected node skips its subtree. Returns
    /// `false` (and stops) once `out` would exceed `cap` entries.
    pub fn visit(&self, margin: f32, cap: usize, mut visible: impl FnMut(Rect) -> bool, out: &mut Vec<Rect>) -> bool {
        let grow = |r: &Rect| [r[0] - margin, r[1] - margin, r[2] + margin, r[3] + margin];
        let mut stack: Vec<u32> = self.root.into_iter().collect();
        while let Some(i) = stack.pop() {
            match &self.nodes[i as usize] {
                Node::Full(r) => {
                    let g = grow(r);
                    if visible(g) {
                        if out.len() >= cap {
                            return false;
                        }
                        out.push(g);
                    }
                }
                Node::Partial(r, children) => {
                    if visible(grow(r)) {
                        stack.extend(children.iter().copied());
                    }
                }
            }
        }
        true
    }

    /// The open sea around the field, out to `reach` metres: four strips (left, right, below, above).
    #[must_use]
    pub fn exterior(&self, reach: f32) -> [Rect; 4] {
        let [w, h] = self.extent;
        [[-reach, -reach, 0.0, h + reach], [w, -reach, w + reach, h + reach], [0.0, -reach, w, 0.0], [0.0, h, w, h + reach]]
    }

    /// Wet rectangles in the tree (diagnostics, tests).
    #[must_use]
    pub fn rects(&self) -> usize {
        self.nodes.iter().filter(|n| matches!(n, Node::Full(_))).count()
    }
}

struct Grid<'a> {
    wet: &'a [bool],
    lw: usize,
    lh: usize,
    leaf: f32,
    extent: [f32; 2],
}

impl Grid<'_> {
    /// Leaves `[x0, x0 + side) × [z0, z0 + side)` in metres, clipped to the field.
    #[allow(clippy::cast_precision_loss)]
    fn rect(&self, x0: usize, z0: usize, side: usize) -> Rect {
        let m = |i: usize| i as f32 * self.leaf;
        [m(x0), m(z0), m(x0 + side).min(self.extent[0]), m(z0 + side).min(self.extent[1])]
    }
}
