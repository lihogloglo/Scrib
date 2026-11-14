//! S2: the image -> material reverse index and the touch on a replaced image (exact, headless,
//! no GPU: `Assets` and their events only).

use bevy_app::{Last, TaskPoolPlugin};
use bevy_asset::{AssetApp, AssetEvent, AssetId, AssetPlugin, Assets, Handle};
use bevy_ecs::message::MessageReader;
use bevy_image::Image;
use bevy_pbr::StandardMaterial;

use super::*;
use crate::rebind::images_of;

/// Every `AssetEvent::<StandardMaterial>::Modified` seen, in order.
#[derive(Resource, Default)]
struct Modified(Vec<AssetId<StandardMaterial>>);

fn record(mut events: MessageReader<AssetEvent<StandardMaterial>>, mut seen: ResMut<Modified>) {
    for event in events.read() {
        if let AssetEvent::Modified { id } = *event {
            seen.0.push(id);
        }
    }
}

fn app() -> App {
    let mut app = App::new();
    app.add_plugins((TaskPoolPlugin::default(), AssetPlugin::default()))
        .init_asset::<Image>()
        .init_asset::<StandardMaterial>()
        .add_plugins(ImageRebindPlugin::<StandardMaterial>::default())
        .init_resource::<Modified>()
        .add_systems(Last, record);
    app
}

fn image(app: &mut App) -> Handle<Image> {
    app.world_mut().resource_mut::<Assets<Image>>().add(Image::default())
}

fn material(app: &mut App, m: StandardMaterial) -> Handle<StandardMaterial> {
    app.world_mut().resource_mut::<Assets<StandardMaterial>>().add(m)
}

/// Replaces `image` under its handle, as residency does (`Assets::insert` on a live id).
fn replace(app: &mut App, image: &Handle<Image>) {
    app.world_mut().resource_mut::<Assets<Image>>().insert(image, Image::default()).expect("live id");
}

/// Runs frames until the touches have surfaced as material events, then clears the record.
fn settle(app: &mut App) -> Vec<AssetId<StandardMaterial>> {
    for _ in 0..4 {
        app.update();
    }
    std::mem::take(&mut app.world_mut().resource_mut::<Modified>().0)
}

struct Scene {
    a: Handle<Image>,
    both_slots: Handle<StandardMaterial>,
    other_image: Handle<StandardMaterial>,
    a_and_c: Handle<StandardMaterial>,
    untextured: Handle<StandardMaterial>,
    _keep: Vec<Handle<Image>>,
}

/// Materials: `a` in two slots; `b` only; `c` and `a` (emissive); none.
fn scene(app: &mut App) -> Scene {
    let (a, b, c) = (image(app), image(app), image(app));
    let both_slots = material(
        app,
        StandardMaterial {
            base_color_texture: Some(a.clone()),
            normal_map_texture: Some(a.clone()),
            ..StandardMaterial::default()
        },
    );
    let other_image =
        material(app, StandardMaterial { base_color_texture: Some(b.clone()), ..StandardMaterial::default() });
    let a_and_c = material(
        app,
        StandardMaterial {
            base_color_texture: Some(c.clone()),
            emissive_texture: Some(a.clone()),
            ..StandardMaterial::default()
        },
    );
    let untextured = material(app, StandardMaterial::default());
    let scene = Scene { a, both_slots, other_image, a_and_c, untextured, _keep: vec![b, c] };
    // Adding the assets is no touch.
    assert_eq!(settle(app), Vec::<AssetId<StandardMaterial>>::new());
    scene
}

fn sorted(mut ids: Vec<AssetId<StandardMaterial>>) -> Vec<AssetId<StandardMaterial>> {
    ids.sort();
    ids
}

#[test]
fn replacing_an_image_touches_exactly_its_materials_once_each() {
    let mut app = app();
    let s = scene(&mut app);
    {
        let users = app.world().resource::<ImageUsers<StandardMaterial>>();
        assert_eq!((users.materials(), users.images()), (3, 3), "the untextured material is not indexed");
        assert_eq!(sorted(users.users(s.a.id()).collect()), sorted(vec![s.both_slots.id(), s.a_and_c.id()]));
    }

    replace(&mut app, &s.a);
    let touched = settle(&mut app);
    assert_eq!(sorted(touched), sorted(vec![s.both_slots.id(), s.a_and_c.id()]), "once each, none else");
    assert!(!app.world().resource::<Modified>().0.contains(&s.other_image.id()));
    assert_eq!(
        *app.world().resource::<ImageRebindStats>(),
        ImageRebindStats { images_replaced: 1, materials_touched: 2 }
    );

    // The touch's own material events re-index the materials but touch nothing again.
    assert_eq!(settle(&mut app), Vec::<AssetId<StandardMaterial>>::new());
    assert_eq!(app.world().resource::<ImageUsers<StandardMaterial>>().materials(), 3);
    let _ = (&s.untextured, &s.other_image);
}

#[test]
fn a_material_using_the_image_in_two_slots_is_indexed_and_touched_once() {
    let mut app = app();
    let s = scene(&mut app);
    let both = app.world().resource::<Assets<StandardMaterial>>().get(&s.both_slots).expect("live").clone();
    assert_eq!(images_of(&both), vec![s.a.id()]);

    // Two replacements of the same image in one frame: still one touch.
    replace(&mut app, &s.a);
    replace(&mut app, &s.a);
    let touched = settle(&mut app);
    assert_eq!(touched.iter().filter(|&&id| id == s.both_slots.id()).count(), 1);
    assert_eq!(app.world().resource::<ImageRebindStats>().materials_touched, 2);
}

#[test]
fn removing_a_material_drops_it_from_the_index() {
    let mut app = app();
    let s = scene(&mut app);
    app.world_mut().resource_mut::<Assets<StandardMaterial>>().remove(&s.a_and_c);
    settle(&mut app);
    {
        let users = app.world().resource::<ImageUsers<StandardMaterial>>();
        assert_eq!(users.users(s.a.id()).collect::<Vec<_>>(), vec![s.both_slots.id()]);
        // `c` had only that user.
        assert_eq!((users.materials(), users.images()), (2, 2));
    }
    replace(&mut app, &s.a);
    assert_eq!(settle(&mut app), vec![s.both_slots.id()]);
}

#[test]
fn a_material_edited_to_another_image_moves_in_the_index() {
    let mut app = app();
    let s = scene(&mut app);
    let d = image(&mut app);
    app.world_mut()
        .resource_mut::<Assets<StandardMaterial>>()
        .get_mut(&s.both_slots)
        .expect("live")
        .into_inner()
        .base_color_texture = Some(d.clone());
    // `normal_map_texture` still holds `a`; give it `d` too.
    app.world_mut()
        .resource_mut::<Assets<StandardMaterial>>()
        .get_mut(&s.both_slots)
        .expect("live")
        .into_inner()
        .normal_map_texture = Some(d.clone());
    settle(&mut app);
    replace(&mut app, &s.a);
    assert_eq!(settle(&mut app), vec![s.a_and_c.id()]);
    replace(&mut app, &d);
    assert_eq!(settle(&mut app), vec![s.both_slots.id()]);
}

#[test]
fn rebind_line_format() {
    assert_eq!(
        rebind::rebind_line(ImageRebindStats { images_replaced: 12, materials_touched: 30 }),
        "texture.rebind: 12 images replaced, 30 materials touched"
    );
}
