use std::collections::HashMap;
use crate::ast::expression::Expression;
use crate::ast::types::Type;

pub struct Program {
    pub name: String,
    pub constants: HashMap<String, Box<dyn Expression>>,
    pub globals: HashMap<String, Type>,
    pub types: HashMap<String, Type>
}

impl Program {
    pub fn new () -> Self {
        Program {
            name: "".to_string(),
            constants: HashMap::new(),
            globals: HashMap::new(),
            types: HashMap::new()
        }
    }
}

