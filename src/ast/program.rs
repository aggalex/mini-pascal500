use std::collections::HashMap;
use crate::grammar::ProgramParser;
use crate::ast::expression::{ExBox, Expression, Invalid};
use crate::ast::types::Type;
use crate::error::parse_error::ParsingError;
use crate::error::PositionBuilder;
use crate::lexer::{Lexer, Token};

pub struct Program {
    pub name: String,
    pub constants: HashMap<String, ExBox>,
    pub globals: HashMap<String, Type>,
    pub types: HashMap<String, Type>,
    pub positioner: PositionBuilder,
}

impl Program {
    pub fn new (src: String) -> (Self, Vec<ParsingError<Token>>) {
        let mut program = Program {
            name: "".to_string(),
            constants: HashMap::new(),
            globals: HashMap::new(),
            types: HashMap::new(),
            positioner: PositionBuilder::new(src.clone()),
        };
        let mut errors = vec![];
        if let Err(err) = ProgramParser::new()
            .parse(&mut program, Lexer::new(&src[..])) {
            errors.push(err.into())
        };
        (program, errors)
    }
}

