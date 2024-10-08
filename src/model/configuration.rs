use std::fs;
use std::io::{Error, Read, Write};
use std::{fs::File, path::Path};

use dirs::config_dir;
use serde::{Deserialize, Serialize};
use tracing::{debug, info};

use super::feature::Features;
use super::role::Role;

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Configuration {
    pub roles: Vec<Role>,
    pub features: Features,
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
        debug!("Searching config file in {:?}", full_config_path);

        let mut config_file: File = if !Path::new(&full_config_path).exists() {
            debug!("{:?} does not exist", full_config_path);

            create_config_file(&full_config_path);
            read_config_file(&full_config_path)
        } else {
            debug!("Configuration file found at {:?}", full_config_path);
            read_config_file(&full_config_path)
        };

        let mut buffer = String::new();
        config_file.read_to_string(&mut buffer)?;

        Ok(serde_json::from_str(&buffer)?)
    }
}

fn read_config_file(full_config_path: &std::path::PathBuf) -> File {
    debug!(
        "Trying to read configuration file at {:?}",
        full_config_path
    );

    File::open(full_config_path).expect("Could not open configuration file")
}

fn create_config_file(full_config_path: &std::path::PathBuf) {
    info!("Creating configuration file at {:?}", full_config_path);
    let mut file = File::create(full_config_path).expect("Could not create configuration file");
    let default_config = Configuration::default();
    let default_content =
        serde_json::to_string_pretty(&default_config).expect("Default config not serializable");
    file.write_all(default_content.as_bytes())
        .expect("Could not write buffer to file");
    file.flush().expect("Could not flush output stream");
}

#[cfg(test)]
mod tests {
    use mockall::automock;

    use super::*;

    #[automock]
    trait ReadFile {
        fn read_to_string(&mut self, buf: &mut String) -> Result<usize, Error>;
    }

    struct MockFile {
        content: String,
    }

    impl ReadFile for MockFile {
        fn read_to_string(&mut self, buf: &mut String) -> Result<usize, Error> {
            buf.push_str(&self.content);
            Ok(self.content.len())
        }
    }

    #[test]
    fn test_load() {
        let mock_content = r#"{
            "features": {
              "LOG_TO_CHANNEL": {
                "ENABLED": true,
                "CHANNEL_ID": 123456789012345678
              },
              "ASSIGN_GUEST_ROLE_ON_JOIN": {
                "ENABLED": false,
                "ROLE_ID": 123456789012345678
              }
            },
            "roles":
            [
                {
                    "id": 123,
                    "name": "Test",
                    "symbol": "🤖",
                    "weight": 20
                }
            ]}"#;
        let mut mock_file = MockFile {
            content: mock_content.to_string(),
        };

        let mut buffer = String::new();
        mock_file
            .read_to_string(&mut buffer)
            .expect("Could not read mock content");

        let config: Configuration =
            serde_json::from_str(&buffer).expect("Could not parse configuration");
        assert_eq!(config.roles.len(), 1);
        let first_role = config.roles.get(0).expect("Could not get role");
        let features = config.features;

        assert_eq!(first_role.id, 123);
        assert_eq!(first_role.name, "Test");
        assert_eq!(first_role.symbol, "🤖");
        assert_eq!(features.log_to_channel.enabled, true);
        assert_eq!(features.log_to_channel.channel_id, 123456789012345678);
        assert_eq!(features.assign_guest_role_on_join.enabled, false);
        assert_eq!(
            features.assign_guest_role_on_join.role_id,
            123456789012345678
        );
    }
}
