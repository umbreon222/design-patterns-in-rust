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
    pub trait Mediator<THandler, TEvent, TResponse> {
        fn mediate(&mut self, handler: THandler);
        fn broadcast(&mut self, event: TEvent) -> Result<TResponse, ()>;
    }
}

use std::collections::HashMap;
use std::rc::Rc;
use core::cell::RefCell;
use command::Command;
use observer::{ Observer, Subject };
use mediator::Mediator;

pub struct Light {
    name: String,
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
pub enum LightActionType {
    On,
    Off
}

pub struct LightAction {
    action_type: LightActionType,
    light_name: String
}

pub struct LightActionHandler {
    light_map: HashMap<String, Rc<RefCell<Light>>>
}

impl LightActionHandler {
    fn new() -> Self {
        Self {
            light_map: HashMap::new()
        }
    }

    fn add_light(&mut self, light: Light) {
        self.light_map.insert(light.name.clone(), Rc::new(RefCell::new(light)));
    }

    fn handle_light_action(&mut self, light_action: &LightAction) {
        // This would normally just call into a service but I'm doing it here so I can both not write a
        // whole service and also use the command pattern from earlier
        match self.light_map.get(&light_action.light_name) {
            Some(light) => {
                let mut remote: Remote;
                match light_action.action_type {
                    LightActionType::On => {
                        remote = Remote { command: Box::new(LightOnCommand { light: light.clone() }) };

                    },
                    LightActionType::Off => {
                        remote = Remote { command: Box::new(LightOffCommand { light: light.clone() }) };
                    }
                }
                remote.execute();
            },
            None => {}
        }
    }
}

pub struct LightMediator {
    light_action_handlers: Vec<LightActionHandler>
}

impl Mediator<LightActionHandler, LightAction, ()> for LightMediator {
    fn mediate(&mut self, handler: LightActionHandler) {
        self.light_action_handlers.push(handler);
    }

    fn broadcast(&mut self, light_action: LightAction) -> Result<(), ()> {
        for handler in &mut self.light_action_handlers {
            handler.handle_light_action(&light_action)
        }

        Ok(())
    }
}

impl LightMediator {
    pub fn new() -> Self {
        Self {
            light_action_handlers: vec![]
        }
    }
}
/* </mediator pattern example> */

fn main() {
    let mut light = Light { name: "light_1".to_string(), state: false, observer_map: HashMap::new() };
    let light_name = light.name.clone();
    let light_observer_key = "light_observer_1".to_string(); 
    light.attach_observer(&light_observer_key, Box::new(LightStateObserver { update_count: 0 }));
    let mut light_action_handler = LightActionHandler::new();
    light_action_handler.add_light(light);
    let mut light_mediator = LightMediator::new();
    light_mediator.mediate(light_action_handler);
    let turn_on_light_1_action = LightAction {
        light_name: light_name.clone(),
        action_type: LightActionType::On
    };
    light_mediator.broadcast(turn_on_light_1_action).expect("Couldn't turn on light 1");
    let turn_off_light_1_action = LightAction {
        light_name: light_name.clone(),
        action_type: LightActionType::Off
    };
    light_mediator.broadcast(turn_off_light_1_action).expect("Couldn't turn off light 1");
}
