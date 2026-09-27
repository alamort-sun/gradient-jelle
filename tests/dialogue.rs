use gradient_jelle::{dialogue::*, JelleState};
struct Echo;
impl TextGenerator for Echo {
    fn generate(&mut self, prompt: &str) -> Result<String, DialogueError> {
        Ok(prompt.into())
    }
}
struct Failure;
impl TextGenerator for Failure {
    fn generate(&mut self, _: &str) -> Result<String, DialogueError> {
        Err(DialogueError::Backend("offline".into()))
    }
}
fn state() -> JelleState {
    JelleState(vecGradient::Vector15D::default())
}
#[test]
fn continuity_and_explicit_preferences_survive_restart() {
    let mut s = Session::default();
    s.remember("Our spider pet uses silk threads".into())
        .unwrap();
    s.reply(&state(), "hello Discordia", &mut Echo).unwrap();
    // History is not silently promoted to preferred examples.
    let data: serde_json::Value = serde_json::from_str(&s.to_json().unwrap()).unwrap();
    assert_eq!(data["preferred"].as_array().unwrap().len(), 0);
    s.prefer_last("The rhythm feels right".into()).unwrap();
    let restored = Session::from_json(&s.to_json().unwrap()).unwrap();
    let prompt = restored.prompt(&state(), "What were we making?").unwrap();
    assert!(prompt.contains("silk threads"));
    assert!(prompt.contains("The rhythm feels right"));
    assert!(prompt.contains("hello Discordia"));
}
#[test]
fn failures_are_transactional_and_invalid_geometry_never_reaches_backend() {
    let mut s = Session::default();
    let before = s.to_json().unwrap();
    assert!(s.reply(&state(), "hello", &mut Failure).is_err());
    assert_eq!(before, s.to_json().unwrap());
    let mut bad = state();
    bad.0.magnetic_south = f64::NAN;
    assert!(matches!(
        s.reply(&bad, "hello", &mut Echo),
        Err(DialogueError::Codec(_))
    ));
    assert_eq!(before, s.to_json().unwrap());
}
#[test]
fn design_controls_change_conditioning_without_losing_poles_or_claiming_truth() {
    let mut s = Session::default();
    s.set_creative_direction(CreativeDirection {
        design_mode: true,
        boldness: 0.95,
        colour_freedom: 0.93,
        alternatives: 4,
    })
    .unwrap();
    let mut v = state();
    v.0.magnetic_north = 0.34;
    v.0.magnetic_south = 0.78;
    let reply = s
        .reply(&v, "Design an avant-garde garden", &mut Echo)
        .unwrap();
    assert!(reply.text.contains("avant-garde"));
    assert!(reply.text.contains("hex colours"));
    assert!(reply.text.contains("\"alternatives\":4"));
    assert_eq!(reply.state.0, v.0);
    assert!(!reply.factual_accuracy_verified);
}
#[test]
fn corrupt_settings_rejected_without_mutation() {
    let mut s = Session::default();
    let before = s.to_json().unwrap();
    assert!(s.set_playfulness(f64::NAN).is_err());
    assert!(s
        .set_creative_direction(CreativeDirection {
            boldness: 2.0,
            ..Default::default()
        })
        .is_err());
    assert_eq!(before, s.to_json().unwrap());
    assert!(Session::from_json(&before.replace("\"version\": 1", "\"version\": 999")).is_err());
}
#[test]
fn forget_removes_user_data_but_preserves_preferences_controls() {
    let mut s = Session::default();
    s.remember("private detail".into()).unwrap();
    s.reply(&state(), "hello", &mut Echo).unwrap();
    s.prefer_last("nice".into()).unwrap();
    s.forget();
    let value: serde_json::Value = serde_json::from_str(&s.to_json().unwrap()).unwrap();
    for key in ["history", "notes", "preferred"] {
        assert!(value[key].as_array().unwrap().is_empty());
    }
    assert!(!s.to_json().unwrap().contains("private detail"));
}
#[test]
fn old_session_format_gets_creative_defaults() {
    let s = Session::from_json(
        r#"{"version":1,"playfulness":0.5,"history":[],"notes":[],"preferred":[]}"#,
    )
    .unwrap();
    assert_eq!(s.creative_direction(), CreativeDirection::default());
}
