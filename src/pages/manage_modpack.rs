use crate::{utils::paths::get_modpack_path, App, Page};
use breeze_pack::packer::Packer;
use console::style;
use dialoguer::{theme::ColorfulTheme, Select};
use std::{
    fs::{self},
    thread,
    time::Duration,
};

#[derive(strum_macros::Display)]
enum ManageOptions {
    #[strum(to_string = "Manage Branch")]
    ManageBranch,
    #[strum(to_string = "Add Branch")]
    AddBranch,
    #[strum(to_string = "Edit Details")]
    EditDetails,
    #[strum(to_string = "Delete")]
    Delete,
}

impl App {
    pub async fn edit_modpack_cli(&mut self, modpack_name: String) {
        // Print modpack details
        let modpack_path = get_modpack_path(modpack_name.clone());

        let packer = Packer::new(modpack_path.into());

        let _ = packer.load();

        let pack = packer.read_pack().unwrap();

        println!("{} {}", style("@").green(), style(pack.name).bold());

        if let Some(author) = pack.author {
            println!("  By {}", style(author).italic());
        }

        if let Some(summary) = pack.summary {
            println!("  {}", style(summary).dim());
        }

        let selections = if pack.branches.is_empty() {
            vec![
                ManageOptions::AddBranch,
                ManageOptions::EditDetails,
                ManageOptions::Delete,
            ]
        } else {
            println!("  Branches:");

            vec![
                ManageOptions::ManageBranch,
                ManageOptions::AddBranch,
                ManageOptions::EditDetails,
                ManageOptions::Delete,
            ]
        };

        for branch_name in pack.branches {
            let branch = packer.read_branch(&branch_name).unwrap();

            println!(
                "  {} {}",
                style("~").red().bright(),
                style(branch_name).bold()
            );

            println!(
                "    Minecraft Version: {}",
                style(branch.game_version).green()
            );

            println!("    Mod Loader: {}", branch.mod_loader.to_styled_string());

            if let Some(loader_version) = branch.loader_version {
                println!(
                    "    Loader Version: {}",
                    style(loader_version).magenta().bright()
                );
            }

            println!("    Total Mods: {}", style(branch.mods.len()).blue());
        }

        println!();

        // Ask the user what they would like to do
        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("What would you like to do?")
            .default(0)
            .items(&selections)
            .interact_opt()
            .unwrap();

        if let Some(selection) = selection {
            println!();

            match selections[selection] {
                ManageOptions::ManageBranch => {
                    self.goto(Page::BranchList(modpack_name));
                }
                ManageOptions::AddBranch => {
                    self.goto(Page::AddBranch(modpack_name));
                }
                ManageOptions::EditDetails => {
                    // ...

                    self.return_home();
                }
                ManageOptions::Delete => {
                    delete_modpack(modpack_name);

                    self.return_home();
                }
            }
        } else {
            self.go_back();
        }
    }
}

fn delete_modpack(modpack_name: String) {
    let modpack_path = get_modpack_path(modpack_name.clone());

    // Delete the modpack
    let _ = fs::remove_file(modpack_path);

    println!("{} has successfully been deleted!", modpack_name);

    thread::sleep(Duration::from_millis(2500));
}
