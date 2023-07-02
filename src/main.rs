use std::path::PathBuf;

use clap::{Parser, Subcommand};

pub mod pack;

#[derive(Parser, Debug)]
#[command(author, about, version)]
struct Args {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    Pack {
        #[arg(short, long)]
        manifest: PathBuf,
        #[arg(short, long)]
        out: PathBuf,
    },
    Unpack {
        #[arg(short, long)]
        beatmod: PathBuf,
        #[arg(short, long)]
        out: PathBuf,
    },
}

fn main() {
    let cli = Args::parse();
    match cli.command {
        Command::Pack { manifest, out } => {
            let bin = pack::pack(manifest).unwrap();
            std::fs::write(out, bin).unwrap();
        }
        Command::Unpack { beatmod, out: _ } => {
            let beatmod = pack::unpack(beatmod).unwrap();
            dbg!(beatmod.manifest);
            //std::fs::write(out, manifest).unwrap();
        }
    }
}
