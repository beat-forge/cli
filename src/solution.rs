// A struct representing a C# solution with build helpers

use std::{path::PathBuf, process::Command};
use anyhow::{Result, Error};

pub struct Solution {
    pub name: String,
    pub path: PathBuf,
}

impl Solution {
    pub fn new<T: Into<String>, U: Into<PathBuf>>(name: T, path: U) -> Solution { 
        Solution { name: name.into(), path: path.into() }
    }

    pub fn restore(&self) -> Result<()> {
        Command::new("dotnet")
            .arg("restore")
            .arg(&self.path)
            .status()
            .expect("Failed to restore solution");
        Ok(())
    }

    pub fn build_debug(&self) -> Result<()> {
        Command::new("dotnet")
            .arg("build")
            .arg(&self.path)
            .arg("-c")
            .arg("Debug")
            .status()
            .expect("Failed to build solution");

        Ok(())
    }

    pub fn build_release(&self) -> Result<()> {
        Command::new("dotnet")
            .arg("build")
            .arg(&self.path)
            .arg("-c")
            .arg("Release")
            .status()
            .expect("Failed to build solution");
        Ok(())
    }
}
