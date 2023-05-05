use crate::{
    error::Error,
    stream::{ReadStream, Stream},
};

#[derive(Debug, Clone)]
pub struct LineNumberTable {
    pub start_pc: u16,
    pub line_number: u16,
}

impl LineNumberTable {
    pub fn new(stream: &mut Stream) -> Result<Self, Error> {
        Ok(Self {
            start_pc: stream.read_u16()?,
            line_number: stream.read_u16()?,
        })
    }
}

#[derive(Debug, Clone)]
pub struct AttributeLineNumberTable {
    pub length: u16,
    pub table: Vec<LineNumberTable>,
}

impl AttributeLineNumberTable {
    pub fn new(stream: &mut Stream) -> Result<Self, Error> {
        let length = stream.read_u16()?;
        let mut table = Vec::with_capacity(length as usize);

        for _ in 0..length {
            table.push(LineNumberTable::new(stream)?)
        }

        Ok(Self { length, table })
    }
}
