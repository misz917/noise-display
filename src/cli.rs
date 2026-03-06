use std::path::PathBuf;

use clap::Parser;

#[derive(Parser, Debug)]
pub struct Args {
    pub path: Option<PathBuf>,
    #[arg(short, long, default_value_t = false)]
    pub negative: bool,
}
