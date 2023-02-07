pub mod command {
    // https://www.dofactory.com/net/command-design-pattern
    pub trait Command {
        fn execute(&mut self);
    }
}

pub mod observer {
    // https://www.dofactory.com/net/observer-design-pattern
    pub trait Observer<T> {
        fn on_subject_updated(&mut self, update_source: &T);
    }

    pub trait Subject<T> {
        fn attach_observer(&mut self, observer_key: &String, observer: Box<dyn Observer<T>>);
        fn detach_observer(&mut self, observer_key: &String) -> bool;
        fn notify_observers(&mut self);
    }
}

pub mod mediator {
    // https://www.dofactory.com/net/mediator-design-pattern
}
use std::collections::HashMap;
use std::rc::Rc;
use core::cell::RefCell;
use command::Command;
use observer::{ Observer, Subject };

pub struct Light {
    state: bool,
    observer_map: HashMap<String, Box<dyn Observer<bool>>>
}

impl Light {
    pub fn on(&mut self) {
        self.state = true;
        self.notify_observers();
    }

    pub fn off(&mut self) {
        self.state = false;
        self.notify_observers();
    }
}

impl Subject<bool> for Light {
    fn attach_observer(&mut self, observer_key: &String, observer: Box<dyn Observer<bool>>) {
        self.observer_map.insert(observer_key.to_string(), observer);
    }

    fn detach_observer(&mut self, observer_key: &String) -> bool {
        if !self.observer_map.contains_key(observer_key) {
            return false;
        }

        self.observer_map.remove(observer_key);
        return true;
    }

    fn notify_observers(&mut self) {
        for observer in self.observer_map.values_mut() {
            observer.on_subject_updated(&self.state);
        }
    }
}

/* <command pattern example> */
pub struct LightOnCommand {
   light: Rc<RefCell<Light>> 
}

impl Command for LightOnCommand {
    fn execute(&mut self) {
        let mut light = self.light.borrow_mut();
        light.on();
    }
}

pub struct LightOffCommand {
   light: Rc<RefCell<Light>> 
}

impl Command for LightOffCommand {
    fn execute(&mut self) {
        let mut light = self.light.borrow_mut();
        light.off();
    }
}

pub struct Remote {
    command: Box<dyn Command>
}

impl Remote {
    pub fn set_command(&mut self, command: Box<dyn Command>) {
        self.command = command
    }

    pub fn execute(&mut self) {
        self.command.execute();
    }
}
/* </command pattern example> */

/* <observer pattern example> */
pub struct LightStateObserver {
    update_count: u8
}

impl Observer<bool> for LightStateObserver {
    fn on_subject_updated(&mut self, update_source: &bool) {
        println!("Light was updated to {}", update_source);
        self.update_count += 1;
    }
}
/* </observer pattern example> */

fn main() {
    let light = Rc::new(RefCell::new(Light { state: false, observer_map: HashMap::new() }));
    let light_observer_key = "light_observer_1".to_string(); 
    light.borrow_mut().attach_observer(&light_observer_key, Box::new(LightStateObserver { update_count: 0 }));
    let on_command = Box::new(LightOnCommand { light: light.clone() });
    let off_command = Box::new(LightOffCommand { light: light.clone() });
    let mut remote = Remote { command: on_command };
    remote.execute();
    remote.set_command(off_command);
    remote.execute();
}
