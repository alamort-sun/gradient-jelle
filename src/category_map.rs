//! Fail when categorical enums are treated as ordered floats without a
//! defined mapping.

use std::collections::HashMap;
use thiserror::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Category {
    A,
    B,
    C,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FloatAxis(pub f32);

#[derive(Debug, Default, Clone)]
pub struct MappingTable {
    forward: HashMap<Category, f32>,
    reverse: Vec<(f32, Category)>,
}

impl MappingTable {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn define(mut self, cat: Category, value: f32) -> Self {
        self.forward.insert(cat, value);
        self.reverse.push((value, cat));
        self
    }
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum MappingError {
    #[error("no mapping defined for categorical ↔ float")]
    UndefinedMapping,
    #[error("category not in mapping table")]
    UnknownCategory,
    #[error("float has no categorical image in mapping table")]
    UnknownFloat,
}

/// Category → float ONLY through an explicit mapping table.
pub fn to_float(cat: Category, table: Option<&MappingTable>) -> Result<FloatAxis, MappingError> {
    let table = table.ok_or(MappingError::UndefinedMapping)?;
    let v = table
        .forward
        .get(&cat)
        .copied()
        .ok_or(MappingError::UnknownCategory)?;
    Ok(FloatAxis(v))
}

/// Float → category ONLY through an explicit mapping table (exact match).
pub fn from_float(axis: FloatAxis, table: Option<&MappingTable>) -> Result<Category, MappingError> {
    let table = table.ok_or(MappingError::UndefinedMapping)?;
    table
        .reverse
        .iter()
        .find(|(v, _)| (*v - axis.0).abs() < f32::EPSILON)
        .map(|(_, c)| *c)
        .ok_or(MappingError::UnknownFloat)
}

#[cfg(test)]
mod unit {
    use super::*;

    #[test]
    fn defined_mapping_works() {
        let t = MappingTable::new()
            .define(Category::A, 0.0)
            .define(Category::B, 1.0)
            .define(Category::C, 2.0);
        assert_eq!(to_float(Category::B, Some(&t)).unwrap().0, 1.0);
        assert_eq!(from_float(FloatAxis(2.0), Some(&t)).unwrap(), Category::C);
    }
}
