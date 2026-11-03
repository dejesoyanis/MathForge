mod parser;
mod set;
mod eval;

pub use eval::{grade, GradeError, Graded, SelfAssessment, SubmittedAnswer};
pub use parser::parse_int_set;
pub use set::IntSet;
