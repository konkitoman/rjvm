use crate::{console::Console, docking::Tab};
use eframe::egui;
use rjvm::logger::{get_logger, Action};

enum LoggerSearchType {
    Name,
    Desc,
    Value,
}

pub struct TabLogger {
    search_buffer: String,
    search: Vec<usize>,
    close_all: bool,
}

impl Tab for TabLogger {
    fn get_title(&self) -> String {
        String::from("Logger")
    }

    fn show(&mut self, ui: &mut eframe::egui::Ui, debugger: &mut crate::debugger::Debugger) {
        if let Some(logger) = get_logger() {
            egui::TopBottomPanel::bottom("logger-bottom")
                .resizable(false)
                .show_inside(ui, |ui| {
                    ui.horizontal_centered(|ui| {
                        ui.text_edit_singleline(&mut self.search_buffer).clicked();
                        if ui.button("Search").clicked() {
                            self.search.clear();
                            let splits = self.search_buffer.split(':').collect::<Vec<&str>>();
                            if let Some(keyword) = splits.get(0) {
                                if match *keyword {
                                    "name" => {
                                        let value = self.search_buffer.replace("name: ", "");
                                        self.search(&LoggerSearchType::Name, &value)
                                    }
                                    "desc" => {
                                        let value = self.search_buffer.replace("desc: ", "");
                                        self.search(&LoggerSearchType::Desc, &value)
                                    }
                                    "value" => {
                                        let value = self.search_buffer.replace("value: ", "");
                                        self.search(&LoggerSearchType::Value, &value)
                                    }
                                    _ => false,
                                } {
                                    let console = debugger.get_component::<Console>().unwrap();
                                    console.log("Search: Finded");
                                }
                            }
                        }
                        if ui.button("clear").clicked() {
                            self.search_buffer.clear();
                            self.search.clear();
                            self.close_all = true;
                        }
                    });
                });
            egui::ScrollArea::vertical()
                .auto_shrink([false, false])
                .show(ui, |ui| {
                    egui::CollapsingHeader::new(&logger.action.name)
                        .show(ui, |ui| self.add_action(ui, &logger.action, 0));
                });
        }
        self.close_all = false;
    }

    fn init(&mut self, debugger: &mut crate::debugger::Debugger) {}
}

impl TabLogger {
    pub fn new() -> Self {
        Self {
            search_buffer: String::new(),
            search: Vec::new(),
            close_all: false,
        }
    }

    fn add_action(&mut self, ui: &mut egui::Ui, action: &rjvm::logger::Action, i: usize) {
        let mut i = i;
        ui.label(format!("Desc: {}", action.desc));
        ui.label(format!("Value: {:?}", action.value));
        for action in action.sub_actions.iter() {
            i += 1;
            let mut col = egui::CollapsingHeader::new(format!("{} {}", i, action.name));

            if self.search.contains(&i) {
                col = col.open(Some(true));
            }

            if self.close_all {
                col = col.open(Some(false));
            }

            col.show(ui, |ui| self.add_action(ui, action, i));
        }
    }

    fn search(&mut self, s_type: &LoggerSearchType, value: &str) -> bool {
        let mut ret = false;
        if let Some(logger) = get_logger() {
            if self._search(s_type, value, &logger.action, 0) {
                ret = true
            }
        }
        ret
    }

    fn _search(
        &mut self,
        s_type: &LoggerSearchType,
        value: &str,
        action: &Action,
        i: usize,
    ) -> bool {
        let mut i = i;
        let mut ret = false;
        match s_type {
            LoggerSearchType::Name => {
                if let Some(_) = action.name.trim().find(value) {
                    self.search.push(i);
                    ret = true;
                }
            }
            LoggerSearchType::Desc => {
                if let Some(_) = action.desc.trim().find(value) {
                    self.search.push(i);
                    ret = true;
                }
            }
            LoggerSearchType::Value => {
                if let Some(_) = format!("{:?}", action.value).trim().find(value) {
                    self.search.push(i);
                    ret = true;
                }
            }
        }
        for a in action.sub_actions.iter() {
            i += 1;
            if self._search(s_type, value, a, i) {
                ret = true;
            }
        }
        ret
    }
}
