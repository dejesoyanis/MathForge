use super::parser::parse_int_set;
use super::set::IntSet;
use crate::content::{Exercise, ExerciseKind};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SubmittedAnswer {
    TrueFalse { value: bool },
    SingleChoice { index: usize },
    Set { raw: String },
    Numeric { value: i64 },
    FreeText {
        text: String,
        self_assessment: Option<SelfAssessment>,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SelfAssessment {
    Correct,
    Partial,
    Incorrect,
}

impl SelfAssessment {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Correct => "correct",
            Self::Partial => "partial",
            Self::Incorrect => "incorrect",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Graded {
    pub auto_graded: bool,
    /// Nur bei automatisch bewerteten Aufgaben gesetzt.
    /// Freitext bleibt immer `None`, auch nach einer Selbsteinschätzung.
    pub correct: Option<bool>,
    pub feedback: String,
    pub normalized: Option<String>,
    pub solution_latex: Option<String>,
    pub explanation: String,
    pub self_assessment: Option<SelfAssessment>,
    pub your_answer_display: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GradeError {
    TypeMismatch,
    ChoiceOutOfRange,
    EmptyFreeText,
    FreeTextTooLong,
    InvalidSet { message: String },
}

const MAX_FREE_TEXT: usize = 5_000;

pub fn grade(exercise: &Exercise, answer: &SubmittedAnswer) -> Result<Graded, GradeError> {
    match (&exercise.kind, answer) {
        (ExerciseKind::TrueFalse { correct }, SubmittedAnswer::TrueFalse { value }) => {
            let is_correct = value == correct;
            Ok(Graded {
                auto_graded: true,
                correct: Some(is_correct),
                feedback: short_feedback(is_correct),
                normalized: None,
                solution_latex: Some(bool_latex(*correct)),
                explanation: exercise.explanation.to_string(),
                self_assessment: None,
                your_answer_display: if *value {
                    "wahr".to_string()
                } else {
                    "falsch".to_string()
                },
            })
        }
        (ExerciseKind::SingleChoice { options, correct_index }, SubmittedAnswer::SingleChoice { index }) => {
            if *index >= options.len() {
                return Err(GradeError::ChoiceOutOfRange);
            }
            let is_correct = *index == *correct_index;
            Ok(Graded {
                auto_graded: true,
                correct: Some(is_correct),
                feedback: short_feedback(is_correct),
                normalized: None,
                solution_latex: None,
                explanation: exercise.explanation.to_string(),
                self_assessment: None,
                your_answer_display: options[*index].to_string(),
            })
        }
        (ExerciseKind::Set { correct }, SubmittedAnswer::Set { raw }) => {
            let parsed = parse_int_set(raw).map_err(|err| GradeError::InvalidSet { message: err.message })?;
            let expected = IntSet::from_slice(correct);
            let is_correct = parsed == expected;
            Ok(Graded {
                auto_graded: true,
                correct: Some(is_correct),
                feedback: if is_correct {
                    "Richtig. Die Menge wurde unabhängig von Reihenfolge und Duplikaten verglichen.".to_string()
                } else {
                    format!(
                        "Nicht korrekt. Deine normalisierte Menge ist {}. Vergleiche sie mit der Musterlösung.",
                        parsed.to_canonical()
                    )
                },
                normalized: Some(parsed.to_canonical()),
                solution_latex: Some(expected.to_latex()),
                explanation: exercise.explanation.to_string(),
                self_assessment: None,
                your_answer_display: parsed.to_canonical(),
            })
        }
        (ExerciseKind::Numeric { correct }, SubmittedAnswer::Numeric { value }) => {
            let is_correct = value == correct;
            Ok(Graded {
                auto_graded: true,
                correct: Some(is_correct),
                feedback: short_feedback(is_correct),
                normalized: Some(value.to_string()),
                solution_latex: Some(correct.to_string()),
                explanation: exercise.explanation.to_string(),
                self_assessment: None,
                your_answer_display: value.to_string(),
            })
        }
        (
            ExerciseKind::FreeText { .. },
            SubmittedAnswer::FreeText {
                text,
                self_assessment,
            },
        ) => {
            let trimmed = text.trim();
            if trimmed.is_empty() {
                return Err(GradeError::EmptyFreeText);
            }
            if trimmed.chars().count() > MAX_FREE_TEXT {
                return Err(GradeError::FreeTextTooLong);
            }
            let feedback = match self_assessment {
                None => "Diese Aufgabe wird nicht automatisch bewertet. Vergleiche deine Begründung mit der Musterlösung und schätze dich danach ein.".to_string(),
                Some(SelfAssessment::Correct) => "Selbsteinschätzung gespeichert: korrekt. Das ist keine automatische Beweisprüfung.".to_string(),
                Some(SelfAssessment::Partial) => "Selbsteinschätzung gespeichert: teilweise korrekt. Das ist keine automatische Beweisprüfung.".to_string(),
                Some(SelfAssessment::Incorrect) => "Selbsteinschätzung gespeichert: nicht korrekt. Das ist keine automatische Beweisprüfung.".to_string(),
            };
            Ok(Graded {
                auto_graded: false,
                correct: None,
                feedback,
                normalized: None,
                solution_latex: None,
                explanation: exercise.explanation.to_string(),
                self_assessment: *self_assessment,
                your_answer_display: trimmed.to_string(),
            })
        }
        _ => Err(GradeError::TypeMismatch),
    }
}

fn short_feedback(correct: bool) -> String {
    if correct {
        "Richtig.".to_string()
    } else {
        "Nicht korrekt.".to_string()
    }
}

fn bool_latex(value: bool) -> String {
    if value {
        "\\text{wahr}".to_string()
    } else {
        "\\text{falsch}".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::content::{Difficulty, Exercise, ExerciseKind};

    fn exercise(kind: ExerciseKind) -> Exercise {
        Exercise {
            id: "t",
            lesson_id: "l01",
            difficulty: Difficulty::Einstieg,
            prompt: "p",
            hint: "h",
            explanation: "Erklärung zur Musterlösung.",
            kind,
        }
    }

    #[test]
    fn grades_every_automatic_kind() {
        let tf = exercise(ExerciseKind::TrueFalse { correct: true });
        let graded = grade(&tf, &SubmittedAnswer::TrueFalse { value: true }).unwrap();
        assert_eq!(graded.correct, Some(true));
        let graded = grade(&tf, &SubmittedAnswer::TrueFalse { value: false }).unwrap();
        assert_eq!(graded.correct, Some(false));

        let choice = exercise(ExerciseKind::SingleChoice {
            options: &["a", "b", "c"],
            correct_index: 1,
        });
        assert_eq!(
            grade(&choice, &SubmittedAnswer::SingleChoice { index: 1 })
                .unwrap()
                .correct,
            Some(true)
        );
        assert_eq!(
            grade(&choice, &SubmittedAnswer::SingleChoice { index: 0 })
                .unwrap()
                .correct,
            Some(false)
        );
        assert!(matches!(
            grade(&choice, &SubmittedAnswer::SingleChoice { index: 9 }),
            Err(GradeError::ChoiceOutOfRange)
        ));

        let set_ex = exercise(ExerciseKind::Set { correct: &[1, 2, 3] });
        let graded = grade(
            &set_ex,
            &SubmittedAnswer::Set {
                raw: "{3,1,2,1}".to_string(),
            },
        )
        .unwrap();
        assert_eq!(graded.correct, Some(true));
        assert_eq!(graded.normalized.as_deref(), Some("{1,2,3}"));
        let graded = grade(
            &set_ex,
            &SubmittedAnswer::Set {
                raw: "{-2,0}".to_string(),
            },
        )
        .unwrap();
        assert_eq!(graded.correct, Some(false));
        assert!(matches!(
            grade(&set_ex, &SubmittedAnswer::Set { raw: "{1,}".to_string() }),
            Err(GradeError::InvalidSet { .. })
        ));

        let numeric = exercise(ExerciseKind::Numeric { correct: 8 });
        assert_eq!(
            grade(&numeric, &SubmittedAnswer::Numeric { value: 8 })
                .unwrap()
                .correct,
            Some(true)
        );
        assert_eq!(
            grade(&numeric, &SubmittedAnswer::Numeric { value: -1 })
                .unwrap()
                .correct,
            Some(false)
        );
    }

    #[test]
    fn free_text_is_never_auto_correct() {
        let free = exercise(ExerciseKind::FreeText {
            sample_solution: "Muster.",
        });
        for assessment in [
            None,
            Some(SelfAssessment::Correct),
            Some(SelfAssessment::Partial),
            Some(SelfAssessment::Incorrect),
        ] {
            let graded = grade(
                &free,
                &SubmittedAnswer::FreeText {
                    text: "Mein Beweisversuch ist lang genug.".to_string(),
                    self_assessment: assessment,
                },
            )
            .unwrap();
            assert!(!graded.auto_graded);
            assert_eq!(graded.correct, None);
            assert_eq!(graded.self_assessment, assessment);
        }
        assert!(matches!(
            grade(
                &free,
                &SubmittedAnswer::FreeText {
                    text: "   ".to_string(),
                    self_assessment: Some(SelfAssessment::Correct),
                }
            ),
            Err(GradeError::EmptyFreeText)
        ));
        assert!(matches!(
            grade(&free, &SubmittedAnswer::TrueFalse { value: true }),
            Err(GradeError::TypeMismatch)
        ));
    }
}
