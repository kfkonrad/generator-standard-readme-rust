mod data_types;
mod prompt;
mod prompt_generate;

use chrono::{Datelike, Local};
use data_types::StandardReadmeConfig;
use prompt::Prompt;
use std::io::Write;
use std::{fs::File, path::Path, process};

use askama::Template;
use clap::Parser;

#[derive(Template)]
#[template(path = "../templates/README.md")]
struct StandardReadmeTemplate<'a> {
    src: StandardReadmeConfig,
    current_year: i32,
    // without a reference to an empty String Askama will complain about type mismatches
    empty_string: &'a String,
}

#[derive(Parser)]
#[command(name = "standard-readme")]
#[command(author = "Kevin F. Konrad")]
#[command(version = "0.1")]
#[command(about = "Generate standard READMEs", long_about = None)]
#[command(disable_version_flag = true)]
struct Cli {
    #[arg(short = 'v', short_alias = 'V', long, action = clap::builder::ArgAction::Version)]
    version: (),
}

fn main() -> anyhow::Result<()> {
    // this is used strictly for the -h/--help and -v/-V/--version flags
    let _cli = Cli::parse();

    if Path::new("README.md").exists() {
        let override_readme = prompt::bool("Warning: found existing README.md. Override?", false)?;
        if !override_readme {
            println!("Exiting.");
            process::exit(0);
        }
    }

    let standard_readme_config = StandardReadmeConfig::prompt()?;
    let hello = StandardReadmeTemplate {
        src: standard_readme_config,
        current_year: Local::now().year(),
        empty_string: &String::new(),
    };
    let rendered_readme = hello.render()?;

    let mut output = File::create("README.md")?;
    write!(output, "{rendered_readme}")?;
    println!("{rendered_readme}");
    Ok(())
}
