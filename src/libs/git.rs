use super::{
    msg::{self, Msg},
    project_config::ProjectConfig,
};
use git2::{Repository, RepositoryInitOptions};
use std::fmt;

#[derive(Debug, Default, Clone)]
pub enum BranchType {
    #[default]
    Release,
    Feature,
    Hotfix,
}

impl fmt::Display for BranchType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub struct Git {
    pub project_config: ProjectConfig,
    pub repo: Repository,
}

impl Git {
    pub fn repo(project_config: &ProjectConfig) -> Self {
        let repo_path = ".";
        let open_repo = Repository::open(&repo_path);
        let repo: Repository = match open_repo {
            Ok(repo) => repo,
            _ => Self::init(&repo_path, &project_config.main_branch).unwrap(),
        };

        Self {
            project_config: project_config.clone(),
            repo: repo,
        }
    }

    pub fn init(repo_path: &str, main_branch: &str) -> Result<Repository, git2::Error> {
        let mut init_options = RepositoryInitOptions::new();
        init_options.initial_head(&main_branch);
        let repo: Repository = Repository::init_opts(&repo_path, &init_options)?;

        Ok(repo)
    }

    pub fn rename_or_new_branch(&mut self) -> Result<Self, git2::Error> {
        if self.next_branch_name().is_none() {
            Msg::new(msg::RELEASE_VERSION_NOT_SET).error().exit()
        }

        if self.prev_branch_name().is_some() {
            return self.rename_branch();
        }

        self.new_branch()
    }

    fn new_branch(&mut self) -> Result<Self, git2::Error> {
        let next_branch_name = self.next_branch_name().unwrap();
        let next_branch = self.repo.find_branch(&next_branch_name, git2::BranchType::Local);
        if next_branch.is_err() {
            let main_branch = &self.repo.find_branch(&self.project_config.main_branch, git2::BranchType::Local)?;
            self.repo.branch(&next_branch_name, &main_branch.get().peel_to_commit()?, false)?;
        }

        Ok(Self::repo(&self.project_config))
    }

    fn rename_branch(&mut self) -> Result<Self, git2::Error> {
        let prev_branch_name = self.prev_branch_name().unwrap();
        let next_branch_name = self.next_branch_name().unwrap();
        let prev_branch = self.repo.find_branch(&prev_branch_name, git2::BranchType::Local);
        if prev_branch.is_err() {
            return Self::repo(&self.project_config).new_branch();
        }

        let _ = prev_branch?.rename(&next_branch_name, false);
        Ok(Self::repo(&self.project_config))
    }

    pub fn checkout(&mut self) -> Result<(), git2::Error> {
        if self.next_branch_name().is_none() {
            Msg::new(msg::RELEASE_VERSION_NOT_SET).error().exit()
        }
        let next_branch_name = self.next_branch_name().unwrap();
        let branch_ref = self.repo.find_branch(&next_branch_name, git2::BranchType::Local)?;
        self.repo
            .set_head(&branch_ref.get().name().unwrap_or(&format!("refs/heads/{}", &self.project_config.main_branch)))?;

        Ok(())
    }

    fn next_branch_name(&mut self) -> Option<String> {
        match self.project_config.next {
            Some(_) => Some(self.get_branch_name(&self.project_config.next.clone().unwrap())),
            _ => None,
        }
    }

    fn prev_branch_name(&mut self) -> Option<String> {
        match self.project_config.prev {
            Some(_) => Some(self.get_branch_name(&self.project_config.prev.clone().unwrap())),
            _ => None,
        }
    }

    fn get_branch_name(&mut self, version: &str) -> String {
        format!("{}/{}", self.project_config.branch_type.to_string().to_lowercase(), &version)
    }
}
