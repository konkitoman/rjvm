use rjvm::jvm::Jvm;

use crate::docking::Tab;

pub struct TabStack {}

impl Tab for TabStack {
    fn get_title(&self) -> String {
        String::from("Stack")
    }

    fn show(&mut self, ui: &mut eframe::egui::Ui, debugger: &mut crate::debugger::Debugger) {}

    fn init(&mut self, debugger: &mut crate::debugger::Debugger) {}
}

impl TabStack {
    pub fn new() -> Self {
        Self {}
    }
}
