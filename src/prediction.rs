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
    Predict { label: &'static str, confidence: f32 },
    Abstain { reason: &'static str },
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

pub fn decide(attempt: &PredictionAttempt) -> Result<Decision, PredictionError> {
    if !(0.0..=1.0).contains(&attempt.confidence) || attempt.confidence.is_nan() {
        return Err(PredictionError::InvalidConfidence);
    }
    if attempt.confidence < ABSTAIN_BELOW {
        if attempt.force {
            return Err(PredictionError::ForcedUncertainty);
        }
        return Ok(Decision::Abstain {
            reason: "confidence below abstain threshold",
        });
    }
    if attempt.force && attempt.confidence < ABSTAIN_BELOW {
        // unreachable due to above, kept for clarity of law
        return Err(PredictionError::ForcedUncertainty);
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
