use super::element::{Cell, ElementType};
use super::grid::WorldGrid;

pub struct Simulation {
    pub grid: WorldGrid,
    pub raw_output_buffer: Vec<u8>,
    pub rng_state: u64,
}

impl Simulation {
    pub fn new(width: u32, height: u32) -> Self {
        let grid = WorldGrid::new(width, height);

        let raw_output_buffer = vec![0; (width * height * 4) as usize];

        let mut sim = Self { grid, raw_output_buffer, rng_state: 0x9E3779B97F4A7C15 };
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

    pub fn spawn_pixel(&mut self, x: u32, y: u32, element: ElementType) {
        if x < self.grid.width && y < self.grid.height {
            let idx = self.grid.get_index(x, y);

            if self.grid.current_frame[idx].element_type == ElementType::Empty {
                let color = match element {
                    ElementType::Sand => [194, 177, 128, 255],
                    ElementType::Water => [14, 135, 204, 255],
                    ElementType::Wall => [110, 110, 110, 255],
                    _ => [0, 0, 0, 255],
                };

                let cell = Cell {
                    element_type: element,
                    color,
                    vel_y: 0.0,
                    temp: 25.0,
                };

                self.grid.current_frame[idx] = cell;
                self.grid.next_frame[idx] = cell;
            }
        }
    }

    pub fn step(&mut self) {
        let width = self.grid.width;
        let height = self.grid.height;

        for cell in self.grid.next_frame.iter_mut() {
            *cell = Cell::empty();
        }

        for y in (0..height).rev() {
            let left_to_right = y % 2 == 0;
            for i in 0..width {
                let x = if left_to_right { i } else { width - 1 - i };
                let current_idx = self.grid.get_index(x, y);
                let cell = self.grid.current_frame[current_idx];

                // maybe extrapolate all element behaviours
                // into seperate components?
                // cleaner that way
                match cell.element_type {
                    ElementType::Sand => {
                        if y + 1 < height {
                            let down_idx = self.grid.get_index(x, y + 1);
                            let below = self.grid.next_frame[down_idx];

                            if below.element_type == ElementType::Empty {
                                self.grid.next_frame[current_idx] = Cell::empty();
                                self.grid.next_frame[down_idx] = cell;
                                continue;
                            } else if below.element_type == ElementType::Water {
                                self.grid.next_frame[current_idx] = below;
                                self.grid.next_frame[down_idx] = cell;
                                continue;
                            } else {
                                let mut moved = false;

                                self.rng_state ^= self.rng_state << 13;
                                self.rng_state ^= self.rng_state >> 7;
                                self.rng_state ^= self.rng_state << 17;

                                let diagonals = if self.rng_state & 1 == 0 { [x.checked_sub(1), Some(x + 1)] } else { [Some(x + 1), x.checked_sub(1)] };

                                for target_x in diagonals {
                                    if let Some(nx) = target_x {
                                        if nx < width {
                                            let diag_idx = self.grid.get_index(nx, y + 1);
                                            let down_idx = self.grid.get_index(nx, y);
                                            if self.grid.next_frame[diag_idx].element_type == ElementType::Empty && self.grid.current_frame[down_idx].element_type == ElementType::Empty && self.grid.next_frame[down_idx].element_type == ElementType::Empty {
                                                self.grid.next_frame[current_idx] = Cell::empty();
                                                self.grid.next_frame[diag_idx] = cell;
                                                moved = true;
                                                break;
                                            } else if self.grid.next_frame[diag_idx].element_type == ElementType::Water && self.grid.current_frame[down_idx].element_type == ElementType::Empty && self.grid.next_frame[down_idx].element_type == ElementType::Empty {
                                                self.grid.next_frame[current_idx] = self.grid.current_frame[diag_idx];
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

                    ElementType::Water => {
                        let mut moved = false;

                        if y + 1 < height {
                            let down_idx = self.grid.get_index(x, y + 1);
                            if self.grid.current_frame[down_idx].element_type == ElementType::Empty
                                && self.grid.next_frame[down_idx].element_type == ElementType::Empty {
                                self.grid.next_frame[down_idx] = cell;
                                moved = true;
                            }
                        }

                        if !moved && y + 1 < height {
                            let left_first = ((x + y) % 2 == 0);
                            let diagonals = if left_first { [x.checked_sub(1), Some(x + 1)] } else { [Some(x + 1), x.checked_sub(1)] };

                            for target_x in diagonals {
                                if let Some(nx) = target_x {
                                    if nx < width {
                                        let diag_idx = self.grid.get_index(nx, y + 1);
                                        let down_idx = self.grid.get_index(nx, y);
                                        if self.grid.current_frame[diag_idx].element_type == ElementType::Empty && self.grid.current_frame[down_idx].element_type == ElementType::Empty
                                            && self.grid.next_frame[diag_idx].element_type == ElementType::Empty && self.grid.next_frame[down_idx].element_type == ElementType::Empty {
                                            self.grid.next_frame[diag_idx] = cell;
                                            moved = true;
                                            break;
                                        }
                                    }
                                }
                            }
                        }

                        let down_idx = self.grid.get_index(x, y + 1);
                        let supported = y + 1 >= height
                            || self.grid.current_frame[down_idx].element_type != ElementType::Empty
                            || self.grid.next_frame[self.grid.get_index(x, y + 1)].element_type != ElementType::Empty;

                        if !moved && supported {
                            let left_first = ((x + y) % 2 == 0);
                            let directions = if left_first { [-1, 1] } else { [1, -1] };

                            let disp_rate = 5;

                            for dx in directions {
                                for i in 1..=disp_rate {
                                    let target_x = (x as i32) + (dx * i);

                                    if target_x < 0 || target_x >= (width as i32) {
                                        break;
                                    }

                                    let nx = target_x as u32;
                                    let check_idx = self.grid.get_index(nx, y);

                                    if self.grid.current_frame[check_idx].element_type != ElementType::Empty {
                                        break;
                                    }

                                    if self.grid.next_frame[check_idx].element_type != ElementType::Empty {
                                        continue;
                                    }

                                    if y + 1 < height {
                                        let down_check_idx = self.grid.get_index(nx, y + 1);
                                        if self.grid.current_frame[down_check_idx].element_type == ElementType::Empty
                                            && self.grid.next_frame[down_check_idx].element_type == ElementType::Empty {
                                            self.grid.next_frame[check_idx] = cell;
                                            moved = true;
                                            break;
                                        }
                                    }

                                    if i == disp_rate || (nx == 0 || nx == width - 1) {
                                        self.grid.next_frame[check_idx] = cell;
                                        moved = true;
                                        break;
                                    }
                                }
                                if moved { break; }
                            }
                        }

                        if !moved {
                            if self.grid.next_frame[current_idx].element_type == ElementType::Empty {
                                self.grid.next_frame[current_idx] = cell;
                            }
                        }
                    }

                    ElementType::Wall => {
                        self.grid.next_frame[current_idx] = cell;
                    }

                    _ => {},
                }
            }
        }

        self.grid.swap_buffers();
        self.update_raw_buffer();
    }
}