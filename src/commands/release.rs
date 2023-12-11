use crate::libs::git::Git;
use crate::libs::helpers::to_abs_path;
use crate::libs::msg::{self, Msg};
use crate::libs::project_config::ProjectConfig;
use clap::Args;
use git2::{ObjectType, Signature};
use regex::Regex;
use std::env;
use std::error::Error;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::path::Path;
use std::process::Command;

#[derive(Debug, Args)]
#[command(args_conflicts_with_subcommands = true)]
pub struct ReleaseArgs {
    project_folder: Option<String>,
}

pub fn cmd(release_args: ReleaseArgs) -> Result<(), Box<dyn Error>> {
    let project_config = ProjectConfig::get()?;

    if project_config.next.is_none() {
        Msg::new(msg::RELEASE_VERSION_NOT_SET).error().exit()
    }

    let next_version = &project_config.clone().next.unwrap();
    let project_name = &project_config.name;

    let project_folder = to_abs_path(&release_args.project_folder.or(Some(".".into())).unwrap());
    env::set_current_dir(&project_folder)?;

    update_changelog(&next_version)?;

    let repo = Git::repo(&project_config).repo;
    let mut index = repo.index()?;

    let files = ["Cargo.toml", "Cargo.lock", "npm/package.json", "CHANGELOG.md"];

    for file in files {
        let path = Path::new(file);
        update_version_in_file(path, &project_name, &next_version)?;
        index.add_path(path)?;
    }

    index.write()?;
    let tree_id = index.write_tree()?;
    let tree = repo.find_tree(tree_id)?;

    let head = repo.head()?;
    let parent_commit = repo.find_commit(head.target().unwrap())?;

    let config = repo.config()?;
    let author_name = config.get_string("user.name").unwrap_or_default();
    let author_email = config.get_string("user.email").unwrap_or_default();
    let signature = Signature::now(&author_name, &author_email)?;
    let commit_id = repo.commit(Some("HEAD"), &signature, &signature, &next_version, &tree, &[&parent_commit])?;
    let commit = repo.find_object(commit_id, Some(ObjectType::Commit))?;

    let tag = format!("v{}", &next_version);
    repo.tag(&tag, &commit, &signature, &next_version, false)?;

    let npm_folder = to_abs_path("npm");
    publish_npm(&npm_folder)?;

    println!("Version {} updated successfully in project folder: {}", next_version, &project_folder);

    Ok(())
}

fn update_version_in_file(file_path: &Path, project_name: &str, next_version: &str) -> Result<(), Box<dyn Error>> {
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

fn update_changelog(next_version: &str) -> Result<(), Box<dyn Error>> {
    let git_cliff_command = Command::new("git-cliff")
        .arg("--sort")
        .arg("newest")
        .arg("--output")
        .arg("CHANGELOG.md")
        .arg("--tag")
        .arg(next_version)
        .status()?;

    if !git_cliff_command.success() {
        eprintln!("Error running 'git cliff'");
        return Ok(());
    }

    Ok(())
}

fn publish_npm(npm_folder: &str) -> Result<(), Box<dyn Error>> {
    let npm = Path::new(r"C:\Program Files\nodejs");
    assert!(env::set_current_dir(&npm).is_ok());

    #[cfg(windows)]
    pub const NPM: &'static str = "npm.cmd";
    #[cfg(not(windows))]
    pub const NPM: &'static str = "npm";

    env::set_current_dir(npm_folder)?;
    let _ = Command::new(NPM).arg("publish").spawn()?.wait();

    Ok(())
}
