use crate::{
    error::Error,
    stream::{ReadStream, Stream},
};

use super::runtime_visible_annotations::Annotation;

#[derive(Debug, Clone)]
pub struct RuntimeInvisibleParameterAnnotations {
    pub num_annotations: u16,
    pub annotations: Vec<Annotation>,
}

impl RuntimeInvisibleParameterAnnotations {
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

#[derive(Debug, Clone)]
pub struct AttributeRuntimeInvisibleParameterAnnotations {
    pub num_parameters: u8,
    pub parameters_annotations: Vec<RuntimeInvisibleParameterAnnotations>,
}

impl AttributeRuntimeInvisibleParameterAnnotations {
    pub fn new(stream: &mut Stream) -> Result<Self, Error> {
        let num_parameters = stream.read_u8()?;
        let mut parameters_annotations = Vec::with_capacity(num_parameters as usize);

        for _ in 0..num_parameters {
            parameters_annotations.push(RuntimeInvisibleParameterAnnotations::new(stream)?)
        }

        Ok(Self {
            num_parameters,
            parameters_annotations,
        })
    }
}
