use crate::prompt;

pub struct RepoData {
    pub name: String,
    pub description: String,
}

pub struct AdditionalParts {
    pub has_banner: bool,
    pub banner_path: Option<String>,
    pub add_standard_readme_badge: bool,
    pub add_more_badges_todo: bool,
    pub add_long_description_todo: bool,
    pub add_security_section: bool,
    pub add_background_section: bool,
    pub add_api_section: bool,
}

pub struct CollaborationData {
    pub maintainer_handle: String,
    pub use_github_com: bool,
    pub custom_domain: Option<String>,
    pub mention_contributing_file: bool,
    pub allow_prs: bool,
    pub use_mit: bool,
    pub custom_license: Option<String>,
    pub license_holder: String,
    pub use_current_year: bool,
    pub custom_year: Option<String>,
}

pub struct StandardReadmeConfig {
    pub repo_data: RepoData,
    pub additional_parts: AdditionalParts,
    pub collaboration_data: CollaborationData,
}

impl prompt::Prompt for RepoData {
    fn prompt() -> anyhow::Result<Self> {
        Ok(Self {
            name: prompt::repo_name()?,
            description: prompt::text("What is the description of this module?")?,
        })
    }
}

impl prompt::Prompt for AdditionalParts {
    fn prompt() -> anyhow::Result<Self> {
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
        let add_security_section =
            prompt::bool("Do you need a prioritized security section?", false)?;
        let add_background_section = prompt::bool("Do you need a background section?", false)?;
        let add_api_section = prompt::bool("Do you need an API section?", false)?;
        Ok(Self {
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
}

impl prompt::Prompt for CollaborationData {
    fn prompt() -> anyhow::Result<Self> {
        let maintainer_handle = prompt::text("What is the username of the main maintainer?")?;
        let use_github_com = prompt::bool("Is the project hosted on github.com?", true)?;
        let custom_domain =
            prompt::optional_domain("Where is the project hosted?", !use_github_com)?;
        let mention_contributing_file = prompt::bool("Do you have a CONTRIBUTING.md file?", false)?;
        let allow_prs = prompt::bool("Are PRs accepted?", true)?;
        let use_mit = prompt::bool("Is an MIT license OK?", true)?;
        let custom_license = prompt::optional_text("What is your license?", !use_mit)?;
        let license_holder = prompt::license_holder()?;
        let use_current_year = prompt::bool("Use the current year?", true)?;
        let custom_year =
            prompt::optional_years("What year(s) would you like to specify?", !use_current_year)?;
        Ok(Self {
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
}

impl prompt::Prompt for StandardReadmeConfig {
    fn prompt() -> anyhow::Result<Self> {
        let repo_data = RepoData::prompt()?;
        let additional_parts = AdditionalParts::prompt()?;
        let collaboration_data = CollaborationData::prompt()?;
        Ok(Self {
            repo_data,
            additional_parts,
            collaboration_data,
        })
    }
}
