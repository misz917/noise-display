use std::path::PathBuf;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(group(
    clap::ArgGroup::new("input")
        .required(true)
        .args(["image", "video", "text"])
))]
pub struct Args {
    #[arg(long)]
    pub image: Option<PathBuf>,

    #[arg(long)]
    pub video: Option<PathBuf>,

    #[arg(long)]
    pub text: Option<String>,
}
