use crate::{
    format::{Branch, Mod, ModLoader, Modpack},
    packer::Packer,
};
use std::path::Path;

pub mod format;
pub mod packer;

pub fn main() {
    let packer = Packer::new(Path::new("./hi.modx").into());

    println!("{:#?}", packer);

    // if let Err(err) = packer.load() {
    //     eprintln!("Error while loading: {}", err)
    // }

    // if packer.is_valid() {
    //     let pack = packer.read_pack().unwrap();

    //     println!("pack data = {:#?}", pack);

    //     for branch_name in pack.branches {
    //         let branch = packer.read_branch(&branch_name).unwrap();

    //         println!("branch '{}' = {:#?}", branch_name, branch);
    //     }
    // }

    if let Err(err) = packer.write_pack(&Modpack {
        name: "love".to_owned(),
        summary: Some("lots of love and i mean loooooots and loooots of love".to_owned()),
        author: Some("me".to_owned()),
        updater: None,
        branches: vec!["1.21.11".to_owned()],
    }) {
        eprintln!("Error while writing pack data: {}", err);
        return;
    }

    if let Err(err) = packer.write_branch(
        "1.21.11",
        &Branch {
            game_version: "1.21.11".to_owned(),
            mod_loader: ModLoader::Fabric,
            loader_version: Some("i dont know".to_owned()),
            mods: vec![
                Mod::Modrinth {
                    name: "Sodium".to_owned(),
                    project_id: "AANobbMI".to_owned(),
                    version: "mc1.21.11-0.8.12-beta.2-fabric".to_owned(),
                },
                Mod::CurseForge {
                    name: "Sodium".to_owned(),
                    project_id: 394468,
                    file_id: 8071333,
                },
            ],
        },
    ) {
        eprintln!("Error while writing branch data: {}", err);
        return;
    }

    if let Err(err) = packer.save() {
        eprintln!("Error while saving: {}", err)
    }
}
