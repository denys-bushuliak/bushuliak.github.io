use std::error::Error;
use builder::{run, Args};
use clap::Parser;

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse().try_into()?;
    run(args)
}
