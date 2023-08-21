use crate::{api::Client, config::Config, utils::progress::*};
use anyhow::Result;
use inquire::{validator::ExactLengthValidator, Password, PasswordDisplayMode, Select};

enum LoginMethod {
    Browser,
    ApiKey,
}

impl std::fmt::Display for LoginMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LoginMethod::Browser => write!(f, "Browser"),
            LoginMethod::ApiKey => write!(f, "Api Key"),
        }
    }
}

pub fn login(client: &mut Client, config: &mut Config) -> Result<()> {
    let method = Select::new(
        "Login method",
        vec![LoginMethod::Browser, LoginMethod::ApiKey],
    )
    .prompt()?;

    let key;
    match method {
        LoginMethod::Browser => {
            // todo:
            todo!("Login with browser")
        }
        LoginMethod::ApiKey => {
            let api_key = Password::new("Api Key")
                .with_validator(ExactLengthValidator::new(36))
                .with_display_mode(PasswordDisplayMode::Masked)
                .without_confirmation()
                .prompt()?;

            key = api_key;
        }
    }

    let pb = make_progress();
    config.set_api_key(key);
    client.api_key = config.api_key.clone();

    let current_user = client.get_me()?;
    finish_progress(&pb, format!("Logged in as {}", current_user.username));
    Ok(())
}
