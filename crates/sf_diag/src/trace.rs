//! Chrome trace (`trace_event` JSON) analysis: per-span CPU time over a
//! window, and the long spans behind frame spikes.
//!
//! Bevy's `trace_chrome` feature writes one event per line
//! (`tracing-chrome`): `B`/`E` pairs per thread, or `X` events with a
//! `dur`, plus `M` metadata naming threads. Traces of a few minutes reach
//! gigabytes, so this streams lines and pulls out the few fields it needs
//! instead of parsing JSON.

use std::collections::HashMap;
use std::io::{self, BufRead};

/// Name of the span that wraps one whole frame (Bevy's `SubApps::update`).
pub const FRAME_SPAN: &str = "update";

/// Spans that only wrap other work (schedules, executors, sub-apps): left
/// out of [`long_spans`] so the system or task inside is what shows.
pub const WRAPPER_PREFIXES: &[&str] = &[
    "update",
    "main app",
    "sub app",
    "schedule",
    "multithreaded executor",
    "frame",
    "system_commands",
    "ExtractSchedule",
];

/// One event's fields.
struct Event<'a> {
    ph: u8,
    tid: u64,
    ts_us: f64,
    dur_us: Option<f64>,
    name: Option<&'a str>,
}

/// The raw text of `"key":<value>` (a number, or a string without quotes).
fn field<'a>(line: &'a str, key: &str) -> Option<&'a str> {
    let pattern = format!("\"{key}\":");
    let start = line.find(&pattern)? + pattern.len();
    let rest = line[start..].trim_start();
    if let Some(s) = rest.strip_prefix('"') {
        // Up to the closing quote, skipping escaped ones.
        let bytes = s.as_bytes();
        let mut i = 0;
        while i < bytes.len() {
            match bytes[i] {
                b'\\' => i += 2,
                b'"' => return Some(&s[..i]),
                _ => i += 1,
            }
        }
        None
    } else {
        let end = rest.find([',', '}']).unwrap_or(rest.len());
        Some(rest[..end].trim())
    }
}

fn parse_event(line: &str) -> Option<Event<'_>> {
    let ph = *field(line, "ph")?.as_bytes().first()?;
    Some(Event {
        ph,
        tid: field(line, "tid").and_then(|v| v.parse().ok()).unwrap_or(0),
        ts_us: field(line, "ts")
            .and_then(|v| v.parse().ok())
            .unwrap_or(0.0),
        dur_us: field(line, "dur").and_then(|v| v.parse().ok()),
        name: field(line, "name"),
    })
}

/// A span's name with escapes undone (`\"` → `"`).
fn unescape(name: &str) -> String {
    name.replace("\\\"", "\"").replace("\\\\", "\\")
}

/// One closed span: thread, name, start and end (µs since trace start).
struct Closed {
    tid: u64,
    name: String,
    start: f64,
    end: f64,
    /// Time spent in direct children.
    child: f64,
    /// Depth on its thread (0 = outermost).
    depth: usize,
}

/// Streams the trace and calls `visit` for every closed span, with the
/// thread-name map as it stands.
fn for_each_span(
    reader: impl BufRead,
    mut visit: impl FnMut(&Closed, &HashMap<u64, String>),
) -> io::Result<()> {
    let mut stacks: HashMap<u64, Vec<(String, f64, f64)>> = HashMap::new();
    let mut threads: HashMap<u64, String> = HashMap::new();
    for line in reader.lines() {
        let line = line?;
        let Some(e) = parse_event(&line) else {
            continue;
        };
        match e.ph {
            b'M' => {
                // {"ph":"M","name":"thread_name","args":{"name":"Compute Task Pool (0)"},...}
                if let Some(args) = line.find("\"args\"")
                    && let Some(name) = field(&line[args..], "name")
                {
                    threads.insert(e.tid, unescape(name));
                }
            }
            b'B' => {
                let name = e.name.map(unescape).unwrap_or_default();
                stacks.entry(e.tid).or_default().push((name, e.ts_us, 0.0));
            }
            b'E' => {
                let stack = stacks.entry(e.tid).or_default();
                let Some((name, start, child)) = stack.pop() else {
                    continue;
                };
                let d = e.ts_us - start;
                if let Some(parent) = stack.last_mut() {
                    parent.2 += d;
                }
                let depth = stack.len();
                visit(
                    &Closed {
                        tid: e.tid,
                        name,
                        start,
                        end: e.ts_us,
                        child,
                        depth,
                    },
                    &threads,
                );
            }
            b'X' => {
                // Complete events carry their duration. Inside an open
                // `B` span they are its children; their own children are
                // unknown.
                let Some(dur) = e.dur_us else { continue };
                let name = e.name.map(unescape).unwrap_or_default();
                let stack = stacks.entry(e.tid).or_default();
                if let Some(parent) = stack.last_mut() {
                    parent.2 += dur;
                }
                let depth = stack.len();
                let closed = Closed {
                    tid: e.tid,
                    name,
                    start: e.ts_us,
                    end: e.ts_us + dur,
                    child: 0.0,
                    depth,
                };
                visit(&closed, &threads);
            }
            _ => {}
        }
    }
    Ok(())
}

/// One span name over the window.
#[derive(Clone, Debug, PartialEq)]
pub struct SpanStat {
    pub name: String,
    pub calls: u64,
    /// Time inside the span minus its children, µs.
    pub self_us: f64,
    pub inclusive_us: f64,
    pub max_us: f64,
}

/// CPU time per span name and thread over a window.
#[derive(Clone, Debug, PartialEq)]
pub struct TraceSummary {
    pub from_s: f64,
    pub to_s: f64,
    /// Number of [`FRAME_SPAN`] spans in the window.
    pub frames: u64,
    /// By self time, largest first.
    pub spans: Vec<SpanStat>,
    /// Outermost-span busy time per thread name, µs, largest first.
    pub threads: Vec<(String, f64)>,
}

/// Summarizes spans that lie fully inside `[from_s, to_s]` (seconds since
/// the trace began).
pub fn summarize(reader: impl BufRead, from_s: f64, to_s: f64) -> io::Result<TraceSummary> {
    let (from, to) = (from_s * 1e6, to_s * 1e6);
    let mut spans: HashMap<String, SpanStat> = HashMap::new();
    let mut busy: HashMap<u64, f64> = HashMap::new();
    let mut names: HashMap<u64, String> = HashMap::new();
    let mut frames = 0;
    for_each_span(reader, |s, threads| {
        if s.start < from || s.end > to {
            return;
        }
        let d = s.end - s.start;
        if s.name == FRAME_SPAN || s.name.starts_with("update:") {
            frames += 1;
        }
        let stat = spans.entry(s.name.clone()).or_insert_with(|| SpanStat {
            name: s.name.clone(),
            calls: 0,
            self_us: 0.0,
            inclusive_us: 0.0,
            max_us: 0.0,
        });
        stat.calls += 1;
        stat.self_us += d - s.child;
        stat.inclusive_us += d;
        stat.max_us = stat.max_us.max(d);
        if s.depth == 0 {
            *busy.entry(s.tid).or_default() += d;
        }
        if let Some(n) = threads.get(&s.tid) {
            names.insert(s.tid, n.clone());
        }
    })?;
    let mut spans: Vec<SpanStat> = spans.into_values().collect();
    spans.sort_by(|a, b| b.self_us.total_cmp(&a.self_us));
    let mut threads: Vec<(String, f64)> = busy
        .into_iter()
        .map(|(tid, us)| {
            (
                names
                    .get(&tid)
                    .cloned()
                    .unwrap_or_else(|| format!("thread {tid}")),
                us,
            )
        })
        .collect();
    threads.sort_by(|a, b| b.1.total_cmp(&a.1));
    Ok(TraceSummary {
        from_s,
        to_s,
        frames,
        spans,
        threads,
    })
}

impl TraceSummary {
    /// Plain-text tables: top self time per frame, schedules inclusive,
    /// per-thread busy time.
    pub fn to_text(&self, top: usize, width: usize) -> String {
        use std::fmt::Write as _;
        #[allow(clippy::cast_precision_loss)]
        let frames = self.frames.max(1) as f64;
        let window = (self.to_s - self.from_s).max(1e-9);
        let mut out = String::new();
        let _ = writeln!(
            out,
            "{} frames in {:.1}..{:.1} s ({:.1} fps traced)",
            self.frames,
            self.from_s,
            self.to_s,
            frames / window
        );
        let _ = writeln!(
            out,
            "\n== top self time per frame (ms), calls/frame, worst call (ms)"
        );
        for s in self.spans.iter().take(top) {
            #[allow(clippy::cast_precision_loss)]
            let calls = s.calls as f64 / frames;
            let _ = writeln!(
                out,
                "{:8.3}  x{:7.1}  max {:7.2}  {}",
                s.self_us / frames / 1e3,
                calls,
                s.max_us / 1e3,
                clip(&s.name, width)
            );
        }
        let _ = writeln!(out, "\n== schedules and sub-apps, inclusive per frame (ms)");
        let mut wrappers: Vec<&SpanStat> = self
            .spans
            .iter()
            .filter(|s| {
                s.name.starts_with("schedule")
                    || s.name.starts_with("sub app")
                    || s.name.starts_with("main app")
                    || s.name == FRAME_SPAN
            })
            .collect();
        wrappers.sort_by(|a, b| b.inclusive_us.total_cmp(&a.inclusive_us));
        for s in wrappers.iter().take(30) {
            let _ = writeln!(
                out,
                "{:8.2}  {}",
                s.inclusive_us / frames / 1e3,
                clip(&s.name, width)
            );
        }
        let _ = writeln!(out, "\n== busy per thread per frame (ms)");
        for (name, us) in self.threads.iter().take(24) {
            let _ = writeln!(out, "{:8.2}  {name}", us / frames / 1e3);
        }
        out
    }
}

fn clip(s: &str, width: usize) -> &str {
    match s.char_indices().nth(width) {
        Some((i, _)) => &s[..i],
        None => s,
    }
}

/// Spans of one name that ran longer than the threshold.
#[derive(Clone, Debug, PartialEq)]
pub struct LongSpan {
    pub name: String,
    pub count: u64,
    pub worst_us: f64,
    /// When the worst one ended, seconds since the trace began.
    pub worst_end_s: f64,
    pub mean_us: f64,
}

/// Every span (wrappers in [`WRAPPER_PREFIXES`] excluded) longer than
/// `min_ms` overlapping `[from_s, to_s]`, grouped by name, by total time.
pub fn long_spans(
    reader: impl BufRead,
    from_s: f64,
    to_s: f64,
    min_ms: f64,
) -> io::Result<Vec<LongSpan>> {
    let (from, to, min) = (from_s * 1e6, to_s * 1e6, min_ms * 1e3);
    let mut by_name: HashMap<String, (u64, f64, f64, f64)> = HashMap::new();
    for_each_span(reader, |s, _| {
        let d = s.end - s.start;
        if s.end < from
            || s.start > to
            || d < min
            || WRAPPER_PREFIXES.iter().any(|p| s.name.starts_with(p))
        {
            return;
        }
        let e = by_name.entry(s.name.clone()).or_insert((0, 0.0, 0.0, 0.0));
        e.0 += 1;
        e.1 += d;
        if d > e.2 {
            e.2 = d;
            e.3 = s.end / 1e6;
        }
    })?;
    let mut out: Vec<(f64, LongSpan)> = by_name
        .into_iter()
        .map(|(name, (count, total, worst, at))| {
            #[allow(clippy::cast_precision_loss)]
            let mean = total / count as f64;
            (
                total,
                LongSpan {
                    name,
                    count,
                    worst_us: worst,
                    worst_end_s: at,
                    mean_us: mean,
                },
            )
        })
        .collect();
    out.sort_by(|a, b| b.0.total_cmp(&a.0));
    Ok(out.into_iter().map(|(_, s)| s).collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Two frames on thread 1; a system with a nested span; a long task on
    /// thread 2 (`tracing-chrome` field order varies, so it does here too).
    const TRACE: &str = r#"[
{"ph":"M","pid":1,"name":"thread_name","tid":2,"args":{"name":"Compute Task Pool (0)"}},
{"ph":"B","pid":1,"tid":1,"ts":1000000.0,"name":"update"},
{"name":"system: name=\"game::stream\"","ph":"B","pid":1,"tid":1,"ts":1000100.0},
{"ph":"B","pid":1,"tid":1,"ts":1000200.0,"name":"spawn"},
{"ph":"E","pid":1,"tid":1,"ts":1000700.0},
{"ph":"E","pid":1,"tid":1,"ts":1001100.0},
{"ph":"E","pid":1,"tid":1,"ts":1010000.0},
{"tid":2,"ph":"B","pid":1,"ts":1002000.0,"name":"bake chunk"},
{"tid":2,"ph":"E","pid":1,"ts":1009000.0},
{"ph":"B","pid":1,"tid":1,"ts":1010000.0,"name":"update"},
{"ph":"X","pid":1,"tid":1,"ts":1010100.0,"dur":300.0,"name":"system: name=\"game::stream\""},
{"ph":"E","pid":1,"tid":1,"ts":1020000.0},
{"ph":"B","pid":1,"tid":1,"ts":9000000.0,"name":"update"},
{"ph":"E","pid":1,"tid":1,"ts":9010000.0},
"#;

    #[test]
    fn summarizes_self_time_frames_and_threads() {
        let s = summarize(TRACE.as_bytes(), 0.5, 2.0).unwrap();
        assert_eq!(s.frames, 2, "the frame at 9 s is outside the window");
        let stream = s
            .spans
            .iter()
            .find(|x| x.name == "system: name=\"game::stream\"")
            .unwrap();
        assert_eq!(stream.calls, 2);
        // First call 1000 µs incl. 500 µs child; second 300 µs.
        assert!((stream.inclusive_us - 1300.0).abs() < 1e-6);
        assert!((stream.self_us - 800.0).abs() < 1e-6);
        assert_eq!(s.threads[0].0, "thread 1");
        assert!((s.threads[0].1 - 20_000.0).abs() < 1e-6);
        assert_eq!(s.threads[1].0, "Compute Task Pool (0)");
        let text = s.to_text(10, 80);
        assert!(text.contains("2 frames"), "{text}");
    }

    #[test]
    fn long_spans_skip_wrappers() {
        let long = long_spans(TRACE.as_bytes(), 0.0, 100.0, 2.0).unwrap();
        let names: Vec<&str> = long.iter().map(|l| l.name.as_str()).collect();
        assert_eq!(names, vec!["bake chunk"], "frames (`update`) are wrappers");
        assert!((long[0].worst_us - 7000.0).abs() < 1e-6);
        assert!((long[0].worst_end_s - 1.009).abs() < 1e-9);
    }
}
