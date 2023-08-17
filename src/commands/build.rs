use std::path::PathBuf;

use crate::{api::Client, config::Config, solution::Solution};
use anyhow::Result;

pub fn build(client: Client, config: &mut Config) -> Result<()> {
    println!("Building project...");
    let cwd = std::env::current_dir()?;
    let manifest_paths = manifest_search(&cwd)?;

    for manifest_path in manifest_paths {
        let solution = Solution::new("custom", manifest_path.parent().unwrap());

        solution.build_release()?;
    }

    Ok(())
}

fn manifest_search(path: &std::path::PathBuf) -> Result<Vec<PathBuf>> {
    let mut manifests = Vec::new();

    for entry in std::fs::read_dir(path)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            let mut sub_manifests = manifest_search(&path)?;
            manifests.append(&mut sub_manifests);
        } else if path.ends_with("beatforge.manifest") {
            manifests.push(path);
        }
    }

    Ok(manifests)
}
