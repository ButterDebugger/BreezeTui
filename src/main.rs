use dioxus::{
    desktop::{muda::Menu, tao::window::Icon},
    prelude::*,
};
use std::path::Path;
mod components;
mod config;
mod edit_mod;
mod edit_modpack;
mod installations;
mod modpacks;
mod mods;
mod utils;
mod views;

// #[tokio::main]
// async fn main() {
fn main() {
    // Load the icon
    let icon = load_icon(Path::new("./assets/icon.png"));

    static CSS: Asset = asset!("/assets/main.css");

    // Configure the window
    let config = dioxus::desktop::Config::new()
        .with_window(
            dioxus::desktop::WindowBuilder::new()
                .with_title("Breeze")
                .with_always_on_top(false)
                .with_transparent(true),
        )
        .with_menu(Menu::new())
        // .with_disable_context_menu(true)
        .with_icon(icon);

    // Launch the app
    LaunchBuilder::desktop().with_cfg(config).launch(views::app::App);

    // let config = config::init();

    // let selections = &["Modpacks", "Mods", "Installations"]; // TODO: add a "Configs" and "Installations" option

    // loop {
    //     println!();

    //     let selection = Select::with_theme(&ColorfulTheme::default())
    //         .with_prompt("What would you like to manage")
    //         .default(0)
    //         .items(&selections[..])
    //         .interact_opt()
    //         .unwrap();

    //     if let Some(selection) = selection {
    //         match selection {
    //             0 => modpacks::cli(config.clone()),
    //             1 => mods::cli(config.clone()),
    //             2 => installations::cli().await,
    //             _ => panic!(),
    //         }
    //     } else {
    //         break;
    //     }
    // }
}

fn load_icon(path: &Path) -> Icon {
    let (icon_rgba, icon_width, icon_height) = {
        let image = image::open(path)
            .expect("Failed to open icon path")
            .into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        (rgba, width, height)
    };
    Icon::from_rgba(icon_rgba, icon_width, icon_height).expect("Failed to open icon")
}
