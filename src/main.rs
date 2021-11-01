extern crate lazy_static;
extern crate regex;
extern crate lalrpop_util;
extern crate colored;

use std::env::args;
use std::fs::File;
use std::io::{Read, stderr};
use lalrpop_util::{lalrpop_mod, ParseError};
use crate::lexer::{Lexer, Token};
use crate::ast::program::Program;
use crate::error::parse_error::ParsingError;

pub mod utils;
pub mod ast;
pub mod store;
pub mod lexer;
pub mod error;

lalrpop_mod!(grammar);

fn main() {
    let files = args()
        .map(|filename| File::open(filename))
        .filter(|res| res.is_ok())
        .map(|res| res.unwrap())
        .map(|mut file| {
            let mut str = String::new();
            file.read_to_string(&mut str).unwrap();
            str
        })
        .map(|src| {
            let mut program = Program::new();
            grammar::ProgramParser::new()
                .parse(&mut program, Lexer::new(&src[..]))
                .map(|_| program)
        })
        .collect::<Result<Vec<Program>, ParseError<usize, Token, ParsingError<Token>>>>();

    if let Err(err) = files {
        panic!("{}", err);
    }

    println!("Hello, world!");
}