use crate::{
    constant_info::Constant,
    error::Error,
    interpeter::{descriptor_vec, Interpeter},
    stream::ReadStream,
    types::Type,
};

impl<'a> Interpeter<'a> {
    pub fn invokespecial(&mut self) -> Result<(), Error> {
        let method_or_interfece_method_index = self.codes.read_u16()?;
        let m_or_i_m = &self.class.get_cp()[method_or_interfece_method_index as usize - 1];
        match m_or_i_m {
            Constant::InterfaceMethodref(a, b) => {
                let a = &self.class.get_cp()[*a as usize - 1];
                let b = &self.class.get_cp()[*b as usize - 1];
                match a {
                    Constant::Class(class_index) => {
                        let class_name = &self.class.get_cp()[*class_index as usize - 1];
                        if let Constant::Utf8(class_name) = class_name {
                            // println!("Class: {:?}", class_name);
                            match b {
                                Constant::NameAndType(name_index, descriptor_index) => {
                                    if let Constant::Utf8(name) =
                                        &self.class.get_cp()[*name_index as usize - 1]
                                    {
                                        if let Constant::Utf8(descriptor) =
                                            &self.class.get_cp()[*descriptor_index as usize - 1]
                                        {
                                            // println!("Name: {}", name);
                                            // println!("Descriptor: {}", descriptor);
                                            let des = descriptor_vec(&descriptor);
                                            let _ref = self.state.stack.pop().unwrap();
                                            if class_name.trim() == "java/lang/Object" {
                                                return Ok(());
                                            }
                                            // println!("Ref: {:?}", _ref);
                                            let mut arguments = Vec::new();
                                            arguments.push(_ref.clone());
                                            for _ in 0..des.0.len() {
                                                arguments.push(self.state.stack.pop().unwrap())
                                            }
                                            // println!("Arguments: {:?}", arguments);
                                            let mut _state =
                                                self.jvm.gen_state(class_name, name, arguments)?;
                                            if let Type::Ref(_ref) = _ref {
                                                let mut class_name = String::new();
                                                if let Some(_obj) =
                                                    self.jvm.heap.lock().unwrap().get(_ref as usize)
                                                {
                                                    if let Type::Object(class_nam, state) = _obj {
                                                        class_name = class_nam.clone();
                                                    }
                                                }
                                                self.jvm.execute_with_stack(
                                                    &class_name,
                                                    "<init>",
                                                    &mut _state,
                                                );
                                                return Ok(());
                                            } else {
                                            }
                                            self.jvm.execute_with_stack(
                                                class_name,
                                                name,
                                                &mut _state,
                                            )?;
                                        }
                                    }
                                }
                                _ => {
                                    panic!("Not implemented: {:?}", b)
                                }
                            }
                        }
                    }
                    _ => {
                        panic!("Not implemented: {:?}", a)
                    }
                }
            }
            _ => {
                println!("{:?}", m_or_i_m);
            }
        }
        Ok(())
    }
}
