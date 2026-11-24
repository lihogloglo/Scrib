//! Exact test of the vendored `bevy_grass` slot-stable field tables (slice GC-3b4d1, own patch "stable tables"):
//! after a one-block move of a 5 x 5 ring, every field that stayed keeps its entries, chunks, mask and heightmap at
//! the same offsets with the same words, and the tables' recorded writes are exactly the new fields' ranges. Here
//! because cargo cannot test the vendored crate, which is not a workspace member.
//!
//! Also the partial upload built on them (slice GC-3b4d2, own patch "partial upload"): what the render world writes of
//! each version, as a pure plan, since the headless app has no GPU.

use std::collections::{HashMap, HashSet};
use std::ops::Range;

use bevy::asset::RenderAssetUsages;
use bevy::camera::visibility::VisibilitySystems;
use bevy::prelude::*;
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};
use bevy_grass::prelude::{GrassField, GrassGround, GrassPlugin, GrassStyle};
use bevy_grass::superfargoth_test::{
    BufferPlan, CHUNK_BYTES, FIELD_BYTES, FieldView, TablesView, UploadPlan, WrittenView, tables_view, upload_plan,
    uploaded_state,
};

const BLOCK: f32 = 64.0;

fn image(size: u32, format: TextureFormat, data: Vec<u8>) -> Image {
    Image::new(
        Extent3d { width: size, height: size, depth_or_array_layers: 1 },
        TextureDimension::D2,
        data,
        format,
        RenderAssetUsages::default(),
    )
}

/// A 16 x 16 density mask and a 17 x 17 heightmap, both unlike any other field's (`key`).
fn images(key: u32) -> (Image, Image) {
    let mask = (0..16 * 16u32)
        .flat_map(|i| [u8::try_from((i * 13 + key * 37) % 256).expect("below 256"), u8::MAX])
        .collect();
    #[expect(clippy::cast_precision_loss, reason = "small test values")]
    let heights = (0..17 * 17u32).flat_map(|i| ((i + key * 1000) as f32 * 1e-5).to_le_bytes()).collect();
    (image(16, TextureFormat::Rg8Unorm, mask), image(17, TextureFormat::R32Float, heights))
}

/// Headless the visibility plugin is absent: mark every field visible, as it would (once: a write every frame
/// would patch the tables' visibility every frame).
fn show(mut fields: Query<&mut InheritedVisibility, With<GrassField>>) {
    for mut visible in &mut fields {
        if !visible.get() {
            *visible = InheritedVisibility::VISIBLE;
        }
    }
}

fn spawn(app: &mut App, style: &Handle<GrassStyle>, (x, z): (i32, i32)) -> Entity {
    let key = u32::try_from((x + 10) * 100 + z + 10).expect("positive");
    let (mask, heights) = images(key);
    let mut assets = app.world_mut().resource_mut::<Assets<Image>>();
    let (mask, heights) = (assets.add(mask), assets.add(heights));
    #[expect(clippy::cast_precision_loss, reason = "small block coordinates")]
    let at = Vec3::new(x as f32 * BLOCK, 0.0, z as f32 * BLOCK);
    app.world_mut()
        .spawn((
            GrassField {
                size: Vec2::splat(BLOCK),
                style: style.clone(),
                ground: GrassGround::Heightmap { image: heights, height_scale: 4.0 },
                density_map: Some(mask),
            },
            Transform::from_translation(at),
            GlobalTransform::from_translation(at),
        ))
        .id()
}

fn settle(app: &mut App) -> TablesView {
    for _ in 0..4 {
        app.update();
    }
    tables_view(app.world())
}

/// `ranges`, sorted and merged where they touch.
fn merged(mut ranges: Vec<Range<u32>>) -> Vec<Range<u32>> {
    ranges.retain(|r| !r.is_empty());
    ranges.sort_by_key(|r| r.start);
    let mut out: Vec<Range<u32>> = Vec::new();
    for r in ranges {
        match out.last_mut() {
            Some(last) if last.end >= r.start => last.end = last.end.max(r.end),
            _ => out.push(r),
        }
    }
    out
}

/// A headless grass app with the 5 x 5 ring of fields spawned (not yet built).
fn ring_app() -> (App, Handle<GrassStyle>, HashMap<(i32, i32), Entity>) {
    let mut app = App::new();
    app.add_plugins((MinimalPlugins, AssetPlugin::default()))
        .init_asset::<Image>()
        .init_asset::<Mesh>()
        .init_asset::<Shader>()
        .add_plugins(GrassPlugin { wind: false, ..GrassPlugin::default() })
        .add_systems(PostUpdate, show.in_set(VisibilitySystems::VisibilityPropagate));
    let style = app.world_mut().resource_mut::<Assets<GrassStyle>>().add(GrassStyle::MEADOW);
    let mut ring = HashMap::new();
    for x in -2..=2 {
        for z in -2..=2 {
            ring.insert((x, z), spawn(&mut app, &style, (x, z)));
        }
    }
    (app, style, ring)
}

/// Moves the ring one block along +X: the column `x` leaves, `x + 5` arrives, in one frame.
fn move_column(app: &mut App, style: &Handle<GrassStyle>, ring: &mut HashMap<(i32, i32), Entity>, x: i32) {
    for z in -2..=2 {
        let gone = ring.remove(&(x, z)).expect("in the ring");
        app.world_mut().despawn(gone);
        ring.insert((x + 5, z), spawn(app, style, (x + 5, z)));
    }
}

#[test]
fn a_one_block_move_keeps_every_staying_field_in_place_and_writes_only_the_new_ones() {
    let (mut app, style, mut ring) = ring_app();
    let before = settle(&mut app);
    assert_eq!(before.fields.len(), 25);

    // One block along +X: the column x = -2 leaves, x = 3 arrives, in one frame.
    move_column(&mut app, &style, &mut ring, -2);
    let after = settle(&mut app);

    assert_eq!((after.since, after.version), (before.version, before.version + 1), "one rebuild");
    assert_eq!(after.fields.len(), 25);
    let old: HashMap<Entity, &FieldView> = before.fields.iter().map(|f| (f.entity, f)).collect();
    let (stayed, new): (Vec<&FieldView>, Vec<&FieldView>) = after.fields.iter().partition(|f| old.contains_key(&f.entity));
    assert_eq!((stayed.len(), new.len()), (20, 5));
    for field in &stayed {
        // Same ranges in every table and the same words the GPU reads for it.
        assert_eq!(*field, old[&field.entity], "a field that stayed moved or changed");
    }
    // The new fields take the removed fields' places: the tables keep their length.
    assert_eq!(after.lens, before.lens);
    let expected = [
        merged(new.iter().map(|f| f.entries.clone()).collect()),
        merged(new.iter().map(|f| f.chunks.clone()).collect()),
        merged(new.iter().filter_map(|f| f.mask.clone()).collect()),
        merged(new.iter().filter_map(|f| f.height.clone()).collect()),
    ];
    for (table, (written, ranges)) in ["entries", "chunks", "masks", "heights"].iter().zip(after.writes.iter().zip(expected)) {
        assert_eq!(*written, WrittenView { full: false, ranges }, "{table}: writes");
    }
}

/// Every byte of each buffer: entries, chunks, scene (masks then heightmaps).
fn buffer_bytes(view: &TablesView) -> [u64; 3] {
    let [entries, chunks, masks, heights] = view.lens.map(u64::from);
    [entries * FIELD_BYTES, chunks * CHUNK_BYTES, (masks + heights) * 4]
}

fn full() -> UploadPlan {
    UploadPlan { fields: BufferPlan::Full, chunks: BufferPlan::Full, scene: BufferPlan::Full }
}

/// `ranges` (table units) as byte ranges at `unit` bytes each, `base` units into the buffer.
fn bytes(ranges: impl IntoIterator<Item = Range<u32>>, unit: u64, base: u64) -> Vec<Range<u64>> {
    ranges.into_iter().map(|r| (base + u64::from(r.start)) * unit..(base + u64::from(r.end)) * unit).collect()
}

/// `ranges`, sorted and merged where they touch.
fn merged_bytes(mut ranges: Vec<Range<u64>>) -> Vec<Range<u64>> {
    ranges.sort_by_key(|r| r.start);
    let mut out: Vec<Range<u64>> = Vec::new();
    for r in ranges {
        match out.last_mut() {
            Some(last) if last.end >= r.start => last.end = last.end.max(r.end),
            _ => out.push(r),
        }
    }
    out
}

/// The partial upload of the vendored `bevy_grass` render world (slice GC-3b4d2, own patch "partial upload"): the
/// first build, a growth and a version the render world never saw upload whole; a one-block move writes exactly the
/// new fields' bytes in each buffer.
#[test]
fn a_one_block_move_uploads_only_the_new_fields_bytes() {
    let (mut app, style, mut ring) = ring_app();
    let empty = uploaded_state(app.world());
    let first = settle(&mut app);
    assert_eq!(upload_plan(app.world(), None), full(), "nothing uploaded yet");
    assert_eq!(upload_plan(app.world(), Some(&empty)), full(), "the first build");
    let built = uploaded_state(app.world());

    move_column(&mut app, &style, &mut ring, -2);
    let moved = settle(&mut app);
    assert_eq!(moved.since, first.version, "one rebuild");
    let old: HashSet<Entity> = first.fields.iter().map(|f| f.entity).collect();
    let new: Vec<&FieldView> = moved.fields.iter().filter(|f| !old.contains(&f.entity)).collect();
    assert_eq!(new.len(), 5);
    // The entry stride, from the words the GPU reads for one field.
    let field = new[0];
    let len = |r: &Range<u32>| u64::from(r.end - r.start);
    let other_words = len(&field.chunks) * 8 + field.mask.as_ref().map_or(0, len) + field.height.as_ref().map_or(0, len);
    let entry_words = (field.words.len() as u64 - other_words) / len(&field.entries);
    assert_eq!((FIELD_BYTES, CHUNK_BYTES), (entry_words * 4, 32), "buffer strides");
    let mask_words = u64::from(moved.lens[2]);
    let expected = UploadPlan {
        fields: BufferPlan::Ranges(merged_bytes(bytes(new.iter().map(|f| f.entries.clone()), FIELD_BYTES, 0))),
        chunks: BufferPlan::Ranges(merged_bytes(bytes(new.iter().map(|f| f.chunks.clone()), CHUNK_BYTES, 0))),
        scene: BufferPlan::Ranges(merged_bytes(
            bytes(new.iter().filter_map(|f| f.mask.clone()), 4, 0)
                .into_iter()
                .chain(bytes(new.iter().filter_map(|f| f.height.clone()), 4, mask_words))
                .collect(),
        )),
    };
    let plan = upload_plan(app.world(), Some(&built));
    assert_eq!(plan, expected, "a move writes the new fields' bytes only");
    let written: u64 = [&plan.fields, &plan.chunks, &plan.scene]
        .iter()
        .map(|p| match p {
            BufferPlan::Ranges(ranges) => ranges.iter().map(|r| r.end - r.start).sum::<u64>(),
            BufferPlan::Full => unreachable!("partial"),
        })
        .sum();
    println!(
        "one-block move uploads {written} bytes, the whole tables {:?} = {} bytes",
        buffer_bytes(&moved),
        buffer_bytes(&moved).iter().sum::<u64>()
    );
    let after_move = uploaded_state(app.world());

    // A second move the render world uploads over the first build's buffers: the move between was never uploaded.
    move_column(&mut app, &style, &mut ring, -1);
    let skipped = settle(&mut app);
    assert_eq!(skipped.since, moved.version, "one rebuild");
    assert_eq!(upload_plan(app.world(), Some(&built)), full(), "a skipped version");
    let after_skip = uploaded_state(app.world());
    assert!(
        matches!(upload_plan(app.world(), Some(&after_move)).fields, BufferPlan::Ranges(_)),
        "the same move over the version it follows is partial"
    );

    // Five more fields than the removed ones left room for: every table grows.
    for z in -2..=2 {
        ring.insert((5, z), spawn(&mut app, &style, (5, z)));
    }
    let grown = settle(&mut app);
    assert_eq!(grown.since, skipped.version, "one rebuild");
    assert!(grown.lens.iter().zip(skipped.lens).all(|(g, s)| *g > s), "every table grew");
    assert_eq!(upload_plan(app.world(), Some(&after_skip)), full(), "a growth");
}
