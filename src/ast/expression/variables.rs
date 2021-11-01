use crate::ast::expression::Expression;
use crate::ast::types::Type;

#[derive(Debug)]
pub enum VarRef {
    Immediate(String),
    Field(Box<VarRef>, String),
    Index(Box<VarRef>, Vec<Box<dyn Expression>>)
}

impl Expression for VarRef {
    fn get_type(&self) -> Type {
        todo!()
    }

    fn as_number(&self) -> Result<i64, &dyn Expression> {
        Err(self)
    }
}