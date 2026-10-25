// The shared surface-state hook (ARCHITECTURE §10, WISHLIST early decision
// 5): every framework material calls it after building its PbrInput.
// Transient deformation, snow, mud, ash and wetness (M6) will modify the
// surface here, reading the camera-centered surface-state ring buffer.
// Until then it is a no-op.

import bevy_pbr::render::pbr_types::PbrInput;

fn apply_surface_state(pbr: ptr<function, PbrInput>, world_xz: vec2<f32>) {
}
