use crate::{
    error::Error,
    stream::{ReadStream, Stream},
};

#[derive(Debug, Clone)]
pub struct AttributeNestMembers {
    pub number_of_classes: u16,
    pub classes: Vec<u16>,
}

impl AttributeNestMembers {
    pub fn new(stream: &mut Stream) -> Result<Self, Error> {
        let number_of_classes = stream.read_u16()?;
        let mut classes = Vec::with_capacity(number_of_classes as usize);

        for _ in 0..number_of_classes {
            classes.push(stream.read_u16()?)
        }

        Ok(Self {
            number_of_classes,
            classes,
        })
    }
}
