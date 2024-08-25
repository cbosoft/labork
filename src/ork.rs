use sled::{Db, Tree};

use crate::error::OrkResult;
use crate::actor::Actor;

const ORK_DB_NAME: &'static str = "./ork_state.db-sled";


pub struct Ork {
    db: Db,
}


impl Ork {

    pub fn open() -> OrkResult<Self> {
        let db = sled::open(ORK_DB_NAME)?;
        Self { db, }.initialised()
    }

    fn initialised(self) -> OrkResult<Self> {
        // re-create actors
        let actors = self.db.open_tree("actors")?; // keys are names, values are SML source
        for item in actors.iter() {
            let (k, v) = item?;
            let name = String::from_utf8(k.to_vec())?;
            let sml_source = String::from_utf8(v.to_vec())?;
            let _ = self.start_actor(name, sml_source)?;
        }

        Ok(self)
    }

    pub fn create_actor(&self, name: String, sml_source: String) -> OrkResult<Tree> {
        let actors = self.db.open_tree("actors")?; // keys are names, values are SML source
        let _ = actors.insert(name.as_bytes(), sml_source.as_bytes())?;
        self.start_actor(name, sml_source)
    }

    fn start_actor(&self, name: String, sml_source: String) -> OrkResult<Tree> {
        // TODO: ensure actor doesn't already exist
        let sm = shakemyleg::compile(&sml_source)?;
        let task_tree_name = format!("{name}_tasks");
        let task_tree = self.db.open_tree(&task_tree_name)?;
        let actor = Actor::new(name, sm, task_tree.clone());
        tokio::task::spawn(actor.run());
        Ok(task_tree)
    }

    pub async fn run(self) {
        log::info!("LabOrk running");
        loop {
            // TODO
        }
    }
}

