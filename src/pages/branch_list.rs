use crate::{modpacks::packer::Packer, utils::paths::get_modpack_path, App, Page};
use dialoguer::{theme::ColorfulTheme, FuzzySelect};
use std::{thread, time::Duration};

impl App {
    pub fn branch_list_cli(&mut self, modpack_name: String) {
        let modpack_path = get_modpack_path(modpack_name.clone());

        let packer = Packer::new(modpack_path.into());

        packer.load().unwrap();

        let pack = packer.read_pack().unwrap();

        // Cancel if there are no branches
        if pack.branches.is_empty() {
            println!("No branches found");

            thread::sleep(Duration::from_millis(2500));

            self.go_back();
            return;
        }

        // Ask the user which branch they want to manage
        let selection = FuzzySelect::with_theme(&ColorfulTheme::default())
            .with_prompt("Which branch would you like to manage?")
            .default(0)
            .max_length(25)
            .items(&pack.branches)
            .interact_opt()
            .unwrap();

        if let Some(selection) = selection {
            self.goto(Page::ManageBranch(
                modpack_name,
                pack.branches[selection].clone(),
            ));
        } else {
            self.go_back();
        }
    }
}
