use crate::docking::Tab;

pub struct Console {
    data: String,
}

impl Console {
    fn new() -> Self {
        Self {
            data: String::new(),
        }
    }

    pub fn log(&mut self, data: &str) {
        self.data.push_str(data);
        self.data.push('\n')
    }
}

pub struct TabConsole {}

impl TabConsole {
    pub fn new() -> Self {
        Self {}
    }
}

impl Tab for TabConsole {
    fn get_title(&self) -> String {
        String::from("Console")
    }

    fn show(&mut self, ui: &mut eframe::egui::Ui, debugger: &mut crate::debugger::Debugger) {
        let console = debugger.get_component::<Console>().unwrap();
        ui.selectable_label(false, &console.data);
    }

    fn init(&mut self, debugger: &mut crate::debugger::Debugger) {
        debugger.add_component(Console::new());
    }
}
