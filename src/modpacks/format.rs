use console::{style, StyledObject};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, strum_macros::Display)]
pub enum ModLoader {
    #[strum(to_string = "Fabric")]
    Fabric,
    #[strum(to_string = "Quilt")]
    Quilt,
    #[strum(to_string = "NeoForge")]
    NeoForge,
    #[strum(to_string = "Forge")]
    Forge,
}

impl ModLoader {
    pub fn to_styled_string(self) -> StyledObject<String> {
        match self {
            ModLoader::Fabric => style(self.to_string()).true_color(219, 182, 155),
            ModLoader::Quilt => style(self.to_string()).true_color(199, 150, 249),
            ModLoader::NeoForge => style(self.to_string()).true_color(249, 158, 107),
            ModLoader::Forge => style(self.to_string()).true_color(149, 158, 239),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "source")]
pub enum Mod {
    Modrinth {
        name: String,
        // Modrinth specific fields
        project_id: String,
        version: String,
    },
    CurseForge {
        name: String,
        // CurseForge specific fields
        project_id: i32,
        file_id: i32,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Branch {
    pub game_version: String,
    pub mod_loader: ModLoader,
    pub loader_version: Option<String>,
    pub mods: Vec<Mod>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Modpack {
    pub name: String,
    pub summary: Option<String>,
    pub author: Option<String>,
    pub updater: Option<Updater>,
    pub branches: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "source")]
pub enum Updater {
    Github {
        
    },
}
