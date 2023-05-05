use crate::docking::Tab;

pub struct TabClassView {}

impl Tab for TabClassView {
    fn get_title(&self) -> String {
        String::from("Class View")
    }

    fn show(&mut self, ui: &mut eframe::egui::Ui, debugger: &mut crate::debugger::Debugger) {}

    fn init(&mut self, debugger: &mut crate::debugger::Debugger) {}
}

impl TabClassView {
    pub fn new() -> Self {
        Self {}
    }
}
