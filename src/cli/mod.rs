use crate::{
    TEMP_FILE_PATH,
    cli::args::Args,
    display::Display,
    extract_frames::extract_frames_with_ffmpeg,
    randomisation_strategy::{
        RandomisationStrategy, black_white::BlackWhiteStrategy, rainbow::RainbowStrategy,
        slide::SlideStrategy,
    },
};
use clap::Parser;
use std::{fs, path::PathBuf, str::FromStr};

pub mod args;

pub struct Cli {}

impl Cli {
    pub fn run() {
        let args = Args::parse();

        if let Some(path) = args.path {
            if let Some(extension) = path.extension() {
                let mut display = Display::new();
                display.set_noise_strategy(Box::new(BlackWhiteStrategy));

                let strat: Box<dyn RandomisationStrategy> = match args.strat.as_deref() {
                    Some("bw") | Some("blackwhite") => Box::new(BlackWhiteStrategy),
                    Some("r") | Some("rainbow") => Box::new(RainbowStrategy),
                    Some("s") | Some("slide") => Box::new(SlideStrategy),
                    _ => Box::new(BlackWhiteStrategy),
                };

                display.set_noise_strategy(strat);

                match extension.to_str().unwrap() {
                    "jpg" => {
                        display.run(&path);
                    }
                    "mp4" => {
                        _ = fs::create_dir(TEMP_FILE_PATH);
                        extract_frames_with_ffmpeg(
                            &path,
                            &PathBuf::from_str(TEMP_FILE_PATH).unwrap(),
                        )
                        .unwrap();

                        display.run(&PathBuf::from_str(TEMP_FILE_PATH).unwrap());
                        fs::remove_dir_all(TEMP_FILE_PATH).unwrap();
                    }
                    _ => {}
                }
            }
        }
    }
}
