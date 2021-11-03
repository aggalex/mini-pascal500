use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::ops::Range;

#[derive(Clone, Eq, PartialEq, Debug)]
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
    pub const PRIMITIVE: &'static [Type] = &[Type::Integer, Type::Real, Type::Char, Type::Boolean];

    pub fn is_primitive(&self) -> bool {
        match self {
            Type::Integer | Type::Real | Type::Boolean | Type::Char => true,
            _ => false
        }
    }
}

impl Display for Type {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let explanation = match &self {
            Type::Integer => "Integer".to_string(),
            Type::Real => "Real".to_string(),
            Type::Boolean => "Boolean".to_string(),
            Type::Char => "Character".to_string(),
            Type::SetOf(ty) => format!("Set of {}", ty),
            Type::ArrayOf(range, ty) =>
                format!("Array of {} [{}]", ty,
                        range.into_iter()
                             .map(|range| format!("{}..{}", range.start, range.end))
                             .collect::<Vec<String>>()
                             .join(";")),
            Type::Record(record) =>
                format!("Record {}{}",
                    record.into_iter()
                        .map(|(key, value)| format!("{}: {}", key, value))
                        .take(3)
                        .collect::<Vec<String>>()
                        .join(";"),
                    if record.len() > 3 { "... end" } else { "end" }
                ),
            Type::Enum(variants) => format!("({}{})",
                    variants.into_iter()
                        .take(3)
                        .map(|var| var.clone())
                        .collect::<Vec<String>>()
                        .join(","),
                    if variants.len() > 3 { "..." } else { "" }
            ),
            Type::Range(range) => format!("{}..{}", range.start, range.end),
            Type::Invalid => "<???>".to_string()
        };
        write!(f, "{}", explanation)
    }
}