use std::path::PathBuf;

use clap::{Parser, Subcommand, ArgAction};

mod pack;
mod publish;
mod init;
mod login;

#[derive(Parser, Debug)]
#[command(author, about, version)]
struct Args {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    Publish {
        #[arg(short, long, default_value = "./forge_manifest.json")]
        manifest: PathBuf,
        #[arg(long, action)]
        dry_run: bool,
        #[arg(long, action)]
        no_save: bool,
    },

    Init {
        #[arg(short, long)]
        name: String,
        #[arg(short, long)]
        category: String,
        #[arg(short, long)]
        artifact: PathBuf,
        #[arg(short, long, default_value = "1.13.2")]
        game_version: String,
    },

    Login {
        #[arg(short, long, allow_hyphen_values = true)]
        api_key: String,
    }
}

fn main() {
    let cli = Args::parse();
    match cli.command {
        Command::Init {
            name,
            category,
            artifact,
            game_version
        } => {
            init::init(name, category, game_version, artifact);
        },
        Command::Publish {
            manifest,
            dry_run,
            no_save,
        } => {
            publish::publish(manifest, dry_run, no_save).unwrap();
        },
        Command::Login {
            api_key,
        } => {
            login::login(api_key);
        }
    }
}

// #[cfg(test)]
// mod test {
//     use super::*;

//     #[test]
//     pub fn test_test_pack() {
//         let in_path = PathBuf::from("./project/beatmanifest.json");

//         let data = pack::pack(in_path).unwrap();
//         std::fs::write("./project/out/WhyIsThereNoLeaderboard.beatmod", data).unwrap();
//     }

//     #[test]
//     pub fn test_test_unpack() {
//         let in_path = PathBuf::from("./project/out/WhyIsThereNoLeaderboard.beatmod");

//         let data = pack::unpack(in_path).unwrap();
//         dbg!(data.manifest);
//     }
// }
