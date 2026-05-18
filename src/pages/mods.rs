use crate::{config::Config, utils::paths::get_mod_names, App, Page};
use console::style;
use dialoguer::{theme::ColorfulTheme, Select};
use std::{
    fs::{self},
    path::PathBuf,
    str::FromStr,
    thread,
    time::Duration,
};

#[derive(strum_macros::Display)]
enum ModsOptions {
    #[strum(to_string = "Edit")]
    Edit,
    #[strum(to_string = "Clear")]
    Clear,
    #[strum(to_string = "Reveal in File Explorer")]
    OpenExplorer,
}

impl App {
    pub fn mods_cli(&mut self) {
        let selections;
        let mod_names = get_mod_names(self.config.clone());

        if mod_names.is_empty() {
            selections = vec![ModsOptions::OpenExplorer];

            // Notify that there are no mods
            println!("{} {}", style("!").red(), style("No Mods Found").bold());
        } else {
            selections = vec![
                ModsOptions::Edit,
                ModsOptions::Clear,
                ModsOptions::OpenExplorer,
            ];

            // Print a sample of the mods
            println!("{} {}", style("#").red(), style("Mods List").bold());

            for mod_name in mod_names {
                println!("  {}", mod_name);
            }
        }

        println!();

        // Ask the user what they want to do
        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("What would you like to do")
            .default(0)
            .items(&selections)
            .interact_opt()
            .unwrap();

        if let Some(selection) = selection {
            match selections[selection] {
                ModsOptions::Edit => {
                    self.goto(Page::ModsList);
                }
                ModsOptions::Clear => {
                    println!();

                    let _ = clear(&self.config, false);
                    self.return_home();
                }
                ModsOptions::OpenExplorer => {
                    println!();

                    open_mods_folder_in_explorer(&self.config);
                    self.return_home();
                }
            }
        } else {
            self.go_back();
        }
    }
}

/// Clears the mods folder of jar files
///
/// # Returns
///
/// `true` if the clear was successful, and `false` otherwise.
pub fn clear(config: &Config, silent: bool) -> bool {
    // Get minecraft path
    let minecraft_path =
        PathBuf::from_str(config.dot_minecraft.as_str()).expect("Minecraft path is invalid");

    // Get the list of installed mods
    let installed_mods = get_mod_names(config.clone());

    if installed_mods.is_empty() {
        if !silent {
            println!("No mods found");

            thread::sleep(Duration::from_millis(2500));
        }
        return false;
    }

    // Delete every mod
    for mod_name in installed_mods {
        let mod_file_name = mod_name + ".jar";
        let _ = fs::remove_file(minecraft_path.join("mods").join(mod_file_name.clone()));
    }

    if !silent {
        println!("Mods have been successfully cleared!");

        thread::sleep(Duration::from_millis(2500));
    }

    true
}

fn open_mods_folder_in_explorer(config: &Config) {
    // Get minecraft path
    let minecraft_path =
        PathBuf::from_str(config.dot_minecraft.as_str()).expect("Minecraft path is invalid");

    // Open the mods directory
    let _ = open::that(minecraft_path.join("mods"));

    println!("Opening mods directory!");

    thread::sleep(Duration::from_millis(2500));
}
