//! Fog on blended NIF surfaces (superfargoth weather.md "Transparents", fogfix
//! 2026-10-19). The fog composite runs on the depth buffer before the
//! transparent pass, so alpha-blended and additive surfaces (foliage without an
//! alpha test, the Ghostfence's energy field, glows, glass, particles) drew
//! over it unfogged at any distance. As in UE and Frostbite, each translucent
//! material fogs itself from the froxel volume at its own screen position and
//! distance: their `StandardMaterial` becomes `sf_atmos_bevy`'s
//! `FroxelFogMaterial` (blend: dimmed + in-scatter; additive: dimmed only, so a
//! glow fades with the transmittance and adds no fog light).
//!
//! One fogged copy per source material, swapped onto the entity; NIF material
//! controllers keep animating the source, and every change is copied over.
//! `--off transfog` is the A/B (the old, unfogged look).

use bevy::platform::collections::HashMap;
use bevy::prelude::*;
use sf_atmos_bevy::{FroxelFogExt, FroxelFogMaterial, FroxelVolume};

/// The fogged copy of each blended source material.
#[derive(Resource, Default)]
struct Fogged(HashMap<AssetId<StandardMaterial>, Handle<FroxelFogMaterial>>);

pub(crate) fn add(app: &mut App) {
    app.init_resource::<Fogged>().add_systems(PostUpdate, (swap, sync).chain());
}

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
        commands.entity(entity).remove::<MeshMaterial3d<StandardMaterial>>().insert(MeshMaterial3d(handle));
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
