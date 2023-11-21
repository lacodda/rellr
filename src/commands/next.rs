use clap::{Args, ValueEnum};
use std::error::Error;

#[derive(Debug, Args)]
pub struct NextArgs {
    #[arg(
        default_value_t = UpdateType::Patch,
        value_enum
    )]
    update_type: UpdateType,
}

#[derive(ValueEnum, Copy, Clone, Debug, PartialEq, Eq)]
pub enum UpdateType {
    Patch,
    Minor,
    Major,
}

pub fn cmd(next_args: NextArgs) -> Result<(), Box<dyn Error>> {
    let next_version = match next_args.update_type {
        UpdateType::Patch => "0.0.1",
        UpdateType::Minor => "0.1.0",
        UpdateType::Major => "1.0.0",
    };
    println!("Next version is {}", &next_version);

    Ok(())
}
