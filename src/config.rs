use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::{
    fs::{create_dir_all, File},
    path::Path,
};

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub api_key: Option<String>,
}

impl Config {
    pub fn new() -> Config {
        let c = Config { api_key: None };
        c.save();
        c
    }

    pub fn save(&self) {
        let project_dirs = ProjectDirs::from("net", "beatforge", "cli").unwrap();
        let config_dir = project_dirs.config_dir();
        create_dir_all(config_dir).unwrap();

        let mut file = File::create(config_dir.join("config.json")).unwrap();
        serde_json::to_writer_pretty(&mut file, &self).unwrap();
    }

    pub fn load() -> Self {
        let project_dirs = ProjectDirs::from("net", "beatforge", "cli").unwrap();
        let config_dir = project_dirs.config_dir();

        if !Path::new(&config_dir.join("config.json")).exists() {
            return Config::new();
        }

        let file = File::open(config_dir.join("config.json")).unwrap();
        serde_json::from_reader(file).unwrap()
    }

    pub fn set_api_key(&mut self, api_key: String) {
        self.api_key = Some(api_key);
        self.save();
    }
}
