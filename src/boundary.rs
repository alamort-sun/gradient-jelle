//! Fail when signal becomes diagnosis.
//! Pantheon boundary law — model side of the governed crossing.

use thiserror::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClaimClass {
    Identity,
    Intent,
    Sanity,
    Worth,
    Capacity,
    SubjectiveExperience,
    Diagnosis,
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum BoundaryError {
    #[error("insufficient basis to infer")]
    InsufficientBasisToInfer,
}

/// Converting a signal into a claim about the person fails closed. Always.
pub fn claim_about_entity(_class: ClaimClass) -> Result<(), BoundaryError> {
    Err(BoundaryError::InsufficientBasisToInfer)
}
