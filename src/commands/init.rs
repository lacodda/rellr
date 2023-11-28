use crate::libs::{project_config::ProjectConfig, msg::{Msg, self}};
use clap::Args;
use std::error::Error;

#[derive(Debug, Args)]
pub struct InitArgs {
    #[arg(required = true, short, long)]
    pub name: String,
    #[arg(required = true, short, long)]
    pub version: String,
}

pub fn cmd(init_args: InitArgs) -> Result<(), Box<dyn Error>> {
    let mut project_config = ProjectConfig::new_if_not_exist(init_args)?;
    let _ = project_config.save();
    Msg::new(msg::RELLR_FILE_WAS_CREATED).info();
    Ok(())
}
