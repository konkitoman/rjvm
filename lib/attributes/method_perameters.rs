use crate::{
    error::Error,
    stream::{ReadStream, Stream},
};

#[derive(Debug, Clone)]
pub struct MethodParameters {
    pub name_index: u16,
    pub access_flags: u16,
}

impl MethodParameters {
    pub fn new(stream: &mut Stream) -> Result<Self, Error> {
        Ok(Self {
            name_index: stream.read_u16()?,
            access_flags: stream.read_u16()?,
        })
    }
}

#[derive(Debug, Clone)]
pub struct AttributeMethodParameters {
    pub count: u8,
    pub parameters: Vec<MethodParameters>,
}

impl AttributeMethodParameters {
    pub fn new(stream: &mut Stream) -> Result<Self, Error> {
        let count = stream.read_u8()?;
        let mut parameters = Vec::with_capacity(count as usize);

        for _ in 0..count {
            parameters.push(MethodParameters::new(stream)?)
        }

        Ok(Self { count, parameters })
    }
}
