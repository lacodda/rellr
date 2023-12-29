use crate::libs::git::Git;
use crate::libs::msg::{self, Msg};
use crate::libs::project_config::ProjectConfig;
use clap::Args;
use std::error::Error;

#[derive(Debug, Args)]
pub struct ResetArgs {
    version: Option<String>,
}

pub fn cmd(_reset_args: ResetArgs) -> Result<(), Box<dyn Error>> {
    let project_config = ProjectConfig::get()?;
    let _ = Git::new(&project_config).reset();
    Msg::new(msg::LAST_COMMIT_WAS_REMOVED).info();
    Ok(())
}
