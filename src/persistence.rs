//! Fail when a database row is treated as valid by persistence alone.
//! Persistence does not confer validity — materialize requires Vector15D::validate.

use thiserror::Error;
use vecGradient::{Vector15D, Vector15DError};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PersistedRow {
    pub id: u64,
    pub payload: Vec<u8>,
    /// Row exists in storage. That is all this flag means.
    pub persisted: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidatedRecord {
    pub id: u64,
    pub payload: Vec<u8>,
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum PersistenceError {
    #[error("row is not persisted")]
    NotPersisted,
    #[error("persistence alone does not confer validity")]
    PersistenceIsNotValidity,
    #[error("Vector15D::validate failed: {0}")]
    Validate(String),
}

/// Materialize a row for use as truth.
///
/// Same idea as space-time `prepare_validated_payload`: deserialize payload →
/// [`Vector15D::validate`]. Existence in the DB is never enough.
pub fn materialize(row: &PersistedRow) -> Result<ValidatedRecord, PersistenceError> {
    if !row.persisted {
        return Err(PersistenceError::NotPersisted);
    }
    let state: Vector15D = serde_json::from_slice(&row.payload)
        .map_err(|_| PersistenceError::PersistenceIsNotValidity)?;
    state
        .validate()
        .map_err(|e: Vector15DError| PersistenceError::Validate(e.to_string()))?;
    Ok(ValidatedRecord {
        id: row.id,
        payload: row.payload.clone(),
    })
}

#[cfg(test)]
mod unit {
    use super::*;

    fn valid_payload() -> Vec<u8> {
        serde_json::to_vec(&Vector15D::default()).expect("serialize default Vector15D")
    }

    #[test]
    fn validated_persisted_row_ok() {
        let row = PersistedRow {
            id: 1,
            payload: valid_payload(),
            persisted: true,
        };
        assert!(materialize(&row).is_ok());
    }

    #[test]
    fn garbage_persisted_row_refused() {
        let row = PersistedRow {
            id: 1,
            payload: b"in-db".to_vec(),
            persisted: true,
        };
        assert_eq!(
            materialize(&row).unwrap_err(),
            PersistenceError::PersistenceIsNotValidity
        );
    }
}
