use std::fmt::Formatter;
use std::num::ParseIntError;
use logos::Logos;
use crate::error::parse_error::ParsingError;
use crate::utils::{FromStrRadix, ParseFloatError};

#[derive(Logos, Clone, Debug, PartialEq)]
pub enum Token {
    #[regex("(?i)program")]
    Program,

    #[regex("(?i)type")]
    Type,

    #[regex("(?i)var")]
    Var,

    #[regex("(?i)array")]
    Array,

    #[regex("(?i)set")]
    Set,

    #[regex("(?i)record")]
    Record,

    #[regex("(?i)of", priority = 3)]
    Of,

    #[regex("(?i)in", priority = 3)]
    In,

    #[regex("(?i)end")]
    End,

    #[token("(")]
    LParen,

    #[token(")")]
    RParen,

    #[token("[")]
    LBrack,

    #[token("]")]
    RBrack,

    #[token(";")]
    Semi,

    #[token(":")]
    Colon,

    #[token(",")]
    Comma,

    #[token("..")]
    Spread,

    #[token(".")]
    Dot,

    #[token("=")]
    Eq,

    #[token("<>")]
    Neq,

    #[token(">")]
    Bg,

    #[token("<")]
    Lt,

    #[token(">=")]
    Bge,

    #[token("<=")]
    Lte,

    #[token("!")]
    Not,

    #[token("+")]
    Plus,

    #[token("-")]
    Minus,

    #[token("*")]
    Mul,

    #[token("/")]
    RDiv,

    #[regex("(?i)div")]
    Div,

    #[regex("(?i)mod")]
    Mod,

    #[regex("(?i)and")]
    And,

    #[regex("(?i)or", priority = 3)]
    Or,

    #[regex("(?i)true|false", parse_bool)]
    Bool(bool),

    #[regex(r"'([^']|\\['nfrbv\\])'", parse_char)]
    Char(char),

    #[regex(r"[0-9]+((\.[0-9]*)|([Ee][-+]?[0-9]+))", parse_real)]
    #[regex(r"(?i)0H[0-9A-Fa-f]+\.(([0-9A-Fa-f]*)|([Ee][-+]?[0-9A-Fa-f]+))", parse_real_hex)]
    #[regex(r"(?i)0B[01]+\.(([01]*)|([Ee][-+]?[01]+))", parse_real_bin)]
    Real(f64),

    #[regex(r"[0-9]+", parse_int)]
    #[regex(r"(?i)0H[0-9A-Fa-f]+", parse_int_hex)]
    #[regex(r"(?i)0B[01]+", parse_int_bin)]
    Integer(i64),

    #[regex(r"_?[A-Za-z][a-zA-Z0-9_]*[a-zA-Z0-9]",
    |lex| lex.slice().to_lowercase().to_string(),
    priority = 0)]
    Ident(String),

    #[error]
    #[regex(r"[ \t\f\n]+", logos::skip)]
    Error
}

fn parse_bool(lex: &mut logos::Lexer<Token>) -> bool {
    let slice = lex.slice();
    slice == "true"
}

fn parse_char(lex: &mut logos::Lexer<Token>) -> char {
    let slice = lex.slice();
    let mut chars = slice.chars();
    chars.next();
    match chars.next().unwrap() {
        '\\' => match chars.next().unwrap() {
            '\'' => '\'',
            '\\' => '\\',
            'n' => '\n',
            'f' => '\x0c',
            'r' => '\r',
            'b' => '\x08',
            'v' => '\x0b',
            c => c
        }
        c => c,
    }
}

fn parse_real(lex: &mut logos::Lexer<Token>) -> Result<f64, ParseFloatError> {
    let slice = lex.slice();
    f64::from_str_radix(slice, 10)
}

fn parse_real_hex(lex: &mut logos::Lexer<Token>) -> Result<f64, ParseFloatError> {
    let slice = lex.slice();
    f64::from_str_radix(&slice[2..], 16)
}

fn parse_real_bin(lex: &mut logos::Lexer<Token>) -> Result<f64, ParseFloatError> {
    let slice = lex.slice();
    f64::from_str_radix(&slice[2..], 2)
}

fn parse_int(lex: &mut logos::Lexer<Token>) -> Result<i64, ParseIntError> {
    let slice = lex.slice();
    i64::from_str_radix(slice, 10)
}

fn parse_int_hex(lex: &mut logos::Lexer<Token>) -> Result<i64, ParseIntError> {
    let slice = lex.slice();
    i64::from_str_radix(&slice[2..], 16)
}

fn parse_int_bin(lex: &mut logos::Lexer<Token>) -> Result<i64, ParseIntError> {
    let slice = lex.slice();
    i64::from_str_radix(&slice[2..], 2)
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Token::Program => "PROGRAM",
            Token::Type => "TYPE",
            Token::Var => "VAR",
            Token::Array => "ARRAY",
            Token::Set => "SET",
            Token::Record => "RECORD",
            Token::Of => "OF",
            Token::In => "IN",
            Token::End => "END",
            Token::LParen => "(",
            Token::RParen => ")",
            Token::LBrack => "[",
            Token::RBrack => "]",
            Token::Semi => ";",
            Token::Colon => ":",
            Token::Comma => ",",
            Token::Spread => "..",
            Token::Dot => ".",
            Token::Eq => "=",
            Token::Neq => "<>",
            Token::Bg => ">",
            Token::Lt => "<",
            Token::Bge => ">=",
            Token::Lte => "<=",
            Token::Not => "!",
            Token::Plus => "+",
            Token::Minus => "-",
            Token::Mul => "*",
            Token::RDiv => "/",
            Token::Div => "DIV",
            Token::Mod => "MOD",
            Token::And => "AND",
            Token::Or => "OR",
            Token::Bool(b) => if *b { "TRUE" } else { "FALSE" },
            Token::Char(c) => {
                let cstr = &c.to_string()[..];
                return write!(f, "'{}'", match c {
                    '\'' => "\\'",
                    '\\' => "\\",
                    '\n' => "\\n",
                    '\x0c' => "\\f",
                    '\r' => "\\r",
                    '\x08' => "\\b",
                    '\x0b' => "\\v",
                    _ => cstr
                })
            },
            Token::Real(r) => return write!(f, "{}", r),
            Token::Integer(i) => return write!(f, "{}", i),
            Token::Ident(str) => str,
            Token::Error => "<???>"
        })
    }
}

pub struct Lexer<'input> {
    pub source: &'input str,
    logos: logos::Lexer<'input, Token>
}

impl<'input> Lexer<'input> {
    pub fn new (source: &'input str) -> Self {
        Lexer {
            source,
            logos: logos::Lexer::new(&source[..]),
        }
    }
}

pub type Spanned<L, T, E> = Result<(L, T, L), E>;

impl<'input> Iterator for Lexer<'input> {
    type Item = Spanned<usize, Token, ParsingError<Token>>;

    fn next(&mut self) -> Option<Self::Item> {
        let token = self.logos.next()?;
        let span = self.logos.span();
        Some(Ok((
            span.start,
            token.clone(),
            span.end
        )))
    }
}