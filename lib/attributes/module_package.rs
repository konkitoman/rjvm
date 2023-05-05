use crate::{
    error::Error,
    stream::{ReadStream, Stream},
};

#[derive(Debug, Clone)]
pub struct AttributeModulePackage {
    pub count: u16,
    pub index: Vec<u16>,
}

impl AttributeModulePackage {
    pub fn new(stream: &mut Stream) -> Result<Self, Error> {
        let count = stream.read_u16()?;
        let mut index = Vec::with_capacity(count as usize);

        for _ in 0..count {
            index.push(stream.read_u16()?)
        }

        Ok(Self { count, index })
    }
}
