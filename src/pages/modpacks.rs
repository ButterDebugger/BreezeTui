use crate::{utils::paths::get_modpack_names, App, Page};
use console::style;
use dialoguer::{theme::ColorfulTheme, Select};

#[derive(strum_macros::Display)]
enum ModpacksOptions {
    #[strum(to_string = "Manage Existing")]
    Manage,
    #[strum(to_string = "Create New")]
    Create,
    #[strum(to_string = "Import from File")]
    Import,
    #[strum(to_string = "Reveal in File Explorer")]
    OpenExplorer,
}

impl App {
    pub fn modpacks_cli(&mut self) {
        let selections;
        let modpack_names = get_modpack_names();

        if modpack_names.is_empty() {
            selections = vec![
                ModpacksOptions::Create,
                ModpacksOptions::Import,
                ModpacksOptions::OpenExplorer,
            ];

            // Notify that there are no modpacks
            println!("{} {}", style("!").red(), style("No Modpacks Saved").bold());
        } else {
            selections = vec![
                ModpacksOptions::Manage,
                ModpacksOptions::Create,
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
                ModpacksOptions::Manage => {
                    self.goto(Page::ModpacksList);
                }
                ModpacksOptions::Create => {
                    self.goto(Page::CreateModpack);
                }
                ModpacksOptions::Import => {
                    // println!();

                    // import(&self.config);
                    // self.return_home();
                }
                ModpacksOptions::OpenExplorer => {
                    // println!();

                    // open_modpacks_folder_in_explorer(&self.config);
                    // self.return_home();
                }
            }
        } else {
            self.go_back();
        }
    }
}
