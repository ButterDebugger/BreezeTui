use crate::{
    utils::{
        minecraft_versions::{get_release_version_names, get_version_names},
        paths::get_modpack_path,
    },
    App,
};
use breeze_pack::{
    format::{Branch, ModLoader, Modpack},
    packer::Packer,
};
use dialoguer::{theme::ColorfulTheme, FuzzySelect, Input};
use regex::Regex;
use std::{thread, time::Duration};

impl App {
    pub async fn add_branch_cli(&mut self, modpack_name: String) {
        let modpack_path = get_modpack_path(modpack_name);

        let packer = Packer::new(modpack_path.into());

        packer.load().unwrap();

        let pack = packer.read_pack().unwrap();

        // Ask the user for the branch name
        let branch_name = Input::<String>::new()
            .with_prompt("What should this branch be called?")
            .with_post_completion_text("Branch name")
            .validate_with(|input: &String| -> Result<(), &str> {
                let re = Regex::new(r"^[a-zA-Z0-9 ._-]*$").unwrap();

                if !re.is_match(input) {
                    Err("Invalid characters")
                } else if pack.branches.contains(input) {
                    Err("A branch with that name already exists")
                } else {
                    Ok(())
                }
            })
            .interact_text()
            .unwrap();

        // Ask the user for the specific minecraft version
        let game_version = {
            let game_versions = get_release_version_names().await.unwrap();

            let game_version_selection = FuzzySelect::with_theme(&ColorfulTheme::default())
                .with_prompt("What Minecraft version is this branch for?")
                .default(1)
                .max_length(25)
                .item("Show All")
                .items(&game_versions)
                .interact()
                .unwrap();

            if game_version_selection == 0 {
                let game_versions = get_version_names().await.unwrap();

                let game_version_selection = FuzzySelect::with_theme(&ColorfulTheme::default())
                    .with_prompt("What Minecraft version is this branch for?")
                    .default(0)
                    .max_length(25)
                    .items(&game_versions)
                    .interact()
                    .unwrap();

                game_versions[game_version_selection].clone()
            } else {
                game_versions[game_version_selection - 1].clone()
            }
        };

        // Ask the user for the mod loader
        let mod_loader = {
            let mod_loaders = vec![
                ModLoader::Fabric,
                ModLoader::NeoForge,
                ModLoader::Quilt,
                ModLoader::Forge,
            ];

            let mod_loader_selection = FuzzySelect::with_theme(&ColorfulTheme::default())
                .with_prompt("What Mod Loader is this branch for?")
                .default(0)
                .max_length(25)
                .items(&mod_loaders)
                .interact()
                .unwrap();

            mod_loaders[mod_loader_selection].clone()
        };

        // Write the new branch
        println!();

        let mut new_branches = pack.branches;
        new_branches.push(branch_name.clone());

        if let Err(err) = packer.write_pack(&Modpack {
            name: pack.name,
            summary: pack.summary,
            author: pack.author,
            updater: pack.updater,
            branches: new_branches,
        }) {
            println!("Failed to write pack data: {}", err);

            thread::sleep(Duration::from_millis(2500));
            self.go_back();
            return;
        }

        if let Err(err) = packer.write_branch(
            &branch_name,
            &Branch {
                game_version,
                mod_loader,
                loader_version: None,
                mods: vec![],
            },
        ) {
            println!("Failed to write branch: {}", err);

            thread::sleep(Duration::from_millis(2500));
            self.go_back();
            return;
        }

        if let Err(err) = packer.save() {
            println!("Failed to save branch: {}", err);

            thread::sleep(Duration::from_millis(2500));
            self.go_back();
            return;
        }

        println!("Branch has been created!");

        thread::sleep(Duration::from_millis(2500));
        self.go_back();
    }
}
