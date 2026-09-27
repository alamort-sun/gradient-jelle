//! Fail when uncertain predictions are forced instead of abstained.
//! Abstention is a valid output.

use thiserror::Error;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PredictionAttempt {
    pub label: &'static str,
    pub confidence: f32,
    /// Caller tried to force a label despite uncertainty.
    pub force: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Decision {
    Predict {
        label: &'static str,
        confidence: f32,
    },
    Abstain {
        reason: &'static str,
    },
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum PredictionError {
    #[error("confidence outside [0, 1]")]
    InvalidConfidence,
    #[error("forcing an uncertain prediction is forbidden")]
    ForcedUncertainty,
}

/// Below this confidence, abstain unless... never force. Force is always Err.
pub const ABSTAIN_BELOW: f32 = 0.55;

/// The default line. `decide` pins the threshold to `ABSTAIN_BELOW`.
pub fn decide(attempt: &PredictionAttempt) -> Result<Decision, PredictionError> {
    decide_with(attempt, ABSTAIN_BELOW)
}

/// The same gate at a caller-chosen line — the entry point learned
/// calibration uses (`learn::CalibrationTable`). The law is identical at
/// any threshold: out-of-range confidence errors, uncertain abstains,
/// forcing is always refused.
pub fn decide_with(
    attempt: &PredictionAttempt,
    threshold: f32,
) -> Result<Decision, PredictionError> {
    if !(0.0..=1.0).contains(&attempt.confidence) || attempt.confidence.is_nan() {
        return Err(PredictionError::InvalidConfidence);
    }
    if attempt.confidence < threshold {
        if attempt.force {
            return Err(PredictionError::ForcedUncertainty);
        }
        return Ok(Decision::Abstain {
            reason: "confidence below abstain threshold",
        });
    }
    Ok(Decision::Predict {
        label: attempt.label,
        confidence: attempt.confidence,
    })
}

#[cfg(test)]
mod unit {
    use super::*;

    #[test]
    fn high_confidence_predicts() {
        let d = decide(&PredictionAttempt {
            label: "ok",
            confidence: 0.9,
            force: false,
        })
        .unwrap();
        assert!(matches!(d, Decision::Predict { .. }));
    }
}
