//! Jelle JEPA + MoE — the learned side of the orchestrator.
//!
//! Two concerns, both composing the locked gates rather than inventing methods
//! on them:
//!
//!   JEPA  = the *soul*. A deterministic latent personality code carried inside
//!           the codec state. `JepaLatent::encode` folds a codec-valid state
//!           into one scalar code; `.predict()` takes one predictable step;
//!           uncertainty is read straight off the codec's `coherence` field, so
//!           a coherent state is a low-uncertainty latent and a shattered state
//!           is a high-uncertainty one.
//!
//!   MoE   = the *routing gate*. Given a latent + the active expert's
//!           categorical<float image, it decides whether the latent is certain
//!           enough to route. An uncertain latent MUST abstain; forcing a route
//!           on an uncertain latent is refused — the routing analogue of the
//!           prediction law "no forced uncertainty."
//!
//! Neither function bypasses a gate. `route` re-enters the category-map gate
//! via `to_float`, so a bad mapping trips the same sword the storm tests do.

use crate::category_map::{to_float, Category, FloatAxis, MappingError, MappingTable};
use crate::JelleState;

/// Default MoE abstain threshold on latent uncertainty.
pub const MOE_THRESH_DEFAULT: f32 = 0.6;

/// A JEPA latent: one scalar personality code and the uncertainty the encoder
/// reports around it. A pure function of a codec-valid state.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct JepaLatent {
    pub code: f64,
    pub uncertainty: f32,
}

impl JepaLatent {
    /// Encode a codec state into a scalar latent.
    ///
    /// The code is a weighted, deterministic fold over the real fields the
    /// codec already validated (the same fields `frame_id` hashes), so two
    /// identical states always encode to an identical latent. `uncertainty` is
    /// read off `coherence`: a perfectly coherent state has zero latent
    /// uncertainty; a shattered one (coherence 0) has full.
    pub fn encode(state: &JelleState) -> JepaLatent {
        let v = &state.0;
        let fields: [f64; 10] = [
            v.amplitude, v.frequency, v.phase, v.coherence, v.entropy,
            v.composition, v.resonance, v.ozone_buffer, v.su2_polarity, v.magnetic_north,
        ];
        // Weighted sum over the validated fields — deterministic and bounded by
        // the field magnitudes the codec already admitted.
        let code: f64 = fields.iter().enumerate().map(|(i, f)| (i as f64 + 1.0) * f).sum();
        let uncertainty = (1.0 - v.coherence).clamp(0.0, 1.0) as f32;
        JepaLatent { code, uncertainty }
    }

    /// One-step JEPA prediction. A single step cannot *reduce* uncertainty, so
    /// it is carried forward; the next code drifts by the reported amount.
    pub fn predict(&self) -> JepaLatent {
        JepaLatent {
            code: self.code + self.uncertainty as f64,
            uncertainty: self.uncertainty,
        }
    }

    /// Routing confidence: the complement of latent uncertainty.
    pub fn certainty(&self) -> f32 {
        1.0 - self.uncertainty
    }
}

/// A MoE routing decision.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MoeRoute {
    /// The latent was certain enough to route to the active expert.
    Routed {
        expert: Category,
        axis: FloatAxis,
        certainty: f32,
    },
    /// The latent was uncertain; the pipeline should abstain instead of route.
    Abstained,
}

/// Errors from the MoE gate. Each names the gate that refused.
#[derive(Debug, PartialEq, thiserror::Error)]
pub enum MoeError {
    #[error("uncertain latent cannot be forced to route (uncertainty {0})")]
    ForcedCertainty(f32),
    #[error("category gate not satisfied inside MoE routing: {0}")]
    Mapping(#[from] MappingError),
}

/// The MoE routing gate. Routes a latent to the active expert's float axis ONLY
/// when the latent is certain enough; otherwise abstains (or refuses a forced
/// certainty).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MoeGate {
    pub threshold: f32,
}

impl Default for MoeGate {
    fn default() -> Self {
        MoeGate::new(MOE_THRESH_DEFAULT)
    }
}

impl MoeGate {
    pub fn new(threshold: f32) -> Self {
        MoeGate { threshold }
    }

    /// Route `latent` to `expert`, resolved through the category-map gate.
    ///
    /// Refuses (`forced` and uncertain => `ForcedCertainty`), abstains
    /// (uncertain, not forced), or routes (certain).
    pub fn route(
        &self,
        latent: &JepaLatent,
        expert: Category,
        mapping: &MappingTable,
        force: bool,
    ) -> Result<MoeRoute, MoeError> {
        // Re-enter the category-map gate: a bad mapping trips the same sword.
        let axis = to_float(expert, Some(mapping))?;
        if latent.uncertainty >= self.threshold {
            if force {
                return Err(MoeError::ForcedCertainty(latent.uncertainty));
            }
            return Ok(MoeRoute::Abstained);
        }
        Ok(MoeRoute::Routed {
            expert,
            axis,
            certainty: latent.certainty(),
        })
    }
}

#[cfg(test)]
mod unit {
    use super::*;

    #[test]
    fn encode_is_deterministic() {
        fn s() -> JelleState {
            use crate::JelleState;
            use vecGradient::{DomainWall, GaugeCoupling};
            JelleState::new(
                 0.5, 0.0, 0.0, 0.9, 0.0, 0.0, 0.0, 0.5,
                DomainWall::Linked, 50.0, 0.3, GaugeCoupling::Spinning, 0.6, 0.2, 0.2,
            ).expect("finite")
        }
        assert_eq!(JepaLatent::encode(&s()), JepaLatent::encode(&s()));
      }

     #[test]
    fn higher_coherence_lowers_uncertainty() {
        use vecGradient::{DomainWall, GaugeCoupling};
        #[derive(Clone, Copy)]
        struct P { s: f64 }
        fn mk(coh: f64) -> JelleState {
            JelleState::new(
                0.5, 0.0, 0.0, coh, 0.0, 0.0, 0.0, 0.5,
                DomainWall::Linked, 50.0, 0.3, GaugeCoupling::Spinning, 0.6, 0.2, 0.2,
            ).expect("finite")
        }
        assert!(JepaLatent::encode(&mk(0.9)).uncertainty < JepaLatent::encode(&mk(0.2)).uncertainty);
      }

     #[test]
    fn routes_on_certain_latent() {
        let t = MappingTable::new().define(Category::A, 0.0);
        let gate = MoeGate::new(0.6);
        let latent = JepaLatent { code: 1.0, uncertainty: 0.1 };
        assert!(matches!(gate.route(&latent, Category::A, &t, false).unwrap(), MoeRoute::Routed { .. }));
      }

     #[test]
    fn abstains_on_uncertain_latent() {
        let t = MappingTable::new().define(Category::A, 0.0);
        let gate = MoeGate::new(0.6);
        let latent = JepaLatent { code: 1.0, uncertainty: 0.7 };
        assert!(matches!(gate.route(&latent, Category::A, &t, false).unwrap(), MoeRoute::Abstained));
      }

     #[test]
    fn refuses_forced_certainty_on_uncertain_latent() {
        let t = MappingTable::new().define(Category::A, 0.0);
        let gate = MoeGate::new(0.6);
        let latent = JepaLatent { code: 1.0, uncertainty: 0.7 };
        assert!(gate.route(&latent, Category::A, &t, true).unwrap_err() == MoeError::ForcedCertainty(0.7));
      }
}
