use crate::ast::expression::ExBox;
use crate::ast::types::Type;
use crate::error::{Position, PositionBuilder, Throwable};

#[derive(Debug)]
pub struct SemanticError {
    pub range: std::ops::Range<usize>,
    pub kind: SemanticErrorKind
}

#[derive(Debug)]
pub enum SemanticErrorKind {
    TypeError {
        expected: Vec<Type>,
        got: Type,
    },
    InvalidLimit
}

impl SemanticError {
    pub fn new (range: std::ops::Range<usize>, kind: SemanticErrorKind) -> SemanticError {
        SemanticError {
            range, kind
        }
    }
}

impl Throwable for SemanticError {
    fn position(&self, positioner: &PositionBuilder) -> Position {
        positioner.pos(self.range.clone())
    }

    fn title(&self) -> String {
        match self.kind {
            SemanticErrorKind::TypeError { .. } => "Type Error".to_string(),
            SemanticErrorKind::InvalidLimit => "Invalid Limit".to_string()
        }
    }

    fn description(&self) -> String {
        match &self.kind {
            SemanticErrorKind::TypeError {
                expected,
                got
            } => format!("Expected {}{}, got {}",
                         expected.into_iter()
                             .take(3)
                             .map(|t| t.to_string())
                             .collect::<Vec<String>>()
                             .join("/"),
                         if expected.len() > 3 { "..." } else { "" },
                         got),
            SemanticErrorKind::InvalidLimit =>
                "This expression cannot be used as a limit".to_string()
        }
    }

    fn notes(&self) -> Vec<String> {
        vec![]
    }
}

impl From<SemanticError> for SemanticErrorKind {
    fn from(e: SemanticError) -> SemanticErrorKind {
        e.kind
    }
}