use crate::libs::{project_config::ProjectConfig, msg::{Msg, self}};
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
    project_config.up_version(&next_args.update_type).save()?;
    Msg::new(&format!("{} {}", &msg::NEXT, &project_config.version)).info();
    Ok(())
}
