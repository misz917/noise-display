use crate::{
    binarization_strategy::{BinarizationStrategy, basic::BasicBinarizationStrategy},
    image_source::{ImageSource, mock_source::MockSource},
    noise_display::{error_codes::NoiseDisplayError, interface::NoiseDisplayInterface},
    noise_strategy::NoiseStrategy,
    screen_buffer::ScreenBuffer,
};
use minifb::{Key, Scale, Window, WindowOptions};

pub mod error_codes;
pub mod interface;

const WINDOW_NAME: &str = "Noise Display";

struct RuntimeResources {
    screen_buffer: Option<ScreenBuffer>,
    window: Window,
}

pub struct NoiseDisplay {
    binarization_strategy: Box<dyn BinarizationStrategy>,
    noise_strategy: Box<dyn NoiseStrategy>,
    image_source: Option<Box<dyn ImageSource>>,
}

const DEFAULT_BINARIZATION_THRESHOLD: u8 = 127;
impl Default for NoiseDisplay {
    fn default() -> Self {
        Self::new(
            Box::new(BasicBinarizationStrategy::new(
                DEFAULT_BINARIZATION_THRESHOLD,
            )),
            Default::default(),
            None,
        )
    }
}

impl NoiseDisplayInterface for NoiseDisplay {
    fn new(
        binarization_strategy: Box<dyn BinarizationStrategy>,
        noise_strategy: Box<dyn NoiseStrategy>,
        image_source: Option<Box<dyn ImageSource>>,
    ) -> Self {
        Self {
            binarization_strategy,
            noise_strategy,
            image_source,
        }
    }

    fn run(&mut self) -> Result<(), NoiseDisplayError> {
        let runtime_resources = self.startup()?;
        self.main_loop(runtime_resources)
    }

    fn set_image_source(&mut self, image_source: Box<dyn ImageSource>) -> &mut Self {
        self.image_source = Some(image_source);

        return self;
    }

    fn set_noise_strategy(&mut self, noise_strategy: Box<dyn NoiseStrategy>) -> &mut Self {
        self.noise_strategy = noise_strategy;

        return self;
    }

    fn set_binarization_strategy(
        &mut self,
        binarization_strategy: Box<dyn BinarizationStrategy>,
    ) -> &mut Self {
        self.binarization_strategy = binarization_strategy;

        return self;
    }
}

impl NoiseDisplay {
    fn init_buffer(image_source: &Box<dyn ImageSource>) -> ScreenBuffer {
        let width = image_source.width();
        let height = image_source.height();
        ScreenBuffer::new(width, height)
    }

    fn init_window(image_source: &Box<dyn ImageSource>) -> Result<Window, minifb::Error> {
        let width = image_source.width();
        let height = image_source.height();
        let fps = image_source.fps();

        let mut window = Window::new(
            WINDOW_NAME,
            width,
            height,
            WindowOptions {
                scale: Scale::FitScreen,
                ..WindowOptions::default()
            },
        )?;
        window.set_target_fps(fps);

        Ok(window)
    }

    fn init_window_default() -> Result<Window, minifb::Error> {
        let mock_source: Box<dyn ImageSource> = Box::new(MockSource::default());
        let window = Self::init_window(&mock_source)?;
        Ok(window)
    }
}

impl NoiseDisplay {
    fn startup(&mut self) -> Result<RuntimeResources, NoiseDisplayError> {
        let mut screen_buffer_option: Option<ScreenBuffer> = None;
        let window: Window;

        if let Some(image_source) = &self.image_source {
            let mut screen_buffer = Self::init_buffer(&image_source);
            window = Self::init_window(&image_source)?;
            self.noise_strategy.init(&mut screen_buffer);

            screen_buffer_option = Some(screen_buffer);
        } else {
            window = Self::init_window_default()?;
        }

        Ok(RuntimeResources {
            screen_buffer: screen_buffer_option,
            window,
        })
    }

    fn main_loop(&mut self, runtime_resources: RuntimeResources) -> Result<(), NoiseDisplayError> {
        let mut mask: Option<Vec<bool>> = None;
        let mut current_image_index = 0;
        let mut last_image_index = 0;

        let mut screen_buffer = runtime_resources.screen_buffer;
        let mut window = runtime_resources.window;

        while window.is_open() && !window.is_key_down(Key::Escape) {
            if let Some(image_source) = &mut self.image_source {
                if let Some(mut indexed_image) = image_source.next() {
                    current_image_index = indexed_image.index();
                    mask = if let Some(next_image) = indexed_image.image_pop() {
                        Some(self.binarization_strategy.binarize(next_image))
                    } else {
                        None
                    }
                }
            }

            if current_image_index != last_image_index {
                last_image_index = current_image_index;
                println!("{}", current_image_index);
            }

            if let Some(screen_buffer) = &mut screen_buffer {
                if let Some(mask) = &mask {
                    self.noise_strategy.randomise(screen_buffer, Some(&mask));
                }

                window
                    .update_with_buffer(
                        screen_buffer.get_buffer(),
                        screen_buffer.width(),
                        screen_buffer.height(),
                    )
                    .map_err(|_| NoiseDisplayError::FailedWindowUpdate)?;
            }
        }

        Ok(())
    }
}
