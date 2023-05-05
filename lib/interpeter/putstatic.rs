use crate::{constant_info::Constant, error::Error, stream::ReadStream};

use super::Interpeter;

impl<'a> Interpeter<'a> {
    pub fn putstatic(&mut self) -> Result<(), Error> {
        let index = self.codes.read_u16()?;
        let a = &self.class.get_cp()[index as usize - 1];
        match a {
            Constant::Fieldref(x, z) => {
                let x = &self.class.get_cp()[*x as usize - 1];
                let z = &self.class.get_cp()[*z as usize - 1];

                match x {
                    Constant::Class(class_index) => match z {
                        Constant::NameAndType(name_index, descriptor_index) => {
                            if let Constant::Utf8(class_name) =
                                &self.class.get_cp()[*class_index as usize - 1]
                            {
                                if let Constant::Utf8(name) =
                                    &self.class.get_cp()[*name_index as usize - 1]
                                {
                                    if let Constant::Utf8(descriptor) =
                                        &self.class.get_cp()[*descriptor_index as usize - 1]
                                    {
                                        let s = format!("{}:{}", name, descriptor);
                                        let c = self.jvm.classes.get(class_name).unwrap();
                                        c.load().unwrap();
                                        self.jvm.init(class_name)?;

                                        let val = self.state.stack.pop().unwrap();
                                        c.get_state().lock().unwrap().statics.insert(s, val);
                                    }
                                }
                            }
                        }
                        _ => {
                            panic!("not: {:?}", z)
                        }
                    },
                    _ => {
                        panic!("is: {:?}", x)
                    }
                }
            }
            _ => {
                panic!("Put static is not implemented for: {:?}", a)
            }
        }
        Ok(())
    }
}
