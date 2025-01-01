use crate::config::Config;
use dialoguer::{theme::ColorfulTheme, Select};
use std::{
    fs::{self},
    path::PathBuf,
    str::FromStr,
};

pub fn gui(config: Config, mod_name: String) {
    let selections = &["Delete"];

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("What would you like to do to ".to_owned() + &mod_name)
        .default(0)
        .items(&selections[..])
        .interact_opt()
        .unwrap();

    if let Some(selection) = selection {
        match selection {
            0 => delete(config, mod_name),
            _ => panic!(),
        }
    } else {
        println!();
        println!("Returning to main menu");
    }
}

fn delete(config: Config, mod_name: String) {
    // Get minecraft path
    let minecraft_path =
        PathBuf::from_str(config.dot_minecraft.as_str()).expect("Minecraft path is invalid");

    let mod_file_name = mod_name.clone() + ".jar";

    // Delete the mod
    let mod_path = minecraft_path.join("mods").join(mod_file_name.clone());
    let _ = fs::remove_file(mod_path);

    println!();
    println!("{} has successfully been deleted!", mod_name)
}
