//! A row-major f64 grid (row 0 = north, column 0 = west) and its 8-neighbourhood.

/// Row-major grid of f64 samples at a uniform spacing. Row 0 is the north edge, column 0 the west edge.
#[derive(Debug, Clone, PartialEq)]
pub struct Grid {
    pub w: usize,
    pub h: usize,
    /// Sample spacing in metres.
    pub spacing: f64,
    pub data: Vec<f64>,
}

/// The 8 neighbour offsets (dx, dy) in a fixed order: N, NE, E, SE, S, SW, W, NW.
pub const NEIGHBOURS: [(isize, isize); 8] = [
    (0, -1),
    (1, -1),
    (1, 0),
    (1, 1),
    (0, 1),
    (-1, 1),
    (-1, 0),
    (-1, -1),
];

/// Distance factor of each [`NEIGHBOURS`] entry, in units of the spacing (1 or √2).
// tunables: const (grid geometry: 1 and sqrt 2)
pub const NEIGHBOUR_DIST: [f64; 8] = [
    1.0,
    std::f64::consts::SQRT_2,
    1.0,
    std::f64::consts::SQRT_2,
    1.0,
    std::f64::consts::SQRT_2,
    1.0,
    std::f64::consts::SQRT_2,
];

impl Grid {
    /// A grid filled with `v`.
    pub fn filled(w: usize, h: usize, spacing: f64, v: f64) -> Self {
        Self {
            w,
            h,
            spacing,
            data: vec![v; w * h],
        }
    }

    /// Sample count.
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// True for an empty grid.
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Value at (x, y).
    pub fn at(&self, x: usize, y: usize) -> f64 {
        self.data[y * self.w + x]
    }

    /// Value at (x, y) with coordinates clamped to the grid (edge extension).
    pub fn at_clamped(&self, x: isize, y: isize) -> f64 {
        let xc = x.clamp(0, self.w as isize - 1).cast_unsigned();
        let yc = y.clamp(0, self.h as isize - 1).cast_unsigned();
        self.data[yc * self.w + xc]
    }

    /// Index of neighbour `k` of sample `i`, or `None` off the grid.
    pub fn neighbour(&self, i: usize, k: usize) -> Option<usize> {
        neighbour(self.w, self.h, i, k)
    }

    /// True for a sample on the outer ring.
    pub fn is_border(&self, i: usize) -> bool {
        let (x, y) = (i % self.w, i / self.w);
        x == 0 || y == 0 || x + 1 == self.w || y + 1 == self.h
    }
}

/// Index of neighbour `k` ([`NEIGHBOURS`]) of sample `i` in a `w` × `h` grid, or `None` off the grid.
#[inline]
pub fn neighbour(w: usize, h: usize, i: usize, k: usize) -> Option<usize> {
    let (dx, dy) = NEIGHBOURS[k];
    let x = (i % w).checked_add_signed(dx)?;
    let y = (i / w).checked_add_signed(dy)?;
    (x < w && y < h).then_some(y * w + x)
}
