//! Scenarios: warm up, then measure for a time, a frame count, or along a
//! camera path.

use serde::{Deserialize, Serialize};

use crate::DiagError;
use crate::gate::Gate;
use crate::path::{CameraPath, Keyframe, Pose};
use crate::report::Tolerance;

/// Warm-up before measuring: both minimums must pass (and the app's hold,
/// if any, must be released). Shaders compile and caches fill here.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Warmup {
    pub seconds: f64,
    pub frames: u32,
}

impl Default for Warmup {
    fn default() -> Self {
        Self {
            seconds: 2.0,
            frames: 30,
        }
    }
}

/// What is measured after the warm-up.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum Measure {
    /// A fixed amount of real time.
    Duration { seconds: f64 },
    /// A fixed number of frames.
    Frames { count: u32 },
    /// Travel a camera path at `speed` units per second, in real time: a
    /// slow frame means a longer jump, as for a player.
    Path {
        keyframes: Vec<Keyframe>,
        speed: f64,
    },
}

/// A benchmark scenario, usually loaded from a data file.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Scenario {
    pub name: String,
    #[serde(default)]
    pub warmup: Warmup,
    pub measure: Measure,
    #[serde(default)]
    pub gates: Vec<Gate>,
    /// A spike is a frame longer than this × the median frame.
    #[serde(default = "default_spike_factor")]
    pub spike_factor: f64,
    #[serde(default)]
    pub tolerance: Tolerance,
}

fn positive(value: f64) -> bool {
    value.is_finite() && value > 0.0
}

fn default_spike_factor() -> f64 {
    1.5
}

impl Scenario {
    pub fn from_json(text: &str) -> Result<Self, DiagError> {
        Ok(serde_json::from_str(text)?)
    }
}

/// Where a scenario is.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Phase {
    Warmup,
    Measure,
    Done,
}

/// What to do with the frame that just ended, and the camera pose for the
/// next one.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Step {
    /// Phase for the next frame.
    pub phase: Phase,
    /// Whether the frame that just ended was measured (commit it to the
    /// recorder) or not (discard its counters).
    pub record: bool,
    /// Camera pose for the next frame, for path scenarios.
    pub pose: Option<Pose>,
    /// True exactly once, on the step that ends measurement: build and
    /// write the report now.
    pub just_finished: bool,
}

/// A running scenario: a small state machine driven once per frame.
#[derive(Clone, Debug)]
pub struct ScenarioRun {
    scenario: Scenario,
    path: Option<(CameraPath, f64)>,
    phase: Phase,
    warm_frames: u32,
    warm_s: f64,
    frames: u32,
    measured_s: f64,
    /// Path distance shown in the frame that is running now.
    shown: f64,
}

impl ScenarioRun {
    pub fn new(scenario: Scenario) -> Result<Self, DiagError> {
        let invalid = |msg: &str| Err(DiagError::Scenario(format!("`{}`: {msg}", scenario.name)));
        if !(scenario.spike_factor.is_finite() && scenario.spike_factor > 1.0) {
            return invalid("spike_factor must be > 1");
        }
        let path = match &scenario.measure {
            Measure::Duration { seconds } if !positive(*seconds) => {
                return invalid("duration must be > 0");
            }
            Measure::Frames { count: 0 } => return invalid("frame count must be > 0"),
            Measure::Path { keyframes, speed } => {
                if !positive(*speed) {
                    return invalid("speed must be > 0");
                }
                Some((CameraPath::new(keyframes.clone())?, *speed))
            }
            _ => None,
        };
        Ok(Self {
            scenario,
            path,
            phase: Phase::Warmup,
            warm_frames: 0,
            warm_s: 0.0,
            frames: 0,
            measured_s: 0.0,
            shown: 0.0,
        })
    }

    pub fn scenario(&self) -> &Scenario {
        &self.scenario
    }

    pub fn phase(&self) -> Phase {
        self.phase
    }

    /// The camera path, if this is a path scenario.
    pub fn path(&self) -> Option<&CameraPath> {
        self.path.as_ref().map(|(p, _)| p)
    }

    /// The pose to hold during warm-up (the path's start).
    pub fn start_pose(&self) -> Option<Pose> {
        self.path().map(|p| p.pose_at_distance(0.0))
    }

    /// Advances by the frame that just ended (`frame_ms` of real time).
    /// `hold` keeps the warm-up going (e.g. until streaming is idle).
    pub fn advance(&mut self, frame_ms: f64, hold: bool) -> Step {
        let dt = frame_ms / 1000.0;
        match self.phase {
            Phase::Warmup => {
                self.warm_frames += 1;
                self.warm_s += dt;
                let warm = &self.scenario.warmup;
                if !hold && self.warm_frames >= warm.frames && self.warm_s >= warm.seconds {
                    self.phase = Phase::Measure;
                }
                Step {
                    phase: self.phase,
                    record: false,
                    pose: self.start_pose(),
                    just_finished: false,
                }
            }
            Phase::Measure => {
                self.frames += 1;
                self.measured_s += dt;
                let (finished, pose) = match (&self.scenario.measure, &self.path) {
                    (_, Some((path, speed))) => {
                        // Finish after the frame that showed the path's end.
                        // (Tolerates arc-length rounding at the end.)
                        let end = path.length() * (1.0 - 1e-12);
                        let finished = self.shown >= end;
                        self.shown = (self.measured_s * speed).min(path.length());
                        (finished, Some(path.pose_at_distance(self.shown)))
                    }
                    (Measure::Duration { seconds }, None) => (self.measured_s >= *seconds, None),
                    (Measure::Frames { count }, None) => (self.frames >= *count, None),
                    (Measure::Path { .. }, None) => unreachable!("path scenarios have a path"),
                };
                if finished {
                    self.phase = Phase::Done;
                }
                Step {
                    phase: self.phase,
                    record: true,
                    pose,
                    just_finished: finished,
                }
            }
            Phase::Done => Step {
                phase: Phase::Done,
                record: false,
                pose: None,
                just_finished: false,
            },
        }
    }
}
