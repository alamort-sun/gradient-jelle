use gradient_jelle::learn::{ArmStats, KindCalibration, THRESH_MAX, THRESH_MIN};
use gradient_jelle::prediction::{decide_with, Decision, PredictionAttempt, PredictionError};
use gradient_jelle::{
    dispatch, seat_mapping, CalibrationTable, Candidate, Category, JelleState, JepaLatent,
    MappingTable, MoeError, MoeGate, MoeRoute,
};

fn invalid_numbers() -> [f32; 5] {
    [f32::NAN, f32::INFINITY, f32::NEG_INFINITY, -0.1, 1.1]
}

#[test]
fn routing_rejects_invalid_numbers_even_when_forced() {
    let map = MappingTable::new().define(Category::A, 0.0);
    for force in [false, true] {
        for bad in invalid_numbers() {
            let latent = JepaLatent {
                code: 1.0,
                uncertainty: bad,
            };
            assert!(matches!(
                MoeGate::default().route(&latent, Category::A, &map, force),
                Err(MoeError::InvalidInput(_))
            ));
            let latent = JepaLatent {
                code: 1.0,
                uncertainty: 1.0,
            };
            assert!(matches!(
                MoeGate::new(bad).route(&latent, Category::A, &map, force),
                Err(MoeError::InvalidInput(_))
            ));
        }
        for code in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
            let latent = JepaLatent {
                code,
                uncertainty: 0.0,
            };
            assert!(matches!(
                MoeGate::default().route(&latent, Category::A, &map, force),
                Err(MoeError::InvalidInput(_))
            ));
        }
    }
}

#[test]
fn routing_boundary_still_abstains_at_threshold() {
    let map = MappingTable::new().define(Category::A, 0.0);
    for threshold in [0.0, 0.6, 1.0] {
        let latent = JepaLatent {
            code: 1.0,
            uncertainty: threshold,
        };
        assert_eq!(
            MoeGate::new(threshold)
                .route(&latent, Category::A, &map, false)
                .unwrap(),
            MoeRoute::Abstained
        );
    }
}

#[test]
fn dispatch_rejects_invalid_fit_and_state_before_encoding() {
    let state = JelleState(vecGradient::Vector15D::default());
    for fit in invalid_numbers() {
        let candidates = [Candidate {
            seat: 9,
            state,
            fit,
        }];
        assert!(matches!(
            dispatch(
                &candidates,
                &MoeGate::default(),
                &seat_mapping(&candidates),
                false
            ),
            Err(MoeError::InvalidInput(_))
        ));
    }
    let mut invalid = state;
    // This field is absent from the scalar encoder: validate the full state.
    invalid.0.magnetic_south = f64::NAN;
    let candidates = [Candidate {
        seat: 9,
        state: invalid,
        fit: 1.0,
    }];
    assert!(matches!(
        dispatch(
            &candidates,
            &MoeGate::default(),
            &seat_mapping(&candidates),
            false
        ),
        Err(MoeError::InvalidInput(_))
    ));
}

#[test]
fn prediction_rejects_bad_thresholds_and_preserves_endpoints() {
    for force in [false, true] {
        for threshold in invalid_numbers() {
            assert_eq!(
                decide_with(
                    &PredictionAttempt {
                        label: "test",
                        confidence: 1.0,
                        force
                    },
                    threshold
                ),
                Err(PredictionError::InvalidThreshold)
            );
        }
    }
    for confidence in [0.0, 1.0] {
        assert!(matches!(
            decide_with(
                &PredictionAttempt {
                    label: "test",
                    confidence,
                    force: false
                },
                confidence
            ),
            Ok(Decision::Predict { .. })
        ));
    }
}

#[test]
fn calibration_bounds_hold_for_new_restored_and_mutated_tables() {
    for base in invalid_numbers().into_iter().chain([0.0, 0.55, 1.0]) {
        let mut table = CalibrationTable::new(base);
        assert!((THRESH_MIN..=THRESH_MAX).contains(&table.base));
        // Public fields and deserialization must not bypass effective bounds.
        table.base = base;
        table.kinds.insert("few".into(), KindCalibration::default());
        for kind in ["new", "few"] {
            assert!((THRESH_MIN..=THRESH_MAX).contains(&table.effective_threshold(kind)));
        }
    }
    let restored: CalibrationTable = serde_json::from_str(r#"{"base":2.0,"kinds":{}}"#).unwrap();
    assert_eq!(restored.effective_threshold("new"), THRESH_MAX);
    let restored: CalibrationTable = serde_json::from_str(r#"{"base":0.55,"kinds":{"bad":{"spoke":{"trials":1,"resolved":2},"abstained":{"trials":0,"resolved":0}}}}"#).unwrap();
    assert_eq!(restored.effective_threshold("bad"), THRESH_MAX);
}

#[test]
fn large_calibration_counters_do_not_overflow() {
    let full = ArmStats {
        trials: u32::MAX,
        resolved: u32::MAX,
    };
    assert!(full.resolve_rate().is_finite());
    let mut table = CalibrationTable::new(0.55);
    table.kinds.insert(
        "full".into(),
        KindCalibration {
            spoke: full,
            abstained: full,
            pending: Some((0, true)),
        },
    );
    table.record_decision("full", true, 1000);
    assert!((THRESH_MIN..=THRESH_MAX).contains(&table.effective_threshold("full")));
    assert_eq!(table.kinds["full"].spoke.trials, u32::MAX);
}
