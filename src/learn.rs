//! Learned calibration — the part of jelle that actually adapts.
//!
//! `decide` gates each attempt at a fixed line (`ABSTAIN_BELOW`). The law
//! itself never moves: uncertain abstains, forcing refuses. What learns is
//! *where the line sits* per flag kind: a kind whose interventions resolve
//! earns a lower line; a kind that recurs regardless earns a higher one.
//!
//! The outcome signal is free — every deliberation opens a pending decision
//! for its kind. The next same-kind flag closes it: recurred within the
//! window means the pattern persisted; a quiet gap means it resolved.
//! Beta-smoothed resolution rates for the spoke/abstain arms shift the
//! threshold inside [`THRESH_MIN`, `THRESH_MAX`] — calibration can move the
//! line but can never remove the gate or silence a kind outright.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Seconds within which a same-kind flag marks the prior decision "persisted"
/// rather than "resolved". Longer than maren's observation cadence, shorter
/// than a session lull.
pub const RECUR_WINDOW_SECS: u64 = 120;

/// Learned threshold bounds — the line may tighten or loosen but never drops
/// below a still-meaningful confidence floor, and never mutes a kind entirely.
pub const THRESH_MIN: f32 = 0.45;
pub const THRESH_MAX: f32 = 0.85;

/// Minimum recorded decisions before a kind's evidence may move the line.
pub const MIN_SAMPLES: u32 = 6;

/// How far the evidence may push the line from base, at most.
const GAIN: f32 = 0.4;

/// Outcome counts for one arm of the gate (spoke or abstained).
#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
pub struct ArmStats {
    pub trials: u32,
    pub resolved: u32,
}

impl ArmStats {
    /// Beta(1,1)-smoothed resolution rate — sparse evidence stays near 0.5
    /// instead of swinging on a single observation.
    pub fn resolve_rate(&self) -> f32 {
        ((self.resolved.min(self.trials) as f64 + 1.0) / (self.trials as f64 + 2.0)) as f32
    }
}

/// Per-kind learning record: both arms' outcomes plus the pending decision
/// awaiting its resolution signal.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct KindCalibration {
    /// Gate allowed a model proposal through.
    pub spoke: ArmStats,
    /// Gate refused a model proposal (no-forced-uncertainty held).
    pub abstained: ArmStats,
    /// Last gate decision for this kind, awaiting outcome:
    /// (unix secs, spoke?).
    #[serde(default)]
    pub pending: Option<(u64, bool)>,
}

/// Per-seat learned gate state. Serializes to a single JSON blob per seat.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CalibrationTable {
    /// The line evidence is learned around — `ABSTAIN_BELOW` by default.
    pub base: f32,
    #[serde(default)]
    pub kinds: HashMap<String, KindCalibration>,
}

// Preserve the infallible API while ensuring restored/publicly mutated tables
// cannot remove the gate. Non-finite configuration uses the strictest bound.
fn bounded_base(base: f32) -> f32 {
    if base.is_finite() {
        base.clamp(THRESH_MIN, THRESH_MAX)
    } else {
        THRESH_MAX
    }
}

impl CalibrationTable {
    pub fn new(base: f32) -> Self {
        CalibrationTable {
            base: bounded_base(base),
            kinds: HashMap::new(),
        }
    }

    /// Effective line for `kind`. Returns bounded `base` until MIN_SAMPLES decisions
    /// exist for that kind, then shifts toward whichever arm resolves better:
    /// abstain resolving more than speak pushes the line up (quieter); speak
    /// resolving more pulls it down (more willing). Always clamped.
    pub fn effective_threshold(&self, kind: &str) -> f32 {
        let base = bounded_base(self.base);
        let Some(k) = self.kinds.get(kind) else {
            return base;
        };
        if k.spoke.resolved > k.spoke.trials || k.abstained.resolved > k.abstained.trials {
            return THRESH_MAX;
        }
        if k.spoke.trials.saturating_add(k.abstained.trials) < MIN_SAMPLES {
            return base;
        }
        let diff = k.abstained.resolve_rate() - k.spoke.resolve_rate();
        (base + GAIN * diff).clamp(THRESH_MIN, THRESH_MAX)
    }

    /// A same-kind flag arrived — close the pending decision, if any.
    /// Recurred inside the window → the pattern persisted (not resolved);
    /// a quiet gap → resolved. Idempotent when nothing is pending.
    pub fn observe_flag(&mut self, kind: &str, now: u64) {
        self.close_pending(kind, now);
    }

    /// Record a gate decision for `kind` — closes any prior pending first
    /// (its outcome is decided by the gap to this flag), counts the trial,
    /// and opens a new pending awaiting the next same-kind flag.
    pub fn record_decision(&mut self, kind: &str, spoke: bool, now: u64) {
        self.close_pending(kind, now);
        let k = self.kinds.entry(kind.to_string()).or_default();
        if spoke {
            k.spoke.trials = k.spoke.trials.saturating_add(1);
        } else {
            k.abstained.trials = k.abstained.trials.saturating_add(1);
        }
        k.pending = Some((now, spoke));
    }

    /// Close pending for `kind` at `now`. Outcome = whether the flag recurred
    /// inside RECUR_WINDOW_SECS. Pending entries from before a long idle
    /// self-resolve (elapsed >> window), so stale state decays correctly.
    fn close_pending(&mut self, kind: &str, now: u64) {
        let Some(k) = self.kinds.get_mut(kind) else {
            return;
        };
        let Some((opened, spoke)) = k.pending.take() else {
            return;
        };
        if now.saturating_sub(opened) > RECUR_WINDOW_SECS {
            if spoke {
                k.spoke.resolved = k.spoke.resolved.saturating_add(1);
            } else {
                k.abstained.resolved = k.abstained.resolved.saturating_add(1);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_table_returns_base() {
        let t = CalibrationTable::new(0.55);
        assert_eq!(t.effective_threshold("grind"), 0.55);
    }

    #[test]
    fn under_min_samples_returns_base() {
        let mut t = CalibrationTable::new(0.55);
        for i in 0..(MIN_SAMPLES - 1) as u64 {
            t.record_decision("grind", true, i * 10_000);
        }
        assert_eq!(t.effective_threshold("grind"), 0.55);
    }

    #[test]
    fn speak_resolving_lowers_line() {
        let mut t = CalibrationTable::new(0.55);
        // Spoke decisions keep resolving: every pending is closed by a flag
        // arriving well past the window (quiet gap = resolved).
        for i in 0..8u64 {
            let t0 = i * 10_000;
            t.record_decision("grind", true, t0);
            t.record_decision("grind", true, t0 + 5_000);
            t.record_decision("grind", true, t0 + 6_000);
        }
        let eff = t.effective_threshold("grind");
        assert!(eff < 0.55, "expected below base, got {eff}");
        assert!(eff >= THRESH_MIN);
    }

    #[test]
    fn abstain_resolving_raises_line() {
        let mut t = CalibrationTable::new(0.55);
        // Abstains resolve (closed by a 5_000s quiet gap); spoke decisions
        // mostly persist (recurred at +10s, inside the window).
        for i in 0..8u64 {
            let t0 = i * 10_000;
            t.record_decision("grind", false, t0); // abstain → resolved next flag
            t.record_decision("grind", true, t0 + 5_000); // closes abstain: resolved
            t.record_decision("grind", true, t0 + 5_010); // closes spoke: persisted
            t.record_decision("grind", true, t0 + 5_020); // closes spoke: persisted
        }
        let eff = t.effective_threshold("grind");
        assert!(eff > 0.55, "expected above base, got {eff}");
        assert!(eff <= THRESH_MAX);
    }

    #[test]
    fn recurrence_within_window_is_persisted_not_resolved() {
        let mut t = CalibrationTable::new(0.55);
        t.record_decision("loop", true, 1_000);
        t.record_decision("loop", true, 1_000 + RECUR_WINDOW_SECS); // inside window
        let k = &t.kinds["loop"];
        assert_eq!(k.spoke.trials, 2);
        assert_eq!(k.spoke.resolved, 0);
    }

    #[test]
    fn quiet_gap_counts_resolved() {
        let mut t = CalibrationTable::new(0.55);
        t.record_decision("loop", true, 1_000);
        t.record_decision("loop", true, 1_000 + RECUR_WINDOW_SECS + 1); // outside window
        let k = &t.kinds["loop"];
        assert_eq!(k.spoke.resolved, 1);
    }

    #[test]
    fn observe_flag_closes_pending_without_new_trial() {
        let mut t = CalibrationTable::new(0.55);
        t.record_decision("drift", false, 1_000);
        t.observe_flag("drift", 1_000 + RECUR_WINDOW_SECS + 10);
        let k = &t.kinds["drift"];
        assert_eq!(k.abstained.trials, 1);
        assert_eq!(k.abstained.resolved, 1);
        assert!(k.pending.is_none());
    }

    #[test]
    fn bounds_hold_under_extreme_evidence() {
        let mut t = CalibrationTable::new(0.55);
        for i in 0..50u64 {
            t.record_decision("panic", true, i * 10_000);
            t.record_decision("panic", true, i * 10_000 + 5_000);
        }
        assert_eq!(t.effective_threshold("panic"), THRESH_MIN);
    }

    #[test]
    fn serializes_round_trip() {
        let mut t = CalibrationTable::new(0.55);
        t.record_decision("grind", true, 42);
        let s = serde_json::to_string(&t).unwrap();
        let back: CalibrationTable = serde_json::from_str(&s).unwrap();
        assert_eq!(back.kinds["grind"].spoke.trials, 1);
        assert_eq!(back.kinds["grind"].pending, Some((42, true)));
    }
}
