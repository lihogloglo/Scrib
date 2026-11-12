// sf_water_bevy: screen-space planar reflections of the opaque snapshot (water.md "Water pass,
// refraction, reflections" step 2; Cichocki, SIGGRAPH 2017; the CPU reference and its tests are
// `sf_water::sspr`).
// `project`: every snapshot pixel above the plane is mirrored in it and projected; its target
// texel keeps the lowest source (atomicMax of the inverted height bits): all sources of one
// target lie on one reflected ray, whose height grows with distance, so the lowest is the nearest.
// `resolve`: the source point is rebuilt from the target's own ray and that height, one- and
// two-texel holes (projection stretch) take a neighbour's height, and the colour is written
// premultiplied by a confidence that fades where the source nears the screen edge. 0 = miss: the
// water keeps the environment map there (sky, off screen, below the plane).

import bevy_render::view::View;

struct Sspr {
    // (plane level render-space Y, texel scale 1 or 2, reflection width, reflection height)
    plane: vec4<f32>,
}

@group(0) @binding(0) var<uniform> view: View;
@group(0) @binding(1) var<uniform> sspr: Sspr;
@group(0) @binding(2) var snapshot_depth: texture_2d<f32>;
@group(0) @binding(3) var snapshot_color: texture_2d<f32>;
@group(0) @binding(4) var<storage, read_write> hash: array<atomic<u32>>;
@group(0) @binding(5) var reflection: texture_storage_2d<rgba16float, write>;

// sf_water::sspr::MIN_HEIGHT: the shore line and the bed seen through the water are not mirrored.
const MIN_HEIGHT: f32 = 0.01;

fn size() -> vec2<u32> {
    return vec2<u32>(sspr.plane.zw);
}

fn uv_to_ndc(uv: vec2<f32>) -> vec2<f32> {
    return vec2(uv.x * 2.0 - 1.0, 1.0 - uv.y * 2.0);
}

// Viewport UV (y down) of world point `p`, and whether it is on screen in front of the camera.
fn project_uv(p: vec3<f32>) -> vec3<f32> {
    let clip = view.clip_from_world * vec4(p, 1.0);
    if clip.w <= 1e-6 {
        return vec3(0.0);
    }
    let ndc = clip.xy / clip.w;
    let uv = vec2(ndc.x * 0.5 + 0.5, 0.5 - ndc.y * 0.5);
    let on = all(uv >= vec2(0.0)) && all(uv < vec2(1.0));
    return vec3(uv, select(0.0, 1.0, on));
}

// Snapshot pixel coordinates of viewport UV `uv`.
fn uv_to_px(uv: vec2<f32>) -> vec2<f32> {
    return view.main_pass_viewport.xy + uv * view.main_pass_viewport.zw;
}

@compute @workgroup_size(8, 8, 1)
fn project(@builtin(global_invocation_id) id: vec3<u32>) {
    let dims = size();
    let scale = max(u32(sspr.plane.y), 1u);
    let px = id.xy * scale;
    if any(id.xy >= dims) || any(px >= textureDimensions(snapshot_depth)) {
        return;
    }
    let depth = textureLoad(snapshot_depth, vec2<i32>(px), 0).r;
    // Reverse Z: 0 is the far plane (sky, not drawn yet).
    if depth <= 0.0 {
        return;
    }
    let uv = (vec2<f32>(px) + 0.5 - view.main_pass_viewport.xy) / view.main_pass_viewport.zw;
    let w = view.world_from_clip * vec4(uv_to_ndc(uv), depth, 1.0);
    let p = w.xyz / w.w;
    let level = sspr.plane.x;
    let height = p.y - level;
    if height < MIN_HEIGHT {
        return;
    }
    let mirror = project_uv(vec3(p.x, level - height, p.z));
    if mirror.z < 0.5 {
        return;
    }
    let t = vec2<u32>(uv_to_px(mirror.xy)) / scale;
    if any(t >= dims) {
        return;
    }
    atomicMax(&hash[t.y * dims.x + t.x], 0xffffffffu - bitcast<u32>(height));
}

fn key_at(t: vec2<i32>) -> u32 {
    let dims = vec2<i32>(size());
    if any(t < vec2(0)) || any(t >= dims) {
        return 0u;
    }
    return atomicLoad(&hash[u32(t.y * dims.x + t.x)]);
}

@compute @workgroup_size(8, 8, 1)
fn resolve(@builtin(global_invocation_id) id: vec3<u32>) {
    let dims = size();
    if any(id.xy >= dims) {
        return;
    }
    let t = vec2<i32>(id.xy);
    var key = key_at(t);
    if key == 0u {
        // Holes: the projection stretches the mirror image (mostly vertically); a neighbour's
        // height on this texel's own ray fills them, the nearest neighbour winning as above.
        key = max(max(key_at(t + vec2(0, -1)), key_at(t + vec2(0, 1))), max(key_at(t + vec2(-1, 0)), key_at(t + vec2(1, 0))));
        if key == 0u {
            key = max(key_at(t + vec2(0, -2)), key_at(t + vec2(0, 2)));
        }
    }
    let level = sspr.plane.x;
    let camera = view.world_position;
    let scale = max(sspr.plane.y, 1.0);
    let uv = ((vec2<f32>(id.xy) + 0.5) * scale - view.main_pass_viewport.xy) / view.main_pass_viewport.zw;
    let near = view.world_from_clip * vec4(uv_to_ndc(uv), 1.0, 1.0);
    let dir = normalize(near.xyz / near.w - camera);
    if key == 0u || dir.y >= -1e-5 || camera.y <= level {
        textureStore(reflection, id.xy, vec4(0.0));
        return;
    }
    // sf_water::sspr::point_on_reflected_ray.
    let height = bitcast<f32>(0xffffffffu - key);
    let hit = camera + dir * ((level - camera.y) / dir.y);
    let up = vec3(dir.x, -dir.y, dir.z);
    let source = project_uv(hit + up * (height / up.y));
    if source.z < 0.5 {
        textureStore(reflection, id.xy, vec4(0.0));
        return;
    }
    let snap = vec2<i32>(textureDimensions(snapshot_color));
    let color = textureLoad(snapshot_color, clamp(vec2<i32>(uv_to_px(source.xy)), vec2(0), snap - 1), 0).rgb;
    // Fade out where the source nears the screen edge: the reflection leaves the screen there.
    let edge = min(source.xy, vec2(1.0) - source.xy);
    let confidence = smoothstep(0.0, 0.04, edge.x) * smoothstep(0.0, 0.08, edge.y);
    textureStore(reflection, id.xy, vec4(color * confidence, confidence));
}
