use std::sync::{Arc, RwLock};

use eframe::egui::Ui;

use crate::debugger::Debugger;

pub trait Tab {
    fn get_title(&self) -> String;
    fn show(&mut self, ui: &mut Ui, debugger: &mut Debugger);
    fn init(&mut self, debugger: &mut Debugger);
}

pub struct TabManager {
    debugger: Arc<RwLock<Debugger>>,
}

impl egui_dock::TabViewer for TabManager {
    type Tab = Arc<RwLock<Box<dyn Tab>>>;

    fn ui(&mut self, ui: &mut Ui, tab: &mut Self::Tab) {
        let mut debugger = self.debugger.write().unwrap();
        let mut tab = tab.write().unwrap();
        tab.show(ui, &mut *debugger)
    }

    fn title(&mut self, tab: &mut Self::Tab) -> eframe::egui::WidgetText {
        let tab = tab.read().unwrap();
        eframe::egui::WidgetText::RichText(tab.get_title().into())
    }
}

impl TabManager {
    pub fn new(debugger: Arc<RwLock<Debugger>>) -> Self {
        Self { debugger }
    }
}
