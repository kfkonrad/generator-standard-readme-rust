use askama::Template;
use crate::data_types::StandardReadmeConfig;

#[derive(Template)]
#[template(path = "../templates/README.md")]
pub struct StandardReadmeTemplate<'a> {
    pub src: StandardReadmeConfig,
    pub current_year: i32,
    // without a reference to an empty String Askama will complain about type mismatches
    pub empty_string: &'a String,
}

#[derive(Template)]
#[template(path = "../templates/README-de.md")]
pub struct StandardReadmeTemplateDE<'a> {
    pub src: StandardReadmeConfig,
    pub current_year: i32,
    pub empty_string: &'a String,
}
