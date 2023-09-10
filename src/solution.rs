use anyhow::Result;
use directories::ProjectDirs;
use slug::slugify;
use std::{path::PathBuf, process::Command};

pub struct Solution {
    pub name: String,
    pub path: PathBuf,
}

impl Solution {
    pub fn new<T: Into<String>, U: Into<PathBuf>>(name: T, path: U) -> Solution {
        Solution {
            name: name.into(),
            path: path.into(),
        }
    }

    pub fn restore(&self) -> Result<()> {
        let mut command = Command::new("dotnet");
        command.arg("restore");
        command.current_dir(&self.path);
        command.spawn()?;

        Ok(())
    }

    pub fn build_debug(&self) -> Result<()> {
        let mut command = Command::new("dotnet");
        command.args(["build", "--configuration", "Debug"]);
        command.current_dir(&self.path);
        command.spawn()?;

        Ok(())
    }

    pub fn build_release(&self) -> Result<()> {
        let mut command = Command::new("dotnet");
        command.args([
            "build",
            "--configuration",
            "Release",
            "--disable-build-servers",
            "--no-incremental",
            "--property:Deterministic=true",
        ]);
        command.current_dir(&self.path);
        command.spawn()?;

        Ok(())
    }

    pub fn build_publish(&self) -> Result<()> {
        let dirs = ProjectDirs::from("net", "beatforge", "cli").unwrap();
        let data_dir = dirs.cache_dir();

        let mut output_path = data_dir.join("publish");
        output_path.push(slugify(&self.name));

        std::fs::create_dir_all(&output_path)?;

        let mut command = Command::new("dotnet");
        command.args([
            "build",
            "--configuration",
            "Release",
            "--disable-build-servers",
            "--no-incremental",
            "--property:Deterministic=true",
            &format!("--output {}", output_path.to_str().unwrap()),
        ]);
        command.current_dir(&self.path);
        command.spawn()?;

        Ok(())
    }
}
