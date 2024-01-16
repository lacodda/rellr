use super::git::BranchType;
use super::msg;
use crate::commands::init::InitArgs;
use crate::commands::next::UpdateType;
use crate::libs::msg::Msg;
use serde::{Deserialize, Serialize};
use serde_json::from_str;
use std::error::Error;
use std::fs::{metadata, read_to_string, File};

pub const PROJECT_CONFIG: &str = "rellr.json";

#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PackageManagerType {
    Cargo,
    Npm,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageManager {
    pub r#type: PackageManagerType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publish: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectConfig {
    pub name: String,
    pub current: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next: Option<String>,
    #[serde(skip)]
    pub prev: Option<String>,
    #[serde(skip)]
    pub branch_type: BranchType,
    pub main_branch: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub changelog: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package_managers: Option<Vec<PackageManager>>,
}

impl ProjectConfig {
    pub fn new(init_args: InitArgs) -> Self {
        Self {
            name: init_args.name,
            current: init_args.version.unwrap_or("0.0.0".to_string()),
            next: None,
            prev: None,
            branch_type: BranchType::Release,
            main_branch: "main".into(),
            changelog: None,
            package_managers: None,
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

    pub fn up_version(&mut self, update_type: &UpdateType) -> Result<Self, Box<dyn Error>> {
        let mut version_vec: Vec<u32> = self.current.split(".").filter_map(|s| s.parse().ok()).collect();
        version_vec = match update_type {
            UpdateType::Patch => Self::increment(version_vec, 2),
            UpdateType::Minor => Self::increment(version_vec, 1),
            UpdateType::Major => Self::increment(version_vec, 0),
        };
        let next = version_vec.iter().map(|&n| n.to_string()).collect::<Vec<String>>().join(".");

        if self.next.clone().is_some_and(|n: String| n == next) {
            return Err(msg::RELEASE_ALREADY_EXISTS.to_string().into());
        }

        self.prev = self.next.clone();
        self.next = Some(next);
        Ok(self.to_owned())
    }

    pub fn next_to_current(&mut self) -> Result<Self, Box<dyn Error>> {
        if self.next.is_none() {
            Msg::new(msg::RELEASE_VERSION_NOT_SET).error().exit()
        }
        self.current = self.next.clone().unwrap();
        self.next = None;
        Ok(self.to_owned())
    }

    fn increment(mut version_vec: Vec<u32>, index: usize) -> Vec<u32> {
        if let Some(value) = version_vec.get_mut(index) {
            *value += 1;

            for i in (index + 1)..version_vec.len() {
                version_vec[i] = 0;
            }
        }
        version_vec
    }
}
