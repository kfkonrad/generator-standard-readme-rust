mod prompt;
mod prompt_generate;
mod data_types;

use data_types::StandardReadmeConfig;
use prompt::Prompt;

fn main() -> anyhow::Result<()> {
    let standard_readme_config = StandardReadmeConfig::prompt()?;
    println!("{standard_readme_config:#?}");
    Ok(())
}
