use clap::Args;
use path_absolutize::*;
use regex::Regex;
use std::env;
use std::error::Error;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::path::Path;
use std::process::Command;

#[derive(Debug, Args)]
pub struct ReleaseArgs {
    #[arg(required = true)]
    project_folder: String,
    #[arg(required = true, short, long)]
    name: String,
    #[arg(required = true, short, long)]
    version: String,
}

pub fn cmd(release_args: ReleaseArgs) -> Result<(), Box<dyn Error>> {
    let project_folder = &release_args.project_folder;
    let project_name = &release_args.name;
    let new_version = &release_args.version;

    let project_folder = Path::new(project_folder);
    let project_folder: String = project_folder.absolutize().unwrap().to_str().unwrap().to_string();

    let npm = Path::new(r"C:\Program Files\nodejs");
    assert!(env::set_current_dir(&npm).is_ok());

    #[cfg(windows)]
    pub const NPM: &'static str = "npm.cmd";
    #[cfg(not(windows))]
    pub const NPM: &'static str = "npm";

    // Change to the project directory
    env::set_current_dir(&project_folder)?;

    // Update Cargo.toml
    let cargo_toml_path = Path::new("Cargo.toml");
    update_version_in_file(cargo_toml_path, &project_name, &new_version)?;

    // Update Cargo.lock
    let cargo_toml_path = Path::new("Cargo.lock");
    update_version_in_file(cargo_toml_path, &project_name, &new_version)?;

    // Update package.json
    let package_json_path = Path::new("npm/package.json");
    update_version_in_file(package_json_path, &project_name, &new_version)?;

    // Run git cliff
    let git_cliff_command = Command::new("git-cliff")
        .arg("--sort")
        .arg("newest")
        .arg("--output")
        .arg("CHANGELOG.md")
        .arg("--tag")
        .arg(new_version)
        .status()?;

    if !git_cliff_command.success() {
        eprintln!("Error running 'git cliff'");
        return Ok(());
    }

    // Commit the changes
    Command::new("git")
        .arg("add")
        .arg("Cargo.toml")
        .arg("Cargo.lock")
        .arg("npm/package.json")
        .arg("CHANGELOG.md")
        .spawn()?
        .wait()?;
    let _ = Command::new("git").arg("commit").arg("-m").arg(new_version).spawn()?.wait();
    let _ = Command::new("git").arg("tag").arg(format!("v{}", new_version)).spawn()?.wait();

    // Change to the project directory
    let project_npm_folder = Path::new(&project_folder).join("npm");
    let project_npm_folder: String = project_npm_folder.absolutize().unwrap().to_str().unwrap().to_string();
    env::set_current_dir(project_npm_folder)?;
    let _ = Command::new(NPM).arg("publish").spawn()?.wait();

    println!("Version {} updated successfully in project folder: {}", new_version, &project_folder);

    Ok(())
}

fn update_version_in_file(file_path: &Path, project_name: &str, new_version: &str) -> Result<(), Box<dyn Error>> {
    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let re = Regex::new(format!(r#"(?m)("*name("|\s)(:|=)\s"{}",*\s*("*version("|\s)(:|=)\s))("\d+\.\d+\.\d+")"#, project_name).as_str()).unwrap();

    let new_contents = re.replace(&contents, |caps: &regex::Captures| {
        let version_match = caps.get(1).unwrap().as_str();
        format!("{}\"{}\"", version_match, new_version)
    });

    let mut file = OpenOptions::new().write(true).truncate(true).open(file_path)?;

    file.write_all(new_contents.as_bytes())?;

    Ok(())
}
