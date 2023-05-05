use rjvm::jvm::Jvm;

use crate::docking::Tab;
use eframe::egui;

pub struct TabHeap {}

impl Tab for TabHeap {
    fn get_title(&self) -> String {
        String::from("Heap")
    }

    fn show(&mut self, ui: &mut eframe::egui::Ui, debugger: &mut crate::debugger::Debugger) {
        let jvm = debugger.get_component::<Jvm>().unwrap();
        egui::ScrollArea::new([true, true]).show(ui, |ui| {
            ui.label(format!("{:?}", jvm.heap.lock().unwrap()));
        });
    }

    fn init(&mut self, debugger: &mut crate::debugger::Debugger) {}
}

impl TabHeap {
    pub fn new() -> Self {
        Self {}
    }
}
