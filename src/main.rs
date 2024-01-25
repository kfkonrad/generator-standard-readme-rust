mod prompt;
mod prompt_generate;

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

fn prompt_repo_data() -> anyhow::Result<RepoData> {
    Ok(RepoData {
        name: prompt::text("What is the name of your repo?")?,
        description: prompt::text("What is the description of this module?")?,
    })
}

fn prompt_additional_parts() -> anyhow::Result<AdditionalParts> {
    let has_banner = prompt::bool("Do have a banner image?", false)?;
    let banner_path: Option<String> = prompt::optional_text(
        "Where is the banner image? Ex: 'img/banner.png'",
        has_banner,
    )?;
    let add_standard_readme_badge =
        prompt::bool("Do you want a standard-readme compliant badge?", true)?;
    let add_more_badges_todo = prompt::bool(
        "Do you want a TODO dropped where more badges should be?",
        false,
    )?;
    let add_long_description_todo = prompt::bool(
        "Do you want a TODO dropped where your long description should be?",
        true,
    )?;
    let add_security_section = prompt::bool("Do you need a prioritized security section?", false)?;
    let add_background_section = prompt::bool("Do you need a background section?", false)?;
    let add_api_section = prompt::bool("Do you need an API section?", false)?;
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
    let maintainer_handle = prompt::text("What is the username of the main maintainer?")?;
    let use_github_com = prompt::bool("Is the project host on github.com?", true)?;
    let custom_domain = prompt::optional_domain("Where is the project hosted?", !use_github_com)?;
    let mention_contributing_file = prompt::bool("Do you have a CONTRIBUTING.md file?", false)?;
    let allow_prs = prompt::bool("Are PRs accepted?", true)?;
    let use_mit = prompt::bool("Is an MIT license OK?", true)?;
    let custom_license = prompt::optional_text("What is your license?", !use_mit)?;
    let license_holder = prompt::text("Who is the License holder (probably your name)?")?;
    let use_current_year = prompt::bool("Use the current year?", true)?;
    let custom_year =
        prompt::optional_years("What year(s) would you like to specify?", !use_current_year)?;
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
    let standard_readme_config = StandardReadmeConfig {
        repo_data,
        additional_parts,
        collaboration_data,
    };
    println!("{standard_readme_config:#?}");
    Ok(())
}
