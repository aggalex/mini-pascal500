// pub mod warning;
pub mod parse_error;
pub mod semantic_error;
pub mod io_error;
// pub mod semantic_error;
// pub mod type_error;
// pub mod class_error;

use std::fmt::{Formatter, Debug};
use lalrpop_util::{ErrorRecovery, ParseError};
use regex::Regex;
use colored::*;
use parse_error::{ErrorVariant, ParsingError};
use std::ops::Range;
use lazy_static::lazy_static;

#[derive(Debug, Clone)]
pub struct Position {
    pub line_no: usize,
    pub offset: Range<usize>,
    pub line: String
}

impl Position {
    fn trace(&self) -> String {
        let diff = self.offset.end - self.offset.start;
        format!(r#"{}

    {}
    {}{}{}

"#,
                self.to_string().blue(),
                self.line,
                "-".repeat(self.offset.start).green(),
                "^".repeat(diff).red().bold(),
                "-".repeat(self.line.len() + 1 - self.offset.end).green()
        )
    }
}

impl std::fmt::Display for Position {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "line {}, {:?}", self.line_no, self.offset)
    }
}

pub struct PositionBuilder {
    pub file: String
}

pub fn get_position<'input, L, T, E> (err: &ErrorRecovery<L, T, E>) -> Range<usize>
    where
        usize: From<L>,
        L: Copy + Clone + std::ops::Add<usize, Output=L>
{
    let l = match &err.error {
        ParseError::InvalidToken { location: l } => *l..(*l+1),
        ParseError::UnrecognizedEOF { location: l, .. } => *l..(*l+1),
        ParseError::UnrecognizedToken { token: (l, _, r), ..} => *l..*r,
        ParseError::ExtraToken { token: (l, .., r) } => *l..*r,
        ParseError::User { .. } => panic!("Invalid Error")
    }.clone();
    usize::from(l.start)..usize::from(l.end)

}

impl PositionBuilder {
    pub fn new (file: String) -> Self {
        Self {
            file
        }
    }

    // #[lazy_static::lazy_staticdebug_ensures(ret.offset < ret.line.len(), "Offset overflow")]
    pub fn pos(&self, mut offset: Range<usize>) -> Position {
        let mut predicate = &self.file[0..offset.end];
        let re = Regex::new(r"(\s|\n)*").unwrap();
        let reversed = &*predicate.chars().rev().collect::<String>();
        let empty_space = re.find(reversed);
        if let Some(es) = empty_space {
            offset.end = offset.end - es.end();
            offset.start = offset.start - es.end();
            predicate = &predicate[..offset.end];
        }
        let lines = predicate.split("\n");
        let line_count = lines.clone().count() - 1;
        let poschar = self.file.chars().nth(offset.start).unwrap();
        let last_line = self.file.split("\n")
            .nth(line_count - (poschar == '\n') as usize)
            .unwrap()
            .to_string();
        let end = lines.last().map_or(0, |l|
            l.len());
        let offset = end-(offset.end-offset.start)..end;

        Position {
            line_no: line_count,
            offset,
            line: last_line
        }
    }

    pub fn printable(&self, e: impl Throwable) -> Printable<'_, impl Throwable> {
        Printable::new(e, self)
    }
}

impl<'input, L, T, E> From<ErrorRecovery<L, T, E>> for ParsingError<String>
    where
        L: std::fmt::Debug + Copy + Clone + std::ops::Add<usize, Output=L>,
        T: std::fmt::Debug + std::fmt::Display + Clone,
        E: std::fmt::Debug,
        usize: From<L>,
{
    fn from(e: ErrorRecovery<L, T, E>) -> Self {
        lazy_static! {
            static ref TOKEN_REGEX: Regex = Regex::new(r##"^r?#*"(?:\\*")*(.*)(?:\\*")*"#*$"##).unwrap();
        }
        parse_error::ParsingError {
            position: get_position(&e),
            expected: match &e.error {
                ParseError::UnrecognizedEOF { expected, .. } |
                ParseError::UnrecognizedToken { expected, .. } => expected.into_iter()
                    .map(|t| {
                        TOKEN_REGEX.captures(t)
                            .map(|t| t[1].to_string())
                            .unwrap_or_else(|| "t".to_string())
                    })
                    .collect::<Vec<String>>()
                ,
                _ => vec![]
            },
            dropped: (&e.dropped_tokens).into_iter()
                .map(|el| el.1.to_string())
                .collect::<Vec<String>>(),
            token: match &e.error {
                ParseError::UnrecognizedToken { token: (_, token, _), .. } |
                ParseError::ExtraToken { token: (_, token, _), .. } => Some(token.to_string()),
                _ => None
            },
            variant: match e.error {
                ParseError::InvalidToken { .. } => ErrorVariant::InvalidToken,
                ParseError::UnrecognizedEOF { .. } => ErrorVariant::UnexpectedEOF,
                ParseError::UnrecognizedToken { .. } => ErrorVariant::UnrecognizedToken,
                ParseError::ExtraToken { .. } => ErrorVariant::ExtraToken,
                ParseError::User { .. } => ErrorVariant::Other
            }
        }
    }
}

lazy_static! {
    pub static ref ERROR: ColoredString = "Error:".red().bold();
    pub static ref WARNING: ColoredString = "Warning:".yellow().bold();
    pub static ref NOTE: ColoredString = "Note:".blue().bold();
}

pub trait Throwable {
    fn position(&self, positioner: &PositionBuilder) -> Position;
    fn title(&self) -> String;
    fn description(&self) -> String;
    fn notes(&self) -> Vec<String>;

    fn static_print(&self) -> String {
        format!("{} {}",
                self.title(),
                self.description())
    }

    fn format(&self, f: &mut Formatter<'_>, positioner: &PositionBuilder) -> std::fmt::Result {
        write!(f, "{} {} at {}{}",
               self.title(),
               self.description(),
               self.position(positioner).trace(),
               self.notes().into_iter()
                   .map(|s| format!("{} {}", &*NOTE, s))
                   .collect::<Vec<String>>()
                   .join("\n")
        )
    }
}

impl<T: Throwable + ?Sized> Throwable for Box<T> {
    fn position(&self, positioner: &PositionBuilder) -> Position {
        (&**self).position(positioner)
    }

    fn title(&self) -> String {
        (&**self).title()
    }

    fn description(&self) -> String {
        (&**self).description()
    }

    fn notes(&self) -> Vec<String> {
        (&**self).notes()
    }
}

pub struct Printable<'a, T: Throwable> {
    pub error: T,
    pub positioner: &'a PositionBuilder
}

impl<'a, T: Throwable> Printable<'a, T> {
    pub fn new(error: T, positioner: &'a PositionBuilder) -> Printable<'a, T> {
        Printable {
            error,
            positioner
        }
    }
}

impl<'a, T: Throwable> std::fmt::Display for Printable<'a, T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.error.format(f, self.positioner)
    }
}