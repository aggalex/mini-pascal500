extern crate lazy_static;
extern crate regex;
extern crate lalrpop_util;
extern crate colored;

use std::env::args;
use std::fs::File;
use std::io::{Read};
use lalrpop_util::{lalrpop_mod, ParseError};
use crate::lexer::{Lexer, Token};
use crate::ast::program::Program;
use crate::error::parse_error::ParsingError;
use crate::error::{PositionBuilder, Printable, Throwable};

pub mod utils;
pub mod ast;
pub mod store;
pub mod lexer;
pub mod error;

lalrpop_mod!(grammar);

fn main() {
    let files = args()
        .skip(1)
        .map(|filename| File::open(filename))
        .filter(|res| res.is_ok())
        .map(|res| res.unwrap())
        .filter_map(|mut file| {
            let mut str = String::new();
            let read = file.read_to_string(&mut str).unwrap_or_else(|err| {
                println!("{}", err.static_print());
                0
            });
            if read > 0 { Some(str) } else { None }
        })
        .map(Program::new)
        .filter_map(|(program, errors)| {
            if errors.len() == 0 {
                Some(program)
            } else {
                errors.into_iter()
                    .map(|err| Printable::new(err, &program.positioner))
                    .for_each(|printable| println!("{}", printable));
                None
            }
        })
        .collect::<Vec<Program>>();

    println!("Hello, world!");
}