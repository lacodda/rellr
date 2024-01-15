use crate::libs::{
    git::Git,
    msg::{self, Msg},
    project_config::ProjectConfig,
};
use clap::{Args, ValueEnum};
use std::error::Error;

#[derive(Debug, Args)]
pub struct NextArgs {
    #[arg(
        default_value_t = UpdateType::Patch,
        value_enum
    )]
    update_type: UpdateType,
}

#[derive(ValueEnum, Copy, Clone, Debug, PartialEq, Eq)]
pub enum UpdateType {
    Patch,
    Minor,
    Major,
}

pub fn cmd(next_args: NextArgs) -> Result<(), Box<dyn Error>> {
    let mut project_config = ProjectConfig::get()?;
    let _ = project_config.up_version(&next_args.update_type)?.save()?;

    Git::new(&project_config).add_or_rename_next_branch()?.checkout_next()?;
    Msg::new(&format!("{} {}", &msg::NEXT, &project_config.next.unwrap())).info();

    Ok(())
}
