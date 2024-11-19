#![allow(clippy::useless_let_if_seq)]

mod data_types;
mod prompt;
mod prompt_generate;
mod templates;

use anyhow::anyhow;
use askama::Template;
use chrono::{Datelike, Local};
use data_types::StandardReadmeConfig;
use prompt::Prompt;
use std::io::Write;
use std::{fs::File, path::Path, process};
use templates::{StandardReadmeTemplate, StandardReadmeTemplateDE};

use clap::Parser;

#[derive(Parser)]
#[command(name = "standard-readme")]
#[command(author = "Kevin F. Konrad")]
#[command(version = "0.2")]
#[command(about = "Generate standard READMEs", long_about = None)]
#[command(disable_version_flag = true)]
struct Cli {
    #[arg(short = 'v', short_alias = 'V', long, action = clap::builder::ArgAction::Version)]
    version: (),
    #[arg(
        long,
        short = 'l',
        help = "Choose language for the Standard Readme (supported: en, de)",
        default_value = "en",

    )]
    language: String,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    if Path::new("README.md").exists() {
        let override_readme = prompt::bool("Warning: found existing README.md. Override?", false)?;
        if !override_readme {
            println!("Exiting.");
            process::exit(0);
        }
    }

    let empty_string = &String::new();

    let rendered_readme = match cli.language.as_str() {
        "en" => StandardReadmeTemplate {
            src: StandardReadmeConfig::prompt()?,
            current_year: Local::now().year(),
            empty_string: empty_string,
        }
        .render()
        .map_err(|e| anyhow!(e)),
        "de" => StandardReadmeTemplateDE {
            src: StandardReadmeConfig::prompt()?,
            current_year: Local::now().year(),
            empty_string: empty_string,
        }
        .render()
        .map_err(|e| anyhow!(e)),
        l => Err(anyhow!(format!("language {} not implemented", l))),
    }?;

    let mut output = File::create("README.md")?;
    write!(output, "{rendered_readme}")?;
    Ok(())
}
