use dialoguer::{theme::ColorfulTheme, Select};
use gpui::{prelude::*, px, size, App, Application, Bounds, WindowBounds, WindowOptions};

mod components;
mod config;
mod edit_mod;
mod edit_modpack;
mod installations;
mod modpacks;
mod mods;
mod utils;
mod views;

#[tokio::main]
async fn main() {
    Application::new().run(|cx: &mut App| {
        let bounds = Bounds::centered(None, size(px(500.), px(500.0)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                ..Default::default()
            },
            |_, cx| {
                cx.new(|_| views::home::Home {})
            },
        )
        .unwrap();
    });

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
