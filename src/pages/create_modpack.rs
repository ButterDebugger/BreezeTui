use std::{thread, time::Duration};

use crate::{
    modpacks::{format::Modpack, packer::Packer},
    utils::paths::{get_modpack_names, get_modpack_path},
    App,
};
use dialoguer::Input;
use regex::Regex;

impl App {
    pub fn create_modpack_cli(&mut self) {
        // Ask the user for the modpack name
        let modpack_name = Input::<String>::new()
            .with_prompt("What should this modpack be called?")
            .with_post_completion_text("Modpack name")
            .validate_with(|input: &String| -> Result<(), &str> {
                let re = Regex::new(r"^[a-zA-Z0-9 ._-]*$").unwrap();

                if !re.is_match(input) {
                    Err("Invalid characters")
                } else if get_modpack_names().contains(input) {
                    Err("A modpack with that name already exists")
                } else {
                    Ok(())
                }
            })
            .interact_text()
            .unwrap();

        println!();

        // Create the modpack
        let modpack_path = get_modpack_path(modpack_name.clone());

        let packer = Packer::new(modpack_path.into());

        let _ = packer.write_pack(&Modpack {
            name: modpack_name,
            summary: None,
            author: None,
            updater: None,
            branches: vec![],
        });

        if packer.save().is_ok() {
            println!("Modpack has been created")
        } else {
            println!("Failed to create the modpack")
        }

        thread::sleep(Duration::from_millis(2500));

        self.go_back();
    }
}
