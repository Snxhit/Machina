use super::element::Cell;

pub struct WorldGrid {
    pub width: u32,
    pub height: u32,
    pub current_frame: Vec<Cell>,
    pub next_frame: Vec<Cell>,
}

impl WorldGrid {
    pub fn new(width: u32, height: u32) -> Self {
        let size = (width * height) as usize;
        Self {
            width,
            height,
            current_frame: vec![Cell::empty(); size],
            next_frame: vec![Cell::empty(); size],
        }
    }

    #[inline(always)]
    pub fn get_index(&self, x: u32, y: u32) -> usize {
        (y * self.width + x) as usize
    }

    pub fn swap_buffers(&mut self) {
        std::mem::swap(&mut self.current_frame, &mut self.next_frame);

        self.next_frame.copy_from_slice(&self.current_frame);
    }
}