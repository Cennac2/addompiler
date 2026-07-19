use std::{collections::HashMap, fs, path::Path};

use log::{debug, error};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub addon_name: String,
    pub paths: AddonPaths,
    pub profiles: Profiles,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AddonPaths {
    pub bp_path: String,
    pub rp_path: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Profiles {
    #[serde(flatten)]
    pub profiles: HashMap<String, ProfileInfo>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ProfileInfo {
    pub before_build: Option<Vec<CommandInfo>>,
    pub after_build: Option<Vec<CommandInfo>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CommandInfo {
    pub command: String,
}

const CONFIG_PATH: &str = "addompiler_config.json";

impl Config {
    pub fn get_config(directory: &String) -> Self {
        let dir = Path::new(directory);
        let path = &dir.join(CONFIG_PATH);

        if !path.exists() {
            error!("Config not found! Did you initialize first?");
            std::process::exit(1);
        }

        let content = match fs::read_to_string(path) {
            Ok(c) => c,
            Err(e) => {
                error!("Failed to read {}: {}", CONFIG_PATH, e);
                String::new()
            }
        };

        let config: Config = match serde_json::from_str(&content) {
            Ok(config) => config,
            Err(e) => {
                error!("Failed to parse {}: {}", CONFIG_PATH, e);
                Config::default()
            }
        };

        match serde_json::to_string_pretty(&config) {
            Ok(serialized) => {
                if let Err(e) = fs::write(path, serialized) {
                    error!("Failed to re-sync config file: {}", e);
                }
            }
            Err(e) => {
                error!("Failed to serialize config for re-sync: {}", e);
            }
        }

        config
    }

    pub fn init_config(directory: &String) {
        debug!("Creating config");
        let dir = Path::new(directory);
        let path = dir.join(CONFIG_PATH);

        if !path.exists() {
            let default_config = Config::default();

            match serde_json::to_string_pretty(&default_config) {
                Ok(serialized) => {
                    if let Err(e) = fs::write(path, serialized) {
                        error!("Failed to create config file: {}", e);
                    }
                }
                Err(e) => {
                    error!("Failed to serialize default config: {}", e);
                }
            }
        }
    }

    // probably not needed at the moment, so leaving it here for later
    // pub fn get_profiles(&self) -> &HashMap<String, ProfileInfo> {
    //     &self.profiles.profiles
    // }

    pub fn get_profile(&self, name: &str) -> Option<&ProfileInfo> {
        self.profiles.profiles.get(name)
    }
}

impl Default for Config {
    fn default() -> Self {
        let mut profiles = HashMap::new();
        profiles.insert(
            String::from("default"),
            ProfileInfo {
                before_build: Some(vec![CommandInfo {
                    command: String::from("echo building"),
                }]),
                after_build: Some(vec![CommandInfo {
                    command: String::from("echo built"),
                }]),
            },
        );

        Config {
            addon_name: String::from("Addon Name"),
            paths: AddonPaths {
                bp_path: String::from("path"),
                rp_path: String::from("path"),
            },
            profiles: Profiles { profiles },
        }
    }
}
