use std::collections::HashSet;

use sled::{Db, Tree};

use crate::error::{OrkError, OrkResult};
use crate::actor::Actor;

const ORK_DB_NAME: &'static str = "./ork_state.db-sled";


#[derive(Clone)]
pub struct Ork {
    db: Db,
    active_actor_names: HashSet<String>,
}


impl Ork {

    pub fn open() -> OrkResult<Self> {
        let db = sled::open(ORK_DB_NAME)?;
        let active_actor_names = HashSet::new();
        Self { db, active_actor_names }.initialised()
    }

    fn initialised(mut self) -> OrkResult<Self> {
        // re-create actors
        let actors = self.db.open_tree("actors")?; // keys are names, values are SML source
        let n = actors.len();
        for item in actors.iter() {
            let (k, v) = item?;
            let name = String::from_utf8(k.to_vec())?;
            let sml_source = String::from_utf8(v.to_vec())?;
            let _ = self.start_actor(name, sml_source)?;
        }

        if self.db.was_recovered() {
            log::info!("Recovered state from store ({n} actors).");
        }
        else {
            // read actors from SML
        }

        Ok(self)
    }

    pub fn create_actor(&mut self, name: String, sml_source: String) -> OrkResult<Tree> {
        let actors = self.db.open_tree("actors")?; // keys are names, values are SML source
        let _ = actors.insert(name.as_bytes(), sml_source.as_bytes())?;
        self.start_actor(name, sml_source)
    }

    fn start_actor(&mut self, name: String, sml_source: String) -> OrkResult<Tree> {
        if self.active_actor_names.contains(&name) {
            return Err(OrkError::ActorAlreadyExists(name));
        }

        let sm = shakemyleg::compile(&sml_source)?;
        let task_tree_name = format!("{name}_tasks");
        let task_tree = self.db.open_tree(&task_tree_name)?;
        let actor = Actor::new(name.clone(), sm, task_tree.clone());
        tokio::task::spawn(actor.run());
        self.active_actor_names.insert(name);
        Ok(task_tree)
    }

    pub async fn run(self) {
        log::info!("LabOrk running");
        loop {
            // TODO
            tokio::task::yield_now().await;
        }
    }
}

