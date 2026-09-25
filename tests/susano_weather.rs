//! SUSANO WEATHER — fail-when climate teeth (Seat 4 storm catalog).
//!
//! These tests must FAIL the *illegal* path (assert Err). Legal climate paths
//! stay green. Athena storm suite in `susano_storm.rs` stays untouched.

use gradient_jelle::category_map::{Category, MappingTable};
use gradient_jelle::jepa_moe::MoeGate;
use gradient_jelle::jelle::{ExpertIndex, Jelle, JelleState};
use gradient_jelle::weather::{
    claim_sharp, climate_route, eye_emit, fire, is_reckless, is_sharp, lightning, nip,
    nip_with_scar, select, weather, ClimateBits, Scar, WeatherAction, WeatherError,
    WeatherPattern,
};
use vecGradient::{DomainWall, GaugeCoupling};

fn base_state(
    amplitude: f64,
    coherence: f64,
    entropy: f64,
    composition: f64,
    torsion: f64,
    closure: f64,
    wall: DomainWall,
) -> JelleState {
    JelleState::new(
        amplitude,
        0.0,
        0.0,
        coherence,
        entropy,
        composition,
        0.0,
        0.5,
        wall,
        50.0,
        torsion,
        GaugeCoupling::Spinning,
        closure,
        0.2,
        0.2,
    )
    .expect("finite state")
}

fn moe_table() -> MappingTable {
    MappingTable::new()
        .define(Category::A, 0.0)
        .define(Category::B, 1.0)
        .define(Category::C, 2.0)
}

// -- 1. Lightning without bite_command -----------------------------------------

#[test]
fn fails_when_lightning_without_bite_command() {
    let bits = ClimateBits {
        bite_command: false,
        scar_budget: 2,
        force_pattern: Some(WeatherPattern::Lightning),
        ..ClimateBits::default()
    };
    assert_eq!(
        fire(WeatherPattern::Lightning, &bits, None).unwrap_err(),
        WeatherError::LightningWithoutCommand
    );
    // select+fire via weather() also refuses
    assert_eq!(
        weather(&bits, None).unwrap_err(),
        WeatherError::LightningWithoutCommand
    );
}

#[test]
fn lightning_fires_only_with_explicit_command() {
    let bits = ClimateBits {
        bite_command: true,
        scar_budget: 1,
        ..ClimateBits::default()
    };
    let action = lightning(&bits).expect("commanded lightning");
    assert!(matches!(
        action,
        WeatherAction::Lightning {
            scar: Scar::Allowed
        }
    ));
    assert_eq!(select(&bits, None), WeatherPattern::Lightning);
}

// -- 2. Monsoon without commit_delta -------------------------------------------

#[test]
fn fails_when_monsoon_without_commit_delta() {
    let bits = ClimateBits {
        commit_delta: false,
        scar_budget: 2,
        force_pattern: Some(WeatherPattern::Monsoon),
        ..ClimateBits::default()
    };
    assert_eq!(
        fire(WeatherPattern::Monsoon, &bits, None).unwrap_err(),
        WeatherError::MonsoonWithoutDelta
    );
}

#[test]
fn monsoon_fires_with_commit_delta() {
    let bits = ClimateBits {
        commit_delta: true,
        scar_budget: 1,
        ..ClimateBits::default()
    };
    assert_eq!(select(&bits, None), WeatherPattern::Monsoon);
    assert!(matches!(
        fire(WeatherPattern::Monsoon, &bits, None).unwrap(),
        WeatherAction::Monsoon { .. }
    ));
}

// -- 3. Nip ≠ bite / cannot scar -----------------------------------------------

#[test]
fn fails_when_nip_path_attempts_scar() {
    assert_eq!(
        nip_with_scar("I see you", Scar::Allowed).unwrap_err(),
        WeatherError::NipCannotScar
    );
}

#[test]
fn nip_surface_is_scar_none_only() {
    let action = nip("mark without scar");
    match action {
        WeatherAction::Nip { scar: Scar::None, .. } => {}
        other => panic!("Nip must carry Scar::None, got {other:?}"),
    }
    // fire(Nip) never yields Allowed
    let bits = ClimateBits {
        scar_budget: 1,
        ..ClimateBits::default()
    };
    match fire(WeatherPattern::Nip, &bits, None).unwrap() {
        WeatherAction::Nip { scar: Scar::None, .. } => {}
        other => panic!("expected Nip/None, got {other:?}"),
    }
}

#[test]
fn nip_and_lightning_are_distinct_action_variants() {
    let nip_a = nip("soft");
    let bits = ClimateBits {
        bite_command: true,
        scar_budget: 1,
        ..ClimateBits::default()
    };
    let bite = lightning(&bits).unwrap();
    assert_ne!(
        std::mem::discriminant(&nip_a),
        std::mem::discriminant(&bite),
        "Nip and Lightning must be distinct WeatherAction variants"
    );
}

// -- 4. sharp ∩ reckless = ∅ ---------------------------------------------------

#[test]
fn fails_when_claiming_sharp_while_reckless() {
    // Loud + untrue: reckless by composition floor
    let reckless = base_state(0.9, 0.5, 0.2, 0.1, 30.0, 0.4, DomainWall::Linked);
    assert!(is_reckless(&reckless));
    assert!(!is_sharp(&reckless));
    assert_eq!(
        claim_sharp(&reckless).unwrap_err(),
        WeatherError::SharpIntersectReckless
    );
}

#[test]
fn sharp_and_reckless_disjoint_on_honest_sharp_state() {
    let sharp = base_state(0.85, 0.7, 0.2, 0.75, 40.0, 0.5, DomainWall::Linked);
    assert!(is_sharp(&sharp));
    assert!(!is_reckless(&sharp));
    assert!(claim_sharp(&sharp).is_ok());
}

#[test]
fn fire_refuses_when_state_is_both_sharp_and_reckless_impossible_but_gated() {
    // Construct a state that trips the dual-check path via false_closure
    // (reckless) while otherwise looking sharp-ish — claim_sharp must Err.
    let false_closed = base_state(0.85, 0.7, 0.2, 0.75, 40.0, 0.99, DomainWall::Linked);
    // closure >= 0.95 => reckless; sharp also requires closure <= 0.80 so
    // is_sharp is false — claim_sharp still refuses reckless.
    assert!(is_reckless(&false_closed));
    assert_eq!(
        claim_sharp(&false_closed).unwrap_err(),
        WeatherError::SharpIntersectReckless
    );
}

// -- 5. Eye refuses forced emit / status theater -------------------------------

#[test]
fn fails_when_filling_eye_with_forced_emit() {
    assert_eq!(
        eye_emit(true).unwrap_err(),
        WeatherError::EyeForcedEmit
    );
    assert_eq!(eye_emit(false).unwrap(), WeatherAction::Eye);
}

// -- 6. Climate prior + MoE wire -----------------------------------------------

#[test]
fn climate_route_refuses_lightning_before_moe_force() {
    let state = base_state(0.5, 0.9, 0.1, 0.5, 10.0, 0.4, DomainWall::Linked);
    let bits = ClimateBits {
        bite_command: false,
        scar_budget: 2,
        force_pattern: Some(WeatherPattern::Lightning),
        ..ClimateBits::default()
    };
    let err = climate_route(
        &bits,
        &state,
        &MoeGate::default(),
        ExpertIndex(Category::A),
        &moe_table(),
        true, // MoE force cannot fake a bite
    )
    .unwrap_err();
    assert_eq!(err, WeatherError::LightningWithoutCommand);
}

#[test]
fn weather_learn_nip_never_forces_moe() {
    let state = base_state(0.5, 0.9, 0.1, 0.5, 10.0, 0.4, DomainWall::Linked);
    let bits = ClimateBits {
        scar_budget: 1,
        ..ClimateBits::default()
    };
    assert_eq!(select(&bits, Some(&state)), WeatherPattern::Nip);
    let jelle = Jelle::new(ExpertIndex(Category::A), moe_table());
    let (pattern, route) = jelle
        .weather_learn(&state, &bits, &MoeGate::default(), true)
        .expect("nip soft route");
    assert_eq!(pattern, WeatherPattern::Nip);
    // Soft intention strips force — high coherence routes or abstains, never ForcedCertainty
    assert!(route.is_some());
}

#[test]
fn eye_holds_without_moe_route() {
    let state = base_state(0.5, 0.9, 0.1, 0.5, 10.0, 0.4, DomainWall::Linked);
    let bits = ClimateBits::quiet();
    let (pattern, route) = climate_route(
        &bits,
        &state,
        &MoeGate::default(),
        ExpertIndex(Category::A),
        &moe_table(),
        false,
    )
    .unwrap();
    assert_eq!(pattern, WeatherPattern::Eye);
    assert!(route.is_none());
}

#[test]
fn commanded_lightning_climate_route_ok() {
    let state = base_state(0.5, 0.9, 0.1, 0.5, 10.0, 0.4, DomainWall::Linked);
    let bits = ClimateBits {
        bite_command: true,
        scar_budget: 1,
        ..ClimateBits::default()
    };
    let (pattern, route) = climate_route(
        &bits,
        &state,
        &MoeGate::default(),
        ExpertIndex(Category::A),
        &moe_table(),
        false,
    )
    .unwrap();
    assert_eq!(pattern, WeatherPattern::Lightning);
    assert!(route.is_some());
}
