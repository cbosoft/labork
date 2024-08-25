use shakemyleg::StateMachine;
use sled::Tree;

use crate::action::Action;


pub struct Actor {
    name: String,
    state_machine: StateMachine,
    tasks: Tree,
}

impl Actor {
    pub fn new(name: String, state_machine: StateMachine, tasks: Tree) -> Self {
        Self {
            name, state_machine, tasks
        }
    }

    pub async fn run(mut self) {
        loop {
            match self.state_machine.run::<(), Action>(()) {
                Ok(Some(action)) => {
                    //
                },
                Ok(None) => {
                    //
                },
                Err(e) => {
                    //
                }
            }
        }
    }
}
