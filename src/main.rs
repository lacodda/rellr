mod commands;
use clap::{Parser, Subcommand};
use commands::{feat, fix, init, next, release, reset};
use std::error::Error;
mod libs;

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
#[command(arg_required_else_help(true))]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    #[command(about = "Configuration initialization", arg_required_else_help = true)]
    Init(init::InitArgs),
    #[command(about = "Create a new release")]
    Next(next::NextArgs),
    #[command(about = "Feature", arg_required_else_help = true)]
    Feat(feat::FeatArgs),
    #[command(about = "Hotfix", arg_required_else_help = true)]
    Fix(fix::FixArgs),
    #[command(about = "Release")]
    Release(release::ReleaseArgs),
    #[command(about = "Remove last commit")]
    Reset(reset::ResetArgs),
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init(args) => init::cmd(args),
        Commands::Next(args) => next::cmd(args),
        Commands::Feat(args) => feat::cmd(args),
        Commands::Fix(args) => fix::cmd(args),
        Commands::Release(args) => release::cmd(args),
        Commands::Reset(args) => reset::cmd(args),
    }
}
