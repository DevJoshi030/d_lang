use macros::sf;

use crate::ast::{Expression, Statement};

#[derive(Debug, Clone)]
pub enum Object {
    Integer {
        value: i64,
    },
    Boolean {
        value: bool,
    },
    Return {
        value: Box<Object>,
    },
    Error {
        message: String,
    },
    Function {
        parameters: Vec<Expression>,
        body: Box<Statement>,
    },
    Null {},
}

impl Object {
    pub fn get_type(&self) -> String {
        match self {
            Object::Integer { .. } => sf!("INTEGER"),
            Object::Boolean { .. } => sf!("BOOLEAN"),
            Object::Return { .. } => sf!("RETURN"),
            Object::Error { .. } => sf!("ERROR"),
            Object::Function { .. } => sf!("FUNCTION"),
            Object::Null {} => sf!("NULL"),
        }
    }
}
