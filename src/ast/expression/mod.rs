// pub mod constants;
pub mod variables;
pub mod operators;
// pub mod constants;

use std::fmt::Debug;
use std::ops::Deref;
use crate::ast::program::Program;
use crate::ast::types::Type;
use crate::error::semantic_error::{SemanticError, SemanticErrorKind};
use crate::lexer::Token;

pub trait Expression: Debug + Send + Sync {
    type Error;

    fn get_type(&self, program: &Program) -> Type;
    #[allow(unused_variables)]
    fn validate(&self, program: &Program) -> Vec<Self::Error> {
        vec![]
    }
    fn is_valid(&self, program: &Program) -> bool {
        self.validate(program).len() == 0
    }
    fn as_number(&self, program: &Program) -> Result<i64, Self::Error>;
}

#[derive(Debug)]
pub struct ExBox {
    expr: Box<dyn Expression<Error = SemanticErrorKind>>,
    pub range: std::ops::Range<usize>
}

impl ExBox {
    pub fn new (expr: impl Expression<Error = SemanticErrorKind> + 'static,
                range: std::ops::Range<usize>) -> ExBox {
        ExBox {
            expr: Box::new(expr),
            range
        }
    }
}

impl std::ops::Deref for ExBox {
    type Target = dyn Expression<Error = SemanticErrorKind>;

    fn deref(&self) -> &Self::Target {
        &*self.expr
    }
}

impl std::ops::DerefMut for ExBox {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut *self.expr
    }
}

impl Expression for ExBox {
    type Error = SemanticError;

    fn get_type(&self, program: &Program) -> Type {
        self.expr.get_type(program)
    }

    fn validate(&self, program: &Program) -> Vec<Self::Error> {
        self.expr.validate(program).into_iter()
            .map(|kind| SemanticError {
                range: self.range.clone(),
                kind
            })
            .collect()
    }

    fn as_number(&self, program: &Program) -> Result<i64, Self::Error> {
        self.expr.as_number(program).map_err(|kind| SemanticError {
            range: self.range.clone(),
            kind
        })
    }
}

impl<E> Expression for Box<dyn Expression<Error = E>> {
    type Error = E;

    fn get_type(&self, program: &Program) -> Type {
        self.deref().get_type(program)
    }

    fn validate(&self, program: &Program) -> Vec<E> {
        self.deref().validate(program)
    }

    fn is_valid(&self, program: &Program) -> bool {
        self.deref().is_valid(program)
    }

    fn as_number(&self, program: &Program) -> Result<i64, Self::Error> {
        self.deref().as_number(program)
    }
}

#[derive(Debug)]
pub struct Invalid;

impl Expression for Invalid {
    type Error = SemanticErrorKind;

    fn get_type(&self, program: &Program) -> Type {
        Type::Invalid
    }

    fn as_number(&self, program: &Program) -> Result<i64, Self::Error> {
        Ok(0)
    }
}

impl Expression for i64 {
    type Error = SemanticErrorKind;

    fn get_type(&self, _program: &Program) -> Type {
        Type::Integer
    }

    fn as_number(&self, program: &Program) -> Result<i64, Self::Error> {
        Ok(*self)
    }
}

impl Expression for f64 {
    type Error = SemanticErrorKind;

    fn get_type(&self, _program: &Program) -> Type {
        Type::Real
    }

    fn as_number(&self, program: &Program) -> Result<i64, Self::Error> {
        Err(SemanticErrorKind::InvalidLimit)
    }
}

impl Expression for char {
    type Error = SemanticErrorKind;

    fn get_type(&self, _program: &Program) -> Type {
        Type::Char
    }

    fn as_number(&self, program: &Program) -> Result<i64, Self::Error> {
        Ok(*self as i64)
    }
}

impl Expression for bool {
    type Error = SemanticErrorKind;

    fn get_type(&self, _program: &Program) -> Type {
        Type::Boolean
    }

    fn as_number(&self, program: &Program) -> Result<i64, Self::Error> {
        Ok(*self as i64)
    }
}