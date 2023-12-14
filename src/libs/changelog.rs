use super::project_config::ProjectConfig;
use git_cliff_core::config::Config;
use git_cliff_core::error::Result;

pub const BUILTIN_CONFIG: &str = include_str!("../../examples/cliff.toml");
pub const DEFAULT_CONFIG: &str = "cliff.toml";
pub const DEFAULT_OUTPUT: &str = "CHANGELOG.md";

pub struct Changelog {
    pub project_config: ProjectConfig,
    pub builtin_config: Result<Config>,
}

impl Changelog {
    pub fn new(project_config: &ProjectConfig) -> Self {
        let builtin_config = Self::get_builtin_config();

        Self {
            project_config: project_config.clone(),
            builtin_config,
        }
    }

    fn get_builtin_config() -> Result<Config> {
        Ok(toml::from_str(&BUILTIN_CONFIG)?)
    }
}
