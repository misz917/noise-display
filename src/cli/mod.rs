use crate::{
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
        let image_source = ImageSourceFactory::new_image_source(&path).unwrap();

        let mut display = NoiseDisplay::default();
        display.set_image_source(image_source);
        display.set_noise_strategy(strategy);
        display.run().unwrap();

        Ok(())
    }
}
