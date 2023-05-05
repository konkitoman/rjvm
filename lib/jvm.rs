use std::{collections::HashMap, sync::Mutex};

use crate::{
    attributes::Attribute,
    class::Class,
    constant_info::Constant,
    error::Error,
    interpeter::Interpeter,
    java_class::JavaClass,
    jvm_state::JvmState,
    stream::{BytesStream, FileStream, Stream},
    types::Type,
    Log, LogDone,
};

pub struct Jvm {
    pub classes: HashMap<String, Class>,
    pub heap: Mutex<Vec<Type>>,
}
use crate::prelude::*;
impl Jvm {
    pub fn new() -> Self {
        Self {
            classes: HashMap::new(),
            heap: Mutex::new(Vec::new()),
        }
    }

    pub fn add_class(&mut self, class: Class, class_name: &str) {
        self.classes.insert(class_name.to_owned(), class);
    }

    pub fn load_base(&mut self, path: &str) {
        Log!("Loading base", "java std", "");
        self.read_module_info(&format!("{}/java_std/classes", path));
        LogDone!();
    }

    pub fn read_module_info(&mut self, path: &str) {
        Log!("Reading module info", "", path.to_string());
        let mut directories = Vec::new();
        {
            let module_info = JavaClass::new(Stream::FileStream(
                FileStream::try_from(format!("{}/module-info.class", path).as_str()).unwrap(),
            ))
            .unwrap();

            for attr in module_info.attributes.iter() {
                if let Attribute::Module(at) = &attr.attribute {
                    if let Constant::Module(module) =
                        module_info.constant_pool[at.module_name_index as usize - 1]
                    {
                        if let Constant::Utf8(name) =
                            module_info.constant_pool[module as usize - 1].clone()
                        {
                            Log!("module finded", "", "");
                            for export in at.exports.iter() {
                                if let Constant::Package(pack) =
                                    module_info.constant_pool[export.index as usize - 1]
                                {
                                    if let Constant::Utf8(name) =
                                        module_info.constant_pool[pack as usize - 1].clone()
                                    {
                                        Log!("finded module dir", "", name.clone());
                                        directories.push(name);
                                        LogDone!();
                                    }
                                }
                            }
                            LogDone!();
                        }
                    }
                }
            }
        }

        for dir_path in directories {
            let dir_path = format!("{}/{}", path, dir_path);
            let dir = std::fs::read_dir(&dir_path).unwrap();

            Log!("dir", "", dir_path);
            for file_path in dir {
                let file_path = file_path.unwrap();
                let p = file_path.path();
                if p.is_file() {
                    let full_path = p.to_str().unwrap().clone();
                    Log!("file", "", file_path);
                    LogDone!();

                    let class_name = full_path.replace(format!("{}/", path).as_str(), "");
                    let class_name = class_name.replace(".class", "");
                    self.add_class(
                        Class::new(Stream::FileStream(FileStream::try_from(full_path).unwrap())),
                        class_name.as_str(),
                    );
                }
            }
            LogDone!();
        }
        LogDone!();
    }

    pub fn execute_new(&self, class_name: &str, args: Vec<Type>) -> Result<usize, Error> {
        let class = self.classes.get(class_name).unwrap();
        class.load().unwrap();

        Log!("execute_new", "", format!("Class: {}", class_name));
        for method in class.get_methods() {
            if let Constant::Utf8(method_name) = &class.get_cp()[method.name_index as usize - 1] {
                if method_name.trim() == "<init>" {
                    for attribute in method.attributes.iter() {
                        if let Attribute::Code(code_attr) = &attribute.attribute {
                            let mut opcodes = BytesStream::from(code_attr.code.as_ref());
                            let mut state =
                                JvmState::new(code_attr.max_stack, code_attr.max_locale, args);
                            while !opcodes.is_empty() {
                                let mut interpeter = Interpeter {
                                    codes: &mut opcodes,
                                    jvm: &self,
                                    class,
                                    state: &mut state,
                                };
                                interpeter.interpeter()?;
                                // interpeter(&mut opcodes, self, class, state);
                                Log!("Interpeter", "", String::new());
                                Log!("State", "", interpeter.state.clone());
                                LogDone!();
                                LogDone!();
                            }
                            break;
                        }
                    }
                    break;
                }
            }
        }
        LogDone!();
        // panic!()
        Ok(0)
    }

    pub fn execute_with_stack(
        &self,
        class_name: &str,
        method_n: &str,
        state: &mut JvmState,
    ) -> Result<(), Error> {
        let class = self.classes.get(class_name).unwrap();
        class.load().unwrap();

        Log!(
            "execute_with_stack",
            "",
            format!(
                "Class: {}, Method: {}, stack: {:?}",
                class_name, method_n, state
            )
        );
        for method in class.get_methods() {
            if let Constant::Utf8(method_name) = &class.get_cp()[method.name_index as usize - 1] {
                if method_name.trim() == method_n {
                    for attribute in method.attributes.iter() {
                        if let Attribute::Code(code_attr) = &attribute.attribute {
                            let mut opcodes = BytesStream::from(code_attr.code.as_ref());
                            while !opcodes.is_empty() {
                                let mut interpeter = Interpeter {
                                    codes: &mut opcodes,
                                    jvm: &self,
                                    class,
                                    state,
                                };
                                interpeter.interpeter()?;
                                // interpeter(&mut opcodes, self, class, state);
                                Log!("Interpeter", "", String::new());
                                Log!("State", "", interpeter.state.clone());
                                LogDone!();
                                LogDone!();
                            }
                        }
                    }
                    break;
                }
            }
        }
        LogDone!();
        Ok(())
    }

    pub fn gen_state(
        &self,
        class_name: &str,
        method_n: &str,
        args: Vec<Type>,
    ) -> Result<JvmState, Error> {
        let class = self.classes.get(class_name).unwrap();
        class.load().unwrap();
        for method in class.get_methods() {
            if let Constant::Utf8(method_name) = &class.get_cp()[method.name_index as usize - 1] {
                if method_name.trim() == method_n {
                    for attribute in method.attributes.iter() {
                        if let Attribute::Code(code_attr) = &attribute.attribute {
                            return Ok(JvmState::new(
                                code_attr.max_stack,
                                code_attr.max_locale,
                                args,
                            ));
                        }
                    }
                    break;
                }
            }
        }

        Err(Error::InvalidClassOrMethod)
    }

    pub fn execute(&self, class_name: &str, method_n: &str, args: Vec<Type>) -> Result<(), Error> {
        let class = self.classes.get(class_name).unwrap();
        Log!(
            "execute",
            "",
            format!("Class: {}, Method: {}", class_name, method_n)
        );
        class.load().unwrap();
        for method in class.get_methods() {
            if let Constant::Utf8(method_name) = &class.get_cp()[method.name_index as usize - 1] {
                if method_name.trim() == method_n {
                    for attribute in method.attributes.iter() {
                        if let Attribute::Code(code_attr) = &attribute.attribute {
                            let mut opcodes = BytesStream::from(code_attr.code.as_ref());
                            let mut state =
                                JvmState::new(code_attr.max_stack, code_attr.max_locale, args);
                            while !opcodes.is_empty() {
                                let mut interpeter = Interpeter {
                                    codes: &mut opcodes,
                                    jvm: &self,
                                    class,
                                    state: &mut state,
                                };
                                interpeter.interpeter()?;
                                Log!("Interpeter", "", String::new());
                                Log!("State", "", interpeter.state.clone());
                                LogDone!();
                                LogDone!();
                            }
                            break;
                        }
                    }
                    break;
                }
            }
        }
        LogDone!();
        Ok(())
    }

    pub fn init(&self, class_name: &str) -> Result<(), Error> {
        let class = self.classes.get(class_name).unwrap();
        class.load().unwrap();
        if !class.get_state().lock().unwrap().instanciated {
            class.get_state().lock().unwrap().instanciated = true;
            // self.execute(class_name, "<init>")?;
            self.execute(class_name, "<clinit>", Vec::new())?
        }
        Ok(())
    }
}
