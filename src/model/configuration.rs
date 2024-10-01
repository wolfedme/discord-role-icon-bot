use std::fs;
use std::io::{Error, Read, Write};
use std::{fs::File, path::Path};

use dirs::config_dir;
use serde::{Deserialize, Serialize};
use tracing::{debug, info};

use super::role::Role;

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Configuration {
    pub roles: Vec<Role>,
}

const CONFIG_PATH: &str = "role-icon-bot/configuration.json";

impl Configuration {
    pub fn load() -> Result<Configuration, Error> {
        let config_dir = config_dir().expect("Could not find configuration directory");
        let full_config_path = config_dir.join(CONFIG_PATH);

        // Ensure the directory exists
        if let Some(parent_dir) = full_config_path.parent() {
            fs::create_dir_all(parent_dir)?;
        }
        // Load or create the configuration file
        debug!("Loading configuration from {:?}", full_config_path);
        let mut config_file: File = if !Path::new(&full_config_path).exists() {
            debug!("Configuration file not found at {:?}", full_config_path);
            info!("Creating configuration file at {:?}", full_config_path);
            let mut file =
                File::create(&full_config_path).expect("Could not create configuration file");
            let default_config = Configuration::default();
            let default_content = serde_json::to_string_pretty(&default_config)
                .expect("Default config not serializable");
            file.write_all(default_content.as_bytes())
                .expect("Could not write buffer to file");
            file.flush().expect("Could not flush output stream");
            File::open(&full_config_path).expect("Could not open configuration file")
        } else {
            debug!("Configuration file found at {:?}", full_config_path);
            File::open(&full_config_path).expect("Could not open configuration file")
        };

        let mut buffer = String::new();
        config_file.read_to_string(&mut buffer)?;

        Ok(serde_json::from_str(&buffer)?)
    }
}
