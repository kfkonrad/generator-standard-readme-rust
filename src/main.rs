mod prompt;
mod prompt_generate;
mod data_types;

use std::{fs::File, path::Path, process};
use std::io::Write;
use data_types::StandardReadmeConfig;
use prompt::Prompt;

use tera::{Context, Tera};

const README_TEMPLATE: &str = include_str!("../templates/README.md");

fn main() -> anyhow::Result<()> {
    if Path::new("README.md").exists() {
        let override_readme = prompt::bool("Warning: found existing README.md. Override?", false)?;
        if !override_readme {
            println!("Exiting.");
            process::exit(0);
        }
    }
    let mut tera = Tera::default();
    tera.add_raw_template("README.md", README_TEMPLATE)?;
    let mut context = Context::new();

    let standard_readme_config = StandardReadmeConfig::prompt()?;
    context.insert("readme", &standard_readme_config);
    let rendered_readme = tera.render("README.md", &context)?;

    let mut output = File::create("README.md")?;
    write!(output, "{rendered_readme}")?;
    Ok(())
}
