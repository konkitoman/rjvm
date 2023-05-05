use crate::{
    error::Error,
    stream::{ReadStream, Stream},
};

#[derive(Debug, Clone)]
pub struct ModuleRequire {
    pub index: u16,
    pub flags: u16,
    pub version_index: u16,
}

impl ModuleRequire {
    pub fn new(stream: &mut Stream) -> Result<Self, Error> {
        Ok(Self {
            index: stream.read_u16()?,
            flags: stream.read_u16()?,
            version_index: stream.read_u16()?,
        })
    }
}

#[derive(Debug, Clone)]
pub struct ModuleExport {
    pub index: u16,
    pub flags: u16,
    pub to_count: u16,
    pub to_index: Vec<u16>,
}

impl ModuleExport {
    pub fn new(stream: &mut Stream) -> Result<Self, Error> {
        let index = stream.read_u16()?;
        let flags = stream.read_u16()?;
        let to_count = stream.read_u16()?;
        let mut to_index = Vec::with_capacity(to_count as usize);

        for _ in 0..to_count {
            to_index.push(stream.read_u16()?)
        }

        Ok(Self {
            index,
            flags,
            to_count,
            to_index,
        })
    }
}

#[derive(Debug, Clone)]
pub struct ModuleOpen {
    pub index: u16,
    pub flags: u16,
    pub to_count: u16,
    pub to_index: Vec<u16>,
}

impl ModuleOpen {
    pub fn new(stream: &mut Stream) -> Result<Self, Error> {
        let index = stream.read_u16()?;
        let flags = stream.read_u16()?;
        let to_count = stream.read_u16()?;
        let mut to_index = Vec::with_capacity(to_count as usize);

        for _ in 0..to_count {
            to_index.push(stream.read_u16()?)
        }

        Ok(Self {
            index,
            flags,
            to_count,
            to_index,
        })
    }
}
#[derive(Debug, Clone)]
pub struct ModuleProvide {
    pub index: u16,
    pub with_count: u16,
    pub with_index: Vec<u16>,
}

impl ModuleProvide {
    pub fn new(stream: &mut Stream) -> Result<Self, Error> {
        let index = stream.read_u16()?;
        let with_count = stream.read_u16()?;
        let mut with_index = Vec::with_capacity(with_count as usize);

        for _ in 0..with_count {
            with_index.push(stream.read_u16()?)
        }

        Ok(Self {
            index,
            with_count,
            with_index,
        })
    }
}

#[derive(Debug, Clone)]
pub struct AttributeModule {
    pub module_name_index: u16,
    pub module_flags: u16,
    pub module_version_index: u16,

    pub requires_count: u16,
    pub requires: Vec<ModuleRequire>,

    pub exports_count: u16,
    pub exports: Vec<ModuleExport>,

    pub opens_count: u16,
    pub opens: Vec<ModuleOpen>,

    pub uses_count: u16,
    pub uses_index: Vec<u16>,

    pub providers_count: u16,
    pub providers: Vec<ModuleProvide>,
}

impl AttributeModule {
    pub fn new(stream: &mut Stream) -> Result<Self, Error> {
        let module_name_index = stream.read_u16()?;
        let module_flags = stream.read_u16()?;
        let module_version_index = stream.read_u16()?;

        let requires_count = stream.read_u16()?;
        let mut requires = Vec::with_capacity(requires_count as usize);

        for _ in 0..requires_count {
            requires.push(ModuleRequire::new(stream)?)
        }

        let exports_count = stream.read_u16()?;
        let mut exports = Vec::with_capacity(exports_count as usize);

        for _ in 0..exports_count {
            exports.push(ModuleExport::new(stream)?)
        }

        let opens_count = stream.read_u16()?;
        let mut opens = Vec::with_capacity(opens_count as usize);

        for _ in 0..opens_count {
            opens.push(ModuleOpen::new(stream)?)
        }

        let uses_count = stream.read_u16()?;
        let mut uses_index = Vec::with_capacity(uses_count as usize);

        for _ in 0..uses_count {
            uses_index.push(stream.read_u16()?)
        }

        let providers_count = stream.read_u16()?;
        let mut providers = Vec::with_capacity(providers_count as usize);

        for _ in 0..providers_count {
            providers.push(ModuleProvide::new(stream)?)
        }

        Ok(Self {
            module_name_index,
            module_flags,
            module_version_index,
            requires_count,
            requires,
            exports_count,
            exports,
            opens_count,
            opens,
            uses_count,
            uses_index,
            providers_count,
            providers,
        })
    }
}
