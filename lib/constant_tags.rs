#[derive(Debug, Clone)]
pub enum ConstantTags {
    Utf8,
    Intiger,
    Float,
    Long,
    Double,
    Class,
    String,
    Filedref,
    Methodref,
    InterfaceMethodref,
    NameAndType,
    MethodHandle,
    MethodType,
    Dynamic,
    InvokeDynamic,
    Module,
    Package,
    Invalid(u8),
}

impl From<u8> for ConstantTags {
    fn from(value: u8) -> Self {
        match value {
            1 => Self::Utf8,
            3 => Self::Intiger,
            4 => Self::Float,
            5 => Self::Long,
            6 => Self::Double,
            7 => Self::Class,
            8 => Self::String,
            9 => Self::Filedref,
            10 => Self::Methodref,
            11 => Self::InterfaceMethodref,
            12 => Self::NameAndType,
            15 => Self::MethodHandle,
            16 => Self::MethodType,
            17 => Self::Dynamic,
            18 => Self::InvokeDynamic,
            19 => Self::Module,
            20 => Self::Package,
            _ => Self::Invalid(value),
        }
    }
}
