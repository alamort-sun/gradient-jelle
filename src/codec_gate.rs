//! Fail when a model output bypasses codec validation.
//!
//! Real gate (same idea as gradient-space-time `prepare_validated_payload`):
//! deserialize bytes → [`Vector15D::validate`]. A boolean flag is not proof.

use thiserror::Error;
use vecGradient::{Vector15D, Vector15DError};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CodecValidated {
    pub frame_id: u64,
    pub bytes: Vec<u8>,
}

#[derive(Debug, Clone)]
pub struct ModelOutput {
    pub raw: Vec<u8>,
    pub frame_id: Option<u64>,
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum CodecGateError {
    #[error("model output bypassed codec validation")]
    BypassedCodec,
    #[error("validated output missing frame id")]
    MissingFrameId,
    #[error("Vector15D::validate failed: {0}")]
    Validate(String),
}

/// Accept model output ONLY if `raw` deserializes to a Vector15D that validates.
/// Raw / unvalidated blobs are refused — the bypass path does not exist.
pub fn accept_model_output(out: &ModelOutput) -> Result<CodecValidated, CodecGateError> {
    let frame_id = out.frame_id.ok_or(CodecGateError::MissingFrameId)?;
    let state: Vector15D =
        serde_json::from_slice(&out.raw).map_err(|_| CodecGateError::BypassedCodec)?;
    state
        .validate()
        .map_err(|e: Vector15DError| CodecGateError::Validate(e.to_string()))?;
    Ok(CodecValidated {
        frame_id,
        bytes: out.raw.clone(),
    })
}

#[cfg(test)]
mod unit {
    use super::*;

    fn valid_raw() -> Vec<u8> {
        serde_json::to_vec(&Vector15D::default()).expect("serialize default Vector15D")
    }

    #[test]
    fn validated_passes() {
        let out = ModelOutput {
            raw: valid_raw(),
            frame_id: Some(7),
        };
        let ok = accept_model_output(&out).unwrap();
        assert_eq!(ok.frame_id, 7);
    }

    #[test]
    fn garbage_bytes_refused() {
        let out = ModelOutput {
            raw: b"not-a-vector".to_vec(),
            frame_id: Some(1),
        };
        assert_eq!(
            accept_model_output(&out).unwrap_err(),
            CodecGateError::BypassedCodec
        );
    }

    #[test]
    fn non_finite_refused_via_validate_mapping() {
        // serde_json cannot serialize NaN; exercise the Validate arm by calling
        // the same Vector15D::validate authority the gate uses, then assert the
        // gate refuses a shape-invalid payload (null amplitude) as BypassedCodec.
        let mut bad = Vector15D::default();
        bad.amplitude = f64::NAN;
        assert!(bad.validate().is_err());
        let mut v = serde_json::to_value(Vector15D::default()).unwrap();
        v["amplitude"] = serde_json::Value::Null;
        let out = ModelOutput {
            raw: serde_json::to_vec(&v).unwrap(),
            frame_id: Some(2),
        };
        assert_eq!(
            accept_model_output(&out).unwrap_err(),
            CodecGateError::BypassedCodec
        );
    }
}
