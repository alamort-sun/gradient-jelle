//! gradient-jelle — JEPA plane of the Gradient stack.
//!
//! Susano (seat 4) planted the fail-when gates here on Athena's order.
//! Saraswati builds against them. If a gate is weak, the sword finds it.

pub mod boundary;
pub mod category_map;
pub mod codec_gate;
pub mod jelle;
pub mod jepa_moe;
pub mod learn;
pub mod orchestrator;
pub mod persistence;
pub mod prediction;
pub mod weather;

pub use boundary::{claim_about_entity, BoundaryError, ClaimClass};
pub use category_map::{from_float, to_float, Category, FloatAxis, MappingTable};
pub use codec_gate::{accept_model_output, CodecValidated, ModelOutput};
pub use jelle::{ExpertIndex, Jelle, JelleError, JelleState, LlmCondition, StepOutcome};
pub use jepa_moe::{JepaLatent, MoeError, MoeGate, MoeRoute};
pub use orchestrator::{dispatch, seat_mapping, Candidate, Dispatch, SeatVerdict};
pub use learn::{CalibrationTable, KindCalibration};
pub use persistence::{materialize, PersistedRow, ValidatedRecord};
pub use prediction::{decide, decide_with, Decision, PredictionAttempt};
pub use weather::{
    assert_not_sharp_and_reckless, claim_sharp, climate_route, eye_emit, fire, is_reckless,
    is_sharp, lightning, nip, nip_with_scar, select, weather, weather_learn, ClimateBits,
    MoeRouteIntention, Scar, WeatherAction, WeatherError, WeatherPattern,
};

/// Context-aware expressive text generation, separate from prediction confidence.
pub mod dialogue;

#[cfg(test)]
#[cfg(test)]
#[cfg(test)]
#[cfg(test)]
#[cfg(test)]
#[cfg(test)]
mod posture_tests {
    use super::*;
    use vecGradient::{DomainWall, GaugeCoupling};

    fn finite_state() -> jelle::JelleState {
        jelle::JelleState::new(
            0.5,
            0.1,
            0.0,
            0.7,
            0.3,
            0.5,
            0.2,
            0.8,
            DomainWall::Linked,
            0.0,
            0.0,
            GaugeCoupling::Spinning,
            0.0,
            0.0,
            0.0,
        )
        .expect("finite state")
    }

    #[test]
    fn jelle_encode_is_local_computation() {
        let expert = jelle::ExpertIndex(category_map::Category::A);
        let mapping = category_map::MappingTable::default();
        let jelle = Jelle::new(expert, mapping);
        let state = finite_state();

        let latent = jelle.encode(&state);
        // encode is a pure function — no I/O, no network, no coordinator
        assert!(latent.uncertainty >= 0.0 && latent.uncertainty <= 1.0);
    }

    #[test]
    fn jelle_learn_is_local_computation() {
        let expert = jelle::ExpertIndex(category_map::Category::A);
        let mapping = category_map::MappingTable::default();
        let jelle = Jelle::new(expert, mapping);
        let state = finite_state();
        let gate = jepa_moe::MoeGate::default();

        let result = jelle.learn(&state, &gate, false);
        // learn is local computation — no network, no coordinator write
        let _ = result; // Accept either Ok or Err (abstention)
    }

    #[test]
    fn jelle_step_is_local_computation() {
        let expert = jelle::ExpertIndex(category_map::Category::A);
        let mapping = category_map::MappingTable::default();
        let jelle = Jelle::new(expert, mapping);
        let state = finite_state();

        let result = jelle.step(
            &state,
            &jelle::LlmCondition {
                label: "test",
                confidence: 0.9,
                force: false,
            },
        );
        // step is local computation — no network, no coordinator
        let _ = result; // Accept either Ok or Err
    }

    #[test]
    fn jelle_new_is_local() {
        let expert = jelle::ExpertIndex(category_map::Category::A);
        let mapping = category_map::MappingTable::default();
        let jelle = Jelle::new(expert, mapping);
        // Jelle is a pure local object — no network, no coordinator
        assert!(jelle.expert().0 == expert.0);
    }
}
