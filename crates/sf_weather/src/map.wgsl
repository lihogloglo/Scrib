// World weather map resolve (weather.md WE4b): the GPU twin of
// `ClimateRaster::site_at` + `WeatherMap::blend` (map.rs). One thread per
// map texel: its climate weights from the raster (own square + the nearest
// other climate within the reach, `border_blend`), then the blend of the
// two table rows, written to layers A, B, C, D at the texel's toroidal storage
// index (world texel mod size). Rects select what to write: the full window
// or the strips that entered it on a recenter.

struct MapParams {
    // World texel of the window's minimum corner.
    window: vec2<i32>,
    // Texels per side.
    size: i32,
    // Squares searched around the own square (half the band, rounded up).
    reach: i32,
    raster_size: vec2<i32>,
    // Raster origin in world texels: integer and fractional parts.
    origin_i: vec2<i32>,
    origin_f: vec2<f32>,
    cells_per_texel: f32,
    cell_m: f32,
    band_m: f32,
    rect_count: u32,
    // (min x, min z, width, height) in world texels; dispatch z picks one.
    rects: array<vec4<i32>, 2>,
}

@group(0) @binding(0) var<uniform> params: MapParams;
// Each square's climate as a table row index.
@group(0) @binding(1) var<storage, read> raster: array<u32>;
// Three vec4 per row: A coverage, type, wetness, storm; B base, thickness,
// density, erosion; C cirrus, precipitation, height-fog density, dimming.
@group(0) @binding(2) var<storage, read> rows: array<vec4<f32>>;
@group(0) @binding(3) var map_out: texture_storage_2d_array<rgba16float, write>;

fn own_at(x: i32, z: i32) -> u32 {
    return raster[u32(x + z * params.raster_size.x)];
}

// Distance in squares from the point (fraction `f` in its square) to the
// square `d` squares away, along one axis.
fn gap(d: i32, f: f32) -> f32 {
    if d < 0 {
        return f + f32(-d - 1);
    }
    if d > 0 {
        return (1.0 - f) + f32(d - 1);
    }
    return 0.0;
}

// `sf_weather::border_weight`: the own climate's weight `distance_m` inside it.
fn border_weight(distance_m: f32, band_m: f32) -> f32 {
    let half = max(0.5 * band_m, 1e-3);
    let t = clamp(distance_m / half, 0.0, 1.0);
    return 0.5 + 0.5 * t * t * (3.0 - 2.0 * t);
}

@compute @workgroup_size(8, 8, 1)
fn resolve(@builtin(global_invocation_id) id: vec3<u32>) {
    if id.z >= params.rect_count {
        return;
    }
    let rect = params.rects[id.z];
    let local = vec2<i32>(id.xy);
    if local.x >= rect.z || local.y >= rect.w {
        return;
    }
    let t = rect.xy + local;
    // Texel center in raster squares; past the raster, projected toward its
    // center onto the edge (`ClimateRaster::site_at`'s radial rule).
    let rs = vec2<f32>(params.raster_size);
    let c = 0.5 * rs;
    let d = (vec2<f32>(t - params.origin_i) + 0.5 - params.origin_f) * params.cells_per_texel - c;
    let k = min(1.0, min(c.x / max(abs(d.x), 1e-6), c.y / max(abs(d.y), 1e-6)));
    let l = clamp(c + d * k, vec2(0.0), rs - 1e-4);
    let g = vec2<i32>(floor(l));
    let f = l - vec2<f32>(g);
    let own = own_at(g.x, g.y);
    var found = false;
    var best_d = 0.0;
    var best_c = own;
    for (var dz = -params.reach; dz <= params.reach; dz += 1) {
        for (var dx = -params.reach; dx <= params.reach; dx += 1) {
            let x = g.x + dx;
            let z = g.y + dz;
            if x < 0 || z < 0 || x >= params.raster_size.x || z >= params.raster_size.y {
                continue;
            }
            let c = own_at(x, z);
            if c == own {
                continue;
            }
            let d = length(vec2(gap(dx, f.x), gap(dz, f.y))) * params.cell_m;
            if !found || d < best_d || (d == best_d && c < best_c) {
                found = true;
                best_d = d;
                best_c = c;
            }
        }
    }
    var w = 1.0;
    if found {
        w = border_weight(best_d, params.band_m);
    }
    let s = ((t % params.size) + params.size) % params.size;
    for (var k = 0u; k < 4u; k += 1u) {
        var v = rows[own * 4u + k] * w;
        if found && w < 1.0 {
            v += rows[best_c * 4u + k] * (1.0 - w);
        }
        textureStore(map_out, s, k, v);
    }
}
