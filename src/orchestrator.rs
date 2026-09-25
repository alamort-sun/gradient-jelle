//! Morgana orchestrator — task dispatch through the governed pipeline.
//!
//! The broker question — "which seat takes this brief" — is a routing
//! decision, so it takes the same law as every other crossing here: a
//! candidate arrives as a codec state + a rule-layer fit, certainty is
//! earned from the latent, and an uncertain dispatch abstains rather than
//! misroutes.
//!
//! Layer split (the same shape as maren's deliberate path):
//!   caller = rule layer — capability match, budget legality, reachability.
//!            Produces `fit` in [0,1]. The gate never manufactures
//!            eligibility; it only refuses to speak without confidence.
//!   gate   = each candidate's seat state encodes to a JepaLatent; fit folds
//!            into certainty; every candidate crosses the MoeGate; the best
//!            routed certainty wins. All-abstain is a valid answer — the
//!            orchestrator asks instead of guessing.

use crate::category_map::{Category, MappingTable};
use crate::jepa_moe::{JepaLatent, MoeError, MoeGate, MoeRoute};
use crate::JelleState;

/// One routable seat: its codec state right now and its rule-layer fit.
#[derive(Debug, Clone, Copy)]
pub struct Candidate {
    /// Pantheon seat number — becomes the `Category::Seat` expert.
    pub seat: u8,
    /// The seat's persisted Vector15D state (registry carries it).
    pub state: JelleState,
    /// Rule-layer fit in [0,1]: capability match x budget legality x
    /// reachability, computed by the caller. `fit == 0` means ineligible —
    /// the gate reads it as full uncertainty.
    pub fit: f32,
}

/// Per-candidate gate outcome, kept for audit — the caller can see who the
/// gate refused and by how much.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SeatVerdict {
    pub seat: u8,
    pub certainty: f32,
    pub routed: bool,
}

/// The dispatch decision.
#[derive(Debug, Clone, PartialEq)]
pub enum Dispatch {
    /// A seat routed with the highest folded certainty.
    Routed {
        seat: u8,
        certainty: f32,
        verdicts: Vec<SeatVerdict>,
    },
    /// No candidate routed — the orchestrator abstains.
    Abstained { verdicts: Vec<SeatVerdict> },
}

/// Dispatch one brief across candidates through the MoE gate.
///
/// `force` propagates to the gate: forcing a route while every candidate is
/// uncertain is refused (`MoeError::ForcedCertainty`) — the same law as
/// prediction. A candidate missing from `mapping` fails the whole dispatch
/// via `MoeError::Mapping` — category gates fail loud, not silent.
pub fn dispatch(
    candidates: &[Candidate],
    gate: &MoeGate,
    mapping: &MappingTable,
    force: bool,
) -> Result<Dispatch, MoeError> {
    let mut verdicts = Vec::with_capacity(candidates.len());
    let mut best: Option<(u8, f32)> = None;
    for c in candidates {
        let latent = JepaLatent::encode(&c.state);
        // Fit folds into certainty: an ineligible or ill-fitting seat reads
        // as an uncertain route, not a banned one — the gate decides.
        let certainty = latent.certainty() * c.fit.clamp(0.0, 1.0);
        let effective = JepaLatent {
            code: latent.code,
            uncertainty: 1.0 - certainty,
        };
        match gate.route(&effective, Category::Seat(c.seat), mapping, force)? {
            MoeRoute::Routed { certainty, .. } => {
                verdicts.push(SeatVerdict {
                    seat: c.seat,
                    certainty,
                    routed: true,
                });
                if best.is_none_or(|(_, b)| certainty > b) {
                    best = Some((c.seat, certainty));
                }
            }
            MoeRoute::Abstained => verdicts.push(SeatVerdict {
                seat: c.seat,
                certainty,
                routed: false,
            }),
        }
    }
    Ok(match best {
        Some((seat, certainty)) => Dispatch::Routed {
            seat,
            certainty,
            verdicts,
        },
        None => Dispatch::Abstained { verdicts },
    })
}

/// The seat-axis mapping for a dispatch: every candidate seat defined on its
/// own float axis (seat number as float). Built from the candidate set so a
/// dispatch can never reference a seat that wasn't offered.
pub fn seat_mapping(candidates: &[Candidate]) -> MappingTable {
    let mut t = MappingTable::new();
    for c in candidates {
        t = t.define(Category::Seat(c.seat), c.seat as f32);
    }
    t
}

#[cfg(test)]
mod unit {
    use super::*;
    use vecGradient::{DomainWall, GaugeCoupling};

    fn state(coherence: f64) -> JelleState {
        JelleState::new(
            0.5,
            0.0,
            0.0,
            coherence,
            0.0,
            0.0,
            0.0,
            0.5,
            DomainWall::Linked,
            50.0,
            0.3,
            GaugeCoupling::Spinning,
            0.6,
            0.2,
            0.2,
        )
        .expect("finite")
    }

    #[test]
    fn clear_winner_routes() {
        let cands = [
            Candidate {
                seat: 4,
                state: state(0.9),
                fit: 1.0,
            },
            Candidate {
                seat: 13,
                state: state(0.2),
                fit: 0.3,
            },
        ];
        let map = seat_mapping(&cands);
        let d = dispatch(&cands, &MoeGate::default(), &map, false).expect("dispatch");
        match d {
            Dispatch::Routed {
                seat, verdicts, ..
            } => {
                assert_eq!(seat, 4);
                assert_eq!(verdicts.len(), 2);
            }
            Dispatch::Abstained { .. } => panic!("expected a route"),
        }
    }

    #[test]
    fn all_ineligible_abstains() {
        let cands = [
            Candidate {
                seat: 4,
                state: state(0.9),
                fit: 0.0, // unreachable — fit zero is full uncertainty
            },
            Candidate {
                seat: 12,
                state: state(0.0),
                fit: 1.0, // eligible but the seat's state is shattered
            },
        ];
        let map = seat_mapping(&cands);
        let d = dispatch(&cands, &MoeGate::default(), &map, false).expect("dispatch");
        assert!(matches!(d, Dispatch::Abstained { .. }));
    }

    #[test]
    fn forced_dispatch_on_uncertain_pool_refused() {
        let cands = [Candidate {
            seat: 9,
            state: state(0.0),
            fit: 1.0,
        }];
        let map = seat_mapping(&cands);
        let e = dispatch(&cands, &MoeGate::default(), &map, true).unwrap_err();
        assert!(matches!(e, MoeError::ForcedCertainty(_)));
    }

    #[test]
    fn unmapped_seat_fails_loud() {
        let cands = [Candidate {
            seat: 7,
            state: state(0.9),
            fit: 1.0,
        }];
        // mapping defines a different seat — the offered one can't resolve
        let map = MappingTable::new().define(Category::Seat(4), 4.0);
        let e = dispatch(&cands, &MoeGate::default(), &map, false).unwrap_err();
        assert!(matches!(e, MoeError::Mapping(_)));
    }
}
