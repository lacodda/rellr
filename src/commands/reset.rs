use clap::Args;
use std::error::Error;
use std::process::Command;

#[derive(Debug, Args)]
pub struct ResetArgs {
    #[arg(required = true)]
    version: String,
}

pub fn cmd(reset_args: ResetArgs) -> Result<(), Box<dyn Error>> {
    let _ = Command::new("git").arg("tag").arg("-d").arg(format!("v{}", &reset_args.version)).spawn()?.wait();
    let _ = Command::new("git").arg("reset").arg("--hard").arg("HEAD~").spawn()?.wait()?;

    println!("Reset version {}", &reset_args.version);

    Ok(())
}
