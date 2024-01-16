use crate::libs::changelog::Changelog;
use crate::libs::git::Git;
use crate::libs::helpers::to_abs_path;
use crate::libs::msg::{self, Msg};
use crate::libs::project_config::ProjectConfig;
use clap::Args;
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
    let mut project_config = ProjectConfig::get()?;

    if project_config.next.is_none() {
        Msg::new(msg::RELEASE_VERSION_NOT_SET).error().exit()
    }

    let mut git = Git::new(&project_config).merge()?;
    let _ = project_config.next_to_current()?.save();

    let version = &project_config.current;
    let project_name = &project_config.name;

    let project_folder = to_abs_path(&release_args.project_folder.or(Some(".".into())).unwrap());
    env::set_current_dir(&project_folder)?;

    let mut paths = vec!["Cargo.toml", "Cargo.lock", "npm/package.json", "rellr.json"];

    let mut changelog = Changelog::new(&project_config);
    let changelog_file_name = changelog.output_file_name();
    paths.push(&changelog_file_name);
    let _ = changelog.build();


    for path in paths.clone() {
        update_version_in_file(&path, &project_name, &version)?;
    }
    
    git.commit(paths)?;

    // let npm_folder = to_abs_path("npm");
    // publish_npm(&npm_folder)?;

    println!("Version {} updated successfully in project folder: {}", &version, &project_folder);

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
