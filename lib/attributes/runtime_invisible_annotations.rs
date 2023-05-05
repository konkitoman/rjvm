use crate::{
    error::Error,
    stream::{ReadStream, Stream},
};

use super::runtime_visible_annotations::Annotation;

#[derive(Debug, Clone)]
pub struct AttributeRuntimeInvisibleAnnotations {
    pub num_annotations: u16,
    pub annotations: Vec<Annotation>,
}

impl AttributeRuntimeInvisibleAnnotations {
    pub fn new(stream: &mut Stream) -> Result<Self, Error> {
        let num_annotations = stream.read_u16()?;
        let mut annotations = Vec::with_capacity(num_annotations as usize);

        for _ in 0..num_annotations {
            annotations.push(Annotation::new(stream)?)
        }

        Ok(Self {
            num_annotations,
            annotations,
        })
    }
}
