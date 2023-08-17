use crate::structs::Instance;
use directories::{BaseDirs, UserDirs};
use regex::Regex;
use std::io::Read;

pub mod progress {
    use indicatif::{ProgressBar, ProgressStyle};
    use std::time::Duration;

    pub fn make_progress() -> ProgressBar {
        let pb = ProgressBar::new_spinner();
        pb.enable_steady_tick(Duration::from_millis(120));
        pb.set_style(
            ProgressStyle::with_template("{spinner:.green} {msg}")
                .unwrap()
                .tick_chars("⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏"),
        );
        pb
    }

    pub fn message<T: Into<String>>(pb: &ProgressBar, message: T) {
        pb.set_message(message.into());
    }

    pub fn finish_progress<T: Into<String>>(pb: &ProgressBar, message: T) {
        pb.set_style(
            ProgressStyle::with_template("{spinner:.green} {msg}")
                .unwrap()
                .tick_chars("✓✓"),
        );
        pb.finish_with_message(message.into());
    }

    pub fn fail_progress<T: Into<String>>(pb: &ProgressBar, message: T) {
        pb.set_style(
            ProgressStyle::with_template("{spinner:.red} {msg}")
                .unwrap()
                .tick_chars("✗✗"),
        );
        pb.finish_with_message(message.into());
    }
}

static STEAM_PATH: &str = r"C:\Program Files (x86)\Steam\steamapps\common\Beat Saber";
static OCULUS_PATH: &str =
    r"C:\Program Files\Oculus\Software\Software\hyperbolic-magnetism-beat-saber";

pub fn get_instance_paths() -> Vec<Instance> {
    // detect instances from steam and oculus
    let steam_path_exists = std::path::Path::new(STEAM_PATH).exists();
    let oculus_path_exists = std::path::Path::new(OCULUS_PATH).exists();

    let mut instances: Vec<Instance> = Vec::new();

    if steam_path_exists {
        let steam_instance = Instance {
            name: "Steam".to_string(),
            path: STEAM_PATH.into(),
            game_version: get_game_version(STEAM_PATH.to_string()),
        };

        instances.push(steam_instance);
    }

    if oculus_path_exists {
        let oculus_instance = Instance {
            name: "Oculus".to_string(),
            path: OCULUS_PATH.into(),
            game_version: get_game_version(OCULUS_PATH.to_string()),
        };

        instances.push(oculus_instance);
    }

    let user_dirs = UserDirs::new().unwrap();
    let documents_dir = user_dirs.document_dir().unwrap();

    let base_dirs = BaseDirs::new().unwrap();
    let data_dir = base_dirs.data_dir();

    let bsm_config_path = data_dir.join("bs-manager").join("config.json");
    if bsm_config_path.exists() {
        let bsm_config_file = std::fs::File::open(bsm_config_path).unwrap();
        let bsm_config: serde_json::Value = serde_json::from_reader(bsm_config_file).unwrap();

        if bsm_config["installation-folder"].as_str().is_none() {
            let bs_manager = documents_dir.join("BSManager").join("BSInstances");

            for entry in std::fs::read_dir(bs_manager).unwrap() {
                let entry = entry.unwrap();
                let path = entry.path();

                if path.is_dir() {
                    instances.push(Instance {
                        name: path.file_name().unwrap().to_str().unwrap().to_string(),
                        game_version: get_game_version(path.to_str().unwrap().to_string()),
                        path,
                    });
                }
            }
        } else {
            let base_path = bsm_config["installation-folder"].as_str().unwrap();
            let mut bs_manager = std::path::Path::new(base_path).join("BSManager");
            bs_manager.push("BSInstances");

            for entry in std::fs::read_dir(base_path).unwrap() {
                let entry = entry.unwrap();
                let path = entry.path();

                if path.is_dir() {
                    instances.push(Instance {
                        name: path.file_name().unwrap().to_str().unwrap().to_string(),
                        game_version: get_game_version(path.to_str().unwrap().to_string()),
                        path,
                    });
                }
            }
        }
    }

    instances
}

pub fn get_game_version(path: String) -> String {
    let mut file =
        std::fs::File::open(format!("{}\\Beat Saber_Data\\globalgamemanagers", path)).unwrap();
    let mut bytes = Vec::new();
    file.read_to_end(&mut bytes).unwrap();

    let out = String::from_utf8_lossy(&*bytes);
    let pos = out.find("public.app-category.games").unwrap();
    let regex = Regex::new(r"[\d]+.[\d]+.[\d]+(p1)?").unwrap();

    let ver = &out[pos..];
    let ver = regex.find(ver).unwrap().as_str();
    ver.to_string()
}
