use std::collections::HashMap;
use std::ops::Range;

#[derive(Clone, Eq, PartialEq)]
pub enum Type {
    Integer,
    Real,
    Boolean,
    Char,
    SetOf(Box<Type>),
    ArrayOf(Vec<Range<usize>>, Box<Type>),
    Record(HashMap<String, Type>),
    Enum(Vec<String>),
    Range(Range<isize>),
    Invalid
}

impl Type {
    pub fn is_primitive(&self) -> bool {
        match self {
            Type::Integer | Type::Real | Type::Boolean | Type::Char => true,
            _ => false
        }
    }
}