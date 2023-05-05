use crate::{
    error::Error,
    stream::{ReadStream, Stream},
};

#[derive(Debug, Clone)]
pub enum VerificationTypeInfo {
    Top,
    Intiger,
    Float,
    Null,
    UninitializatedThis,
    Object(u16),
    Uninitializated(u16),
    Long,
    Double,
}

impl VerificationTypeInfo {
    pub fn new(stream: &mut Stream) -> Result<Self, Error> {
        let tag = stream.read_u8()?;
        Ok(match tag {
            0 => Self::Top,
            1 => Self::Intiger,
            2 => Self::Float,
            3 => Self::Double,
            4 => Self::Long,
            5 => Self::Null,
            6 => Self::UninitializatedThis,
            7 => Self::Object(stream.read_u16()?),
            8 => Self::Uninitializated(stream.read_u16()?),
            _ => return Err(Error::InvalidVerificationType(tag)),
        })
    }
}

#[derive(Debug, Clone)]
pub struct StackMapFrameFullFrame {
    pub offset: u16,
    pub number_of_locals: u16,
    pub locals: Vec<VerificationTypeInfo>,
    pub number_of_stack_items: u16,
    pub stack: Vec<VerificationTypeInfo>,
}

impl StackMapFrameFullFrame {
    pub fn new(stream: &mut Stream) -> Result<Self, Error> {
        let offset = stream.read_u16()?;
        let number_of_locals = stream.read_u16()?;
        let mut locals = Vec::with_capacity(number_of_locals as usize);
        for _ in 0..number_of_locals {
            locals.push(VerificationTypeInfo::new(stream)?)
        }

        let number_of_stack_items = stream.read_u16()?;
        let mut stack = Vec::with_capacity(number_of_stack_items as usize);
        for _ in 0..number_of_stack_items {
            stack.push(VerificationTypeInfo::new(stream)?)
        }

        Ok(Self {
            offset,
            number_of_locals,
            locals,
            number_of_stack_items,
            stack,
        })
    }
}

#[derive(Debug, Clone)]
pub enum StackMapFrame {
    SameFrame,
    SameLocale1(VerificationTypeInfo),
    SameLocale1Extended(u16, VerificationTypeInfo),
    ChopFrame(u16),
    SameFrameExtended(u16),
    AppendFrame(u16, Vec<VerificationTypeInfo>),
    FillFrame(StackMapFrameFullFrame),
}

impl StackMapFrame {
    pub fn new(stream: &mut Stream) -> Result<Self, Error> {
        let _type = stream.read_u8()?;
        Ok(match _type {
            0..64 => Self::SameFrame,
            64..128 => Self::SameLocale1(VerificationTypeInfo::new(stream)?),
            247 => {
                Self::SameLocale1Extended(stream.read_u16()?, VerificationTypeInfo::new(stream)?)
            }
            248..251 => Self::ChopFrame(stream.read_u16()?),
            251 => Self::SameFrameExtended(stream.read_u16()?),
            252..255 => {
                let offset = stream.read_u16()?;
                let mut locals = Vec::with_capacity(_type as usize - 251);
                for _ in 0.._type - 251 {
                    locals.push(VerificationTypeInfo::new(stream)?)
                }
                Self::AppendFrame(offset, locals)
            }
            255 => Self::FillFrame(StackMapFrameFullFrame::new(stream)?),
            _ => return Err(Error::InvalidStackMapFrame(_type)),
        })
    }
}

#[derive(Debug, Clone)]
pub struct AttributeStackMapTable {
    pub number_of_entris: u16,
    pub entries: Vec<StackMapFrame>,
}

impl AttributeStackMapTable {
    pub fn new(stream: &mut Stream) -> Result<Self, Error> {
        let number_of_entris = stream.read_u16()?;
        let mut entries = Vec::with_capacity(number_of_entris as usize);

        for _ in 0..number_of_entris {
            entries.push(StackMapFrame::new(stream)?)
        }

        Ok(Self {
            number_of_entris,
            entries,
        })
    }
}
