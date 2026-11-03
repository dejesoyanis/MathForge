#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Difficulty {
    Einstieg,
    Aufbau,
    Vertiefung,
}

impl Difficulty {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Einstieg => "Einstieg",
            Self::Aufbau => "Aufbau",
            Self::Vertiefung => "Vertiefung",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Block {
    Heading(&'static str),
    Paragraph(&'static str),
    Math(&'static str),
    Bullets(&'static [&'static str]),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Example {
    pub title: &'static str,
    pub blocks: &'static [Block],
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Misconception {
    pub title: &'static str,
    pub text: &'static str,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Exercise {
    pub id: &'static str,
    pub lesson_id: &'static str,
    pub difficulty: Difficulty,
    pub prompt: &'static str,
    pub hint: &'static str,
    pub explanation: &'static str,
    pub kind: ExerciseKind,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ExerciseKind {
    TrueFalse { correct: bool },
    SingleChoice {
        options: &'static [&'static str],
        correct_index: usize,
    },
    Set { correct: &'static [i64] },
    Numeric { correct: i64 },
    FreeText { sample_solution: &'static str },
}

impl ExerciseKind {
    pub fn name(self) -> &'static str {
        match self {
            Self::TrueFalse { .. } => "true_false",
            Self::SingleChoice { .. } => "single_choice",
            Self::Set { .. } => "set",
            Self::Numeric { .. } => "numeric",
            Self::FreeText { .. } => "free_text",
        }
    }

    pub fn is_auto_graded(self) -> bool {
        !matches!(self, Self::FreeText { .. })
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Lesson {
    pub id: &'static str,
    pub order: u8,
    pub title: &'static str,
    pub description: &'static str,
    pub difficulty: Difficulty,
    pub objectives: &'static [&'static str],
    pub theory: &'static [Block],
    pub examples: &'static [Example],
    pub misconceptions: &'static [Misconception],
    pub summary: &'static [Block],
    pub exercises: &'static [Exercise],
}

pub const CONTENT_VERSION: &str = "1";

pub fn push_block_strings(block: &Block, out: &mut Vec<&'static str>) {
    match *block {
        Block::Heading(text) | Block::Paragraph(text) | Block::Math(text) => out.push(text),
        Block::Bullets(items) => out.extend(items.iter().copied()),
    }
}
