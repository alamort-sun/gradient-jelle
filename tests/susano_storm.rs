//! SUSANO STORM — Athena fail-when suite (seat 4).
//!
//! These tests must FAIL the *illegal* path (assert Err / abstain).
//! They are the sword Saraswati's builds must survive.

use gradient_jelle::boundary::{claim_about_entity, BoundaryError, ClaimClass};
use gradient_jelle::category_map::{from_float, to_float, Category, FloatAxis, MappingError, MappingTable};
use gradient_jelle::codec_gate::{accept_model_output, CodecGateError, ModelOutput};
use gradient_jelle::persistence::{materialize, PersistedRow, PersistenceError};
use gradient_jelle::prediction::{decide, Decision, PredictionAttempt, PredictionError, ABSTAIN_BELOW};
use vecGradient::Vector15D;

fn valid_vector_bytes() -> Vec<u8> {
    serde_json::to_vec(&Vector15D::default()).expect("serialize default Vector15D")
}

// -- 1. model output bypasses codec validation ---------------------------------

#[test]
fn fails_when_model_bypasses_codec_validation() {
    // Garbage bytes are not a Vector15D — boolean flags cannot paper over this.
    let bypass = ModelOutput {
        raw: b"looks-like-a-frame".to_vec(),
        frame_id: Some(1),
    };
    assert_eq!(
        accept_model_output(&bypass).unwrap_err(),
        CodecGateError::BypassedCodec
    );
}

#[test]
fn accepts_only_codec_validated_model_output() {
    let ok = ModelOutput {
        raw: valid_vector_bytes(),
        frame_id: Some(42),
    };
    let v = accept_model_output(&ok).unwrap();
    assert_eq!(v.frame_id, 42);
}

#[test]
fn fails_when_model_output_is_not_a_vector15d() {
    // Non-codec payload (including null/missing fields) must not pass the gate.
    let mut v = serde_json::to_value(Vector15D::default()).unwrap();
    v["amplitude"] = serde_json::Value::Null;
    let out = ModelOutput {
        raw: serde_json::to_vec(&v).unwrap(),
        frame_id: Some(3),
    };
    assert_eq!(
        accept_model_output(&out).unwrap_err(),
        CodecGateError::BypassedCodec
    );
    // And the codec authority itself still rejects non-finite in-memory states.
    let mut bad = Vector15D::default();
    bad.amplitude = f64::NAN;
    assert!(bad.validate().is_err());
}

// -- 2. persistence alone does not confer validity -----------------------------

#[test]
fn fails_when_persistence_alone_confers_validity() {
    // Row is in the DB, but payload is not a validated Vector15D.
    let row = PersistedRow {
        id: 9,
        payload: b"in-db".to_vec(),
        persisted: true,
    };
    assert_eq!(
        materialize(&row).unwrap_err(),
        PersistenceError::PersistenceIsNotValidity
    );
}

#[test]
fn persisted_and_codec_validated_may_materialize() {
    let row = PersistedRow {
        id: 9,
        payload: valid_vector_bytes(),
        persisted: true,
    };
    assert!(materialize(&row).is_ok());
}

// -- 3. categorical enums as ordered floats without mapping --------------------

#[test]
fn fails_when_categorical_cast_to_float_without_mapping() {
    assert_eq!(
        to_float(Category::A, None).unwrap_err(),
        MappingError::UndefinedMapping
    );
}

#[test]
fn fails_when_float_cast_to_categorical_without_mapping() {
    assert_eq!(
        from_float(FloatAxis(1.0), None).unwrap_err(),
        MappingError::UndefinedMapping
    );
}

#[test]
fn defined_mapping_is_required_bridge() {
    let table = MappingTable::new()
        .define(Category::A, 0.0)
        .define(Category::B, 1.0);
    assert_eq!(to_float(Category::B, Some(&table)).unwrap().0, 1.0);
    // Unmapped category still fails even with a table present.
    assert_eq!(
        to_float(Category::C, Some(&table)).unwrap_err(),
        MappingError::UnknownCategory
    );
}

// -- 4. uncertain predictions forced instead of abstained ----------------------

#[test]
fn fails_when_uncertain_prediction_is_forced() {
    let forced = PredictionAttempt {
        label: "diagnosis-shaped",
        confidence: ABSTAIN_BELOW - 0.1,
        force: true,
    };
    assert_eq!(
        decide(&forced).unwrap_err(),
        PredictionError::ForcedUncertainty
    );
}

#[test]
fn uncertain_prediction_abstains_instead_of_forcing() {
    let soft = PredictionAttempt {
        label: "maybe",
        confidence: ABSTAIN_BELOW - 0.1,
        force: false,
    };
    match decide(&soft).unwrap() {
        Decision::Abstain { reason } => assert!(!reason.is_empty()),
        other => panic!("expected Abstain, got {other:?}"),
    }
}

// -- pantheon: signal must not become diagnosis --------------------------------
// ClaimClass stays split from codec JoinClass (do not collapse into B4 tables).

#[test]
fn fails_when_signal_becomes_diagnosis() {
    assert_eq!(
        claim_about_entity(ClaimClass::Diagnosis),
        Err(BoundaryError::InsufficientBasisToInfer)
    );
}

#[test]
fn fails_when_signal_becomes_identity() {
    assert_eq!(
        claim_about_entity(ClaimClass::Identity),
        Err(BoundaryError::InsufficientBasisToInfer)
    );
}
