use tracing_subscriber::{EnvFilter, fmt};

use crate::cli::Cli;

pub mod binarization_strategy;
pub mod cli;
pub mod color;
pub mod extract_frames;
pub mod image_source;
pub mod noise_strategy;
pub mod screen_buffer;

pub mod noise_display;

fn init_tracing() {
    fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("off")),
        )
        .with_target(true)
        .with_thread_ids(true)
        .with_thread_names(true)
        .init();
}

fn main() {
    init_tracing();
    Cli::new_run().unwrap();
}
