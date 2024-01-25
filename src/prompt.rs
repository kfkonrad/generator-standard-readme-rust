use crate::prompt_generate;

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

pub trait Prompt {
    fn prompt() -> anyhow::Result<Self>
    where
        Self: Sized;
}
