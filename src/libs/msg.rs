use colored::*;
use std::process;

pub struct Msg {
    pub(crate) text: String,
}

impl Msg {
    pub fn new(text: &str) -> Self {
        Msg { text: text.into() }
    }

    pub fn info(&mut self) -> &mut Self {
        println!("{}", &self.text.cyan());
        self
    }

    pub fn warn(&mut self) -> &mut Self {
        println!("{}", &self.text.yellow());
        self
    }

    pub fn error(&mut self) -> &mut Self {
        println!("{}", &self.text.red());
        self
    }

    pub fn exit(&mut self) {
        process::exit(1);
    }
}

pub const RELLR_FILE_IS_MISSING: &str = "The rellr configuration file is missing in the selected directory";
pub const RELLR_FILE_IS_ALREADY_CREATED: &str = "The rellr configuration file has already been created";
pub const RELLR_FILE_WAS_CREATED: &str = "The rellr configuration file was created successfully";
pub const RELLR_INIT_HELP: &str = "To create a configuration file, run the command: `rellr init -n <your-project-name> -v <your-project-version>`";
pub const NEXT: &str = "Next version:";
pub const RELEASE_ALREADY_EXISTS: &str = "The release already exists";
pub const RELEASE_VERSION_NOT_SET: &str = "The release version has not yet been set";
pub const FEAT_WAS_CREATED: &str = "New feature was created successfully";
pub const FIX_WAS_CREATED: &str = "New hotfix was created successfully";

