mod class_view;
mod classes;
mod console;
mod control;
mod debugger;
mod docking;
mod heap;
mod logger;
mod stack;
mod testing;

use std::sync::Arc;
use std::sync::RwLock;

use class_view::TabClassView;
use classes::TabClasses;
use console::TabConsole;
use control::TabControl;
use debugger::Debugger;
use docking::Tab;
use docking::TabManager;
use eframe::egui;
use eframe::egui::style::Margin;
use eframe::egui::Context;
use eframe::egui::Frame;
use eframe::epaint::Color32;
use eframe::epaint::Rounding;
use eframe::epaint::Shadow;
use eframe::epaint::Vec2;
use eframe::App;
use egui_dock::Node;
use egui_dock::Tree;
use heap::TabHeap;
use logger::TabLogger;
use rjvm::jvm::Jvm;
use stack::TabStack;
use testing::TabTesting;

pub struct GUI {
    debugger: Arc<RwLock<Debugger>>,
    tree: Tree<Arc<RwLock<Box<dyn Tab>>>>,
    tabs: Vec<Arc<RwLock<Box<dyn Tab>>>>,
    tab_manager: TabManager,
}

impl App for GUI {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("toolbar").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("Tabs", |ui| {
                    let tabs = self.tabs.clone();
                    for (id, tab) in tabs.iter().enumerate() {
                        let name = tab.read().unwrap().get_title();
                        if self.is_open(id) {
                            if ui.checkbox(&mut true, name).clicked() {
                                self.close(id);
                            }
                        } else {
                            if ui.checkbox(&mut false, name).clicked() {
                                self.open(id);
                            }
                        }
                    }
                });
            });
        });
        egui_dock::DockArea::new(&mut self.tree).show(ctx, &mut self.tab_manager);
    }
}

impl GUI {
    pub fn new() -> Self {
        let debugger = Arc::new(RwLock::new(Debugger::new()));
        Self {
            tab_manager: TabManager::new(debugger.clone()),
            debugger,
            tabs: Vec::new(),
            tree: Tree::new(Vec::new()),
        }
    }

    pub fn register_tab<T: 'static + Tab>(&mut self, tab: T) -> usize {
        let mut tab = tab;
        let id = self.tabs.len();
        tab.init(&mut *self.debugger.write().unwrap());
        self.tabs.push(Arc::new(RwLock::new(Box::new(tab))));
        id
    }

    pub fn close(&mut self, id: usize) {
        if self.is_open(id) {
            for (ind, node) in self.tree.iter_mut().enumerate() {
                if let Node::Leaf { tabs, active, .. } = node {
                    let mut index = 0;
                    let mut finded = false;

                    for (i, tab) in tabs.iter().enumerate() {
                        if tab.read().unwrap().get_title()
                            == self.tabs[id].read().unwrap().get_title()
                        {
                            index = i;
                            finded = true;
                            break;
                        }
                    }

                    if finded {
                        tabs.remove(index);
                        *active = 0.into();
                        break;
                    }
                }
            }

            self.tree.remove_empty_leaf()
        }
    }

    pub fn open(&mut self, id: usize) {
        if !self.is_open(id) {
            self.tree.push_to_focused_leaf(self.tabs[id].clone())
        }
    }

    pub fn is_open(&mut self, id: usize) -> bool {
        for node in self.tree.iter() {
            if let Node::Leaf {
                rect,
                viewport,
                tabs,
                active,
            } = node
            {
                for tab in tabs.iter() {
                    if tab.read().unwrap().get_title() == self.tabs[id].read().unwrap().get_title()
                    {
                        return true;
                    }
                }
            }
        }

        false
    }
}

pub fn main() {
    rjvm::logger::set_logger(Some(rjvm::logger::Logger::new()));

    let mut gui = GUI::new();

    gui.debugger.write().unwrap().add_component(Jvm::new());

    gui.register_tab(TabConsole::new());
    gui.register_tab(TabLogger::new());
    gui.register_tab(TabTesting::new());
    gui.register_tab(TabClasses::new());
    gui.register_tab(TabClassView::new());
    gui.register_tab(TabStack::new());
    gui.register_tab(TabHeap::new());
    gui.register_tab(TabControl::new());

    gui.open(0);

    eframe::run_native(
        "RJVM Controler",
        eframe::NativeOptions {
            always_on_top: false,
            maximized: false,
            decorated: true,
            fullscreen: false,
            drag_and_drop_support: false,
            icon_data: None,
            initial_window_pos: None,
            initial_window_size: None,
            min_window_size: None,
            max_window_size: None,
            resizable: true,
            transparent: false,
            vsync: true,
            multisampling: 0,
            depth_buffer: 0,
            stencil_buffer: 0,
            hardware_acceleration: eframe::HardwareAcceleration::Preferred,
            renderer: eframe::Renderer::Wgpu,
            follow_system_theme: true,
            default_theme: eframe::Theme::Dark,
            run_and_return: false,
        },
        Box::new(move |context| Box::new(gui)),
    );
}

// pub struct Debuger {
//     jvm: Jvm,

//     show_console: bool,
//     show_logger: bool,

//     logger_search_buffer: String,
//     logger_search: Vec<usize>,
//     logger_close_all: bool,
// }

// impl Default for Debuger {
//     fn default() -> Self {
//         Self {
//             jvm: Jvm::new(),
//             show_console: false,
//             show_logger: true,
//             logger_search_buffer: String::new(),
//             logger_search: Vec::new(),
//             logger_close_all: true,
//         }
//     }
// }

// impl App for Debuger {
//     fn update(&mut self, ctx: &eframe::egui::Context, frame: &mut eframe::Frame) {
//         egui::TopBottomPanel::top("toolbar").show(ctx, |ui| {
//             egui::menu::bar(ui, |ui| {
//                 ui.menu_button("view", |ui| {
//                     ui.checkbox(&mut self.show_console, "console");
//                     ui.checkbox(&mut self.show_logger, "logger");
//                 })
//             })
//         });
//         self.render_console(ctx);
//         self.render_logger(ctx);
//         egui::CentralPanel::default().show(ctx, |ui| {
//             if ui.button("add std").clicked() {
//                 self.jvm.load_base("../..");
//             }
//         });
//     }
// }

// pub enum LoggerSearchType {
//     Name,
//     Desc,
//     Value,
// }

// impl Debuger {
//     pub fn render_logger(&mut self, ctx: &Context) {
//         if self.show_logger {
//             egui::SidePanel::left("logger-panel")
//                 .frame(Frame::window(&ctx.style()).inner_margin(Margin::same(0.0)))
//                 .show(ctx, |ui| self.logger(ui));
//         }
//     }

//     fn logger_search(&mut self, s_type: &LoggerSearchType, value: &str) -> bool {
//         let mut ret = false;
//         if let Some(logger) = logger::get_logger() {
//             if self._logger_search(s_type, value, &logger.action, 0) {
//                 ret = true
//             }
//         }
//         ret
//     }

//     fn _logger_search(
//         &mut self,
//         s_type: &LoggerSearchType,
//         value: &str,
//         action: &logger::Action,
//         i: usize,
//     ) -> bool {
//         let mut i = i;
//         let mut ret = false;
//         match s_type {
//             LoggerSearchType::Name => {
//                 if let Some(_) = action.name.trim().find(value) {
//                     self.logger_search.push(i);
//                     ret = true;
//                 }
//             }
//             LoggerSearchType::Desc => {
//                 if let Some(_) = action.desc.trim().find(value) {
//                     self.logger_search.push(i);
//                     ret = true;
//                 }
//             }
//             LoggerSearchType::Value => {
//                 if let Some(_) = format!("{:?}", action.value).trim().find(value) {
//                     self.logger_search.push(i);
//                     ret = true;
//                 }
//             }
//         }
//         for a in action.sub_actions.iter() {
//             i += 1;
//             if self._logger_search(s_type, value, a, i) {
//                 ret = true;
//             }
//         }
//         ret
//     }

//     fn add_action(&mut self, ui: &mut egui::Ui, action: &rjvm::logger::Action, i: usize) {
//         let mut i = i;
//         ui.label(format!("Desc: {}", action.desc));
//         ui.label(format!("Value: {:?}", action.value));
//         for action in action.sub_actions.iter() {
//             i += 1;
//             let mut col = egui::CollapsingHeader::new(format!("{} {}", i, action.name));

//             if self.logger_search.contains(&i) {
//                 col = col.open(Some(true));
//             }

//             if self.logger_close_all {
//                 col = col.open(Some(false));
//             }

//             col.show(ui, |ui| self.add_action(ui, action, i));
//         }
//     }

//     pub fn logger(&mut self, ui: &mut egui::Ui) {
//         if let Some(logger) = logger::get_logger() {
//             egui::TopBottomPanel::bottom("logger-bottom")
//                 .resizable(false)
//                 .show_inside(ui, |ui| {
//                     ui.horizontal_centered(|ui| {
//                         ui.text_edit_singleline(&mut self.logger_search_buffer)
//                             .clicked();
//                         if ui.button("Search").clicked() {
//                             self.logger_search.clear();
//                             let splits =
//                                 self.logger_search_buffer.split(':').collect::<Vec<&str>>();
//                             if let Some(keyword) = splits.get(0) {
//                                 if match *keyword {
//                                     "name" => {
//                                         let value = self.logger_search_buffer.replace("name: ", "");
//                                         self.logger_search(&LoggerSearchType::Name, &value)
//                                     }
//                                     "desc" => {
//                                         let value = self.logger_search_buffer.replace("desc: ", "");
//                                         self.logger_search(&LoggerSearchType::Desc, &value)
//                                     }
//                                     "value" => {
//                                         let value =
//                                             self.logger_search_buffer.replace("value: ", "");
//                                         self.logger_search(&LoggerSearchType::Value, &value)
//                                     }
//                                     _ => false,
//                                 } {
//                                     println!("Finded")
//                                 }
//                             }
//                         }
//                         if ui.button("clear").clicked() {
//                             self.logger_search_buffer.clear();
//                             self.logger_search.clear();
//                             self.logger_close_all = true;
//                         }
//                     });
//                 });
//             egui::ScrollArea::vertical()
//                 .auto_shrink([false, false])
//                 .show(ui, |ui| {
//                     egui::CollapsingHeader::new(&logger.action.name)
//                         .show(ui, |ui| self.add_action(ui, &logger.action, 0));
//                 });
//         }
//         self.logger_close_all = false;
//     }

//     pub fn render_console(&mut self, ctx: &Context) {
//         if self.show_console {
//             egui::SidePanel::right("console-panel")
//                 .frame(egui::Frame::window(&ctx.style()).inner_margin(Margin::same(0.0)))
//                 .show(ctx, |ui| self.console(ui));
//         }
//     }

//     pub fn console(&mut self, ui: &mut egui::Ui) {
//         egui::ScrollArea::vertical().show(ui, |ui| {
//             ui.label("In console");
//             ui.label("t\nt\n\n\nt\n")
//         });
//         egui::TopBottomPanel::bottom("console-control").show_inside(ui, |ui| {
//             ui.horizontal(|ui| {
//                 ui.text_edit_singleline(&mut String::new());
//                 ui.button("Send")
//             })
//         });
//     }
// }

// fn main() {
//     logger::set_logger(Some(logger::Logger::new()));
//     eframe::run_native(
//         "RJvm Debuger",
//         eframe::NativeOptions {
//             always_on_top: false,
//             maximized: false,
//             decorated: true,
//             fullscreen: false,
//             drag_and_drop_support: false,
//             icon_data: None,
//             initial_window_pos: None,
//             initial_window_size: None,
//             min_window_size: None,
//             max_window_size: None,
//             resizable: true,
//             transparent: false,
//             vsync: true,
//             multisampling: 0,
//             depth_buffer: 0,
//             stencil_buffer: 0,
//             hardware_acceleration: eframe::HardwareAcceleration::Preferred,
//             renderer: eframe::Renderer::Wgpu,
//             follow_system_theme: true,
//             default_theme: eframe::Theme::Dark,
//             run_and_return: false,
//         },
//         Box::new(|context| Box::new(Debuger::default())),
//     );
// }
