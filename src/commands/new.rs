use crate::{
    api::Client,
    config::Config,
    helpers::forge_generator::{self, ForgeGenerator},
    utils::{progress::{make_progress, finish_progress}, get_instance_paths, get_game_version},
    structs::Instance,
};
use convert_case::{Case, Casing};
use inquire::{Confirm, MultiSelect, Select, Text, validator::{self, MinLengthValidator, Validation}, CustomUserError};
use slug::slugify;
use std::path::PathBuf;

#[derive(Clone)]
pub struct FileExistsValidatior;

impl validator::StringValidator for FileExistsValidatior {
    fn validate(&self, input: &str) -> Result<Validation, CustomUserError> {
        if std::path::Path::new(input).exists() {
            Ok(Validation::Valid)
        } else {
            Ok(Validation::Invalid("Path does not exist.".into()))
        }
    }
}

pub fn new(client: Client, _config: &mut Config) -> anyhow::Result<()> {
    let mod_name = Text::new("What is the name of the mod?").prompt()?;

    // slugs
    let mut mod_slug = slugify(&mod_name);
    let mut mod_name_pascal = mod_slug.to_case(Case::Pascal);

    let mut pascal_chosen = Confirm::new(&format!(
        "Is {} a good name for the solution?",
        mod_name_pascal
    ))
    .prompt()?;

    while !pascal_chosen {
        mod_name_pascal = Text::new("What would you like the name of the solution to be?")
            .with_help_message("This must be in PascalCase. e.g. beat forge -> BeatForge")
            .prompt()?;

        pascal_chosen = Confirm::new(&format!(
            "Is {} a good name for the solution?",
            mod_name_pascal
        ))
        .prompt()?;
    }

    let mut slug_chosen = Confirm::new(&format!(
        "Is {} a good identifier for your mod? (THIS CANNOT BE CHANGED LATER)",
        mod_slug
    ))
    .prompt()?;

    while !slug_chosen {
        mod_slug = slugify(Text::new("What would you like your mod's identifier to be?").prompt()?);
        slug_chosen = Confirm::new(&format!(
            "Is {} a good identifier for your mod? (THIS CANNOT BE CHANGED LATER)",
            mod_slug
        ))
        .prompt()?;
    }

    // description
    let _mod_desc = Text::new("What is the description of the mod?").prompt()?;

    // categories
    let categories = client.get_categories()?;
    let _mod_category =
        Select::new("What category does your mod fit into?", categories).prompt()?;

    // version
    let beat_saber_versions = client.get_beat_saber_versions()?;
    let _mod_bs_versions = MultiSelect::new(
        "What version(s) of Beat Saber are you targeting?",
        beat_saber_versions,
    )
    .with_validator(MinLengthValidator::new(1))
    .prompt()?;

    let instances = get_instance_paths();
    let instance = if instances.len() == 0 {
        let ipath: PathBuf = Text::new("Could not autodetect Beat Saber install. Please enter the path to your Beat Saber install.")
            .with_validator(FileExistsValidatior)
            .prompt()?.into();

        Instance {
            path: ipath.clone(),
            name: "Custom".into(),
            game_version: get_game_version(ipath.to_str().unwrap().to_string()),
        }

    } else {
        Select::new("Which Beat Saber install would you like to build in?", instances).prompt()?
    };

    let pb = make_progress();
    pb.set_message(format!("Creating {}...", mod_name_pascal));

    let mod_path = std::env::current_dir()?.join(&mod_name_pascal);
    ForgeGenerator::new(mod_name_pascal, mod_path.to_str().unwrap().to_string(), instance).generate();

    finish_progress(&pb, "Done!");

    Ok(())
}
