use std::thread::current;

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
        sim.gen_demo();
        sim
    }

    fn gen_demo(&mut self) {
        for x in 0..self.grid.width {
            for y in 0..self.grid.height {
                let idx = self.grid.get_index(x, y);
                if y > self.grid.height - 20 || (x > 80 && x < 220 && y > 80 && y < 100) {
                    self.grid.current_frame[idx] = Cell {
                        element_type: ElementType::Sand,
                        color: [194, 177, 128, 255],
                        vel_y: 0.0,
                        temp: 25.0,
                    };
                }
            }
        }
        self.update_raw_buffer();
        self.grid.next_frame.copy_from_slice(&self.grid.current_frame);
    }

    pub fn update_raw_buffer(&mut self) {
        for (i, cell) in self.grid.current_frame.iter().enumerate() {
            let buffer_idx = i * 4;
            self.raw_output_buffer[buffer_idx..buffer_idx + 4].copy_from_slice(&cell.color);
        }
    }

    pub fn step(&mut self) {
        let width = self.grid.width;
        let height = self.grid.height;

        for y in (0..height).rev() {
            for x in 0..width {
                let current_idx = self.grid.get_index(x, y);
                let cell = self.grid.current_frame[current_idx];

                // maybe extrapolate all element behaviours
                // into seperate components?
                // cleaner that way
                match cell.element_type {
                    ElementType::Sand => {
                        if y + 1 < height {
                            let down_idx = self.grid.get_index(x, y + 1);
                            let below_element = self.grid.current_frame[down_idx].element_type;

                            if below_element == ElementType::Empty {
                                self.grid.next_frame[current_idx] = Cell::empty();
                                self.grid.next_frame[down_idx] = cell;
                                continue;
                            } else {
                                let left_first = (current_idx % 2 == 0);

                                let mut moved = false;
                                let diagonals = if left_first { [x.checked_sub(1), Some(x + 1)] } else { [Some(x + 1), x.checked_sub(1)] };

                                for target_x in diagonals {
                                    if let Some(nx) = target_x {
                                        if nx < width {
                                            let diag_idx = self.grid.get_index(nx, y + 1);
                                            if self.grid.current_frame[diag_idx].element_type == ElementType::Empty {
                                                self.grid.next_frame[current_idx] = Cell::empty();
                                                self.grid.next_frame[diag_idx] = cell;
                                                moved = true;
                                                break;
                                            }
                                        }
                                    }
                                }
                                if !moved {
                                    self.grid.next_frame[current_idx] = cell;
                                }
                            }
                        } else {
                            self.grid.next_frame[current_idx] = cell;
                        }
                    },
                    _ => {},
                }
            }
        }



        self.grid.swap_buffers();
        self.update_raw_buffer();
    }
}