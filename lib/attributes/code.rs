use crate::{
    attribute_info::{AttributeInfo, ExeptionTable},
    constant_info::Constant,
    error::Error,
    stream::{ReadStream, Stream},
};

#[derive(Debug, Clone)]
pub struct AttributeCode {
    pub max_stack: u16,
    pub max_locale: u16,
    pub code_length: u32,
    pub code: Vec<u8>,
    pub exeption_table_length: u16,
    pub exeption_table: Vec<ExeptionTable>,
    pub attributes_count: u16,
    pub attributes: Vec<AttributeInfo>,
}

impl AttributeCode {
    pub fn new(stream: &mut Stream, constant_pool: &[Constant]) -> Result<Self, Error> {
        let max_stack = stream.read_u16()?;
        let max_locale = stream.read_u16()?;
        let code_length = stream.read_u32()?;
        let mut code = Vec::with_capacity(code_length as usize);
        for _ in 0..code_length {
            code.push(stream.read_u8()?)
        }

        let exeption_table_length = stream.read_u16()?;
        let mut exeption_table = Vec::with_capacity(exeption_table_length as usize);
        for _ in 0..exeption_table_length {
            exeption_table.push(ExeptionTable::new(&mut *stream)?)
        }

        let attributes_count = stream.read_u16()?;
        let mut attributes = Vec::with_capacity(attributes_count as usize);
        for _ in 0..attributes_count {
            attributes.push(AttributeInfo::new(&mut *stream, constant_pool)?)
        }

        Ok(Self {
            max_stack,
            max_locale,
            code_length,
            code,
            exeption_table_length,
            exeption_table,
            attributes_count,
            attributes,
        })
    }
}
