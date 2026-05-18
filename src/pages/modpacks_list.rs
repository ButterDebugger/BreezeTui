use crate::{utils::paths::get_modpack_names, App, Page};
use dialoguer::{theme::ColorfulTheme, FuzzySelect};
use std::{thread, time::Duration};

impl App {
    pub fn modpacks_list_cli(&mut self) {
        // Get the list of modpacks
        let modpack_names: Vec<String> = get_modpack_names();

        if modpack_names.is_empty() {
            println!("No modpacks found");

            thread::sleep(Duration::from_millis(2500));

            self.go_back();
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
            self.goto(Page::ManageModpack(modpack_names[selection].clone()));
        } else {
            self.go_back();
        }
    }
}
