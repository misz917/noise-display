pub struct ScreenBuffer {
    buffer: Vec<u32>,
    width: usize,
    height: usize,
}

impl ScreenBuffer {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            buffer: vec![0; width * height],
            width,
            height,
        }
    }

    #[inline]
    pub fn width(&self) -> usize {
        self.width
    }

    #[inline]
    pub fn height(&self) -> usize {
        self.height
    }

    #[inline]
    pub fn get_buffer(&self) -> &[u32] {
        &self.buffer
    }

    #[inline]
    pub fn get_buffer_mut(&mut self) -> &mut [u32] {
        &mut self.buffer
    }
}
