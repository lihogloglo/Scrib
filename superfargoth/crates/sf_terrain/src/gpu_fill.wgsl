// Clipmap GPU fill (`gpu_fill.rs`; terrain.md "Runtime", worldgen.md "Pipeline" stage 7): the host half of a
// provider's `GpuField` (`provider.rs`). The provider's WGSL (`field_height`, `field_albedo`) comes first; this
// part declares the bindings and runs two passes over a frame's jobs (one job = one toroidal clipmap piece):
//
// - `fill_eval`: heights of every piece plus one sample of margin into `fill_scratch` (the normals difference
//   their neighbours), macro albedo straight into `fill_out` (sRGB RGBA8);
// - `fill_pack`: the pieces' heights (R32Float) and normals (Rg8Snorm, `clipmap::fill_normals`) into
//   `fill_out`, rows padded to 256 bytes for the buffer-to-texture copies.
//
// Threads are numbered across the frame's jobs (`first_*`, `count_*` prefix sums); a dispatch is
// (min(groups, FILL_GROUPS_X), ceil(groups / FILL_GROUPS_X)) workgroups of 64.

@group(0) @binding(0) var<storage, read> field_data: array<u32>;
@group(0) @binding(1) var<storage, read> fill_jobs: array<FillJob>;
@group(0) @binding(2) var<storage, read_write> fill_scratch: array<f32>;
@group(0) @binding(3) var<storage, read_write> fill_out: array<u32>;

const FILL_GROUPS_X: u32 = 32768u;
const FILL_KIND_HEIGHTS: u32 = 0u;

struct FillJob {
    kind: u32,
    /// Height level (heights) or macro level (macro).
    level: u32,
    /// Level-0 global index = sample index << shift.
    shift: u32,
    w: u32,
    h: u32,
    min_x: i32,
    min_z: i32,
    /// First f32 of the (w + 2) x (h + 2) margin heights.
    scratch: u32,
    /// First word and row pitch (words) of the heights or macro rows.
    out_h: u32,
    pitch_h: u32,
    /// First word and row pitch (words) of the normal rows (two texels a word).
    out_n: u32,
    pitch_n: u32,
    first_a: u32,
    count_a: u32,
    first_b: u32,
    count_b: u32,
}

fn fill_thread(wg: vec3<u32>, li: u32) -> u32 {
    return (wg.y * FILL_GROUPS_X + wg.x) * 64u + li;
}

fn fill_srgb(v: f32) -> f32 {
    let c = clamp(v, 0.0, 1.0);
    if c <= 0.0031308 {
        return c * 12.92;
    }
    return 1.055 * pow(c, 1.0 / 2.4) - 0.055;
}

@compute @workgroup_size(64)
fn fill_eval(@builtin(workgroup_id) wg: vec3<u32>, @builtin(local_invocation_index) li: u32) {
    let t = fill_thread(wg, li);
    let n = arrayLength(&fill_jobs);
    var j = 0u;
    loop {
        if j >= n {
            return;
        }
        let f = fill_jobs[j].first_a;
        if t >= f && t < f + fill_jobs[j].count_a {
            break;
        }
        j += 1u;
    }
    let job = fill_jobs[j];
    let local = t - job.first_a;
    let scale = i32(1u << job.shift);
    if job.kind == FILL_KIND_HEIGHTS {
        let gw = job.w + 2u;
        let s = vec2<i32>(job.min_x + i32(local % gw) - 1, job.min_z + i32(local / gw) - 1);
        fill_scratch[job.scratch + local] = field_height(s * scale, job.level);
    } else {
        let ix = local % job.w;
        let iz = local / job.w;
        let s = vec2<i32>(job.min_x + i32(ix), job.min_z + i32(iz));
        let c = field_albedo(s * scale, job.level);
        fill_out[job.out_h + iz * job.pitch_h + ix] = pack4x8unorm(vec4<f32>(fill_srgb(c.x), fill_srgb(c.y), fill_srgb(c.z), 1.0));
    }
}

/// The normal at margin-grid index `c` (row width `gw`), as `clipmap::fill_normals`: (x, z) snorm.
fn fill_normal(scratch: u32, c: u32, gw: u32, span: f32) -> vec2<f32> {
    let dx = (fill_scratch[scratch + c + 1u] - fill_scratch[scratch + c - 1u]) / span;
    let dz = (fill_scratch[scratch + c + gw] - fill_scratch[scratch + c - gw]) / span;
    let len = sqrt(dx * dx + dz * dz + 1.0);
    return vec2<f32>(-dx / len, -dz / len);
}

@compute @workgroup_size(64)
fn fill_pack(@builtin(workgroup_id) wg: vec3<u32>, @builtin(local_invocation_index) li: u32) {
    let t = fill_thread(wg, li);
    let n = arrayLength(&fill_jobs);
    var j = 0u;
    loop {
        if j >= n {
            return;
        }
        let f = fill_jobs[j].first_b;
        if t >= f && t < f + fill_jobs[j].count_b {
            break;
        }
        j += 1u;
    }
    let job = fill_jobs[j];
    let local = t - job.first_b;
    let ix = local % job.w;
    let iz = local / job.w;
    let gw = job.w + 2u;
    let c = (iz + 1u) * gw + ix + 1u;
    fill_out[job.out_h + iz * job.pitch_h + ix] = bitcast<u32>(fill_scratch[job.scratch + c]);
    if ix % 2u == 0u {
        let span = exp2(f32(job.level));
        let a = fill_normal(job.scratch, c, gw, span);
        var b = vec2<f32>(0.0);
        if ix + 1u < job.w {
            b = fill_normal(job.scratch, c + 1u, gw, span);
        }
        fill_out[job.out_n + iz * job.pitch_n + ix / 2u] = pack4x8snorm(vec4<f32>(a, b));
    }
}
