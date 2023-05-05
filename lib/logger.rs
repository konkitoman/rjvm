#[derive(Debug)]
pub struct Action {
    pub name: String,
    pub desc: String,
    pub value: Box<dyn std::fmt::Debug>,
    pub sub_actions: Vec<Action>,
}

impl Action {
    pub fn new<T: 'static + std::fmt::Debug>(name: &str, desc: &str, value: T) -> Self {
        Self {
            name: name.to_owned(),
            desc: desc.to_owned(),
            value: Box::new(value),
            sub_actions: Vec::new(),
        }
    }
}

#[derive(Debug)]
pub struct Logger {
    pub current_action: Vec<usize>,
    pub action: Action,
}

impl Logger {
    pub fn new() -> Self {
        Self {
            current_action: Vec::new(),
            action: Action::new("Logger start", "First action", "".to_owned()),
        }
    }

    pub fn get_action(&mut self, path: &[usize]) -> &mut Action {
        let mut action = &mut self.action;
        for i in path.iter() {
            action = &mut action.sub_actions[*i]
        }
        action
    }

    pub fn get_last(&mut self) -> &mut Action {
        let b = self.current_action.clone();
        self.get_action(&b)
    }

    pub fn add_action(&mut self, action: Action) {
        let last = self.get_last();
        let index = last.sub_actions.len();
        last.sub_actions.push(action);
        self.current_action.push(index);
    }

    pub fn exit_action(&mut self) {
        self.current_action.pop();
    }
}

pub static mut LOGGER: Option<Logger> = None;

pub fn get_logger<'a>() -> &'a mut Option<Logger> {
    unsafe { &mut LOGGER }
}

pub fn set_logger(value: Option<Logger>) {
    unsafe { LOGGER = value }
}
