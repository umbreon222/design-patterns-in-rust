pub mod command {
    pub trait Command {
        fn execute(&mut self);
    }
}

pub mod observer {
    // https://www.dofactory.com/net/observer-design-pattern
}

pub mod mediator {
    // https://www.dofactory.com/net/mediator-design-pattern
}

use std::rc::Rc;
use core::cell::RefCell;
use command::Command;

pub struct Light {
    state: bool
}

impl Light {
    pub fn on(&mut self) {
        self.state = true;
        println!("Light turned on");
    }

    pub fn off(&mut self) {
        self.state = false;
        println!("Light turned off");
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

fn execute_command_pattern_example() {
    let light = Rc::new(RefCell::new(Light { state: false }));
    let on_command = Box::new(LightOnCommand { light: light.clone() });
    let off_command = Box::new(LightOffCommand { light: light.clone() });
    let mut remote = Remote { command: on_command };
    remote.execute();
    remote.set_command(off_command);
    remote.execute();
}
/* </command pattern example> */

fn main() {
    execute_command_pattern_example();
}
