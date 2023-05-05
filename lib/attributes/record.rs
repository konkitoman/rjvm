use crate::{
    attribute_info::AttributeInfo,
    constant_info::Constant,
    error::Error,
    stream::{ReadStream, Stream},
};

#[derive(Debug, Clone)]
pub struct RecordComponent {
    pub name_index: u16,
    pub descriptor_index: u16,
    pub attributes_count: u16,
    pub attributes: Vec<AttributeInfo>,
}

impl RecordComponent {
    pub fn new(stream: &mut Stream, constant_pool: &[Constant]) -> Result<Self, Error> {
        let name_index = stream.read_u16()?;
        let descriptor_index = stream.read_u16()?;
        let attributes_count = stream.read_u16()?;
        let mut attributes = Vec::with_capacity(attributes_count as usize);

        for _ in 0..attributes_count {
            attributes.push(AttributeInfo::new(stream, constant_pool)?)
        }

        Ok(Self {
            name_index,
            descriptor_index,
            attributes_count,
            attributes,
        })
    }
}

#[derive(Debug, Clone)]
pub struct AttributeRecord {
    pub component_count: u16,
    pub components: Vec<RecordComponent>,
}

impl AttributeRecord {
    pub fn new(stream: &mut Stream, constant_pool: &[Constant]) -> Result<Self, Error> {
        let component_count = stream.read_u16()?;
        let mut components = Vec::with_capacity(component_count as usize);

        for _ in 0..component_count {
            components.push(RecordComponent::new(stream, constant_pool)?)
        }

        Ok(Self {
            component_count,
            components,
        })
    }
}
