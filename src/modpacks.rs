use crate::{
    config::Config,
    edit_modpack,
    utils::{get_modpack_names, has_mods},
    App, Page,
};
use console::style;
use dialoguer::{theme::ColorfulTheme, FuzzySelect, Input, Select};
use libium::modpack::zip_create_from_directory;
use std::{
    env::temp_dir,
    fs::{self, create_dir_all},
    path::PathBuf,
    str::FromStr,
    thread,
    time::Duration,
};

#[derive(strum_macros::Display)]
enum ModpacksOptions {
    #[strum(to_string = "Edit")]
    Edit,
    #[strum(to_string = "Stash")]
    Stash,
    #[strum(to_string = "Import")]
    Import,
    #[strum(to_string = "Reveal in File Explorer")]
    OpenExplorer,
}

impl App {
    pub fn modpacks_cli(&mut self) {
        let selections;
        let modpack_names = get_modpack_names(self.config.clone());

        if modpack_names.is_empty() {
            selections = vec![
                ModpacksOptions::Stash,
                ModpacksOptions::Import,
                ModpacksOptions::OpenExplorer,
            ];

            // Notify that there are no modpacks
            println!("{} {}", style("!").red(), style("No Modpacks Saved").bold());
        } else {
            selections = vec![
                ModpacksOptions::Edit,
                ModpacksOptions::Stash,
                ModpacksOptions::Import,
                ModpacksOptions::OpenExplorer,
            ];

            // Print a sample of the modpacks
            println!("{} {}", style("#").red(), style("Modpacks List").bold());

            for modpack_name in modpack_names {
                println!("  {}", modpack_name);
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
                ModpacksOptions::Edit => {
                    self.goto(Page::ModpacksList);
                }
                ModpacksOptions::Stash => {
                    println!();

                    let _ = stash(&self.config, false);
                    self.return_home();
                }
                ModpacksOptions::Import => {
                    println!();

                    import(&self.config);
                    self.return_home();
                }
                ModpacksOptions::OpenExplorer => {
                    println!();

                    open_modpacks_folder_in_explorer(&self.config);
                    self.return_home();
                }
            }
        } else {
            self.go_back();
        }
    }
}

fn import(config: &Config) {
    // Get minecraft path
    let minecraft_path =
        PathBuf::from_str(config.dot_minecraft.as_str()).expect("Minecraft path is invalid");

    // Ask user for modpack archive path
    println!("Please select the modpack archive");

    let archive_path = rfd::FileDialog::new().pick_file();

    if archive_path.is_none() {
        println!("No file selected");

        thread::sleep(Duration::from_millis(2500));
        return;
    }

    let archive_path = archive_path.unwrap();

    // Check if file exists
    if !archive_path.is_file() {
        println!("The file {} does not exist", archive_path.display());

        thread::sleep(Duration::from_millis(2500));
        return;
    }

    // Check if modpack archive is valid
    let extension = archive_path.extension();
    let file_stem = archive_path.file_stem();

    if extension.is_none() || file_stem.is_none() {
        println!(
            "The file {} is not a valid modpack archive",
            archive_path.display()
        );

        thread::sleep(Duration::from_millis(2500));
        return;
    }

    let extension = extension.unwrap().to_str().unwrap();
    let file_stem = file_stem.unwrap().to_str().unwrap();

    println!();

    // Ask the user for the modpacks name
    let pack_name: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("What is the name of this modpack?")
        .with_initial_text(file_stem)
        .interact_text()
        .unwrap();

    println!();

    // Save the modpack
    match extension {
        "zip" => {
            // Move the zip file into the modpacks directory
            let modpack_path = minecraft_path
                .join("modpacks")
                .join(pack_name.clone() + ".zip");

            fs::rename(archive_path, modpack_path).unwrap();
        }
        "7z" => {
            // Create temp directory
            let temp_path = temp_dir().join("breeze_7z");
            create_dir_all(temp_path.clone()).expect("Failed to create temp directory");

            // Unzip the 7z file into the temp directory
            sevenz_rust::decompress_file(&archive_path, &temp_path)
                .expect("Failed to decompress 7z file");

            // Delete the 7z file
            let _ = fs::remove_file(&archive_path);

            // Repackage the temp directory into a zip file
            let modpack_path = minecraft_path
                .join("modpacks")
                .join(pack_name.clone() + ".zip");

            zip_create_from_directory(&modpack_path, &temp_path).unwrap();
        }
        _ => {
            println!(
                "The file {} is not a supported modpack archive at this time",
                archive_path.display()
            );

            thread::sleep(Duration::from_millis(2500));
            return;
        }
    }

    // Ask if user wants to use the modpack
    let use_selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Would you like to use switch to this modpack now?")
        .default(0)
        .items(&["Yes", "No"])
        .interact_opt()
        .unwrap();

    println!();

    if let Some(selection) = use_selection {
        if selection == 0 {
            edit_modpack::load(config, pack_name);
        }
    }
}

/// Stashes the currently active mods by archiving them into a modpack.
///
/// # Returns
///
/// `true` if the stash was successful, and `false` otherwise.
pub fn stash(config: &Config, silent: bool) -> bool {
    // Get minecraft path
    let minecraft_path =
        PathBuf::from_str(config.dot_minecraft.as_str()).expect("Minecraft path is invalid");

    if !has_mods(config.clone()) {
        if !silent {
            println!("You do not have any mods installed");

            thread::sleep(Duration::from_millis(2500));
        }
        return false;
    }

    // Ask the user if they want to replace an existing modpack
    let replace_selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Would you like to update an existing modpack with the currently active mods?")
        .default(0)
        .items(&["Yes", "No"])
        .interact_opt()
        .unwrap();

    println!();

    if replace_selection.is_none() {
        if !silent {
            println!("Returning to main menu");

            thread::sleep(Duration::from_millis(2500));
        }
        return false;
    }

    let replace_selection = replace_selection.unwrap();

    // Ask the user for the modpack's name
    let modpack_name: String = match replace_selection {
        0 => {
            // Ask the user to select an existing modpack
            let modpack_names: Vec<String> = get_modpack_names(config.clone());

            let selection = FuzzySelect::with_theme(&ColorfulTheme::default())
                .with_prompt("What is the name of the modpack you would like to update?")
                .default(0)
                .max_length(25)
                .items(&modpack_names)
                .interact_opt()
                .unwrap();

            if let Some(selection) = selection {
                modpack_names[selection].clone()
            } else {
                panic!();
            }
        }
        1 => {
            // Ask the user for a new modpack name
            Input::with_theme(&ColorfulTheme::default())
                .with_prompt("What name would you like to give to the currently active modpack?")
                .interact_text()
                .unwrap()
        }
        _ => panic!(),
    };

    println!();

    // Create temp directory
    let temp_path = temp_dir().join("breeze_mods");
    create_dir_all(temp_path.clone()).expect("Failed to create temp directory");

    // Move all active mods to the temp directory
    for entry in fs::read_dir(minecraft_path.join("mods"))
        .expect("Cannot read mods directory")
        .flatten()
    {
        let entry_path = entry.path();
        if entry_path.is_file() && entry_path.extension().unwrap() == "jar" {
            let _ = fs::rename(
                // TODO: handle error (program using jar)
                entry_path.clone(),
                temp_path.join(entry_path.file_name().unwrap()),
            );
        }
    }

    // Create zipped modpack path
    let modpack_path = minecraft_path.join("modpacks").join(modpack_name + ".zip");

    // Archive mods
    let _ = zip_create_from_directory(&modpack_path, &temp_path);
    let _ = fs::remove_dir_all(temp_path);

    if !silent {
        println!("Your mods have successfully been stashed away!");

        thread::sleep(Duration::from_millis(2500));
    }

    true
}

fn open_modpacks_folder_in_explorer(config: &Config) {
    // Get minecraft path
    let minecraft_path =
        PathBuf::from_str(config.dot_minecraft.as_str()).expect("Minecraft path is invalid");

    // Open the modpacks directory
    let _ = open::that(minecraft_path.join("modpacks"));

    println!("Opening modpacks directory!");

    thread::sleep(Duration::from_millis(2500));
}
