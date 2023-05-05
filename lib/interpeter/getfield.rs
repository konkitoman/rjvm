use crate::{
    constant_info::Constant, error::Error, interpeter::descriptor_vec, stream::ReadStream,
    types::Type,
};

use super::Interpeter;

impl<'a> Interpeter<'a> {
    pub fn getfield(&mut self) -> Result<(), Error> {
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
                                        // println!("Name class {}", class_name);
                                        // println!("Name: {}", name);
                                        // println!("Descriptor: {}", descriptor);
                                        let s = format!("{}:{}", name, descriptor);
                                        let c = self.jvm.classes.get(class_name).unwrap();
                                        c.load().unwrap();
                                        self.jvm.init(class_name);

                                        let desc = descriptor_vec(descriptor);
                                        let _ref = self.state.stack.pop().unwrap();
                                        if let Type::Ref(_ref) = _ref {
                                            match &mut self.jvm.heap.lock().unwrap()
                                                [_ref as usize - 1]
                                            {
                                                Type::Object(n, state) => {
                                                    self.state.stack.push(
                                                        state.fields.get(&s).unwrap().clone(),
                                                    );
                                                }
                                                _ => {
                                                    panic!()
                                                }
                                            }
                                        } else {
                                            panic!()
                                        }
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
