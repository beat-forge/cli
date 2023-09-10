use core::panic;

use crate::{api::Client, config::Config, utils::manifest_search, solution::Solution};
use anyhow::Result;
use forge_lib::structs::v1::ForgeManifestTypes;
use inquire::Select;

pub fn publish(client: &mut Client, config: &mut Config) -> Result<()> {
    if client.api_key.is_none() {
        return Err(anyhow::anyhow!("You must be logged in to publish mods"));
    }

    let cwd = std::env::current_dir()?;
    let manifest_types = manifest_search(&cwd)?;

    let manifest = Select::new("Select a mod to publish", manifest_types).prompt()?;

    match manifest {
        ForgeManifestTypes::Mod(forge_mod) => {
            let solution = Solution::new(forge_mod.inner.name.clone(), cwd.clone());
            solution.build_publish()?;
        }
        ForgeManifestTypes::Module(module) => {
            panic!("You cannot publish a module alone");
        }
        ForgeManifestTypes::Parent(parent) => {
            todo!("Parent manifest")
        }
        _ => {
            return Err(anyhow::anyhow!(
                "You can only publish mods, modules, and parents"
            ))
        }
    }

    Ok(())
}
