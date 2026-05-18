use crate::{App, Page};
use dialoguer::{theme::ColorfulTheme, Select};

#[derive(strum_macros::Display)]
enum HomeOptions {
    #[strum(to_string = "Modpacks")]
    Modpacks,
    #[strum(to_string = "Mods")]
    Mods,
    #[strum(to_string = "Installations")]
    Installations,
}

// TODO: add a "Config" option

impl App {
    pub fn home_cli(&mut self) {
        let selections = [
            HomeOptions::Modpacks,
            HomeOptions::Mods,
            HomeOptions::Installations,
        ];

        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("What would you like to manage")
            .default(0)
            .items(&selections)
            .interact_opt()
            .unwrap();

        if let Some(selection) = selection {
            match selections[selection] {
                HomeOptions::Modpacks => self.goto(Page::Modpacks),
                HomeOptions::Mods => self.goto(Page::Mods),
                HomeOptions::Installations => self.goto(Page::Installations),
            }
        } else {
            self.exit()
        }
    }
}
