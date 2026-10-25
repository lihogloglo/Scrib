//! The far shadow map (weather.md "Distant shadows", tier 2; terrain.md
//! "Far shadows"): [`sf_terrain::shadow::far_shadow_map`] around the camera,
//! toward the shadowed directional light, computed on the async pool and
//! uploaded as a filterable texture. Materials that can't march the height
//! clipmap (far-field impostors and proxies) read it with one fetch; the
//! terrain itself marches the clipmap (`shaders/far_shadow.wesl`).

use std::time::{Duration, Instant};

use bevy_asset::{Assets, Handle, RenderAssetUsages};
use bevy_ecs::prelude::*;
use bevy_image::{Image, ImageSampler};
use bevy_light::DirectionalLight;
use bevy_math::{Vec3, Vec4};
use bevy_render::render_resource::{Extent3d, TextureDimension, TextureFormat};
use bevy_tasks::{AsyncComputeTaskPool, Task, block_on, poll_once};
use bevy_transform::components::GlobalTransform;
use sf_coords::WorldPos;
use sf_terrain::shadow::{ShadowHeights, ShadowMarch, far_shadow_map};

use crate::{TerrainCamera, TerrainHeightsMap, TerrainOrigin, TerrainState};

/// Texels per side.
pub const FAR_MAP_SIZE: usize = 384;
/// Texel size, meters (the map spans 9.2 km).
pub const FAR_MAP_TEXEL: f64 = 24.0;
/// The march: first step two texels out (closer occluders are the
/// cascades' and the receiver's own shading), 4 km reach.
const MARCH: ShadowMarch = ShadowMarch { steps: 40, first: 2.0 * FAR_MAP_TEXEL, reach: 4000.0 };
/// Refresh when the light turns this much (cos of 0.25°), the camera moves
/// this far from the map's center, or this long passes (tiles stream in).
const REFRESH_COS: f32 = 0.999_990_5;
const REFRESH_MOVE: f64 = 1000.0;
const REFRESH_AGE: Duration = Duration::from_secs(8);

/// The far shadow map, for other plugins' materials.
#[derive(Resource, Clone, Debug, PartialEq)]
pub struct FarShadowMap {
    /// `Rg16Float`, [`FAR_MAP_SIZE`]²: (shadow height − base height,
    /// occluder distance), meters; texel (0, 0) at the map's min corner.
    pub image: Handle<Image>,
    /// Render space: (x, z) of the min corner, 1 / extent (1/m; 0 = no map
    /// yet), base height (render y). Follows the floating origin.
    pub params: Vec4,
    /// Bumps with every upload (consumers re-bind the image).
    pub generation: u32,
}

/// A finished map and where it was computed.
struct Computed {
    map: ShadowHeights,
    center: WorldPos,
    ms: f32,
}

#[derive(Default)]
pub(crate) struct FarMapState {
    task: Option<Task<Computed>>,
    /// Center and light of the uploaded (or in-flight) map, and when.
    center: Option<WorldPos>,
    light: Option<Vec3>,
    at: Option<Instant>,
    /// Center of the uploaded map (its params).
    uploaded: Option<WorldPos>,
}

pub(crate) fn setup_far_map(mut commands: Commands, mut images: ResMut<Assets<Image>>) {
    let size = u32::try_from(FAR_MAP_SIZE).expect("small");
    let mut image = Image::new_fill(
        Extent3d { width: size, height: size, depth_or_array_layers: 1 },
        TextureDimension::D2,
        &[0; 4],
        TextureFormat::Rg16Float,
        RenderAssetUsages::default(),
    );
    image.sampler = ImageSampler::linear();
    commands.insert_resource(FarShadowMap { image: images.add(image), params: Vec4::ZERO, generation: 0 });
}

/// Uploads finished maps, keeps the params on the floating origin, and
/// starts a refresh when the light or camera has moved.
#[allow(clippy::too_many_arguments)]
pub(crate) fn update_far_map(
    mut local: Local<FarMapState>,
    map: Option<ResMut<FarShadowMap>>,
    mut images: ResMut<Assets<Image>>,
    state: Option<Res<TerrainState>>,
    heights_map: Res<TerrainHeightsMap>,
    origin: Res<TerrainOrigin>,
    camera: Query<&GlobalTransform, With<TerrainCamera>>,
    lights: Query<(&DirectionalLight, &GlobalTransform)>,
    mut terrain_materials: ResMut<Assets<crate::TerrainMaterial>>,
) {
    let (Some(mut map), Some(state)) = (map, state) else { return };
    let local = &mut *local;

    // 1. A finished map: upload it.
    if let Some(task) = &mut local.task
        && let Some(done) = block_on(poll_once(task))
    {
        local.task = None;
        let base = done.center.to_f64().y;
        if let Some(mut image) = images.get_mut(&map.image) {
            let mut bytes = Vec::with_capacity(done.map.data.len() * 4);
            for [hs, occ] in &done.map.data {
                #[allow(clippy::cast_possible_truncation)]
                bytes.extend(f16_bits((f64::from(*hs) - base) as f32).to_le_bytes());
                bytes.extend(f16_bits(*occ).to_le_bytes());
            }
            image.data = Some(bytes);
        }
        local.uploaded = Some(done.center);
        map.generation = map.generation.wrapping_add(1);
        // Re-bind the re-created texture.
        let ids: Vec<_> = terrain_materials.ids().collect();
        for id in ids {
            if let Some(mut m) = terrain_materials.get_mut(id) {
                let _: &mut crate::TerrainMaterial = &mut m;
            }
        }
        log::debug!("far shadow map: {:.0} ms", done.ms);
    }

    // 2. Params in render space (the origin may have moved).
    if let Some(center) = local.uploaded {
        let c = origin.0.delta(center);
        #[allow(clippy::cast_precision_loss)]
        let half = FAR_MAP_SIZE as f64 * FAR_MAP_TEXEL * 0.5;
        #[allow(clippy::cast_possible_truncation)]
        let params = Vec4::new((c.x - half) as f32, (c.z - half) as f32, (0.5 / half) as f32, c.y as f32);
        if map.params != params {
            map.params = params;
        }
    }

    // 3. Start a refresh when due.
    if local.task.is_some() {
        return;
    }
    let Some(cam) = camera.iter().next() else { return };
    let Some(to_light) = lights.iter().find(|(l, _)| l.shadow_maps_enabled).map(|(_, t)| t.back().as_vec3()) else {
        return;
    };
    let cam_world = origin.0.offset(cam.translation());
    let due = match (local.center, local.light, local.at) {
        (Some(c), Some(l), Some(at)) => {
            let moved = c.delta(cam_world);
            l.dot(to_light) < REFRESH_COS || moved.x.hypot(moved.z) > REFRESH_MOVE || at.elapsed() > REFRESH_AGE
        }
        _ => true,
    };
    if !due {
        return;
    }
    let Some(heights) = heights_map.0.get(&state.ws).cloned() else { return };
    // Snap the center to the texel grid so texels don't swim between maps.
    let p = cam_world.to_f64();
    let snap = |v: f64| (v / FAR_MAP_TEXEL).round() * FAR_MAP_TEXEL;
    let center = WorldPos::from_f64(bevy_math::DVec3::new(snap(p.x), p.y.round(), snap(p.z)));
    let l = to_light.as_dvec3().to_array();
    local.center = Some(center);
    local.light = Some(to_light);
    local.at = Some(Instant::now());
    local.task = Some(AsyncComputeTaskPool::get().spawn(async move {
        let started = Instant::now();
        let map = far_shadow_map(&heights, center, FAR_MAP_SIZE, FAR_MAP_TEXEL, l, MARCH);
        Computed { map, center, ms: started.elapsed().as_secs_f32() * 1e3 }
    }));
}

/// IEEE half bits of `v`, rounded to nearest; out-of-range values clamp to
/// the largest finite half, tiny ones flush to zero.
fn f16_bits(v: f32) -> u16 {
    let b = v.to_bits();
    #[allow(clippy::cast_possible_truncation)]
    let sign = ((b >> 16) & 0x8000) as u16;
    #[allow(clippy::cast_possible_wrap)]
    let exp = ((b >> 23) & 0xff) as i32 - 127 + 15;
    if v.is_nan() || exp <= 0 {
        return sign;
    }
    if exp >= 31 {
        return sign | 0x7bff;
    }
    let man = b & 0x007f_ffff;
    #[allow(clippy::cast_sign_loss)]
    let h = ((exp as u32) << 10 | (man >> 13)) + ((man >> 12) & 1);
    #[allow(clippy::cast_possible_truncation)]
    let h = h.min(0x7bff) as u16;
    sign | h
}

#[cfg(test)]
mod tests {
    use super::f16_bits;

    #[test]
    fn half_bits() {
        assert_eq!(f16_bits(0.0), 0);
        assert_eq!(f16_bits(1.0), 0x3c00);
        assert_eq!(f16_bits(-2.0), 0xc000);
        assert_eq!(f16_bits(1000.0), 0x63d0);
        assert_eq!(f16_bits(1.0e6), 0x7bff);
    }
}
