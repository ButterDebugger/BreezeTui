use crate::{config::Config, modpacks, mods, utils::has_mods, App};
use dialoguer::{theme::ColorfulTheme, Input, Select};
use libium::modpack::zip_extract;
use std::{
    fs::{self},
    path::PathBuf,
    str::FromStr,
    thread,
    time::Duration,
};

impl App {
    pub fn edit_modpack_cli(&mut self, modpack_name: String) {
        let selections = &["Load", "Rename", "Delete"];

        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("What would you like to do to ".to_owned() + &modpack_name)
            .default(0)
            .items(&selections[..])
            .interact_opt()
            .unwrap();

        if let Some(selection) = selection {
            println!();

            match selection {
                0 => load(&self.config, modpack_name),
                1 => rename(&self.config, modpack_name),
                2 => delete(&self.config, modpack_name),
                _ => unreachable!(),
            }

            self.return_home();
        } else {
            self.go_back();
        }
    }
}

pub fn load(config: &Config, modpack_name: String) {
    // Get minecraft path
    let minecraft_path =
        PathBuf::from_str(config.dot_minecraft.as_str()).expect("Minecraft path is invalid");

    let modpack_file_name = modpack_name.clone() + ".zip";

    // Cancel if the user already has mods installed
    if has_mods(config.clone()) {
        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("You already have mods installed. What would you like to do with them?")
            .default(0)
            .items(&["Stash", "Clear"])
            .interact_opt()
            .unwrap();

        println!();

        if let Some(selection) = selection {
            let success = match selection {
                0 => modpacks::stash(config, true),
                1 => mods::clear(config, true),
                _ => unreachable!(),
            };

            if !success {
                println!("There was an error stashing your mods. Please try again.");

                thread::sleep(Duration::from_millis(2500));
                return;
            }
        } else {
            return;
        }
    }

    // Extract the file into the mods dir
    let modpack_path = minecraft_path
        .join("modpacks")
        .join(modpack_file_name.clone());
    let mods_path = minecraft_path.join("mods");

    let _ = zip_extract(&modpack_path, &mods_path);

    println!("{} has successfully been loaded!", modpack_name);

    thread::sleep(Duration::from_millis(2500));
}

fn rename(config: &Config, modpack_name: String) {
    // Get minecraft path
    let minecraft_path =
        PathBuf::from_str(config.dot_minecraft.as_str()).expect("Minecraft path is invalid");

    let modpack_file_name = modpack_name.clone() + ".zip";

    // Get the new name
    let new_name: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("What is the new name of this modpack?")
        .interact_text()
        .unwrap();

    println!();

    let new_file_name = new_name.clone() + ".zip";

    // Rename the modpack
    let old_path = minecraft_path
        .join("modpacks")
        .join(modpack_file_name.clone());
    let new_path = minecraft_path.join("modpacks").join(new_file_name.clone());
    let _ = fs::rename(old_path, new_path);

    println!(
        "{} has successfully been renamed to {}",
        modpack_name, new_name
    );

    thread::sleep(Duration::from_millis(2500));
}

fn delete(config: &Config, modpack_name: String) {
    // Get minecraft path
    let minecraft_path =
        PathBuf::from_str(config.dot_minecraft.as_str()).expect("Minecraft path is invalid");

    let modpack_file_name = modpack_name.clone() + ".zip";

    // Delete the modpack
    let modpack_path = minecraft_path
        .join("modpacks")
        .join(modpack_file_name.clone());
    let _ = fs::remove_file(modpack_path);

    println!("{} has successfully been deleted!", modpack_name);

    thread::sleep(Duration::from_millis(2500));
}
