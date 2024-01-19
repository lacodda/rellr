use crate::libs::changelog::Changelog;
use crate::libs::git::Git;
use crate::libs::helpers::check_files_existence;
use crate::libs::msg::{self, Msg};
use crate::libs::project_config::{ProjectConfig, PROJECT_CONFIG};
use clap::Args;
use regex::Regex;
use std::error::Error;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::path::Path;

#[derive(Debug, Args)]
#[command(args_conflicts_with_subcommands = true)]
pub struct ReleaseArgs {
    project_folder: Option<String>,
}

pub fn cmd(_release_args: ReleaseArgs) -> Result<(), Box<dyn Error>> {
    let mut project_config = ProjectConfig::get()?;

    if project_config.next.is_none() {
        Msg::new(msg::RELEASE_VERSION_NOT_SET).error().exit()
    }

    let paths = project_config.paths();
    let mut paths: Vec<&str> = paths.iter().map(|s| s.as_str()).collect();
    let non_existent_files = check_files_existence(paths.clone());
    if !non_existent_files.is_empty() {
        Msg::new(msg::RELEASE_FAILED).error();
        Msg::new(&format!("{}\n{}", msg::FILES_ARE_MISSING, &non_existent_files.join("\n"))).warn().exit()
    }

    let mut git = Git::new(&project_config).merge()?;
    let _ = project_config.next_to_current()?.save();

    let version = &project_config.current;
    let project_name = &project_config.name;

    // UPDATE VERSION
    for path in &paths {
        update_version_in_file(path, &project_name, &version)?;
    }

    // CHANGELOG.md
    let mut changelog = Changelog::new(&project_config);
    let _ = changelog.build();
    let output_file_name = changelog.output_file_name();
    paths.push(&output_file_name);

    // GIT ADD and COMMIT
    paths.push(PROJECT_CONFIG);
    git.commit(paths)?;

    Msg::new(&format!("{} {}", &msg::RELEASE_COMPLETED_SUCCESSFULLY, &version)).info();

    let _ = project_config.publish();

    Ok(())
}

fn update_version_in_file(path: &str, project_name: &str, next_version: &str) -> Result<(), Box<dyn Error>> {
    let file_path = Path::new(path);
    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let re = Regex::new(format!(r#"(?m)("*name("|\s)(:|=)\s"{}",*\s*("*version("|\s)(:|=)\s))("\d+\.\d+\.\d+")"#, project_name).as_str()).unwrap();

    let new_contents = re.replace(&contents, |caps: &regex::Captures| {
        let version_match = caps.get(1).unwrap().as_str();
        format!("{}\"{}\"", version_match, next_version)
    });

    let mut file = OpenOptions::new().write(true).truncate(true).open(file_path)?;

    file.write_all(new_contents.as_bytes())?;

    Ok(())
}
