use crate::{
    attributes::Attribute,
    constant_info::Constant,
    error::Error,
    stream::{ReadStream, Stream},
};

#[derive(Debug, Clone)]
pub struct ExeptionTable {
    pub start_pc: u16,
    pub end_pc: u16,
    pub handler_pc: u16,
    pub catch_type: u16,
}

impl ExeptionTable {
    pub fn new(stream: &mut Stream) -> Result<Self, Error> {
        Ok(Self {
            start_pc: stream.read_u16()?,
            end_pc: stream.read_u16()?,
            handler_pc: stream.read_u16()?,
            catch_type: stream.read_u16()?,
        })
    }
}

#[derive(Debug, Clone)]
pub struct AttributeInfo {
    pub attribute_name_index: u16,
    pub attribute_length: u32,
    pub attribute: Attribute,
}

impl AttributeInfo {
    pub fn new(stream: &mut Stream, constant_pool: &[Constant]) -> Result<Self, Error> {
        let attribute_name_index = stream.read_u16()?;
        let attribute_length = stream.read_u32()?;
        let attribute = Attribute::new(
            stream,
            constant_pool,
            attribute_name_index,
            attribute_length,
        )?;

        Ok(Self {
            attribute_name_index,
            attribute_length,
            attribute,
        })
    }
}
