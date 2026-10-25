//! Particle emitters in the real game data (nif-runtime.md "Particles"):
//! which controllers are `NiBSPArrayController`s and how they emit (flag
//! 0x8 at nodes, 0x10 at vertices), emission windows against the controller
//! range, and emitters under keyframe-animated nodes. Skips (passes) without
//! a Morrowind install. `-- --nocapture` prints one line per controller, and
//! per creature model the clips each system emits in (`emit:` curves).

use std::collections::HashMap;

use mw_install::MorrowindInstall;
use sf_vfs::Vfs;
use tes3::nif::{
    NiAVObject, NiGeometry, NiGeometryData, NiKey, NiKeyframeController, NiNode, NiParticleSystemController, NiStream, NiTriBasedGeom,
    NiType,
};

/// `NiBSPArrayController` flags, as `OpenMW` reads them (at node, at vertex).
const AT_NODE: u16 = 0x8;
const AT_VERTEX: u16 = 0x10;

#[derive(Default)]
struct Census {
    systems: usize,
    bsp_array: usize,
    at_vertex: usize,
    at_node: usize,
    windowed: usize,
    animated_emitters: usize,
}

fn get<'a, T>(stream: &'a NiStream, key: NiKey) -> Option<&'a T>
where
    &'a T: TryFrom<&'a NiType>,
{
    stream.objects.get(key).and_then(|o| <&T>::try_from(o).ok())
}

fn name(stream: &NiStream, key: NiKey) -> String {
    get::<NiAVObject>(stream, key).map_or_else(|| "?".into(), |o| o.base.name.clone())
}

fn survey(path: &str, stream: &NiStream, census: &mut Census) {
    let mut parent = HashMap::new();
    for (key, object) in &stream.objects {
        if let Ok(node) = <&NiNode>::try_from(object) {
            for child in &node.children {
                parent.insert(child.key, key);
            }
        }
    }
    let animated = |mut key: NiKey| {
        loop {
            if get::<NiAVObject>(stream, key).is_some_and(|o| o.base.controllers_of_type::<NiKeyframeController>(stream).next().is_some()) {
                return true;
            }
            let Some(&up) = parent.get(&key) else {
                return false;
            };
            key = up;
        }
    };
    for object in stream.objects.values() {
        let Ok(controller) = <&NiParticleSystemController>::try_from(object) else {
            continue;
        };
        census.systems += 1;
        let kind = std::str::from_utf8(object.type_name()).unwrap_or("?");
        let bsp = kind == "NiBSPArrayController";
        let flags = controller.base.flags;
        if bsp {
            census.bsp_array += 1;
            census.at_vertex += usize::from(flags & AT_VERTEX != 0);
            census.at_node += usize::from(flags & AT_NODE != 0);
        }
        let emitter = controller.emitter.key;
        let (nodes, vertices) = get::<NiNode>(stream, emitter).map_or((0, 0), |node| {
            let nodes = node.children_of_type_recursive::<NiNode>(stream).count();
            let vertices = node
                .children_of_type_recursive::<NiTriBasedGeom>(stream)
                .next()
                .and_then(|g| stream.get_as::<_, NiGeometryData>(g.base.geometry_data))
                .map_or(0, |d| d.vertices.len());
            (nodes, vertices)
        });
        let geometry_vertices = get::<NiGeometry>(stream, emitter)
            .and_then(|g| stream.get_as::<_, NiGeometryData>(g.geometry_data))
            .map_or(0, |d| d.vertices.len());
        let windowed = controller.emit_stop_time > controller.emit_start_time;
        census.windowed += usize::from(windowed);
        let moving = !controller.emitter.is_null() && animated(emitter);
        census.animated_emitters += usize::from(moving);
        let t = &controller.base;
        eprintln!(
            "{path}: {kind} flags={flags:#x} emitter={:?} nodes={nodes} verts={} emit=[{}, {}] range=[{}, {}] f={} rate={} use_rate={} life={} animated={moving}",
            name(stream, emitter),
            vertices.max(geometry_vertices),
            controller.emit_start_time,
            controller.emit_stop_time,
            t.start_time,
            t.stop_time,
            t.frequency,
            controller.birth_rate,
            controller.use_birth_rate,
            controller.lifespan,
        );
    }
}

#[test]
fn particle_emitters_census() {
    let Ok((install, _)) = MorrowindInstall::detect(None) else {
        eprintln!("no Morrowind install detected; skipping");
        return;
    };
    let vfs = mw_vfs::mount_install(&install, Vfs::builder()).unwrap().vfs;
    let mut census = Census::default();
    let mut paths: Vec<_> = vfs.paths().filter(|p| p.extension() == Some("nif")).collect();
    paths.sort();
    for path in paths {
        let Ok(bytes) = vfs.read(path) else { continue };
        let Ok(stream) = mw_nif::parse(&bytes) else {
            continue;
        };
        survey(path.as_str(), &stream, &mut census);
    }
    eprintln!(
        "systems {}, NiBSPArrayController {} (at vertex {}, at node {}), emission windows {}, animated emitters {}",
        census.systems, census.bsp_array, census.at_vertex, census.at_node, census.windowed, census.animated_emitters
    );
    assert!(census.systems > 100, "install read");
}

/// Seconds of `clip` during which step curve `c` is at least 0.5.
fn on_time(c: &sf_anim::SourceCurve, duration: f32) -> f32 {
    let times = &c.curve.times;
    (0..times.len())
        .filter(|&i| c.curve.values[i] >= 0.5)
        .map(|i| times.get(i + 1).copied().unwrap_or(duration).min(duration) - times[i])
        .sum()
}

/// Creature emission windows on the clip timelines (`emit:` curves,
/// `mw_anim::effects`), imported as the bake imports them: `x<model>.kf` with
/// its `.nif` companion, else the model's own controllers. Prints, per clip,
/// the systems that emit in it and for how long.
#[test]
fn actor_emission_on_clip_timelines() {
    let Ok((install, _)) = MorrowindInstall::detect(None) else {
        eprintln!("no Morrowind install detected; skipping");
        return;
    };
    let vfs = mw_vfs::mount_install(&install, Vfs::builder()).unwrap().vfs;
    let read = |p: &str| sf_vfs::VfsPath::new(p).ok().and_then(|p| vfs.read(&p).ok());
    let mut by_model: HashMap<String, Vec<(String, String, f32, f32)>> = HashMap::new();
    let mut paths: Vec<String> = vfs
        .paths()
        .map(|p| p.as_str().to_owned())
        .filter(|p| p.starts_with("meshes/r/") && p.to_ascii_lowercase().ends_with(".nif"))
        .collect();
    paths.sort();
    for path in &paths {
        let Some(bytes) = read(path) else { continue };
        let Ok(model) = mw_nif::load(&bytes) else { continue };
        let kf = format!("{}.kf", path.trim_end_matches(".nif"));
        if model.particles.is_empty() || (model.text_keys.is_empty() && read(&kf).is_none()) {
            continue;
        }
        let source = match read(&kf) {
            Some(kf_bytes) => sf_anim::AnimImporter::import_with(&mw_anim::KfAnimImporter, &kf, &kf_bytes, Some(&bytes)),
            None => sf_anim::AnimImporter::import(&mw_anim::NifAnimImporter, path, &bytes),
        };
        let Ok(source) = source else { continue };
        let rows = by_model.entry(path.clone()).or_default();
        for name in mw_anim::effects::emission_names(&model) {
            for clip in &source.clips {
                let Some(c) = clip.curves.iter().find(|c| c.name == name) else { continue };
                let on = on_time(c, clip.duration);
                if on > 0.0 {
                    rows.push((name.clone(), clip.name.clone(), on, clip.duration));
                }
            }
            if !rows.iter().any(|r| r.0 == name) {
                rows.push((name.clone(), "(no clip)".into(), 0.0, 0.0));
            }
        }
    }
    let mut models: Vec<_> = by_model.iter().collect();
    models.sort_by_key(|m| m.0);
    for (path, rows) in &models {
        eprintln!("{path}:");
        for (name, clip, on, duration) in *rows {
            eprintln!("  {name} in {clip}: {on:.2} of {duration:.2} s");
        }
    }
    assert!(models.len() > 10, "creature models with particles and animation groups");
    let rows = |model: &str| {
        by_model.get(&format!("meshes/r/x{model}.nif")).or(by_model.get(&format!("meshes/r/{model}.nif"))).cloned().unwrap_or_default()
    };
    // Every system emits in some clip (a window between groups would never show).
    let silent: Vec<_> = by_model.iter().flat_map(|(p, r)| r.iter().filter(|r| r.2 <= 0.0).map(move |r| format!("{p} {}", r.0))).collect();
    eprintln!("systems emitting in no clip: {silent:?}");
    // The flame atronach's body burns through its idles; the dremora's dust
    // only in its deaths; Dagoth Ur's hands in some attack or cast.
    let atronach = rows("atronach_fire");
    assert!(atronach.iter().any(|r| r.1.starts_with("Idle") && r.2 >= r.3 - 1e-3), "{atronach:?}");
    let dremora = rows("dremora");
    assert!(!dremora.is_empty() && dremora.iter().all(|r| r.1.to_ascii_lowercase().starts_with("death")), "{dremora:?}");
    assert!(rows("dagothr").iter().any(|r| r.2 > 0.0), "dagoth ur emits");
}
