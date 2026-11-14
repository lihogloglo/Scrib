//! Rebind on reallocation (textures.md §6 "Part B: residency", slice S2): when an image is replaced
//! under its existing handle, every material that uses it is marked modified so Bevy rebuilds its
//! bind group with the new texture view.
//!
//! **Why Bevy needs it (0.20 source).** Replacing an image (`Assets<Image>::insert` on a live id,
//! or a reload) queues `AssetEvent::<Image>::Modified` for that image only
//! (`bevy_asset/src/assets.rs:383-396`). Materials are re-extracted and re-prepared only on their
//! own `AssetEvent` (`bevy_render/src/erased_render_asset.rs:311-329`), and nothing in
//! `bevy_pbr`/`bevy_render` reads `AssetEvent<Image>` for materials (only `camera.rs` for render
//! targets). `GpuImage::prepare_asset` keeps the old `TextureView` only when the descriptor is
//! unchanged and the texture has `COPY_DST` (`bevy_render/src/texture/gpu_image.rs:202-216`); any
//! other replacement (another size, another mip count: what residency does) makes a new view,
//! and the material's bind group keeps the old one. A touched material goes through
//! `prepare_material` again (`bevy_render/src/material_bind_groups.rs:2762`), whose fast path
//! `try_update_data` sees the changed GPU resource and frees and reallocates the binding
//! (:1136-1169, :2796-2810), so the slab's bind group is rebuilt.
//!
//! **What it does.** [`ImageRebindPlugin<M>`] keeps [`ImageUsers<M>`], a reverse index image ->
//! materials of type `M`, from `AssetEvent<M>` (`Added`/`Modified` re-read the material's images,
//! `Removed` drops it). On `AssetEvent<Image>::Modified` it touches (`Assets::get_mut` +
//! deref, which queues `AssetEvent::<M>::Modified`) each material using a replaced image, once per
//! frame however many of its slots or images were replaced. A material's images are what its
//! `#[dependency]` fields visit (`VisitAssetDependencies`), so it works for any material type
//! that tags its image handles, `StandardMaterial` included; a type that doesn't (e.g. 0.20's
//! `ExtendedMaterial`, which does not visit `base`) is not indexed.
//!
//! `LoadedWithDependencies` is not a trigger: it also fires on first loads (a material waiting
//! for its first image retries by itself, `bind_group.rs` `RetryNextUpdate`), and a replaced
//! asset already emits `Modified` (`insert` on an occupied slot).
//!
//! **Timing.** It runs in `PostUpdate` after `AssetEventSystems` (where the image event is
//! emitted); the material events it queues go out in the next frame's `AssetEventSystems`, so the
//! material rebinds one frame after the image. For that frame the old bind group still holds the
//! old view, which keeps the old texture alive (Bevy never `destroy`s an image texture): one frame
//! of the old texture, never an invalid binding. Bevy's per-type `asset_events` systems are
//! `pub(crate)`, so ordering between them is not possible.
//!
//! **Consumers of material events.** A touch is an ordinary material `Modified`: anything keyed
//! on material bind groups already invalidates on it (the static scene's `invalidate_chain_slabs`
//! re-asks the material's binding), so no extra hook is needed.
//!
//! Counters: [`ImageRebindStats`] (images replaced, materials touched), log line
//! `texture.rebind: ...` once each burst settles.

use std::collections::{HashMap, HashSet};
use std::marker::PhantomData;

use bevy_app::{App, Plugin, PostUpdate};
use bevy_asset::{Asset, AssetEvent, AssetEventSystems, AssetId, Assets, UntypedAssetId};
use bevy_ecs::message::{MessageCursor, Messages};
use bevy_ecs::prelude::*;
use bevy_image::Image;
use bevy_log::info;
use bevy_time::{Real, Time};

use crate::SETTLE;

/// Keeps [`ImageUsers<M>`] and touches the users of a replaced image (module docs).
pub struct ImageRebindPlugin<M>(PhantomData<fn() -> M>);

impl<M> Default for ImageRebindPlugin<M> {
    fn default() -> Self {
        Self(PhantomData)
    }
}

/// Reverse index: which materials of type `M` use each image.
#[derive(Resource)]
pub struct ImageUsers<M: Asset> {
    by_image: HashMap<AssetId<Image>, HashSet<AssetId<M>>>,
    by_material: HashMap<AssetId<M>, Vec<AssetId<Image>>>,
}

impl<M: Asset> Default for ImageUsers<M> {
    fn default() -> Self {
        Self { by_image: HashMap::new(), by_material: HashMap::new() }
    }
}

impl<M: Asset> ImageUsers<M> {
    /// The materials using `image`.
    pub fn users(&self, image: AssetId<Image>) -> impl Iterator<Item = AssetId<M>> + '_ {
        self.by_image.get(&image).into_iter().flatten().copied()
    }

    /// Indexed materials (those with at least one image).
    #[must_use]
    pub fn materials(&self) -> usize {
        self.by_material.len()
    }

    /// Images used by at least one indexed material.
    #[must_use]
    pub fn images(&self) -> usize {
        self.by_image.len()
    }

    /// Sets `material`'s images (replacing what it had).
    pub fn set(&mut self, material: AssetId<M>, images: Vec<AssetId<Image>>) {
        self.remove(material);
        if images.is_empty() {
            return;
        }
        for &image in &images {
            self.by_image.entry(image).or_default().insert(material);
        }
        self.by_material.insert(material, images);
    }

    /// Drops `material` from the index.
    pub fn remove(&mut self, material: AssetId<M>) {
        let Some(images) = self.by_material.remove(&material) else { return };
        for image in images {
            if let Some(users) = self.by_image.get_mut(&image) {
                users.remove(&material);
                if users.is_empty() {
                    self.by_image.remove(&image);
                }
            }
        }
    }
}

/// The distinct images a material's `#[dependency]` fields hold.
pub fn images_of<M: Asset>(material: &M) -> Vec<AssetId<Image>> {
    let mut images = Vec::new();
    material.visit_dependencies(&mut |id: UntypedAssetId| {
        if let Ok(id) = id.try_typed::<Image>()
            && !images.contains(&id)
        {
            images.push(id);
        }
    });
    images
}

/// Running totals since start, over every registered material type.
#[derive(Resource, Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct ImageRebindStats {
    /// `AssetEvent::<Image>::Modified` events (an image replaced under its handle).
    pub images_replaced: u64,
    /// Material touches (one per material per frame with a replaced image).
    pub materials_touched: u64,
}

impl<M: Asset> Plugin for ImageRebindPlugin<M> {
    fn build(&self, app: &mut App) {
        app.init_resource::<ImageUsers<M>>();
        // The image counter and the log line once, however many material types are registered.
        if !app.world().contains_resource::<ImageRebindStats>() {
            app.init_resource::<ImageRebindStats>()
                .add_systems(PostUpdate, (count_replaced_images, log_rebind_counts).chain().after(AssetEventSystems));
        }
        app.add_systems(PostUpdate, (index_materials::<M>, touch_users::<M>).chain().after(AssetEventSystems));
    }
}

/// Updates [`ImageUsers<M>`] from this frame's material events. Without `Assets<M>` (the type is
/// not registered) there is nothing to index.
pub fn index_materials<M: Asset>(
    mut users: ResMut<ImageUsers<M>>,
    events: Option<Res<Messages<AssetEvent<M>>>>,
    assets: Option<Res<Assets<M>>>,
    mut cursor: Local<MessageCursor<AssetEvent<M>>>,
) {
    let (Some(events), Some(assets)) = (events, assets) else { return };
    for event in cursor.read(&events) {
        match *event {
            AssetEvent::Added { id } | AssetEvent::Modified { id } => match assets.get(id) {
                Some(material) => users.set(id, images_of(material)),
                None => users.remove(id),
            },
            AssetEvent::Removed { id } => users.remove(id),
            AssetEvent::Unused { .. } | AssetEvent::LoadedWithDependencies { .. } => {}
        }
    }
}

/// Touches every material of type `M` that uses an image replaced this frame, once each.
pub fn touch_users<M: Asset>(
    users: Res<ImageUsers<M>>,
    images: Option<Res<Messages<AssetEvent<Image>>>>,
    assets: Option<ResMut<Assets<M>>>,
    stats: Option<ResMut<ImageRebindStats>>,
    mut cursor: Local<MessageCursor<AssetEvent<Image>>>,
) {
    let (Some(images), Some(mut assets)) = (images, assets) else { return };
    let touched: HashSet<AssetId<M>> = cursor
        .read(&images)
        .filter_map(|event| if let AssetEvent::Modified { id } = *event { Some(id) } else { None })
        .flat_map(|image| users.users(image))
        .collect();
    let mut count = 0;
    for id in touched {
        if let Some(material) = assets.get_mut(id) {
            // Marks it changed: queues `AssetEvent::<M>::Modified` when the guard drops.
            let _ = material.into_inner();
            count += 1;
        }
    }
    if let Some(mut stats) = stats {
        stats.materials_touched += count;
    }
}

/// Counts `AssetEvent::<Image>::Modified`.
fn count_replaced_images(
    mut stats: ResMut<ImageRebindStats>,
    images: Option<Res<Messages<AssetEvent<Image>>>>,
    mut cursor: Local<MessageCursor<AssetEvent<Image>>>,
) {
    let Some(images) = images else { return };
    let replaced = cursor.read(&images).filter(|event| matches!(event, AssetEvent::Modified { .. })).count();
    if replaced > 0 {
        stats.images_replaced += replaced as u64;
    }
}

/// `texture.rebind: ...` once each burst of replacements has settled for [`SETTLE`].
fn log_rebind_counts(
    stats: Res<ImageRebindStats>,
    time: Option<Res<Time<Real>>>,
    mut seen: Local<(ImageRebindStats, std::time::Duration)>,
    mut logged: Local<ImageRebindStats>,
) {
    let Some(time) = time else { return };
    let now = *stats;
    if now != seen.0 {
        *seen = (now, time.elapsed());
        return;
    }
    if now != *logged && time.elapsed().saturating_sub(seen.1) >= SETTLE {
        *logged = now;
        info!("{}", rebind_line(now));
    }
}

/// The log line: `texture.rebind: 12 images replaced, 30 materials touched`.
#[must_use]
pub fn rebind_line(s: ImageRebindStats) -> String {
    format!("texture.rebind: {} images replaced, {} materials touched", s.images_replaced, s.materials_touched)
}
