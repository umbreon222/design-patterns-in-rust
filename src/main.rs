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
    pub trait Mediator<T> {
        fn manage(&mut self, item: T, item_name: &String);
    }
}

use std::collections::HashMap;
use std::rc::Rc;
use core::cell::RefCell;
use command::Command;
use observer::{ Observer, Subject };
use mediator::Mediator;

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

pub struct LightStateObserver {
    update_count: u8
}

impl Observer<bool> for LightStateObserver {
    fn on_subject_updated(&mut self, update_source: &bool) {
        let mut light_state = "off";
        if *update_source {
            light_state = "on"
        }
        println!("Light was switched {}", light_state);
        self.update_count += 1;
    }
}
/* </observer pattern example> */

/* <mediator pattern example> */
pub enum LightAction {
    On,
    Off
}

pub struct LightMediator {
    light_map: HashMap<String, Rc<RefCell<Light>>>
}

impl Mediator<Light> for LightMediator {
    fn manage(&mut self, light: Light, light_name: &String) {
        self.light_map.insert(light_name.clone(), Rc::new(RefCell::new(light)));
    }
}

impl LightMediator {
    fn perform_light_action(&mut self, light_action: LightAction, light_name: &String) {
        match self.light_map.get(light_name) {
            Some(light) => {
                let mut remote: Remote;
                match light_action {
                    LightAction::On => {
                        remote = Remote { command: Box::new(LightOnCommand { light: light.clone() }) };

                    },
                    LightAction::Off => {
                        remote = Remote { command: Box::new(LightOffCommand { light: light.clone() }) };
                    }
                }
                remote.execute();
            },
            None => {}
        }
    }
}
/* </mediator pattern example> */

fn main() {
    let light_name = "light_1".to_string();
    let mut light = Light { state: false, observer_map: HashMap::new() };
    let light_observer_key = "light_observer_1".to_string(); 
    light.attach_observer(&light_observer_key, Box::new(LightStateObserver { update_count: 0 }));
    let mut light_mediator = LightMediator { light_map: HashMap::new() };
    light_mediator.manage(light, &light_name);
    light_mediator.perform_light_action(LightAction::On, &light_name);
    light_mediator.perform_light_action(LightAction::Off, &light_name);
}
