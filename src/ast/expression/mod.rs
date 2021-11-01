// pub mod constants;
pub mod variables;
pub mod operators;
// pub mod constants;

use std::fmt::Debug;
use std::ops::Deref;
use crate::ast::program::Program;
use crate::ast::types::Type;

pub enum Error {
    InvalidOperands
}

pub trait Expression: Debug + Send + Sync {
    fn get_type(&self, program: &Program) -> Type;
    fn validate(&self, program: &Program) -> Vec<Error> {
        vec![]
    }
    fn is_valid(&self, program: &Program) -> bool {
        self.validate(program).len() == 0
    }
    fn as_number(&self, program: &Program) -> Result<i64, /*unfoldable*/ &dyn Expression>;
}

pub trait Optimization: Expression {
    fn optimize(&self) -> Box<dyn Expression>;
}

impl Expression for Box<dyn Expression> {
    fn get_type(&self, program: &Program) -> Type {
        self.deref().get_type(program)
    }

    fn validate(&self, program: &Program) -> Vec<Error> {
        self.deref().validate(program)
    }

    fn is_valid(&self, program: &Program) -> bool {
        self.deref().is_valid(program)
    }

    fn as_number(&self, program: &Program) -> Result<i64, /*unfoldable*/ &dyn Expression> {
        self.deref().as_number(program)
    }
}

impl Expression for i64 {
    fn get_type(&self, program: &Program) -> Type {
        Type::Integer
    }

    fn as_number(&self, program: &Program) -> Result<i64, /*unfoldable*/ &dyn Expression> {
        Ok(*self)
    }
}

impl Expression for f64 {
    fn get_type(&self, program: &Program) -> Type {
        Type::Real
    }

    fn as_number(&self, program: &Program) -> Result<i64, /*unfoldable*/ &dyn Expression> {
        Err(self)
    }
}

impl Expression for char {
    fn get_type(&self, program: &Program) -> Type {
        Type::Char
    }

    fn as_number(&self, program: &Program) -> Result<i64, /*unfoldable*/ &dyn Expression> {
        Ok(*self as i64)
    }
}

impl Expression for bool {
    fn get_type(&self, program: &Program) -> Type {
        Type::Boolean
    }

    fn as_number(&self, program: &Program) -> Result<i64, /*unfoldable*/ &dyn Expression> {
        Ok(*self as i64)
    }
}