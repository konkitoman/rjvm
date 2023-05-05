use std::collections::HashMap;

#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct ClassState {
    pub fields: HashMap<String, Type>,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum Type {
    Ref(u16),
    Int(i32),
    Long(i64),
    Float(f32),
    Double(f64),
    Array(Vec<Type>),
    Object(String, ClassState),
    None,
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum TypeTag {
    Int,
    Long,
    Float,
    Double,
    L(String),
}
