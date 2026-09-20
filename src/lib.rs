//! gradient-jelle — JEPA plane of the Gradient stack.
//!
//! Susano (seat 4) planted the fail-when gates here on Athena's order.
//! Saraswati builds against them. If a gate is weak, the sword finds it.

pub mod codec_gate;
pub mod persistence;
pub mod category_map;
pub mod prediction;
pub mod boundary;
pub mod jelle;
pub mod jepa_moe;

pub use codec_gate::{CodecValidated, ModelOutput, accept_model_output};
pub use jelle::{Jelle, JelleState, ExpertIndex, LlmCondition, StepOutcome, JelleError};
pub use persistence::{PersistedRow, ValidatedRecord, materialize};
pub use category_map::{Category, FloatAxis, MappingTable, to_float, from_float};
pub use prediction::{PredictionAttempt, Decision, decide};
pub use boundary::{ClaimClass, BoundaryError, claim_about_entity};
pub use jepa_moe::{JepaLatent, MoeError, MoeGate, MoeRoute};
