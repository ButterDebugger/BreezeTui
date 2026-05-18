use crate::config::Config;
use std::{fs, path::PathBuf, str::FromStr};

pub fn get_breeze_dir() -> PathBuf {
    home::home_dir()
        .expect("Could not get user's home directory")
        .join(".breeze")
}

pub fn get_modpacks_dir() -> PathBuf {
    get_breeze_dir().join("modpacks")
}

pub fn get_modpack_path(modpack_name: String) -> PathBuf {
    get_modpacks_dir().join(format!("{}.modx", modpack_name))
}

pub fn get_config_path() -> PathBuf {
    get_breeze_dir().join("config.json")
}

/// Gets the default Minecraft instance directory based on the current compilation `target_os`
pub fn get_minecraft_dir() -> Option<PathBuf> {
    let home = home::home_dir().expect("Could not get user's home directory");

    #[cfg(target_os = "windows")]
    return Some(home.join("AppData").join("Roaming").join(".minecraft"));

    #[cfg(target_os = "macos")]
    return Some(
        home.join("Library")
            .join("Application Support")
            .join("minecraft"),
    );

    #[cfg(target_os = "linux")]
    return Some(home.join(".minecraft"));

    #[allow(unreachable_code)]
    None
}

pub fn has_mods(config: Config) -> bool {
    // Get minecraft path
    let minecraft_path =
        PathBuf::from_str(config.dot_minecraft.as_str()).expect("Minecraft path is invalid");

    // Loop through each item in the mods path and check if there is a mod file
    for entry in fs::read_dir(minecraft_path.join("mods"))
        .expect("Cannot read mods directory")
        .flatten()
    {
        let entry_path = entry.path();
        if entry_path.is_file() && entry_path.extension().unwrap() == "jar" {
            return true;
        }
    }

    false
}

pub fn get_mod_names(config: Config) -> Vec<String> {
    // Get minecraft path
    let minecraft_path =
        PathBuf::from_str(config.dot_minecraft.as_str()).expect("Minecraft path is invalid");

    // Get the list of installed mods
    let mut installed_mods: Vec<String> = Vec::new();
    for entry in fs::read_dir(minecraft_path.join("mods"))
        .expect("Cannot read mods directory")
        .flatten()
    {
        let entry_path = entry.path();
        if entry_path.is_file() && entry_path.extension().unwrap() == "jar" {
            installed_mods.push(
                entry_path
                    .file_stem()
                    .unwrap()
                    .to_string_lossy()
                    .to_string(),
            );
        }
    }

    installed_mods
}

pub fn get_modpack_names() -> Vec<String> {
    // Get the list of modpacks
    let mut modpack_names: Vec<String> = Vec::new();

    for entry in fs::read_dir(get_modpacks_dir())
        .expect("Cannot read modpacks directory")
        .flatten()
    {
        let entry_path = entry.path();
        if entry_path.is_file() && entry_path.extension().unwrap() == "modx" {
            modpack_names.push(
                entry_path
                    .file_stem()
                    .unwrap()
                    .to_string_lossy()
                    .to_string(),
            );
        }
    }

    modpack_names
}
