use crate::{config::Config, edit_modpack, utils::has_mods};
use dialoguer::{theme::ColorfulTheme, FuzzySelect, Input, Select};
use libium::modpack::zip_create_from_directory;
use std::{
    env::temp_dir,
    fs::{self, create_dir_all},
    path::PathBuf,
    str::FromStr,
};

pub fn gui(config: Config) {
    let selections = &["Edit", "Stash"]; // TODO: add an "Import" option to add a modpack from a website

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Regarding your modpacks, what would you like to do")
        .default(0)
        .items(&selections[..])
        .interact_opt()
        .unwrap();

    if let Some(selection) = selection {
        match selection {
            0 => edit(config),
            1 => stash(config),
            _ => panic!(),
        }
    } else {
        println!();
        println!("Returning to main menu");
    }
}

fn edit(config: Config) {
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

    if modpack_names.is_empty() {
        println!();
        println!("No modpacks found");
        return;
    }

    let selection = FuzzySelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Which modpack would you like to edit?")
        .default(0)
        .max_length(25)
        .items(&modpack_names)
        .interact_opt()
        .unwrap();

    if let Some(selection) = selection {
        edit_modpack::gui(config, modpack_names[selection].clone());
    } else {
        println!();
        println!("Returning to main menu");
    }
}

fn stash(config: Config) {
    if !has_mods(config.clone()) {
        println!();
        println!("You do not have any mods installed");
        return;
    }

    // Get minecraft path
    let minecraft_path =
        PathBuf::from_str(config.dot_minecraft.as_str()).expect("Minecraft path is invalid");

    // Ask for the modpack's name
    let modpack_name: String = Input::new()
        .with_prompt("What is the name of this modpack?")
        .interact_text()
        .unwrap();

    // Create temp directory
    let temp_path = temp_dir().join("breeze_mods");
    create_dir_all(temp_path.clone()).expect("Failed to create temp directory");

    // Move all active mods to the temp directory
    for ele in fs::read_dir(minecraft_path.join("mods")).expect("Cannot read mods directory") {
        if let Ok(entry) = ele {
            let entry_path = entry.path();
            if entry_path.is_file() && entry_path.extension().unwrap() == "jar" {
                let _ = fs::rename(
                    // TODO: handle error (program using jar)
                    entry_path.clone(),
                    temp_path.join(entry_path.file_name().unwrap()),
                );
            }
        }
    }

    // Create zipped modpack path
    let modpack_path = minecraft_path.join("modpacks").join(modpack_name + ".zip");

    // Archive mods
    let _ = zip_create_from_directory(&modpack_path, &temp_path);
    let _ = fs::remove_dir_all(temp_path);

    println!();
    println!("Your mods have successfully been stashed away!")
}
