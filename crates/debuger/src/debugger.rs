use std::any::Any;

pub struct Debugger {
    components: Vec<Box<dyn Any>>,
}

impl Debugger {
    pub fn new() -> Self {
        Self {
            components: Vec::new(),
        }
    }

    pub fn add_component<T: 'static>(&mut self, data: T) -> Option<Box<T>> {
        let res = self.remove_component();
        self.components.push(Box::new(data));
        res
    }

    pub fn get_component<T: 'static>(&mut self) -> Option<&mut T> {
        for component in self.components.iter_mut() {
            if let Some(component) = component.downcast_mut::<T>() {
                return Some(component);
            }
        }
        None
    }

    pub fn remove_component<T: 'static>(&mut self) -> Option<Box<T>> {
        let mut index = 0;
        let mut finded = false;
        for (i, component) in self.components.iter().enumerate() {
            if let Some(_) = component.downcast_ref::<T>() {
                index = i;
                finded = true;
                break;
            }
        }

        if finded {
            return Some(self.components.remove(index).downcast::<T>().unwrap());
        }

        None
    }
}
