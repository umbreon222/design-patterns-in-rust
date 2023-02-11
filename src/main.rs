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
    use std::{any::{TypeId, Any}, collections::HashMap};

    // https://www.dofactory.com/net/mediator-design-pattern
    pub trait Mediator {
        fn mediate(&mut self, handler: Box<dyn Handler>);
        fn broadcast(&mut self, event_type: TypeId, event: Box<dyn Any>);
    }

    pub trait Handler {
        fn handle_event(&mut self, event: &Box<dyn Any>);
        fn handles_type(&self) -> TypeId;
    }

    pub struct ConcreteMediator {
        handlers: HashMap<TypeId, Vec<Box<dyn Handler>>>
    }
    
    impl ConcreteMediator {
        pub fn new() -> Self {
            Self {
                handlers: HashMap::new()
            }
        }
    }
    
    impl Mediator for ConcreteMediator {
        fn mediate(&mut self, handler: Box<dyn Handler>) {
            let handler_map_value: &mut Vec<Box<dyn Handler>>;
            match self.handlers.get_mut(&handler.handles_type()) {
              Some(temp_handler_map_value) => {
                handler_map_value = temp_handler_map_value;
              },
              None => {
                self.handlers.insert(handler.handles_type(), vec![]);
                handler_map_value = self.handlers.get_mut(&handler.handles_type()).unwrap();
              }
            }
            handler_map_value.push(handler);
        }
    
        fn broadcast(&mut self, event_type: TypeId, event: Box<dyn Any>) {
            match self.handlers.get_mut(&event_type) {
                Some(handlers) => {
                    for handler in handlers {
                        handler.handle_event(&event);
                    }
                },
                None => {}
            }
        }
    }
}

use std::any::{ TypeId, Any };
use std::collections::HashMap;
use std::rc::Rc;
use core::cell::RefCell;
use command::Command;
use observer::{ Observer, Subject };
use mediator::{ Mediator, Handler, ConcreteMediator };

pub struct Light {
    name: String,
    state: bool,
    observer_map: HashMap<String, Box<dyn Observer<bool>>>
}

impl Light {
    pub fn new(name: String, initial_state: bool) -> Self {
        Self {
            name,
            state: initial_state,
            observer_map: HashMap::new()
        }
    }

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
        self.update_count += 1;
        let mut light_state = "off";
        if *update_source {
            light_state = "on"
        }
        println!("Light was switched {} and has been switched a total of {} time(s)", light_state, self.update_count);
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
}

impl Handler for LightActionHandler {
    fn handle_event(&mut self, event: &Box<dyn Any>) {
        match event.downcast_ref::<LightAction>() {
            Some(light_action) => {
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
            },
            None => {}
        }
    }

    fn handles_type(&self) -> TypeId {
        TypeId::of::<LightAction>()
    }
}
/* </mediator pattern example> */

fn main() {
    let mut light = Light::new("light_1".to_string(), false);
    let light_name = light.name.clone();
    let light_observer_key = "light_observer_1".to_string(); 
    light.attach_observer(&light_observer_key, Box::new(LightStateObserver { update_count: 0 }));
    let mut light_action_handler = LightActionHandler::new();
    light_action_handler.add_light(light);
    let mut light_mediator = ConcreteMediator::new();
    light_mediator.mediate(Box::new(light_action_handler));
    let turn_on_light_1_action = LightAction {
        light_name: light_name.clone(),
        action_type: LightActionType::On
    };
    light_mediator.broadcast(TypeId::of::<LightAction>(), Box::new(turn_on_light_1_action));
    let turn_off_light_1_action = LightAction {
        light_name: light_name.clone(),
        action_type: LightActionType::Off
    };
    light_mediator.broadcast(TypeId::of::<LightAction>(), Box::new(turn_off_light_1_action));
}
