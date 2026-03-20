use crate::image_source::ImageSource;
use image::DynamicImage;
use std::{collections::LinkedList, fs, path::Path};

pub struct DirImageSource {
    memory: LinkedList<DynamicImage>,
}

impl ImageSource for DirImageSource {
    fn new(path: &Path) -> Self {
        assert!(path.is_dir());

        let mut memory = LinkedList::new();

        let paths = fs::read_dir(path).unwrap();
        for (i, file_name) in paths.map(|f| f.unwrap().file_name()).enumerate() {
            let image = image::open(path.join(file_name)).unwrap();
            memory.push_back(image);
            print!("\r{}", i);
        }

        Self { memory }
    }

    fn next(&mut self) -> Option<image::DynamicImage> {
        self.memory.pop_front()
    }
}
