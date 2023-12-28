use super::{
    msg::{self, Msg},
    project_config::ProjectConfig,
};
use git2::{build::CheckoutBuilder, Error, MergeOptions, ObjectType, Repository, RepositoryInitOptions, ResetType, Signature};
use std::{fmt, path::Path};

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

#[allow(dead_code)]
enum VersionType {
    Main,
    Next,
}

pub struct Git {
    pub project_config: ProjectConfig,
    pub repo: Repository,
}

impl Git {
    pub fn new(project_config: &ProjectConfig) -> Self {
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

    pub fn add_branch(&mut self, name: &str) -> Result<Self, git2::Error> {
        let branch_name = self.get_branch_name(&name);
        let branch = self.repo.find_branch(&branch_name, git2::BranchType::Local);
        if branch.is_err() {
            let main_branch = &self.repo.find_branch(&self.project_config.main_branch, git2::BranchType::Local)?;
            self.repo.branch(&branch_name, &main_branch.get().peel_to_commit()?, false)?;
        }

        Ok(Self::new(&self.project_config))
    }

    pub fn add_or_rename_next_branch(&mut self) -> Result<Self, git2::Error> {
        if self.next_branch_name().is_none() {
            Msg::new(msg::RELEASE_VERSION_NOT_SET).error().exit()
        }

        if self.prev_branch_name().is_some() {
            return self.rename_next_branch();
        }

        self.add_next_branch()
    }

    fn add_next_branch(&mut self) -> Result<Self, git2::Error> {
        self.add_branch(&self.project_config.next.clone().unwrap())
    }

    fn rename_next_branch(&mut self) -> Result<Self, git2::Error> {
        let prev_branch_name = self.prev_branch_name().unwrap();
        let next_branch_name = self.next_branch_name().unwrap();
        let prev_branch = self.repo.find_branch(&prev_branch_name, git2::BranchType::Local);
        if prev_branch.is_err() {
            return Self::new(&self.project_config).add_next_branch();
        }

        let _ = prev_branch?.rename(&next_branch_name, false);
        Ok(Self::new(&self.project_config))
    }

    pub fn checkout(&mut self, name: Option<&str>) -> Result<(), git2::Error> {
        let main_branch_ref = self.get_branch_ref_name(VersionType::Main);
        let ref_name: String = match name {
            Some(name) => {
        let branch_name = self.get_branch_name(&name);
        let branch_ref = self.repo.find_branch(&branch_name, git2::BranchType::Local)?;
                branch_ref.get().name().unwrap_or(&main_branch_ref).to_owned()
            }
            None => main_branch_ref,
        };

        self.repo.set_head(&ref_name)?;
        self.repo.checkout_head(Some(CheckoutBuilder::default().force()))?;
        Ok(())
    }

    pub fn commit(&mut self, paths: Vec<&str>) -> Result<(), git2::Error> {
        if self.next_branch_name().is_none() {
            Msg::new(msg::RELEASE_VERSION_NOT_SET).error().exit()
        }
        let next_version = self.project_config.next.clone().unwrap();

        let mut index = self.repo.index()?;

        for path in paths {
            index.add_path(Path::new(path))?;
        }

        index.write()?;
        let tree_id = index.write_tree()?;
        let tree = self.repo.find_tree(tree_id)?;

        let mut git = Self::new(&self.project_config);
        let signature = git.get_signature()?;
        let head = self.repo.head()?;
        let parent_commit = self.repo.find_commit(head.target().unwrap())?;

        let commit_id = self.repo.commit(Some("HEAD"), &signature, &signature, &next_version, &tree, &[&parent_commit])?;
        let commit = self.repo.find_object(commit_id, Some(ObjectType::Commit))?;

        let tag = format!("v{}", &next_version);
        self.repo.tag(&tag, &commit, &signature, &next_version, false)?;

        Ok(())
    }

    pub fn checkout_next(&mut self) -> Result<(), git2::Error> {
        self.checkout(Some(&self.project_config.next.clone().unwrap()))
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

    fn get_branch_name(&mut self, name: &str) -> String {
        format!("{}/{}", self.project_config.branch_type.to_string().to_lowercase(), &name)
    }

    fn get_branch_ref_name(&mut self, version_type: VersionType) -> String {
        let next_branch_name = self.next_branch_name().unwrap();
        let name = match version_type {
            VersionType::Main => &self.project_config.main_branch,
            VersionType::Next => &next_branch_name,
        };

        format!("refs/heads/{}", name)
    }

    fn get_signature(&mut self) -> Result<Signature<'_>, git2::Error> {
        let config = self.repo.config()?;
        let author_name = config.get_string("user.name").unwrap_or_default();
        let author_email = config.get_string("user.email").unwrap_or_default();
        let signature = Signature::now(&author_name, &author_email)?;

        Ok(signature)
    }
}
