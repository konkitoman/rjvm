use crate::{
    attribute_info::AttributeInfo,
    constant_info::Constant,
    constant_tags::ConstantTags,
    error::Error,
    field_info::FieldInfo,
    method_info::MethodInfo,
    stream::{ReadStream, Stream},
};

#[derive(Debug)]
pub struct JavaClass {
    pub minor: u16,
    pub major: u16,
    pub constant_pool: Vec<Constant>,
    pub access_flags: u16,
    pub this_class: u16,
    pub super_class: u16,
    pub interfaces: Vec<u16>,
    pub fields: Vec<FieldInfo>,
    pub methods: Vec<MethodInfo>,
    pub attributes: Vec<AttributeInfo>,
}

impl JavaClass {
    pub fn new(stream: Stream) -> Result<Self, Error> {
        let mut stream = stream;
        let magic = stream.read::<4>()?;

        if magic != [0xCA, 0xFE, 0xBA, 0xBE] {
            return Err(Error::InvalidMagic);
        }

        let minor = stream.read_u16()?;
        let major = stream.read_u16()?;

        let constant_pool_count = stream.read_i16()?;

        let mut constant_pool: Vec<Constant> = Vec::with_capacity(constant_pool_count as usize);

        let mut index = 1;
        while index < constant_pool_count {
            let tag: ConstantTags = stream.read_u8()?.into();
            match tag {
                ConstantTags::Utf8 => {
                    let length = stream.read_u16()?;
                    let mut bytes = Vec::with_capacity(length as usize);
                    for _ in 0..length {
                        bytes.push(stream.read_u8()?);
                    }

                    match String::from_utf8(bytes) {
                        Ok(s) => constant_pool.push(Constant::Utf8(s)),
                        Err(_) => constant_pool.push(Constant::None),
                    }
                }
                ConstantTags::Intiger => constant_pool.push(Constant::Intiger(stream.read_i32()?)),
                ConstantTags::Float => constant_pool.push(Constant::Float(stream.read_f32()?)),
                ConstantTags::Long => {
                    constant_pool.push(Constant::Long(stream.read_i64()?));
                    index += 1;
                    constant_pool.push(Constant::None);
                }
                ConstantTags::Double => {
                    constant_pool.push(Constant::Double(stream.read_f64()?));
                    index += 1;
                    constant_pool.push(Constant::None);
                }
                ConstantTags::Class => constant_pool.push(Constant::Class(stream.read_u16()?)),
                ConstantTags::String => constant_pool.push(Constant::String(stream.read_u16()?)),
                ConstantTags::Filedref => {
                    constant_pool.push(Constant::Fieldref(stream.read_u16()?, stream.read_u16()?))
                }
                ConstantTags::Methodref => constant_pool.push(Constant::InterfaceMethodref(
                    stream.read_u16()?,
                    stream.read_u16()?,
                )),
                ConstantTags::InterfaceMethodref => constant_pool.push(
                    Constant::InterfaceMethodref(stream.read_u16()?, stream.read_u16()?),
                ),
                ConstantTags::NameAndType => constant_pool.push(Constant::NameAndType(
                    stream.read_u16()?,
                    stream.read_u16()?,
                )),
                ConstantTags::MethodHandle => constant_pool.push(Constant::MethodHandle(
                    stream.read_u8()?,
                    stream.read_u16()?,
                )),
                ConstantTags::MethodType => {
                    constant_pool.push(Constant::MethodType(stream.read_u16()?))
                }
                ConstantTags::Dynamic => {
                    constant_pool.push(Constant::Dynamic(stream.read_u16()?, stream.read_u16()?))
                }
                ConstantTags::InvokeDynamic => constant_pool.push(Constant::InvokeDynamic(
                    stream.read_u16()?,
                    stream.read_u16()?,
                )),
                ConstantTags::Module => constant_pool.push(Constant::Module(stream.read_u16()?)),
                ConstantTags::Package => constant_pool.push(Constant::Package(stream.read_u16()?)),
                ConstantTags::Invalid(a) => {
                    return Err(Error::InvalidConstantTag(a));
                }
            }

            index += 1;
        }

        let access_flags = stream.read_u16()?;

        let this_class = stream.read_u16()?;

        let super_class = stream.read_u16()?;

        let interfaces_count = stream.read_u16()?;
        let mut interfaces = Vec::with_capacity(interfaces_count as usize);
        for _ in 0..interfaces_count {
            interfaces.push(stream.read_u16()?)
        }

        let fields_count = stream.read_u16()?;
        let mut fields = Vec::with_capacity(fields_count as usize);

        for _ in 0..fields_count {
            fields.push(FieldInfo::new(&mut stream, &constant_pool)?);
        }

        let methods_count = stream.read_u16()?;
        let mut methods = Vec::with_capacity(methods_count as usize);

        for _ in 0..methods_count {
            methods.push(MethodInfo::new(&mut stream, &constant_pool)?);
        }

        let attributes_count = stream.read_u16()?;
        let mut attributes = Vec::with_capacity(attributes_count as usize);

        for _ in 0..attributes_count {
            attributes.push(AttributeInfo::new(&mut stream, &constant_pool)?);
        }

        Ok(Self {
            minor,
            major,
            constant_pool,
            access_flags,
            this_class,
            super_class,
            interfaces,
            fields,
            methods,
            attributes,
        })
    }
}
