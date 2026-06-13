use std::path::PathBuf;

/// Program to parse markdown to html
#[derive(clap::Parser, Debug)]
#[command(about, long_about)]
pub struct Args {
    /// Source folder with markdowns
    #[arg(short, long)]
    pub r#in: PathBuf,

    /// Destination folder for prepared html
    #[arg(short, long)]
    pub out: PathBuf,
}
