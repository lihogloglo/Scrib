//! Portal-frame feasibility over every door link of the install (M8 prep):
//! door shapes, openings, pair congruence, classes, and arrival facing.
//!
//! `cargo run --release -p mw_world --example portal_report [-- <Morrowind folder>]`
//! Spec: `docs/specs/world-adapter.md` ("Portal frames: feasibility").
#![allow(clippy::cast_precision_loss, clippy::too_many_lines)]

use std::collections::BTreeMap;

use glam::Vec3;
use mw_install::MorrowindInstall;
use mw_world::portal::{self, DoorPortal, Facing, PortalClass, PortalReport};
use mw_world::{WorldRecords, ids, worldspaces};
use sf_coords::WorldTransform;
use sf_vfs::Vfs;
use sf_world::{DoorLink, LinkEnd};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let explicit = std::env::args().nth(1).map(std::path::PathBuf::from);
    let (install, _) = MorrowindInstall::detect(explicit.as_deref())?;
    let vfs = mw_vfs::mount_install(&install, Vfs::builder())?.vfs;
    let content = mw_records::load(&vfs, &install.game_files())?;
    let db = &content.db;
    let records = WorldRecords::build(db);
    let (ends, _) = worldspaces::door_ends(db);
    let models = portal::door_models(db);
    let bounds = portal::load_bounds(&vfs, &models);
    let lookup = |m: &str| bounds.get(m).copied();
    let report = portal::analyze(&ends, &records.door_links, &models, &lookup);
    let r = &report;

    println!("door references with a destination: {}", ends.len());
    println!(
        "door models: {} distinct, {} with visible bounds",
        models.values().filter_map(|m| m.model.as_ref()).collect::<std::collections::BTreeSet<_>>().len(),
        bounds.len()
    );
    println!(
        "links: {} two-way, {} one-way; ambiguous doors (2+ candidates within {} m): {}",
        records.doors.paired,
        records.doors.one_way,
        worldspaces::PAIR_DISTANCE_M,
        r.ambiguous.len()
    );
    for &i in r.ambiguous.iter().take(8) {
        println!("  ambiguous: {} ({})", r.doors[i].cell, r.doors[i].base);
    }

    // Shapes.
    let mut shapes: BTreeMap<_, usize> = BTreeMap::new();
    for d in &r.doors {
        *shapes.entry(d.geometry.shape).or_default() += 1;
    }
    println!("\ndoor shapes: {shapes:?}");
    let edge =
        r.doors.iter().filter(|d| d.geometry.shape == portal::DoorShape::Slab && d.geometry.hinge.abs() > 0.35).count();
    let slabs = shapes.get(&portal::DoorShape::Slab).copied().unwrap_or(0);
    println!("slabs whose origin sits at an edge (hinge, |h| > 0.35): {edge} of {slabs}");

    // Per model: uses, shape, extent, thickness, hinge, front convention.
    let mut per_model: BTreeMap<&str, Vec<&DoorPortal>> = BTreeMap::new();
    for d in &r.doors {
        per_model.entry(d.model.as_deref().unwrap_or("<none>")).or_default().push(d);
    }
    let mut rows: Vec<_> = per_model.iter().collect();
    rows.sort_by_key(|(_, v)| std::cmp::Reverse(v.len()));
    println!("\nmost used door models (uses, shape, w x h x t m, hinge, front +thin/-thin):");
    for (m, v) in rows.iter().take(25) {
        let g = &v[0].geometry;
        let flipped = v.iter().filter(|d| d.geometry.front_known && d.geometry.front_flipped).count();
        let known = v.iter().filter(|d| d.geometry.front_known).count();
        println!(
            "  {:4} {:10} {:.2} x {:.2} x {:.2}  h {:+.2}  front {}/{}  {m}",
            v.len(),
            format!("{:?}", g.shape),
            g.extent.x,
            g.extent.y,
            g.thickness,
            g.hinge,
            known - flipped,
            flipped
        );
    }
    let mixed = per_model
        .values()
        .filter(|v| {
            let f = v.iter().filter(|d| d.geometry.front_known && d.geometry.front_flipped).count();
            let k = v.iter().filter(|d| d.geometry.front_known).count();
            f > 0 && f < k
        })
        .count();
    let (flip_total, known_total) = (
        r.doors.iter().filter(|d| d.geometry.front_known && d.geometry.front_flipped).count(),
        r.doors.iter().filter(|d| d.geometry.front_known).count(),
    );
    println!(
        "front side vs model thin axis: {flip_total} of {known_total} doors open toward -thin; models used both ways: {mixed}"
    );
    for shape in [portal::DoorShape::Thick, portal::DoorShape::Horizontal, portal::DoorShape::Missing] {
        let names: Vec<_> = rows
            .iter()
            .filter(|(_, v)| v[0].geometry.shape == shape)
            .take(8)
            .map(|(m, v)| format!("{m} ({})", v.len()))
            .collect();
        println!("  {shape:?} models: {names:?}");
    }

    // Classes.
    println!(
        "\nlink classes: portal-ready {}, portal-with-adjustment {}, fade-only {} (of {} links)",
        r.count(PortalClass::Ready),
        r.count(PortalClass::Adjust),
        r.count(PortalClass::FadeOnly),
        r.links.len()
    );
    let mut reasons: BTreeMap<_, Vec<String>> = BTreeMap::new();
    for l in &r.links {
        let a = &r.doors[l.a];
        let name = match l.b {
            Some(b) => format!("{} <-> {} (x{:.2})", a.cell, r.doors[b].cell, l.size_ratio),
            None => format!("{} ({})", a.cell, a.base),
        };
        reasons.entry((l.class, l.reason)).or_default().push(name);
    }
    for ((class, reason), names) in &reasons {
        println!("  {class:?}/{reason:?}: {}", names.len());
        for n in names.iter().take(6) {
            println!("      {n}");
        }
    }
    let mut ratio_hist = [0usize; 6];
    for l in r.links.iter().filter(|l| l.b.is_some()) {
        let bucket = [1.05, 1.15, 1.3, 1.6, 2.0].iter().position(|&t| l.size_ratio <= t).unwrap_or(5);
        ratio_hist[bucket] += 1;
    }
    println!("size ratio of two-way pairs (<=1.05, 1.15, 1.3, 1.6, 2.0, more): {ratio_hist:?}");
    let same_model = r
        .links
        .iter()
        .filter(|l| l.b.is_some_and(|b| l.size_ratio > 1.05 && r.doors[b].model == r.doors[l.a].model))
        .count();
    println!("pairs off by > 5% that use the same model on both sides (reference scale differs): {same_model}");

    // Arrivals.
    arrivals(r);

    // Census Office.
    let census = ids::interior("Seyda Neen, Census and Excise Office");
    for (_, link) in
        records.door_links.iter().filter(|(_, l)| l.two_way && (l.a.worldspace == census || l.b.worldspace == census))
    {
        detail(r, &ends, link);
    }
    Ok(())
}

fn arrivals(r: &PortalReport) {
    let mut facing: BTreeMap<Facing, usize> = BTreeMap::new();
    let (mut tilted, mut far, mut behind, mut horizontal) = (0, 0, 0, 0);
    let mut suspicious = Vec::new();
    let mut ahead_hist = [0usize; 6];
    let mut sill_hist = [0usize; 7];
    for l in &r.links {
        let Some(b) = l.b else { continue };
        for (door, check) in [(l.a, l.arrival_a), (b, l.arrival_b)] {
            let Some(c) = check else { continue };
            tilted += usize::from(c.tilted);
            if r.doors[door].geometry.shape == portal::DoorShape::Horizontal {
                horizontal += 1;
                continue;
            }
            *facing.entry(c.facing_class()).or_default() += 1;
            far += usize::from(c.distance > 4.0);
            behind += usize::from(c.ahead < 0.0);
            ahead_hist[[0.25, 0.5, 1.0, 2.0, 4.0].iter().position(|&t| c.ahead <= t).unwrap_or(5)] += 1;
            sill_hist[[-0.25, 0.25, 0.75, 1.25, 1.75, 2.5].iter().position(|&t| c.above_sill <= t).unwrap_or(6)] += 1;
            if c.facing_class() != Facing::Away {
                suspicious.push((c.facing, &r.doors[door], c));
            }
        }
    }
    println!(
        "\narrivals at two-way doors (forward = rotation * engine -Z, i.e. Morrowind +Y; horizontal doors excluded: {horizontal}):"
    );
    println!("  facing vs the door's front normal: {facing:?}");
    println!(
        "  ahead of the door plane (<=0.25, 0.5, 1, 2, 4, more m): {ahead_hist:?}; behind: {behind}; > 4 m away: {far}"
    );
    println!(
        "  marker height above the opening's bottom (<=-0.25, 0.25, 0.75, 1.25, 1.75, 2.5, more m): {sill_hist:?}"
    );
    println!("  carrying pitch/roll (all doors): {tilted}");
    suspicious.sort_by(|a, b| a.0.total_cmp(&b.0));
    println!("  suspicious markers at upright doors (not facing away from the door), most turned first:");
    for (cos, d, c) in &suspicious {
        println!(
            "    {:>4.0} deg  {} [{}] {:?}  ahead {:.2} lateral {:+.2} m",
            cos.clamp(-1.0, 1.0).acos().to_degrees(),
            d.cell,
            d.base,
            d.geometry.shape,
            c.ahead,
            c.lateral
        );
    }
}

fn pose_str(t: &WorldTransform) -> String {
    let p = t.pos.to_f64();
    let f = t.rotation * Vec3::Z;
    format!(
        "({:.3}, {:.3}, {:.3}) +Z yaw {:.1} deg (+Z = ({:.3}, {:.3}, {:.3}))",
        p.x,
        p.y,
        p.z,
        f.x.atan2(f.z).to_degrees(),
        f.x,
        f.y,
        f.z
    )
}

#[allow(clippy::many_single_char_names)]
fn detail(r: &PortalReport, ends: &[worldspaces::DoorEnd], link: &DoorLink) {
    let find = |end: &LinkEnd| r.doors.iter().position(|d| Some(d.id) == end.door);
    let (Some(a), Some(b)) = (find(&link.a), find(&link.b)) else { return };
    println!("\nCensus Office pair:");
    for i in [a, b] {
        let d = &r.doors[i];
        let g = &d.geometry;
        let e = ends.iter().find(|e| e.id == d.id).expect("door end");
        println!("  {} [{}] model {:?} scale-applied", d.cell, d.base, d.model);
        println!("    reference pose {}", pose_str(&e.pose));
        println!("    portal frame   {}", pose_str(&g.frame));
        println!(
            "    opening {:.3} x {:.3} m, thickness {:.3} m, hinge {:+.2}, shape {:?}",
            g.extent.x, g.extent.y, g.thickness, g.hinge, g.shape
        );
    }
    let l = r.links.iter().find(|l| l.a == a || l.a == b).expect("link");
    println!("  class {:?} ({:?}), size ratio {:.3}", l.class, l.reason, l.size_ratio);
    println!("  arrival at a: {:?}\n  arrival at b: {:?}", l.arrival_a, l.arrival_b);
    // The crossing transform a -> b: T = b.frame · R_y(π) · a.frame⁻¹.
    let end = |i: usize| LinkEnd {
        worldspace: link.a.worldspace.clone(),
        frame: r.doors[i].geometry.frame,
        arrival: r.doors[i].geometry.frame,
        door: None,
        extent: r.doors[i].geometry.extent,
    };
    let (ea, eb) = (end(a), end(b));
    let yaw = |q: glam::Quat| {
        let z = q * Vec3::Z;
        z.x.atan2(z.z).to_degrees()
    };
    let rot = eb.frame.rotation * glam::Quat::from_rotation_y(std::f32::consts::PI) * ea.frame.rotation.inverse();
    println!("  crossing a -> b: rotation yaw {:+.1} deg; a's frame origin maps to b's frame origin", yaw(rot));
    // A traveler 0.5 m through a (behind its plane, facing in) lands 0.5 m
    // in front of b, facing out; compare with the authored arrival at b.
    let through = WorldTransform {
        pos: ea.frame.pos.offset(ea.frame.rotation * Vec3::new(0.0, 0.0, -0.5)),
        rotation: ea.frame.rotation,
        scale: Vec3::ONE,
    };
    let landed = DoorLink::cross(&ea, &eb, &through);
    let authored = ends.iter().find(|e| e.id == r.doors[a].id).expect("a").arrival;
    println!("  stepping 0.5 m through a lands at {}", pose_str(&landed));
    println!(
        "  authored arrival at b {} ; offset from portal landing {:.2} m, facing differs {:.1} deg",
        pose_str(&authored),
        landed.pos.delta(authored.pos).length(),
        portal::arrival_forward(&landed).angle_between(portal::arrival_forward(&authored)).to_degrees()
    );
}
