use std::fs::File;
use std::path::PathBuf;
use std::io::{read_to_string, Write};
use std::net::Ipv4Addr;

use serde::{Serialize, Deserialize};

use crate::error::{OrkError, OrkResult};


fn default_ip() -> Ipv4Addr {
    Ipv4Addr::new(127, 0, 0, 1)
}


fn default_api_port() -> u16 {
    3370 // drop first '3', flip horizontally = Ork
}


fn default_ui_port() -> u16 {
    8370 // drop the '8', flip horizontally = Ork
}


#[derive(Deserialize, Serialize, Clone)]
pub struct Config {
    #[serde(default = "default_ip")]
    pub api_ip: Ipv4Addr,
    #[serde(default = "default_api_port")]
    pub api_port: u16,
    #[serde(default = "default_ip")]
    pub ui_ip: Ipv4Addr,
    #[serde(default = "default_ui_port")]
    pub ui_port: u16,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            api_ip: default_ip(),
            api_port: default_api_port(),
            ui_ip: default_ip(),
            ui_port: default_ui_port(),
        }
    }
}

impl Config {
    pub fn open() -> OrkResult<Self> {
        let config_path = PathBuf::new().join("config.yaml");
        if config_path.is_file() {
            let f = File::options()
                .create(false)
                .read(true)
                .open(&config_path)?;
            let config_yaml = read_to_string(f)?;
            let config: Config = serde_yaml::from_str(&config_yaml)?;
            log::info!("Opened ork config at {config_path:?}.");
            Ok(config)
        }
        else if config_path.is_dir() {
            Err(OrkError::ConfigNotAFile)
        }
        else {
            let mut f = File::options()
                .create(true)
                .write(true)
                .open(&config_path)?;
            let config = Config::default();
            let config_yaml = serde_yaml::to_string(&config)?;
            write!(&mut f, "{}", config_yaml)?;
            log::info!("Created new ork config at {config_path:?}.");
            Ok(config)
        }
    }
}
