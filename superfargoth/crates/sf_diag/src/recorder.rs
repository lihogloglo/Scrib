//! Per-frame columns: frame time plus one column per counter.

use std::io::{self, Write};

use crate::counters::{CounterDef, CounterKind, Counters};

/// One counter's column and run totals.
#[derive(Clone, Debug)]
pub struct Series {
    def: CounterDef,
    /// One value per recorded frame. Untouched frames hold 0 for
    /// [`CounterKind::Sum`] and NaN for [`CounterKind::Gauge`].
    values: Vec<f64>,
    total: f64,
    last: Option<f64>,
    reports: u64,
}

impl Series {
    pub fn def(&self) -> &CounterDef {
        &self.def
    }

    /// Per-frame values, aligned with [`Recorder::frame_ms`].
    pub fn values(&self) -> &[f64] {
        &self.values
    }

    /// Sum of every reported value over the run, frames or not.
    pub fn total(&self) -> f64 {
        self.total
    }

    /// The most recent reported value.
    pub fn last(&self) -> Option<f64> {
        self.last
    }

    /// Number of frames (or flushes) in which the counter was reported.
    pub fn reports(&self) -> u64 {
        self.reports
    }

    fn fill(&self) -> f64 {
        match self.def.kind {
            CounterKind::Sum => 0.0,
            CounterKind::Gauge => f64::NAN,
        }
    }
}

/// Records frames and counter columns for one measured run.
///
/// Call [`Recorder::commit_frame`] once per frame. Tools without frames
/// (a bake CLI) call [`Recorder::flush`] instead, which only updates totals.
#[derive(Clone, Debug, Default)]
pub struct Recorder {
    frame_ms: Vec<f64>,
    series: Vec<Series>,
}

impl Recorder {
    pub fn new() -> Self {
        Self::default()
    }

    /// Frame times in milliseconds, one per committed frame.
    pub fn frame_ms(&self) -> &[f64] {
        &self.frame_ms
    }

    /// Counter series in registration order.
    pub fn series(&self) -> &[Series] {
        &self.series
    }

    /// The series of a counter, by name.
    pub fn get(&self, name: &str) -> Option<&Series> {
        self.series.iter().find(|s| s.def.name == name)
    }

    /// Records one frame: its duration and every counter's value, which is
    /// then reset.
    pub fn commit_frame(&mut self, frame_ms: f64, counters: &Counters) {
        self.frame_ms.push(frame_ms);
        let frames = self.frame_ms.len();
        for series in &mut self.series {
            let fill = series.fill();
            series.values.push(fill);
        }
        self.absorb(counters, Some(frames - 1));
    }

    /// Adds every counter's value to the run totals without a frame row,
    /// and resets the counters.
    pub fn flush(&mut self, counters: &Counters) {
        self.absorb(counters, None);
    }

    /// Resets the counters, dropping their values (warm-up frames).
    pub fn discard(counters: &Counters) {
        counters.clear();
    }

    /// Adds (or replaces) a derived per-frame column computed from the
    /// recorded ones, e.g. [`crate::pacing::derive_work`]. `values` must have
    /// one entry per recorded frame; NaN marks a frame without a value.
    ///
    /// # Panics
    /// If `values` is not aligned with [`Recorder::frame_ms`].
    pub fn set_derived(&mut self, def: CounterDef, values: Vec<f64>) {
        assert_eq!(values.len(), self.frame_ms.len(), "derived column `{}` not aligned with the frames", def.name);
        let finite = values.iter().copied().filter(|v| v.is_finite());
        let (total, reports) = finite.fold((0.0, 0u64), |(t, n), v| (t + v, n + 1));
        let last = values.iter().rev().copied().find(|v| v.is_finite());
        let series = Series { def, values, total, last, reports };
        match self.series.iter_mut().find(|s| s.def.name == series.def.name) {
            Some(existing) => *existing = series,
            None => self.series.push(series),
        }
    }

    /// Forgets everything recorded so far.
    pub fn reset(&mut self) {
        self.frame_ms.clear();
        self.series.clear();
    }

    fn absorb(&mut self, counters: &Counters, row: Option<usize>) {
        let frames = self.frame_ms.len();
        counters.drain(|id, def, value| {
            // `drain` visits every registered counter in id order, so a
            // counter registered since the last drain is always the next
            // series to create.
            let index = id.index();
            if self.series.len() == index {
                let mut series = Series {
                    def: def.clone(),
                    values: Vec::new(),
                    total: 0.0,
                    last: None,
                    reports: 0,
                };
                series.values = vec![series.fill(); frames];
                self.series.push(series);
            }
            let series = &mut self.series[index];
            if let Some(value) = value {
                series.total += value;
                series.last = Some(value);
                series.reports += 1;
                if let Some(row) = row {
                    series.values[row] = value;
                }
            }
        });
    }

    /// Writes the per-frame CSV: `frame,t_s,frame_ms,<counter>...`. `t_s`
    /// is the time at the end of the frame since the first recorded frame
    /// began. Missing gauge values are empty cells.
    pub fn write_csv(&self, mut w: impl Write) -> io::Result<()> {
        write!(w, "frame,t_s,frame_ms")?;
        for series in &self.series {
            write!(w, ",{}", csv_field(&series.def.name))?;
        }
        writeln!(w)?;
        let mut t_ms = 0.0;
        for (frame, &ms) in self.frame_ms.iter().enumerate() {
            t_ms += ms;
            write!(w, "{frame},{},{}", number(t_ms / 1000.0), number(ms))?;
            for series in &self.series {
                let value = series.values[frame];
                if value.is_nan() {
                    write!(w, ",")?;
                } else {
                    write!(w, ",{}", number(value))?;
                }
            }
            writeln!(w)?;
        }
        Ok(())
    }
}

/// Quotes a CSV field when needed (RFC 4180).
fn csv_field(field: &str) -> String {
    if field.contains([',', '"', '\n', '\r']) {
        format!("\"{}\"", field.replace('"', "\"\""))
    } else {
        field.to_owned()
    }
}

/// Integers print without a fraction; everything else with up to 4
/// decimals (0.1 µs for milliseconds), trailing zeros trimmed.
fn number(value: f64) -> String {
    if value.fract() == 0.0 && value.abs() < 1e15 {
        format!("{value:.0}")
    } else {
        let text = format!("{value:.4}");
        text.trim_end_matches('0').trim_end_matches('.').to_owned()
    }
}
