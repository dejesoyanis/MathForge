export type Block =
  | { type: "heading"; text: string }
  | { type: "paragraph"; text: string }
  | { type: "math"; latex: string }
  | { type: "bullets"; items: string[] };

export type Example = {
  title: string;
  blocks: Block[];
};

export type Misconception = {
  title: string;
  text: string;
};

export type ExerciseKind = "true_false" | "single_choice" | "set" | "numeric" | "free_text";

export type Exercise = {
  id: string;
  lesson_id: string;
  difficulty: string;
  kind: ExerciseKind;
  prompt: string;
  hint: string;
  options?: string[];
};

export type LessonSummary = {
  id: string;
  order: number;
  title: string;
  description: string;
  difficulty: string;
  exercise_count: number;
};

export type LessonDetail = {
  id: string;
  order: number;
  title: string;
  description: string;
  difficulty: string;
  content_version: string;
  objectives: string[];
  theory: Block[];
  examples: Example[];
  misconceptions: Misconception[];
  summary: Block[];
  exercises: Exercise[];
};

export type LessonListResponse = {
  content_version: string;
  lessons: LessonSummary[];
};

export type SelfAssessment = "correct" | "partial" | "incorrect";

export type Answer =
  | { type: "true_false"; value: boolean }
  | { type: "single_choice"; index: number }
  | { type: "set"; value: string }
  | { type: "numeric"; value: number }
  | { type: "free_text"; value: string; self_assessment?: SelfAssessment };

export type SubmitResponse = {
  exercise_id: string;
  attempt_id: number;
  auto_graded: boolean;
  correct: boolean | null;
  feedback: string;
  normalized: string | null;
  solution_latex: string | null;
  explanation: string;
  self_assessment: SelfAssessment | null;
  your_answer_display: string;
};

export type LessonProgressItem = {
  lesson_id: string;
  attempted: number;
  exercise_total: number;
  correct: number;
  completed: boolean;
  completed_at: string | null;
};

export type LatestAttempt = {
  exercise_id: string;
  lesson_id: string;
  auto_graded: boolean;
  correct: boolean | null;
  self_assessment: SelfAssessment | null;
  feedback: string;
  normalized: string | null;
  answer_display: string;
  explanation: string;
  solution_latex: string | null;
  created_at: string;
};

export type ProgressResponse = {
  exercises_attempted: number;
  exercises_correct: number;
  self_assessed_correct: number;
  lessons_completed: number;
  lessons_total: number;
  exercises_total: number;
  next_lesson_id: string | null;
  lessons: LessonProgressItem[];
  latest_attempts: LatestAttempt[];
  score_note: string;
};

export type LabResponse = {
  a: string;
  b: string;
  a_latex: string;
  b_latex: string;
  union: string;
  union_latex: string;
  intersection: string;
  intersection_latex: string;
  difference_a_b: string;
  difference_a_b_latex: string;
  difference_b_a: string;
  difference_b_a_latex: string;
  symmetric_difference: string;
  symmetric_difference_latex: string;
  a_subseteq_b: boolean;
  b_subseteq_a: boolean;
  a_subsetneq_b: boolean;
  b_subsetneq_a: boolean;
  equal: boolean;
  cardinality_a: number;
  cardinality_b: number;
  cardinality_union: number;
  cardinality_intersection: number;
  cardinality_difference_a_b: number;
  cardinality_difference_b_a: number;
  cardinality_symmetric_difference: number;
};

export type ExamQuestion = {
  exercise: Exercise;
  lesson_title: string;
};

export type ExamResultItem = {
  exercise_id: string;
  auto_graded: boolean;
  correct: boolean | null;
  feedback: string;
  explanation: string;
  solution_latex: string | null;
  normalized: string | null;
  your_answer_display: string;
};

export type ExamAttemptSummary = {
  id: string;
  started_at: string;
  completed_at: string | null;
  auto_correct: number | null;
  auto_total: number | null;
  status: string;
};

export type ExamDetail = {
  id: string;
  started_at: string;
  completed_at: string | null;
  status: string;
  auto_correct: number | null;
  auto_total: number | null;
  score_note: string;
  questions: ExamQuestion[];
  results?: ExamResultItem[];
};
