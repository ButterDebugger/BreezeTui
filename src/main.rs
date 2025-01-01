use dialoguer::{theme::ColorfulTheme, Select};
mod config;
mod edit_mod;
mod edit_modpack;
mod modpacks;
mod mods;
mod utils;

fn main() {
    let config = config::init();

    let selections = &["Modpacks", "Mods"]; // TODO: add a "Configs" and "Installations" option

    loop {
        println!();

        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("What would you like to manage")
            .default(0)
            .items(&selections[..])
            .interact_opt()
            .unwrap();

        if let Some(selection) = selection {
            match selection {
                0 => modpacks::gui(config.clone()),
                1 => mods::gui(config.clone()),
                _ => panic!(),
            }
        } else {
            break;
        }
    }
}
