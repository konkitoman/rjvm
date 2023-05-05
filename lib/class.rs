use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use crate::{
    attribute_info::AttributeInfo,
    constant_info::Constant,
    error::Error,
    field_info::FieldInfo,
    java_class::JavaClass,
    method_info::MethodInfo,
    stream::{FileStream, Stream},
    types::Type,
};

#[derive(Debug)]
pub struct ClassStaticState {
    pub statics: HashMap<String, Type>,
    pub instanciated: bool,
}

impl ClassStaticState {
    pub fn new() -> Self {
        Self {
            statics: HashMap::new(),
            instanciated: false,
        }
    }
}

#[derive(Debug)]
pub enum Class {
    JavaClass(JavaClass, Arc<Mutex<ClassStaticState>>),
    LazyClass(Stream),
}

impl TryFrom<&str> for Class {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(Self::new(Stream::FileStream(FileStream::try_from(value)?)))
    }
}

#[allow(dead_code)]
impl Class {
    pub fn new(stream: Stream) -> Self {
        Self::LazyClass(stream)
    }

    pub fn load(&self) -> Result<(), Error> {
        if let Self::LazyClass(stream) = self {
            let stream = unsafe { std::ptr::read(stream) };
            unsafe {
                std::ptr::write(
                    self as *const _ as *mut _,
                    Self::JavaClass(
                        JavaClass::new(stream)?,
                        Arc::new(Mutex::new(ClassStaticState::new())),
                    ),
                )
            }
        }
        Ok(())
    }

    pub fn get_cp(&self) -> &Vec<Constant> {
        self.load().unwrap();
        if let Self::JavaClass(class, _) = self {
            return &class.constant_pool;
        }
        {
            panic!("")
        }
    }

    pub fn get_access_flags(&self) -> u16 {
        self.load().unwrap();
        if let Self::JavaClass(class, _) = self {
            return class.access_flags;
        }
        {
            panic!("")
        }
    }

    pub fn get_methods(&self) -> &Vec<MethodInfo> {
        self.load().unwrap();
        if let Self::JavaClass(class, _) = self {
            return &class.methods;
        }
        {
            panic!("")
        }
    }

    pub fn get_interfaces(&self) -> &Vec<u16> {
        self.load().unwrap();
        if let Self::JavaClass(class, _) = self {
            return &class.interfaces;
        }
        {
            panic!("")
        }
    }

    pub fn get_fields(&self) -> &Vec<FieldInfo> {
        self.load().unwrap();
        if let Self::JavaClass(class, _) = self {
            return &class.fields;
        }
        {
            panic!("")
        }
    }

    pub fn get_attributes(&self) -> &Vec<AttributeInfo> {
        self.load().unwrap();
        if let Self::JavaClass(class, _) = self {
            return &class.attributes;
        }
        {
            panic!("")
        }
    }

    pub fn get_this_class(&self) -> u16 {
        self.load().unwrap();
        if let Self::JavaClass(class, _) = self {
            return class.this_class;
        }
        {
            panic!("")
        }
    }

    pub fn get_super_class(&self) -> u16 {
        self.load().unwrap();
        if let Self::JavaClass(class, _) = self {
            return class.super_class;
        }
        {
            panic!("")
        }
    }

    pub fn get_state(&self) -> Arc<Mutex<ClassStaticState>> {
        if let Self::JavaClass(_, state) = self {
            state.clone()
        } else {
            panic!("")
        }
    }
}
