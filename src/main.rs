use inquire::{
    error::CustomUserError,
    validator::Validation,
    Confirm, Text,
};
use fancy_regex::Regex;

#[derive(Debug)]
struct RepoData {
    name: String,
    description: String,
}

#[derive(Debug)]
struct AdditionalParts {
    has_banner: bool,
    banner_path: Option<String>,
    add_standard_readme_badge: bool,
    add_more_badges_todo: bool,
    add_long_description_todo: bool,
    add_security_section: bool,
    add_background_section: bool,
    add_api_section: bool,
}

#[derive(Debug)]
struct CollaborationData {
    maintainer_handle: String,
    use_github_com: bool,
    custom_domain: Option<String>,
    mention_contributing_file: bool,
    allow_prs: bool,
    use_mit: bool,
    custom_license: Option<String>,
    license_holder: String,
    use_current_year: bool,
    custom_year: Option<String>,
}

#[derive(Debug)]
struct StandardReadmeConfig {
    repo_data: RepoData,
    additional_parts: AdditionalParts,
    collaboration_data: CollaborationData,
}

type ValidationResult = Result<Validation, CustomUserError>;

fn validate_not_empty(input: &str) -> ValidationResult {
    let re = Regex::new("^.+$")?;
    if re.is_match(input)? {
        Ok(Validation::Valid)
    } else {
        Ok(Validation::Invalid("Must not be empty!".into()))
    }
}


fn validate_domain(input: &str) -> ValidationResult {
    let re = Regex::new(r"^(((?!-))(xn--|_)?[a-z0-9-]{0,61}[a-z0-9]{1,1}\.)*(xn--)?([a-z0-9][a-z0-9\-]{0,60}|[a-z0-9-]{1,30}\.[a-z]{2,})$")?;
    if re.is_match(input)? {
        Ok(Validation::Valid)
    } else {
        Ok(Validation::Invalid("Must be valid domain!".into()))
    }
}

fn validate_years(input: &str) -> ValidationResult {
    let re = Regex::new(r"^\d\d\d\d(-\d\d\d\d)?$")?;
    if re.is_match(input)? {
        Ok(Validation::Valid)
    } else {
        Ok(Validation::Invalid("Must be YYYY or YYYY-YYYY, eg. 2023 or 2023-2024".into()))
    }
}

fn prompt_text(message: &str) -> anyhow::Result<String> {
    let result = Text::new(message)
        .with_validator(validate_not_empty)
        .prompt();
    match result {
        Ok(text) => Ok(text),
        Err(e) => Err(anyhow::anyhow!(e)),
    }
}

fn prompt_domain(message: &str) -> anyhow::Result<String> {
    let result = Text::new(message)
        .with_validator(validate_domain)
        .prompt();
    match result {
        Ok(text) => Ok(text),
        Err(e) => Err(anyhow::anyhow!(e)),
    }
}

fn prompt_years(message: &str) -> anyhow::Result<String> {
    let result = Text::new(message)
        .with_validator(validate_years)
        .prompt();
    match result {
        Ok(text) => Ok(text),
        Err(e) => Err(anyhow::anyhow!(e)),
    }
}

fn prompt_optional_text(message: &str, condition: bool) -> anyhow::Result<Option<String>> {
    if condition {
        Ok(Some(prompt_text(
            message
        )?))
    } else {
        Ok(None)
    }
}


fn prompt_optional_domain(message: &str, condition: bool) -> anyhow::Result<Option<String>> {
    if condition {
        Ok(Some(prompt_domain(
            message
        )?))
    } else {
        Ok(None)
    }
}

fn prompt_optional_years(message: &str, condition: bool) -> anyhow::Result<Option<String>> {
    if condition {
        Ok(Some(prompt_years(
            message
        )?))
    } else {
        Ok(None)
    }}


fn prompt_bool(message: &str, default: bool) -> anyhow::Result<bool> {
    let result = Confirm::new(message).with_default(default).prompt();

    match result {
        Ok(text) => Ok(text),
        Err(e) => Err(anyhow::anyhow!(e)),
    }
}

fn prompt_repo_data() -> anyhow::Result<RepoData> {
    Ok(RepoData {
        name: prompt_text("What is the name of your repo?")?,
        description: prompt_text("What is the description of this module?")?,
    })
}

fn prompt_additional_parts() -> anyhow::Result<AdditionalParts> {
    let has_banner = prompt_bool("Do have a banner image?", false)?;
    let banner_path: Option<String> = prompt_optional_text("Where is the banner image? Ex: 'img/banner.png'", has_banner)?;
    let add_standard_readme_badge =
        prompt_bool("Do you want a standard-readme compliant badge?", true)?;
    let add_more_badges_todo = prompt_bool(
        "Do you want a TODO dropped where more badges should be?",
        false,
    )?;
    let add_long_description_todo = prompt_bool(
        "Do you want a TODO dropped where your long description should be?",
        true,
    )?;
    let add_security_section = prompt_bool("Do you need a prioritized security section?", false)?;
    let add_background_section = prompt_bool("Do you need a background section?", false)?;
    let add_api_section = prompt_bool("Do you need an API section?", false)?;
    Ok(AdditionalParts {
        has_banner,
        banner_path,
        add_standard_readme_badge,
        add_more_badges_todo,
        add_long_description_todo,
        add_security_section,
        add_background_section,
        add_api_section,
    })
}

fn prompt_collaboration_data() -> anyhow::Result<CollaborationData> {
    let maintainer_handle = prompt_text("What is the username of the main maintainer?")?;
    let use_github_com= prompt_bool("Is the project host on github.com?", true)?;
    let custom_domain= prompt_optional_domain("Where is the project hosted?", !use_github_com)?;
    let mention_contributing_file= prompt_bool("Do you have a CONTRIBUTING.md file?", true)?;
    let allow_prs = prompt_bool("Are PRs accepted?", true)?;
    let use_mit = prompt_bool("Is an MIT license OK?", true)?;
    let custom_license = prompt_optional_text("What is your license?", !use_mit)?;
    let license_holder = prompt_text("Who is the License holder (probably your name)?")?;
    let use_current_year = prompt_bool("Use the current year?", true)?;
    let custom_year = prompt_optional_years("What year(s) would you like to specify?", !use_current_year)?;
    Ok(CollaborationData {
        maintainer_handle,
        use_github_com,
        custom_domain,
        mention_contributing_file,
        allow_prs,
        use_mit,
        custom_license,
        license_holder,
        use_current_year,
        custom_year,
    })
}

fn main() -> anyhow::Result<()> {
    let repo_data = prompt_repo_data()?;
    let additional_parts = prompt_additional_parts()?;
    let collaboration_data = prompt_collaboration_data()?;
    let standard_readme_config = StandardReadmeConfig{ repo_data, additional_parts, collaboration_data };
    println!("{standard_readme_config:#?}");
    Ok(())
}
