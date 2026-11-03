mod exam;
mod l01;
mod l02;
mod l03;
mod l04;
mod l05;
mod l06;
mod l07;
mod l08;
mod l09;
mod l10;
mod model;

pub use exam::{exam_exercises, SCORE_NOTE};
pub use model::*;

static LESSON_LIST: [&Lesson; 10] = [
    &l01::LESSON,
    &l02::LESSON,
    &l03::LESSON,
    &l04::LESSON,
    &l05::LESSON,
    &l06::LESSON,
    &l07::LESSON,
    &l08::LESSON,
    &l09::LESSON,
    &l10::LESSON,
];

pub fn lessons() -> &'static [&'static Lesson] {
    &LESSON_LIST
}

pub fn lesson_by_id(id: &str) -> Option<&'static Lesson> {
    lessons().iter().copied().find(|lesson| lesson.id == id)
}

pub fn practice_exercise(id: &str) -> Option<&'static Exercise> {
    lessons()
        .iter()
        .flat_map(|lesson| lesson.exercises.iter())
        .find(|exercise| exercise.id == id)
}

pub fn exercise_anywhere(id: &str) -> Option<&'static Exercise> {
    practice_exercise(id).or_else(|| exam_exercises().iter().find(|exercise| exercise.id == id))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::{parse_int_set, IntSet};
    use std::collections::HashSet;

    fn all_strings(lesson: &Lesson) -> Vec<&'static str> {
        let mut out = vec![
            lesson.id,
            lesson.title,
            lesson.description,
        ];
        out.extend(lesson.objectives.iter().copied());
        for block in lesson.theory.iter().chain(lesson.summary.iter()) {
            push_block_strings(block, &mut out);
        }
        for example in lesson.examples {
            out.push(example.title);
            for block in example.blocks {
                push_block_strings(block, &mut out);
            }
        }
        for item in lesson.misconceptions {
            out.push(item.title);
            out.push(item.text);
        }
        for exercise in lesson.exercises {
            out.push(exercise.prompt);
            out.push(exercise.hint);
            out.push(exercise.explanation);
            if let ExerciseKind::SingleChoice { options, .. } = exercise.kind {
                out.extend(options.iter().copied());
            }
            if let ExerciseKind::FreeText { sample_solution } = exercise.kind {
                out.push(sample_solution);
            }
        }
        out
    }

    #[test]
    fn catalog_is_complete_and_unique() {
        assert_eq!(lessons().len(), 10);
        let mut ids = HashSet::new();
        let mut exercise_count = 0;
        let mut orders = HashSet::new();
        for lesson in lessons() {
            assert!(ids.insert(lesson.id), "doppelte Lektion {}", lesson.id);
            assert!(orders.insert(lesson.order));
            assert!(lesson.objectives.len() >= 3);
            assert!(lesson.examples.len() >= 2);
            assert!(lesson.misconceptions.len() >= 2);
            assert!(!lesson.theory.is_empty());
            assert!(!lesson.summary.is_empty());
            assert!(lesson.exercises.len() >= 3);
            for exercise in lesson.exercises {
                assert_eq!(exercise.lesson_id, lesson.id);
                assert!(ids.insert(exercise.id));
                assert!(!exercise.prompt.is_empty());
                assert!(!exercise.hint.is_empty());
                assert!(!exercise.explanation.is_empty());
                if let ExerciseKind::SingleChoice { options, correct_index } = exercise.kind {
                    assert!(correct_index < options.len());
                    assert!(options.len() >= 2);
                }
                exercise_count += 1;
            }
            for text in all_strings(lesson) {
                assert!(!text.contains('ß'), "ß in {}: {text}", lesson.id);
            }
        }
        assert!(exercise_count >= 30, "nur {exercise_count} Übungen");
        assert_eq!(exercise_count, 51);

        assert_eq!(exam_exercises().len(), 10);
        let mut exam_lessons = HashSet::new();
        for exercise in exam_exercises() {
            assert!(exercise.kind.is_auto_graded());
            assert!(ids.insert(exercise.id));
            assert!(exam_lessons.insert(exercise.lesson_id));
            assert!(lesson_by_id(exercise.lesson_id).is_some());
        }
        assert_eq!(exam_lessons.len(), 10);
    }

    #[test]
    fn seeded_set_answers_match_operations() {
        expect_set("l03e01", |got| {
            let a = parse_int_set("{1,2,4}").unwrap();
            let b = parse_int_set("{2,3}").unwrap();
            assert_eq!(got, a.union(&b));
        });
        expect_set("l03e02", |got| {
            let a = parse_int_set("{1,2,4}").unwrap();
            let b = parse_int_set("{2,3,4}").unwrap();
            assert_eq!(got, a.intersection(&b));
        });
        expect_set("l03e03", |got| {
            let a = parse_int_set("{1,2,4}").unwrap();
            let b = parse_int_set("{2,3}").unwrap();
            assert_eq!(got, a.difference(&b));
        });
        expect_set("l03e04", |got| {
            let a = parse_int_set("{1,2,3}").unwrap();
            let b = parse_int_set("{2,3,5}").unwrap();
            assert_eq!(got, a.symmetric_difference(&b));
        });
        expect_set("l04e01", |got| {
            let universe = parse_int_set("{0,1,2,3,4}").unwrap();
            let a = parse_int_set("{0,2,4}").unwrap();
            assert_eq!(got, a.complement(&universe).unwrap());
        });
        expect_set("l04e02", |got| {
            let universe = parse_int_set("{1,2,3,4,5}").unwrap();
            let a = parse_int_set("{1,2}").unwrap();
            let b = parse_int_set("{2,3}").unwrap();
            assert_eq!(got, a.union(&b).complement(&universe).unwrap());
        });
        expect_set("l04e05", |got| {
            let universe = parse_int_set("{1,2,3,4}").unwrap();
            let a = parse_int_set("{1,4}").unwrap();
            let b = parse_int_set("{4}").unwrap();
            assert_eq!(got, a.intersection(&b).complement(&universe).unwrap());
        });
        expect_set("l07e03", |got| {
            let a = parse_int_set("{1,2,3}").unwrap();
            let b = parse_int_set("{2,4}").unwrap();
            assert_eq!(got, a.union(&a.intersection(&b)));
        });
        expect_set("x03", |got| {
            let a = parse_int_set("{1,3,5}").unwrap();
            let b = parse_int_set("{2,3,4}").unwrap();
            assert_eq!(got, a.intersection(&b));
        });
        expect_set("x04", |got| {
            let universe = parse_int_set("{1,2,3,4,5,6}").unwrap();
            let a = parse_int_set("{2,4,6}").unwrap();
            assert_eq!(got, a.complement(&universe).unwrap());
        });
    }

    fn expect_set(id: &str, check: impl FnOnce(IntSet)) {
        let exercise = exercise_anywhere(id).unwrap_or_else(|| panic!("fehlt {id}"));
        match exercise.kind {
            ExerciseKind::Set { correct } => check(IntSet::from_slice(correct)),
            _ => panic!("{id} ist keine Mengenaufgabe"),
        }
    }
}
