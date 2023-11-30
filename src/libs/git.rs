use git2::Repository;
use std::fmt;

#[derive(Debug)]
pub enum BranchType {
    Feature,
    Release,
    Hotfix,
}

impl fmt::Display for BranchType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub struct Git {
    pub version: String,
}

impl Git {
    pub fn new(version: &str) -> Self {
        Self { version: version.to_string() }
    }

    pub fn branch(&mut self, branch_type: BranchType) -> Result<(), git2::Error> {
        let new_branch_name = format!("{}/{}", branch_type.to_string().to_lowercase(), &self.version);
        let repo = Repository::open(".")?;
        let head = repo.head()?;

        let new_branch = repo.branch(&new_branch_name, &head.peel_to_commit()?, false)?;
        let new_branch_name = new_branch.get().name().unwrap();
        repo.set_head(&new_branch_name)?;

        Ok(())
    }
}
