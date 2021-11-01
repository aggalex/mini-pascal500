use std::fmt::Formatter;
use std::ops::Range;
use crate::error::{ERROR, Position, PositionBuilder, Throwable};
use crate::lexer;

use super::NOTE;

#[derive(Debug, Clone)]
pub struct ErrorRef (pub usize, pub Range<usize>);

impl std::fmt::Display for ErrorRef {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "!!Error")
    }
}

#[derive(Clone, Copy, Debug)]
pub enum ErrorVariant {
    InvalidToken,
    UnrecognizedToken,
    UnexpectedEOF,
    ExtraToken,
    Other
}

#[derive(Clone, Debug)]
pub struct ParsingError<T: std::fmt::Debug + Clone> {
    pub dropped: Vec<T>,
    pub position: Range<usize>,
    pub expected: Vec<String>,
    pub token: Option<T>,
    pub variant: ErrorVariant,
}

impl std::fmt::Display for ParsingError<lexer::Token> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} at {:?}{}",
               self.title(),
               self.description(),
               self.position,
               self.notes().into_iter()
                   .map(|s| format!("{} {}", &*NOTE, s))
                   .collect::<Vec<String>>()
                   .join("\n")
        )
    }
}

impl std::fmt::Display for ErrorVariant {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            ErrorVariant::InvalidToken => "Stray token in program",
            ErrorVariant::UnrecognizedToken => "Unrecognized token",
            ErrorVariant::UnexpectedEOF => "Unexpected end of file",
            ErrorVariant::ExtraToken => "Extra token",
            ErrorVariant::Other => ""
        })
    }
}

impl<T: std::fmt::Debug + std::fmt::Display + Clone> Throwable for ParsingError<T> {
    fn position(&self, positioner: &PositionBuilder) -> Position {
        positioner.pos(self.position.clone())
    }

    fn title(&self) -> String {
        ERROR.to_string()
    }

    fn description(&self) -> String {
        self.variant.to_string().replace("token", &if let Some(t) = &self.token {
            format!("'{}'", t)
        } else {
            "token".to_string()
        })
    }

    fn notes(&self) -> Vec<String> {
        vec![]
    }
}