mod commands;
use clap::{Parser, Subcommand};
use commands::{next, release, reset};
use std::error::Error;

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
#[command(arg_required_else_help(true))]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    #[command(about = "Create a new release", arg_required_else_help = true)]
    Next(next::NextArgs),
    #[command(about = "Release", arg_required_else_help = true)]
    Release(release::ReleaseArgs),
    #[command(about = "Remove last commit", arg_required_else_help = true)]
    Reset(reset::ResetArgs),
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Next(args) => next::cmd(args),
        Commands::Release(args) => release::cmd(args),
        Commands::Reset(args) => reset::cmd(args),
    }
}
