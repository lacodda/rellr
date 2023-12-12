use crate::libs::git::{BranchType, Git};
use crate::libs::msg::{self, Msg};
use crate::libs::project_config::ProjectConfig;
use clap::Args;
use std::error::Error;

#[derive(Debug, Args)]
pub struct FixArgs {
    #[arg(required = true)]
    name: String,
}

pub fn cmd(fix_args: FixArgs) -> Result<(), Box<dyn Error>> {
    let mut project_config = ProjectConfig::get()?;
    project_config.branch_type = BranchType::Hotfix;
    Git::new(&project_config).add_branch(&fix_args.name)?.checkout(&fix_args.name)?;

    Msg::new(msg::FIX_WAS_CREATED).info();
    Ok(())
}
