use std::error::Error;
use std::fmt::{Display, Formatter};
use std::num::{ParseIntError};
use std::str::FromStr;



pub trait FromStrRadix: Sized {
    type Error: Error;

    fn from_str_radix(s: &str, radix: u32) -> Result<Self, Self::Error>;
}

#[derive(Debug)]
pub enum ParseFloatError {
    IntError(ParseIntError),
    MissingPoint
}

impl From<ParseIntError> for ParseFloatError {
    fn from(err: ParseIntError) -> Self {
        ParseFloatError::IntError(err)
    }
}

impl Display for ParseFloatError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseFloatError::IntError(err) => err.fmt(f),
            ParseFloatError::MissingPoint => write!(f, "Missing floating point")
        }
    }
}

impl Error for ParseFloatError {

}

impl FromStrRadix for f64 {
    type Error = ParseFloatError;

    fn from_str_radix(s: &str, radix: u32) -> Result<Self, ParseFloatError> {
        let (base, exp) = match s.find(".") {
            Some(dot) => {
                let exp = s.len() - dot;
                let mut int_str = s
                    .to_string();
                int_str.replace_range(dot..dot+1, "");
                let int = i64::from_str_radix(&int_str, radix)?;
                (int, exp)
            }
            None => {
                let mut split = s.split(|c: char| c == 'e' || c == 'E');
                let base = split.next()
                    .ok_or(ParseFloatError::MissingPoint)?;
                let exp = split.next()
                    .ok_or(ParseFloatError::MissingPoint)?;
                (i64::from_str_radix(base, radix).unwrap(), usize::from_str(exp).unwrap())
            }
        };
        Ok((radix as f64).powf(-(exp as f64)) * base as f64)
    }
}

impl FromStrRadix for f32 {
    type Error = ParseFloatError;

    fn from_str_radix(s: &str, radix: u32) -> Result<f32, ParseFloatError> {
        f64::from_str_radix(s, radix).map(|f| f as f32)
    }
}

#[macro_export]
macro_rules! token {
    ($t:ident => $ident:path) => {
        if let $ident(token) = $t { token } else { panic!("Wrong token") }
    }
}