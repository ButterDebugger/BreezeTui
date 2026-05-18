use crate::utils::paths::{get_config_path, get_minecraft_dir, get_modpacks_dir};
use dialoguer::{theme::ColorfulTheme, Confirm, Input};
use serde::{Deserialize, Serialize};
use std::{
    fs::{self},
    path::PathBuf,
    str::FromStr,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    version: i32,
    pub dot_minecraft: String,
    pub active_modpack: Option<String>,
}

const CURRENT_CONFIG_VERSION: i32 = 2;

impl Config {
    pub fn load_from_disk() -> Option<Self> {
        let config_path = get_config_path();

        // Load the config from disk if it exists
        if config_path.exists() {
            let config_file =
                fs::File::open(config_path.clone()).expect("Failed to open config file");

            // Read the config
            let config: Result<Config, serde_json::Error> = serde_json::from_reader(config_file);

            if let Ok(config) = config {
                if config.version == CURRENT_CONFIG_VERSION {
                    return Some(config);
                }
            }
        }

        None
    }

    pub fn create_config_prompt() -> Self {
        // Get the minecraft path
        let mut dot_minecraft_path: PathBuf = get_minecraft_dir().unwrap();

        println!(
            "The default minecraft directory is {}",
            dot_minecraft_path.display()
        );

        // Ask the user if they would like to change the minecraft directory path
        if Confirm::new()
            .with_prompt("Would you like to specify a custom minecraft directory?")
            .default(false)
            .interact()
            .unwrap()
        {
            println!();

            let custom_path: String = Input::with_theme(&ColorfulTheme::default())
                .with_prompt("What is the path to the minecraft directory?")
                .interact_text()
                .unwrap();

            dot_minecraft_path = PathBuf::from(custom_path);

            println!(
                "The minecraft directory has been set to {}",
                dot_minecraft_path.display()
            );
        }

        // Return the config
        Config {
            version: CURRENT_CONFIG_VERSION,
            dot_minecraft: dot_minecraft_path.to_str().unwrap().to_string(),
            active_modpack: None,
        }
    }

    // Create the necessary folders
    pub fn init(self) {
        // Get minecraft path
        let minecraft_path =
            PathBuf::from_str(self.dot_minecraft.as_str()).expect("Minecraft path is invalid");

        // Make sure the mods and modpacks directories exist
        let _ = fs::create_dir_all(minecraft_path.join("mods"));
        let _ = fs::create_dir_all(get_modpacks_dir());
    }

    pub fn save(self) {
        let config_path = get_config_path();

        if let Some(parent) = config_path.parent() {
            fs::create_dir_all(parent).expect("Failed to create config directory tree");
        }

        let config_file = fs::File::create(config_path).expect("Failed to create config file");

        serde_json::to_writer_pretty(config_file, &self).expect("Failed to write config file");
    }
}
