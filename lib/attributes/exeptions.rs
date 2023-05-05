use crate::{
    error::Error,
    stream::{ReadStream, Stream},
};

#[derive(Debug, Clone)]
pub struct AttributeExeptions {
    pub number_of_exeptions: u16,
    pub exeptions_index_table: Vec<u16>,
}

impl AttributeExeptions {
    pub fn new(stream: &mut Stream) -> Result<Self, Error> {
        let number_of_exeptions = stream.read_u16()?;
        let mut exeptions_index_table = Vec::with_capacity(number_of_exeptions as usize);

        for _ in 0..number_of_exeptions {
            exeptions_index_table.push(stream.read_u16()?)
        }

        Ok(Self {
            number_of_exeptions,
            exeptions_index_table,
        })
    }
}
