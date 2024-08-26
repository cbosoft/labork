use std::collections::{HashMap, HashSet};
use std::fs::{read_to_string, File};
use std::path::PathBuf;

use sled::Db;

use crate::action::Action;
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
            let actions_store = self.db.open_tree("actions")?;
            // read action definitions from YAML
            let actors_path = PathBuf::new().join("actions");
            if actors_path.exists() {
                let contents = std::fs::read_dir(actors_path)?;
                for child in contents {
                    let child = child?;
                    let src = read_to_string(child.path())?;
                    let actions: HashMap<String, Action> = serde_yaml::from_str(&src)?;

                    for (name, action) in &actions {
                        let action_bytes = bincode::serialize(action)?;
                        let _ = actions_store.insert(name, action_bytes)?;
                        log::info!("Registered action '{name}'")
                    }
                }
            }

            // read actors from SML
            let actors_path = PathBuf::new().join("actors");
            if actors_path.exists() {
                let contents = std::fs::read_dir(actors_path)?;
                for child in contents {
                    let child = child?;
                    let name = child.path().with_extension("").file_name().unwrap().to_str().unwrap().to_string();
                    let sml_source = read_to_string(child.path())?;
                    self.create_actor(name, sml_source)?;
                }
            }
        }

        Ok(self)
    }

    pub fn create_actor(&mut self, name: String, sml_source: String) -> OrkResult<()> {
        let actors = self.db.open_tree("actors")?; // keys are names, values are SML source
        if actors.contains_key(&name)? {
            return Err(OrkError::ActorAlreadyExists(name));
        }
        let _ = actors.insert(name.as_bytes(), sml_source.as_bytes())?;
        log::info!("Created actor '{name}'");
        self.start_actor(name, sml_source)
    }

    fn start_actor(&mut self, name: String, sml_source: String) -> OrkResult<()> {
        if self.active_actor_names.contains(&name) {
            return Err(OrkError::ActorAlreadyExists(name));
        }

        let sm = shakemyleg::compile(&sml_source)?;
        let actor = Actor::new(name.clone(), sm, &self.db);
        tokio::task::spawn(actor?.run());
        self.active_actor_names.insert(name);
        Ok(())
    }

    pub async fn run(self) {
        log::info!("LabOrk running");
        loop {
            // TODO
            tokio::task::yield_now().await;
        }
    }
}

