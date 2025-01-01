use crate::{config::Config, edit_mod, utils::get_mod_names};
use dialoguer::{theme::ColorfulTheme, FuzzySelect, Select};
use std::{
    fs::{self},
    path::PathBuf,
    str::FromStr,
};

pub fn gui(config: Config) {
    let selections = &["Edit", "Clear", "Reveal in File Explorer"];

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Regarding your mods, what would you like to do")
        .default(0)
        .items(&selections[..])
        .interact_opt()
        .unwrap();

    if let Some(selection) = selection {
        match selection {
            0 => edit(config),
            1 => clear(config),
            2 => open(config),
            _ => panic!(),
        }
    } else {
        println!();
        println!("Returning to main menu");
    }
}

fn edit(config: Config) {
    // Get the list of installed mods
    let installed_mods = get_mod_names(config.clone());

    if installed_mods.is_empty() {
        println!();
        println!("No mods found");
        return;
    }

    let selection = FuzzySelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Which modpack would you like to edit?")
        .default(0)
        .max_length(25)
        .items(&installed_mods)
        .interact_opt()
        .unwrap();

    if let Some(selection) = selection {
        edit_mod::gui(config, installed_mods[selection].clone());
    } else {
        println!();
        println!("Returning to main menu");
    }
}

fn clear(config: Config) {
    // Get minecraft path
    let minecraft_path =
        PathBuf::from_str(config.dot_minecraft.as_str()).expect("Minecraft path is invalid");

    // Get the list of installed mods
    let installed_mods = get_mod_names(config);

    if installed_mods.is_empty() {
        println!();
        println!("No mods found");
        return;
    }

    // Delete every mod
    for mod_name in installed_mods {
        let mod_file_name = mod_name + ".jar";
        let _ = fs::remove_file(minecraft_path.join("mods").join(mod_file_name.clone()));
    }

    println!();
    println!("Mods have been successfully cleared!");
}

fn open(config: Config) {
    // Get minecraft path
    let minecraft_path =
        PathBuf::from_str(config.dot_minecraft.as_str()).expect("Minecraft path is invalid");

    // Open the mods directory
    let _ = open::that(minecraft_path.join("mods"));

    println!();
    println!("Opening mods directory!");
}
