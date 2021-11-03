use crate::ast::expression::{ExBox, Expression};
use crate::ast::program::Program;
use crate::ast::types::Type;
use crate::error::semantic_error::SemanticErrorKind;

#[derive(Debug)]
pub enum VarRef<E: Expression = ExBox> {
    Immediate(String),
    Field(Box<VarRef>, String),
    Index(Box<VarRef>, Vec<E>)
}

impl<E, Exp: Expression<Error = E>> Expression for VarRef<Exp> {
    type Error = SemanticErrorKind;

    fn get_type(&self, _program: &Program) -> Type {
        todo!()
    }

    fn as_number(&self, program: &Program) -> Result<i64, Self::Error> {
        Err(SemanticErrorKind::InvalidLimit)
    }
}