use crate::{
    error::Error,
    stream::{ReadStream, Stream},
};

#[derive(Debug, Clone)]
pub enum Value {
    ConstantValueIndex(u16),
    EnumConstantValue(u16, u16),
    ClassInfoIndex(u16),
    Annotation(Annotation),
    ArrayValue(u16, Vec<ElementValue>),
}

#[derive(Debug, Clone)]
pub struct ElementValue {
    pub tag: u8,
    pub value: Value,
}

impl ElementValue {
    pub fn new(stream: &mut Stream) -> Result<Self, Error> {
        let tag = stream.read_u8()?;
        let value = match tag as char {
            'B' | 'C' | 'D' | 'F' | 'I' | 'J' | 'S' | 'Z' | 's' => {
                Value::ConstantValueIndex(stream.read_u16()?)
            }
            'e' => Value::EnumConstantValue(stream.read_u16()?, stream.read_u16()?),
            'c' => Value::ClassInfoIndex(stream.read_u16()?),
            '@' => Value::Annotation(Annotation::new(stream)?),
            '[' => {
                let len = stream.read_u16()?;
                let mut values = Vec::with_capacity(len as usize);
                for _ in 0..len {
                    values.push(ElementValue::new(stream)?)
                }
                Value::ArrayValue(len, values)
            }

            _ => panic!("Invalid Tag"),
        };

        Ok(Self { tag, value })
    }
}

#[derive(Debug, Clone)]
pub struct AnnotationPairs {
    pub element_name_index: u16,
    pub value: ElementValue,
}

impl AnnotationPairs {
    pub fn new(stream: &mut Stream) -> Result<Self, Error> {
        Ok(Self {
            element_name_index: stream.read_u16()?,
            value: ElementValue::new(stream)?,
        })
    }
}

#[derive(Debug, Clone)]
pub struct Annotation {
    pub type_index: u16,
    pub num_element_value_pairs: u16,
    pub element_value_pairs: Vec<AnnotationPairs>,
}

impl Annotation {
    pub fn new(stream: &mut Stream) -> Result<Self, Error> {
        let type_index = stream.read_u16()?;
        let num_element_value_pairs = stream.read_u16()?;
        let mut element_value_pairs = Vec::with_capacity(num_element_value_pairs as usize);

        for _ in 0..num_element_value_pairs {
            element_value_pairs.push(AnnotationPairs::new(stream)?)
        }

        Ok(Self {
            type_index,
            num_element_value_pairs,
            element_value_pairs,
        })
    }
}

#[derive(Debug, Clone)]
pub struct AttributeRuntimeVisibleAnnotations {
    pub num_annotations: u16,
    pub annotations: Vec<Annotation>,
}

impl AttributeRuntimeVisibleAnnotations {
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
