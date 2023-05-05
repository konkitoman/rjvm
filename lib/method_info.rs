use crate::{
    attribute_info::AttributeInfo,
    constant_info::Constant,
    error::Error,
    stream::{ReadStream, Stream},
};

#[derive(Debug, Clone)]
pub struct MethodInfo {
    pub access_flags: u16,
    pub name_index: u16,
    pub descriptor_index: u16,
    pub attributes_count: u16,
    pub attributes: Vec<AttributeInfo>,
}

impl MethodInfo {
    pub fn new(stream: &mut Stream, constant_pool: &[Constant]) -> Result<Self, Error> {
        let access_flags = stream.read_u16()?;
        let name_index = stream.read_u16()?;
        let descriptor_index = stream.read_u16()?;
        let attributes_count = stream.read_u16()?;
        let mut attributes = Vec::with_capacity(attributes_count as usize);

        for _ in 0..attributes_count {
            attributes.push(AttributeInfo::new(&mut *stream, constant_pool)?);
        }

        Ok(Self {
            access_flags,
            name_index,
            descriptor_index,
            attributes_count,
            attributes,
        })
    }
}
