#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ElementType {
    Empty,
    Sand,
    Water,
    Wall,
    Custom,
}

#[derive(Clone, Copy, Debug)]
pub struct Cell {
    pub element_type: ElementType,
    pub color: [u8; 4],
    pub vel_y: f32,
    pub temp: f32,
}

impl Cell {
    pub fn empty() -> Self {
        Self {
            element_type: ElementType::Empty,
            color: [0, 0, 0, 1],
            vel_y: 0.0,
            temp: 25.0,
        }
    }
}