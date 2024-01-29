mod data_types;
mod prompt;
mod prompt_generate;

use data_types::StandardReadmeConfig;
use prompt::Prompt;
use std::io::Write;
use std::{fs::File, path::Path, process};
use chrono::{Local, Datelike};

use askama::Template;

#[derive(Template)]
#[template(path = "../templates/README.md")]
struct StandardReadmeTemplate<'a> {
    src: StandardReadmeConfig,
    current_year: i32,
    // without a reference to an empty String Askama will complain about type mismatches
    empty_string: &'a String,
}

fn main() -> anyhow::Result<()> {
    if Path::new("README.md").exists() {
        let override_readme = prompt::bool("Warning: found existing README.md. Override?", false)?;
        if !override_readme {
            println!("Exiting.");
            process::exit(0);
        }
    }

    let standard_readme_config = StandardReadmeConfig::prompt()?;
    let hello = StandardReadmeTemplate { src: standard_readme_config, current_year: Local::now().year(), empty_string: &"".to_string() };
    let rendered_readme = hello.render()?;

    let mut output = File::create("README.md")?;
    write!(output, "{rendered_readme}")?;
    println!("{rendered_readme}");
    Ok(())
}
