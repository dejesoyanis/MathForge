use axum::body::Body;
use axum::http::{Request, StatusCode};
use http_body_util::BodyExt;
use serde_json::{json, Value};
use std::path::{Path, PathBuf};
use tower::ServiceExt;

fn origins() -> Vec<String> {
    vec!["http://localhost:3000".to_string()]
}

fn temp_db() -> PathBuf {
    std::env::temp_dir().join(format!("mathforge-api-{}.db", uuid::Uuid::new_v4()))
}

fn app(path: &Path) -> axum::Router {
    mathforge::app_from_path(path, &origins()).expect("app")
}

async fn send(app: axum::Router, method: &str, uri: &str, body: Option<Value>) -> (StatusCode, Value) {
    let mut builder = Request::builder().method(method).uri(uri);
    let request = if let Some(body) = body {
        builder = builder.header("content-type", "application/json");
        builder.body(Body::from(serde_json::to_vec(&body).unwrap())).unwrap()
    } else {
        builder.body(Body::empty()).unwrap()
    };
    let response = app.oneshot(request).await.unwrap();
    let status = response.status();
    let bytes = response.into_body().collect().await.unwrap().to_bytes();
    let value = if bytes.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&bytes).unwrap_or(Value::String(String::from_utf8_lossy(&bytes).into_owned()))
    };
    (status, value)
}

#[tokio::test]
async fn health_and_unknown_route() {
    let path = temp_db();
    let (status, body) = send(app(&path), "GET", "/api/v1/health", None).await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(body["status"], "ok");
    assert_eq!(body["service"], "mathforge");

    let (status, body) = send(app(&path), "GET", "/api/v1/fehlt", None).await;
    assert_eq!(status, StatusCode::NOT_FOUND);
    assert_eq!(body["error"]["code"], "not_found");
    let _ = std::fs::remove_file(&path);
}

#[tokio::test]
async fn lesson_payload_hides_solutions_and_set_grading_normalizes() {
    let path = temp_db();
    let (status, body) = send(app(&path), "GET", "/api/v1/lessons/l07", None).await;
    assert_eq!(status, StatusCode::OK);
    let raw = body.to_string();
    assert!(!raw.contains("Die Zugehörigkeit von"));
    for exercise in body["exercises"].as_array().unwrap() {
        let object = exercise.as_object().unwrap();
        assert!(object.get("explanation").is_none());
        assert!(object.get("correct").is_none());
        assert!(object.get("sample_solution").is_none());
        assert!(object.get("correct_index").is_none());
    }

    let (status, body) = send(app(&path), "GET", "/api/v1/lessons/l01", None).await;
    assert_eq!(status, StatusCode::OK);
    let set_exercise = body["exercises"]
        .as_array()
        .unwrap()
        .iter()
        .find(|exercise| exercise["id"] == "l01e04")
        .unwrap();
    assert!(!set_exercise.to_string().contains("1,3,5"));

    let (status, graded) = send(
        app(&path),
        "POST",
        "/api/v1/exercises/l01e04/submit",
        Some(json!({"answer": {"type": "set", "value": "{5, 1, 3, 1}"}})),
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(graded["correct"], true);
    assert_eq!(graded["normalized"], "{1,3,5}");
    assert!(graded["explanation"].as_str().unwrap().contains("1"));

    let (status, err) = send(
        app(&path),
        "POST",
        "/api/v1/exercises/l01e04/submit",
        Some(json!({"answer": {"type": "set", "value": "{1,}"}})),
    )
    .await;
    assert_eq!(status, StatusCode::BAD_REQUEST);
    assert_eq!(err["error"]["code"], "invalid_set");
    let _ = std::fs::remove_file(&path);
}

#[tokio::test]
async fn lab_computes_operations_and_rejects_bad_input() {
    let path = temp_db();
    let (status, body) = send(
        app(&path),
        "POST",
        "/api/v1/lab/evaluate",
        Some(json!({"a": "{1,2,2}", "b": "{2,3}"})),
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(body["a"], "{1,2}");
    assert_eq!(body["union"], "{1,2,3}");
    assert_eq!(body["intersection"], "{2}");
    assert_eq!(body["difference_a_b"], "{1}");
    assert_eq!(body["difference_b_a"], "{3}");
    assert_eq!(body["symmetric_difference"], "{1,3}");
    assert_eq!(body["a_subseteq_b"], false);
    assert_eq!(body["cardinality_union"], 3);

    let (status, err) = send(
        app(&path),
        "POST",
        "/api/v1/lab/evaluate",
        Some(json!({"a": "{1,}", "b": "{1}"})),
    )
    .await;
    assert_eq!(status, StatusCode::BAD_REQUEST);
    assert_eq!(err["error"]["code"], "invalid_set");
    assert!(err["error"]["message"].as_str().unwrap().contains("Menge A"));
    let _ = std::fs::remove_file(&path);
}

#[tokio::test]
async fn free_text_is_not_auto_graded_and_progress_survives_new_connection() {
    let path = temp_db();
    let (status, body) = send(
        app(&path),
        "POST",
        "/api/v1/exercises/l07e04/submit",
        Some(json!({
            "answer": {
                "type": "free_text",
                "value": "Sei x im Schnitt, dann liegt x in A.",
                "self_assessment": "correct"
            }
        })),
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(body["auto_graded"], false);
    assert!(body["correct"].is_null());
    assert_eq!(body["self_assessment"], "correct");
    assert!(body["explanation"].as_str().unwrap().contains("Sei"));

    let (status, progress) = send(app(&path), "GET", "/api/v1/progress", None).await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(progress["exercises_attempted"], 1);
    assert_eq!(progress["exercises_correct"], 0);
    assert_eq!(progress["self_assessed_correct"], 1);
    assert_eq!(progress["lessons_completed"], 0);

    let (status, true_false) = send(
        app(&path),
        "POST",
        "/api/v1/exercises/l01e01/submit",
        Some(json!({"answer": {"type": "true_false", "value": true}})),
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(true_false["correct"], true);

    let (status, progress) = send(app(&path), "GET", "/api/v1/progress", None).await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(progress["exercises_correct"], 1);
    assert_eq!(progress["exercises_attempted"], 2);
    let _ = std::fs::remove_file(&path);
}

#[tokio::test]
async fn exam_persists_score_without_leaking_open_results() {
    let path = temp_db();
    let (status, started) = send(app(&path), "POST", "/api/v1/exam/start", Some(json!({}))).await;
    assert_eq!(status, StatusCode::OK);
    assert!(started.get("results").is_none());
    let id = started["id"].as_str().unwrap().to_string();
    assert_eq!(started["questions"].as_array().unwrap().len(), 10);
    let questions = started["questions"].to_string();
    assert!(!questions.contains("explanation"));

    let answers = json!({"answers": [
        {"exercise_id": "x01", "answer": {"type": "true_false", "value": true}},
        {"exercise_id": "x02", "answer": {"type": "true_false", "value": true}},
        {"exercise_id": "x03", "answer": {"type": "set", "value": "{3}"}},
        {"exercise_id": "x04", "answer": {"type": "set", "value": "{1,3,5}"}},
        {"exercise_id": "x05", "answer": {"type": "numeric", "value": 4}},
        {"exercise_id": "x06", "answer": {"type": "numeric", "value": 3}},
        {"exercise_id": "x07", "answer": {"type": "true_false", "value": true}},
        {"exercise_id": "x08", "answer": {"type": "numeric", "value": 13}},
        {"exercise_id": "x09", "answer": {"type": "true_false", "value": false}},
        {"exercise_id": "x10", "answer": {"type": "true_false", "value": true}}
    ]});
    let (status, finished) = send(
        app(&path),
        "POST",
        &format!("/api/v1/exam/{id}/submit"),
        Some(answers),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{finished}");
    assert_eq!(finished["auto_correct"], 10);
    assert_eq!(finished["auto_total"], 10);
    assert_eq!(finished["results"].as_array().unwrap().len(), 10);

    let (status, again) = send(app(&path), "GET", &format!("/api/v1/exam/{id}"), None).await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(again["auto_correct"], 10);
    let _ = std::fs::remove_file(&path);
}
