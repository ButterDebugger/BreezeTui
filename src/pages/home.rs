use crate::{App, Page};
use dialoguer::{theme::ColorfulTheme, Select};

impl App {
    pub fn home_cli(&mut self) {
        let selections = &["Modpacks", "Mods", "Installations"]; // TODO: add a "Configs" and "Installations" option

        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("What would you like to manage")
            .default(0)
            .items(&selections[..])
            .interact_opt()
            .unwrap();

        if let Some(selection) = selection {
            match selection {
                0 => self.goto(Page::Modpacks),
                1 => self.goto(Page::Mods),
                2 => self.goto(Page::Installations),
                _ => unreachable!(),
            }
        } else {
            self.exit()
        }
    }
}
