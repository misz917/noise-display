use tracing_subscriber::{EnvFilter, fmt};

use crate::cli::Cli;

pub mod cli;
pub mod color;
pub mod extract_frames;
pub mod image_source;
pub mod into_binary;
pub mod noise_strategy;
pub mod screen_buffer;

pub mod noise_display;

const DEFAULT_TARGET_FPS: usize = 30;
const BINARIZATION_THRESHOLD: u8 = 127;

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
