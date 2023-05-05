use crate::{
    error::Error,
    stream::{ReadStream, Stream},
};

use super::runtime_visible_annotations::Annotation;

#[derive(Debug, Clone)]
pub struct AttributeRuntimeVisibleTypeAnnotations {
    pub num: u16,
    pub annotations: Vec<Annotation>,
}

impl AttributeRuntimeVisibleTypeAnnotations {
    pub fn new(stream: &mut Stream) -> Result<Self, Error> {
        let num = stream.read_u16()?;
        let mut annotations = Vec::with_capacity(num as usize);

        for _ in 0..num {
            annotations.push(Annotation::new(stream)?)
        }

        Ok(Self { num, annotations })
    }
}
