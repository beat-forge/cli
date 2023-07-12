use std::{str::FromStr, path::PathBuf};

use forge_lib::structs::manifest::{ForgeManifest, ModCategory, Include};
use semver::VersionReq;

pub fn init(name: String, category: String, game_version: String, artifact: PathBuf) {
    let manifest = ForgeManifest {
        _id: slug::slugify(&name),
        name,
        category: ModCategory::from_str(&category).unwrap(),
        artifact: artifact.to_str().unwrap().to_string(),
        description: "".to_string(),
        website: "".to_string(),
        includes: vec![],
        pre_exec: "".to_string(),
        post_exec: "".to_string(),
        depends: vec![],
        conflicts: vec![],
        version: semver::Version::new(0, 0, 0),
        manifest_version: 1,
        game_version: VersionReq::parse(&game_version).unwrap(),
    };

    println!("Saving manifest to forge_manifest.json");

    let manifest_json = serde_json::to_string_pretty(&manifest).unwrap();
    std::fs::write("./forge_manifest.json", manifest_json).unwrap();
}