//! Body-part seam diagnosis (superfargoth `docs/specs/smooth-bodies.md`,
//! "Shading seams"): for one NPC's naked body, where adjacent parts meet at
//! rest, compares the two sides of each seam: position gap, normal angle,
//! UVs, vertex colors and materials. A visible seam with no gap is a
//! shading seam (normals) or a color seam (texture, material, vertex color).
//!
//! `cargo run --release -p mw_actors --example seam_report -- [npc id]`
//! (default `fargoth`).

#![allow(clippy::too_many_lines, clippy::cast_precision_loss)] // a linear diagnostic script, small counts

use std::sync::Arc;

use glam::Vec3;
use mw_actors::MwActors;
use mw_install::MorrowindInstall;
use sf_character::{
    CharacterSource, CharacterState, DecodedPart, PartKind, PartLookup, SeamLink, SkinSurface,
    SmoothSettings, View, blend_seam_normals, find_seams, resolve_recipe,
};
use sf_model::{CookSettings, Cooked, CookedSkinned, ModelImporter, Vertex};
use sf_records::RecordId;
use sf_vfs::Vfs;

struct Part {
    id: String,
    surface: SkinSurface,
    vertices: Vec<Vertex>,
    materials: Vec<String>,
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let npc = args.first().map_or("fargoth", String::as_str);
    let (install, _) = MorrowindInstall::detect(None).expect("Morrowind install");
    let vfs = Arc::new(mw_vfs::mount_install(&install, Vfs::builder()).unwrap().vfs);
    let content = mw_records::load(&vfs, &install.game_files()).unwrap();
    let a = MwActors::new(Arc::new(content.db), Arc::clone(&vfs));
    let recipe = a
        .recipe(
            "NPC_",
            &RecordId::new(npc),
            &CharacterState {
                worn: Some(Vec::new()),
                held: None,
            },
        )
        .unwrap();
    let skeleton =
        mw_anim::rig::skeleton(&mw_nif::load(&vfs.read(&recipe.skeleton).unwrap()).unwrap())
            .unwrap();
    let rest = skeleton.rest_model();
    let joint = |n: &str| skeleton.find(n).and_then(|j| u16::try_from(j).ok());
    let outfit = resolve_recipe(&a, a.slots(), &recipe, View::ThirdPerson).unwrap();
    let mut parts = Vec::new();
    for (_, p) in &outfit.parts {
        let part = a.part(p).unwrap();
        let model = mw_bake::NifImporter
            .import(part.model.as_str(), &vfs.read(&part.model).unwrap())
            .unwrap();
        let cooked = sf_model::cook(&model, &CookSettings::default()).unwrap();
        let skinned = match (&part.kind, &cooked) {
            (PartKind::Rigid { joint, offset }, c) => {
                CookedSkinned::rigid(c, joint, *offset).unwrap()
            }
            (PartKind::Skinned, Cooked::Skinned(s)) => Some(s.clone()),
            _ => None,
        };
        let Some(s) = skinned else { continue };
        let surface = SkinSurface::from_cooked(&s, 0, Some(part), joint, &rest).unwrap();
        let decoded = DecodedPart::decode(&s, 0, Some(part)).unwrap();
        let vertices: Vec<Vertex> = decoded
            .submeshes
            .iter()
            .flat_map(|d| d.vertices.iter().copied())
            .collect();
        let materials: Vec<String> = s
            .model
            .materials
            .iter()
            .map(|m| {
                format!(
                    "{} tex={:?} color={:?} extras={:?}",
                    m.name, m.base_color_texture, m.base_color, m.extras
                )
            })
            .collect();
        println!(
            "{} [{}] {} v, kind {}, has_colors {}",
            p.as_str(),
            part.model.as_str(),
            vertices.len(),
            if matches!(part.kind, PartKind::Rigid { .. }) {
                "rigid"
            } else {
                "skinned"
            },
            s.model.has_colors
        );
        for m in &materials {
            println!("    {m}");
        }
        parts.push(Part {
            id: p.as_str().to_owned(),
            surface,
            vertices,
            materials,
        });
    }
    println!();
    for (i, pa) in parts.iter().enumerate() {
        for pb in &parts[i + 1..] {
            report(pa, pb);
        }
    }
    // Seam shading: rim normal vs the normal of the corner it links to, per
    // part pair, before and after `blend_seam_normals` (hair left out, as live).
    let hair = recipe.hair.as_ref().map(RecordId::as_str);
    let keep: Vec<&Part> = parts
        .iter()
        .filter(|p| Some(p.id.as_str()) != hair)
        .collect();
    let mut surfaces: Vec<SkinSurface> = keep.iter().map(|p| p.surface.clone()).collect();
    let parents: Vec<Option<usize>> = (0..skeleton.len()).map(|j| skeleton.parent(j)).collect();
    let settings = SmoothSettings::default();
    let seams = find_seams(&surfaces, &parents, &settings);
    let before = rim_angles(&surfaces, &seams);
    let changed = blend_seam_normals(&mut surfaces, &parents, &settings);
    let after = rim_angles(&surfaces, &seams);
    for (p, s) in keep.iter().zip(&surfaces) {
        let n = p
            .surface
            .normals
            .iter()
            .zip(&s.normals)
            .filter(|(a, b)| a != b)
            .count();
        let turn = p
            .surface
            .normals
            .iter()
            .zip(&s.normals)
            .map(|(a, b)| a.dot(*b).clamp(-1.0, 1.0).acos().to_degrees())
            .fold(0.0, f32::max);
        // Open edges (positions merged exactly): a closed piece has no rim.
        let key = |v: u32| p.surface.positions[v as usize].to_array().map(f32::to_bits);
        let mut edges: std::collections::HashMap<([u32; 3], [u32; 3]), usize> =
            std::collections::HashMap::new();
        for t in &p.surface.triangles {
            for k in 0..3 {
                let (a, b) = (key(t[k]), key(t[(k + 1) % 3]));
                *edges
                    .entry(if a < b { (a, b) } else { (b, a) })
                    .or_default() += 1;
            }
        }
        let open: Vec<f32> = edges
            .iter()
            .filter(|(_, c)| **c == 1)
            .map(|((a, _), _)| f32::from_bits(a[2]))
            .collect();
        let (lo, hi) = open
            .iter()
            .fold((f32::MAX, f32::MIN), |(l, h), y| (l.min(*y), h.max(*y)));
        println!(
            "  {}: {n} normals changed, largest turn {turn:.0} deg; {} open edges, z {lo:.3}..{hi:.3}",
            p.id,
            open.len()
        );
    }
    println!(
        "\nseam shading: {} links, {changed} rim normals changed",
        seams.len()
    );
    for ((pair, b), a) in before.iter().zip(after.values()) {
        let (mean_b, max_b) = (
            b.iter().sum::<f32>() / b.len() as f32,
            b.iter().copied().fold(0.0, f32::max),
        );
        let (mean_a, max_a) = (
            a.iter().sum::<f32>() / a.len() as f32,
            a.iter().copied().fold(0.0, f32::max),
        );
        println!(
            "  {} -> {}: {} rims, angle mean {mean_b:.1} max {max_b:.1} -> mean {mean_a:.1} max {max_a:.1} deg",
            keep[pair.0].id,
            keep[pair.1].id,
            b.len()
        );
    }
}

/// Per (rim part, other part): angle between each rim normal and its linked corner's normal.
fn rim_angles(
    parts: &[SkinSurface],
    seams: &[SeamLink],
) -> std::collections::BTreeMap<(usize, usize), Vec<f32>> {
    let mut out: std::collections::BTreeMap<(usize, usize), Vec<f32>> =
        std::collections::BTreeMap::new();
    for l in seams {
        let (a, b) = (
            parts[l.a.0 as usize].normals[l.a.1 as usize],
            parts[l.b.0 as usize].normals[l.b.1 as usize],
        );
        out.entry((l.a.0 as usize, l.b.0 as usize))
            .or_default()
            .push(a.dot(b).clamp(-1.0, 1.0).acos().to_degrees());
    }
    out
}

fn report(pa: &Part, pb: &Part) {
    // Every vertex of A with a vertex of B within 3 mm: the seam's two sides.
    let mut pairs = Vec::new();
    for (va, p) in pa.surface.positions.iter().enumerate() {
        let (vb, d) = pb
            .surface
            .positions
            .iter()
            .enumerate()
            .map(|(k, q)| (k, p.distance(*q)))
            .fold((0, f32::MAX), |b, x| if x.1 < b.1 { x } else { b });
        if d < 0.003 {
            pairs.push((va, vb, d));
        }
    }
    let near_1cm = pa
        .surface
        .positions
        .iter()
        .filter(|p| pb.surface.positions.iter().any(|q| p.distance(*q) < 0.01))
        .count();
    if pairs.is_empty() && near_1cm == 0 {
        return;
    }
    let angle = |a: Vec3, b: Vec3| a.dot(b).clamp(-1.0, 1.0).acos().to_degrees();
    let angles: Vec<f32> = pairs
        .iter()
        .map(|&(a, b, _)| angle(pa.surface.normals[a], pb.surface.normals[b]))
        .collect();
    let mean = |v: &[f32]| {
        if v.is_empty() {
            0.0
        } else {
            v.iter().sum::<f32>() / v.len() as f32
        }
    };
    let max = |v: &[f32]| v.iter().copied().fold(0.0, f32::max);
    let gaps: Vec<f32> = pairs.iter().map(|p| p.2 * 1e3).collect();
    let duv: Vec<f32> = pairs
        .iter()
        .map(|&(a, b, _)| {
            (glam::Vec2::from(pa.vertices[a].uv) - glam::Vec2::from(pb.vertices[b].uv)).length()
        })
        .collect();
    let dcol: Vec<f32> = pairs
        .iter()
        .map(|&(a, b, _)| {
            pa.vertices[a]
                .color
                .iter()
                .zip(pb.vertices[b].color)
                .map(|(x, y)| f32::from(x.abs_diff(y)))
                .fold(0.0, f32::max)
        })
        .collect();
    println!(
        "{} | {}: {} co-located (<3 mm, gap mean {:.2} max {:.2} mm), {} within 1 cm; normal angle mean {:.1} max {:.1} deg; uv delta mean {:.3} max {:.3}; color delta max {:.0}; same material {}",
        pa.id,
        pb.id,
        pairs.len(),
        mean(&gaps),
        max(&gaps),
        near_1cm,
        mean(&angles),
        max(&angles),
        mean(&duv),
        max(&duv),
        max(&dcol),
        pa.materials == pb.materials
    );
    for &(a, b, _) in pairs.iter().take(3) {
        println!(
            "    e.g. A uv {:?} col {:?} n {:.2} | B uv {:?} col {:?} n {:.2}",
            pa.vertices[a].uv,
            pa.vertices[a].color,
            pa.surface.normals[a],
            pb.vertices[b].uv,
            pb.vertices[b].color,
            pb.surface.normals[b]
        );
    }
}
