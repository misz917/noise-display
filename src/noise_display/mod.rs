use crate::{
    image_source::{ImageSource, mock_source::MockSource},
    into_binary::IntoFlatBinary,
    noise_display::{error_codes::NoiseDisplayError, interface::NoiseDisplayInterface},
    noise_strategy::NoiseStrategy,
    screen_buffer::ScreenBuffer,
};
use minifb::{Key, Scale, Window, WindowOptions};

pub mod error_codes;
pub mod interface;

const WINDOW_NAME: &str = "Noise Display";
const DEFAULT_TARGET_FPS: usize = 30;

struct RuntimeResources {
    screen_buffer: ScreenBuffer,
    window: Window,
    target_fps: usize,
}

pub struct NoiseDisplay {
    binarization_threshold: u8,
    // binarization_strategy: Box<dyn BinarizationStrategy>,
    noise_strategy: Box<dyn NoiseStrategy>,
    image_source: Option<Box<dyn ImageSource>>,
    screen_buffer: Option<ScreenBuffer>,
    window: Option<Window>,
    target_fps: usize,
}

impl NoiseDisplayInterface for NoiseDisplay {
    fn new(
        binarization_threshold: u8,
        noise_strategy: Box<dyn NoiseStrategy>,
        image_source: Option<Box<dyn ImageSource>>,
    ) -> Self {
        Self {
            binarization_threshold,
            noise_strategy,
            image_source,
            screen_buffer: None,
            window: None,
            target_fps: DEFAULT_TARGET_FPS,
        }
    }

    fn set_image_source(&mut self, image_source: Box<dyn ImageSource>) -> &mut Self {
        self.image_source = Some(image_source);

        return self;
    }

    fn run(&mut self) -> Result<(), NoiseDisplayError> {
        self.startup()?;
        self.main_loop()
    }
}

impl Default for NoiseDisplay {
    fn default() -> Self {
        Self::new(127, Default::default(), None)
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
    fn startup(&mut self) -> Result<(), NoiseDisplayError> {
        if let Some(image_source) = &self.image_source {
            let mut screen_buffer = Self::init_buffer(&image_source);
            let window = Self::init_window(&image_source)?;
            let target_fps = image_source.fps();

            self.noise_strategy.init(&mut screen_buffer);

            self.screen_buffer = Some(screen_buffer);
            self.window = Some(window);
            self.target_fps = target_fps;
        } else {
            self.window = Some(Self::init_window_default()?);
            self.target_fps = DEFAULT_TARGET_FPS;
        }

        Ok(())
    }

    fn main_loop(&mut self) -> Result<(), NoiseDisplayError> {
        let mut mask: Option<Vec<bool>> = None;
        let mut current_image_index = 0;
        let mut last_image_index = 0;

        let window = self.window.as_mut().ok_or(NoiseDisplayError::Unexpected)?;
        while window.is_open() && !window.is_key_down(Key::Escape) {
            if let Some(image_source) = &mut self.image_source {
                if let Some(mut indexed_image) = image_source.next() {
                    current_image_index = indexed_image.index();
                    mask = if let Some(next_image) = indexed_image.image_pop() {
                        Some(next_image.binarize_and_flatten(self.binarization_threshold))
                    } else {
                        None
                    }
                }
            }

            if current_image_index != last_image_index {
                last_image_index = current_image_index;
                println!("{}", current_image_index);
            }

            if let Some(screen_buffer) = &mut self.screen_buffer {
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

/*
   fn display(&mut self, mut image_source: Box<dyn ImageSource>) {
       self.image_source = Some(image_source);
       self.startup();

       ////////

       // first time it has to return a Some
       let mut image = image_source.next().unwrap();

       let width = image.width() as usize;
       let height = image.height() as usize;

       let mut window = self.init_window(width, height, self.target_fps).unwrap();
       let mut screen_buffer = ScreenBuffer::new(width, height);

       let mut mask = image.binarize_and_flatten(self.binarization_threshold);

       self.noise_strategy.init(&mut screen_buffer);

       while window.is_open() && !window.is_key_down(Key::Escape) {
           self.noise_strategy
               .randomise(&mut screen_buffer, Some(&mask));

           window
               .update_with_buffer(screen_buffer.get_buffer(), width, height)
               .unwrap();

           if let Some(new_image) = image_source.next() {
               image = new_image;
               mask = image.binarize_and_flatten(self.binarization_threshold);
           }
       }
   }
*/
