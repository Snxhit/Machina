use super::element::{Cell, ElementType};
use super::grid::WorldGrid;

pub struct Simulation {
    pub grid: WorldGrid,
    pub raw_output_buffer: Vec<u8>,
}

impl Simulation {
    pub fn new(width: u32, height: u32) -> Self {
        let grid = WorldGrid::new(width, height);

        let raw_output_buffer = vec![0; (width * height * 4) as usize];

        let mut sim = Self { grid, raw_output_buffer };
        sim
    }

    fn gen_demo(&mut self) {
        for x in 0..self.grid.width {
            for y in 0..self.grid.height {
                let idx = self.grid.get_index(x, y);
                if y > self.grid.height - 20 || (x > 100 && x < 200 && y > 80 && y < 90) {
                    self.grid.current_frame[idx] = Cell {
                        element_type: ElementType::Sand,
                        color: [0, 0, 0, 1],
                        vel_y: 0.0,
                        temp: 25.0,
                    };
                }
            }
        }
        self.update_raw_buffer();
    }

    pub fn update_raw_buffer(&mut self) {
        for (i, cell) in self.grid.current_frame.iter().enumerate() {
            let buffer_idx = i * 4;
            self.raw_output_buffer[buffer_idx..buffer_idx + 4].copy_from_slice(&cell.color);
        }
    }

    pub fn step(&mut self) {
        self.grid.swap_buffers();
        self.update_raw_buffer();
    }
}