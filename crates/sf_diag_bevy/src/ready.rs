//! Readiness gate for automated captures (screenshots, benchmark starts).
//!
//! A frame is only worth capturing once the CPU side has settled (assets
//! loaded, streaming idle) *and* the GPU side can draw it: Bevy compiles
//! render pipelines asynchronously and skips a draw whose pipeline is still
//! queued or compiling, so a "loaded" mesh can be invisible for a few frames.
//! The gate passes once both held for `stable_frames` consecutive frames
//! (the loading-screen practice of Bevy's `loading_screen` example, which
//! waits on `PipelineCache`; Unreal's PSO precaching hides such primitives).
//! Past `timeout_secs` it reports a timeout instead of passing silently, so
//! the caller can fail the run rather than save a half-drawn frame.
//!
//! Pure logic: the caller feeds one [`ReadyGate::step`] per frame with its
//! CPU verdict and the GPU counts (e.g. `PipelineCache::waiting_pipelines`,
//! extracted to the main world).

/// GPU work that keeps a frame from showing everything.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct GpuPending {
    /// Render/compute pipelines queued or compiling.
    pub pipelines: usize,
    /// Visible draws skipped because their pipeline isn't ready (0 if unknown).
    pub draws: usize,
}

impl GpuPending {
    pub fn is_clear(&self) -> bool {
        self.pipelines == 0 && self.draws == 0
    }
}

/// What the gate saw, for the log line (and slice-2 per-shot receipts).
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct GateReceipt {
    /// Frames stepped from the first step to the verdict.
    pub frames_waited: u32,
    /// Frames from the first CPU-ready frame to the verdict.
    pub frames_after_cpu: u32,
    /// `now` at the verdict.
    pub secs: f32,
    /// GPU counts at the first CPU-ready frame (None: the CPU never settled).
    pub first_pending: Option<GpuPending>,
    /// GPU counts at the verdict.
    pub last_pending: GpuPending,
    /// CPU verdict at the last step.
    pub cpu_ready: bool,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GateStatus {
    Waiting,
    Ready(GateReceipt),
    TimedOut(GateReceipt),
}

/// See the module docs. Feed once per frame; restart with [`ReadyGate::reset`].
#[derive(Clone, Debug)]
pub struct ReadyGate {
    pub stable_frames: u32,
    pub timeout_secs: f32,
    run: u32,
    receipt: GateReceipt,
}

impl ReadyGate {
    pub fn new(stable_frames: u32, timeout_secs: f32) -> Self {
        Self { stable_frames: stable_frames.max(1), timeout_secs, run: 0, receipt: GateReceipt::default() }
    }

    /// Forget progress (e.g. the scene changed under the gate).
    pub fn reset(&mut self) {
        self.run = 0;
        self.receipt = GateReceipt::default();
    }

    /// One frame: `now` in seconds on the timeout's clock.
    pub fn step(&mut self, now: f32, cpu_ready: bool, gpu: GpuPending) -> GateStatus {
        let r = &mut self.receipt;
        r.frames_waited += 1;
        if cpu_ready && r.first_pending.is_none() {
            r.first_pending = Some(gpu);
        }
        if r.first_pending.is_some() {
            r.frames_after_cpu += 1;
        }
        r.secs = now;
        r.last_pending = gpu;
        r.cpu_ready = cpu_ready;
        self.run = if cpu_ready && gpu.is_clear() { self.run + 1 } else { 0 };
        if self.run >= self.stable_frames {
            GateStatus::Ready(*r)
        } else if now >= self.timeout_secs {
            GateStatus::TimedOut(*r)
        } else {
            GateStatus::Waiting
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const CLEAR: GpuPending = GpuPending { pipelines: 0, draws: 0 };
    const BUSY: GpuPending = GpuPending { pipelines: 3, draws: 7 };

    #[test]
    fn waits_for_cpu_then_stable_gpu() {
        let mut g = ReadyGate::new(3, 120.0);
        assert_eq!(g.step(0.0, false, CLEAR), GateStatus::Waiting);
        assert_eq!(g.step(0.1, true, BUSY), GateStatus::Waiting);
        assert_eq!(g.step(0.2, true, CLEAR), GateStatus::Waiting);
        assert_eq!(g.step(0.3, true, CLEAR), GateStatus::Waiting);
        let GateStatus::Ready(r) = g.step(0.4, true, CLEAR) else { panic!("not ready") };
        assert_eq!(r.frames_waited, 5);
        assert_eq!(r.frames_after_cpu, 4);
        assert_eq!(r.first_pending, Some(BUSY));
    }

    #[test]
    fn a_new_pipeline_restarts_the_run() {
        let mut g = ReadyGate::new(2, 120.0);
        g.step(0.0, true, CLEAR);
        assert_eq!(g.step(0.1, true, GpuPending { pipelines: 1, draws: 0 }), GateStatus::Waiting);
        assert_eq!(g.step(0.2, true, CLEAR), GateStatus::Waiting);
        assert!(matches!(g.step(0.3, true, CLEAR), GateStatus::Ready(_)));
    }

    #[test]
    fn cpu_regress_restarts_the_run() {
        let mut g = ReadyGate::new(2, 120.0);
        g.step(0.0, true, CLEAR);
        assert_eq!(g.step(0.1, false, CLEAR), GateStatus::Waiting);
        assert_eq!(g.step(0.2, true, CLEAR), GateStatus::Waiting);
        assert!(matches!(g.step(0.3, true, CLEAR), GateStatus::Ready(_)));
    }

    #[test]
    fn times_out_with_what_was_pending() {
        let mut g = ReadyGate::new(2, 1.0);
        assert_eq!(g.step(0.5, false, BUSY), GateStatus::Waiting);
        let GateStatus::TimedOut(r) = g.step(1.0, false, BUSY) else { panic!("no timeout") };
        assert!(!r.cpu_ready);
        assert_eq!(r.first_pending, None);
        assert_eq!(r.last_pending, BUSY);
        g.reset();
        assert_eq!(g.step(0.0, true, CLEAR), GateStatus::Waiting);
    }
}
