// pub mod constants;
pub mod variables;
pub mod operators;
// pub mod constants;

use std::fmt::Debug;
use std::ops::Deref;
use crate::ast::types::Type;

pub enum Error {
    InvalidOperands
}

pub trait Expression: Debug + Send + Sync {
    fn get_type(&self) -> Type;
    fn validate(&self) -> Vec<Error> {
        vec![]
    }
    fn is_valid(&self) -> bool {
        self.validate().len() == 0
    }
    fn as_number(&self) -> Result<i64, /*unfoldable*/ &dyn Expression>;
}

pub trait Optimization: Expression {
    fn optimize(&self) -> Box<dyn Expression>;
}

impl Expression for Box<dyn Expression> {
    fn get_type(&self) -> Type {
        self.deref().get_type()
    }

    fn validate(&self) -> Vec<Error> {
        self.deref().validate()
    }

    fn is_valid(&self) -> bool {
        self.deref().is_valid()
    }

    fn as_number(&self) -> Result<i64, &dyn Expression> {
        self.deref().as_number()
    }
}

impl Expression for i64 {
    fn get_type(&self) -> Type {
        Type::Integer
    }

    fn as_number(&self) -> Result<i64, &dyn Expression> {
        Ok(*self)
    }
}

impl Expression for f64 {
    fn get_type(&self) -> Type {
        Type::Real
    }

    fn as_number(&self) -> Result<i64, &dyn Expression> {
        Err(self)
    }
}

impl Expression for char {
    fn get_type(&self) -> Type {
        Type::Char
    }

    fn as_number(&self) -> Result<i64, &dyn Expression> {
        Ok(*self as i64)
    }
}

impl Expression for bool {
    fn get_type(&self) -> Type {
        Type::Boolean
    }

    fn as_number(&self) -> Result<i64, &dyn Expression> {
        Ok(*self as i64)
    }
}