use super::project_config::ProjectConfig;
use git_cliff_core::changelog::Changelog as GitCliffChangelog;
use git_cliff_core::commit::Commit;
use git_cliff_core::config::Config;
use git_cliff_core::error::Result;
use git_cliff_core::release::Release;
use git_cliff_core::repo::Repository;
use std::env;
use std::fs::File;
use std::io::Write;

pub const BUILTIN_CONFIG: &str = include_str!("../../examples/cliff.toml");
pub const DEFAULT_OUTPUT: &str = "CHANGELOG.md";
// pub const DEFAULT_CONFIG: &str = "cliff.toml";

#[allow(dead_code)]
enum ProcessOutput<'a> {
    Releases(Vec<Release<'a>>),
    Version(String),
}

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

    pub fn build(&mut self) -> Result<()> {
        let config = Self::get_builtin_config().unwrap();
        let repositories = vec![env::current_dir()?];
        let mut releases = Vec::<Release>::new();
        let mut versions = Vec::<String>::new();
        for repository in repositories {
            let repository = Repository::init(repository)?;
            let process_output = self.process_repository(Box::leak(Box::new(repository)), config.clone())?;

            match process_output {
                ProcessOutput::Releases(release) => releases.extend(release),
                ProcessOutput::Version(version) => versions.push(version),
            }
        }

        if !versions.is_empty() {
            let buf = versions.join("\n");
            let mut output = File::create(DEFAULT_OUTPUT)?;
            output.write_all(buf.as_bytes())?;
            return Ok(());
        }

        let changelog = GitCliffChangelog::new(releases, &config)?;
        let mut output = File::create(DEFAULT_OUTPUT)?;
        let _ = changelog.generate(&mut output);

        Ok(())
    }

    fn get_builtin_config() -> Result<Config> {
        Ok(toml::from_str(&BUILTIN_CONFIG)?)
    }

    fn process_repository<'a>(&mut self, repository: &'static Repository, config: Config) -> Result<ProcessOutput<'a>> {
        let topo_order = false;
        let mut tags = repository.tags(&config.git.tag_pattern, topo_order)?;
        let skip_regex = config.git.skip_tags.as_ref();
        let ignore_regex = config.git.ignore_tags.as_ref();
        tags = tags
            .into_iter()
            .filter(|(_, name)| {
                // Keep skip tags to drop commits in the later stage.
                let skip = skip_regex.map(|r| r.is_match(name)).unwrap_or_default();

                let ignore = ignore_regex
                    .map(|r| {
                        if r.as_str().trim().is_empty() {
                            return false;
                        }

                        r.is_match(name)
                    })
                    .unwrap_or_default();

                skip || !ignore
            })
            .collect();

        let mut commits = repository.commits(None, None, None)?;
        if let Some(commit_limit_value) = config.git.limit_commits {
            commits = commits.drain(..commits.len().min(commit_limit_value)).collect();
        }

        // Update tags.
        let tag = &self.project_config.current;
        if let Some(commit_id) = commits.first().map(|c| c.id().to_string()) {
            match tags.get(&commit_id) {
                Some(tag) => {
                    println!("There is already a tag ({}) for {}", tag, commit_id)
                }
                None => {
                    tags.insert(commit_id, tag.to_string());
                }
            }
        }

        // Process releases.
        let mut releases = vec![Release::default()];
        let mut release_index = 0;
        let mut previous_release = Release::default();
        let mut first_processed_tag = None;
        for git_commit in commits.into_iter().rev() {
            let commit = Commit::from(&git_commit);
            let commit_id = commit.id.to_string();
            releases[release_index].commits.push(commit);
            if let Some(tag) = tags.get(&commit_id) {
                releases[release_index].version = Some(tag.to_string());
                releases[release_index].commit_id = Some(commit_id);
                releases[release_index].timestamp = git_commit.time().seconds();
                if first_processed_tag.is_none() {
                    first_processed_tag = Some(tag);
                }
                previous_release.previous = None;
                releases[release_index].previous = Some(Box::new(previous_release));
                previous_release = releases[release_index].clone();
                releases.push(Release::default());
                release_index += 1;
            }
        }

        if release_index > 0 {
            previous_release.previous = None;
            releases[release_index].previous = Some(Box::new(previous_release));
        }

        // Set the previous release if the first release does not have one set.
        if !releases.is_empty() && releases.first().and_then(|r| r.previous.as_ref()).and_then(|p| p.version.as_ref()).is_none() {
            // Get the previous tag of the first processed tag in the release loop.
            let first_tag = first_processed_tag
                .map(|tag| {
                    tags.iter()
                        .enumerate()
                        .find(|(_, (_, v))| v == &tag)
                        .and_then(|(i, _)| i.checked_sub(1))
                        .and_then(|i| tags.get_index(i))
                })
                .or_else(|| Some(tags.last()))
                .flatten();

            // Set the previous release if the first tag is found.
            if let Some((commit_id, version)) = first_tag {
                let previous_release = Release {
                    commit_id: Some(commit_id.to_string()),
                    version: Some(version.to_string()),
                    ..Release::default()
                };
                releases[0].previous = Some(Box::new(previous_release));
            }
        }

        Ok(ProcessOutput::Releases(releases))
    }
}
