#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use dialoguer::{theme::ColorfulTheme, Select};
use eframe::egui;
mod config;
mod edit_mod;
mod edit_modpack;
mod installations;
mod modpacks;
mod mods;
mod utils;

#[tokio::main]
async fn main() {
    let _ = gui();

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

fn gui() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
        ..Default::default()
    };
    eframe::run_native(
        "My egui App",
        options,
        Box::new(|_| Ok(Box::<MyApp>::default())),
    )
}

struct MyApp {
    name: String,
    age: u32,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            name: "Arthur".to_owned(),
            age: 42,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("My egui Application");
            ui.horizontal(|ui| {
                let name_label = ui.label("Your name: ");
                ui.text_edit_singleline(&mut self.name)
                    .labelled_by(name_label.id);
            });
            ui.add(egui::Slider::new(&mut self.age, 0..=120).text("age"));
            if ui.button("Increment").clicked() {
                self.age += 1;
            }
            ui.label(format!("Hello '{}', age {}", self.name, self.age));

            // ui.image(egui::include_image!(
            //     "../../../crates/egui/assets/ferris.png"
            // ));
        });
    }
}
