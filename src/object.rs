use macros::sf;

#[derive(Debug)]
pub enum Object {
    Integer { value: i64 },
    Boolean { value: bool },
    Null {},
}

impl Object {
    pub fn get_type(&self) -> String {
        match self {
            Object::Integer { value: _ } => sf!("INTEGER"),
            Object::Boolean { value: _ } => sf!("BOOLEAN"),
            Object::Null {} => sf!("NULL"),
        }
    }

    pub fn inspect(&self) -> String {
        match self {
            Object::Integer { value } => sf!(format!("{}", value)),
            Object::Boolean { value } => sf!(format!("{}", value)),
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
