use crate::modpacks::{
    self,
    format::{Branch, Modpack},
};
use anyhow::{anyhow, Ok, Result};
use std::{
    env::temp_dir,
    fs::{self, create_dir_all, remove_dir_all, File},
    path::Path,
};
use uuid::Uuid;

#[derive(Debug)]
pub struct Packer {
    archive_path: Box<Path>,
    staging_path: Box<Path>,
}

impl Packer {
    pub fn new(archive_path: Box<Path>) -> Self {
        Self {
            archive_path,
            staging_path: temp_dir()
                .join(format!("breeze_unpacked_{}", Uuid::new_v4()))
                .into(),
        }
    }

    /// Serialize `modpack` as `modpack.json` in the staging directory
    pub fn write_pack(&self, modpack: &Modpack) -> Result<()> {
        // Create the parent directories
        create_dir_all(&self.staging_path)?;

        // Create the pack data file
        let file = File::create(self.staging_path.join("modpack.json"))?;

        // Write the json to the file
        serde_json::to_writer_pretty(file, modpack)?;

        Ok(())
    }

    /// Deserialize `modpack.json` from the staging directory
    pub fn read_pack(&self) -> Result<Modpack> {
        let json = fs::read_to_string(self.staging_path.join("modpack.json"))?;

        Ok(serde_json::from_str::<Modpack>(&json)?)
    }

    /// Serialize `branch` as `<name>/branch.json` in the staging directory
    pub fn write_branch(&self, name: &str, branch: &Branch) -> Result<()> {
        let branch_dir = self.staging_path.join(name);

        create_dir_all(&branch_dir)?;

        let file = File::create(branch_dir.join("branch.json"))?;

        serde_json::to_writer_pretty(file, branch)?;

        Ok(())
    }

    /// Deserialize `<name>/branch.json` from the staging directory
    pub fn read_branch(&self, name: &str) -> Result<Branch> {
        let branch_path = self.staging_path.join(name).join("branch.json");
        let json = fs::read_to_string(branch_path)?;

        Ok(serde_json::from_str::<Branch>(&json)?)
    }

    /// Checks if the modpack contains all of its required resources
    pub fn is_valid(&self) -> bool {
        // Check and get the pack data
        let modpack = self.read_pack();

        if modpack.is_err() {
            return false;
        }

        let modpack = modpack.unwrap();

        // Make sure each branch is defined
        for branch_name in modpack.branches {
            // Check and get the branch data
            let branch = self.read_branch(&branch_name);

            if branch.is_err() {
                return false;
            }
        }

        true
    }

    /// Updates the modpack archive with the staged modpack
    pub fn save(&self) -> Result<()> {
        // Cancel if the modpack is not valid
        if !self.is_valid() {
            return Err(anyhow!("Modpack is not valid"));
        }

        // Compress the staging directory
        sevenz_rust::compress_to_path(&*self.staging_path, &*self.archive_path)?;

        Ok(())
    }

    /// Load the modpack archive into the staging path, replacing whats there
    pub fn load(&self) -> Result<()> {
        if self.staging_path.exists() {
            remove_dir_all(&*self.staging_path)?;
        }

        create_dir_all(&*self.staging_path)?;

        sevenz_rust::decompress_file(&*self.archive_path, &*self.staging_path)?;

        Ok(())
    }
}
