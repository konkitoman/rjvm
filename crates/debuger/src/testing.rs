use rjvm::jvm::Jvm;

use crate::docking::Tab;

pub struct TabTesting {}

impl Tab for TabTesting {
    fn get_title(&self) -> String {
        String::from("Testing")
    }

    fn show(&mut self, ui: &mut eframe::egui::Ui, debugger: &mut crate::debugger::Debugger) {
        if ui.button("add std").clicked() {
            let jvm = debugger.get_component::<Jvm>().unwrap();
            jvm.load_base("../..");
        }
    }

    fn init(&mut self, debugger: &mut crate::debugger::Debugger) {}
}

impl TabTesting {
    pub fn new() -> Self {
        Self {}
    }
}
