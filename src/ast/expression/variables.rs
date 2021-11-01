use crate::ast::expression::Expression;
use crate::ast::program::Program;
use crate::ast::types::Type;

#[derive(Debug)]
pub enum VarRef {
    Immediate(String),
    Field(Box<VarRef>, String),
    Index(Box<VarRef>, Vec<Box<dyn Expression>>)
}

impl Expression for VarRef {
    fn get_type(&self, program: &Program) -> Type {
        todo!()
    }

    fn as_number(&self, program: &Program) -> Result<i64, /*unfoldable*/ &dyn Expression> {
        Err(self)
    }
}