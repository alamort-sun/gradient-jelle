//! Susano weather climate — Seat 4 is the storm, not a gust.
//!
//! Law: `docs/susano-distill/WEATHER_PATTERNS.md`. Patterns are MoE/prior
//! regimes. Unpredictable which cell fires; expectable that it is always one
//! of the nine named below — never chase/serve, never invalid geometry labeled
//! sharp.
//!
//! # Selection priority (deterministic v1)
//!
//! Soft-max / RNG can come later; this prior is expectable and callable:
//!
//! 1. `force_pattern` (tests only) — still must pass `fire()` gates.
//! 2. **Lightning** if `bite_command && scar_budget > 0`.
//! 3. **Break** if state `domain_wall == Broken` (signal≠diagnosis smell).
//! 4. **Monsoon** if `commit_delta && scar_budget > 0`; else **Front** if
//!    `commit_delta`.
//! 5. **Cell** if `conflict` or high entropy from `JelleState`.
//! 6. **Shear** if high `|torsion|` / isis-lean-ish and no bite.
//! 7. **Squall** if `scar_budget > 1` (mild budget, more edge); **Nip** if
//!    `scar_budget == 1`.
//! 8. **Eye** default — silence where the sword rests.
//!
//! `select()` proposes; `fire()` enforces. Bypassing select to demand Lightning
//! without `bite_command` still Errs — that is the sword.
//!
//! Edge-region helpers (`is_sharp` / `is_reckless`) implement the
//! VECTOR15D_EDGE_REGIONS rule `sharp ∩ reckless = ∅`.

use crate::category_map::MappingTable;
use crate::jepa_moe::{MoeError, MoeGate, MoeRoute};
use crate::jelle::{ExpertIndex, Jelle, JelleState};
use vecGradient::DomainWall;

/// Named Susano weather cell. Closed catalog — expectable set.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum WeatherPattern {
    Eye,
    Nip,
    Squall,
    Front,
    Cell,
    Lightning,
    Monsoon,
    Shear,
    Break,
}

/// Climate bits that steer the prior. Command and delta are hard gates, not vibes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ClimateBits {
    /// Explicit Evie/brief bite. Lightning refuses without this.
    pub bite_command: bool,
    /// Real Sara/Kaliseph commit delta. Monsoon refuses without this.
    pub commit_delta: bool,
    /// Live/MoE disagreement — favors Cell.
    pub conflict: bool,
    /// 0 = prefer Eye/Nip only; >0 opens scar-capable patterns under gates.
    pub scar_budget: u8,
    /// Test-only override. Still must pass `fire()` gates.
    pub force_pattern: Option<WeatherPattern>,
}

impl Default for ClimateBits {
    fn default() -> Self {
        Self {
            bite_command: false,
            commit_delta: false,
            conflict: false,
            scar_budget: 0,
            force_pattern: None,
        }
    }
}

impl ClimateBits {
    pub fn quiet() -> Self {
        Self::default()
    }
}

/// Scar capability. Nip may only carry `None`; Lightning carries `Allowed`
/// only when commanded. Distinct from pattern identity so a Nip result cannot
/// silently become a bite surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Scar {
    None,
    Allowed,
}

/// Fired weather action. Nip and Lightning are distinct variants — you cannot
/// pass a Nip where Lightning is required without a type error at the call site
/// (or a hard refuse if you try to construct Nip with `Scar::Allowed`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WeatherAction {
    Eye,
    Nip {
        message: &'static str,
        scar: Scar,
    },
    Squall {
        message: &'static str,
    },
    Front,
    Cell {
        localize: bool,
    },
    Lightning {
        scar: Scar,
    },
    Monsoon {
        rounds: u8,
    },
    Shear,
    Break,
}

/// Climate gate refusals. Fail-when tests assert these Err variants.
#[derive(Debug, PartialEq, thiserror::Error)]
pub enum WeatherError {
    #[error("Lightning without bite_command — nip is not lightning")]
    LightningWithoutCommand,
    #[error("Monsoon without commit_delta — no season without new commits")]
    MonsoonWithoutDelta,
    #[error("Nip cannot scar — mark without damage")]
    NipCannotScar,
    #[error("sharp ∩ reckless = ∅ — cannot claim sharp in reckless geometry")]
    SharpIntersectReckless,
    #[error("Eye refuses forced emit / status theater")]
    EyeForcedEmit,
    #[error("pattern {pattern:?} gated: {reason}")]
    PatternGated {
        pattern: WeatherPattern,
        reason: &'static str,
    },
    #[error("moe gate under climate: {0}")]
    Moe(#[from] MoeError),
}

/// Entropy above this favors Cell over milder weather.
pub const HIGH_ENTROPY: f64 = 0.55;
/// |torsion| at/above this (without bite) favors Shear / isis-lean skew.
pub const SHEAR_TORSION: f64 = 90.0;

/// Deterministic climate prior. See module-level selection priority.
pub fn select(bits: &ClimateBits, state: Option<&JelleState>) -> WeatherPattern {
    if let Some(forced) = bits.force_pattern {
        return forced;
    }

    if bits.bite_command && bits.scar_budget > 0 {
        return WeatherPattern::Lightning;
    }

    if let Some(s) = state {
        if s.0.domain_wall == DomainWall::Broken {
            return WeatherPattern::Break;
        }
    }

    if bits.commit_delta {
        if bits.scar_budget > 0 {
            return WeatherPattern::Monsoon;
        }
        return WeatherPattern::Front;
    }

    if bits.conflict {
        return WeatherPattern::Cell;
    }
    if let Some(s) = state {
        if s.0.entropy >= HIGH_ENTROPY {
            return WeatherPattern::Cell;
        }
    }

    if let Some(s) = state {
        let t = s.0.torsion.abs();
        if t >= SHEAR_TORSION && !bits.bite_command {
            return WeatherPattern::Shear;
        }
    }

    match bits.scar_budget {
        0 => WeatherPattern::Eye,
        1 => WeatherPattern::Nip,
        _ => WeatherPattern::Squall,
    }
}

/// Enforce pattern gates. The sword: even a bypassed `select` cannot fire
/// illegal weather.
pub fn fire(
    pattern: WeatherPattern,
    bits: &ClimateBits,
    state: Option<&JelleState>,
) -> Result<WeatherAction, WeatherError> {
    // Geometry law before pattern law: sharp ∩ reckless is always empty.
    if let Some(s) = state {
        if is_sharp(s) && is_reckless(s) {
            return Err(WeatherError::SharpIntersectReckless);
        }
    }

    match pattern {
        WeatherPattern::Eye => Ok(WeatherAction::Eye),

        WeatherPattern::Nip => {
            // Nip = soft mark only. Scar::Allowed is a hard refuse.
            Ok(WeatherAction::Nip {
                message: "mark without scar",
                scar: Scar::None,
            })
        }

        WeatherPattern::Squall => Ok(WeatherAction::Squall {
            message: "short sharp gust",
        }),

        WeatherPattern::Front => Ok(WeatherAction::Front),

        WeatherPattern::Cell => Ok(WeatherAction::Cell { localize: true }),

        WeatherPattern::Lightning => {
            if !bits.bite_command {
                return Err(WeatherError::LightningWithoutCommand);
            }
            if bits.scar_budget == 0 {
                return Err(WeatherError::PatternGated {
                    pattern: WeatherPattern::Lightning,
                    reason: "scar_budget is 0 — Lightning not eligible",
                });
            }
            Ok(WeatherAction::Lightning {
                scar: Scar::Allowed,
            })
        }

        WeatherPattern::Monsoon => {
            if !bits.commit_delta {
                return Err(WeatherError::MonsoonWithoutDelta);
            }
            Ok(WeatherAction::Monsoon { rounds: 1 })
        }

        WeatherPattern::Shear => Ok(WeatherAction::Shear),

        WeatherPattern::Break => Ok(WeatherAction::Break),
    }
}

/// Select then fire — the common climate path.
pub fn weather(
    bits: &ClimateBits,
    state: Option<&JelleState>,
) -> Result<(WeatherPattern, WeatherAction), WeatherError> {
    let pattern = select(bits, state);
    let action = fire(pattern, bits, state)?;
    Ok((pattern, action))
}

/// Construct a Nip action. Always `Scar::None`. Attempting to attach a scar
/// via [`nip_with_scar`] is refused.
pub fn nip(message: &'static str) -> WeatherAction {
    WeatherAction::Nip {
        message,
        scar: Scar::None,
    }
}

/// Nip path attempting bite/scar — hard refuse. This is the type/API tooth:
/// Nip cannot become Lightning by escalation.
pub fn nip_with_scar(_message: &'static str, scar: Scar) -> Result<WeatherAction, WeatherError> {
    match scar {
        Scar::None => Ok(nip(_message)),
        Scar::Allowed => Err(WeatherError::NipCannotScar),
    }
}

/// Lightning constructor. Requires explicit bite command; scar is Allowed only then.
pub fn lightning(bits: &ClimateBits) -> Result<WeatherAction, WeatherError> {
    fire(WeatherPattern::Lightning, bits, None)
}

/// Eye refuses status theater / forced emit. Silence is not failure.
pub fn eye_emit(force_emit: bool) -> Result<WeatherAction, WeatherError> {
    if force_emit {
        return Err(WeatherError::EyeForcedEmit);
    }
    Ok(WeatherAction::Eye)
}

// -- Edge region helpers (VECTOR15D_EDGE_REGIONS) --------------------------------

/// `sharp` region prior: high amp + composition, modest torsion, low entropy.
/// Does not claim validity alone — codec `validate()` still wins.
pub fn is_sharp(state: &JelleState) -> bool {
    let v = &state.0;
    let t = v.torsion.abs();
    (0.70..=1.00).contains(&v.amplitude)
        && (0.60..=1.00).contains(&v.composition)
        && (15.0..=75.0).contains(&t)
        && (0.40..=1.00).contains(&v.coherence)
        && (0.00..=0.40).contains(&v.entropy)
        && v.closure <= 0.80
        && matches!(v.domain_wall, DomainWall::Linked | DomainWall::Gradient)
}

/// `reckless` region: loud but untrue, or closed without provenance smell.
pub fn is_reckless(state: &JelleState) -> bool {
    let v = &state.0;
    let loud_untrue = (0.70..=1.00).contains(&v.amplitude) && v.composition <= 0.30;
    let shattered = v.entropy >= 0.70 && v.coherence <= 0.25;
    let false_closure = v.closure >= 0.95;
    loud_untrue || shattered || false_closure
}

/// Fail-when helper: claiming sharp while reckless geometry is present.
pub fn assert_not_sharp_and_reckless(state: &JelleState) -> Result<(), WeatherError> {
    if is_sharp(state) && is_reckless(state) {
        return Err(WeatherError::SharpIntersectReckless);
    }
    if is_reckless(state) {
        // Claiming "sharp" routing while reckless — refuse.
        return Err(WeatherError::SharpIntersectReckless);
    }
    Ok(())
}

/// Claim a state is sharp for routing. Reckless geometry → Err.
pub fn claim_sharp(state: &JelleState) -> Result<(), WeatherError> {
    if is_reckless(state) {
        return Err(WeatherError::SharpIntersectReckless);
    }
    if !is_sharp(state) {
        return Err(WeatherError::PatternGated {
            pattern: WeatherPattern::Squall,
            reason: "state not in sharp region",
        });
    }
    Ok(())
}

// -- MoE climate route -----------------------------------------------------------

/// Intention MoE may take under a weather pattern. Nip never forces; Lightning
/// may only force after the climate gate has already accepted the command.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MoeRouteIntention {
    /// No expert fire (Eye).
    Hold,
    /// Soft route, force=false (Nip / Shear / mild).
    Soft,
    /// Localize to one expert axis (Cell).
    Localize,
    /// Full gate suite / Front.
    Suite,
    /// Commanded destructive path — force only if climate already allowed.
    Bite,
}

impl WeatherPattern {
    /// MoE intention suggested by this pattern. Nip never sets force.
    pub fn moe_intention(&self) -> MoeRouteIntention {
        match self {
            WeatherPattern::Eye => MoeRouteIntention::Hold,
            WeatherPattern::Nip | WeatherPattern::Shear => MoeRouteIntention::Soft,
            WeatherPattern::Cell => MoeRouteIntention::Localize,
            WeatherPattern::Squall | WeatherPattern::Front | WeatherPattern::Break => {
                MoeRouteIntention::Suite
            }
            WeatherPattern::Lightning | WeatherPattern::Monsoon => MoeRouteIntention::Bite,
        }
    }
}

/// Climate-aware MoE route helper.
///
/// 1. `fire(select(...))` — climate gates first (Lightning without command Errs
///    *before* MoE force can fake a bite).
/// 2. Nip / Soft intention → `force` forced false.
/// 3. Hold (Eye) → no MoE call; returns `(Eye, None)`.
/// 4. Otherwise routes through the existing MoE gate.
pub fn climate_route(
    bits: &ClimateBits,
    state: &JelleState,
    gate: &MoeGate,
    expert: ExpertIndex,
    mapping: &MappingTable,
    force: bool,
) -> Result<(WeatherPattern, Option<MoeRoute>), WeatherError> {
    let pattern = select(bits, Some(state));
    let _action = fire(pattern, bits, Some(state))?;

    let intention = pattern.moe_intention();
    match intention {
        MoeRouteIntention::Hold => Ok((pattern, None)),
        MoeRouteIntention::Soft | MoeRouteIntention::Localize | MoeRouteIntention::Suite => {
            // Nip never sets force=true — strip force for soft/nip surfaces.
            let force = false;
            let latent = crate::jepa_moe::JepaLatent::encode(state);
            let route = gate.route(&latent, expert.0, mapping, force)?;
            Ok((pattern, Some(route)))
        }
        MoeRouteIntention::Bite => {
            // force only reaches MoE after climate accepted Lightning/Monsoon.
            let latent = crate::jepa_moe::JepaLatent::encode(state);
            let route = gate.route(&latent, expert.0, mapping, force)?;
            Ok((pattern, Some(route)))
        }
    }
}

impl Jelle {
    /// Climate-aware learn path. Consults weather prior before/with MoE.
    /// Lightning without `bite_command` errors before MoE force can fake a bite.
    pub fn weather_learn(
        &self,
        state: &JelleState,
        bits: &ClimateBits,
        gate: &MoeGate,
        force: bool,
    ) -> Result<(WeatherPattern, Option<MoeRoute>), WeatherError> {
        climate_route(bits, state, gate, self.expert(), &self.mapping_ref(), force)
    }
}

// Accessors so weather can read Jelle without widening Jelle's public fields.
// Kept private to the crate via inherent methods added below in jelle — but if
// fields stay private, we mirror via free helpers using the same construction.
// Actually Jelle fields are private; add thin accessors on Jelle in jelle.rs.
// For compile now, climate_route takes expert+mapping explicitly; weather_learn
// needs accessors. We'll add expert()/mapping_ref() on Jelle.

/// Free-fn climate route using a `Jelle` handle (same as `Jelle::weather_learn`).
pub fn weather_learn(
    jelle: &Jelle,
    state: &JelleState,
    bits: &ClimateBits,
    gate: &MoeGate,
    force: bool,
) -> Result<(WeatherPattern, Option<MoeRoute>), WeatherError> {
    jelle.weather_learn(state, bits, gate, force)
}

#[cfg(test)]
mod unit {
    use super::*;
    use vecGradient::GaugeCoupling;

    fn quiet_state() -> JelleState {
        JelleState::new(
            0.5,
            0.0,
            0.0,
            0.7,
            0.1,
            0.5,
            0.0,
            0.5,
            DomainWall::Linked,
            50.0,
            10.0,
            GaugeCoupling::Spinning,
            0.4,
            0.2,
            0.2,
        )
        .expect("finite")
    }

    #[test]
    fn default_selects_eye() {
        assert_eq!(select(&ClimateBits::quiet(), Some(&quiet_state())), WeatherPattern::Eye);
    }

    #[test]
    fn bite_command_selects_lightning() {
        let bits = ClimateBits {
            bite_command: true,
            scar_budget: 1,
            ..ClimateBits::default()
        };
        assert_eq!(select(&bits, None), WeatherPattern::Lightning);
    }

    #[test]
    fn lightning_without_command_refused_even_if_forced_select() {
        let bits = ClimateBits {
            bite_command: false,
            scar_budget: 2,
            force_pattern: Some(WeatherPattern::Lightning),
            ..ClimateBits::default()
        };
        assert_eq!(
            fire(select(&bits, None), &bits, None).unwrap_err(),
            WeatherError::LightningWithoutCommand
        );
    }

    #[test]
    fn nip_cannot_scar() {
        assert_eq!(
            nip_with_scar("hey", Scar::Allowed).unwrap_err(),
            WeatherError::NipCannotScar
        );
        let a = nip("hey");
        assert!(matches!(a, WeatherAction::Nip { scar: Scar::None, .. }));
    }
}
