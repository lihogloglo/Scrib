//! Fog on blended translucent surfaces for any host (render-order.md RO-4).
//!
//! The fog composite runs on the depth buffer before the transparent pass, so
//! alpha-blended and additive surfaces (foliage without an alpha test, energy
//! fields, glows, glass, particles) draw over it unfogged at any distance. As
//! in UE and Frostbite, each translucent material fogs itself from the froxel
//! volume at its own screen position and distance: [`FogTranslucentsPlugin`]
//! swaps every blended `StandardMaterial` for a [`FroxelFogMaterial`] (blend:
//! dimmed + in-scatter; additive: dimmed only, so a glow fades with the
//! transmittance and adds no fog light). The fog material also carries the
//! shadow policy (rule 4: translucents don't cast).
//!
//! One fogged copy per source material, swapped onto the entity; a host's
//! material animation keeps changing the source, and every change is copied
//! over. Hosts that don't want it simply don't add the plugin.

use std::collections::HashMap;

use bevy_app::{App, Plugin, PostUpdate};
use bevy_asset::{AssetEvent, AssetId, Assets, Handle};
use bevy_ecs::prelude::*;
use bevy_light::NotShadowCaster;
use bevy_pbr::{MeshMaterial3d, StandardMaterial};
use bevy_material::AlphaMode;

use crate::{FroxelFogExt, FroxelFogMaterial, FroxelVolume};

/// Swaps blended `StandardMaterial`s for the fogged [`FroxelFogMaterial`]
/// (needs `AtmosPlugin`'s [`FroxelVolume`]).
pub struct FogTranslucentsPlugin;

impl Plugin for FogTranslucentsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Fogged>().add_systems(PostUpdate, (swap, sync).chain());
    }
}

/// The fogged copy of each blended source material.
#[derive(Resource, Default)]
struct Fogged(HashMap<AssetId<StandardMaterial>, Handle<FroxelFogMaterial>>);


fn blended(m: &StandardMaterial) -> bool {
    matches!(m.alpha_mode, AlphaMode::Blend | AlphaMode::Add | AlphaMode::Premultiplied | AlphaMode::Multiply)
}

/// Puts the fogged copy on every entity drawing a blended `StandardMaterial`
/// (retried while the material isn't loaded yet).
#[allow(clippy::too_many_arguments)] // a system's parameters
fn swap(
    mut commands: Commands,
    changed: Query<(Entity, &MeshMaterial3d<StandardMaterial>), Changed<MeshMaterial3d<StandardMaterial>>>,
    all: Query<&MeshMaterial3d<StandardMaterial>>,
    standard: Res<Assets<StandardMaterial>>,
    mut fog_materials: ResMut<Assets<FroxelFogMaterial>>,
    volume: Res<FroxelVolume>,
    mut fogged: ResMut<Fogged>,
    mut pending: Local<Vec<Entity>>,
) {
    let retry = std::mem::take(&mut *pending);
    let candidates = changed.iter().map(|(e, _)| e).chain(retry);
    for entity in candidates {
        let Ok(material) = all.get(entity) else { continue };
        let Some(base) = standard.get(&material.0) else {
            pending.push(entity);
            continue;
        };
        if !blended(base) {
            continue;
        }
        let handle = fogged
            .0
            .entry(material.id())
            .or_insert_with(|| fog_materials.add(FroxelFogMaterial { base: base.clone(), extension: FroxelFogExt::new(&volume) }))
            .clone();
        // The marker too (rule 4): the material casts none, and without it shadow-map
        // visibility and caster tracking (lamp cache) still treat the mesh as a caster.
        commands.entity(entity).try_remove::<MeshMaterial3d<StandardMaterial>>().try_insert((MeshMaterial3d(handle), NotShadowCaster));
    }
}

/// Copies changed source materials (NIF controllers: alpha, colour, UV) into
/// their fogged copies, and forgets sources that are gone.
fn sync(
    mut events: MessageReader<AssetEvent<StandardMaterial>>,
    standard: Res<Assets<StandardMaterial>>,
    mut fog_materials: ResMut<Assets<FroxelFogMaterial>>,
    mut fogged: ResMut<Fogged>,
) {
    for event in events.read() {
        match event {
            AssetEvent::Modified { id } => {
                if let (Some(handle), Some(base)) = (fogged.0.get(id), standard.get(*id))
                    && let Some(mut m) = fog_materials.get_mut(handle)
                {
                    m.base = base.clone();
                }
            }
            AssetEvent::Removed { id } => {
                fogged.0.remove(id);
            }
            _ => {}
        }
    }
}
