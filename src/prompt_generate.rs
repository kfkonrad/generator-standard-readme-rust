macro_rules! _without_optional {
  ($ident:ident, $regex:expr) => {
      paste::paste!{
          /// # Errors
          ///
          /// Returns an error if the used validation regex is invalid
          pub fn [<validate_ $ident>](input: &str) -> Result<inquire::validator::Validation, inquire::error::CustomUserError> {
              let re = fancy_regex::Regex::new($regex)?;
              if re.is_match(input)? {
                  Ok(inquire::validator::Validation::Valid)
              } else {
                  Ok(inquire::validator::Validation::Invalid("Must not be empty!".into()))
              }
          }

          /// # Errors
          ///
          /// Returns an error if retrieving the input from the user was unsuccessful
          pub fn $ident(message: &str) -> anyhow::Result<String> {
              let result = inquire::Text::new(message).with_validator([<validate_ $ident>]).prompt();
              match result {
                  Ok(text) => Ok(text),
                  Err(e) => Err(anyhow::anyhow!(e)),
              }
          }
      }
  };
}
pub(crate) use _without_optional as without_optional;

macro_rules! _with_optional {
  ($ident:ident, $regex:expr) => {
    $crate::prompt_generate::without_optional!($ident, $regex);
      paste::paste!{

          /// # Errors
          ///
          /// Returns an error if retrieving the input from the user was unsuccessful
          pub fn [<optional_ $ident>](message: &str, condition: bool) -> anyhow::Result<Option<String>> {
              if condition {
                  Ok(Some($ident(message)?))
              } else {
                  Ok(None)
              }
          }
      }
  };
}

pub(crate) use _with_optional as with_optional;
