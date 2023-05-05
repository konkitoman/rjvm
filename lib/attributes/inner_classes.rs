use crate::{
    error::Error,
    stream::{ReadStream, Stream},
};

#[derive(Debug, Clone)]
pub struct Class {
    pub inner_class_info_index: u16,
    pub outer_class_info_index: u16,
    pub inner_name_index: u16,
    pub inner_class_access_flags: u16,
}

impl Class {
    pub fn new(stream: &mut Stream) -> Result<Self, Error> {
        Ok(Self {
            inner_class_info_index: stream.read_u16()?,
            outer_class_info_index: stream.read_u16()?,
            inner_name_index: stream.read_u16()?,
            inner_class_access_flags: stream.read_u16()?,
        })
    }
}

#[derive(Debug, Clone)]
pub struct AttributeInnerClasses {
    pub number_of_classes: u16,
    pub classes: Vec<Class>,
}

impl AttributeInnerClasses {
    pub fn new(stream: &mut Stream) -> Result<Self, Error> {
        let number_of_classes = stream.read_u16()?;
        let mut classes = Vec::with_capacity(number_of_classes as usize);

        for _ in 0..number_of_classes {
            classes.push(Class::new(stream)?)
        }

        Ok(Self {
            number_of_classes,
            classes,
        })
    }
}
