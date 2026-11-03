use crate::error::AppError;
use rusqlite::{params, Connection, OptionalExtension};
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::time::Duration;

#[derive(Clone)]
pub struct AppState {
    pub db: Arc<Mutex<Connection>>,
}

#[derive(Debug, Clone)]
pub struct AttemptRow {
    pub id: i64,
    pub exercise_id: String,
    pub lesson_id: String,
    pub answer_json: String,
    pub auto_graded: bool,
    pub is_correct: Option<bool>,
    pub self_assessment: Option<String>,
    pub feedback: String,
    pub normalized: Option<String>,
    pub answer_display: String,
    pub created_at: String,
}

#[derive(Debug, Clone)]
pub struct NewAttempt {
    pub exercise_id: String,
    pub lesson_id: String,
    pub answer_json: String,
    pub auto_graded: bool,
    pub is_correct: Option<bool>,
    pub self_assessment: Option<String>,
    pub feedback: String,
    pub normalized: Option<String>,
    pub answer_display: String,
}

#[derive(Debug, Clone)]
pub struct ExamRow {
    pub id: String,
    pub started_at: String,
    pub completed_at: Option<String>,
    pub auto_correct: Option<i64>,
    pub auto_total: Option<i64>,
    pub answers_json: String,
    pub results_json: String,
}

pub fn open(path: &Path) -> Result<Connection, AppError> {
    if let Some(parent) = path.parent() {
        if !parent.as_os_str().is_empty() {
            std::fs::create_dir_all(parent).map_err(|err| {
                AppError::internal(format!("Das Datenverzeichnis konnte nicht angelegt werden: {err}"))
            })?;
        }
    }
    let conn = Connection::open(path).map_err(db_err)?;
    prepare(&conn)?;
    Ok(conn)
}

pub fn open_in_memory() -> Result<Connection, AppError> {
    let conn = Connection::open_in_memory().map_err(db_err)?;
    prepare(&conn)?;
    Ok(conn)
}

fn prepare(conn: &Connection) -> Result<(), AppError> {
    conn.busy_timeout(Duration::from_secs(3)).map_err(db_err)?;
    conn.execute_batch(
        "
        PRAGMA foreign_keys = ON;
        CREATE TABLE IF NOT EXISTS exercise_attempts (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            exercise_id TEXT NOT NULL,
            lesson_id TEXT NOT NULL,
            answer_json TEXT NOT NULL,
            auto_graded INTEGER NOT NULL,
            is_correct INTEGER,
            self_assessment TEXT,
            feedback TEXT NOT NULL,
            normalized TEXT,
            answer_display TEXT NOT NULL,
            created_at TEXT NOT NULL
        );
        CREATE TABLE IF NOT EXISTS lesson_progress (
            lesson_id TEXT PRIMARY KEY,
            completed_at TEXT,
            updated_at TEXT NOT NULL
        );
        CREATE TABLE IF NOT EXISTS exam_attempts (
            id TEXT PRIMARY KEY,
            started_at TEXT NOT NULL,
            completed_at TEXT,
            auto_correct INTEGER,
            auto_total INTEGER,
            answers_json TEXT NOT NULL,
            results_json TEXT NOT NULL
        );
        ",
    )
    .map_err(db_err)
}

pub fn now_iso() -> String {
    chrono::Utc::now().to_rfc3339_opts(chrono::SecondsFormat::Secs, true)
}

pub fn insert_attempt(conn: &Connection, attempt: &NewAttempt) -> Result<i64, AppError> {
    conn.execute(
        "INSERT INTO exercise_attempts (
            exercise_id, lesson_id, answer_json, auto_graded, is_correct,
            self_assessment, feedback, normalized, answer_display, created_at
        ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
        params![
            attempt.exercise_id,
            attempt.lesson_id,
            attempt.answer_json,
            if attempt.auto_graded { 1 } else { 0 },
            attempt.is_correct.map(|value| if value { 1 } else { 0 }),
            attempt.self_assessment,
            attempt.feedback,
            attempt.normalized,
            attempt.answer_display,
            now_iso(),
        ],
    )
    .map_err(db_err)?;
    Ok(conn.last_insert_rowid())
}

pub fn list_attempts(conn: &Connection) -> Result<Vec<AttemptRow>, AppError> {
    let mut stmt = conn
        .prepare(
            "SELECT id, exercise_id, lesson_id, answer_json, auto_graded, is_correct,
                    self_assessment, feedback, normalized, answer_display, created_at
             FROM exercise_attempts
             ORDER BY id DESC",
        )
        .map_err(db_err)?;
    let rows = stmt
        .query_map([], |row| {
            Ok(AttemptRow {
                id: row.get(0)?,
                exercise_id: row.get(1)?,
                lesson_id: row.get(2)?,
                answer_json: row.get(3)?,
                auto_graded: row.get::<_, i64>(4)? != 0,
                is_correct: row.get::<_, Option<i64>>(5)?.map(|value| value != 0),
                self_assessment: row.get(6)?,
                feedback: row.get(7)?,
                normalized: row.get(8)?,
                answer_display: row.get(9)?,
                created_at: row.get(10)?,
            })
        })
        .map_err(db_err)?;
    rows.collect::<Result<Vec<_>, _>>().map_err(db_err)
}

pub fn completed_lesson_ids(conn: &Connection) -> Result<Vec<(String, String)>, AppError> {
    let mut stmt = conn
        .prepare(
            "SELECT lesson_id, completed_at FROM lesson_progress
             WHERE completed_at IS NOT NULL
             ORDER BY lesson_id",
        )
        .map_err(db_err)?;
    let rows = stmt
        .query_map([], |row| Ok((row.get(0)?, row.get(1)?)))
        .map_err(db_err)?;
    rows.collect::<Result<Vec<_>, _>>().map_err(db_err)
}

pub fn mark_lesson_complete(conn: &Connection, lesson_id: &str) -> Result<(), AppError> {
    let now = now_iso();
    conn.execute(
        "INSERT INTO lesson_progress (lesson_id, completed_at, updated_at)
         VALUES (?1, ?2, ?2)
         ON CONFLICT(lesson_id) DO UPDATE SET
            completed_at = excluded.completed_at,
            updated_at = excluded.updated_at",
        params![lesson_id, now],
    )
    .map_err(db_err)?;
    Ok(())
}

pub fn reset_all(conn: &Connection) -> Result<(), AppError> {
    let tx = conn.unchecked_transaction().map_err(db_err)?;
    tx.execute("DELETE FROM exercise_attempts", [])
        .map_err(db_err)?;
    tx.execute("DELETE FROM lesson_progress", [])
        .map_err(db_err)?;
    tx.execute("DELETE FROM exam_attempts", [])
        .map_err(db_err)?;
    tx.commit().map_err(db_err)?;
    Ok(())
}

pub fn insert_exam(conn: &Connection, id: &str) -> Result<(), AppError> {
    conn.execute(
        "INSERT INTO exam_attempts (id, started_at, completed_at, auto_correct, auto_total, answers_json, results_json)
         VALUES (?1, ?2, NULL, NULL, NULL, '[]', '[]')",
        params![id, now_iso()],
    )
    .map_err(db_err)?;
    Ok(())
}

pub fn finish_exam(
    conn: &Connection,
    id: &str,
    auto_correct: i64,
    auto_total: i64,
    answers_json: &str,
    results_json: &str,
) -> Result<(), AppError> {
    let changed = conn
        .execute(
            "UPDATE exam_attempts
             SET completed_at = ?2, auto_correct = ?3, auto_total = ?4, answers_json = ?5, results_json = ?6
             WHERE id = ?1 AND completed_at IS NULL",
            params![id, now_iso(), auto_correct, auto_total, answers_json, results_json],
        )
        .map_err(db_err)?;
    if changed == 0 {
        return Err(AppError::conflict(
            "exam_closed",
            "Dieser Prüfungsversuch ist bereits abgeschlossen oder existiert nicht.",
        ));
    }
    Ok(())
}

pub fn exam_by_id(conn: &Connection, id: &str) -> Result<Option<ExamRow>, AppError> {
    conn.query_row(
        "SELECT id, started_at, completed_at, auto_correct, auto_total, answers_json, results_json
         FROM exam_attempts WHERE id = ?1",
        params![id],
        map_exam,
    )
    .optional()
    .map_err(db_err)
}

pub fn list_exams(conn: &Connection) -> Result<Vec<ExamRow>, AppError> {
    let mut stmt = conn
        .prepare(
            "SELECT id, started_at, completed_at, auto_correct, auto_total, answers_json, results_json
             FROM exam_attempts
             ORDER BY started_at DESC, rowid DESC",
        )
        .map_err(db_err)?;
    let rows = stmt.query_map([], map_exam).map_err(db_err)?;
    rows.collect::<Result<Vec<_>, _>>().map_err(db_err)
}

pub fn latest_open_exam(conn: &Connection) -> Result<Option<ExamRow>, AppError> {
    conn.query_row(
        "SELECT id, started_at, completed_at, auto_correct, auto_total, answers_json, results_json
         FROM exam_attempts
         WHERE completed_at IS NULL
         ORDER BY started_at DESC, rowid DESC
         LIMIT 1",
        [],
        map_exam,
    )
    .optional()
    .map_err(db_err)
}

fn map_exam(row: &rusqlite::Row<'_>) -> rusqlite::Result<ExamRow> {
    Ok(ExamRow {
        id: row.get(0)?,
        started_at: row.get(1)?,
        completed_at: row.get(2)?,
        auto_correct: row.get(3)?,
        auto_total: row.get(4)?,
        answers_json: row.get(5)?,
        results_json: row.get(6)?,
    })
}

fn db_err(err: rusqlite::Error) -> AppError {
    eprintln!("sqlite: {err}");
    AppError::internal("Die Datenbankoperation ist fehlgeschlagen.")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn progress_and_exam_survive_reopen() {
        let path = std::env::temp_dir().join(format!("mathforge-{}-reopen.db", uuid::Uuid::new_v4()));
        {
            let conn = open(&path).unwrap();
            let id = insert_attempt(
                &conn,
                &NewAttempt {
                    exercise_id: "l01e01".to_string(),
                    lesson_id: "l01".to_string(),
                    answer_json: r#"{"type":"true_false","value":true}"#.to_string(),
                    auto_graded: true,
                    is_correct: Some(true),
                    self_assessment: None,
                    feedback: "Richtig.".to_string(),
                    normalized: None,
                    answer_display: "wahr".to_string(),
                },
            )
            .unwrap();
            assert!(id > 0);
            mark_lesson_complete(&conn, "l01").unwrap();
            insert_exam(&conn, "exam-1").unwrap();
            finish_exam(&conn, "exam-1", 8, 10, "[]", r#"[{"exercise_id":"x01"}]"#).unwrap();
        }
        {
            let conn = open(&path).unwrap();
            let attempts = list_attempts(&conn).unwrap();
            assert_eq!(attempts.len(), 1);
            assert_eq!(attempts[0].exercise_id, "l01e01");
            assert_eq!(attempts[0].is_correct, Some(true));
            let lessons = completed_lesson_ids(&conn).unwrap();
            assert_eq!(lessons.len(), 1);
            let exam = exam_by_id(&conn, "exam-1").unwrap().unwrap();
            assert_eq!(exam.auto_correct, Some(8));
            assert_eq!(exam.auto_total, Some(10));
            assert!(exam.completed_at.is_some());
        }
        let _ = std::fs::remove_file(&path);
    }
}
