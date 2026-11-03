use super::bad_json;
use super::dto::*;
use crate::content::{self, exercise_anywhere, lesson_by_id, lessons, practice_exercise, SCORE_NOTE};
use crate::db::{self, AppState, ExamRow, NewAttempt};
use crate::domain::{self, grade};
use crate::error::AppError;
use axum::extract::{Path, State};
use axum::extract::rejection::JsonRejection;
use axum::Json;
use std::collections::{HashMap, HashSet};
use std::sync::MutexGuard;
use rusqlite::Connection;

fn lock(state: &AppState) -> Result<MutexGuard<'_, Connection>, AppError> {
    state
        .db
        .lock()
        .map_err(|_| AppError::internal("Die Datenbanksperre ist unterbrochen."))
}

pub async fn health() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok",
        service: "mathforge",
        version: env!("CARGO_PKG_VERSION"),
    })
}

pub async fn fallback() -> AppError {
    AppError::not_found("not_found", "Dieser Endpunkt existiert nicht.")
}

pub async fn list_lessons() -> Json<LessonListResponse> {
    Json(LessonListResponse {
        content_version: content::CONTENT_VERSION,
        lessons: lessons().iter().copied().map(lesson_summary).collect(),
    })
}

pub async fn get_lesson(Path(id): Path<String>) -> Result<Json<LessonDetail>, AppError> {
    let lesson = lesson_by_id(&id).ok_or_else(|| {
        AppError::not_found("lesson_not_found", "Diese Lektion gibt es nicht.")
    })?;
    Ok(Json(lesson_detail(lesson)))
}

pub async fn get_progress(State(state): State<AppState>) -> Result<Json<ProgressResponse>, AppError> {
    let conn = lock(&state)?;
    Ok(Json(progress_from_db(&conn)?))
}

pub async fn reset_progress(State(state): State<AppState>) -> Result<Json<ResetResponse>, AppError> {
    let conn = lock(&state)?;
    db::reset_all(&conn)?;
    Ok(Json(ResetResponse { reset: true }))
}

pub async fn complete_lesson(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<CompleteResponse>, AppError> {
    let lesson = lesson_by_id(&id).ok_or_else(|| {
        AppError::not_found("lesson_not_found", "Diese Lektion gibt es nicht.")
    })?;
    let conn = lock(&state)?;
    let attempts = db::list_attempts(&conn)?;
    let done: HashSet<&str> = attempts.iter().map(|row| row.exercise_id.as_str()).collect();
    let missing: Vec<&str> = lesson
        .exercises
        .iter()
        .map(|exercise| exercise.id)
        .filter(|id| !done.contains(id))
        .collect();
    if !missing.is_empty() {
        return Err(AppError::bad_request(
            "lesson_incomplete",
            "Die Lektion kann erst abgeschlossen werden, wenn jede Übung mindestens einmal bearbeitet wurde.",
            Some(format!("Noch offen: {}", missing.join(", "))),
        ));
    }
    db::mark_lesson_complete(&conn, lesson.id)?;
    Ok(Json(CompleteResponse {
        lesson_id: lesson.id.to_string(),
        completed: true,
    }))
}

pub async fn submit_exercise(
    State(state): State<AppState>,
    Path(id): Path<String>,
    payload: Result<Json<SubmitRequest>, JsonRejection>,
) -> Result<Json<SubmitResponse>, AppError> {
    let Json(payload) = payload.map_err(bad_json)?;
    let exercise = practice_exercise(&id).ok_or_else(|| {
        AppError::not_found("exercise_not_found", "Diese Übung gibt es nicht.")
    })?;
    let answer = answer_to_domain(&payload.answer);
    let graded = grade(exercise, &answer).map_err(map_grade_error)?;
    let conn = lock(&state)?;
    let attempt_id = db::insert_attempt(
        &conn,
        &NewAttempt {
            exercise_id: exercise.id.to_string(),
            lesson_id: exercise.lesson_id.to_string(),
            answer_json: serde_json::to_string(&payload.answer).map_err(|_| {
                AppError::internal("Die Antwort konnte nicht gespeichert werden.")
            })?,
            auto_graded: graded.auto_graded,
            is_correct: graded.correct,
            self_assessment: graded.self_assessment.map(|value| value.as_str().to_string()),
            feedback: graded.feedback.clone(),
            normalized: graded.normalized.clone(),
            answer_display: graded.your_answer_display.clone(),
        },
    )?;
    Ok(Json(submit_response(exercise.id, attempt_id, &graded)))
}

pub async fn evaluate_lab(
    payload: Result<Json<LabRequest>, JsonRejection>,
) -> Result<Json<LabResponse>, AppError> {
    let Json(payload) = payload.map_err(bad_json)?;
    let a = domain::parse_int_set(&payload.a).map_err(|err| {
        AppError::bad_request(
            "invalid_set",
            format!("Menge A ist ungültig: {}", err.message),
            None,
        )
    })?;
    let b = domain::parse_int_set(&payload.b).map_err(|err| {
        AppError::bad_request(
            "invalid_set",
            format!("Menge B ist ungültig: {}", err.message),
            None,
        )
    })?;
    Ok(Json(lab_response(&a, &b)))
}

pub async fn start_exam(State(state): State<AppState>) -> Result<Json<ExamDetail>, AppError> {
    let conn = lock(&state)?;
    if let Some(existing) = db::latest_open_exam(&conn)? {
        return Ok(Json(exam_detail(&existing)?));
    }
    let id = uuid::Uuid::new_v4().to_string();
    db::insert_exam(&conn, &id)?;
    let created = db::exam_by_id(&conn, &id)?.ok_or_else(|| {
        AppError::internal("Der Prüfungsversuch wurde nicht gefunden.")
    })?;
    Ok(Json(exam_detail(&created)?))
}

pub async fn list_exam_attempts(
    State(state): State<AppState>,
) -> Result<Json<Vec<ExamAttemptSummary>>, AppError> {
    let conn = lock(&state)?;
    let rows = db::list_exams(&conn)?;
    Ok(Json(rows.iter().map(exam_summary).collect()))
}

pub async fn get_exam(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<ExamDetail>, AppError> {
    let conn = lock(&state)?;
    let row = db::exam_by_id(&conn, &id)?.ok_or_else(|| {
        AppError::not_found("exam_not_found", "Diesen Prüfungsversuch gibt es nicht.")
    })?;
    Ok(Json(exam_detail(&row)?))
}

pub async fn submit_exam(
    State(state): State<AppState>,
    Path(id): Path<String>,
    payload: Result<Json<ExamSubmitRequest>, JsonRejection>,
) -> Result<Json<ExamDetail>, AppError> {
    let Json(payload) = payload.map_err(bad_json)?;
    let conn = lock(&state)?;
    let row = db::exam_by_id(&conn, &id)?.ok_or_else(|| {
        AppError::not_found("exam_not_found", "Diesen Prüfungsversuch gibt es nicht.")
    })?;
    if row.completed_at.is_some() {
        return Err(AppError::conflict(
            "exam_closed",
            "Dieser Prüfungsversuch ist bereits abgeschlossen.",
        ));
    }

    let answers_json = serde_json::to_string(&payload.answers).map_err(|_| {
        AppError::internal("Die Abgabe konnte nicht gespeichert werden.")
    })?;
    let mut by_id: HashMap<String, AnswerBody> = HashMap::new();
    for item in payload.answers {
        if by_id.insert(item.exercise_id.clone(), item.answer).is_some() {
            return Err(AppError::bad_request(
                "duplicate_answer",
                format!("Für {} wurde mehr als eine Antwort gesendet.", item.exercise_id),
                None,
            ));
        }
    }

    let mut results = Vec::new();
    let mut correct = 0i64;
    let mut total = 0i64;
    for exercise in content::exam_exercises() {
        let Some(answer_body) = by_id.remove(exercise.id) else {
            return Err(AppError::bad_request(
                "missing_answer",
                format!("Die Antwort für {} fehlt.", exercise.id),
                None,
            ));
        };
        let graded = grade(exercise, &answer_to_domain(&answer_body)).map_err(map_grade_error)?;
        if graded.auto_graded {
            total += 1;
            if graded.correct == Some(true) {
                correct += 1;
            }
        }
        results.push(result_item(exercise.id, &graded));
    }
    if !by_id.is_empty() {
        return Err(AppError::bad_request(
            "unknown_exercise",
            "Die Abgabe enthält eine unbekannte Aufgaben-ID.",
            None,
        ));
    }

    let results_json = serde_json::to_string(&results).map_err(|_| {
        AppError::internal("Die Auswertung konnte nicht gespeichert werden.")
    })?;
    db::finish_exam(&conn, &id, correct, total, &answers_json, &results_json)?;
    let updated = db::exam_by_id(&conn, &id)?.ok_or_else(|| {
        AppError::internal("Der abgeschlossene Versuch wurde nicht gefunden.")
    })?;
    Ok(Json(exam_detail(&updated)?))
}

fn progress_from_db(conn: &Connection) -> Result<ProgressResponse, AppError> {
    let attempts = db::list_attempts(conn)?;
    let mut seen = HashSet::new();
    let mut latest = Vec::new();
    for row in &attempts {
        if seen.insert(row.exercise_id.clone()) {
            latest.push(row);
        }
    }
    let attempted_ids: HashSet<&str> = attempts.iter().map(|row| row.exercise_id.as_str()).collect();
    let completed = db::completed_lesson_ids(conn)?;
    let completed_at: HashMap<&str, &str> = completed
        .iter()
        .map(|(id, at)| (id.as_str(), at.as_str()))
        .collect();

    let mut lesson_items = Vec::new();
    let mut next_lesson_id = None;
    let mut exercises_total = 0;
    for lesson in lessons() {
        exercises_total += lesson.exercises.len();
        let attempted = lesson
            .exercises
            .iter()
            .filter(|exercise| attempted_ids.contains(exercise.id))
            .count();
        let correct = latest
            .iter()
            .filter(|row| row.lesson_id == lesson.id && row.auto_graded && row.is_correct == Some(true))
            .count();
        let completed_flag = completed_at.contains_key(lesson.id);
        if next_lesson_id.is_none() && !completed_flag {
            next_lesson_id = Some(lesson.id.to_string());
        }
        lesson_items.push(LessonProgressItem {
            lesson_id: lesson.id.to_string(),
            attempted,
            exercise_total: lesson.exercises.len(),
            correct,
            completed: completed_flag,
            completed_at: completed_at.get(lesson.id).map(|value| (*value).to_string()),
        });
    }

    let exercises_correct = latest
        .iter()
        .filter(|row| row.auto_graded && row.is_correct == Some(true))
        .count();
    let self_assessed_correct = latest
        .iter()
        .filter(|row| row.self_assessment.as_deref() == Some("correct"))
        .count();

    let mut latest_attempts = Vec::new();
    for row in latest {
        let Some(exercise) = exercise_anywhere(&row.exercise_id) else {
            continue;
        };
        if practice_exercise(&row.exercise_id).is_none() {
            continue;
        }
        let solution_latex = solution_latex_for(exercise);
        latest_attempts.push(LatestAttempt {
            exercise_id: row.exercise_id.clone(),
            lesson_id: row.lesson_id.clone(),
            auto_graded: row.auto_graded,
            correct: row.is_correct,
            self_assessment: row.self_assessment.clone(),
            feedback: row.feedback.clone(),
            normalized: row.normalized.clone(),
            answer_display: row.answer_display.clone(),
            explanation: exercise.explanation.to_string(),
            solution_latex,
            created_at: row.created_at.clone(),
        });
    }

    Ok(ProgressResponse {
        exercises_attempted: attempted_ids.len(),
        exercises_correct,
        self_assessed_correct,
        lessons_completed: completed.len(),
        lessons_total: lessons().len(),
        exercises_total,
        next_lesson_id,
        lessons: lesson_items,
        latest_attempts,
        score_note: PRACTICE_SCORE_NOTE,
    })
}

fn solution_latex_for(exercise: &content::Exercise) -> Option<String> {
    match exercise.kind {
        content::ExerciseKind::TrueFalse { correct } => Some(if correct {
            "\\text{wahr}".to_string()
        } else {
            "\\text{falsch}".to_string()
        }),
        content::ExerciseKind::Set { correct } => Some(crate::domain::IntSet::from_slice(correct).to_latex()),
        content::ExerciseKind::Numeric { correct } => Some(correct.to_string()),
        content::ExerciseKind::SingleChoice { .. } | content::ExerciseKind::FreeText { .. } => None,
    }
}

fn exam_summary(row: &ExamRow) -> ExamAttemptSummary {
    ExamAttemptSummary {
        id: row.id.clone(),
        started_at: row.started_at.clone(),
        completed_at: row.completed_at.clone(),
        auto_correct: row.auto_correct,
        auto_total: row.auto_total,
        status: if row.completed_at.is_some() {
            "abgeschlossen".to_string()
        } else {
            "offen".to_string()
        },
    }
}

fn exam_detail(row: &ExamRow) -> Result<ExamDetail, AppError> {
    let questions = content::exam_exercises()
        .iter()
        .map(|exercise| ExamQuestion {
            lesson_title: lesson_by_id(exercise.lesson_id)
                .map(|lesson| lesson.title.to_string())
                .unwrap_or_else(|| exercise.lesson_id.to_string()),
            exercise: public_exercise(exercise),
        })
        .collect();
    let results = if row.completed_at.is_some() {
        Some(parse_results(&row.results_json)?)
    } else {
        None
    };
    Ok(ExamDetail {
        id: row.id.clone(),
        started_at: row.started_at.clone(),
        completed_at: row.completed_at.clone(),
        status: if row.completed_at.is_some() {
            "abgeschlossen".to_string()
        } else {
            "offen".to_string()
        },
        auto_correct: row.auto_correct,
        auto_total: row.auto_total,
        score_note: SCORE_NOTE,
        questions,
        results,
    })
}

fn parse_results(raw: &str) -> Result<Vec<ExamResultItem>, AppError> {
    serde_json::from_str(raw).map_err(|_| AppError::internal("Die gespeicherte Auswertung ist beschädigt."))
}
