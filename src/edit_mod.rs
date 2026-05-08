use crate::App;
use dialoguer::{theme::ColorfulTheme, Select};
use std::{
    fs::{self},
    path::PathBuf,
    str::FromStr,
    thread,
    time::Duration,
};

impl App {
    pub fn edit_mod_cli(&mut self, mod_name: String) {
        let selections = &["Delete"];

        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("What would you like to do to ".to_owned() + &mod_name)
            .default(0)
            .items(&selections[..])
            .interact_opt()
            .unwrap();

        if let Some(selection) = selection {
            println!();

            match selection {
                0 => delete_mod(self, mod_name),
                _ => unreachable!(),
            }

            self.return_home();
        } else {
            self.go_back();
        }
    }
}

fn delete_mod(app: &mut App, mod_name: String) {
    // Get minecraft path
    let minecraft_path =
        PathBuf::from_str(app.config.dot_minecraft.as_str()).expect("Minecraft path is invalid");

    let mod_file_name = mod_name.clone() + ".jar";

    // Delete the mod
    let mod_path = minecraft_path.join("mods").join(mod_file_name.clone());
    let _ = fs::remove_file(mod_path);

    println!("{} has successfully been deleted!", mod_name);

    thread::sleep(Duration::from_millis(2500));
}
