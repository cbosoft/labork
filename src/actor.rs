use std::time::Duration;

use serde::Deserialize;
use shakemyleg::StateMachine;
use sled::{Db, Tree};

use crate::error::OrkResult;
use crate::action::{Action, ActionExt};


#[derive(Deserialize)]
struct ActionList {
    pub action: String,
}

pub struct Actor {
    name: String,
    state_machine: StateMachine,
    status: Tree,
    tasks: Tree,
    data: Tree,
    actions: Tree,
}

impl Actor {
    pub fn new(name: String, state_machine: StateMachine, db: &Db) -> OrkResult<Self> {
        let status = db.open_tree("actor_status")?;

        let task_tree_name = format!("{name}_tasks");
        let tasks = db.open_tree(&task_tree_name)?;
        
        let data_tree_name = format!("{name}_data");
        let data = db.open_tree(&data_tree_name)?;
        data.clear()?;

        let actions = db.open_tree("actions")?;
        Ok(Self {
            name, state_machine, status, tasks, data, actions
        })
    }

    pub async fn run(mut self) {
        loop {
            match self.state_machine.run::<(), ActionList>(()) {
                Ok(Some(action_list)) => {
                    // TODO: list literals in SML
                    for action_name in vec![action_list.action] {
                        // set status
                        self.status.insert(&self.name, action_name.as_bytes()).unwrap();

                        // get action
                        let action: Action = match self.actions.get(&action_name).unwrap() {
                            Some(action_bytes) => {
                                bincode::deserialize(&action_bytes).unwrap()
                            },
                            None => {
                                if action_name == "NoOp" {
                                    Action::NoOp
                                }
                                else {
                                    // error: unknown action requested!
                                    panic!("Unknown action {}", action_name);
                                }
                            }
                        };

                        action.run(self.data.clone());
                    }
                },
                Ok(None) => {
                    // Nothing to do (machine is complete)
                    log::info!("Actor {} is finished.", self.name);
                    break;
                },
                Err(e) => {
                    log::warn!("Failed to run state machine for {0}: {e:?}", self.name);
                }
            }

            tokio::time::sleep(Duration::from_secs(1)).await;
        }
    }
}
