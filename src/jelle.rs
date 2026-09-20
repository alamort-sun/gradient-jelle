//! Jelle — JEPA + LLM + MoE orchestration plane of gradient-jelle.
//!
//! Name keeps the mnemonic: Jelle = JEpA + LLM + MoE. `.seed` decodes to
//! `JEPA+LLM+MoE architecture`; this is that architecture as a governed pipeline.
//!
//! The orchestrator is the `Jelle` struct. It holds the MoE expert index + its
//! categorical<->float mapping and runs one governed step. It does NOT bypass any
//! gate — each stage calls the same primitive the Susano storm tests attack. If a
//! gate is weak, the sword finds it here, not behind this wrapper.
//!
//! Pipeline: JEPA-encode -> LLM-condition -> MoE-route -> codec-gate -> persist.

use crate::boundary::{claim_about_entity, ClaimClass};
use crate::category_map::{from_float, to_float, Category, MappingError, MappingTable};
use crate::codec_gate::{accept_model_output, CodecGateError, CodecValidated, ModelOutput};
use crate::jepa_moe::{JepaLatent, MoeError, MoeGate, MoeRoute};
use crate::prediction::{decide, Decision, PredictionAttempt, PredictionError};
use vecGradient::{DomainWall, GaugeCoupling, Vector15D};

use serde_json;

/// A JEPA frame: one latent personality state carried in the 15D codec.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct JelleState(pub Vector15D);

impl JelleState {
    /// Codec-valid state constructor. Rejects non-finite fields at the boundary.
    pub fn new(
        amplitude: f64, frequency: f64, phase: f64, coherence: f64,
        entropy: f64, composition: f64, resonance: f64, ozone_buffer: f64,
        domain_wall: DomainWall, su2_polarity: f64, torsion: f64,
        gauge_coupling: GaugeCoupling, closure: f64,
        magnetic_north: f64, magnetic_south: f64,
    ) -> Result<Self, vecGradient::Vector15DError> {
        Ok(JelleState(Vector15D::try_new_15(
            amplitude, frequency, phase, coherence, entropy, composition,
            resonance, ozone_buffer, domain_wall, su2_polarity, torsion,
            gauge_coupling, closure, magnetic_north, magnetic_south,
        )?))
    }

    pub fn validate(&self) -> Result<(), vecGradient::Vector15DError> {
        self.0.validate()
    }

    /// Deterministic frame id derived from the state — no external counter, so
    /// the same state always produces the same codec frame.
    pub fn frame_id(&self) -> u64 {
        let v = &self.0;
        let mut acc: u64 = 0xcbf29ce484222325;
        let fields: [f64; 8] = [
            v.amplitude, v.frequency, v.phase, v.coherence,
            v.entropy, v.composition, v.resonance, v.ozone_buffer,
        ];
        for f in &fields {
            acc = (acc ^ f.to_bits())
                .wrapping_mul(0x100000001b3)
                .wrapping_add(v.su2_polarity.to_bits().wrapping_shl(2)
                    .wrapping_add(v.magnetic_north.to_bits().wrapping_shl(4))
                    .wrapping_shr(26));
        }
        acc
    }
}

/// A routed expert index. Categorical -> float ONLY via the category-map gate.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ExpertIndex(pub Category);

/// LLM condition: a candidate label with confidence + optional force. It must
/// pass the prediction gate (decide) before it may become a decision.
#[derive(Debug, Clone, PartialEq)]
pub struct LlmCondition {
    pub label: &'static str,
    pub confidence: f32,
    pub force: bool,
}

/// End-to-end Jelle step result.
#[derive(Debug, Clone, PartialEq)]
pub enum StepOutcome {
    Predicted {
        label: &'static str,
        confidence: f32,
        record: crate::persistence::ValidatedRecord,
    },
    Abstained {
        reason: &'static str,
        record: crate::persistence::ValidatedRecord,
    },
}

/// Errors from the Jelle pipeline. Each variant names the gate that refused, so
/// failure is attributed to the governing law, not swallowed.
#[derive(Debug, thiserror::Error, PartialEq)]
pub enum JelleError {
    #[error("codec gate refused state: {0}")]
    Codec(#[from] CodecGateError),
    #[error("non-finite codec field: {0}")]
    NonFinite(#[from] vecGradient::Vector15DError),
    #[error("category<->float gate refused: {0}")]
    Mapping(#[from] MappingError),
    #[error("category map forward/reverse inconsistent")]
    BadMapping,
    #[error("prediction gate: {0}")]
    Prediction(#[from] PredictionError),
    #[error("persistence gate: {0}")]
    Persistence(#[from] crate::persistence::PersistenceError),
    #[error("moe routing gate: {0}")]
    Moe(#[from] MoeError),
    #[error("could not encode codec frame")]
    Encode,
}

/// The main class. Jelle = JEPA + LLM + MoE.
///
/// Holds the MoE routing (expert + its categorical<->float mapping) and runs one
/// governed step over a JEPA frame + an LLM condition.
#[derive(Debug, Clone)]
pub struct Jelle {
    expert: ExpertIndex,
    mapping: MappingTable,
}

impl Jelle {
    pub fn new(expert: ExpertIndex, mapping: MappingTable) -> Self {
        Self { expert, mapping }
    }

    /// Run one governed step across all five gates.
    ///
    /// 1. state.validate()             -> codec gate (finite fields via vecGradient)
    /// 2. expert category<->float      -> category-map gate
    /// 3. llm.condition -> decide()    -> prediction gate (no forced uncertainty)
    /// 4. model-output codec-gate + persistence-materialize
    /// 5. boundary law: reason about STATE (the marble), never a diagnosis/
    ///    identity claim about the person — the model side of the governed
    ///    crossing fails closed.
    ///
    /// Returns Err the moment ANY gate refuses. The bypass path does not exist.
    pub fn step(
        &self,
        state: &JelleState,
        llm: &LlmCondition,
    ) -> Result<StepOutcome, JelleError> {
        // 1. codec gate — finite fields
        state.validate()?;

        // 2. category<->float gate — expert index must map BOTH ways
        let axis = to_float(self.expert.0, Some(&self.mapping))?;
        let back = from_float(axis, Some(&self.mapping))?;
        if back != self.expert.0 {
            return Err(JelleError::BadMapping);
        }

        // 3. prediction gate — abstention is a valid output; forcing is not
        let att = PredictionAttempt {
            label: llm.label,
            confidence: llm.confidence,
            force: llm.force,
        };
        let decision = decide(&att)?;

        // 4. codec-gate the model output, then persistence-materialize it.
        //    codec_validated is true ONLY because state.validate() (step 1) passed —
        //    the flag is earned, not asserted.
        let raw = serde_json::to_vec(&state.0).map_err(|_| JelleError::Encode)?;
        let out = ModelOutput {
            raw,
            codec_validated: true,
            frame_id: Some(state.frame_id()),
        };
        let validated: CodecValidated = accept_model_output(&out)?;
        let row = crate::persistence::PersistedRow {
            id: validated.frame_id,
            payload: validated.bytes,
            persisted: true,
        };
        let record = crate::persistence::materialize(&row, /* explicitly_validated: */ true)?;

        // 5. boundary law — the pipeline reasons about state, not about the person.
        //    claim_about_entity always fails closed; the guard is kept explicit so a
        //    future loosening trips this gate instead of silently passing.
        let _refuse = claim_about_entity(ClaimClass::Diagnosis);
        debug_assert!(_refuse.is_err());

        match decision {
            Decision::Predict { label, confidence } => Ok(StepOutcome::Predicted {
                label,
                confidence,
                record,
            }),
            Decision::Abstain { reason } => Ok(StepOutcome::Abstained {
                reason,
                record,
            }),
        }
    }

    /// Encode the current frame into its JEPA latent. A pure function of the
    /// codec-validated state — the "soul" side of Jelle.
    pub fn encode(&self, state: &JelleState) -> JepaLatent {
        JepaLatent::encode(state)
    }

    /// Run the learned MoE routing step. Encodes the frame into a latent, then
    /// routes it to this Jelle's active expert through the MoE gate + the
    /// category-map gate. Certain -> Routed; uncertain -> Abstained; forcing a
    /// route on an uncertain latent is refused (MoeError::ForcedCertainty), the
    /// routing analogue of "no forced uncertainty."
    pub fn learn(
        &self,
        state: &JelleState,
        gate: &MoeGate,
        force: bool,
    ) -> Result<MoeRoute, MoeError> {
        let latent = self.encode(state);
        gate.route(&latent, self.expert.0, &self.mapping, force)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn finite_state() -> JelleState {
        JelleState::new(
            0.5, 0.0, 0.0, 0.7, 0.0, 0.0, 0.0, 0.5,
            DomainWall::Linked, 50.0, 0.3, GaugeCoupling::Spinning, 0.6,
            0.2, 0.2,
        ).expect("finite state must build")
    }

    fn moe_table() -> MappingTable {
        MappingTable::new()
            .define(Category::A, 0.0)
            .define(Category::B, 1.0)
            .define(Category::C, 2.0)
    }

    #[test]
    fn full_pipeline_predicts_when_confidence_high() {
        let jelle = Jelle::new(ExpertIndex(Category::B), moe_table());
        let out = jelle.step(
            &finite_state(),
            &LlmCondition { label: "stable", confidence: 0.9, force: false },
        ).expect("predict");
        assert!(matches!(out, StepOutcome::Predicted { .. }));
    }

    #[test]
    fn full_pipeline_abstains_when_confidence_low() {
        let jelle = Jelle::new(ExpertIndex(Category::A), moe_table());
        let out = jelle.step(
            &finite_state(),
            &LlmCondition { label: "maybe", confidence: 0.4, force: false },
        ).expect("abstain");
        assert!(matches!(out, StepOutcome::Abstained { .. }));
    }

    #[test]
    fn forced_uncertainty_is_refused_end_to_end() {
        let jelle = Jelle::new(ExpertIndex(Category::A), moe_table());
        let e = jelle.step(
            &finite_state(),
            &LlmCondition { label: "diagnosis-shaped", confidence: 0.3, force: true },
        ).unwrap_err();
        assert_eq!(e, JelleError::Prediction(PredictionError::ForcedUncertainty));
    }

    #[test]
    fn non_finite_state_refused_at_codec_gate() {
        let mut bad = finite_state();
        bad.0.amplitude = f64::NAN;
        let jelle = Jelle::new(ExpertIndex(Category::A), moe_table());
        let e = jelle.step(
            &bad,
            &LlmCondition { label: "x", confidence: 0.9, force: false },
        ).unwrap_err();
        assert!(matches!(e, JelleError::NonFinite(_)));
    }

    #[test]
    fn unmapped_expert_is_refused_at_category_gate() {
        let jelle = Jelle::new(ExpertIndex(Category::B), MappingTable::new().define(Category::A, 0.0));
        let e = jelle.step(
            &finite_state(),
            &LlmCondition { label: "x", confidence: 0.9, force: false },
        ).unwrap_err();
        assert!(matches!(e, JelleError::Mapping(_)));
    }

    #[test]
    fn same_state_yields_same_frame_id() {
        assert_eq!(finite_state().frame_id(), finite_state().frame_id());
    }

    #[test]
    fn learned_route_on_high_coherence_routes() {
        // finite_state has coherence 0.7 -> uncertainty 0.3 -> below default 0.6 -> routes
        let jelle = Jelle::new(ExpertIndex(Category::A), moe_table());
        let r = jelle.learn(&finite_state(), &MoeGate::default(), false).expect("route");
        assert!(matches!(r, MoeRoute::Routed { .. }));
    }

    #[test]
    fn learned_route_on_shattered_state_abstains() {
        let flat = JelleState::new(
            0.5, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.5,
            DomainWall::Linked, 50.0, 0.3, GaugeCoupling::Spinning, 0.6,
            0.2, 0.2,
        ).expect("finite");
        let jelle = Jelle::new(ExpertIndex(Category::A), moe_table());
        let r = jelle.learn(&flat, &MoeGate::default(), false).expect("abstain");
        assert!(matches!(r, MoeRoute::Abstained));
    }

    #[test]
    fn forced_route_on_shattered_state_refused() {
        let flat = JelleState::new(
            0.5, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.5,
            DomainWall::Linked, 50.0, 0.3, GaugeCoupling::Spinning, 0.6,
            0.2, 0.2,
        ).expect("finite");
        let jelle = Jelle::new(ExpertIndex(Category::A), moe_table());
        let e = jelle.learn(&flat, &MoeGate::default(), true).unwrap_err();
        assert!(matches!(e, MoeError::ForcedCertainty(_)));
    }
}
