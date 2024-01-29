use std::env;

use crate::prompt_generate;
use convert_case::{Case, Casing};
use git2::{Config, Repository};

prompt_generate::with_optional!(text, "^.+$");
prompt_generate::with_optional!(years, r"^$");
prompt_generate::with_optional!(
    domain,
    r"^(((?!-))(xn--|_)?[a-z0-9-]{0,61}[a-z0-9]{1,1}\.)*(xn--)?([a-z0-9][a-z0-9\-]{0,60}|[a-z0-9-]{1,30}\.[a-z]{2,})$"
);

/// # Errors
///
/// Returns an error if retrieving the input from the user was unsuccessful
pub fn bool(message: &str, default: bool) -> anyhow::Result<bool> {
    let result = inquire::Confirm::new(message)
        .with_default(default)
        .prompt();

    match result {
        Ok(text) => Ok(text),
        Err(e) => Err(anyhow::anyhow!(e)),
    }
}

fn git_user_name() -> Option<String> {
    match {
        let repo = Repository::open(".").ok()?;
        let config = repo.config().ok()?;
        config.get_string("user.name").ok()
    } {
        Some(name) => Some(name),
        None => {
            let config = Config::open_default().ok()?;
            config.get_string("user.name").ok()
        }
    }
}

fn inquire_text_with_default(message: &str, default: &str) -> anyhow::Result<String> {
    let result = inquire::Text::new(message)
        .with_initial_value(default)
        .with_validator(validate_text)
        .prompt();
    match result {
        Ok(text) => Ok(text),
        Err(e) => Err(anyhow::anyhow!(e)),
    }
}

pub fn license_holder() -> anyhow::Result<String> {
    let suggested_licensee = git_user_name().unwrap_or_default();
    inquire_text_with_default(
        "Who is the License holder (probably your name)?",
        &suggested_licensee,
    )
}

pub fn repo_name() -> anyhow::Result<String> {
    let full_current_dir = env::current_dir().unwrap_or_default();
    let current_dir = match full_current_dir.components().last() {
        Some(dirname) => dirname.as_os_str().to_string_lossy().to_string(),
        None => String::new(),
    };
    let suggested_name = current_dir.to_case(Case::Kebab);
    inquire_text_with_default("What is the name of your repo?", &suggested_name)
}

pub trait Prompt {
    fn prompt() -> anyhow::Result<Self>
    where
        Self: Sized;
}
