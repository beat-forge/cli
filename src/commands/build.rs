use std::path::PathBuf;

use crate::{api::Client, config::Config, solution::Solution, utils::manifest_search};
use anyhow::Result;
use forge_lib::structs::v1::ForgeManifestTypes;
use inquire::MultiSelect;

pub fn build(_client: Client, _config: &mut Config) -> Result<()> {
    println!("Building project...");
    let cwd = std::env::current_dir()?;
    let manifests = manifest_search(&cwd)?;

    for manifest in manifests {
        match manifest {
            ForgeManifestTypes::Lib(lib) => {

            }
            ForgeManifestTypes::Mod(forge_mod) => {
                
            }
            ForgeManifestTypes::Module(module) => {
                todo!("Module manifest")
            }
            ForgeManifestTypes::Parent(parent) => {
                todo!("Parent manifest")
            }
        }
    }

    Ok(())
}