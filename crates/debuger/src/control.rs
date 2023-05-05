use egui_kman_additions::SingleLineComplition;
use rjvm::{
    class::Class,
    constant_info::Constant,
    jvm::Jvm,
    method_info::MethodInfo,
    stream::{FileStream, Stream},
};

use crate::{console::Console, docking::Tab};
use eframe::egui::{self, RichText};

pub struct TabControl {
    class_path: String,
    class_name: String,
    module_path: String,

    execute_class: String,
    execute_class_complitions: Vec<String>,
    execute_method: String,
    execute_method_complitions: Vec<String>,
    execute_descriptor: Vec<MethodInfo>,
    execute_args: String,
}

impl Tab for TabControl {
    fn get_title(&self) -> String {
        String::from("Control")
    }

    fn show(&mut self, ui: &mut eframe::egui::Ui, d: &mut crate::debugger::Debugger) {
        ui.label("class path: ");
        ui.text_edit_singleline(&mut self.class_path);
        ui.label("class name: ");
        ui.text_edit_singleline(&mut self.class_name);
        if ui.button("add").clicked() {
            if let Ok(stream) = FileStream::try_from(self.class_path.as_str()) {
                d.get_component::<Jvm>().unwrap().add_class(
                    Class::LazyClass(Stream::FileStream(stream)),
                    &self.class_name,
                );
                d.get_component::<Console>().unwrap().log(&format!(
                    "Added unchecked class: path: \"{}\", name: \"{}\"",
                    self.class_path, self.class_name
                ));
                return;
            }

            d.get_component::<Console>()
                .unwrap()
                .log("Invalid class path");
        }

        ui.separator();

        ui.label("module path: ");
        ui.text_edit_singleline(&mut self.module_path);
        if ui.button("add").clicked() {
            d.get_component::<Jvm>()
                .unwrap()
                .read_module_info(&self.module_path);
            d.get_component::<Console>()
                .unwrap()
                .log("Readed module info");
        }

        ui.separator();

        ui.label(RichText::new("Executing").size(21.0));
        ui.label("Class:");

        let class = SingleLineComplition::new(
            "select_class",
            self.execute_class_complitions
                .iter()
                .map(std::ops::Deref::deref)
                .collect::<Vec<&str>>()
                .as_ref(),
        )
        .exact(true)
        .show(ui);

        if class.response.gained_focus() {
            self.execute_class_complitions.clear();
            self.execute_method_complitions.clear();
            {
                let mut jvm = d.get_component::<Jvm>().unwrap();
                for class in jvm.classes.iter() {
                    self.execute_class_complitions.push(class.0.clone())
                }
            }
        }

        ui.separator();
        ui.label("Method: ");
        let method = SingleLineComplition::new(
            "select_method",
            self.execute_method_complitions
                .iter()
                .map(std::ops::Deref::deref)
                .collect::<Vec<&str>>()
                .as_ref(),
        )
        .exact(true)
        .show(ui);

        if method.response.gained_focus() {
            if let Some(class) = &class.value {
                self.execute_class_complitions.clear();
                {
                    let mut jvm = d.get_component::<Jvm>().unwrap();
                    if let Some(class) = jvm.classes.get(class) {
                        for method in class.get_methods() {
                            if let Constant::Utf8(name) =
                                &class.get_cp()[method.name_index as usize - 1]
                            {
                                self.execute_method_complitions.push(name.clone())
                            }
                        }
                    }
                }
            }
        }

        if method.value_selected {
            if let Some(class_name) = &class.value {
                if let Some(method_name) = &method.value {
                    self.execute_descriptor.clear();
                    let mut jvm = d.get_component::<Jvm>().unwrap();
                    if let Some(class) = jvm.classes.get(class_name) {
                        for m in class.get_methods().iter() {
                            if let Constant::Utf8(name) = &class.get_cp()[m.name_index as usize - 1]
                            {
                                if name.trim() == method_name.trim() {
                                    self.execute_descriptor.push(m.clone())
                                }
                            }
                        }
                    }
                }
            }
        }

        ui.label("Args: ");
        let mut distribuitor = Vec::new();
        if let Some(class_name) = &class.value {
            let mut jvm = d.get_component::<Jvm>().unwrap();
            if let Some(class) = jvm.classes.get(class_name) {
                for m in self.execute_descriptor.iter() {
                    if let Constant::Utf8(dist) = &class.get_cp()[m.descriptor_index as usize - 1] {
                        distribuitor.push(dist.clone())
                    }
                }
            }
        }

        for d in distribuitor {
            ui.label(d);
        }

        ui.text_edit_singleline(&mut self.execute_args);

        if ui.button("Execute").clicked() {}
    }

    fn init(&mut self, debugger: &mut crate::debugger::Debugger) {}
}

impl TabControl {
    pub fn new() -> Self {
        Self {
            class_path: String::new(),
            class_name: String::new(),
            module_path: String::new(),

            execute_class: String::new(),
            execute_class_complitions: Vec::new(),
            execute_method: String::new(),
            execute_method_complitions: Vec::new(),
            execute_descriptor: Vec::new(),
            execute_args: String::new(),
        }
    }
}
