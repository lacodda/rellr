use crate::libs::git::{BranchType, Git};
use crate::libs::msg::{self, Msg};
use crate::libs::project_config::ProjectConfig;
use clap::Args;
use std::error::Error;

#[derive(Debug, Args)]
pub struct FeatArgs {
    #[arg(required = true)]
    name: String,
}

pub fn cmd(feat_args: FeatArgs) -> Result<(), Box<dyn Error>> {
    let mut project_config = ProjectConfig::get()?;
    project_config.branch_type = BranchType::Feature;
    Git::repo(&project_config).add_branch(&feat_args.name)?.checkout(&feat_args.name)?;

    Msg::new(msg::FEAT_WAS_CREATED).info();
    Ok(())
}
