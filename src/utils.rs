use crate::config::Config;
use std::{
    fs::{self},
    path::PathBuf,
    str::FromStr,
};

pub fn has_mods(config: Config) -> bool {
    // Get minecraft path
    let minecraft_path =
        PathBuf::from_str(config.dot_minecraft.as_str()).expect("Minecraft path is invalid");

    // Loop through each item in the mods path and check if there is a mod file
    for ele in fs::read_dir(minecraft_path.join("mods")).expect("Cannot read mods directory") {
        if let Ok(entry) = ele {
            let entry_path = entry.path();
            if entry_path.is_file() && entry_path.extension().unwrap() == "jar" {
                return true;
            }
        }
    }

    return false;
}

pub fn get_mod_names(config: Config) -> Vec<String> {
    // Get minecraft path
    let minecraft_path =
        PathBuf::from_str(config.dot_minecraft.as_str()).expect("Minecraft path is invalid");

    // Get the list of installed mods
    let mut installed_mods: Vec<String> = Vec::new();
    for ele in fs::read_dir(minecraft_path.join("mods")).expect("Cannot read mods directory") {
        if let Ok(entry) = ele {
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
    }

    return installed_mods;
}

pub fn get_modpack_names(config: Config) -> Vec<String> {
    // Get minecraft path
    let minecraft_path =
        PathBuf::from_str(config.dot_minecraft.as_str()).expect("Minecraft path is invalid");

    // Get the list of modpacks
    let mut modpack_names: Vec<String> = Vec::new();
    for ele in
        fs::read_dir(minecraft_path.join("modpacks")).expect("Cannot read modpacks directory")
    {
        if let Ok(entry) = ele {
            let entry_path = entry.path();
            if entry_path.is_file() && entry_path.extension().unwrap() == "zip" {
                modpack_names.push(
                    entry_path
                        .file_stem()
                        .unwrap()
                        .to_string_lossy()
                        .to_string(),
                );
            }
        }
    }

    return modpack_names;
}
