mod api;
mod commands;
mod config;
mod helpers;
mod solution;
mod structs;
mod utils;

use anyhow::{Error, Result};
use api::Client;
use clap::{Parser, Subcommand};
use commands::{build::build, login::login, new::new, publish::publish};

#[derive(Parser, Debug)]
#[clap(version = env!("CARGO_PKG_VERSION"), author = "BeatForge")]
struct Cli {
    #[arg(
        long,
        default_value = "https://api.beatforge.net",
        allow_hyphen_values = true
    )]
    api_url: String,

    #[arg(long, allow_hyphen_values = true)]
    api_key: Option<String>,

    #[command(subcommand)]
    subcmd: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Login,
    New,
    Init,
    Build,
    Publish,
    Install,
    Add,
}

fn main() -> Result<(), Error> {
    let cli = Cli::parse();
    let mut config = config::Config::load();

    let mut client = Client::new(cli.api_url, config.api_key.clone().or(cli.api_key));
    match cli.subcmd {
        Commands::Login => {
            login(&mut client, &mut config)?;
        }
        Commands::New => {
            new(client, &mut config)?;
        }
        Commands::Init => {
            todo!("Init command")
        }
        Commands::Build => {
            build(client, &mut config)?;
        }
        Commands::Publish => {
            publish(&mut client, &mut config)?;
        }
        Commands::Install => {
            todo!("Install command")
        }
        Commands::Add => {
            todo!("Add command")
        }
    }

    Ok(())
}
