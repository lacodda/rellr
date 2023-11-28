use super::msg;
use crate::commands::init::InitArgs;
use crate::libs::msg::Msg;
use serde::{Deserialize, Serialize};
use serde_json::from_str;
use std::error::Error;
use std::fs::{metadata, read_to_string, File};

pub const PROJECT_CONFIG: &str = "rellr.json";

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ProjectConfig {
    pub name: String,
    pub version: String,
}

impl ProjectConfig {
    pub fn new(init_args: InitArgs) -> Self {
        Self {  
            name: init_args.name,
            version: init_args.version,
        }
    }

    pub fn save(&mut self) -> Result<(), Box<dyn Error>> {
        let file = File::create(PROJECT_CONFIG)?;
        serde_json::to_writer_pretty(&file, &self)?;
        Ok(())
    }

    pub fn get() -> Result<Self, Box<dyn Error>> {
        let data = read_to_string(PROJECT_CONFIG);
        if data.is_err() {
            Msg::new(msg::RELLR_FILE_IS_MISSING).error();
            Msg::new(msg::RELLR_INIT_HELP).info().exit()
        }
        let data = data.unwrap();
        let project_config: Self = from_str(&data).unwrap();
        Ok(project_config)
    }

    pub fn new_if_not_exist(init_args: InitArgs) -> Result<Self, Box<dyn Error>> {
        if metadata(PROJECT_CONFIG).is_ok() {
            Msg::new(msg::RELLR_FILE_IS_ALREADY_CREATED).warn().exit()
        }

        Ok(Self::new(init_args))
    }
}
