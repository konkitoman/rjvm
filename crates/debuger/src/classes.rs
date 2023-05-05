use rjvm::{constant_info::Constant, java_class::JavaClass, jvm::Jvm};

use crate::docking::Tab;
use eframe::egui;

pub struct TabClasses {}

impl Tab for TabClasses {
    fn get_title(&self) -> String {
        String::from("Classes")
    }

    fn show(&mut self, ui: &mut eframe::egui::Ui, debugger: &mut crate::debugger::Debugger) {
        let jvm = debugger.get_component::<Jvm>().unwrap();
        let text_height = ui.text_style_height(&egui::TextStyle::Body);
        let max = ui.available_size().y - (text_height * 7.0);

        egui::ScrollArea::new([true, true])
            .auto_shrink([false, false])
            .show_rows(ui, text_height, jvm.classes.len(), |ui, range| {
                for i in range {
                    if let Some((name, class)) = jvm.classes.iter().nth(i) {
                        egui::CollapsingHeader::new(name).show(ui, |ui| {
                            egui::ScrollArea::new([true, true])
                                .auto_shrink([false, true])
                                .show(ui, |ui| match class {
                                    rjvm::class::Class::JavaClass(class, static_state) => {
                                        ui.label(format!(
                                            "Version: {}.{}",
                                            class.major, class.minor
                                        ));

                                        fn get_name(class: &JavaClass, index: usize) -> String {
                                            let mut name = String::new();

                                            match &class.constant_pool[index] {
                                                Constant::Class(class_index) => {
                                                    match &class.constant_pool
                                                        [*class_index as usize - 1]
                                                    {
                                                        Constant::Utf8(string) => {
                                                            name = string.clone();
                                                        }

                                                        Constant::NameAndType(_, name_index) => {
                                                            if let Constant::Utf8(string) = &class
                                                                .constant_pool
                                                                [*name_index as usize]
                                                            {
                                                                name = string.clone()
                                                            }
                                                        }
                                                        _ => {}
                                                    }
                                                }
                                                Constant::Utf8(string) => {
                                                    name = string.clone();
                                                }
                                                _ => {}
                                            }
                                            name
                                        }

                                        let name = get_name(&class, class.this_class as usize - 1);
                                        ui.label(format!("This: {}", name));

                                        let name = get_name(&class, class.super_class as usize - 1);
                                        ui.label(format!("Super: {}", name));

                                        egui::CollapsingHeader::new("Constant pool").show(
                                            ui,
                                            |ui| {
                                                egui::ScrollArea::new([true, true])
                                                    .auto_shrink([false, true])
                                                    .max_height(max)
                                                    .show_rows(
                                                        ui,
                                                        ui.text_style_height(
                                                            &egui::TextStyle::Body,
                                                        ),
                                                        class.constant_pool.len(),
                                                        |ui, range| {
                                                            for i in range {
                                                                if let Some(constant) =
                                                                    class.constant_pool.get(i)
                                                                {
                                                                    ui.label(format!(
                                                                        "{:?}",
                                                                        constant
                                                                    ));
                                                                }
                                                            }
                                                        },
                                                    );
                                            },
                                        );

                                        egui::CollapsingHeader::new("Methods").show(ui, |ui| {
                                            egui::ScrollArea::new([true, true])
                                                .auto_shrink([false, true])
                                                .max_height(max)
                                                .show_rows(
                                                    ui,
                                                    ui.text_style_height(&egui::TextStyle::Body),
                                                    class.methods.len(),
                                                    |ui, range| {
                                                        for i in range {
                                                            if let Some(method) =
                                                                class.methods.get(i)
                                                            {
                                                                ui.label(format!("{:?}", method));
                                                            }
                                                        }
                                                    },
                                                );
                                        });
                                    }
                                    rjvm::class::Class::LazyClass(lazy) => {
                                        ui.label("Is not loaded!");
                                        if ui.button("Load").clicked() {
                                            class.load();
                                        }
                                    }
                                });
                        });
                    }
                }
            });
    }

    fn init(&mut self, debugger: &mut crate::debugger::Debugger) {}
}

impl TabClasses {
    pub fn new() -> Self {
        Self {}
    }
}
