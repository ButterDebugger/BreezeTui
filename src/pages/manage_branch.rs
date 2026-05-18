use dialoguer::{theme::ColorfulTheme, Input, Select};

use crate::{modpacks::packer::Packer, utils::paths::get_modpack_path, App};

#[derive(strum_macros::Display)]
enum ManageOptions {
    #[strum(to_string = "Add Mod")]
    AddMod,
    #[strum(to_string = "Remove Mod")]
    RemoveMod,
}

impl App {
    pub fn manage_branch_cli(&mut self, modpack_name: String, branch_name: String) {
        let modpack_path = get_modpack_path(modpack_name.clone());

        let packer = Packer::new(modpack_path.into());

        packer.load().unwrap();

        let branch = packer.read_branch(&branch_name).unwrap();

        //
        let selections = if branch.mods.is_empty() {
            vec![ManageOptions::AddMod]
        } else {
            vec![ManageOptions::AddMod, ManageOptions::RemoveMod]
        };

        // Ask the user what they want to do
        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("What would you like to do?")
            .default(0)
            .items(&selections)
            .interact_opt()
            .unwrap();

        if let Some(selection) = selection {
            match selections[selection] {
                ManageOptions::AddMod => {
                    add_mod(&packer);

                    self.go_back();
                }
                ManageOptions::RemoveMod => {
                    remove_mod(&packer);

                    self.go_back();
                }
            }
        } else {
            self.go_back();
        }
    }
}

pub fn add_mod(packer: &Packer) {
    // Ask the user for the mod's project ID
    let project_id: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("What is the mod's project ID?")
        .interact_text()
        .unwrap();
}

pub fn remove_mod(packer: &Packer) {}
