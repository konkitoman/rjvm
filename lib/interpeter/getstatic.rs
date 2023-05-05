use crate::{constant_info::Constant, error::Error, stream::ReadStream};

use super::Interpeter;

impl<'a> Interpeter<'a> {
    pub fn getstatic(&mut self) -> Result<(), Error> {
        let index = self.codes.read_u16()?;
        if let Constant::Fieldref(index, name_and_type_index) =
            &self.class.get_cp()[index as usize - 1]
        {
            let class_or_interface = &self.class.get_cp()[*index as usize - 1];
            let name_and_type = &self.class.get_cp()[*name_and_type_index as usize - 1];
            match class_or_interface {
                Constant::Class(class_index) => {
                    if let Constant::Utf8(class_name) =
                        &self.class.get_cp()[*class_index as usize - 1]
                    {
                        if let Constant::NameAndType(name_index, descriptor_index) = name_and_type {
                            if let Constant::Utf8(name) =
                                &self.class.get_cp()[*name_index as usize - 1]
                            {
                                if let Constant::Utf8(descriptor) =
                                    &self.class.get_cp()[*descriptor_index as usize - 1]
                                {
                                    let s = format!("{}:{}", name, descriptor);
                                    // println!("CL: {}", class_name);
                                    let c = self.jvm.classes.get(class_name).unwrap();
                                    c.load()?;
                                    self.jvm.init(&class_name)?;
                                    if let Some(var) = c.get_state().lock().unwrap().statics.get(&s)
                                    {
                                        self.state.stack.push(var.clone());
                                    } else {
                                        panic!("Statid do not exist")
                                    }
                                } else {
                                    panic!("Is not Utf8")
                                }
                            } else {
                                panic!("Is not Utf8")
                            }
                        } else {
                            panic!("is not name_and_type: {:?}", name_and_type)
                        }
                    } else {
                        panic!("Is not a Utf8")
                    }
                }
                _ => {
                    panic!("Getstati is not implemented for: {:?}", class_or_interface)
                }
            }
        }
        Ok(())
    }
}
