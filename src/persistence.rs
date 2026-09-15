//! Fail when a database row is treated as valid by persistence alone.
//! Persistence does not confer validity.

use thiserror::Error;

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
}

/// Materialize a row for use as truth. A persisted row still requires an
/// explicit validity check — existence in the DB is not enough.
pub fn materialize(row: &PersistedRow, explicitly_validated: bool) -> Result<ValidatedRecord, PersistenceError> {
    if !row.persisted {
        return Err(PersistenceError::NotPersisted);
    }
    if !explicitly_validated {
        return Err(PersistenceError::PersistenceIsNotValidity);
    }
    Ok(ValidatedRecord {
        id: row.id,
        payload: row.payload.clone(),
    })
}

#[cfg(test)]
mod unit {
    use super::*;

    #[test]
    fn validated_persisted_row_ok() {
        let row = PersistedRow {
            id: 1,
            payload: b"x".to_vec(),
            persisted: true,
        };
        assert!(materialize(&row, true).is_ok());
    }
}
