use dialoguer::{theme::ColorfulTheme, Confirm, Input};
use libium::{get_minecraft_dir, HOME};
use serde::{Deserialize, Serialize};
use std::{
    fs::{self},
    path::PathBuf,
    str::FromStr,
};

#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    version: i32,
    pub dot_minecraft: String,
}

const CURRENT_CONFIG_VERSION: i32 = 1;

pub fn init() -> Config {
    // Get config path and file
    let config_path = HOME.join(".breeze/config.json");

    if config_path.exists() {
        let config_file = fs::File::open(config_path.clone()).expect("Failed to open config file");

        // Read the config
        let maybe_config: Result<Config, serde_json::Error> = serde_json::from_reader(config_file);

        if !maybe_config.is_err() {
            let config: Config = maybe_config.unwrap();
            if config.version == CURRENT_CONFIG_VERSION {
                return config;
            }
        }
        // Fall through and create a new config
    }

    println!();

    // Get the minecraft path
    let mut dot_minecraft_path: PathBuf = get_minecraft_dir();

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

    // Create the config
    if let Some(parent) = config_path.parent() {
        fs::create_dir_all(parent).expect("Failed to create config directory tree");
    }
    let config_file = fs::File::create(config_path).expect("Failed to create config file");

    // Write the config
    let config = Config {
        version: CURRENT_CONFIG_VERSION,
        dot_minecraft: dot_minecraft_path.to_str().unwrap().to_string(),
    };
    serde_json::to_writer_pretty(config_file, &config).expect("Failed to write config file");

    // Run setup
    setup(config.clone());

    // Return the config
    return config;
}

fn setup(config: Config) {
    // Get minecraft path
    let minecraft_path =
        PathBuf::from_str(config.dot_minecraft.as_str()).expect("Minecraft path is invalid");

    // Make sure the mods and modpacks directories exist
    let _ = fs::create_dir_all(minecraft_path.join("mods"));
    let _ = fs::create_dir_all(minecraft_path.join("modpacks"));
}
