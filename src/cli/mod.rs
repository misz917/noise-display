use crate::{
    BINARIZATION_THRESHOLD, DEFAULT_TARGET_FPS,
    cli::args::Args,
    image_source::image_source_factory::ImageSourceFactory,
    noise_display::{NoiseDisplay, interface::NoiseDisplayInterface},
    noise_strategy::{
        NoiseStrategy, black_white::BlackWhiteStrategy, rainbow::RainbowStrategy,
        slide::SlideStrategy,
    },
};
use clap::Parser;

pub mod args;

pub struct Cli;

impl Cli {
    pub fn new_run() -> Result<(), Box<dyn std::error::Error>> {
        let args = Args::parse();

        let path = args.path.ok_or("Path not provided")?;
        let strategy: Box<dyn NoiseStrategy> = match args.strat.as_deref() {
            Some("bw") => Box::new(BlackWhiteStrategy),
            Some("r") => Box::new(RainbowStrategy),
            Some("s") => Box::new(SlideStrategy),
            _ => Box::new(BlackWhiteStrategy),
        };

        let image_source = ImageSourceFactory::new_image_source(&path);
        let mut display = NoiseDisplay::new(DEFAULT_TARGET_FPS, strategy, BINARIZATION_THRESHOLD);
        display.display(image_source);

        Ok(())
    }
}

/*
impl Cli {
    pub fn run() {
        let args = Args::parse();

        if let Some(path) = args.path {
            if let Some(extension) = path.extension() {
                let mut display = Display::new();
                display.set_noise_strategy(Box::new(BlackWhiteStrategy));

                let strat: Box<dyn NoiseStrategy> = match args.strat.as_deref() {
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
*/
