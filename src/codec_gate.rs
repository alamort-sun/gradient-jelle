//! Fail when a model output bypasses codec validation.

use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CodecValidated {
    pub frame_id: u64,
    pub bytes: Vec<u8>,
}

#[derive(Debug, Clone)]
pub struct ModelOutput {
    pub raw: Vec<u8>,
    /// True only if this blob already passed the codec validator.
    pub codec_validated: bool,
    pub frame_id: Option<u64>,
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum CodecGateError {
    #[error("model output bypassed codec validation")]
    BypassedCodec,
    #[error("validated output missing frame id")]
    MissingFrameId,
}

/// Accept model output ONLY if it carries proof of codec validation.
/// Raw / unvalidated blobs are refused — the bypass path does not exist.
pub fn accept_model_output(out: &ModelOutput) -> Result<CodecValidated, CodecGateError> {
    if !out.codec_validated {
        return Err(CodecGateError::BypassedCodec);
    }
    let frame_id = out.frame_id.ok_or(CodecGateError::MissingFrameId)?;
    Ok(CodecValidated {
        frame_id,
        bytes: out.raw.clone(),
    })
}

#[cfg(test)]
mod unit {
    use super::*;

    #[test]
    fn validated_passes() {
        let out = ModelOutput {
            raw: b"frame".to_vec(),
            codec_validated: true,
            frame_id: Some(7),
        };
        let ok = accept_model_output(&out).unwrap();
        assert_eq!(ok.frame_id, 7);
    }
}
