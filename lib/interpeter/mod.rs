use crate::{
    class::Class,
    constant_info::Constant,
    error::Error,
    jvm::Jvm,
    jvm_state::JvmState,
    opcodes::Opcodes,
    stream::{BytesStream, ReadStream},
    types::{ClassState, Type, TypeTag},
    Log, LogDone,
};

mod getfield;
mod getstatic;
mod invokespecial;
mod invokestatic;
mod putfield;
mod putstatic;

use crate::prelude::*;

pub struct Interpeter<'a> {
    pub codes: &'a mut BytesStream,
    pub jvm: &'a Jvm,
    pub class: &'a Class,
    pub state: &'a mut JvmState,
}

impl<'a> Interpeter<'a> {
    pub fn aload(&mut self, index: u8) {
        self.state.stack.push(Type::Ref(index as u16));
    }
    pub fn astore(&mut self, index: u8) {
        self.state.local_variabiles[index as usize] = self.state.stack.pop().unwrap()
    }
    pub fn iconst(&mut self, index: i32) {
        self.state.stack.push(Type::Int(index))
    }
    pub fn istore(&mut self, index: u8) {
        if let Some(Type::Int(val)) = self.state.stack.pop() {
            self.state.local_variabiles[index as usize] = Type::Int(val)
        } else {
            panic!("Stack last iten is not a int!")
        }
    }

    pub fn iload(&mut self, index: u8) {
        if let Type::Int(val) = &self.state.local_variabiles[index as usize] {
            self.state.stack.push(Type::Int(*val))
        } else {
            panic!("Local vairabile is not int");
        }
    }
    pub fn interpeter(&mut self) -> Result<(), Error> {
        let opcode: Opcodes = self.codes.read_u8().unwrap().into();

        Log!("Executing opcode", "", opcode.clone());

        match opcode {
            Opcodes::Aload => {
                let index = self.codes.read_u8()?;
                self.aload(index)
            }
            Opcodes::Aload0 => self.aload(0),
            Opcodes::Aload1 => self.aload(1),
            Opcodes::Aload2 => self.aload(2),
            Opcodes::Aload3 => self.aload(3),
            Opcodes::Astore => {
                let index = self.codes.read_u8()?;
                self.astore(index)
            }
            Opcodes::Astore0 => self.astore(0),
            Opcodes::Astore1 => self.astore(1),
            Opcodes::Astore2 => self.astore(2),
            Opcodes::Astore3 => self.astore(3),
            Opcodes::Invokespecial => self.invokespecial()?,
            Opcodes::Return => self.state.stack.clear(),
            Opcodes::IconstM1 => self.iconst(-1),
            Opcodes::Iconst0 => self.iconst(0),
            Opcodes::Iconst1 => self.iconst(1),
            Opcodes::Iconst2 => self.iconst(2),
            Opcodes::Iconst3 => self.iconst(3),
            Opcodes::Iconst4 => self.iconst(4),
            Opcodes::Iconst5 => self.iconst(5),
            Opcodes::Istore0 => self.istore(0),
            Opcodes::Istore1 => self.istore(1),
            Opcodes::Istore2 => self.istore(2),
            Opcodes::Istore3 => self.istore(3),
            Opcodes::Istore => {
                let index = self.codes.read_u8()?;
                self.istore(index)
            }
            Opcodes::Iload0 => self.iload(0),
            Opcodes::Iload1 => self.iload(1),
            Opcodes::Iload2 => self.iload(2),
            Opcodes::Iload3 => self.iload(3),
            Opcodes::Lload => {
                let index = self.codes.read_u8()?;
                self.iload(index)
            }
            Opcodes::Bipush => {
                let byte = self.codes.read_i8()?;
                self.state.stack.push(Type::Int(byte as i32));
            }
            Opcodes::Sipush => {
                let val = self.codes.read_i16()?;
                self.state.stack.push(Type::Int(val as i32));
            }
            Opcodes::Ldc => {
                let index = self.codes.read_u8()?;
                match &self.class.get_cp()[index as usize - 1] {
                    Constant::Intiger(val) => self.state.stack.push(Type::Int(*val)),
                    Constant::Float(val) => self.state.stack.push(Type::Float(*val)),
                    Constant::Long(val) => self.state.stack.push(Type::Long(*val)),
                    Constant::Double(val) => self.state.stack.push(Type::Double(*val)),
                    _ => {
                        panic!("Invalid constant")
                    }
                }
            }
            Opcodes::Getstatic => self.getstatic()?,
            Opcodes::Putstatic => self.putstatic()?,
            Opcodes::Getfield => self.getfield()?,
            Opcodes::Putfield => self.putfield()?,
            Opcodes::Invokestatic => self.invokestatic()?,
            Opcodes::Ireturn => {
                let last = self.state.stack.pop().unwrap();
                self.state.stack.clear();
                self.state.stack.push(last)
            }
            Opcodes::Iadd => {
                if let Some(Type::Int(b)) = self.state.stack.pop() {
                    if let Some(Type::Int(a)) = self.state.stack.pop() {
                        self.state.stack.push(Type::Int(a + b))
                    } else {
                        panic!("Stack last item is not int!")
                    }
                } else {
                    panic!("Stack last item is not int!")
                }
            }
            Opcodes::New => {
                // println!("{:?}", self.state);
                let index = self.codes.read_u16()?;
                let a = &self.class.get_cp()[index as usize - 1];
                match a {
                    Constant::Class(class_index) => {
                        if let Constant::Utf8(class_name) =
                            &self.class.get_cp()[*class_index as usize - 1]
                        {
                            let mut heap = self.jvm.heap.lock().unwrap();
                            let index = heap.len();
                            heap.push(Type::Object(class_name.clone(), ClassState::default()));
                            self.state.stack.push(Type::Ref(index as u16));
                            // let c = self.jvm.classes.get(class_name).unwrap();
                            // for method in c.get_methods() {
                            //     if let Constant::Utf8(method_name) =
                            //         &c.get_cp()[method.name_index as usize - 1]
                            //     {
                            //         if let Constant::Utf8(descriptor) =
                            //             &c.get_cp()[method.descriptor_index as usize - 1]
                            //         {
                            //             if method_name == "<init>" {
                            //                 let desc = descriptor_vec(descriptor);
                            //                 println!("Desc: {:?}", desc);
                            //                 println!("Name: {}", method_name);

                            //                 // for _ in 0..desc.0.len() {
                            //                 //     arguments.push(self.state.stack.pop().unwrap())
                            //                 // }

                            //                 self.jvm.execute_new(class_name, arguments);
                            //                 break;
                            //             }
                            //         }
                            //     }
                            // }
                        }
                    }
                    _ => {
                        panic!()
                    }
                }
            }
            Opcodes::Newarray => {
                let _type = match self.codes.read_u8()? {
                    4 => {
                        //boolean
                        TypeTag::Int
                    }
                    5 => {
                        //char
                        TypeTag::Int
                    }
                    6 => TypeTag::Float,
                    7 => TypeTag::Double,
                    8 => {
                        // byte
                        TypeTag::Int
                    }
                    9 => {
                        // short
                        TypeTag::Int
                    }
                    10 => TypeTag::Int,
                    11 => TypeTag::Long,
                    _ => {
                        panic!("Invalid type")
                    }
                };

                if let Type::Int(len) = self.state.stack.pop().unwrap() {
                    let mut vec = Vec::with_capacity(len as usize);
                    for _ in 0..len {
                        match _type {
                            TypeTag::Int => vec.push(Type::Int(0)),
                            TypeTag::Long => vec.push(Type::Long(0)),
                            TypeTag::Float => vec.push(Type::Float(0.)),
                            TypeTag::Double => vec.push(Type::Double(0.)),
                            TypeTag::L(_) => todo!(),
                        }
                    }
                    let index = self.jvm.heap.lock().unwrap().len();
                    self.jvm.heap.lock().unwrap().push(Type::Array(vec));
                    self.state.stack.push(Type::Ref(index as u16));
                } else {
                    panic!("Length is not int")
                }
            }
            Opcodes::Dup => self
                .state
                .stack
                .push(self.state.stack.last().unwrap().clone()),
            Opcodes::Iastore => {
                let value = self.state.stack.pop().unwrap();
                let index = self.state.stack.pop().unwrap();
                let array_ref = self.state.stack.pop().unwrap();
                if let Type::Int(index) = index {
                    if let Type::Ref(array_ref) = array_ref {
                        if let Type::Array(array) =
                            &mut self.jvm.heap.lock().unwrap()[array_ref as usize]
                        {
                            array[index as usize] = value;
                        } else {
                            panic!()
                        }
                    } else {
                        panic!()
                    }
                }
            }
            Opcodes::Iaload => {
                let index = self.state.stack.pop().unwrap();
                let array_ref = self.state.stack.pop().unwrap();
                if let Type::Int(index) = index {
                    if let Type::Ref(array_ref) = array_ref {
                        let mut array_ref = array_ref;
                        if let Type::Ref(_ref) = &self.jvm.heap.lock().unwrap()[array_ref as usize]
                        {
                            array_ref = *_ref;
                        }
                        if let Type::Array(array) =
                            &self.jvm.heap.lock().unwrap()[array_ref as usize]
                        {
                            self.state.stack.push(array[index as usize].clone())
                        } else {
                            panic!()
                        }
                    } else {
                        panic!()
                    }
                } else {
                    panic!()
                }
            }

            _ => {
                panic!("Optcode not implemented: {:?}", opcode)
            }
        }
        LogDone!();
        Ok(())
    }
}

fn descriptor_vec(data: &str) -> (Vec<TypeTag>, Vec<TypeTag>) {
    let mut _in = Vec::new();
    let mut _out = Vec::new();
    let mut buffer = String::new();
    let mut reading_buffer = false;
    let mut readding_in = false;
    for ch in data.chars() {
        if readding_in {
            if reading_buffer {
                match ch {
                    ';' => {
                        reading_buffer = false;
                        _in.push(TypeTag::L(buffer.clone()));
                        buffer.clear();
                        continue;
                    }
                    _ => {
                        buffer.push(ch);
                        continue;
                    }
                }
            }
            match ch {
                'I' => {
                    _in.push(TypeTag::Int);
                    continue;
                }
                'L' => {
                    reading_buffer = true;
                    continue;
                }
                ')' => {
                    readding_in = false;
                    continue;
                }
                _ => {
                    panic!("")
                }
            }
        }
        if reading_buffer {
            match ch {
                ';' => {
                    reading_buffer = false;
                    _out.push(TypeTag::L(buffer.clone()));
                    buffer.clear();
                    continue;
                }
                _ => {
                    buffer.push(ch);
                    continue;
                }
            }
        }
        match ch {
            'I' => {
                _out.push(TypeTag::Int);
                continue;
            }
            'L' => {
                reading_buffer = true;
                continue;
            }
            '(' => {
                readding_in = true;
                continue;
            }
            _ => {}
        }
    }
    (_in, _out)
}
