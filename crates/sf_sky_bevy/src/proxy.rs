//! Emissive proxies for punctual lights on the ray-traced path.
//!
//! Bevy Solari 0.20 lights the scene from directional lights, emissive
//! meshes and environment maps only; `PointLight`, `SpotLight` and
//! `RectLight` are ignored. Until it supports them, each point light with a
//! [`PointLightProxy`] gets a child sphere that is registered only with the
//! ray tracer (`RaytracingMesh3d`, no `Mesh3d`): it lights the scene and
//! shows in reflections, but is never rasterized, so it adds nothing to the
//! raster path and no visible blob. Its luminance is set so that, from
//! afar, it has the light's intensity ([`sf_sky::sphere_light_luminance`]).
//! The raster light stays (forward-rendered transparent surfaces still use
//! it) with its shadow maps off, since ray tracing provides the shadows.
//!
//! Spot lights are not proxied (a sphere can't express the cone).

use bevy_app::App;
use bevy_ecs::prelude::*;
use bevy_light::PointLight;

use crate::{SkyRenderOptions, SkySystems};

/// Represent this point light with an emissive proxy on the ray-traced
/// path. Added to every `PointLight` automatically when
/// [`SkyRenderOptions::auto_light_proxies`] is set.
#[derive(Component, Clone, Copy, Debug, Default, PartialEq)]
pub struct PointLightProxy {
    /// Emitter radius in meters. `None`: the light's own `radius`, or
    /// [`SkyRenderOptions::default_proxy_radius`] when that is 0.
    pub radius: Option<f32>,
}

/// Opt a point light out of automatic proxies.
#[derive(Component, Clone, Copy, Debug, Default)]
pub struct NoLightProxy;

/// On a proxy entity: the light it stands in for.
#[derive(Component, Clone, Copy, Debug)]
pub struct LightProxy {
    pub light: Entity,
}

pub(crate) fn build(app: &mut App) {
    app.add_systems(
        bevy_app::Update,
        auto_proxies.in_set(SkySystems::RenderPath),
    );
    #[cfg(feature = "solari")]
    solari::build(app);
}

#[allow(clippy::type_complexity)]
fn auto_proxies(
    mut commands: Commands,
    options: Res<SkyRenderOptions>,
    lights: Query<
        Entity,
        (
            With<PointLight>,
            Without<PointLightProxy>,
            Without<NoLightProxy>,
        ),
    >,
) {
    if options.auto_light_proxies {
        for entity in &lights {
            commands.entity(entity).insert(PointLightProxy::default());
        }
    }
}

#[cfg(feature = "solari")]
mod solari {
    use std::f32::consts::{PI, TAU};

    use bevy_app::{App, Startup, Update};
    use bevy_asset::{Assets, Handle, RenderAssetUsages};
    use bevy_camera::visibility::InheritedVisibility;
    use bevy_color::{Color, LinearRgba};
    use bevy_ecs::prelude::*;
    use bevy_light::PointLight;
    use bevy_math::Vec3;
    use bevy_mesh::{Indices, Mesh, PrimitiveTopology};
    use bevy_pbr::{MeshMaterial3d, StandardMaterial};
    use bevy_solari::prelude::RaytracingMesh3d;
    use bevy_transform::components::Transform;

    use super::{LightProxy, PointLightProxy};
    use crate::{ActiveRenderPath, SkyRenderOptions, SkySystems};
    use sf_sky::RenderPath;

    /// On a proxied light: its proxy, and what the proxy changed.
    #[derive(Component, Clone, Debug)]
    pub(super) struct ProxyLink {
        proxy: Entity,
        material: Handle<StandardMaterial>,
        shadows_were_enabled: bool,
    }

    #[derive(Resource)]
    pub(super) struct ProxyMesh(Handle<Mesh>);

    pub(super) fn build(app: &mut App) {
        app.add_systems(Startup, create_proxy_mesh).add_systems(
            Update,
            (attach_or_detach, update_proxies)
                .chain()
                .after(super::auto_proxies)
                .in_set(SkySystems::RenderPath),
        );
    }

    fn create_proxy_mesh(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>) {
        commands.insert_resource(ProxyMesh(meshes.add(unit_sphere(16, 8))));
    }

    /// A unit UV sphere in the layout Solari requires: position, normal,
    /// uv0, tangent; u32 indices; triangle list.
    fn unit_sphere(sectors: u32, stacks: u32) -> Mesh {
        let mut positions = Vec::new();
        let mut normals = Vec::new();
        let mut uvs = Vec::new();
        let mut tangents = Vec::new();
        for i in 0..=stacks {
            #[expect(clippy::cast_precision_loss, reason = "small counts")]
            let v = i as f32 / stacks as f32;
            let (sin_t, cos_t) = (v * PI).sin_cos();
            for j in 0..=sectors {
                #[expect(clippy::cast_precision_loss, reason = "small counts")]
                let u = j as f32 / sectors as f32;
                let (sin_p, cos_p) = (u * TAU).sin_cos();
                let n = Vec3::new(sin_t * cos_p, cos_t, sin_t * sin_p);
                positions.push(n.to_array());
                normals.push(n.to_array());
                uvs.push([u, v]);
                tangents.push([-sin_p, 0.0, cos_p, 1.0]);
            }
        }
        let mut indices = Vec::new();
        let row = sectors + 1;
        for i in 0..stacks {
            for j in 0..sectors {
                let a = i * row + j;
                let b = a + row;
                indices.extend_from_slice(&[a, a + 1, b, a + 1, b + 1, b]);
            }
        }
        Mesh::new(
            PrimitiveTopology::TriangleList,
            RenderAssetUsages::default(),
        )
        .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, positions)
        .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, normals)
        .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, uvs)
        .with_inserted_attribute(Mesh::ATTRIBUTE_TANGENT, tangents)
        .with_inserted_indices(Indices::U32(indices))
    }

    fn proxy_radius(proxy: PointLightProxy, light: &PointLight, options: &SkyRenderOptions) -> f32 {
        proxy.radius.unwrap_or(if light.radius > 0.0 {
            light.radius
        } else {
            options.default_proxy_radius
        })
    }

    fn emissive(light: &PointLight, radius: f32, visible: bool) -> LinearRgba {
        if !visible {
            return LinearRgba::BLACK;
        }
        let luminance = sf_sky::sphere_light_luminance(light.intensity, radius);
        let c = light.color.to_linear();
        LinearRgba::rgb(c.red * luminance, c.green * luminance, c.blue * luminance)
    }

    #[allow(clippy::type_complexity, clippy::too_many_arguments)]
    fn attach_or_detach(
        mut commands: Commands,
        active: Res<ActiveRenderPath>,
        options: Res<SkyRenderOptions>,
        mesh: Option<Res<ProxyMesh>>,
        mut materials: ResMut<Assets<StandardMaterial>>,
        mut unlinked: Query<
            (
                Entity,
                &mut PointLight,
                &PointLightProxy,
                Option<&InheritedVisibility>,
            ),
            Without<ProxyLink>,
        >,
        mut linked: Query<(Entity, &mut PointLight, &ProxyLink)>,
        mut removed: RemovedComponents<PointLightProxy>,
    ) {
        let ray_traced = active.0 == RenderPath::RayTraced;
        if ray_traced && let Some(mesh) = mesh {
            for (entity, mut light, proxy, visibility) in &mut unlinked {
                let radius = proxy_radius(*proxy, &light, &options);
                let visible = visibility.is_none_or(|v| v.get());
                let material = materials.add(StandardMaterial {
                    base_color: Color::BLACK,
                    emissive: emissive(&light, radius, visible),
                    ..StandardMaterial::default()
                });
                let proxy_entity = commands
                    .spawn((
                        LightProxy { light: entity },
                        RaytracingMesh3d(mesh.0.clone()),
                        MeshMaterial3d(material.clone()),
                        Transform::from_scale(Vec3::splat(radius)),
                        ChildOf(entity),
                    ))
                    .id();
                let shadows_were_enabled = light.shadow_maps_enabled;
                if shadows_were_enabled {
                    light.shadow_maps_enabled = false;
                }
                commands.entity(entity).insert(ProxyLink {
                    proxy: proxy_entity,
                    material,
                    shadows_were_enabled,
                });
            }
        }
        if !ray_traced {
            for (entity, mut light, link) in &mut linked {
                detach(&mut commands, entity, &mut light, link);
            }
        }
        for entity in removed.read() {
            if let Ok((entity, mut light, link)) = linked.get_mut(entity) {
                detach(&mut commands, entity, &mut light, link);
            }
        }
    }

    fn detach(commands: &mut Commands, entity: Entity, light: &mut PointLight, link: &ProxyLink) {
        if let Ok(mut proxy) = commands.get_entity(link.proxy) {
            proxy.despawn();
        }
        if link.shadows_were_enabled {
            light.shadow_maps_enabled = true;
        }
        commands.entity(entity).remove::<ProxyLink>();
    }

    /// Follows changes of intensity, color, radius and visibility.
    #[allow(clippy::type_complexity)]
    fn update_proxies(
        options: Res<SkyRenderOptions>,
        mut materials: ResMut<Assets<StandardMaterial>>,
        lights: Query<
            (
                &PointLight,
                &PointLightProxy,
                &ProxyLink,
                Option<&InheritedVisibility>,
            ),
            Or<(
                Changed<PointLight>,
                Changed<PointLightProxy>,
                Changed<InheritedVisibility>,
            )>,
        >,
        mut transforms: Query<&mut Transform, With<LightProxy>>,
    ) {
        for (light, proxy, link, visibility) in &lights {
            let radius = proxy_radius(*proxy, light, &options);
            let visible = visibility.is_none_or(|v| v.get());
            let value = emissive(light, radius, visible);
            if let Some(material) = materials.get(&link.material)
                && material.emissive != value
                && let Some(mut material) = materials.get_mut(&link.material)
            {
                material.emissive = value;
            }
            if let Ok(mut transform) = transforms.get_mut(link.proxy) {
                let scale = Vec3::splat(radius);
                if transform.scale != scale {
                    transform.scale = scale;
                }
            }
        }
    }
}
