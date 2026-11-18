//! Clipmap GPU fill plans (terrain.md "Runtime", worldgen.md "Pipeline" stage 7, slice G2): the Bevy-free half of
//! filling clipmap pieces from a provider's [`crate::GpuField`]. [`plan`] turns a frame's pieces into the job
//! table `gpu_fill.wgsl` reads, the scratch and output sizes and the buffer-to-texture copies; [`compose`] joins
//! the provider's WGSL with the fill passes. `sf_terrain_bevy` runs it; tests can run it headless on wgpu.

use crate::clipmap::SampleRect;

/// The fill passes (`fill_eval`, `fill_pack`) and their bindings, after the provider's field functions.
pub const GPU_FILL_WGSL: &str = include_str!("gpu_fill.wgsl");

/// Threads per workgroup of both passes.
// tunables: const (layout: the shader's @workgroup_size)
pub const FILL_WORKGROUP: u32 = 64;
/// Workgroups along x of a dispatch (the shader's `FILL_GROUPS_X`); more go along y.
// tunables: const (layout: under the 65535 per-dimension dispatch limit)
pub const FILL_GROUPS_X: u32 = 32768;
/// Words of one job in the job table (`FillJob`).
// tunables: const (layout: the shader's struct)
pub const JOB_WORDS: usize = 16;
/// Row pitch alignment of buffer-to-texture copies, bytes (WebGPU `COPY_BYTES_PER_ROW_ALIGNMENT`).
// tunables: const (format: the API's copy alignment)
pub const ROW_ALIGN: u32 = 256;

/// What a piece fills.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FillKind {
    /// Heights (`R32Float`) and normals (`Rg8Snorm`) of a height level.
    Heights,
    /// Macro albedo (sRGB RGBA8) of a macro level (sample spacing twice the height level's).
    Macro,
}

/// One clipmap piece to fill: samples `rect` of `level`'s lattice, written at `texel` of array layer `level`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FillRequest {
    pub kind: FillKind,
    pub level: u8,
    pub rect: SampleRect,
    pub texel: [u32; 2],
}

/// Which texture a copy writes.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FillTarget {
    Heights,
    Normals,
    Macro,
}

/// One buffer-to-texture copy out of the output buffer.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FillCopy {
    pub target: FillTarget,
    pub layer: u32,
    pub texel: [u32; 2],
    pub size: [u32; 2],
    /// Byte offset in the output buffer and bytes per row (a multiple of [`ROW_ALIGN`]).
    pub offset: u64,
    pub bytes_per_row: u32,
}

/// A frame's fill: the job table and what the passes and copies need.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct FillPlan {
    pub jobs: Vec<[u32; JOB_WORDS]>,
    /// Threads of `fill_eval` and `fill_pack`.
    pub threads: [u32; 2],
    /// f32 of the scratch buffer and words of the output buffer.
    pub scratch_len: u32,
    pub out_len: u32,
    pub copies: Vec<FillCopy>,
    /// Texels written (heights + macro), for the counters.
    pub texels: u64,
}

impl FillPlan {
    /// The job table as little-endian bytes (at least one job: a storage binding can't be empty).
    pub fn job_bytes(&self) -> Vec<u8> {
        let mut out: Vec<u8> = self.jobs.iter().flatten().flat_map(|w| w.to_le_bytes()).collect();
        if out.is_empty() {
            out.resize(JOB_WORDS * 4, 0);
            // count_a = count_b = 0: no thread finds it.
        }
        out
    }

    /// Workgroups (x, y) of a dispatch of `threads`.
    pub fn groups(threads: u32) -> [u32; 2] {
        let g = threads.div_ceil(FILL_WORKGROUP);
        if g <= FILL_GROUPS_X { [g, u32::from(g > 0)] } else { [FILL_GROUPS_X, g.div_ceil(FILL_GROUPS_X)] }
    }
}

fn pitch_words(w: u32, bytes_per_texel: u32) -> u32 {
    (w * bytes_per_texel).next_multiple_of(ROW_ALIGN) / 4
}

fn u(v: i64) -> u32 {
    u32::try_from(v).expect("clipmap pieces are small and non-negative")
}

fn i(v: i64) -> u32 {
    i32::try_from(v).expect("sample index within i32").cast_unsigned()
}

/// The jobs, sizes and copies of `requests`, in order.
pub fn plan(requests: &[FillRequest]) -> FillPlan {
    let mut p = FillPlan::default();
    for r in requests {
        let (w, h) = (u(r.rect.size[0]), u(r.rect.size[1]));
        if w == 0 || h == 0 {
            continue;
        }
        let level = u32::from(r.level);
        let mut job = [0u32; JOB_WORDS];
        job[1] = level;
        job[3] = w;
        job[4] = h;
        job[5] = i(r.rect.min[0]);
        job[6] = i(r.rect.min[1]);
        job[12] = p.threads[0];
        match r.kind {
            FillKind::Heights => {
                let (ph, pn) = (pitch_words(w, 4), pitch_words(w, 2));
                job[0] = 0;
                job[2] = level;
                job[7] = p.scratch_len;
                job[8] = p.out_len;
                job[9] = ph;
                job[10] = p.out_len + ph * h;
                job[11] = pn;
                job[13] = (w + 2) * (h + 2);
                job[14] = p.threads[1];
                job[15] = w * h;
                p.copies.push(FillCopy {
                    target: FillTarget::Heights,
                    layer: level,
                    texel: r.texel,
                    size: [w, h],
                    offset: u64::from(p.out_len) * 4,
                    bytes_per_row: ph * 4,
                });
                p.copies.push(FillCopy {
                    target: FillTarget::Normals,
                    layer: level,
                    texel: r.texel,
                    size: [w, h],
                    offset: u64::from(job[10]) * 4,
                    bytes_per_row: pn * 4,
                });
                p.scratch_len += job[13];
                p.out_len += (ph + pn) * h;
                p.threads[1] += job[15];
            }
            FillKind::Macro => {
                let ph = pitch_words(w, 4);
                job[0] = 1;
                job[2] = level + 1;
                job[8] = p.out_len;
                job[9] = ph;
                job[13] = w * h;
                job[14] = p.threads[1];
                p.copies.push(FillCopy {
                    target: FillTarget::Macro,
                    layer: level,
                    texel: r.texel,
                    size: [w, h],
                    offset: u64::from(p.out_len) * 4,
                    bytes_per_row: ph * 4,
                });
                p.out_len += ph * h;
            }
        }
        p.threads[0] += job[13];
        p.texels += u64::from(w * h);
        p.jobs.push(job);
    }
    p
}

/// The provider's field WGSL followed by the fill passes: one shader module.
pub fn compose(field_wgsl: &str) -> String {
    format!("{field_wgsl}\n{GPU_FILL_WGSL}")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn rect(x: i64, z: i64, w: i64, h: i64) -> SampleRect {
        SampleRect { min: [x, z], size: [w, h] }
    }

    #[test]
    fn plan_lays_jobs_out_back_to_back_with_aligned_rows() {
        let p = plan(&[
            FillRequest { kind: FillKind::Heights, level: 3, rect: rect(-5, 7, 100, 3), texel: [507, 7] },
            FillRequest { kind: FillKind::Macro, level: 2, rect: rect(0, 0, 10, 2), texel: [0, 0] },
            FillRequest { kind: FillKind::Heights, level: 0, rect: rect(1, 1, 0, 9), texel: [1, 1] },
        ]);
        assert_eq!(p.jobs.len(), 2, "empty pieces are skipped");
        let [a, b] = [p.jobs[0], p.jobs[1]];
        assert_eq!(a[5].cast_signed(), -5);
        assert_eq!((a[13], a[15]), (102 * 5, 300));
        assert_eq!((b[12], b[13], b[2]), (510, 20, 3), "macro threads follow, its shift is level + 1");
        assert_eq!(p.threads, [530, 300]);
        assert_eq!(p.scratch_len, 510);
        // Heights rows: 400 B -> 512 B (128 words); normals 200 B -> 256 B (64 words); macro 40 B -> 256 B.
        assert_eq!((a[9], a[11], a[10]), (128, 64, 128 * 3));
        assert_eq!(b[8], (128 + 64) * 3);
        assert_eq!(p.out_len, (128 + 64) * 3 + 64 * 2);
        assert!(p.copies.iter().all(|c| c.bytes_per_row % ROW_ALIGN == 0 && c.offset % 4 == 0));
        assert_eq!(p.copies.len(), 3);
        assert_eq!(p.texels, 320);
    }

    #[test]
    fn dispatches_fold_into_two_dimensions() {
        assert_eq!(FillPlan::groups(0), [0, 0]);
        assert_eq!(FillPlan::groups(65), [2, 1]);
        let [x, y] = FillPlan::groups(FILL_WORKGROUP * FILL_GROUPS_X * 2 + 1);
        assert_eq!((x, y), (FILL_GROUPS_X, 3));
        assert!(GPU_FILL_WGSL.contains(&format!("const FILL_GROUPS_X: u32 = {FILL_GROUPS_X}u;")));
        assert!(GPU_FILL_WGSL.contains(&format!("@workgroup_size({FILL_WORKGROUP})")));
    }
}
