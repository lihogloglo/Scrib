//! Measurement: decode every `.bik` of a Morrowind install (video.md "Build plan" V1 gate: all files
//! decode, at most ~4 ms/frame at 640x480 on the owner's laptop, release build).
//!
//! `cargo test -p sf_video --release --test bik_corpus -- --ignored --nocapture`
//! Directory override: env `SF_VIDEO_BIK_DIR`.

use std::fs::File;
use std::io::BufReader;
use std::panic::{AssertUnwindSafe, catch_unwind};
use std::path::PathBuf;
use std::time::Instant;

use sf_video::{Frame, VideoDecoders};

const DEFAULT_DIR: &str =
    "C:/Program Files (x86)/Steam/steamapps/common/Morrowind/Data Files/Video/";

struct Row {
    frames: u32,
    header_frames: u32,
    size: (u32, u32),
    audio: String,
    samples: usize,
    ms: f64,
}

fn decode(path: &std::path::Path, decoders: &VideoDecoders) -> Result<Row, String> {
    let file = File::open(path).map_err(|e| e.to_string())?;
    let mut source = decoders
        .probe_and_open(Box::new(BufReader::new(file)))
        .map_err(|e| e.to_string())?;
    let info = source.info();
    let mut frame = Frame::default();
    let mut pcm = Vec::new();
    let mut frames = 0u32;
    let start = Instant::now();
    loop {
        match source.next_frame(&mut frame) {
            Ok(Some(_)) => frames += 1,
            Ok(None) => break,
            Err(e) => return Err(format!("frame {frames}: {e}")),
        }
        source.next_audio(&mut pcm).map_err(|e| e.to_string())?;
    }
    let ms = start.elapsed().as_secs_f64() * 1000.0;
    Ok(Row {
        frames,
        header_frames: info.frames,
        size: (info.width, info.height),
        audio: info
            .audio
            .map_or_else(|| "none".into(), |a| format!("{} Hz x{}", a.sample_rate, a.channels)),
        samples: pcm.len(),
        ms,
    })
}

#[test]
#[ignore = "measurement: decode every Morrowind .bik"]
fn decode_every_morrowind_bik() {
    let dir = std::env::var("SF_VIDEO_BIK_DIR").map_or_else(|_| PathBuf::from(DEFAULT_DIR), PathBuf::from);
    let mut paths: Vec<PathBuf> = std::fs::read_dir(&dir)
        .unwrap_or_else(|e| panic!("{}: {e}", dir.display()))
        .filter_map(|e| e.ok().map(|e| e.path()))
        .filter(|p| p.extension().is_some_and(|x| x.eq_ignore_ascii_case("bik")))
        .collect();
    paths.sort();
    assert!(!paths.is_empty(), "no .bik files in {}", dir.display());

    let decoders = VideoDecoders::default();
    let mut failures = 0;
    println!(
        "{:<20} {:>9} {:>11} {:<12} {:>10} {:>9} {:>8}",
        "file", "size", "frames/hdr", "audio", "samples", "total ms", "ms/frame"
    );
    for path in &paths {
        let name = path.file_name().unwrap_or_default().to_string_lossy();
        match catch_unwind(AssertUnwindSafe(|| decode(path, &decoders))) {
            Ok(Ok(r)) => {
                if r.frames != r.header_frames {
                    failures += 1;
                }
                println!(
                    "{name:<20} {:>9} {:>11} {:<12} {:>10} {:>9.0} {:>8.2}",
                    format!("{}x{}", r.size.0, r.size.1),
                    format!("{}/{}", r.frames, r.header_frames),
                    r.audio,
                    r.samples,
                    r.ms,
                    r.ms / f64::from(r.frames.max(1)),
                );
            }
            Ok(Err(e)) => {
                failures += 1;
                println!("{name:<20} FAILED: {e}");
            }
            Err(_) => {
                failures += 1;
                println!("{name:<20} PANICKED");
            }
        }
    }
    println!("{} files, {failures} failed", paths.len());
}
