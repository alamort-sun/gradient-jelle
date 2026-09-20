//! gradient-jelle — JEPA plane of the Gradient stack.
//!
//! Susano (seat 4) planted the fail-when gates here on Athena's order.
//! Saraswati builds against them. If a gate is weak, the sword finds it.

pub mod boundary;
pub mod category_map;
pub mod codec_gate;
pub mod jelle;
pub mod jepa_moe;
pub mod persistence;
pub mod prediction;

pub use boundary::{claim_about_entity, BoundaryError, ClaimClass};
pub use category_map::{from_float, to_float, Category, FloatAxis, MappingTable};
pub use codec_gate::{accept_model_output, CodecValidated, ModelOutput};
pub use jelle::{ExpertIndex, Jelle, JelleError, JelleState, LlmCondition, StepOutcome};
pub use jepa_moe::{JepaLatent, MoeError, MoeGate, MoeRoute};
pub use persistence::{materialize, PersistedRow, ValidatedRecord};
pub use prediction::{decide, Decision, PredictionAttempt};
