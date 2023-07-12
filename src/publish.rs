use std::{
    fmt::{Display, Formatter},
    path::PathBuf, time::Duration,
};

use forge_lib::structs::manifest::ForgeManifest;
use indicatif::{ProgressBar, ProgressStyle};

type Result<T> = std::result::Result<T, PublishError>;

#[derive(Debug)]
pub struct PublishError {
    inner: Option<Box<dyn std::error::Error>>,
    kind: PublishErrorKind,
}

impl PublishError {
    fn new(kind: PublishErrorKind) -> Self {
        Self { inner: None, kind }
    }
}

impl Display for PublishError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.kind)
    }
}

impl From<std::io::Error> for PublishError {
    fn from(err: std::io::Error) -> Self {
        Self {
            inner: Some(Box::new(err)),
            kind: PublishErrorKind::Io,
        }
    }
}

impl From<minreq::Error> for PublishError {
    fn from(err: minreq::Error) -> Self {
        Self {
            inner: Some(Box::new(err)),
            kind: PublishErrorKind::Network,
        }
    }
}

#[derive(Debug)]
enum PublishErrorKind {
    ManifestNotFound,
    Authentication,
    Io,
    Network,
}

impl Display for PublishErrorKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            PublishErrorKind::ManifestNotFound => write!(f, "Manifest not found."),
            PublishErrorKind::Io => write!(f, "IO error."),
            PublishErrorKind::Network => write!(f, "Network error."),
            PublishErrorKind::Authentication => write!(f, "Authentication error."),
        }
    }
}

#[must_use]
pub fn publish(manifest_path: PathBuf, dry_run: bool, save: bool) -> Result<()> {
    let pb = ProgressBar::new_spinner();
    pb.enable_steady_tick(Duration::from_millis(120));
    pb.set_style(ProgressStyle::with_template("{spinner:.green} {msg}").unwrap().tick_chars("⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏"));
    if !manifest_path.exists() {
        return Err(PublishError::new(PublishErrorKind::ManifestNotFound));
    }

    if std::env::var("BEATFORGE_API_KEY").is_err() {
        return Err(PublishError::new(PublishErrorKind::Authentication));
    }

    pb.set_message("Parsing manifest...");
    let manifest = ForgeManifest::try_from(manifest_path.clone())?;

    let name = manifest.name.replace(" ", "_");

    pb.set_message(format!("Packing {}...", name));
    let forge_mod = crate::pack::pack(manifest, manifest_path)?;

    if save {
        std::fs::write(format!("{}.forgemod", name), forge_mod.clone())?;
    }

    if dry_run {
        pb.finish_with_message(format!("Dry run complete. {} not uploaded.", name));
        return Ok(());
    }

    pb.set_message(format!("Uploading {}...", name));
    let res = minreq::post("https://api.beatforge.net/mods")
        .with_header("Content-Type", "application/octet-stream")
        .with_header(
            "Authorization",
            format!("Bearer {}", std::env::var("BEATFORGE_API_KEY").unwrap()),
        )
        .with_body(forge_mod)
        .send()?;

    if res.status_code != 200 | 201 {
        pb.finish_with_message(format!("Failed to upload {}.", name));
        return Err(PublishError::new(PublishErrorKind::Network));
    }

    pb.finish_with_message(format!("Uploaded {}.", name));

    Ok(())
}
