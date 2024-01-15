use crate::libs::{project_config::ProjectConfig, msg::{Msg, self}};
use clap::Args;
use std::error::Error;

#[derive(Debug, Args)]
pub struct InitArgs {
    #[arg(required = true)]
    pub name: String,
    #[arg(short, long)]
    pub version: Option<String>,
}

pub fn cmd(init_args: InitArgs) -> Result<(), Box<dyn Error>> {
    let _ = ProjectConfig::new_if_not_exist(init_args)?.save()?;
    Msg::new(msg::RELLR_FILE_WAS_CREATED).info();
    Ok(())
}
