//! `sf_texture::rewrap` over every DDS/KTX2 under `textures/` in the installed game (textures.md T2):
//! prints the format mix, how many ship a full mip chain, the rewrap outcomes and the rewrap time.
//! Every texture is rewrapped as `Role::Color` (the mix is what's measured; T3 assigns real roles).
//! `MORROWIND_DIR` or the detected install.
//!
//! `cargo test --release -p mw_bake --test rewrap_real_install -- --ignored --nocapture`
#![allow(clippy::cast_precision_loss)]

use std::collections::BTreeMap;
use std::time::{Duration, Instant};

use mw_install::MorrowindInstall;
use sf_texture::{AlphaUse, Rewrap, Role, read_source, rewrap};
use sf_vfs::Vfs;

#[test]
#[ignore = "measurement: needs a Morrowind install; rewraps every texture"]
fn rewrap_format_mix_over_the_install() {
    let explicit = std::env::var_os("MORROWIND_DIR").map(std::path::PathBuf::from);
    let (install, _) = MorrowindInstall::detect(explicit.as_deref()).expect("Morrowind install");
    let vfs = mw_vfs::mount_install(&install, Vfs::builder())
        .expect("vfs")
        .vfs;
    let role = Role::Color {
        alpha: AlphaUse::None,
    };
    let mut formats: BTreeMap<String, (usize, usize)> = BTreeMap::new(); // (files, full chain)
    let mut outcomes: BTreeMap<String, usize> = BTreeMap::new();
    let (mut files, mut bytes_in, mut bytes_out, mut filled_levels) =
        (0usize, 0usize, 0usize, 0u32);
    let mut spent = Duration::ZERO;
    let paths: Vec<_> = vfs
        .paths()
        .filter(|path| {
            let p = path.as_str();
            let ext = std::path::Path::new(p).extension().unwrap_or_default();
            p.starts_with("textures/")
                && (ext.eq_ignore_ascii_case("dds") || ext.eq_ignore_ascii_case("ktx2"))
        })
        .cloned()
        .collect();
    for path in &paths {
        let Ok(bytes) = vfs.read(path) else { continue };
        files += 1;
        bytes_in += bytes.len();
        let key = match read_source(&bytes) {
            Ok(Some(source)) => {
                let full = source.levels.len()
                    == sf_texture::ktx2_out::mip_count(source.width, source.height) as usize;
                (format!("{:?}", source.format), full)
            }
            Ok(None) => ("not BCn".to_owned(), false),
            Err(reason) => (format!("skip {reason:?}"), false),
        };
        let entry = formats.entry(key.0).or_default();
        entry.0 += 1;
        entry.1 += usize::from(key.1);
        let started = Instant::now();
        let outcome = rewrap(&bytes, role);
        spent += started.elapsed();
        let label = match &outcome {
            Rewrap::Rewrapped(out) if out.shipped == out.levels => {
                "rewrapped, full chain".to_owned()
            }
            Rewrap::Rewrapped(out) => {
                filled_levels += out.levels - out.shipped;
                "rewrapped, levels filled".to_owned()
            }
            Rewrap::Cook(reason) => format!("cook {reason:?}"),
            Rewrap::Skip(reason) => format!("skip {reason:?}"),
        };
        if let Rewrap::Rewrapped(out) = &outcome {
            bytes_out += out.ktx2.len();
        }
        *outcomes.entry(label).or_default() += 1;
    }
    println!(
        "rewrap over {files} textures ({:.1} MB in, {:.1} MB KTX2 out)",
        bytes_in as f64 / 1e6,
        bytes_out as f64 / 1e6
    );
    println!("format mix (files, of which full chain):");
    for (format, (count, full)) in &formats {
        println!("  {format:<40} {count:>6} {full:>6}");
    }
    println!("outcomes:");
    for (label, count) in &outcomes {
        println!("  {label:<40} {count:>6}");
    }
    println!(
        "rewrap total {:.0} ms ({:.3} ms per file), {filled_levels} levels filled",
        spent.as_secs_f64() * 1e3,
        spent.as_secs_f64() * 1e3 / files.max(1) as f64
    );
    assert!(files > 0, "no textures found");
}
