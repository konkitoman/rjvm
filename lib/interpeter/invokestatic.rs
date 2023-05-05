use crate::{
    attributes::Attribute, constant_info::Constant, error::Error, interpeter::descriptor_vec,
    jvm_state::JvmState, stream::ReadStream,
};

use super::Interpeter;

impl<'a> Interpeter<'a> {
    pub fn invokestatic(&mut self) -> Result<(), Error> {
        let index = self.codes.read_u16()?;

        let a = &self.class.get_cp()[index as usize - 1];
        match a {
            Constant::InterfaceMethodref(x, z) => {
                let a = &self.class.get_cp()[*x as usize - 1];
                let b = &self.class.get_cp()[*z as usize - 1];
                match a {
                    Constant::Class(class_name_index) => {
                        if let Constant::Utf8(class_name) =
                            &self.class.get_cp()[*class_name_index as usize - 1]
                        {
                            match b {
                                Constant::NameAndType(name_index, descriptor_index) => {
                                    if let Constant::Utf8(name) =
                                        &self.class.get_cp()[*name_index as usize - 1]
                                    {
                                        if let Constant::Utf8(descriptor) =
                                            &self.class.get_cp()[*descriptor_index as usize - 1]
                                        {
                                            // println!("Class Name: {}", class_name);
                                            // println!("Name: {}", name);
                                            // println!(
                                            //     "Descriptor: {:?}",
                                            //     descriptor_vec(&descriptor)
                                            // );
                                            let c = self.jvm.classes.get(class_name).unwrap();
                                            c.load()?;
                                            self.jvm.init(class_name)?;
                                            for method in c.get_methods() {
                                                if let Constant::Utf8(method_name) =
                                                    &c.get_cp()[method.name_index as usize - 1]
                                                {
                                                    if method_name.trim() == name.trim() {
                                                        for attr in method.attributes.iter() {
                                                            if let Attribute::Code(co_attr) =
                                                                &attr.attribute
                                                            {
                                                                let mut args = Vec::new();
                                                                let des =
                                                                    descriptor_vec(&descriptor);
                                                                for _ in 0..des.0.len() {
                                                                    args.push(
                                                                        self.state
                                                                            .stack
                                                                            .pop()
                                                                            .unwrap(),
                                                                    )
                                                                }

                                                                let mut tmp_state = JvmState::new(
                                                                    co_attr.max_stack,
                                                                    co_attr.max_locale,
                                                                    args,
                                                                );

                                                                self.jvm.execute_with_stack(
                                                                    class_name,
                                                                    name,
                                                                    &mut tmp_state,
                                                                )?;
                                                                for _ in 0..des.1.len() {
                                                                    self.state.stack.push(
                                                                        tmp_state
                                                                            .stack
                                                                            .pop()
                                                                            .unwrap(),
                                                                    )
                                                                }
                                                                break;
                                                            }
                                                        }

                                                        break;
                                                    }
                                                }
                                            }
                                        } else {
                                            panic!("")
                                        }
                                    } else {
                                        panic!("")
                                    }
                                }
                                _ => {
                                    panic!("")
                                }
                            }
                        }
                    }
                    _ => {
                        panic!("Not implemented for: {:?}", a)
                    }
                }
            }
            _ => {
                panic!("Not implemented for: {:?}", a)
            }
        }
        Ok(())
    }
}
