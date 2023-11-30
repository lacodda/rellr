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
    project_config = project_config.up_version(&next_args.update_type)?;
    project_config.save()?;

    Git::repo(&project_config).branch()?.checkout()?;

    Msg::new(&format!("{} {}", &msg::NEXT, &project_config.next.unwrap())).info();

    Ok(())
}
