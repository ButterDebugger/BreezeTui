use crate::config::Config;
use console::style;

mod config;
mod modpacks;
mod pages;
mod utils;

#[derive(Debug, Default, Clone, strum_macros::Display)]
enum Page {
    #[default]
    #[strum(to_string = "Home")]
    Home,
    #[strum(to_string = "Mods")]
    Mods,
    #[strum(to_string = "Mods List")]
    ModsList,
    #[strum(to_string = "Edit Mod")]
    EditMod(String),
    #[strum(to_string = "Modpacks")]
    Modpacks,
    #[strum(to_string = "Modpacks List")]
    ModpacksList,
    #[strum(to_string = "Manage Modpack")]
    ManageModpack(String),
    #[strum(to_string = "Add Branch")]
    AddBranch(String),
    #[strum(to_string = "Branch List")]
    BranchList(String),
    #[strum(to_string = "Manage Branch")]
    ManageBranch(String, String),
    #[strum(to_string = "Create Modpack")]
    CreateModpack,
    #[strum(to_string = "Installations")]
    Installations,
}

#[derive(Debug, Clone)]
struct App {
    config: Config,
    path: Vec<Page>,
}

impl App {
    fn new() -> Self {
        // Get the config
        let config = if let Some(loaded_config) = Config::load_from_disk() {
            loaded_config
        } else {
            Config::create_config_prompt()
        };

        // Initialize and save the config
        config.clone().init();
        config.clone().save();

        // Return the app
        Self {
            config,
            path: vec![Page::default()],
        }
    }

    /// Extends the path to go to the inputted page
    fn goto(&mut self, page: Page) {
        self.path.push(page);
    }

    /// Goes back to the previous page
    ///
    /// Should be used when the user backs out of the current page
    fn go_back(&mut self) {
        self.path.pop();
    }

    /// Returns to the home page
    ///
    /// Should be used after an action is completed
    fn return_home(&mut self) {
        self.path.clear();

        self.path.push(Page::Home);
    }

    /// Exits out of the app loop
    fn exit(&mut self) {
        self.path.clear();
    }

    /// Starts the app loop
    async fn run(&mut self) {
        loop {
            let _ = clearscreen::clear();

            println!();

            // Get current page
            let current_page = self.path.last();

            if current_page.is_none() {
                // If there is no path, break out of the app
                break;
            }

            let current_page = current_page.unwrap();

            // Output full path
            for i in 0..self.path.len() {
                let mut styled_page = style(&self.path[i]);

                if i != self.path.len() - 1 {
                    styled_page = styled_page.dim();
                }

                print!("{} {} ", style("▶").blue(), styled_page);
            }

            println!();
            println!();

            // Output the current menu
            match &current_page {
                Page::Home => self.home_cli(),
                Page::Mods => self.mods_cli(),
                Page::ModsList => self.mods_list_cli(),
                Page::EditMod(mod_name) => self.edit_mod_cli(mod_name.to_string()),
                Page::Modpacks => self.modpacks_cli(),
                Page::CreateModpack => self.create_modpack_cli(),
                Page::ModpacksList => self.modpacks_list_cli(),
                Page::AddBranch(modpack_name) => {
                    self.add_branch_cli(modpack_name.to_string()).await
                }
                Page::ManageModpack(modpack_name) => {
                    self.edit_modpack_cli(modpack_name.to_string()).await
                }
                Page::BranchList(modpack_name) => self.branch_list_cli(modpack_name.to_string()),
                Page::ManageBranch(modpack_name, branch_name) => {
                    self.manage_branch_cli(modpack_name.to_string(), branch_name.to_string())
                }
                Page::Installations => self.installations_cli().await,
            }
        }
    }
}

#[tokio::main]
async fn main() {
    let mut app = App::new();

    app.run().await;
}
