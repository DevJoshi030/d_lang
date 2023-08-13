use macros::sf;

#[derive(Debug, Clone)]
pub enum Object {
    Integer { value: i64 },
    Boolean { value: bool },
    Return { value: Box<Object> },
    Error { message: String },
    Null {},
}

impl Object {
    pub fn get_type(&self) -> String {
        match self {
            Object::Integer { .. } => sf!("INTEGER"),
            Object::Boolean { .. } => sf!("BOOLEAN"),
            Object::Return { .. } => sf!("RETURN"),
            Object::Error { .. } => sf!("ERROR"),
            Object::Null {} => sf!("NULL"),
        }
    }

    pub fn inspect(&self) -> String {
        match self {
            Object::Integer { value } => sf!(format!("{}", value)),
            Object::Boolean { value } => sf!(format!("{}", value)),
            Object::Return { value } => sf!(format!("{}", value.inspect())),
            Object::Error { message } => sf!(format!("ERROR: {}", message)),
            Object::Null {} => sf!("null"),
        }
    }

    pub fn get_bool_obj(obj: bool) -> Object {
        match obj {
            true => Object::Boolean { value: true },
            false => Object::Boolean { value: false },
        }
    }

    pub fn get_null_object() -> Object {
        Object::Null {}
    }
}
