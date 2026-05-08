use crate::{utils::get_mod_names, App, Page};
use dialoguer::{theme::ColorfulTheme, FuzzySelect};
use std::{thread, time::Duration};

impl App {
    pub fn mods_list_cli(&mut self) {
        // Get the list of installed mods
        let installed_mods = get_mod_names(self.config.clone());

        if installed_mods.is_empty() {
            println!("No mods found");

            thread::sleep(Duration::from_millis(2500));

            self.go_back();
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
            self.goto(Page::EditMod(installed_mods[selection].clone()));
        } else {
            self.go_back();
        }
    }
}
