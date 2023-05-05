use crate::{
    error::Error,
    stream::{ReadStream, Stream},
};

#[derive(Debug, Clone)]
pub struct BootstrapMethods {
    pub bootstrap_method_ref: u16,
    pub num_bootstrap_arguments: u16,
    pub bootstrap_arguments: Vec<u16>,
}

impl BootstrapMethods {
    pub fn new(stream: &mut Stream) -> Result<Self, Error> {
        let bootstrap_method_ref = stream.read_u16()?;
        let num_bootstrap_arguments = stream.read_u16()?;
        let mut bootstrap_arguments = Vec::with_capacity(num_bootstrap_arguments as usize);
        for _ in 0..num_bootstrap_arguments {
            bootstrap_arguments.push(stream.read_u16()?)
        }

        Ok(Self {
            bootstrap_method_ref,
            num_bootstrap_arguments,
            bootstrap_arguments,
        })
    }
}

#[derive(Debug, Clone)]
pub struct AttributeBootstrapMethods {
    pub num_bootstrap_methods: u16,
    pub bootstrap_methods: Vec<BootstrapMethods>,
}

impl AttributeBootstrapMethods {
    pub fn new(stream: &mut Stream) -> Result<Self, Error> {
        let num_bootstrap_methods = stream.read_u16()?;
        let mut bootstrap_methods = Vec::with_capacity(num_bootstrap_methods as usize);

        for _ in 0..num_bootstrap_methods {
            bootstrap_methods.push(BootstrapMethods::new(stream)?)
        }

        Ok(Self {
            num_bootstrap_methods,
            bootstrap_methods,
        })
    }
}
