use crate::{
    error::Error,
    stream::{ReadStream, Stream},
};

#[derive(Debug, Clone)]
pub struct LocalVariabileTable {
    pub start_pc: u16,
    pub length: u16,
    pub name_index: u16,
    pub descriptor_index: u16,
    pub index: u16,
}

impl LocalVariabileTable {
    pub fn new(stream: &mut Stream) -> Result<Self, Error> {
        Ok(Self {
            start_pc: stream.read_u16()?,
            length: stream.read_u16()?,
            name_index: stream.read_u16()?,
            descriptor_index: stream.read_u16()?,
            index: stream.read_u16()?,
        })
    }
}

#[derive(Debug, Clone)]
pub struct AttributeLocalVariabileTable {
    pub length: u16,
    pub table: Vec<LocalVariabileTable>,
}

impl AttributeLocalVariabileTable {
    pub fn new(stream: &mut Stream) -> Result<Self, Error> {
        let length = stream.read_u16()?;
        let mut table = Vec::with_capacity(length as usize);

        for _ in 0..length {
            table.push(LocalVariabileTable::new(stream)?)
        }

        Ok(Self { length, table })
    }
}
