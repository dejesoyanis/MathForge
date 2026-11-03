use crate::content::{Block, Exercise, ExerciseKind, Lesson};
use crate::domain::{GradeError, Graded, SelfAssessment, SubmittedAnswer};
use crate::domain::IntSet;
use crate::error::AppError;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SubmitRequest {
    pub answer: AnswerBody,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ExamSubmitRequest {
    pub answers: Vec<ExamAnswerInput>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ExamAnswerInput {
    pub exercise_id: String,
    pub answer: AnswerBody,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "type", rename_all = "snake_case", deny_unknown_fields)]
pub enum AnswerBody {
    TrueFalse { value: bool },
    SingleChoice { index: u32 },
    Set { value: String },
    Numeric { value: i64 },
    FreeText {
        value: String,
        #[serde(default)]
        self_assessment: Option<AssessmentBody>,
    },
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum AssessmentBody {
    Correct,
    Partial,
    Incorrect,
}

impl AssessmentBody {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Correct => "correct",
            Self::Partial => "partial",
            Self::Incorrect => "incorrect",
        }
    }

    fn to_domain(self) -> SelfAssessment {
        match self {
            Self::Correct => SelfAssessment::Correct,
            Self::Partial => SelfAssessment::Partial,
            Self::Incorrect => SelfAssessment::Incorrect,
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LabRequest {
    pub a: String,
    pub b: String,
}

#[derive(Serialize)]
pub struct HealthResponse {
    pub status: &'static str,
    pub service: &'static str,
    pub version: &'static str,
}

#[derive(Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum BlockDto {
    Heading { text: String },
    Paragraph { text: String },
    Math { latex: String },
    Bullets { items: Vec<String> },
}

#[derive(Serialize)]
pub struct ExampleDto {
    pub title: String,
    pub blocks: Vec<BlockDto>,
}

#[derive(Serialize)]
pub struct MisconceptionDto {
    pub title: String,
    pub text: String,
}

#[derive(Serialize)]
pub struct PublicExercise {
    pub id: String,
    pub lesson_id: String,
    pub difficulty: String,
    pub kind: String,
    pub prompt: String,
    pub hint: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<Vec<String>>,
}

#[derive(Serialize)]
pub struct LessonSummary {
    pub id: String,
    pub order: u8,
    pub title: String,
    pub description: String,
    pub difficulty: String,
    pub exercise_count: usize,
}

#[derive(Serialize)]
pub struct LessonDetail {
    pub id: String,
    pub order: u8,
    pub title: String,
    pub description: String,
    pub difficulty: String,
    pub content_version: &'static str,
    pub objectives: Vec<String>,
    pub theory: Vec<BlockDto>,
    pub examples: Vec<ExampleDto>,
    pub misconceptions: Vec<MisconceptionDto>,
    pub summary: Vec<BlockDto>,
    pub exercises: Vec<PublicExercise>,
}

#[derive(Serialize)]
pub struct LessonListResponse {
    pub content_version: &'static str,
    pub lessons: Vec<LessonSummary>,
}

#[derive(Serialize)]
pub struct SubmitResponse {
    pub exercise_id: String,
    pub attempt_id: i64,
    pub auto_graded: bool,
    pub correct: Option<bool>,
    pub feedback: String,
    pub normalized: Option<String>,
    pub solution_latex: Option<String>,
    pub explanation: String,
    pub self_assessment: Option<String>,
    pub your_answer_display: String,
}

#[derive(Serialize)]
pub struct LessonProgressItem {
    pub lesson_id: String,
    pub attempted: usize,
    pub exercise_total: usize,
    pub correct: usize,
    pub completed: bool,
    pub completed_at: Option<String>,
}

#[derive(Serialize)]
pub struct LatestAttempt {
    pub exercise_id: String,
    pub lesson_id: String,
    pub auto_graded: bool,
    pub correct: Option<bool>,
    pub self_assessment: Option<String>,
    pub feedback: String,
    pub normalized: Option<String>,
    pub answer_display: String,
    pub explanation: String,
    pub solution_latex: Option<String>,
    pub created_at: String,
}

#[derive(Serialize)]
pub struct ProgressResponse {
    pub exercises_attempted: usize,
    pub exercises_correct: usize,
    pub self_assessed_correct: usize,
    pub lessons_completed: usize,
    pub lessons_total: usize,
    pub exercises_total: usize,
    pub next_lesson_id: Option<String>,
    pub lessons: Vec<LessonProgressItem>,
    pub latest_attempts: Vec<LatestAttempt>,
    pub score_note: &'static str,
}

pub const PRACTICE_SCORE_NOTE: &str = "«Richtige Antworten» zählt Übungen, deren letzter automatisch bewerteter Versuch korrekt ist. Freitext- und Beweisaufgaben werden nie automatisch als korrekt gezählt. Eine Selbsteinschätzung erscheint separat und verändert diese Zahl nicht. Es werden keine Prozentwerte ausgegeben.";

#[derive(Serialize)]
pub struct LabResponse {
    pub a: String,
    pub b: String,
    pub a_latex: String,
    pub b_latex: String,
    pub union: String,
    pub union_latex: String,
    pub intersection: String,
    pub intersection_latex: String,
    pub difference_a_b: String,
    pub difference_a_b_latex: String,
    pub difference_b_a: String,
    pub difference_b_a_latex: String,
    pub symmetric_difference: String,
    pub symmetric_difference_latex: String,
    pub a_subseteq_b: bool,
    pub b_subseteq_a: bool,
    pub a_subsetneq_b: bool,
    pub b_subsetneq_a: bool,
    pub equal: bool,
    pub cardinality_a: usize,
    pub cardinality_b: usize,
    pub cardinality_union: usize,
    pub cardinality_intersection: usize,
    pub cardinality_difference_a_b: usize,
    pub cardinality_difference_b_a: usize,
    pub cardinality_symmetric_difference: usize,
}

#[derive(Serialize)]
pub struct ExamQuestion {
    pub exercise: PublicExercise,
    pub lesson_title: String,
}

#[derive(Serialize, Deserialize)]
pub struct ExamResultItem {
    pub exercise_id: String,
    pub auto_graded: bool,
    pub correct: Option<bool>,
    pub feedback: String,
    pub explanation: String,
    pub solution_latex: Option<String>,
    pub normalized: Option<String>,
    pub your_answer_display: String,
}

#[derive(Serialize)]
pub struct ExamAttemptSummary {
    pub id: String,
    pub started_at: String,
    pub completed_at: Option<String>,
    pub auto_correct: Option<i64>,
    pub auto_total: Option<i64>,
    pub status: String,
}

#[derive(Serialize)]
pub struct ExamDetail {
    pub id: String,
    pub started_at: String,
    pub completed_at: Option<String>,
    pub status: String,
    pub auto_correct: Option<i64>,
    pub auto_total: Option<i64>,
    pub score_note: &'static str,
    pub questions: Vec<ExamQuestion>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<ExamResultItem>>,
}

#[derive(Serialize)]
pub struct ResetResponse {
    pub reset: bool,
}

#[derive(Serialize)]
pub struct CompleteResponse {
    pub lesson_id: String,
    pub completed: bool,
}

pub fn answer_to_domain(answer: &AnswerBody) -> SubmittedAnswer {
    match answer {
        AnswerBody::TrueFalse { value } => SubmittedAnswer::TrueFalse { value: *value },
        AnswerBody::SingleChoice { index } => SubmittedAnswer::SingleChoice { index: *index as usize },
        AnswerBody::Set { value } => SubmittedAnswer::Set { raw: value.clone() },
        AnswerBody::Numeric { value } => SubmittedAnswer::Numeric { value: *value },
        AnswerBody::FreeText { value, self_assessment } => SubmittedAnswer::FreeText {
            text: value.clone(),
            self_assessment: self_assessment.map(AssessmentBody::to_domain),
        },
    }
}

pub fn map_grade_error(err: GradeError) -> AppError {
    match err {
        GradeError::TypeMismatch => AppError::bad_request(
            "wrong_answer_type",
            "Der Antworttyp passt nicht zu dieser Aufgabe.",
            None,
        ),
        GradeError::ChoiceOutOfRange => AppError::bad_request(
            "choice_out_of_range",
            "Die gewählte Option existiert nicht.",
            None,
        ),
        GradeError::EmptyFreeText => AppError::bad_request(
            "empty_text",
            "Die Begründung darf nicht leer sein.",
            None,
        ),
        GradeError::FreeTextTooLong => AppError::bad_request(
            "text_too_long",
            "Die Begründung darf höchstens 5000 Zeichen lang sein.",
            None,
        ),
        GradeError::InvalidSet { message } => AppError::bad_request("invalid_set", message, None),
    }
}

pub fn submit_response(exercise_id: &str, attempt_id: i64, graded: &Graded) -> SubmitResponse {
    SubmitResponse {
        exercise_id: exercise_id.to_string(),
        attempt_id,
        auto_graded: graded.auto_graded,
        correct: graded.correct,
        feedback: graded.feedback.clone(),
        normalized: graded.normalized.clone(),
        solution_latex: graded.solution_latex.clone(),
        explanation: graded.explanation.clone(),
        self_assessment: graded.self_assessment.map(SelfAssessment::as_str).map(str::to_string),
        your_answer_display: graded.your_answer_display.clone(),
    }
}

pub fn result_item(exercise_id: &str, graded: &Graded) -> ExamResultItem {
    ExamResultItem {
        exercise_id: exercise_id.to_string(),
        auto_graded: graded.auto_graded,
        correct: graded.correct,
        feedback: graded.feedback.clone(),
        explanation: graded.explanation.clone(),
        solution_latex: graded.solution_latex.clone(),
        normalized: graded.normalized.clone(),
        your_answer_display: graded.your_answer_display.clone(),
    }
}

pub fn public_exercise(exercise: &Exercise) -> PublicExercise {
    let options = match exercise.kind {
        ExerciseKind::SingleChoice { options, .. } => {
            Some(options.iter().map(|item| (*item).to_string()).collect())
        }
        _ => None,
    };
    PublicExercise {
        id: exercise.id.to_string(),
        lesson_id: exercise.lesson_id.to_string(),
        difficulty: exercise.difficulty.as_str().to_string(),
        kind: exercise.kind.name().to_string(),
        prompt: exercise.prompt.to_string(),
        hint: exercise.hint.to_string(),
        options,
    }
}

pub fn lesson_summary(lesson: &Lesson) -> LessonSummary {
    LessonSummary {
        id: lesson.id.to_string(),
        order: lesson.order,
        title: lesson.title.to_string(),
        description: lesson.description.to_string(),
        difficulty: lesson.difficulty.as_str().to_string(),
        exercise_count: lesson.exercises.len(),
    }
}

pub fn lesson_detail(lesson: &Lesson) -> LessonDetail {
    LessonDetail {
        id: lesson.id.to_string(),
        order: lesson.order,
        title: lesson.title.to_string(),
        description: lesson.description.to_string(),
        difficulty: lesson.difficulty.as_str().to_string(),
        content_version: crate::content::CONTENT_VERSION,
        objectives: lesson.objectives.iter().map(|item| (*item).to_string()).collect(),
        theory: lesson.theory.iter().map(block_dto).collect(),
        examples: lesson
            .examples
            .iter()
            .map(|example| ExampleDto {
                title: example.title.to_string(),
                blocks: example.blocks.iter().map(block_dto).collect(),
            })
            .collect(),
        misconceptions: lesson
            .misconceptions
            .iter()
            .map(|item| MisconceptionDto {
                title: item.title.to_string(),
                text: item.text.to_string(),
            })
            .collect(),
        summary: lesson.summary.iter().map(block_dto).collect(),
        exercises: lesson.exercises.iter().map(public_exercise).collect(),
    }
}

fn block_dto(block: &Block) -> BlockDto {
    match *block {
        Block::Heading(text) => BlockDto::Heading { text: text.to_string() },
        Block::Paragraph(text) => BlockDto::Paragraph { text: text.to_string() },
        Block::Math(latex) => BlockDto::Math { latex: latex.to_string() },
        Block::Bullets(items) => BlockDto::Bullets {
            items: items.iter().map(|item| (*item).to_string()).collect(),
        },
    }
}

pub fn lab_response(a: &IntSet, b: &IntSet) -> LabResponse {
    let union = a.union(b);
    let intersection = a.intersection(b);
    let difference_a_b = a.difference(b);
    let difference_b_a = b.difference(a);
    let symmetric = a.symmetric_difference(b);
    LabResponse {
        a: a.to_canonical(),
        b: b.to_canonical(),
        a_latex: a.to_latex(),
        b_latex: b.to_latex(),
        union: union.to_canonical(),
        union_latex: union.to_latex(),
        intersection: intersection.to_canonical(),
        intersection_latex: intersection.to_latex(),
        difference_a_b: difference_a_b.to_canonical(),
        difference_a_b_latex: difference_a_b.to_latex(),
        difference_b_a: difference_b_a.to_canonical(),
        difference_b_a_latex: difference_b_a.to_latex(),
        symmetric_difference: symmetric.to_canonical(),
        symmetric_difference_latex: symmetric.to_latex(),
        a_subseteq_b: a.is_subset(b),
        b_subseteq_a: b.is_subset(a),
        a_subsetneq_b: a.is_proper_subset(b),
        b_subsetneq_a: b.is_proper_subset(a),
        equal: a == b,
        cardinality_a: a.len(),
        cardinality_b: b.len(),
        cardinality_union: union.len(),
        cardinality_intersection: intersection.len(),
        cardinality_difference_a_b: difference_a_b.len(),
        cardinality_difference_b_a: difference_b_a.len(),
        cardinality_symmetric_difference: symmetric.len(),
    }
}
